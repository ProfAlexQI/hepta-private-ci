#!/usr/bin/env python3
"""Q0.45 bind executable index objects to normalized worktree content."""

from __future__ import annotations

import hashlib
import os
import re
import subprocess
from pathlib import Path
from typing import Final

import verify_windows_gnullvm_q044_contract as q044
from hepta_q045_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB


ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q045_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "a0254c5242a50cabfb3bd1746f21950223cb8d55"
Q044_BLOB_CONTRACT = "scripts/hepta_q044_blob_contract.py"
Q044_SOURCE = "scripts/verify_windows_gnullvm_q044_contract.py"
DIRECT_ADAPTER = "scripts/verify-windows-gnullvm-direct-bazel.py"
JOB_ADAPTER = "scripts/verify-windows-gnullvm-job-executable.py"
ZERO_OBJECT_ID: Final = "0" * 40
UNSAFE_ATTRIBUTES: Final = frozenset(
    {"filter", "ident", "working-tree-encoding"}
)


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing Q0.45 path: {relative}")
    return path.read_text(encoding="utf-8")


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


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
        f"expected exactly one Git index entry for {relative}; "
        f"observed {len(entries)}",
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


def _canonical_worktree_file(relative: str) -> Path:
    root = ROOT.resolve(strict=True)
    lexical = root / Path(relative)
    try:
        resolved = lexical.resolve(strict=True)
    except OSError as error:
        fail(f"cannot resolve executable worktree path {relative}: {error}")
    require(
        os.path.normcase(str(resolved)) == os.path.normcase(str(lexical)),
        f"executable worktree path traverses a link or junction: {relative}",
    )
    require(
        resolved.is_file(),
        f"executable worktree path is not a regular file: {relative}",
    )
    return resolved


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
            f"unsafe Git clean attribute for {relative}: "
            f"{attribute}={value}",
        )


