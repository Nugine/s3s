use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use bytes::Bytes;
use http_body::{Body, Frame};
use tokio::time::Interval;

use crate::{http::Response, StdError};

// sends whitespace while the future is pending
pin_project_lite::pin_project! {

    pub struct KeepAliveBody<F> {
        #[pin]
        inner: F,
        initial_body: Option<Bytes>,
        response: Option<Response>,
        interval: Interval,
        done: bool,
    }
}
impl<F> KeepAliveBody<F> {
    pub fn new(inner: F, interval: Duration) -> Self {
        Self {
            inner,
            initial_body: None,
            response: None,
            interval: tokio::time::interval(interval),
            done: false,
        }
    }

    pub fn with_initial_body(inner: F, initial_body: Bytes, interval: Duration) -> Self {
        Self {
            inner,
            initial_body: Some(initial_body),
            response: None,
            interval: tokio::time::interval(interval),
            done: false,
        }
    }
}

impl<F> Body for KeepAliveBody<F>
where
    F: Future<Output = Response>,
{
    type Data = Bytes;

    type Error = StdError;

    fn poll_frame(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        if self.done {
            return Poll::Ready(None);
        }
        let mut this = self.project();
        if let Some(initial_body) = this.initial_body.take() {
            cx.waker().wake_by_ref();
            return Poll::Ready(Some(Ok(Frame::data(initial_body))));
        }
        loop {
            if let Some(response) = &mut this.response {
                let frame = std::task::ready!(Pin::new(&mut response.body).poll_frame(cx)?);
                if let Some(frame) = frame {
                    return Poll::Ready(Some(Ok(frame)));
                }
                *this.done = true;
                return Poll::Ready(Some(Ok(Frame::trailers(std::mem::take(&mut response.headers)))));
            }
            match this.inner.as_mut().poll(cx) {
                Poll::Ready(response) => {
                    *this.response = Some(response);
                }
                Poll::Pending => match this.interval.poll_tick(cx) {
                    Poll::Ready(_) => return Poll::Ready(Some(Ok(Frame::data(Bytes::from_static(b" "))))),
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }
}
