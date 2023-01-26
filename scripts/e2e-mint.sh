#!/bin/bash -ex
./scripts/s3s-proxy.sh &
sleep 5s
./scripts/mint.sh > /tmp/mint.log
cat /tmp/mint.log
