#!/bin/bash -e

# https://github.com/awslabs/aws-sdk-rust/commits/main/aws-models/s3.json
COMMIT=d85c1e7393b03e31fdc569bc07159dfc27812b9e

mkdir -p target
F="codegen/s3.json"
URL=https://github.com/awslabs/aws-sdk-rust/raw/$COMMIT/aws-models/s3.json
wget --output-document $F $URL
