//! AWS restXml serializer
//!
//! See <https://smithy.io/2.0/aws/protocols/aws-restxml-protocol.html#xml-shape-serialization>
//!

use crate::dto::{self, Timestamp, TimestampFormat};
use crate::utils;

use std::fmt;
use std::io::Write;

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;
use rust_utils::str_from_ascii;

/// A data type that can be serialized with AWS restXml serializer
pub trait Serialize {
    /// Serializes the data type
    ///
    /// # Errors
    /// Returns an error if the serialization fails
    fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult;
}

/// A data type that can be serialized with AWS restXml serializer
pub trait SerializeContent {
    /// Serializes the content of the data type
    ///
    /// # Errors
    /// Returns an error if the serialization fails
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult;
}

/// AWS restXml serializer
pub struct Serializer<W: Write> {
    /// inner writer
    inner: Writer<W>,
}

/// XML serialization error
#[derive(Debug, thiserror::Error)]
#[error("XML serialization error: {inner}")]
pub struct SerError {
    /// inner error
    inner: quick_xml::Error,
}

/// XML serialization result
pub type SerResult<T = (), E = SerError> = std::result::Result<T, E>;

impl<W: Write> Serializer<W> {
    /// Creates a new serializer
    pub fn new(w: W) -> Self {
        Self { inner: Writer::new(w) }
    }

    /// Writes an event
    fn event(&mut self, event: Event<'_>) -> SerResult {
        self.inner.write_event(event).map_err(wrap_xml)
    }

    /// Writes an element
    fn element(&mut self, name: &str, f: impl FnOnce(&mut Self) -> SerResult) -> SerResult {
        self.event(start(name))?;
        f(self)?;
        self.event(end(name))
    }

    /// Serializes a type
    ///
    /// # Errors
    /// Returns an error if the underlying writer returns an error
    pub fn content<T: SerializeContent + ?Sized>(&mut self, name: &str, val: &T) -> SerResult {
        self.element(name, |s| val.serialize_content(s))
    }

    /// Serializes a flattened `list`
    ///
    /// # Errors
    /// Returns an error if the underlying writer returns an error
    pub fn flattened_list<'a, T: SerializeContent + ?Sized + 'a>(
        &mut self,
        name: &str,
        iter: impl IntoIterator<Item = &'a T>,
    ) -> SerResult {
        for member in iter {
            self.content(name, member)?;
        }
        Ok(())
    }

    /// Serializes a `list`
    ///
    /// # Errors
    /// Returns an error if the underlying writer returns an error
    pub fn list<'a, T: SerializeContent + ?Sized + 'a>(
        &mut self,
        name: &str,
        member_name: &str,
        iter: impl IntoIterator<Item = &'a T>,
    ) -> SerResult {
        self.element(name, |s| {
            for member in iter {
                s.content(member_name, member)?;
            }
            Ok(())
        })
    }

    /// Writes `<?xml version="1.0" encoding="UTF-8"?>` to the xml
    ///
    /// # Errors
    /// Returns an error if the underlying writer returns an error
    pub fn decl(&mut self) -> SerResult {
        self.event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
    }

    pub fn timestamp(&mut self, name: &str, val: &Timestamp, fmt: TimestampFormat) -> SerResult {
        utils::fmt_timestamp(val, fmt, |b| self.content(name, str_from_ascii(b).unwrap()))
    }
}

impl<W: Write> fmt::Debug for Serializer<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Serializer").finish_non_exhaustive()
    }
}

impl SerializeContent for bool {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.event(text(utils::fmt_boolean(*self)))
    }
}

impl SerializeContent for i32 {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        utils::fmt_integer(*self, |t| s.event(text(t)))
    }
}

impl SerializeContent for i64 {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        utils::fmt_long(*self, |t| s.event(text(t)))
    }
}

impl SerializeContent for str {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.event(text(self))
    }
}

impl SerializeContent for &'_ str {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.event(text(self))
    }
}

impl SerializeContent for String {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        s.event(text(self.as_str()))
    }
}

impl SerializeContent for dto::Event {
    fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {
        self.as_ref().serialize_content(s)
    }
}

/// wrap error
const fn wrap_xml(inner: quick_xml::Error) -> SerError {
    SerError { inner }
}

/// start event
fn start(name: &str) -> Event<'_> {
    Event::Start(BytesStart::new(name))
}

/// end event
fn end(name: &str) -> Event<'_> {
    Event::End(BytesEnd::new(name))
}

/// text event
fn text(content: &str) -> Event<'_> {
    Event::Text(BytesText::new(content))
}
