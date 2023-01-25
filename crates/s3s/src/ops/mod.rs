mod generated;
pub use self::generated::*;

use crate::auth::S3Auth;
use crate::error::*;
use crate::header;
use crate::header::{AmzContentSha256, AmzDate, AuthorizationV4, CredentialV4};
use crate::http;
use crate::http::{AwsChunkedStream, Body, Multipart};
use crate::http::{OrderedHeaders, OrderedQs};
use crate::http::{Request, Response};
use crate::path::{ParseS3PathError, S3Path};
use crate::signature_v4;
use crate::signature_v4::PresignedUrl;
use crate::stream::wrap;
use crate::utils::is_base64_encoded;

use std::mem;
use std::ops::Not;

use bytes::Bytes;
use hyper::Method;
use hyper::StatusCode;
use mime::Mime;
use tracing::debug;

#[async_trait::async_trait]
pub trait Operation: Send + Sync + 'static {
    fn name(&self) -> &'static str;

    async fn call(&self, s3: &dyn S3, req: &mut Request) -> S3Result<Response>;
}

fn serialize_error(x: S3Error) -> S3Result<Response> {
    let mut res = Response::default();
    *res.status_mut() = x.status_code().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    http::set_xml_body(&mut res, &x)?;
    Ok(res)
}

fn extract_s3_path(req: &mut Request, base_domain: Option<&str>) -> S3Result<S3Path> {
    let uri_path = urlencoding::decode(req.uri().path()).map_err(|_| S3ErrorCode::InvalidURI)?;

    let result = match (base_domain, req.headers().get(crate::header::names::HOST)) {
        (Some(base_domain), Some(val)) => {
            let on_err = |e| s3_error!(e, InvalidRequest, "invalid header: Host: {val:?}");
            let host = val.to_str().map_err(on_err)?;

            debug!(?base_domain, ?host, ?uri_path, "parsing virtual-hosted-style request");
            crate::path::parse_virtual_hosted_style(base_domain, host, &uri_path)
        }
        _ => {
            debug!(?uri_path, "parsing path-style request");
            crate::path::parse_path_style(&uri_path)
        }
    };

    result.map_err(|err| match err {
        ParseS3PathError::InvalidPath => s3_error!(InvalidURI),
        ParseS3PathError::InvalidBucketName => s3_error!(InvalidBucketName),
        ParseS3PathError::KeyTooLong => s3_error!(KeyTooLongError),
    })
}

fn extract_qs(req: &mut Request) -> S3Result<Option<OrderedQs>> {
    let Some(query) = req.uri().query() else { return Ok(None) };
    match OrderedQs::parse(query) {
        Ok(ans) => Ok(Some(ans)),
        Err(source) => Err(S3Error::with_source(S3ErrorCode::InvalidURI, Box::new(source))),
    }
}

fn unknown_operation() -> S3Error {
    S3Error::with_message(S3ErrorCode::NotImplemented, "Unknown operation")
}

fn check_query_pattern(qs: &OrderedQs, name: &str, val: &str) -> bool {
    match qs.get_unique(name) {
        Some(v) => v == val,
        None => false,
    }
}

fn extract_headers(req: &Request) -> S3Result<OrderedHeaders<'_>> {
    OrderedHeaders::from_headers(req.headers()).map_err(|source| invalid_request!(source, "invalid headers"))
}

fn extract_mime(hs: &OrderedHeaders<'_>) -> S3Result<Option<Mime>> {
    let Some(content_type) = hs.get(crate::header::names::CONTENT_TYPE) else { return Ok(None) };
    match content_type.parse::<Mime>() {
        Ok(x) => Ok(Some(x)),
        Err(e) => Err(invalid_request!(e, "invalid content type")),
    }
}

