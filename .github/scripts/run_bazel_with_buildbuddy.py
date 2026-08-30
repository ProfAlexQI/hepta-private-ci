#!/usr/bin/env python3

"""Compatibility wrapper plus Q0.17-Q0.31 qualification ratchets."""

import os
import subprocess
import sys
from collections.abc import Mapping, MutableMapping, Sequence

import run_bazel_with_buildbuddy_base as _base
from run_bazel_with_buildbuddy_base import *  # noqa: F403
from run_bazel_q017_policy import QUALIFICATION_BAZELRC_GIT_BLOB_SHA1
from run_bazel_q017_policy import _git_blob_sha1
from run_bazel_q017_policy import _insert_before_separator
from run_bazel_q017_policy import _is_keyless_windows_gnullvm
from run_bazel_q017_policy import _qualification_workspace_bazelrc
from run_bazel_q022_negative_targets import (
    validate_keyless_windows_gnullvm_final_args as _validate_q026_compatibility_base,
)
from run_bazel_q027_lane_semantics import (
    validate_keyless_windows_gnullvm_final_args as _validate_q027_compatibility_base,
)
from run_bazel_q028_startup_contract import (
    validate_keyless_windows_gnullvm_final_args,
)
from run_bazel_q029_execution_context import (
    bind_verified_bazelisk as _q029_bind_verified_bazelisk,
)
from run_bazel_q029_execution_context import (
    prepare_bazelisk_environment as _q029_prepare_bazelisk_environment,
)
from run_bazel_q029_execution_context import (
    validate_keyless_windows_gnullvm_execution_context as _q029_validate_execution_context,
)
from run_bazel_q031_direct_bazel import prepare_bazelisk_environment
from run_bazel_q031_direct_bazel import resolve_verified_bazel_command
from run_bazel_q031_direct_bazel import validate_keyless_windows_gnullvm_command

# Keep every selected compatibility layer machine-visible. Q0.31 replaces only
# the final Bazelisk-mediated launch with a directly rehashed CAS Bazel launch.
assert _validate_q026_compatibility_base is not None
assert _validate_q027_compatibility_base is not None
assert _q029_bind_verified_bazelisk is not None
assert _q029_prepare_bazelisk_environment is not None
assert _q029_validate_execution_context is not None


def bazel_command(*args: str, env: Mapping[str, str] | None = None) -> list[str]:
    env = os.environ if env is None else env
    command = _base.bazel_command(*args, env=env)
    if not _is_keyless_windows_gnullvm(command[1:], env):
        return command

    bazelrc = _qualification_workspace_bazelrc(
        env,
        expected_blob=QUALIFICATION_BAZELRC_GIT_BLOB_SHA1,
    )
    command_idx = next(
        (
            index
            for index, arg in enumerate(command[1:], start=1)
            if not arg.startswith("-")
        ),
        len(command),
    )
    startup = command[1:command_idx]
    from run_bazel_q017_policy import _has_rc_control

    if any(_has_rc_control(arg) for arg in startup):
        raise ValueError(
            "credential-free Windows gnullvm qualification rejects caller rc controls"
        )

    output_base = env.get("BAZEL_OUTPUT_BASE")
    if not output_base:
        raise ValueError(
            "credential-free Windows gnullvm qualification requires BAZEL_OUTPUT_BASE"
        )
    strict_rc = [
        f"--output_base={output_base}",
        "--nomaster_bazelrc",
        "--nosystem_rc",
        "--noworkspace_rc",
        "--nohome_rc",
        f"--bazelrc={bazelrc}",
    ]
    command = [command[0], *startup, *strict_rc, *command[command_idx:]]
    if "--announce_rc" not in command[command_idx + len(strict_rc) + 1 :]:
        command = _insert_before_separator(command, "--announce_rc")
    validate_keyless_windows_gnullvm_final_args(command[1:], env)
    return command


def bind_verified_bazelisk(
    command: Sequence[str],
    env: MutableMapping[str, str],
) -> list[str]:
    """Compatibility name retained for the Q0.29 wrapper-order regression."""

    return resolve_verified_bazel_command(command, env)


def validate_keyless_windows_gnullvm_execution_context(
    command: Sequence[str],
    env: Mapping[str, str],
) -> None:
    """Compatibility name retained while enforcing the stronger Q0.31 contract."""

    validate_keyless_windows_gnullvm_command(command, env)


def executable_command(
    *args: str,
    env: MutableMapping[str, str] | None = None,
) -> list[str]:
    """Return the command that is safe to launch for the selected CI lane."""

    env = os.environ if env is None else env
    command = bazel_command(*args, env=env)
    if not _is_keyless_windows_gnullvm(command[1:], env):
        return command

    # Q0.28 has already bound startup and lane semantics. Q0.29 binds the
    # runner/job/path inputs; Q0.31 resolves and rehashes the actual cached
    # Bazel executable immediately before process launch.
    prepare_bazelisk_environment(env)
    command = bind_verified_bazelisk(command, env)
    validate_keyless_windows_gnullvm_execution_context(command, env)
    return command


def main() -> None:
    config = remote_config(sys.argv[1:], os.environ)  # noqa: F405
    if config is None:
        print(
            "BuildBuddy key unavailable; using local Bazel configuration.",
            file=sys.stderr,
        )
    else:
        host = (
            "OpenAI tenant" if uses_openai_host(os.environ) else "generic"  # noqa: F405
        )
        print(f"Using {host} BuildBuddy configuration: {config}.", file=sys.stderr)

    try:
        command = executable_command(*sys.argv[1:])
    except ValueError as error:
        print(
            f"Bazel qualification boundary rejected invocation: {error}",
            file=sys.stderr,
        )
        raise SystemExit(2) from error
    if os.name == "nt":
        result = subprocess.run(command, check=False)
        raise SystemExit(result.returncode)
    os.execvpe(command[0], command, os.environ)


if __name__ == "__main__":
    main()
