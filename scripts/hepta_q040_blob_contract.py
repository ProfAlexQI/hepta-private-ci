"""Immutable path identities for the Q0.40 additive source ratchet."""

from typing import Final


Q039_BLOB_OVERRIDES: Final = {
    ".github/scripts/test_run_bazel_qualification_boundary.sh": "8c8a93fea6b44742f265c0f55fb1e4713039e3d4",
    ".github/workflows/windows-gnullvm-qualification-boundary.yml": "f800625126a84a900b7ff6aed3cb115dd52c1e82",
    ".github/workflows/windows-setup-bazel-token-boundary.yml": "55a7cd88fe7692ea57e49c4cb585b5b1974686fd",
    "scripts/verify-windows-gnullvm-direct-bazel.py": "063995fd150e2c8be46143860cf817eaff6481f3",
    "scripts/verify-windows-gnullvm-job-executable.py": "232dcd2a5ded1ecd85e318fd78738d58f18a3a94",
}

BLOBS: Final = {
    **Q039_BLOB_OVERRIDES,
    "scripts/verify-windows-gnullvm-bazel-query-executable.py": "7bcaaf0e246256c2b45360d5cfe6c0fb9a854ef5",
    ".github/scripts/test_run_bazel_query_executable.py": "0fd706daff8378a0e1e0781e1db558027e8d0f45",
    "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py": "53a729fb4795fcecaf611c79b039c5dae7c0a6dc",
    "scripts/hepta_q039_blob_contract.py": "c0657cc4a3dd171f0c76fa6a61a78f2998834bed",
    "scripts/verify_windows_gnullvm_q039_contract.py": "220cc4be505a78f27ec3e9be1e425efe9d0c9692",
}

EXECUTABLE: Final = (
    ".github/scripts/test_run_bazel_qualification_boundary.sh",
    ".github/scripts/test_run_bazel_query_executable.py",
    "scripts/verify-windows-gnullvm-bazel-query-executable.py",
    "scripts/verify-windows-gnullvm-direct-bazel.py",
    "scripts/verify-windows-gnullvm-job-executable.py",
    "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py",
    "scripts/verify_windows_gnullvm_q040_contract.py",
)

DIRECT: Final = """#!/usr/bin/env python3

from verify_windows_gnullvm_q040_contract import main


if __name__ == "__main__":
    main("direct-bazel")
"""

JOB: Final = DIRECT.replace('main("direct-bazel")', 'main("job-executable")')
