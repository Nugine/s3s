fmt:
    cargo fmt

dev:
    cargo fmt
    cargo clippy
    cargo test

doc:
    cargo doc --open --no-deps --all-features

download-model:
    ./scripts/download-model.sh

codegen:
    ./scripts/codegen.sh

install:
    cargo install --offline --path crates/s3s-fs --features binary

sync-version:
    cargo set-version -p s3s            0.1.1
    cargo set-version -p s3s-codegen    0.0.0
    cargo set-version -p s3s-fs         0.1.1
    cargo set-version -p s3s-aws        0.0.1
