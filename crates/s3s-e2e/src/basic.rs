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
    case!(tcx, Basic, Put, test_put_object_tiny);
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
}
