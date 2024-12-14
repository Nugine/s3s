use super::dto::RustTypes;
use super::ops::Operations;
use super::rust;
use super::rust::default_value_literal;

use crate::declare_codegen;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ops::Not;

use codegen_writer::g;
use codegen_writer::glines;
use stdx::default::default;

pub fn codegen(ops: &Operations, rust_types: &RustTypes) {
    declare_codegen!();

    glines![
        "use super::*;"
        ""
        "use crate::dto::*;"
        ""
        "use std::io::Write;"
        ""
    ];

    let (root_type_names, field_type_names) = collect_xml_types(ops, rust_types);

    for (&ty_name, &xml_name) in &root_type_names {
        match xml_name {
            Some(xml_name) if xml_name != ty_name => {
                if can_impl_serialize(ty_name) {
                    g!("//   Serialize: {ty_name} {xml_name:?}");
                }
                if can_impl_deserialize(rust_types, ty_name) {
                    g!("// Deserialize: {ty_name} {xml_name:?}");
                }
            }
            _ => {
                if can_impl_serialize(ty_name) {
                    g!("//   Serialize: {ty_name}");
                }
                if can_impl_deserialize(rust_types, ty_name) {
                    g!("// Deserialize: {ty_name}");
                }
            }
        }
    }
    g!();
    for ty_name in &field_type_names {
        if can_impl_serialize_content(ty_name) {
            g!("//   SerializeContent: {ty_name}");
        }
        if can_impl_deserialize_content(rust_types, ty_name) {
            g!("// DeserializeContent: {ty_name}");
        }
    }
    g!();

    codegen_xml_serde(ops, rust_types, &root_type_names);
    codegen_xml_serde_content(ops, rust_types, &field_type_names);
}

pub fn is_xml_payload(field: &rust::StructField) -> bool {
    let streaming = field.type_ == "StreamingBlob" || field.type_ == "SelectObjectContentEventStream";
    field.position == "payload" && field.type_ != "Policy" && streaming.not()
}

pub fn is_xml_output(ty: &rust::Struct) -> bool {
    ty.xml_name.is_some() || ty.fields.iter().any(|field| field.position == "xml")
}

fn collect_xml_types<'a>(
    ops: &'a Operations,
    rust_types: &'a RustTypes,
) -> (BTreeMap<&'a str, Option<&'a str>>, BTreeSet<&'a str>) {
    let mut root_type_names: BTreeMap<&str, Option<&str>> = default();
    let mut field_type_names: BTreeSet<&str> = default();

    let mut q: VecDeque<&str> = default();

    for op in ops.values() {
        for ty_name in [op.input.as_str(), op.output.as_str()] {
            let rust_type = &rust_types[ty_name];
            let rust::Type::Struct(ty) = rust_type else { panic!() };

            if is_xml_output(ty) {
                root_type_names.insert(ty_name, None);
                field_type_names.insert(ty_name);
                q.push_back(ty_name);
            } else {
                assert!(ty.xml_name.is_none());
            }

            let mut payload_count = 0;
            for field in &ty.fields {
                if is_xml_payload(field) {
                    root_type_names.insert(&field.type_, field.xml_name.as_deref());
                    field_type_names.insert(&field.type_);
                    q.push_back(&field.type_);
                    payload_count += 1;
                }
            }
            assert!(payload_count <= 1);
        }
    }

    {
        let extra = ["Progress", "Stats", "AssumeRoleOutput"];
        for ty_name in extra {
            root_type_names.insert(ty_name, None);
            field_type_names.insert(ty_name);
            q.push_back(ty_name);
        }
    }

    while let Some(name) = q.pop_front() {
        let rust_type = &rust_types[name];
        match rust_type {
            rust::Type::Struct(ty) => {
                for field in &ty.fields {
                    let is_xml_field = field.position == "xml" || is_xml_payload(field);
                    if is_xml_field.not() {
                        continue;
                    }

                    let field_type = &rust_types[field.type_.as_str()];
                    if let rust::Type::List(list_ty) = field_type {
                        field_type_names.insert(list_ty.member.type_.as_str());
                        q.push_back(list_ty.member.type_.as_str());
                    } else {
                        field_type_names.insert(field.type_.as_str());
                        q.push_back(field.type_.as_str());
                    }
                }
            }
            rust::Type::Alias(_) => {}
            rust::Type::List(ty) => {
                field_type_names.insert(ty.member.type_.as_str());
                q.push_back(ty.member.type_.as_str());
            }
            rust::Type::StrEnum(ty) => {
                field_type_names.insert(ty.name.as_str());
            }
            rust::Type::StructEnum(ty) => {
                for variant in &ty.variants {
                    field_type_names.insert(variant.type_.as_str());
                    q.push_back(variant.type_.as_str());
                }
            }
            rust::Type::Provided(ty) => {
                assert!(matches!(ty.name.as_str(), "Body" | "Event"));
            }
            rust::Type::Map(_) => unimplemented!(),
            rust::Type::Timestamp(_) => {}
        }
    }

    (root_type_names, field_type_names)
}

