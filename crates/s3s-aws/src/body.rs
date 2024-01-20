use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use aws_smithy_types::body::SdkBody;

use futures::Stream;
use hyper::body::HttpBody;

pub fn s3s_body_into_sdk_body(body: s3s::Body) -> SdkBody {
    SdkBody::from_body_0_4(body.boxed())
}

pub fn sdk_body_into_s3s_body(body: SdkBody) -> s3s::Body {
    s3s::Body::from(Box::pin(Wrapper(body)) as s3s::stream::DynByteStream)
}

struct Wrapper(SdkBody);

impl Stream for Wrapper {
    type Item = Result<bytes::Bytes, s3s::StdError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        hyper::body::HttpBody::poll_data(Pin::new(&mut self.0), cx)
    }
}

impl s3s::stream::ByteStream for Wrapper {
    fn remaining_length(&self) -> s3s::stream::RemainingLength {
        hyper::body::HttpBody::size_hint(&self.0).into()
    }
}
