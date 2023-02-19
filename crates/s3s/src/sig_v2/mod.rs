//! AWS Signature Version 2
//!
//! <https://docs.aws.amazon.com/AmazonS3/latest/userguide/RESTAuthentication.html>
//!

mod authorization_v2;
pub use self::authorization_v2::*;

mod presigned_url_v2;
pub use self::presigned_url_v2::*;

mod methods;
pub use self::methods::*;
