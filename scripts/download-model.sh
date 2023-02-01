#!/bin/bash -e
mkdir -p target
F="codegen/s3.json"
COMMIT=2e49e06ecb041afaf3fbed5a4a4f6c6afbb052fa
URL=https://github.com/awslabs/aws-sdk-rust/raw/$COMMIT/aws-models/s3.json
wget --output-document $F $URL
