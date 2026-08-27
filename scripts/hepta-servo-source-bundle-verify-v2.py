#!/usr/bin/env python3
"""Canonical v2 entrypoint for offline Servo source-bundle verification.

Git tree entries use `base_name_compare` semantics: when one name ends, a tree
is compared as if it had a trailing slash. The v1 verifier used ordinary byte
ordering. This entrypoint patches the tree reconstruction to match Git exactly.
"""

from __future__ import annotations

import functools
import importlib.util
import sys
from pathlib import Path
from types import ModuleType
from typing import Any

BASE_SCRIPT = Path(__file__).with_name("hepta-servo-source-bundle-verify.py")


class SourceBundleVerifierV2Error(RuntimeError):
    pass


def git_tree_entry_compare(
    left: tuple[bytes, bytes, bytes],
    right: tuple[bytes, bytes, bytes],
) -> int:
    left_name, left_mode, _left_object = left
    right_name, right_mode, _right_object = right
    common = min(len(left_name), len(right_name))
    left_prefix = left_name[:common]
    right_prefix = right_name[:common]
    if left_prefix != right_prefix:
        return -1 if left_prefix < right_prefix else 1
    left_next = (
        left_name[common]
        if common < len(left_name)
        else (ord("/") if left_mode == b"40000" else 0)
    )
    right_next = (
        right_name[common]
        if common < len(right_name)
        else (ord("/") if right_mode == b"40000" else 0)
    )
    return (left_next > right_next) - (left_next < right_next)


def load_base() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_source_bundle_verify_v1_core",
        BASE_SCRIPT,
    )
    if specification is None or specification.loader is None:
        raise SourceBundleVerifierV2Error("cannot load source-bundle verifier core")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)

    def object_id(node: Any) -> bytes:
        entries: list[tuple[bytes, bytes, bytes]] = []
        for name, (mode, object_id_value) in node.files.items():
            entries.append((name, mode, object_id_value))
        for name, child in node.directories.items():
            entries.append((name, b"40000", child.object_id()))
        payload = bytearray()
        for name, mode, object_id_value in sorted(
            entries,
            key=functools.cmp_to_key(git_tree_entry_compare),
        ):
            payload.extend(mode)
            payload.extend(b" ")
            payload.extend(name)
            payload.extend(b"\0")
            payload.extend(object_id_value)
        return module.git_hash(b"tree", bytes(payload))

    module.TreeNode.object_id = object_id
    return module


def main() -> int:
    base = load_base()
    result = base.main()
    if not isinstance(result, int):
        raise SourceBundleVerifierV2Error(
            "source-bundle verifier core returned a non-integer status"
        )
    return result


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SourceBundleVerifierV2Error as error:
        print(f"HEPTA_SERVO_SOURCE_BUNDLE_VERIFY_V2=FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
