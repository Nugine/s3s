use std::hash::Hasher;

use digest::Digest;
use numeric_cast::TruncatingCast;
use rust_utils::default::default;

use crate::fs::InternalInfo;

#[derive(Default)]
pub struct ChecksumCalculator {
    pub crc32: Option<crc32fast::Hasher>,
    pub crc32c: Option<crc32c::Crc32cHasher>,
    pub sha1: Option<sha1::Sha1>,
    pub sha256: Option<sha2::Sha256>,
}

impl ChecksumCalculator {
    pub fn update(&mut self, data: &[u8]) {
        if let Some(crc32) = &mut self.crc32 {
            crc32.update(data);
        }
        if let Some(crc32c) = &mut self.crc32c {
            crc32c.write(data);
        }
        if let Some(sha1) = &mut self.sha1 {
            sha1.update(data);
        }
        if let Some(sha256) = &mut self.sha256 {
            sha256.update(data);
        }
    }

    pub fn finalize(self) -> s3s::dto::Checksum {
        let mut ans: s3s::dto::Checksum = default();
        if let Some(crc32) = self.crc32 {
            let sum = crc32.finalize().to_be_bytes();
            ans.checksum_crc32 = Some(base64(&sum));
        }
        if let Some(crc32c) = self.crc32c {
            let sum = crc32c.finish().truncating_cast::<u32>().to_be_bytes();
            ans.checksum_crc32c = Some(base64(&sum));
        }
        if let Some(sha1) = self.sha1 {
            let sum = sha1.finalize();
            ans.checksum_sha1 = Some(base64(sum.as_ref()));
        }
        if let Some(sha256) = self.sha256 {
            let sum = sha256.finalize();
            ans.checksum_sha256 = Some(base64(sum.as_ref()));
        }
        ans
    }
}

fn base64(input: &[u8]) -> String {
    base64_simd::STANDARD.encode_to_string(input)
}

pub fn modify_internal_info(info: &mut serde_json::Map<String, serde_json::Value>, checksum: &s3s::dto::Checksum) {
    if let Some(checksum_crc32) = &checksum.checksum_crc32 {
        info.insert("checksum_crc32".to_owned(), serde_json::Value::String(checksum_crc32.clone()));
    }
    if let Some(checksum_crc32c) = &checksum.checksum_crc32c {
        info.insert("checksum_crc32c".to_owned(), serde_json::Value::String(checksum_crc32c.clone()));
    }
    if let Some(checksum_sha1) = &checksum.checksum_sha1 {
        info.insert("checksum_sha1".to_owned(), serde_json::Value::String(checksum_sha1.clone()));
    }
    if let Some(checksum_sha256) = &checksum.checksum_sha256 {
        info.insert("checksum_sha256".to_owned(), serde_json::Value::String(checksum_sha256.clone()));
    }
}

pub fn from_internal_info(info: &InternalInfo) -> s3s::dto::Checksum {
    let mut ans: s3s::dto::Checksum = default();
    if let Some(checksum_crc32) = info.get("checksum_crc32") {
        ans.checksum_crc32 = Some(checksum_crc32.as_str().unwrap().to_owned());
    }
    if let Some(checksum_crc32c) = info.get("checksum_crc32c") {
        ans.checksum_crc32c = Some(checksum_crc32c.as_str().unwrap().to_owned());
    }
    if let Some(checksum_sha1) = info.get("checksum_sha1") {
        ans.checksum_sha1 = Some(checksum_sha1.as_str().unwrap().to_owned());
    }
    if let Some(checksum_sha256) = info.get("checksum_sha256") {
        ans.checksum_sha256 = Some(checksum_sha256.as_str().unwrap().to_owned());
    }
    ans
}
