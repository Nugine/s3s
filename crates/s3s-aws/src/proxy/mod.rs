macro_rules! wrap_sdk_error {
    ($e:expr) => {{
        use aws_sdk_s3::types::SdkError;
        use s3s::{S3Error, S3ErrorCode};

        let mut err = S3Error::new(S3ErrorCode::InternalError);
        let source = $e;
        tracing::debug!("sdk error: {:?}", source);

        if let SdkError::ServiceError(ref e) = source {
            let meta = e.err().meta();
            if let Some(val) = meta.code().and_then(|s| S3ErrorCode::from_bytes(s.as_bytes())) {
                err.set_code(val);
            }
            if let Some(val) = meta.message() {
                err.set_message(val.to_owned());
            }
            if let Some(val) = meta.request_id() {
                err.set_request_id(val);
            }
            err.set_status_code(e.raw().http().status());
        }
        err.set_source(Box::new(source));

        err
    }};
}

mod generated;

pub struct Proxy(aws_sdk_s3::Client);

impl From<aws_sdk_s3::Client> for Proxy {
    fn from(value: aws_sdk_s3::Client) -> Self {
        Self(value)
    }
}

async fn transform_body(body: Option<s3s::dto::StreamingBlob>) -> aws_sdk_s3::types::ByteStream {
    use aws_smithy_http::body::SdkBody;
    use futures::Stream;

    const AGGREGATION_THRESHOLD: usize = 8 * 1024;

    match body {
        None => SdkBody::empty().into(),
        Some(stream) => {
            let can_aggregate = match stream.size_hint() {
                (_, Some(upper)) => upper <= AGGREGATION_THRESHOLD,
                _ => false,
            };
            if can_aggregate {
                return aggregate(stream).await.into();
            }
            SdkBody::from(hyper::Body::wrap_stream(stream.0)).into()
        }
    }
}

async fn aggregate(stream: s3s::dto::StreamingBlob) -> aws_smithy_http::body::SdkBody {
    use aws_smithy_http::body::SdkBody;
    use bytes::BufMut;
    use futures::TryStreamExt;
    use std::future::ready;

    let result: Result<Vec<bytes::Bytes>, _> = stream.try_collect().await;
    match result {
        Ok(buf) => {
            let mut vec: Vec<u8> = Vec::with_capacity(buf.iter().map(|b| b.len()).sum());
            buf.into_iter().for_each(|bytes| vec.put(bytes));
            tracing::debug!(len=?vec.len(), "aggregated body");
            SdkBody::from(vec)
        }
        Err(err) => {
            let stream = futures::stream::once(ready(<Result<bytes::Bytes, _>>::Err(err)));
            SdkBody::from(hyper::Body::wrap_stream(stream))
        }
    }
}
