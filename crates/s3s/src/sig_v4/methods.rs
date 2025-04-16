//! Signature v4 methods

use super::AmzDate;

use crate::auth::SecretKey;
use crate::http::OrderedHeaders;
use crate::utils::crypto::{hex, hex_sha256, hex_sha256_chunk, hmac_sha256};
use crate::utils::stable_sort_by_first;

use hyper::Method;
use hyper::body::Bytes;
use smallvec::SmallVec;
use stdx::str::StrExt;
use zeroize::Zeroize;

/// custom uri encode
#[allow(clippy::indexing_slicing, clippy::inline_always, clippy::unwrap_used)]
fn uri_encode(output: &mut String, input: &str, encode_slash: bool) {
    /// hex uppercase
    #[inline(always)] // perf
    fn to_hex(x: u8) -> u8 {
        b"0123456789ABCDEF"[usize::from(x)]
    }

    let mut buf: SmallVec<[u8; 512]> = SmallVec::with_capacity(input.len());

    for &byte in input.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-' | b'~' | b'.' => buf.push(byte),
            b'/' => {
                if encode_slash {
                    buf.push(b'%');
                    buf.push(b'2');
                    buf.push(b'F');
                } else {
                    buf.push(byte);
                }
            }
            _ => {
                buf.push(b'%');
                buf.push(to_hex(byte.wrapping_shr(4)));
                buf.push(to_hex(byte & 15));
            }
        }
    }

    let s = str::from_ascii_simd(buf.as_ref()).unwrap();
    output.push_str(s);
}

/// custom uri encode
fn uri_encode_string(input: &str, encode_slash: bool) -> String {
    let mut output = String::with_capacity(input.len());
    uri_encode(&mut output, input, encode_slash);
    output
}

/// is skipped header
fn is_skipped_header(header: &str) -> bool {
    header == "authorization"
}

/// is skipped query string
fn is_skipped_query_string(name: &str) -> bool {
    name == "X-Amz-Signature"
}

/// sha256 hash of an empty string
const EMPTY_STRING_SHA256_HASH: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

/// Payload
#[derive(Debug, Clone, Copy)]
pub enum Payload<'a> {
    /// unsigned
    Unsigned,
    /// empty
    Empty,
    /// single chunk
    SingleChunk(&'a [u8]),
    /// multiple chunks
    MultipleChunks,
}

/// create canonical request
#[must_use]
pub fn create_canonical_request(
    method: &Method,
    uri_path: &str,
    decoded_query_strings: &[(impl AsRef<str>, impl AsRef<str>)],
    signed_headers: &OrderedHeaders<'_>,
    payload: Payload<'_>,
) -> String {
    let mut ans = String::with_capacity(256);

    {
        // <HTTPMethod>\n
        ans.push_str(method.as_str());
        ans.push('\n');
    }

    {
        // <CanonicalURI>\n
        uri_encode(&mut ans, uri_path, false);
        ans.push('\n');
    }

    {
        // <CanonicalQueryString>\n
        let encoded_query_strings = {
            let mut qs = <SmallVec<[(String, String); 16]>>::new();
            for (n, v) in decoded_query_strings {
                let name = uri_encode_string(n.as_ref(), true);
                let value = uri_encode_string(v.as_ref(), true);
                qs.push((name, value));
            }
            stable_sort_by_first(&mut qs);
            qs
        };

        if let Some((first, remain)) = encoded_query_strings.split_first() {
            {
                let (name, value) = first;
                ans.push_str(name);
                ans.push('=');
                ans.push_str(value);
            }
            for (name, value) in remain {
                ans.push('&');
                ans.push_str(name);
                ans.push('=');
                ans.push_str(value);
            }
        }

        ans.push('\n');
    }

    {
        // <CanonicalHeaders>\n

        // FIXME: check HOST, Content-Type, x-amz-security-token, x-amz-content-sha256

        for &(name, value) in signed_headers.as_ref() {
            if is_skipped_header(name) {
                continue;
            }
            ans.push_str(name);
            ans.push(':');
            ans.push_str(value.trim());
            ans.push('\n');
        }
        ans.push('\n');
    }

    {
        // <SignedHeaders>\n
        let mut first_flag = true;
        for &(name, _) in signed_headers.as_ref() {
            if is_skipped_header(name) {
                continue;
            }
            if first_flag {
                first_flag = false;
            } else {
                ans.push(';');
            }
            ans.push_str(name);
        }

        ans.push('\n');
    }

    {
        // <HashedPayload>
        match payload {
            Payload::Unsigned => ans.push_str("UNSIGNED-PAYLOAD"),
            Payload::Empty => ans.push_str(EMPTY_STRING_SHA256_HASH),
            Payload::SingleChunk(data) => hex_sha256(data, |s| ans.push_str(s)),
            Payload::MultipleChunks => ans.push_str("STREAMING-AWS4-HMAC-SHA256-PAYLOAD"),
        }
    }

    ans
}

