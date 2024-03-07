pub mod crypto;
pub mod parser;

pub mod format;

pub fn stable_sort_by_first<T>(v: &mut [(T, T)])
where
    T: Ord,
{
    v.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
}

pub fn is_base64_encoded(bytes: &[u8]) -> bool {
    base64_simd::STANDARD.check(bytes).is_ok()
}

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
