#!/usr/bin/env python3
"""Canonical v3 entrypoint for Servo build-input sealing.

The v1 core restricted toolchain version text more narrowly than real rustc and
linker version lines. This entrypoint retains the v2 independent toolchain
receipt binding while permitting a bounded set of printable punctuation used by
real version strings. It does not relax commands, paths, network, or authority.
"""

from __future__ import annotations

import importlib.util
import re
import sys
from pathlib import Path
from types import ModuleType

V2_SCRIPT = Path(__file__).with_name("hepta-servo-build-input-seal-v2.py")
SAFE_VERSION_PATTERN = re.compile(
    r"^[A-Za-z0-9][A-Za-z0-9.+_ :()/,=@-]{0,255}$"
)


class BuildInputV3Error(RuntimeError):
    pass


def load_v2() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_build_input_seal_v2_core",
        V2_SCRIPT,
    )
    if specification is None or specification.loader is None:
        raise BuildInputV3Error("cannot load build-input v2 sealer")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    original_load_base = module.load_base

    def load_patched_base() -> ModuleType:
        base = original_load_base()
        base.VERSION_PATTERN = SAFE_VERSION_PATTERN
        return base

    module.load_base = load_patched_base
    return module


def main() -> int:
    v2 = load_v2()
    result = v2.main()
    if not isinstance(result, int):
        raise BuildInputV3Error("build-input v2 sealer returned a non-integer status")
    return result


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except BuildInputV3Error as error:
        print(f"HEPTA_SERVO_BUILD_INPUT_SEAL_V3=FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
