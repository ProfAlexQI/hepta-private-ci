#!/usr/bin/env python3
"""Q0.46 bind Git executable, repository, index, and worktree context."""

from __future__ import annotations

import hashlib
import os
import re
import shutil
import stat
import subprocess
from dataclasses import dataclass
from functools import lru_cache
from pathlib import Path
from types import MappingProxyType
from typing import Final, Mapping


ROOT = Path(__file__).resolve().parents[1]
ZERO_OBJECT_ID: Final = "0" * 40
UNSAFE_ATTRIBUTES: Final = frozenset(
    {"filter", "ident", "working-tree-encoding"}
)


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def _same_path(left: Path, right: Path) -> bool:
    return os.path.normcase(os.path.normpath(str(left))) == os.path.normcase(
        os.path.normpath(str(right))
    )


def _path_is_within(path: Path, directory: Path) -> bool:
    try:
        path.relative_to(directory)
    except ValueError:
        return False
    return True


def _canonical_existing(
    lexical: Path,
    *,
    owner: str,
    directory: bool,
) -> Path:
    require(lexical.is_absolute(), f"{owner} must be absolute")
    require(not lexical.is_symlink(), f"{owner} must not be a symlink")
    try:
        resolved = lexical.resolve(strict=True)
    except OSError as error:
        fail(f"cannot resolve {owner}: {error}")
    require(
        _same_path(lexical, resolved),
        f"{owner} traverses a link or junction: {lexical}",
    )
    require(
        resolved.is_dir() if directory else resolved.is_file(),
        f"{owner} has the wrong file type: {resolved}",
    )
    return resolved


def _environment_value(
    env: Mapping[str, str],
    name: str,
) -> str | None:
    folded = name.casefold()
    matches = [value for key, value in env.items() if key.casefold() == folded]
    require(
        len(matches) <= 1,
        f"duplicate case-insensitive environment variable: {name}",
    )
    return matches[0] if matches else None


def sanitized_git_environment(
    base: Mapping[str, str],
) -> dict[str, str]:
    """Return a local-only Git environment with ambient Git controls removed."""

    clean = {
        key: value
        for key, value in base.items()
        if not key.upper().startswith("GIT_")
        and key.casefold() not in {"lang", "lc_all", "path"}
    }
    path_value = _environment_value(base, "PATH")
    require(bool(path_value), "Git proof requires a nonempty PATH")
    clean["PATH"] = str(path_value)
    clean["LANG"] = "C"
    clean["LC_ALL"] = "C"
    clean["GIT_CONFIG_NOSYSTEM"] = "1"
    clean["GIT_CONFIG_GLOBAL"] = os.devnull
    clean["GIT_OPTIONAL_LOCKS"] = "0"
    return clean


def _validate_git_binary(
    candidate: Path,
    *,
    base_env: Mapping[str, str],
) -> Path:
    require(candidate.is_absolute(), "Git executable path must be absolute")
    require(not candidate.is_symlink(), "Git executable must not be a symlink")
    try:
        resolved = candidate.resolve(strict=True)
    except OSError as error:
        fail(f"cannot resolve Git executable: {error}")
    require(resolved.is_file(), "Git executable must be a regular file")

    root = ROOT.resolve(strict=True)
    require(
        not _path_is_within(resolved, root),
        "Git executable must not come from the candidate repository",
    )

    if os.name != "nt":
        mode = stat.S_IMODE(resolved.stat().st_mode)
        require(
            mode & (stat.S_IWGRP | stat.S_IWOTH) == 0,
            "Git executable must not be group/world writable",
        )

    if _environment_value(base_env, "GITHUB_ACTIONS") == "true":
        runner_os = _environment_value(base_env, "RUNNER_OS")
        require(
            runner_os in {"Linux", "macOS", "Windows"},
            f"unsupported GitHub-hosted runner OS: {runner_os!r}",
        )
        if runner_os in {"Linux", "macOS"}:
            approved = {
                Path("/usr/bin/git").resolve(strict=False),
                Path("/usr/local/bin/git").resolve(strict=False),
            }
            require(
                any(_same_path(resolved, path) for path in approved),
                f"unapproved hosted Git executable: {resolved}",
            )
        else:
            program_files = _environment_value(base_env, "ProgramFiles")
            require(bool(program_files), "Windows Git proof requires ProgramFiles")
            git_root = Path(str(program_files)) / "Git"
            try:
                git_root = git_root.resolve(strict=True)
            except OSError as error:
                fail(f"cannot resolve Windows Git installation root: {error}")
            require(
                _path_is_within(resolved, git_root)
                and resolved.name.casefold() == "git.exe",
                f"unapproved hosted Windows Git executable: {resolved}",
            )
    return resolved


