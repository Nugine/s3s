#!/bin/bash -e
F="codegen/s3.json"
if [ ! -f $F ]; then
    ./scripts/download-model.sh
fi
cargo run -p s3s-codegen -- $F
cargo fmt
