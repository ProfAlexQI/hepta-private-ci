#!/usr/bin/env python3
"""Normalize and verify the bounded Hepta V8 source-closure candidate."""

from __future__ import annotations

import argparse
import json
import os
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CARGO_MANIFEST = ROOT / "codex-rs" / "Cargo.toml"
QUALIFICATION_MANIFEST = ROOT / "qualification" / "gap-closure" / "MANIFEST.json"
BASE_COMMIT = "726c4f1f548a39b6b1a679e8f2f17898a9a447bf"
CANDIDATE_BRANCH = "codex/hepta-v8-gap-closure-20260905"

RUST_PACKAGES = {
    "hepta-bellman-operator": "codex-hepta-bellman-operator",
    "hepta-infer-worker-host": "codex-hepta-infer-worker-host",
    "hepta-intelligence-eval": "codex-hepta-intelligence-eval",
    "hepta-intuition": "codex-hepta-intuition",
    "hepta-learning-artifacts": "codex-hepta-learning-artifacts",
    "hepta-learning-ledger": "codex-hepta-learning-ledger",
    "hepta-ndu": "codex-hepta-ndu",
    "hepta-neuron": "codex-hepta-neuron",
    "hepta-objective": "codex-hepta-objective",
    "hepta-plasticity": "codex-hepta-plasticity",
    "hepta-prompt-optimizer": "codex-hepta-prompt-optimizer",
    "hepta-prompt-registry": "codex-hepta-prompt-registry",
}

REQUIRED_OTHER_FILES = (
    "apps/hepta-control-ui/package.json",
    "apps/hepta-control-ui/src/control.js",
    "apps/hepta-control-ui/test/control.test.js",
    "tools/hepta-engineering-control/hepta_engineering_control.py",
    "tools/hepta-engineering-control/test_hepta_engineering_control.py",
    "docs/readiness/GAP_CLOSURE_IMPLEMENTATION.md",
    "qualification/gap-closure/MANIFEST.json",
)

DENIED_AUTHORITY_FLAGS = (
    "runtime_authority",
    "production_writer",
    "production_activation",
    "effect_execution",
    "automatic_selection",
    "automatic_promotion",
    "automatic_merge",
    "release_authority",
    "physical_safety_qualified",
    "longitudinal_efficacy_qualified",
    "autonomous_propagation",
)


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
    insertion = "".join(f'    "{member}",\n' for member in missing)
    CARGO_MANIFEST.write_text(text.replace(anchor, anchor + insertion, 1), encoding="utf-8")
    return True


def verify() -> list[str]:
    failures: list[str] = []
    try:
        workspace_manifest = tomllib.loads(CARGO_MANIFEST.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        return [f"cannot parse codex-rs/Cargo.toml: {error}"]

    workspace = workspace_manifest.get("workspace")
    members = set(workspace.get("members", ())) if isinstance(workspace, dict) else set()
    for root_name, package_name in RUST_PACKAGES.items():
        root = ROOT / "codex-rs" / root_name
        for relative in ("Cargo.toml", "BUILD.bazel", "src/lib.rs", "src/lib_tests.rs"):
            path = root / relative
            if not path.is_file():
                failures.append(f"missing source file: {path.relative_to(ROOT)}")
        manifest_path = root / "Cargo.toml"
        if manifest_path.is_file():
            try:
                manifest = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
            except (OSError, tomllib.TOMLDecodeError) as error:
                failures.append(f"invalid manifest {manifest_path.relative_to(ROOT)}: {error}")
            else:
                package = manifest.get("package")
                if not isinstance(package, dict) or package.get("name") != package_name:
                    failures.append(
                        f"package identity mismatch for {root_name}: expected {package_name}"
                    )
                lints = manifest.get("lints")
                if not isinstance(lints, dict) or lints.get("workspace") is not True:
                    failures.append(f"workspace lints are not enabled for {root_name}")
        if root_name not in members:
            failures.append(f"workspace member is missing: {root_name}")
        lib_path = root / "src" / "lib.rs"
        if lib_path.is_file() and "#![forbid(unsafe_code)]" not in lib_path.read_text(
            encoding="utf-8"
        ):
            failures.append(f"unsafe-code prohibition is missing: {root_name}")

    for relative in REQUIRED_OTHER_FILES:
        if not (ROOT / relative).is_file():
            failures.append(f"missing required file: {relative}")

    bootstrap = ROOT / "qualification" / "value-learning" / "bootstrap"
    if bootstrap.exists():
        failures.append("temporary value-learning bootstrap payload was not removed")

    if QUALIFICATION_MANIFEST.is_file():
        try:
            manifest = json.loads(QUALIFICATION_MANIFEST.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as error:
            failures.append(f"invalid qualification manifest: {error}")
        else:
            if manifest.get("base_commit") != BASE_COMMIT:
                failures.append("qualification base commit does not match the canonical source")
            if manifest.get("candidate_branch") != CANDIDATE_BRANCH:
                failures.append("qualification candidate branch is incorrect")
            authority = manifest.get("authority")
            if not isinstance(authority, dict):
                failures.append("qualification authority posture is missing")
            else:
                for flag in DENIED_AUTHORITY_FLAGS:
                    if authority.get(flag) is not False:
                        failures.append(f"authority flag must remain false: {flag}")
            expected_roots = sorted(f"codex-rs/{name}" for name in RUST_PACKAGES)
            expected_roots.extend(
                ["apps/hepta-control-ui", "tools/hepta-engineering-control"]
            )
            if sorted(manifest.get("source_roots", ())) != sorted(expected_roots):
                failures.append("qualification source inventory is not closed-world")

    for relative in (
        "tools/hepta-engineering-control/hepta_engineering_control.py",
        "tools/hepta-engineering-control/test_hepta_engineering_control.py",
        "scripts/hepta-gap-closure.py",
    ):
        path = ROOT / relative
        if path.is_file():
            try:
                compile(path.read_text(encoding="utf-8"), str(path), "exec")
            except SyntaxError as error:
                failures.append(f"python syntax error in {relative}: {error}")

    return failures


def emit_status() -> None:
    print(
        json.dumps(
            {
                "authority_granted": False,
                "base_commit": BASE_COMMIT,
                "candidate_branch": CANDIDATE_BRANCH,
                "candidate_head": os.environ.get("GITHUB_SHA", "local"),
                "implemented_modules": sorted(RUST_PACKAGES),
                "status": "verified_source_candidate",
            },
            sort_keys=True,
        )
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=("normalize", "verify"))
    args = parser.parse_args()

    try:
        changed = normalize_workspace() if args.command == "normalize" else False
    except (OSError, RuntimeError) as error:
        print(error, file=sys.stderr)
        return 1

    failures = verify()
    if failures:
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    emit_status()
    if args.command == "normalize":
        print(json.dumps({"workspace_changed": changed}, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
