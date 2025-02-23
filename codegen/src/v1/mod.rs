use crate::v2::smithy;
use crate::v2::utils::o;

mod rust;

mod access;
mod dto;
mod error;
mod headers;
mod minio;
mod ops;
mod s3_trait;
mod sts;
mod xml;

mod aws_conv;
mod aws_proxy;

use codegen_writer::Codegen;

pub fn run() {
    let model = {
        let mut s3_model = smithy::Model::load_json("model/s3.json").unwrap();
        let mut sts_model = smithy::Model::load_json("model/sts.json").unwrap();
        sts::reduce(&mut sts_model);
        s3_model.shapes.append(&mut sts_model.shapes);
        minio::patch(&mut s3_model);
        s3_model
    };

    let ops = ops::collect_operations(&model);
    let rust_types = dto::collect_rust_types(&model, &ops);

    {
        let path = "crates/s3s/src/dto/generated.rs";
        let cg = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(cg, || dto::codegen(&rust_types, &ops));
    }

    {
        let path = "crates/s3s/src/header/generated.rs";
        let cg = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(cg, || headers::codegen(&model));
    }

    {
        let path = "crates/s3s/src/error/generated.rs";
        let cg = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(cg, || error::codegen(&model));
    }

    {
        let path = "crates/s3s/src/xml/generated.rs";
        let cg = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(cg, || xml::codegen(&ops, &rust_types));
    }

    {
        let path = "crates/s3s/src/s3_trait.rs";
        let cg = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(cg, || s3_trait::codegen(&ops));
    }

    {
        let path = "crates/s3s/src/ops/generated.rs";
        let cg = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(cg, || ops::codegen(&ops, &rust_types));
    }

    {
        let path = "crates/s3s/src/access/generated.rs";
        let cg = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(cg, || access::codegen(&ops));
    }

    {
        let path = "crates/s3s-aws/src/conv/generated.rs";
        let cg = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(cg, || aws_conv::codegen(&ops, &rust_types));
    }

    {
        let path = "crates/s3s-aws/src/proxy/generated.rs";
        let cg = Codegen::create_file(path).unwrap();
        codegen_writer::scoped(cg, || aws_proxy::codegen(&ops, &rust_types));
    }
}
