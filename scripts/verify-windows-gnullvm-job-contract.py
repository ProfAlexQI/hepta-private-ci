#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
POLICY = SCRIPTS / "run_bazel_q017_policy.py"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
JOB_CONTRACT = SCRIPTS / "run_bazel_q023_job_contract.py"
JOB_TEST = SCRIPTS / "test_run_bazel_job_contract.py"
FIXTURE = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
BOUNDARY = ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
BAZEL_WORKFLOW = ROOT / ".github" / "workflows" / "bazel.yml"
REPO_CHECKS = ROOT / ".github" / "workflows" / "repo-checks.yml"
CLIPPY_TARGETS = ROOT / "scripts" / "list-bazel-clippy-targets.sh"
RELEASE_TARGETS = ROOT / "scripts" / "list-bazel-release-targets.sh"
BAZELVERSION = ROOT / ".bazelversion"

EXPECTED_FORMATTED_POLICY_BLOB = "ab1a40a8221d94cf4d16f9c60542823f46b53b9c"
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
        fail(f"missing Q0.23 contract path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    payload = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(payload, usedforsecurity=False).hexdigest()


def require(text: str, expected: str, owner: str) -> None:
    if expected not in text:
        fail(f"{owner} lacks Q0.23 contract text: {expected}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require(text, before, owner)
    require(text, after, owner)
    if text.index(before) >= text.index(after):
        fail(f"{owner} must place {before!r} before {after!r}")


def main() -> None:
    policy = read(POLICY)
    wrapper = read(WRAPPER)
    contract = read(JOB_CONTRACT)
    test = read(JOB_TEST)
    fixture = read(FIXTURE)
    boundary = read(BOUNDARY)
    bazel_workflow = read(BAZEL_WORKFLOW)
    repo_checks = read(REPO_CHECKS)
    clippy_targets = read(CLIPPY_TARGETS)
    release_targets = read(RELEASE_TARGETS)

    if git_blob_sha(POLICY) != EXPECTED_FORMATTED_POLICY_BLOB:
        fail("formatted Q0.17 policy blob drifted")
    if not WRAPPER.stat().st_mode & stat.S_IXUSR:
        fail("run_bazel_with_buildbuddy.py lost its executable bit")
    if not JOB_TEST.stat().st_mode & stat.S_IXUSR:
        fail("test_run_bazel_job_contract.py lost its executable bit")

    for expected in (
        "Q0.17-Q0.23 qualification ratchets",
        "prepare_bazelisk_environment(os.environ)",
        "command = bind_verified_bazelisk(command, os.environ)",
        "validate_keyless_windows_gnullvm_command(command, os.environ)",
        "legacy_policy_args(command[1:], env)",
        "_retired_q022_validator",
    ):
        require(wrapper, expected, "BuildBuddy wrapper")
    if "_retired_q022_validator(" in wrapper:
        fail("retired Q0.22 metadata validator remains executable")
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

    for expected in (
        'REPOSITORY = "ProfHepta/hepta-private-ci"',
        'BAZEL_VERSION = "9.0.0"',
        EXPECTED_BAZELISK_SHA256,
        EXPECTED_BAZEL_SHA256,
        'TEST_JOB = "test-windows-shard"',
        'CLIPPY_JOB = "clippy"',
        'RELEASE_JOB = "verify-release-build"',
        "prepare_bazelisk_environment",
        "bind_verified_bazelisk",
        "validate_keyless_windows_gnullvm_command",
        "exact build metadata",
        "exact configs",
        "startup arguments are not exact",
        "unclassified Bazel option",
        "exact canonical release target payload",
        "CLIPPY_TARGET_PREFIX",
        "BAZEL_TEST_SHARD_COUNT",
        "RUNNER_ENVIRONMENT",
        "RUNNER_ARCH",
        "GITHUB_JOB",
        "CODEX_BAZEL_BIN",
        "workspace .bazeliskrc is forbidden",
        "runner-home .bazeliskrc is forbidden",
    ):
        require(contract, expected, "Q0.23 job contract")

    required_tests = (
        "test_all_three_canonical_jobs_pass",
        "test_release_job_cannot_use_test_command",
        "test_clippy_cannot_claim_release_metadata",
        "test_clippy_rejects_extra_allowlisted_config",
        "test_test_shard_rejects_negative_target",
        "test_clippy_allows_only_its_canonical_negative_target",
        "test_release_payload_is_exact",
        "test_unknown_job_fails_closed",
        "test_unclassified_option_fails_closed",
        "test_additional_startup_option_fails_closed",
        "test_cache_root_drift_fails_closed",
        "test_bazelisk_override_fails_closed",
        "test_conflicting_version_override_fails_closed",
        "test_bazelversion_drift_fails_closed",
        "test_verified_bazelisk_is_replaced_with_absolute_path",
        "test_bazelisk_digest_drift_fails_closed",
    )
    for test_name in required_tests:
        require(test, test_name, "Q0.23 regression")

    for expected in (
        "python3 .github/scripts/test_run_bazel_job_contract.py",
        "python3 scripts/verify-windows-gnullvm-job-contract.py",
    ):
        require(fixture, expected, "qualification fixture")
    require(
        boundary,
        "python3 scripts/verify-windows-gnullvm-job-contract.py",
        "qualification workflow",
    )
    require(
        repo_checks,
        "python3 -m unittest discover -s .github/scripts "
        "-p 'test_run_bazel*.py'",
        "ordinary repo-checks",
    )

    workflow_tokens = (
        "test-windows-shard:",
        "BAZEL_TEST_SHARD_COUNT: 4",
        "--build_metadata=TAG_windows_test_shard=${BAZEL_TEST_SHARD}",
        "clippy:",
        "--build_metadata=TAG_job=clippy",
        "./scripts/list-bazel-clippy-targets.sh",
        "verify-release-build:",
        "--build_metadata=TAG_job=verify-release-build",
        "--build_metadata=TAG_rust_debug_assertions=off",
        "./scripts/list-bazel-release-targets.sh",
        "--windows-cross-compile",
    )
    for expected in workflow_tokens:
        require(bazel_workflow, expected, "Bazel workflow")

    require(clippy_targets, "echo //codex-rs/...", "Clippy target source")
    require(
        clippy_targets,
        "echo -- -//codex-rs/v8-poc:all",
        "Clippy target source",
    )
    for expected in (
        "//codex-rs/...",
        "-//codex-rs/core/tests/remote_env_windows:smoke-test",
        "-//codex-rs/v8-poc:all",
    ):
        require(release_targets, expected, "release target source")

    if BAZELVERSION.read_bytes() != b"9.0.0\n":
        fail(".bazelversion bytes drifted")
    if git_blob_sha(BAZELVERSION) != EXPECTED_BAZELVERSION_BLOB:
        fail(".bazelversion Git blob drifted")
    wrappers = [
        path
        for path in (ROOT / "tools").glob("bazel*")
        if path.name != "buildifier"
    ]
    if wrappers:
        fail(
            "Bazelisk tools/bazel wrapper surface is forbidden: "
            + ", ".join(str(path.relative_to(ROOT)) for path in wrappers)
        )

    print("PASS_WINDOWS_GNULLVM_Q0_23_JOB_EXECUTABLE_CONTRACT_SOURCE")


if __name__ == "__main__":
    main()
