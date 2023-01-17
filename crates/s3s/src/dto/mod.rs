mod generated;
pub use self::generated::*;

mod streaming_blob;
pub use self::streaming_blob::*;

mod timestamp;
pub use self::timestamp::*;

mod copy_source;
pub use self::copy_source::*;

mod range;
pub use self::range::Range;

mod content_type;
pub use self::content_type::*;

pub type List<T> = Vec<T>;
pub type Map<K, V> = std::collections::HashMap<K, V>;

pub type Body = hyper::body::Bytes;

pub type Unit = ();

#[derive(Debug, thiserror::Error)]
#[error("ParseEnumError")]
pub struct ParseEnumError(());

impl From<ListObjectsRequest> for ListObjectsV2Request {
    fn from(v1: ListObjectsRequest) -> Self {
        let ListObjectsRequest {
            bucket,
            delimiter,
            encoding_type,
            expected_bucket_owner,
            marker,
            max_keys,
            prefix,
            request_payer,
        } = v1;

        Self {
            bucket,
            continuation_token: None,
            delimiter,
            encoding_type,
            expected_bucket_owner,
            fetch_owner: false,
            max_keys,
            prefix,
            request_payer,
            start_after: marker,
        }
    }
}
