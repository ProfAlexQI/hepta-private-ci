#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
CONTRACT = SCRIPTS / "run_bazel_q029_execution_context.py"
TEST = SCRIPTS / "test_run_bazel_execution_context.py"
FIXTURE = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
BOUNDARY = ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
REPO_CHECKS = ROOT / ".github" / "workflows" / "repo-checks.yml"
BAZEL_WORKFLOW = ROOT / ".github" / "workflows" / "bazel.yml"
SETUP_CI = ROOT / ".github" / "actions" / "setup-ci" / "action.yml"
SETUP_BAZEL = ROOT / ".github" / "actions" / "setup-bazel-ci" / "action.yml"
SETUP_DEV_DRIVE = SCRIPTS / "setup-dev-drive.ps1"
BAZELVERSION = ROOT / ".bazelversion"

EXPECTED_BLOBS = {
    SCRIPTS / "run_bazel_q027_lane_semantics.py": (
        "a507da0da4ac370a73d79eb305b227f0a080170a"
    ),
    SCRIPTS / "run_bazel_q028_startup_contract.py": (
        "6711a2d5cdb63466f1895d539391af44e48b793f"
    ),
    SCRIPTS / "test_run_bazel_startup_contract.py": (
        "db2968ba6da135a1edb023be3d518e6e1768e0f5"
    ),
    SCRIPTS / "run_bazel_with_buildbuddy_base.py": (
        "913708d5651678c1623faac2b18656c2b86300bb"
    ),
    SCRIPTS / "run-bazel-ci-impl.sh": (
        "2fe7cf37a0fddc1bb2f42f3e8a1e3b5a9e30f96b"
    ),
    ROOT / "scripts" / "verify-windows-gnullvm-final-command.py": (
        "234283066fc97a9d76ff40f043f790ef53bee29e"
    ),
    ROOT / "scripts" / "verify-windows-gnullvm-startup-contract.py": (
        "6d795d7764f489baf63189ee979a39e3b5408f99"
    ),
    BAZEL_WORKFLOW: "55c470b88085fea874fca38573d49fd0c1d18cfe",
    SETUP_CI: "8abd2dbd5f09585734f8213011a9ed540a2ee88e",
    SETUP_BAZEL: "ac4f5aa97c7556f6049bd1d0a33220759d9d13d1",
    SETUP_DEV_DRIVE: "dfd2ea1f0a3b9942e25a06c74a978864f77f615c",
    BAZELVERSION: "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e",
}
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
        fail(f"missing Q0.29 source path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    framed = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require(text: str, expected: str, owner: str) -> None:
    if expected not in text:
        fail(f"{owner} lacks Q0.29 contract text: {expected}")


def require_order(text: str, before: str, after: str, owner: str) -> None:
    require(text, before, owner)
    require(text, after, owner)
    if text.index(before) >= text.index(after):
        fail(f"{owner} must place {before!r} before {after!r}")


def main() -> None:
    for path, expected in EXPECTED_BLOBS.items():
        read(path)
        observed = git_blob_sha(path)
        if observed != expected:
            fail(
                "immutable Q0.28 execution input drifted: "
                f"{path.relative_to(ROOT)} expected {expected}, observed {observed}"
            )

    wrapper = read(WRAPPER)
    contract = read(CONTRACT)
    test = read(TEST)
    fixture = read(FIXTURE)
    boundary = read(BOUNDARY)
    repo_checks = read(REPO_CHECKS)
    bazel_workflow = read(BAZEL_WORKFLOW)
    setup_ci = read(SETUP_CI)
    setup_bazel = read(SETUP_BAZEL)
    setup_dev_drive = read(SETUP_DEV_DRIVE)

    for path in (WRAPPER, TEST, FIXTURE):
        if not path.stat().st_mode & stat.S_IXUSR:
            fail(
                "required Q0.29 launcher lost executable mode: "
                f"{path.relative_to(ROOT)}"
            )

    for expected in (
        "Q0.17-Q0.29 qualification ratchets",
        "from run_bazel_q028_startup_contract import (",
        "from run_bazel_q029_execution_context import bind_verified_bazelisk",
        "from run_bazel_q029_execution_context import prepare_bazelisk_environment",
        "validate_keyless_windows_gnullvm_execution_context",
        "def executable_command(",
        "command = bazel_command(*args, env=env)",
        "prepare_bazelisk_environment(env)",
        "command = bind_verified_bazelisk(command, env)",
        "validate_keyless_windows_gnullvm_execution_context(command, env)",
        "command = executable_command(*sys.argv[1:])",
    ):
        require(wrapper, expected, "BuildBuddy wrapper")
    require_order(
        wrapper,
        "validate_keyless_windows_gnullvm_final_args(command[1:], env)",
        "def executable_command(",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "command = bazel_command(*args, env=env)",
        "prepare_bazelisk_environment(env)",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "prepare_bazelisk_environment(env)",
        "command = bind_verified_bazelisk(command, env)",
        "BuildBuddy wrapper",
    )
    require_order(
        wrapper,
        "command = bind_verified_bazelisk(command, env)",
        "validate_keyless_windows_gnullvm_execution_context(command, env)",
        "BuildBuddy wrapper",
    )

    for expected in (
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
        "validate_keyless_windows_gnullvm_execution_context",
        "CODEX_BAZEL_BIN is forbidden",
        '"BAZELISK_FORMAT_URL"',
        '"BAZELISK_HOME_WINDOWS"',
        '"USE_BAZEL_FALLBACK_VERSION"',
        "workspace .bazeliskrc is forbidden",
        "runner-home .bazeliskrc is forbidden",
        "RUNNER_ENVIRONMENT",
        "RUNNER_ARCH",
        "GITHUB_JOB",
        "GITHUB_SHA",
        "BAZEL_TEST_SHARD_COUNT",
        "CI_BUILD_ROOT",
        "BAZEL_OUTPUT_USER_ROOT",
        "BAZEL_REPOSITORY_CACHE",
        "BAZEL_REPO_CONTENTS_CACHE",
        "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR",
        "if path.drive and not path.root and len(path.parts) == 1",
        "startup arguments are not exact",
        "compact execution log escaped RUNNER_TEMP",
        "Bazelisk executable SHA-256 drifted before launch",
    ):
        require(contract, expected, "Q0.29 execution-context policy")

    required_tests = (
        "test_wrapper_launch_order_binds_context_after_q027",
        "test_wrapper_non_keyless_path_remains_passthrough",
        "test_all_three_canonical_jobs_pass",
        "test_bare_dev_drive_root_is_canonicalized",
        "test_drive_relative_subdirectory_fails_closed",
        "test_unknown_job_fails_closed",
        "test_self_hosted_runner_fails_closed",
        "test_job_command_mismatch_fails_closed",
        "test_job_metadata_mismatch_fails_closed",
        "test_commit_metadata_drift_fails_closed",
        "test_shard_topology_drift_fails_closed",
        "test_additional_startup_option_fails_closed",
        "test_cache_root_drift_fails_closed",
        "test_execution_log_escape_fails_closed",
        "test_execution_log_job_mismatch_fails_closed",
        "test_bazelisk_override_fails_closed",
        "test_bazelisk_format_url_override_fails_closed",
        "test_codex_bazel_override_fails_closed",
        "test_conflicting_version_override_fails_closed",
        "test_bazelversion_drift_fails_closed",
        "test_workspace_bazeliskrc_fails_closed",
        "test_verified_bazelisk_replaces_argv_zero",
        "test_bazelisk_digest_drift_fails_closed",
    )
    for test_name in required_tests:
        require(test, test_name, "Q0.29 regression suite")

    for expected in (
        "python3 .github/scripts/test_run_bazel_startup_contract.py",
        "python3 .github/scripts/test_run_bazel_execution_context.py",
        "python3 scripts/verify-windows-gnullvm-startup-contract.py",
        "python3 scripts/verify-windows-gnullvm-execution-context.py",
    ):
        require(fixture, expected, "qualification fixture")
    for expected in (
        "python3 scripts/verify-windows-gnullvm-startup-contract.py",
        "python3 scripts/verify-windows-gnullvm-execution-context.py",
        '"startup_vector_source_contract":',
        '"execution_context_source_contract":',
        '"official_bazelisk_windows_x86_64_sha256":',
        '"official_bazel_windows_x86_64_sha256":',
        '"windows_executable_digest_executed": False',
        '"a0_candidate_qualified": False',
    ):
        require(boundary, expected, "qualification workflow")

    require(
        repo_checks,
        "python3 -m unittest discover -s .github/scripts "
        "-p 'test_run_bazel*.py'",
        "ordinary repo-checks",
    )
    for expected in (
        "test-windows-shard:",
        "BAZEL_TEST_SHARD_COUNT: 4",
        "--build_metadata=TAG_windows_test_shard=${BAZEL_TEST_SHARD}",
        "clippy:",
        "--build_metadata=TAG_job=clippy",
        "verify-release-build:",
        "--build_metadata=TAG_job=verify-release-build",
        "--windows-cross-compile",
    ):
        require(bazel_workflow, expected, "Bazel workflow")
    for expected in (
        'bazel_output_user_root="$CI_BUILD_ROOT/b"',
        'bazel_repository_cache="$CI_BUILD_ROOT/bazel-repository-cache"',
        (
            'bazel_repo_contents_cache="$CI_BUILD_ROOT/'
            'bazel-repo-contents-cache-$GITHUB_RUN_ID-$GITHUB_JOB"'
        ),
    ):
        require(setup_ci, expected, "setup-ci action")
    require(
        setup_bazel,
        "bazel-contrib/setup-bazel@c5acdfb288317d0b5c0bbd7a396a3dc868bb0f86",
        "setup-bazel-ci action",
    )
    require(setup_bazel, "bazelisk-version: 1.28.1", "setup-bazel-ci action")
    require(setup_dev_drive, '"CI_BUILD_ROOT=$Drive"', "Dev Drive setup")

    if BAZELVERSION.read_bytes() != b"9.0.0\n":
        fail(".bazelversion bytes drifted")
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

    print("PASS_WINDOWS_GNULLVM_Q0_29_EXECUTION_CONTEXT_SOURCE")


if __name__ == "__main__":
    main()
