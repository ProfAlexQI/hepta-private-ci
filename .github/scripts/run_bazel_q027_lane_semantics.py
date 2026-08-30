"""Q0.27 exact semantic contract for keyless Windows gnullvm lanes."""

from __future__ import annotations

import re
from collections.abc import Mapping, Sequence

from run_bazel_q017_policy import CI_EXACT_OPTIONS
from run_bazel_q017_policy import _option_args
from run_bazel_q022_negative_targets import CANONICAL_RELEASE_TARGETS
from run_bazel_q022_negative_targets import CANONICAL_SKIP_INCOMPATIBLE
from run_bazel_q022_negative_targets import CANONICAL_TEST_TAG_FILTER
from run_bazel_q022_negative_targets import CANONICAL_TEST_VERBOSE_TIMEOUTS
from run_bazel_q022_negative_targets import CLIPPY_JOB_METADATA
from run_bazel_q022_negative_targets import RELEASE_JOB_METADATA
from run_bazel_q022_negative_targets import (
    validate_keyless_windows_gnullvm_final_args as _validate_q026,
)

CANONICAL_ANNOUNCE_RC = "--announce_rc"
CANONICAL_REMOTE_DOWNLOAD_TOPLEVEL = "--remote_download_toplevel"
COMMIT_METADATA_PREFIX = "--build_metadata=COMMIT_SHA="
TEST_SHARD_METADATA_PREFIX = "--build_metadata=TAG_windows_test_shard="
RELEASE_DEBUG_METADATA = "--build_metadata=TAG_rust_debug_assertions=off"
RELEASE_COMPILATION_MODE = "--compilation_mode=fastbuild"
RELEASE_RUSTC_FLAG = (
    "--@rules_rust//rust/settings:extra_rustc_flag=-Cdebug-assertions=no"
)
RELEASE_EXEC_RUSTC_FLAG = (
    "--@rules_rust//rust/settings:extra_exec_rustc_flag=-Cdebug-assertions=no"
)

_COMMON_EXACT_OPTIONS = frozenset(CI_EXACT_OPTIONS.values()) | {
    CANONICAL_ANNOUNCE_RC,
}
_COMMON_DYNAMIC_PREFIXES = (
    "--action_env=",
    "--host_action_env=",
    "--test_env=PATH=",
    "--repo_contents_cache=",
    "--repository_cache=",
    "--execution_log_compact_file=",
)
_SHA256_HEX = re.compile(r"[0-9a-f]{40}\Z")
_TEST_SHARD = re.compile(r"[1-4]\Z")


def _require_exact_occurrence(
    options: Sequence[str], expected: str, *, owner: str
) -> None:
    observed = [option for option in options if option == expected]
    if observed != [expected]:
        raise ValueError(
            f"credential-free Windows gnullvm {owner} requires exactly "
            f"{expected!r}; observed {observed!r}"
        )


def _lane_name(command: str, options: Sequence[str]) -> str:
    if command == "test":
        return "test"
    if CLIPPY_JOB_METADATA in options:
        return "clippy"
    if RELEASE_JOB_METADATA in options:
        return "release"
    raise ValueError(
        "credential-free Windows gnullvm semantic contract requires "
        "a recognized test, Clippy, or release lane"
    )


