"""Q0.30/Q0.32 direct Bazel CAS, token, and launch authority."""

from __future__ import annotations

import os
import shutil
import subprocess
from collections.abc import Callable, Mapping, MutableMapping, Sequence
from pathlib import Path

from run_bazel_q028_startup_contract import (
    validate_keyless_windows_gnullvm_final_args as _validate_q028,
)
from run_bazel_q029_job_executable import (
    BAZELISK_WINDOWS_X86_64_SHA256,
    BAZEL_WINDOWS_X86_64_SHA256,
    _sha256_file,
    _split_command,
    _validate_bazelisk_inputs,
    _validate_job_binding,
    _validate_paths,
    _validate_runner_identity,
    prepare_bazelisk_environment as _prepare_q029,
)

BAZELISK_BARE_OVERRIDE = "BAZELISK"
SETUP_BAZEL_TRANSPORT_TOKEN = "BAZELISK_GITHUB_TOKEN"


def prepare_bazelisk_environment(env: MutableMapping[str, str]) -> None:
    """Compose Q0.29 and reject the remaining bare Bazelisk override."""

    if env.get(BAZELISK_BARE_OVERRIDE):
        raise ValueError("Bazelisk override BAZELISK is forbidden")
    _prepare_q029(env)


def _verified_regular_file(path: Path, *, owner: str) -> Path:
    if path.is_symlink():
        raise ValueError(f"{owner} must not be a symlink")
    try:
        resolved = path.resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve {owner}: {error}") from error
    if not resolved.is_file():
        raise ValueError(f"{owner} must be a regular file")
    return resolved


def _parse_bazelisk_child_path(stdout: str) -> str:
    path_values: list[str] = []
    for line in stdout.splitlines():
        name, separator, value = line.partition("=")
        if not separator:
            continue
        if name.casefold() == SETUP_BAZEL_TRANSPORT_TOKEN.casefold():
            raise ValueError(
                "Bazelisk --print_env leaked the setup-only transport token"
            )
        if name.casefold() == "path":
            path_values.append(value)
    if len(path_values) != 1:
        raise ValueError(
            "Bazelisk --print_env must emit exactly one PATH binding; "
            f"observed {len(path_values)}"
        )
    child_path = path_values[0]
    leading = child_path.split(";", 1)[0]
    if not leading:
        raise ValueError("Bazelisk --print_env emitted an empty leading PATH entry")
    return child_path


def _require_cas_identity(path: Path) -> None:
    expected_suffix = (
        "downloads",
        "sha256",
        BAZEL_WINDOWS_X86_64_SHA256,
        "bin",
        "bazel.exe",
    )
    expected = tuple(part.casefold() for part in expected_suffix)
    observed = tuple(part.casefold() for part in path.parts[-5:])
    if observed != expected:
        raise ValueError(
            "cached Bazel path is outside the expected Bazelisk "
            f"content-addressed store: {path}"
        )


def resolve_verified_bazel_command(
    command: Sequence[str],
    env: MutableMapping[str, str],
    *,
    which: Callable[..., str | None] = shutil.which,
    run: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> list[str]:
    """Verify Bazelisk, resolve its CAS child, and return direct Bazel argv."""

    if not command or command[0].casefold() not in {"bazel", "bazel.exe"}:
        raise ValueError("unverified Bazel argv[0] is forbidden")

    workspace = _validate_bazelisk_inputs(env)
    resolved = which("bazel", path=env.get("PATH"))
    if not resolved:
        raise ValueError("official Bazelisk executable was not found on PATH")
    bazelisk = _verified_regular_file(Path(resolved), owner="Bazelisk executable")
    observed_bazelisk = digest_file(bazelisk)
    if observed_bazelisk != BAZELISK_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "Bazelisk executable SHA-256 drifted: "
            f"expected {BAZELISK_WINDOWS_X86_64_SHA256}, "
            f"observed {observed_bazelisk}"
        )

    # The setup action's token may authenticate the public Bazelisk release
    # lookup, but it must exist only in this private resolver environment.
    resolver_env = dict(env)
    env.pop(SETUP_BAZEL_TRANSPORT_TOKEN, None)
    try:
        result = run(
            [str(bazelisk), "--print_env"],
            cwd=workspace,
            env=resolver_env,
            capture_output=True,
            text=True,
            check=False,
            timeout=180,
        )
    finally:
        resolver_env.pop(SETUP_BAZEL_TRANSPORT_TOKEN, None)
    if result.returncode != 0:
        raise ValueError(
            "Bazelisk failed to resolve the pinned Bazel binary: "
            f"exit={result.returncode}, stderr={result.stderr.strip()!r}"
        )
    if digest_file(bazelisk) != BAZELISK_WINDOWS_X86_64_SHA256:
        raise ValueError("Bazelisk executable changed during child resolution")

    child_path = _parse_bazelisk_child_path(result.stdout)
    real_dir_value = child_path.split(";", 1)[0]
    real_bazel = _verified_regular_file(
        Path(real_dir_value) / "bazel.exe",
        owner="cached Bazel executable",
    )
    _require_cas_identity(real_bazel)
    observed_bazel = digest_file(real_bazel)
    if observed_bazel != BAZEL_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "cached Bazel executable SHA-256 drifted: "
            f"expected {BAZEL_WINDOWS_X86_64_SHA256}, "
            f"observed {observed_bazel}"
        )

    # Preserve Bazelisk's verified child PATH so nested `bazel` invocations
    # resolve to the same CAS object while this launch bypasses Bazelisk.
    env["PATH"] = child_path
    return [str(real_bazel), *command[1:]]


def validate_keyless_windows_gnullvm_command(
    command: Sequence[str],
    env: Mapping[str, str],
    *,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> None:
    """Validate and rehash the direct Bazel command immediately before launch."""

    if env.get(SETUP_BAZEL_TRANSPORT_TOKEN):
        raise ValueError(
            "setup-bazel transport token reached direct Bazel launch"
        )

    _validate_bazelisk_inputs(env)
    job = _validate_runner_identity(env)
    startup, command_name, options, targets = _split_command(command)

    executable = Path(command[0])
    if not executable.is_absolute():
        raise ValueError("verified direct Bazel executable path must be absolute")
    real_bazel = _verified_regular_file(
        executable,
        owner="verified direct Bazel executable",
    )
    _require_cas_identity(real_bazel)
    observed_bazel = digest_file(real_bazel)
    if observed_bazel != BAZEL_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "verified direct Bazel executable changed before launch: "
            f"expected {BAZEL_WINDOWS_X86_64_SHA256}, observed {observed_bazel}"
        )

    _validate_q028(command[1:], env)
    _validate_paths(options, env, job)
    _validate_job_binding(command_name, options, targets, env, job)