const SPECIAL_TYPES: &[&str] = &["AssumeRoleOutput"];

fn can_impl_serialize(ty_name: &str) -> bool {
    if SPECIAL_TYPES.contains(&ty_name) {
        return false;
    }
    can_impl_serialize_content(ty_name)
}

fn can_impl_serialize_content(_ty_name: &str) -> bool {
    true
}

fn can_impl_deserialize(rust_types: &RustTypes, ty_name: &str) -> bool {
    if SPECIAL_TYPES.contains(&ty_name) {
        return false;
    }

    can_impl_deserialize_content(rust_types, ty_name)
}

fn can_impl_deserialize_content(rust_types: &RustTypes, ty_name: &str) -> bool {
    let rust_type = &rust_types[ty_name];
    match rust_type {
        rust::Type::Struct(ty) => {
            for field in &ty.fields {
                if matches!(field.position.as_str(), "header" | "query" | "metadata") {
                    return false;
                }
            }
        }
        rust::Type::Alias(_) => {}
        rust::Type::List(_) => {}
        rust::Type::Map(_) => {}
        rust::Type::Provided(_) => {}
        rust::Type::StrEnum(_) => {}
        rust::Type::StructEnum(_) => {}
        rust::Type::Timestamp(_) => {}
    }
    true
}

fn s3_unwrapped_xml_output(ops: &Operations, ty_name: &str) -> bool {
    ops.iter().any(|(_, op)| op.s3_unwrapped_xml_output && op.output == ty_name)
}

fn codegen_xml_serde(ops: &Operations, rust_types: &RustTypes, root_type_names: &BTreeMap<&str, Option<&str>>) {
    for (rust_type, xml_name) in root_type_names.iter().map(|(&name, xml_name)| (&rust_types[name], xml_name)) {
        let rust::Type::Struct(ty) = rust_type else { panic!("{rust_type:#?}") };

        // https://github.com/Nugine/s3s/pull/127
        if s3_unwrapped_xml_output(ops, &ty.name) {
            assert_eq!(ty.name, "GetBucketLocationOutput");
            continue; // manually implemented
        }

        // https://github.com/Nugine/s3s/issues/2
        let xml_name = xml_name.or(ty.xml_name.as_deref()).unwrap_or(&ty.name);

        if can_impl_serialize(&ty.name) {
            g!("impl Serialize for {} {{", ty.name);
            g!("fn serialize<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {{");

            g!("s.content(\"{xml_name}\", self)");

            g!("}}");
            g!("}}");

            g!();
        }

        if can_impl_deserialize(rust_types, &ty.name) {
            g!("impl<'xml> Deserialize<'xml> for {} {{", ty.name);
            g!("fn deserialize(d: &mut Deserializer<'xml>) -> DeResult<Self> {{");

            g!("d.named_element(\"{xml_name}\", Deserializer::content)");

            g!("}}");
            g!("}}");
            g!();
        }
    }
}

fn codegen_xml_serde_content(ops: &Operations, rust_types: &RustTypes, field_type_names: &BTreeSet<&str>) {
    for rust_type in field_type_names.iter().map(|&name| &rust_types[name]) {
        match rust_type {
            rust::Type::Alias(_) => {}
            rust::Type::Provided(_) => {}
            rust::Type::Timestamp(_) => {}
            rust::Type::List(_) => panic!(),
            rust::Type::Map(_) => panic!(),
            rust::Type::StrEnum(ty) => {
                {
                    g!("impl SerializeContent for {} {{", ty.name);
                    g!("fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {{");

                    g!("self.as_str().serialize_content(s)");

                    g!("}}");
                    g!("}}");
                }
                {
                    g!("impl<'xml> DeserializeContent<'xml> for {} {{", ty.name);
                    g!("fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {{");
                    g!("String::deserialize_content(d).map(Self::from)");
                    g!("}}");
                    g!("}}");
                }
            }
            rust::Type::StructEnum(ty) => {
                {
                    g!("impl SerializeContent for {} {{", ty.name);
                    g!("fn serialize_content<W: Write>(&self, s: &mut Serializer<W>) -> SerResult {{");

                    g!("match self {{");

                    for variant in &ty.variants {
                        g!("Self::{0}(x) => s.content(\"{0}\", x),", variant.name);
                    }

                    g!("}}");

                    g!("}}");
                    g!("}}");
                }
                {
                    g!("impl<'xml> DeserializeContent<'xml> for {} {{", ty.name);
                    g!("fn deserialize_content(d: &mut Deserializer<'xml>) -> DeResult<Self> {{");

                    g!("d.element(|d, x| match x {{");
                    for variant in &ty.variants {
                        g!("b\"{0}\" => Ok(Self::{0}(d.content()?)),", variant.name);
                    }
                    g!("_ => Err(DeError::UnexpectedTagName)");
                    g!("}})");

                    g!("}}");
                    g!("}}");
                }
            }
            rust::Type::Struct(ty) => codegen_xml_serde_content_struct(ops, rust_types, ty),
        }
    }
}

