"""Q0.30 cached-Bazel CAS and explicit output-base execution closure."""

from __future__ import annotations

import subprocess
from collections.abc import Callable, Mapping, MutableMapping, Sequence
from pathlib import Path

from run_bazel_q029_execution_context import BAZEL_WINDOWS_X86_64_SHA256
from run_bazel_q029_execution_context import _absolute_windows_path
from run_bazel_q029_execution_context import _require_env
from run_bazel_q029_execution_context import _require_path
from run_bazel_q029_execution_context import _sha256_file
from run_bazel_q029_execution_context import _split_command
from run_bazel_q029_execution_context import _validate_bazelisk_inputs
from run_bazel_q029_execution_context import _validate_paths
from run_bazel_q029_execution_context import _validate_runner_and_job

SETUP_BAZEL_TRANSPORT_TOKEN = "BAZELISK_GITHUB_TOKEN"
OUTPUT_BASE_PREFIX = "--output_base="
BAZEL_CAS_SUFFIX = (
    "downloads",
    "sha256",
    BAZEL_WINDOWS_X86_64_SHA256,
    "bin",
    "bazel.exe",
)


def clear_setup_bazel_transport_token(env: MutableMapping[str, str]) -> bool:
    """Remove setup-bazel's transport token before repository qualification."""

    return env.pop(SETUP_BAZEL_TRANSPORT_TOKEN, None) is not None


def bind_output_base_startup(
    command: Sequence[str],
    env: Mapping[str, str],
) -> list[str]:
    """Append one exact explicit output base after the Q0.29 startup vector."""

    if not command:
        raise ValueError("cannot bind output_base on an empty Bazel command")
    command_idx = next(
        (
            index
            for index, argument in enumerate(command[1:], start=1)
            if not argument.startswith("-")
        ),
        None,
    )
    if command_idx is None:
        raise ValueError("Bazel command verb is missing")
    startup = list(command[1:command_idx])
    observed = [
        argument for argument in startup if argument.startswith(OUTPUT_BASE_PREFIX)
    ]
    if observed:
        raise ValueError(
            "Q0.30 requires output_base to be absent before final startup binding; "
            f"observed {observed!r}"
        )
    output_base = _require_env(env, "BAZEL_OUTPUT_BASE")
    return [
        command[0],
        *startup,
        f"{OUTPUT_BASE_PREFIX}{output_base}",
        *command[command_idx:],
    ]


def _parse_print_env(stdout: str) -> str:
    path_values: list[str] = []
    for line in stdout.splitlines():
        name, separator, value = line.partition("=")
        if not separator:
            continue
        if name.casefold() == SETUP_BAZEL_TRANSPORT_TOKEN.casefold():
            raise ValueError(
                "Bazelisk --print_env retained the setup-only transport token"
            )
        if name.casefold() == "path":
            path_values.append(value)

    if len(path_values) != 1:
        raise ValueError(
            "Bazelisk --print_env must emit exactly one PATH binding; "
            f"observed {len(path_values)}"
        )
    path_value = path_values[0]
    first = path_value.split(";", 1)[0]
    if not first:
        raise ValueError("Bazelisk --print_env emitted an empty leading PATH entry")
    return path_value


def _validate_bazel_cas_path(executable: Path) -> None:
    observed = tuple(part.casefold() for part in executable.parts[-5:])
    expected = tuple(part.casefold() for part in BAZEL_CAS_SUFFIX)
    if observed != expected:
        raise ValueError(
            "cached Bazel path is outside the reviewed content-addressed layout: "
            f"{executable}"
        )


