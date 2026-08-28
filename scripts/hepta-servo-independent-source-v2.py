#!/usr/bin/env python3
"""Canonical v2 entrypoint for independent Servo source acquisition.

The v1 implementation contains the source acquisition and receipt logic. This
entrypoint removes inherited Git configuration and installs exactly one process-
scoped Git configuration item: `tar.umask=0022`. That makes `git archive` file
modes independent of the runner process umask.

Git 2.55 emits the archive prefix itself as a directory entry without a trailing
slash. The v1 validator expected every entry to start with the slash-terminated
prefix. The canonical v2 wrapper accepts only that exact root directory entry;
all other entries still pass through the original fail-closed validator.
"""

from __future__ import annotations

import importlib.util
import os
import sys
import tarfile
from pathlib import Path
from types import ModuleType

BASE_SCRIPT = Path(__file__).with_name("hepta-servo-independent-source.py")


class DeterministicArchiveEnvironmentError(RuntimeError):
    pass


def install_deterministic_archive_environment() -> None:
    for key in list(os.environ):
        if key == "GIT_CONFIG_COUNT" or key.startswith("GIT_CONFIG_KEY_") or key.startswith(
            "GIT_CONFIG_VALUE_"
        ):
            del os.environ[key]
    os.environ.update(
        {
            "GIT_CONFIG_COUNT": "1",
            "GIT_CONFIG_KEY_0": "tar.umask",
            "GIT_CONFIG_VALUE_0": "0022",
            "TZ": "UTC",
            "SOURCE_DATE_EPOCH": "0",
        }
    )


def install_archive_root_compatibility(module: ModuleType) -> None:
    original = getattr(module, "safe_archive_member", None)
    if not callable(original):
        raise DeterministicArchiveEnvironmentError(
            "source-pipeline core has no archive-member validator"
        )

    def safe_archive_member(prefix: str, member: tarfile.TarInfo) -> None:
        root_name = prefix.rstrip("/")
        if member.name == root_name:
            if not member.isdir():
                module.fail("source archive root entry must be a directory")
            return
        original(prefix, member)

    module.safe_archive_member = safe_archive_member


def load_base() -> ModuleType:
    install_deterministic_archive_environment()
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_independent_source_v1_core",
        BASE_SCRIPT,
    )
    if specification is None or specification.loader is None:
        raise DeterministicArchiveEnvironmentError("cannot load source-pipeline core")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    install_archive_root_compatibility(module)
    return module


def main() -> int:
    base = load_base()
    result = base.main()
    if not isinstance(result, int):
        raise DeterministicArchiveEnvironmentError(
            "source-pipeline core returned a non-integer status"
        )
    return result


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except DeterministicArchiveEnvironmentError as error:
        print(f"HEPTA_SERVO_INDEPENDENT_SOURCE_V2=FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
