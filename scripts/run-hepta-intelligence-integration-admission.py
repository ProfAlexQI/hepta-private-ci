#!/usr/bin/env python3
from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path
from types import SimpleNamespace
from typing import Mapping

import hepta_q046_git_context as q046_git

VERIFIER = Path(__file__).with_name(
    "verify-hepta-intelligence-integration-admission-api-v2.py"
)
TOKEN_ENV_RE = re.compile(r"^[A-Z_][A-Z0-9_]{0,63}$")
PROXY_NAMES = frozenset(
    {
        "all_proxy",
        "ftp_proxy",
        "http_proxy",
        "https_proxy",
        "no_proxy",
    }
)
CANONICAL_GIT_NAMES = frozenset(
    {
        "GIT_CONFIG_GLOBAL",
        "GIT_CONFIG_NOSYSTEM",
        "GIT_DIR",
        "GIT_INDEX_FILE",
        "GIT_LITERAL_PATHSPECS",
        "GIT_OPTIONAL_LOCKS",
        "GIT_WORK_TREE",
    }
)


class AdmissionRunnerError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AdmissionRunnerError(message)


def token_environment_name(args: list[str]) -> str:
    observed: list[str] = []
    index = 0
    while index < len(args):
        argument = args[index]
        if argument == "--token-env":
            require(index + 1 < len(args), "--token-env requires a value")
            observed.append(args[index + 1])
            index += 2
            continue
        if argument.startswith("--token-env="):
            observed.append(argument.split("=", 1)[1])
        index += 1
    require(len(observed) <= 1, "duplicate --token-env arguments")
    name = observed[0] if observed else "GITHUB_TOKEN"
    require(
        TOKEN_ENV_RE.fullmatch(name) is not None,
        f"invalid token environment variable name: {name!r}",
    )
    require(
        not name.startswith("GIT_"),
        "token environment variable must not control Git",
    )
    require(
        name.casefold() not in PROXY_NAMES,
        "token environment variable must not control a proxy",
    )
    require(
        name
        not in {
            "HOME",
            "LANG",
            "LC_ALL",
            "PATH",
            "PYTHONHASHSEED",
            "GITHUB_ACTIONS",
            "RUNNER_OS",
        },
        "token environment variable collides with the execution boundary",
    )
    return name


def path_fingerprint(path: Path) -> tuple[int, int, int, int, int, int]:
    observed = os.stat(path, follow_symlinks=False)
    return (
        observed.st_dev,
        observed.st_ino,
        observed.st_mode,
        observed.st_size,
        observed.st_mtime_ns,
        observed.st_ctime_ns,
    )


def context_fingerprints(
    context: q046_git.GitContext,
) -> dict[str, tuple[int, int, int, int, int, int]]:
    return {
        "executable": path_fingerprint(context.executable),
        "git_dir": path_fingerprint(context.git_dir),
        "index": path_fingerprint(context.index),
        "root": path_fingerprint(context.root),
    }


def run_bound_git(
    context: q046_git.GitContext,
    *args: str,
) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(
        [*context.prefix, *args],
        cwd=context.root,
        env=dict(context.env),
        check=False,
        capture_output=True,
        text=True,
    )
    require(
        completed.returncode == 0,
        "bound Git command failed "
        f"({completed.returncode}): {' '.join(args)}: "
        f"{completed.stderr.strip()}",
    )
    require(
        completed.stderr == "",
        f"bound Git command wrote stderr: {' '.join(args)}",
    )
    return completed


def one_line(value: str, owner: str) -> str:
    lines = value.splitlines()
    require(
        len(lines) == 1 and bool(lines[0]),
        f"{owner} did not return one nonempty line",
    )
    require("\x00" not in lines[0], f"{owner} returned a NUL byte")
    return lines[0]


def tracked_status(context: q046_git.GitContext) -> str:
    return run_bound_git(
        context,
        "status",
        "--porcelain=v1",
        "--untracked-files=no",
    ).stdout


def child_environment(
    context: q046_git.GitContext,
    *,
    token_name: str,
    source: Mapping[str, str],
) -> dict[str, str]:
    token = source.get(token_name, "")
    require(bool(token), f"token environment variable is empty: {token_name}")

    child = {
        key: value
        for key, value in context.env.items()
        if not key.upper().startswith("GIT_")
        and key.casefold() not in PROXY_NAMES
    }
    child.update(
        {
            "PATH": str(context.executable.parent),
            "LANG": "C",
            "LC_ALL": "C",
            "PYTHONHASHSEED": "0",
            "GIT_CONFIG_NOSYSTEM": "1",
            "GIT_CONFIG_GLOBAL": os.devnull,
            "GIT_OPTIONAL_LOCKS": "0",
            "GIT_DIR": str(context.git_dir),
            "GIT_WORK_TREE": str(context.root),
            "GIT_INDEX_FILE": str(context.index),
            "GIT_LITERAL_PATHSPECS": "1",
            token_name: token,
        }
    )
    surviving = sorted(
        key
        for key in child
        if key.upper().startswith("GIT_")
        and key not in CANONICAL_GIT_NAMES
    )
    require(
        not surviving,
        f"noncanonical Git environment survived: {surviving}",
    )
    surviving_proxies = sorted(
        key for key in child if key.casefold() in PROXY_NAMES
    )
    require(
        not surviving_proxies,
        f"proxy environment survived: {surviving_proxies}",
    )
    return child


