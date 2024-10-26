use crate::Body;
use crate::S3Request;
use crate::S3Response;
use crate::S3Result;

use hyper::http::Extensions;
use hyper::HeaderMap;
use hyper::Method;
use hyper::StatusCode;
use hyper::Uri;

#[async_trait::async_trait]
pub trait S3Route: Send + Sync + 'static {
    fn is_match(&self, method: &Method, uri: &Uri, headers: &HeaderMap, extensions: &mut Extensions) -> bool;

    async fn call(&self, req: S3Request<Body>) -> S3Result<S3Response<(StatusCode, Body)>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::header;

    pub struct AssumeRole {}

    #[async_trait::async_trait]
    impl S3Route for AssumeRole {
        fn is_match(&self, method: &Method, uri: &Uri, headers: &HeaderMap, _: &mut Extensions) -> bool {
            if method == Method::POST && uri.path() == "/" {
                if let Some(val) = headers.get(header::CONTENT_TYPE) {
                    if val.as_bytes() == b"application/x-www-form-urlencoded" {
                        return true;
                    }
                }
            }
            false
        }

        async fn call(&self, _: S3Request<Body>) -> S3Result<S3Response<(StatusCode, Body)>> {
            tracing::debug!("call AssumeRole");
            return Err(s3_error!(NotImplemented));
        }
    }
}
