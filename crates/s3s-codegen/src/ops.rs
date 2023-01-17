use crate::dto::RustTypes;
use crate::gen::Codegen;
use crate::rust::codegen_doc;
use crate::{default, f, headers, o};
use crate::{dto, rust, smithy};

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fmt;
use std::ops::Not;

use heck::ToSnakeCase;
use serde_json::Value;

#[derive(Debug)]
pub struct Operation {
    pub name: String,
    pub input: String,
    pub output: String,
    pub doc: Option<String>,

    pub http_method: String,
    pub http_uri: String,
    pub http_code: u16,
}

pub type Operations = BTreeMap<String, Operation>;

pub fn collect_operations(model: &smithy::Model) -> Operations {
    let mut operations: Operations = default();
    let mut insert = |name, op| assert!(operations.insert(name, op).is_none());

    for (shape_name, shape) in &model.shapes {
        let smithy::Shape::Operation(sh) = shape else { continue };

        let name = dto::to_type_name(shape_name).to_owned();
        if name == "SelectObjectContent" {
            continue; // TODO(further): impl SelectObjectContent
        }

        let cvt = |sn| {
            if sn == "smithy.api#Unit" {
                "Unit"
            } else {
                dto::to_type_name(sn)
            }
        };
        let input = o(cvt(&sh.input.target));
        let output = o(cvt(&sh.output.target));

        let op = Operation {
            name: name.clone(),
            input,
            output,

            doc: sh.traits.doc().map(o),

            http_method: sh.traits.http_method().unwrap().to_owned(),
            http_uri: sh.traits.http_uri().unwrap().to_owned(),
            http_code: sh.traits.http_code().unwrap(),
        };
        insert(name, op)
    }

    operations
}

pub fn codegen(ops: &Operations, rust_types: &RustTypes, g: &mut Codegen) {
    let prelude = [
        "#![allow(clippy::declare_interior_mutable_const)]",
        "#![allow(clippy::borrow_interior_mutable_const)]",
        "",
        "use crate::dto::*;", //
        "use crate::header::names::*;",
        "use crate::http;",
        "use crate::xml;",
        "use crate::error::*;",
        "use crate::path::S3Path;",
        "",
        "use std::io::Write;",
        "",
    ];

    for line in prelude {
        g.ln(line)
    }

    codegen_async_trait(ops, g);

    for op in ops.values() {
        codegen_doc(op.doc.as_deref(), g);
        g.ln(f!("pub struct {};", op.name));
        g.ln("")
    }

    codegen_xml_ser(ops, rust_types, g);
    codegen_xml_de(ops, rust_types, g);
    codegen_http(ops, rust_types, g);
    codegen_router(ops, rust_types, g);
}

fn codegen_async_trait(ops: &Operations, g: &mut Codegen) {
    g.ln("#[async_trait::async_trait]");
    g.ln("pub trait S3: Send + Sync + 'static {");

    for op in ops.values() {
        let method_name = op.name.to_snake_case();

        let input_is_unit = op.input == "Unit";
        let output_is_unit = op.output == "Unit";

        match (input_is_unit, output_is_unit) {
            (false, false) => {
                g.ln(f!(
                    "async fn {method_name}(&self, _input: {}) -> S3Result<{}> {{",
                    op.input,
                    op.output
                ));
            }
            (false, true) => {
                g.ln(f!("async fn {method_name}(&self, _input: {}) -> S3Result {{", op.input));
            }
            (true, false) => {
                g.ln(f!("async fn {method_name}(&self) -> S3Result<{}> {{", op.output));
            }
            (true, true) => panic!(),
        }

        g.ln(f!("Err(s3_error!(NotImplemented, \"{} is not implemented yet\"))", op.name));
        g.ln("}");
        g.ln("")
    }

    g.ln("}");
    g.lf();
}

fn is_xml_payload(field: &rust::StructField) -> bool {
    let streaming = field.type_ == "StreamingBlob" || field.type_ == "SelectObjectContentEventStream";
    field.position == "payload" && field.type_ != "Policy" && streaming.not()
}

fn is_xml_output(ty: &rust::Struct) -> bool {
    ty.xml_name.is_some() || ty.fields.iter().any(|field| field.position == "xml")
}

