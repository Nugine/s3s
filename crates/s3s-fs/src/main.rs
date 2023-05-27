#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use s3s_fs::FileSystem;
use s3s_fs::Result;

use s3s::auth::SimpleAuth;
use s3s::service::S3ServiceBuilder;

use std::net::TcpListener;
use std::path::PathBuf;

use clap::Parser;
use hyper::server::Server;
use tracing::info;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(long, default_value = "localhost")]
    host: String,

    #[clap(long, default_value = "8014")]
    port: u16,

    #[clap(long, requires("secret-key"))]
    access_key: Option<String>,

    #[clap(long, requires("access-key"))]
    secret_key: Option<String>,

    #[clap(long)]
    domain_name: Option<String>,

    root: PathBuf,
}

fn setup_tracing() {
    use tracing_subscriber::EnvFilter;

    let env_filter = EnvFilter::from_default_env();
    // let enable_color = std::io::stdout().is_terminal(); // TODO
    let enable_color = false;

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(env_filter)
        .with_ansi(enable_color)
        .init();
}

#[tokio::main]
async fn main() -> Result {
    setup_tracing();
    let opt = Opt::parse();

    // Setup S3 provider
    let fs = FileSystem::new(opt.root)?;

    // Setup S3 service
    let service = {
        let mut b = S3ServiceBuilder::new(fs);

        // Enable authentication
        if let (Some(ak), Some(sk)) = (opt.access_key, opt.secret_key) {
            b.set_auth(SimpleAuth::from_single(ak, sk));
        }

        // Enable parsing virtual-hosted-style requests
        if let Some(domain_name) = opt.domain_name {
            b.set_base_domain(domain_name);
        }

        b.build()
    };

    // Run server
    let listener = TcpListener::bind((opt.host.as_str(), opt.port))?;
    let local_addr = listener.local_addr()?;

    let server = Server::from_tcp(listener)?.serve(service.into_shared().into_make_service());

    info!("server is running at http://{local_addr}");
    server.with_graceful_shutdown(shutdown_signal()).await?;

    info!("server is stopped");
    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
