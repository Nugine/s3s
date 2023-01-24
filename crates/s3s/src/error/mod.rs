mod generated;
pub use self::generated::*;

use crate::xml;

use std::borrow::Cow;
use std::convert::Infallible;
use std::fmt;
use std::io::Write;
use std::str::FromStr;

use hyper::StatusCode;

pub type S3Result<T = (), E = S3Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub struct S3Error(Box<Inner>);

#[derive(Debug)]
struct Inner {
    code: S3ErrorCode,
    message: Option<Cow<'static, str>>,
    // resource: Option<String>,
    request_id: Option<String>,
    status_code: Option<StatusCode>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl S3Error {
    #[must_use]
    pub fn new(code: S3ErrorCode) -> Self {
        Self(Box::new(Inner {
            code,
            message: None,
            // resource: None,
            request_id: None,
            status_code: None,
            source: None,
        }))
    }

    #[must_use]
    pub fn with_message(code: S3ErrorCode, msg: impl Into<Cow<'static, str>>) -> Self {
        let mut this = Self::new(code);
        this.0.message = Some(msg.into());
        this
    }

    #[must_use]
    pub fn with_source(code: S3ErrorCode, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        let mut this = Self::new(code);
        this.0.source = Some(source);
        this
    }

    pub fn set_code(&mut self, val: S3ErrorCode) {
        self.0.code = val;
    }

    pub fn set_message(&mut self, val: impl Into<Cow<'static, str>>) {
        self.0.message = Some(val.into());
    }

    pub fn set_request_id(&mut self, val: impl Into<String>) {
        self.0.request_id = Some(val.into());
    }

    pub fn set_source(&mut self, val: Box<dyn std::error::Error + Send + Sync + 'static>) {
        self.0.source = Some(val);
    }

    pub fn set_status_code(&mut self, val: StatusCode) {
        self.0.status_code = Some(val);
    }

    #[must_use]
    pub fn code(&self) -> &S3ErrorCode {
        &self.0.code
    }

    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.0.message.as_deref()
    }

    #[must_use]
    pub fn request_id(&self) -> Option<&str> {
        self.0.request_id.as_deref()
    }

    #[must_use]
    pub fn source(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        self.0.source.as_deref()
    }

    #[must_use]
    pub fn status_code(&self) -> Option<StatusCode> {
        self.0.status_code.or_else(|| self.0.code.status_code())
    }

    #[must_use]
    pub fn internal_error<E>(source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::with_source(S3ErrorCode::InternalError, Box::new(source))
    }
}

impl xml::Serialize for S3Error {
    fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Error", self)
    }
}

impl xml::SerializeContent for S3Error {
    fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {
        s.content("Code", self.0.code.as_str())?;
        if let Some(val) = self.0.message.as_deref() {
            s.content("Message", val)?;
        }
        // if let Some(val) = self.0.resource.as_deref() {
        //     s.content("Resource", val)?;
        // }
        if let Some(val) = self.0.request_id.as_deref() {
            s.content("RequestId", val)?;
        }
        Ok(())
    }
}

impl From<S3ErrorCode> for S3Error {
    fn from(code: S3ErrorCode) -> Self {
        Self::new(code)
    }
}

impl From<Infallible> for S3Error {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl fmt::Display for S3Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("S3Error");
        d.field("code", &self.0.code);
        if let Some(ref message) = self.0.message {
            d.field("message", &message);
        }
        if let Some(ref request_id) = self.0.request_id {
            d.field("request_id", &request_id);
        }
        if let Some(ref status_code) = self.0.status_code {
            d.field("status_code", &status_code);
        }
        if let Some(ref source) = self.0.source {
            d.field("source", &source);
        }
        d.finish_non_exhaustive()
    }
}

#[macro_export]
macro_rules! s3_error {
    ($source:expr, $code:ident) => {{
        let mut err = $crate::s3_error!($code);
        err.set_source(Box::new($source));
        err
    }};
    ($source:expr, $code:ident, $($arg:tt)+) => {{
        let mut err = $crate::s3_error!($code, $($arg)+);
        err.set_source(Box::new($source));
        err
    }};
    ($code:ident) => {
        $crate::S3Error::new($crate::S3ErrorCode::$code)
    };
    ($code:ident, $msg:literal) => {
        $crate::S3Error::with_message($crate::S3ErrorCode::$code, $msg)
    };
    ($code:ident, $fmt:literal, $($arg:tt)+) => {
        $crate::S3Error::with_message($crate::S3ErrorCode::$code, format!($fmt, $($arg)+))
    };
}

impl FromStr for S3ErrorCode {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_bytes(s.as_bytes()).unwrap())
    }
}
