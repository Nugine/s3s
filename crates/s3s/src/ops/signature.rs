use crate::auth::Credentials;
use crate::auth::S3Auth;
use crate::error::*;
use crate::http;
use crate::http::{AwsChunkedStream, Body, Multipart};
use crate::http::{OrderedHeaders, OrderedQs};
use crate::path::S3Path;
use crate::sig_v2;
use crate::sig_v2::{AuthorizationV2, PresignedUrlV2};
use crate::sig_v4;
use crate::sig_v4::PostSignatureInfo;
use crate::sig_v4::PresignedUrlV4;
use crate::sig_v4::{AmzContentSha256, AmzDate};
use crate::sig_v4::{AuthorizationV4, CredentialV4};
use crate::utils::is_base64_encoded;

use std::mem;
use std::ops::Not;

use bytestring::ByteString;
use hyper::Method;
use hyper::Uri;
use mime::Mime;
use tracing::debug;

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

pub struct SignatureContext<'a> {
    pub auth: Option<&'a dyn S3Auth>,
    pub base_domain: Option<&'a str>,

    pub req_method: &'a Method,
    pub req_uri: &'a Uri,
    pub req_body: &'a mut Body,

    pub qs: Option<&'a OrderedQs>,
    pub hs: OrderedHeaders<'a>,

    pub decoded_uri_path: String,
    pub s3_path: &'a S3Path,

    pub host: Option<&'a str>,
    pub content_length: Option<u64>,
    pub mime: Option<Mime>,
    pub decoded_content_length: Option<usize>,

    pub transformed_body: Option<Body>,
    pub multipart: Option<Multipart>,
}

fn require_auth(auth: Option<&dyn S3Auth>) -> S3Result<&dyn S3Auth> {
    auth.ok_or_else(|| s3_error!(NotImplemented, "This service has no authentication provider"))
}

