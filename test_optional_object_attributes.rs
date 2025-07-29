use s3s::dto::{ListObjectsV2Input, OptionalObjectAttributes, OptionalObjectAttributesList};

fn main() {
    // Test 1: Using None to omit the header entirely (this should work without causing a 400 error)
    let input_with_none = ListObjectsV2Input {
        bucket: "test-bucket".to_string(),
        continuation_token: None,
        delimiter: None,
        encoding_type: None,
        expected_bucket_owner: None,
        fetch_owner: None,
        max_keys: None,
        optional_object_attributes: None, // This should omit the header
        prefix: None,
        request_payer: None,
        start_after: None,
    };

    println!("Test 1: optional_object_attributes = None");
    println!("This should omit the x-amz-optional-object-attributes header entirely");
    
    // Test 2: Using Some with specific attributes
    let input_with_some = ListObjectsV2Input {
        bucket: "test-bucket".to_string(),
        continuation_token: None,
        delimiter: None,
        encoding_type: None,
        expected_bucket_owner: None,
        fetch_owner: None,
        max_keys: None,
        optional_object_attributes: Some(OptionalObjectAttributesList::from(vec![
            OptionalObjectAttributes::from_static(OptionalObjectAttributes::RESTORE_STATUS),
        ])),
        prefix: None,
        request_payer: None,
        start_after: None,
    };

    println!("Test 2: optional_object_attributes = Some([RestoreStatus])");
    println!("This should include the x-amz-optional-object-attributes header with RestoreStatus");

    // The key improvement: users can now set it to None instead of being forced to use an empty vec
    println!("✅ Success: optional_object_attributes is now Option<OptionalObjectAttributesList>");
}