fn extract_amz_content_sha256<'a>(hs: &'_ OrderedHeaders<'a>) -> S3Result<Option<AmzContentSha256<'a>>> {
    let Some(val) = hs.get(crate::header::names::X_AMZ_CONTENT_SHA256) else { return Ok(None) };
    match AmzContentSha256::parse(val) {
        Ok(x) => Ok(Some(x)),
        Err(e) => {
            let mut err: S3Error = S3ErrorCode::Unknown("XAmzContentSHA256Mismatch".into()).into();
            err.set_message("invalid header: x-amz-content-sha256");
            err.set_source(Box::new(e));
            Err(err)
        }
    }
}

fn extract_authorization_v4<'a>(hs: &'_ OrderedHeaders<'a>) -> S3Result<Option<AuthorizationV4<'a>>> {
    let Some(val) = hs.get(crate::header::names::AUTHORIZATION) else { return Ok(None) };
    match AuthorizationV4::parse(val) {
        Ok(x) => Ok(Some(x)),
        Err(e) => Err(invalid_request!(e, "invalid header: authorization")),
    }
}

fn extract_amz_date(hs: &'_ OrderedHeaders<'_>) -> S3Result<Option<AmzDate>> {
    let Some(val) = hs.get(crate::header::names::X_AMZ_DATE) else { return Ok(None) };
    match AmzDate::parse(val) {
        Ok(x) => Ok(Some(x)),
        Err(e) => Err(invalid_request!(e, "invalid header: x-amz-date")),
    }
}

fn extract_decoded_content_length(hs: &'_ OrderedHeaders<'_>) -> S3Result<Option<usize>> {
    let Some(val) = hs.get(crate::header::names::X_AMZ_DECODED_CONTENT_LENGTH) else { return Ok(None) };
    match atoi::atoi::<usize>(val.as_bytes()) {
        Some(x) => Ok(Some(x)),
        None => Err(invalid_request!("invalid header: x-amz-decoded-content-length")),
    }
}

async fn extract_full_body(req: &Request, body: &mut Body) -> S3Result<Bytes> {
    if let Some(bytes) = body.bytes() {
        return Ok(bytes);
    }

    let content_length = req
        .headers()
        .get(hyper::header::CONTENT_LENGTH)
        .and_then(|val| atoi::atoi::<u64>(val.as_bytes()));

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

pub async fn call(req: &mut Request, s3: &dyn S3, auth: Option<&dyn S3Auth>, base_domain: Option<&str>) -> S3Result<Response> {
    match call_inner(req, s3, auth, base_domain).await {
        Ok(res) => Ok(res),
        Err(err) => serialize_error(err),
    }
}

async fn call_inner(req: &mut Request, s3: &dyn S3, auth: Option<&dyn S3Auth>, base_domain: Option<&str>) -> S3Result<Response> {
    let s3_path = extract_s3_path(req, base_domain)?;
    let qs = extract_qs(req)?;

    // check signature
    let mut body = mem::take(req.body_mut());
    let multipart: Option<Multipart>;
    {
        let headers = extract_headers(req)?;
        let mime = extract_mime(&headers)?;
        let decoded_content_length = extract_decoded_content_length(&headers)?;
        let body_transformed;
        {
            let mut scx = SignatureContext {
                auth,
                req,
                qs: qs.as_ref(),
                headers,
                mime,
                body: &mut body,
                multipart: None,
                body_transformed: false,
                decoded_content_length,
            };

            scx.check().await?;

            multipart = scx.multipart;
            body_transformed = scx.body_transformed;
        }
        if body_transformed {
            if let Some(val) = req.headers_mut().get_mut(header::names::CONTENT_LENGTH) {
                let len = decoded_content_length.unwrap_or(0);
                if len > 0 {
                    *val = crate::utils::fmt_usize(len, |s| http::HeaderValue::try_from(s).unwrap())
                } else {
                    *val = http::HeaderValue::from_static("0");
                }
            }
        }
        let has_multipart = multipart.is_some();
        debug!(?body_transformed, ?decoded_content_length, ?has_multipart);
    }

    let (op, needs_full_body) = 'resolve: {
        if let Some(multipart) = multipart {
            if req.method() == Method::POST {
                match s3_path {
                    S3Path::Root => return Err(unknown_operation()),
                    S3Path::Bucket { .. } => {
                        // POST object
                        req.extensions_mut().insert(multipart);
                        break 'resolve (&PutObject as &'static dyn Operation, false);
                    }
                    S3Path::Object { .. } => return Err(s3_error!(MethodNotAllowed)),
                }
            }
        }
        resolve_route(req, &s3_path, qs.as_ref())?
    };

    debug!(op = %op.name(), "resolved route");

    req.extensions_mut().insert(s3_path);
    if let Some(qs) = qs {
        req.extensions_mut().insert(qs);
    }

    if needs_full_body {
        extract_full_body(req, &mut body).await?;
    }
    *req.body_mut() = body;

    op.call(s3, req).await
}

