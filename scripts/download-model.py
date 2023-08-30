#!/usr/bin/env python3
from pathlib import Path
from argparse import ArgumentParser
import urllib.request
import hashlib

# https://github.com/awslabs/aws-sdk-rust/commits/main/aws-models/s3.json

COMMIT = "d09fb3435954efee106e07f277f47808f29219d3"
URL = f"https://github.com/awslabs/aws-sdk-rust/raw/{COMMIT}/aws-models/s3.json"
SHA256 = "d280b085f83a95dd7e22aea88f72f699a489fd018861f20c23cfda9d1c4a052b"


def sha256sum(arg: bytes | Path) -> str:
    if isinstance(arg, bytes):
        data = arg
    elif isinstance(arg, Path):
        with open(arg, "rb") as f:
            data = f.read()
    else:
        raise TypeError()

    return hashlib.sha256(data).hexdigest()


def download(dst: Path):
    print(f"Downloading {URL} to {dst}")
    resp = urllib.request.urlopen(URL)
    data = resp.read()

    sha256 = sha256sum(data)
    print(f"SHA256: {sha256}")

    with open(dst, "wb") as f:
        f.write(data)


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("--force", action="store_true")
    parser.add_argument("filename", type=Path)
    args = parser.parse_args()

    assert isinstance(args.force, bool)
    assert isinstance(args.filename, Path)

    dst: Path = args.filename
    needs_download: bool = args.force or (not dst.exists()) or sha256sum(dst) != SHA256

    if needs_download:
        download(dst)
