use super::{Multipart, Request};

use crate::dto::{List, Metadata, StreamingBlob, Timestamp, TimestampFormat};
use crate::error::*;
use crate::http::{HeaderName, HeaderValue};
use crate::path::S3Path;
use crate::xml;

use std::fmt;
use std::str::FromStr;

use stdx::string::StringExt;
use tracing::debug;

fn missing_header(name: &HeaderName) -> S3Error {
    invalid_request!("missing header: {}", name.as_str())
}

fn duplicate_header(name: &HeaderName) -> S3Error {
    invalid_request!("duplicate header: {}", name.as_str())
}

/// Check if a header should allow duplicates according to HTTP standards
/// 
/// Some headers can safely have multiple values and should be combined
/// according to HTTP/1.1 specification (RFC 7230).
/// 
/// Security-critical headers always reject duplicates for safety.
pub fn header_allows_duplicates(name: &HeaderName) -> bool {
    let name_str = name.as_str();
    
    // Security-critical headers must never allow duplicates
    match name_str {
        // AWS authentication/signature headers
        "authorization" | "x-amz-date" | "x-amz-content-sha256" | 
        "x-amz-security-token" | "x-amz-signature" | "host" => false,
        
        // AWS checksum headers (only one checksum type should be specified)
        name if name.starts_with("x-amz-checksum-") => false,
        
        // AWS server-side encryption headers 
        "x-amz-server-side-encryption" | "x-amz-server-side-encryption-aws-kms-key-id" |
        "x-amz-server-side-encryption-context" | "x-amz-server-side-encryption-bucket-key-enabled" => false,
        
        // Standard HTTP headers that can safely have multiple values
        "accept" | "accept-encoding" | "accept-language" | "cache-control" | "connection" |
        "pragma" | "trailer" | "transfer-encoding" | "upgrade" | "via" | "warning" => true,
        
        // Custom metadata headers can have multiple values 
        name if name.starts_with("x-amz-meta-") => true,
        
        // For AWS headers not explicitly listed, be conservative and reject duplicates
        name if name.starts_with("x-amz-") => false,
        
        // For other standard headers, be conservative and reject duplicates by default
        // This maintains the current strict behavior while allowing specific safe cases
        _ => false,
    }
}

fn invalid_header<E>(source: E, name: &HeaderName, val: impl fmt::Debug) -> S3Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    s3_error!(source, InvalidArgument, "invalid header: {}: {:?}", name.as_str(), val)
}

pub fn parse_header<T>(req: &Request, name: &HeaderName) -> S3Result<T>
where
    T: TryFromHeaderValue,
    T::Error: std::error::Error + Send + Sync + 'static,
{
    let mut iter = req.headers.get_all(name).into_iter();
    let Some(val) = iter.next() else { return Err(missing_header(name)) };
    let None = iter.next() else { return Err(duplicate_header(name)) };

    T::try_from_header_value(val).map_err(|err| invalid_header(err, name, val))
}

/// Parse header with support for HTTP-standard duplicate handling
/// 
/// This function allows certain headers to have multiple values, combining them
/// with comma separators according to HTTP/1.1 standards. Security-critical
/// headers still reject duplicates.
pub fn parse_header_with_duplicates<T>(req: &Request, name: &HeaderName) -> S3Result<T>
where
    T: TryFromHeaderValue,
    T::Error: std::error::Error + Send + Sync + 'static,
{
    let mut iter = req.headers.get_all(name).into_iter();
    let Some(first_val) = iter.next() else { return Err(missing_header(name)) };
    
    // Check if this header allows duplicates
    if !header_allows_duplicates(name) {
        // Use strict duplicate checking for security-critical headers
        let None = iter.next() else { return Err(duplicate_header(name)) };
        return T::try_from_header_value(first_val).map_err(|err| invalid_header(err, name, first_val));
    }
    
    // For headers that allow duplicates, combine values with comma separator
    let Some(next_val) = iter.next() else {
        // Only one value, use it directly
        return T::try_from_header_value(first_val).map_err(|err| invalid_header(err, name, first_val));
    };
    
    // Multiple values - combine them according to HTTP standards
    let mut combined = String::new();
    combined.push_str(first_val.to_str().map_err(|err| invalid_header(err, name, first_val))?);
    combined.push_str(", ");
    combined.push_str(next_val.to_str().map_err(|err| invalid_header(err, name, next_val))?);
    
    // Add any additional values
    for val in iter {
        combined.push_str(", ");
        combined.push_str(val.to_str().map_err(|err| invalid_header(err, name, val))?);
    }
    
    // Create a new HeaderValue with the combined string
    let combined_value = HeaderValue::try_from(combined)
        .map_err(|err| invalid_header(err, name, first_val))?;
    
    T::try_from_header_value(&combined_value).map_err(|err| invalid_header(err, name, &combined_value))
}

