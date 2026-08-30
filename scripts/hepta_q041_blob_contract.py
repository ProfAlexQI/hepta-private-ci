"""Immutable path identities for the Q0.41 startup-order composition."""

from typing import Final


BLOBS: Final = {
    "scripts/hepta_q040_blob_contract.py": (
        "57c896107f8eab161a8ccde407e661bc2d9f4ac8"
    ),
    "scripts/verify_windows_gnullvm_q040_contract.py": (
        "a8363f65f46fd0482034a107bf517bad9a6a6143"
    ),
    ".github/scripts/run_bazel_q039_startup_order.py": (
        "e0923474a529b37ef416ab9af90cc0745079afe5"
    ),
    ".github/scripts/run_bazel_with_buildbuddy.py": (
        "cf5e7d990e1c649dac505ff98199cffa60def08d"
    ),
    ".github/scripts/test_run_bazel_startup_contract.py": (
        "20a37801e00df21fda03011102fde499963892bc"
    ),
    "scripts/verify-windows-gnullvm-startup-contract.py": (
        "de42d9bb3fbfef16ad880e0ab05345ef3731dfbd"
    ),
    "scripts/verify-windows-gnullvm-direct-bazel.py": (
        "66967eee2cf6358a1715b16902a8138310d9ab47"
    ),
    "scripts/verify-windows-gnullvm-job-executable.py": (
        "c5473fecf25aebf6620efae3e560ce4b96047494"
    ),
}
EXECUTABLE: Final = (
    ".github/scripts/run_bazel_with_buildbuddy.py",
    ".github/scripts/test_run_bazel_startup_contract.py",
    "scripts/verify-windows-gnullvm-startup-contract.py",
    "scripts/verify-windows-gnullvm-direct-bazel.py",
    "scripts/verify-windows-gnullvm-job-executable.py",
    "scripts/verify_windows_gnullvm_q041_contract.py",
)
DIRECT = """#!/usr/bin/env python3

from verify_windows_gnullvm_q041_contract import main


if __name__ == "__main__":
    main("direct-bazel")
"""
JOB = DIRECT.replace('main("direct-bazel")', 'main("job-executable")')
