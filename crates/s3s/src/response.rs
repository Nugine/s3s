use hyper::http::Extensions;
use hyper::http::HeaderValue;
use hyper::HeaderMap;

#[non_exhaustive]
pub struct S3Response<T> {
    /// Operation output
    pub output: T,

    /// Response headers, overrides the headers in `output`.
    pub headers: HeaderMap<HeaderValue>,

    /// Response extensions.
    ///
    /// It is used to pass custom data between middlewares.
    pub extensions: Extensions,
}

impl<T> S3Response<T> {
    pub fn with_headers(output: T, headers: HeaderMap<HeaderValue>) -> Self {
        Self {
            output,
            headers,
            extensions: Extensions::new(),
        }
    }

    pub fn new(output: T) -> Self {
        Self {
            output,
            headers: HeaderMap::new(),
            extensions: Extensions::new(),
        }
    }

    pub fn map_output<U>(self, f: impl FnOnce(T) -> U) -> S3Response<U> {
        S3Response {
            output: f(self.output),
            headers: self.headers,
            extensions: self.extensions,
        }
    }
}
