//! x-amz-copy-source

use crate::http;
use crate::path;

use std::fmt::Write;

/// x-amz-copy-source
#[derive(Debug, Clone, PartialEq)]
pub enum CopySource {
    /// bucket repr
    Bucket {
        /// bucket
        bucket: Box<str>,
        /// key
        key: Box<str>,
        /// version id
        version_id: Option<Box<str>>,
    },
    /// access point repr
    AccessPoint {
        /// region
        region: Box<str>,
        /// account id
        account_id: Box<str>,
        /// access point name
        access_point_name: Box<str>,
        /// key
        key: Box<str>,
    },
}

/// [`CopySource`]
#[derive(Debug, thiserror::Error)]
pub enum ParseCopySourceError {
    /// pattern mismatch
    #[error("ParseAmzCopySourceError: PatternMismatch")]
    PatternMismatch,

    /// invalid bucket name
    #[error("ParseAmzCopySourceError: InvalidBucketName")]
    InvalidBucketName,

    /// invalid key
    #[error("ParseAmzCopySourceError: InvalidKey")]
    InvalidKey,

    #[error("ParseAmzCopySourceError: InvalidEncoding")]
    InvalidEncoding,
}

impl CopySource {
    /// Parses [`CopySource`] from header
    /// # Errors
    /// Returns an error if the header is invalid
    pub fn parse(header: &str) -> Result<Self, ParseCopySourceError> {
        let header = urlencoding::decode(header).map_err(|_| ParseCopySourceError::InvalidEncoding)?;
        let header = header.strip_prefix('/').unwrap_or(&header);

        // FIXME: support access point
        match header.split_once('/') {
            None => Err(ParseCopySourceError::PatternMismatch),
            Some((bucket, remaining)) => {
                let (key, version_id) = match remaining.split_once('?') {
                    Some((key, remaining)) => {
                        let version_id = remaining
                            .split_once('=')
                            .and_then(|(name, val)| (name == "versionId").then_some(val));
                        (key, version_id)
                    }
                    None => (remaining, None),
                };

                if !path::check_bucket_name(bucket) {
                    return Err(ParseCopySourceError::InvalidBucketName);
                }

                if !path::check_key(key) {
                    return Err(ParseCopySourceError::InvalidKey);
                }

                Ok(Self::Bucket {
                    bucket: bucket.into(),
                    key: key.into(),
                    version_id: version_id.map(Into::into),
                })
            }
        }
    }

    #[must_use]
    pub fn format_to_string(&self) -> String {
        let mut buf = String::new();
        match self {
            CopySource::Bucket { bucket, key, version_id } => {
                write!(&mut buf, "{bucket}/{key}").unwrap();
                if let Some(version_id) = version_id {
                    write!(&mut buf, "?versionId={version_id}").unwrap();
                }
            }
            CopySource::AccessPoint { .. } => {
                unimplemented!()
            }
        }
        buf
    }
}

impl http::TryFromHeaderValue for CopySource {
    type Error = ParseCopySourceError;

    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let header = val.to_str().map_err(|_| ParseCopySourceError::InvalidEncoding)?;
        Self::parse(header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_style() {
        {
            let header = "awsexamplebucket/reports/january.pdf";
            let val = CopySource::parse(header).unwrap();
            match val {
                CopySource::Bucket { bucket, key, version_id } => {
                    assert_eq!(&*bucket, "awsexamplebucket");
                    assert_eq!(&*key, "reports/january.pdf");
                    assert!(version_id.is_none());
                }
                CopySource::AccessPoint { .. } => panic!(),
            }
        }

        {
            let header = "awsexamplebucket/reports/january.pdf?versionId=QUpfdndhfd8438MNFDN93jdnJFkdmqnh893";
            let val = CopySource::parse(header).unwrap();
            match val {
                CopySource::Bucket { bucket, key, version_id } => {
                    assert_eq!(&*bucket, "awsexamplebucket");
                    assert_eq!(&*key, "reports/january.pdf");
                    assert_eq!(version_id.as_deref().unwrap(), "QUpfdndhfd8438MNFDN93jdnJFkdmqnh893");
                }
                CopySource::AccessPoint { .. } => panic!(),
            }
        }
    }
}
