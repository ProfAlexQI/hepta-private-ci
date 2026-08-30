#!/usr/bin/env python3
from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path
from types import MappingProxyType

import hepta_q046_git_context as q046_git

PARTIAL_CLONE_KEY = re.compile(
    r"^(?:extensions\.partialClone|remote\..+\.(?:promisor|partialclonefilter))$",
    re.IGNORECASE,
)


class ObjectGraphError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ObjectGraphError(message)


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
        raise ObjectGraphError(
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


def require_canonical_directory(path: Path, owner: str) -> Path:
    require(path_exists_or_is_link(path), f"{owner} is missing: {path}")
    require(not path.is_symlink(), f"{owner} must not be a symlink: {path}")
    try:
        resolved = path.resolve(strict=True)
    except OSError as error:
        raise ObjectGraphError(f"cannot resolve {owner}: {error}") from error
    require(resolved == path, f"{owner} traverses a path alias: {path}")
    require(resolved.is_dir(), f"{owner} is not a directory: {path}")
    return resolved


def validate_replacement_refs(refs: list[str]) -> None:
    require(not refs, f"Git replacement refs are forbidden: {refs}")


def validate_partial_clone_config(raw: str) -> None:
    entries = [line for line in raw.splitlines() if line]
    invalid: list[str] = []
    for entry in entries:
        key = entry.split(None, 1)[0]
        if PARTIAL_CLONE_KEY.fullmatch(key):
            invalid.append(entry)
    require(
        not invalid,
        f"Git partial-clone or promisor configuration is forbidden: {invalid}",
    )


def require_immutable_object_graph(context: q046_git.GitContext) -> None:
    git_dir = context.git_dir
    require_canonical_directory(git_dir / "objects", "Git object database")
    require_canonical_directory(git_dir / "refs", "Git reference directory")

    forbidden = (
        ("Git common-directory redirect", git_dir / "commondir"),
        ("Git graft ancestry", git_dir / "info" / "grafts"),
        ("Git shallow boundary", git_dir / "shallow"),
        ("Git object alternates", git_dir / "objects" / "info" / "alternates"),
        (
            "Git HTTP object alternates",
            git_dir / "objects" / "info" / "http-alternates",
        ),
    )
    for owner, path in forbidden:
        require(
            not path_exists_or_is_link(path),
            f"{owner} is forbidden during admission verification: {path}",
        )

    replace_root = git_dir / "refs" / "replace"
    require(
        not replace_root.is_symlink(),
        f"Git replacement-ref root must not be a symlink: {replace_root}",
    )
    replacement_refs = run_git(
        context,
        "for-each-ref",
        "--format=%(refname)",
        "refs/replace",
    ).stdout.splitlines()
    validate_replacement_refs(replacement_refs)

    shallow = run_git(
        context,
        "rev-parse",
        "--is-shallow-repository",
    ).stdout.strip()
    require(shallow == "false", f"Git repository is shallow: {shallow!r}")

    config = run_git(
        context,
        "config",
        "--local",
        "--get-regexp",
        r"^(extensions\.partialClone|remote\..+\.(promisor|partialclonefilter))$",
        allowed_returncodes=frozenset({0, 1}),
    )
    validate_partial_clone_config(config.stdout)

    git_objects = run_git(
        context,
        "rev-parse",
        "--path-format=absolute",
        "--git-path",
        "objects",
    ).stdout.strip()
    require(
        Path(git_objects) == git_dir / "objects",
        f"Git object database path drifted: {git_objects!r}",
    )


def self_test() -> None:
    validate_replacement_refs([])
    validate_partial_clone_config("")
    for refs in (["refs/replace/" + "a" * 40], ["refs/replace/one", "two"]):
        try:
            validate_replacement_refs(refs)
        except ObjectGraphError:
            pass
        else:
            raise ObjectGraphError(
                f"replacement-ref fixture unexpectedly passed: {refs!r}"
            )
    for config in (
        "extensions.partialClone origin\n",
        "remote.origin.promisor true\n",
        "remote.origin.partialclonefilter blob:none\n",
    ):
        try:
            validate_partial_clone_config(config)
        except ObjectGraphError:
            pass
        else:
            raise ObjectGraphError(
                f"partial-clone fixture unexpectedly passed: {config!r}"
            )
    print("PASS_HEPTA_INTELLIGENCE_Q0_58_OBJECT_GRAPH_FIXTURES")


def main() -> int:
    try:
        if sys.argv[1:] == ["--self-test"]:
            self_test()
            return 0
        require(sys.argv[1:] == ["--check"], "expected --check or --self-test")
        context = trusted_context()
        require_immutable_object_graph(context)
        print("PASS_HEPTA_INTELLIGENCE_Q0_58_IMMUTABLE_OBJECT_GRAPH")
        return 0
    except (ObjectGraphError, subprocess.TimeoutExpired, OSError) as error:
        print(
            f"FAIL_HEPTA_INTELLIGENCE_Q0_58_OBJECT_GRAPH: {error}",
            file=sys.stderr,
        )
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
