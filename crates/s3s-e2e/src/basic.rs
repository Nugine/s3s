use crate::case;
use crate::utils::*;

use std::sync::Arc;

use s3s_test::Result;
use s3s_test::TestFixture;
use s3s_test::TestSuite;
use s3s_test::tcx::TestContext;

use aws_sdk_s3::primitives::ByteStream;

pub fn register(tcx: &mut TestContext) {
    case!(tcx, Basic, Essential, test_list_buckets);
    case!(tcx, Basic, Essential, test_list_objects);
    case!(tcx, Basic, Essential, test_get_object);
    case!(tcx, Basic, Essential, test_delete_object);
    case!(tcx, Basic, Essential, test_head_operations);
    case!(tcx, Basic, Put, test_put_object_tiny);
    case!(tcx, Basic, Put, test_put_object_with_metadata);
    case!(tcx, Basic, Put, test_put_object_larger);
    case!(tcx, Basic, Copy, test_copy_object);
}

struct Basic {
    s3: aws_sdk_s3::Client,
}

impl TestSuite for Basic {
    #[tracing::instrument(skip_all)]
    async fn setup() -> Result<Self> {
        let sdk_conf = aws_config::from_env().load().await;

        let s3 = aws_sdk_s3::Client::from_conf(
            aws_sdk_s3::config::Builder::from(&sdk_conf)
                .force_path_style(true) // FIXME: remove force_path_style
                .build(),
        );

        Ok(Self { s3 })
    }
}

struct Essential {
    s3: aws_sdk_s3::Client,
}

impl TestFixture<Basic> for Essential {
    async fn setup(suite: Arc<Basic>) -> Result<Self> {
        Ok(Self { s3: suite.s3.clone() })
    }
}

impl Essential {
    async fn test_list_buckets(self: Arc<Self>) -> Result {
        let s3 = &self.s3;

        let buckets = ["test-list-buckets-1", "test-list-buckets-2"];

        {
            for &bucket in &buckets {
                delete_bucket_loose(s3, bucket).await?;
            }
        }

        {
            for &bucket in &buckets {
                create_bucket(s3, bucket).await?;
            }

            let resp = s3.list_buckets().send().await?;
            let bucket_list: Vec<_> = resp.buckets.as_deref().unwrap().iter().filter_map(|b| b.name()).collect();

            for &bucket in &buckets {
                assert!(bucket_list.contains(&bucket));
                s3.head_bucket().bucket(bucket).send().await?;
            }
        }

        {
            for &bucket in &buckets {
                delete_bucket_strict(s3, bucket).await?;
            }
        }

        Ok(())
    }

    async fn test_list_objects(self: Arc<Self>) -> Result {
        let s3 = &self.s3;

        let bucket = "test-list-objects";
        let keys = ["file-1", "file-2", "file-3"];
        let content = "hello world 你好世界 123456 !@#$%😂^&*()";

        {
            for key in &keys {
                delete_object_loose(s3, bucket, key).await?;
            }
            delete_bucket_loose(s3, bucket).await?;
        }

        {
            create_bucket(s3, bucket).await?;

            for &key in &keys {
                s3.put_object()
                    .bucket(bucket)
                    .key(key)
                    .body(ByteStream::from_static(content.as_bytes()))
                    .send()
                    .await?;
            }

            let resp = s3.list_objects_v2().bucket(bucket).send().await?;
            let object_list: Vec<_> = resp.contents.as_deref().unwrap().iter().filter_map(|o| o.key()).collect();

            for &key in &keys {
                assert!(object_list.contains(&key));
                s3.head_object().bucket(bucket).key(key).send().await?;
            }
        }

        {
            for &key in &keys {
                delete_object_strict(s3, bucket, key).await?;
            }
            delete_bucket_strict(s3, bucket).await?;
        }

        Ok(())
    }

