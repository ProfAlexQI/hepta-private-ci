#!/usr/bin/env python3

"""Q0.29 compatibility plus Q0.34 workspace/target source verifier."""

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
Q028_POLICY = SCRIPTS / "run_bazel_q028_startup_contract.py"
Q029_POLICY = SCRIPTS / "run_bazel_q029_job_executable.py"
Q030_POLICY = SCRIPTS / "run_bazel_q030_direct_bazel.py"
Q034_POLICY = SCRIPTS / "run_bazel_q034_workspace_targets.py"
WRAPPER_BASE = SCRIPTS / "run_bazel_with_buildbuddy_base.py"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
JOB_TEST = SCRIPTS / "test_run_bazel_job_executable.py"
Q034_TEST = SCRIPTS / "test_run_bazel_workspace_targets.py"
FIXTURE = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
BOUNDARY = ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
BAZEL_WORKFLOW = ROOT / ".github" / "workflows" / "bazel.yml"
SETUP_CI = ROOT / ".github" / "actions" / "setup-ci" / "action.yml"
SETUP_BAZEL = ROOT / ".github" / "actions" / "setup-bazel-ci" / "action.yml"
DEV_DRIVE = SCRIPTS / "setup-dev-drive.ps1"
CLIPPY_TARGETS = ROOT / "scripts" / "list-bazel-clippy-targets.sh"
RELEASE_TARGETS = ROOT / "scripts" / "list-bazel-release-targets.sh"
BAZELVERSION = ROOT / ".bazelversion"

