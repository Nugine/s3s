use s3s::service::SharedS3Service;
use s3s::{S3Error, S3Result};

use std::ops::Not;
use std::task::{Context, Poll};

use aws_smithy_http::body::SdkBody;
use aws_smithy_http::byte_stream::ByteStream;
use aws_smithy_http::result::ConnectorError;

use hyper::header::HOST;
use hyper::http;
use hyper::service::Service;
use hyper::{Request, Response};

use futures::future::BoxFuture;

#[derive(Clone)]
pub struct Connector(SharedS3Service);

impl From<SharedS3Service> for Connector {
    fn from(val: SharedS3Service) -> Self {
        Self(val)
    }
}

fn on_err(e: S3Error) -> ConnectorError {
    let kind = aws_smithy_types::retry::ErrorKind::ServerError;
    ConnectorError::other(Box::new(e), Some(kind))
}

// From <https://github.com/awslabs/aws-sdk-rust/discussions/253#discussioncomment-1478233>
impl Service<Request<SdkBody>> for Connector {
    type Response = Response<SdkBody>;

    type Error = ConnectorError;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx).map_err(on_err)
    }

    fn call(&mut self, req: Request<SdkBody>) -> Self::Future {
        let req = convert_input(req);
        let service = self.0.clone();
        Box::pin(async move { convert_output(service.as_ref().call(req).await) })
    }
}

fn convert_input(mut req: Request<SdkBody>) -> Request<s3s::Body> {
    if req.headers().contains_key(HOST).not() {
        let host = auto_host_header(req.uri());
        req.headers_mut().insert(HOST, host);
    }

    req.map(|sdk_body| s3s::Body::from(hyper::Body::wrap_stream(ByteStream::from(sdk_body))))
}

fn convert_output(result: S3Result<Response<s3s::Body>>) -> Result<Response<SdkBody>, ConnectorError> {
    match result {
        Ok(res) => Ok(res.map(|s3s_body| SdkBody::from(hyper::Body::from(s3s_body)))),
        Err(e) => Err(on_err(e)),
    }
}

// From <https://docs.rs/hyper/0.14.23/src/hyper/client/client.rs.html#253-260>
fn auto_host_header(uri: &http::Uri) -> http::HeaderValue {
    let hostname = uri.host().expect("authority implies host");
    match get_non_default_port(uri) {
        Some(port) => http::HeaderValue::try_from(format!("{hostname}:{port}")),
        None => http::HeaderValue::from_str(hostname),
    }
    .expect("uri host is valid header value")
}

/// From <https://docs.rs/hyper/0.14.23/src/hyper/client/client.rs.html#860-872>
fn get_non_default_port(uri: &http::Uri) -> Option<http::uri::Port<&str>> {
    match (uri.port().map(|p| p.as_u16()), is_schema_secure(uri)) {
        (Some(443), true) => None,
        (Some(80), false) => None,
        _ => uri.port(),
    }
}

fn is_schema_secure(uri: &http::Uri) -> bool {
    uri.scheme_str()
        .is_some_and(|scheme_str| matches!(scheme_str, "wss" | "https"))
}