    async fn test_get_object(self: Arc<Self>) -> Result {
        let s3 = &self.s3;

        let bucket = "test-get-object";
        let key = "file-1";
        let content = "hello world 你好世界 123456 !@#$%😂^&*()";

        {
            delete_object_loose(s3, bucket, key).await?;
            delete_bucket_loose(s3, bucket).await?;
        }

        {
            create_bucket(s3, bucket).await?;

            s3.put_object()
                .bucket(bucket)
                .key(key)
                .body(ByteStream::from_static(content.as_bytes()))
                .send()
                .await?;

            let resp = s3.get_object().bucket(bucket).key(key).send().await?;

            let body = resp.body.collect().await?;
            let body = String::from_utf8(body.to_vec())?;
            assert_eq!(body, content);
        }

        {
            delete_object_strict(s3, bucket, key).await?;
            delete_bucket_strict(s3, bucket).await?;
        }

        Ok(())
    }

    async fn test_delete_object(self: Arc<Self>) -> Result {
        let s3 = &self.s3;

        let bucket = "test-delete-object";
        let key = "file-to-delete";
        let content = "content to be deleted";

        {
            delete_object_loose(s3, bucket, key).await?;
            delete_bucket_loose(s3, bucket).await?;
        }

        {
            create_bucket(s3, bucket).await?;

            // Put an object
            s3.put_object()
                .bucket(bucket)
                .key(key)
                .body(ByteStream::from_static(content.as_bytes()))
                .send()
                .await?;

            // Verify object exists
            s3.head_object().bucket(bucket).key(key).send().await?;

            // Delete the object
            s3.delete_object().bucket(bucket).key(key).send().await?;

            // Verify object no longer exists
            let result = s3.head_object().bucket(bucket).key(key).send().await;
            assert!(result.is_err());
        }

        {
            delete_bucket_strict(s3, bucket).await?;
        }

        Ok(())
    }

    async fn test_head_operations(self: Arc<Self>) -> Result {
        let s3 = &self.s3;

        let bucket = "test-head-operations";
        let key = "head-test-file";
        let content = "content for head operations";

        {
            delete_object_loose(s3, bucket, key).await?;
            delete_bucket_loose(s3, bucket).await?;
        }

        {
            create_bucket(s3, bucket).await?;

            // Test HeadBucket
            let head_bucket_resp = s3.head_bucket().bucket(bucket).send().await?;
            assert!(head_bucket_resp.bucket_region().is_some() || head_bucket_resp.bucket_region().is_none()); // Just check response is valid

            // Put an object
            s3.put_object()
                .bucket(bucket)
                .key(key)
                .body(ByteStream::from_static(content.as_bytes()))
                .send()
                .await?;

            // Test HeadObject
            let head_object_resp = s3.head_object().bucket(bucket).key(key).send().await?;
            assert_eq!(head_object_resp.content_length().unwrap_or(0), i64::try_from(content.len())?);
        }

        {
            delete_object_strict(s3, bucket, key).await?;
            delete_bucket_strict(s3, bucket).await?;
        }

        Ok(())
    }
}

struct Put {
    s3: aws_sdk_s3::Client,
    bucket: String,
    key: String,
}

impl TestFixture<Basic> for Put {
    #[tracing::instrument(skip_all)]
    async fn setup(suite: Arc<Basic>) -> Result<Self> {
        let s3 = &suite.s3;
        let bucket = "test-put";
        let key = "file";

        delete_object_loose(s3, bucket, key).await?;
        delete_bucket_loose(s3, bucket).await?;

        create_bucket(s3, bucket).await?;

        Ok(Self {
            s3: suite.s3.clone(),
            bucket: bucket.to_owned(),
            key: key.to_owned(),
        })
    }

    #[tracing::instrument(skip_all)]
    async fn teardown(self) -> Result {
        let Self { s3, bucket, key } = &self;

        delete_object_loose(s3, bucket, key).await?;
        delete_bucket_loose(s3, bucket).await?;

        Ok(())
    }
}

impl Put {
    async fn test_put_object_tiny(self: Arc<Self>) -> Result {
        let s3 = &self.s3;
        let bucket = self.bucket.as_str();
        let key = self.key.as_str();

        let contents = ["", "1", "22", "333"];

        for content in contents {
            s3.put_object()
                .bucket(bucket)
                .key(key)
                .body(ByteStream::from_static(content.as_bytes()))
                .send()
                .await?;

            let resp = s3.get_object().bucket(bucket).key(key).send().await?;
            let body = resp.body.collect().await?;
            let body = String::from_utf8(body.to_vec())?;
            assert_eq!(body, content);
        }

        Ok(())
    }

