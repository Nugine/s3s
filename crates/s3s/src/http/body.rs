use crate::error::StdError;
use crate::stream::ByteStream;
use crate::stream::DynByteStream;
use crate::stream::RemainingLength;

use std::fmt;
use std::mem;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use bytes::Bytes;
use futures::Stream;

use http_body::Frame;

pin_project_lite::pin_project! {
    #[derive(Default)]
    pub struct Body {
        #[pin]
        kind: Kind,
    }
}

pin_project_lite::pin_project! {
    #[project = KindProj]
    #[derive(Default)]
    enum Kind {
        #[default]
        Empty,
        Once {
            inner: Bytes,
        },
        Hyper {
            #[pin]
            inner: hyper::body::Incoming,
        },
        DynStream {
            #[pin]
            inner: DynByteStream
        }
    }
}

impl Body {
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    fn once(bytes: Bytes) -> Self {
        Self {
            kind: Kind::Once { inner: bytes },
        }
    }

    fn hyper(body: hyper::body::Incoming) -> Self {
        Self {
            kind: Kind::Hyper { inner: body },
        }
    }

    fn dyn_stream(stream: DynByteStream) -> Self {
        Self {
            kind: Kind::DynStream { inner: stream },
        }
    }

    #[must_use]
    pub fn from_http_body<B>(body: B) -> Self
    where
        B: http_body::Body<Data = Bytes, Error = StdError> + Send + Sync + 'static,
    {
        Self::dyn_stream(Box::pin(crate::stream::BodyWrapper::new(body)))
    }
}

impl From<Bytes> for Body {
    fn from(bytes: Bytes) -> Self {
        Self::once(bytes)
    }
}

impl From<Vec<u8>> for Body {
    fn from(value: Vec<u8>) -> Self {
        Self::once(value.into())
    }
}

impl From<String> for Body {
    fn from(value: String) -> Self {
        Self::once(value.into())
    }
}

impl From<hyper::body::Incoming> for Body {
    fn from(body: hyper::body::Incoming) -> Self {
        Self::hyper(body)
    }
}

impl From<DynByteStream> for Body {
    fn from(stream: DynByteStream) -> Self {
        Self::dyn_stream(stream)
    }
}

impl http_body::Body for Body {
    type Data = Bytes;

    type Error = StdError;

    fn poll_frame(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let mut this = self.project();
        match this.kind.as_mut().project() {
            KindProj::Empty => {
                Poll::Ready(None) //
            }
            KindProj::Once { inner } => {
                let bytes = mem::take(inner);
                this.kind.set(Kind::Empty);
                if bytes.is_empty() {
                    Poll::Ready(None)
                } else {
                    Poll::Ready(Some(Ok(Frame::data(bytes))))
                }
            }
            KindProj::Hyper { inner } => {
                http_body::Body::poll_frame(inner, cx).map_err(|e| Box::new(e) as StdError)
                //
            }
            KindProj::DynStream { inner } => {
                Stream::poll_next(inner, cx).map_ok(Frame::data)
                //
            }
        }
    }

    fn is_end_stream(&self) -> bool {
        match &self.kind {
            Kind::Empty => true,
            Kind::Once { inner } => inner.is_empty(),
            Kind::Hyper { inner } => http_body::Body::is_end_stream(inner),
            Kind::DynStream { inner } => inner.remaining_length().exact() == Some(0),
        }
    }

    fn size_hint(&self) -> http_body::SizeHint {
        match &self.kind {
            Kind::Empty => http_body::SizeHint::with_exact(0),
            Kind::Once { inner } => http_body::SizeHint::with_exact(inner.len() as u64),
            Kind::Hyper { inner } => http_body::Body::size_hint(inner),
            Kind::DynStream { inner } => inner.remaining_length().into(),
        }
    }
}

impl Stream for Body {
    type Item = Result<Bytes, StdError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match http_body::Body::poll_frame(self, cx) {
            Poll::Ready(Some(Ok(f))) if f.is_data() => Poll::Ready(Some(Ok(f.into_data().expect("already checked")))),
            Poll::Ready(Some(Ok(f))) if f.is_trailers() => Poll::Ready(None),
            Poll::Ready(Some(Ok(_))) => Poll::Ready(Some(Err("unexpected BodyFrame".into()))),
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl ByteStream for Body {
    fn remaining_length(&self) -> RemainingLength {
        match &self.kind {
            Kind::Empty => RemainingLength::new_exact(0),
            Kind::Once { inner } => RemainingLength::new_exact(inner.len()),
            Kind::Hyper { inner } => http_body::Body::size_hint(inner).into(),
            Kind::DynStream { inner } => inner.remaining_length(),
        }
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Body");
        match &self.kind {
            Kind::Empty => {}
            Kind::Once { inner } => {
                d.field("once", inner);
            }
            Kind::Hyper { inner } => {
                d.field("hyper", inner);
            }
            Kind::DynStream { inner } => {
                d.field("dyn_stream", &"{..}");
                d.field("remaining_length", &inner.remaining_length());
            }
        }
        d.finish()
    }
}

impl Body {
    /// Stores all bytes in memory.
    ///
    /// WARNING: This function may cause **unbounded memory allocation**.
    ///
    /// # Errors
    /// Returns an error if `hyper` fails to read the body.
    pub async fn store_all_unlimited(&mut self) -> Result<Bytes, StdError> {
        if let Some(bytes) = self.bytes() {
            return Ok(bytes);
        }
        let body = mem::take(self);
        let bytes = http_body_util::BodyExt::collect(body).await?.to_bytes();
        *self = Self::from(bytes.clone());
        Ok(bytes)
    }

    pub fn bytes(&self) -> Option<Bytes> {
        match &self.kind {
            Kind::Empty => Some(Bytes::new()),
            Kind::Once { inner } => Some(inner.clone()),
            _ => None,
        }
    }

    pub fn take_bytes(&mut self) -> Option<Bytes> {
        match mem::take(&mut self.kind) {
            Kind::Empty => Some(Bytes::new()),
            Kind::Once { inner } => Some(inner),
            _ => None,
        }
    }
}
