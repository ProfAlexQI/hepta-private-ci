#!/usr/bin/env python3
"""Canonical v2 Servo build-input sealer.

The v2 sealer requires an independently captured toolchain receipt and verifies
that every toolchain field in the build recipe exactly matches it. It does not
run a build or create an artifact.
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import os
import sys
from pathlib import Path
from types import ModuleType
from typing import Any

BASE_SCRIPT = Path(__file__).with_name("hepta-servo-build-input-seal.py")
TOOLCHAIN_SCHEMA = "hepta.browser.servo_toolchain_receipt.v1"
OUTPUT_SCHEMA = "hepta.browser.servo_build_input_manifest.v2"


class BuildInputV2Error(RuntimeError):
    pass


def fail(message: str) -> None:
    raise BuildInputV2Error(message)


def load_base() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_servo_build_input_seal_v1_core",
        BASE_SCRIPT,
    )
    if specification is None or specification.loader is None:
        raise BuildInputV2Error("cannot load build-input v1 core")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


def require_exact_keys(value: dict[str, Any], expected: set[str], label: str) -> None:
    actual = set(value)
    if actual != expected:
        fail(f"{label} keys differ: {sorted(actual ^ expected)}")


def require_closed_authority(value: Any, authority_keys: set[str], label: str) -> None:
    if not isinstance(value, dict) or set(value) != authority_keys:
        fail(f"{label} authority keys differ")
    enabled = sorted(key for key, item in value.items() if item is not False)
    if enabled:
        fail(f"{label} attempted to enable authority: {enabled}")


def validate_toolchain_receipt(
    receipt: dict[str, Any],
    base: ModuleType,
) -> dict[str, str]:
    require_exact_keys(
        receipt,
        {
            "schema",
            "schema_version",
            "target",
            "host",
            "rustc",
            "cargo",
            "linker",
            "capture",
            "machine_local_paths_included",
            "authority",
        },
        "toolchain receipt",
    )
    if receipt.get("schema") != TOOLCHAIN_SCHEMA or receipt.get("schema_version") != 1:
        fail("toolchain receipt schema is invalid")
    target = receipt.get("target")
    host = receipt.get("host")
    if target not in base.ALLOWED_TARGETS:
        fail("toolchain receipt target is outside the initial platform allowlist")
    if not isinstance(host, str) or not base.VERSION_PATTERN.fullmatch(host):
        fail("toolchain receipt host is invalid")
    rustc = receipt.get("rustc")
    cargo = receipt.get("cargo")
    linker = receipt.get("linker")
    capture = receipt.get("capture")
    if not isinstance(rustc, dict) or not isinstance(cargo, dict) or not isinstance(linker, dict):
        fail("toolchain receipt component facts are missing")
    if not isinstance(capture, dict):
        fail("toolchain receipt capture facts are missing")
    for section, label in ((rustc, "rustc"), (cargo, "cargo"), (linker, "linker")):
        for key in ("binary_sha256", "output_sha256"):
            base.require_sha256(section.get(key), f"{label} {key}")
        length = section.get("binary_bytes")
        if not isinstance(length, int) or isinstance(length, bool) or length <= 0:
            fail(f"{label} binary length is invalid")
    if rustc.get("host") != host or cargo.get("host") != host:
        fail("toolchain receipt host projections differ")
    base.require_git_object(rustc.get("commit_hash"), "rustc commit hash")
    for key in ("version", "release"):
        value = rustc.get(key)
        if not isinstance(value, str) or not base.VERSION_PATTERN.fullmatch(value):
            fail(f"rustc {key} is invalid")
        value = cargo.get(key)
        if not isinstance(value, str) or not base.VERSION_PATTERN.fullmatch(value):
            fail(f"cargo {key} is invalid")
    for key in ("kind", "version"):
        value = linker.get(key)
        if not isinstance(value, str) or not base.VERSION_PATTERN.fullmatch(value):
            fail(f"linker {key} is invalid")
    if capture.get("network_access_used") is not False:
        fail("toolchain receipt attempted to use network access")
    if capture.get("build_run") is not False or capture.get("artifact_created") is not False:
        fail("toolchain receipt attempted to claim build execution")
    if receipt.get("machine_local_paths_included") is not False:
        fail("toolchain receipt contains machine-local paths")
    require_closed_authority(receipt.get("authority"), base.AUTHORITY_KEYS, "toolchain receipt")
    return {
        "rustc_version": rustc["version"],
        "rustc_commit_hash": rustc["commit_hash"],
        "cargo_version": cargo["version"],
        "host": host,
        "target": target,
        "rustc_binary_sha256": rustc["binary_sha256"],
        "cargo_binary_sha256": cargo["binary_sha256"],
        "linker_kind": linker["kind"],
        "linker_version": linker["version"],
        "linker_binary_sha256": linker["binary_sha256"],
    }


def seal_v2(
    source: dict[str, Any],
    source_raw: bytes,
    recipe: dict[str, Any],
    recipe_raw: bytes,
    toolchain: dict[str, Any],
    toolchain_raw: bytes,
    base: ModuleType,
) -> dict[str, Any]:
    expected_projection = validate_toolchain_receipt(toolchain, base)
    recipe_projection = recipe.get("toolchain")
    if not isinstance(recipe_projection, dict):
        fail("build recipe has no toolchain projection")
    if recipe_projection != expected_projection:
        differing = sorted(
            key
            for key in set(recipe_projection) | set(expected_projection)
            if recipe_projection.get(key) != expected_projection.get(key)
        )
        fail(f"build recipe toolchain differs from captured receipt: {differing}")
    manifest = base.seal(source, source_raw, recipe, recipe_raw)
    manifest["schema"] = OUTPUT_SCHEMA
    manifest["toolchain_receipt_sha256"] = base.sha256_bytes(toolchain_raw)
    manifest["qualification"]["toolchain_receipt_independently_captured"] = True
    return manifest


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-verification", required=True)
    parser.add_argument("--recipe", required=True)
    parser.add_argument("--toolchain-receipt", required=True)
    parser.add_argument("--output", required=True)
    return parser.parse_args()


def main() -> int:
    base = load_base()
    try:
        arguments = parse_arguments()
        source_path = base.safe_absolute_input(
            arguments.source_verification,
            "--source-verification",
        )
        recipe_path = base.safe_absolute_input(arguments.recipe, "--recipe")
        toolchain_path = base.safe_absolute_input(
            arguments.toolchain_receipt,
            "--toolchain-receipt",
        )
        output = Path(arguments.output)
        if not output.is_absolute() or output != output.parent.resolve(strict=True) / output.name:
            fail("--output must be a canonical absolute path")
        source, source_raw = base.read_canonical_json(
            source_path,
            "source verification receipt",
        )
        recipe, recipe_raw = base.read_canonical_json(recipe_path, "build recipe")
        toolchain, toolchain_raw = base.read_canonical_json(
            toolchain_path,
            "toolchain receipt",
        )
        manifest = seal_v2(
            source,
            source_raw,
            recipe,
            recipe_raw,
            toolchain,
            toolchain_raw,
            base,
        )
        base.write_atomic(output, manifest)
    except (BuildInputV2Error, base.BuildInputError, OSError, UnicodeError) as error:
        print(f"HEPTA_SERVO_BUILD_INPUT_SEAL_V2=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": OUTPUT_SCHEMA,
                "status": manifest["status"],
                "commit": base.EXPECTED_COMMIT,
                "tree": base.EXPECTED_TREE,
                "target": manifest["build"]["target"],
                "toolchain_receipt_bound": True,
                "build_run": False,
                "artifact_created": False,
                "servo_runtime_qualified": False,
                "authority": "all_false",
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