fn codegen_xml_ser(ops: &Operations, rust_types: &RustTypes, g: &mut Codegen) {
    let mut root_type_names: BTreeSet<&str> = default();
    let mut field_type_names: BTreeSet<&str> = default();

    let mut q: VecDeque<&str> = default();

    for op in ops.values() {
        let ty_name = op.output.as_str();

        if ty_name == "Unit" {
            continue;
        }

        let rust_type = &rust_types[ty_name];
        let rust::Type::Struct(ty) = rust_type else { panic!() };

        if is_xml_output(ty) {
            root_type_names.insert(ty_name);
            field_type_names.insert(ty_name);
            q.push_back(ty_name);
        }

        let mut payload_count = 0;
        for field in &ty.fields {
            if is_xml_payload(field) {
                root_type_names.insert(&field.type_);
                field_type_names.insert(&field.type_);
                q.push_back(&field.type_);
                payload_count += 1;
            }
        }
        assert!(payload_count <= 1);
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
            rust::Type::UnitEnum(ty) => {
                field_type_names.insert(ty.name.as_str());
            }
            rust::Type::StructEnum(ty) => {
                for variant in &ty.variants {
                    field_type_names.insert(variant.type_.as_str());
                    q.push_back(variant.type_.as_str());
                }
            }
            rust::Type::Provided(ty) => {
                assert_eq!(ty.name, "Body");
            }
            rust::Type::Map(_) => unimplemented!(),
            rust::Type::Timestamp(_) => {}
        }
    }

    for rust_type in field_type_names.iter().map(|&name| &rust_types[name]) {
        match rust_type {
            rust::Type::Struct(ty) => {
                g.ln(f!("impl xml::SerializeContent for {} {{", ty.name));
                g.ln(f!(
                    "fn serialize_content<W: Write>(&self, {}: &mut xml::Serializer<W>) -> xml::SerResult {{",
                    if ty.fields.is_empty() { '_' } else { 's' }
                ));

                for field in ty.fields.iter().filter(|x| x.position == "xml") {
                    let xml_name = field.xml_name.as_ref().unwrap_or(&field.camel_name);

                    let field_ty = &rust_types[field.type_.as_str()];
                    if let rust::Type::List(list_ty) = field_ty {
                        if field.option_type {
                            g.ln(f!("if let Some(iter) = &self.{} {{", field.name));
                        } else {
                            g.ln("{");
                            g.ln(f!("let iter = &self.{};", field.name));
                        }
                        if field.xml_flattened {
                            g.ln(f!("s.flattened_list(\"{}\", iter)?;", xml_name));
                        } else {
                            let member_xml_name = list_ty.member.xml_name.as_deref().unwrap();
                            g.ln(f!("s.list(\"{}\", \"{}\", iter)?;", xml_name, member_xml_name));
                        }
                        g.ln("}");
                    } else if let rust::Type::Timestamp(ts_ty) = field_ty {
                        let fmt = ts_ty.format.as_deref().unwrap_or("DateTime");
                        if field.option_type {
                            g.ln(f!("if let Some(ref val) = self.{} {{", field.name));
                            g.ln(f!("s.timestamp(\"{}\", val, TimestampFormat::{})?;", xml_name, fmt));
                            g.ln("}");
                        } else {
                            g.ln(f!(
                                "s.timestamp(\"{}\", &self.{}, TimestampFormat::{})?;",
                                xml_name,
                                field.name,
                                fmt
                            ));
                        }
                    } else if field.option_type {
                        g.ln(f!("if let Some(ref val) = self.{} {{", field.name));
                        g.ln(f!("s.content(\"{}\", val)?;", xml_name));
                        g.ln("}");
                    } else {
                        g.ln(f!("s.content(\"{}\", &self.{})?;", xml_name, field.name));
                    }
                }

                g.ln("Ok(())");

                g.ln("}");
                g.ln("}");
            }
            rust::Type::UnitEnum(ty) => {
                g.ln(f!("impl xml::SerializeContent for {} {{", ty.name));
                g.ln("fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {");

                g.ln("self.as_str().serialize_content(s)");

                g.ln("}");
                g.ln("}");
            }
            rust::Type::StructEnum(ty) => {
                g.ln(f!("impl xml::SerializeContent for {} {{", ty.name));
                g.ln("fn serialize_content<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {");

                g.ln("match self {");

                for variant in &ty.variants {
                    g.ln(f!("Self::{0}(x) => s.content(\"{0}\", x),", variant.name));
                }

                g.ln("}");

                g.ln("}");
                g.ln("}");
            }
            rust::Type::Alias(_) => {}
            rust::Type::Provided(_) => {}
            rust::Type::Timestamp(_) => {}
            rust::Type::List(_) => panic!(),
            rust::Type::Map(_) => panic!(),
        }
        g.lf();
    }

    for rust_type in root_type_names.iter().map(|&name| &rust_types[name]) {
        let rust::Type::Struct(ty) = rust_type else { panic!("{rust_type:#?}") };

        g.ln(f!("impl xml::Serialize for {} {{", ty.name));
        g.ln("fn serialize<W: Write>(&self, s: &mut xml::Serializer<W>) -> xml::SerResult {");

        let xml_name = ty.xml_name.as_deref().unwrap_or(ty.name.as_str());
        g.ln(f!("s.content(\"{xml_name}\", self)"));

        g.ln("}");
        g.ln("}");

        g.lf();
    }
}

fn default_value_literal(v: &Value) -> &dyn fmt::Display {
    match v {
        Value::Bool(x) => x,
        Value::Number(x) => x,
        _ => unimplemented!(),
    }
}