def _resolve_git_binary(base_env: Mapping[str, str]) -> Path:
    path_value = _environment_value(base_env, "PATH")
    require(bool(path_value), "Git proof requires PATH")
    candidate = shutil.which("git", path=str(path_value))
    require(bool(candidate), "Git executable was not found on PATH")
    return _validate_git_binary(Path(str(candidate)), base_env=base_env)


def _one_output_line(output: str, *, owner: str) -> str:
    lines = output.splitlines()
    require(len(lines) == 1 and bool(lines[0]), f"{owner} output drifted")
    require("\x00" not in lines[0], f"{owner} output contains NUL")
    return lines[0]


def _validate_reported_path(
    output: str,
    expected: Path,
    *,
    owner: str,
) -> None:
    value = Path(_one_output_line(output, owner=owner))
    require(value.is_absolute(), f"{owner} did not report an absolute path")
    try:
        observed = value.resolve(strict=True)
    except OSError as error:
        fail(f"cannot resolve {owner} output: {error}")
    require(
        _same_path(observed, expected),
        f"{owner} drifted: expected {expected}, observed {observed}",
    )


@dataclass(frozen=True)
class GitContext:
    executable: Path
    root: Path
    git_dir: Path
    index: Path
    env: Mapping[str, str]

    @property
    def prefix(self) -> tuple[str, ...]:
        return (
            str(self.executable),
            "--no-optional-locks",
            "--literal-pathspecs",
            f"--git-dir={self.git_dir}",
            f"--work-tree={self.root}",
        )


def _run_raw(
    executable: Path,
    env: Mapping[str, str],
    root: Path,
    *args: str,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [str(executable), *args],
        cwd=root,
        env=dict(env),
        capture_output=True,
        text=True,
        check=False,
    )


@lru_cache(maxsize=1)
def trusted_git_context() -> GitContext:
    """Resolve one immutable local Git context for the checked-out candidate."""

    base_env = dict(os.environ)
    root = _canonical_existing(ROOT, owner="repository root", directory=True)
    git_dir = _canonical_existing(
        root / ".git",
        owner="repository Git directory",
        directory=True,
    )
    index = _canonical_existing(
        git_dir / "index",
        owner="repository Git index",
        directory=False,
    )
    env = sanitized_git_environment(base_env)
    executable = _resolve_git_binary(base_env)

    version = _run_raw(executable, env, root, "--version")
    require(version.returncode == 0, "Git executable version probe failed")
    version_line = _one_output_line(
        version.stdout,
        owner="Git executable version",
    )
    require(
        re.fullmatch(r"git version [0-9][ -~]{0,95}", version_line) is not None,
        f"Git executable returned a noncanonical version: {version_line!r}",
    )
    require(not version.stderr, "Git executable version probe wrote stderr")

    prefix = (
        "--no-optional-locks",
        "--literal-pathspecs",
        f"--git-dir={git_dir}",
        f"--work-tree={root}",
    )
    top = _run_raw(
        executable,
        env,
        root,
        *prefix,
        "rev-parse",
        "--show-toplevel",
    )
    require(top.returncode == 0, "Git top-level proof failed")
    require(not top.stderr, "Git top-level proof wrote stderr")
    _validate_reported_path(top.stdout, root, owner="Git top-level")

    absolute_git_dir = _run_raw(
        executable,
        env,
        root,
        *prefix,
        "rev-parse",
        "--absolute-git-dir",
    )
    require(absolute_git_dir.returncode == 0, "Git directory proof failed")
    require(not absolute_git_dir.stderr, "Git directory proof wrote stderr")
    _validate_reported_path(
        absolute_git_dir.stdout,
        git_dir,
        owner="Git directory",
    )

    reported_index = _run_raw(
        executable,
        env,
        root,
        *prefix,
        "rev-parse",
        "--path-format=absolute",
        "--git-path",
        "index",
    )
    require(reported_index.returncode == 0, "Git index-path proof failed")
    require(not reported_index.stderr, "Git index-path proof wrote stderr")
    _validate_reported_path(
        reported_index.stdout,
        index,
        owner="Git index path",
    )

    return GitContext(
        executable=executable,
        root=root,
        git_dir=git_dir,
        index=index,
        env=MappingProxyType(dict(env)),
    )


