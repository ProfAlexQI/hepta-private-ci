"""Immutable identities for the Q0.45 index/worktree object binding."""

from typing import Final


BLOBS: Final = {
    "scripts/hepta_q044_blob_contract.py": (
        "3d156a5275cc406a65219116fe077ac159a7ccf2"
    ),
    "scripts/verify_windows_gnullvm_q044_contract.py": (
        "7840a69f17fc01a62028b692f16371bd895a3e22"
    ),
    "scripts/verify-windows-gnullvm-direct-bazel.py": (
        "83942ff49de5e62520c1ace4274106bb5cfa83a0"
    ),
    "scripts/verify-windows-gnullvm-job-executable.py": (
        "10d6ada79dabf9e0e6635dfd5b2c278afc1976f5"
    ),
}

EXECUTABLE: Final = (
    "scripts/verify_windows_gnullvm_q044_contract.py",
    "scripts/verify-windows-gnullvm-direct-bazel.py",
    "scripts/verify-windows-gnullvm-job-executable.py",
    "scripts/verify_windows_gnullvm_q045_contract.py",
)

DIRECT: Final = """#!/usr/bin/env python3

from verify_windows_gnullvm_q045_contract import main


if __name__ == "__main__":
    main("direct-bazel")
"""
JOB: Final = DIRECT.replace('main("direct-bazel")', 'main("job-executable")')
