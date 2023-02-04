#!/usr/bin/env python3
from dataclasses import dataclass
from typing import Any, Dict, Optional
import json
import sys
from pprint import pprint
from itertools import groupby

# https://github.com/minio/mint#mint-log-format
@dataclass
class MintLog:
    name: str
    function: Optional[str]
    args: Optional[Dict[str, Any]]
    duration: int
    status: str
    alert: Optional[str]
    message: Optional[str]
    error: Optional[str]


def from_json(x: Any) -> MintLog:
    return MintLog(
        name=x["name"],
        function=x.get("function"),
        args=x.get("args"),
        duration=x["duration"],
        status=x["status"],
        alert=x.get("alert"),
        message=x.get("message"),
        error=x.get("error"),
    )


if __name__ == "__main__":
    log_path = sys.argv[1]
    with open(log_path) as f:
        logs = [from_json(json.loads(line.strip())) for line in f.readlines()]

    for x in logs:
        if ":" in x.name:
            name, function = x.name.split(":")
            x.name = name.strip()
            x.function = function.strip()

    groups = {k: list(v) for k, v in groupby(logs, lambda x: x.name)}
    counts = {}

    for name, group in groups.items():
        pass_count = len(list(x for x in group if x.status == "PASS"))
        fail_count = len(list(x for x in group if x.status == "FAIL"))
        na_count = len(list(x for x in group if x.status == "NA"))
        counts[name] = {"pass": pass_count, "fail": fail_count, "na": na_count}

        print(f"{name:<20} passed {pass_count:>3}, failed {fail_count:>3}, na {na_count:>3}")
    print()

    total_pass_count = sum(c["pass"] for c in counts.values())
    total_fail_count = sum(c["fail"] for c in counts.values())
    total_na_count = sum(c["na"] for c in counts.values())
    name = "summary"
    print(f"{name:<20} passed {total_pass_count:>3}, failed {total_fail_count:>3}, na {total_na_count:>3}")

    assert counts["aws-sdk-go"]["fail"] == 0
    assert counts["aws-sdk-ruby"]["fail"] == 0
    assert counts["mc"]["fail"] == 0
    assert counts["s3cmd"]["fail"] == 0
    assert counts["s3select"]["fail"] == 0

    # FIXME: E2E tests
    assert counts["aws-sdk-php"]["pass"] >= 12
    assert counts["awscli"]["pass"] >= 10
    assert counts["minio-dotnet"]["pass"] >= 1
    assert counts["minio-go"]["pass"] >= 1
    assert counts["minio-java"]["pass"] >= 8
    assert counts["minio-py"]["pass"] >= 13
    assert counts["versioning"]["pass"] >= 4

    # assert counts["minio-js"]["pass"] >= 0
