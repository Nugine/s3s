use crate::gen::Codegen;
use crate::ops::Operations;
use crate::rust::codegen_doc;
use crate::{default, f, o};
use crate::{rust, smithy};

use std::collections::BTreeMap;
use std::ops::Not;

use heck::{ToShoutySnakeCase, ToSnakeCase};
use serde_json::Value;

pub fn to_type_name(shape_name: &str) -> &str {
    shape_name.strip_prefix("com.amazonaws.s3#").unwrap()
}

pub type RustTypes = BTreeMap<String, rust::Type>;

#[deny(clippy::shadow_unrelated)]
pub fn collect_rust_types(model: &smithy::Model, ops: &Operations) -> RustTypes {
    let mut space: BTreeMap<String, rust::Type> = default();
    let mut insert = |k: String, v: rust::Type| assert!(space.insert(k, v).is_none());

    for (shape_name, shape) in &model.shapes {
        let rs_shape_name = match to_type_name(shape_name) {
            "SelectObjectContentEventStream" => o("SelectObjectContentEvent"), // rename
            s => s.to_owned(),
        };

        let provided_types = [
            "Body",          //
            "StreamingBlob", //
            "CopySource",    //
            "Range",         //
            "ContentType",   //
            "Event",         //
        ];

        if provided_types.contains(&rs_shape_name.as_str()) {
            let ty = rust::Type::provided(&rs_shape_name);
            insert(rs_shape_name, ty);
            continue;
        }

        match shape {
            smithy::Shape::Boolean(shape) => {
                let ty = rust::Type::alias(&rs_shape_name, "bool", shape.traits.doc());
                insert(rs_shape_name, ty);
            }
            smithy::Shape::Integer(shape) => {
                let ty = rust::Type::alias(&rs_shape_name, "i32", shape.traits.doc());
                insert(rs_shape_name, ty);
            }
            smithy::Shape::Long(shape) => {
                let ty = rust::Type::alias(&rs_shape_name, "i64", shape.traits.doc());
                insert(rs_shape_name, ty);
            }
            smithy::Shape::String(shape) => {
                let ty = rust::Type::alias(&rs_shape_name, "String", shape.traits.doc());
                insert(rs_shape_name, ty);
            }
            smithy::Shape::Timestamp(shape) => {
                let format = shape.traits.timestamp_format().map(|s| match s {
                    "date-time" => "DateTime",
                    "http-date" => "HttpDate",
                    "epoch-seconds" => "EpochSeconds",
                    _ => unimplemented!(),
                });
                let ty = rust::Type::Timestamp(rust::Timestamp {
                    name: rs_shape_name.clone(),
                    format: format.map(o),
                    doc: shape.traits.doc().map(o),
                });
                insert(rs_shape_name, ty);
            }
            smithy::Shape::Blob(_) => {
                unimplemented!();
            }
            smithy::Shape::List(shape) => {
                let ty = rust::Type::List(rust::List {
                    name: rs_shape_name.clone(),
                    member: rust::ListMember {
                        type_: to_type_name(&shape.member.target).to_owned(),
                        xml_name: shape.member.traits.xml_name().map(o),
                    },
                    doc: shape.traits.doc().map(o),
                });
                insert(rs_shape_name, ty);
            }
            smithy::Shape::Map(shape) => {
                let ty = rust::Type::Map(rust::Map {
                    name: rs_shape_name.clone(),
                    key_type: to_type_name(&shape.key.target).to_owned(),
                    value_type: to_type_name(&shape.value.target).to_owned(),
                    doc: shape.traits.doc().map(o),
                });
                insert(rs_shape_name, ty);
            }
            smithy::Shape::Enum(shape) => {
                let mut variants = Vec::new();
                for (variant_name, variant) in &shape.members {
                    let rs_variant_name = match variant_name.as_str() {
                        "CRC32C" => o("CRC32C"),
                        _ => variant_name.to_shouty_snake_case(),
                    };

                    let value = variant.traits.enum_value().unwrap().to_owned();
                    assert!(value.is_ascii());

                    let variant = rust::StrEnumVariant {
                        name: rs_variant_name,
                        value,
                        doc: variant.traits.doc().map(o),
                    };
                    variants.push(variant);
                }
                let ty = rust::Type::StrEnum(rust::StrEnum {
                    name: rs_shape_name.clone(),
                    variants,
                    doc: shape.traits.doc().map(o),
                });
                insert(rs_shape_name, ty);
            }
            smithy::Shape::Structure(shape) => {
                let mut fields = Vec::new();
                for (field_name, field) in &shape.members {
                    let rs_field_name = if field_name == "Type" {
                        "type_".into()
                    } else {
                        field_name.to_snake_case()
                    };

                    let field_type = to_type_name(&field.target).to_owned();

                    let default_value = field.traits.default_value().map(o);
                    let is_required = field.traits.required();

                    let is_op_input = rs_shape_name
                        .strip_suffix("Request")
                        .map(|op| ops.contains_key(op))
                        .unwrap_or(false);

                    let option_type = 'optional: {
                        if field_type == "StreamingBlob" && default_value.as_ref().unwrap() == "" {
                            break 'optional true;
                        }
                        if is_op_input && is_required.not() {
                            break 'optional true;
                        }
                        is_required.not() && default_value.is_none()
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
                        name: rs_field_name,
                        type_: field_type,
                        doc: field.traits.doc().map(o),

                        camel_name: field_name.to_owned(),

                        option_type,
                        default_value,
                        is_required,

                        position,

                        http_header: field.traits.http_header().map(o),
                        http_query: field.traits.http_query().map(o),
                        xml_name: field.traits.xml_name().map(o),
                        xml_flattened: field.traits.xml_flattened(),
                    };
                    fields.push(field);
                }
                let ty = rust::Type::Struct(rust::Struct {
                    name: rs_shape_name.clone(),
                    fields,
                    doc: shape.traits.doc().map(ToOwned::to_owned),

                    xml_name: shape.traits.xml_name().map(o),
                    is_error_type: shape.traits.error().is_some(),
                });
                insert(rs_shape_name, ty);
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
                    name: rs_shape_name.clone(),
                    variants,
                    doc: shape.traits.doc().map(o),
                });
                insert(rs_shape_name, ty);
            }
            smithy::Shape::Operation(_) => {}
            smithy::Shape::Service(_) => {}
        }
    }

    patch_types(&mut space);
    unify_operation_types(ops, &mut space);

    space
}

