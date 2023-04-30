#![cfg_attr(docsrs, feature(doc_cfg))]
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
    clippy::bool_assert_comparison,  // I don't like `assert!(!expression)`. It's very misleading.
    clippy::multiple_crate_versions, // Sometimes not fixable
    clippy::module_name_repetitions,
)]
// TODO
#![allow(
    clippy::single_match_else,
    clippy::stable_sort_primitive,
    clippy::string_add_assign,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::transmute_ptr_to_ptr,
    clippy::trivially_copy_pass_by_ref,
    clippy::unchecked_duration_subtraction,
    clippy::unicode_not_nfc,
    clippy::uninlined_format_args,
    clippy::unnecessary_box_returns,
    clippy::unnecessary_join,
    clippy::unnecessary_wraps,
    clippy::unnested_or_patterns,
    clippy::unreadable_literal,
    clippy::unsafe_derive_deserialize,
    clippy::unused_async,
    clippy::unused_self,
    clippy::used_underscore_binding,
    clippy::verbose_bit_mask,
    clippy::wildcard_imports,
    clippy::zero_sized_map_values
)]

#[macro_use]
mod utils;

#[macro_use]
mod error;

mod header;
mod http;
mod ops;
mod request;
mod s3_trait;
mod sig_v2;
mod sig_v4;
mod xml;

pub mod auth;
pub mod dto;
pub mod path;
pub mod service;
pub mod stream;

pub use self::error::*;
pub use self::http::Body;
pub use self::request::S3Request;
pub use self::s3_trait::S3;
