#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
LANE_POLICY = SCRIPTS / "run_bazel_q022_negative_targets.py"
EXECUTABLE_MODULES = (
    SCRIPTS / "run_bazel_q027_common.py",
    SCRIPTS / "run_bazel_q027_bazelisk.py",
    SCRIPTS / "run_bazel_q027_paths.py",
    SCRIPTS / "run_bazel_q027_lane.py",
    SCRIPTS / "run_bazel_q027_executable_contract.py",
)
TEST_SUPPORT = SCRIPTS / "_q027_test_support.py"
EXECUTABLE_TEST = SCRIPTS / "test_run_bazel_executable_contract.py"
JOB_TEST = SCRIPTS / "test_run_bazel_job_contract.py"
BOUNDARY_FIXTURE = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
BOUNDARY_WORKFLOW = (
    ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
)
BAZEL_WORKFLOW = ROOT / ".github" / "workflows" / "bazel.yml"
SETUP_CI = ROOT / ".github" / "actions" / "setup-ci" / "action.yml"
SETUP_BAZEL = ROOT / ".github" / "actions" / "setup-bazel-ci" / "action.yml"
PREPARE_BAZEL = ROOT / ".github" / "actions" / "prepare-bazel-ci" / "action.yml"
CLIPPY_TARGETS = ROOT / "scripts" / "list-bazel-clippy-targets.sh"
RELEASE_TARGETS = ROOT / "scripts" / "list-bazel-release-targets.sh"
BAZELVERSION = ROOT / ".bazelversion"

BAZELVERSION_BLOB = "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e"
BAZELISK_SHA256 = (
    "b9d65a1f7c2d7af885a96a4fd5aa36b40fb41816d30944390569eef908bdc954"
)
BAZEL_SHA256 = (
    "463faee497df2913854d80776784137cb47f42960b4ef4e4f85068c8da4849a8"
)


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing Q0.27 contract path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    payload = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(payload, usedforsecurity=False).hexdigest()