struct SignatureContext<'a> {
    auth: Option<&'a dyn S3Auth>,
    req: &'a Request,
    qs: Option<&'a OrderedQs>,
    headers: OrderedHeaders<'a>,
    mime: Option<Mime>,
    body: &'a mut Body,
    multipart: Option<Multipart>,
    body_transformed: bool,
    decoded_content_length: Option<usize>,
}

impl SignatureContext<'_> {
    async fn check(&mut self) -> S3Result<()> {
        // POST auth
        if self.req.method() == Method::POST {
            if let Some(ref mime) = self.mime {
                if mime.type_() == mime::MULTIPART && mime.subtype() == mime::FORM_DATA {
                    debug!("checking post signature");
                    return self.check_post_signature().await;
                }
            }
        }

        // query auth
        if let Some(qs) = self.qs {
            if qs.has("X-Amz-Signature") {
                debug!("checking presigned url");
                return self.check_presigned_url().await;
            }
        }

        // header auth
        debug!("checking header auth");
        self.check_header_auth().await
    }

    async fn check_post_signature(&mut self) -> S3Result<()> {
        let auth = require_auth(self.auth)?;

        let mime = self.mime.as_ref().unwrap(); // assume: multipart

        let boundary = mime
            .get_param(mime::BOUNDARY)
            .ok_or_else(|| invalid_request!("missing boundary"))?;

        let body = mem::take(self.body);
        let multipart = http::transform_multipart(body, boundary.as_str().as_bytes())
            .await
            .map_err(|e| s3_error!(e, MalformedPOSTRequest))?;
        self.body_transformed = true;

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

        let secret_key = auth.get_secret_key(credential.access_key_id).await?;

        let string_to_sign = info.policy;
        let signature = signature_v4::calculate_signature(string_to_sign, &secret_key, &amz_date, credential.aws_region);

        if signature != info.x_amz_signature {
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        self.multipart = Some(multipart);

        Ok(())
    }

    async fn check_presigned_url(&mut self) -> S3Result<()> {
        let qs = self.qs.unwrap(); // assume: qs has "X-Amz-Signature"

        let presigned_url = PresignedUrl::parse(qs).map_err(|err| invalid_request!(err, "missing presigned fields"))?;

        // ASK: how to use it?
        let _content_sha256: Option<AmzContentSha256<'_>> = extract_amz_content_sha256(&self.headers)?;

        let auth = require_auth(self.auth)?;

        let secret_key = auth.get_secret_key(presigned_url.credential.access_key_id).await?;

        let signature = {
            let headers = self.headers.find_multiple(&presigned_url.signed_headers);
            let method = self.req.method();
            let uri_path = self.req.uri().path();

            let canonical_request = signature_v4::create_presigned_canonical_request(method, uri_path, qs.as_ref(), &headers);

            let region = presigned_url.credential.aws_region;
            let amz_date = &presigned_url.amz_date;
            let string_to_sign = signature_v4::create_string_to_sign(&canonical_request, amz_date, region);

            signature_v4::calculate_signature(&string_to_sign, &secret_key, amz_date, region)
        };

        if signature != presigned_url.signature {
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn check_header_auth(&mut self) -> S3Result<()> {
        let authorization: AuthorizationV4<'_> = match extract_authorization_v4(&self.headers)? {
            Some(mut a) => {
                a.signed_headers.sort_unstable();
                a
            }
            None => {
                if self.auth.is_some() {
                    return Err(s3_error!(AccessDenied, "Access Denied"));
                }
                return Ok(());
            }
        };

        let auth = require_auth(self.auth)?;

        let amz_content_sha256 =
            extract_amz_content_sha256(&self.headers)?.ok_or_else(|| invalid_request!("missing header: x-amz-content-sha256"))?;

        let secret_key = auth.get_secret_key(authorization.credential.access_key_id).await?;

        let amz_date = extract_amz_date(&self.headers)?.ok_or_else(|| invalid_request!("missing header: x-amz-date"))?;

        let is_stream = matches!(amz_content_sha256, AmzContentSha256::MultipleChunks);

        let signature = {
            let method = self.req.method();
            let uri_path = self.req.uri().path();
            let query_strings: &[(String, String)] = self.qs.as_ref().map_or(&[], AsRef::as_ref);

            // here requires that `auth.signed_headers` is sorted
            let headers = self.headers.find_multiple(&authorization.signed_headers);

            let canonical_request = if is_stream {
                let payload = signature_v4::Payload::MultipleChunks;
                signature_v4::create_canonical_request(method, uri_path, query_strings, &headers, payload)
            } else if matches!(*self.req.method(), Method::GET | Method::HEAD) {
                let payload = signature_v4::Payload::Empty;
                signature_v4::create_canonical_request(method, uri_path, query_strings, &headers, payload)
            } else {
                let bytes = extract_full_body(self.req, self.body).await?;

                debug!(len=?bytes.len(), "extracted full body");

                let payload = if matches!(amz_content_sha256, AmzContentSha256::UnsignedPayload) {
                    signature_v4::Payload::Unsigned
                } else if bytes.is_empty() {
                    signature_v4::Payload::Empty
                } else {
                    signature_v4::Payload::SingleChunk(&bytes)
                };

                signature_v4::create_canonical_request(method, uri_path, query_strings, &headers, payload)
            };

            let region = authorization.credential.aws_region;
            let string_to_sign = signature_v4::create_string_to_sign(&canonical_request, &amz_date, region);
            signature_v4::calculate_signature(&string_to_sign, &secret_key, &amz_date, region)
        };

        if signature != authorization.signature {
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        if is_stream {
            let decoded_content_length = self
                .decoded_content_length
                .ok_or_else(|| s3_error!(MissingContentLength, "missing header: x-amz-decoded-content-length"))?;

            let stream = AwsChunkedStream::new(
                mem::take(self.body),
                signature.into(),
                amz_date,
                authorization.credential.aws_region.into(),
                secret_key.into(),
                decoded_content_length,
            );

            debug!(len=?stream.remaining_length(), "aws-chunked");

            *self.body = Body::from(wrap(stream));
            self.body_transformed = true;
        }

        Ok(())
    }
}

struct PostSignatureInfo<'a> {
    policy: &'a str,
    x_amz_algorithm: &'a str,
    x_amz_credential: &'a str,
    x_amz_date: &'a str,
    x_amz_signature: &'a str,
}

impl<'a> PostSignatureInfo<'a> {
    fn extract(m: &'a Multipart) -> Option<Self> {
        let policy = m.find_field_value("policy")?;
        let x_amz_algorithm = m.find_field_value("x-amz-algorithm")?;
        let x_amz_credential = m.find_field_value("x-amz-credential")?;
        let x_amz_date = m.find_field_value("x-amz-date")?;
        let x_amz_signature = m.find_field_value("x-amz-signature")?;
        Some(Self {
            policy,
            x_amz_algorithm,
            x_amz_credential,
            x_amz_date,
            x_amz_signature,
        })
    }
}

fn require_auth(auth: Option<&dyn S3Auth>) -> S3Result<&dyn S3Auth> {
    auth.ok_or_else(|| s3_error!(NotImplemented, "This service has no authentication provider"))
}
