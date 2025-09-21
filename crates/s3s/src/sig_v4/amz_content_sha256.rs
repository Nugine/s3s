//! x-amz-content-sha256

use crate::utils::crypto::is_sha256_checksum;

/// x-amz-content-sha256
///
/// See [Common Request Headers](https://docs.aws.amazon.com/AmazonS3/latest/API/RESTCommonRequestHeaders.html)
#[derive(Debug)]
pub enum AmzContentSha256<'a> {
    /// `STREAMING-AWS4-HMAC-SHA256-PAYLOAD`
    MultipleChunks,
    /// `STREAMING-AWS4-HMAC-SHA256-PAYLOAD-TRAILER`
    MultipleChunksWithTrailer,
    /// `STREAMING-UNSIGNED-PAYLOAD-TRAILER`
    UnsignedPayloadWithTrailer,
    /// single chunk
    SingleChunk {
        /// the checksum of single chunk payload
        #[allow(dead_code)] // TODO: check this field when calculating the payload checksum
        payload_checksum: &'a str,
    },
    /// `UNSIGNED-PAYLOAD`
    UnsignedPayload,
}

/// [`AmzContentSha256`]
#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum ParseAmzContentSha256Error {
    /// invalid checksum
    #[error("ParseAmzContentSha256Error: InvalidChecksum")]
    InvalidChecksum,
}

impl<'a> AmzContentSha256<'a> {
    /// Parses `AmzContentSha256` from `x-amz-content-sha256` header
    ///
    /// # Errors
    /// Returns an `Err` if the header is invalid
    pub fn parse(header: &'a str) -> Result<Self, ParseAmzContentSha256Error> {
        match header {
            "UNSIGNED-PAYLOAD" => Ok(Self::UnsignedPayload),
            "STREAMING-AWS4-HMAC-SHA256-PAYLOAD" => Ok(Self::MultipleChunks),
            "STREAMING-AWS4-HMAC-SHA256-PAYLOAD-TRAILER" => Ok(Self::MultipleChunksWithTrailer),
            "STREAMING-UNSIGNED-PAYLOAD-TRAILER" => Ok(Self::UnsignedPayloadWithTrailer),
            payload_checksum => {
                if !is_sha256_checksum(payload_checksum) {
                    return Err(ParseAmzContentSha256Error::InvalidChecksum);
                }
                Ok(Self::SingleChunk { payload_checksum })
            }
        }
    }
}
