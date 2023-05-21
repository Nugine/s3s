use crate::dto::GetObjectInput;
use crate::dto::Timestamp;
use crate::dto::TimestampFormat;
use crate::header;
use crate::utils;
use crate::S3Request;
use crate::S3Result;

use hyper::http::HeaderName;
use hyper::http::HeaderValue;
use hyper::HeaderMap;

use rust_utils::default::default;

pub fn extract_overrided_response_headers(req: &S3Request<GetObjectInput>) -> S3Result<HeaderMap<HeaderValue>> {
    let mut map: HeaderMap<HeaderValue> = default();

    add(&mut map, header::CONTENT_TYPE, req.input.response_content_type.as_deref())?;
    add(&mut map, header::CONTENT_LANGUAGE, req.input.response_content_language.as_deref())?;
    add_ts(&mut map, header::EXPIRES, req.input.response_expires.as_ref())?;
    add(&mut map, header::CACHE_CONTROL, req.input.response_cache_control.as_deref())?;
    add(&mut map, header::CONTENT_DISPOSITION, req.input.response_content_disposition.as_deref())?;
    add(&mut map, header::CONTENT_ENCODING, req.input.response_content_encoding.as_deref())?;

    Ok(map)
}

fn add(map: &mut HeaderMap<HeaderValue>, name: HeaderName, value: Option<&str>) -> S3Result<()> {
    let error = |e| invalid_request!(e, "invalid overrided header: {name}: {value:?}");
    if let Some(value) = value {
        let value = value.parse().map_err(error)?;
        map.insert(name, value);
    }
    Ok(())
}

fn add_ts(map: &mut HeaderMap<HeaderValue>, name: HeaderName, value: Option<&Timestamp>) -> S3Result<()> {
    let error = |e| invalid_request!(e, "invalid overrided header: {name}: {value:?}");
    if let Some(value) = value {
        let value = utils::fmt_timestamp(value, TimestampFormat::HttpDate, HeaderValue::from_bytes).map_err(error)?;
        map.insert(name, value);
    }
    Ok(())
}
