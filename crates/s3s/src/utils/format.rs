use crate::dto::{Timestamp, TimestampFormat};

use std::fmt::Write;

use arrayvec::{ArrayString, ArrayVec};

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
