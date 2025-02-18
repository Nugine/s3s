#!/usr/bin/env python
from pathlib import Path


def main():
    crates = Path("crates")
    for crate in crates.iterdir():
        license_file = crate / "LICENSE"
        if not license_file.exists():
            license_file.symlink_to("../../LICENSE")


if __name__ == "__main__":
    main()