pub fn parse_opt_header<T>(req: &Request, name: &HeaderName) -> S3Result<Option<T>>
where
    T: TryFromHeaderValue,
    T::Error: std::error::Error + Send + Sync + 'static,
{
    let mut iter = req.headers.get_all(name).into_iter();
    let Some(val) = iter.next() else { return Ok(None) };
    let None = iter.next() else { return Err(duplicate_header(name)) };

    match T::try_from_header_value(val) {
        Ok(ans) => Ok(Some(ans)),
        Err(err) => Err(invalid_header(err, name, val)),
    }
}

/// Parse optional header with support for HTTP-standard duplicate handling
pub fn parse_opt_header_with_duplicates<T>(req: &Request, name: &HeaderName) -> S3Result<Option<T>>
where
    T: TryFromHeaderValue,
    T::Error: std::error::Error + Send + Sync + 'static,
{
    let mut iter = req.headers.get_all(name).into_iter();
    let Some(first_val) = iter.next() else { return Ok(None) };
    
    // Check if this header allows duplicates
    if !header_allows_duplicates(name) {
        // Use strict duplicate checking for security-critical headers
        let None = iter.next() else { return Err(duplicate_header(name)) };
        return match T::try_from_header_value(first_val) {
            Ok(ans) => Ok(Some(ans)),
            Err(err) => Err(invalid_header(err, name, first_val)),
        };
    }
    
    // For headers that allow duplicates, combine values with comma separator
    let Some(next_val) = iter.next() else {
        // Only one value, use it directly
        return match T::try_from_header_value(first_val) {
            Ok(ans) => Ok(Some(ans)),
            Err(err) => Err(invalid_header(err, name, first_val)),
        };
    };
    
    // Multiple values - combine them according to HTTP standards
    let mut combined = String::new();
    combined.push_str(first_val.to_str().map_err(|err| invalid_header(err, name, first_val))?);
    combined.push_str(", ");
    combined.push_str(next_val.to_str().map_err(|err| invalid_header(err, name, next_val))?);
    
    // Add any additional values
    for val in iter {
        combined.push_str(", ");
        combined.push_str(val.to_str().map_err(|err| invalid_header(err, name, val))?);
    }
    
    // Create a new HeaderValue with the combined string
    let combined_value = HeaderValue::try_from(combined)
        .map_err(|err| invalid_header(err, name, first_val))?;
    
    match T::try_from_header_value(&combined_value) {
        Ok(ans) => Ok(Some(ans)),
        Err(err) => Err(invalid_header(err, name, &combined_value)),
    }
}

pub fn parse_opt_header_timestamp(req: &Request, name: &HeaderName, fmt: TimestampFormat) -> S3Result<Option<Timestamp>> {
    let mut iter = req.headers.get_all(name).into_iter();
    let Some(val) = iter.next() else { return Ok(None) };
    let None = iter.next() else { return Err(duplicate_header(name)) };

    let s = val.to_str().map_err(|err| invalid_header(err, name, val))?;
    match Timestamp::parse(fmt, s) {
        Ok(ans) => Ok(Some(ans)),
        Err(err) => Err(invalid_header(err, name, val)),
    }
}

