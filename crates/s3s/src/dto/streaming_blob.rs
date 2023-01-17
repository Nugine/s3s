//! Streaming blob

use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::BoxStream;
use futures::{Stream, StreamExt};
use hyper::body::Bytes;

/// A new type of `BoxStream<'static, io::Result<Bytes>>`
pub struct StreamingBlob(pub BoxStream<'static, Result<Bytes, Box<dyn std::error::Error + Send + Sync>>>);

impl StreamingBlob {
    pub fn wrap<S, E>(stream: S) -> Self
    where
        S: Stream<Item = Result<Bytes, E>> + Send + 'static,
        E: std::error::Error + Send + Sync + 'static,
    {
        Self(Box::pin(stream.map(|x| {
            x.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        })))
    }
}

impl fmt::Debug for StreamingBlob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamingBlob").finish_non_exhaustive()
    }
}

impl Stream for StreamingBlob {
    type Item = Result<Bytes, Box<dyn std::error::Error + Send + Sync>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
