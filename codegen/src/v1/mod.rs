use crate::v2::smithy;
use crate::v2::utils::o;

mod rust;

mod access;
mod dto;
mod error;
mod headers;
mod ops;
mod s3_trait;
mod sts;
mod xml;

mod aws_conv;
mod aws_proxy;

use codegen_writer::Codegen;

pub fn run() {
    let model = {
        let mut s3_model = smithy::Model::load_json("model/s3.json");
        let mut sts_model = smithy::Model::load_json("model/sts.json");
        sts::reduce(&mut sts_model);
        s3_model.shapes.append(&mut sts_model.shapes);
        s3_model
    };

    let ops = ops::collect_operations(&model);
    let rust_types = dto::collect_rust_types(&model, &ops);

    {
        let path = "crates/s3s/src/dto/generated.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || dto::codegen(&rust_types, &ops));
    }

    {
        let path = "crates/s3s/src/header/generated.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || headers::codegen(&model));
    }

    {
        let path = "crates/s3s/src/error/generated.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || error::codegen(&model));
    }

    {
        let path = "crates/s3s/src/xml/generated.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || xml::codegen(&ops, &rust_types));
    }

    {
        let path = "crates/s3s/src/s3_trait.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || s3_trait::codegen(&ops));
    }

    {
        let path = "crates/s3s/src/ops/generated.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || ops::codegen(&ops, &rust_types));
    }

    {
        let path = "crates/s3s/src/access/generated.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || access::codegen(&ops));
    }

    {
        let path = "crates/s3s-aws/src/conv/generated.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || aws_conv::codegen(&ops, &rust_types));
    }

    {
        let path = "crates/s3s-aws/src/proxy/generated.rs";
        let gen = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(gen, || aws_proxy::codegen(&ops, &rust_types));
    }
}
