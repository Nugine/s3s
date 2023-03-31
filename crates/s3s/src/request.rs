use hyper::{
    http::{Extensions, HeaderValue},
    HeaderMap, Uri,
};

use crate::auth::Credentials;

#[derive(Debug)]
#[non_exhaustive]
pub struct S3Request<T> {
    /// Operation input
    pub input: T,

    /// Identity information.
    ///
    /// `None` means anonymous request.
    pub credentials: Option<Credentials>,

    /// Request extensions
    pub extensions: Extensions,

    // Headers
    pub headers: HeaderMap<HeaderValue>,

    // Raw URI
    pub uri: Uri,
}

impl<T> S3Request<T> {
    pub fn from_input(input: T) -> Self {
        Self {
            input,
            credentials: Default::default(),
            extensions: Default::default(),
            headers: Default::default(),
            uri: Default::default(),
        }
    }

    pub fn map_input<U>(self, f: impl FnOnce(T) -> U) -> S3Request<U> {
        S3Request {
            input: f(self.input),
            credentials: self.credentials,
            extensions: self.extensions,
            headers: self.headers,
            uri: self.uri,
        }
    }
}
