//! A path in the S3 storage.
//!
//! + [Request styles](https://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAPI.html#virtual-hosted-path-style-requests)
//! + [Bucket nameing rules](https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html#bucketnamingrules)

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

#[derive(Debug, PartialEq, Eq)]
pub enum S3PathRef<'a> {
    Root,
    Bucket { bucket: &'a str },
    Object { bucket: &'a str, key: &'a str },
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

impl From<S3PathRef<'_>> for S3Path {
    fn from(val: S3PathRef<'_>) -> Self {
        match val {
            S3PathRef::Root => S3Path::Root,
            S3PathRef::Bucket { bucket } => S3Path::Bucket { bucket: bucket.into() },
            S3PathRef::Object { bucket, key } => S3Path::Object {
                bucket: bucket.into(),
                key: key.into(),
            },
        }
    }
}

/// See [bucket nameing rules](https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html#bucketnamingrules)
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

    if name.parse::<IpAddr>().is_ok() {
        return false;
    }

    if name.starts_with("xn--") {
        return false;
    }

    true
}

/// The name for a key is a sequence of Unicode characters whose UTF-8 encoding is at most 1,024 bytes long.
/// See [object keys](https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys)
#[must_use]
pub const fn check_key(key: &str) -> bool {
    key.len() <= 1024
}

/// Parses a path-style request
/// # Errors
/// Returns an `Err` if the s3 path is invalid
pub fn parse_path_style(uri_path: &str) -> Result<S3PathRef, ParseS3PathError> {
    let path = if let Some(("", x)) = uri_path.split_once('/') {
        x
    } else {
        return Err(ParseS3PathError::InvalidPath);
    };

    if path.is_empty() {
        return Ok(S3PathRef::Root);
    }

    let (bucket, key) = match path.split_once('/') {
        None => (path, None),
        Some((x, "")) => (x, None),
        Some((bucket, key)) => (bucket, Some(key)),
    };

    if !check_bucket_name(bucket) {
        return Err(ParseS3PathError::InvalidBucketName);
    }

    let key = match key {
        None => return Ok(S3PathRef::Bucket { bucket }),
        Some(k) => k,
    };

    if !check_key(key) {
        return Err(ParseS3PathError::KeyTooLong);
    }

    Ok(S3PathRef::Object { bucket, key })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_style() {
        assert_eq!(parse_path_style("/"), Ok(S3PathRef::Root));

        assert_eq!(parse_path_style("/bucket"), Ok(S3PathRef::Bucket { bucket: "bucket" }));

        assert_eq!(parse_path_style("/bucket/"), Ok(S3PathRef::Bucket { bucket: "bucket" }));

        assert_eq!(
            parse_path_style("/bucket/dir/object"),
            Ok(S3PathRef::Object {
                bucket: "bucket",
                key: "dir/object",
            })
        );

        assert_eq!(parse_path_style("asd").unwrap_err(), ParseS3PathError::InvalidPath);

        assert_eq!(parse_path_style("a/").unwrap_err(), ParseS3PathError::InvalidPath);

        assert_eq!(parse_path_style("/*").unwrap_err(), ParseS3PathError::InvalidBucketName);

        let too_long_path = format!("/{}/{}", "asd", "b".repeat(2048).as_str());

        assert_eq!(parse_path_style(&too_long_path).unwrap_err(), ParseS3PathError::KeyTooLong);
    }
}