fn codegen_xml_de(ops: &Operations, rust_types: &RustTypes, g: &mut Codegen) {
    let mut root_type_names: BTreeSet<&str> = default();
    let mut field_type_names: BTreeSet<&str> = default();

    let mut q: VecDeque<&str> = default();

    for op in ops.values() {
        let ty_name = op.input.as_str();
        if ty_name == "Unit" {
            continue;
        }

        let rust_type = &rust_types[ty_name];
        let rust::Type::Struct(ty) = rust_type else { panic!() };
        assert!(ty.xml_name.is_none());

        let mut payload_count = 0;
        for field in &ty.fields {
            if is_xml_payload(field) {
                root_type_names.insert(&field.type_);
                field_type_names.insert(&field.type_);
                q.push_back(&field.type_);
                payload_count += 1;
            }
        }
        assert!(payload_count <= 1);
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
            rust::Type::UnitEnum(ty) => {
                field_type_names.insert(ty.name.as_str());
            }
            rust::Type::StructEnum(ty) => {
                for variant in &ty.variants {
                    field_type_names.insert(variant.type_.as_str());
                    q.push_back(variant.type_.as_str());
                }
            }
            rust::Type::Provided(_) => unimplemented!(),
            rust::Type::Map(_) => unimplemented!(),
            rust::Type::Timestamp(_) => {}
        }
    }

    for rust_type in field_type_names.iter().map(|&name| &rust_types[name]) {
        match rust_type {
            rust::Type::Struct(ty) => {
                g.ln(f!("impl<'xml> xml::DeserializeContent<'xml> for {} {{", ty.name));
                g.ln(f!(
                    "fn deserialize_content({}: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {{",
                    if ty.fields.is_empty() { "_" } else { "d" },
                ));

                for field in &ty.fields {
                    assert!(field.position == "xml");
                }

                for field in &ty.fields {
                    g.ln(f!("let mut {}: Option<{}> = None;", field.name, field.type_));
                }

                if ty.fields.is_empty().not() {
                    g.ln("d.for_each_element(|d, x| match x {");
                    for field in &ty.fields {
                        let xml_name = field.xml_name.as_ref().unwrap_or(&field.camel_name);
                        let field_name = field.name.as_str();
                        let field_type = &rust_types[field.type_.as_str()];

                        g.ln(f!("b\"{xml_name}\" => {{"));

                        if let rust::Type::List(list_ty) = field_type {
                            if field.xml_flattened {
                                g.ln(f!("let ans: {} = d.content()?;", list_ty.member.type_));
                                g.ln(f!("{field_name}.get_or_insert_with(List::new).push(ans);"))
                            } else {
                                g.ln(f!(
                                    "if {field_name}.is_some() {{ return Err(xml::DeError::DuplicateField); }}"
                                ));
                                g.ln(f!("{field_name} = Some(d.list_content(\"member\")?);"));
                            }
                        } else if let rust::Type::Timestamp(ts_ty) = field_type {
                            let fmt = ts_ty.format.as_deref().unwrap_or("DateTime");

                            g.ln(f!(
                                "if {field_name}.is_some() {{ return Err(xml::DeError::DuplicateField); }}"
                            ));

                            g.ln(f!("{field_name} = Some(d.timestamp(TimestampFormat::{fmt})?);"));
                        } else {
                            g.ln(f!(
                                "if {field_name}.is_some() {{ return Err(xml::DeError::DuplicateField); }}"
                            ));
                            g.ln(f!("{field_name} = Some(d.content()?);"));
                        }

                        g.ln("Ok(())");
                        g.ln("}");
                    }
                    g.ln("_ => Err(xml::DeError::UnexpectedTagName)");
                    g.ln("})?;");
                }

                g.ln("Ok(Self {");
                for field in &ty.fields {
                    if let Some(ref default_value) = field.default_value {
                        let literal = default_value_literal(default_value);
                        g.ln(f!("{0}: {0}.unwrap_or({1}),", field.name, literal));
                        continue;
                    }

                    if field.option_type {
                        g.ln(f!("{},", field.name));
                    } else {
                        g.ln(f!("{0}: {0}.ok_or(xml::DeError::MissingField)?,", field.name));
                    }
                }
                g.ln("})");

                g.ln("}");
                g.ln("}");
            }
            rust::Type::UnitEnum(ty) => {
                g.ln(f!("impl<'xml> xml::DeserializeContent<'xml> for {} {{", ty.name));
                g.ln("fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {");

                g.ln("d.text(|t|Self::from_bytes(t.as_ref()).ok_or(xml::DeError::InvalidContent))");

                g.ln("}");
                g.ln("}");
            }
            rust::Type::StructEnum(ty) => {
                g.ln(f!("impl<'xml> xml::DeserializeContent<'xml> for {} {{", ty.name));
                g.ln("fn deserialize_content(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {");

                g.ln("d.element(|d, x| match x {");
                for variant in &ty.variants {
                    g.ln(f!("b\"{0}\" => Ok(Self::{0}(d.content()?)),", variant.name));
                }
                g.ln("_ => Err(xml::DeError::UnexpectedTagName)");
                g.ln("})");

                g.ln("}");
                g.ln("}");
            }

            rust::Type::Alias(_) => {}
            rust::Type::Provided(_) => panic!(),
            rust::Type::List(_) => panic!(),
            rust::Type::Map(_) => panic!(),
            rust::Type::Timestamp(_) => {}
        }
        g.lf();
    }

    for rust_type in root_type_names.iter().map(|&name| &rust_types[name]) {
        let rust::Type::Struct(ty) = rust_type else { panic!("{rust_type:#?}") };

        g.ln(f!("impl<'xml> xml::Deserialize<'xml> for {} {{", ty.name));
        g.ln("fn deserialize(d: &mut xml::Deserializer<'xml>) -> xml::DeResult<Self> {");

        let xml_name = ty.xml_name.as_ref().unwrap_or(&ty.name);
        g.ln(f!("d.named_element(\"{xml_name}\", |d|d.content())"));

        g.ln("}");
        g.ln("}");
        g.lf();
    }
}