def _run_git(
    context: GitContext,
    *args: str,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [*context.prefix, *args],
        cwd=context.root,
        env=dict(context.env),
        capture_output=True,
        text=True,
        check=False,
    )


def _git_relative(path_or_relative: str | Path) -> str:
    candidate = Path(path_or_relative)
    if candidate.is_absolute():
        try:
            candidate = candidate.resolve(strict=False).relative_to(
                ROOT.resolve(strict=True)
            )
        except (OSError, ValueError) as error:
            fail(
                "executable proof path escaped repository: "
                f"{path_or_relative}: {error}"
            )
    normalized = candidate.as_posix()
    require(
        normalized not in {"", "."}
        and not normalized.startswith("../")
        and "/../" not in f"/{normalized}",
        f"non-canonical executable proof path: {normalized!r}",
    )
    return normalized


def _validate_git_index_output(relative: str, output: str) -> str:
    entries = output.splitlines()
    require(
        len(entries) == 1,
        f"expected one Git index entry for {relative}; observed {len(entries)}",
    )
    fields = entries[0].split(maxsplit=3)
    require(len(fields) == 4, f"malformed Git index entry for {relative}")
    mode, object_id, stage, indexed_path = fields
    require(mode == "100755", f"lost Git executable mode: {relative}")
    require(stage == "0", f"unmerged Git index stage for {relative}: {stage}")
    require(
        re.fullmatch(r"[0-9a-f]{40}", object_id) is not None
        and object_id != ZERO_OBJECT_ID,
        f"invalid Git object ID for {relative}: {object_id!r}",
    )
    require(
        indexed_path == relative,
        f"Git index path drift for {relative}: {indexed_path!r}",
    )
    return object_id


def _validate_attribute_output(relative: str, output: str) -> None:
    for line in output.splitlines():
        fields = line.split(": ", 2)
        require(
            len(fields) == 3 and fields[0] == relative,
            f"malformed Git attribute output for {relative}: {line!r}",
        )
        _, attribute, value = fields
        require(
            attribute not in UNSAFE_ATTRIBUTES or value == "unset",
            f"unsafe Git clean attribute for {relative}: {attribute}={value}",
        )


def _canonical_worktree_file(relative: str) -> Path:
    root = ROOT.resolve(strict=True)
    lexical = root / Path(relative)
    return _canonical_existing(
        lexical,
        owner=f"executable worktree path {relative}",
        directory=False,
    )


def _file_fingerprint(path: Path) -> tuple[int, int, int, int, int, int]:
    observed = os.stat(path, follow_symlinks=False)
    return (
        observed.st_dev,
        observed.st_ino,
        observed.st_mode,
        observed.st_size,
        observed.st_mtime_ns,
        observed.st_ctime_ns,
    )


def _worktree_object_id(
    context: GitContext,
    relative: str,
) -> tuple[str, Path]:
    path = _canonical_worktree_file(relative)
    before = _file_fingerprint(path)

    attributes = _run_git(context, "check-attr", "--all", "--", relative)
    require(
        attributes.returncode == 0,
        f"Git attribute lookup failed: {relative}",
    )
    require(
        not attributes.stderr,
        f"Git attribute lookup wrote stderr: {relative}",
    )
    _validate_attribute_output(relative, attributes.stdout)

    hashed = _run_git(
        context,
        "hash-object",
        f"--path={relative}",
        "--",
        relative,
    )
    require(hashed.returncode == 0, f"worktree Git hash failed: {relative}")
    require(not hashed.stderr, f"worktree Git hash wrote stderr: {relative}")
    object_id = _one_output_line(
        hashed.stdout,
        owner=f"worktree Git hash for {relative}",
    )
    require(
        re.fullmatch(r"[0-9a-f]{40}", object_id) is not None
        and object_id != ZERO_OBJECT_ID,
        f"invalid worktree Git object ID for {relative}: {object_id!r}",
    )

    attributes_after = _run_git(
        context,
        "check-attr",
        "--all",
        "--",
        relative,
    )
    require(
        attributes_after.returncode == 0,
        f"second Git attribute lookup failed: {relative}",
    )
    require(
        not attributes_after.stderr,
        f"second Git attribute lookup wrote stderr: {relative}",
    )
    _validate_attribute_output(relative, attributes_after.stdout)
    require(
        attributes_after.stdout == attributes.stdout,
        f"Git attributes changed during executable proof: {relative}",
    )
    require(
        _file_fingerprint(path) == before,
        f"worktree file changed during executable proof: {relative}",
    )
    return object_id, path


