use crate::auth::S3Auth;
use crate::error::{S3Error, S3Result};
use crate::http::{Body, Request};
use crate::s3_trait::S3;

use std::convert::Infallible;
use std::fmt;
use std::future::{ready, Ready};
use std::sync::Arc;

use futures::future::BoxFuture;
use hyper::service::Service;
use tracing::{debug, error};

pub struct S3ServiceBuilder {
    s3: Arc<dyn S3>,
    auth: Option<Box<dyn S3Auth>>,
    base_domain: Option<String>,
}

impl S3ServiceBuilder {
    #[must_use]
    pub fn new(s3: impl S3) -> Self {
        Self {
            s3: Arc::new(s3),
            auth: None,
            base_domain: None,
        }
    }

    pub fn set_auth(&mut self, auth: impl S3Auth) {
        self.auth = Some(Box::new(auth));
    }

    pub fn set_base_domain(&mut self, base_domain: impl Into<String>) {
        self.base_domain = Some(base_domain.into());
    }

    #[must_use]
    pub fn build(self) -> S3Service {
        S3Service {
            s3: self.s3,
            auth: self.auth,
            base_domain: self.base_domain,
        }
    }
}

pub struct S3Service {
    s3: Arc<dyn S3>,
    auth: Option<Box<dyn S3Auth>>,
    base_domain: Option<String>,
}

impl S3Service {
    #[tracing::instrument(
        level = "debug",
        skip(self, req),
        fields(start_time=?time::OffsetDateTime::now_utc())
    )]
    pub async fn call(&self, req: hyper::Request<Body>) -> S3Result<hyper::Response<Body>> {
        debug!(?req);

        let mut req = Request::from(req);

        let s3 = &self.s3;
        let auth = self.auth.as_deref();
        let base_domain = self.base_domain.as_deref();
        let result = crate::ops::call(&mut req, s3, auth, base_domain).await.map(Into::into);

        match result {
            Ok(ref res) => debug!(?res),
            Err(ref err) => error!(?err),
        }

        result
    }

    #[must_use]
    pub fn into_shared(self) -> SharedS3Service {
        SharedS3Service(Arc::new(self))
    }

    async fn call_shared(self: Arc<Self>, req: hyper::Request<Body>) -> S3Result<hyper::Response<Body>> {
        self.call(req).await
    }
}

impl fmt::Debug for S3Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("S3Service").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub struct SharedS3Service(Arc<S3Service>);

impl SharedS3Service {
    #[must_use]
    pub fn into_make_service(self) -> MakeService<Self> {
        MakeService(self)
    }
}

impl AsRef<S3Service> for SharedS3Service {
    fn as_ref(&self) -> &S3Service {
        &self.0
    }
}

// TODO(blocking): GAT?
// See https://github.com/tower-rs/tower/issues/636
impl Service<hyper::Request<hyper::body::Incoming>> for SharedS3Service {
    type Response = hyper::Response<Body>;

    type Error = S3Error;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: hyper::Request<hyper::body::Incoming>) -> Self::Future {
        let req = req.map(Body::from);
        let service = self.0.clone();
        Box::pin(service.call_shared(req))
    }
}

#[derive(Clone)]
pub struct MakeService<S>(S);

impl<T, S: Clone> Service<T> for MakeService<S> {
    type Response = S;

    type Error = Infallible;

    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, _: T) -> Self::Future {
        ready(Ok(self.0.clone()))
    }
}
