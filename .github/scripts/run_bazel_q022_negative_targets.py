"""Q0.22/Q0.23 fail-closed Bazel target and job-metadata policy."""

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
CANONICAL_RELEASE_TARGETS = (
    "//codex-rs/...",
    "-//codex-rs/core/tests/remote_env_windows:smoke-test",
    "-//codex-rs/v8-poc:all",
)


def validate_keyless_windows_gnullvm_final_args(
    args: Sequence[str], env: Mapping[str, str]
) -> None:
    """Extend Q0.21 with exact target and unambiguous job-metadata ratchets."""

    _validate_q021(args, env)
    command_idx, separator_idx, options = _option_args(args)
    targets = tuple(args[separator_idx + 1 :])

    if BUILD_METADATA_OPTION in options:
        raise ValueError(
            "credential-free Windows gnullvm qualification rejects split-form "
            "--build_metadata"
        )

    job_metadata = [
        option for option in options if option.startswith(JOB_METADATA_LIKE_PREFIX)
    ]
    if any(
        not option.startswith(JOB_METADATA_PREFIX)
        or len(option) == len(JOB_METADATA_PREFIX)
        for option in job_metadata
    ):
        raise ValueError(
            "credential-free Windows gnullvm qualification rejects malformed "
            "TAG_job build metadata"
        )
    if len(job_metadata) > 1:
        raise ValueError(
            "credential-free Windows gnullvm qualification rejects ambiguous "
            f"TAG_job build metadata: {job_metadata!r}"
        )

    release_job = job_metadata == [RELEASE_JOB_METADATA]
    if release_job:
        if args[command_idx] != "build" or targets != CANONICAL_RELEASE_TARGETS:
            raise ValueError(
                "credential-free Windows gnullvm release qualification requires "
                f"the exact canonical target set; observed {targets!r}"
            )
        return

    negative_targets = [target for target in targets if target.startswith("-")]
    if negative_targets:
        raise ValueError(
            "credential-free Windows gnullvm qualification rejects negative "
            f"targets outside the release lane: {negative_targets!r}"
        )