fn status_code_name(code: u16) -> &'static str {
    match code {
        204 => "NO_CONTENT",
        _ => unimplemented!(),
    }
}

fn codegen_http(ops: &Operations, rust_types: &RustTypes, g: &mut Codegen) {
    codegen_header_value(ops, rust_types, g);

    for op in ops.values() {
        g.ln(f!("impl {} {{", op.name));

        codegen_op_http_de(op, rust_types, g);
        codegen_op_http_ser(op, rust_types, g);

        g.ln("}");
        g.lf();
    }

    for op in ops.values() {
        codegen_op_http_call(op, g);
        g.lf();
    }
}

fn codegen_header_value(ops: &Operations, rust_types: &RustTypes, g: &mut Codegen) {
    let mut unit_enum_names: BTreeSet<&str> = default();

    for op in ops.values() {
        for ty_name in [op.input.as_str(), op.output.as_str()] {
            let rust_type = &rust_types[ty_name];
            match rust_type {
                rust::Type::Provided(_) => {}
                rust::Type::Struct(ty) => {
                    for field in ty.fields.iter().filter(|field| field.position == "header") {
                        let field_type = &rust_types[field.type_.as_str()];
                        match field_type {
                            rust::Type::List(list_ty) => {
                                let member_type = &rust_types[list_ty.member.type_.as_str()];
                                if let rust::Type::UnitEnum(ty) = member_type {
                                    unit_enum_names.insert(ty.name.as_str());
                                }
                            }
                            rust::Type::UnitEnum(ty) => {
                                unit_enum_names.insert(ty.name.as_str());
                            }
                            rust::Type::Alias(_) => {}
                            rust::Type::Provided(_) => {}
                            rust::Type::Timestamp(_) => {}
                            _ => unimplemented!("{field_type:#?}"),
                        }
                    }
                }
                _ => unimplemented!(),
            }
        }
    }

    for rust_type in unit_enum_names.iter().map(|&x| &rust_types[x]) {
        let rust::Type::UnitEnum(ty) = rust_type else { panic!() };

        g.ln(f!("impl http::TryIntoHeaderValue for {} {{", ty.name));
        g.ln("type Error = std::convert::Infallible;");
        g.ln("fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {");
        g.ln("match self {");
        for variant in &ty.variants {
            g.ln(f!("Self::{0} => {{", variant.name));

            // TODO(blocking): inline_const https://github.com/rust-lang/rust/pull/104087
            // g.ln("Ok(const { http::HeaderValue::from_static(\"{}\") })");

            g.ln("Ok(http::HeaderValue::from_static(\"{}\"))");
            g.ln("}");
        }
        g.ln("}");
        g.ln("}");
        g.ln("}");
        g.lf();
    }

    for rust_type in unit_enum_names.iter().map(|&x| &rust_types[x]) {
        let rust::Type::UnitEnum(ty) = rust_type else { panic!() };

        g.ln(f!("impl http::TryFromHeaderValue for {} {{", ty.name));
        g.ln("type Error = http::ParseHeaderError;");
        g.ln("fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {");
        g.ln("    Self::from_bytes(val.as_bytes()).ok_or(http::ParseHeaderError::Enum)");
        g.ln("}");
        g.ln("}");
        g.lf();
    }
}

fn codegen_op_http_ser(op: &Operation, rust_types: &RustTypes, g: &mut Codegen) {
    let output = op.output.as_str();
    let rust_type = &rust_types[output];
    match rust_type {
        rust::Type::Provided(ty) => {
            assert_eq!(ty.name, "Unit");
            g.ln("pub fn serialize_http() -> http::Response {");

            if op.http_code == 200 {
                g.ln("http::Response::default()")
            } else {
                g.ln("let mut res = http::Response::default();");

                let code_name = status_code_name(op.http_code);
                g.ln(f!("*res.status_mut() = http::StatusCode::{};", code_name));

                g.ln("res");
            }

            g.ln("}");
        }
        rust::Type::Struct(ty) => {
            g.ln(f!("pub fn serialize_http(x: {}) -> S3Result<http::Response> {{", output));

            assert!(ty.fields.is_empty().not());
            for field in &ty.fields {
                assert!(["header", "metadata", "xml", "payload"].contains(&field.position.as_str()),);
            }

            g.ln("let mut res = http::Response::default();");

            if op.http_code != 200 {
                let code_name = status_code_name(op.http_code);
                g.ln(f!("*res.status_mut() = http::StatusCode::{};", code_name));
            }

            if is_xml_output(ty) {
                g.ln("http::set_xml_body(&mut res, &x)?;");
            } else if let Some(field) = ty.fields.iter().find(|x| x.position == "payload") {
                match field.type_.as_str() {
                    "Policy" => {
                        assert!(field.option_type);
                        g.ln(f!("if let Some(val) = x.{} {{", field.name));
                        g.ln("*res.body_mut() = http::Body::from(val);");
                        g.ln("}")
                    }
                    "StreamingBlob" => {
                        if field.option_type {
                            g.ln(f!("if let Some(val) = x.{} {{", field.name));
                            g.ln("http::set_stream_body(&mut res, val);");
                            g.ln("}")
                        } else {
                            g.ln(f!("http::set_stream_body(&mut res, x.{});", field.name));
                        }
                    }
                    "SelectObjectContentEventStream" => {
                        unimplemented!()
                    }
                    _ => {
                        if field.option_type {
                            g.ln(f!("if let Some(ref val) = x.{} {{", field.name));
                            g.ln("    http::set_xml_body(&mut res, val)?;");
                            g.ln("}")
                        } else {
                            g.ln(f!("http::set_xml_body(&mut res, &x.{})?;", field.name));
                        }
                    }
                }
            }

            for field in &ty.fields {
                if field.position == "header" {
                    let field_name = field.name.as_str();
                    let header_name = crate::headers::to_constant_name(field.http_header.as_deref().unwrap());

                    let field_type = &rust_types[field.type_.as_str()];
                    if let rust::Type::Timestamp(ts_ty) = field_type {
                        assert!(field.option_type);
                        let fmt = ts_ty.format.as_deref().unwrap_or("HttpDate");
                        g.ln(f!(
                            "http::add_opt_header_timestamp(&mut res, {header_name}, x.{field_name}, TimestampFormat::{fmt})?;"
                        ));
                    } else if field.option_type {
                        g.ln(f!("http::add_opt_header(&mut res, {header_name}, x.{field_name})?;"))
                    } else {
                        g.ln(f!("http::add_header(&mut res, {header_name}, x.{field_name})?;"))
                    }
                }
                if field.position == "metadata" {
                    assert!(field.option_type);
                    g.ln(f!("http::add_opt_metadata(&mut res, x.{})?;", field.name));
                }
            }

            g.ln("Ok(res)");

            g.ln("}");
        }
        _ => unimplemented!(),
    }
    g.lf();
}

