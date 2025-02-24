use crate::dto::Checksum;

use std::hash::Hasher;

use digest::Digest;
use numeric_cast::TruncatingCast;
use stdx::default::default;

#[derive(Default)]
pub struct ChecksumHasher {
    pub crc32: Option<crc32fast::Hasher>,
    pub crc32c: Option<crc32c::Crc32cHasher>,
    pub sha1: Option<sha1::Sha1>,
    pub sha256: Option<sha2::Sha256>,
    pub crc64nvme: Option<crc64fast_nvme::Digest>,
}

impl ChecksumHasher {
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
        if let Some(crc64nvme) = &mut self.crc64nvme {
            crc64nvme.write(data);
        }
    }

    #[must_use]
    pub fn finalize(self) -> Checksum {
        let mut ans: Checksum = default();
        if let Some(crc32) = self.crc32 {
            let sum = crc32.finalize().to_be_bytes();
            ans.checksum_crc32 = Some(Self::base64(&sum));
        }
        if let Some(crc32c) = self.crc32c {
            let sum = crc32c.finish().truncating_cast::<u32>().to_be_bytes();
            ans.checksum_crc32c = Some(Self::base64(&sum));
        }
        if let Some(sha1) = self.sha1 {
            let sum = sha1.finalize();
            ans.checksum_sha1 = Some(Self::base64(sum.as_ref()));
        }
        if let Some(sha256) = self.sha256 {
            let sum = sha256.finalize();
            ans.checksum_sha256 = Some(Self::base64(sum.as_ref()));
        }
        if let Some(crc64nvme) = self.crc64nvme {
            let sum = crc64nvme.sum64().to_be_bytes();
            ans.checksum_crc64nvme = Some(Self::base64(&sum));
        }
        ans
    }

    fn base64(input: &[u8]) -> String {
        base64_simd::STANDARD.encode_to_string(input)
    }
}