def _require_safe_attributes(relative: str) -> None:
    result = subprocess.run(
        ["git", "check-attr", "--all", "--", relative],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    require(result.returncode == 0, f"Git attribute lookup failed: {relative}")
    _validate_attribute_output(relative, result.stdout)


def _worktree_object_id(relative: str) -> str:
    _canonical_worktree_file(relative)
    _require_safe_attributes(relative)
    result = subprocess.run(
        ["git", "hash-object", f"--path={relative}", "--", relative],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    require(result.returncode == 0, f"worktree Git hash failed: {relative}")
    object_ids = result.stdout.splitlines()
    require(
        len(object_ids) == 1
        and re.fullmatch(r"[0-9a-f]{40}", object_ids[0]) is not None
        and object_ids[0] != ZERO_OBJECT_ID,
        f"invalid worktree Git object ID for {relative}: {result.stdout!r}",
    )
    return object_ids[0]


def _require_same_object(
    relative: str,
    index_object_id: str,
    worktree_object_id: str,
) -> None:
    require(
        index_object_id == worktree_object_id,
        "Git index/worktree object drift for "
        f"{relative}: index={index_object_id}, "
        f"worktree={worktree_object_id}",
    )


def require_git_executable(path_or_relative: str | Path) -> None:
    """Bind mode, stage, path, and normalized object identity."""

    relative = _git_relative(path_or_relative)
    result = subprocess.run(
        ["git", "ls-files", "--stage", "--", relative],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    require(result.returncode == 0, f"Git index lookup failed: {relative}")
    index_object_id = _validate_git_index_output(relative, result.stdout)
    worktree_object_id = _worktree_object_id(relative)
    _require_same_object(relative, index_object_id, worktree_object_id)


def prove_object_binding_fail_closed() -> None:
    relative = "scripts/example.py"
    object_id = "a" * 40
    valid = f"100755 {object_id} 0\t{relative}\n"
    require(
        _validate_git_index_output(relative, valid) == object_id,
        "valid Git-index fixture lost object identity",
    )
    _require_same_object(relative, object_id, object_id)
    _validate_attribute_output(
        relative,
        f"{relative}: text: set\n{relative}: eol: lf\n",
    )

    invalid_index = (
        "",
        valid + valid,
        f"100644 {object_id} 0\t{relative}\n",
        f"100755 {object_id} 1\t{relative}\n",
        f"100755 {ZERO_OBJECT_ID} 0\t{relative}\n",
        f"100755 {'A' * 40} 0\t{relative}\n",
        f"100755 not-an-object 0\t{relative}\n",
        f"100755 {object_id} 0\tscripts/other.py\n",
        f"100755 {object_id}\n",
    )
    for output in invalid_index:
        try:
            _validate_git_index_output(relative, output)
        except SystemExit:
            continue
        fail(f"Git-index adversarial fixture unexpectedly passed: {output!r}")

    for output in (
        "other.py: text: set\n",
        f"{relative}: filter: required\n",
        f"{relative}: ident: set\n",
        f"{relative}: working-tree-encoding: UTF-16\n",
        "malformed\n",
    ):
        try:
            _validate_attribute_output(relative, output)
        except SystemExit:
            continue
        fail(f"Git-attribute adversarial fixture unexpectedly passed: {output!r}")

    try:
        _require_same_object(relative, object_id, "b" * 40)
    except SystemExit:
        pass
    else:
        fail("Git index/worktree object mismatch unexpectedly passed")

    try:
        _git_relative("../escape.py")
    except SystemExit:
        pass
    else:
        fail("repository-escape executable path unexpectedly passed")


def patch_q044_compatibility() -> None:
    require(
        blob(ROOT / Q044_BLOB_CONTRACT) == BLOBS[Q044_BLOB_CONTRACT],
        "immutable Q0.44 blob contract drifted",
    )
    require(
        blob(ROOT / Q044_SOURCE) == BLOBS[Q044_SOURCE],
        "immutable Q0.44 source contract drifted",
    )

    q044.BLOBS = dict(q044.BLOBS)
    q044.BLOBS[DIRECT_ADAPTER] = BLOBS[DIRECT_ADAPTER]
    q044.BLOBS[JOB_ADAPTER] = BLOBS[JOB_ADAPTER]
    q044.DIRECT = DIRECT
    q044.JOB = JOB
    q044.require_git_executable = require_git_executable


def validate_increment() -> None:
    require(
        blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA,
        "Q0.45 blob contract drifted",
    )
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.45 blob: {relative}")
        require(blob(path) == expected, f"Q0.45 blob drift: {relative}")
    for relative in EXECUTABLE:
        require_git_executable(relative)

    require(read(DIRECT_ADAPTER) == DIRECT, "Q0.45 direct adapter drifted")
    require(read(JOB_ADAPTER) == JOB, "Q0.45 job adapter drifted")
    prove_object_binding_fail_closed()

    require(
        q044.require_git_executable is require_git_executable,
        "Q0.44 replay bypassed Q0.45 object binding",
    )
    for owner, observed in (
        ("Q0.43", q044.q043.require_git_mode),
        ("Q0.41", q044.q041.require_executable),
        ("Q0.40", q044.q041.q040.require_executable),
    ):
        require(
            observed is require_git_executable,
            f"{owner} retained an unbound executable proof",
        )
    require(
        q044.q041.q040.load_source is q044.strict_load_source,
        "dynamic verifier loading bypassed Q0.45 object binding",
    )

    print("PASS_WINDOWS_GNULLVM_Q0_45_INDEX_WORKTREE_OBJECT_BINDING_SOURCE")


def main(owner: str = "q045") -> None:
    require(
        owner in {"q045", "direct-bazel", "job-executable"},
        f"unknown Q0.45 owner {owner!r}",
    )
    patch_q044_compatibility()
    q044.main("q044" if owner == "q045" else owner)
    validate_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_45_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
