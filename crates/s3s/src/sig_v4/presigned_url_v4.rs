//! presigned url information

use super::AmzDate;
use super::CredentialV4;

use crate::http::OrderedQs;
use crate::utils::crypto::is_sha256_checksum;

use smallvec::SmallVec;

/// Presigned url information
#[derive(Debug)]
pub struct PresignedUrlV4<'a> {
    /// algorithm
    pub algorithm: &'a str,
    /// credential
    pub credential: CredentialV4<'a>,
    /// amz date
    pub amz_date: AmzDate,
    /// expires
    pub expires: time::Duration,
    /// signed headers
    pub signed_headers: SmallVec<[&'a str; 16]>,
    /// signature
    pub signature: &'a str,
}

/// [`PresignedUrlV4`]
#[derive(Debug, thiserror::Error)]
#[error("ParsePresignedUrlError")]
pub struct ParsePresignedUrlError {
    /// priv place holder
    _priv: (),
}

/// query strings of a presigned url
struct PresignedQs<'a> {
    /// X-Amz-Algorithm
    x_amz_algorithm: &'a str,
    /// X-Amz-Credential
    x_amz_credential: &'a str,
    /// X-Amz-Date
    x_amz_date: &'a str,
    /// X-Amz-Expires
    x_amz_expires: &'a str,
    /// X-Amz-SignedHeaders
    x_amz_signed_headers: &'a str,
    /// X-Amz-Signature
    x_amz_signature: &'a str,
}

impl<'a> PresignedQs<'a> {
    /// Creates `PresignedQs` from `OrderedQs`
    fn from_ordered_qs(qs: &'a OrderedQs) -> Option<Self> {
        Some(PresignedQs {
            x_amz_algorithm: qs.get_unique("X-Amz-Algorithm")?,
            x_amz_credential: qs.get_unique("X-Amz-Credential")?,
            x_amz_date: qs.get_unique("X-Amz-Date")?,
            x_amz_expires: qs.get_unique("X-Amz-Expires")?,
            x_amz_signed_headers: qs.get_unique("X-Amz-SignedHeaders")?,
            x_amz_signature: qs.get_unique("X-Amz-Signature")?,
        })
    }
}

impl<'a> PresignedUrlV4<'a> {
    /// Parses `PresignedUrl` from query
    ///
    /// # Errors
    /// Returns `ParsePresignedUrlError` if it failed to parse
    pub fn parse(qs: &'a OrderedQs) -> Result<Self, ParsePresignedUrlError> {
        let err = || ParsePresignedUrlError { _priv: () };

        let info = PresignedQs::from_ordered_qs(qs).ok_or_else(err)?;

        let algorithm = info.x_amz_algorithm;

        let credential = CredentialV4::parse(info.x_amz_credential).map_err(|_e| err())?;

        let amz_date = AmzDate::parse(info.x_amz_date).map_err(|_e| err())?;

        let expires = parse_expires(info.x_amz_expires).ok_or_else(err)?;

        if !info.x_amz_signed_headers.is_ascii() {
            return Err(err());
        }
        let signed_headers = info.x_amz_signed_headers.split(';').collect();

        if !is_sha256_checksum(info.x_amz_signature) {
            return Err(err());
        }
        let signature = info.x_amz_signature;

        Ok(Self {
            algorithm,
            credential,
            amz_date,
            expires,
            signed_headers,
            signature,
        })
    }
}

fn parse_expires(s: &str) -> Option<time::Duration> {
    let x = s.parse::<u32>().ok().filter(|&x| x > 0)?;
    Some(time::Duration::new(i64::from(x), 0))
}
