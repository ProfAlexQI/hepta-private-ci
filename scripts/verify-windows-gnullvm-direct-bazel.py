#!/usr/bin/env python3

"""Q0.30-Q0.34 direct-Bazel, token, workspace, and target source verifier."""

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / ".github" / "scripts"
Q029_POLICY = SCRIPTS / "run_bazel_q029_job_executable.py"
Q030_POLICY = SCRIPTS / "run_bazel_q030_direct_bazel.py"
Q034_POLICY = SCRIPTS / "run_bazel_q034_workspace_targets.py"
WRAPPER = SCRIPTS / "run_bazel_with_buildbuddy.py"
DIRECT_TEST = SCRIPTS / "test_run_bazel_direct_bazel.py"
TOKEN_TEST = SCRIPTS / "test_run_bazel_setup_token_boundary.py"
Q034_TEST = SCRIPTS / "test_run_bazel_workspace_targets.py"
BOUNDARY = SCRIPTS / "test_run_bazel_qualification_boundary.sh"
WORKFLOW = ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
SETUP_BAZEL = ROOT / ".github" / "actions" / "setup-bazel-ci" / "action.yml"
JOB_VERIFIER = ROOT / "scripts" / "verify-windows-gnullvm-job-executable.py"
BAZELVERSION = ROOT / ".bazelversion"

EXPECTED_Q029_POLICY_BLOB = "2d57d5e222b87a89b2f8b1c93c476f450b03e646"
EXPECTED_Q030_POLICY_BLOB = "1614f53c9572cdda3b1d7cf227f3a730e27b2adb"
EXPECTED_Q034_POLICY_BLOB = "4eec6be942606d9b5ba62f07064ece625afb2e06"
EXPECTED_WRAPPER_BLOB = "1a261df205703c9b903e54571162672dddb1d6ae"
EXPECTED_DIRECT_TEST_BLOB = "f03bb5d31ce5bca1c82f9a6349b506387e43b8e7"
EXPECTED_TOKEN_TEST_BLOB = "5778dd884ef087362a99b665fbb1c60cf2dce5f0"
EXPECTED_Q034_TEST_BLOB = "a53f8a086a4071b606607385543c1f2cf0a1f370"
EXPECTED_BOUNDARY_BLOB = "967ce708d716230fb791e4f054ca624ed225a85a"
EXPECTED_WORKFLOW_BLOB = "f9baeaa495628400437a2599082e52cf9d236a13"
EXPECTED_SETUP_BAZEL_BLOB = "890567be46f3fd78c11b89a20950bef2f7af4bf6"
EXPECTED_JOB_VERIFIER_BLOB = "18df5d111edd64e0b2df1800d48ca3801cdfba97"
EXPECTED_BAZELVERSION_BLOB = "f7ee06693c17a06e2a0f51ef7eb2a61866e77b8e"
PINNED_SETUP_BAZEL = (
    "bazel-contrib/setup-bazel@"
    "c5acdfb288317d0b5c0bbd7a396a3dc868bb0f86"
)
SETUP_SCRUB_STEP = "- name: Scrub setup-only Bazelisk GitHub token"
SETUP_EMPTY_EXPORT = (
    "printf '%s\\n' 'BAZELISK_GITHUB_TOKEN=' >> \"$GITHUB_ENV\""
)


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing Q0.30-Q0.34 contract path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def git_blob_sha(path: Path) -> str:
    content = path.read_bytes()
    framed = f"blob {len(content)}\0".encode("ascii") + content
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def require_token(text: str, token: str, owner: str) -> None:
    require(token in text, f"{owner} lacks Q0.30-Q0.34 token: {token}")


