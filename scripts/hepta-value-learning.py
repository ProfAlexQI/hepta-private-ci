#!/usr/bin/env python3
"""Closed-world verifier for the V8.2 value-learning source candidate."""

from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "qualification/value-learning/MANIFEST.json"
EXPECTED_BASE = "726c4f1f548a39b6b1a679e8f2f17898a9a447bf"
REQUIRED_FALSE_CLAIMS = {
    "runtime_activation_complete",
    "production_selection_complete",
    "longitudinal_efficacy_proven",
    "physical_safety_qualified",
    "operator_acceptance_complete",
    "release_authorized",
    "autonomous_propagation_enabled",
}
REQUIRED_FALSE_AUTHORITY = {
    "runtime_activation",
    "production_writer",
    "model_invocation",
    "provider_dispatch",
    "external_effect",
    "selection",
    "promotion",
    "release",
}
REQUIRED_FILES = {
    "objective.compiler": [
        "codex-rs/hepta-objective/Cargo.toml",
        "codex-rs/hepta-objective/src/lib.rs",
        "codex-rs/hepta-objective/src/compiler.rs",
    ],
    "utility.ndu": [
        "codex-rs/hepta-ndu/Cargo.toml",
        "codex-rs/hepta-ndu/src/lib.rs",
        "codex-rs/hepta-ndu/src/evaluator.rs",
        "codex-rs/hepta-ndu/src/preference.rs",
    ],
    "learning.ledger": [
        "codex-rs/hepta-learning-ledger/Cargo.toml",
        "codex-rs/hepta-learning-ledger/src/lib.rs",
        "codex-rs/hepta-learning-ledger/src/ledger.rs",
    ],
    "learning.artifacts": [
        "codex-rs/hepta-learning-artifacts/Cargo.toml",
        "codex-rs/hepta-learning-artifacts/src/lib.rs",
        "codex-rs/hepta-learning-artifacts/src/registry.rs",
    ],
}


def fail(message: str) -> None:
    raise ValueError(message)


def read_manifest() -> dict[str, Any]:
    try:
        payload = json.loads(MANIFEST.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        fail(f"cannot read manifest: {exc}")
    if not isinstance(payload, dict):
        fail("manifest root must be an object")
    return payload


def require_mapping(payload: dict[str, Any], key: str) -> dict[str, Any]:
    value = payload.get(key)
    if not isinstance(value, dict):
        fail(f"manifest field {key!r} must be an object")
    return value


def verify() -> None:
    payload = read_manifest()
    if payload.get("schema_version") != 1:
        fail("unsupported manifest schema")
    candidate = require_mapping(payload, "candidate")
    if candidate.get("base_commit") != EXPECTED_BASE:
        fail("candidate base commit drift")

    roots = payload.get("implemented_source_roots")
    if not isinstance(roots, list):
        fail("implemented_source_roots must be an array")
    observed = set()
    for item in roots:
        if not isinstance(item, dict):
            fail("implemented source root must be an object")
        module_id = item.get("module_id")
        path = item.get("path")
        package = item.get("package")
        if not all(isinstance(value, str) and value for value in (module_id, path, package)):
            fail("implemented source root identity is incomplete")
        observed.add(module_id)
        cargo = ROOT / path / "Cargo.toml"
        if not cargo.is_file():
            fail(f"missing Cargo manifest for {module_id}: {cargo.relative_to(ROOT)}")
        manifest_text = cargo.read_text(encoding="utf-8")
        if f'name = "{package}"' not in manifest_text:
            fail(f"package identity drift for {module_id}")

    if observed != set(REQUIRED_FILES):
        fail(f"implemented module set drift: {sorted(observed)}")
    for module_id, paths in REQUIRED_FILES.items():
        for relative in paths:
            path = ROOT / relative
            if not path.is_file() or path.stat().st_size == 0:
                fail(f"missing source evidence for {module_id}: {relative}")

    authority = require_mapping(payload, "authority_posture")
    for key in REQUIRED_FALSE_AUTHORITY:
        if authority.get(key) is not False:
            fail(f"authority posture must remain false: {key}")

    claims = require_mapping(payload, "claims")
    if claims.get("source_implementation_complete_for_listed_roots") is not True:
        fail("listed source roots are not asserted complete")
    for key in REQUIRED_FALSE_CLAIMS:
        if claims.get(key) is not False:
            fail(f"unsupported capability claim must remain false: {key}")

    vertical = ROOT / "codex-rs/hepta-shadow-qualification/src/value_learning_tests.rs"
    if not vertical.is_file():
        fail("missing value-learning vertical qualification")

    for module_id, paths in REQUIRED_FILES.items():
        for relative in paths:
            if not relative.endswith(".rs"):
                continue
            source = (ROOT / relative).read_text(encoding="utf-8")
            if "unsafe" in source and "forbid(unsafe_code)" not in source:
                fail(f"unsafe token outside explicit crate prohibition: {module_id}/{relative}")

    print(
        json.dumps(
            {
                "status": "verified",
                "base_commit": EXPECTED_BASE,
                "implemented_modules": sorted(observed),
                "authority_granted": False,
            },
            sort_keys=True,
        )
    )


def main(argv: list[str]) -> int:
    if argv != ["verify"]:
        print("usage: hepta-value-learning.py verify", file=sys.stderr)
        return 2
    try:
        verify()
    except ValueError as exc:
        print(f"verification failed: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