def verify_context_stability(
    context: q046_git.GitContext,
    *,
    expected_head: str,
    expected_fingerprints: dict[
        str, tuple[int, int, int, int, int, int]
    ],
) -> None:
    observed_head = one_line(
        run_bound_git(context, "rev-parse", "HEAD").stdout,
        "bound Git HEAD",
    )
    require(
        observed_head == expected_head,
        "checked-out HEAD changed during admission verification",
    )
    require(
        context_fingerprints(context) == expected_fingerprints,
        "Git executable or repository context changed during verification",
    )


def run_verifier(args: list[str]) -> int:
    token_name = token_environment_name(args)
    try:
        context = q046_git.trusted_git_context()
    except SystemExit as error:
        raise AdmissionRunnerError(
            f"trusted Git context rejected checkout: {error}"
        ) from error

    expected_head = one_line(
        run_bound_git(context, "rev-parse", "HEAD").stdout,
        "bound Git HEAD",
    )
    require(
        tracked_status(context) == "",
        "tracked worktree is dirty before admission verification",
    )
    fingerprints = context_fingerprints(context)
    child = child_environment(
        context,
        token_name=token_name,
        source=os.environ,
    )

    completed = subprocess.run(
        [sys.executable, str(VERIFIER), *args],
        cwd=context.root,
        env=child,
        check=False,
    )
    verify_context_stability(
        context,
        expected_head=expected_head,
        expected_fingerprints=fingerprints,
    )
    require(
        tracked_status(context) == "",
        "tracked worktree is dirty after admission verification",
    )
    return completed.returncode


def check_clean() -> int:
    try:
        context = q046_git.trusted_git_context()
    except SystemExit as error:
        raise AdmissionRunnerError(
            f"trusted Git context rejected checkout: {error}"
        ) from error
    require(tracked_status(context) == "", "tracked worktree is dirty")
    print("PASS_HEPTA_INTELLIGENCE_ADMISSION_BOUND_GIT_CLEAN")
    return 0


def self_test() -> int:
    fake = SimpleNamespace(
        executable=Path("/usr/bin/git"),
        root=Path("/repo"),
        git_dir=Path("/repo/.git"),
        index=Path("/repo/.git/index"),
        env={
            "PATH": "/attacker:/usr/bin",
            "HOME": "/home/runner",
            "GITHUB_ACTIONS": "true",
            "RUNNER_OS": "Linux",
            "GIT_OBJECT_DIRECTORY": "/attacker/objects",
            "HTTP_PROXY": "http://attacker.invalid",
        },
    )
    environment = child_environment(
        fake,
        token_name="GH_TOKEN",
        source={"GH_TOKEN": "fixture-token"},
    )
    require(environment["PATH"] == "/usr/bin", "Git PATH was not pinned")
    require(
        environment["GIT_DIR"] == "/repo/.git",
        "Git directory was not bound",
    )
    require(
        environment["GIT_WORK_TREE"] == "/repo",
        "Git worktree was not bound",
    )
    require(
        environment["GIT_INDEX_FILE"] == "/repo/.git/index",
        "Git index was not bound",
    )
    require(
        "GIT_OBJECT_DIRECTORY" not in environment,
        "ambient Git object directory survived",
    )
    require(
        "HTTP_PROXY" not in environment,
        "ambient proxy survived",
    )
    require(
        token_environment_name(["--token-env", "GH_TOKEN"]) == "GH_TOKEN",
        "token environment parsing drifted",
    )
    for invalid in (
        ["--token-env"],
        ["--token-env", "GH_TOKEN", "--token-env=OTHER_TOKEN"],
        ["--token-env", "lowercase"],
        ["--token-env", "GIT_DIR"],
        ["--token-env", "HTTP_PROXY"],
        ["--token-env", "PATH"],
    ):
        try:
            token_environment_name(invalid)
        except AdmissionRunnerError:
            continue
        raise AdmissionRunnerError(
            f"invalid token argument unexpectedly passed: {invalid!r}"
        )
    print("PASS_HEPTA_INTELLIGENCE_ADMISSION_BOUND_GIT_FIXTURES")
    return 0


def main() -> int:
    try:
        if sys.argv[1:] == ["--self-test"]:
            return self_test()
        if sys.argv[1:] == ["--check-clean"]:
            return check_clean()
        return run_verifier(sys.argv[1:])
    except AdmissionRunnerError as error:
        print(
            f"FAIL_HEPTA_INTELLIGENCE_ADMISSION_BOUND_GIT: {error}",
            file=sys.stderr,
        )
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
