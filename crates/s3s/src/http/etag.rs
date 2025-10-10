use super::de::TryFromHeaderValue;
use super::ser::TryIntoHeaderValue;

use crate::dto::ETag;
use crate::dto::ParseETagError;

use http::HeaderValue;
use http::header::InvalidHeaderValue;

impl TryFromHeaderValue for ETag {
    type Error = ParseETagError;

    fn try_from_header_value(value: &HeaderValue) -> Result<Self, Self::Error> {
        Self::parse_http_header(value.as_bytes())
    }
}

impl TryIntoHeaderValue for ETag {
    type Error = InvalidHeaderValue;

    fn try_into_header_value(self) -> Result<HeaderValue, Self::Error> {
        self.to_http_header()
    }
}
