/// verify sha256 checksum string
pub fn is_sha256_checksum(s: &str) -> bool {
    // TODO: optimize
    let is_lowercase_hex = |c: u8| matches!(c, b'0'..=b'9' | b'a'..=b'f');
    s.len() == 64 && s.as_bytes().iter().copied().all(is_lowercase_hex)
}

/// `hmac_sha1(key, data)`
pub fn hmac_sha1(key: impl AsRef<[u8]>, data: impl AsRef<[u8]>) -> [u8; 20] {
    use hmac::{Hmac, Mac};
    use sha1::Sha1;

    let mut m = <Hmac<Sha1>>::new_from_slice(key.as_ref()).unwrap();
    m.update(data.as_ref());
    m.finalize().into_bytes().into()
}

/// `hmac_sha256(key, data)`
pub fn hmac_sha256(key: impl AsRef<[u8]>, data: impl AsRef<[u8]>) -> [u8; 32] {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut m = <Hmac<Sha256>>::new_from_slice(key.as_ref()).unwrap();
    m.update(data.as_ref());
    m.finalize().into_bytes().into()
}
