#![allow(
    clippy::missing_errors_doc, // TODO
    clippy::missing_panics_doc, // TODO
    // Test framework is allowed to use panic operations
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
)]

mod error;
mod runner;
mod traits;

pub mod build;
pub mod cli;
pub mod report;
pub mod tcx;

pub use self::error::{Failed, Result};
pub use self::traits::*;
