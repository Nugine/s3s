#!/bin/bash
DATA_DIR="/tmp"

if [ -n "$1" ]; then
	DATA_DIR="$1"
fi

if [ -z "$RUST_LOG" ]; then
    export RUST_LOG="s3s_fs=debug,s3s=debug"
fi

s3s-fs \
    --access-key AKIAIOSFODNN7EXAMPLE \
    --secret-key wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY \
    "$DATA_DIR"
