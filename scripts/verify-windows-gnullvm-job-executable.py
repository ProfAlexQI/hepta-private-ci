#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
Q028_POLICY = SCRIPTS / "run_bazel_q028_startup_contract.py"
Q029_POLICY = SCRIPTS / "run_bazel_q029_job_executable.py"
WRAPPER_BASE = SCRIPTS / "run_bazel_with_buildbuddy_base.py"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
TEST = SCRIPTS / "test_run_bazel_job_executable.py"
FIXTURE = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
BOUNDARY = ROOT / ".github" / "workflows" / (
    "windows-gnullvm-qualification-boundary.yml"
)
BAZEL_WORKFLOW = ROOT / ".github" / "workflows" / "bazel.yml"
SETUP_CI = ROOT / ".github" / "actions" / "setup-ci" / "action.yml"
SETUP_BAZEL = ROOT / ".github" / "actions" / "setup-bazel-ci" / "action.yml"
DEV_DRIVE = SCRIPTS / "setup-dev-drive.ps1"
CLIPPY_TARGETS = ROOT / "scripts" / "list-bazel-clippy-targets.sh"
RELEASE_TARGETS = ROOT / "scripts" / "list-bazel-release-targets.sh"
BAZELVERSION = ROOT / ".bazelversion"

EXPECTED_Q028_POLICY_BLOB = "6711a2d5cdb63466f1895d539391af44e48b793f"
EXPECTED_WRAPPER_BASE_BLOB = "913708d5651678c1623faac2b18656c2b86300bb"
EXPECTED_BAZEL_WORKFLOW_BLOB = "55c470b88085fea874fca38573d49fd0c1d18cfe"
EXPECTED_SETUP_CI_BLOB = "8abd2dbd5f09585734f8213011a9ed540a2ee88e"
EXPECTED_SETUP_BAZEL_BLOB = "ac4f5aa97c7556f6049bd1d0a33220759d9d13d1"
EXPECTED_DEV_DRIVE_BLOB = "dfd2ea1f0a3b9942e25a06c74a978864f77f615c"
EXPECTED_CLIPPY_TARGETS_BLOB = "d12a256d00673350b1a10fd8070f7001840e65c4"
EXPECTED_RELEASE_TARGETS_BLOB = "154f0b3580f3ba3216b6e4b840a3a7364e24e007"
EXPECTED_BAZELVERSION_BLOB = "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e"
EXPECTED_BAZELISK_SHA256 = (
    "b9d65a1f7c2d7af885a96a4fd5aa36b40fb41816d30944390569eef908bdc954"
)
EXPECTED_BAZEL_SHA256 = (
    "463faee497df2913854d80776784137cb47f42960b4ef4e4f85068c8da4849a8"
)


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing Q0.29 contract path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    framed = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require(text: str, token: str, owner: str) -> None:
    if token not in text:
        fail(f"{owner} lacks Q0.29 contract token: {token}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require(text, before, owner)
    require(text, after, owner)
    if text.index(before) >= text.index(after):
        fail(f"{owner} must place {before!r} before {after!r}")


def require_blob(path: Path, expected: str, owner: str) -> None:
    observed = git_blob_sha(path)
    if observed != expected:
        fail(
            f"{owner} Git blob drifted: expected {expected}, "
            f"observed {observed}"
        )


def require_executable(path: Path) -> None:
    if not path.stat().st_mode & stat.S_IXUSR:
        fail(f"required executable lost mode: {path.relative_to(ROOT)}")


def main() -> None:
    q028 = read(Q028_POLICY)
    q029 = read(Q029_POLICY)
    wrapper_base = read(WRAPPER_BASE)
    wrapper = read(WRAPPER)
    test = read(TEST)
    fixture = read(FIXTURE)
    boundary = read(BOUNDARY)
    bazel_workflow = read(BAZEL_WORKFLOW)
    setup_ci = read(SETUP_CI)
    setup_bazel = read(SETUP_BAZEL)
    dev_drive = read(DEV_DRIVE)
    clippy_targets = read(CLIPPY_TARGETS)
    release_targets = read(RELEASE_TARGETS)

    for path in (WRAPPER, TEST, FIXTURE):
        require_executable(path)

    for path, expected, owner in (
        (Q028_POLICY, EXPECTED_Q028_POLICY_BLOB, "selected Q0.28 policy"),
        (WRAPPER_BASE, EXPECTED_WRAPPER_BASE_BLOB, "BuildBuddy base wrapper"),
        (BAZEL_WORKFLOW, EXPECTED_BAZEL_WORKFLOW_BLOB, "Bazel workflow"),
        (SETUP_CI, EXPECTED_SETUP_CI_BLOB, "setup-ci action"),
        (SETUP_BAZEL, EXPECTED_SETUP_BAZEL_BLOB, "setup-bazel-ci action"),
        (DEV_DRIVE, EXPECTED_DEV_DRIVE_BLOB, "Dev Drive setup"),
        (
            CLIPPY_TARGETS,
            EXPECTED_CLIPPY_TARGETS_BLOB,
            "Clippy target generator",
        ),
        (
            RELEASE_TARGETS,
            EXPECTED_RELEASE_TARGETS_BLOB,
            "release target generator",
        ),
    ):
        require_blob(path, expected, owner)

    require(
        q028,
        "Q0.28 exact startup-vector contract",
        "selected Q0.28 startup policy",
    )

    for token in (
        "Q0.17-Q0.29 qualification ratchets",
        "from run_bazel_q028_startup_contract import (",
        "from run_bazel_q029_job_executable import bind_verified_bazelisk",
        "prepare_bazelisk_environment(os.environ)",
        "command = bind_verified_bazelisk(command, os.environ)",
        "validate_keyless_windows_gnullvm_command(command, os.environ)",
    ):
        require(wrapper, token, "BuildBuddy wrapper")
    require_order(
        wrapper,
        "prepare_bazelisk_environment(os.environ)",
        "command = bind_verified_bazelisk(command, os.environ)",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "command = bind_verified_bazelisk(command, os.environ)",
        "validate_keyless_windows_gnullvm_command(command, os.environ)",
        "BuildBuddy wrapper",
    )

    for token in (
        'REPOSITORY = "ProfHepta/hepta-private-ci"',
        'BAZEL_VERSION = "9.0.0"',
        'BAZELISK_VERSION = "1.28.1"',
        EXPECTED_BAZELISK_SHA256,
        EXPECTED_BAZEL_SHA256,
        'TEST_JOB = "test-windows-shard"',
        'CLIPPY_JOB = "clippy"',
        'RELEASE_JOB = "verify-release-build"',
        "prepare_bazelisk_environment",
        "bind_verified_bazelisk",
        "validate_keyless_windows_gnullvm_command",
        "allow_drive_root_shorthand=True",
        "CI_BUILD_ROOT must be a dedicated Windows drive root",
        'env.get("GITHUB_EVENT_NAME") not in {"pull_request", "push"}',
        "GITHUB_SHA must be one lowercase 40-hex Git object ID",
        "GITHUB_REPOSITORY",
        "RUNNER_ENVIRONMENT",
        "RUNNER_ARCH",
        "GITHUB_JOB",
        "CODEX_BAZEL_BIN",
        'name.startswith("BAZELISK_")',
        '"BAZELISK_GITHUB_TOKEN"',
        "BAZELISK_SKIP_WRAPPER",
        "BAZELISK_VERIFY_SHA256",
        "workspace Bazel wrapper surface is forbidden",
        "workspace .bazeliskrc is forbidden",
        "runner-home .bazeliskrc is forbidden",
        "_validate_q028(command[1:], env)",
        "Bazelisk executable SHA-256 drifted before launch",
        "exact canonical release target payload",
        "test shard requires positive workspace targets",
        "clippy requires target prefix",
    ):
        require(q029, token, "Q0.29 job/executable policy")

    required_tests = (
        "test_all_three_canonical_jobs_pass",
        "test_drive_root_shorthand_is_canonical",
        "test_unknown_job_fails_closed",
        "test_repository_runner_and_event_identity_fail_closed",
        "test_github_sha_must_be_canonical",
        "test_cache_and_build_root_drift_fail_closed",
        "test_drive_relative_and_nested_build_roots_fail_closed",
        "test_execution_log_escape_and_wrong_job_fail_closed",
        "test_job_metadata_cannot_spoof_another_lane",
        "test_test_and_clippy_targets_are_bound",
        "test_release_target_payload_is_exact",
        "test_bazel_executable_overrides_fail_closed",
        "test_setup_bazel_github_token_is_transport_only",
        "test_conflicting_required_bazelisk_control_fails_closed",
        "test_bazelversion_and_bazeliskrc_drift_fail_closed",
        "test_workspace_bazel_wrapper_is_forbidden",
        "test_verified_bazelisk_replaces_argv0_with_absolute_path",
        "test_unverified_argv0_symlink_and_digest_fail_closed",
        "test_executable_is_rehashed_immediately_before_launch",
    )
    for test_name in required_tests:
        require(test, test_name, "Q0.29 regression")

    for token in (
        "python3 .github/scripts/test_run_bazel_job_executable.py",
        "python3 scripts/verify-windows-gnullvm-job-executable.py",
    ):
        require(fixture, token, "qualification boundary fixture")
    require(
        boundary,
        "python3 scripts/verify-windows-gnullvm-job-executable.py",
        "qualification workflow",
    )

    for token in (
        "test-windows-shard:",
        "BAZEL_TEST_SHARD_COUNT: 4",
        "--build_metadata=TAG_windows_test_shard=${BAZEL_TEST_SHARD}",
        "clippy:",
        "--build_metadata=TAG_job=clippy",
        "verify-release-build:",
        "--build_metadata=TAG_job=verify-release-build",
        "--build_metadata=TAG_rust_debug_assertions=off",
        "--windows-cross-compile",
    ):
        require(bazel_workflow, token, "Bazel workflow")

    for token in (
        'bazel_output_base="$CI_BUILD_ROOT/o"',
        'bazel_output_user_root="$CI_BUILD_ROOT/b"',
        'bazel_repository_cache="$CI_BUILD_ROOT/bazel-repository-cache"',
        "bazel-repo-contents-cache-$GITHUB_RUN_ID-$GITHUB_JOB",
        'cargo_target_dir="$CI_BUILD_ROOT/cargo-target"',
        'tmp="$CI_BUILD_ROOT/tmp"',
    ):
        require(setup_ci, token, "setup-ci action")
    require(
        setup_bazel,
        "bazelisk-version: 1.28.1",
        "setup-bazel-ci action",
    )
    require(
        dev_drive,
        '"CI_BUILD_ROOT=$Drive"',
        "Dev Drive setup",
    )
    require(
        wrapper_base,
        'env.get("CODEX_BAZEL_BIN", "bazel")',
        "BuildBuddy base wrapper",
    )

    for token in (
        '"//codex-rs/..."',
        '"-//codex-rs/v8-poc:all"',
    ):
        require(clippy_targets, token, "Clippy target generator")
    for token in (
        '"//codex-rs/..."',
        '"-//codex-rs/core/tests/remote_env_windows:smoke-test"',
        '"-//codex-rs/v8-poc:all"',
    ):
        require(release_targets, token, "release target generator")

    if BAZELVERSION.read_bytes() != b"9.0.0\n":
        fail(".bazelversion bytes drifted")
    if git_blob_sha(BAZELVERSION) != EXPECTED_BAZELVERSION_BLOB:
        fail(".bazelversion Git blob drifted")

    wrappers = sorted((ROOT / "tools").glob("bazel*"))
    if wrappers:
        fail(
            "workspace Bazel wrapper surface is forbidden: "
            + ", ".join(str(path.relative_to(ROOT)) for path in wrappers)
        )

    print("PASS_WINDOWS_GNULLVM_Q0_29_JOB_EXECUTABLE_SOURCE")


if __name__ == "__main__":
    main()
