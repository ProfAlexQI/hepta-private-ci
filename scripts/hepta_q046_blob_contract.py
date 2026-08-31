"""Immutable identities for the Q0.46 Git-context authority closure."""

from typing import Final


BLOBS: Final = {
    "scripts/hepta_q045_blob_contract.py": (
        "a0254c5242a50cabfb3bd1746f21950223cb8d55"
    ),
    "scripts/verify_windows_gnullvm_q045_contract.py": (
        "9075e29a3a53096d8abfdd77e2f5aba056cb07a9"
    ),
    "scripts/hepta_q046_git_context.py": (
        "e2d96eb4bad1d245e74d2490c78eae03bb38a58e"
    ),
    "scripts/verify-windows-gnullvm-direct-bazel.py": (
        "924c1087b045578136f92ff43a9ebec67c9d22b7"
    ),
    "scripts/verify-windows-gnullvm-job-executable.py": (
        "085187dd5e0695b310dad0c324be1bb82e5464dd"
    ),
}

EXECUTABLE: Final = (
    "scripts/verify_windows_gnullvm_q045_contract.py",
    "scripts/hepta_q046_git_context.py",
    "scripts/verify-windows-gnullvm-direct-bazel.py",
    "scripts/verify-windows-gnullvm-job-executable.py",
    "scripts/verify_windows_gnullvm_q046_contract.py",
)

DIRECT: Final = """#!/usr/bin/env python3

from verify_windows_gnullvm_q046_contract import main


if __name__ == "__main__":
    main("direct-bazel")
"""
JOB: Final = DIRECT.replace('main("direct-bazel")', 'main("job-executable")')