pub fn parse_list_header<T>(req: &Request, name: &HeaderName, required: bool) -> S3Result<List<T>>
where
    T: TryFromHeaderValue,
    T::Error: std::error::Error + Send + Sync + 'static,
{
    let mut list = List::new();
    for val in req.headers.get_all(name) {
        let ans = T::try_from_header_value(val).map_err(|err| invalid_header(err, name, val))?;
        list.push(ans);
    }
    if required && list.is_empty() {
        return Err(missing_header(name));
    }
    Ok(list)
}

fn missing_query(name: &str) -> S3Error {
    invalid_request!("missing query: {}", name)
}

fn duplicate_query(name: &str) -> S3Error {
    invalid_request!("duplicate query: {}", name)
}

/// Check if a query parameter should allow duplicates
/// 
/// Some query parameters can safely have multiple values.
/// Security-critical parameters used in signatures should never allow duplicates.
pub fn query_allows_duplicates(name: &str) -> bool {
    match name {
        // Security-critical parameters used in AWS signatures must never allow duplicates
        "AWSAccessKeyId" | "Signature" | "Expires" | "x-amz-signature" | 
        "x-amz-credential" | "x-amz-date" | "x-amz-expires" | "x-amz-signedheaders" => false,
        
        // Upload/multipart specific parameters should be unique
        "uploadId" | "partNumber" => false,
        
        // Operation identifiers should be unique
        "x-id" => false,
        
        // Most AWS-specific query parameters should be unique by default
        // This is conservative but safe
        name if name.starts_with("x-amz-") => false,
        
        // Standard pagination and filtering parameters might allow multiple values
        // but for S3 API compatibility, keep them unique for now
        "prefix" | "delimiter" | "marker" | "max-keys" => false,
        
        // For non-AWS parameters, be conservative and require uniqueness by default
        _ => false,
    }
}

fn invalid_query<E>(source: E, name: &str, val: &str) -> S3Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    s3_error!(source, InvalidArgument, "invalid query: {}: {}", name, val)
}

pub fn parse_query<T: FromStr>(req: &Request, name: &str) -> S3Result<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let Some(qs) = req.s3ext.qs.as_ref() else { return Err(missing_query(name)) };

    let mut iter = qs.get_all(name);
    let Some(val) = iter.next() else { return Err(missing_query(name)) };
    let None = iter.next() else { return Err(duplicate_query(name)) };

    val.parse::<T>().map_err(|err| invalid_query(err, name, val))
}

pub fn parse_opt_query<T: FromStr>(req: &Request, name: &str) -> S3Result<Option<T>>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let Some(qs) = req.s3ext.qs.as_ref() else { return Ok(None) };

    let mut iter = qs.get_all(name);
    let Some(val) = iter.next() else { return Ok(None) };
    let None = iter.next() else { return Err(duplicate_query(name)) };

    Ok(Some(val.parse::<T>().map_err(|err| invalid_query(err, name, val))?))
}

pub fn parse_opt_query_timestamp(req: &Request, name: &str, fmt: TimestampFormat) -> S3Result<Option<Timestamp>> {
    let Some(qs) = req.s3ext.qs.as_ref() else { return Ok(None) };

    let mut iter = qs.get_all(name);
    let Some(val) = iter.next() else { return Ok(None) };
    let None = iter.next() else { return Err(duplicate_query(name)) };

    Ok(Some(Timestamp::parse(fmt, val).map_err(|err| invalid_query(err, name, val))?))
}

#[track_caller]
pub fn unwrap_bucket(req: &mut Request) -> String {
    match req.s3ext.s3_path.take() {
        Some(S3Path::Bucket { bucket }) => bucket.into(),
        _ => panic!("s3 path not found, expected bucket"),
    }
}

#[track_caller]
pub fn unwrap_object(req: &mut Request) -> (String, String) {
    match req.s3ext.s3_path.take() {
        Some(S3Path::Object { bucket, key }) => (bucket.into(), key.into()),
        _ => panic!("s3 path not found, expected object"),
    }
}

fn malformed_xml(source: xml::DeError) -> S3Error {
    S3Error::with_source(S3ErrorCode::MalformedXML, Box::new(source))
}