fn codegen_op_http_de(op: &Operation, rust_types: &RustTypes, g: &mut Codegen) {
    let input = op.input.as_str();
    let rust_type = &rust_types[input];
    match rust_type {
        rust::Type::Provided(ty) => {
            assert_eq!(ty.name, "Unit");
        }
        rust::Type::Struct(ty) => {
            g.ln(f!(
                "pub fn deserialize_http(req: &mut http::Request) -> S3Result<{}> {{",
                input
            ));

            if op.name == "PutObject" {
                // POST object
                g.ln("if let Some(m) = req.extensions_mut().remove::<http::Multipart>() {");
                g.ln("    return Self::deserialize_http_multipart(req, m);");
                g.ln("}");
                g.lf();
            }

            let path_pattern = PathPattern::parse(op.http_uri.as_str());
            match path_pattern {
                PathPattern::Root => {}
                PathPattern::Bucket => {
                    if op.name != "WriteGetObjectResponse" {
                        g.ln("let bucket = http::unwrap_bucket(req);");
                        g.lf();
                    }
                }
                PathPattern::Object => {
                    g.ln("let (bucket, key) = http::unwrap_object(req);");
                    g.lf();
                }
            }

            for field in &ty.fields {
                match field.position.as_str() {
                    "bucket" => {
                        assert_eq!(field.name, "bucket");
                    }
                    "key" => {
                        assert_eq!(field.name, "key");
                    }
                    "query" => {
                        let query = field.http_query.as_deref().unwrap();

                        let field_type = &rust_types[&field.type_];

                        if let rust::Type::List(_) = field_type {
                            panic!()
                        } else if let rust::Type::Timestamp(ts_ty) = field_type {
                            assert!(field.option_type);
                            let fmt = ts_ty.format.as_deref().unwrap_or("DateTime");
                            g.ln(f!(
                                "let {}: Option<{}> = http::parse_opt_query_timestamp(req, \"{}\", TimestampFormat::{})?;",
                                field.name,
                                field.type_,
                                query,
                                fmt
                            ));
                        } else if field.option_type {
                            g.ln(f!(
                                "let {}: Option<{}> = http::parse_opt_query(req, \"{}\")?;",
                                field.name,
                                field.type_,
                                query
                            ));
                        } else if let Some(ref default_value) = field.default_value {
                            let literal = default_value_literal(default_value);
                            g.ln(f!(
                                "let {}: {} = http::parse_opt_query(req, \"{}\")?.unwrap_or({});",
                                field.name,
                                field.type_,
                                query,
                                literal,
                            ));
                        } else {
                            g.ln(f!(
                                "let {}: {} = http::parse_query(req, \"{}\")?;",
                                field.name,
                                field.type_,
                                query,
                            ));
                        }
                    }
                    "header" => {
                        let header = headers::to_constant_name(field.http_header.as_deref().unwrap());
                        let field_type = &rust_types[&field.type_];

                        if let rust::Type::List(_) = field_type {
                            assert!(field.option_type.not());
                            g.ln(f!(
                                "let {}: {} = http::parse_list_header(req, &{})?;",
                                field.name,
                                field.type_,
                                header
                            ));
                        } else if let rust::Type::Timestamp(ts_ty) = field_type {
                            assert!(field.option_type);
                            let fmt = ts_ty.format.as_deref().unwrap_or("HttpDate");
                            g.ln(f!(
                                "let {}: Option<{}> = http::parse_opt_header_timestamp(req, &{}, TimestampFormat::{})?;",
                                field.name,
                                field.type_,
                                header,
                                fmt
                            ));
                        } else if field.option_type {
                            g.ln(f!(
                                "let {}: Option<{}> = http::parse_opt_header(req, &{})?;",
                                field.name,
                                field.type_,
                                header
                            ));
                        } else if let Some(ref default_value) = field.default_value {
                            // ASK: content length
                            // In S3 smithy model, content-length has a default value (0).
                            // Why? Is it correct???

                            let literal = default_value_literal(default_value);
                            g.ln(f!(
                                "let {}: {} = http::parse_opt_header(req, &{})?.unwrap_or({});",
                                field.name,
                                field.type_,
                                header,
                                literal,
                            ));
                        } else {
                            g.ln(f!(
                                "let {}: {} = http::parse_header(req, &{})?;",
                                field.name,
                                field.type_,
                                header
                            ));
                        }
                    }
                    "metadata" => {
                        assert!(field.option_type);
                        g.ln(f!(
                            "let {}: Option<{}> = http::parse_opt_metadata(req)?;",
                            field.name,
                            field.type_
                        ));
                    }
                    "payload" => match field.type_.as_str() {
                        "Policy" => {
                            assert!(field.option_type.not());
                            g.ln(f!("let {}: {} = http::take_string_body(req)?;", field.name, field.type_));
                        }
                        "StreamingBlob" => {
                            assert!(field.option_type);
                            g.ln(f!(
                                "let {}: Option<{}> = Some(http::take_stream_body(req));",
                                field.name,
                                field.type_
                            ));
                        }
                        _ => {
                            if field.option_type {
                                g.ln(f!(
                                    "let {}: Option<{}> = http::take_opt_xml_body(req)?;",
                                    field.name,
                                    field.type_
                                ));
                            } else {
                                g.ln(f!("let {}: {} = http::take_xml_body(req)?;", field.name, field.type_));
                            }
                        }
                    },

                    _ => unimplemented!(),
                }
                g.lf();
            }

            g.ln(f!("Ok({} {{", input));
            for field in &ty.fields {
                match field.position.as_str() {
                    "bucket" | "key" | "query" | "header" | "metadata" | "payload" => {
                        g.ln(f!("{},", field.name));
                    }
                    _ => unimplemented!(),
                }
            }
            g.ln("})");

            g.ln("}");
            g.lf();

            if op.name == "PutObject" {
                codegen_op_http_de_multipart(op, rust_types, g);
            }
        }
        _ => unimplemented!(),
    }
    g.lf();
}

