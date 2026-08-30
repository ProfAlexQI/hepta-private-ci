"""Q0.34 exact workspace and target-manifest launch authority."""

from __future__ import annotations

import subprocess
from collections.abc import Callable, Mapping, Sequence
from pathlib import Path

from run_bazel_q022_negative_targets import CANONICAL_RELEASE_TARGETS
from run_bazel_q029_job_executable import (
    CLIPPY_JOB,
    CLIPPY_TARGET_PREFIX,
    RELEASE_JOB,
    TEST_JOB,
    _require_env,
    _split_command,
)
from run_bazel_q030_direct_bazel import (
    validate_keyless_windows_gnullvm_command as _validate_q032,
)

WINDOWS_TEST_QUERY = (
    'tests(//...) except tests(//third_party/v8:all) except '
    'attr(tags, "manual", tests(//...))'
)
CLIPPY_MANUAL_TEST_QUERY = (
    'kind("rust_test rule", attr(tags, "manual", '
    '//codex-rs/... except //codex-rs/v8-poc/...))'
)
QUERY_OPTIONS = (
    "--config=ci-windows",
    "--noshow_progress",
    "--nouse_action_cache",
    "--nouse_analysis_cache",
    "--output=label",
)
CRC32_POLYNOMIAL = 0x04C11DB7


def _crc32_table() -> tuple[int, ...]:
    table: list[int] = []
    for value in range(256):
        crc = value << 24
        for _ in range(8):
            if crc & 0x80000000:
                crc = ((crc << 1) ^ CRC32_POLYNOMIAL) & 0xFFFFFFFF
            else:
                crc = (crc << 1) & 0xFFFFFFFF
        table.append(crc)
    return tuple(table)


CRC32_TABLE = _crc32_table()


def _posix_cksum(data: bytes) -> int:
    """Return the POSIX `cksum` CRC used by the reviewed shard generator."""

    crc = 0
    for value in data:
        crc = ((crc << 8) & 0xFFFFFFFF) ^ CRC32_TABLE[
            ((crc >> 24) ^ value) & 0xFF
        ]

    length = len(data)
    while length:
        value = length & 0xFF
        crc = ((crc << 8) & 0xFFFFFFFF) ^ CRC32_TABLE[
            ((crc >> 24) ^ value) & 0xFF
        ]
        length >>= 8
    return (~crc) & 0xFFFFFFFF


def _canonical_workspace(env: Mapping[str, str]) -> Path:
    raw = Path(_require_env(env, "GITHUB_WORKSPACE"))
    if not raw.is_absolute():
        raise ValueError("GITHUB_WORKSPACE must be absolute")
    if raw.is_symlink():
        raise ValueError("GITHUB_WORKSPACE must not be a symlink")
    try:
        workspace = raw.resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve GITHUB_WORKSPACE: {error}") from error
    if not workspace.is_dir():
        raise ValueError("GITHUB_WORKSPACE must resolve to a directory")
    return workspace


def _query_labels(
    command: Sequence[str],
    env: Mapping[str, str],
    workspace: Path,
    expression: str,
    *,
    run: Callable[..., subprocess.CompletedProcess[str]],
) -> list[str]:
    startup, _command_name, _options, _targets = _split_command(command)
    query_command = [
        command[0],
        *startup,
        "query",
        *QUERY_OPTIONS,
        "--",
        expression,
    ]
    result = run(
        query_command,
        cwd=workspace,
        env=dict(env),
        capture_output=True,
        text=True,
        check=False,
        timeout=180,
    )
    if result.returncode != 0:
        raise ValueError(
            "target-manifest Bazel query failed: "
            f"exit={result.returncode}"
        )

    labels: list[str] = []
    for raw_line in result.stdout.splitlines():
        if not raw_line:
            continue
        if raw_line != raw_line.strip():
            raise ValueError("target-manifest query emitted padded output")
        if not raw_line.startswith("//"):
            raise ValueError(
                "target-manifest query emitted a non-workspace label"
            )
        labels.append(raw_line)
    if not labels:
        raise ValueError("target-manifest query returned no labels")
    if len(labels) != len(set(labels)):
        raise ValueError("target-manifest query returned duplicate labels")
    return labels


def _windows_test_targets(
    labels: Sequence[str],
    env: Mapping[str, str],
) -> list[str]:
    try:
        shard = int(_require_env(env, "BAZEL_TEST_SHARD"))
        shard_count = int(_require_env(env, "BAZEL_TEST_SHARD_COUNT"))
    except ValueError as error:
        raise ValueError("Windows test shard values must be integers") from error
    if shard_count != 4 or shard not in range(1, shard_count + 1):
        raise ValueError("Windows test target manifest requires shard N/4")

    selected = [
        target
        for target in sorted(labels)
        if (_posix_cksum(f"{target}\n".encode("utf-8")) % shard_count) + 1 == shard
    ]
    if not selected:
        raise ValueError(
            "Windows test target manifest selected no targets for "
            f"shard {shard}/{shard_count}"
        )
    return selected


def _clippy_targets(labels: Sequence[str]) -> list[str]:
    manual_targets = [
        target
        for target in sorted(labels)
        if not target.endswith("-test-bin")
    ]
    return [*CLIPPY_TARGET_PREFIX, *manual_targets]


def _expected_targets(
    command: Sequence[str],
    env: Mapping[str, str],
    workspace: Path,
    *,
    run: Callable[..., subprocess.CompletedProcess[str]],
) -> list[str]:
    job = _require_env(env, "GITHUB_JOB")
    if job == TEST_JOB:
        labels = _query_labels(
            command,
            env,
            workspace,
            WINDOWS_TEST_QUERY,
            run=run,
        )
        return _windows_test_targets(labels, env)
    if job == CLIPPY_JOB:
        labels = _query_labels(
            command,
            env,
            workspace,
            CLIPPY_MANUAL_TEST_QUERY,
            run=run,
        )
        return _clippy_targets(labels)
    if job == RELEASE_JOB:
        return list(CANONICAL_RELEASE_TARGETS)
    raise ValueError(f"unknown target-manifest job {job!r}")


def validate_keyless_windows_gnullvm_execution(
    command: Sequence[str],
    env: Mapping[str, str],
    *,
    run: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
) -> Path:
    """Return the only permitted cwd after recomputing the complete targets."""

    _validate_q032(command, env)
    workspace = _canonical_workspace(env)
    _startup, _command_name, _options, observed_targets = _split_command(command)
    expected_targets = _expected_targets(
        command,
        env,
        workspace,
        run=run,
    )
    if observed_targets != expected_targets:
        raise ValueError(
            "final Bazel target manifest is not exact: "
            f"expected {expected_targets!r}, observed {observed_targets!r}"
        )

    # Re-run the Q0.32 executable/digest/path checks after target discovery so
    # the last validation before launch still rehashes the direct Bazel binary.
    _validate_q032(command, env)
    return workspace
