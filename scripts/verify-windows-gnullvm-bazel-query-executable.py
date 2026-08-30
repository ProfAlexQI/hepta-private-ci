#!/usr/bin/env python3
"""Verify the exact Bazelisk and direct Bazel used by the Q0.40 query smoke."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import os
import shutil
import subprocess
import sys
from collections.abc import Callable, Mapping, MutableMapping
from pathlib import Path
from tempfile import TemporaryDirectory
from typing import Final


ROOT = Path(__file__).resolve().parents[1]
BASE_PATH = ROOT / "scripts" / "verify-windows-gnullvm-bazel-query-vector.py"
SPEC = importlib.util.spec_from_file_location("_hepta_q040_query_vector_base", BASE_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot load {BASE_PATH}")
BASE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = BASE
SPEC.loader.exec_module(BASE)

BAZEL_VERSION: Final = "9.0.0"
BAZELISK_VERSION: Final = "1.28.1"
BAZELISK_LINUX_X86_64_SHA256: Final = (
    "22e7d3a188699982f661cf4687137ee52d1f24fec1ec893d91a6c4d791a75de8"
)
BAZEL_LINUX_X86_64_SHA256: Final = (
    "c44a93f25398c68f904fa1d19b61d321de6c0d2f09dca375d7bc0dc9b9428403"
)
TRANSPORT_TOKEN: Final = "BAZELISK_GITHUB_TOKEN"
REQUIRED_BAZELISK_ENV: Final = {
    "USE_BAZEL_VERSION": BAZEL_VERSION,
    "BAZELISK_VERIFY_SHA256": BAZEL_LINUX_X86_64_SHA256,
    "BAZELISK_SKIP_WRAPPER": "true",
}
FORBIDDEN_BAZELISK_ENV: Final = {
    "BAZELISK",
    "BAZEL_REAL",
    "BAZELISK_BASE_URL",
    "BAZELISK_FORMAT_URL",
    "BAZELISK_HOME",
    "BAZELISK_HOME_LINUX",
    "BAZELISK_NOJDK",
    "USE_BAZEL_FALLBACK_VERSION",
}
PASS_SOURCE: Final = "PASS_WINDOWS_GNULLVM_Q0_40_QUERY_EXECUTABLE_SOURCE"
PASS_EXECUTED: Final = "PASS_WINDOWS_GNULLVM_Q0_40_DIRECT_BAZEL_QUERY_EXECUTED"


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def _sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _scrub_transport_token(env: MutableMapping[str, str]) -> None:
    for name in list(env):
        if name.casefold() == TRANSPORT_TOKEN.casefold():
            env.pop(name, None)


def _verified_regular_file(path: Path, *, owner: str) -> Path:
    require(not path.is_symlink(), f"{owner} must not be a symlink")
    try:
        resolved = path.resolve(strict=True)
    except OSError as error:
        fail(f"cannot resolve {owner}: {error}")
    require(resolved.is_file(), f"{owner} must be a regular file")
    return resolved


def _validate_bazelisk_inputs(
    env: MutableMapping[str, str],
    workspace: Path,
) -> None:
    _scrub_transport_token(env)
    for name in sorted(FORBIDDEN_BAZELISK_ENV):
        require(not env.get(name), f"Bazelisk override {name} is forbidden")
    for name, expected in REQUIRED_BAZELISK_ENV.items():
        observed = env.get(name)
        require(
            observed in {None, "", expected},
            f"Bazelisk override {name} conflicts with required value {expected!r}",
        )
        env[name] = expected

    candidates = [workspace / ".bazeliskrc"]
    home = env.get("HOME") or env.get("USERPROFILE")
    if home:
        candidates.append(Path(home) / ".bazeliskrc")
    for candidate in candidates:
        require(
            not candidate.exists() and not candidate.is_symlink(),
            f"Bazelisk config file is forbidden: {candidate}",
        )

    wrapper_dir = workspace / "tools"
    if wrapper_dir.exists():
        require(
            not any(wrapper_dir.glob("bazel*")),
            "workspace Bazel wrapper surface is forbidden",
        )


def _parse_bazelisk_child_path(stdout: str) -> str:
    paths: list[str] = []
    for line in stdout.splitlines():
        name, separator, value = line.partition("=")
        if not separator:
            continue
        folded = name.casefold()
        require(
            folded != TRANSPORT_TOKEN.casefold(),
            "Bazelisk --print_env emitted the setup-only transport token",
        )
        if folded == "path":
            paths.append(value)
    require(
        len(paths) == 1,
        "Bazelisk --print_env must emit exactly one PATH binding; "
        f"observed {len(paths)}",
    )
    child_path = paths[0]
    leading = child_path.split(os.pathsep, 1)[0]
    require(
        bool(leading),
        "Bazelisk --print_env emitted an empty leading PATH entry",
    )
    require(
        Path(leading).is_absolute(),
        "Bazelisk --print_env emitted a non-absolute leading PATH entry",
    )
    return child_path


def _require_bazel_cas_identity(path: Path) -> None:
    expected = (
        "downloads",
        "sha256",
        BAZEL_LINUX_X86_64_SHA256,
        "bin",
        "bazel",
    )
    require(
        tuple(path.parts[-5:]) == expected,
        "cached Bazel path is outside the reviewed content-addressed layout: "
        f"{path}",
    )


def _direct_environment(
    resolver_env: Mapping[str, str],
    child_path: str,
) -> dict[str, str]:
    direct = dict(resolver_env)
    for name in list(direct):
        folded = name.casefold()
        if (
            folded == TRANSPORT_TOKEN.casefold()
            or folded.startswith("bazelisk_")
            or folded in {"bazelisk", "bazel_real", "use_bazel_version"}
        ):
            direct.pop(name, None)
    direct["PATH"] = child_path
    return direct


def _validate_direct_environment(env: Mapping[str, str]) -> None:
    for name, value in env.items():
        folded = name.casefold()
        require(
            folded != TRANSPORT_TOKEN.casefold() or not value,
            "setup-only transport token reached direct Bazel",
        )
        require(
            not folded.startswith("bazelisk_"),
            f"Bazelisk control reached direct Bazel: {name}",
        )
        require(
            folded not in {"bazelisk", "bazel_real", "use_bazel_version"},
            f"Bazelisk executable/version override reached direct Bazel: {name}",
        )


def resolve_verified_linux_bazel(
    workspace: Path,
    *,
    base_env: Mapping[str, str] | None = None,
    which: Callable[..., str | None] = shutil.which,
    run: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> tuple[Path, dict[str, str]]:
    resolver_env = dict(os.environ if base_env is None else base_env)
    _validate_bazelisk_inputs(resolver_env, workspace)

    bazelisk_value = which("bazel", path=resolver_env.get("PATH"))
    require(bool(bazelisk_value), "pinned Bazelisk executable was not found on PATH")
    bazelisk = _verified_regular_file(
        Path(str(bazelisk_value)),
        owner="Bazelisk executable",
    )
    observed_bazelisk = digest_file(bazelisk)
    require(
        observed_bazelisk == BAZELISK_LINUX_X86_64_SHA256,
        "Bazelisk executable SHA-256 drifted: "
        f"expected {BAZELISK_LINUX_X86_64_SHA256}, "
        f"observed {observed_bazelisk}",
    )

    result = run(
        [str(bazelisk), "--print_env"],
        cwd=workspace,
        env=dict(resolver_env),
        capture_output=True,
        text=True,
        check=False,
        timeout=300,
    )
    require(
        result.returncode == 0,
        "Bazelisk failed to resolve the pinned Bazel binary: "
        f"exit={result.returncode}",
    )
    require(
        digest_file(bazelisk) == BAZELISK_LINUX_X86_64_SHA256,
        "Bazelisk executable changed during child resolution",
    )

    child_path = _parse_bazelisk_child_path(result.stdout)
    leading = Path(child_path.split(os.pathsep, 1)[0])
    bazel = _verified_regular_file(
        leading / "bazel",
        owner="cached Bazel executable",
    )
    _require_bazel_cas_identity(bazel)
    observed_bazel = digest_file(bazel)
    require(
        observed_bazel == BAZEL_LINUX_X86_64_SHA256,
        "cached Bazel executable SHA-256 drifted: "
        f"expected {BAZEL_LINUX_X86_64_SHA256}, observed {observed_bazel}",
    )

    direct_env = _direct_environment(resolver_env, child_path)
    _validate_direct_environment(direct_env)
    return bazel, direct_env


def _validate_direct_bazel(
    bazel: Path,
    env: Mapping[str, str],
    *,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> Path:
    resolved = _verified_regular_file(bazel, owner="direct Bazel executable")
    _require_bazel_cas_identity(resolved)
    require(
        digest_file(resolved) == BAZEL_LINUX_X86_64_SHA256,
        "direct Bazel executable changed before parser launch",
    )
    _validate_direct_environment(env)
    leading_value = env.get("PATH", "").split(os.pathsep, 1)[0]
    require(bool(leading_value), "direct Bazel PATH has an empty head")
    try:
        leading = Path(leading_value).resolve(strict=True)
    except OSError as error:
        fail(f"cannot resolve direct Bazel PATH head: {error}")
    require(
        leading == resolved.parent,
        "direct Bazel PATH head is not the verified CAS directory",
    )
    return resolved


def execute_parser_smoke(
    *,
    base_env: Mapping[str, str] | None = None,
    which: Callable[..., str | None] = shutil.which,
    run: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
    digest_file: Callable[[Path], str] = _sha256_file,
) -> None:
    with TemporaryDirectory(prefix="hepta-q040-query-") as temporary:
        workspace = Path(temporary).resolve()
        (workspace / "MODULE.bazel").write_text(
            'module(name = "hepta_q040_query_probe", version = "1.0")\n',
            encoding="utf-8",
        )
        (workspace / "BUILD.bazel").write_text(
            'filegroup(name = "probe", srcs = [])\n',
            encoding="utf-8",
        )
        (workspace / "empty.bazelrc").write_text("", encoding="utf-8")

        bazel, direct_env = resolve_verified_linux_bazel(
            workspace,
            base_env=base_env,
            which=which,
            run=run,
            digest_file=digest_file,
        )
        bazel = _validate_direct_bazel(
            bazel,
            direct_env,
            digest_file=digest_file,
        )
        command = BASE.parser_smoke_command(bazel, workspace)
        result = run(
            command,
            cwd=workspace,
            env=direct_env,
            capture_output=True,
            text=True,
            check=False,
            timeout=300,
        )
        require(
            result.returncode == 0,
            f"direct Bazel {BAZEL_VERSION} query parser rejected the canonical "
            f"vector: exit={result.returncode}",
        )
        observed = [line for line in result.stdout.splitlines() if line]
        require(
            observed == ["//:probe"],
            f"direct Bazel query smoke returned a noncanonical label set: {observed!r}",
        )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--execute",
        action="store_true",
        help="execute the verified direct Bazel 9 parser smoke",
    )
    args = parser.parse_args()

    BASE.validate_source()
    if args.execute:
        execute_parser_smoke()
        print(PASS_EXECUTED)
    else:
        print(PASS_SOURCE)


if __name__ == "__main__":
    main()
