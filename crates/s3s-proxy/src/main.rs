#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use s3s::auth::SimpleAuth;
use s3s::host::SingleDomain;
use s3s::service::S3ServiceBuilder;
use tokio::net::TcpListener;

use std::error::Error;
use std::io::IsTerminal;

use aws_credential_types::provider::ProvideCredentials;

use clap::Parser;
use tracing::info;

use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder as ConnBuilder;

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

    let env_filter = EnvFilter::from_default_env();
    let enable_color = std::io::stdout().is_terminal();

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(env_filter)
        .with_ansi(enable_color)
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    setup_tracing();
    let opt = Opt::parse();

    // Setup S3 provider
    let sdk_conf = aws_config::from_env().endpoint_url(&opt.endpoint_url).load().await;
    let client = aws_sdk_s3::Client::from_conf(aws_sdk_s3::config::Builder::from(&sdk_conf).force_path_style(true).build());
    let proxy = s3s_aws::Proxy::from(client);

    // Setup S3 service
    let service = {
        let mut b = S3ServiceBuilder::new(proxy);

        // Enable authentication
        if let Some(cred_provider) = sdk_conf.credentials_provider() {
            let cred = cred_provider.provide_credentials().await?;
            b.set_auth(SimpleAuth::from_single(cred.access_key_id(), cred.secret_access_key()));
        }

        // Enable parsing virtual-hosted-style requests
        if let Some(domain_name) = opt.domain_name {
            b.set_host(SingleDomain::new(domain_name));
        }

        b.build()
    };

    // Run server
    let listener = TcpListener::bind((opt.host.as_str(), opt.port)).await?;

    let hyper_service = service.into_shared();

    let http_server = ConnBuilder::new(TokioExecutor::new());
    let graceful = hyper_util::server::graceful::GracefulShutdown::new();

    let mut ctrl_c = std::pin::pin!(tokio::signal::ctrl_c());

    info!("server is running at http://{}:{}/", opt.host, opt.port);
    info!("server is forwarding requests to {}", opt.endpoint_url);

    loop {
        let (socket, _) = tokio::select! {
            res =  listener.accept() => {
                match res {
                    Ok(conn) => conn,
                    Err(err) => {
                        tracing::error!("error accepting connection: {err}");
                        continue;
                    }
                }
            }
            _ = ctrl_c.as_mut() => {
                break;
            }
        };

        let conn = http_server.serve_connection(TokioIo::new(socket), hyper_service.clone());
        let conn = graceful.watch(conn.into_owned());
        tokio::spawn(async move {
            let _ = conn.await;
        });
    }

    tokio::select! {
        () = graceful.shutdown() => {
             tracing::debug!("Gracefully shutdown!");
        },
        () = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
             tracing::debug!("Waited 10 seconds for graceful shutdown, aborting...");
        }
    }

    info!("server is stopped");

    Ok(())
}
