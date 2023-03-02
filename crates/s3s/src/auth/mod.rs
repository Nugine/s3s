//! S3 Authentication

mod secret_key;
pub use self::secret_key::SecretKey;

mod simple_auth;
pub use self::simple_auth::SimpleAuth;

mod credentials;
pub use self::credentials::Credentials;

use crate::error::S3Result;

/// S3 Authentication Provider
#[async_trait::async_trait]
pub trait S3Auth: Send + Sync + 'static {
    /// lookup `secret_key` by `access_key`
    async fn get_secret_key(&self, access_key: &str) -> S3Result<SecretKey>;
}
