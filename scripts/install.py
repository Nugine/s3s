#!/usr/bin/env python
from dataclasses import dataclass
import argparse
import os

INSTALLERS = {}


def installer(name):
    def decorator(f):
        INSTALLERS[name] = f
        return f

    return decorator


def sh(cmd):
    os.system(cmd)


@installer("s3s-fs")
def install_s3s_fs():
    sh("cargo install --path crates/s3s-fs --features binary")


@installer("s3s-proxy")
def install_s3s_proxy():
    sh("cargo install --path crates/s3s-proxy")


@installer("s3s-e2e")
def install_s3s_e2e():
    sh("cargo install --path crates/s3s-test --bin s3s-e2e")


@installer("all")
def install_all():
    for name, f in INSTALLERS.items():
        if name != "all":
            f()


@dataclass(kw_only=True, frozen=True)
class CliArgs:
    name: str

    @staticmethod
    def parse():
        parser = argparse.ArgumentParser()
        parser.add_argument("name", type=str, choices=list(INSTALLERS.keys()))
        args = parser.parse_args()
        return CliArgs(**args.__dict__)


def main(args: CliArgs):
    INSTALLERS[args.name]()


if __name__ == "__main__":
    main(CliArgs.parse())
