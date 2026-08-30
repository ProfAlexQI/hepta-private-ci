"""Immutable path identities for the Q0.43 selected Q0.42 composition."""

from typing import Final


BLOBS: Final = {
    "scripts/hepta_q041_blob_contract.py": (
        "badc328fc16bfb233e32fa1f71c37a246c15577c"
    ),
    "scripts/verify_windows_gnullvm_q041_contract.py": (
        "e9ff9452a997d74f6dc1d3c7791b7b24c3954c96"
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
}

EXECUTABLE: Final = (
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
JOB: Final = DIRECT.replace('main("direct-bazel")', 'main("job-executable")')
