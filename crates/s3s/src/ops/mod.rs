mod generated;
pub use self::generated::*;

use crate::auth::Credentials;
use crate::auth::S3Auth;
use crate::error::*;
use crate::header;
use crate::http;
use crate::http::{AwsChunkedStream, Body, Multipart};
use crate::http::{OrderedHeaders, OrderedQs};
use crate::http::{Request, Response};
use crate::path::{ParseS3PathError, S3Path};
use crate::request::S3Request;
use crate::s3_trait::S3;
use crate::sig_v2;
use crate::sig_v2::AuthorizationV2;
use crate::sig_v2::PresignedUrlV2;
use crate::sig_v4;
use crate::sig_v4::PostSignatureInfo;
use crate::sig_v4::PresignedUrlV4;
use crate::sig_v4::{AmzContentSha256, AmzDate};
use crate::sig_v4::{AuthorizationV4, CredentialV4};
use crate::stream::aggregate_unlimited;
use crate::stream::VecByteStream;
use crate::utils::is_base64_encoded;

use std::mem;
use std::ops::Not;
use std::sync::Arc;

use bytes::Bytes;
use bytestring::ByteString;
use hyper::http::HeaderValue;
use hyper::HeaderMap;
use hyper::Method;
use hyper::StatusCode;
use hyper::Uri;
use mime::Mime;
use tracing::debug;

#[async_trait::async_trait]
pub trait Operation: Send + Sync + 'static {
    fn name(&self) -> &'static str;

    async fn call(&self, s3: &Arc<dyn S3>, req: &mut Request) -> S3Result<Response>;
}

fn build_s3_request<T>(input: T, req: &mut Request) -> S3Request<T> {
    let credentials = req.s3ext.credentials.take();
    let extensions = mem::take(&mut req.extensions);
    let headers = mem::take(&mut req.headers);
    let uri = mem::take(&mut req.uri);

    S3Request {
        input,
        credentials,
        extensions,
        headers,
        uri,
    }
}

fn serialize_error(x: S3Error) -> S3Result<Response> {
    let status = x.status_code().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let mut res = Response::with_status(status);
    http::set_xml_body(&mut res, &x)?;
    Ok(res)
}

fn unknown_operation() -> S3Error {
    S3Error::with_message(S3ErrorCode::NotImplemented, "Unknown operation")
}

fn extract_host(req: &Request) -> S3Result<Option<String>> {
    let Some(val) = req.headers.get(crate::header::HOST) else { return Ok(None) };
    let on_err = |e| s3_error!(e, InvalidRequest, "invalid header: Host: {val:?}");
    let host = val.to_str().map_err(on_err)?;
    Ok(Some(host.into()))
}

fn extract_s3_path(host: Option<&str>, uri_path: &str, base_domain: Option<&str>) -> S3Result<S3Path> {
    let result = match (base_domain, host) {
        (Some(base_domain), Some(host)) => {
            debug!(?base_domain, ?host, ?uri_path, "parsing virtual-hosted-style request");
            crate::path::parse_virtual_hosted_style(base_domain, host, uri_path)
        }
        _ => {
            debug!(?uri_path, "parsing path-style request");
            crate::path::parse_path_style(uri_path)
        }
    };

    result.map_err(|err| match err {
        ParseS3PathError::InvalidPath => s3_error!(InvalidURI),
        ParseS3PathError::InvalidBucketName => s3_error!(InvalidBucketName),
        ParseS3PathError::KeyTooLong => s3_error!(KeyTooLongError),
    })
}

fn extract_qs(req_uri: &Uri) -> S3Result<Option<OrderedQs>> {
    let Some(query) = req_uri.query() else { return Ok(None) };
    match OrderedQs::parse(query) {
        Ok(ans) => Ok(Some(ans)),
        Err(source) => Err(S3Error::with_source(S3ErrorCode::InvalidURI, Box::new(source))),
    }
}

fn check_query_pattern(qs: &OrderedQs, name: &str, val: &str) -> bool {
    match qs.get_unique(name) {
        Some(v) => v == val,
        None => false,
    }
}

