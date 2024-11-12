use s3s_test::Result;

use std::fmt;

use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::error::SdkError;
use tracing::error;

#[macro_export]
macro_rules! case {
    ($tcx: expr, $s:ident, $x:ident, $c:ident) => {{
        let mut suite = $tcx.suite::<$s>(stringify!($s));
        let mut fixture = suite.fixture::<$x>(stringify!($x));
        fixture.case(stringify!($c), $x::$c);
    }};
}

pub fn check<T, E>(result: Result<T, SdkError<E>>, allowed_codes: &[&str]) -> Result<Option<T>, SdkError<E>>
where
    E: fmt::Debug + ProvideErrorMetadata,
{
    if let Err(SdkError::ServiceError(ref err)) = result {
        if let Some(code) = err.err().code() {
            if allowed_codes.contains(&code) {
                return Ok(None);
            }
        }
    }
    if let Err(ref err) = result {
        error!(?err);
    }
    match result {
        Ok(val) => Ok(Some(val)),
        Err(err) => Err(err),
    }
}

#[tracing::instrument(skip(s3))]
pub async fn create_bucket(s3: &aws_sdk_s3::Client, bucket: &str) -> Result {
    s3.create_bucket().bucket(bucket).send().await?;
    Ok(())
}

#[tracing::instrument(skip(s3))]
pub async fn delete_bucket_loose(s3: &aws_sdk_s3::Client, bucket: &str) -> Result {
    let result = s3.delete_bucket().bucket(bucket).send().await;
    check(result, &["NoSuchBucket"])?;
    Ok(())
}

#[tracing::instrument(skip(s3))]
pub async fn delete_bucket_strict(s3: &aws_sdk_s3::Client, bucket: &str) -> Result {
    s3.delete_bucket().bucket(bucket).send().await?;
    Ok(())
}

#[tracing::instrument(skip(s3))]
pub async fn delete_object_loose(s3: &aws_sdk_s3::Client, bucket: &str, key: &str) -> Result {
    let result = s3.delete_object().bucket(bucket).key(key).send().await;
    check(result, &["NoSuchKey", "NoSuchBucket"])?;
    Ok(())
}

#[tracing::instrument(skip(s3))]
pub async fn delete_object_strict(s3: &aws_sdk_s3::Client, bucket: &str, key: &str) -> Result {
    s3.delete_object().bucket(bucket).key(key).send().await?;
    Ok(())
}
