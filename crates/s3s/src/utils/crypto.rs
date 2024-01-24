use std::mem::MaybeUninit;

use hex_simd::{AsOut, AsciiCase};
use hyper::body::Bytes;

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

pub fn hex(data: impl AsRef<[u8]>) -> String {
    hex_simd::encode_to_string(data, hex_simd::AsciiCase::Lower)
}

/// `f(hex(src))`
fn hex_bytes32<R>(src: impl AsRef<[u8]>, f: impl FnOnce(&str) -> R) -> R {
    let buf: &mut [_] = &mut [MaybeUninit::uninit(); 64];
    let ans = hex_simd::encode_as_str(src.as_ref(), buf.as_out(), AsciiCase::Lower);
    f(ans)
}

#[cfg(not(all(feature = "openssl", not(windows))))]
fn sha256(data: &[u8]) -> impl AsRef<[u8; 32]> {
    use sha2::{Digest, Sha256};
    <Sha256 as Digest>::digest(data)
}

#[cfg(all(feature = "openssl", not(windows)))]
fn sha256(data: &[u8]) -> impl AsRef<[u8]> {
    use openssl::hash::{Hasher, MessageDigest};
    let mut h = Hasher::new(MessageDigest::sha256()).unwrap();
    h.update(data).unwrap();
    h.finish().unwrap()
}

#[cfg(not(all(feature = "openssl", not(windows))))]
fn sha256_chunk(chunk: &[Bytes]) -> impl AsRef<[u8; 32]> {
    use sha2::{Digest, Sha256};
    let mut h = <Sha256 as Digest>::new();
    chunk.iter().for_each(|data| h.update(data));
    h.finalize()
}

#[cfg(all(feature = "openssl", not(windows)))]
fn sha256_chunk(chunk: &[Bytes]) -> impl AsRef<[u8]> {
    use openssl::hash::{Hasher, MessageDigest};
    let mut h = Hasher::new(MessageDigest::sha256()).unwrap();
    chunk.iter().for_each(|data| h.update(data).unwrap());
    h.finish().unwrap()
}

/// `f(hex(sha256(data)))`
pub fn hex_sha256<R>(data: &[u8], f: impl FnOnce(&str) -> R) -> R {
    hex_bytes32(sha256(data).as_ref(), f)
}

/// `f(hex(sha256(chunk)))`
pub fn hex_sha256_chunk<R>(chunk: &[Bytes], f: impl FnOnce(&str) -> R) -> R {
    hex_bytes32(sha256_chunk(chunk).as_ref(), f)
}
