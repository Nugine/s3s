use crate::http;

use hyper::header::InvalidHeaderValue;

pub type ContentType = mime::Mime;

#[derive(Debug, thiserror::Error)]
pub enum ParseContentTypeError {
    #[error("Expected UTF-8")]
    ExpectedUtf8,
    #[error("Mime: {0}")]
    Mime(mime::FromStrError),
}

impl http::TryFromHeaderValue for ContentType {
    type Error = ParseContentTypeError;

    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| ParseContentTypeError::ExpectedUtf8)?;
        val.parse().map_err(ParseContentTypeError::Mime)
    }
}

impl http::TryIntoHeaderValue for ContentType {
    type Error = InvalidHeaderValue;

    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        http::HeaderValue::from_str(self.as_ref())
    }
}