fn patch_types(space: &mut RustTypes) {
    // patch LifecycleExpiration
    {
        let Some(rust::Type::Struct(ty)) = space.get_mut("LifecycleExpiration") else { panic!() };
        for field_name in ["days", "expired_object_delete_marker"] {
            let field = ty.fields.iter_mut().find(|x| x.name == field_name).unwrap();
            field.default_value = None;
            field.option_type = true;
        }
    }

    // patch SelectObjectContent input
    {
        let Some(rust::Type::Struct(mut ty)) = space.remove("SelectObjectContentRequest") else { panic!() };
        let request = rust::Struct {
            name: ty.name.clone(),
            fields: ty.fields.iter().filter(|x| x.position == "xml").cloned().collect(),
            doc: ty.doc.clone(),
            xml_name: None,
            is_error_type: false,
        };

        ty.fields.iter().for_each(|x| assert!(x.name != "request"));

        ty.fields.retain(|x| x.position != "xml");
        ty.fields.push(rust::StructField {
            name: o("request"),
            type_: request.name.clone(),
            doc: None,
            camel_name: request.name.clone(),
            option_type: false,
            default_value: None,
            is_required: false,
            position: o("payload"),
            http_header: None,
            http_query: None,
            xml_name: Some(request.name.clone()),
            xml_flattened: false,
        });
        ty.name = o("SelectObjectContentInput");

        space.insert("SelectObjectContentInput".into(), rust::Type::Struct(ty));
        space.insert("SelectObjectContentRequest".into(), rust::Type::Struct(request));
    }
}

