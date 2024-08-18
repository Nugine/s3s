use crate::{http::Response, StdError};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use bytes::Bytes;
use http_body::{Body, Frame};
use tokio::time::Interval;

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
    pub fn new(inner: F, interval: Duration, initial_body: Option<Bytes>) -> Self {
        Self {
            inner,
            initial_body,
            response: None,
            interval: tokio::time::interval(interval),
            done: false,
        }
    }
}

impl<F> Body for KeepAliveBody<F>
where
    F: Future<Output = Result<Response, StdError>>,
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
                Poll::Ready(response) => match response {
                    Ok(response) => {
                        *this.response = Some(response);
                    }
                    Err(e) => {
                        *this.done = true;
                        return Poll::Ready(Some(Err(e)));
                    }
                },
                Poll::Pending => match this.interval.poll_tick(cx) {
                    Poll::Ready(_) => return Poll::Ready(Some(Ok(Frame::data(Bytes::from_static(b" "))))),
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }

    fn is_end_stream(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt;
    use hyper::{header::HeaderValue, StatusCode};

    use super::*;

    #[tokio::test]
    async fn keep_alive_body() {
        let body = KeepAliveBody::new(
            async {
                let mut res = Response::with_status(StatusCode::OK);
                res.body = Bytes::from_static(b" world").into();
                res.headers.insert("key", HeaderValue::from_static("value"));
                Ok(res)
            },
            Duration::from_secs(1),
            Some(Bytes::from_static(b"hello")),
        );

        let aggregated = body.collect().await.unwrap();

        assert_eq!(aggregated.trailers().unwrap().get("key").unwrap(), "value");

        let buf = aggregated.to_bytes();

        assert_eq!(buf, b"hello world".as_slice());
    }

    #[tokio::test]
    async fn keep_alive_body_no_initial() {
        let body = KeepAliveBody::new(
            async {
                let mut res = Response::with_status(StatusCode::OK);
                res.body = Bytes::from_static(b"hello world").into();
                Ok(res)
            },
            Duration::from_secs(1),
            None,
        );

        let aggregated = body.collect().await.unwrap();

        let buf = aggregated.to_bytes();

        assert_eq!(buf, b"hello world".as_slice());
    }

    #[tokio::test]
    async fn keep_alive_body_fill_withespace() {
        let body = KeepAliveBody::new(
            async {
                tokio::time::sleep(Duration::from_millis(450)).await;

                let mut res = Response::with_status(StatusCode::OK);
                res.body = Bytes::from_static(b"hello world").into();
                Ok(res)
            },
            Duration::from_millis(100),
            None,
        );

        let aggregated = body.collect().await.unwrap();

        let buf = aggregated.to_bytes();

        let ans1 = b"     hello world";
        let ans2 = b"    hello world";

        assert!(buf.as_ref() == ans1 || buf.as_ref() == ans2, "buf: {buf:?}");
    }
}