fn extract_headers(headers: &HeaderMap<HeaderValue>) -> S3Result<OrderedHeaders<'_>> {
    OrderedHeaders::from_headers(headers).map_err(|source| invalid_request!(source, "invalid headers"))
}

fn extract_mime(hs: &OrderedHeaders<'_>) -> S3Result<Option<Mime>> {
    let Some(content_type) = hs.get_unique(crate::header::CONTENT_TYPE) else { return Ok(None) };
    match content_type.parse::<Mime>() {
        Ok(x) => Ok(Some(x)),
        Err(e) => Err(invalid_request!(e, "invalid content type")),
    }
}

fn extract_amz_content_sha256<'a>(hs: &'_ OrderedHeaders<'a>) -> S3Result<Option<AmzContentSha256<'a>>> {
    let Some(val) = hs.get_unique(crate::header::X_AMZ_CONTENT_SHA256) else { return Ok(None) };
    match AmzContentSha256::parse(val) {
        Ok(x) => Ok(Some(x)),
        Err(e) => {
            let mut err: S3Error = S3ErrorCode::Custom(ByteString::from_static("XAmzContentSHA256Mismatch")).into();
            err.set_message("invalid header: x-amz-content-sha256");
            err.set_source(Box::new(e));
            Err(err)
        }
    }
}

fn extract_authorization_v4<'a>(hs: &'_ OrderedHeaders<'a>) -> S3Result<Option<AuthorizationV4<'a>>> {
    let Some(val) = hs.get_unique(crate::header::AUTHORIZATION) else { return Ok(None) };
    match AuthorizationV4::parse(val) {
        Ok(x) => Ok(Some(x)),
        Err(e) => Err(invalid_request!(e, "invalid header: authorization")),
    }
}

fn extract_amz_date(hs: &'_ OrderedHeaders<'_>) -> S3Result<Option<AmzDate>> {
    let Some(val) = hs.get_unique(crate::header::X_AMZ_DATE) else { return Ok(None) };
    match AmzDate::parse(val) {
        Ok(x) => Ok(Some(x)),
        Err(e) => Err(invalid_request!(e, "invalid header: x-amz-date")),
    }
}

fn extract_content_length(req: &Request) -> Option<u64> {
    req.headers
        .get(hyper::header::CONTENT_LENGTH)
        .and_then(|val| atoi::atoi::<u64>(val.as_bytes()))
}

fn extract_decoded_content_length(hs: &'_ OrderedHeaders<'_>) -> S3Result<Option<usize>> {
    let Some(val) = hs.get_unique(crate::header::X_AMZ_DECODED_CONTENT_LENGTH) else { return Ok(None) };
    match atoi::atoi::<usize>(val.as_bytes()) {
        Some(x) => Ok(Some(x)),
        None => Err(invalid_request!("invalid header: x-amz-decoded-content-length")),
    }
}

async fn extract_full_body(content_length: Option<u64>, body: &mut Body) -> S3Result<Bytes> {
    if let Some(bytes) = body.bytes() {
        return Ok(bytes);
    }

    let bytes = body
        .store_all_unlimited()
        .await
        .map_err(|e| S3Error::with_source(S3ErrorCode::InternalError, e))?;

    if bytes.is_empty().not() {
        let content_length = content_length.ok_or(S3ErrorCode::MissingContentLength)?;
        if bytes.len() as u64 != content_length {
            return Err(s3_error!(IncompleteBody));
        }
    }

    Ok(bytes)
}

#[allow(clippy::declare_interior_mutable_const)]
fn fmt_content_length(len: usize) -> http::HeaderValue {
    const ZERO: http::HeaderValue = http::HeaderValue::from_static("0");
    if len > 0 {
        crate::utils::fmt_usize(len, |s| http::HeaderValue::try_from(s).unwrap())
    } else {
        ZERO
    }
}

pub async fn call(
    req: &mut Request,
    s3: &Arc<dyn S3>,
    auth: Option<&dyn S3Auth>,
    base_domain: Option<&str>,
) -> S3Result<Response> {
    let op = match prepare(req, auth, base_domain).await {
        Ok(op) => op,
        Err(err) => {
            debug!(?err, "failed to prepare");
            return serialize_error(err);
        }
    };

    match op.call(s3, req).await {
        Ok(res) => Ok(res),
        Err(err) => {
            debug!(op = %op.name(), ?err, "op returns error");
            serialize_error(err)
        }
    }
}

