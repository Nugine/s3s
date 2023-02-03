#!/bin/bash
./scripts/s3s-proxy.sh > target/s3s-proxy.ansi &
sleep 3s
./scripts/mint.sh > target/mint.log
