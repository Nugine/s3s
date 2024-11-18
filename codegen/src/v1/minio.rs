use super::smithy;

fn git_branch() -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .unwrap();
    let stdout = core::str::from_utf8(&output.stdout).unwrap();
    stdout.trim().to_owned()
}

/// <https://github.com/Nugine/s3s/issues/192>
pub fn patch(model: &mut smithy::Model) {
    let branch_name = git_branch();
    if !matches!(branch_name.as_str(), "minio" | "feat/minio") {
        return;
    }

    let patches = smithy::Model::load_json("model/minio-patches.json");

    for (shape_name, patch) in patches.shapes {
        match model.shapes.get_mut(&shape_name) {
            None => {
                model.shapes.insert(shape_name, patch);
            }
            Some(shape) => match shape {
                smithy::Shape::Structure(shape) => {
                    let smithy::Shape::Structure(patch) = patch else { panic!() };
                    for (field_name, member) in patch.members {
                        assert!(shape.members.insert(field_name, member).is_none());
                    }
                }
                _ => unimplemented!(),
            },
        }
    }
}
