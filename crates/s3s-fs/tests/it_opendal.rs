use s3s::auth::SimpleAuth;
use s3s::host::SingleDomain;
use s3s::service::S3ServiceBuilder;
use s3s_fs::FileSystem;

use std::env;
use std::fs;
use std::sync::Once;

use anyhow::Result;
use opendal::services::S3;
use opendal::Operator;
use futures_util::stream::StreamExt;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;
use tokio::net::TcpListener;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder as ConnBuilder;
use tracing::{debug, error};
use uuid::Uuid;

const FS_ROOT: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/s3s-fs-tests-opendal");
const DOMAIN_NAME: &str = "localhost:8015";
const REGION: &str = "us-west-2";
const ACCESS_KEY: &str = "AKIAIOSFODNN7EXAMPLE";
const SECRET_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";

static INIT: Once = Once::new();

fn setup_tracing() {
    use tracing_subscriber::EnvFilter;

    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(EnvFilter::from_default_env())
            .with_test_writer()
            .init();
    });
}

async fn start_server() {
    setup_tracing();

    // Setup S3 provider
    fs::create_dir_all(FS_ROOT).unwrap();
    let fs = FileSystem::new(FS_ROOT).unwrap();

    // Setup S3 service
    let service = {
        let mut b = S3ServiceBuilder::new(fs);
        b.set_auth(SimpleAuth::from_single(ACCESS_KEY, SECRET_KEY));
        b.set_host(SingleDomain::new(DOMAIN_NAME).unwrap());
        b.build()
    };

    // Start HTTP server
    let listener = TcpListener::bind(DOMAIN_NAME).await.unwrap();
    debug!("Server listening on {}", DOMAIN_NAME);

    tokio::spawn(async move {
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let service_clone = service.clone();
            
            // Create a new http_server instance for each connection
            let http_server = ConnBuilder::new(TokioExecutor::new());
            let conn = http_server.serve_connection(TokioIo::new(socket), service_clone).into_owned();
            tokio::spawn(async move {
                let _ = conn.await;
            });
        }
    });

    // Give the server a moment to start
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
}

fn create_operator() -> Operator {
    let builder = S3::default()
        .endpoint(&format!("http://{DOMAIN_NAME}"))
        .region(REGION)
        .access_key_id(ACCESS_KEY)
        .secret_access_key(SECRET_KEY)
        .bucket("test-bucket");

    Operator::new(builder).unwrap().finish()
}

async fn serial() -> MutexGuard<'static, ()> {
    use std::sync::LazyLock;
    static LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    LOCK.lock().await
}

macro_rules! log_and_unwrap {
    ($result:expr) => {
        match $result {
            Ok(ans) => {
                ans
            }
            Err(err) => {
                error!(?err);
                return Err(err.into());
            }
        }
    };
}

#[tokio::test]
#[tracing::instrument]
async fn test_operator_info() -> Result<()> {
    let _guard = serial().await;
    start_server().await;
    
    let op = create_operator();
    let info = op.info();
    
    debug!("Operator scheme: {:?}", info.scheme());
    debug!("Operator capabilities: {:?}", info.full_capability());
    
    // Basic smoke test - operator should be created successfully
    assert_eq!(info.scheme(), opendal::Scheme::S3);
    
    Ok(())
}

#[tokio::test]
#[tracing::instrument]
async fn test_write_and_read() -> Result<()> {
    let _guard = serial().await;
    start_server().await;
    
    let op = create_operator();
    let key = format!("test-write-read-{}", Uuid::new_v4());
    let content = "hello world\nनमस्ते दुनिया\n";

    // Write data
    log_and_unwrap!(op.write(&key, content).await);
    debug!("Written data to key: {key}");

    // Read data back
    let data = log_and_unwrap!(op.read(&key).await);
    let data_vec = data.to_vec();
    let read_content = std::str::from_utf8(&data_vec)?;
    
    assert_eq!(read_content, content);
    debug!("Read data matches written data");

    // Clean up
    log_and_unwrap!(op.delete(&key).await);
    debug!("Deleted key: {key}");

    Ok(())
}

#[tokio::test]
#[tracing::instrument]
async fn test_stat() -> Result<()> {
    let _guard = serial().await;
    start_server().await;
    
    let op = create_operator();
    let key = format!("test-stat-{}", Uuid::new_v4());
    let content = "test content for stat";

    // Write data
    log_and_unwrap!(op.write(&key, content).await);
    
    // Get metadata
    let metadata = log_and_unwrap!(op.stat(&key).await);
    
    assert_eq!(metadata.content_length(), content.len() as u64);
    assert!(metadata.is_file());
    debug!("Metadata: {:?}", metadata);

    // Clean up
    log_and_unwrap!(op.delete(&key).await);

    Ok(())
}

#[tokio::test]
#[tracing::instrument]
async fn test_list() -> Result<()> {
    let _guard = serial().await;
    start_server().await;
    
    let op = create_operator();
    let prefix = format!("test-list-{}/", Uuid::new_v4());
    let key1 = format!("{}file1.txt", prefix);
    let key2 = format!("{}file2.txt", prefix);
    let content = "test content";

    // Write test files
    log_and_unwrap!(op.write(&key1, content).await);
    log_and_unwrap!(op.write(&key2, content).await);
    
    // List files with prefix
    let mut lister = log_and_unwrap!(op.lister(&prefix).await);
    let mut found_keys = Vec::new();
    
    while let Some(entry) = lister.next().await {
        let entry = log_and_unwrap!(entry);
        found_keys.push(entry.path().to_string());
        debug!("Found entry: {}", entry.path());
    }
    
    assert!(found_keys.contains(&key1));
    assert!(found_keys.contains(&key2));
    debug!("Found {} files with prefix {}", found_keys.len(), prefix);

    // Clean up
    log_and_unwrap!(op.delete(&key1).await);
    log_and_unwrap!(op.delete(&key2).await);

    Ok(())
}

#[tokio::test]
#[tracing::instrument]
async fn test_delete_non_existent() -> Result<()> {
    let _guard = serial().await;
    start_server().await;
    
    let op = create_operator();
    let key = format!("non-existent-{}", Uuid::new_v4());

    // Delete non-existent key should not error (S3 behavior)
    log_and_unwrap!(op.delete(&key).await);
    debug!("Delete non-existent key succeeded");

    Ok(())
}

#[tokio::test]
#[tracing::instrument]
async fn test_range_read() -> Result<()> {
    let _guard = serial().await;
    start_server().await;
    
    let op = create_operator();
    let key = format!("test-range-{}", Uuid::new_v4());
    let content = "0123456789abcdefghijklmnopqrstuvwxyz";

    // Write data
    log_and_unwrap!(op.write(&key, content).await);
    
    // Read range
    let range_data = log_and_unwrap!(op.read_with(&key).range(5..15).await);
    let range_vec = range_data.to_vec();
    let range_content = std::str::from_utf8(&range_vec)?;
    
    assert_eq!(range_content, &content[5..15]);
    debug!("Range read: {} -> {}", 5, 15);

    // Clean up
    log_and_unwrap!(op.delete(&key).await);

    Ok(())
}