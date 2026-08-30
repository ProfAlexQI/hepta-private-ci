"""Q0.28 exact startup-vector contract for keyless Windows gnullvm."""

from __future__ import annotations

from collections.abc import Mapping, Sequence
from pathlib import Path

from run_bazel_q017_policy import _command_index
from run_bazel_q027_lane_semantics import (
    validate_keyless_windows_gnullvm_final_args as _validate_q027,
)

DISABLED_REPO_CONTENTS_CACHE = "--noexperimental_remote_repo_contents_cache"
OUTPUT_USER_ROOT_PREFIX = "--output_user_root="
STRICT_STARTUP_FLAGS = (
    "--nomaster_bazelrc",
    "--nosystem_rc",
    "--noworkspace_rc",
    "--nohome_rc",
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
        canonical_workspace = workspace.resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve GITHUB_WORKSPACE: {error}") from error
    if not canonical_workspace.is_dir():
        raise ValueError("GITHUB_WORKSPACE must resolve to a directory")
    return canonical_workspace


def _expected_startup(env: Mapping[str, str]) -> list[str]:
    expected: list[str] = []
    output_user_root = env.get("BAZEL_OUTPUT_USER_ROOT")
    if output_user_root:
        expected.append(f"{OUTPUT_USER_ROOT_PREFIX}{output_user_root}")
    expected.extend(
        (
            DISABLED_REPO_CONTENTS_CACHE,
            *STRICT_STARTUP_FLAGS,
            f"--bazelrc={_canonical_workspace(env) / '.bazelrc'}",
        )
    )
    return expected


def _validate_exact_startup(
    args: Sequence[str], env: Mapping[str, str]
) -> None:
    command_idx = _command_index(args)
    if command_idx == len(args):
        raise ValueError("expected a Bazel command")
    observed = list(args[:command_idx])
    expected = _expected_startup(env)
    if observed != expected:
        raise ValueError(
            "credential-free Windows gnullvm qualification requires the exact "
            f"startup vector; expected {expected!r}, observed {observed!r}"
        )


def validate_keyless_windows_gnullvm_final_args(
    args: Sequence[str], env: Mapping[str, str]
) -> None:
    """Compose Q0.27 lane semantics with one exact startup vector."""

    _validate_q027(args, env)
    if env.get("GITHUB_ACTIONS") == "true":
        _validate_exact_startup(args, env)
