use crate::gen::Codegen;
use crate::ops::Operations;
use crate::rust::codegen_doc;
use crate::{default, f, o};
use crate::{rust, smithy};

use std::collections::BTreeMap;
use std::ops::Not;

use heck::{ToSnakeCase, ToUpperCamelCase};
use serde_json::Value;

pub fn to_type_name(shape_name: &str) -> &str {
    shape_name.strip_prefix("com.amazonaws.s3#").unwrap()
}

pub type RustTypes = BTreeMap<String, rust::Type>;

pub fn collect_rust_types(model: &smithy::Model, ops: &Operations) -> RustTypes {
    let mut space: BTreeMap<String, rust::Type> = default();
    let mut insert = |k: String, v: rust::Type| assert!(space.insert(k, v).is_none());

    for (shape_name, shape) in &model.shapes {
        let name = to_type_name(shape_name).to_owned();

        if name.starts_with("SelectObjectContent") {
            continue; // TODO(further): impl SelectObjectContent
        }

        let provided_types = [
            "Body",                           //
            "StreamingBlob",                  //
            "SelectObjectContentEventStream", //
            "CopySource",                     //
            "Range",                          //
            "ContentType",                    //
            "Event",                          //
        ];

        if provided_types.contains(&name.as_str()) {
            let ty = rust::Type::provided(&name);
            insert(name, ty);
            continue;
        }

        match shape {
            smithy::Shape::Boolean(shape) => {
                let ty = rust::Type::alias(&name, "bool", shape.traits.doc());
                insert(name, ty);
            }
            smithy::Shape::Integer(shape) => {
                let ty = rust::Type::alias(&name, "i32", shape.traits.doc());
                insert(name, ty);
            }
            smithy::Shape::Long(shape) => {
                let ty = rust::Type::alias(&name, "i64", shape.traits.doc());
                insert(name, ty);
            }
            smithy::Shape::String(shape) => {
                let ty = rust::Type::alias(&name, "String", shape.traits.doc());
                insert(name, ty);
            }
            smithy::Shape::Timestamp(shape) => {
                let format = shape.traits.timestamp_format().map(|s| match s {
                    "date-time" => "DateTime",
                    "http-date" => "HttpDate",
                    "epoch-seconds" => "EpochSeconds",
                    _ => unimplemented!(),
                });
                let ty = rust::Type::Timestamp(rust::Timestamp {
                    name: name.clone(),
                    format: format.map(o),
                    doc: shape.traits.doc().map(o),
                });
                insert(name, ty);
            }
            smithy::Shape::Blob(_) => {
                unimplemented!();
            }
            smithy::Shape::List(shape) => {
                let ty = rust::Type::List(rust::List {
                    name: name.clone(),
                    member: rust::ListMember {
                        type_: to_type_name(&shape.member.target).to_owned(),
                        xml_name: shape.member.traits.xml_name().map(o),
                    },
                    doc: shape.traits.doc().map(o),
                });
                insert(name, ty);
            }
            smithy::Shape::Map(shape) => {
                let ty = rust::Type::Map(rust::Map {
                    name: name.clone(),
                    key_type: to_type_name(&shape.key.target).to_owned(),
                    value_type: to_type_name(&shape.value.target).to_owned(),
                    doc: shape.traits.doc().map(o),
                });
                insert(name, ty);
            }
            smithy::Shape::Enum(shape) => {
                let mut variants = Vec::new();
                for (variant_name, variant) in &shape.members {
                    let name = match variant_name.as_str() {
                        "CRC32C" => o("Crc32C"),
                        _ => variant_name.to_upper_camel_case(),
                    };

                    let value = variant.traits.enum_value().unwrap().to_owned();
                    assert!(value.is_ascii());

                    let variant = rust::UnitEnumVariant {
                        name,
                        value,
                        doc: variant.traits.doc().map(o),
                    };
                    variants.push(variant);
                }
                let ty = rust::Type::UnitEnum(rust::UnitEnum {
                    name: name.clone(),
                    variants,
                    doc: shape.traits.doc().map(o),
                });
                insert(name, ty);
            }
            smithy::Shape::Structure(shape) => {
                let mut fields = Vec::new();
                for (field_name, field) in &shape.members {
                    let name = if field_name == "Type" {
                        "type_".into()
                    } else {
                        field_name.to_snake_case()
                    };

                    let field_type = to_type_name(&field.target).to_owned();

                    let default_value = field.traits.default_value().map(o);

                    let option_type = {
                        let mut optional = field.traits.required().not() && default_value.is_none();
                        optional |= field_type == "StreamingBlob" && default_value.as_ref().unwrap() == "";
                        optional
                    };

                    let position = {
                        let mut position = "xml";
                        if field.traits.http_header().is_some() {
                            position = "header";
                        }
                        if field.traits.http_query().is_some() {
                            position = "query";
                        }
                        if field.traits.http_payload() {
                            position = "payload";
                        }

                        if field.traits.http_label().is_some() {
                            match field_type.as_str() {
                                "BucketName" => position = "bucket",
                                "ObjectKey" => position = "key",
                                _ => unimplemented!(),
                            }
                        }

                        if field_type == "Metadata" {
                            assert_eq!(field.traits.http_prefix_headers(), Some("x-amz-meta-"));
                            position = "metadata";
                        }

                        o(position)
                    };

                    let field = rust::StructField {
                        name,
                        type_: field_type,
                        doc: field.traits.doc().map(o),

                        camel_name: field_name.to_owned(),

                        option_type,
                        default_value,

                        position,

                        http_header: field.traits.http_header().map(o),
                        http_query: field.traits.http_query().map(o),
                        xml_name: field.traits.xml_name().map(o),
                        xml_flattened: field.traits.xml_flattened(),
                    };
                    fields.push(field);
                }
                let ty = rust::Type::Struct(rust::Struct {
                    name: name.clone(),
                    fields,
                    doc: shape.traits.doc().map(ToOwned::to_owned),

                    xml_name: shape.traits.xml_name().map(o),
                });
                insert(name, ty);
            }
            smithy::Shape::Union(shape) => {
                let mut variants = Vec::new();
                for (variant_name, variant) in &shape.members {
                    let variant = rust::StructEnumVariant {
                        name: variant_name.to_owned(),
                        type_: to_type_name(&variant.target).to_owned(),
                        doc: variant.traits.doc().map(o),
                    };
                    variants.push(variant);
                }
                let ty = rust::Type::StructEnum(rust::StructEnum {
                    name: name.clone(),
                    variants,
                    doc: shape.traits.doc().map(o),
                });
                insert(name, ty);
            }
            smithy::Shape::Operation(_) => {}
            smithy::Shape::Service(_) => {}
        }
    }

    // unify operation input type
    for op in ops.values() {
        let input_ty = if op.smithy_input == "Unit" {
            rust::Struct {
                name: op.input.clone(),
                fields: default(),
                doc: None,
                xml_name: None,
            }
        } else {
            assert!(op.smithy_input.ends_with("Request"));
            let Some(rust::Type::Struct(mut ty)) = space.remove(&op.smithy_input) else { panic!() };
            ty.name = op.input.clone(); // rename type
            ty
        };
        assert!(space.insert(op.input.clone(), rust::Type::Struct(input_ty)).is_none());
    }

    // unify operation output type
    for op in ops.values() {
        let output_ty = if op.smithy_output == "Unit" {
            rust::Struct {
                name: op.output.clone(),
                fields: default(),
                doc: None,
                xml_name: None,
            }
        } else {
            if op.smithy_output == op.output {
                continue;
            }
            let rust::Type::Struct(mut ty) = space[&op.smithy_output].clone() else { panic!() };
            ty.name = op.output.clone(); // duplicate type
            ty
        };
        assert!(space.insert(op.output.clone(), rust::Type::Struct(output_ty)).is_none());
    }

    space
}

