"""Immutable job, option, and target lane validation for Q0.27."""

from __future__ import annotations

from collections.abc import Mapping, Sequence
from pathlib import Path

from run_bazel_q027_bazelisk import _validate_bazelisk_configuration
from run_bazel_q027_common import BAZEL_WINDOWS_X86_64_SHA256
from run_bazel_q027_common import CI_EXACT_OPTIONS
from run_bazel_q027_common import CLIPPY_JOB
from run_bazel_q027_common import CLIPPY_JOB_METADATA
from run_bazel_q027_common import CLIPPY_OPTIONS
from run_bazel_q027_common import CANONICAL_CLIPPY_NEGATIVE_TARGET
from run_bazel_q027_common import CANONICAL_RELEASE_TARGETS
from run_bazel_q027_common import RELEASE_JOB
from run_bazel_q027_common import RELEASE_JOB_METADATA
from run_bazel_q027_common import RELEASE_OPTIONS
from run_bazel_q027_common import TEST_JOB
from run_bazel_q027_common import TEST_OPTIONS
from run_bazel_q027_common import _DYNAMIC_OPTION_PREFIXES
from run_bazel_q027_common import _WINDOWS_INHERITED_ENV_NAMES
from run_bazel_q027_common import _require_env
from run_bazel_q027_common import _sha256_file
from run_bazel_q027_common import _validate_runner_identity
from run_bazel_q027_paths import _split_command
from run_bazel_q027_paths import _validate_runner_paths
from run_bazel_q027_paths import _validate_startup

def _expected_lane(
    job: str, env: Mapping[str, str]
) -> tuple[str, tuple[str, ...], set[str], set[str]]:
    sha = _require_env(env, "GITHUB_SHA")
    common_metadata = {
        f"--build_metadata=COMMIT_SHA={sha}",
        "--build_metadata=TAG_windows_gnullvm_local=true",
    }
    if job == TEST_JOB:
        shard = _require_env(env, "BAZEL_TEST_SHARD")
        if shard not in {"1", "2", "3", "4"}:
            raise ValueError("BAZEL_TEST_SHARD must be one of 1, 2, 3, or 4")
        if env.get("BAZEL_TEST_SHARD_COUNT") != "4":
            raise ValueError("BAZEL_TEST_SHARD_COUNT must equal 4")
        return (
            "test",
            ("ci-windows",),
            common_metadata
            | {f"--build_metadata=TAG_windows_test_shard={shard}"},
            TEST_OPTIONS,
        )
    if job == CLIPPY_JOB:
        return (
            "build",
            ("clippy", "ci-windows"),
            common_metadata | {CLIPPY_JOB_METADATA},
            CLIPPY_OPTIONS,
        )
    return (
        "build",
        ("ci-windows",),
        common_metadata
        | {
            RELEASE_JOB_METADATA,
            "--build_metadata=TAG_rust_debug_assertions=off",
        },
        RELEASE_OPTIONS,
    )


def _validate_job_options(
    command_name: str,
    options: Sequence[str],
    env: Mapping[str, str],
    job: str,
) -> None:
    expected_command, expected_configs, metadata, lane_options = _expected_lane(
        job, env
    )
    if command_name != expected_command:
        raise ValueError(
            f"job {job} requires Bazel command {expected_command!r}, "
            f"observed {command_name!r}"
        )
    configs = tuple(
        option.removeprefix("--config=")
        for option in options
        if option.startswith("--config=")
    )
    if configs != expected_configs:
        raise ValueError(
            f"job {job} requires exact configs {expected_configs!r}, "
            f"observed {configs!r}"
        )
    observed_metadata = [
        option for option in options if option.startswith("--build_metadata=")
    ]
    if (
        len(observed_metadata) != len(set(observed_metadata))
        or set(observed_metadata) != metadata
    ):
        raise ValueError(
            f"job {job} requires exact build metadata {sorted(metadata)!r}; "
            f"observed {observed_metadata!r}"
        )

    allowed = set(CI_EXACT_OPTIONS.values()) | lane_options | metadata | {"--announce_rc"}
    for option in options:
        if option.startswith("--config=") or option in allowed:
            continue
        if option.startswith(_DYNAMIC_OPTION_PREFIXES):
            continue
        raise ValueError(f"job {job} rejects unclassified Bazel option {option!r}")

    for required in lane_options:
        if options.count(required) != 1:
            raise ValueError(f"job {job} requires exactly one {required!r}")

    # The Q0.17 policy validates values and symmetry; this layer additionally
    # rejects inherited environment switches outside the reviewed Windows SDK set.
    for prefix in ("--action_env=", "--host_action_env="):
        for option in options:
            if not option.startswith(prefix):
                continue
            payload = option.removeprefix(prefix)
            name = payload.partition("=")[0]
            if name != "PATH" and name not in _WINDOWS_INHERITED_ENV_NAMES:
                raise ValueError(f"job {job} rejects environment binding {option!r}")


def _validate_targets(targets: Sequence[str], job: str) -> None:
    if not targets:
        raise ValueError(f"job {job} requires at least one Bazel target")
    if len(targets) != len(set(targets)):
        raise ValueError(f"job {job} rejects duplicate Bazel targets")
    if job == RELEASE_JOB:
        if tuple(targets) != CANONICAL_RELEASE_TARGETS:
            raise ValueError(
                "release job requires the exact canonical release target payload"
            )
        return
    if job == TEST_JOB:
        invalid = [
            target
            for target in targets
            if target.startswith("-") or not target.startswith("//")
        ]
        if invalid:
            raise ValueError(f"test shard rejects non-positive targets: {invalid!r}")
        return

    if len(targets) < 2 or tuple(targets[:2]) != (
        "//codex-rs/...",
        CANONICAL_CLIPPY_NEGATIVE_TARGET,
    ):
        raise ValueError(
            "clippy requires the canonical //codex-rs/... plus V8 exclusion prefix"
        )
    for target in targets[2:]:
        if target.startswith("-"):
            raise ValueError(f"clippy rejects additional negative target {target!r}")
        if not target.startswith("//codex-rs/"):
            raise ValueError(f"clippy rejects target outside //codex-rs: {target!r}")
        if "/v8-poc:" in target:
            raise ValueError(f"clippy rejects direct v8-poc target {target!r}")


def validate_keyless_windows_gnullvm_command(
    command: Sequence[str], env: Mapping[str, str]
) -> None:
    """Validate the verified direct-Bazel command immediately before launch."""
    _validate_bazelisk_configuration(env)
    job = _validate_runner_identity(env)
    startup, command_name, options, targets = _split_command(command)
    executable = Path(command[0])
    if not executable.is_absolute() or executable.name.casefold() != "bazel.exe":
        raise ValueError("verified direct Bazel executable path is not canonical")
    if _sha256_file(executable) != BAZEL_WINDOWS_X86_64_SHA256:
        raise ValueError("verified direct Bazel executable changed before launch")
    _validate_startup(startup, env)
    _validate_runner_paths(options, env, job)
    _validate_job_options(command_name, options, env, job)
    _validate_targets(targets, job)
