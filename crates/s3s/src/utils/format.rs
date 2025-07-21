use crate::dto::{Timestamp, TimestampFormat};

use arrayvec::ArrayVec;

pub const fn fmt_boolean(val: bool) -> &'static str {
    if val { "true" } else { "false" }
}

pub fn fmt_integer<T>(val: i32, f: impl FnOnce(&str) -> T) -> T {
    let mut buf = itoa::Buffer::new();
    f(buf.format(val))
}

pub fn fmt_long<T>(val: i64, f: impl FnOnce(&str) -> T) -> T {
    let mut buf = itoa::Buffer::new();
    f(buf.format(val))
}

pub fn fmt_usize<T>(val: usize, f: impl FnOnce(&str) -> T) -> T {
    let mut buf = itoa::Buffer::new();
    f(buf.format(val))
}

pub fn fmt_timestamp<T>(val: &Timestamp, fmt: TimestampFormat, f: impl FnOnce(&[u8]) -> T) -> T {
    let mut buf = ArrayVec::<u8, 32>::new();
    #[allow(clippy::unwrap_used)] // Buffer is large enough for timestamp formatting
    val.format(fmt, &mut buf).unwrap();
    f(&buf)
}