EXPECTED_Q028_POLICY_BLOB = "86225acd9158132df8cd5ae9dc6720205a7c47a6"
EXPECTED_Q029_POLICY_BLOB = "2d57d5e222b87a89b2f8b1c93c476f450b03e646"
EXPECTED_Q030_POLICY_BLOB = "1614f53c9572cdda3b1d7cf227f3a730e27b2adb"
EXPECTED_Q034_POLICY_BLOB = "4eec6be942606d9b5ba62f07064ece625afb2e06"
EXPECTED_WRAPPER_BASE_BLOB = "913708d5651678c1623faac2b18656c2b86300bb"
EXPECTED_WRAPPER_BLOB = "1a261df205703c9b903e54571162672dddb1d6ae"
EXPECTED_Q034_TEST_BLOB = "a53f8a086a4071b606607385543c1f2cf0a1f370"
EXPECTED_BAZEL_WORKFLOW_BLOB = "55c470b88085fea874fca38573d49fd0c1d18cfe"
EXPECTED_SETUP_CI_BLOB = "8abd2dbd5f09585734f8213011a9ed540a2ee88e"
EXPECTED_SETUP_BAZEL_BLOB = "890567be46f3fd78c11b89a20950bef2f7af4bf6"
EXPECTED_DEV_DRIVE_BLOB = "dfd2ea1f0a3b9942e25a06c74a978864f77f615c"
EXPECTED_CLIPPY_TARGETS_BLOB = "d12a256d00673350b1a10fd8070f7001840e65c4"
EXPECTED_RELEASE_TARGETS_BLOB = "154f0b3580f3ba3216b6e4b840a3a7364e24e007"
EXPECTED_BAZELVERSION_BLOB = "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e"


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing Q0.29/Q0.34 contract path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    framed = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def require_token(text: str, token: str, owner: str) -> None:
    require(token in text, f"{owner} lacks contract token: {token}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require_token(text, before, owner)
    require_token(text, after, owner)
    require(
        text.index(before) < text.index(after),
        f"{owner} must place {before!r} before {after!r}",
    )


def require_executable(path: Path) -> None:
    require(
        bool(path.stat().st_mode & stat.S_IXUSR),
        f"required executable lost mode: {path.relative_to(ROOT)}",
    )


def main() -> None:
    q028 = read(Q028_POLICY)
    q029 = read(Q029_POLICY)
    q030 = read(Q030_POLICY)
    q034 = read(Q034_POLICY)
    wrapper_base = read(WRAPPER_BASE)
    wrapper = read(WRAPPER)
    job_test = read(JOB_TEST)
    q034_test = read(Q034_TEST)
    fixture = read(FIXTURE)
    boundary = read(BOUNDARY)
    bazel_workflow = read(BAZEL_WORKFLOW)
    setup_ci = read(SETUP_CI)
    setup_bazel = read(SETUP_BAZEL)
    dev_drive = read(DEV_DRIVE)
    clippy_targets = read(CLIPPY_TARGETS)
    release_targets = read(RELEASE_TARGETS)

    for path in (WRAPPER, JOB_TEST, Q034_TEST, FIXTURE):
        require_executable(path)

    for path, expected, owner in (
        (Q028_POLICY, EXPECTED_Q028_POLICY_BLOB, "selected Q0.28 policy"),
        (Q029_POLICY, EXPECTED_Q029_POLICY_BLOB, "selected Q0.29 policy"),
        (Q030_POLICY, EXPECTED_Q030_POLICY_BLOB, "selected Q0.32 policy"),
        (Q034_POLICY, EXPECTED_Q034_POLICY_BLOB, "Q0.34 policy"),
        (WRAPPER_BASE, EXPECTED_WRAPPER_BASE_BLOB, "BuildBuddy base wrapper"),
        (WRAPPER, EXPECTED_WRAPPER_BLOB, "Q0.34 public wrapper"),
        (Q034_TEST, EXPECTED_Q034_TEST_BLOB, "Q0.34 regression"),
        (BAZEL_WORKFLOW, EXPECTED_BAZEL_WORKFLOW_BLOB, "Bazel workflow"),
        (SETUP_CI, EXPECTED_SETUP_CI_BLOB, "setup-ci action"),
        (SETUP_BAZEL, EXPECTED_SETUP_BAZEL_BLOB, "Q0.33 setup-bazel action"),
        (DEV_DRIVE, EXPECTED_DEV_DRIVE_BLOB, "Dev Drive setup"),
        (CLIPPY_TARGETS, EXPECTED_CLIPPY_TARGETS_BLOB, "Clippy targets"),
        (RELEASE_TARGETS, EXPECTED_RELEASE_TARGETS_BLOB, "release targets"),
    ):
        require(git_blob_sha(path) == expected, f"{owner} drifted")

    for token in (
        'REPOSITORY = "ProfHepta/hepta-private-ci"',
        'BAZEL_VERSION = "9.0.0"',
        'TEST_JOB = "test-windows-shard"',
        'CLIPPY_JOB = "clippy"',
        'RELEASE_JOB = "verify-release-build"',
        "CI_BUILD_ROOT must be a dedicated Windows drive root",
        "test shard requires positive workspace targets",
        "clippy requires target prefix",
        "release job requires the exact canonical release target payload",
        "job {job} rejects duplicate Bazel targets",
    ):
        require_token(q029, token, "Q0.29 executable-job policy")

    for token in (
        "resolve_verified_bazel_command",
        "validate_keyless_windows_gnullvm_command",
        "_validate_child_path(real_bazel, env)",
        "_validate_q028(command[1:], env)",
        "verified direct Bazel executable changed before launch",
    ):
        require_token(q030, token, "Q0.32 direct-Bazel policy")

    for token in (
        "TEST_TARGET_QUERY",
        "CLIPPY_TARGET_QUERY",
        "def posix_cksum(",
        "def _expected_test_targets(",
        "def _expected_clippy_targets(",
        "def _require_exact_targets(",
        "cwd=workspace",
        "target vector drifted from the reviewed generator",
        "return workspace",
    ):
        require_token(q034, token, "Q0.34 workspace/target policy")

    for token in (
        "test_posix_cksum_matches_reviewed_shell_generator",
        "test_exact_test_shard_vector_is_accepted",
        "test_omitted_or_substituted_test_target_fails_closed",
        "test_exact_clippy_vector_is_accepted",
        "test_omitted_clippy_target_fails_closed",
        "test_query_failure_does_not_echo_query_output",
        "test_wrapper_binds_final_windows_launch_cwd_and_environment",
    ):
        require_token(q034_test, token, "Q0.34 regression")

    for token in (
        "from run_bazel_q034_workspace_targets import (",
        "validate_keyless_windows_gnullvm_workspace_and_targets",
        "launch_cwd = validate_keyless_windows_gnullvm_workspace_and_targets(",
        "cwd=launch_cwd",
        "env=os.environ",
    ):
        require_token(wrapper, token, "Q0.34 public wrapper")
    require_order(
        wrapper,
        "validate_keyless_windows_gnullvm_command(command, os.environ)",
        "launch_cwd = validate_keyless_windows_gnullvm_workspace_and_targets(",
        "Q0.34 public wrapper",
    )
    require_order(
        wrapper,
        "launch_cwd = validate_keyless_windows_gnullvm_workspace_and_targets(",
        "cwd=launch_cwd",
        "Q0.34 public wrapper",
    )

    for token in (
        "test_all_three_canonical_jobs_pass",
        "test_drive_root_shorthand_is_canonical",
        "test_bazel_executable_overrides_fail_closed",
        "test_verified_bazelisk_replaces_argv0_with_absolute_path",
        "test_executable_is_rehashed_immediately_before_launch",
    ):
        require_token(job_test, token, "Q0.29 regression")

    for token in (
        "python3 .github/scripts/test_run_bazel_job_executable.py",
        "python3 .github/scripts/test_run_bazel_direct_bazel.py",
        "python3 .github/scripts/test_run_bazel_workspace_targets.py",
        "python3 scripts/verify-windows-gnullvm-job-executable.py",
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
    ):
        require_token(fixture, token, "qualification boundary fixture")

    for token in (
        "python3 scripts/verify-windows-gnullvm-job-executable.py",
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
    ):
        require_token(boundary, token, "qualification workflow")

    for token in (
        "test-windows-shard:",
        "BAZEL_TEST_SHARD_COUNT: 4",
        "LC_ALL=C sort",
        "clippy:",
        "list-bazel-clippy-targets.sh",
        "verify-release-build:",
        "list-bazel-release-targets.sh",
    ):
        require_token(bazel_workflow, token, "Bazel workflow")

    for token in (
        'bazel_output_base="$CI_BUILD_ROOT/o"',
        'bazel_output_user_root="$CI_BUILD_ROOT/b"',
        'bazel_repository_cache="$CI_BUILD_ROOT/bazel-repository-cache"',
        "bazel-repo-contents-cache-$GITHUB_RUN_ID-$GITHUB_JOB",
    ):
        require_token(setup_ci, token, "setup-ci action")
    for token in (
        "bazelisk-version: 1.28.1",
        "- name: Scrub setup-only Bazelisk GitHub token",
        "'BAZELISK_GITHUB_TOKEN='",
        "unset BAZELISK_GITHUB_TOKEN",
    ):
        require_token(setup_bazel, token, "setup-bazel-ci action")
    require_token(dev_drive, '"CI_BUILD_ROOT=$Drive"', "Dev Drive setup")
    require_token(
        wrapper_base,
        'env.get("CODEX_BAZEL_BIN", "bazel")',
        "BuildBuddy base wrapper",
    )

    for token in ('"//codex-rs/..."', '"-//codex-rs/v8-poc:all"'):
        require_token(clippy_targets, token, "Clippy target generator")
    for token in (
        '"//codex-rs/..."',
        '"-//codex-rs/core/tests/remote_env_windows:smoke-test"',
        '"-//codex-rs/v8-poc:all"',
    ):
        require_token(release_targets, token, "release target generator")

    require(BAZELVERSION.read_bytes() == b"9.0.0\n", ".bazelversion bytes drifted")
    require(
        git_blob_sha(BAZELVERSION) == EXPECTED_BAZELVERSION_BLOB,
        ".bazelversion Git blob drifted",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_29_JOB_EXECUTABLE_SOURCE")
    print("PASS_WINDOWS_GNULLVM_Q0_34_WORKSPACE_TARGET_AUTHORITY_SOURCE")


if __name__ == "__main__":
    main()