def require(text: str, token: str, owner: str) -> None:
    if token not in text:
        fail(f"{owner} lacks Q0.27 contract token: {token}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require(text, before, owner)
    require(text, after, owner)
    if text.index(before) >= text.index(after):
        fail(f"{owner} must place {before!r} before {after!r}")


def require_executable(path: Path) -> None:
    if not path.stat().st_mode & stat.S_IXUSR:
        fail(f"required executable lost mode: {path.relative_to(ROOT)}")


def main() -> None:
    wrapper = read(WRAPPER)
    lane_policy = read(LANE_POLICY)
    policy = "\n".join(read(path) for path in EXECUTABLE_MODULES)
    support = read(TEST_SUPPORT)
    executable_test = read(EXECUTABLE_TEST)
    job_test = read(JOB_TEST)
    fixture = read(BOUNDARY_FIXTURE)
    boundary_workflow = read(BOUNDARY_WORKFLOW)
    bazel_workflow = read(BAZEL_WORKFLOW)
    setup_ci = read(SETUP_CI)
    setup_bazel = read(SETUP_BAZEL)
    prepare_bazel = read(PREPARE_BAZEL)
    clippy_targets = read(CLIPPY_TARGETS)
    release_targets = read(RELEASE_TARGETS)

    for path in (
        WRAPPER,
        EXECUTABLE_TEST,
        JOB_TEST,
        BOUNDARY_FIXTURE,
        CLIPPY_TARGETS,
    ):
        require_executable(path)

    for token in (
        'CANONICAL_CLIPPY_NEGATIVE_TARGET = "-//codex-rs/v8-poc:all"',
        "outside the canonical V8 exclusion",
        "Q0.22-Q0.27 fail-closed",
    ):
        require(lane_policy, token, "Q0.22-Q0.27 lane policy")

    for token in (
        "Q0.17-Q0.27 qualification ratchets",
        "prepare_bazelisk_environment(os.environ)",
        "command = resolve_verified_bazel_command(command, os.environ)",
        "validate_keyless_windows_gnullvm_command(command, os.environ)",
        'f"--output_base={output_base}"',
        "rejects caller startup options",
        "os.execvpe(command[0], command, os.environ)",
    ):
        require(wrapper, token, "BuildBuddy wrapper")
    require_order(
        wrapper,
        "prepare_bazelisk_environment(os.environ)",
        "command = resolve_verified_bazel_command(command, os.environ)",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "command = resolve_verified_bazel_command(command, os.environ)",
        "validate_keyless_windows_gnullvm_command(command, os.environ)",
        "BuildBuddy wrapper",
    )

    for token in (
        'REPOSITORY = "ProfHepta/hepta-private-ci"',
        'BAZEL_VERSION = "9.0.0"',
        'BAZELISK_VERSION = "1.28.1"',
        BAZELISK_SHA256,
        BAZEL_SHA256,
        'TEST_JOB = "test-windows-shard"',
        'CLIPPY_JOB = "clippy"',
        'RELEASE_JOB = "verify-release-build"',
        "prepare_bazelisk_environment",
        "resolve_verified_bazel_command",
        "validate_keyless_windows_gnullvm_command",
        '[str(bazelisk), "--print_env"]',
        "cached Bazel executable SHA-256 drifted",
        "Bazelisk content-addressed store",
        'env["PATH"] = child_path',
        "startup arguments are not exact",
        "requires exact build metadata",
        "requires exact configs",
        "rejects unclassified Bazel option",
        "exact canonical release target payload",
        "canonical //codex-rs/... plus V8 exclusion prefix",
        "RUNNER_ENVIRONMENT",
        "RUNNER_ARCH",
        "GITHUB_JOB",
        "BAZEL_OUTPUT_BASE",
        "CODEX_BAZEL_BIN",
        "Bazelisk config file is forbidden",
    ):
        require(policy, token, "Q0.27 modular executable policy")

    for token in (
        "class Q027TestCase",
        "validate_keyless_windows_gnullvm_command",
        "execution-log-{command_name}-{job}-123.zst",
    ):
        require(support, token, "Q0.27 test support")
    for token in (
        "test_q026_clippy_canonical_negative_target_passes",
        "test_q026_clippy_arbitrary_negative_target_fails",
        "test_output_base_is_explicit_startup_input",
        "test_caller_startup_option_fails_closed",
        "test_resolver_verifies_bazelisk_and_cached_bazel",
        "test_cached_bazel_is_rehashed_even_when_bazelisk_succeeds",
        "test_bazelisk_redirect_override_fails_closed",
        "test_home_bazeliskrc_fails_closed",
    ):
        require(executable_test, token, "Q0.27 executable regression")
    for token in (
        "test_all_three_real_jobs_pass",
        "test_job_identity_cannot_be_spoofed_by_metadata",
        "test_clippy_rejects_extra_allowlisted_config",
        "test_release_job_rejects_test_command",
        "test_output_base_escape_fails_closed",
        "test_unclassified_option_fails_closed",
    ):
        require(job_test, token, "Q0.27 job regression")

    for token in (
        "python3 .github/scripts/test_run_bazel_executable_contract.py",
        "python3 .github/scripts/test_run_bazel_job_contract.py",
        "python3 scripts/verify-windows-gnullvm-executable-contract.py",
    ):
        require(fixture, token, "qualification fixture")
    require(
        boundary_workflow,
        "python3 scripts/verify-windows-gnullvm-executable-contract.py",
        "qualification workflow",
    )
    for token in (
        '"executable_contract": '
        '"PASS_WINDOWS_GNULLVM_Q0_27_EXECUTABLE_CONTRACT_SOURCE"',
        '"bazelisk_version": "1.28.1"',
        '"bazel_version": "9.0.0"',
        '"cached_bazel_rehashed": True',
        '"job_identity_bound": True',
        '"output_base_explicit": True',
    ):
        require(boundary_workflow, token, "qualification receipt")

    for token in (
        'bazel_output_base="$CI_BUILD_ROOT/o"',
        'bazel_output_user_root="$CI_BUILD_ROOT/b"',
        'bazel_repository_cache="$CI_BUILD_ROOT/bazel-repository-cache"',
        'bazel_repo_contents_cache="$CI_BUILD_ROOT/'
        'bazel-repo-contents-cache-$GITHUB_RUN_ID-$GITHUB_JOB"',
        'echo "BAZEL_OUTPUT_BASE=$bazel_output_base"',
        'echo "BAZEL_OUTPUT_USER_ROOT=$bazel_output_user_root"',
    ):
        require(setup_ci, token, "setup-ci action")
    for token in (
        "bazel-contrib/setup-bazel@c5acdfb288317d0b5c0bbd7a396a3dc868bb0f86",
        "bazelisk-version: 1.28.1",
        "output-base: ${{ steps.setup_ci.outputs.bazel-output-base }}",
    ):
        require(setup_bazel, token, "setup-bazel-ci action")
    for token in (
        "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR=",
        "${RUNNER_TEMP}/bazel-execution-logs",
    ):
        require(prepare_bazel, token, "prepare-bazel-ci action")

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
    require(clippy_targets, "echo //codex-rs/...", "Clippy target generator")
    require(
        clippy_targets,
        "echo -- -//codex-rs/v8-poc:all",
        "Clippy target generator",
    )
    for token in (
        '"//codex-rs/..."',
        '"-//codex-rs/core/tests/remote_env_windows:smoke-test"',
        '"-//codex-rs/v8-poc:all"',
    ):
        require(release_targets, token, "release target generator")

    if BAZELVERSION.read_bytes() != b"9.0.0\n":
        fail(".bazelversion bytes drifted from Bazel 9.0.0")
    if git_blob_sha(BAZELVERSION) != BAZELVERSION_BLOB:
        fail(".bazelversion Git blob identity drifted")
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

    print("PASS_WINDOWS_GNULLVM_Q0_27_EXECUTABLE_CONTRACT_SOURCE")


if __name__ == "__main__":
    main()
