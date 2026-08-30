"""Immutable path identities for the Q0.41 direct-query executable ratchet."""

from typing import Final


Q040_BLOB_OVERRIDES: Final = {
    ".github/scripts/test_run_bazel_qualification_boundary.sh": "8c8a93fea6b44742f265c0f55fb1e4713039e3d4",
    ".github/workflows/windows-gnullvm-qualification-boundary.yml": "f800625126a84a900b7ff6aed3cb115dd52c1e82",
    "scripts/verify-windows-gnullvm-direct-bazel.py": "66967eee2cf6358a1715b16902a8138310d9ab47",
    "scripts/verify-windows-gnullvm-job-executable.py": "c5473fecf25aebf6620efae3e560ce4b96047494",
}

BLOBS: Final = {
    **Q040_BLOB_OVERRIDES,
    "scripts/hepta_q040_blob_contract.py": "57c896107f8eab161a8ccde407e661bc2d9f4ac8",
    "scripts/verify_windows_gnullvm_q040_contract.py": "a8363f65f46fd0482034a107bf517bad9a6a6143",
    "scripts/verify-windows-gnullvm-bazel-query-executable.py": "27ad7cff8c2bc72390f658d8de263f0b986f32a0",
    ".github/scripts/test_run_bazel_query_executable.py": "34262d1bbb027bbda3c76a174633640f3546e571",
}

EXECUTABLE: Final = (
    ".github/scripts/test_run_bazel_qualification_boundary.sh",
    ".github/scripts/test_run_bazel_query_executable.py",
    "scripts/verify-windows-gnullvm-bazel-query-executable.py",
    "scripts/verify-windows-gnullvm-direct-bazel.py",
    "scripts/verify-windows-gnullvm-job-executable.py",
    "scripts/verify_windows_gnullvm_q041_contract.py",
)

DIRECT: Final = """#!/usr/bin/env python3

from verify_windows_gnullvm_q041_contract import main


if __name__ == "__main__":
    main("direct-bazel")
"""

JOB: Final = DIRECT.replace('main("direct-bazel")', 'main("job-executable")')
