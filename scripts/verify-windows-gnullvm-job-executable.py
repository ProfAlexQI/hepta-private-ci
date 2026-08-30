#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
Q028_POLICY = SCRIPTS / "run_bazel_q028_startup_contract.py"
Q029_POLICY = SCRIPTS / "run_bazel_q029_job_executable.py"
Q030_POLICY = SCRIPTS / "run_bazel_q030_direct_bazel.py"
Q034_POLICY = SCRIPTS / "run_bazel_q034_execution_manifest.py"
WRAPPER_BASE = SCRIPTS / "run_bazel_with_buildbuddy_base.py"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
JOB_TEST = SCRIPTS / "test_run_bazel_job_executable.py"
EXECUTION_TEST = SCRIPTS / "test_run_bazel_execution_manifest.py"
FIXTURE = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
BOUNDARY = (
    ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
)
BAZEL_WORKFLOW = ROOT / ".github" / "workflows" / "bazel.yml"
SETUP_CI = ROOT / ".github" / "actions" / "setup-ci" / "action.yml"
SETUP_BAZEL = ROOT / ".github" / "actions" / "setup-bazel-ci" / "action.yml"
DEV_DRIVE = SCRIPTS / "setup-dev-drive.ps1"
CLIPPY_TARGETS = ROOT / "scripts" / "list-bazel-clippy-targets.sh"
RELEASE_TARGETS = ROOT / "scripts" / "list-bazel-release-targets.sh"
BAZELVERSION = ROOT / ".bazelversion"

EXPECTED_Q028_BLOB = "86225acd9158132df8cd5ae9dc6720205a7c47a6"
EXPECTED_Q029_BLOB = "2d57d5e222b87a89b2f8b1c93c476f450b03e646"
EXPECTED_Q030_BLOB = "1614f53c9572cdda3b1d7cf227f3a730e27b2adb"
EXPECTED_Q034_BLOB = "6e0f9b873b1bf3585781ef1c12e41f29eeb0403a"
EXPECTED_WRAPPER_BASE_BLOB = "913708d5651678c1623faac2b18656c2b86300bb"
EXPECTED_WRAPPER_BLOB = "cf5e7d990e1c649dac505ff98199cffa60def08d"
EXPECTED_JOB_TEST_BLOB = "07d4cbdc50c0acc8bd6e76d88cef21b8b7990b46"
EXPECTED_EXECUTION_TEST_BLOB = "d6fb968f7bdc1721152f48e81bdd2415ff50f6f4"
EXPECTED_FIXTURE_BLOB = "ef870de5dbb9dbcc560d930051cc9f1c464cea68"
EXPECTED_BOUNDARY_BLOB = "3ec965f7483deb177556b83a1ecef36702c3e680"
EXPECTED_BAZEL_WORKFLOW_BLOB = "55c470b88085fea874fca38573d49fd0c1d18cfe"
EXPECTED_SETUP_CI_BLOB = "8abd2dbd5f09585734f8213011a9ed540a2ee88e"
EXPECTED_SETUP_BAZEL_BLOB = "890567be46f3fd78c11b89a20950bef2f7af4bf6"
EXPECTED_DEV_DRIVE_BLOB = "dfd2ea1f0a3b9942e25a06c74a978864f77f615c"
EXPECTED_CLIPPY_TARGETS_BLOB = "a7f17d0aaffc280d711a57f64dfdaa10e8c12c58"
EXPECTED_RELEASE_TARGETS_BLOB = "154f0b3580f3ba3216b6e4b840a3a7364e24e007"
EXPECTED_BAZELVERSION_BLOB = "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e"


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing Q0.29-Q0.34 contract path: {path.relative_to(ROOT)}")
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


