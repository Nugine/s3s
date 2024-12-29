#![forbid(unsafe_code)]
#![deny(
    clippy::all, //
    clippy::pedantic, //
)]
#![warn(
    clippy::dbg_macro, //
)]
#![allow(
    clippy::single_match_else, //
    clippy::wildcard_imports,
    clippy::match_same_arms,
    clippy::let_underscore_untyped,
)]
#![allow(
    unknown_lints,
    // FIXME: https://github.com/rust-lang/rust-clippy/issues/13885
    clippy::literal_string_with_formatting_args,
)]

mod v1;
mod v2;

fn main() {
    v1::run();
    v2::run();
}
