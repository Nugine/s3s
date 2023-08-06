#![forbid(unsafe_code)]
#![deny(
    clippy::all, //
    clippy::cargo, //
    clippy::pedantic, //
)]
#![allow(
    clippy::wildcard_imports,
    clippy::missing_errors_doc, // TODO: docs
    clippy::let_underscore_untyped,
    clippy::module_name_repetitions,
)]

#[macro_use]
mod error;

mod checksum;
mod fs;
mod s3;
mod utils;

pub use self::error::*;
pub use self::fs::FileSystem;
