"""Q0.30 transport-token and cached-Bazel executable ratchet."""

from __future__ import annotations

import subprocess
from collections.abc import Callable, Mapping, MutableMapping, Sequence
from pathlib import Path

from run_bazel_q029_execution_context import BAZELISK_WINDOWS_X86_64_SHA256
from run_bazel_q029_execution_context import BAZEL_WINDOWS_X86_64_SHA256
from run_bazel_q029_execution_context import _sha256_file
from run_bazel_q029_execution_context import (
    prepare_bazelisk_environment as _prepare_q029,
)

BAZELISK_TRANSPORT_TOKEN_ENV = "BAZELISK_GITHUB_TOKEN"

# Immutable upstream provenance for the compatibility exception and the
# cached-Bazel rehash. setup-bazel exports its input token under the transport
# variable below; Bazelisk v1.28.1 accepts a metadata->CAS cache hit before its
# BAZELISK_VERIFY_SHA256 download-only check.
SETUP_BAZEL_ACTION_COMMIT = "c5acdfb288317d0b5c0bbd7a396a3dc868bb0f86"
SETUP_BAZEL_CONFIG_JS_BLOB = "92bb7cd0077d8958b1bbca368a25169971d7a8d3"
BAZELISK_SOURCE_COMMIT = "1e6aaf11d51e83ec8d18e66b461f49d4b7877321"
BAZELISK_CORE_GO_BLOB = "15b131a22fc28377d3cc3d70ac602123d1530c08"


def prepare_bazelisk_environment(env: MutableMapping[str, str]) -> None:
    """Run Q0.29 while preserving setup-bazel's transport-only token."""

    missing = object()
    token = env.pop(BAZELISK_TRANSPORT_TOKEN_ENV, missing)
    try:
        _prepare_q029(env)
    finally:
        if token is missing:
            env.pop(BAZELISK_TRANSPORT_TOKEN_ENV, None)
        else:
            env[BAZELISK_TRANSPORT_TOKEN_ENV] = token


def _parse_bazelisk_child_path(stdout: str) -> str:
    path_values = []
    for line in stdout.splitlines():
        name, separator, value = line.partition("=")
        if separator and name.casefold() == "path":
            path_values.append(value)
    if len(path_values) != 1:
        raise ValueError(
            "Bazelisk --print_env must emit exactly one PATH binding; "
            f"observed {len(path_values)}"
        )
    path_value = path_values[0]
    if not path_value.split(";", 1)[0]:
        raise ValueError("Bazelisk --print_env emitted an empty leading PATH entry")
    return path_value


def _require_cached_bazel(
    executable: Path,
    *,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> Path:
    if not executable.is_absolute():
        raise ValueError("cached Bazel executable path must be absolute")
    if executable.is_symlink():
        raise ValueError("cached Bazel executable must not be a symlink")
    try:
        canonical = executable.resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve cached Bazel executable: {error}") from error
    if not canonical.is_file():
        raise ValueError("cached Bazel executable must be a regular file")

    observed = digest_file(canonical)
    if observed != BAZEL_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "cached Bazel executable SHA-256 drifted: "
            f"expected {BAZEL_WINDOWS_X86_64_SHA256}, observed {observed}"
        )

    expected_suffix = (
        "downloads",
        "sha256",
        BAZEL_WINDOWS_X86_64_SHA256,
        "bin",
        "bazel.exe",
    )
    observed_suffix = tuple(part.casefold() for part in canonical.parts[-5:])
    if observed_suffix != tuple(part.casefold() for part in expected_suffix):
        raise ValueError(
            "cached Bazel executable is outside the reviewed Bazelisk CAS: "
            f"{canonical}"
        )
    return canonical


def resolve_verified_bazel_command(
    command: Sequence[str],
    env: MutableMapping[str, str],
    *,
    run: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> list[str]:
    """Resolve Bazelisk's child, rehash the cache hit, and launch Bazel directly."""

    if not command:
        raise ValueError("cannot resolve an empty Bazel command")
    bazelisk = Path(command[0])
    if not bazelisk.is_absolute():
        raise ValueError("verified Bazelisk path must be absolute")
    if bazelisk.is_symlink() or not bazelisk.is_file():
        raise ValueError("verified Bazelisk must remain a regular non-symlink file")
    observed_bazelisk = digest_file(bazelisk)
    if observed_bazelisk != BAZELISK_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "Bazelisk executable SHA-256 drifted before child resolution: "
            f"expected {BAZELISK_WINDOWS_X86_64_SHA256}, "
            f"observed {observed_bazelisk}"
        )

    workspace_value = env.get("GITHUB_WORKSPACE")
    if not workspace_value:
        raise ValueError("GITHUB_WORKSPACE is required for Bazelisk child resolution")
    try:
        workspace = Path(workspace_value).resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve GITHUB_WORKSPACE: {error}") from error
    if not workspace.is_dir():
        raise ValueError("GITHUB_WORKSPACE must resolve to a directory")

    result = run(
        [str(bazelisk), "--print_env"],
        cwd=workspace,
        env=dict(env),
        capture_output=True,
        text=True,
        check=False,
        timeout=180,
    )
    if result.returncode != 0:
        raise ValueError(
            "Bazelisk failed to resolve the pinned Bazel executable: "
            f"exit={result.returncode}, stderr={result.stderr.strip()!r}"
        )

    child_path = _parse_bazelisk_child_path(result.stdout)
    real_dir = Path(child_path.split(";", 1)[0])
    if not real_dir.is_absolute():
        raise ValueError("Bazelisk child PATH must lead with an absolute directory")
    real_bazel = _require_cached_bazel(
        real_dir / "bazel.exe",
        digest_file=digest_file,
    )

    # Match Bazelisk's child PATH so nested Bazel invocations resolve to the
    # same independently verified executable, then bypass Bazelisk for launch.
    env["PATH"] = child_path
    env.pop(BAZELISK_TRANSPORT_TOKEN_ENV, None)
    return [str(real_bazel), *command[1:]]


def validate_verified_bazel_prelaunch(
    command: Sequence[str],
    env: Mapping[str, str],
    *,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> None:
    """Rehash the exact Bazel executable immediately before process launch."""

    if not command:
        raise ValueError("cannot validate an empty Bazel command")
    executable = _require_cached_bazel(Path(command[0]), digest_file=digest_file)
    path_value = env.get("PATH")
    if not path_value:
        raise ValueError("verified Bazel child PATH is missing")
    if env.get(BAZELISK_TRANSPORT_TOKEN_ENV):
        raise ValueError("Bazelisk transport token survived into the Bazel launch")
    leading_path = Path(path_value.split(";", 1)[0])
    if not leading_path.is_absolute():
        raise ValueError(
            "verified Bazel child PATH must lead with an absolute directory"
        )
    try:
        canonical_leading_path = leading_path.resolve(strict=True)
    except OSError as error:
        raise ValueError(
            f"cannot resolve verified Bazel PATH entry: {error}"
        ) from error
    if canonical_leading_path != executable.parent:
        raise ValueError(
            "verified Bazel child PATH does not lead with the executable directory"
        )