def reject_token(text: str, token: str, owner: str) -> None:
    require(token not in text, f"{owner} contains forbidden token: {token}")


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
    q029 = read(Q029_POLICY)
    q030 = read(Q030_POLICY)
    q034 = read(Q034_POLICY)
    wrapper = read(WRAPPER)
    direct_test = read(DIRECT_TEST)
    token_test = read(TOKEN_TEST)
    q034_test = read(Q034_TEST)
    boundary = read(BOUNDARY)
    workflow = read(WORKFLOW)
    setup_bazel = read(SETUP_BAZEL)
    job_verifier = read(JOB_VERIFIER)

    for path in (
        WRAPPER,
        DIRECT_TEST,
        TOKEN_TEST,
        Q034_TEST,
        BOUNDARY,
        JOB_VERIFIER,
    ):
        require_executable(path)

    for path, expected, owner in (
        (Q029_POLICY, EXPECTED_Q029_POLICY_BLOB, "Q0.29 compatibility policy"),
        (Q030_POLICY, EXPECTED_Q030_POLICY_BLOB, "Q0.32 direct-Bazel policy"),
        (Q034_POLICY, EXPECTED_Q034_POLICY_BLOB, "Q0.34 workspace/target policy"),
        (WRAPPER, EXPECTED_WRAPPER_BLOB, "Q0.34 public wrapper"),
        (DIRECT_TEST, EXPECTED_DIRECT_TEST_BLOB, "Q0.32 direct-Bazel regression"),
        (TOKEN_TEST, EXPECTED_TOKEN_TEST_BLOB, "Q0.33 setup-token regression"),
        (Q034_TEST, EXPECTED_Q034_TEST_BLOB, "Q0.34 regression"),
        (BOUNDARY, EXPECTED_BOUNDARY_BLOB, "qualification fixture"),
        (WORKFLOW, EXPECTED_WORKFLOW_BLOB, "qualification workflow"),
        (SETUP_BAZEL, EXPECTED_SETUP_BAZEL_BLOB, "setup-bazel-ci action"),
        (JOB_VERIFIER, EXPECTED_JOB_VERIFIER_BLOB, "Q0.34 job verifier"),
    ):
        require(git_blob_sha(path) == expected, f"{owner} drifted")

    for token in (
        'TEST_JOB = "test-windows-shard"',
        'CLIPPY_JOB = "clippy"',
        'RELEASE_JOB = "verify-release-build"',
        "test shard requires positive workspace targets",
        "clippy requires target prefix",
        "release job requires the exact canonical release target payload",
    ):
        require_token(q029, token, "Q0.29 policy")

    for token in (
        'SETUP_BAZEL_TRANSPORT_TOKEN = "BAZELISK_GITHUB_TOKEN"',
        "def consume_setup_bazel_transport_token(",
        "def _require_transport_token_absent(",
        "resolve_verified_bazel_command",
        "cwd=workspace",
        "cached Bazel executable SHA-256 drifted",
        'env["PATH"] = child_path',
        "_validate_child_path(real_bazel, env)",
        "_validate_q028(command[1:], env)",
        "verified direct Bazel executable changed before launch",
    ):
        require_token(q030, token, "Q0.32 direct-Bazel policy")
    reject_token(
        q030,
        "stderr={result.stderr.strip()!r}",
        "Q0.32 direct-Bazel policy",
    )
    reject_token(q030, "result.stdout.strip()", "Q0.32 direct-Bazel policy")

    for token in (
        "TEST_TARGET_QUERY",
        "CLIPPY_TARGET_QUERY",
        "def posix_cksum(",
        "def _query_labels(",
        "cwd=workspace",
        "capture_output=True",
        "def _expected_test_targets(",
        "def _expected_clippy_targets(",
        "def _require_exact_targets(",
        "expected_sha256=",
        "observed_sha256=",
        "changed during target-vector recomputation",
        "return workspace",
    ):
        require_token(q034, token, "Q0.34 workspace/target policy")
    reject_token(q034, "result.stdout.strip()", "Q0.34 workspace/target policy")
    reject_token(q034, "result.stderr.strip()", "Q0.34 workspace/target policy")

    for token in (
        "Compatibility wrapper plus Q0.17-Q0.34 qualification ratchets",
        "from run_bazel_q034_workspace_targets import (",
        "validate_keyless_windows_gnullvm_workspace_and_targets",
        "launch_cwd = None",
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
        "test_prepare_consumes_setup_bazel_transport_token",
        "test_resolver_rejects_unconsumed_transport_token",
        "test_resolver_verifies_bazelisk_and_cached_bazel",
        "test_direct_bazel_is_rehashed_immediately_before_launch",
        "test_final_command_rejects_transport_token",
        "test_final_path_head_drift_fails_closed",
    ):
        require_token(direct_test, token, "Q0.32 direct-Bazel regression")

    for token in (
        "test_setup_bazel_is_centralized_in_scrubbed_composite_action",
        "test_scrub_step_is_immediately_after_setup_bazel",
        "test_scrub_exports_only_an_empty_value_without_reading_secret",
        "test_setup_download_retains_default_token_only_inside_upstream_action",
        PINNED_SETUP_BAZEL,
        SETUP_SCRUB_STEP,
        SETUP_EMPTY_EXPORT,
    ):
        require_token(token_test, token, "Q0.33 token regression")

    for token in (
        "test_posix_cksum_matches_reviewed_shell_generator",
        "test_exact_test_shard_vector_is_accepted",
        "test_omitted_or_substituted_test_target_fails_closed",
        "test_exact_clippy_vector_is_accepted",
        "test_omitted_clippy_target_fails_closed",
        "test_query_failure_does_not_echo_query_output",
        "test_release_reuses_existing_exact_payload_without_query",
        "test_wrapper_binds_final_windows_launch_cwd_and_environment",
    ):
        require_token(q034_test, token, "Q0.34 regression")

    for token in (
        "python3 .github/scripts/test_run_bazel_direct_bazel.py",
        "python3 .github/scripts/test_run_bazel_setup_token_boundary.py",
        "python3 .github/scripts/test_run_bazel_workspace_targets.py",
        "python3 scripts/verify-windows-gnullvm-job-executable.py",
        "python3 scripts/verify-windows-gnullvm-direct-bazel.py",
    ):
        require_token(boundary, token, "qualification boundary fixture")

    for token in (
        PINNED_SETUP_BAZEL,
        "bazelisk-version: 1.28.1",
        SETUP_SCRUB_STEP,
        SETUP_EMPTY_EXPORT,
        "unset BAZELISK_GITHUB_TOKEN",
        "- name: Configure Bazel repository cache",
    ):
        require_token(setup_bazel, token, "setup-bazel-ci action")
    require_order(
        setup_bazel,
        "- name: Set up Bazel",
        SETUP_SCRUB_STEP,
        "setup-bazel-ci action",
    )
    require_order(
        setup_bazel,
        SETUP_SCRUB_STEP,
        "- name: Configure Bazel repository cache",
        "setup-bazel-ci action",
    )
    scrub_start = setup_bazel.index(SETUP_SCRUB_STEP)
    scrub_end = setup_bazel.index(
        "- name: Configure Bazel repository cache",
        scrub_start,
    )
    scrub_block = setup_bazel[scrub_start:scrub_end]
    reject_token(scrub_block, "${BAZELISK_GITHUB_TOKEN", "setup scrub")
    reject_token(scrub_block, "$BAZELISK_GITHUB_TOKEN", "setup scrub")

    for token in (
        "final_bazel_cwd_bound_to_canonical_workspace",
        "test_target_vector_recomputed_and_exact",
        "clippy_target_vector_recomputed_and_exact",
        "release_target_vector_exact",
        '"workspace_target_vector_executed_on_this_linux_source_job": False',
        '"runtime_authority": False',
        '"production_authority": False',
        '"operator_acceptance": False',
        '"promotion": False',
        '"release_authority": False',
        '"callers_ratchet": False',
    ):
        require_token(workflow, token, "qualification workflow")

    require_token(
        job_verifier,
        f'EXPECTED_SETUP_BAZEL_BLOB = "{EXPECTED_SETUP_BAZEL_BLOB}"',
        "Q0.34 job verifier",
    )
    reject_token(
        job_verifier,
        'EXPECTED_SETUP_BAZEL_BLOB = "ac4f5aa97c7556f6049bd1d0a33220759d9d13d1"',
        "Q0.34 job verifier",
    )

    require(
        BAZELVERSION.read_bytes() == b"9.0.0\n",
        ".bazelversion bytes drifted",
    )
    require(
        git_blob_sha(BAZELVERSION) == EXPECTED_BAZELVERSION_BLOB,
        ".bazelversion Git blob drifted",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_30_DIRECT_BAZEL_SOURCE")
    print("PASS_WINDOWS_GNULLVM_Q0_32_TRANSPORT_AND_PATH_SOURCE")
    print("PASS_WINDOWS_GNULLVM_Q0_33_SETUP_TOKEN_JOB_BOUNDARY_SOURCE")
    print("PASS_WINDOWS_GNULLVM_Q0_34_WORKSPACE_TARGET_AUTHORITY_SOURCE")


if __name__ == "__main__":
    main()
