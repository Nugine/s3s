//! aws-chunked stream

use crate::auth::SecretKey;
use crate::error::StdError;
use crate::sig_v4;
use crate::sig_v4::AmzDate;
use crate::stream::{ByteStream, DynByteStream, RemainingLength};
use crate::utils::SyncBoxFuture;

use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::pin_mut;
use futures::stream::{Stream, StreamExt};
use hyper::body::{Buf, Bytes};
use memchr::memchr;
use transform_stream::AsyncTryStream;

/// Aws chunked stream
pub struct AwsChunkedStream {
    /// inner
    inner: AsyncTryStream<Bytes, AwsChunkedStreamError, SyncBoxFuture<'static, Result<(), AwsChunkedStreamError>>>,

    remaining_length: usize,
}

impl Debug for AwsChunkedStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AwsChunkedStream").finish_non_exhaustive()
    }
}

/// signature ctx
#[derive(Debug)]
struct SignatureCtx {
    /// date
    amz_date: AmzDate,

    /// region
    region: Box<str>,

    /// secret_key
    secret_key: SecretKey,

    /// previous chunk's signature
    prev_signature: Box<str>,
}

/// [`AwsChunkedStream`]
#[derive(Debug, thiserror::Error)]
pub enum AwsChunkedStreamError {
    /// Underlying error
    #[error("AwsChunkedStreamError: Underlying: {}",.0)]
    Underlying(StdError),
    /// Signature mismatch
    #[error("AwsChunkedStreamError: SignatureMismatch")]
    SignatureMismatch,
    /// Format error
    #[error("AwsChunkedStreamError: FormatError")]
    FormatError,
    /// Incomplete stream
    #[error("AwsChunkedStreamError: Incomplete")]
    Incomplete,
}

/// Chunk meta
struct ChunkMeta<'a> {
    /// chunk size
    size: usize,
    /// chunk signature
    signature: &'a [u8],
}

/// nom parser
fn parse_chunk_meta(mut input: &[u8]) -> nom::IResult<&[u8], ChunkMeta<'_>> {
    use crate::utils::parser::consume;

    use nom::bytes::complete::{tag, take, take_till1};
    use nom::combinator::{all_consuming, map_res};
    use nom::number::complete::hex_u32;
    use nom::sequence::delimited;

    let s = &mut input;

    let size = consume(s, take_till1(|c| c == b';'))?;
    let (_, size) = map_res(hex_u32, TryInto::try_into)(size)?;

    let signature = consume(s, all_consuming(delimited(tag(b";chunk-signature="), take(64_usize), tag(b"\r\n"))))?;

    Ok((input, ChunkMeta { size, signature }))
}

/// check signature
fn check_signature(ctx: &SignatureCtx, expected_signature: &[u8], chunk_data: &[Bytes]) -> Option<Box<str>> {
    let string_to_sign = sig_v4::create_chunk_string_to_sign(&ctx.amz_date, &ctx.region, &ctx.prev_signature, chunk_data);

    let chunk_signature = sig_v4::calculate_signature(&string_to_sign, &ctx.secret_key, &ctx.amz_date, &ctx.region);

    (chunk_signature.as_bytes() == expected_signature).then(|| chunk_signature.into())
}

