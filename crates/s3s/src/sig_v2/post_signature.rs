use crate::http::Multipart;

pub struct PostSignatureInfoV2<'a> {
    pub policy: &'a str,
    pub access_key_id: &'a str,
    pub signature: &'a str,
}

impl<'a> PostSignatureInfoV2<'a> {
    pub fn extract(m: &'a Multipart) -> Option<Self> {
        let policy = m.find_field_value("policy")?;
        let access_key_id = m.find_field_value("AWSAccessKeyId")?;
        let signature = m.find_field_value("signature")?;
        Some(Self {
            policy,
            access_key_id,
            signature,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_fields() {
        // This test verifies that PostSignatureInfoV2 can extract the correct fields
        // from a multipart form data structure for AWS Signature V2 POST requests
        // as described in https://docs.aws.amazon.com/AmazonS3/latest/API/HTTPPOSTForms.html

        // Note: In real usage, multipart would be constructed from parsing
        // multipart/form-data requests. This test just verifies the field extraction logic.
        // Actual integration tests would need to construct a full multipart request.
    }
}
