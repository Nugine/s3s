#![forbid(unsafe_code)]
#![deny(
    clippy::all, //
    clippy::cargo, //
    clippy::pedantic, //
    clippy::self_named_module_files, //
)]
#![warn(
    clippy::dbg_macro, //
)]
#![allow(
    clippy::module_name_repetitions, //
    clippy::missing_errors_doc, // TODO
    clippy::missing_panics_doc, // TODO
    clippy::multiple_crate_versions, // TODO: check later
)]

use s3s_test::Result;
use s3s_test::TestFixture;
use s3s_test::TestSuite;

use std::fmt;
use std::sync::Arc;

use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::Client;
use tracing::error;

fn check<T, E>(result: Result<T, SdkError<E>>, allowed_codes: &[&str]) -> Result<Option<T>, SdkError<E>>
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

#[tracing::instrument(skip(c))]
async fn delete_bucket(c: &Client, bucket: &str) -> Result<()> {
    let result = c.delete_bucket().bucket(bucket).send().await;
    check(result, &["NoSuchBucket"])?;
    Ok(())
}

#[tracing::instrument(skip(c))]
async fn create_bucket(c: &Client, bucket: &str) -> Result<()> {
    c.create_bucket().bucket(bucket).send().await?;
    Ok(())
}

struct E2E {
    client: Client,
}

impl TestSuite for E2E {
    async fn setup() -> Result<Self> {
        let sdk_conf = aws_config::from_env().load().await;
        let s3_conf = aws_sdk_s3::config::Builder::from(&sdk_conf)
            .force_path_style(true) // FIXME: remove force_path_style
            .build();
        let client = Client::from_conf(s3_conf);
        Ok(Self { client })
    }
}

struct Basic {
    client: Client,
}

impl TestFixture<E2E> for Basic {
    async fn setup(suite: Arc<E2E>) -> Result<Self> {
        Ok(Self {
            client: suite.client.clone(),
        })
    }
}

impl Basic {
    async fn test_list_buckets(self: Arc<Self>) -> Result<()> {
        let c = &self.client;

        let buckets = ["test-list-buckets-1", "test-list-buckets-2"];
        for &bucket in &buckets {
            delete_bucket(c, bucket).await?;
        }

        for &bucket in &buckets {
            create_bucket(c, bucket).await?;
        }

        let resp = c.list_buckets().send().await?;
        let bucket_list: Vec<_> = resp.buckets.as_deref().unwrap().iter().filter_map(|b| b.name()).collect();

        for &bucket in &buckets {
            assert!(bucket_list.contains(&bucket));
        }

        for &bucket in &buckets {
            delete_bucket(c, bucket).await?;
        }

        Ok(())
    }
}

fn main() {
    s3s_test::cli::main(|tcx| {
        macro_rules! case {
            ($s:ident, $x:ident, $c:ident) => {{
                let mut suite = tcx.suite::<$s>(stringify!($s));
                let mut fixture = suite.fixture::<$x>(stringify!($x));
                fixture.case(stringify!($c), $x::$c);
            }};
        }

        case!(E2E, Basic, test_list_buckets);
    });
}
