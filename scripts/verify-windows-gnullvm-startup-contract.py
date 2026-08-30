#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WRAPPER = ROOT / ".github" / "scripts" / "run_bazel_with_buildbuddy.py"
Q027_POLICY = ROOT / ".github" / "scripts" / "run_bazel_q027_lane_semantics.py"
Q028_POLICY = ROOT / ".github" / "scripts" / "run_bazel_q028_startup_contract.py"
Q028_TEST = ROOT / ".github" / "scripts" / "test_run_bazel_startup_contract.py"
FINAL_VERIFIER = ROOT / "scripts" / "verify-windows-gnullvm-final-command.py"
BOUNDARY = ROOT / ".github" / "scripts" / "test_run_bazel_qualification_boundary.sh"
REPO_CHECKS = ROOT / ".github" / "workflows" / "repo-checks.yml"

EXPECTED_Q027_POLICY_BLOB = "a507da0da4ac370a73d79eb305b227f0a080170a"


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing required path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    framed = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def require_token(text: str, token: str, owner: str) -> None:
    require(token in text, f"{owner} lacks required token: {token}")


def require_executable(path: Path) -> None:
    require(
        bool(path.stat().st_mode & stat.S_IXUSR),
        f"required executable lost mode: {path.relative_to(ROOT)}",
    )


def main() -> None:
    wrapper = read(WRAPPER)
    q027_policy = read(Q027_POLICY)
    q028_policy = read(Q028_POLICY)
    q028_test = read(Q028_TEST)
    final_verifier = read(FINAL_VERIFIER)
    boundary = read(BOUNDARY)
    repo_checks = read(REPO_CHECKS)

    for path in (WRAPPER, Q028_TEST, FINAL_VERIFIER, BOUNDARY):
        require_executable(path)

    require(
        git_blob_sha(Q027_POLICY) == EXPECTED_Q027_POLICY_BLOB,
        "selected Q0.27 lane-semantic policy drifted",
    )

    for token in (
        "from run_bazel_q022_negative_targets import (",
        "_validate_q026_compatibility_base",
        "from run_bazel_q027_lane_semantics import (",
        "_validate_q027_compatibility_base",
        "from run_bazel_q028_startup_contract import (",
        "validate_keyless_windows_gnullvm_final_args(command[1:], env)",
    ):
        require_token(wrapper, token, "run_bazel_with_buildbuddy.py")

    for token in (
        "validate_keyless_windows_gnullvm_final_args as _validate_q027",
        "DISABLED_REPO_CONTENTS_CACHE",
        "OUTPUT_USER_ROOT_PREFIX",
        "STRICT_STARTUP_FLAGS",
        "def _canonical_workspace",
        "def _expected_startup",
        "def _validate_exact_startup",
        "requires the exact ",
        "startup vector; expected",
        'env.get("GITHUB_ACTIONS") == "true"',
    ):
        require_token(q028_policy, token, "run_bazel_q028_startup_contract.py")

    for token in (
        "test_canonical_startup_vector_passes",
        "test_exact_output_root_from_environment_passes",
        "test_startup_jvm_option_fails_closed",
        "test_positive_repository_contents_cache_fails_closed",
        "test_duplicate_negative_repository_contents_cache_fails_closed",
        "test_output_root_drift_fails_closed",
    ):
        require_token(q028_test, token, "test_run_bazel_startup_contract.py")

    for token in (
        "test_canonical_test_lane_passes",
        "test_canonical_clippy_lane_passes",
        "test_exact_release_target_set_passes",
        "test_test_exclude_all_fails_closed",
        "test_clippy_arbitrary_exclusion_fails_closed",
        "from run_bazel_q027_lane_semantics import (",
        "from run_bazel_q028_startup_contract import (",
    ):
        require_token(
            final_verifier,
            token,
            "verify-windows-gnullvm-final-command.py",
        )

    for token in (
        "python3 .github/scripts/test_run_bazel_lane_semantics.py",
        "python3 .github/scripts/test_run_bazel_startup_contract.py",
        "python3 scripts/verify-windows-gnullvm-lane-semantics.py",
        "python3 scripts/verify-windows-gnullvm-startup-contract.py",
    ):
        require_token(boundary, token, "qualification boundary fixture")

    require_token(
        repo_checks,
        "python3 -m unittest discover -s .github/scripts "
        "-p 'test_run_bazel*.py'",
        "repo-checks.yml",
    )

    print("PASS_WINDOWS_GNULLVM_STARTUP_CONTRACT_SOURCE")


if __name__ == "__main__":
    main()
