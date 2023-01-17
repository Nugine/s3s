//! AWS Signature Version 4
//!
//! See <https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html>
//!
//! See <https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html>
//!

mod presigned_url;
pub use self::presigned_url::*;

mod methods;
pub use self::methods::*;
