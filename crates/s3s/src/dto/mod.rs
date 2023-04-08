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

mod event;
pub use self::event::Event;

mod event_stream;
pub use self::event_stream::*;

pub type List<T> = Vec<T>;
pub type Map<K, V> = std::collections::HashMap<K, V>;

pub type Body = hyper::body::Bytes;

pub type Unit = ();

impl From<ListObjectsInput> for ListObjectsV2Input {
    fn from(v1: ListObjectsInput) -> Self {
        let ListObjectsInput {
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
            fetch_owner: None,
            max_keys,
            prefix,
            request_payer,
            start_after: marker,
        }
    }
}
