#![forbid(unsafe_code)]
#![deny(
    clippy::all, //
    clippy::cargo, //
    clippy::pedantic, //
)]
#![allow(
    clippy::wildcard_imports,
    clippy::missing_errors_doc, // TODO: docs
)]

#[macro_use]
mod error;

mod fs;
mod s3;
mod utils;

pub use self::error::*;
pub use self::fs::FileSystem;
