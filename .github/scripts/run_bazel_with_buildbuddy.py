#!/usr/bin/env python3

"""Compatibility wrapper plus Q0.17-Q0.34 qualification ratchets."""

import os
import subprocess
import sys
from collections.abc import Mapping

import run_bazel_with_buildbuddy_base as _base
from run_bazel_q017_policy import QUALIFICATION_BAZELRC_GIT_BLOB_SHA1
from run_bazel_q017_policy import _has_rc_control
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
from run_bazel_q029_job_executable import (
    prepare_bazelisk_environment as _prepare_q029_compatibility_base,
)
from run_bazel_q030_direct_bazel import prepare_bazelisk_environment
from run_bazel_q030_direct_bazel import resolve_verified_bazel_command
from run_bazel_q030_direct_bazel import (
    validate_keyless_windows_gnullvm_command,
)
from run_bazel_q034_workspace_targets import (
    validate_keyless_windows_gnullvm_workspace_and_targets,
)
from run_bazel_with_buildbuddy_base import *  # noqa: F403

# Keep selected compatibility layers machine-visible while Q0.34 composes
# Q0.29/Q0.30 internally and adds canonical workspace/target authority.
assert _validate_q026_compatibility_base is not None
assert _validate_q027_compatibility_base is not None
assert _prepare_q029_compatibility_base is not None


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


def main() -> None:
    config = remote_config(sys.argv[1:], os.environ)  # noqa: F405
    if config is None:
        print(
            "BuildBuddy key unavailable; using local Bazel configuration.",
            file=sys.stderr,
        )
    else:
        host = (
            "OpenAI tenant"
            if uses_openai_host(os.environ)  # noqa: F405
            else "generic"
        )
        print(f"Using {host} BuildBuddy configuration: {config}.", file=sys.stderr)

    launch_cwd = None
    try:
        command = bazel_command(*sys.argv[1:])
        if _is_keyless_windows_gnullvm(command[1:], os.environ):
            prepare_bazelisk_environment(os.environ)
            command = resolve_verified_bazel_command(command, os.environ)
            validate_keyless_windows_gnullvm_command(command, os.environ)
            launch_cwd = validate_keyless_windows_gnullvm_workspace_and_targets(
                command,
                os.environ,
            )
    except ValueError as error:
        print(
            f"Bazel qualification boundary rejected invocation: {error}",
            file=sys.stderr,
        )
        raise SystemExit(2) from error
    if os.name == "nt":
        if launch_cwd is None:
            result = subprocess.run(command, check=False)
        else:
            result = subprocess.run(
                command,
                cwd=launch_cwd,
                env=os.environ,
                check=False,
            )
        raise SystemExit(result.returncode)
    os.execvpe(command[0], command, os.environ)


if __name__ == "__main__":
    main()
