#!/usr/bin/env python3
"""Q0.42 compose Git-index mode proof onto selected Q0.41."""

from __future__ import annotations

import hashlib
import stat
import subprocess
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q041_contract as q041
from hepta_q042_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB


ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q042_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "3b15e949305f9edade977a368db25c3c78932a8a"
Q041_BLOB_CONTRACT = ROOT / "scripts" / "hepta_q041_blob_contract.py"
Q041_SOURCE_CONTRACT = ROOT / "scripts" / "verify_windows_gnullvm_q041_contract.py"
EXPECTED_Q041_BLOB_CONTRACT: Final = (
    "badc328fc16bfb233e32fa1f71c37a246c15577c"
)
EXPECTED_Q041_SOURCE_CONTRACT: Final = (
    "009e0cc942f17a60a4eff238d925609e321b76c2"
)
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
    require(path.is_file(), f"missing Q0.42 path: {relative}")
    return path.read_text(encoding="utf-8")


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require_git_executable(relative: str) -> None:
    path = ROOT / relative
    require(
        bool(path.stat().st_mode & stat.S_IXUSR),
        f"lost filesystem executable mode: {relative}",
    )
    result = subprocess.run(
        ["git", "ls-files", "--stage", "--", relative],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    entries = result.stdout.splitlines()
    require(
        result.returncode == 0
        and len(entries) == 1
        and entries[0].split(maxsplit=1)[0] == "100755",
        f"lost Git executable mode: {relative}",
    )


def require_tokens(text: str, tokens: tuple[str, ...], owner: str) -> None:
    for token in tokens:
        require(token in text, f"{owner} lacks Q0.42 token: {token}")


def patch_q041_compatibility() -> None:
    require(
        blob(Q041_BLOB_CONTRACT) == EXPECTED_Q041_BLOB_CONTRACT,
        "immutable Q0.41 blob contract drifted",
    )
    require(
        blob(Q041_SOURCE_CONTRACT) == EXPECTED_Q041_SOURCE_CONTRACT,
        "immutable Q0.41 source contract drifted",
    )

    q041.q040.BLOBS[RECEIPT] = BLOBS[RECEIPT]
    q041.BLOBS[DIRECT_ADAPTER] = BLOBS[DIRECT_ADAPTER]
    q041.BLOBS[JOB_ADAPTER] = BLOBS[JOB_ADAPTER]
    q041.DIRECT = DIRECT
    q041.JOB = JOB


def validate_increment() -> None:
    require(
        blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA,
        "Q0.42 blob contract drifted",
    )
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.42 blob: {relative}")
        require(blob(path) == expected, f"Q0.42 blob drift: {relative}")
    for relative in EXECUTABLE:
        require_git_executable(relative)

    require(read(DIRECT_ADAPTER) == DIRECT, "Q0.42 direct adapter drifted")
    require(read(JOB_ADAPTER) == JOB, "Q0.42 job adapter drifted")

    receipt = read(RECEIPT)
    require_tokens(
        receipt,
        (
            "def require_git_executable",
            '"git", "ls-files", "--stage", "--", relative',
            'entries[0].split(maxsplit=1)[0] == "100755"',
            "Q0.42 receipt-truth verifier must be one Git 100755 entry",
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


def main(owner: str = "q042") -> None:
    require(
        owner in {"q042", "direct-bazel", "job-executable"},
        f"unknown Q0.42 owner {owner!r}",
    )
    patch_q041_compatibility()
    q041.main("q041" if owner == "q042" else owner)
    validate_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_42_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
