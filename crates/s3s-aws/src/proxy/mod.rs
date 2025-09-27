#[cfg(feature = "minio")]
mod generated_minio;

#[cfg(not(feature = "minio"))]
mod generated;

mod meta;

pub struct Proxy(aws_sdk_s3::Client);

impl From<aws_sdk_s3::Client> for Proxy {
    fn from(value: aws_sdk_s3::Client) -> Self {
        Self(value)
    }
}
