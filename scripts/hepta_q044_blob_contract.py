"""Immutable identities for the Q0.44 cross-platform mode-proof closure."""

from typing import Final


BLOBS: Final = {
    "scripts/hepta_q043_blob_contract.py": (
        "5c06fb0a5a9544e6464ac4bf8e96cb4546d5a849"
    ),
    "scripts/verify_windows_gnullvm_q043_contract.py": (
        "f4c35533e6b40696840be984413408a55ddca8ec"
    ),
    "scripts/verify_windows_gnullvm_q041_contract.py": (
        "e9ff9452a997d74f6dc1d3c7791b7b24c3954c96"
    ),
    "scripts/verify_windows_gnullvm_q040_contract.py": (
        "a8363f65f46fd0482034a107bf517bad9a6a6143"
    ),
    "scripts/verify-windows-gnullvm-startup-contract.py": (
        "de42d9bb3fbfef16ad880e0ab05345ef3731dfbd"
    ),
    "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py": (
        "d14cd13ce4d21819e818336d315ed34da149a081"
    ),
    "scripts/verify-windows-gnullvm-direct-bazel.py": (
        "d722146ee80072778235807c18def86a76bb92e2"
    ),
    "scripts/verify-windows-gnullvm-job-executable.py": (
        "799330c1460ede9c84a692945248d87a3d4b2a67"
    ),
}

EXECUTABLE: Final = (
    "scripts/verify_windows_gnullvm_q041_contract.py",
    "scripts/verify_windows_gnullvm_q043_contract.py",
    "scripts/verify-windows-gnullvm-startup-contract.py",
    "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py",
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
