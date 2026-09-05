#!/usr/bin/env python3
"""Normalize and verify the bounded Hepta V8 source-closure candidate."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tomllib
from pathlib import Path

from hepta_module_doc_metadata import synchronize as synchronize_module_metadata

from hepta_source_registry_closure import normalize as normalize_source_registries
from hepta_source_registry_closure import verify as verify_source_registries

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
    "qualification/gap-closure/PLAN_AUDIT.json",
    "scripts/hepta_source_registry_closure.py",
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
    missing = [member for member in RUST_PACKAGES if f'    "{member}",\n' not in text]
    if not missing:
        return False
    anchor = '    "hepta-evidence",\n'
    if anchor not in text:
        raise RuntimeError("workspace member insertion anchor is missing")
    insertion = "".join(f'    "{member}",\n' for member in missing)
    CARGO_MANIFEST.write_text(
        text.replace(anchor, anchor + insertion, 1), encoding="utf-8"
    )
    return True


def normalize_ndu_helpers() -> bool:
    changed = False
    digest_path = ROOT / "codex-rs" / "hepta-ndu" / "src" / "evaluation_digest.rs"
    digest_text = digest_path.read_text(encoding="utf-8")
    public_axis_helper = (
        "pub(crate) fn push_axis_values(bytes: &mut Vec<u8>, values: &[AxisValue])"
    )
    if public_axis_helper not in digest_text:
        private_axis_helper = (
            "fn push_axis_values(bytes: &mut Vec<u8>, values: &[AxisValue])"
        )
        if private_axis_helper not in digest_text:
            raise RuntimeError("NDU axis digest helper signature is missing")
        digest_text = digest_text.replace(private_axis_helper, public_axis_helper, 1)
        changed = True
    if "\nfn push_id(bytes: &mut Vec<u8>, value: &StableId)" not in digest_text:
        digest_text += (
            "\nfn push_id(bytes: &mut Vec<u8>, value: &StableId) {\n"
            "    let raw = value.as_str().as_bytes();\n"
            "    push_len(bytes, raw.len());\n"
            "    bytes.extend_from_slice(raw);\n"
            "}\n\n"
            "fn push_len(bytes: &mut Vec<u8>, value: usize) {\n"
            "    let converted = u32::try_from(value).unwrap_or(u32::MAX);\n"
            "    bytes.extend_from_slice(&converted.to_be_bytes());\n"
            "}\n"
        )
        changed = True
    digest_path.write_text(digest_text, encoding="utf-8")

    evaluator_path = ROOT / "codex-rs" / "hepta-ndu" / "src" / "evaluator.rs"
    evaluator_text = evaluator_path.read_text(encoding="utf-8")
    if "use crate::AxisLimit;\n" not in evaluator_text:
        anchor = "use crate::AxisDirection;\n"
        if anchor not in evaluator_text:
            raise RuntimeError("NDU AxisDirection import anchor is missing")
        evaluator_text = evaluator_text.replace(
            anchor, anchor + "use crate::AxisLimit;\n", 1
        )
        changed = True
    if "use crate::evaluation_digest::push_axis_values;\n" not in evaluator_text:
        anchor = "use crate::evaluation_digest::digest_profile;\n"
        if anchor not in evaluator_text:
            raise RuntimeError("NDU digest import anchor is missing")
        evaluator_text = evaluator_text.replace(
            anchor,
            anchor + "use crate::evaluation_digest::push_axis_values;\n",
            1,
        )
        changed = True
    public_normalizer = "pub(crate) fn normalize_axis_values(values: &mut [AxisValue])"
    if public_normalizer not in evaluator_text:
        private_normalizer = "fn normalize_axis_values(values: &mut [AxisValue])"
        if private_normalizer not in evaluator_text:
            raise RuntimeError("NDU axis normalizer signature is missing")
        evaluator_text = evaluator_text.replace(
            private_normalizer, public_normalizer, 1
        )
        changed = True
    evaluator_path.write_text(evaluator_text, encoding="utf-8")

    scoring_path = ROOT / "codex-rs" / "hepta-ndu" / "src" / "scoring.rs"
    scoring_text = scoring_path.read_text(encoding="utf-8")
    normalizer_import = "use crate::evaluator::normalize_axis_values;\n"
    if normalizer_import not in scoring_text:
        anchor = "use crate::mul_q32_ties_even;\n"
        if anchor not in scoring_text:
            raise RuntimeError("NDU scoring import anchor is missing")
        scoring_text = scoring_text.replace(anchor, anchor + normalizer_import, 1)
        scoring_path.write_text(scoring_text, encoding="utf-8")
        changed = True

    return changed


def normalize_source() -> bool:
    workspace_changed = normalize_workspace()
    ndu_changed = normalize_ndu_helpers()
    registry_changed = normalize_source_registries()
    metadata_changed = bool(synchronize_module_metadata(ROOT, write=True))
    return workspace_changed or ndu_changed or registry_changed or metadata_changed


def verify() -> list[str]:
    failures: list[str] = []
    try:
        workspace_manifest = tomllib.loads(CARGO_MANIFEST.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        return [f"cannot parse codex-rs/Cargo.toml: {error}"]

    workspace = workspace_manifest.get("workspace")
    members = (
        set(workspace.get("members", ())) if isinstance(workspace, dict) else set()
    )
    for root_name, package_name in RUST_PACKAGES.items():
        root = ROOT / "codex-rs" / root_name
        for relative in ("Cargo.toml", "BUILD.bazel", "src/lib.rs"):
            path = root / relative
            if not path.is_file():
                failures.append(f"missing source file: {path.relative_to(ROOT)}")
        test_files = tuple((root / "src").glob("*_tests.rs"))
        if not test_files:
            failures.append(
                f"missing focused Rust tests under: {root.relative_to(ROOT)}"
            )
        manifest_path = root / "Cargo.toml"
        if manifest_path.is_file():
            try:
                manifest = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
            except (OSError, tomllib.TOMLDecodeError) as error:
                failures.append(
                    f"invalid manifest {manifest_path.relative_to(ROOT)}: {error}"
                )
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
                failures.append(
                    "qualification base commit does not match the canonical source"
                )
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

    failures.extend(verify_source_registries())

    # Source presence cannot hide stale guides or a broken full module index.
    document_check = subprocess.run(
        [sys.executable, str(ROOT / "scripts/hepta-module-docs.py"), "verify"],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if document_check.returncode:
        failures.append(
            "module document integrity: "
            + (document_check.stderr or document_check.stdout).strip()
        )

    for relative in (
        "tools/hepta-engineering-control/hepta_engineering_control.py",
        "tools/hepta-engineering-control/test_hepta_engineering_control.py",
        "scripts/hepta-gap-closure.py",
        "scripts/hepta_source_registry_closure.py",
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
        changed = normalize_source() if args.command == "normalize" else False
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
        print(json.dumps({"source_changed": changed}, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())
