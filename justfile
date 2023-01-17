fmt:
    cargo fmt

dev:
    cargo fmt
    cargo clippy
    cargo test

doc:
    cargo doc --open --no-deps --all-features

download-model:
    #!/bin/bash -e
    mkdir -p target
    F="target/s3.json"
    COMMIT=2e49e06ecb041afaf3fbed5a4a4f6c6afbb052fa
    URL=https://github.com/awslabs/aws-sdk-rust/raw/$COMMIT/aws-models/s3.json
    wget --output-document $F $URL

codegen:
    #!/bin/bash -e
    F="target/s3.json"
    if [ ! -f $F ]; then
        just download-model
    fi
    cargo run -p s3s-codegen -- $F
    cargo fmt

install:
    cargo install --offline --path crates/s3s-fs --features binary

sync-version:
    cargo set-version -p s3s            0.1.0
    cargo set-version -p s3s-codegen    0.0.0
    cargo set-version -p s3s-fs         0.1.0
