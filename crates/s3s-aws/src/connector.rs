use crate::body::{s3s_body_into_sdk_body, sdk_body_into_s3s_body};

use s3s::service::SharedS3Service;
use s3s::S3Result;

use std::ops::Not;

use aws_sdk_s3::config::RuntimeComponents;
use aws_smithy_runtime_api::client::http::{HttpClient, HttpConnectorSettings, SharedHttpConnector};
use aws_smithy_runtime_api::client::http::{HttpConnector, HttpConnectorFuture};
use aws_smithy_runtime_api::client::orchestrator::HttpRequest as AwsHttpRequest;
use aws_smithy_runtime_api::client::orchestrator::HttpResponse as AwsHttpResponse;
use aws_smithy_runtime_api::client::result::ConnectorError;

use hyper::header::HOST;
use hyper::http;
use hyper::{Request, Response};

#[derive(Debug)]
pub struct Client(SharedS3Service);

impl HttpClient for Client {
    fn http_connector(&self, _: &HttpConnectorSettings, _: &RuntimeComponents) -> SharedHttpConnector {
        SharedHttpConnector::new(Connector(self.0.clone()))
    }
}

impl From<SharedS3Service> for Client {
    fn from(val: SharedS3Service) -> Self {
        Self(val)
    }
}

#[derive(Debug, Clone)]
pub struct Connector(SharedS3Service);

impl From<SharedS3Service> for Connector {
    fn from(val: SharedS3Service) -> Self {
        Self(val)
    }
}

fn on_err<E>(e: E) -> ConnectorError
where
    E: std::error::Error + Send + Sync + 'static,
{
    let kind = aws_smithy_runtime_api::client::retries::ErrorKind::ServerError;
    ConnectorError::other(Box::new(e), Some(kind))
}

impl HttpConnector for Connector {
    fn call(&self, req: AwsHttpRequest) -> HttpConnectorFuture {
        let service = self.0.clone();
        HttpConnectorFuture::new_boxed(Box::pin(async move { convert_output(service.as_ref().call(convert_input(req)?).await) }))
    }
}

fn convert_input(req: AwsHttpRequest) -> Result<Request<s3s::Body>, ConnectorError> {
    let mut req = req.try_into_http1x().map_err(on_err)?;

    if req.headers().contains_key(HOST).not() {
        let host = auto_host_header(req.uri());
        req.headers_mut().insert(HOST, host);
    }

    Ok(req.map(sdk_body_into_s3s_body))
}

fn convert_output(result: S3Result<Response<s3s::Body>>) -> Result<AwsHttpResponse, ConnectorError> {
    match result {
        Ok(res) => res.map(s3s_body_into_sdk_body).try_into().map_err(on_err),
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