fn codegen_op_http_de_multipart(op: &Operation, rust_types: &RustTypes, g: &mut Codegen) {
    assert_eq!(op.name, "PutObject");

    g.ln(f!(
        "pub fn deserialize_http_multipart(req: &mut http::Request, mut m: http::Multipart) -> S3Result<{}> {{",
        op.input
    ));

    {
        g.ln("let (bucket, key) = http::unwrap_object(req);");
        g.lf();
    }

    {
        g.ln("let body: Option<StreamingBlob> = m.take_file_stream().map(StreamingBlob::wrap);");
        g.lf();
    }

    let rust::Type::Struct(ty) = &rust_types[op.input.as_str()] else { panic!() };

    for field in &ty.fields {
        match field.position.as_str() {
            "bucket" | "key" | "payload" => {}
            "header" => {
                let header = field.http_header.as_deref().unwrap();
                assert!(header.as_bytes().iter().all(|&x| x == b'-' || x.is_ascii_alphanumeric()));
                let header = header.to_ascii_lowercase();

                let field_type = &rust_types[field.type_.as_str()];

                if let rust::Type::Timestamp(ts_ty) = field_type {
                    assert!(field.option_type);
                    let fmt = ts_ty.format.as_deref().unwrap_or("HttpDate");
                    g.ln(f!(
                        "let {}: Option<{}> = http::parse_field_value_timestamp(&m, \"{}\", TimestampFormat::{})?;",
                        field.name,
                        field.type_,
                        header,
                        fmt
                    ));
                } else if field.option_type {
                    g.ln(f!(
                        "let {}: Option<{}> = http::parse_field_value(&m, \"{}\")?;",
                        field.name,
                        field.type_,
                        header
                    ));
                } else if let Some(ref default_value) = field.default_value {
                    g.ln(f!(
                        "let {}: {} = http::parse_field_value(&m, \"{}\")?.unwrap_or({});",
                        field.name,
                        field.type_,
                        header,
                        default_value_literal(default_value)
                    ));
                } else {
                    unimplemented!()
                }
            }
            "metadata" => {
                assert!(field.option_type);
                g.ln(f!("let {}: Option<{}> = {{", field.name, field.type_));
                g.ln(f!("    let mut metadata: {} = Default::default();", field.type_));
                g.ln("    for (name, value) in m.fields() {");
                g.ln("        if let Some(key) = name.strip_prefix(\"x-amz-meta-\") {");
                g.ln("            if key.is_empty() { continue; }");
                g.ln("            metadata.insert(key.to_owned(), value.to_owned());");
                g.ln("        }");
                g.ln("    }");
                g.ln("    if metadata.is_empty() { None } else { Some(metadata) }");
                g.ln("};");
            }
            _ => unimplemented!(),
        }
        g.lf();
    }

    g.ln(f!("Ok({} {{", op.input));
    for field in &ty.fields {
        g.ln(f!("{},", field.name));
    }
    g.ln("})");
    g.ln("}");
}

