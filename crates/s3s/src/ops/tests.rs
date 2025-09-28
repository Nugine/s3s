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
fn complete_multipart_upload_serialize_http_excludes_trailer_headers() {
    use crate::dto::{CompleteMultipartUploadOutput, RequestCharged, ServerSideEncryption};
    use crate::ops::generated::CompleteMultipartUpload;

    // Create a CompleteMultipartUploadOutput with all trailer fields populated
    let output = CompleteMultipartUploadOutput {
        location: Some("http://example.com/bucket/key".to_string()),
        bucket: Some("test-bucket".to_string()),
        key: Some("test-key".to_string()),
        e_tag: Some("\"test-etag\"".to_string()),
        bucket_key_enabled: Some(true),
        expiration: Some("expiry=123".to_string()),
        request_charged: Some(RequestCharged::from_static(RequestCharged::REQUESTER)),
        ssekms_key_id: Some("key-id".to_string()),
        server_side_encryption: Some(ServerSideEncryption::from_static(ServerSideEncryption::AES256)),
        version_id: Some("ver123".to_string()),
        ..Default::default()
    };

    // Call serialize_http
    let resp = CompleteMultipartUpload::serialize_http(output).unwrap();

    // Verify that trailer headers are NOT present in regular headers
    // These should only be sent as trailers when using KeepAliveBody
    assert!(resp.headers.get("x-amz-server-side-encryption-bucket-key-enabled").is_none());
    assert!(resp.headers.get("x-amz-expiration").is_none());
    assert!(resp.headers.get("x-amz-request-charged").is_none());
    assert!(resp.headers.get("x-amz-server-side-encryption-aws-kms-key-id").is_none());
    assert!(resp.headers.get("x-amz-server-side-encryption").is_none());
    assert!(resp.headers.get("x-amz-version-id").is_none());

    // Verify that only the XML body is set (no headers except content-type)
    assert_eq!(resp.headers.len(), 1); // Only content-type header
    assert_eq!(resp.headers.get("content-type").unwrap(), "application/xml");
    
    // Verify the XML body content is correct
    let body_bytes = resp.body.bytes().expect("Expected bytes body");
    let body_str = std::str::from_utf8(&body_bytes).unwrap();
    
    // Should contain the key elements but no XML declaration (set_xml_body_no_decl)
    assert!(body_str.contains("<Location>http://example.com/bucket/key</Location>"));
    assert!(body_str.contains("<Bucket>test-bucket</Bucket>"));
    assert!(body_str.contains("<Key>test-key</Key>"));
    assert!(body_str.contains("<ETag>&quot;test-etag&quot;</ETag>")); // Quotes are HTML encoded
    // Should not contain XML declaration
    assert!(!body_str.contains("<?xml"));
}

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
