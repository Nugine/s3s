use super::Response;

use crate::dto::{Metadata, StreamingBlob, Timestamp, TimestampFormat};
use crate::error::{S3Error, S3Result};
use crate::http::{HeaderName, HeaderValue};
use crate::{utils, xml};

use std::convert::Infallible;
use std::fmt::Write as _;

use hyper::header::{IntoHeaderName, InvalidHeaderValue};

pub fn add_header<N, V>(res: &mut Response, name: N, value: V) -> S3Result
where
    N: IntoHeaderName,
    V: TryIntoHeaderValue,
    V::Error: std::error::Error + Send + Sync + 'static,
{
    let val = value.try_into_header_value().map_err(S3Error::internal_error)?;
    res.headers_mut().insert(name, val);
    Ok(())
}

pub fn add_opt_header<N, V>(res: &mut Response, name: N, value: Option<V>) -> S3Result
where
    N: IntoHeaderName,
    V: TryIntoHeaderValue,
    V::Error: std::error::Error + Send + Sync + 'static,
{
    if let Some(value) = value {
        let val = value.try_into_header_value().map_err(S3Error::internal_error)?;
        res.headers_mut().insert(name, val);
    }
    Ok(())
}

pub fn add_opt_header_timestamp<N>(res: &mut Response, name: N, value: Option<Timestamp>, fmt: TimestampFormat) -> S3Result
where
    N: IntoHeaderName,
{
    if let Some(ref value) = value {
        let val = utils::fmt_timestamp(value, fmt, HeaderValue::from_bytes).map_err(S3Error::internal_error)?;
        res.headers_mut().insert(name, val);
    }
    Ok(())
}

pub trait TryIntoHeaderValue {
    type Error;
    fn try_into_header_value(self) -> Result<HeaderValue, Self::Error>;
}

impl TryIntoHeaderValue for bool {
    type Error = Infallible;

    #[allow(clippy::declare_interior_mutable_const)]
    fn try_into_header_value(self) -> Result<HeaderValue, Self::Error> {
        const TRUE: HeaderValue = HeaderValue::from_static("true");
        const FALSE: HeaderValue = HeaderValue::from_static("false");
        Ok(if self { TRUE } else { FALSE })
    }
}

impl TryIntoHeaderValue for i32 {
    type Error = Infallible;

    fn try_into_header_value(self) -> Result<HeaderValue, Self::Error> {
        Ok(HeaderValue::from(self))
    }
}

impl TryIntoHeaderValue for i64 {
    type Error = Infallible;

    fn try_into_header_value(self) -> Result<HeaderValue, Self::Error> {
        Ok(HeaderValue::from(self))
    }
}

impl TryIntoHeaderValue for String {
    type Error = InvalidHeaderValue;

    fn try_into_header_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::try_from(self)
    }
}

pub fn set_xml_body<T: xml::Serialize>(res: &mut Response, val: &T) -> S3Result {
    let mut buf = Vec::with_capacity(256);
    {
        let mut ser = xml::Serializer::new(&mut buf);
        ser.decl()
            .and_then(|_| val.serialize(&mut ser))
            .map_err(S3Error::internal_error)?;
    }
    *res.body_mut() = hyper::Body::from(buf);
    res.headers_mut()
        .insert(hyper::header::CONTENT_TYPE, HeaderValue::from_static(mime::TEXT_XML.as_ref()));
    Ok(())
}

pub fn set_stream_body(res: &mut Response, stream: StreamingBlob) {
    *res.body_mut() = hyper::Body::wrap_stream(stream.0);
}

pub fn add_opt_metadata(res: &mut Response, metadata: Option<Metadata>) -> S3Result {
    if let Some(map) = metadata {
        let mut buf = String::new();
        for (key, val) in map {
            write!(&mut buf, "x-amz-meta-{key}").unwrap();
            let name = HeaderName::from_bytes(buf.as_bytes()).map_err(S3Error::internal_error)?;
            let value = HeaderValue::try_from(val).map_err(S3Error::internal_error)?;
            res.headers_mut().insert(name, value);
            buf.clear();
        }
    }
    Ok(())
}