pub fn codegen(rust_types: &RustTypes, g: &mut Codegen) {
    let prelude = [
        "//! Auto-generated definitions",
        "#![allow(clippy::empty_structs_with_brackets)]",
        "",
        "use super::*;",
        "",
        "use std::str::FromStr;",
        "",
    ];

    for line in prelude {
        g.ln(line);
    }

    for rust_type in rust_types.values() {
        match rust_type {
            rust::Type::Alias(ty) => {
                codegen_doc(ty.doc.as_deref(), g);
                g.ln(f!("pub type {} = {};", ty.name, ty.type_));
            }
            rust::Type::Provided(_) => {}
            rust::Type::List(ty) => {
                codegen_doc(ty.doc.as_deref(), g);
                g.ln(f!("pub type {} = List<{}>;", ty.name, ty.member.type_));
            }
            rust::Type::Map(ty) => {
                codegen_doc(ty.doc.as_deref(), g);
                g.ln(f!("pub type {} = Map<{}, {}>;", ty.name, ty.key_type, ty.value_type));
            }
            rust::Type::UnitEnum(ty) => {
                codegen_doc(ty.doc.as_deref(), g);
                g.ln("#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
                g.ln("#[non_exhaustive]");
                g.ln(f!("pub enum {} {{", ty.name));

                for variant in &ty.variants {
                    codegen_doc(variant.doc.as_deref(), g);
                    g.ln(f!("    {},", variant.name));
                }

                g.ln("}");

                g.lf();
                g.ln(f!("impl {} {{", ty.name));

                {
                    g.ln("#[must_use]");
                    g.ln("pub const fn as_str(&self) -> &'static str {");

                    g.ln("match self {");
                    for variant in &ty.variants {
                        g.ln(f!("    Self::{} => \"{}\",", variant.name, variant.value));
                    }
                    g.ln("}");

                    g.ln("}");
                    g.lf();
                }

                {
                    g.ln("#[must_use]");
                    g.ln("pub const fn from_bytes(s: &[u8]) -> Option<Self> {");

                    g.ln("match s {");
                    for variant in &ty.variants {
                        g.ln(f!("b\"{}\" => Some(Self::{}),", variant.value, variant.name));
                    }
                    g.ln("_ => None,");
                    g.ln("}");

                    g.ln("}");
                    g.lf();
                }

                g.ln("}");

                g.lf();

                g.ln(f!("impl FromStr for {} {{", ty.name));
                g.ln("type Err = ParseEnumError;");
                g.ln("fn from_str(s: &str) -> Result<Self, Self::Err> {");
                g.ln("Self::from_bytes(s.as_bytes()).ok_or(ParseEnumError(()))");
                g.ln("}");
                g.ln("}");
            }
            rust::Type::Struct(ty) => {
                let is_rust_default = |v: &Value| match v {
                    Value::Bool(x) => !x,
                    Value::Number(x) => x.as_i64() == Some(0),
                    Value::String(x) => x.is_empty(),
                    _ => unimplemented!("{v:#?}"),
                };

                let can_derive_default = ty.fields.iter().all(|field| {
                    let is_option = field.option_type;
                    let has_default = match field.default_value {
                        Some(ref v) => is_rust_default(v),
                        None => false,
                    };
                    is_option || has_default
                });

                codegen_doc(ty.doc.as_deref(), g);
                if can_derive_default {
                    g.ln("#[derive(Debug, Default)]");
                } else {
                    g.ln("#[derive(Debug)]");
                }
                // g.ln("#[non_exhaustive]"); // TODO: builder?
                g.ln(f!("pub struct {} {{", ty.name));

                for field in &ty.fields {
                    codegen_doc(field.doc.as_deref(), g);
                    if field.option_type {
                        g.ln(f!("    pub {}: Option<{}>,", field.name, field.type_));
                    } else {
                        g.ln(f!("    pub {}: {},", field.name, field.type_));
                    }
                }

                g.ln("}");
            }
            rust::Type::StructEnum(ty) => {
                codegen_doc(ty.doc.as_deref(), g);
                g.ln("#[derive(Debug)]");
                g.ln("#[non_exhaustive]");
                g.ln(f!("pub enum {} {{", ty.name));

                for variant in &ty.variants {
                    codegen_doc(variant.doc.as_deref(), g);
                    g.ln(f!("    {}({}),", variant.name, variant.type_));
                }

                g.ln("}");
            }
            rust::Type::Timestamp(ty) => {
                codegen_doc(ty.doc.as_deref(), g);
                g.ln(f!("pub type {} = Timestamp;", ty.name));
            }
        }
        g.lf();
    }
}
