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
ACTIONS_CHECKOUT_RESIDUE_VALUES = MappingProxyType(
    {
        "core.sparsecheckout": "false",
        "core.sparsecheckoutcone": "false",
        "index.sparse": "false",
    }
)
MAX_ACTIONS_CHECKOUT_RESIDUE_BYTES = 1024


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


def fsync_directory(path: Path) -> None:
    if os.name == "nt":
        return
    flags = os.O_RDONLY
    if hasattr(os, "O_DIRECTORY"):
        flags |= os.O_DIRECTORY
    descriptor = os.open(path, flags)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


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


def validate_actions_checkout_residue_names(names: list[str]) -> list[str]:
    require(
        all(name != "" and "\x00" not in name for name in names),
        "invalid actions/checkout residue config key",
    )
    folded = [name.casefold() for name in names]
    require(
        len(folded) == len(set(folded)),
        "duplicate actions/checkout residue config key",
    )
    require(
        set(folded) == set(ACTIONS_CHECKOUT_RESIDUE_VALUES),
        "actions/checkout residue contains a noncanonical config key set: "
        f"{sorted(folded)!r}",
    )
    return folded


def validate_actions_checkout_residue_value(
    name: str,
    values: list[str],
) -> None:
    expected = ACTIONS_CHECKOUT_RESIDUE_VALUES.get(name.casefold())
    require(expected is not None, f"unexpected actions/checkout residue key: {name}")
    require(
        values == [expected],
        "actions/checkout residue value drifted: "
        f"{name}={values!r}, expected={[expected]!r}",
    )


def normalize_actions_checkout_residue(
    context: q046_git.GitContext,
) -> bool:
    """Remove only the exact inert split-config residue left by checkout v6.

    `git sparse-checkout disable` enables per-worktree configuration and writes
    three explicit `false` values before actions/checkout removes the enabling
    key from the main local config.  The ignored file must not survive the
    admission boundary, but it is safe to remove only after its file identity,
    complete key set, values, and disabled extension have all been verified.
    """

    config = canonical_regular_file(context.git_dir / "config", "local Git config")
    config_worktree = context.git_dir / "config.worktree"
    if not path_exists_or_is_link(config_worktree):
        return False

    residue = canonical_regular_file(
        config_worktree,
        "actions/checkout split-config residue",
    )
    require(
        residue.stat().st_size <= MAX_ACTIONS_CHECKOUT_RESIDUE_BYTES,
        "actions/checkout split-config residue is too large",
    )
    config_before = file_fingerprint(config)
    residue_before = file_fingerprint(residue)

    extension = run_git(
        context,
        "config",
        "--local",
        "--get-all",
        "extensions.worktreeConfig",
        allowed_returncodes=frozenset({0, 1}),
    )
    require(
        extension.returncode == 1 and extension.stdout == "",
        "extensions.worktreeConfig is still enabled or ambiguous",
    )

    names = run_git(
        context,
        "config",
        "--file",
        str(residue),
        "--name-only",
        "--list",
    ).stdout.splitlines()
    folded_names = validate_actions_checkout_residue_names(names)
    for name in folded_names:
        values = run_git(
            context,
            "config",
            "--file",
            str(residue),
            "--get-all",
            name,
        ).stdout.splitlines()
        validate_actions_checkout_residue_value(name, values)

    require(
        file_fingerprint(config) == config_before,
        "local Git config changed while inspecting checkout residue",
    )
    require(
        file_fingerprint(residue) == residue_before,
        "checkout residue changed while it was being inspected",
    )

    residue.unlink()
    fsync_directory(context.git_dir)
    require(
        not path_exists_or_is_link(config_worktree),
        "checkout residue remained after verified normalization",
    )
    require(
        file_fingerprint(config) == config_before,
        "local Git config changed during checkout residue normalization",
    )
    return True


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
    validate_actions_checkout_residue_names(
        ["core.sparsecheckout", "core.sparsecheckoutcone", "index.sparse"]
    )
    for name, expected in ACTIONS_CHECKOUT_RESIDUE_VALUES.items():
        validate_actions_checkout_residue_value(name, [expected])

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

    invalid_residue_names = (
        ["core.sparsecheckout", "core.sparsecheckoutcone"],
        [
            "core.sparsecheckout",
            "core.sparsecheckoutcone",
            "index.sparse",
            "include.path",
        ],
        ["core.sparsecheckout", "core.sparsecheckout", "index.sparse"],
    )
    for names in invalid_residue_names:
        try:
            validate_actions_checkout_residue_names(names)
        except ConfigGraphError:
            pass
        else:
            raise ConfigGraphError(
                f"checkout residue key fixture unexpectedly passed: {names!r}"
            )

    for name, values in (
        ("core.sparsecheckout", ["true"]),
        ("core.sparsecheckoutcone", ["false", "false"]),
        ("index.sparse", []),
        ("include.path", ["false"]),
    ):
        try:
            validate_actions_checkout_residue_value(name, values)
        except ConfigGraphError:
            pass
        else:
            raise ConfigGraphError(
                "checkout residue value fixture unexpectedly passed: "
                f"{name}={values!r}"
            )

    print("PASS_HEPTA_INTELLIGENCE_Q0_60_CONFIG_GRAPH_FIXTURES")


def main() -> int:
    try:
        arguments = sys.argv[1:]
        if arguments == ["--self-test"]:
            self_test()
            return 0
        if arguments == ["--normalize-actions-checkout-residue"]:
            context = trusted_context()
            removed = normalize_actions_checkout_residue(context)
            require_closed_config_graph(context)
            disposition = "REMOVED" if removed else "ABSENT"
            print(
                "PASS_HEPTA_INTELLIGENCE_Q0_63_ACTIONS_CHECKOUT_CONFIG_"
                f"{disposition}"
            )
            return 0
        require(
            arguments == ["--check"],
            "expected --check, --self-test, or "
            "--normalize-actions-checkout-residue",
        )
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
