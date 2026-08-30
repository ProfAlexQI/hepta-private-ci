#!/usr/bin/env python3
"""Q0.43 compose corrected Q0.41, Git-mode proof, and direct query."""

from __future__ import annotations

import hashlib
import importlib.util
import subprocess
import sys
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q041_contract as q041
from hepta_q043_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB

ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q043_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "5c06fb0a5a9544e6464ac4bf8e96cb4546d5a849"
Q041_BLOB = ROOT / "scripts" / "hepta_q041_blob_contract.py"
Q041_SOURCE = ROOT / "scripts" / "verify_windows_gnullvm_q041_contract.py"
EXPECTED_Q041_BLOB: Final = "badc328fc16bfb233e32fa1f71c37a246c15577c"
EXPECTED_Q041_SOURCE: Final = "e9ff9452a997d74f6dc1d3c7791b7b24c3954c96"
RECEIPT = "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py"
QUERY = "scripts/verify-windows-gnullvm-bazel-query-executable.py"
FIXTURE = ".github/scripts/test_run_bazel_qualification_boundary.sh"
WORKFLOW = ".github/workflows/windows-gnullvm-qualification-boundary.yml"
DIRECT_ADAPTER = "scripts/verify-windows-gnullvm-direct-bazel.py"
JOB_ADAPTER = "scripts/verify-windows-gnullvm-job-executable.py"


def require(value: bool, message: str) -> None:
    if not value:
        raise SystemExit(message)


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing Q0.43 path: {relative}")
    return path.read_text(encoding="utf-8")


def require_git_mode(relative: str) -> None:
    result = subprocess.run(
        ["git", "ls-files", "--stage", "--", relative],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    rows = result.stdout.splitlines()
    require(
        result.returncode == 0
        and len(rows) == 1
        and rows[0].split(maxsplit=1)[0] == "100755",
        f"Q0.43 requires one Git 100755 entry: {relative}",
    )


def load(relative: str, name: str):
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    require(spec is not None and spec.loader is not None, f"cannot load {relative}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def patch_parent() -> None:
    require(blob(Q041_BLOB) == EXPECTED_Q041_BLOB, "Q0.41 blob contract drifted")
    require(
        blob(Q041_SOURCE) == EXPECTED_Q041_SOURCE,
        "corrected Q0.41 source contract drifted",
    )
    for relative in (FIXTURE, WORKFLOW):
        q041.BLOBS[relative] = BLOBS[relative]
        q041.q040.BLOBS[relative] = BLOBS[relative]
    q041.q040.BLOBS[RECEIPT] = BLOBS[RECEIPT]
    q041.BLOBS[DIRECT_ADAPTER] = BLOBS[DIRECT_ADAPTER]
    q041.BLOBS[JOB_ADAPTER] = BLOBS[JOB_ADAPTER]
    q041.DIRECT = DIRECT
    q041.JOB = JOB


def validate_increment() -> None:
    require(blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA, "Q0.43 contract drifted")
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.43 blob: {relative}")
        require(blob(path) == expected, f"Q0.43 blob drift: {relative}")
    for relative in EXECUTABLE:
        require_git_mode(relative)
    require(read(DIRECT_ADAPTER) == DIRECT, "Q0.43 direct adapter drifted")
    require(read(JOB_ADAPTER) == JOB, "Q0.43 job adapter drifted")

    receipt = read(RECEIPT)
    for token in (
        '"git", "ls-files", "--stage", "--", relative',
        'entries[0].split(maxsplit=1)[0] == "100755"',
        "PASS_WINDOWS_GNULLVM_Q0_42_RECEIPT_GIT_MODE_TRUTH_SOURCE",
    ):
        require(token in receipt, f"receipt verifier lacks {token!r}")
    require("stat.S_IXUSR" not in receipt, "receipt uses host executable bits")

    query = read(QUERY)
    for token in (
        "def resolve_verified_linux_bazel",
        '[str(bazelisk), "--print_env"]',
        "cached Bazel executable SHA-256 drifted",
        "direct Bazel executable changed before parser launch",
        "PASS_WINDOWS_GNULLVM_Q0_41_DIRECT_BAZEL_QUERY_EXECUTED",
    ):
        require(token in query, f"direct-query verifier lacks {token!r}")
    load(QUERY, "_hepta_q043_query").main()


def main(owner: str = "q043") -> None:
    require(
        owner in {"q043", "direct-bazel", "job-executable"},
        f"unknown Q0.43 owner {owner!r}",
    )
    patch_parent()
    q041.main("q041" if owner == "q043" else owner)
    validate_increment()
    print(f"PASS_WINDOWS_GNULLVM_Q0_43_{owner.upper().replace('-', '_')}_SOURCE")


if __name__ == "__main__":
    main()