impl AwsChunkedStream {
    /// Constructs a `ChunkedStream`
    pub fn new<S>(
        body: S,
        seed_signature: Box<str>,
        amz_date: AmzDate,
        region: Box<str>,
        secret_key: SecretKey,
        decoded_content_length: usize,
    ) -> Self
    where
        S: Stream<Item = Result<Bytes, StdError>> + Send + Sync + 'static,
    {
        let inner = AsyncTryStream::<_, _, SyncBoxFuture<'static, Result<(), AwsChunkedStreamError>>>::new(|mut y| {
            #[allow(clippy::shadow_same)] // necessary for `pin_mut!`
            Box::pin(async move {
                pin_mut!(body);
                let mut prev_bytes = Bytes::new();
                let mut buf: Vec<u8> = Vec::new();
                let mut ctx = SignatureCtx {
                    amz_date,
                    region,
                    secret_key,
                    prev_signature: seed_signature,
                };

                loop {
                    let meta = {
                        match Self::read_meta_bytes(body.as_mut(), prev_bytes, &mut buf).await {
                            None => break,
                            Some(Err(e)) => return Err(AwsChunkedStreamError::Underlying(e)),
                            Some(Ok(remaining_bytes)) => prev_bytes = remaining_bytes,
                        };
                        if let Ok((_, meta)) = parse_chunk_meta(&buf) {
                            meta
                        } else {
                            return Err(AwsChunkedStreamError::FormatError);
                        }
                    };

                    let data: Vec<Bytes> = {
                        match Self::read_data(body.as_mut(), prev_bytes, meta.size).await {
                            None => return Err(AwsChunkedStreamError::Incomplete),
                            Some(Err(e)) => return Err(e),
                            Some(Ok((data, remaining_bytes))) => {
                                prev_bytes = remaining_bytes;
                                data
                            }
                        }
                    };

                    match check_signature(&ctx, meta.signature, &data) {
                        None => return Err(AwsChunkedStreamError::SignatureMismatch),
                        Some(signature) => ctx.prev_signature = signature,
                    }

                    for bytes in data {
                        y.yield_ok(bytes).await;
                    }
                }

                Ok(())
            })
        });
        Self {
            inner,
            remaining_length: decoded_content_length,
        }
    }

    /// read meta bytes and return remaining bytes
    async fn read_meta_bytes<S>(mut body: Pin<&mut S>, prev_bytes: Bytes, buf: &mut Vec<u8>) -> Option<Result<Bytes, StdError>>
    where
        S: Stream<Item = Result<Bytes, StdError>> + Send + 'static,
    {
        buf.clear();

        let mut push_meta_bytes = |mut bytes: Bytes| {
            if let Some(idx) = memchr(b'\n', bytes.as_ref()) {
                let len = idx.wrapping_add(1); // assume: idx < bytes.len()
                let leading = bytes.split_to(len);
                buf.extend_from_slice(leading.as_ref());
                return Some(bytes);
            }

            buf.extend_from_slice(bytes.as_ref());
            None
        };

        if let Some(remaining_bytes) = push_meta_bytes(prev_bytes) {
            return Some(Ok(remaining_bytes));
        }

        loop {
            match body.next().await? {
                Err(e) => return Some(Err(e)),
                Ok(bytes) => {
                    if let Some(remaining_bytes) = push_meta_bytes(bytes) {
                        return Some(Ok(remaining_bytes));
                    }
                }
            }
        }
    }

    /// read data and return remaining bytes
    async fn read_data<S>(
        mut body: Pin<&mut S>,
        prev_bytes: Bytes,
        mut data_size: usize,
    ) -> Option<Result<(Vec<Bytes>, Bytes), AwsChunkedStreamError>>
    where
        S: Stream<Item = Result<Bytes, StdError>> + Send + 'static,
    {
        let mut bytes_buffer = Vec::new();
        let mut push_data_bytes = |mut bytes: Bytes| {
            if data_size == 0 {
                return Some(bytes);
            }
            if data_size <= bytes.len() {
                let data = bytes.split_to(data_size);
                bytes_buffer.push(data);
                data_size = 0;
                Some(bytes)
            } else {
                data_size = data_size.wrapping_sub(bytes.len());
                bytes_buffer.push(bytes);
                None
            }
        };

        let mut remaining_bytes = 'outer: loop {
            if let Some(remaining_bytes) = push_data_bytes(prev_bytes) {
                break 'outer remaining_bytes;
            }

            loop {
                match body.next().await? {
                    Err(e) => return Some(Err(AwsChunkedStreamError::Underlying(e))),
                    Ok(bytes) => {
                        if let Some(remaining_bytes) = push_data_bytes(bytes) {
                            break 'outer remaining_bytes;
                        }
                    }
                }
            }
        };
        if remaining_bytes.starts_with(b"\r\n") {
            // fast path
            remaining_bytes.advance(2);
        } else {
            for &expected_byte in b"\r\n" {
                loop {
                    match *remaining_bytes.as_ref() {
                        [] => match body.next().await? {
                            Err(e) => return Some(Err(AwsChunkedStreamError::Underlying(e))),
                            Ok(bytes) => remaining_bytes = bytes,
                        },

                        [x, ..] if x == expected_byte => {
                            remaining_bytes.advance(1);
                            break;
                        }
                        _ => return Some(Err(AwsChunkedStreamError::FormatError)),
                    }
                }
            }
        }

        Some(Ok((bytes_buffer, remaining_bytes)))
    }

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes, AwsChunkedStreamError>>> {
        let ans = Pin::new(&mut self.inner).poll_next(cx);
        if let Poll::Ready(Some(Ok(ref bytes))) = ans {
            self.remaining_length = self.remaining_length.saturating_sub(bytes.len());
        }
        ans
    }

    pub fn exact_remaining_length(&self) -> usize {
        self.remaining_length
    }

    pub fn into_byte_stream(self) -> DynByteStream {
        crate::stream::into_dyn(self)
    }
}

