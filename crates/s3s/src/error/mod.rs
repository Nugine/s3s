mod generated;
pub use self::generated::*;

use crate::xml;

use std::borrow::Cow;
use std::fmt;
use std::io::Write;
use std::str::FromStr;

pub type S3Result<T = (), E = S3Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub struct S3Error {
    code: S3ErrorCode,
    message: Option<Cow<'static, str>>,
    resource: Option<String>,
    request_id: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl S3Error {
    #[must_use]
    pub fn new(code: S3ErrorCode) -> Self {
        Self {
            code,
            message: None,
            resource: None,
            request_id: None,
            source: None,
        }
    }

    #[must_use]
    pub fn with_message(code: S3ErrorCode, msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            code,
            message: Some(msg.into()),
            resource: None,
            request_id: None,
            source: None,
        }
    }

    #[must_use]
    pub fn with_source(code: S3ErrorCode, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            code,
            message: None,
            resource: None,
            request_id: None,
            source: Some(source),
        }
    }

    pub fn set_code(&mut self, val: S3ErrorCode) {
        self.code = val;
    }

    pub fn set_message(&mut self, val: impl Into<Cow<'static, str>>) {
        self.message = Some(val.into());
    }

    pub fn set_source(&mut self, val: Box<dyn std::error::Error + Send + Sync + 'static>) {
        self.source = Some(val);
    }

    #[must_use]
    pub fn code(&self) -> S3ErrorCode {
        self.code
    }

    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    #[must_use]
    pub fn source(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        self.source.as_deref()
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
        s.content("Code", self.code.as_str())?;
        if let Some(val) = self.message.as_deref() {
            s.content("Message", val)?;
        }
        if let Some(val) = self.resource.as_deref() {
            s.content("Resource", val)?;
        }
        if let Some(val) = self.request_id.as_deref() {
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

impl fmt::Display for S3Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("S3Error");
        d.field("code", &self.code);
        if let Some(ref message) = self.message {
            d.field("message", &message);
        }
        if let Some(ref source) = self.source {
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

#[derive(Debug, thiserror::Error)]
#[error("Unknown S3 Error Code")]
pub struct ParseS3ErrorCodeError(());

impl FromStr for S3ErrorCode {
    type Err = ParseS3ErrorCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes()).ok_or(ParseS3ErrorCodeError(()))
    }
}
