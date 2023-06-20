//! S3 Authentication

mod secret_key;
pub use self::secret_key::SecretKey;

mod simple_auth;
pub use self::simple_auth::SimpleAuth;

mod credentials;
pub use self::credentials::Credentials;

mod context;
pub use self::context::S3AuthContext;

use crate::error::S3Result;

/// S3 Authentication Provider
#[async_trait::async_trait]
pub trait S3Auth: Send + Sync + 'static {
    /// Gets the corresponding secret key of the access key.
    ///
    /// This method is usually implemented as a database query.
    async fn get_secret_key(&self, access_key: &str) -> S3Result<SecretKey>;

    /// Checks if the current request can access the resource.
    ///
    /// By default, this method rejects all anonymous requests
    /// and returns [`AccessDenied`](crate::S3ErrorCode::AccessDenied) error.
    ///
    /// An authentication provider can override this method to implement custom access control.
    ///
    /// Common fields in the context:
    /// + [`cx.credentials()`](S3AuthContext::credentials)
    /// + [`cx.s3_path()`](S3AuthContext::s3_path)
    /// + [`cx.extensions_mut()`](S3AuthContext::extensions_mut)
    async fn check_access(&self, cx: &mut S3AuthContext<'_>) -> S3Result<()> {
        match cx.credentials() {
            Some(_) => Ok(()),
            None => Err(s3_error!(AccessDenied, "Signature is required")),
        }
    }
}
