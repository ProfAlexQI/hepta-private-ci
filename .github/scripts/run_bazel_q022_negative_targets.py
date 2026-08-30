"""Q0.22/Q0.23 fail-closed Bazel target, lane, and test-selection policy."""

from collections.abc import Mapping, Sequence

from run_bazel_q017_policy import *  # noqa: F403
from run_bazel_q017_policy import _option_args
from run_bazel_q017_policy import (
    validate_keyless_windows_gnullvm_final_args as _validate_q021,
)

RELEASE_JOB_METADATA = "--build_metadata=TAG_job=verify-release-build"
CLIPPY_JOB_METADATA = "--build_metadata=TAG_job=clippy"
CANONICAL_TEST_TAG_FILTER = "--test_tag_filters=-argument-comment-lint"
CANONICAL_RELEASE_TARGETS = (
    "//codex-rs/...",
    "-//codex-rs/core/tests/remote_env_windows:smoke-test",
    "-//codex-rs/v8-poc:all",
)
CANONICAL_SKIP_INCOMPATIBLE = "--skip_incompatible_explicit_targets"
CANONICAL_TEST_VERBOSE_TIMEOUTS = "--test_verbose_timeout_warnings"

FORBIDDEN_SELECTION_PREFIXES = (
    "--test_filter=",
    "--test_arg=",
    "--test_lang_filters=",
    "--test_size_filters=",
    "--test_timeout_filters=",
    "--build_tag_filters=",
    "--build_tests_only",
)


def _reject_selection_overrides(options: Sequence[str]) -> None:
    for option in options:
        if option.startswith(FORBIDDEN_SELECTION_PREFIXES):
            raise ValueError(
                "credential-free Windows gnullvm qualification rejects "
                f"test-selection override {option!r}"
            )


def _require_exact_presence(
    options: Sequence[str], expected: str, *, owner: str
) -> None:
    observed = [option for option in options if option == expected]
    if observed != [expected]:
        raise ValueError(
            f"credential-free Windows gnullvm {owner} requires exactly "
            f"{expected!r}; observed {observed!r}"
        )


def _reject_present(options: Sequence[str], value: str, *, owner: str) -> None:
    if value in options:
        raise ValueError(
            f"credential-free Windows gnullvm {owner} rejects {value!r}"
        )


def validate_keyless_windows_gnullvm_final_args(
    args: Sequence[str], env: Mapping[str, str]
) -> None:
    """Extend Q0.21 with exact target, lane, and test-selection contracts."""

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
    release_job = RELEASE_JOB_METADATA in options
    clippy_job = CLIPPY_JOB_METADATA in options

    _reject_selection_overrides(options)

    if command == "test":
        if release_job or clippy_job:
            raise ValueError(
                "credential-free Windows gnullvm test qualification rejects "
                "build-lane metadata"
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
        _require_exact_presence(
            options,
            CANONICAL_SKIP_INCOMPATIBLE,
            owner="test qualification",
        )
        _require_exact_presence(
            options,
            CANONICAL_TEST_VERBOSE_TIMEOUTS,
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

    if release_job == clippy_job:
        raise ValueError(
            "credential-free Windows gnullvm build qualification requires exactly "
            "one recognized lane metadata tag"
        )
    if test_tag_filters:
        raise ValueError(
            "credential-free Windows gnullvm build qualification rejects "
            f"test-tag filters: {test_tag_filters!r}"
        )
    _reject_present(
        options,
        CANONICAL_TEST_VERBOSE_TIMEOUTS,
        owner="build qualification",
    )

    if release_job:
        if configs != ("ci-windows",):
            raise ValueError(
                "credential-free Windows gnullvm release qualification has "
                f"non-canonical configs: {configs!r}"
            )
        _reject_present(
            options,
            CANONICAL_SKIP_INCOMPATIBLE,
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
    _require_exact_presence(
        options,
        CANONICAL_SKIP_INCOMPATIBLE,
        owner="clippy qualification",
    )
    negative_targets = [target for target in targets if target.startswith("-")]
    if negative_targets:
        raise ValueError(
            "credential-free Windows gnullvm clippy qualification rejects "
            f"negative targets: {negative_targets!r}"
        )
