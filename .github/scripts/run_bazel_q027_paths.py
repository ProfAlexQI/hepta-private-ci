"""Final Bazel startup and runner-path validation for Q0.27."""

from __future__ import annotations

import re
from collections.abc import Mapping, Sequence
from pathlib import PureWindowsPath

from run_bazel_q017_policy import _qualification_workspace_bazelrc
from run_bazel_q027_common import _require_env
from run_bazel_q027_common import _validate_environment_roots

def _split_command(
    command: Sequence[str],
) -> tuple[list[str], str, list[str], list[str]]:
    command_idx = next(
        (
            index
            for index, arg in enumerate(command[1:], start=1)
            if not arg.startswith("-")
        ),
        len(command),
    )
    if command_idx == len(command):
        raise ValueError("expected a Bazel command")
    try:
        separator_idx = command.index("--", command_idx + 1)
    except ValueError as error:
        raise ValueError("expected one Bazel target separator") from error
    if "--" in command[separator_idx + 1 :]:
        raise ValueError("multiple Bazel target separators are forbidden")
    return (
        list(command[1:command_idx]),
        command[command_idx],
        list(command[command_idx + 1 : separator_idx]),
        list(command[separator_idx + 1 :]),
    )



def _validate_startup(startup: Sequence[str], env: Mapping[str, str]) -> None:
    bazelrc = _qualification_workspace_bazelrc(env)
    expected = [
        f"--output_user_root={_require_env(env, 'BAZEL_OUTPUT_USER_ROOT')}",
        "--noexperimental_remote_repo_contents_cache",
        f"--output_base={_require_env(env, 'BAZEL_OUTPUT_BASE')}",
        "--nomaster_bazelrc",
        "--nosystem_rc",
        "--noworkspace_rc",
        "--nohome_rc",
        f"--bazelrc={bazelrc}",
    ]
    if list(startup) != expected:
        raise ValueError(
            "keyless Windows gnullvm startup arguments are not exact: "
            f"expected {expected!r}, observed {list(startup)!r}"
        )



def _validate_runner_paths(
    options: Sequence[str], env: Mapping[str, str], job: str
) -> None:
    _validate_environment_roots(env, job)
    for env_name, prefix in (
        ("BAZEL_REPO_CONTENTS_CACHE", "--repo_contents_cache="),
        ("BAZEL_REPOSITORY_CACHE", "--repository_cache="),
    ):
        observed = [option for option in options if option.startswith(prefix)]
        required = [f"{prefix}{env[env_name]}"]
        if observed != required:
            raise ValueError(f"non-canonical {prefix} binding: {observed!r}")

    logs = [
        option
        for option in options
        if option.startswith("--execution_log_compact_file=")
    ]
    if len(logs) != 1:
        raise ValueError("exactly one compact execution log path is required")
    log_value = logs[0].split("=", 1)[1]
    log_path = PureWindowsPath(log_value)
    log_root = PureWindowsPath(env["CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR"])
    try:
        relative = log_path.relative_to(log_root)
    except ValueError as error:
        raise ValueError("compact execution log escaped RUNNER_TEMP") from error
    expected_name = re.compile(
        rf"execution-log-(build|test)-{re.escape(job)}-[0-9]+\.zst"
    )
    if len(relative.parts) != 1 or not expected_name.fullmatch(relative.name):
        raise ValueError(f"non-canonical compact execution log name {relative!s}")