/// create string to sign
#[must_use]
pub fn create_string_to_sign(canonical_request: &str, amz_date: &AmzDate, region: &str, service: &str) -> String {
    let mut ans = String::with_capacity(256);

    {
        // <Algorithm>\n
        ans.push_str("AWS4-HMAC-SHA256\n");
    }

    {
        // <RequestDateTime>\n
        ans.push_str(&amz_date.fmt_iso8601());
        ans.push('\n');
    }

    {
        // <CredentialScope>\n
        ans.push_str(&amz_date.fmt_date());
        ans.push('/');
        ans.push_str(region); // TODO: use a `Region` type
        ans.push('/');
        ans.push_str(service);
        ans.push_str("/aws4_request\n");
    }

    {
        // <HashedCanonicalRequest>
        hex_sha256(canonical_request.as_bytes(), |s| ans.push_str(s));
    }

    ans
}

/// create `string_to_sign` of a chunk
pub fn create_chunk_string_to_sign(
    amz_date: &AmzDate,
    region: &str,
    service: &str,
    prev_signature: &str,
    chunk_data: &[Bytes],
) -> String {
    let mut ans = String::with_capacity(256);

    {
        ans.push_str("AWS4-HMAC-SHA256-PAYLOAD\n");
    }
    {
        ans.push_str(&amz_date.fmt_iso8601());
        ans.push('\n');
    }
    {
        ans.push_str(&amz_date.fmt_date());
        ans.push('/');
        ans.push_str(region); // TODO: use a `Region` type
        ans.push('/');
        ans.push_str(service);
        ans.push_str("/aws4_request\n");
    }
    {
        ans.push_str(prev_signature);
        ans.push('\n');
    }
    {
        ans.push_str(EMPTY_STRING_SHA256_HASH);
        ans.push('\n');
    }
    {
        if chunk_data.is_empty() {
            ans.push_str(EMPTY_STRING_SHA256_HASH);
        } else {
            hex_sha256_chunk(chunk_data, |s| ans.push_str(s));
        }
    }

    ans
}

/// calculate signature
#[must_use]
pub fn calculate_signature(
    string_to_sign: &str,
    secret_key: &SecretKey,
    amz_date: &AmzDate,
    region: &str,
    service: &str,
) -> String {
    let mut secret = {
        let secret_key = secret_key.expose();
        let mut buf = <SmallVec<[u8; 128]>>::with_capacity(secret_key.len().saturating_add(4));
        buf.extend_from_slice(b"AWS4");
        buf.extend_from_slice(secret_key.as_bytes());
        buf
    };

    // DateKey
    let date = amz_date.fmt_date();
    let date_key = hmac_sha256(secret.as_slice(), date.as_bytes());

    secret.zeroize();
    drop(secret);

    // DateRegionKey
    let date_region_key = hmac_sha256(date_key, region); // TODO: use a `Region` type

    // DateRegionServiceKey
    let date_region_service_key = hmac_sha256(date_region_key, service);

    // SigningKey
    let signing_key = hmac_sha256(date_region_service_key, "aws4_request");

    // Signature
    hex(hmac_sha256(signing_key, string_to_sign))
}

