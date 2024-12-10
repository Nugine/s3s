use crate::auth::Credentials;

use hyper::http::{Extensions, HeaderValue};
use hyper::{HeaderMap, Method, Uri};
use stdx::default::default;

#[derive(Debug)]
#[non_exhaustive]
pub struct S3Request<T> {
    /// HTTP method
    pub method: Method,

    // Raw URI
    pub uri: Uri,

    // Headers
    pub headers: HeaderMap<HeaderValue>,

    /// Operation input
    pub input: T,

    /// Request extensions
    ///
    /// It is used to pass custom data between middlewares.
    pub extensions: Extensions,

    /// Identity information.
    ///
    /// `None` means anonymous request.
    pub credentials: Option<Credentials>,

    /// The requested region.
    pub region: Option<String>,

    /// The requested service.
    pub service: Option<String>,
}

impl<T> S3Request<T> {
    pub fn new(input: T) -> Self {
        Self {
            method: default(),
            uri: default(),
            headers: default(),
            input,
            extensions: default(),
            credentials: default(),
            region: default(),
            service: default(),
        }
    }

    pub fn map_input<U>(self, f: impl FnOnce(T) -> U) -> S3Request<U> {
        S3Request {
            method: self.method,
            uri: self.uri,
            headers: self.headers,
            input: f(self.input),
            extensions: self.extensions,
            credentials: self.credentials,
            region: self.region,
            service: self.service,
        }
    }
}
