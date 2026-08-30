"""Q0.22 fail-closed Bazel negative-target policy."""

from collections.abc import Mapping, Sequence

from run_bazel_q017_policy import *  # noqa: F403
from run_bazel_q017_policy import _option_args
from run_bazel_q017_policy import (
    validate_keyless_windows_gnullvm_final_args as _validate_q021,
)

RELEASE_JOB_METADATA = "--build_metadata=TAG_job=verify-release-build"
CANONICAL_RELEASE_TARGETS = (
    "//codex-rs/...",
    "-//codex-rs/core/tests/remote_env_windows:smoke-test",
    "-//codex-rs/v8-poc:all",
)


def validate_keyless_windows_gnullvm_final_args(
    args: Sequence[str], env: Mapping[str, str]
) -> None:
    """Extend Q0.21 with an exact release-target and exclusion ratchet."""

    _validate_q021(args, env)
    command_idx, separator_idx, options = _option_args(args)
    targets = tuple(args[separator_idx + 1 :])
    release_job = RELEASE_JOB_METADATA in options

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
