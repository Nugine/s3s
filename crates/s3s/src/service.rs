use crate::access::S3Access;
use crate::auth::S3Auth;
use crate::host::S3Host;
use crate::http::{Body, Request};
use crate::route::S3Route;
use crate::s3_trait::S3;
use crate::{HttpError, HttpRequest, HttpResponse};

use std::fmt;
use std::sync::Arc;

use futures::future::BoxFuture;
use tracing::{debug, error};

pub struct S3ServiceBuilder {
    s3: Arc<dyn S3>,
    host: Option<Box<dyn S3Host>>,
    auth: Option<Box<dyn S3Auth>>,
    access: Option<Box<dyn S3Access>>,
    route: Option<Box<dyn S3Route>>,
}

impl S3ServiceBuilder {
    #[must_use]
    pub fn new(s3: impl S3) -> Self {
        Self {
            s3: Arc::new(s3),
            host: None,
            auth: None,
            access: None,
            route: None,
        }
    }

    pub fn set_host(&mut self, host: impl S3Host) {
        self.host = Some(Box::new(host));
    }

    pub fn set_auth(&mut self, auth: impl S3Auth) {
        self.auth = Some(Box::new(auth));
    }

    pub fn set_access(&mut self, access: impl S3Access) {
        self.access = Some(Box::new(access));
    }

    pub fn set_route(&mut self, route: impl S3Route) {
        self.route = Some(Box::new(route));
    }

    #[must_use]
    pub fn build(self) -> S3Service {
        S3Service {
            inner: Arc::new(Inner {
                s3: self.s3,
                host: self.host,
                auth: self.auth,
                access: self.access,
                route: self.route,
            }),
        }
    }
}

#[derive(Clone)]
pub struct S3Service {
    inner: Arc<Inner>,
}

struct Inner {
    s3: Arc<dyn S3>,
    host: Option<Box<dyn S3Host>>,
    auth: Option<Box<dyn S3Auth>>,
    access: Option<Box<dyn S3Access>>,
    route: Option<Box<dyn S3Route>>,
}

impl S3Service {
    #[tracing::instrument(
        level = "debug",
        skip(self, req),
        fields(start_time=?time::OffsetDateTime::now_utc())
    )]
    pub async fn call(&self, req: HttpRequest) -> Result<HttpResponse, HttpError> {
        debug!(?req);

        let t0 = std::time::Instant::now();

        let mut req = Request::from(req);

        let ccx = crate::ops::CallContext {
            s3: &self.inner.s3,
            host: self.inner.host.as_deref(),
            auth: self.inner.auth.as_deref(),
            access: self.inner.access.as_deref(),
            route: self.inner.route.as_deref(),
        };
        let result = match crate::ops::call(&mut req, &ccx).await {
            Ok(res) => Ok(HttpResponse::from(res)),
            Err(err) => Err(HttpError::new(Box::new(err))),
        };

        let duration = t0.elapsed();

        match result {
            Ok(ref res) => {
                if res.status().is_server_error() {
                    error!(?duration, ?res);
                } else {
                    debug!(?duration, ?res);
                }
            }
            Err(ref err) => error!(?duration, ?err),
        }

        result
    }

    async fn call_owned(self, req: HttpRequest) -> Result<HttpResponse, HttpError> {
        self.call(req).await
    }
}

impl fmt::Debug for S3Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("S3Service").finish_non_exhaustive()
    }
}

impl hyper::service::Service<http::Request<hyper::body::Incoming>> for S3Service {
    type Response = HttpResponse;

    type Error = HttpError;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: http::Request<hyper::body::Incoming>) -> Self::Future {
        let req = req.map(Body::from);
        let service = self.clone();
        Box::pin(service.call_owned(req))
    }
}

impl tower::Service<http::Request<hyper::body::Incoming>> for S3Service {
    type Response = HttpResponse;

    type Error = HttpError;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<hyper::body::Incoming>) -> Self::Future {
        let req = req.map(Body::from);
        let service = self.clone();
        Box::pin(service.call_owned(req))
    }
}

#[cfg(test)]
#[allow(clippy::panic, clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
mod tests {
    use super::*;

    use crate::{S3Error, S3Request, S3Response};

    use stdx::mem::output_size;

    macro_rules! print_future_size {
        ($func:path) => {
            println!("{:<24}: {}", stringify!($func), output_size(&$func));
        };
    }

    macro_rules! print_type_size {
        ($ty:path) => {
            println!("{:<24}: {}", stringify!($ty), std::mem::size_of::<$ty>());
        };
    }

    #[test]
    fn future_size() {
        print_type_size!(std::time::Instant);

        print_type_size!(HttpRequest);
        print_type_size!(HttpResponse);
        print_type_size!(HttpError);

        print_type_size!(S3Request<()>);
        print_type_size!(S3Response<()>);
        print_type_size!(S3Error);

        print_type_size!(S3Service);

        print_future_size!(crate::ops::call);
        print_future_size!(S3Service::call);
        print_future_size!(S3Service::call_owned);

        // In case the futures are made too large accidentally
        assert!(output_size(&crate::ops::call) <= 1600);
        assert!(output_size(&S3Service::call) <= 2900);
        assert!(output_size(&S3Service::call_owned) <= 3200);
    }
}
