"""Q0.22-Q0.27 fail-closed Bazel identity, lane, and selection policy."""

from collections.abc import Mapping, Sequence

from run_bazel_q017_policy import *  # noqa: F403
from run_bazel_q017_policy import _option_args
from run_bazel_q017_policy import (
    validate_keyless_windows_gnullvm_final_args as _validate_q021,
)

BUILD_METADATA_OPTION = "--build_metadata"
JOB_METADATA_PREFIX = "--build_metadata=TAG_job="
JOB_METADATA_LIKE_PREFIX = "--build_metadata=TAG_job"
RELEASE_JOB_METADATA = "--build_metadata=TAG_job=verify-release-build"
CLIPPY_JOB_METADATA = "--build_metadata=TAG_job=clippy"
CANONICAL_TEST_TAG_FILTER = "--test_tag_filters=-argument-comment-lint"
CANONICAL_RELEASE_TARGETS = (
    "//codex-rs/...",
    "-//codex-rs/core/tests/remote_env_windows:smoke-test",
    "-//codex-rs/v8-poc:all",
)
CANONICAL_CLIPPY_NEGATIVE_TARGET = "-//codex-rs/v8-poc:all"
CANONICAL_SKIP_INCOMPATIBLE = "--skip_incompatible_explicit_targets"
CANONICAL_TEST_VERBOSE_TIMEOUTS = "--test_verbose_timeout_warnings"

FORBIDDEN_SELECTION_SPLIT_FLAGS = {
    "--test_filter",
    "--test_arg",
    "--test_tag_filters",
    "--test_lang_filters",
    "--test_size_filters",
    "--test_timeout_filters",
    "--build_tag_filters",
    "--build_tests_only",
    "--nobuild_tests_only",
}
FORBIDDEN_SELECTION_PREFIXES = (
    "--test_filter=",
    "--test_arg=",
    "--test_lang_filters=",
    "--test_size_filters=",
    "--test_timeout_filters=",
    "--build_tag_filters=",
    "--build_tests_only=",
    "--nobuild_tests_only=",
)
SKIP_INCOMPATIBLE_FLAG_FAMILY = (
    CANONICAL_SKIP_INCOMPATIBLE,
    "--noskip_incompatible_explicit_targets",
)
TEST_VERBOSE_TIMEOUT_FLAG_FAMILY = (
    CANONICAL_TEST_VERBOSE_TIMEOUTS,
    "--notest_verbose_timeout_warnings",
)


def _reject_selection_overrides(options: Sequence[str]) -> None:
    for option in options:
        if option in FORBIDDEN_SELECTION_SPLIT_FLAGS or option.startswith(
            FORBIDDEN_SELECTION_PREFIXES
        ):
            raise ValueError(
                "credential-free Windows gnullvm qualification rejects "
                f"test-selection override {option!r}"
            )


def _matches_flag_family(option: str, family: Sequence[str]) -> bool:
    return any(option == name or option.startswith(f"{name}=") for name in family)


def _flag_family(options: Sequence[str], family: Sequence[str]) -> list[str]:
    return [option for option in options if _matches_flag_family(option, family)]


def _require_exact_flag_family(
    options: Sequence[str],
    expected: str,
    family: Sequence[str],
    *,
    owner: str,
) -> None:
    observed = _flag_family(options, family)
    if observed != [expected]:
        raise ValueError(
            f"credential-free Windows gnullvm {owner} requires exactly "
            f"{expected!r}; observed {observed!r}"
        )


def _reject_flag_family(
    options: Sequence[str], family: Sequence[str], *, owner: str
) -> None:
    observed = _flag_family(options, family)
    if observed:
        raise ValueError(
            f"credential-free Windows gnullvm {owner} rejects flag family "
            f"{observed!r}"
        )


def _job_metadata(options: Sequence[str]) -> list[str]:
    if BUILD_METADATA_OPTION in options:
        raise ValueError(
            "credential-free Windows gnullvm qualification rejects split-form "
            "--build_metadata"
        )

    observed = [
        option for option in options if option.startswith(JOB_METADATA_LIKE_PREFIX)
    ]
    if any(
        not option.startswith(JOB_METADATA_PREFIX)
        or len(option) == len(JOB_METADATA_PREFIX)
        for option in observed
    ):
        raise ValueError(
            "credential-free Windows gnullvm qualification rejects malformed "
            "TAG_job build metadata"
        )
    if len(observed) > 1:
        raise ValueError(
            "credential-free Windows gnullvm qualification rejects ambiguous "
            f"TAG_job build metadata: {observed!r}"
        )
    return observed


