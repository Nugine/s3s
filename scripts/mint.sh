#!/bin/bash
mkdir -p /tmp/mint
docker run \
    -e "SERVER_ENDPOINT=localhost:8014"   \
    -e "ACCESS_KEY=minioadmin" \
    -e "SECRET_KEY=minioadmin" \
    --network host \
    -v /tmp/mint:/mint/log \
    minio/mint:edge

./scripts/report-mint.py /tmp/mint/log.json
