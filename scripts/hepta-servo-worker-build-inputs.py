#!/usr/bin/env python3
"""Canonical fail-closed entrypoint for Servo worker build-input freezing.

This wrapper narrows command, feature and environment semantics before delegating
serialization and recomputation to `hepta-servo-worker-build-manifest.py`.
"""
from __future__ import annotations

import importlib.util
import pathlib
import sys
from types import ModuleType
from typing import Sequence

ROOT = pathlib.Path(__file__).resolve().parents[1]
ENGINE_PATH = ROOT / "scripts/hepta-servo-worker-build-manifest.py"
FORBIDDEN_CARGO_OPERATIONS = {
    "fetch",
    "generate-lockfile",
    "install",
    "login",
    "owner",
    "package",
    "publish",
    "search",
    "update",
    "vendor",
    "yank",
}
SECRET_KEY_MARKERS = (
    "TOKEN",
    "SECRET",
    "PASSWORD",
    "PASSWD",
    "CREDENTIAL",
    "AUTH",
    "COOKIE",
    "PRIVATE",
    "SSH",
    "AWS",
    "AZURE",
    "GITHUB",
    "OPENAI",
)


def load_engine() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_worker_build_manifest_engine",
        ENGINE_PATH,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load canonical Servo build-input engine")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


def option(argv: Sequence[str], name: str) -> str:
    positions = [index for index, item in enumerate(argv) if item == name]
    if len(positions) != 1 or positions[0] + 1 >= len(argv):
        raise RuntimeError(f"{name} must occur exactly once with one value")
    return argv[positions[0] + 1]


def repeated(argv: Sequence[str], name: str) -> list[str]:
    values: list[str] = []
    index = 0
    while index < len(argv):
        if argv[index] == name:
            if index + 1 >= len(argv):
                raise RuntimeError(f"{name} is missing its value")
            values.append(argv[index + 1])
            index += 2
        else:
            index += 1
    return values


def validate_command(engine: ModuleType, path: pathlib.Path) -> None:
    value, _ = engine.load_json(path, "Servo worker build command")
    arguments = value.get("argv")
    if not isinstance(arguments, list) or len(arguments) < 4:
        raise RuntimeError("build argv is incomplete")
    for item in arguments:
        if (
            not isinstance(item, str)
            or not item
            or any(character in item for character in "\0\n\r")
        ):
            raise RuntimeError("build argv contains an empty or ambiguous item")
    executable = pathlib.PurePath(arguments[0]).name.lower()
    if executable not in {"cargo", "cargo.exe"}:
        raise RuntimeError("build command must invoke Cargo directly")
    if arguments[1] not in {"build", "rustc"}:
        raise RuntimeError("build command must use cargo build or cargo rustc")
    if any(item in FORBIDDEN_CARGO_OPERATIONS for item in arguments[1:]):
        raise RuntimeError("build command contains a registry or acquisition operation")
    if "--locked" not in arguments:
        raise RuntimeError("build command must include --locked")
    if "--offline" not in arguments:
        raise RuntimeError("build command must include --offline")
    if value.get("network_access_during_build") is not False:
        raise RuntimeError("build command attempted to enable network access")


def validate_environment(engine: ModuleType, path: pathlib.Path) -> None:
    value, _ = engine.load_json(path, "Servo worker build environment")
    variables = value.get("variables")
    if not isinstance(variables, dict):
        raise RuntimeError("build environment variables must be an object")
    for key, item in variables.items():
        if any(marker in key for marker in SECRET_KEY_MARKERS):
            raise RuntimeError(
                f"build environment key {key!r} is secret- or identity-bearing"
            )
        if (
            not isinstance(item, str)
            or any(character in item for character in "\0\n\r")
        ):
            raise RuntimeError(f"build environment value for {key!r} is ambiguous")
    if variables.get("CARGO_NET_OFFLINE") not in {"true", "1"}:
        raise RuntimeError("CARGO_NET_OFFLINE must be true")
    if value.get("network_access_during_build") is not False:
        raise RuntimeError("build environment attempted to enable network access")


def validate_invocation(
    argv: Sequence[str],
    engine: ModuleType | None = None,
) -> None:
    if not argv or argv[0] not in {"create", "verify"}:
        raise RuntimeError("command must be create or verify")
    engine = engine or load_engine()
    features = repeated(argv, "--feature")
    if len(features) != len(set(features)):
        raise RuntimeError("duplicate Cargo features are forbidden")
    validate_command(engine, pathlib.Path(option(argv, "--build-command")))
    validate_environment(engine, pathlib.Path(option(argv, "--environment")))


def main(argv: Sequence[str] | None = None) -> int:
    arguments = list(sys.argv[1:] if argv is None else argv)
    try:
        engine = load_engine()
        validate_invocation(arguments, engine)
        return int(engine.main(arguments))
    except (OSError, RuntimeError) as error:
        print(f"HEPTA_SERVO_WORKER_BUILD_INPUTS=FAIL: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
