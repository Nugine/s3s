use super::*;

// use crate::service::S3Service;

// use stdx::mem::output_size;

// #[test]
// #[ignore]
// fn track_future_size() {
//     macro_rules! future_size {
//         ($f:path, $v:expr) => {
//             (stringify!($f), output_size(&$f), $v)
//         };
//     }

//     #[rustfmt::skip]
//     let sizes = [
//         future_size!(S3Service::call,                           2704),
//         future_size!(call,                                      1512),
//         future_size!(prepare,                                   1440),
//         future_size!(SignatureContext::check,                   776),
//         future_size!(SignatureContext::v2_check,                296),
//         future_size!(SignatureContext::v2_check_presigned_url,  168),
//         future_size!(SignatureContext::v2_check_header_auth,    184),
//         future_size!(SignatureContext::v4_check,                752),
//         future_size!(SignatureContext::v4_check_post_signature, 368),
//         future_size!(SignatureContext::v4_check_presigned_url,  456),
//         future_size!(SignatureContext::v4_check_header_auth,    640),
//     ];

//     println!("{sizes:#?}");
//     for (name, size, expected) in sizes {
//         assert_eq!(size, expected, "{name:?} size changed: prev {expected}, now {size}");
//     }
// }

#[test]
fn error_custom_headers() {
    fn redirect307(location: &str) -> S3Error {
        let mut err = S3Error::new(S3ErrorCode::TemporaryRedirect);

        err.set_headers({
            let mut headers = HeaderMap::new();
            headers.insert(crate::header::LOCATION, location.parse().unwrap());
            headers
        });

        err
    }

    let res = serialize_error(redirect307("http://example.com"), false).unwrap();
    assert_eq!(res.status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(res.headers.get("location").unwrap(), "http://example.com");

    let body = res.body.bytes().unwrap();
    let body = std::str::from_utf8(&body).unwrap();
    assert_eq!(
        body,
        concat!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
            "<Error><Code>TemporaryRedirect</Code></Error>"
        )
    );
}

#[test]
fn test_looks_like_virtual_hosted_style() {
    use super::looks_like_virtual_hosted_style;

    // Should detect virtual-hosted-style hosts with paths that clearly indicate VH style
    assert!(looks_like_virtual_hosted_style("bucket.example.com", "/"));
    assert!(looks_like_virtual_hosted_style("test.localhost", "/"));
    assert!(looks_like_virtual_hosted_style("bucket.example.com", "/Invalid_Bucket_Name")); // Invalid bucket name
    assert!(looks_like_virtual_hosted_style("my-bucket.s3.amazonaws.com", "/has_underscores"));
    assert!(looks_like_virtual_hosted_style("my-bucket.s3.amazonaws.com", "/Has-Capitals"));

    // Should NOT warn for ambiguous cases (valid bucket names in path - could be path-style)
    assert!(!looks_like_virtual_hosted_style("bucket.example.com", "/object.txt"));
    assert!(!looks_like_virtual_hosted_style("my-bucket.s3.amazonaws.com", "/path/to/object"));

    // Should not detect path-style hosts (no dots or IP addresses)
    assert!(!looks_like_virtual_hosted_style("localhost", "/bucket/object"));
    assert!(!looks_like_virtual_hosted_style("127.0.0.1", "/bucket/object"));
    assert!(!looks_like_virtual_hosted_style("192.168.1.1:8080", "/bucket/object"));
    assert!(!looks_like_virtual_hosted_style("s3", "/bucket/object"));

    // Edge cases
    assert!(!looks_like_virtual_hosted_style("::1", "/bucket/object"));
    assert!(!looks_like_virtual_hosted_style("[::1]:8080", "/bucket/object"));

    // MAIN FIX: Should not warn for legitimate path-style requests on domain names
    assert!(!looks_like_virtual_hosted_style("s3.example.com", "/bucket/object"));
    assert!(!looks_like_virtual_hosted_style("api.example.com", "/my-bucket/file.txt"));
    assert!(!looks_like_virtual_hosted_style("minio.company.com", "/test-bucket/"));
    assert!(!looks_like_virtual_hosted_style("s3.amazonaws.com", "/valid-bucket-name/key"));
}
