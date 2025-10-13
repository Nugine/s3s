use crate::http::Multipart;

pub struct PostSignatureV2<'a> {
    pub policy: &'a str,
    pub access_key_id: &'a str,
    pub signature: &'a str,
}

impl<'a> PostSignatureV2<'a> {
    pub fn extract(m: &'a Multipart) -> Option<Self> {
        let policy = m.find_field_value("policy")?;
        let access_key_id = m.find_field_value("awsaccesskeyid")?;
        let signature = m.find_field_value("signature")?;
        Some(Self {
            policy,
            access_key_id,
            signature,
        })
    }
}
