//! S3 Authentication

mod secret_key;
pub use self::secret_key::{Credentials, SecretKey};

mod simple_auth;
pub use self::simple_auth::SimpleAuth;

use crate::error::S3Result;

/// S3 Authentication Provider
#[async_trait::async_trait]
pub trait S3Auth: Send + Sync + 'static {
    /// Gets the corresponding secret key of the access key.
    ///
    /// This method is usually implemented as a database query.
    async fn get_secret_key(&self, access_key: &str) -> S3Result<SecretKey>;
}
