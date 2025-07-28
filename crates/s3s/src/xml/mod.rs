#![allow(clippy::missing_errors_doc)] // TODO

mod de;
pub use self::de::*;

mod ser;
pub use self::ser::*;

mod generated;
//mod generated_bak;

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
}
