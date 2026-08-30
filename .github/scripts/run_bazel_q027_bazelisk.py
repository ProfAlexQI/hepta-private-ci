"""Bazelisk and cached-Bazel verification for Q0.27."""

from __future__ import annotations

import shutil
import subprocess
from collections.abc import Callable, Mapping, MutableMapping, Sequence
from pathlib import Path

from run_bazel_q027_common import BAZELISK_FORBIDDEN_ENV
from run_bazel_q027_common import BAZELISK_REQUIRED_ENV
from run_bazel_q027_common import BAZELISK_WINDOWS_X86_64_SHA256
from run_bazel_q027_common import BAZELVERSION_BYTES
from run_bazel_q027_common import BAZELVERSION_GIT_BLOB_SHA1
from run_bazel_q027_common import BAZEL_WINDOWS_X86_64_SHA256
from run_bazel_q027_common import _git_blob_sha1
from run_bazel_q027_common import _require_env
from run_bazel_q027_common import _sha256_file
from run_bazel_q027_common import _validate_environment_roots
from run_bazel_q027_common import _validate_runner_identity

def prepare_bazelisk_environment(env: MutableMapping[str, str]) -> None:
    """Validate runtime identity and install exact Bazelisk controls."""
    for name in (
        "GITHUB_REPOSITORY",
        "GITHUB_JOB",
        "GITHUB_SHA",
        "GITHUB_RUN_ID",
        "GITHUB_WORKSPACE",
        "RUNNER_ENVIRONMENT",
        "RUNNER_ARCH",
        "RUNNER_TEMP",
        "CI_BUILD_ROOT",
        "BAZEL_OUTPUT_BASE",
        "BAZEL_OUTPUT_USER_ROOT",
        "BAZEL_REPOSITORY_CACHE",
        "BAZEL_REPO_CONTENTS_CACHE",
        "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR",
        "CODEX_BAZEL_WINDOWS_PATH",
    ):
        _require_env(env, name)
    if env.get("CODEX_BAZEL_BIN"):
        raise ValueError("CODEX_BAZEL_BIN is forbidden in qualifying GitHub jobs")
    for name in sorted(BAZELISK_FORBIDDEN_ENV):
        if env.get(name):
            raise ValueError(f"Bazelisk override {name} is forbidden")
    job = _validate_runner_identity(env)
    _validate_environment_roots(env, job)
    for name, expected in BAZELISK_REQUIRED_ENV.items():
        observed = env.get(name)
        if observed not in {None, "", expected}:
            raise ValueError(
                f"Bazelisk override {name} conflicts with required value {expected!r}"
            )
        env[name] = expected


def _validate_bazelisk_configuration(env: Mapping[str, str]) -> Path:
    if env.get("CODEX_BAZEL_BIN"):
        raise ValueError("CODEX_BAZEL_BIN is forbidden in qualifying GitHub jobs")
    for name, expected in BAZELISK_REQUIRED_ENV.items():
        if env.get(name) != expected:
            raise ValueError(f"{name} must equal {expected!r}")
    for name in sorted(BAZELISK_FORBIDDEN_ENV):
        if env.get(name):
            raise ValueError(f"Bazelisk override {name} is forbidden")

    workspace = Path(_require_env(env, "GITHUB_WORKSPACE")).resolve(strict=True)
    bazelversion = workspace / ".bazelversion"
    if bazelversion.is_symlink() or not bazelversion.is_file():
        raise ValueError(".bazelversion must be a regular non-symlink file")
    data = bazelversion.read_bytes()
    if data != BAZELVERSION_BYTES:
        raise ValueError(".bazelversion bytes drifted from Bazel 9.0.0")
    if _git_blob_sha1(data) != BAZELVERSION_GIT_BLOB_SHA1:
        raise ValueError(".bazelversion Git blob identity drifted")

    bazelisk_rcs = [workspace / ".bazeliskrc"]
    for name in ("USERPROFILE", "HOME"):
        value = env.get(name)
        if value:
            bazelisk_rcs.append(Path(value) / ".bazeliskrc")
    for path in bazelisk_rcs:
        if path.exists() or path.is_symlink():
            raise ValueError(f"Bazelisk config file is forbidden: {path}")
    return workspace


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
    value = path_values[0]
    first = value.split(";", 1)[0]
    if not first:
        raise ValueError("Bazelisk --print_env emitted an empty leading PATH entry")
    return value


def resolve_verified_bazel_command(
    command: Sequence[str],
    env: MutableMapping[str, str],
    *,
    which: Callable[..., str | None] = shutil.which,
    run: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> list[str]:
    """Resolve Bazelisk, verify its cached Bazel, then execute Bazel directly."""
    workspace = _validate_bazelisk_configuration(env)
    resolved = which("bazel", path=env.get("PATH"))
    if not resolved:
        raise ValueError("official Bazelisk executable was not found on PATH")
    unresolved = Path(resolved)
    if unresolved.is_symlink():
        raise ValueError("Bazelisk PATH entry must not be a symlink")
    try:
        bazelisk = unresolved.resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve Bazelisk executable: {error}") from error
    if not bazelisk.is_file():
        raise ValueError("Bazelisk PATH entry is not a regular file")
    observed_bazelisk = digest_file(bazelisk)
    if observed_bazelisk != BAZELISK_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "Bazelisk executable SHA-256 drifted: "
            f"expected {BAZELISK_WINDOWS_X86_64_SHA256}, "
            f"observed {observed_bazelisk}"
        )

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
            "Bazelisk failed to resolve the pinned Bazel binary: "
            f"exit={result.returncode}, stderr={result.stderr.strip()!r}"
        )
    child_path = _parse_bazelisk_child_path(result.stdout)
    real_dir = Path(child_path.split(";", 1)[0])
    candidate = real_dir / "bazel.exe"
    if candidate.is_symlink():
        raise ValueError("Bazelisk resolved Bazel must not be a symlink")
    try:
        real_bazel = candidate.resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve cached Bazel executable: {error}") from error
    if not real_bazel.is_file():
        raise ValueError("Bazelisk resolved path is not a regular Bazel executable")
    observed_bazel = digest_file(real_bazel)
    if observed_bazel != BAZEL_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "cached Bazel executable SHA-256 drifted: "
            f"expected {BAZEL_WINDOWS_X86_64_SHA256}, observed {observed_bazel}"
        )
    expected_suffix = (
        "downloads",
        "sha256",
        BAZEL_WINDOWS_X86_64_SHA256,
        "bin",
        "bazel.exe",
    )
    if tuple(part.casefold() for part in real_bazel.parts[-5:]) != tuple(
        part.casefold() for part in expected_suffix
    ):
        raise ValueError(
            "cached Bazel path is outside the Bazelisk content-addressed store: "
            f"{real_bazel}"
        )

    # Match Bazelisk's child environment so nested `bazel` calls resolve to the
    # same independently verified binary, then bypass Bazelisk for this launch.
    env["PATH"] = child_path
    return [str(real_bazel), *command[1:]]


