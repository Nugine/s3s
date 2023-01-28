use super::SelectObjectContentEvent;

use crate::S3Result;

use std::fmt;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use futures::Stream;

pub struct SelectObjectContentEventStream {
    inner: Pin<Box<dyn Stream<Item = S3Result<SelectObjectContentEvent>> + Send + Sync + 'static>>,
}

impl SelectObjectContentEventStream {
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = S3Result<SelectObjectContentEvent>> + Send + Sync + 'static,
    {
        Self { inner: Box::pin(stream) }
    }
}

impl Stream for SelectObjectContentEventStream {
    type Item = S3Result<SelectObjectContentEvent>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl fmt::Debug for SelectObjectContentEventStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SelectObjectContentEventStream")
            .field("size_hint", &self.size_hint())
            .finish_non_exhaustive()
    }
}
