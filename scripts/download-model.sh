#!/bin/bash -e

# https://github.com/awslabs/aws-sdk-rust/commits/main/aws-models/s3.json
COMMIT=fdec383fad0c276473b76339b52a6ae48f23b045

mkdir -p target
F="codegen/s3.json"
URL=https://github.com/awslabs/aws-sdk-rust/raw/$COMMIT/aws-models/s3.json
wget --output-document $F $URL