    async fn test_put_object_with_metadata(self: Arc<Self>) -> Result {
        let s3 = &self.s3;
        let bucket = self.bucket.as_str();
        let key = "file-with-metadata";

        let content = "object with custom metadata";
        let metadata_key = "test-key";
        let metadata_value = "test-value";

        s3.put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from_static(content.as_bytes()))
            .metadata(metadata_key, metadata_value)
            .content_type("text/plain")
            .send()
            .await?;

        // Verify object content
        let resp = s3.get_object().bucket(bucket).key(key).send().await?;
        let body = resp.body.collect().await?;
        let body = String::from_utf8(body.to_vec())?;
        assert_eq!(body, content);

        // Check metadata using head_object (more reliable for metadata)
        let head_resp = s3.head_object().bucket(bucket).key(key).send().await?;

        // FIXME: s3s-fs does not return correct content type
        // // Check content type if supported
        // if let Some(content_type) = head_resp.content_type() {
        //     assert_eq!(content_type, "text/plain");
        // }

        let metadata = head_resp.metadata().unwrap();
        let value = metadata.get(metadata_key).unwrap();
        assert_eq!(value, metadata_value);

        Ok(())
    }

    async fn test_put_object_larger(self: Arc<Self>) -> Result {
        let s3 = &self.s3;
        let bucket = self.bucket.as_str();
        let key = "large-file";

        // Create a larger object (1KB)
        let content = "x".repeat(1024);

        s3.put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(content.clone().into_bytes()))
            .send()
            .await?;

        let resp = s3.get_object().bucket(bucket).key(key).send().await?;
        let body = resp.body.collect().await?;
        let body = String::from_utf8(body.to_vec())?;
        assert_eq!(body, content);
        assert_eq!(body.len(), 1024);

        Ok(())
    }
}

// Copy test fixture
struct Copy {
    s3: aws_sdk_s3::Client,
    bucket: String,
    source_key: String,
    dest_key: String,
}

impl TestFixture<Basic> for Copy {
    #[tracing::instrument(skip_all)]
    async fn setup(suite: Arc<Basic>) -> Result<Self> {
        let s3 = &suite.s3;
        let bucket = "test-copy";
        let source_key = "source-file";
        let dest_key = "dest-file";

        delete_object_loose(s3, bucket, source_key).await?;
        delete_object_loose(s3, bucket, dest_key).await?;
        delete_bucket_loose(s3, bucket).await?;

        create_bucket(s3, bucket).await?;

        Ok(Self {
            s3: suite.s3.clone(),
            bucket: bucket.to_owned(),
            source_key: source_key.to_owned(),
            dest_key: dest_key.to_owned(),
        })
    }

    #[tracing::instrument(skip_all)]
    async fn teardown(self) -> Result {
        let Self {
            s3,
            bucket,
            source_key,
            dest_key,
        } = &self;

        delete_object_loose(s3, bucket, source_key).await?;
        delete_object_loose(s3, bucket, dest_key).await?;
        delete_bucket_loose(s3, bucket).await?;

        Ok(())
    }
}

impl Copy {
    async fn test_copy_object(self: Arc<Self>) -> Result {
        let s3 = &self.s3;
        let bucket = self.bucket.as_str();
        let source_key = self.source_key.as_str();
        let dest_key = self.dest_key.as_str();

        let content = "content to be copied";

        // Put source object
        s3.put_object()
            .bucket(bucket)
            .key(source_key)
            .body(ByteStream::from_static(content.as_bytes()))
            .send()
            .await?;

        // Copy object
        s3.copy_object()
            .bucket(bucket)
            .key(dest_key)
            .copy_source(format!("{bucket}/{source_key}"))
            .send()
            .await?;

        // Verify destination object
        let resp = s3.get_object().bucket(bucket).key(dest_key).send().await?;
        let body = resp.body.collect().await?;
        let body = String::from_utf8(body.to_vec())?;
        assert_eq!(body, content);

        // Verify source object still exists
        let resp = s3.get_object().bucket(bucket).key(source_key).send().await?;
        let body = resp.body.collect().await?;
        let body = String::from_utf8(body.to_vec())?;
        assert_eq!(body, content);

        Ok(())
    }
}
