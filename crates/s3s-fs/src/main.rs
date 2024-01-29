#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use s3s_fs::FileSystem;
use s3s_fs::Result;

use s3s::auth::SimpleAuth;
use s3s::service::S3ServiceBuilder;

use std::io::IsTerminal;
use std::path::PathBuf;

use tokio::net::TcpListener;

use clap::{CommandFactory, Parser};
use tracing::info;

use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder as ConnBuilder;

#[derive(Debug, Parser)]
#[command(version)]
struct Opt {
    /// Host name to listen on.
    #[arg(long, default_value = "localhost")]
    host: String,

    /// Port number to listen on.
    #[arg(long, default_value = "8014")] // The original design was finished on 2020-08-14.
    port: u16,

    /// Access key used for authentication.
    #[arg(long)]
    access_key: Option<String>,

    /// Secret key used for authentication.
    #[arg(long)]
    secret_key: Option<String>,

    /// Domain name used for virtual-hosted-style requests.
    #[arg(long)]
    domain_name: Option<String>,

    /// Root directory of stored data.
    root: PathBuf,
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

fn check_cli_args(opt: &Opt) {
    use clap::error::ErrorKind;

    let mut cmd = Opt::command();

    // TODO: how to specify the requirements with clap derive API?
    if let (Some(_), None) | (None, Some(_)) = (&opt.access_key, &opt.secret_key) {
        let msg = "access key and secret key must be specified together";
        cmd.error(ErrorKind::MissingRequiredArgument, msg).exit();
    }

    if let Some(ref s) = opt.domain_name {
        if s.contains('/') {
            let msg = format!("expected domain name, found URL-like string: {s:?}");
            cmd.error(ErrorKind::InvalidValue, msg).exit();
        }
    }
}

fn main() -> Result {
    let opt = Opt::parse();
    check_cli_args(&opt);

    setup_tracing();

    run(opt)
}

#[tokio::main]
async fn run(opt: Opt) -> Result {
    // Setup S3 provider
    let fs = FileSystem::new(opt.root)?;

    // Setup S3 service
    let service = {
        let mut b = S3ServiceBuilder::new(fs);

        // Enable authentication
        if let (Some(ak), Some(sk)) = (opt.access_key, opt.secret_key) {
            b.set_auth(SimpleAuth::from_single(ak, sk));
            info!("authentication is enabled");
        }

        // Enable parsing virtual-hosted-style requests
        if let Some(domain_name) = opt.domain_name {
            b.set_base_domain(domain_name);
            info!("virtual-hosted-style requests are enabled");
        }

        b.build()
    };

    // Run server
    let listener = TcpListener::bind((opt.host.as_str(), opt.port)).await?;
    let local_addr = listener.local_addr()?;

    let hyper_service = service.into_shared();

    let connection = ConnBuilder::new(TokioExecutor::new());

    let server = async move {
        loop {
            let (socket, _) = match listener.accept().await {
                Ok(ok) => ok,
                Err(err) => {
                    tracing::error!("error accepting connection: {err}");
                    continue;
                }
            };
            let service = hyper_service.clone();
            let conn = connection.clone();
            tokio::spawn(async move {
                let _ = conn.serve_connection(TokioIo::new(socket), service).await;
            });
        }
    };

    let task = tokio::spawn(server);
    info!("server is running at http://{local_addr}");

    tokio::signal::ctrl_c().await?;
    task.abort();

    info!("server is stopped");
    Ok(())
}