impl SignatureContext<'_> {
    pub async fn check(&mut self) -> S3Result<Option<Credentials>> {
        if let Some(result) = self.v2_check().await {
            debug!("checked signature v2");
            return Ok(Some(result?));
        }

        if let Some(result) = self.v4_check().await {
            debug!("checked signature v4");
            return Ok(Some(result?));
        }

        Ok(None)
    }

    #[tracing::instrument(skip(self))]
    pub async fn v4_check(&mut self) -> Option<S3Result<Credentials>> {
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

    pub async fn v4_check_post_signature(&mut self) -> S3Result<Credentials> {
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

        let expected_signature = info.x_amz_signature;
        if signature != expected_signature {
            debug!(?signature, expected=?expected_signature, "signature mismatch");
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        self.multipart = Some(multipart);
        Ok(Credentials { access_key, secret_key })
    }

    pub async fn v4_check_presigned_url(&mut self) -> S3Result<Credentials> {
        let qs = self.qs.unwrap(); // assume: qs has "X-Amz-Signature"

        let presigned_url = PresignedUrlV4::parse(qs).map_err(|err| invalid_request!(err, "missing presigned url v4 fields"))?;

        if presigned_url.algorithm != "AWS4-HMAC-SHA256" {
            return Err(s3_error!(
                NotImplemented,
                "X-Amz-Algorithm other than AWS4-HMAC-SHA256 is not implemented"
            ));
        }

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

        let expected_signature = presigned_url.signature;
        if signature != expected_signature {
            debug!(?signature, expected=?expected_signature, "signature mismatch");
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        Ok(Credentials {
            access_key: access_key.into(),
            secret_key,
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn v4_check_header_auth(&mut self) -> S3Result<Credentials> {
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
                let payload = if matches!(amz_content_sha256, AmzContentSha256::UnsignedPayload) {
                    sig_v4::Payload::Unsigned
                } else {
                    sig_v4::Payload::Empty
                };
                sig_v4::create_canonical_request(method, uri_path, query_strings, &headers, payload)
            } else if matches!(amz_content_sha256, AmzContentSha256::UnsignedPayload) {
                sig_v4::create_canonical_request(method, uri_path, query_strings, &headers, sig_v4::Payload::Unsigned)
            } else {
                let bytes = super::extract_full_body(self.content_length, self.req_body).await?;
                if bytes.len() < 1024 {
                    debug!(len=?bytes.len(), body=?bytes, "extracted full body");
                } else {
                    debug!(len=?bytes.len(), "extracted full body");
                }

                if bytes.is_empty() {
                    sig_v4::create_canonical_request(method, uri_path, query_strings, &headers, sig_v4::Payload::Empty)
                } else {
                    sig_v4::create_canonical_request(
                        method,
                        uri_path,
                        query_strings,
                        &headers,
                        sig_v4::Payload::SingleChunk(&bytes),
                    )
                }
            };

            let region = authorization.credential.aws_region;
            let string_to_sign = sig_v4::create_string_to_sign(&canonical_request, &amz_date, region);
            sig_v4::calculate_signature(&string_to_sign, &secret_key, &amz_date, region)
        };

        let expected_signature = authorization.signature;
        if signature != expected_signature {
            debug!(?signature, expected=?expected_signature, "signature mismatch");
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
    pub async fn v2_check(&mut self) -> Option<S3Result<Credentials>> {
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

    fn v2_virtual_hosted_bucket(&self) -> Option<&str> {
        match (self.base_domain, self.host) {
            (Some(base_domain), Some(host)) if base_domain != host => self.s3_path.get_bucket_name(),
            _ => None,
        }
    }

    pub async fn v2_check_header_auth(&mut self, auth_v2: AuthorizationV2<'_>) -> S3Result<Credentials> {
        let method = &self.req_method;

        let date = self.hs.get_unique("date").or_else(|| self.hs.get_unique("x-amz-date"));
        if date.is_none() {
            return Err(invalid_request!("missing date"));
        }

        let auth = require_auth(self.auth)?;
        let access_key = auth_v2.access_key;
        let secret_key = auth.get_secret_key(access_key).await?;

        let vh_bucket = self.v2_virtual_hosted_bucket();

        let string_to_sign =
            sig_v2::create_string_to_sign(sig_v2::Mode::HeaderAuth, method, self.req_uri.path(), self.qs, &self.hs, vh_bucket);
        let signature = sig_v2::calculate_signature(&secret_key, &string_to_sign);

        debug!(?string_to_sign, "sig_v2 header_auth");

        let expected_signature = auth_v2.signature;
        if signature != expected_signature {
            debug!(?signature, expected=?expected_signature, "signature mismatch");
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        Ok(Credentials {
            access_key: access_key.into(),
            secret_key,
        })
    }

    pub async fn v2_check_presigned_url(&mut self) -> S3Result<Credentials> {
        let qs = self.qs.unwrap(); // assume: qs has "Signature"
        let presigned_url = PresignedUrlV2::parse(qs).map_err(|err| invalid_request!(err, "missing presigned url v2 fields"))?;

        if time::OffsetDateTime::now_utc() > presigned_url.expires_time {
            return Err(s3_error!(AccessDenied, "Request has expired"));
        }

        let auth = require_auth(self.auth)?;
        let access_key = presigned_url.access_key;
        let secret_key = auth.get_secret_key(access_key).await?;

        let vh_bucket = self.v2_virtual_hosted_bucket();

        let string_to_sign = sig_v2::create_string_to_sign(
            sig_v2::Mode::PresignedUrl,
            self.req_method,
            self.req_uri.path(),
            self.qs,
            &self.hs,
            vh_bucket,
        );
        let signature = sig_v2::calculate_signature(&secret_key, &string_to_sign);

        let expected_signature = presigned_url.signature;
        if signature != expected_signature {
            debug!(?signature, expected=?expected_signature, "signature mismatch");
            return Err(s3_error!(SignatureDoesNotMatch));
        }

        Ok(Credentials {
            access_key: access_key.into(),
            secret_key,
        })
    }
}
