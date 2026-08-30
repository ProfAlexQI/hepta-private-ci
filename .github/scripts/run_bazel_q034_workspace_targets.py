"""Q0.34 canonical workspace and generated-target authority contract."""

from __future__ import annotations

import hashlib
import subprocess
from collections.abc import Callable, Mapping, Sequence
from pathlib import Path

from run_bazel_q029_job_executable import (
    BAZEL_WINDOWS_X86_64_SHA256,
    CLIPPY_JOB,
    RELEASE_JOB,
    TEST_JOB,
    _sha256_file,
    _split_command,
    _validate_bazelisk_inputs,
    _validate_runner_identity,
)
from run_bazel_q030_direct_bazel import (
    _require_cas_identity,
    _require_transport_token_absent,
    _verified_regular_file,
)

TEST_TARGET_QUERY = (
    'tests(//...) except tests(//third_party/v8:all) '
    'except attr(tags, "manual", tests(//...))'
)
CLIPPY_TARGET_QUERY = (
    'kind("rust_test rule", '
    'attr(tags, "manual", //codex-rs/... except //codex-rs/v8-poc/...))'
)
CLIPPY_TARGET_PREFIX = (
    "//codex-rs/...",
    "-//codex-rs/v8-poc:all",
)
QUERY_TIMEOUT_SECONDS = 300


def _crc_table() -> tuple[int, ...]:
    polynomial = 0x04C11DB7
    values: list[int] = []
    for index in range(256):
        crc = index << 24
        for _ in range(8):
            if crc & 0x80000000:
                crc = ((crc << 1) ^ polynomial) & 0xFFFFFFFF
            else:
                crc = (crc << 1) & 0xFFFFFFFF
        values.append(crc)
    return tuple(values)


_CRC_TABLE = _crc_table()


def posix_cksum(data: bytes) -> int:
    """Return the POSIX cksum CRC used by the reviewed shard generator."""

    crc = 0
    for byte in data:
        crc = ((crc << 8) & 0xFFFFFFFF) ^ _CRC_TABLE[
            ((crc >> 24) ^ byte) & 0xFF
        ]
    length = len(data)
    while length:
        byte = length & 0xFF
        length >>= 8
        crc = ((crc << 8) & 0xFFFFFFFF) ^ _CRC_TABLE[
            ((crc >> 24) ^ byte) & 0xFF
        ]
    return (~crc) & 0xFFFFFFFF


def _vector_digest(values: Sequence[str]) -> str:
    payload = b"\0".join(value.encode("ascii") for value in values)
    return hashlib.sha256(payload).hexdigest()


def _canonical_labels(stdout: str, *, owner: str) -> tuple[str, ...]:
    labels = tuple(stdout.splitlines())
    if not labels:
        raise ValueError(f"{owner} produced no labels")
    if len(labels) != len(set(labels)):
        raise ValueError(f"{owner} produced duplicate labels")
    for label in labels:
        if (
            not label
            or not label.isascii()
            or label != label.strip()
            or not label.startswith("//")
            or label.startswith("-")
            or any(character.isspace() for character in label)
        ):
            raise ValueError(f"{owner} produced a non-canonical workspace label")
    return labels


def _query_labels(
    executable: Path,
    startup: Sequence[str],
    expression: str,
    workspace: Path,
    env: Mapping[str, str],
    *,
    run: Callable[..., subprocess.CompletedProcess[str]],
    owner: str,
) -> tuple[str, ...]:
    command = [
        str(executable),
        *startup,
        "query",
        f"--repo_contents_cache={env['BAZEL_REPO_CONTENTS_CACHE']}",
        f"--repository_cache={env['BAZEL_REPOSITORY_CACHE']}",
        "--output=label",
        expression,
    ]
    result = run(
        command,
        cwd=workspace,
        env=dict(env),
        capture_output=True,
        text=True,
        check=False,
        timeout=QUERY_TIMEOUT_SECONDS,
    )
    if result.returncode != 0:
        raise ValueError(
            f"{owner} failed while recomputing the reviewed target vector: "
            f"exit={result.returncode}"
        )
    return _canonical_labels(result.stdout, owner=owner)


