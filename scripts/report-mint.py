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
    logs = []
    with open(log_path) as f:
        for line in f.readlines():
            line = line.strip()
            if len(line) == 0:
                continue

            if line.find("{") != 0:
                line = line[line.find("{") :]

            try:
                json_value = json.loads(line)
            except Exception as e:
                print(f"error parsing log line: {line}")
                continue

            logs.append(from_json(json_value))

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

    passed_groups = [
        "aws-sdk-go",
        "aws-sdk-php",
        "aws-sdk-ruby",
        "awscli",
        "minio-go",
        "minio-py",
        "s3cmd",
        "s3select",
    ]

    for group in passed_groups:
        assert counts[group]["fail"] == 0, f'group "{group}" failed'

    # Won't fix
    assert counts["mc"]["pass"] >= 14
    assert counts["minio-java"]["pass"] >= 17

    # FIXME: E2E tests
    # https://github.com/Nugine/s3s/issues/4
    assert "minio-dotnet" not in counts
    assert counts["minio-js"]["pass"] >= 219
    assert counts["versioning"]["pass"] >= 4
