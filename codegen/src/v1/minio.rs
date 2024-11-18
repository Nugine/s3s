use core::str;

use super::o;
use super::smithy;
use super::smithy::BooleanShape;
use super::smithy::StructureMember;

use serde_json::json;

fn git_branch() -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    stdout.trim().to_owned()
}

pub fn patch(model: &mut smithy::Model) {
    let branch_name = git_branch();
    if !matches!(branch_name.as_str(), "minio" | "feat/minio") {
        return;
    }

    model.shapes.insert(
        o("com.amazonaws.s3#ForceDelete"),
        smithy::Shape::Boolean(BooleanShape {
            traits: smithy::Traits::default(),
        }),
    );

    let ty = "com.amazonaws.s3#DeleteBucketRequest";
    let Some(smithy::Shape::Structure(shape)) = model.shapes.get_mut(ty) else { panic!() };
    shape.members.insert(
        o("ForceDelete"),
        StructureMember {
            target: o("com.amazonaws.s3#ForceDelete"),
            traits: smithy::Traits::from_value(json!( {
                "smithy.api#httpHeader": "x-minio-force-delete",
                "s3s#minio": ""
            })),
        },
    );
}
