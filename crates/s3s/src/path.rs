#![deny(missing_docs)]

//! A path in the S3 storage.
//!
//! + [Request styles](https://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAPI.html#virtual-hosted-path-style-requests)
//! + [Bucket naming rules](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html)

use std::net::IpAddr;

/// A path in the S3 storage
#[derive(Debug, PartialEq, Eq)]
pub enum S3Path {
    /// Root path
    Root,
    /// Bucket path
    Bucket {
        /// Bucket name
        bucket: Box<str>,
    },
    /// Object path
    Object {
        /// Bucket name
        bucket: Box<str>,
        /// Object key
        key: Box<str>,
    },
}

/// An error which can be returned when parsing a s3 path
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseS3PathError {
    /// The request is not a valid path-style request
    #[error("The request is not a valid path-style request")]
    InvalidPath,

    /// The bucket name is invalid
    #[error("The bucket name is invalid")]
    InvalidBucketName,

    /// The object key is too long
    #[error("The object key is too long")]
    KeyTooLong,
}

impl S3Path {
    /// Create a new S3 path representing the root
    #[must_use]
    pub fn root() -> Self {
        Self::Root
    }

    /// Create a new S3 path representing the bucket
    #[must_use]
    pub fn bucket(bucket: &str) -> Self {
        Self::Bucket { bucket: bucket.into() }
    }

    /// Create a new S3 path representing the object
    #[must_use]
    pub fn object(bucket: &str, key: &str) -> Self {
        Self::Object {
            bucket: bucket.into(),
            key: key.into(),
        }
    }

    /// Returns `true` if the path is root
    #[must_use]
    pub fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    /// Returns the bucket name if the path is bucket
    #[must_use]
    pub fn as_bucket(&self) -> Option<&str> {
        match self {
            Self::Bucket { bucket } => Some(bucket),
            _ => None,
        }
    }

    /// Returns the bucket name and object key if the path is object
    #[must_use]
    pub fn as_object(&self) -> Option<(&str, &str)> {
        match self {
            Self::Object { bucket, key } => Some((bucket, key)),
            _ => None,
        }
    }

    /// Returns the bucket name part if the path is bucket or object
    #[must_use]
    pub fn get_bucket_name(&self) -> Option<&str> {
        match self {
            Self::Root => None,
            Self::Bucket { bucket } | Self::Object { bucket, .. } => Some(bucket),
        }
    }

    /// Returns the object key part if the path is object
    #[must_use]
    pub fn get_object_key(&self) -> Option<&str> {
        match self {
            Self::Root | Self::Bucket { .. } => None,
            Self::Object { key, .. } => Some(key),
        }
    }
}

/// See [bucket naming rules](https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html)
#[allow(clippy::manual_is_variant_and)] // FIXME: https://github.com/rust-lang/rust-clippy/issues/15202
#[must_use]
pub fn check_bucket_name(name: &str) -> bool {
    if !(3_usize..64).contains(&name.len()) {
        return false;
    }

    if !name
        .as_bytes()
        .iter()
        .all(|&b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'.' || b == b'-')
    {
        return false;
    }

    if name.as_bytes().first().map(|&b| b.is_ascii_lowercase() || b.is_ascii_digit()) != Some(true) {
        return false;
    }

    if name.as_bytes().last().map(|&b| b.is_ascii_lowercase() || b.is_ascii_digit()) != Some(true) {
        return false;
    }

    if name.contains("..") {
        return false;
    }

    if name.parse::<IpAddr>().is_ok() {
        return false;
    }

    if name.starts_with("xn--") {
        return false;
    }

    true
}

/// Check if the key is valid
///
/// The name for a key is a sequence of Unicode characters whose UTF-8 encoding is at most 1,024 bytes long.
/// See [object keys](https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys)
#[must_use]
pub const fn check_key(key: &str) -> bool {
    key.len() <= 1024
}

/// Parses a path-style request
/// # Errors
/// Returns an `Err` if the s3 path is invalid
pub fn parse_path_style(uri_path: &str) -> Result<S3Path, ParseS3PathError> {
    let Some(path) = uri_path.strip_prefix('/') else { return Err(ParseS3PathError::InvalidPath) };

    if path.is_empty() {
        return Ok(S3Path::root());
    }

    let (bucket, key) = match path.split_once('/') {
        None => (path, None),
        Some((x, "")) => (x, None),
        Some((bucket, key)) => (bucket, Some(key)),
    };

    if !check_bucket_name(bucket) {
        return Err(ParseS3PathError::InvalidBucketName);
    }

    let Some(key) = key else { return Ok(S3Path::bucket(bucket)) };

    if !check_key(key) {
        return Err(ParseS3PathError::KeyTooLong);
    }

    Ok(S3Path::object(bucket, key))
}

