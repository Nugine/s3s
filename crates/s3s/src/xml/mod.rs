#![allow(clippy::missing_errors_doc)] // TODO

mod de;
pub use self::de::*;

mod ser;
pub use self::ser::*;

mod generated;

#[cfg(test)]
mod tests;
