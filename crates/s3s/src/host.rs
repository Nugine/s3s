use crate::error::S3Result;

use std::borrow::Cow;

use rust_utils::default::default;

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

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DomainError {
    #[error("The domain is invalid")]
    InvalidDomain,

    #[error("Some subdomains overlap with each other")]
    OverlappingSubdomains,

    #[error("No base domains are specified")]
    ZeroDomains,
}

/// Naive check for a valid domain.
fn is_valid_domain(mut s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    if let Some((host, port)) = s.split_once(':') {
        if port.is_empty() {
            return false;
        }

        if port.parse::<u16>().is_err() {
            return false;
        }

        s = host;
    }

    for part in s.split('.') {
        if part.is_empty() {
            return false;
        }

        if part.as_bytes().iter().any(|&b| !b.is_ascii_alphanumeric() && b != b'-') {
            return false;
        }
    }

    true
}

fn parse_host_header<'a>(base_domain: &'a str, host: &'a str) -> Option<VirtualHost<'a>> {
    if host == base_domain {
        return Some(VirtualHost::new(base_domain));
    }

    if let Some(bucket) = host.strip_suffix(base_domain).and_then(|h| h.strip_suffix('.')) {
        return Some(VirtualHost::with_bucket(base_domain, bucket));
    };

    None
}

#[derive(Debug)]
pub struct SingleDomain {
    base_domain: String,
}

impl SingleDomain {
    /// Create a new `SingleDomain` with the base domain.
    ///
    /// # Errors
    /// Returns an error if the base domain is invalid.
    pub fn new(base_domain: &str) -> Result<Self, DomainError> {
        if !is_valid_domain(base_domain) {
            return Err(DomainError::InvalidDomain);
        }

        Ok(Self {
            base_domain: base_domain.into(),
        })
    }
}

impl S3Host for SingleDomain {
    fn parse_host_header<'a>(&'a self, host: &'a str) -> S3Result<VirtualHost<'a>> {
        let base_domain = self.base_domain.as_str();

        if let Some(vh) = parse_host_header(base_domain, host) {
            return Ok(vh);
        }

        if is_valid_domain(host) {
            let bucket = host.to_ascii_lowercase();
            return Ok(VirtualHost::with_bucket(host, bucket));
        }

        Err(s3_error!(InvalidRequest, "Invalid host header"))
    }
}

#[derive(Debug)]
pub struct MultiDomain {
    base_domains: Vec<String>,
}

impl MultiDomain {
    /// Create a new `MultiDomain` with the base domains.
    ///
    /// # Errors
    /// Returns an error if
    /// + any of the base domains are invalid.
    /// + any of the base domains overlap with each other.
    /// + no base domains are specified.
    pub fn new<I>(base_domains: I) -> Result<Self, DomainError>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut v: Vec<String> = default();

        for domain in base_domains {
            let domain = domain.as_ref();

            if !is_valid_domain(domain) {
                return Err(DomainError::InvalidDomain);
            }

            for other in &v {
                if domain.ends_with(other) || other.ends_with(domain) {
                    return Err(DomainError::OverlappingSubdomains);
                }
            }

            v.push(domain.to_owned());
        }

        if v.is_empty() {
            return Err(DomainError::ZeroDomains);
        }

        Ok(Self { base_domains: v })
    }
}

impl S3Host for MultiDomain {
    fn parse_host_header<'a>(&'a self, host: &'a str) -> S3Result<VirtualHost<'a>> {
        for base_domain in &self.base_domains {
            if let Some(vh) = parse_host_header(base_domain, host) {
                return Ok(vh);
            }
        }

        if is_valid_domain(host) {
            let bucket = host.to_ascii_lowercase();
            return Ok(VirtualHost::with_bucket(host, bucket));
        }

        Err(s3_error!(InvalidRequest, "Invalid host header"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::S3ErrorCode;

    #[test]
    fn single_domain_new() {
        let domain = "example.com";
        let result = SingleDomain::new(domain);
        let sd = result.unwrap();
        assert_eq!(sd.base_domain, domain);

        let domain = "example.com.org";
        let result = SingleDomain::new(domain);
        let sd = result.unwrap();
        assert_eq!(sd.base_domain, domain);

        let domain = "example.com.";
        let result = SingleDomain::new(domain);
        let err = result.unwrap_err();
        assert!(matches!(err, DomainError::InvalidDomain));

        let domain = "example.com:";
        let result = SingleDomain::new(domain);
        let err = result.unwrap_err();
        assert!(matches!(err, DomainError::InvalidDomain));

        let domain = "example.com:80";
        let result = SingleDomain::new(domain);
        assert!(result.is_ok());
    }

    #[test]
    fn multi_domain_new() {
        let domains = ["example.com", "example.org"];
        let result = MultiDomain::new(&domains);
        let md = result.unwrap();
        assert_eq!(md.base_domains, domains);

        let domains = ["example.com", "example.com"];
        let result = MultiDomain::new(&domains);
        let err = result.unwrap_err();
        assert!(matches!(err, DomainError::OverlappingSubdomains));

        let domains = ["example.com", "example.com.org"];
        let result = MultiDomain::new(&domains);
        let md = result.unwrap();
        assert_eq!(md.base_domains, domains);

        let domains: [&str; 0] = [];
        let result = MultiDomain::new(&domains);
        let err = result.unwrap_err();
        assert!(matches!(err, DomainError::ZeroDomains));
    }

    #[test]
    fn multi_domain_parse() {
        let domains = ["example.com", "example.org"];
        let md = MultiDomain::new(domains.iter().copied()).unwrap();

        let host = "example.com";
        let result = md.parse_host_header(host);
        let vh = result.unwrap();
        assert_eq!(vh.domain(), host);
        assert_eq!(vh.bucket(), None);

        let host = "example.org";
        let result = md.parse_host_header(host);
        let vh = result.unwrap();
        assert_eq!(vh.domain(), host);
        assert_eq!(vh.bucket(), None);

        let host = "example.com.org";
        let result = md.parse_host_header(host);
        let vh = result.unwrap();
        assert_eq!(vh.domain(), host);
        assert_eq!(vh.bucket(), Some("example.com.org"));

        let host = "example.com.org.";
        let result = md.parse_host_header(host);
        let err = result.unwrap_err();
        assert!(matches!(err.code(), S3ErrorCode::InvalidRequest));

        let host = "example.com.org.example.com";
        let result = md.parse_host_header(host);
        let vh = result.unwrap();
        assert_eq!(vh.domain(), "example.com");
        assert_eq!(vh.bucket(), Some("example.com.org"));
    }
}
