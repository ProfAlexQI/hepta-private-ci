#!/usr/bin/env python3
"""Q0.44 bind ancestor executable proof to the Git index."""

from __future__ import annotations

import hashlib
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q043_contract as q043
from hepta_q044_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB


ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "scripts" / "hepta_q044_blob_contract.py"
CONTRACT_SHA: Final = "20693e54fdef525928fb6c7a25584d9d8d39f81e"
Q043_BLOBS = ROOT / "scripts" / "hepta_q043_blob_contract.py"
Q043_SOURCE = ROOT / "scripts" / "verify_windows_gnullvm_q043_contract.py"
EXPECTED_Q043_BLOBS: Final = "5c06fb0a5a9544e6464ac4bf8e96cb4546d5a849"
EXPECTED_Q043_SOURCE: Final = "f4c35533e6b40696840be984413408a55ddca8ec"
DIRECT_PATH = "scripts/verify-windows-gnullvm-direct-bazel.py"
JOB_PATH = "scripts/verify-windows-gnullvm-job-executable.py"


def require(value: bool, message: str) -> None:
    if not value:
        raise SystemExit(message)


def blob(path: Path) -> str:
    data = path.read_bytes()
    return hashlib.sha1(
        f"blob {len(data)}\0".encode("ascii") + data,
        usedforsecurity=False,
    ).hexdigest()


def patch_parent() -> None:
    require(blob(Q043_BLOBS) == EXPECTED_Q043_BLOBS, "Q0.43 blob contract drifted")
    require(blob(Q043_SOURCE) == EXPECTED_Q043_SOURCE, "Q0.43 source drifted")
    q043.BLOBS[DIRECT_PATH] = BLOBS[DIRECT_PATH]
    q043.BLOBS[JOB_PATH] = BLOBS[JOB_PATH]
    q043.DIRECT = DIRECT
    q043.JOB = JOB
    q043.q041.require_executable = q043.require_git_mode
    q043.q041.q040.require_executable = q043.require_git_mode


def validate() -> None:
    require(blob(CONTRACT) == CONTRACT_SHA, "Q0.44 blob contract drifted")
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.44 path: {relative}")
        require(blob(path) == expected, f"Q0.44 blob drift: {relative}")
    for relative in EXECUTABLE:
        q043.require_git_mode(relative)
    require((ROOT / DIRECT_PATH).read_text() == DIRECT, "direct adapter drifted")
    require((ROOT / JOB_PATH).read_text() == JOB, "job adapter drifted")
    require(
        q043.q041.require_executable is q043.require_git_mode,
        "Q0.41 proof was not rebound",
    )
    require(
        q043.q041.q040.require_executable is q043.require_git_mode,
        "Q0.40 proof was not rebound",
    )


def main(owner: str = "q044") -> None:
    require(owner in {"q044", "direct-bazel", "job-executable"}, "unknown owner")
    patch_parent()
    q043.main("q043" if owner == "q044" else owner)
    validate()
    print(f"PASS_WINDOWS_GNULLVM_Q0_44_{owner.upper().replace('-', '_')}_SOURCE")


if __name__ == "__main__":
    main()
