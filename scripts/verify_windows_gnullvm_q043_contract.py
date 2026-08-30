#!/usr/bin/env python3
"""Q0.43 compose both Q0.42 source-contract and Git-mode repairs."""

from __future__ import annotations

import hashlib
import re
import subprocess
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q041_contract as q041
import verify_windows_gnullvm_q042_contract as q042
from hepta_q043_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB


ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q043_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "2a81bf06569eb39c315ee808d47a80453257eb21"
Q041_SOURCE = "scripts/verify_windows_gnullvm_q041_contract.py"
Q042_BLOB_CONTRACT = "scripts/hepta_q042_blob_contract.py"
Q042_SOURCE = "scripts/verify_windows_gnullvm_q042_contract.py"
RECEIPT = "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py"
DIRECT_ADAPTER = "scripts/verify-windows-gnullvm-direct-bazel.py"
JOB_ADAPTER = "scripts/verify-windows-gnullvm-job-executable.py"


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing Q0.43 path: {relative}")
    return path.read_text(encoding="utf-8")


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require_git_executable(relative: str) -> None:
    normalized = relative.replace("\\", "/")
    result = subprocess.run(
        ["git", "ls-files", "--stage", "--", normalized],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    entries = result.stdout.splitlines()
    require(result.returncode == 0, f"git index lookup failed: {normalized}")
    require(len(entries) == 1, f"expected one Git index entry: {normalized}")
    fields = entries[0].split(maxsplit=3)
    require(len(fields) == 4, f"malformed Git index entry: {normalized}")
    mode, object_id, stage, indexed_path = fields
    require(mode == "100755", f"lost Git executable mode: {normalized}")
    require(stage == "0", f"non-zero Git index stage: {normalized}")
    require(
        re.fullmatch(r"[0-9a-f]{40}", object_id) is not None,
        f"invalid Git object ID: {normalized}",
    )
    require(indexed_path == normalized, f"Git index path drifted: {normalized}")


def patch_q042_compatibility() -> None:
    require(
        blob(ROOT / Q041_SOURCE) == BLOBS[Q041_SOURCE],
        "selected Q0.41 token-repair source drifted",
    )
    require(
        blob(ROOT / Q042_BLOB_CONTRACT) == BLOBS[Q042_BLOB_CONTRACT],
        "immutable Q0.42 blob contract drifted",
    )
    require(
        blob(ROOT / Q042_SOURCE) == BLOBS[Q042_SOURCE],
        "immutable Q0.42 source contract drifted",
    )

    q042.BLOBS = dict(q042.BLOBS)
    q042.BLOBS[Q041_SOURCE] = BLOBS[Q041_SOURCE]
    q042.BLOBS[DIRECT_ADAPTER] = BLOBS[DIRECT_ADAPTER]
    q042.BLOBS[JOB_ADAPTER] = BLOBS[JOB_ADAPTER]
    q042.EXPECTED_Q041_SOURCE_CONTRACT = BLOBS[Q041_SOURCE]
    q042.DIRECT = DIRECT
    q042.JOB = JOB

    # Git executable mode is a tree/index property. Replace every inherited
    # host-filesystem mode check before either predecessor verifier executes.
    q041.require_executable = require_git_executable
    q042.require_git_executable = require_git_executable


def validate_increment() -> None:
    require(
        blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA,
        "Q0.43 blob contract drifted",
    )
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.43 blob: {relative}")
        require(blob(path) == expected, f"Q0.43 blob drift: {relative}")
    for relative in EXECUTABLE:
        require_git_executable(relative)

    require(read(DIRECT_ADAPTER) == DIRECT, "Q0.43 direct adapter drifted")
    require(read(JOB_ADAPTER) == JOB, "Q0.43 job adapter drifted")

    q041_source = read(Q041_SOURCE)
    for token in (
        'output_user_root = env.get("BAZEL_OUTPUT_USER_ROOT")',
        "if not output_user_root:",
    ):
        require(token in q041_source, f"Q0.41 token repair missing: {token}")

    require(
        q041.require_executable is require_git_executable,
        "Q0.41 executable proof was not replaced with the Git-index proof",
    )
    require(
        q042.require_git_executable is require_git_executable,
        "Q0.42 executable proof was not replaced with the Git-index proof",
    )

    receipt = read(RECEIPT)
    for token in (
        '["git", "ls-files", "--stage", "--", relative]',
        'entries[0].split(maxsplit=1)[0] == "100755"',
        "PASS_WINDOWS_GNULLVM_Q0_42_RECEIPT_GIT_MODE_TRUTH_SOURCE",
    ):
        require(token in receipt, f"receipt Git-mode proof missing: {token}")

    print("PASS_WINDOWS_GNULLVM_Q0_43_PARALLEL_Q0_42_COMPOSITION_SOURCE")


def main(owner: str = "q043") -> None:
    require(
        owner in {"q043", "direct-bazel", "job-executable"},
        f"unknown Q0.43 owner {owner!r}",
    )
    patch_q042_compatibility()
    q042.main("q042" if owner == "q043" else owner)
    validate_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_43_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
