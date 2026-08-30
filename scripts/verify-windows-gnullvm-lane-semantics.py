#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WRAPPER = ROOT / ".github" / "scripts" / "run_bazel_with_buildbuddy.py"
Q026_POLICY = ROOT / ".github" / "scripts" / "run_bazel_q022_negative_targets.py"
Q027_POLICY = ROOT / ".github" / "scripts" / "run_bazel_q027_lane_semantics.py"
Q027_TEST = ROOT / ".github" / "scripts" / "test_run_bazel_lane_semantics.py"
Q030_TEST = ROOT / ".github" / "scripts" / "test_run_bazel_direct_bazel.py"
BOUNDARY = ROOT / ".github" / "scripts" / "test_run_bazel_qualification_boundary.sh"
REPO_CHECKS = ROOT / ".github" / "workflows" / "repo-checks.yml"
BAZEL_WORKFLOW = ROOT / ".github" / "workflows" / "bazel.yml"

EXPECTED_Q026_POLICY_BLOB = "e0729bd796b342568c624d15faf3638a1372d01d"
EXPECTED_Q027_POLICY_BLOB = "a507da0da4ac370a73d79eb305b227f0a080170a"
EXPECTED_BAZEL_WORKFLOW_BLOB = "55c470b88085fea874fca38573d49fd0c1d18cfe"


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
    q026_policy = read(Q026_POLICY)
    q027_policy = read(Q027_POLICY)
    q027_test = read(Q027_TEST)
    q030_test = read(Q030_TEST)
    boundary = read(BOUNDARY)
    repo_checks = read(REPO_CHECKS)
    bazel_workflow = read(BAZEL_WORKFLOW)

    for path in (WRAPPER, Q027_TEST, Q030_TEST, BOUNDARY):
        require_executable(path)

    for path, expected, owner in (
        (Q026_POLICY, EXPECTED_Q026_POLICY_BLOB, "selected Q0.26 compatibility policy"),
        (Q027_POLICY, EXPECTED_Q027_POLICY_BLOB, "selected Q0.27 lane policy"),
        (BAZEL_WORKFLOW, EXPECTED_BAZEL_WORKFLOW_BLOB, "reviewed Bazel workflow"),
    ):
        require(
            git_blob_sha(path) == expected,
            f"{owner} drifted",
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
        'CANONICAL_CLIPPY_NEGATIVE_TARGET = "-//codex-rs/v8-poc:all"',
        "invalid_negative_targets",
        "outside the canonical V8 exclusion",
        "validate_keyless_windows_gnullvm_final_args as _validate_q021",
    ):
        require_token(q026_policy, token, "run_bazel_q022_negative_targets.py")

    for token in (
        "validate_keyless_windows_gnullvm_final_args as _validate_q026",
        "CANONICAL_ANNOUNCE_RC",
        "CANONICAL_REMOTE_DOWNLOAD_TOPLEVEL",
        "COMMIT_METADATA_PREFIX",
        "TEST_SHARD_METADATA_PREFIX",
        "RELEASE_DEBUG_METADATA",
        "RELEASE_COMPILATION_MODE",
        "RELEASE_RUSTC_FLAG",
        "RELEASE_EXEC_RUSTC_FLAG",
        "def _metadata_contract",
        "does not match GITHUB_SHA",
        "reviewed four-shard topology",
        "def _reject_unreviewed_options",
        "rejects unreviewed explicit options",
        "def _validate_q027_semantics",
    ):
        require_token(q027_policy, token, "run_bazel_q027_lane_semantics.py")

    for token in (
        "test_canonical_test_lane_passes",
        "test_canonical_clippy_lane_passes",
        "test_canonical_release_lane_passes",
        "test_release_semantics_are_all_required",
        "test_clippy_config_cannot_be_neutralized",
        "test_test_execution_cannot_be_weakened",
        "test_arbitrary_build_settings_fail_closed",
        "test_commit_metadata_is_bound_to_github_sha",
        "test_test_shard_metadata_is_bound",
    ):
        require_token(q027_test, token, "test_run_bazel_lane_semantics.py")

    for token in (
        "test_q026_canonical_clippy_negative_target_passes",
        "test_q026_arbitrary_clippy_negative_target_fails_closed",
    ):
        require_token(q030_test, token, "test_run_bazel_direct_bazel.py")

    for token in (
        "python3 .github/scripts/test_run_bazel_lane_semantics.py",
        "python3 .github/scripts/test_run_bazel_direct_bazel.py",
        "python3 scripts/verify-windows-gnullvm-lane-semantics.py",
    ):
        require_token(boundary, token, "qualification boundary fixture")

    require_token(
        repo_checks,
        "python3 -m unittest discover -s .github/scripts -p 'test_run_bazel*.py'",
        "repo-checks.yml",
    )

    print("PASS_WINDOWS_GNULLVM_LANE_SEMANTICS_SOURCE")


if __name__ == "__main__":
    main()
