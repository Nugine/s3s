#!/usr/bin/env python3
from pathlib import Path
from argparse import ArgumentParser
import urllib.request
import hashlib

# https://github.com/awslabs/aws-sdk-rust/commits/main/aws-models/s3.json

COMMIT = "7f1c89f5c26c9e61751293d1bed79c6de44fc98f"
URL = f"https://github.com/awslabs/aws-sdk-rust/raw/{COMMIT}/aws-models/s3.json"
SHA256 = "8cea7cfdc587ff6f8282557fc8311a12e3355364f05f43b257f21b1a14f44e83"


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
    if sha256 != SHA256:
        print("SHA256 mismatch")

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
