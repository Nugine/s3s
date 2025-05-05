fmt:
    cargo fmt

dev:
    cargo fmt
    cargo clippy --all-features --all-targets
    cargo test

doc:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open --no-deps --all-features

download-model:
    ./scripts/download-model.py --force codegen/s3.json

codegen:
    ./scripts/download-model.py codegen/s3.json
    cargo run -p s3s-codegen -- codegen/s3.json
    cargo fmt

install-s3s-fs:
    cargo install --offline --path crates/s3s-fs --features binary

install-s3s-proxy:
    cargo install --offline --path crates/s3s-proxy

install:
    just install-s3s-fs
    just install-s3s-proxy

sync-version:
    cargo set-version -p s3s            0.9.1
    cargo set-version -p s3s-aws        0.9.1
    cargo set-version -p s3s-fs         0.9.1

publish:
    cargo publish -p s3s
    cargo publish -p s3s-aws
    cargo publish -p s3s-fs
