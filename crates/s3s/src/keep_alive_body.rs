use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use bytes::Bytes;
use http_body::Body;
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
    }
}
impl<F> KeepAliveBody<F> {
    pub fn new(inner: F, interval: Duration) -> Self {
        Self {
            inner,
            initial_body: None,
            response: None,
            interval: tokio::time::interval(interval),
        }
    }

    pub fn with_initial_body(inner: F, initial_body: Bytes, interval: Duration) -> Self {
        Self {
            inner,
            initial_body: Some(initial_body),
            response: None,
            interval: tokio::time::interval(interval),
        }
    }
}

impl<F> Body for KeepAliveBody<F>
where
    F: Future<Output = Response>,
{
    type Data = Bytes;

    type Error = StdError;

    fn poll_data(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        let mut this = self.project();
        if let Some(initial_body) = this.initial_body.take() {
            cx.waker().wake_by_ref();
            return Poll::Ready(Some(Ok(initial_body)));
        }
        loop {
            if let Some(response) = this.response {
                return Pin::new(&mut response.body).poll_data(cx);
            }
            match this.inner.as_mut().poll(cx) {
                Poll::Ready(response) => {
                    *this.response = Some(response);
                }
                Poll::Pending => match this.interval.poll_tick(cx) {
                    Poll::Ready(_) => return Poll::Ready(Some(Ok(Bytes::from_static(b" ")))),
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }

    fn poll_trailers(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<Option<hyper::HeaderMap>, Self::Error>> {
        let this = self.project();

        if let Some(response) = this.response {
            Pin::new(&mut response.body).poll_trailers(cx)
        } else {
            Poll::Ready(Ok(None))
        }
    }
}
