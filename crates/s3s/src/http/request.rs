use super::Body;

use hyper::http::Extensions;
use hyper::http::HeaderValue;
use hyper::HeaderMap;
use hyper::Method;
use hyper::Uri;

pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub headers: HeaderMap<HeaderValue>,
    pub extensions: Extensions,
    pub body: Body,
}

impl From<hyper::Request<Body>> for Request {
    fn from(req: hyper::Request<Body>) -> Self {
        let (parts, body) = req.into_parts();
        Self {
            method: parts.method,
            uri: parts.uri,
            headers: parts.headers,
            extensions: parts.extensions,
            body,
        }
    }
}
