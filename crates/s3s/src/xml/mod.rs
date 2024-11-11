#![allow(clippy::missing_errors_doc)] // TODO

mod de;
pub use self::de::*;

mod ser;
pub use self::ser::*;

mod generated;

mod manually {
    use super::*;

    use quick_xml::events::BytesEnd;
    use quick_xml::events::BytesStart;
    use quick_xml::events::Event;

    use crate::dto::BucketLocationConstraint;
    use crate::dto::GetBucketLocationOutput;

    impl Serialize for GetBucketLocationOutput {
        fn serialize<W: std::io::Write>(&self, s: &mut Serializer<W>) -> SerResult {
            if let Some(location_constraint) = &self.location_constraint {
                s.content("LocationConstraint", location_constraint)?;
            } else {
                s.content("LocationConstraint", "")?;
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
            s.event(start_with_ns("AssumeRoleResponse", "https://sts.amazonaws.com/doc/2011-06-15/"))?;
            s.content("AssumeRoleResult", self)?;
            s.event(end("AssumeRoleResponse"))?;
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

    fn start_with_ns<'a>(name: &'a str, ns: &'a str) -> Event<'a> {
        let mut e = BytesStart::new(name);
        e.push_attribute(("xmlns", ns));
        Event::Start(e)
    }

    fn end(name: &str) -> Event<'_> {
        Event::End(BytesEnd::new(name))
    }
}
