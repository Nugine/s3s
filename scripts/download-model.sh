#!/bin/bash -e
mkdir -p target
F="codegen/s3.json"
COMMIT=39d457149ac9ed01e73bb274dfe4341e82c8bbc3
URL=https://github.com/awslabs/aws-sdk-rust/raw/$COMMIT/aws-models/s3.json
wget --output-document $F $URL
