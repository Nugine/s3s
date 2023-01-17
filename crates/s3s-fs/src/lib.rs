#![forbid(unsafe_code)]
#![deny(clippy::all)]

#[macro_use]
mod error;

mod fs;
mod s3;
mod utils;

pub use self::error::*;
pub use self::fs::FileSystem;