def resolve_verified_cached_bazel(
    command: Sequence[str],
    env: MutableMapping[str, str],
    *,
    run: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> list[str]:
    """Resolve Bazelisk's cached Bazel, rehash it, then bypass Bazelisk."""

    if not command:
        raise ValueError("cannot resolve cached Bazel from an empty command")
    if env.get(SETUP_BAZEL_TRANSPORT_TOKEN):
        raise ValueError(
            "setup-only Bazelisk transport token must be cleared before resolution"
        )

    bazelisk = Path(command[0])
    if not bazelisk.is_absolute():
        raise ValueError("verified Bazelisk path must be absolute before resolution")

    workspace = Path(_require_env(env, "GITHUB_WORKSPACE")).resolve(strict=True)
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
            f"exit={result.returncode}"
        )

    child_path = _parse_print_env(result.stdout)
    first_path = child_path.split(";", 1)[0]
    unresolved = Path(first_path) / "bazel.exe"
    if unresolved.is_symlink():
        raise ValueError("Bazelisk-resolved Bazel must not be a symlink")
    try:
        executable = unresolved.resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve cached Bazel executable: {error}") from error
    if not executable.is_file():
        raise ValueError("Bazelisk-resolved path is not a regular Bazel executable")

    _validate_bazel_cas_path(executable)
    observed = digest_file(executable)
    if observed != BAZEL_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "cached Bazel executable SHA-256 drifted: "
            f"expected {BAZEL_WINDOWS_X86_64_SHA256}, observed {observed}"
        )

    env["PATH"] = child_path
    env.pop(SETUP_BAZEL_TRANSPORT_TOKEN, None)
    env.pop("BAZEL_REAL", None)
    env.pop("BAZELISK", None)
    return [str(executable), *command[1:]]


def _validate_exact_startup(
    startup: Sequence[str],
    env: Mapping[str, str],
) -> None:
    workspace = Path(_require_env(env, "GITHUB_WORKSPACE")).resolve(strict=True)
    expected = [
        f"--output_user_root={_require_env(env, 'BAZEL_OUTPUT_USER_ROOT')}",
        "--noexperimental_remote_repo_contents_cache",
        "--nomaster_bazelrc",
        "--nosystem_rc",
        "--noworkspace_rc",
        "--nohome_rc",
        f"--bazelrc={workspace / '.bazelrc'}",
        f"{OUTPUT_BASE_PREFIX}{_require_env(env, 'BAZEL_OUTPUT_BASE')}",
    ]
    if list(startup) != expected:
        raise ValueError(
            "Q0.30 final startup arguments are not exact: "
            f"expected {expected!r}, observed {list(startup)!r}"
        )


def _validate_output_base(env: Mapping[str, str]) -> None:
    build_root = _absolute_windows_path(
        _require_env(env, "CI_BUILD_ROOT"),
        owner="CI_BUILD_ROOT",
    )
    _require_path(env, "BAZEL_OUTPUT_BASE", build_root / "o")


def _validate_child_path(executable: Path, env: Mapping[str, str]) -> None:
    path_value = _require_env(env, "PATH")
    first = path_value.split(";", 1)[0]
    if not first:
        raise ValueError("final PATH has an empty leading entry")
    try:
        first_path = Path(first).resolve(strict=True)
    except OSError as error:
        raise ValueError(f"cannot resolve final PATH head: {error}") from error
    if first_path != executable.parent:
        raise ValueError(
            "final PATH head is not the verified cached Bazel directory: "
            f"expected {executable.parent}, observed {first_path}"
        )


def validate_keyless_windows_gnullvm_cached_bazel_context(
    command: Sequence[str],
    env: Mapping[str, str],
    *,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> None:
    """Revalidate the complete Q0.30 command immediately before direct launch."""

    if env.get(SETUP_BAZEL_TRANSPORT_TOKEN):
        raise ValueError("setup-only Bazelisk transport token reached final launch")

    _validate_bazelisk_inputs(env)
    startup, command_name, options, targets = _split_command(command)
    if not targets:
        raise ValueError("qualification requires at least one Bazel target")

    job = _validate_runner_and_job(command_name, options, env)
    _validate_exact_startup(startup, env)
    _validate_paths(command_name, options, env, job)
    _validate_output_base(env)

    executable = Path(command[0])
    if not executable.is_absolute():
        raise ValueError("cached Bazel executable path must be absolute")
    if executable.is_symlink() or not executable.is_file():
        raise ValueError("cached Bazel executable must be a regular non-symlink file")
    _validate_bazel_cas_path(executable)
    observed = digest_file(executable)
    if observed != BAZEL_WINDOWS_X86_64_SHA256:
        raise ValueError(
            "cached Bazel executable SHA-256 drifted before launch: "
            f"expected {BAZEL_WINDOWS_X86_64_SHA256}, observed {observed}"
        )
    _validate_child_path(executable, env)
