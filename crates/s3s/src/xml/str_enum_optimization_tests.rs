#[cfg(test)]
#[allow(clippy::module_inception)]
mod str_enum_optimization_tests {
    use crate::dto::BucketVersioningStatus;
    use crate::xml::{Deserializer, DeserializeContent};

    #[test]
    fn test_str_enum_optimization_functional() {
        // Test case 1: Known static value "Enabled"
        let xml_enabled = br"Enabled";
        let mut deserializer = Deserializer::new(xml_enabled);
        let status = BucketVersioningStatus::deserialize_content(&mut deserializer).unwrap();
        
        assert_eq!(status.as_str(), "Enabled");
        assert_eq!(status.as_str(), BucketVersioningStatus::ENABLED);
        
        // Test case 2: Another known static value "Suspended"
        let xml_suspended = br"Suspended";
        let mut deserializer2 = Deserializer::new(xml_suspended);
        let status2 = BucketVersioningStatus::deserialize_content(&mut deserializer2).unwrap();
        
        assert_eq!(status2.as_str(), "Suspended");
        assert_eq!(status2.as_str(), BucketVersioningStatus::SUSPENDED);
        
        // Test case 3: Unknown value should still work correctly
        let xml_unknown = br"Unknown";
        let mut deserializer3 = Deserializer::new(xml_unknown);
        let status3 = BucketVersioningStatus::deserialize_content(&mut deserializer3).unwrap();
        
        assert_eq!(status3.as_str(), "Unknown");
    }

    #[test]
    fn test_static_vs_from_static() {
        // Create values using different methods and verify they're equivalent
        let static_enabled = BucketVersioningStatus::from_static(BucketVersioningStatus::ENABLED);
        let from_string_enabled = BucketVersioningStatus::from("Enabled".to_owned());
        
        assert_eq!(static_enabled.as_str(), from_string_enabled.as_str());
        
        // Verify pointer equality for static values (indirect test for optimization)
        assert_eq!(static_enabled.as_str().as_ptr(), BucketVersioningStatus::ENABLED.as_ptr());
    }

    #[test]
    fn test_xml_deserialization_with_various_enum_types() {
        use crate::dto::{BucketAccelerateStatus, ChecksumAlgorithm};
        
        // Test BucketAccelerateStatus optimization
        let xml_enabled = br"Enabled";
        let mut deserializer = Deserializer::new(xml_enabled);
        let accel_status = BucketAccelerateStatus::deserialize_content(&mut deserializer).unwrap();
        assert_eq!(accel_status.as_str(), BucketAccelerateStatus::ENABLED);
        
        // Test ChecksumAlgorithm optimization  
        let xml_crc32 = br"CRC32";
        let mut deserializer2 = Deserializer::new(xml_crc32);
        let checksum_algo = ChecksumAlgorithm::deserialize_content(&mut deserializer2).unwrap();
        assert_eq!(checksum_algo.as_str(), ChecksumAlgorithm::CRC32);
    }
}