fn codegen_op_http_call(op: &Operation, g: &mut Codegen) {
    g.ln("#[async_trait::async_trait]");
    g.ln(f!("impl super::Operation for {} {{", op.name));

    g.ln("fn name(&self) -> &'static str {");
    g.ln(f!("\"{}\"", op.name));
    g.ln("}");
    g.lf();

    let arg = if op.input != "Unit" { "req" } else { "_" };
    g.ln(f!(
        "async fn call(&self, s3: &dyn S3, {arg}: &mut http::Request) -> S3Result<http::Response> {{"
    ));

    let method = op.name.to_snake_case();

    if op.input != "Unit" {
        g.ln("let input = Self::deserialize_http(req)?;");
        g.ln(f!("let result = s3.{}(input).await;", method));
    } else {
        g.ln(f!("let result = s3.{}().await;", method));
    }

    g.ln("let res = match result {");
    if op.output != "Unit" {
        g.ln("Ok(output) => Self::serialize_http(output)?,")
    } else {
        g.ln("Ok(()) => Self::serialize_http(),")
    }
    g.ln("Err(err) => super::serialize_error(err)?,");
    g.ln("};");

    g.ln("Ok(res)");

    g.ln("}");
    g.ln("}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PathPattern {
    Root,
    Bucket,
    Object,
}

impl PathPattern {
    fn parse(part: &str) -> Self {
        let path = match part.split_once('?') {
            None => part,
            Some((p, _)) => p,
        };

        assert!(path.starts_with('/'));

        if path == "/" {
            return Self::Root;
        }

        match path[1..].split_once('/') {
            None => Self::Bucket,
            Some(_) => Self::Object,
        }
    }

    fn query_tag(part: &str) -> Option<String> {
        let (_, q) = part.split_once('?')?;
        let qs: Vec<(String, String)> = serde_urlencoded::from_str(q).unwrap();
        assert!(qs.iter().filter(|(_, v)| v.is_empty()).count() <= 1);
        qs.into_iter().find(|(_, v)| v.is_empty()).map(|(n, _)| n)
    }

    fn query_patterns(part: &str) -> Vec<(String, String)> {
        let Some((_, q)) = part.split_once('?') else{ return Vec::new() };
        let mut qs: Vec<(String, String)> = serde_urlencoded::from_str(q).unwrap();
        qs.retain(|(n, v)| n != "x-id" && v.is_empty().not());
        qs
    }
}

struct Route<'a> {
    op: &'a Operation,
    query_tag: Option<String>,
    query_patterns: Vec<(String, String)>,
    required_headers: Vec<&'a str>,
    required_query_strings: Vec<&'a str>,
    needs_full_body: bool,
}

fn collect_routes<'a>(ops: &'a Operations, rust_types: &'a RustTypes) -> HashMap<String, HashMap<PathPattern, Vec<Route<'a>>>> {
    let mut ans: HashMap<String, HashMap<PathPattern, Vec<Route<'_>>>> = default();
    for op in ops.values() {
        let pat = PathPattern::parse(&op.http_uri);
        let map = ans.entry(op.http_method.clone()).or_default();
        let vec = map.entry(pat).or_default();

        vec.push(Route {
            op,
            query_tag: PathPattern::query_tag(&op.http_uri),
            query_patterns: PathPattern::query_patterns(&op.http_uri),

            required_headers: required_headers(op, rust_types),
            required_query_strings: required_query_strings(op, rust_types),

            needs_full_body: needs_full_body(op, rust_types),
        });
    }
    for map in ans.values_mut() {
        for vec in map.values_mut() {
            vec.sort_by_key(|r| r.op.name.as_str());
            vec.reverse();

            vec.sort_by_key(|r| r.required_headers.len());
            vec.sort_by_key(|r| r.required_query_strings.len());
            vec.sort_by_key(|r| r.query_patterns.len());
            vec.sort_by_key(|r| r.query_tag.is_some() as u8);
            vec.reverse();
        }
    }
    ans
}

fn required_headers<'a>(op: &Operation, rust_types: &'a RustTypes) -> Vec<&'a str> {
    if op.input == "Unit" {
        return default();
    }

    let input_type = &rust_types[op.input.as_str()];
    let rust::Type::Struct(ty) = input_type else { panic!() };

    let mut ans: Vec<&'a str> = default();
    for field in &ty.fields {
        let is_required = field.option_type.not() && field.default_value.is_none();
        if is_required && field.position == "header" {
            let header = field.http_header.as_deref().unwrap();
            ans.push(header)
        }
    }
    ans
}

fn required_query_strings<'a>(op: &Operation, rust_types: &'a RustTypes) -> Vec<&'a str> {
    if op.input == "Unit" {
        return default();
    }

    let input_type = &rust_types[op.input.as_str()];
    let rust::Type::Struct(ty) = input_type else { panic!() };

    let mut ans: Vec<&'a str> = default();
    for field in &ty.fields {
        let is_required = field.option_type.not() && field.default_value.is_none();
        if is_required && field.position == "query" {
            let header = field.http_query.as_deref().unwrap();
            ans.push(header)
        }
    }
    ans
}

fn needs_full_body(op: &Operation, rust_types: &RustTypes) -> bool {
    if op.input == "Unit" {
        return false;
    }
    if op.http_method == "GET" {
        return false;
    }

    let rust::Type::Struct(ty) = &rust_types[op.input.as_str()] else { panic!() };
    assert!(ty.xml_name.is_none());

    let has_xml_payload = ty.fields.iter().any(is_xml_payload);
    let has_string_payload = ty.fields.iter().any(|field| field.type_ == "Policy");
    has_xml_payload || has_string_payload
}