/// create presigned canonical request
pub fn create_presigned_canonical_request(
    method: &Method,
    uri_path: &str,
    decoded_query_strings: &[(impl AsRef<str>, impl AsRef<str>)],
    signed_headers: &OrderedHeaders<'_>,
) -> String {
    let mut ans = String::with_capacity(256);
    {
        // <HTTPMethod>\n
        ans.push_str(method.as_str());
        ans.push('\n');
    }
    {
        // <CanonicalURI>\n
        uri_encode(&mut ans, uri_path, false);
        ans.push('\n');
    }
    {
        // <CanonicalQueryString>\n
        let encoded_query_strings = {
            let mut qs = <SmallVec<[(String, String); 16]>>::new();
            for (n, v) in decoded_query_strings {
                if is_skipped_query_string(n.as_ref()) {
                    continue;
                }
                let name = uri_encode_string(n.as_ref(), true);
                let value = uri_encode_string(v.as_ref(), true);
                qs.push((name, value));
            }
            stable_sort_by_first(&mut qs);
            qs
        };

        if let Some((first, remain)) = encoded_query_strings.split_first() {
            {
                let (name, value) = first;
                ans.push_str(name);
                ans.push('=');
                ans.push_str(value);
            }
            for (name, value) in remain {
                ans.push('&');
                ans.push_str(name);
                ans.push('=');
                ans.push_str(value);
            }
        }

        ans.push('\n');
    }
    {
        // <CanonicalHeaders>\n

        for &(name, value) in signed_headers.as_ref() {
            if is_skipped_header(name) {
                continue;
            }
            ans.push_str(name);
            ans.push(':');
            ans.push_str(value.trim());
            ans.push('\n');
        }
        ans.push('\n');
    }
    {
        // <SignedHeaders>\n
        let mut first_flag = true;
        for &(name, _) in signed_headers.as_ref() {
            if is_skipped_header(name) {
                continue;
            }
            if first_flag {
                first_flag = false;
            } else {
                ans.push(';');
            }
            ans.push_str(name);
        }

        ans.push('\n');
    }
    {
        // <Payload>
        ans.push_str("UNSIGNED-PAYLOAD");
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::http::OrderedQs;
    use crate::sig_v4::PresignedUrlV4;

    #[test]
    fn example_get_object() {
        // let access_key_id = "AKIAIOSFODNN7EXAMPLE";
        let secret_access_key = SecretKey::from("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
        let timestamp = "20130524T000000Z";
        // let bucket = "examplebucket";
        let region = "us-east-1";
        let service = "s3";
        let path = "/test.txt";

        let headers = OrderedHeaders::from_slice_unchecked(&[
            ("host", "examplebucket.s3.amazonaws.com"),
            ("range", "bytes=0-9"),
            ("x-amz-content-sha256", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            ("x-amz-date", "20130524T000000Z"),
        ]);

        let method = Method::GET;
        let qs: &[(String, String)] = &[];

        let canonical_request = create_canonical_request(&method, path, qs, &headers, Payload::Empty);

        assert_eq!(
            canonical_request,
            concat!(
                "GET\n",
                "/test.txt\n",
                "\n",
                "host:examplebucket.s3.amazonaws.com\n",
                "range:bytes=0-9\n",
                "x-amz-content-sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n",
                "x-amz-date:20130524T000000Z\n",
                "\n",
                "host;range;x-amz-content-sha256;x-amz-date\n",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            )
        );

        let date = AmzDate::parse(timestamp).unwrap();
        let string_to_sign = create_string_to_sign(&canonical_request, &date, region, service);
        assert_eq!(
            string_to_sign,
            concat!(
                "AWS4-HMAC-SHA256\n",
                "20130524T000000Z\n",
                "20130524/us-east-1/s3/aws4_request\n",
                "7344ae5b7ee6c3e7e6b0fe0640412a37625d1fbfff95c48bbb2dc43964946972",
            )
        );

        let signature = calculate_signature(&string_to_sign, &secret_access_key, &date, region, service);
        assert_eq!(signature, "f0e8bdb87c964420e857bd35b5d6ed310bd44f0170aba48dd91039c6036bdb41");
    }

    #[test]
    fn example_put_object_single_chunk() {
        // let access_key_id = "AKIAIOSFODNN7EXAMPLE";
        let secret_access_key = SecretKey::from("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
        let timestamp = "20130524T000000Z";
        // let bucket = "examplebucket";
        let region = "us-east-1";
        let service = "s3";
        let path = "/test$file.text";

        let headers = OrderedHeaders::from_slice_unchecked(&[
            ("date", "Fri, 24 May 2013 00:00:00 GMT"),
            ("host", "examplebucket.s3.amazonaws.com"),
            ("x-amz-content-sha256", "44ce7dd67c959e0d3524ffac1771dfbba87d2b6b4b4e99e42034a8b803f8b072"),
            ("x-amz-date", "20130524T000000Z"),
            ("x-amz-storage-class", "REDUCED_REDUNDANCY"),
        ]);

        let method = Method::PUT;
        let payload = "Welcome to Amazon S3.";
        let qs: &[(String, String)] = &[];

        let canonical_request = create_canonical_request(&method, path, qs, &headers, Payload::SingleChunk(payload.as_bytes()));

        assert_eq!(
            canonical_request,
            concat!(
                "PUT\n",
                "/test%24file.text\n",
                "\n",
                "date:Fri, 24 May 2013 00:00:00 GMT\n",
                "host:examplebucket.s3.amazonaws.com\n",
                "x-amz-content-sha256:44ce7dd67c959e0d3524ffac1771dfbba87d2b6b4b4e99e42034a8b803f8b072\n",
                "x-amz-date:20130524T000000Z\n",
                "x-amz-storage-class:REDUCED_REDUNDANCY\n",
                "\n",
                "date;host;x-amz-content-sha256;x-amz-date;x-amz-storage-class\n",
                "44ce7dd67c959e0d3524ffac1771dfbba87d2b6b4b4e99e42034a8b803f8b072",
            )
        );

        let date = AmzDate::parse(timestamp).unwrap();
        let string_to_sign = create_string_to_sign(&canonical_request, &date, region, service);
        assert_eq!(
            string_to_sign,
            concat!(
                "AWS4-HMAC-SHA256\n",
                "20130524T000000Z\n",
                "20130524/us-east-1/s3/aws4_request\n",
                "9e0e90d9c76de8fa5b200d8c849cd5b8dc7a3be3951ddb7f6a76b4158342019d",
            )
        );

        let signature = calculate_signature(&string_to_sign, &secret_access_key, &date, region, service);
        assert_eq!(signature, "98ad721746da40c64f1a55b78f14c238d841ea1380cd77a1b5971af0ece108bd");
    }

    #[test]
    fn example_put_object_multiple_chunks_seed_signature() {
        // let access_key_id = "AKIAIOSFODNN7EXAMPLE";
        let secret_access_key = SecretKey::from("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
        let timestamp = "20130524T000000Z";
        // let bucket = "examplebucket";
        let region = "us-east-1";
        let service = "s3";
        let path = "/examplebucket/chunkObject.txt";

        let headers = OrderedHeaders::from_slice_unchecked(&[
            ("content-encoding", "aws-chunked"),
            ("content-length", "66824"),
            ("host", "s3.amazonaws.com"),
            ("x-amz-content-sha256", "STREAMING-AWS4-HMAC-SHA256-PAYLOAD"),
            ("x-amz-date", "20130524T000000Z"),
            ("x-amz-decoded-content-length", "66560"),
            ("x-amz-storage-class", "REDUCED_REDUNDANCY"),
        ]);

        let method = Method::PUT;
        let qs: &[(String, String)] = &[];

        let canonical_request = create_canonical_request(&method, path, qs, &headers, Payload::MultipleChunks);

        assert_eq!(
            canonical_request,
            concat!(
                "PUT\n",
                "/examplebucket/chunkObject.txt\n",
                "\n",
                "content-encoding:aws-chunked\n",
                "content-length:66824\n",
                "host:s3.amazonaws.com\n",
                "x-amz-content-sha256:STREAMING-AWS4-HMAC-SHA256-PAYLOAD\n",
                "x-amz-date:20130524T000000Z\n",
                "x-amz-decoded-content-length:66560\n",
                "x-amz-storage-class:REDUCED_REDUNDANCY\n",
                "\n",
                "content-encoding;content-length;host;x-amz-content-sha256;x-amz-date;x-amz-decoded-content-length;x-amz-storage-class\n",
                "STREAMING-AWS4-HMAC-SHA256-PAYLOAD",
            )
        );

        let date = AmzDate::parse(timestamp).unwrap();
        let string_to_sign = create_string_to_sign(&canonical_request, &date, region, service);
        assert_eq!(
            string_to_sign,
            concat!(
                "AWS4-HMAC-SHA256\n",
                "20130524T000000Z\n",
                "20130524/us-east-1/s3/aws4_request\n",
                "cee3fed04b70f867d036f722359b0b1f2f0e5dc0efadbc082b76c4c60e316455",
            )
        );

        let signature = calculate_signature(&string_to_sign, &secret_access_key, &date, region, service);
        assert_eq!(signature, "4f232c4386841ef735655705268965c44a0e4690baa4adea153f7db9fa80a0a9",);
    }

    #[test]
    fn example_put_object_multiple_chunks_chunk_signature() {
        let secret_access_key = SecretKey::from("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
        let timestamp = "20130524T000000Z";
        let region = "us-east-1";
        let service = "s3";
        let date = AmzDate::parse(timestamp).unwrap();

        let seed_signature = "4f232c4386841ef735655705268965c44a0e4690baa4adea153f7db9fa80a0a9";

        let chunk1_string_to_sign =
            create_chunk_string_to_sign(&date, region, service, seed_signature, &[Bytes::from(vec![b'a'; 64 * 1024])]);
        assert_eq!(
            chunk1_string_to_sign,
            concat!(
                "AWS4-HMAC-SHA256-PAYLOAD\n",
                "20130524T000000Z\n",
                "20130524/us-east-1/s3/aws4_request\n",
                "4f232c4386841ef735655705268965c44a0e4690baa4adea153f7db9fa80a0a9\n",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n",
                "bf718b6f653bebc184e1479f1935b8da974d701b893afcf49e701f3e2f9f9c5a",
            )
        );

        let chunk1_signature = calculate_signature(&chunk1_string_to_sign, &secret_access_key, &date, region, service);
        assert_eq!(chunk1_signature, "ad80c730a21e5b8d04586a2213dd63b9a0e99e0e2307b0ade35a65485a288648");

        let chunk2_string_to_sign =
            create_chunk_string_to_sign(&date, region, service, &chunk1_signature, &[Bytes::from(vec![b'a'; 1024])]);
        assert_eq!(
            chunk2_string_to_sign,
            concat!(
                "AWS4-HMAC-SHA256-PAYLOAD\n",
                "20130524T000000Z\n",
                "20130524/us-east-1/s3/aws4_request\n",
                "ad80c730a21e5b8d04586a2213dd63b9a0e99e0e2307b0ade35a65485a288648\n",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n",
                "2edc986847e209b4016e141a6dc8716d3207350f416969382d431539bf292e4a",
            )
        );

        let chunk2_signature = calculate_signature(&chunk2_string_to_sign, &secret_access_key, &date, region, service);
        assert_eq!(chunk2_signature, "0055627c9e194cb4542bae2aa5492e3c1575bbb81b612b7d234b86a503ef5497");

        let chunk3_string_to_sign = create_chunk_string_to_sign(&date, region, service, &chunk2_signature, &[]);
        assert_eq!(
            chunk3_string_to_sign,
            concat!(
                "AWS4-HMAC-SHA256-PAYLOAD\n",
                "20130524T000000Z\n",
                "20130524/us-east-1/s3/aws4_request\n",
                "0055627c9e194cb4542bae2aa5492e3c1575bbb81b612b7d234b86a503ef5497\n",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            )
        );

        let chunk3_signature = calculate_signature(&chunk3_string_to_sign, &secret_access_key, &date, region, service);
        assert_eq!(chunk3_signature, "b6c6ea8a5354eaf15b3cb7646744f4275b71ea724fed81ceb9323e279d449df9");
    }

    #[test]
    fn example_get_bucket_lifecycle_configuration() {
        // let access_key_id = "AKIAIOSFODNN7EXAMPLE";
        let secret_access_key = SecretKey::from("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
        let timestamp = "20130524T000000Z";
        // let bucket = "examplebucket";
        let region = "us-east-1";
        let service = "s3";
        let path = "/";

        let headers = OrderedHeaders::from_slice_unchecked(&[
            ("host", "examplebucket.s3.amazonaws.com"),
            ("x-amz-content-sha256", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            ("x-amz-date", "20130524T000000Z"),
        ]);

        let query_strings = &[("lifecycle", "")];

        let method = Method::GET;

        let canonical_request = create_canonical_request(&method, path, query_strings, &headers, Payload::Empty);
        assert_eq!(
            canonical_request,
            concat!(
                "GET\n",
                "/\n",
                "lifecycle=\n",
                "host:examplebucket.s3.amazonaws.com\n",
                "x-amz-content-sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n",
                "x-amz-date:20130524T000000Z\n",
                "\n",
                "host;x-amz-content-sha256;x-amz-date\n",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            )
        );

        let date = AmzDate::parse(timestamp).unwrap();
        let string_to_sign = create_string_to_sign(&canonical_request, &date, region, service);
        assert_eq!(
            string_to_sign,
            concat!(
                "AWS4-HMAC-SHA256\n",
                "20130524T000000Z\n",
                "20130524/us-east-1/s3/aws4_request\n",
                "9766c798316ff2757b517bc739a67f6213b4ab36dd5da2f94eaebf79c77395ca",
            )
        );

        let signature = calculate_signature(&string_to_sign, &secret_access_key, &date, region, service);
        assert_eq!(signature, "fea454ca298b7da1c68078a5d1bdbfbbe0d65c699e0f91ac7a200a0136783543");
    }

    #[test]
    fn example_list_objects() {
        // let access_key_id = "AKIAIOSFODNN7EXAMPLE";
        let secret_access_key = SecretKey::from("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
        let timestamp = "20130524T000000Z";
        // let bucket = "examplebucket";
        let region = "us-east-1";
        let service = "s3";
        let path = "/";

        let headers = OrderedHeaders::from_slice_unchecked(&[
            ("host", "examplebucket.s3.amazonaws.com"),
            ("x-amz-content-sha256", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            ("x-amz-date", "20130524T000000Z"),
        ]);

        let query_strings = &[("max-keys", "2"), ("prefix", "J")];

        let method = Method::GET;

        let canonical_request = create_canonical_request(&method, path, query_strings, &headers, Payload::Empty);

        assert_eq!(
            canonical_request,
            concat!(
                "GET\n",
                "/\n",
                "max-keys=2&prefix=J\n",
                "host:examplebucket.s3.amazonaws.com\n",
                "x-amz-content-sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\n",
                "x-amz-date:20130524T000000Z\n",
                "\n",
                "host;x-amz-content-sha256;x-amz-date\n",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            )
        );

        let date = AmzDate::parse(timestamp).unwrap();
        let string_to_sign = create_string_to_sign(&canonical_request, &date, region, service);
        assert_eq!(
            string_to_sign,
            concat!(
                "AWS4-HMAC-SHA256\n",
                "20130524T000000Z\n",
                "20130524/us-east-1/s3/aws4_request\n",
                "df57d21db20da04d7fa30298dd4488ba3a2b47ca3a489c74750e0f1e7df1b9b7",
            )
        );

        let signature = calculate_signature(&string_to_sign, &secret_access_key, &date, region, service);
        assert_eq!(signature, "34b48302e7b5fa45bde8084f4b7868a86f0a534bc59db6670ed5711ef69dc6f7");
    }

    #[test]
    fn example_presigned_url() {
        use hyper::Uri;

        // let access_key_id = "AKIAIOSFODNN7EXAMPLE";
        let secret_access_key = SecretKey::from("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");

        let method = Method::GET;

        let uri = Uri::from_static(concat!(
            "https://s3.amazonaws.com/test.txt",
            "?X-Amz-Algorithm=AWS4-HMAC-SHA256",
            "&X-Amz-Credential=AKIAIOSFODNN7EXAMPLE%2F20130524%2Fus-east-1%2Fs3%2Faws4_request",
            "&X-Amz-Date=20130524T000000Z",
            "&X-Amz-Expires=86400",
            "&X-Amz-SignedHeaders=host",
            "&X-Amz-Signature=aeeed9bbccd4d02ee5c0109b86d86835f995330da4c265957d157751f604d404"
        ));

        let headers = OrderedHeaders::from_slice_unchecked(&[("host", "examplebucket.s3.amazonaws.com")]);

        let query_strings = &[
            ("X-Amz-Algorithm", "AWS4-HMAC-SHA256"),
            ("X-Amz-Credential", "AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request"),
            ("X-Amz-Date", "20130524T000000Z"),
            ("X-Amz-Expires", "86400"),
            ("X-Amz-SignedHeaders", "host"),
            ("X-Amz-Signature", "aeeed9bbccd4d02ee5c0109b86d86835f995330da4c265957d157751f604d404"),
        ];

        let qs = OrderedQs::from_vec_unchecked(query_strings.iter().map(|&(n, v)| (n.to_owned(), v.to_owned())).collect());

        let info = PresignedUrlV4::parse(&qs).unwrap();

        let canonical_request = create_presigned_canonical_request(&method, uri.path(), query_strings, &headers);

        assert_eq!(
            canonical_request,
            concat!(
                "GET\n",
                "/test.txt\n",
                "X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAIOSFODNN7EXAMPLE%2F20130524%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20130524T000000Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host\n",
                "host:examplebucket.s3.amazonaws.com\n",
                "\n",
                "host\n",
                "UNSIGNED-PAYLOAD",
            )
        );

        let string_to_sign = create_string_to_sign(
            &canonical_request,
            &info.amz_date,
            info.credential.aws_region,
            info.credential.aws_service,
        );
        assert_eq!(
            string_to_sign,
            concat!(
                "AWS4-HMAC-SHA256\n",
                "20130524T000000Z\n",
                "20130524/us-east-1/s3/aws4_request\n",
                "3bfa292879f6447bbcda7001decf97f4a54dc650c8942174ae0a9121cf58ad04",
            )
        );

        let signature = calculate_signature(
            &string_to_sign,
            &secret_access_key,
            &info.amz_date,
            info.credential.aws_region,
            info.credential.aws_service,
        );
        assert_eq!(signature, "aeeed9bbccd4d02ee5c0109b86d86835f995330da4c265957d157751f604d404");
        assert_eq!(signature, info.signature);
    }

    #[test]
    fn special_20230204() {
        use hyper::header::HeaderName;
        use hyper::header::HeaderValue;

        let mut req = http::Request::<hyper::body::Bytes>::default();

        *req.method_mut() = Method::GET;
        *req.uri_mut() = hyper::Uri::from_static(
            "http://localhost:8014/minio-java-test-1gqr1v4?prefix=prefix&suffix=suffix&events=s3%3AObjectCreated%3A%2A&events=s3%3AObjectAccessed%3A%2A",
        );

        let x_amz_date = "20230204T155111Z";
        let headers = [
            ("content-md5", "1B2M2Y8AsgTpgAmY7PhCfg=="),
            ("host", "localhost:8014"),
            ("x-amz-content-sha256", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            ("x-amz-date", x_amz_date),
        ];
        for (name, value) in &headers {
            req.headers_mut()
                .insert(HeaderName::from_static(name), HeaderValue::from_static(value));
        }

        let signed_header_names = &["content-md5", "host", "x-amz-content-sha256", "x-amz-date"];

        let payload = Payload::Empty;
        let date = AmzDate::parse(x_amz_date).unwrap();
        let region = "us-east-1";
        let service = "s3";

        let secret_access_key = SecretKey::from("minioadmin");

        {
            let uri_path = req.uri().path();
            let qs = req.uri().query().map(|q| OrderedQs::parse(q).unwrap());
            let query_strings: &[_] = qs.as_ref().map_or(&[], |x| x.as_ref());

            let signed_headers = OrderedHeaders::from_headers(req.headers())
                .unwrap()
                .find_multiple_with_on_missing(signed_header_names, |_| None);

            let canonical_request = create_canonical_request(req.method(), uri_path, query_strings, &signed_headers, payload);

            let string_to_sign = create_string_to_sign(&canonical_request, &date, region, service);

            let signature = calculate_signature(&string_to_sign, &secret_access_key, &date, region, service);
            assert_eq!(signature, "96ad058ca27352e0fc2bd4efd8973792077570667bdaf749655f42e204bc649c");
        }
    }
}
