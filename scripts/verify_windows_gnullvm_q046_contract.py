#!/usr/bin/env python3
"""Q0.46 bind executable proof to one trusted local Git context."""

from __future__ import annotations

from pathlib import Path
from typing import Final

import hepta_q046_git_context as git_context
import verify_windows_gnullvm_q045_contract as q045
from hepta_q046_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB


ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q046_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "6b4437e8c9d4ee75ec37268c11f93a7d11c8a7de"
Q045_BLOB_CONTRACT = "scripts/hepta_q045_blob_contract.py"
Q045_SOURCE = "scripts/verify_windows_gnullvm_q045_contract.py"
DIRECT_ADAPTER = "scripts/verify-windows-gnullvm-direct-bazel.py"
JOB_ADAPTER = "scripts/verify-windows-gnullvm-job-executable.py"


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing Q0.46 path: {relative}")
    return path.read_text(encoding="utf-8")


def patch_q045_compatibility() -> None:
    require(
        git_context.blob(ROOT / Q045_BLOB_CONTRACT)
        == BLOBS[Q045_BLOB_CONTRACT],
        "immutable Q0.45 blob contract drifted",
    )
    require(
        git_context.blob(ROOT / Q045_SOURCE) == BLOBS[Q045_SOURCE],
        "immutable Q0.45 source contract drifted",
    )

    q045.BLOBS = dict(q045.BLOBS)
    q045.BLOBS[DIRECT_ADAPTER] = BLOBS[DIRECT_ADAPTER]
    q045.BLOBS[JOB_ADAPTER] = BLOBS[JOB_ADAPTER]
    q045.DIRECT = DIRECT
    q045.JOB = JOB
    q045.require_git_executable = git_context.require_git_executable


def validate_increment() -> None:
    require(
        git_context.blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA,
        "Q0.46 blob contract drifted",
    )
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.46 blob: {relative}")
        require(
            git_context.blob(path) == expected,
            f"Q0.46 blob drift: {relative}",
        )
    for relative in EXECUTABLE:
        git_context.require_git_executable(relative)

    require(read(DIRECT_ADAPTER) == DIRECT, "Q0.46 direct adapter drifted")
    require(read(JOB_ADAPTER) == JOB, "Q0.46 job adapter drifted")
    git_context.prove_git_context_fail_closed()

    require(
        q045.require_git_executable is git_context.require_git_executable,
        "Q0.45 replay bypassed Q0.46 Git-context binding",
    )
    require(
        q045.q044.require_git_executable is git_context.require_git_executable,
        "Q0.44 replay bypassed Q0.46 Git-context binding",
    )
    for owner, observed in (
        ("Q0.43", q045.q044.q043.require_git_mode),
        ("Q0.41", q045.q044.q041.require_executable),
        ("Q0.40", q045.q044.q041.q040.require_executable),
    ):
        require(
            observed is git_context.require_git_executable,
            f"{owner} retained an ambient Git executable proof",
        )
    require(
        q045.q044.q041.q040.load_source is q045.q044.strict_load_source,
        "dynamic verifier loading bypassed Q0.46 Git-context binding",
    )

    context = git_context.trusted_git_context()
    require(
        context.git_dir == context.root / ".git",
        "Q0.46 Git directory escaped the candidate checkout",
    )
    require(
        context.index == context.git_dir / "index",
        "Q0.46 Git index escaped the candidate checkout",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_46_TRUSTED_GIT_CONTEXT_SOURCE")


def main(owner: str = "q046") -> None:
    require(
        owner in {"q046", "direct-bazel", "job-executable"},
        f"unknown Q0.46 owner {owner!r}",
    )
    patch_q045_compatibility()
    q045.main("q045" if owner == "q046" else owner)
    validate_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_46_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
