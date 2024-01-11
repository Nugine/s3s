use s3s::header::{X_AMZ_ID_2, X_AMZ_REQUEST_ID};
use s3s::{s3_error, S3Result};

use aws_sdk_s3::operation::{RequestId, RequestIdExt};
use hyper::header::HeaderValue;
use hyper::HeaderMap;

pub fn build_headers<T>(output: &T) -> S3Result<HeaderMap<HeaderValue>>
where
    T: RequestId + RequestIdExt,
{
    let mut header = HeaderMap::new();
    if let Some(id) = output.request_id() {
        let val = HeaderValue::from_str(id).map_err(|_| s3_error!(InternalError, "invalid request id"))?;
        header.insert(X_AMZ_REQUEST_ID, val);
    }
    if let Some(id) = output.extended_request_id() {
        let val = HeaderValue::from_str(id).map_err(|_| s3_error!(InternalError, "invalid extended request id"))?;
        header.insert(X_AMZ_ID_2, val);
    }
    Ok(header)
}
