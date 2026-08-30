"""Immutable path identities for the Q0.42 Git-mode proof composition."""

from typing import Final


BLOBS: Final = {
    "scripts/hepta_q041_blob_contract.py": (
        "badc328fc16bfb233e32fa1f71c37a246c15577c"
    ),
    "scripts/verify_windows_gnullvm_q041_contract.py": (
        "009e0cc942f17a60a4eff238d925609e321b76c2"
    ),
    "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py": (
        "d14cd13ce4d21819e818336d315ed34da149a081"
    ),
    "scripts/verify-windows-gnullvm-direct-bazel.py": (
        "f1df9dafe5ac52f44e125d06574c4870bd9176e3"
    ),
    "scripts/verify-windows-gnullvm-job-executable.py": (
        "54d4bd732c79b95032d6e7568aca3ae499895b85"
    ),
}

EXECUTABLE: Final = (
    "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py",
    "scripts/verify-windows-gnullvm-direct-bazel.py",
    "scripts/verify-windows-gnullvm-job-executable.py",
    "scripts/verify_windows_gnullvm_q042_contract.py",
)

DIRECT: Final = """#!/usr/bin/env python3

from verify_windows_gnullvm_q042_contract import main


if __name__ == "__main__":
    main("direct-bazel")
"""
JOB: Final = DIRECT.replace('main("direct-bazel")', 'main("job-executable")')
