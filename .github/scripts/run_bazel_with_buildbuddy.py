#!/usr/bin/env python3

"""Compatibility wrapper plus Q0.17 qualification ratchet."""

import os
import subprocess
import sys
from collections.abc import Mapping

import run_bazel_with_buildbuddy_base as _base
from run_bazel_with_buildbuddy_base import *  # noqa: F403
from run_bazel_q017_policy import QUALIFICATION_BAZELRC_GIT_BLOB_SHA1
from run_bazel_q017_policy import _git_blob_sha1
from run_bazel_q017_policy import _insert_before_separator
from run_bazel_q017_policy import _is_keyless_windows_gnullvm
from run_bazel_q017_policy import _qualification_workspace_bazelrc
from run_bazel_q017_policy import validate_keyless_windows_gnullvm_final_args


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
        (index for index, arg in enumerate(command[1:], start=1) if not arg.startswith("-")),
        len(command),
    )
    startup = command[1:command_idx]
    from run_bazel_q017_policy import _has_rc_control

    if any(_has_rc_control(arg) for arg in startup):
        raise ValueError(
            "credential-free Windows gnullvm qualification rejects caller rc controls"
        )
    strict_rc = [
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
        host = "OpenAI tenant" if uses_openai_host(os.environ) else "generic"  # noqa: F405
        print(f"Using {host} BuildBuddy configuration: {config}.", file=sys.stderr)

    try:
        command = bazel_command(*sys.argv[1:])
    except ValueError as error:
        print(
            f"Bazel qualification boundary rejected invocation: {error}",
            file=sys.stderr,
        )
        raise SystemExit(2) from error
    if os.name == "nt":
        result = subprocess.run(command, check=False)
        raise SystemExit(result.returncode)
    os.execvp(command[0], command)


if __name__ == "__main__":
    main()
