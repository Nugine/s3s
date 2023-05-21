use crate::auth::Credentials;
use crate::utils::default;

use hyper::http::{Extensions, HeaderValue};
use hyper::{HeaderMap, Uri};

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
    ///
    /// It is used to pass custom data between middlewares.
    pub extensions: Extensions,

    // Headers
    pub headers: HeaderMap<HeaderValue>,

    // Raw URI
    pub uri: Uri,
}

impl<T> S3Request<T> {
    pub fn new(input: T) -> Self {
        Self {
            input,
            credentials: default(),
            extensions: default(),
            headers: default(),
            uri: default(),
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
