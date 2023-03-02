#![forbid(unsafe_code)]
#![deny(
    clippy::all, //
    clippy::must_use_candidate, //
)]

use s3s::service::S3Service;

use std::env;
use std::fs;

use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::model::*;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::Client;
use aws_sdk_s3::Credentials;
use aws_sdk_s3::Region;

use anyhow::Result;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;
use tracing::{debug, error};
use uuid::Uuid;

const FS_ROOT: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/s3s-fs-tests-aws");
const DOMAIN_NAME: &str = "localhost:8014";
const REGION: &str = "us-west-2";

fn setup_tracing() {
    use tracing_subscriber::EnvFilter;

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "it_aws=debug,s3s_fs=debug,s3s=debug");
    }

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .with_test_writer()
        .init()
}

fn config() -> &'static SdkConfig {
    static CONFIG: Lazy<SdkConfig> = Lazy::new(|| {
        setup_tracing();

        let cred = Credentials::for_tests();

        let conn = {
            fs::create_dir_all(FS_ROOT).unwrap();
            let fs = s3s_fs::FileSystem::new(FS_ROOT).unwrap();

            let auth = s3s::auth::SimpleAuth::from_single(cred.access_key_id(), cred.secret_access_key());

            let mut service = S3Service::new(Box::new(fs));
            service.set_auth(Box::new(auth));
            service.set_base_domain(DOMAIN_NAME);

            s3s_aws::Connector::from(service.into_shared())
        };

        SdkConfig::builder()
            .credentials_provider(SharedCredentialsProvider::new(cred))
            .http_connector(conn)
            .region(Region::new(REGION))
            .endpoint_url(format!("http://{DOMAIN_NAME}"))
            .build()
    });
    &CONFIG
}

async fn serial() -> MutexGuard<'static, ()> {
    static LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
    LOCK.lock().await
}

async fn create_bucket(c: &Client, bucket: &str) -> Result<()> {
    let location = BucketLocationConstraint::from(REGION);
    let cfg = CreateBucketConfiguration::builder().location_constraint(location).build();

    c.create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket)
        .send()
        .await?;

    debug!("created bucket: {bucket:?}");
    Ok(())
}

async fn delete_object(c: &Client, bucket: &str, key: &str) -> Result<()> {
    c.delete_object().bucket(bucket).key(key).send().await?;
    Ok(())
}

async fn delete_bucket(c: &Client, bucket: &str) -> Result<()> {
    c.delete_bucket().bucket(bucket).send().await?;
    Ok(())
}

#[tokio::test]
#[tracing::instrument]
async fn test_list_buckets() -> Result<()> {
    let c = Client::new(config());
    let result = c.list_buckets().send().await;

    match result {
        Ok(ref ans) => debug!(?ans),
        Err(ref err) => error!(?err),
    }

    result?;
    Ok(())
}

#[tokio::test]
#[tracing::instrument]
async fn test_single_object() -> Result<()> {
    let _guard = serial().await;

    let c = Client::new(config());
    let bucket = format!("test-single-object-{}", Uuid::new_v4());
    let bucket = bucket.as_str();
    let key = "sample.txt";
    let content = "hello world\n你好世界\n";

    create_bucket(&c, bucket).await?;

    {
        let body = ByteStream::from_static(content.as_bytes());
        c.put_object().bucket(bucket).key(key).body(body).send().await?;
    }

    {
        let ans = c.get_object().bucket(bucket).key(key).send().await?;

        let content_length: usize = ans.content_length().try_into().unwrap();
        let body = ans.body.collect().await?.into_bytes();

        assert_eq!(content_length, content.len());
        assert_eq!(body.as_ref(), content.as_bytes());
    }

    {
        delete_object(&c, bucket, key).await?;
        delete_bucket(&c, bucket).await?;
    }

    Ok(())
}

#[tokio::test]
#[tracing::instrument]
async fn test_multipart() -> Result<()> {
    let _guard = serial().await;

    let c = Client::new(config());

    let bucket = format!("test-multipart-{}", Uuid::new_v4());
    let bucket = bucket.as_str();
    create_bucket(&c, bucket).await?;

    let key = "sample.txt";
    let content = "abcdefghijklmnopqrstuvwxyz/0123456789/!@#$%^&*();\n";

    let upload_id = {
        let ans = c.create_multipart_upload().bucket(bucket).key(key).send().await?;
        ans.upload_id.unwrap()
    };
    let upload_id = upload_id.as_str();

    let upload_parts = {
        let body = ByteStream::from_static(content.as_bytes());
        let part_number = 1;

        let ans = c
            .upload_part()
            .bucket(bucket)
            .key(key)
            .upload_id(upload_id)
            .body(body)
            .part_number(part_number)
            .send()
            .await?;

        let part = CompletedPart::builder()
            .e_tag(ans.e_tag.unwrap_or_default())
            .part_number(part_number)
            .build();

        vec![part]
    };

    {
        let upload = CompletedMultipartUpload::builder().set_parts(Some(upload_parts)).build();

        let _ = c
            .complete_multipart_upload()
            .bucket(bucket)
            .key(key)
            .multipart_upload(upload)
            .upload_id(upload_id)
            .send()
            .await?;
    }

    {
        let ans = c.get_object().bucket(bucket).key(key).send().await?;

        let content_length: usize = ans.content_length().try_into().unwrap();
        let body = ans.body.collect().await?.into_bytes();

        assert_eq!(content_length, content.len());
        assert_eq!(body.as_ref(), content.as_bytes());
    }

    {
        delete_object(&c, bucket, key).await?;
        delete_bucket(&c, bucket).await?;
    }

    Ok(())
}
