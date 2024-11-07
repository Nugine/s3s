#![forbid(unsafe_code)]
#![deny(
    clippy::all, //
    clippy::cargo, //
    clippy::pedantic, //
    clippy::self_named_module_files, //
)]
#![warn(
    clippy::dbg_macro, //
)]
#![allow(
    clippy::module_name_repetitions, //
    clippy::missing_errors_doc, // TODO
    clippy::missing_panics_doc, // TODO
    clippy::multiple_crate_versions, // TODO: check later
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
