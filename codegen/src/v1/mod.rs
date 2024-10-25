mod rust;
mod smithy;

mod access;
mod dto;
mod error;
mod headers;
mod ops;
mod s3_trait;
mod xml;

mod aws_conv;
mod aws_proxy;

use codegen_writer::Codegen;

fn o<T: ToOwned + ?Sized>(x: &T) -> T::Owned {
    x.to_owned()
}

pub fn run() {
    let model: smithy::Model = {
        let json_path = "model/s3.json";
        let json_file = std::fs::read(json_path).unwrap();
        serde_json::from_slice(&json_file).unwrap()
    };
    assert_eq!(model.smithy, "2.0");

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
