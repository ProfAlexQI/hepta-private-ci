"""Q0.27 fail-closed startup and final-option authority policy."""

from __future__ import annotations

from collections.abc import Mapping, Sequence
from pathlib import Path

from run_bazel_q017_policy import CI_EXACT_OPTIONS
from run_bazel_q017_policy import _option_args
from run_bazel_q022_negative_targets import CANONICAL_SKIP_INCOMPATIBLE
from run_bazel_q022_negative_targets import CANONICAL_TEST_TAG_FILTER
from run_bazel_q022_negative_targets import CANONICAL_TEST_VERBOSE_TIMEOUTS
from run_bazel_q022_negative_targets import CLIPPY_JOB_METADATA
from run_bazel_q022_negative_targets import RELEASE_JOB_METADATA
from run_bazel_q022_negative_targets import (
    validate_keyless_windows_gnullvm_final_args as _validate_q026,
)

CANONICAL_ANNOUNCE_RC = "--announce_rc"
ANNOUNCE_RC_FLAG_FAMILY = (
    CANONICAL_ANNOUNCE_RC,
    "--noannounce_rc",
)
CANONICAL_REMOTE_DOWNLOAD_TOPLEVEL = "--remote_download_toplevel"
REMOTE_DOWNLOAD_TOPLEVEL_FLAG_FAMILY = (
    CANONICAL_REMOTE_DOWNLOAD_TOPLEVEL,
    "--noremote_download_toplevel",
)
CANONICAL_RELEASE_COMPILATION_MODE = "--compilation_mode=fastbuild"
CANONICAL_RELEASE_RUSTC_FLAG = (
    "--@rules_rust//rust/settings:extra_rustc_flag=-Cdebug-assertions=no"
)
CANONICAL_RELEASE_EXEC_RUSTC_FLAG = (
    "--@rules_rust//rust/settings:extra_exec_rustc_flag=-Cdebug-assertions=no"
)
CANONICAL_RELEASE_DEBUG_METADATA = "--build_metadata=TAG_rust_debug_assertions=off"
COMMIT_METADATA_PREFIX = "--build_metadata=COMMIT_SHA="
SHARD_METADATA_PREFIX = "--build_metadata=TAG_windows_test_shard="

STRICT_STARTUP_FLAGS = (
    "--nomaster_bazelrc",
    "--nosystem_rc",
    "--noworkspace_rc",
    "--nohome_rc",
)
DISABLED_REPO_CONTENTS_CACHE = "--noexperimental_remote_repo_contents_cache"
OUTPUT_USER_ROOT_PREFIX = "--output_user_root="

# These prefixes are accepted only because Q0.17 has already validated their
# exact value, multiplicity, environment binding, and configured-root scope.
Q017_VALIDATED_DYNAMIC_PREFIXES = (
    "--action_env=",
    "--host_action_env=",
    "--test_env=",
    "--repo_contents_cache=",
    "--repository_cache=",
    "--execution_log_compact_file=",
)


def _matches_boolean_family(option: str, family: Sequence[str]) -> bool:
    return any(option == name or option.startswith(f"{name}=") for name in family)


def _require_exact_boolean_family(
    options: Sequence[str],
    expected: str,
    family: Sequence[str],
    *,
    owner: str,
) -> None:
    observed = [option for option in options if _matches_boolean_family(option, family)]
    if observed != [expected]:
        raise ValueError(
            f"credential-free Windows gnullvm {owner} requires exactly "
            f"{expected!r}; observed {observed!r}"
        )


def _reject_or_require_single_positive_boolean(
    options: Sequence[str],
    expected: str,
    family: Sequence[str],
    *,
    owner: str,
) -> None:
    observed = [option for option in options if _matches_boolean_family(option, family)]
    if observed and observed != [expected]:
        raise ValueError(
            f"credential-free Windows gnullvm {owner} permits only one "
            f"{expected!r}; observed {observed!r}"
        )