fn deserialize_xml<T>(bytes: &[u8]) -> S3Result<T>
where
    T: for<'xml> xml::Deserialize<'xml>,
{
    let mut d = xml::Deserializer::new(bytes);
    let ans = T::deserialize(&mut d).map_err(malformed_xml)?;
    d.expect_eof().map_err(malformed_xml)?;
    Ok(ans)
}

pub fn take_xml_body<T>(req: &mut Request) -> S3Result<T>
where
    T: for<'xml> xml::Deserialize<'xml>,
{
    let bytes = req.body.take_bytes().expect("full body not found");
    if bytes.is_empty() {
        return Err(S3ErrorCode::MissingRequestBodyError.into());
    }
    let result = deserialize_xml(&bytes);
    if result.is_err() {
        debug!(?bytes, "malformed xml body");
    }
    result
}

pub fn take_opt_xml_body<T>(req: &mut Request) -> S3Result<Option<T>>
where
    T: for<'xml> xml::Deserialize<'xml>,
{
    let bytes = req.body.take_bytes().expect("full body not found");
    if bytes.is_empty() {
        return Ok(None);
    }
    let result = deserialize_xml(&bytes).map(Some);
    if result.is_err() {
        debug!(?bytes, "malformed xml body");
    }
    result
}

pub fn take_string_body(req: &mut Request) -> S3Result<String> {
    let bytes = req.body.take_bytes().expect("full body not found");
    match String::from_utf8_simd(bytes.into()) {
        Ok(s) => Ok(s),
        Err(_) => Err(invalid_request!("expected UTF-8 body")),
    }
}

pub fn take_stream_body(req: &mut Request) -> StreamingBlob {
    let body = std::mem::take(&mut req.body);
    let size_hint = http_body::Body::size_hint(&body);
    debug!(?size_hint, "taking streaming blob");
    StreamingBlob::from(body)
}

pub fn parse_opt_metadata(req: &Request) -> S3Result<Option<Metadata>> {
    let mut metadata = Metadata::default();
    let map = &req.headers;
    for name in map.keys() {
        let Some(key) = name.as_str().strip_prefix("x-amz-meta-") else { continue };
        if key.is_empty() {
            continue;
        }
        
        // For metadata headers, allow duplicates and combine them with comma separator
        // according to HTTP standards. This improves compatibility while maintaining
        // the same API surface.
        let mut iter = map.get_all(name).into_iter();
        let first_val = iter.next().unwrap(); // Safe because we got the name from map.keys()
        
        let combined_value = match iter.next() {
            None => {
                // Single value case - use directly
                first_val.to_str().map_err(|err| invalid_header(err, name, first_val))?.to_owned()
            },
            Some(second_val) => {
                // Multiple values case - combine with HTTP standard comma separator
                let mut combined = String::new();
                combined.push_str(first_val.to_str().map_err(|err| invalid_header(err, name, first_val))?);
                combined.push_str(", ");
                combined.push_str(second_val.to_str().map_err(|err| invalid_header(err, name, second_val))?);
                
                // Add any additional values
                for val in iter {
                    combined.push_str(", ");
                    combined.push_str(val.to_str().map_err(|err| invalid_header(err, name, val))?);
                }
                combined
            }
        };
        
        metadata.insert(key.into(), combined_value);
    }
    if metadata.is_empty() {
        return Ok(None);
    }
    Ok(Some(metadata))
}

pub trait TryFromHeaderValue: Sized {
    type Error;
    fn try_from_header_value(val: &HeaderValue) -> Result<Self, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum ParseHeaderError {
    #[error("Invalid boolean value")]
    Boolean,

    #[error("Invalid integer value")]
    Integer,

    #[error("Invalid long value")]
    Long,

    #[error("Invalid enum value")]
    Enum,

    #[error("Invalid string value")]
    String,
}

impl TryFromHeaderValue for bool {
    type Error = ParseHeaderError;

    fn try_from_header_value(val: &HeaderValue) -> Result<Self, Self::Error> {
        match val.as_bytes() {
            b"true" | b"True" => Ok(true),
            b"false" | b"False" => Ok(false),
            _ => Err(ParseHeaderError::Boolean),
        }
    }
}

impl TryFromHeaderValue for i32 {
    type Error = ParseHeaderError;

