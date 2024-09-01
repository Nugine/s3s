use crate::error::S3Result;

use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct VirtualHost<'a> {
    domain: Cow<'a, str>,
    bucket: Option<Cow<'a, str>>,
    // pub(crate) region: Option<Cow<'a, str>>,
}

impl<'a> VirtualHost<'a> {
    pub fn new(domain: impl Into<Cow<'a, str>>) -> Self {
        Self {
            domain: domain.into(),
            bucket: None,
        }
    }

    pub fn with_bucket(domain: impl Into<Cow<'a, str>>, bucket: impl Into<Cow<'a, str>>) -> Self {
        Self {
            domain: domain.into(),
            bucket: Some(bucket.into()),
        }
    }

    #[inline]
    #[must_use]
    pub fn domain(&self) -> &str {
        self.domain.as_ref()
    }

    #[inline]
    #[must_use]
    pub fn bucket(&self) -> Option<&str> {
        self.bucket.as_deref()
    }
}

pub trait S3Host: Send + Sync + 'static {
    /// Parses the `Host` header of the HTTP request.
    ///
    /// # Errors
    /// Returns an error if the `Host` is invalid for this service.
    fn parse_host_header<'a>(&'a self, host: &'a str) -> S3Result<VirtualHost<'a>>;
}

pub struct SingleDomain {
    base_domain: String,
}

impl SingleDomain {
    #[must_use]
    pub fn new(base_domain: impl Into<String>) -> Self {
        Self {
            base_domain: base_domain.into(),
        }
    }
}

impl S3Host for SingleDomain {
    fn parse_host_header<'a>(&'a self, host: &'a str) -> S3Result<VirtualHost<'a>> {
        let base_domain = self.base_domain.as_str();

        if host == base_domain {
            return Ok(VirtualHost::new(base_domain));
        }

        if let Some(bucket) = host.strip_suffix(&self.base_domain).and_then(|h| h.strip_suffix('.')) {
            return Ok(VirtualHost::with_bucket(base_domain, bucket));
        };

        let bucket = host.to_ascii_lowercase();
        Ok(VirtualHost::with_bucket(host, bucket))
    }
}
