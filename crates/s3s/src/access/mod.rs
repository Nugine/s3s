mod context;
pub use self::context::S3AccessContext;

use crate::error::S3Result;

#[async_trait::async_trait]
pub trait S3Access: Send + Sync + 'static {
    /// Checks whether the current request have accesses to the resources.
    ///
    /// This method is called before deserializing the operation input.
    ///
    /// By default, this method rejects all anonymous requests
    /// and returns [`AccessDenied`](crate::S3ErrorCode::AccessDenied) error.
    ///
    /// An access control provider can override this method to implement custom logic.
    ///
    /// Common fields in the context:
    /// + [`cx.credentials()`](S3AccessContext::credentials)
    /// + [`cx.s3_path()`](S3AccessContext::s3_path)
    /// + [`cx.s3_op().name()`](crate::S3Operation::name)
    /// + [`cx.extensions_mut()`](S3AccessContext::extensions_mut)
    async fn check(&self, cx: &mut S3AccessContext<'_>) -> S3Result<()> {
        default_check(cx)
    }
}

pub(crate) fn default_check(cx: &mut S3AccessContext<'_>) -> S3Result<()> {
    match cx.credentials() {
        Some(_) => Ok(()),
        None => Err(s3_error!(AccessDenied, "Signature is required")),
    }
}
