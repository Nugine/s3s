#!/bin/bash -ex

cargo build -p s3s-fs --bins --release --features binary
cargo build -p s3s-test --bins --release

DATA_DIR="/tmp/s3s-e2e"
mkdir -p "$DATA_DIR"

if [ -z "$RUST_LOG" ]; then
    export RUST_LOG="s3s_fs=debug,s3s=debug"
fi

./target/release/s3s-fs \
    --access-key    AKEXAMPLES3S    \
    --secret-key    SKEXAMPLES3S    \
    --host          localhost       \
    --port          8014            \
    --domain        localhost:8014  \
    --domain        localhost       \
    "$DATA_DIR" | tee target/s3s-fs.log &

sleep 1s

export AWS_ACCESS_KEY_ID=AKEXAMPLES3S
export AWS_SECRET_ACCESS_KEY=SKEXAMPLES3S
export AWS_REGION=us-east-1
export AWS_ENDPOINT_URL=http://localhost:8014

if [ -z "$RUST_LOG" ]; then
    export RUST_LOG="s3s_e2e=debug,s3s_test=info,s3s=debug"
fi
export RUST_BACKTRACE=full

./target/release/s3s-e2e "$@" | tee target/s3s-e2e.log

killall s3s-fs
