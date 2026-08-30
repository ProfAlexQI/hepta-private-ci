#!/usr/bin/env python3

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
POLICY = ROOT / ".github" / "scripts" / "run_bazel_q027_closed_world.py"
WRAPPER = ROOT / ".github" / "scripts" / "run_bazel_with_buildbuddy.py"
TEST = ROOT / ".github" / "scripts" / "test_run_bazel_closed_world.py"
NEGATIVE_TEST = ROOT / ".github" / "scripts" / "test_run_bazel_negative_targets.py"
FINAL_VERIFIER = ROOT / "scripts" / "verify-windows-gnullvm-final-command.py"
BOUNDARY = ROOT / ".github" / "scripts" / "test_run_bazel_qualification_boundary.sh"


def fail(message: str) -> None:
    raise SystemExit(message)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing required path: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def require_token(text: str, token: str, owner: str) -> None:
    require(token in text, f"{owner} lacks required token: {token}")


def main() -> None:
    policy = read(POLICY)
    wrapper = read(WRAPPER)
    test = read(TEST)
    negative_test = read(NEGATIVE_TEST)
    final_verifier = read(FINAL_VERIFIER)
    boundary = read(BOUNDARY)

    for token in (
        "Q0.27 fail-closed startup and final-option authority policy",
        "ANNOUNCE_RC_FLAG_FAMILY",
        "REMOTE_DOWNLOAD_TOPLEVEL_FLAG_FAMILY",
        "STRICT_STARTUP_FLAGS",
        "Q017_VALIDATED_DYNAMIC_PREFIXES",
        "def _validate_exact_startup",
        "def _validate_metadata",
        "def _validate_closed_world_options",
        "rejects unrecognized final Bazel options",
        "startup vector; ",
        "validate_keyless_windows_gnullvm_final_args as _validate_q026",
    ):
        require_token(policy, token, "run_bazel_q027_closed_world.py")

    for token in (
        "from run_bazel_q027_closed_world import (",
        "validate_keyless_windows_gnullvm_final_args(command[1:], env)",
        "from run_bazel_q022_negative_targets import (",
    ):
        require_token(wrapper, token, "run_bazel_with_buildbuddy.py")

    for token in (
        "test_canonical_clippy_command_passes",
        "test_canonical_release_command_passes",
        "test_canonical_test_command_passes",
        "test_announce_rc_disable_alias_fails_closed",
        "test_announce_rc_false_form_fails_closed",
        "test_platform_specific_config_override_fails_closed",
        "test_invocation_policy_fails_closed",
        "test_dependency_override_families_fail_closed",
        "test_arbitrary_define_and_starlark_setting_fail_closed",
        "test_unrecognized_build_metadata_fails_closed",
        "test_commit_metadata_must_match_github_sha",
        "test_remote_download_boolean_override_fails_closed",
        "test_startup_repo_cache_reenable_fails_closed",
        "test_arbitrary_startup_jvm_option_fails_closed",
    ):
        require_token(test, token, "test_run_bazel_closed_world.py")

    for token in (
        "test_canonical_test_lane_passes",
        "test_canonical_clippy_lane_passes",
        "test_test_exclude_all_fails_closed",
        "test_clippy_arbitrary_exclusion_fails_closed",
        "test_release_target_drop_fails_closed",
    ):
        require_token(
            negative_test,
            token,
            "test_run_bazel_negative_targets.py",
        )

    for token in (
        "test qualification rejects ",
        "clippy qualification rejects ",
        "test_canonical_test_lane_passes",
        "test_clippy_arbitrary_exclusion_fails_closed",
    ):
        require_token(
            final_verifier,
            token,
            "verify-windows-gnullvm-final-command.py",
        )

    for token in (
        "python3 .github/scripts/test_run_bazel_closed_world.py",
        "python3 scripts/verify-windows-gnullvm-closed-world.py",
    ):
        require_token(boundary, token, "qualification boundary fixture")

    print("PASS_WINDOWS_GNULLVM_CLOSED_WORLD_SOURCE")


if __name__ == "__main__":
    main()
