#!/usr/bin/env python3
from __future__ import annotations

import os
import re
import stat
import subprocess
import sys
from pathlib import Path
from types import MappingProxyType

import hepta_q046_git_context as q046_git

PARTIAL_CLONE_KEY = re.compile(
    r"^(?:extensions\.partialClone|remote\..+\.(?:promisor|partialclonefilter))$",
    re.IGNORECASE,
)


class ConfigGraphError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ConfigGraphError(message)


def immutable_context(context: q046_git.GitContext) -> q046_git.GitContext:
    environment = dict(context.env)
    environment["GIT_NO_LAZY_FETCH"] = "1"
    environment["GIT_NO_REPLACE_OBJECTS"] = "1"
    return q046_git.GitContext(
        executable=context.executable,
        root=context.root,
        git_dir=context.git_dir,
        index=context.index,
        env=MappingProxyType(environment),
    )


def trusted_context() -> q046_git.GitContext:
    try:
        return immutable_context(q046_git.trusted_git_context())
    except SystemExit as error:
        raise ConfigGraphError(
            f"trusted Git context rejected checkout: {error}"
        ) from error


def run_git(
    context: q046_git.GitContext,
    *args: str,
    allowed_returncodes: frozenset[int] = frozenset({0}),
) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(
        [*context.prefix, *args],
        cwd=context.root,
        env=dict(context.env),
        check=False,
        capture_output=True,
        text=True,
        stdin=subprocess.DEVNULL,
        timeout=30,
    )
    require(
        completed.returncode in allowed_returncodes,
        "bound Git command failed "
        f"({completed.returncode}): {' '.join(args)}: "
        f"{completed.stderr.strip()}",
    )
    require(
        completed.stderr == "",
        f"bound Git command wrote stderr: {' '.join(args)}",
    )
    return completed


def path_exists_or_is_link(path: Path) -> bool:
    return path.exists() or path.is_symlink()


def canonical_regular_file(path: Path, owner: str) -> Path:
    require(path_exists_or_is_link(path), f"{owner} is missing: {path}")
    require(not path.is_symlink(), f"{owner} must not be a symlink: {path}")
    try:
        resolved = path.resolve(strict=True)
    except OSError as error:
        raise ConfigGraphError(f"cannot resolve {owner}: {error}") from error
    require(resolved == path, f"{owner} traverses a path alias: {path}")
    require(resolved.is_file(), f"{owner} is not a regular file: {path}")
    if os.name != "nt":
        mode = stat.S_IMODE(resolved.stat().st_mode)
        require(
            mode & (stat.S_IWGRP | stat.S_IWOTH) == 0,
            f"{owner} must not be group/world writable: {path}",
        )
    return resolved


def file_fingerprint(path: Path) -> tuple[int, int, int, int, int, int]:
    observed = os.stat(path, follow_symlinks=False)
    return (
        observed.st_dev,
        observed.st_ino,
        observed.st_mode,
        observed.st_size,
        observed.st_mtime_ns,
        observed.st_ctime_ns,
    )


def validate_local_config_names(names: list[str]) -> None:
    invalid: list[str] = []
    for name in names:
        require(name != "" and "\x00" not in name, "invalid local Git config key")
        folded = name.casefold()
        if (
            folded.startswith("include.")
            or folded.startswith("includeif.")
            or folded in {
                "extensions.partialclone",
                "extensions.worktreeconfig",
            }
        ):
            invalid.append(name)
    require(
        not invalid,
        f"external or split local Git configuration is forbidden: {invalid}",
    )


def validate_expanded_partial_clone_config(raw: str) -> None:
    invalid: list[str] = []
    for line in raw.splitlines():
        require(line != "" and "\x00" not in line, "invalid expanded config row")
        key = line.split(None, 1)[0]
        if PARTIAL_CLONE_KEY.fullmatch(key):
            invalid.append(line)
    require(
        not invalid,
        f"expanded partial-clone or promisor configuration is forbidden: {invalid}",
    )


def require_closed_config_graph(context: q046_git.GitContext) -> None:
    config = canonical_regular_file(context.git_dir / "config", "local Git config")
    config_worktree = context.git_dir / "config.worktree"
    require(
        not path_exists_or_is_link(config_worktree),
        f"split worktree Git config is forbidden: {config_worktree}",
    )
    before = file_fingerprint(config)

    names = run_git(
        context,
        "config",
        "--local",
        "--name-only",
        "--list",
    ).stdout.splitlines()
    validate_local_config_names(names)

    expanded = run_git(
        context,
        "config",
        "--local",
        "--includes",
        "--get-regexp",
        r"^(extensions\.partialClone|remote\..+\.(promisor|partialclonefilter))$",
        allowed_returncodes=frozenset({0, 1}),
    )
    validate_expanded_partial_clone_config(expanded.stdout)

    require(
        file_fingerprint(config) == before,
        "local Git config changed during object-graph verification",
    )
    require(
        not path_exists_or_is_link(config_worktree),
        "split worktree Git config appeared during verification",
    )


def self_test() -> None:
    validate_local_config_names(["core.repositoryformatversion", "remote.origin.url"])
    validate_expanded_partial_clone_config("")
    rejected_names = (
        ["include.path"],
        ["Include.Path"],
        ["includeIf.gitdir:/tmp/.path"],
        ["extensions.worktreeConfig"],
        ["extensions.partialClone"],
    )
    for names in rejected_names:
        try:
            validate_local_config_names(names)
        except ConfigGraphError:
            pass
        else:
            raise ConfigGraphError(
                f"local config fixture unexpectedly passed: {names!r}"
            )
    for raw in (
        "extensions.partialClone origin\n",
        "remote.origin.promisor true\n",
        "remote.origin.partialclonefilter blob:none\n",
    ):
        try:
            validate_expanded_partial_clone_config(raw)
        except ConfigGraphError:
            pass
        else:
            raise ConfigGraphError(
                f"expanded config fixture unexpectedly passed: {raw!r}"
            )
    print("PASS_HEPTA_INTELLIGENCE_Q0_60_CONFIG_GRAPH_FIXTURES")


def main() -> int:
    try:
        if sys.argv[1:] == ["--self-test"]:
            self_test()
            return 0
        require(sys.argv[1:] == ["--check"], "expected --check or --self-test")
        context = trusted_context()
        require_closed_config_graph(context)
        print("PASS_HEPTA_INTELLIGENCE_Q0_60_CLOSED_CONFIG_GRAPH")
        return 0
    except (ConfigGraphError, subprocess.TimeoutExpired, OSError) as error:
        print(
            f"FAIL_HEPTA_INTELLIGENCE_Q0_60_CONFIG_GRAPH: {error}",
            file=sys.stderr,
        )
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
