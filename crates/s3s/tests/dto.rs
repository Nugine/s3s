#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing
)]

use s3s::dto::GetObjectInput;

#[test]
fn builder() {
    let input = {
        let mut b = GetObjectInput::builder();
        b.set_bucket("hello".to_owned());
        b.set_key("world".to_owned());
        b.build().unwrap()
    };

    assert_eq!(input.bucket, "hello");
    assert_eq!(input.key, "world");
}
