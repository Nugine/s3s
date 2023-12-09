use s3s::{s3_error, S3Result};

use aws_sdk_s3::operation::{RequestId, RequestIdExt};
use hyper::header::HeaderValue;

pub fn request_id(x: &impl RequestId) -> S3Result<Option<HeaderValue>> {
    let Some(id) = x.request_id() else { return Ok(None) };
    let val = HeaderValue::from_str(id).map_err(|_| s3_error!(InternalError, "invalid request id"))?;
    Ok(Some(val))
}

pub fn extended_request_id(x: &impl RequestIdExt) -> S3Result<Option<HeaderValue>> {
    let Some(id) = x.extended_request_id() else { return Ok(None) };
    let val = HeaderValue::from_str(id).map_err(|_| s3_error!(InternalError, "invalid extended request id"))?;
    Ok(Some(val))
}
