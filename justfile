dev:
    just fetch
    just fmt
    just lint
    just test

fetch:
    uv sync
    cargo fetch

fmt:
    uvx ruff format
    cargo fmt

lint:
    uvx ruff check
    cargo clippy --all-features --all-targets

test:
    cargo test --all-features

doc:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open --no-deps --all-features

model:
    uv run model/main.py update

codegen:
    cargo run -p s3s-codegen -- model/s3.json
    cargo fmt
    cargo check

install name:
    uv run ./scripts/install.py {{name}}

# ------------------------------------------------

sync-version:
    cargo set-version -p s3s            0.11.0-dev
    cargo set-version -p s3s-policy     0.11.0-dev
    cargo set-version -p s3s-aws        0.11.0-dev
    cargo set-version -p s3s-fs         0.11.0-dev
    cargo set-version -p s3s-test       0.11.0-dev

publish:
    cargo publish -p s3s
    cargo publish -p s3s-policy
    cargo publish -p s3s-aws
    cargo publish -p s3s-fs
    cargo publish -p s3s-test

# ------------------------------------------------

assert_unchanged:
    #!/bin/bash -ex
    [[ -z "$(git status -s)" ]] # https://stackoverflow.com/a/9393642

ci-rust:
    cargo fmt --all --check
    cargo clippy --all-features --all-targets -- -D warnings
    just ci-test
    just codegen
    just assert_unchanged

ci-test:
    cargo test --all-features

ci-python:
    uvx ruff format --check
    uvx ruff check
    just model
    just assert_unchanged
