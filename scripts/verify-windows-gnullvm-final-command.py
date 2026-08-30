#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WRAPPER = ROOT / ".github" / "scripts" / "run_bazel_with_buildbuddy.py"
BASE = ROOT / ".github" / "scripts" / "run_bazel_with_buildbuddy_base.py"
POLICY = ROOT / ".github" / "scripts" / "run_bazel_q017_policy.py"
NEGATIVE_POLICY = ROOT / ".github" / "scripts" / "run_bazel_q022_negative_targets.py"
TEST = ROOT / ".github" / "scripts" / "test_run_bazel_final_command.py"
NEGATIVE_TEST = ROOT / ".github" / "scripts" / "test_run_bazel_negative_targets.py"
BOUNDARY = ROOT / ".github" / "scripts" / "test_run_bazel_qualification_boundary.sh"
BAZELRC = ROOT / ".bazelrc"
BAZEL_WORKFLOW = ROOT / ".github" / "workflows" / "bazel.yml"
RELEASE_TARGETS = ROOT / "scripts" / "list-bazel-release-targets.sh"
EXPECTED_BASE_BLOB = "913708d5651678c1623faac2b18656c2b86300bb"
EXPECTED_BAZELRC_BLOB = "0736ecbb6e8183b31f0e2739abef901c47235e9d"


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
        f"required launcher lost executable mode: {path.relative_to(ROOT)}",
    )


def main() -> None:
    wrapper = read(WRAPPER)
    base = read(BASE)
    policy = read(POLICY)
    negative_policy = read(NEGATIVE_POLICY)
    test = read(TEST)
    negative_test = read(NEGATIVE_TEST)
    boundary = read(BOUNDARY)
    read(BAZELRC)
    bazel_workflow = read(BAZEL_WORKFLOW)
    release_targets = read(RELEASE_TARGETS)

    for path in (WRAPPER, BASE, TEST, NEGATIVE_TEST, BOUNDARY, RELEASE_TARGETS):
        require_executable(path)

    require(
        git_blob_sha(BASE) == EXPECTED_BASE_BLOB,
        "compatibility base wrapper drifted from the reviewed Q0.19 implementation",
    )
    require(
        git_blob_sha(BAZELRC) == EXPECTED_BAZELRC_BLOB,
        "reviewed workspace .bazelrc blob drifted",
    )

    for token in (
        "import run_bazel_with_buildbuddy_base as _base",
        "from run_bazel_with_buildbuddy_base import *",
        "from run_bazel_q017_policy import QUALIFICATION_BAZELRC_GIT_BLOB_SHA1",
        "from run_bazel_q022_negative_targets import (",
        "from run_bazel_q027_closed_world import (",
        "validate_keyless_windows_gnullvm_final_args(command[1:], env)",
        '"--nomaster_bazelrc"',
        '"--nosystem_rc"',
        '"--noworkspace_rc"',
        '"--nohome_rc"',
        'f"--bazelrc={bazelrc}"',
        '"--announce_rc"',
    ):
        require_token(wrapper, token, "run_bazel_with_buildbuddy.py")

    for token in (
        f'QUALIFICATION_BAZELRC_GIT_BLOB_SHA1 = "{EXPECTED_BAZELRC_BLOB}"',
        "CI_SPLIT_FORM_FORBIDDEN",
        "CI_REMOTE_ENDPOINT_PREFIXES",
        "CI_REMOTE_EXECUTION_PREFIXES",
        "CI_RC_CONTROL_FLAGS",
        "credential-free Windows gnullvm qualification forbids user.bazelrc",
        "credential-free Windows gnullvm qualification rejects remote endpoint",
        "credential-free Windows gnullvm qualification rejects execution override",
        "ci-windows must be the final command-line config",
        "action_env and host_action_env bindings must be identical",
        "invalid Bazel target payload",
    ):
        require_token(policy, token, "run_bazel_q017_policy.py")

    for token in (
        'RELEASE_JOB_METADATA = "--build_metadata=TAG_job=verify-release-build"',
        'CANONICAL_RELEASE_TARGETS = (',
        '"//codex-rs/..."',
        '"-//codex-rs/core/tests/remote_env_windows:smoke-test"',
        '"-//codex-rs/v8-poc:all"',
        "the exact canonical target set",
        "test qualification rejects ",
        "clippy qualification rejects ",
        "validate_keyless_windows_gnullvm_final_args as _validate_q021",
    ):
        require_token(
            negative_policy,
            token,
            "run_bazel_q022_negative_targets.py",
        )

    for token in (
        "test_exact_command_is_bound_to_one_reviewed_rc_and_announces_it",
        "test_workspace_bazelrc_drift_fails_closed",
        "test_workspace_user_bazelrc_fails_closed",
        "test_split_form_authority_option_fails_closed",
        "test_remote_endpoint_injection_fails_closed",
        "test_additional_strategy_injection_fails_closed",
        "test_additional_action_environment_fails_closed",
        "test_duplicate_exact_metadata_fails_closed",
        "test_caller_rc_control_fails_closed",
        "test_boolean_equals_rc_reenable_fails_closed",
        "test_master_bazelrc_reenable_fails_closed",
        "test_option_smuggling_after_target_separator_fails_closed",
        "test_release_target_exclusions_remain_canonical_payload",
        "test_authenticated_windows_path_remains_remote_passthrough",
        "test_pinned_blob_identity_is_explicit",
    ):
        require_token(test, token, "test_run_bazel_final_command.py")

    for token in (
        "test_canonical_test_lane_passes",
        "test_canonical_clippy_lane_passes",
        "test_exact_release_target_set_passes",
        "test_test_exclude_all_fails_closed",
        "test_clippy_arbitrary_exclusion_fails_closed",
        "test_release_target_drop_fails_closed",
        "test_release_target_addition_fails_closed",
        "test_release_metadata_on_test_command_fails_closed",
        "test_unclassified_build_fails_closed",
    ):
        require_token(
            negative_test,
            token,
            "test_run_bazel_negative_targets.py",
        )

    for token in (
        '"//codex-rs/..."',
        '"-//codex-rs/core/tests/remote_env_windows:smoke-test"',
        '"-//codex-rs/v8-poc:all"',
    ):
        require_token(release_targets, token, "list-bazel-release-targets.sh")

    for token in (
        'bazel_target_lines="$(bash ./scripts/list-bazel-release-targets.sh)"',
        "--build_metadata=TAG_job=verify-release-build",
        "bazel_wrapper_args+=(--windows-cross-compile)",
    ):
        require_token(bazel_workflow, token, "bazel.yml")

    for token in (
        "python3 .github/scripts/test_run_bazel_final_command.py",
        "python3 .github/scripts/test_run_bazel_negative_targets.py",
        "python3 scripts/verify-windows-gnullvm-final-command.py",
    ):
        require_token(boundary, token, "qualification boundary fixture")

    print("PASS_WINDOWS_GNULLVM_FINAL_COMMAND_SOURCE")


if __name__ == "__main__":
    main()
