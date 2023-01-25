use crate::error::StdError;

use std::fmt;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use bytes::Bytes;
use futures::Stream;

pub trait ByteStream: Stream {
    fn remaining_length(&self) -> RemainingLength {
        RemainingLength::unknown()
    }
}

pub type DynByteStream = Pin<Box<dyn ByteStream<Item = Result<Bytes, StdError>> + Send + Sync + 'static>>;

pub struct RemainingLength {
    lower: usize,
    upper: Option<usize>,
}

impl RemainingLength {
    #[must_use]
    pub fn new(lower: usize, upper: Option<usize>) -> Self {
        if let Some(upper) = upper {
            assert!(lower <= upper);
        }
        Self { lower, upper }
    }

    #[must_use]
    pub fn unknown() -> Self {
        Self { lower: 0, upper: None }
    }

    #[must_use]
    pub fn new_exact(n: usize) -> Self {
        Self {
            lower: n,
            upper: Some(n),
        }
    }

    #[must_use]
    pub fn exact(&self) -> Option<usize> {
        self.upper.filter(|&upper| upper == self.lower)
    }

    #[must_use]
    fn into_size_hint(self) -> http_body::SizeHint {
        let mut sz = http_body::SizeHint::new();
        sz.set_lower(self.lower as u64);
        if let Some(upper) = self.upper {
            sz.set_upper(upper as u64);
        }
        sz
    }

    #[must_use]
    fn from_size_hint(sz: http_body::SizeHint) -> Self {
        Self {
            lower: sz.lower() as usize,
            upper: sz.upper().map(|x| x as usize),
        }
    }
}

impl From<RemainingLength> for http_body::SizeHint {
    fn from(value: RemainingLength) -> Self {
        value.into_size_hint()
    }
}

impl From<http_body::SizeHint> for RemainingLength {
    fn from(value: http_body::SizeHint) -> Self {
        Self::from_size_hint(value)
    }
}

impl fmt::Debug for RemainingLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.upper {
            Some(upper) => write!(f, "({}..{})", self.lower, upper),
            None => write!(f, "({}..)", self.lower),
        }
    }
}

pin_project_lite::pin_project! {
    pub(crate) struct StreamWrapper<S> {
        #[pin]
        inner: S
    }
}

impl<S, E> Stream for StreamWrapper<S>
where
    S: Stream<Item = Result<Bytes, E>> + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    type Item = Result<Bytes, StdError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.inner.poll_next(cx).map_err(|e| Box::new(e) as StdError)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<S> ByteStream for StreamWrapper<S>
where
    StreamWrapper<S>: Stream<Item = Result<Bytes, StdError>>,
{
    fn remaining_length(&self) -> RemainingLength {
        RemainingLength::unknown()
    }
}

pub(crate) fn wrap<S>(inner: S) -> DynByteStream
where
    StreamWrapper<S>: ByteStream<Item = Result<Bytes, StdError>> + Send + Sync + 'static,
{
    Box::pin(StreamWrapper { inner })
}
