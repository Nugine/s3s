mod generated;
pub use self::generated::S3Access;

mod context;
pub use self::context::S3AccessContext;

use crate::error::S3Result;

pub(crate) fn default_check(cx: &mut S3AccessContext<'_>) -> S3Result<()> {
    match cx.credentials() {
        Some(_) => Ok(()),
        None => Err(s3_error!(AccessDenied, "Signature is required")),
    }
}
