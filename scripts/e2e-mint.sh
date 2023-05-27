#!/bin/bash
./scripts/s3s-proxy.sh > target/s3s-proxy.log &
sleep 3s
./scripts/mint.sh | tee target/mint.log
