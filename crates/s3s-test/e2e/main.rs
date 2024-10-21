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
use std::process::Termination;
use std::sync::Arc;

use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::error::SdkError;
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

#[tracing::instrument(skip(s3))]
async fn delete_bucket(s3: &aws_sdk_s3::Client, bucket: &str) -> Result<()> {
    let result = s3.delete_bucket().bucket(bucket).send().await;
    check(result, &["NoSuchBucket"])?;
    Ok(())
}

#[tracing::instrument(skip(s3))]
async fn create_bucket(s3: &aws_sdk_s3::Client, bucket: &str) -> Result<()> {
    s3.create_bucket().bucket(bucket).send().await?;
    Ok(())
}

struct E2E {
    s3: aws_sdk_s3::Client,
}

impl TestSuite for E2E {
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

struct Basic {
    s3: aws_sdk_s3::Client,
}

impl TestFixture<E2E> for Basic {
    async fn setup(suite: Arc<E2E>) -> Result<Self> {
        Ok(Self { s3: suite.s3.clone() })
    }
}

impl Basic {
    async fn test_list_buckets(self: Arc<Self>) -> Result<()> {
        let s3 = &self.s3;

        let buckets = ["test-list-buckets-1", "test-list-buckets-2"];
        for &bucket in &buckets {
            delete_bucket(s3, bucket).await?;
        }

        for &bucket in &buckets {
            create_bucket(s3, bucket).await?;
        }

        let resp = s3.list_buckets().send().await?;
        let bucket_list: Vec<_> = resp.buckets.as_deref().unwrap().iter().filter_map(|b| b.name()).collect();

        for &bucket in &buckets {
            assert!(bucket_list.contains(&bucket));
        }

        for &bucket in &buckets {
            delete_bucket(s3, bucket).await?;
        }

        Ok(())
    }
}

fn main() -> impl Termination {
    s3s_test::cli::main(|tcx| {
        macro_rules! case {
            ($s:ident, $x:ident, $c:ident) => {{
                let mut suite = tcx.suite::<$s>(stringify!($s));
                let mut fixture = suite.fixture::<$x>(stringify!($x));
                fixture.case(stringify!($c), $x::$c);
            }};
        }

        case!(E2E, Basic, test_list_buckets);
    })
}