fn unify_operation_types(ops: &Operations, space: &mut RustTypes) {
    // unify operation input type
    for op in ops.values() {
        if op.name == "SelectObjectContent" {
            continue;
        }
        let input_ty = if op.smithy_input == "Unit" {
            rust::Struct {
                name: op.input.clone(),
                fields: default(),
                doc: None,
                xml_name: None,
                is_error_type: false,
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
                is_error_type: false,
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
}

pub fn codegen(rust_types: &RustTypes, g: &mut Codegen) {
    g.lines([
        "//! Auto generated by codegen/src/dto.rs",
        "",
        "#![allow(clippy::empty_structs_with_brackets)]",
        "",
        "use super::*;",
        "",
        "use std::borrow::Cow;",
        "use std::convert::Infallible;",
        "use std::fmt;",
        "use std::str::FromStr;",
        "",
    ]);

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
            rust::Type::StrEnum(ty) => {
                codegen_str_enum(ty, rust_types, g);
            }
            rust::Type::Struct(ty) => {
                codegen_struct(ty, rust_types, g);
            }
            rust::Type::StructEnum(ty) => {
                codegen_struct_enum(ty, rust_types, g);
            }
            rust::Type::Timestamp(ty) => {
                codegen_doc(ty.doc.as_deref(), g);
                g.ln(f!("pub type {} = Timestamp;", ty.name));
            }
        }
        g.lf();
    }
}

fn codegen_struct(ty: &rust::Struct, _rust_types: &RustTypes, g: &mut Codegen) {
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
        g.ln("#[derive(Default)]");
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
    g.lf();

    g.ln(f!("impl fmt::Debug for {} {{", ty.name));
    g.ln("fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {");
    g.ln(f!("let mut d = f.debug_struct(\"{}\");", ty.name));
    for field in &ty.fields {
        if field.option_type {
            g.ln(f!("if let Some(ref val) = self.{} {{", field.name));
            g.ln(f!("d.field(\"{}\", val);", field.name));
            g.ln("}");
        } else {
            g.ln(f!("d.field(\"{0}\", &self.{0});", field.name));
        }
    }
    g.ln("d.finish_non_exhaustive()");
    g.ln("}");
    g.ln("}");
}

fn codegen_str_enum(ty: &rust::StrEnum, _rust_types: &RustTypes, g: &mut Codegen) {
    codegen_doc(ty.doc.as_deref(), g);
    g.ln("#[derive(Debug, Clone, PartialEq, Eq)]");
    g.ln(f!("pub struct {}(Cow<'static, str>);", ty.name));
    g.lf();

    g.ln(f!("impl {} {{", ty.name));
    {
        for variant in &ty.variants {
            codegen_doc(variant.doc.as_deref(), g);
            g.ln(f!("pub const {}: &str = \"{}\";", variant.name, variant.value));
            g.lf();
        }

        g.ln("#[must_use]");
        g.ln("pub fn as_str(&self) -> &str {");
        g.ln("&self.0");
        g.ln("}");
        g.lf();

        g.ln("#[must_use]");
        g.ln("pub fn from_static(s: &'static str) -> Self {");
        g.ln("Self(Cow::from(s))");
        g.ln("}");
        g.lf();
    }
    g.ln("}");
    g.lf();

    g.ln(f!("impl From<String> for {} {{", ty.name));
    g.ln("fn from(s: String) -> Self {");
    g.ln("Self(Cow::from(s))");
    g.ln("}");
    g.ln("}");
    g.lf();

    g.ln(f!("impl From<{}> for Cow<'static, str> {{", ty.name));
    g.ln(f!("fn from(s: {}) -> Self {{", ty.name));
    g.ln("s.0");
    g.ln("}");
    g.ln("}");
    g.lf();

    g.ln(f!("impl FromStr for {} {{", ty.name));
    g.ln("type Err = Infallible;");
    g.ln("fn from_str(s: &str) -> Result<Self, Self::Err> {");
    g.ln("Ok(Self::from(s.to_owned()))");
    g.ln("}");
    g.ln("}");
}

fn codegen_struct_enum(ty: &rust::StructEnum, _rust_types: &RustTypes, g: &mut Codegen) {
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