impl Stream for AwsChunkedStream {
    type Item = Result<Bytes, AwsChunkedStreamError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.poll(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl ByteStream for AwsChunkedStream {
    fn remaining_length(&self) -> RemainingLength {
        RemainingLength::new_exact(self.remaining_length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn join(bytes: &[&[u8]]) -> Bytes {
        let mut buf = Vec::new();
        for b in bytes {
            buf.extend_from_slice(b);
        }
        buf.into()
    }

    #[tokio::test]
    async fn example_put_object_chunked_stream() {
        let chunk1_meta = b"10000;chunk-signature=ad80c730a21e5b8d04586a2213dd63b9a0e99e0e2307b0ade35a65485a288648\r\n";
        let chunk2_meta = b"400;chunk-signature=0055627c9e194cb4542bae2aa5492e3c1575bbb81b612b7d234b86a503ef5497\r\n";
        let chunk3_meta = b"0;chunk-signature=b6c6ea8a5354eaf15b3cb7646744f4275b71ea724fed81ceb9323e279d449df9\r\n";

        let chunk1_data = vec![b'a'; 0x10000]; // 65536
        let chunk2_data = vec![b'a'; 1024];
        let chunk3_data = [];
        let decoded_content_length = chunk1_data.len() + chunk2_data.len() + chunk3_data.len();

        let chunk1 = join(&[chunk1_meta, &chunk1_data, b"\r\n"]);
        let chunk2 = join(&[chunk2_meta, &chunk2_data, b"\r\n"]);
        let chunk3 = join(&[chunk3_meta, &chunk3_data, b"\r\n"]);

        let chunk_results: Vec<Result<Bytes, _>> = vec![Ok(chunk1), Ok(chunk2), Ok(chunk3)];

        let seed_signature = "4f232c4386841ef735655705268965c44a0e4690baa4adea153f7db9fa80a0a9";
        let timestamp = "20130524T000000Z";
        let region = "us-east-1";
        let secret_access_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";

        let date = AmzDate::parse(timestamp).unwrap();

        let stream = futures::stream::iter(chunk_results.into_iter());
        let mut chunked_stream = AwsChunkedStream::new(
            stream,
            seed_signature.into(),
            date,
            region.into(),
            secret_access_key.into(),
            decoded_content_length,
        );

        let ans1 = chunked_stream.next().await.unwrap();
        assert_eq!(ans1.unwrap(), chunk1_data.as_slice());

        let ans2 = chunked_stream.next().await.unwrap();
        assert_eq!(ans2.unwrap(), chunk2_data.as_slice());

        {
            assert!(chunked_stream.next().await.is_none());
            assert!(chunked_stream.next().await.is_none());
            assert!(chunked_stream.next().await.is_none());
        }
    }
}
