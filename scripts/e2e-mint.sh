#!/bin/bash
./scripts/s3s-proxy.sh | tee target/s3s-proxy.ansi &
sleep 3s
./scripts/mint.sh | tee target/mint.log
