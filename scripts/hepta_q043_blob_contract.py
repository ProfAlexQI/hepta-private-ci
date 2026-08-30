"""Immutable path identities for the Q0.43 source composition."""

from typing import Final


BLOBS: Final = {
    "scripts/hepta_q040_blob_contract.py": "57c896107f8eab161a8ccde407e661bc2d9f4ac8",
    "scripts/verify_windows_gnullvm_q040_contract.py": (
        "a8363f65f46fd0482034a107bf517bad9a6a6143"
    ),
    "scripts/hepta_q041_blob_contract.py": "badc328fc16bfb233e32fa1f71c37a246c15577c",
    "scripts/verify_windows_gnullvm_q041_contract.py": (
        "e9ff9452a997d74f6dc1d3c7791b7b24c3954c96"
    ),
    ".github/scripts/test_run_bazel_query_executable.py": (
        "34262d1bbb027bbda3c76a174633640f3546e571"
    ),
    ".github/scripts/test_run_bazel_qualification_boundary.sh": (
        "8c8a93fea6b44742f265c0f55fb1e4713039e3d4"
    ),
    ".github/workflows/windows-gnullvm-qualification-boundary.yml": (
        "f800625126a84a900b7ff6aed3cb115dd52c1e82"
    ),
    ".github/workflows/windows-setup-bazel-token-boundary.yml": (
        "55a7cd88fe7692ea57e49c4cb585b5b1974686fd"
    ),
    ".github/actions/setup-bazel-ci/action.yml": (
        "890567be46f3fd78c11b89a20950bef2f7af4bf6"
    ),
    "scripts/verify-windows-gnullvm-bazel-query-vector.py": (
        "c20b6acfd18624f76bfe1c529e11a25b125ab01f"
    ),
    "scripts/verify-windows-gnullvm-bazel-query-executable.py": (
        "27ad7cff8c2bc72390f658d8de263f0b986f32a0"
    ),
    "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py": (
        "d14cd13ce4d21819e818336d315ed34da149a081"
    ),
    "scripts/verify-windows-gnullvm-direct-bazel.py": (
        "983e3ba15472a8d2a5da50efa221a710f1138466"
    ),
    "scripts/verify-windows-gnullvm-job-executable.py": (
        "ef6750ad65823c4a18a2d0ea84a5f706088c6990"
    ),
    ".bazelversion": "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e",
}

EXECUTABLE: Final = (
    ".github/scripts/test_run_bazel_query_executable.py",
    ".github/scripts/test_run_bazel_qualification_boundary.sh",
    "scripts/verify-windows-gnullvm-bazel-query-executable.py",
    "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py",
    "scripts/verify-windows-gnullvm-direct-bazel.py",
    "scripts/verify-windows-gnullvm-job-executable.py",
    "scripts/verify_windows_gnullvm_q043_contract.py",
)

DIRECT: Final = """#!/usr/bin/env python3

from verify_windows_gnullvm_q043_contract import main


if __name__ == "__main__":
    main("direct-bazel")
"""
JOB: Final = DIRECT.replace(
    'main("direct-bazel")',
    'main("job-executable")',
)
