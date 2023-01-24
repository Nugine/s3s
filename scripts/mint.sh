#!/bin/bash
docker run \
    -e "SERVER_ENDPOINT=localhost:8014"   \
    -e "ACCESS_KEY=minioadmin" -e "SECRET_KEY=minioadmin" \
    --network host \
    minio/mint:edge
