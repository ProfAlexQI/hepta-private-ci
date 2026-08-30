#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
WRAPPER = ROOT / ".github" / "scripts" / "run_bazel_with_buildbuddy.py"
BASE = ROOT / ".github" / "scripts" / "run_bazel_with_buildbuddy_base.py"
POLICY = ROOT / ".github" / "scripts" / "run_bazel_q017_policy.py"
TEST = ROOT / ".github" / "scripts" / "test_run_bazel_final_command.py"
BOUNDARY = ROOT / ".github" / "scripts" / "test_run_bazel_qualification_boundary.sh"
BAZELRC = ROOT / ".bazelrc"
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
    test = read(TEST)
    boundary = read(BOUNDARY)
    read(BAZELRC)

    for path in (WRAPPER, BASE, TEST, BOUNDARY):
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
        "test_authenticated_windows_path_remains_remote_passthrough",
        "test_pinned_blob_identity_is_explicit",
    ):
        require_token(test, token, "test_run_bazel_final_command.py")

    require_token(
        boundary,
        "python3 .github/scripts/test_run_bazel_final_command.py",
        "qualification boundary fixture",
    )
    require_token(
        boundary,
        "python3 scripts/verify-windows-gnullvm-final-command.py",
        "qualification boundary fixture",
    )

    print("PASS_WINDOWS_GNULLVM_FINAL_COMMAND_SOURCE")


if __name__ == "__main__":
    main()
