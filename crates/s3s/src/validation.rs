//! Validation API for S3 bucket names

/// Trait for validating S3 names
///
/// Implementations should return `true` for valid names and `false` for invalid ones.
pub trait NameValidation: Send + Sync {
    /// Validate a bucket name
    fn validate_bucket_name(&self, name: &str) -> bool;
}

/// AWS-compliant name validation
#[derive(Debug, Clone, Default)]
pub struct AwsNameValidation;

impl NameValidation for AwsNameValidation {
    fn validate_bucket_name(&self, name: &str) -> bool {
        crate::path::check_bucket_name(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test implementation that allows all bucket names
    struct RelaxedValidation;

    impl NameValidation for RelaxedValidation {
        fn validate_bucket_name(&self, _name: &str) -> bool {
            true // Allow any bucket name
        }
    }

    #[test]
    fn test_default_validation() {
        let validator = AwsNameValidation;

        // Valid bucket names should pass
        assert!(validator.validate_bucket_name("valid-bucket"));
        assert!(validator.validate_bucket_name("my.example.bucket"));

        // Invalid bucket names should fail
        assert!(!validator.validate_bucket_name("InvalidBucket")); // Uppercase
        assert!(!validator.validate_bucket_name("invalid_bucket")); // Underscore
        assert!(!validator.validate_bucket_name("192.168.1.1")); // IP address
    }

    #[test]
    fn test_relaxed_validation() {
        let validator = RelaxedValidation;

        // All bucket names should pass, even invalid ones
        assert!(validator.validate_bucket_name("valid-bucket"));
        assert!(validator.validate_bucket_name("InvalidBucket")); // Uppercase - allowed
        assert!(validator.validate_bucket_name("invalid_bucket")); // Underscore - allowed
        assert!(validator.validate_bucket_name("192.168.1.1")); // IP address - allowed
        assert!(validator.validate_bucket_name("xn--example")); // xn-- prefix - allowed
        assert!(validator.validate_bucket_name("ab")); // Too short - allowed
        assert!(validator.validate_bucket_name(&"a".repeat(100))); // Too long - allowed
    }
}
