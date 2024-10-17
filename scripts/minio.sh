#!/bin/bash -ex

mkdir -p /tmp/minio
docker run \
    -p 9000:9000 -p 9001:9001 \
    -e "MINIO_DOMAIN=localhost:9000" \
    -e "MINIO_HTTP_TRACE=1" \
    -v /tmp/minio:/data \
    minio/minio:latest server /data --console-address ":9001" &
