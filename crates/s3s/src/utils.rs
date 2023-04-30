use crate::dto::{Timestamp, TimestampFormat};

use std::fmt::Write;
use std::future::Future;
use std::pin::Pin;

use arrayvec::{ArrayString, ArrayVec};

/// TODO(blocking): use `unicode_simd::from_ascii`
pub fn from_ascii(s: &[u8]) -> Option<&str> {
    ascii::AsciiStr::from_ascii(s).ok().map(ascii::AsciiStr::as_str)
}

/// TODO(blocking): SIMD
/// `https://github.com/rusticstuff/simdutf8/issues/73`
pub fn from_utf8_vec(v: Vec<u8>) -> Option<String> {
    String::from_utf8(v).ok()
}

/// on-stack formatting
pub const fn fmt_boolean(val: bool) -> &'static str {
    if val {
        "true"
    } else {
        "false"
    }
}

/// on-stack formatting
pub fn fmt_integer<T>(val: i32, f: impl FnOnce(&str) -> T) -> T {
    let mut buf = ArrayString::<16>::new();
    write!(&mut buf, "{val}").unwrap();
    f(buf.as_str())
}

/// on-stack formatting
pub fn fmt_long<T>(val: i64, f: impl FnOnce(&str) -> T) -> T {
    let mut buf = ArrayString::<32>::new();
    write!(&mut buf, "{val}").unwrap();
    f(buf.as_str())
}

pub fn fmt_usize<T>(val: usize, f: impl FnOnce(&str) -> T) -> T {
    let mut buf = ArrayString::<32>::new();
    write!(&mut buf, "{val}").unwrap();
    f(buf.as_str())
}

/// on-stack formatting
#[allow(clippy::unwrap_used)]
pub fn fmt_timestamp<T>(val: &Timestamp, fmt: TimestampFormat, f: impl FnOnce(&[u8]) -> T) -> T {
    let mut buf = ArrayVec::<u8, 32>::new();
    val.format(fmt, &mut buf).unwrap();
    f(&buf)
}

/// verify sha256 checksum string
pub fn is_sha256_checksum(s: &str) -> bool {
    let is_lowercase_hex = |&c: &u8| c.is_ascii_digit() || (b'a'..=b'f').contains(&c);
    s.len() == 64 && s.as_bytes().iter().all(is_lowercase_hex)
}

// /// lazy-initialized regex
// macro_rules! static_regex {
//     ($re: literal) => {{
//         use once_cell::sync::Lazy;
//         use regex::Regex;

//         static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(const_str::verified_regex!($re)).unwrap());

//         &*PATTERN
//     }};
// }

macro_rules! invalid_request {
    ($msg:literal) => {
        s3_error!(InvalidRequest, $msg)
    };
    ($fmt:literal, $($arg:tt)+) => {
        s3_error!(InvalidRequest, $fmt, $($arg)+)
    };
    ($source:expr, $($arg:tt)+) => {{
        let mut err = invalid_request!($($arg)+);
        err.set_source(Box::new($source));
        err
    }};
}

pub fn is_base64_encoded(bytes: &[u8]) -> bool {
    base64_simd::STANDARD.check(bytes).is_ok()
}

/// `Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>`
pub type SyncBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>;

pub fn stable_sort_by_first<T>(v: &mut [(T, T)])
where
    T: Ord,
{
    v.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
}

pub mod parser {
    pub struct Error;

    pub fn parse<'a, T>(input: &'a str, f: impl FnOnce(&mut Parser<'a>) -> Result<T>) -> Result<T> {
        let mut p = Parser::new(input);
        let val = f(&mut p)?;
        Ok(val)
    }

    pub struct Parser<'a> {
        input: &'a str,
    }

    pub type Result<T, E = Error> = std::result::Result<T, E>;

    impl<'a> Parser<'a> {
        fn new(input: &'a str) -> Self {
            Self { input }
        }

        pub fn nom<T>(&mut self, f: impl FnOnce(&'a str) -> nom::IResult<&'a str, T>) -> Result<T> {
            match f(self.input) {
                Ok((input, output)) => {
                    self.input = input;
                    Ok(output)
                }
                Err(_) => Err(Error),
            }
        }
    }
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

pub fn default<T: Default>() -> T {
    T::default()
}