#[allow(clippy::too_many_lines)]
fn codegen_xml_serde_content_struct(_ops: &Operations, rust_types: &RustTypes, ty: &rust::Struct) {
    if can_impl_serialize_content(&ty.name) {
        g!("impl SerializeContent for {} {{", ty.name);
        g!(
            "fn serialize_content<W: Write>(&self, {}: &mut Serializer<W>) -> SerResult {{",
            if ty.fields.is_empty() { '_' } else { 's' }
        );

        for field in ty.fields.iter().filter(|x| x.position == "xml") {
            let xml_name = field.xml_name.as_ref().unwrap_or(&field.camel_name);

            let field_ty = &rust_types[field.type_.as_str()];
            if let rust::Type::List(list_ty) = field_ty {
                if field.option_type {
                    g!("if let Some(iter) = &self.{} {{", field.name);
                } else {
                    g!("{{");
                    g!("let iter = &self.{};", field.name);
                }
                if field.xml_flattened {
                    g!("s.flattened_list(\"{xml_name}\", iter)?;");
                } else {
                    let member_xml_name = list_ty.member.xml_name.as_deref().unwrap();
                    g!("s.list(\"{xml_name}\", \"{member_xml_name}\", iter)?;");
                }
                g!("}}");
            } else if let rust::Type::Timestamp(ts_ty) = field_ty {
                let fmt = ts_ty.format.as_deref().unwrap_or("DateTime");
                if field.option_type {
                    g!("if let Some(ref val) = self.{} {{", field.name);
                    g!("s.timestamp(\"{xml_name}\", val, TimestampFormat::{fmt})?;");
                    g!("}}");
                } else {
                    g!("s.timestamp(\"{}\", &self.{}, TimestampFormat::{})?;", xml_name, field.name, fmt);
                }
            } else if field.option_type {
                g!("if let Some(ref val) = self.{} {{", field.name);
                g!("s.content(\"{xml_name}\", val)?;");
                g!("}}");
            } else {
                let default_is_zero = match field.default_value.as_ref() {
                    Some(v) => v.as_u64() == Some(0),
                    None => false,
                };
                let skip_zero = default_is_zero && ty.name == "DefaultRetention"; // ASK: the real condition?

                if skip_zero {
                    g!("if self.{} != 0 {{", field.name);
                    g!("s.content(\"{}\", &self.{})?;", xml_name, field.name);
                    g!("}}");
                } else {
                    g!("s.content(\"{}\", &self.{})?;", xml_name, field.name);
                }
            }
        }

        g!("Ok(())");

        g!("}}");
        g!("}}");
        g!();
    }
    if can_impl_deserialize_content(rust_types, &ty.name) {
        g!("impl<'xml> DeserializeContent<'xml> for {} {{", ty.name);
        g!(
            "fn deserialize_content({}: &mut Deserializer<'xml>) -> DeResult<Self> {{",
            if ty.fields.is_empty() { "_" } else { "d" },
        );

        for field in &ty.fields {
            g!("let mut {}: Option<{}> = None;", field.name, field.type_);
        }

        if ty.fields.is_empty().not() {
            g!("d.for_each_element(|d, x| match x {{");
            for field in &ty.fields {
                let xml_name = field.xml_name.as_ref().unwrap_or(&field.camel_name);
                let field_name = field.name.as_str();
                let field_type = &rust_types[field.type_.as_str()];

                g!("b\"{xml_name}\" => {{");

                if let rust::Type::List(list_ty) = field_type {
                    if field.xml_flattened {
                        g!("let ans: {} = d.content()?;", list_ty.member.type_);
                        g!("{field_name}.get_or_insert_with(List::new).push(ans);");
                    } else {
                        let member_xml_name = list_ty.member.xml_name.as_deref().unwrap();

                        g!("if {field_name}.is_some() {{ return Err(DeError::DuplicateField); }}");
                        g!("{field_name} = Some(d.list_content(\"{member_xml_name}\")?);");
                    }
                } else if let rust::Type::Timestamp(ts_ty) = field_type {
                    let fmt = ts_ty.format.as_deref().unwrap_or("DateTime");

                    g!("if {field_name}.is_some() {{ return Err(DeError::DuplicateField); }}");

                    g!("{field_name} = Some(d.timestamp(TimestampFormat::{fmt})?);");
                } else {
                    g!("if {field_name}.is_some() {{ return Err(DeError::DuplicateField); }}");
                    g!("{field_name} = Some(d.content()?);");
                }

                g!("Ok(())");
                g!("}}");
            }
            g!("_ => Err(DeError::UnexpectedTagName)");
            g!("}})?;");
        }

        g!("Ok(Self {{");
        for field in &ty.fields {
            if let Some(ref default_value) = field.default_value {
                let literal = default_value_literal(default_value);
                g!("{0}: {0}.unwrap_or({1}),", field.name, literal);
                continue;
            }

            if field.option_type {
                g!("{},", field.name);
            } else {
                // g!("{0}: {0}.ok_or_else(||dbg!(DeError::MissingField))?,", field.name);
                g!("{0}: {0}.ok_or(DeError::MissingField)?,", field.name);
            }
        }
        g!("}})");

        g!("}}");
        g!("}}");
    }
}