async fn prepare(req: &mut Request, auth: Option<&dyn S3Auth>, base_domain: Option<&str>) -> S3Result<&'static dyn Operation> {
    let s3_path;
    let mut content_length;
    {
        let decoded_uri_path = urlencoding::decode(req.uri.path())
            .map_err(|_| S3ErrorCode::InvalidURI)?
            .into_owned();

        let host = extract_host(req)?;

        req.s3ext.s3_path = Some(extract_s3_path(host.as_deref(), &decoded_uri_path, base_domain)?);
        s3_path = req.s3ext.s3_path.as_ref().unwrap();

        req.s3ext.qs = extract_qs(&req.uri)?;
        content_length = extract_content_length(req);

        let hs = extract_headers(&req.headers)?;
        let mime = extract_mime(&hs)?;
        let decoded_content_length = extract_decoded_content_length(&hs)?;

        let body_changed;
        let transformed_body;
        {
            let mut scx = SignatureContext {
                auth,
                base_domain,

                req_method: &req.method,
                req_uri: &req.uri,
                req_body: &mut req.body,

                qs: req.s3ext.qs.as_ref(),
                hs,

                decoded_uri_path,

                host: host.as_deref(),
                content_length,
                decoded_content_length,
                mime,

                multipart: None,
                transformed_body: None,
            };

            let credentials = scx.check().await?;

            body_changed = scx.transformed_body.is_some() || scx.multipart.is_some();
            transformed_body = scx.transformed_body;

            req.s3ext.multipart = scx.multipart;
            req.s3ext.credentials = credentials;
        }

        if body_changed {
            // invalidate the original content length
            if let Some(val) = req.headers.get_mut(header::CONTENT_LENGTH) {
                *val = fmt_content_length(decoded_content_length.unwrap_or(0))
            }
            if let Some(val) = &mut content_length {
                *val = 0;
            }
        }
        if let Some(body) = transformed_body {
            req.body = body;
        }

        let has_multipart = req.s3ext.multipart.is_some();
        debug!(?body_changed, ?decoded_content_length, ?has_multipart);
    }

    let (op, needs_full_body) = 'resolve: {
        if let Some(multipart) = &mut req.s3ext.multipart {
            if req.method == Method::POST {
                match s3_path {
                    S3Path::Root => return Err(unknown_operation()),
                    S3Path::Bucket { .. } => {
                        // POST object
                        debug!(?multipart);
                        let file_stream = multipart.take_file_stream().expect("missing file stream");
                        let vec_bytes = aggregate_unlimited(file_stream).await.map_err(S3Error::internal_error)?;
                        let vec_stream = VecByteStream::new(vec_bytes);
                        req.s3ext.vec_stream = Some(vec_stream);
                        break 'resolve (&PutObject as &'static dyn Operation, false);
                    }
                    S3Path::Object { .. } => return Err(s3_error!(MethodNotAllowed)),
                }
            }
        }
        resolve_route(req, s3_path, req.s3ext.qs.as_ref())?
    };

    debug!(op = %op.name(), ?s3_path, "resolved route");

    if needs_full_body {
        extract_full_body(content_length, &mut req.body).await?;
    }

    Ok(op)
}

struct SignatureContext<'a> {
    auth: Option<&'a dyn S3Auth>,
    base_domain: Option<&'a str>,

    req_method: &'a Method,
    req_uri: &'a Uri,
    req_body: &'a mut Body,

    qs: Option<&'a OrderedQs>,
    hs: OrderedHeaders<'a>,

    decoded_uri_path: String,

    host: Option<&'a str>,
    content_length: Option<u64>,
    mime: Option<Mime>,
    decoded_content_length: Option<usize>,

    transformed_body: Option<Body>,
    multipart: Option<Multipart>,
}

fn require_auth(auth: Option<&dyn S3Auth>) -> S3Result<&dyn S3Auth> {
    auth.ok_or_else(|| s3_error!(NotImplemented, "This service has no authentication provider"))
}

