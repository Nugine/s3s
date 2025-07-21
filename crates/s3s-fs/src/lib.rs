#![allow(
    clippy::wildcard_imports,
    clippy::missing_errors_doc, // TODO: docs
    clippy::let_underscore_untyped,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions, // TODO: check later
    // Temporary allows for implementation details - should be reviewed
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
)]

#[macro_use]
mod error;

mod checksum;
mod fs;
mod s3;
mod utils;

pub use self::error::*;
pub use self::fs::FileSystem;
