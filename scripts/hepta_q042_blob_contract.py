"""Immutable path identities for the Q0.42 bounded Q0.41 successor."""

from typing import Final


BLOBS: Final = {
    "scripts/hepta_q040_blob_contract.py": (
        "57c896107f8eab161a8ccde407e661bc2d9f4ac8"
    ),
    "scripts/verify_windows_gnullvm_q040_contract.py": (
        "a8363f65f46fd0482034a107bf517bad9a6a6143"
    ),
    "scripts/verify_windows_gnullvm_q041_composition.py": (
        "db9e24f48b946375df82db1fbe51c85bc890deae"
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
        "f1df9dafe5ac52f44e125d06574c4870bd9176e3"
    ),
    "scripts/verify-windows-gnullvm-job-executable.py": (
        "54d4bd732c79b95032d6e7568aca3ae499895b85"
    ),
    ".github/scripts/test_run_bazel_qualification_boundary.sh": (
        "fff609894c393153fe8cdcaf8d0f82816f9873a7"
    ),
    ".github/workflows/windows-gnullvm-qualification-boundary.yml": (
        "4b0be1cf1569e02de21019ecc323deb99b78610b"
    ),
}

EXECUTABLE: Final = (
    ".github/scripts/run_bazel_with_buildbuddy.py",
    ".github/scripts/test_run_bazel_startup_contract.py",
    "scripts/verify-windows-gnullvm-startup-contract.py",
    "scripts/verify_windows_gnullvm_q041_composition.py",
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
