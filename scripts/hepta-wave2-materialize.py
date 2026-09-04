#!/usr/bin/env python3
"""Materialize, normalize and verify the remaining declared V8 source roots."""

from __future__ import annotations

import argparse
import json
import sys
import tomllib
from pathlib import Path

import hepta_source_registry_closure as source_registry
from hepta_remaining_source_wave import RUST_PACKAGES
from hepta_remaining_source_wave import SOURCE_ROOTS
from hepta_remaining_source_wave import format_and_stage
from hepta_remaining_source_wave import normalize_files
from hepta_remaining_source_wave import verify as verify_remaining

ROOT = Path(__file__).resolve().parents[1]
CARGO_MANIFEST = ROOT / "codex-rs" / "Cargo.toml"
BINDINGS_PATH = ROOT / "docs" / "modules" / "SOURCE_BINDINGS.json"
AUDIT_PATH = ROOT / "qualification" / "gap-closure" / "PLAN_AUDIT.json"
QUALIFICATION_MANIFEST = ROOT / "qualification" / "gap-closure" / "MANIFEST.json"

source_registry.SOURCE_ROOTS.update(SOURCE_ROOTS)


def normalize_workspace() -> bool:
    text = CARGO_MANIFEST.read_text(encoding="utf-8")
    missing = [
        member
        for member in RUST_PACKAGES
        if f'    "{member}",\n' not in text
    ]
    if not missing:
        return False
    anchor = '    "hepta-evidence",\n'
    if anchor not in text:
        raise RuntimeError("workspace member insertion anchor is missing")
    insertion = "".join(f'    "{member}",\n' for member in sorted(missing))
    CARGO_MANIFEST.write_text(
        text.replace(anchor, anchor + insertion, 1),
        encoding="utf-8",
    )
    return True


def normalize() -> bool:
    source_changed = normalize_files()
    workspace_changed = normalize_workspace()
    formatting_changed = format_and_stage()
    registry_changed = source_registry.normalize()
    return source_changed or workspace_changed or formatting_changed or registry_changed


def verify() -> list[str]:
    failures = verify_remaining()
    failures.extend(source_registry.verify())

    try:
        cargo = tomllib.loads(CARGO_MANIFEST.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        failures.append(f"cannot parse Cargo workspace: {error}")
    else:
        workspace = cargo.get("workspace")
        members = set(workspace.get("members", ())) if isinstance(workspace, dict) else set()
        for member in sorted(RUST_PACKAGES):
            if member not in members:
                failures.append(f"wave2 workspace member is missing: {member}")

    try:
        bindings_document = json.loads(BINDINGS_PATH.read_text(encoding="utf-8"))
        bindings = bindings_document.get("bindings")
    except (OSError, json.JSONDecodeError) as error:
        failures.append(f"cannot parse source bindings: {error}")
    else:
        if not isinstance(bindings, list):
            failures.append("source bindings must be a list")
        else:
            for binding in bindings:
                if not isinstance(binding, dict):
                    failures.append("source binding contains a non-object record")
                    continue
                missing = binding.get("missingDeclaredRoots")
                status = binding.get("sourceStatus")
                if isinstance(missing, list) and missing:
                    failures.append(
                        f"source binding still has missing roots: {binding.get('module')}"
                    )
                if status in {
                    "target_unmaterialized",
                    "target_partially_materialized",
                    "external_with_adapter_target",
                    "existing_declared_unbound",
                    "existing_legacy_aggregate",
                }:
                    failures.append(
                        f"source binding is not closed: {binding.get('module')}={status}"
                    )

    expected_roots = sorted(
        {
            root
            for roots in source_registry.SOURCE_ROOTS.values()
            for root in roots
        }
    )
    try:
        manifest = json.loads(QUALIFICATION_MANIFEST.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        failures.append(f"cannot parse qualification manifest: {error}")
    else:
        if sorted(manifest.get("source_roots", ())) != expected_roots:
            failures.append("qualification manifest does not cover every declared source root")
        authority = manifest.get("authority")
        if not isinstance(authority, dict) or any(authority.values()):
            failures.append("qualification manifest grants authority")

    try:
        audit = json.loads(AUDIT_PATH.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        failures.append(f"cannot parse plan audit: {error}")
    else:
        if audit.get("implementedModuleCount") != len(source_registry.SOURCE_ROOTS):
            failures.append("plan audit implemented-module count is stale")
        if audit.get("unresolvedSourceBindingCount") != 0:
            failures.append("plan audit still reports unresolved source bindings")

    return failures


def emit_status(changed: bool | None = None) -> None:
    status = {
        "authority_granted": False,
        "implemented_module_count": len(source_registry.SOURCE_ROOTS),
        "remaining_source_binding_count": 0,
        "status": "all_declared_source_roots_materialized",
    }
    if changed is not None:
        status["source_changed"] = changed
    print(json.dumps(status, sort_keys=True))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=("normalize", "verify"))
    args = parser.parse_args()
    try:
        changed = normalize() if args.command == "normalize" else None
        failures = verify()
    except (OSError, RuntimeError) as error:
        print(error, file=sys.stderr)
        return 1
    if failures:
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    emit_status(changed)
    return 0


if __name__ == "__main__":
    sys.exit(main())
