#![forbid(unsafe_code)]
#![deny(
    clippy::all, //
    clippy::must_use_candidate, //
    clippy::cargo, //
)]

#[macro_use]
mod error;

mod body;
mod event_stream;

pub mod conv;

mod connector;
pub use self::connector::Connector;

mod proxy;
pub use self::proxy::Proxy;