impl SignatureContext<'_> {
    async fn check(&mut self) -> S3Result<Option<Credentials>> {
        if let Some(result) = self.v2_check().await {
            debug!("checked signature v2");
            return result.map(Some);
        }

        if let Some(result) = self.v4_check().await {
            debug!("checked signature v4");
            return result.map(Some);
        }

        if self.auth.is_some() {
            return Err(s3_error!(AccessDenied, "Signature is required"));
        }

        Ok(None)
    }

    #[tracing::instrument(skip(self))]
    async fn v4_check(&mut self) -> Option<S3Result<Credentials>> {
        // POST auth
        if self.req_method == Method::POST {
            if let Some(ref mime) = self.mime {
                if mime.type_() == mime::MULTIPART && mime.subtype() == mime::FORM_DATA {
                    debug!("checking post signature");
                    return Some(self.v4_check_post_signature().await);
                }
            }
        }

        // query auth
        if let Some(qs) = self.qs {
            if qs.has("X-Amz-Signature") {
                debug!("checking presigned url");
                return Some(self.v4_check_presigned_url().await);
            }
        }

        // header auth
        if self.hs.get_unique(crate::header::AUTHORIZATION).is_some() {
            debug!("checking header auth");
            return Some(self.v4_check_header_auth().await);
        }

        None
    }

    async fn v4_check_post_signature(&mut self) -> S3Result<Credentials> {
        let auth = require_auth(self.auth)?;

        let multipart = {
            let mime = self.mime.as_ref().unwrap(); // assume: multipart

            let boundary = mime
                .get_param(mime::BOUNDARY)
                .ok_or_else(|| invalid_request!("missing boundary"))?;

            let body = mem::take(self.req_body);
            http::transform_multipart(body, boundary.as_str().as_bytes())
                .await
                .map_err(|e| s3_error!(e, MalformedPOSTRequest))?
        };

        let info = PostSignatureInfo::extract(&multipart).ok_or_else(|| invalid_request!("missing required multipart fields"))?;

        if is_base64_encoded(info.policy.as_bytes()).not() {
            return Err(invalid_request!("invalid field: policy"));
        }

        if info.x_amz_algorithm != "AWS4-HMAC-SHA256" {
            return Err(s3_error!(
                NotImplemented,
                "x-amz-algorithm other than AWS4-HMAC-SHA256 is not implemented"
            ));
        }

        let credential =
            CredentialV4::parse(info.x_amz_credential).map_err(|_| invalid_request!("invalid field: x-amz-credential"))?;

        let amz_date = AmzDate::parse(info.x_amz_date).map_err(|_| invalid_request!("invalid field: x-amz-date"))?;

        let access_key = credential.access_key_id.to_owned();
        let secret_key = auth.get_secret_key(&access_key).await?;

        let string_to_sign = info.policy;
        let signature = sig_v4::calculate_signature(string_to_sign, &secret_key, &amz_date, credential.aws_region);

        if signature != info.x_amz_signature {
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        self.multipart = Some(multipart);
        Ok(Credentials { access_key, secret_key })
    }

    async fn v4_check_presigned_url(&mut self) -> S3Result<Credentials> {
        let qs = self.qs.unwrap(); // assume: qs has "X-Amz-Signature"

        let presigned_url = PresignedUrlV4::parse(qs).map_err(|err| invalid_request!(err, "missing presigned url v4 fields"))?;

        // ASK: how to use it?
        let _content_sha256: Option<AmzContentSha256<'_>> = extract_amz_content_sha256(&self.hs)?;

        {
            // check expiration
            let now = time::OffsetDateTime::now_utc();

            let date = presigned_url
                .amz_date
                .to_time()
                .ok_or_else(|| invalid_request!("invalid amz date"))?;

            let duration = now - date;
            if duration.is_positive().not() {
                return Err(invalid_request!("invalid presigned url date"))?;
            }

            if duration > presigned_url.expires {
                return Err(s3_error!(AccessDenied, "Request has expired"));
            }
        }

        let auth = require_auth(self.auth)?;
        let access_key = presigned_url.credential.access_key_id;
        let secret_key = auth.get_secret_key(access_key).await?;

        let signature = {
            let headers = self.hs.find_multiple(&presigned_url.signed_headers);
            let method = &self.req_method;
            let uri_path = &self.decoded_uri_path;

            let canonical_request = sig_v4::create_presigned_canonical_request(method, uri_path, qs.as_ref(), &headers);

            let region = presigned_url.credential.aws_region;
            let amz_date = &presigned_url.amz_date;
            let string_to_sign = sig_v4::create_string_to_sign(&canonical_request, amz_date, region);

            sig_v4::calculate_signature(&string_to_sign, &secret_key, amz_date, region)
        };

        if signature != presigned_url.signature {
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        Ok(Credentials {
            access_key: access_key.into(),
            secret_key,
        })
    }

    #[tracing::instrument(skip(self))]
    async fn v4_check_header_auth(&mut self) -> S3Result<Credentials> {
        let authorization: AuthorizationV4<'_> = {
            // assume: headers has "authorization"
            let mut a = extract_authorization_v4(&self.hs)?.unwrap();
            a.signed_headers.sort_unstable();
            a
        };

        let auth = require_auth(self.auth)?;

        let amz_content_sha256 =
            extract_amz_content_sha256(&self.hs)?.ok_or_else(|| invalid_request!("missing header: x-amz-content-sha256"))?;

        let access_key = authorization.credential.access_key_id;
        let secret_key = auth.get_secret_key(access_key).await?;

        let amz_date = extract_amz_date(&self.hs)?.ok_or_else(|| invalid_request!("missing header: x-amz-date"))?;

        let is_stream = matches!(amz_content_sha256, AmzContentSha256::MultipleChunks);

        let signature = {
            let method = &self.req_method;
            let uri_path = &self.decoded_uri_path;
            let query_strings: &[(String, String)] = self.qs.as_ref().map_or(&[], AsRef::as_ref);

            // here requires that `auth.signed_headers` is sorted
            let headers = self.hs.find_multiple(&authorization.signed_headers);

            let canonical_request = if is_stream {
                let payload = sig_v4::Payload::MultipleChunks;
                sig_v4::create_canonical_request(method, uri_path, query_strings, &headers, payload)
            } else if matches!(*self.req_method, Method::GET | Method::HEAD) {
                let payload = sig_v4::Payload::Empty;
                sig_v4::create_canonical_request(method, uri_path, query_strings, &headers, payload)
            } else {
                let bytes = extract_full_body(self.content_length, self.req_body).await?;

                if bytes.len() < 1024 {
                    debug!(len=?bytes.len(), body=?bytes, "extracted full body");
                } else {
                    debug!(len=?bytes.len(), "extracted full body");
                }

                let payload = if matches!(amz_content_sha256, AmzContentSha256::UnsignedPayload) {
                    sig_v4::Payload::Unsigned
                } else if bytes.is_empty() {
                    sig_v4::Payload::Empty
                } else {
                    sig_v4::Payload::SingleChunk(&bytes)
                };

                sig_v4::create_canonical_request(method, uri_path, query_strings, &headers, payload)
            };

            let region = authorization.credential.aws_region;
            let string_to_sign = sig_v4::create_string_to_sign(&canonical_request, &amz_date, region);
            sig_v4::calculate_signature(&string_to_sign, &secret_key, &amz_date, region)
        };

        if signature != authorization.signature {
            debug!(?signature, expected=?authorization.signature, "signature mismatch");
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        if is_stream {
            let decoded_content_length = self
                .decoded_content_length
                .ok_or_else(|| s3_error!(MissingContentLength, "missing header: x-amz-decoded-content-length"))?;

            let stream = AwsChunkedStream::new(
                mem::take(self.req_body),
                signature.into(),
                amz_date,
                authorization.credential.aws_region.into(),
                secret_key.clone(),
                decoded_content_length,
            );

            debug!(len=?stream.exact_remaining_length(), "aws-chunked");

            self.transformed_body = Some(Body::from(stream.into_byte_stream()));
        }

        Ok(Credentials {
            access_key: access_key.into(),
            secret_key,
        })
    }

    #[tracing::instrument(skip(self))]
    async fn v2_check(&mut self) -> Option<S3Result<Credentials>> {
        if let Some(qs) = self.qs {
            if qs.has("Signature") {
                debug!("checking presigned url");
                return Some(self.v2_check_presigned_url().await);
            }
        }

        if let Some(auth) = self.hs.get_unique(crate::header::AUTHORIZATION) {
            if let Ok(auth) = AuthorizationV2::parse(auth) {
                debug!("checking header auth");
                return Some(self.v2_check_header_auth(auth).await);
            }
        }

        None
    }

    async fn v2_check_header_auth(&mut self, auth_v2: AuthorizationV2<'_>) -> S3Result<Credentials> {
        let method = &self.req_method;
        let uri_s3_path = extract_s3_path(self.host, self.req_uri.path(), self.base_domain)?;

        let date = sig_v2::get_date(&self.hs).ok_or_else(|| invalid_request!("missing date"))?;

        let auth = require_auth(self.auth)?;
        let access_key = auth_v2.access_key;
        let secret_key = auth.get_secret_key(access_key).await?;

        let string_to_sign = sig_v2::create_string_to_sign(method, date, &self.hs, &uri_s3_path, self.qs);
        let signature = sig_v2::calculate_signature(&secret_key, &string_to_sign);

        if signature != auth_v2.signature {
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        Ok(Credentials {
            access_key: access_key.into(),
            secret_key,
        })
    }

    async fn v2_check_presigned_url(&mut self) -> S3Result<Credentials> {
        let qs = self.qs.unwrap(); // assume: qs has "Signature"
        let presigned_url = PresignedUrlV2::parse(qs).map_err(|err| invalid_request!(err, "missing presigned url v2 fields"))?;

        let uri_s3_path = extract_s3_path(self.host, self.req_uri.path(), self.base_domain)?;

        if time::OffsetDateTime::now_utc() > presigned_url.expires_time {
            return Err(s3_error!(AccessDenied, "Request has expired"));
        }

        let auth = require_auth(self.auth)?;
        let access_key = presigned_url.access_key;
        let secret_key = auth.get_secret_key(access_key).await?;

        let string_to_sign =
            sig_v2::create_string_to_sign(self.req_method, presigned_url.expires_str, &self.hs, &uri_s3_path, self.qs);
        let signature = sig_v2::calculate_signature(&secret_key, &string_to_sign);

        if signature != presigned_url.signature {
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        Ok(Credentials {
            access_key: access_key.into(),
            secret_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::service::S3Service;

    pub trait OutputSize<A> {
        type Output;
    }

    macro_rules! impl_output_size {
        (($($ty:tt,)*)) => {
            impl<$($ty,)* F, R> OutputSize<($($ty,)*)> for F
            where
                F: Fn($($ty,)*) -> R ,
            {
                type Output = R;
            }
        };
    }

    impl_output_size!(());
    impl_output_size!((A0,));
    impl_output_size!((A0, A1,));
    impl_output_size!((A0, A1, A2,));
    impl_output_size!((A0, A1, A2, A3,));

    #[inline]
    #[must_use]
    pub const fn output_size<F, A>(_: &F) -> usize
    where
        F: OutputSize<A>,
    {
        std::mem::size_of::<F::Output>()
    }

    #[test]
    #[ignore]
    fn track_future_size() {
        macro_rules! future_size {
            ($f:path, $v:expr) => {
                (stringify!($f), output_size(&$f), $v)
            };
        }

        #[rustfmt::skip]
        let sizes = [
            future_size!(S3Service::call,                           2600),
            future_size!(call,                                      1424),
            future_size!(prepare,                                   1352),
            future_size!(SignatureContext::check,                   744),
            future_size!(SignatureContext::v2_check,                280),
            future_size!(SignatureContext::v2_check_presigned_url,  168),
            future_size!(SignatureContext::v2_check_header_auth,    184),
            future_size!(SignatureContext::v4_check,                720),
            future_size!(SignatureContext::v4_check_post_signature, 368),
            future_size!(SignatureContext::v4_check_presigned_url,  456),
            future_size!(SignatureContext::v4_check_header_auth,    624),
        ];

        println!("{:#?}", sizes);
        for (name, size, expected) in sizes {
            assert_eq!(size, expected, "{name:?} size changed: prev {expected}, now {size}");
        }
    }
}
