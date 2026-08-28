#!/usr/bin/env python3
"""Canonical v2 entrypoint for the Linux Servo build preflight.

The v1 core performs byte and receipt cross-binding. This entrypoint additionally
requires the toolchain receipt to carry exactly the frozen version-only commands,
minimal capture environment, linker allowlist, and path-free printable version
text before a build can be considered ready.
"""

from __future__ import annotations

import importlib.util
import re
import sys
from pathlib import Path
from types import ModuleType
from typing import Any

BASE_SCRIPT = Path(__file__).with_name("hepta-servo-build-preflight.py")
TOOLCHAIN_CAPTURE_ENVIRONMENT = {
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_TERMINAL_PROMPT": "0",
    "LANG": "C",
    "LC_ALL": "C",
    "TZ": "UTC",
}
ALLOWED_LINKER_KINDS = {"clang", "cc", "lld", "mold", "msvc-link", "zig"}
PATH_FREE_VERSION = re.compile(r"^[A-Za-z0-9][A-Za-z0-9.+_ :(),=@-]{0,255}$")


class BuildPreflightV2Error(RuntimeError):
    pass


def fail(message: str) -> None:
    raise BuildPreflightV2Error(message)


def load_base() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_build_preflight_v1_core",
        BASE_SCRIPT,
    )
    if specification is None or specification.loader is None:
        raise BuildPreflightV2Error("cannot load build-preflight core")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    original = module.toolchain_projection
    base_error = getattr(module, "BuildPreflightError", RuntimeError)

    def toolchain_projection(value: dict[str, Any]) -> dict[str, str]:
        try:
            projection = original(value)
        except base_error as error:
            raise BuildPreflightV2Error(str(error)) from error
        linker = value.get("linker")
        capture = value.get("capture")
        rustc = value.get("rustc")
        cargo = value.get("cargo")
        if not all(isinstance(item, dict) for item in (linker, capture, rustc, cargo)):
            fail("toolchain receipt component facts are incomplete")
        linker_kind = linker.get("kind")
        if linker_kind not in ALLOWED_LINKER_KINDS:
            fail("toolchain receipt linker kind is outside the allowlist")
        expected_commands = [
            "rustc -vV",
            "cargo -Vv",
            f"{linker_kind} --version",
        ]
        if capture.get("commands") != expected_commands:
            fail("toolchain receipt version commands differ from the frozen set")
        if capture.get("minimal_environment") != TOOLCHAIN_CAPTURE_ENVIRONMENT:
            fail("toolchain receipt capture environment differs from the frozen allowlist")
        values = (
            rustc.get("version"),
            rustc.get("release"),
            rustc.get("host"),
            cargo.get("version"),
            cargo.get("release"),
            cargo.get("host"),
            linker.get("version"),
            value.get("host"),
        )
        if any(not isinstance(item, str) or not PATH_FREE_VERSION.fullmatch(item) for item in values):
            fail("toolchain receipt contains unsafe or path-like version text")
        return projection

    module.toolchain_projection = toolchain_projection
    return module


def main() -> int:
    base = load_base()
    try:
        result = base.main()
    except getattr(base, "BuildPreflightError", RuntimeError) as error:
        raise BuildPreflightV2Error(str(error)) from error
    if not isinstance(result, int):
        raise BuildPreflightV2Error("build-preflight core returned a non-integer status")
    return result


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except BuildPreflightV2Error as error:
        print(f"HEPTA_SERVO_BUILD_PREFLIGHT_V2=FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