def _expected_test_targets(
    labels: Sequence[str],
    shard: str,
    shard_count: str,
) -> tuple[str, ...]:
    if not shard.isdecimal() or not shard_count.isdecimal():
        raise ValueError("test shard identity must be decimal")
    shard_number = int(shard)
    count = int(shard_count)
    if count != 4 or shard_number not in {1, 2, 3, 4}:
        raise ValueError("test target vector requires one of four canonical shards")

    ordered = sorted(labels, key=lambda value: value.encode("ascii"))
    selected = tuple(
        label
        for label in ordered
        if (posix_cksum(label.encode("ascii") + b"\n") % count) + 1
        == shard_number
    )
    if not selected:
        raise ValueError("reviewed test-target generator selected an empty shard")
    return selected


def _expected_clippy_targets(labels: Sequence[str]) -> tuple[str, ...]:
    dynamic = tuple(
        label for label in labels if not label.endswith("-test-bin")
    )
    expected = (*CLIPPY_TARGET_PREFIX, *dynamic)
    if len(expected) != len(set(expected)):
        raise ValueError("reviewed Clippy target generator produced duplicates")
    return expected


def _require_exact_targets(
    *,
    job: str,
    observed: Sequence[str],
    expected: Sequence[str],
) -> None:
    if tuple(observed) == tuple(expected):
        return
    raise ValueError(
        f"{job} target vector drifted from the reviewed generator: "
        f"expected_count={len(expected)}, observed_count={len(observed)}, "
        f"expected_sha256={_vector_digest(expected)}, "
        f"observed_sha256={_vector_digest(observed)}"
    )


def validate_keyless_windows_gnullvm_workspace_and_targets(
    command: Sequence[str],
    env: Mapping[str, str],
    *,
    run: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> Path:
    """Recompute exact target vectors and return the canonical launch cwd."""

    _require_transport_token_absent(
        env,
        owner="Q0.34 workspace and target authority",
    )
    workspace = _validate_bazelisk_inputs(env)
    job = _validate_runner_identity(env)
    startup, _command_name, _options, observed_targets = _split_command(command)

    executable = _verified_regular_file(
        Path(command[0]),
        owner="Q0.34 verified direct Bazel executable",
    )
    _require_cas_identity(executable)

    if job == TEST_JOB:
        labels = _query_labels(
            executable,
            startup,
            TEST_TARGET_QUERY,
            workspace,
            env,
            run=run,
            owner="reviewed Windows test-target query",
        )
        expected_targets = _expected_test_targets(
            labels,
            env.get("BAZEL_TEST_SHARD", ""),
            env.get("BAZEL_TEST_SHARD_COUNT", ""),
        )
        _require_exact_targets(
            job=job,
            observed=observed_targets,
            expected=expected_targets,
        )
    elif job == CLIPPY_JOB:
        labels = _query_labels(
            executable,
            startup,
            CLIPPY_TARGET_QUERY,
            workspace,
            env,
            run=run,
            owner="reviewed Windows Clippy-target query",
        )
        expected_targets = _expected_clippy_targets(labels)
        _require_exact_targets(
            job=job,
            observed=observed_targets,
            expected=expected_targets,
        )
    elif job != RELEASE_JOB:
        raise ValueError(f"unsupported Q0.34 job {job!r}")

    # The target query is an executable boundary too. Rehash the same CAS child
    # after the query and immediately before the caller starts the final build.
    observed_digest = digest_file(executable)
    if observed_digest != BAZEL_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "verified direct Bazel executable changed during target-vector "
            f"recomputation: expected {BAZEL_WINDOWS_X86_64_SHA256}, "
            f"observed {observed_digest}"
        )
    _require_transport_token_absent(
        env,
        owner="Q0.34 final workspace-bound launch",
    )
    return workspace
