use crate::HttpResponse;

use super::Body;

use hyper::HeaderMap;
use hyper::StatusCode;
use hyper::http::Extensions;
use hyper::http::HeaderValue;

#[derive(Default)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap<HeaderValue>,
    pub body: Body,
    pub extensions: Extensions,
}

impl From<Response> for HttpResponse {
    fn from(res: Response) -> Self {
        let mut ans = HttpResponse::default();
        *ans.status_mut() = res.status;
        *ans.headers_mut() = res.headers;
        *ans.body_mut() = res.body;
        *ans.extensions_mut() = res.extensions;
        ans
    }
}

impl Response {
    #[must_use]
    pub fn with_status(status: StatusCode) -> Self {
        Self {
            status,
            ..Default::default()
        }
    }
}