fn codegen_router(ops: &Operations, rust_types: &RustTypes, g: &mut Codegen) {
    let routes = collect_routes(ops, rust_types);

    let methods = ["HEAD", "GET", "POST", "PUT", "DELETE"];
    assert_eq!(methods.len(), routes.keys().count());
    for method in routes.keys() {
        assert!(methods.contains(&method.as_str()));
    }

    g.ln("pub fn resolve_route(req: &http::Request, s3_path: &S3Path, qs: Option<&http::OrderedQs>) -> S3Result<(&'static dyn super::Operation, bool)> {");

    let succ = |route: &Route, g: &mut Codegen, return_: bool| {
        if return_ {
            g.ln(f!(
                "return Ok((&{} as &'static dyn super::Operation, {}));",
                route.op.name,
                route.needs_full_body
            ));
        } else {
            g.ln(f!(
                "Ok((&{} as &'static dyn super::Operation, {}))",
                route.op.name,
                route.needs_full_body
            ));
        }
    };

    g.ln("match req.method().clone() {");
    for &method in &methods {
        g.ln(f!("hyper::Method::{} => match s3_path {{", method));

        for pattern in [PathPattern::Root, PathPattern::Bucket, PathPattern::Object] {
            let s3_path_pattern = match pattern {
                PathPattern::Root => "S3Path::Root",
                PathPattern::Bucket => "S3Path::Bucket{ .. }",
                PathPattern::Object => "S3Path::Object{ .. }",
            };

            g.ln(f!("{s3_path_pattern} => {{"));
            match routes[method].get(&pattern) {
                None => g.ln("Err(super::unknown_operation())"),
                Some(group) => {
                    // DEBUG:
                    // for route in group {
                    //     println!(
                    //         "{:<50} qt={:?}, qp={:?}, qs={:?}, hs={:?}",
                    //         route.op.name,
                    //         route.query_tag,
                    //         route.query_patterns,
                    //         route.required_query_strings,
                    //         route.required_headers
                    //     );
                    //     println!("     {}", route.op.http_uri);
                    //     println!();
                    // }

                    assert!(group.is_empty().not());
                    if group.len() == 1 {
                        let route = &group[0];
                        assert!(route.query_tag.is_none());
                        assert!(route.required_headers.is_empty());
                        assert!(route.required_query_strings.is_empty());
                        assert!(route.needs_full_body.not());
                        succ(route, g, false);
                    } else {
                        let is_final_op = |route: &Route| {
                            route.required_headers.is_empty()
                                && route.required_query_strings.is_empty()
                                && route.query_patterns.is_empty()
                                && route.query_tag.is_none()
                        };
                        let final_count = group.iter().filter(|r| is_final_op(r)).count();
                        assert!(final_count <= 1);
                        if final_count == 1 {}

                        g.ln("if let Some(qs) = qs {");
                        for route in group {
                            if let Some(ref tag) = route.query_tag {
                                assert!(tag.as_bytes().iter().all(|&x| x == b'-' || x.is_ascii_alphabetic()), "{tag}");
                                g.ln(f!("if qs.has(\"{tag}\") {{"));
                                succ(route, g, true);
                                g.ln("}");
                            }
                        }
                        for route in group {
                            let qp = route.query_patterns.as_slice();
                            assert!(qp.len() <= 1);
                            if let Some((ref n, ref v)) = qp.first() {
                                g.ln(f!("if super::check_query_pattern(qs, \"{n}\",\"{v}\") {{"));
                                succ(route, g, true);
                                g.ln("}");
                            }
                        }
                        g.ln("}");

                        for route in group {
                            if route.query_tag.is_some() || route.query_patterns.is_empty().not() {
                                continue;
                            }

                            let qs = route.required_query_strings.as_slice();
                            let hs = route.required_headers.as_slice();
                            assert!(qs.len() <= 1);
                            assert!(hs.len() <= 2);

                            if qs.is_empty() && hs.is_empty() {
                                continue;
                            }

                            let mut cond: String = default();
                            for q in qs {
                                cond.push_str(&f!("qs.has(\"{q}\")"));
                            }
                            for h in hs {
                                if cond.is_empty().not() {
                                    cond.push_str(" && ");
                                }
                                cond.push_str(&f!("req.headers().contains_key(\"{h}\")"));
                            }

                            if qs.is_empty().not() {
                                g.ln("if let Some(qs) = qs {");
                                g.ln(f!("if {cond} {{"));
                                succ(route, g, true);
                                g.ln("}");
                                g.ln("}");
                            } else {
                                g.ln(f!("if {cond} {{"));
                                succ(route, g, true);
                                g.ln("}");
                            }
                        }

                        if final_count == 1 {
                            let route = group.last().unwrap();
                            assert!(is_final_op(route));
                            succ(route, g, false);
                        } else {
                            g.ln("Err(super::unknown_operation())");
                        }
                    }
                }
            }
            g.ln("}");
        }

        g.ln("}");
    }
    g.ln("_ => Err(super::unknown_operation())");
    g.ln("}");

    g.ln("}");
}
