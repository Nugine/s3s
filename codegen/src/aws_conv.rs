use heck::ToUpperCamelCase;

use crate::dto::RustTypes;
use crate::f;
use crate::gen::Codegen;
use crate::ops::Operations;
use crate::rust;

pub fn codegen(ops: &Operations, rust_types: &RustTypes, g: &mut Codegen) {
    g.ln("use super::*;");
    g.lf();
    g.ln("use aws_sdk_s3::input::*;");
    g.ln("use aws_sdk_s3::output::*;");
    g.ln("use aws_sdk_s3::model::*;");
    g.ln("use aws_sdk_s3::error::*;");
    g.lf();

    for (name, rust_type) in rust_types {
        if name.starts_with("SelectObjectContent") {
            continue; // TODO: SelectObjectContent
        }

        match rust_type {
            rust::Type::Alias(_) => continue,
            rust::Type::Provided(_) => continue,
            rust::Type::Timestamp(_) => continue,
            rust::Type::List(_) => continue,
            rust::Type::Map(_) => continue,
            rust::Type::StrEnum(_) => {}
            rust::Type::Struct(_) => {}
            rust::Type::StructEnum(_) => {}
        }

        let s3s_path = f!("s3s::dto::{name}");
        let aws_name = aws_ty_name(name);

        g.ln(f!("impl AwsConversion for {s3s_path} {{"));
        g.ln(f!("    type Target = {aws_name};"));
        g.ln("type Error = S3Error;");
        g.lf();

        if contains_deprecated_field(name) {
            g.ln("#[allow(deprecated)]");
        }
        g.ln("fn try_from_aws(x: Self::Target) -> S3Result<Self> {");
        match rust_type {
            rust::Type::Struct(ty) => {
                if ty.fields.is_empty() {
                    g.ln("let _ = x;");
                }

                g.ln("Ok(Self {");
                for field in &ty.fields {
                    let s3s_field_name = field.name.as_str();
                    let aws_field_name = match s3s_field_name {
                        "checksum_crc32c" => "checksum_crc32_c",
                        "type_" => "r#type",
                        s => s,
                    };

                    if field.type_ == "StreamingBlob" {
                        g.ln(f!("{s3s_field_name}: Some(try_from_aws(x.{aws_field_name})?),"));
                    } else if field.option_type || field.default_value.is_some() {
                        g.ln(f!("{s3s_field_name}: try_from_aws(x.{aws_field_name})?,"));
                    } else {
                        g.ln(f!("{s3s_field_name}: unwrap_from_aws(x.{aws_field_name}, \"{s3s_field_name}\")?,"));
                    }
                }
                g.ln("})");
            }
            rust::Type::StrEnum(ty) => {
                g.ln("Ok(match x {");
                for variant in &ty.variants {
                    let s3s_variant_name = variant.name.as_str();
                    let aws_variant_name = match s3s_variant_name {
                        "CRC32C" => "Crc32C".to_owned(),
                        _ => s3s_variant_name.to_upper_camel_case(),
                    };
                    g.ln(f!("{aws_name}::{aws_variant_name} => Self::from_static(Self::{s3s_variant_name}),"));
                }
                g.ln(f!("{aws_name}::Unknown(_) => Self::from(x.as_str().to_owned()),"));
                g.ln("_ => Self::from(x.as_str().to_owned()),");
                g.ln("})");
            }
            rust::Type::StructEnum(ty) => {
                g.ln("Ok(match x {");
                for variant in &ty.variants {
                    g.ln(f!("{aws_name}::{0}(v) => Self::{0}(try_from_aws(v)?),", variant.name));
                }
                g.ln(f!("_ => unimplemented!(\"unknown variant of {aws_name}: {{x:?}}\"),"));
                g.ln("})");
            }
            _ => panic!(),
        }
        g.ln("}");
        g.lf();

        if contains_deprecated_field(name) {
            g.ln("#[allow(deprecated)]");
        }
        g.ln("fn try_into_aws(x: Self) -> S3Result<Self::Target> {");
        match rust_type {
            rust::Type::Struct(ty) => {
                if ty.fields.is_empty() {
                    g.ln("let _ = x;");
                    g.ln("let y = Self::Target::builder();");
                } else {
                    g.ln("let mut y = Self::Target::builder();");
                }

                for field in &ty.fields {
                    let s3s_field_name = field.name.as_str();
                    let aws_field_name = match s3s_field_name {
                        "checksum_crc32c" => "checksum_crc32_c",
                        "type_" => "type",
                        s => s,
                    };

                    if field.option_type {
                        g.ln(f!("y = y.set_{aws_field_name}(try_into_aws(x.{s3s_field_name})?);"));
                    } else {
                        g.ln(f!("y = y.set_{aws_field_name}(Some(try_into_aws(x.{s3s_field_name})?));"));
                    }
                }

                if is_op_input(&ty.name, ops) {
                    g.ln("y.build().map_err(S3Error::internal_error)");
                } else {
                    g.ln("Ok(y.build())");
                }
            }
            rust::Type::StrEnum(_) => {
                g.ln(f!("Ok({aws_name}::from(x.as_str()))"));
            }
            rust::Type::StructEnum(ty) => {
                g.ln("Ok(match x {");
                for variant in &ty.variants {
                    g.ln(f!("Self::{0}(v) => {aws_name}::{0}(try_into_aws(v)?),", variant.name));
                }
                g.ln(f!("_ => unimplemented!(\"unknown variant of {}: {{x:?}}\"),", ty.name));
                g.ln("})");
            }
            _ => panic!(),
        }
        g.ln("}");

        g.ln("}");
        g.lf();
    }
}

fn aws_ty_name(name: &str) -> &str {
    match name {
        "BucketCannedACL" => "BucketCannedAcl",
        "CORSConfiguration" => "CorsConfiguration",
        "CORSRule" => "CorsRule",
        "CSVInput" => "CsvInput",
        "CSVOutput" => "CsvOutput",
        "JSONInput" => "JsonInput",
        "JSONOutput" => "JsonOutput",
        "JSONType" => "JsonType",
        "MFADelete" => "MfaDelete",
        "MFADeleteStatus" => "MfaDeleteStatus",
        "ObjectCannedACL" => "ObjectCannedAcl",
        "SSEKMS" => "Ssekms",
        "SSES3" => "Sses3",
        _ => name,
    }
}

fn is_op_input(name: &str, ops: &Operations) -> bool {
    if let Some(op) = name.strip_suffix("Input") {
        if ops.contains_key(op) {
            return true;
        }
    }
    false
}

fn contains_deprecated_field(name: &str) -> bool {
    matches!(name, "LifecycleRule" | "ReplicationRule")
}
