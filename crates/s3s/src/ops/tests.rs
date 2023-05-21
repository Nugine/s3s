use super::*;

use crate::service::S3Service;

use rust_utils::mem::output_size;

#[test]
#[ignore]
fn track_future_size() {
    macro_rules! future_size {
        ($f:path, $v:expr) => {
            (stringify!($f), output_size(&$f), $v)
        };
    }

    #[rustfmt::skip]
    let sizes = [
        future_size!(S3Service::call,                           2600),
        future_size!(call,                                      1424),
        future_size!(prepare,                                   1352),
        future_size!(SignatureContext::check,                   744),
        future_size!(SignatureContext::v2_check,                280),
        future_size!(SignatureContext::v2_check_presigned_url,  168),
        future_size!(SignatureContext::v2_check_header_auth,    184),
        future_size!(SignatureContext::v4_check,                720),
        future_size!(SignatureContext::v4_check_post_signature, 368),
        future_size!(SignatureContext::v4_check_presigned_url,  456),
        future_size!(SignatureContext::v4_check_header_auth,    624),
    ];

    println!("{sizes:#?}");
    for (name, size, expected) in sizes {
        assert_eq!(size, expected, "{name:?} size changed: prev {expected}, now {size}");
    }
}
