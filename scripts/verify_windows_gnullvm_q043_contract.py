#!/usr/bin/env python3
"""Q0.43 compose both selected Q0.42 repairs on the Q0.41 ancestry."""

from __future__ import annotations

import hashlib
import subprocess
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q041_contract as q041
from hepta_q043_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB


ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q043_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "dc72975160e289f44413eb3064386e9a7cfcc260"
Q041_BLOB_CONTRACT = ROOT / "scripts" / "hepta_q041_blob_contract.py"
Q041_SOURCE_CONTRACT = (
    ROOT / "scripts" / "verify_windows_gnullvm_q041_contract.py"
)
EXPECTED_Q041_BLOB_CONTRACT: Final = (
    "badc328fc16bfb233e32fa1f71c37a246c15577c"
)
EXPECTED_Q041_SOURCE_CONTRACT: Final = (
    "e9ff9452a997d74f6dc1d3c7791b7b24c3954c96"
)
RECEIPT = "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py"
DIRECT_ADAPTER = "scripts/verify-windows-gnullvm-direct-bazel.py"
JOB_ADAPTER = "scripts/verify-windows-gnullvm-job-executable.py"
SOURCE_CONTRACT = "scripts/verify_windows_gnullvm_q043_contract.py"


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
    normalized = Path(relative).as_posix()
    path = ROOT / normalized
    require(path.is_file(), f"missing executable path: {normalized}")
    result = subprocess.run(
        ["git", "ls-files", "--stage", "--", normalized],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    entries = [
        entry
        for entry in result.stdout.splitlines()
        if entry.strip()
    ]
    require(
        result.returncode == 0
        and len(entries) == 1
        and entries[0].split(maxsplit=1)[0] == "100755",
        f"Q0.43 requires one Git 100755 entry: {normalized}",
    )


def require_tokens(text: str, tokens: tuple[str, ...], owner: str) -> None:
    for token in tokens:
        require(token in text, f"{owner} lacks Q0.43 token: {token}")


def patch_q041_compatibility() -> None:
    require(
        blob(Q041_BLOB_CONTRACT) == EXPECTED_Q041_BLOB_CONTRACT,
        "immutable Q0.41 blob contract drifted",
    )
    require(
        blob(Q041_SOURCE_CONTRACT) == EXPECTED_Q041_SOURCE_CONTRACT,
        "selected Q0.42 source-contract repair drifted",
    )

    q041.q040.BLOBS[RECEIPT] = BLOBS[RECEIPT]
    q041.BLOBS[DIRECT_ADAPTER] = BLOBS[DIRECT_ADAPTER]
    q041.BLOBS[JOB_ADAPTER] = BLOBS[JOB_ADAPTER]
    q041.DIRECT = DIRECT
    q041.JOB = JOB

    # Q0.40 and Q0.41 run on the Ubuntu source lane, but executable authority
    # is a Git tree property. The selected successor removes the redundant
    # host-mode precondition without modifying either immutable ancestor file.
    q041.require_executable = require_git_executable
    q041.q040.require_executable = require_git_executable


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

    q041_source = read(
        "scripts/verify_windows_gnullvm_q041_contract.py"
    )
    require_tokens(
        q041_source,
        (
            'output_user_root = env.get("BAZEL_OUTPUT_USER_ROOT")',
            "if not output_user_root:",
            "startup_module.main()",
            "PASS_WINDOWS_GNULLVM_Q0_41_",
        ),
        "selected Q0.42 source-contract repair",
    )
    require(
        "requires BAZEL_OUTPUT_USER_ROOT" not in q041_source,
        "selected Q0.42 retained the impossible raw source assertion",
    )

    receipt = read(RECEIPT)
    require_tokens(
        receipt,
        (
            "def require_git_executable",
            '"git", "ls-files", "--stage", "--", relative',
            'entries[0].split(maxsplit=1)[0] == "100755"',
            "PASS_WINDOWS_GNULLVM_Q0_39_RECEIPT_STEP_TRUTH_SOURCE",
            "PASS_WINDOWS_GNULLVM_Q0_42_RECEIPT_GIT_MODE_TRUTH_SOURCE",
        ),
        "receipt-truth verifier",
    )
    for forbidden in (
        "import stat",
        "stat.S_IXUSR",
        "Path(__file__).stat()",
    ):
        require(
            forbidden not in receipt,
            f"receipt verifier retains host-mode proof: {forbidden}",
        )

    source_contract = read(SOURCE_CONTRACT)
    require_tokens(
        source_contract,
        (
            "q041.q040.BLOBS[RECEIPT] = BLOBS[RECEIPT]",
            "q041.require_executable = require_git_executable",
            "q041.q040.require_executable = require_git_executable",
        ),
        "Q0.43 source contract",
    )
    for forbidden in ("import stat", "stat.S_IXUSR"):
        require(
            forbidden not in source_contract,
            f"Q0.43 source retains host-mode proof: {forbidden}",
        )


def main(owner: str = "q043") -> None:
    require(
        owner in {"q043", "direct-bazel", "job-executable"},
        f"unknown Q0.43 owner {owner!r}",
    )
    patch_q041_compatibility()
    require(
        q041.require_executable is require_git_executable,
        "Q0.41 executable authority was not replaced",
    )
    require(
        q041.q040.require_executable is require_git_executable,
        "Q0.40 executable authority was not replaced",
    )
    q041.main("q041" if owner == "q043" else owner)
    validate_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_43_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