def validate_keyless_windows_gnullvm_final_args(
    args: Sequence[str], env: Mapping[str, str]
) -> None:
    """Extend Q0.21 with exact identity, lane, and selection contracts."""

    _validate_q021(args, env)
    command_idx, separator_idx, options = _option_args(args)
    command = args[command_idx]
    targets = tuple(args[separator_idx + 1 :])
    configs = tuple(
        option.removeprefix("--config=")
        for option in options
        if option.startswith("--config=")
    )
    test_tag_filters = [
        option for option in options if option.startswith("--test_tag_filters=")
    ]
    job_metadata = _job_metadata(options)
    release_job = job_metadata == [RELEASE_JOB_METADATA]
    clippy_job = job_metadata == [CLIPPY_JOB_METADATA]

    _reject_selection_overrides(options)

    if command == "test":
        if job_metadata:
            raise ValueError(
                "credential-free Windows gnullvm test qualification rejects "
                f"build-lane metadata: {job_metadata!r}"
            )
        if configs != ("ci-windows",):
            raise ValueError(
                "credential-free Windows gnullvm test qualification requires "
                f"exact configs ('ci-windows',); observed {configs!r}"
            )
        if test_tag_filters != [CANONICAL_TEST_TAG_FILTER]:
            raise ValueError(
                "credential-free Windows gnullvm test qualification requires "
                f"exactly {CANONICAL_TEST_TAG_FILTER!r}; "
                f"observed {test_tag_filters!r}"
            )
        _require_exact_flag_family(
            options,
            CANONICAL_SKIP_INCOMPATIBLE,
            SKIP_INCOMPATIBLE_FLAG_FAMILY,
            owner="test qualification",
        )
        _require_exact_flag_family(
            options,
            CANONICAL_TEST_VERBOSE_TIMEOUTS,
            TEST_VERBOSE_TIMEOUT_FLAG_FAMILY,
            owner="test qualification",
        )
        negative_targets = [target for target in targets if target.startswith("-")]
        if negative_targets:
            raise ValueError(
                "credential-free Windows gnullvm test qualification rejects "
                f"negative targets: {negative_targets!r}"
            )
        return

    if command != "build":
        raise ValueError(
            "credential-free Windows gnullvm qualification permits only build/test"
        )

    if not (release_job or clippy_job):
        raise ValueError(
            "credential-free Windows gnullvm build qualification requires exactly "
            "one recognized lane metadata tag"
        )
    if test_tag_filters:
        raise ValueError(
            "credential-free Windows gnullvm build qualification rejects "
            f"test-tag filters: {test_tag_filters!r}"
        )
    _reject_flag_family(
        options,
        TEST_VERBOSE_TIMEOUT_FLAG_FAMILY,
        owner="build qualification",
    )

    if release_job:
        if configs != ("ci-windows",):
            raise ValueError(
                "credential-free Windows gnullvm release qualification has "
                f"non-canonical configs: {configs!r}"
            )
        _reject_flag_family(
            options,
            SKIP_INCOMPATIBLE_FLAG_FAMILY,
            owner="release qualification",
        )
        if targets != CANONICAL_RELEASE_TARGETS:
            raise ValueError(
                "credential-free Windows gnullvm release qualification requires "
                f"the exact canonical target set; observed {targets!r}"
            )
        return

    if configs != ("clippy", "ci-windows"):
        raise ValueError(
            "credential-free Windows gnullvm clippy qualification requires "
            "exact configs ('clippy', 'ci-windows'); "
            f"observed {configs!r}"
        )
    _require_exact_flag_family(
        options,
        CANONICAL_SKIP_INCOMPATIBLE,
        SKIP_INCOMPATIBLE_FLAG_FAMILY,
        owner="clippy qualification",
    )
    invalid_negative_targets = [
        target
        for target in targets
        if target.startswith("-") and target != CANONICAL_CLIPPY_NEGATIVE_TARGET
    ]
    if invalid_negative_targets:
        raise ValueError(
            "credential-free Windows gnullvm clippy qualification rejects negative targets "
            "outside the canonical V8 exclusion: "
            f"{invalid_negative_targets!r}"
        )
