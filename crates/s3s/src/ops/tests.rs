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
fn extract_mime_handles_invalid_content_type() {
    // Test empty content-type
    let mut headers = HeaderMap::new();
    headers.insert(crate::header::CONTENT_TYPE, HeaderValue::from_static(""));
    let ordered_headers = OrderedHeaders::from_headers(&headers).unwrap();
    let mime = extract_mime(&ordered_headers);
    // Should default to application/octet-stream
    assert_eq!(mime, Some(mime::APPLICATION_OCTET_STREAM));

    // Test content-type without slash (e.g., "text")
    let mut headers = HeaderMap::new();
    headers.insert(crate::header::CONTENT_TYPE, HeaderValue::from_static("text"));
    let ordered_headers = OrderedHeaders::from_headers(&headers).unwrap();
    let mime = extract_mime(&ordered_headers);
    // Should default to application/octet-stream
    assert_eq!(mime, Some(mime::APPLICATION_OCTET_STREAM));

    // Test valid content-type
    let mut headers = HeaderMap::new();
    headers.insert(crate::header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    let ordered_headers = OrderedHeaders::from_headers(&headers).unwrap();
    let mime = extract_mime(&ordered_headers);
    assert_eq!(mime, Some("text/plain".parse::<Mime>().unwrap()));

    // Test missing content-type
    let headers = HeaderMap::new();
    let ordered_headers = OrderedHeaders::from_headers(&headers).unwrap();
    let mime = extract_mime(&ordered_headers);
    assert_eq!(mime, None);
}
