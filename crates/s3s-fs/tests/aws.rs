#![forbid(unsafe_code)]
#![deny(
    clippy::all, //
    clippy::must_use_candidate, //
)]

use s3s::service::S3Service;

use std::fs;

use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::Client;
use aws_sdk_s3::Credentials;
use aws_sdk_s3::Region;

use once_cell::sync::Lazy;

const FS_ROOT: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/s3s-fs-tests-aws");
const DOMAIN_NAME: &str = "localhost:8014";
const REGION: &str = "us-west-2";

fn setup_tracing() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init()
}

fn config() -> &'static SdkConfig {
    static CONFIG: Lazy<SdkConfig> = Lazy::new(|| {
        setup_tracing();

        let cred = Credentials::for_tests();

        let conn = {
            fs::create_dir_all(FS_ROOT).unwrap();
            let fs = s3s_fs::FileSystem::new(FS_ROOT).unwrap();

            let auth = s3s::SimpleAuth::from_single(cred.access_key_id(), cred.secret_access_key());

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

#[tokio::test]
async fn list_buckets() {
    let client = Client::new(config());
    let result = client.list_buckets().send().await;

    match result {
        Ok(ans) => {
            println!("{ans:#?}");
        }
        Err(err) => {
            println!("{err:#?}");
            panic!();
        }
    }
}
