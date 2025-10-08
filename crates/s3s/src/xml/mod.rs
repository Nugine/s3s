#![allow(clippy::missing_errors_doc)] // TODO

mod de;
pub use self::de::*;

mod ser;
pub use self::ser::*;

#[cfg(feature = "minio")]
mod generated_minio;

#[cfg(not(feature = "minio"))]
mod generated;

mod manually {
    use super::*;

    use crate::dto::BucketLocationConstraint;
    use crate::dto::GetBucketLocationOutput;

    impl Serialize for GetBucketLocationOutput {
        fn serialize<W: std::io::Write>(&self, s: &mut Serializer<W>) -> SerResult {
            let xmlns = "http://s3.amazonaws.com/doc/2006-03-01/";
            if let Some(location_constraint) = &self.location_constraint {
                s.content_with_ns("LocationConstraint", xmlns, location_constraint)?;
            } else {
                s.content_with_ns("LocationConstraint", xmlns, "")?;
            }
            Ok(())
        }
    }

    impl<'xml> Deserialize<'xml> for GetBucketLocationOutput {
        fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
            let mut location_constraint: Option<BucketLocationConstraint> = None;
            d.for_each_element(|d, x| match x {
                b"LocationConstraint" => {
                    if location_constraint.is_some() {
                        return Err(DeError::DuplicateField);
                    }
                    let val: BucketLocationConstraint = d.content()?;
                    if !val.as_str().is_empty() {
                        location_constraint = Some(val);
                    }
                    Ok(())
                }
                _ => Err(DeError::UnexpectedTagName),
            })?;
            Ok(Self { location_constraint })
        }
    }

    use crate::dto::AssumeRoleOutput;

    impl Serialize for AssumeRoleOutput {
        fn serialize<W: std::io::Write>(&self, s: &mut Serializer<W>) -> SerResult {
            let xmlns = "https://sts.amazonaws.com/doc/2011-06-15/";
            s.element_with_ns("AssumeRoleResponse", xmlns, |s| {
                s.content("AssumeRoleResult", self) //
            })?;
            Ok(())
        }
    }

    impl<'xml> Deserialize<'xml> for AssumeRoleOutput {
        fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {
            d.named_element("AssumeRoleResponse", |d| {
                d.named_element("AssumeRoleResult", Self::deserialize_content) //
            })
        }
    }

    use crate::dto::ETag;
    use crate::dto::ParseETagError;

    use stdx::default::default;

    impl SerializeContent for ETag {
        fn serialize_content<W: std::io::Write>(&self, s: &mut Serializer<W>) -> SerResult {
            let val = self.value();
            if val.len() <= 64 {
                let mut buf: arrayvec::ArrayString<72> = default();
                buf.push('"');
                buf.push_str(val);
                buf.push('"');
                <str as SerializeContent>::serialize_content(&buf, s)
            } else {
                let buf = format!("\"{val}\"");
                <str as SerializeContent>::serialize_content(&buf, s)
            }
        }
    }

    impl<'xml> DeserializeContent<'xml> for ETag {
        fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {
            let val: String = d.content()?;

            // try to parse as quoted ETag first
            // fallback if the ETag is not quoted
            match ETag::parse_http_header(val.as_bytes()) {
                Ok(v) => Ok(v),
                Err(ParseETagError::InvalidFormat) => Ok(ETag::Strong(val)),
                Err(ParseETagError::InvalidChar) => Err(DeError::InvalidContent),
            }
        }
    }
}