/// Parses a virtual-hosted-style request
/// # Errors
/// Returns an `Err` if the s3 path is invalid
pub fn parse_virtual_hosted_style(vh_bucket: Option<&str>, uri_path: &str) -> Result<S3Path, ParseS3PathError> {
    let Some(bucket) = vh_bucket else { return parse_path_style(uri_path) };

    let Some(key) = uri_path.strip_prefix('/') else { return Err(ParseS3PathError::InvalidPath) };

    if !check_bucket_name(bucket) {
        return Err(ParseS3PathError::InvalidBucketName);
    }

    if key.is_empty() {
        return Ok(S3Path::Bucket { bucket: bucket.into() });
    }

    if !check_key(key) {
        return Err(ParseS3PathError::KeyTooLong);
    }

    Ok(S3Path::Object {
        bucket: bucket.into(),
        key: key.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::host::{S3Host, SingleDomain};

    #[test]
    fn bucket_naming_rules() {
        let cases = [
            ("docexamplebucket1", true),
            ("log-delivery-march-2020", true),
            ("my-hosted-content", true),
            ("docexamplewebsite.com", true),
            ("www.docexamplewebsite.com", true),
            ("my.example.s3.bucket", true),
            ("doc_example_bucket", false),
            ("DocExampleBucket", false),
            ("doc-example-bucket-", false),
        ];

        for (input, expected) in cases {
            assert_eq!(check_bucket_name(input), expected);
        }
    }

    #[test]
    fn path_style() {
        let too_long_path = format!("/{}/{}", "asd", "b".repeat(2048).as_str());

        let cases = [
            ("/", Ok(S3Path::Root)),
            ("/bucket", Ok(S3Path::bucket("bucket"))),
            ("/bucket/", Ok(S3Path::bucket("bucket"))),
            ("/bucket/dir/object", Ok(S3Path::object("bucket", "dir/object"))),
            ("asd", Err(ParseS3PathError::InvalidPath)),
            ("a/", Err(ParseS3PathError::InvalidPath)),
            ("/*", Err(ParseS3PathError::InvalidBucketName)),
            (too_long_path.as_str(), Err(ParseS3PathError::KeyTooLong)),
        ];

        for (uri_path, expected) in cases {
            assert_eq!(parse_path_style(uri_path), expected);
        }
    }

    #[test]
    fn virtual_hosted_style() {
        {
            let s3_host = SingleDomain::new("s3.us-east-1.amazonaws.com").unwrap();
            let host = "s3.us-east-1.amazonaws.com";
            let uri_path = "/example.com/homepage.html";
            let vh = s3_host.parse_host_header(host).unwrap();
            let ans = parse_virtual_hosted_style(vh.bucket(), uri_path);
            let expected = Ok(S3Path::object("example.com", "homepage.html"));
            assert_eq!(ans, expected);
        }

        {
            let s3_host = SingleDomain::new("s3.eu-west-1.amazonaws.com").unwrap();
            let host = "doc-example-bucket1.eu.s3.eu-west-1.amazonaws.com";
            let uri_path = "/homepage.html";
            let vh = s3_host.parse_host_header(host).unwrap();
            let ans = parse_virtual_hosted_style(vh.bucket(), uri_path);
            let expected = Ok(S3Path::object("doc-example-bucket1.eu", "homepage.html"));
            assert_eq!(ans, expected);
        }

        {
            let s3_host = SingleDomain::new("s3.eu-west-1.amazonaws.com").unwrap();
            let host = "doc-example-bucket1.eu.s3.eu-west-1.amazonaws.com";
            let uri_path = "/";
            let vh = s3_host.parse_host_header(host).unwrap();
            let ans = parse_virtual_hosted_style(vh.bucket(), uri_path);
            let expected = Ok(S3Path::bucket("doc-example-bucket1.eu"));
            assert_eq!(ans, expected);
        }

        {
            let s3_host = SingleDomain::new("s3.us-east-1.amazonaws.com").unwrap();
            let host = "example.com";
            let uri_path = "/homepage.html";
            let vh = s3_host.parse_host_header(host).unwrap();
            let ans = parse_virtual_hosted_style(vh.bucket(), uri_path);
            let expected = Ok(S3Path::object("example.com", "homepage.html"));
            assert_eq!(ans, expected);
        }
    }
}