def _canonical_workspace(env: Mapping[str, str]) -> Path:
    workspace_value = env.get("GITHUB_WORKSPACE")
    if not workspace_value:
        raise ValueError(
            "credential-free Windows gnullvm qualification requires GITHUB_WORKSPACE"
        )
    workspace = Path(workspace_value)
    if not workspace.is_absolute():
        raise ValueError("GITHUB_WORKSPACE must be absolute")
    try:
        resolved = workspace.resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve GITHUB_WORKSPACE: {error}") from error
    if not resolved.is_dir():
        raise ValueError("GITHUB_WORKSPACE must resolve to a directory")
    return resolved


def _validate_exact_startup(
    startup: Sequence[str], env: Mapping[str, str]
) -> None:
    # run_bazel_with_buildbuddy.py has already verified the exact Git blob of
    # this file before it installs the strict rc vector. Q0.27 binds the final
    # startup argv so no later JVM, rc, policy, or repository-cache control can
    # coexist with that verified input.
    bazelrc = _canonical_workspace(env) / ".bazelrc"
    expected: list[str] = []
    output_root = env.get("BAZEL_OUTPUT_USER_ROOT")
    if output_root:
        expected.append(f"{OUTPUT_USER_ROOT_PREFIX}{output_root}")
    expected.extend(
        (
            DISABLED_REPO_CONTENTS_CACHE,
            *STRICT_STARTUP_FLAGS,
            f"--bazelrc={bazelrc}",
        )
    )
    if list(startup) != expected:
        raise ValueError(
            "credential-free Windows gnullvm qualification requires the exact "
            "startup vector; "
            f"expected {expected!r}, observed {list(startup)!r}"
        )


def _is_lowercase_hex(value: str, lengths: set[int]) -> bool:
    return len(value) in lengths and all(
        character in "0123456789abcdef" for character in value
    )


def _validate_metadata(
    options: Sequence[str],
    env: Mapping[str, str],
    *,
    command: str,
    release_job: bool,
    clippy_job: bool,
) -> set[str]:
    metadata = [option for option in options if option.startswith("--build_metadata=")]
    allowed = {CI_EXACT_OPTIONS["--build_metadata=TAG_windows_gnullvm_local="]}

    commit_metadata = [
        option for option in metadata if option.startswith(COMMIT_METADATA_PREFIX)
    ]
    expected_sha = env.get("GITHUB_SHA")
    if expected_sha:
        expected = f"{COMMIT_METADATA_PREFIX}{expected_sha}"
        if commit_metadata != [expected]:
            raise ValueError(
                "credential-free Windows gnullvm qualification requires exact "
                f"commit metadata {expected!r}; observed {commit_metadata!r}"
            )
        allowed.add(expected)
    elif commit_metadata:
        if len(commit_metadata) != 1:
            raise ValueError(
                "credential-free Windows gnullvm qualification rejects "
                "duplicate commit metadata"
            )
        value = commit_metadata[0][len(COMMIT_METADATA_PREFIX) :]
        if not _is_lowercase_hex(value, {40, 64}):
            raise ValueError(
                "credential-free Windows gnullvm qualification rejects "
                "malformed commit metadata"
            )
        allowed.add(commit_metadata[0])

    shard_metadata = [
        option for option in metadata if option.startswith(SHARD_METADATA_PREFIX)
    ]
    expected_shard = env.get("BAZEL_TEST_SHARD")
    if expected_shard:
        if command != "test" or expected_shard not in {"1", "2", "3", "4"}:
            raise ValueError(
                "credential-free Windows gnullvm qualification has invalid "
                "shard environment"
            )
        if env.get("BAZEL_TEST_SHARD_COUNT") not in {None, "4"}:
            raise ValueError(
                "credential-free Windows gnullvm qualification requires four "
                "test shards"
            )
        expected = f"{SHARD_METADATA_PREFIX}{expected_shard}"
        if shard_metadata != [expected]:
            raise ValueError(
                "credential-free Windows gnullvm qualification requires exact "
                f"shard metadata {expected!r}; observed {shard_metadata!r}"
            )
        allowed.add(expected)
    elif shard_metadata:
        raise ValueError(
            "credential-free Windows gnullvm qualification rejects unexpected "
            "shard metadata"
        )

    if release_job:
        debug_metadata = [
            option for option in metadata if option == CANONICAL_RELEASE_DEBUG_METADATA
        ]
        if debug_metadata != [CANONICAL_RELEASE_DEBUG_METADATA]:
            raise ValueError(
                "credential-free Windows gnullvm release qualification requires "
                "exactly one debug-assertions metadata binding"
            )
        allowed.update((RELEASE_JOB_METADATA, CANONICAL_RELEASE_DEBUG_METADATA))
    elif clippy_job:
        allowed.add(CLIPPY_JOB_METADATA)
    else:
        if command != "test":
            raise ValueError(
                "credential-free Windows gnullvm qualification has no recognized lane"
            )

    unexpected = [option for option in metadata if option not in allowed]
    if unexpected:
        raise ValueError(
            "credential-free Windows gnullvm qualification rejects unrecognized "
            f"build metadata: {unexpected!r}"
        )
    return allowed


