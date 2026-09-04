#!/usr/bin/env python3
"""Deterministically extend the primary qualification script for Wave 2."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "scripts" / "hepta-gap-closure.py"

IMPORTS_OLD = """from hepta_source_registry_closure import normalize as normalize_source_registries
from hepta_source_registry_closure import verify as verify_source_registries
"""
IMPORTS_NEW = """from hepta_remaining_source_wave import (
    RUST_PACKAGES as REMAINING_RUST_PACKAGES,
)
from hepta_remaining_source_wave import (
    SOURCE_ROOTS as REMAINING_SOURCE_ROOTS,
)
from hepta_remaining_source_wave import format_and_stage as format_remaining_and_stage
from hepta_remaining_source_wave import normalize_files as normalize_remaining_files
from hepta_remaining_source_wave import verify as verify_remaining_sources
from hepta_source_registry_closure import SOURCE_ROOTS as REGISTRY_SOURCE_ROOTS
from hepta_source_registry_closure import normalize as normalize_source_registries
from hepta_source_registry_closure import verify as verify_source_registries

REGISTRY_SOURCE_ROOTS.update(REMAINING_SOURCE_ROOTS)
"""

NORMALIZE_OLD = """def normalize_source() -> bool:
    workspace_changed = normalize_workspace()
    ndu_changed = normalize_ndu_helpers()
    registry_changed = normalize_source_registries()
    return workspace_changed or ndu_changed or registry_changed
"""
NORMALIZE_NEW = """def normalize_source() -> bool:
    remaining_files_changed = normalize_remaining_files()
    workspace_changed = normalize_workspace()
    remaining_format_changed = format_remaining_and_stage()
    ndu_changed = normalize_ndu_helpers()
    registry_changed = normalize_source_registries()
    return (
        remaining_files_changed
        or workspace_changed
        or remaining_format_changed
        or ndu_changed
        or registry_changed
    )
"""

ROOTS_OLD = """            expected_roots = sorted(f\"codex-rs/{name}\" for name in RUST_PACKAGES)
            expected_roots.extend(
                [\"apps/hepta-control-ui\", \"tools/hepta-engineering-control\"]
            )
            if sorted(manifest.get(\"source_roots\", ())) != sorted(expected_roots):
                failures.append(\"qualification source inventory is not closed-world\")
"""
ROOTS_NEW = """            expected_roots = {
                *(f\"codex-rs/{name}\" for name in RUST_PACKAGES),
                \"apps/hepta-control-ui\",
                \"tools/hepta-engineering-control\",
            }
            expected_roots.update(
                root
                for roots in REMAINING_SOURCE_ROOTS.values()
                for root in roots
            )
            if sorted(manifest.get(\"source_roots\", ())) != sorted(expected_roots):
                failures.append(\"qualification source inventory is not closed-world\")
"""

REQUIRED_OLD = """    \"scripts/hepta_source_registry_closure.py\",
)
"""
REQUIRED_NEW = """    \"scripts/hepta_source_registry_closure.py\",
    \"scripts/hepta_remaining_source_wave.py\",
    \"scripts/hepta_wave2_memory_sources.py\",
    \"scripts/hepta_wave2_control_sources.py\",
    \"scripts/hepta_wave2_external_sources.py\",
)
"""

PYTHON_OLD = """        \"scripts/hepta-gap-closure.py\",
        \"scripts/hepta_source_registry_closure.py\",
    ):
"""
PYTHON_NEW = """        \"scripts/hepta-gap-closure.py\",
        \"scripts/hepta_source_registry_closure.py\",
        \"scripts/hepta_remaining_source_wave.py\",
        \"scripts/hepta_wave2_memory_sources.py\",
        \"scripts/hepta_wave2_control_sources.py\",
        \"scripts/hepta_wave2_external_sources.py\",
    ):
"""


def replace_once(text: str, old: str, new: str, label: str) -> str:
    if new in text:
        return text
    if old not in text:
        raise RuntimeError(f"primary gate patch anchor is missing: {label}")
    return text.replace(old, new, 1)


def normalize() -> bool:
    text = TARGET.read_text(encoding="utf-8")
    original = text
    text = replace_once(text, IMPORTS_OLD, IMPORTS_NEW, "imports")

    marker = "RUST_PACKAGES.update(REMAINING_RUST_PACKAGES)\n"
    if marker not in text:
        start = text.index("RUST_PACKAGES = {")
        end = text.index("\n}\n", start) + len("\n}\n")
        text = text[:end] + marker + text[end:]

    text = replace_once(text, REQUIRED_OLD, REQUIRED_NEW, "required files")
    text = replace_once(text, NORMALIZE_OLD, NORMALIZE_NEW, "normalize function")
    text = replace_once(text, ROOTS_OLD, ROOTS_NEW, "source inventory")

    verify_marker = "    failures.extend(verify_remaining_sources())\n"
    registry_marker = "    failures.extend(verify_source_registries())\n"
    if verify_marker not in text:
        if registry_marker not in text:
            raise RuntimeError("primary gate verify anchor is missing")
        text = text.replace(
            registry_marker,
            verify_marker + registry_marker,
            1,
        )

    text = replace_once(text, PYTHON_OLD, PYTHON_NEW, "python syntax inventory")
    compile(text, str(TARGET), "exec")
    if text == original:
        return False
    TARGET.write_text(text, encoding="utf-8")
    return True


def verify() -> list[str]:
    text = TARGET.read_text(encoding="utf-8")
    required = (
        "RUST_PACKAGES.update(REMAINING_RUST_PACKAGES)",
        "REGISTRY_SOURCE_ROOTS.update(REMAINING_SOURCE_ROOTS)",
        "normalize_remaining_files()",
        "format_remaining_and_stage()",
        "verify_remaining_sources()",
    )
    failures = [f"primary gate marker is missing: {value}" for value in required if value not in text]
    try:
        compile(text, str(TARGET), "exec")
    except SyntaxError as error:
        failures.append(f"primary gate syntax error: {error}")
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=("normalize", "verify"))
    args = parser.parse_args()
    try:
        changed = normalize() if args.command == "normalize" else None
        failures = verify()
    except (OSError, RuntimeError, ValueError) as error:
        print(error, file=sys.stderr)
        return 1
    if failures:
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    if changed is not None:
        print(f"primary_gate_changed={str(changed).lower()}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
