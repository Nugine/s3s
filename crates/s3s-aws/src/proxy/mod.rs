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
