"""Immutable path identities for the Q0.39 additive source ratchet."""

from typing import Final

BLOBS: Final = {
    ".github/scripts/run_bazel_q030_direct_bazel.py": "1614f53c9572cdda3b1d7cf227f3a730e27b2adb",
    ".github/scripts/run_bazel_q034_execution_manifest.py": "463ab343008a9ad9d2696a051c1d1dca4ec14b1d",
    ".github/scripts/test_run_bazel_setup_action_yaml.py": "4df760d5807b498559557e4ac9d3f2ec63027c9c",
    ".github/scripts/test_run_bazel_query_vector.py": "feab46ece83c981fc5f06a5c77843252f4e7c7be",
    ".github/scripts/test_run_bazel_execution_manifest.py": "d6fb968f7bdc1721152f48e81bdd2415ff50f6f4",
    ".github/scripts/test_run_bazel_qualification_boundary.sh": "fff609894c393153fe8cdcaf8d0f82816f9873a7",
    ".github/workflows/windows-gnullvm-qualification-boundary.yml": "4b0be1cf1569e02de21019ecc323deb99b78610b",
    ".github/workflows/windows-setup-bazel-token-boundary.yml": "7808e9f4c176c7f2396392c2889828ac21f8fcad",
    ".github/workflows/blocking-ci.yml": "d2293c3fa89fdebef6c956e3fb478d42eb8ce636",
    ".github/actions/setup-bazel-ci/action.yml": "890567be46f3fd78c11b89a20950bef2f7af4bf6",
    "scripts/hepta_setup_action_yaml.py": "823a0e6ff88b2640311dd5ff32327b50c8ca1a22",
    "scripts/verify-windows-gnullvm-setup-action-yaml.py": "a38eab25a40f9212421e05fb492ae18a1ccc2978",
    "scripts/verify-windows-gnullvm-setup-token-cross-platform.py": "58ef7be03beeb19c47eccdb679292b95f1d1952b",
    "scripts/verify-windows-gnullvm-bazel-query-vector.py": "c20b6acfd18624f76bfe1c529e11a25b125ab01f",
    "scripts/verify-windows-gnullvm-direct-bazel.py": "3788dfaa838dfa1e589a023712f9ebd5a742b25b",
    "scripts/verify-windows-gnullvm-job-executable.py": "d4321545a4f18b09d24c0d69f18fab8fa2a28737",
    "scripts/verify_windows_gnullvm_q038_direct_bazel_base.py": "4bb0d7d018dce14b849b7c27e377d3ced66f6088",
    "scripts/verify_windows_gnullvm_q038_job_executable_base.py": "9a40e599cd6c725f5e41550be8fe8477b9422d47",
    ".bazelversion": "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e",
}
EXECUTABLE: Final = (
    ".github/scripts/test_run_bazel_setup_action_yaml.py",
    ".github/scripts/test_run_bazel_query_vector.py",
    ".github/scripts/test_run_bazel_qualification_boundary.sh",
    "scripts/verify-windows-gnullvm-setup-action-yaml.py",
    "scripts/verify-windows-gnullvm-bazel-query-vector.py",
    "scripts/verify-windows-gnullvm-direct-bazel.py",
    "scripts/verify-windows-gnullvm-job-executable.py",
    "scripts/verify_windows_gnullvm_q039_contract.py",
)
DIRECT = """#!/usr/bin/env python3

from verify_windows_gnullvm_q039_contract import main


if __name__ == "__main__":
    main("direct-bazel")
"""
JOB = DIRECT.replace('main("direct-bazel")', 'main("job-executable")')
