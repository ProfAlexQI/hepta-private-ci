#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import stat
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
POLICY = ROOT / ".github" / "scripts" / "run_bazel_q022_negative_targets.py"
FINAL_TEST = ROOT / ".github" / "scripts" / "test_run_bazel_final_command.py"
NEGATIVE_TEST = (
    ROOT / ".github" / "scripts" / "test_run_bazel_negative_targets.py"
)
LANE_TEST = ROOT / ".github" / "scripts" / "test_run_bazel_lane_policy.py"
BOUNDARY = ROOT / ".github" / "scripts" / "test_run_bazel_qualification_boundary.sh"
BAZEL_WORKFLOW = ROOT / ".github" / "workflows" / "bazel.yml"
RELEASE_TARGETS = ROOT / "scripts" / "list-bazel-release-targets.sh"

EXPECTED_BAZEL_WORKFLOW_BLOB = "55c470b88085fea874fca38573d49fd0c1d18cfe"
EXPECTED_RELEASE_TARGETS_BLOB = "154f0b3580f3ba3216b6e4b840a3a7364e24e007"


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
        f"required executable lost mode: {path.relative_to(ROOT)}",
    )


def main() -> None:
    policy = read(POLICY)
    final_test = read(FINAL_TEST)
    negative_test = read(NEGATIVE_TEST)
    lane_test = read(LANE_TEST)
    boundary = read(BOUNDARY)
    bazel_workflow = read(BAZEL_WORKFLOW)
    release_targets = read(RELEASE_TARGETS)

    for path in (
        FINAL_TEST,
        NEGATIVE_TEST,
        LANE_TEST,
        BOUNDARY,
        RELEASE_TARGETS,
    ):
        require_executable(path)

    require(
        git_blob_sha(BAZEL_WORKFLOW) == EXPECTED_BAZEL_WORKFLOW_BLOB,
        "Windows Bazel qualification workflow drifted",
    )
    require(
        git_blob_sha(RELEASE_TARGETS) == EXPECTED_RELEASE_TARGETS_BLOB,
        "reviewed release-target generator drifted",
    )

    for token in (
        'BUILD_METADATA_OPTION = "--build_metadata"',
        'JOB_METADATA_PREFIX = "--build_metadata=TAG_job="',
        'JOB_METADATA_LIKE_PREFIX = "--build_metadata=TAG_job"',
        'RELEASE_JOB_METADATA = "--build_metadata=TAG_job=verify-release-build"',
        'CLIPPY_JOB_METADATA = "--build_metadata=TAG_job=clippy"',
        'CANONICAL_TEST_TAG_FILTER = "--test_tag_filters=-argument-comment-lint"',
        "FORBIDDEN_SELECTION_SPLIT_FLAGS",
        "FORBIDDEN_SELECTION_PREFIXES",
        "SKIP_INCOMPATIBLE_FLAG_FAMILY",
        "TEST_VERBOSE_TIMEOUT_FLAG_FAMILY",
        '"--test_filter"',
        '"--test_arg"',
        '"--test_tag_filters"',
        '"--test_lang_filters"',
        '"--test_size_filters"',
        '"--test_timeout_filters"',
        '"--build_tag_filters"',
        '"--build_tests_only"',
        '"--nobuild_tests_only"',
        '"--test_filter="',
        '"--test_arg="',
        '"--test_lang_filters="',
        '"--test_size_filters="',
        '"--test_timeout_filters="',
        '"--build_tag_filters="',
        '"--build_tests_only="',
        '"--nobuild_tests_only="',
        "def _matches_flag_family",
        "def _flag_family",
        "def _require_exact_flag_family",
        "def _reject_flag_family",
        "rejects flag family",
        "split-form --build_metadata",
        "malformed TAG_job build metadata",
        "ambiguous TAG_job build metadata",
        "exact configs ('ci-windows',)",
        "exact configs ('clippy', 'ci-windows')",
        "one recognized lane metadata tag",
        "the exact canonical target set",
        "test qualification rejects negative targets",
        "clippy qualification rejects negative targets",
    ):
        require_token(policy, token, "run_bazel_q022_negative_targets.py")

    for token in (
        'CLIPPY_JOB_METADATA = "--build_metadata=TAG_job=clippy"',
        'RELEASE_JOB_METADATA = "--build_metadata=TAG_job=verify-release-build"',
        "def release_args",
        "test_release_target_exclusions_remain_canonical_payload",
        "test_arbitrary_negative_target_fails_closed",
    ):
        require_token(final_test, token, "test_run_bazel_final_command.py")

    for token in (
        "test_duplicate_release_job_metadata_fails_closed",
        "test_release_plus_alternate_job_metadata_fails_closed",
        "test_split_build_metadata_fails_closed",
        "test_malformed_job_metadata_fails_closed",
        "test_empty_job_metadata_fails_closed",
        "test_duplicate_clippy_job_metadata_fails_closed",
        "test_test_lane_rejects_job_metadata",
        "test_test_exclude_all_fails_closed",
        "test_clippy_arbitrary_exclusion_fails_closed",
        "test_release_target_drop_fails_closed",
        "test_release_target_addition_fails_closed",
        "test_release_target_reorder_fails_closed",
        "test_release_metadata_on_test_command_fails_closed",
        "test_unclassified_build_fails_closed",
    ):
        require_token(
            negative_test,
            token,
            "test_run_bazel_negative_targets.py",
        )

    for token in (
        "test_canonical_test_lane_passes",
        "test_canonical_clippy_lane_passes",
        "test_canonical_release_lane_passes",
        "test_extra_test_config_fails_closed",
        "test_test_filter_fails_closed",
        "test_test_arg_fails_closed",
        "test_split_selection_forms_fail_closed",
        "test_alternate_test_tag_filter_fails_closed",
        "test_duplicate_test_tag_filter_fails_closed",
        "test_build_tag_filter_fails_closed",
        "test_nobuild_tests_only_equals_form_fails_closed",
        "test_unclassified_build_fails_closed",
        "test_clippy_without_skip_incompatible_fails_closed",
        "test_clippy_skip_disable_alias_fails_closed",
        "test_clippy_skip_false_alias_fails_closed",
        "test_test_skip_disable_alias_fails_closed",
        "test_test_timeout_disable_alias_fails_closed",
        "test_test_timeout_false_alias_fails_closed",
        "test_build_timeout_false_alias_fails_closed",
        "test_release_skip_true_alias_fails_closed",
        "test_release_skip_disable_alias_fails_closed",
        "test_release_with_clippy_config_fails_closed",
        "test_release_with_arbitrary_exclusion_fails_closed",
        "test_test_negative_target_fails_closed",
    ):
        require_token(lane_test, token, "test_run_bazel_lane_policy.py")

    for token in (
        "python3 .github/scripts/test_run_bazel_final_command.py",
        "python3 .github/scripts/test_run_bazel_negative_targets.py",
        "python3 .github/scripts/test_run_bazel_lane_policy.py",
        "python3 scripts/verify-windows-gnullvm-final-command.py",
        "python3 scripts/verify-windows-gnullvm-lane-policy.py",
    ):
        require_token(boundary, token, "qualification boundary fixture")

    for token in (
        "--test_tag_filters=-argument-comment-lint",
        "--build_metadata=TAG_job=clippy",
        "--build_metadata=TAG_job=verify-release-build",
        'bazel_target_lines="$(bash ./scripts/list-bazel-release-targets.sh)"',
        "bazel_wrapper_args+=(--windows-cross-compile)",
    ):
        require_token(bazel_workflow, token, "bazel.yml")

    for forbidden in (
        "--test_filter=",
        "--test_arg=",
        "--test_lang_filters=",
        "--test_size_filters=",
        "--test_timeout_filters=",
        "--build_tag_filters=",
    ):
        require(
            forbidden not in bazel_workflow,
            f"bazel.yml contains forbidden selection override: {forbidden}",
        )

    expected_release_targets = (
        '"//codex-rs/..."',
        '"-//codex-rs/core/tests/remote_env_windows:smoke-test"',
        '"-//codex-rs/v8-poc:all"',
    )
    for token in expected_release_targets:
        require_token(release_targets, token, "list-bazel-release-targets.sh")

    print("PASS_WINDOWS_GNULLVM_LANE_POLICY_SOURCE")


if __name__ == "__main__":
    main()
