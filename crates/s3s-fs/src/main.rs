#![forbid(unsafe_code)]
#![deny(clippy::all)]

use s3s_fs::FileSystem;
use s3s_fs::Result;

use s3s::service::S3Service;
use s3s::SimpleAuth;

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

    root: PathBuf,
}

fn setup_tracing() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init()
}

#[tokio::main]
async fn main() -> Result {
    setup_tracing();

    let opt = Opt::parse();

    let fs = FileSystem::new(opt.root)?;
    let mut service = S3Service::new(Box::new(fs));

    if let (Some(ak), Some(sk)) = (opt.access_key, opt.secret_key) {
        let mut auth = SimpleAuth::new();
        auth.register(ak, sk);
        service.set_auth(Box::new(auth));
    }

    let listener = TcpListener::bind((opt.host.as_str(), opt.port))?;
    let server = Server::from_tcp(listener)?.serve(service.into_shared().into_make_service());

    info!("server is running at http://{}:{}/", opt.host, opt.port);
    let task = tokio::spawn(server);

    tokio::signal::ctrl_c().await?;
    task.abort();

    Ok(())
}
