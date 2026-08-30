"""Immutable identities for the Q0.44 Git-mode ancestor composition."""

from typing import Final


BLOBS: Final = {
    "scripts/hepta_q043_blob_contract.py": (
        "5c06fb0a5a9544e6464ac4bf8e96cb4546d5a849"
    ),
    "scripts/verify_windows_gnullvm_q043_contract.py": (
        "f4c35533e6b40696840be984413408a55ddca8ec"
    ),
    "scripts/verify-windows-gnullvm-direct-bazel.py": (
        "d722146ee80072778235807c18def86a76bb92e2"
    ),
    "scripts/verify-windows-gnullvm-job-executable.py": (
        "799330c1460ede9c84a692945248d87a3d4b2a67"
    ),
}

EXECUTABLE: Final = (
    "scripts/verify-windows-gnullvm-direct-bazel.py",
    "scripts/verify-windows-gnullvm-job-executable.py",
    "scripts/verify_windows_gnullvm_q044_contract.py",
)

DIRECT: Final = """#!/usr/bin/env python3

from verify_windows_gnullvm_q044_contract import main


if __name__ == "__main__":
    main("direct-bazel")
"""
JOB: Final = DIRECT.replace('main("direct-bazel")', 'main("job-executable")')
