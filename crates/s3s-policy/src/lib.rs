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
)]

pub mod model;
pub mod pattern;

#[cfg(test)]
mod tests;
