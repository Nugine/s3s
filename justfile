fmt:
    cargo fmt

dev:
    cargo fmt
    cargo clippy
    cargo test

doc:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open --no-deps --all-features

download-model:
    ./scripts/download-model.sh

codegen:
    ./scripts/codegen.sh

install:
    cargo install --offline --path crates/s3s-fs --features binary

sync-version:
    cargo set-version -p s3s            0.2.0-dev
    cargo set-version -p s3s-codegen    0.0.0
    cargo set-version -p s3s-fs         0.2.0-dev
    cargo set-version -p s3s-aws        0.2.0-dev
