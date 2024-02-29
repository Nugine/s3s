#![forbid(unsafe_code)]
#![deny(
    clippy::all, //
    clippy::must_use_candidate, //
)]

use s3s::auth::SimpleAuth;
use s3s::service::S3ServiceBuilder;
use s3s_fs::FileSystem;

use std::env;
use std::fs;

use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;

use aws_sdk_s3::types::BucketLocationConstraint;
use aws_sdk_s3::types::ChecksumMode;
use aws_sdk_s3::types::CompletedMultipartUpload;
use aws_sdk_s3::types::CompletedPart;
use aws_sdk_s3::types::CreateBucketConfiguration;

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

        // Fake credentials
        let cred = Credentials::for_tests();

        // Setup S3 provider
        fs::create_dir_all(FS_ROOT).unwrap();
        let fs = FileSystem::new(FS_ROOT).unwrap();

        // Setup S3 service
        let service = {
            let mut b = S3ServiceBuilder::new(fs);
            b.set_auth(SimpleAuth::from_single(cred.access_key_id(), cred.secret_access_key()));
            b.set_base_domain(DOMAIN_NAME);
            b.build()
        };

        // Convert to aws http connector
        let conn = s3s_aws::Connector::from(service.into_shared());

        // Setup aws sdk config
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

macro_rules! log_and_unwrap {
    ($result:expr) => {
        match $result {
            Ok(ans) => {
                debug!(?ans);
                ans
            }
            Err(err) => {
                error!(?err);
                return Err(err.into());
            }
        }
    };
}

#[tokio::test]
#[tracing::instrument]
async fn test_list_buckets() -> Result<()> {
    let c = Client::new(config());
    let response1 = log_and_unwrap!(c.list_buckets().send().await);
    assert!(response1.buckets().is_empty());

    let bucket1 = format!("test-list-buckets-1-{}", Uuid::new_v4());
    let bucket1_str = bucket1.as_str();
    let bucket2 = format!("test-list-buckets-2-{}", Uuid::new_v4());
    let bucket2_str = bucket2.as_str();

    create_bucket(&c, bucket1_str).await?;
    create_bucket(&c, bucket2_str).await?;

    let response2 = log_and_unwrap!(c.list_buckets().send().await);
    let bucket_names: Vec<_> = response2.buckets().iter().filter_map(|bucket| bucket.name()).collect();
    assert!(bucket_names.contains(&bucket1_str));
    assert!(bucket_names.contains(&bucket2_str));

    Ok(())
}

#[tokio::test]
#[tracing::instrument]
async fn test_list_objects_v2() -> Result<()> {
    let c = Client::new(config());
    let bucket = format!("test-list-objects-v2-{}", Uuid::new_v4());
    let bucket_str = bucket.as_str();
    create_bucket(&c, bucket_str).await?;

    let test_prefix = "/this/is/a/test/";
    let key1 = "this/is/a/test/path/file1.txt";
    let key2 = "this/is/a/test/path/file2.txt";
    {
        let content = "hello world\nनमस्ते दुनिया\n";
        let crc32c = base64_simd::STANDARD.encode_to_string(crc32c::crc32c(content.as_bytes()).to_be_bytes());
        c.put_object()
            .bucket(bucket_str)
            .key(key1)
            .body(ByteStream::from_static(content.as_bytes()))
            .checksum_crc32_c(crc32c.as_str())
            .send()
            .await?;
        c.put_object()
            .bucket(bucket_str)
            .key(key2)
            .body(ByteStream::from_static(content.as_bytes()))
            .checksum_crc32_c(crc32c.as_str())
            .send()
            .await?;
    }

    let result = c.list_objects_v2().bucket(bucket_str).prefix(test_prefix).send().await;

    let response = log_and_unwrap!(result);

    let contents: Vec<_> = response.contents().iter().filter_map(|obj| obj.key()).collect();
    assert!(!contents.is_empty());
    assert!(contents.contains(&key1));
    assert!(contents.contains(&key2));

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
    let crc32c = base64_simd::STANDARD.encode_to_string(crc32c::crc32c(content.as_bytes()).to_be_bytes());

    create_bucket(&c, bucket).await?;

    {
        let body = ByteStream::from_static(content.as_bytes());
        c.put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .checksum_crc32_c(crc32c.as_str())
            .send()
            .await?;
    }

    {
        let ans = c
            .get_object()
            .bucket(bucket)
            .key(key)
            .checksum_mode(ChecksumMode::Enabled)
            .send()
            .await?;

        let content_length: usize = ans.content_length().unwrap().try_into().unwrap();
        let checksum_crc32c = ans.checksum_crc32_c.unwrap();
        let body = ans.body.collect().await?.into_bytes();

        assert_eq!(content_length, content.len());
        assert_eq!(checksum_crc32c, crc32c);
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

        let content_length: usize = ans.content_length().unwrap().try_into().unwrap();
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
async fn test_upload_part_copy() -> Result<()> {
    let _guard = serial().await;

    let c = Client::new(config());
    let src_bucket = format!("test-copy{}", Uuid::new_v4());
    let src_bucket = src_bucket.as_str();
    let src_key = "copied.txt";
    let src_content = "hello world\nनमस्ते दुनिया\n";
    let crc32c = base64_simd::STANDARD.encode_to_string(crc32c::crc32c(src_content.as_bytes()).to_be_bytes());

    create_bucket(&c, src_bucket).await?;

    {
        let src_body = ByteStream::from_static(src_content.as_bytes());
        c.put_object()
            .bucket(src_bucket)
            .key(src_key)
            .body(src_body)
            .checksum_crc32_c(crc32c.as_str())
            .send()
            .await?;
    }

    let bucket = format!("test-uploadpartcopy-{}", Uuid::new_v4());
    let bucket = bucket.as_str();
    create_bucket(&c, bucket).await?;

    let key = "sample.txt";

    let upload_id = {
        let ans = c.create_multipart_upload().bucket(bucket).key(key).send().await?;
        ans.upload_id.unwrap()
    };
    let upload_id = upload_id.as_str();
    let src_path = format!("{src_bucket}/{src_key}");
    let upload_parts = {
        let part_number = 1;
        let _ans = c
            .upload_part_copy()
            .bucket(bucket)
            .key(key)
            .copy_source(src_path)
            .upload_id(upload_id)
            .part_number(part_number)
            .send()
            .await?;
        let part = CompletedPart::builder().part_number(part_number).build();
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

        let content_length: usize = ans.content_length().unwrap().try_into().unwrap();
        let body = ans.body.collect().await?.into_bytes();

        assert_eq!(content_length, src_content.len());
        assert_eq!(body.as_ref(), src_content.as_bytes());
    }
    println!("{} CK3", key);
    {
        delete_object(&c, bucket, key).await?;
        delete_bucket(&c, bucket).await?;
        delete_object(&c, src_bucket, src_key).await?;
        delete_bucket(&c, src_bucket).await?;
    }

    Ok(())
}
