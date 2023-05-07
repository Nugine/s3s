#!/bin/bash -e

# https://github.com/awslabs/aws-sdk-rust/commits/main/aws-models/s3.json
COMMIT=bb355d940cfce6f8a44a0c7ba128d96ae9dc847a

mkdir -p target
F="codegen/s3.json"
URL=https://github.com/awslabs/aws-sdk-rust/raw/$COMMIT/aws-models/s3.json
wget --output-document $F $URL