def _allowed_exact_options(
    *, command: str, release_job: bool, clippy_job: bool
) -> set[str]:
    allowed = set(CI_EXACT_OPTIONS.values())
    allowed.add(CANONICAL_ANNOUNCE_RC)
    if command == "test":
        allowed.update(
            (
                "--config=ci-windows",
                CANONICAL_TEST_TAG_FILTER,
                CANONICAL_SKIP_INCOMPATIBLE,
                CANONICAL_TEST_VERBOSE_TIMEOUTS,
                CANONICAL_REMOTE_DOWNLOAD_TOPLEVEL,
            )
        )
    elif clippy_job:
        allowed.update(
            ("--config=clippy", "--config=ci-windows", CANONICAL_SKIP_INCOMPATIBLE)
        )
    elif release_job:
        allowed.update(
            (
                "--config=ci-windows",
                CANONICAL_RELEASE_COMPILATION_MODE,
                CANONICAL_RELEASE_RUSTC_FLAG,
                CANONICAL_RELEASE_EXEC_RUSTC_FLAG,
                CANONICAL_RELEASE_DEBUG_METADATA,
            )
        )
    return allowed


def _is_allowed_dynamic(option: str) -> bool:
    return option.startswith(Q017_VALIDATED_DYNAMIC_PREFIXES)


def _validate_closed_world_options(
    options: Sequence[str], env: Mapping[str, str], *, command: str
) -> None:
    job_metadata = [
        option
        for option in options
        if option.startswith("--build_metadata=TAG_job=")
    ]
    release_job = job_metadata == [RELEASE_JOB_METADATA]
    clippy_job = job_metadata == [CLIPPY_JOB_METADATA]

    _require_exact_boolean_family(
        options,
        CANONICAL_ANNOUNCE_RC,
        ANNOUNCE_RC_FLAG_FAMILY,
        owner="final command",
    )
    _reject_or_require_single_positive_boolean(
        options,
        CANONICAL_REMOTE_DOWNLOAD_TOPLEVEL,
        REMOTE_DOWNLOAD_TOPLEVEL_FLAG_FAMILY,
        owner="download posture",
    )
    metadata = _validate_metadata(
        options,
        env,
        command=command,
        release_job=release_job,
        clippy_job=clippy_job,
    )
    allowed = _allowed_exact_options(
        command=command,
        release_job=release_job,
        clippy_job=clippy_job,
    )
    allowed.update(metadata)

    unexpected = [
        option
        for option in options
        if option not in allowed and not _is_allowed_dynamic(option)
    ]
    if unexpected:
        raise ValueError(
            "qualification rejects unrecognized final Bazel options: "
            f"{unexpected!r}"
        )


def validate_keyless_windows_gnullvm_final_args(
    args: Sequence[str], env: Mapping[str, str]
) -> None:
    """Compose Q0.26 with exact startup and closed-world final options."""

    _validate_q026(args, env)
    command_idx, _separator_idx, options = _option_args(args)
    _validate_exact_startup(args[:command_idx], env)
    _validate_closed_world_options(options, env, command=args[command_idx])
