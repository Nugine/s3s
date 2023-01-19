use s3s::S3Error;
use s3s::S3ErrorCode;

use std::panic::Location;

use tracing::error;

#[derive(Debug)]
pub struct Error {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

impl Error {
    #[must_use]
    #[track_caller]
    pub fn new(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        log(&*source);
        Self { source }
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    #[track_caller]
    fn from(source: E) -> Self {
        Self::new(Box::new(source))
    }
}

impl From<Error> for S3Error {
    fn from(e: Error) -> Self {
        S3Error::with_source(S3ErrorCode::InternalError, e.source)
    }
}

#[inline]
#[track_caller]
pub(crate) fn log(source: &dyn std::error::Error) {
    if cfg!(feature = "binary") {
        let location = Location::caller();
        let span_trace = tracing_error::SpanTrace::capture();

        error!(
            target: "s3s_fs_internal_error",
            %location,
            error=%source,
            "span trace:\n{span_trace}"
        );
    }
}

macro_rules! try_ {
    ($result:expr) => {
        match $result {
            Ok(val) => val,
            Err(err) => {
                $crate::error::log(&err);
                return Err(::s3s::S3Error::internal_error(err));
            }
        }
    };
}