def _metadata_contract(
    options: Sequence[str], env: Mapping[str, str], lane: str
) -> set[str]:
    observed = [
        option for option in options if option.startswith("--build_metadata=")
    ]
    if len(observed) != len(set(observed)):
        raise ValueError(
            "credential-free Windows gnullvm semantic contract rejects "
            f"duplicate build metadata: {observed!r}"
        )

    expected = {
        CI_EXACT_OPTIONS["--build_metadata=TAG_windows_gnullvm_local="],
    }

    commit_metadata = [
        option for option in observed if option.startswith(COMMIT_METADATA_PREFIX)
    ]
    if len(commit_metadata) > 1:
        raise ValueError(
            "credential-free Windows gnullvm semantic contract rejects "
            f"ambiguous commit metadata: {commit_metadata!r}"
        )
    github_sha = env.get("GITHUB_SHA")
    if commit_metadata:
        observed_sha = commit_metadata[0][len(COMMIT_METADATA_PREFIX) :]
        if not _SHA256_HEX.fullmatch(observed_sha):
            raise ValueError(
                "credential-free Windows gnullvm commit metadata must be "
                "one lowercase 40-hex Git object ID"
            )
        if github_sha and observed_sha != github_sha:
            raise ValueError(
                "credential-free Windows gnullvm commit metadata does not "
                "match GITHUB_SHA"
            )
        expected.add(commit_metadata[0])
    elif github_sha:
        raise ValueError(
            "credential-free Windows gnullvm semantic contract requires "
            "COMMIT_SHA metadata"
        )

    shard_metadata = [
        option
        for option in observed
        if option.startswith(TEST_SHARD_METADATA_PREFIX)
    ]
    if lane == "test":
        if len(shard_metadata) > 1:
            raise ValueError(
                "credential-free Windows gnullvm test qualification rejects "
                f"ambiguous shard metadata: {shard_metadata!r}"
            )
        expected_shard = env.get("BAZEL_TEST_SHARD")
        expected_count = env.get("BAZEL_TEST_SHARD_COUNT")
        if shard_metadata:
            observed_shard = shard_metadata[0][len(TEST_SHARD_METADATA_PREFIX) :]
            if not _TEST_SHARD.fullmatch(observed_shard):
                raise ValueError(
                    "credential-free Windows gnullvm test shard metadata "
                    "must be one of 1, 2, 3, or 4"
                )
            if expected_shard and observed_shard != expected_shard:
                raise ValueError(
                    "credential-free Windows gnullvm test shard metadata "
                    "does not match BAZEL_TEST_SHARD"
                )
            if expected_count and expected_count != "4":
                raise ValueError(
                    "credential-free Windows gnullvm qualification requires "
                    "the reviewed four-shard topology"
                )
            expected.add(shard_metadata[0])
        elif expected_shard:
            raise ValueError(
                "credential-free Windows gnullvm test qualification requires "
                "shard metadata"
            )
    elif shard_metadata:
        raise ValueError(
            "credential-free Windows gnullvm build qualification rejects "
            f"test-shard metadata: {shard_metadata!r}"
        )

    if lane == "clippy":
        expected.add(CLIPPY_JOB_METADATA)
    elif lane == "release":
        expected.update((RELEASE_JOB_METADATA, RELEASE_DEBUG_METADATA))

    if set(observed) != expected:
        raise ValueError(
            "credential-free Windows gnullvm semantic contract has "
            f"non-canonical build metadata: {observed!r}; "
            f"expected {sorted(expected)!r}"
        )
    return expected


def _is_common_dynamic_option(option: str) -> bool:
    return option.startswith(_COMMON_DYNAMIC_PREFIXES)


def _reject_unreviewed_options(
    options: Sequence[str],
    allowed_exact: set[str],
    *,
    lane: str,
) -> None:
    unreviewed = [
        option
        for option in options
        if option not in allowed_exact and not _is_common_dynamic_option(option)
    ]
    if unreviewed:
        raise ValueError(
            "credential-free Windows gnullvm "
            f"{lane} qualification rejects unreviewed explicit options: "
            f"{unreviewed!r}"
        )


def _validate_q027_semantics(
    args: Sequence[str], env: Mapping[str, str]
) -> None:
    command_idx, _, options = _option_args(args)
    command = args[command_idx]
    lane = _lane_name(command, options)

    _require_exact_occurrence(
        options,
        CANONICAL_ANNOUNCE_RC,
        owner=f"{lane} qualification",
    )
    metadata = _metadata_contract(options, env, lane)
    allowed = set(_COMMON_EXACT_OPTIONS) | metadata

    if lane == "test":
        _require_exact_occurrence(
            options,
            CANONICAL_REMOTE_DOWNLOAD_TOPLEVEL,
            owner="test qualification",
        )
        allowed.update(
            (
                "--config=ci-windows",
                CANONICAL_TEST_TAG_FILTER,
                CANONICAL_SKIP_INCOMPATIBLE,
                CANONICAL_TEST_VERBOSE_TIMEOUTS,
                CANONICAL_REMOTE_DOWNLOAD_TOPLEVEL,
            )
        )
    elif lane == "clippy":
        allowed.update(
            (
                "--config=clippy",
                "--config=ci-windows",
                CANONICAL_SKIP_INCOMPATIBLE,
                CLIPPY_JOB_METADATA,
            )
        )
    else:
        for expected in (
            RELEASE_COMPILATION_MODE,
            RELEASE_RUSTC_FLAG,
            RELEASE_EXEC_RUSTC_FLAG,
            RELEASE_DEBUG_METADATA,
        ):
            _require_exact_occurrence(
                options,
                expected,
                owner="release qualification",
            )
        allowed.update(
            (
                "--config=ci-windows",
                RELEASE_JOB_METADATA,
                RELEASE_DEBUG_METADATA,
                RELEASE_COMPILATION_MODE,
                RELEASE_RUSTC_FLAG,
                RELEASE_EXEC_RUSTC_FLAG,
            )
        )

    _reject_unreviewed_options(options, allowed, lane=lane)


def validate_keyless_windows_gnullvm_final_args(
    args: Sequence[str], env: Mapping[str, str]
) -> None:
    """Compose Q0.26 with an exact lane-semantic option grammar."""

    _validate_q026(args, env)
    if env.get("GITHUB_ACTIONS") == "true":
        _validate_q027_semantics(args, env)
