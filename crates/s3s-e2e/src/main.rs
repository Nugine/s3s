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
    clippy::wildcard_imports,
)]

mod utils;

mod advanced;
mod basic;

use s3s_test::tcx::TestContext;

fn register(tcx: &mut TestContext) {
    basic::register(tcx);
    advanced::register(tcx);
}

s3s_test::main!(register);