def require_blob(path: Path, expected: str, owner: str) -> None:
    observed = git_blob_sha(path)
    require(
        observed == expected,
        f"{owner} drifted: expected {expected}, observed {observed}",
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
    execution_test = read(EXECUTION_TEST)
    fixture = read(FIXTURE)
    boundary = read(BOUNDARY)
    bazel_workflow = read(BAZEL_WORKFLOW)
    setup_ci = read(SETUP_CI)
    setup_bazel = read(SETUP_BAZEL)
    dev_drive = read(DEV_DRIVE)
    clippy_targets = read(CLIPPY_TARGETS)
    release_targets = read(RELEASE_TARGETS)

    for path in (
        WRAPPER,
        JOB_TEST,
        EXECUTION_TEST,
        FIXTURE,
        CLIPPY_TARGETS,
        RELEASE_TARGETS,
    ):
        require_executable(path)

    for path, expected, owner in (
        (Q028_POLICY, EXPECTED_Q028_BLOB, "selected Q0.28 policy"),
        (Q029_POLICY, EXPECTED_Q029_BLOB, "selected Q0.29 policy"),
        (Q030_POLICY, EXPECTED_Q030_BLOB, "selected Q0.32 policy"),
        (Q034_POLICY, EXPECTED_Q034_BLOB, "selected Q0.34 policy"),
        (WRAPPER_BASE, EXPECTED_WRAPPER_BASE_BLOB, "BuildBuddy base wrapper"),
        (WRAPPER, EXPECTED_WRAPPER_BLOB, "public Bazel wrapper"),
        (JOB_TEST, EXPECTED_JOB_TEST_BLOB, "Q0.29 job test"),
        (
            EXECUTION_TEST,
            EXPECTED_EXECUTION_TEST_BLOB,
            "Q0.34 execution test",
        ),
        (FIXTURE, EXPECTED_FIXTURE_BLOB, "qualification fixture"),
        (BOUNDARY, EXPECTED_BOUNDARY_BLOB, "qualification workflow"),
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

    for token in (
        "Q0.28/Q0.30 exact startup-vector contract",
        "OUTPUT_USER_ROOT_PREFIX",
        "OUTPUT_BASE_PREFIX",
        "requires BAZEL_OUTPUT_USER_ROOT",
        "requires BAZEL_OUTPUT_BASE",
    ):
        require_token(q028, token, "Q0.28 startup policy")

    for token in (
        'REPOSITORY = "ProfHepta/hepta-private-ci"',
        'BAZEL_VERSION = "9.0.0"',
        'BAZELISK_VERSION = "1.28.1"',
        'TEST_JOB = "test-windows-shard"',
        'CLIPPY_JOB = "clippy"',
        'RELEASE_JOB = "verify-release-build"',
        "prepare_bazelisk_environment",
        "allow_drive_root_shorthand=True",
        "CI_BUILD_ROOT must be a dedicated Windows drive root",
        "GITHUB_SHA must be one lowercase 40-hex Git object ID",
        "exact canonical release target payload",
        "test shard requires positive workspace targets",
        "clippy requires target prefix",
    ):
        require_token(q029, token, "Q0.29 job policy")

    for token in (
        "resolve_verified_bazel_command",
        "validate_keyless_windows_gnullvm_command",
        "final PATH head is not the verified cached Bazel directory",
        "verified direct Bazel executable changed before launch",
    ):
        require_token(q030, token, "Q0.32 direct Bazel policy")

    for token in (
        "validate_keyless_windows_gnullvm_execution",
        "WINDOWS_TEST_QUERY",
        "CLIPPY_MANUAL_TEST_QUERY",
        "final Bazel target manifest is not exact",
        "GITHUB_WORKSPACE must not be a symlink",
    ):
        require_token(q034, token, "Q0.34 execution policy")

    for token in (
        "Q0.17-Q0.30 qualification ratchets",
        "from run_bazel_q030_direct_bazel import prepare_bazelisk_environment",
        "from run_bazel_q030_direct_bazel import resolve_verified_bazel_command",
        "validate_keyless_windows_gnullvm_command(command, os.environ)",
        "from run_bazel_q034_execution_manifest import (",
        "launch_cwd = validate_keyless_windows_gnullvm_execution(",
        "result = subprocess.run(command, check=False, cwd=launch_cwd)",
    ):
        require_token(wrapper, token, "BuildBuddy wrapper")
    for before, after in (
        (
            "prepare_bazelisk_environment(os.environ)",
            "command = resolve_verified_bazel_command(command, os.environ)",
        ),
        (
            "command = resolve_verified_bazel_command(command, os.environ)",
            "validate_keyless_windows_gnullvm_command(command, os.environ)",
        ),
        (
            "validate_keyless_windows_gnullvm_command(command, os.environ)",
            "launch_cwd = validate_keyless_windows_gnullvm_execution(",
        ),
    ):
        require_order(wrapper, before, after, "BuildBuddy wrapper")

    for token in (
        "test_all_three_canonical_jobs_pass",
        "test_drive_root_shorthand_is_canonical",
        "test_bazel_executable_overrides_fail_closed",
        "test_verified_bazelisk_replaces_argv0_with_absolute_path",
        "test_executable_is_rehashed_immediately_before_launch",
    ):
        require_token(job_test, token, "Q0.29 regression")

    for token in (
        "test_test_target_omission_addition_and_reorder_fail_closed",
        "test_clippy_target_omission_addition_and_reorder_fail_closed",
        "test_release_target_manifest_remains_exact_without_query",
        "test_qualifying_windows_launch_uses_canonical_workspace",
    ):
        require_token(execution_test, token, "Q0.34 regression")

    for token in (
        "python3 .github/scripts/test_run_bazel_job_executable.py",
        "python3 .github/scripts/test_run_bazel_direct_bazel.py",
        "python3 .github/scripts/test_run_bazel_execution_manifest.py",
        "python3 scripts/verify-windows-gnullvm-job-executable.py",
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
    ):
        require_token(fixture, token, "qualification boundary fixture")

    for token in (
        "python3 scripts/verify-windows-gnullvm-job-executable.py",
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
        '"final_bazel_cwd_bound_to_canonical_workspace": True',
    ):
        require_token(boundary, token, "qualification workflow")

    for token in (
        "test-windows-shard:",
        "BAZEL_TEST_SHARD_COUNT: 4",
        "--build_metadata=TAG_windows_test_shard=${BAZEL_TEST_SHARD}",
        "clippy:",
        "--build_metadata=TAG_job=clippy",
        "verify-release-build:",
        "--build_metadata=TAG_job=verify-release-build",
        "--build_metadata=TAG_rust_debug_assertions=off",
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
        "printf '%s\\n' 'BAZELISK_GITHUB_TOKEN='",
        "unset BAZELISK_GITHUB_TOKEN",
    ):
        require_token(setup_bazel, token, "setup-bazel-ci action")

    require_token(dev_drive, '"CI_BUILD_ROOT=$Drive"', "Dev Drive setup")
    require_token(
        wrapper_base,
        'env.get("CODEX_BAZEL_BIN", "bazel")',
        "BuildBuddy base wrapper",
    )
    require_token(clippy_targets, "| LC_ALL=C sort", "Clippy target generator")
    for token in ('"//codex-rs/..."', '"-//codex-rs/v8-poc:all"'):
        require_token(clippy_targets, token, "Clippy target generator")
    for token in (
        '"//codex-rs/..."',
        '"-//codex-rs/core/tests/remote_env_windows:smoke-test"',
        '"-//codex-rs/v8-poc:all"',
    ):
        require_token(release_targets, token, "release target generator")

    require(
        BAZELVERSION.read_bytes() == b"9.0.0\n",
        ".bazelversion bytes drifted",
    )
    require_blob(
        BAZELVERSION,
        EXPECTED_BAZELVERSION_BLOB,
        ".bazelversion",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_29_JOB_EXECUTABLE_SOURCE")
    print("PASS_WINDOWS_GNULLVM_Q0_34_JOB_CWD_TARGET_BINDING_SOURCE")


if __name__ == "__main__":
    main()
