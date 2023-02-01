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
            crate::error::SetStatusCode(&mut err, e).call();
        }
        err.set_source(Box::new(source));

        err
    }};
}

pub struct SetStatusCode<'a, 'b, E, R>(pub &'a mut s3s::S3Error, pub &'b aws_smithy_http::result::ServiceError<E, R>);

impl<'a, 'b, E> SetStatusCode<'a, 'b, E, aws_smithy_http::operation::Response> {
    pub fn call(self) {
        let Self(err, e) = self;
        err.set_status_code(e.raw().http().status());
    }
}

impl<'a, 'b, E> SetStatusCode<'a, 'b, E, aws_smithy_http::event_stream::RawMessage> {
    pub fn call(self) {}
}