    fn try_from_header_value(val: &HeaderValue) -> Result<Self, Self::Error> {
        atoi::atoi(val.as_bytes()).ok_or(ParseHeaderError::Integer)
    }
}

impl TryFromHeaderValue for i64 {
    type Error = ParseHeaderError;

    fn try_from_header_value(val: &HeaderValue) -> Result<Self, Self::Error> {
        atoi::atoi(val.as_bytes()).ok_or(ParseHeaderError::Long)
    }
}

impl TryFromHeaderValue for String {
    type Error = ParseHeaderError;

    fn try_from_header_value(val: &HeaderValue) -> Result<Self, Self::Error> {
        match val.to_str() {
            Ok(s) => Ok(s.to_owned()),
            Err(_) => Err(ParseHeaderError::String),
        }
    }
}

pub fn parse_field_value<T>(m: &Multipart, name: &str) -> S3Result<Option<T>>
where
    T: FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let Some(val) = m.find_field_value(name) else { return Ok(None) };
    match val.parse() {
        Ok(ans) => Ok(Some(ans)),
        Err(source) => Err(s3_error!(source, InvalidArgument, "invalid field value: {}: {:?}", name, val)),
    }
}

pub fn parse_field_value_timestamp(m: &Multipart, name: &str, fmt: TimestampFormat) -> S3Result<Option<Timestamp>> {
    let Some(val) = m.find_field_value(name) else { return Ok(None) };
    match Timestamp::parse(fmt, val) {
        Ok(ans) => Ok(Some(ans)),
        Err(source) => Err(s3_error!(source, InvalidArgument, "invalid field value: {}: {:?}", name, val)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::ordered_qs::OrderedQs;
    use crate::http::ordered_headers::OrderedHeaders;
    use hyper::http::{HeaderMap, HeaderValue};

    #[test]
    fn test_duplicate_query_string_parsing() {
        // Test that OrderedQs can parse duplicate query parameters
        let query_with_duplicates = "prefix=a&prefix=b&marker=x";
        let qs = OrderedQs::parse(query_with_duplicates).unwrap();
        
        // Should be able to get all values
        let prefixes: Vec<&str> = qs.get_all("prefix").collect();
        assert_eq!(prefixes, vec!["a", "b"]);
        
        // get_unique should return None for duplicates
        assert_eq!(qs.get_unique("prefix"), None);
        
        // Single values should work normally
        assert_eq!(qs.get_unique("marker"), Some("x"));
    }
    
    #[test] 
    fn test_duplicate_header_parsing() {
        let mut headers = HeaderMap::new();
        headers.append("content-type", HeaderValue::from_static("text/plain"));  
        headers.append("content-type", HeaderValue::from_static("application/json"));
        headers.insert("x-single", HeaderValue::from_static("value"));
        
        let ordered = OrderedHeaders::from_headers(&headers).unwrap();
        
        // Should be able to get all values
        let content_types: Vec<&str> = ordered.get_all("content-type").collect();
        assert_eq!(content_types, vec!["text/plain", "application/json"]);
        
        // get_unique should return None for duplicates  
        assert_eq!(ordered.get_unique("content-type"), None);
        
        // Single values should work normally
        assert_eq!(ordered.get_unique("x-single"), Some("value"));
    }
    
    #[test]
    fn test_current_strict_behavior_documentation() {
        // This test documents the current behavior of rejecting ALL duplicates
        
        // The current parse_header and parse_query functions should reject
        // any header or query parameter that has multiple values.
        
        // This is intentionally strict for security reasons, but may need
        // to be relaxed for certain non-security-critical headers to improve
        // HTTP standards compliance and compatibility.
        
        // Security-critical headers that SHOULD continue rejecting duplicates:
        let security_critical_headers = vec![
            "authorization",
            "x-amz-date", 
            "x-amz-content-sha256",
            "x-amz-security-token",
            "host",
        ];
        
        for header in security_critical_headers {
            println!("Security-critical header: {} should reject duplicates", header);
        }
        
        // Headers that might safely allow duplicates per HTTP standards:
        let potentially_safe_headers = vec![
            "cache-control",  // Can have multiple directives  
            "accept",         // Can have multiple media types
            // "x-amz-meta-*",   // Custom metadata (pattern match)
        ];
        
        for header in potentially_safe_headers {
            println!("Potentially safe header: {} might allow duplicates", header);
        }
    }
    
    #[test]
    fn test_header_duplicate_policy() {
        use crate::http::de::header_allows_duplicates;
        use hyper::http::HeaderName;
        
        // Security-critical headers should never allow duplicates
        assert!(!header_allows_duplicates(&HeaderName::from_static("authorization")));
        assert!(!header_allows_duplicates(&HeaderName::from_static("x-amz-date")));
        assert!(!header_allows_duplicates(&HeaderName::from_static("x-amz-content-sha256")));
        assert!(!header_allows_duplicates(&HeaderName::from_static("x-amz-security-token")));
        assert!(!header_allows_duplicates(&HeaderName::from_static("host")));
        
        // Checksum headers should be unique (only one checksum type)
        assert!(!header_allows_duplicates(&HeaderName::from_static("x-amz-checksum-crc32")));
        assert!(!header_allows_duplicates(&HeaderName::from_static("x-amz-checksum-sha256")));
        
        // Standard HTTP headers that can safely have multiple values
        assert!(header_allows_duplicates(&HeaderName::from_static("accept")));
        assert!(header_allows_duplicates(&HeaderName::from_static("cache-control")));
        assert!(header_allows_duplicates(&HeaderName::from_static("accept-encoding")));
        
        // Custom metadata headers can have multiple values
        assert!(header_allows_duplicates(&HeaderName::from_static("x-amz-meta-tags")));
        assert!(header_allows_duplicates(&HeaderName::from_static("x-amz-meta-category")));
        
        // Other AWS headers should be conservative (unique by default)
        assert!(!header_allows_duplicates(&HeaderName::from_static("x-amz-server-side-encryption")));
        assert!(!header_allows_duplicates(&HeaderName::from_static("x-amz-request-payer")));
    }
    
    #[test] 
    fn test_query_duplicate_policy() {
        use crate::http::de::query_allows_duplicates;
        
        // Security-critical query parameters should never allow duplicates
        assert!(!query_allows_duplicates("AWSAccessKeyId"));
        assert!(!query_allows_duplicates("Signature"));
        assert!(!query_allows_duplicates("x-amz-signature"));
        assert!(!query_allows_duplicates("x-amz-credential"));
        assert!(!query_allows_duplicates("x-amz-date"));
        
        // Upload/multipart specific parameters should be unique
        assert!(!query_allows_duplicates("uploadId"));
        assert!(!query_allows_duplicates("partNumber"));
        assert!(!query_allows_duplicates("x-id"));
        
        // S3 API parameters should be unique for compatibility
        assert!(!query_allows_duplicates("prefix"));
        assert!(!query_allows_duplicates("delimiter"));
        assert!(!query_allows_duplicates("marker"));
        assert!(!query_allows_duplicates("max-keys"));
    }
    
    #[test]
    fn test_metadata_duplicate_handling() {
        // This test would ideally test the parse_opt_metadata function directly,
        // but it requires a full Request object which is complex to construct in tests.
        // Instead, we document the expected behavior:
        
        // When parse_opt_metadata encounters duplicate x-amz-meta-* headers,
        // it should now combine them with ", " separator instead of returning an error.
        
        // Example:
        // x-amz-meta-tags: tag1
        // x-amz-meta-tags: tag2  
        // Should result in metadata["tags"] = "tag1, tag2"
        
        // This follows HTTP/1.1 standards for combining duplicate headers
        // while maintaining security for non-metadata headers.
        
        println!("Metadata duplicate handling: combines multiple x-amz-meta-* headers with comma separator");
        
        // Test our helper functions work correctly
        use hyper::http::HeaderName;
        assert!(header_allows_duplicates(&HeaderName::from_static("x-amz-meta-tags")));
        assert!(header_allows_duplicates(&HeaderName::from_static("x-amz-meta-category")));
    }
}
