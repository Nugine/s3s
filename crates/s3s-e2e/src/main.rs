#![allow(
    clippy::missing_errors_doc, // TODO
    clippy::missing_panics_doc, // TODO
    clippy::wildcard_imports,
    // E2E tests are allowed to use panic operations
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
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
