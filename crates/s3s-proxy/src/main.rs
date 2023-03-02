#![forbid(unsafe_code)]
#![deny(clippy::all)]

use s3s::auth::SimpleAuth;
use s3s::service::S3Service;

use std::error::Error;
use std::net::TcpListener;

use aws_credential_types::provider::ProvideCredentials;

use clap::Parser;
use hyper::server::Server;
use tracing::info;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(long, default_value = "localhost")]
    host: String,

    #[clap(long, default_value = "8014")]
    port: u16,

    #[clap(long)]
    domain_name: Option<String>,

    #[clap(long)]
    endpoint_url: String,
}

fn setup_tracing() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    setup_tracing();
    let opt = Opt::parse();

    // Setup S3 service
    let conf = aws_config::from_env().endpoint_url(&opt.endpoint_url).load().await;
    let proxy = s3s_aws::Proxy::from(aws_sdk_s3::Client::new(&conf));
    let mut service = S3Service::new(Box::new(proxy));

    // Enable authentication
    if let Some(cred_provider) = conf.credentials_provider() {
        let cred = cred_provider.provide_credentials().await?;
        let auth = SimpleAuth::from_single(cred.access_key_id(), cred.secret_access_key());
        service.set_auth(Box::new(auth));
    }

    // Enable parsing virtual-hosted-style requests
    if let Some(domain_name) = opt.domain_name {
        service.set_base_domain(domain_name);
    }

    // Run server
    let listener = TcpListener::bind((opt.host.as_str(), opt.port))?;
    let server = Server::from_tcp(listener)?.serve(service.into_shared().into_make_service());

    info!("server is running at http://{}:{}/", opt.host, opt.port);
    info!("server is forwarding requests to {}", opt.endpoint_url);
    let task = tokio::spawn(server);

    tokio::signal::ctrl_c().await?;
    task.abort();

    Ok(())
}