def require_git_executable(path_or_relative: str | Path) -> None:
    """Bind Git binary, repository, index, mode, path, and content."""

    context = trusted_git_context()
    relative = _git_relative(path_or_relative)

    indexed = _run_git(
        context,
        "ls-files",
        "--stage",
        "--full-name",
        "--",
        relative,
    )
    require(indexed.returncode == 0, f"Git index lookup failed: {relative}")
    require(not indexed.stderr, f"Git index lookup wrote stderr: {relative}")
    index_object_id = _validate_git_index_output(relative, indexed.stdout)
    worktree_object_id, _ = _worktree_object_id(context, relative)
    require(
        index_object_id == worktree_object_id,
        "Git index/worktree object drift for "
        f"{relative}: index={index_object_id}, "
        f"worktree={worktree_object_id}",
    )


def prove_git_context_fail_closed() -> None:
    hostile = {
        "PATH": os.environ.get("PATH", os.defpath),
        "HOME": str(ROOT),
        "GIT_DIR": "/tmp/attacker",
        "git_index_file": "/tmp/attacker-index",
        "GIT_CONFIG_COUNT": "1",
        "GIT_CONFIG_KEY_0": "core.attributesFile",
        "GIT_CONFIG_VALUE_0": "/tmp/attacker-attributes",
        "LANG": "attacker",
        "LC_ALL": "attacker",
    }
    sanitized = sanitized_git_environment(hostile)
    for name in hostile:
        if name.upper().startswith("GIT_"):
            require(name not in sanitized, f"ambient Git variable survived: {name}")
    require(sanitized["LANG"] == "C", "Git LANG is not canonical")
    require(sanitized["LC_ALL"] == "C", "Git LC_ALL is not canonical")
    require(
        sanitized["GIT_CONFIG_NOSYSTEM"] == "1",
        "system Git config was not disabled",
    )
    require(
        sanitized["GIT_CONFIG_GLOBAL"] == os.devnull,
        "global Git config was not redirected to the null device",
    )
    require(
        sanitized["GIT_OPTIONAL_LOCKS"] == "0",
        "optional Git locks were not disabled",
    )

    repository_file = Path(__file__).resolve()
    try:
        _validate_git_binary(repository_file, base_env=os.environ)
    except SystemExit:
        pass
    else:
        fail("repository-local Git executable fixture unexpectedly passed")

    try:
        _validate_reported_path(
            f"{ROOT.parent}\n",
            ROOT.resolve(strict=True),
            owner="adversarial Git top-level",
        )
    except SystemExit:
        pass
    else:
        fail("wrong Git top-level fixture unexpectedly passed")

    relative = "scripts/example.py"
    object_id = "a" * 40
    valid = f"100755 {object_id} 0\t{relative}\n"
    require(
        _validate_git_index_output(relative, valid) == object_id,
        "valid Git-index fixture lost object identity",
    )
    for invalid in (
        valid + valid,
        f"100644 {object_id} 0\t{relative}\n",
        f"100755 {object_id} 1\t{relative}\n",
        f"100755 {'A' * 40} 0\t{relative}\n",
        f"100755 {object_id} 0\tscripts/other.py\n",
    ):
        try:
            _validate_git_index_output(relative, invalid)
        except SystemExit:
            continue
        fail(f"Git-index adversarial fixture unexpectedly passed: {invalid!r}")


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


if __name__ == "__main__":
    prove_git_context_fail_closed()
    print("PASS_WINDOWS_GNULLVM_Q0_46_GIT_CONTEXT_FIXTURES")
