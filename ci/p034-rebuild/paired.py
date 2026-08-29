#!/usr/bin/env python3
"""Fail-closed aggregation for P0.3.4 X64/ARM64 exact-head receipts."""
from __future__ import annotations

import hashlib
import json
import os
import sys
from pathlib import Path

P033_HEAD = "eddcb59ca43a76ac83b64507983bd908f406ff48"
EXPECTED_LANES = {"linux-arm64": "ARM64", "linux-x64": "X64"}
FALSE_AUTHORITY = (
    "wired", "public_api_exported", "default_projection_pointer_changed",
    "default_recall_query_changed", "production_projection_gate",
    "production_authority", "external_effects", "operator_accepted",
    "promoted", "callers_ratchet",
)
REQUIRED_CHECKS = {
    "exact_predecessor_binding", "exact_scope_binding", "python_compile",
    "source_and_sqlite_contract", "clippy_delta_policy_selftest",
    "runner_arch_binding", "governed_rustfmt", "core_all_target_check",
    "p034_focused_tests", "p02_durable_compatibility", "core_full_tests",
    "core_exact_predecessor_clippy_delta", "clean_source_tree",
}


def one(root: Path, pattern: str) -> Path:
    matches = sorted(root.rglob(pattern))
    if len(matches) != 1:
        raise ValueError(f"expected one {pattern} under {root}, found {len(matches)}")
    return matches[0]


def main() -> int:
    if len(sys.argv) != 4:
        raise SystemExit("usage: paired.py EVIDENCE_ROOT EXPECTED_HEAD OUTPUT_JSON")
    root = Path(sys.argv[1]).resolve()
    expected_head = sys.argv[2]
    output = Path(sys.argv[3]).resolve()
    errors: list[str] = []
    lanes: dict[str, object] = {}
    trees: set[str] = set()
    runs: set[int] = set()
    rustcs: set[str] = set()
    cargos: set[str] = set()

    try:
        bundle_receipt_path = one(root / "linux-x64", "candidate-bundle-receipt.json")
        bundle_receipt = json.loads(bundle_receipt_path.read_text())
        bundle_path = one(root / "linux-x64", "candidate.bundle")
        observed_bundle_sha = hashlib.sha256(bundle_path.read_bytes()).hexdigest()
        if bundle_receipt.get("schema") != "hepta_intelligence_p0_3_4_candidate_bundle_v1":
            errors.append("candidate bundle receipt schema mismatch")
        if bundle_receipt.get("base_head") != P033_HEAD:
            errors.append("candidate bundle base mismatch")
        if bundle_receipt.get("head") != expected_head:
            errors.append("candidate bundle head mismatch")
        if bundle_receipt.get("successor_commit_count") != 1:
            errors.append("candidate bundle is not one successor commit")
        if bundle_receipt.get("qualified_before_publication") is not True:
            errors.append("candidate was not X64-qualified before publication")
        if bundle_receipt.get("force_push") is not False or bundle_receipt.get("history_rewrite") is not False:
            errors.append("candidate bundle publication policy drift")
        if bundle_receipt.get("published") is not False:
            errors.append("candidate claims publication before paired qualification")
        if bundle_receipt.get("bundle_sha256") != observed_bundle_sha:
            errors.append("candidate bundle digest mismatch")
    except Exception as error:
        bundle_receipt = {}
        observed_bundle_sha = None
        errors.append(f"candidate bundle evidence: {error}")

    for lane, arch in EXPECTED_LANES.items():
        lane_errors: list[str] = []
        try:
            path = one(root / lane, "qualification-receipt.json")
            receipt = json.loads(path.read_text())
        except Exception as error:
            errors.append(f"{lane}: {error}")
            continue
        if receipt.get("schema") != "hepta_intelligence_p0_3_4_exact_head_qualification_v3":
            lane_errors.append("receipt schema mismatch")
        if receipt.get("head") != expected_head:
            lane_errors.append("head mismatch")
        if receipt.get("qualification_lane") != lane or receipt.get("runner_arch") != arch:
            lane_errors.append("lane/architecture mismatch")
        if receipt.get("qualified") is not True:
            lane_errors.append("lane is not qualified")
        checks = receipt.get("checks") or []
        observed_checks = {item.get("id") for item in checks if isinstance(item, dict)}
        if not REQUIRED_CHECKS.issubset(observed_checks):
            lane_errors.append(f"missing checks: {sorted(REQUIRED_CHECKS - observed_checks)}")
        if not checks or any(
            not isinstance(item, dict)
            or item.get("passed") is not True
            or item.get("exit_code") != 0
            for item in checks
        ):
            lane_errors.append("executable check failure")
        clippy = receipt.get("clippy_delta_receipt") or {}
        candidate = clippy.get("candidate") or {}
        baseline = clippy.get("baseline") or {}
        if (
            clippy.get("passed") is not True
            or clippy.get("introduced_diagnostic_count") != 0
            or clippy.get("compiler_failures_attributed_away") is not False
            or clippy.get("inherited_lint_suppressions_added") is not False
            or clippy.get("declared_dormant_source_expectation") is not True
            or clippy.get("undeclared_lint_suppressions_added") is not False
            or candidate.get("exact_scope_bound") is not True
            or baseline.get("head") != P033_HEAD
        ):
            lane_errors.append("exact-predecessor Clippy delta failure")
        drift = [key for key in FALSE_AUTHORITY if receipt.get(key) is not False]
        if drift:
            lane_errors.append(f"authority drift: {drift}")
        for value, bucket, label in (
            (receipt.get("tree"), trees, "tree"),
            (receipt.get("run_id"), runs, "run id"),
            (receipt.get("rustc"), rustcs, "rustc"),
            (receipt.get("cargo"), cargos, "cargo"),
        ):
            if value in (None, "", 0):
                lane_errors.append(f"{label} missing")
            else:
                bucket.add(value)
        if lane_errors:
            errors.extend(f"{lane}: {item}" for item in lane_errors)
        lanes[lane] = {
            "head": receipt.get("head"),
            "tree": receipt.get("tree"),
            "runner_arch": receipt.get("runner_arch"),
            "run_id": receipt.get("run_id"),
            "rustc": receipt.get("rustc"),
            "cargo": receipt.get("cargo"),
            "qualified": receipt.get("qualified"),
            "introduced_clippy_diagnostics": clippy.get("introduced_diagnostic_count"),
        }

    if len(trees) != 1:
        errors.append(f"lanes do not bind one tree: {sorted(trees)}")
    if len(runs) != 1:
        errors.append(f"lanes do not bind one workflow run: {sorted(runs)}")
    if len(rustcs) != 1 or len(cargos) != 1:
        errors.append("lanes do not bind one Rust/Cargo identity")
    bundle_tree = bundle_receipt.get("tree")
    if len(trees) == 1 and bundle_tree != next(iter(trees)):
        errors.append("candidate bundle tree differs from lane tree")
    qualified = not errors and set(lanes) == set(EXPECTED_LANES)
    paired = {
        "schema": "hepta_intelligence_p0_3_4_paired_linux_qualification_v2",
        "head": expected_head,
        "tree": next(iter(trees)) if len(trees) == 1 else None,
        "dependency_head": P033_HEAD,
        "required_lanes": EXPECTED_LANES,
        "lanes": lanes,
        "candidate_bundle_sha256": observed_bundle_sha,
        "publication_authorized": qualified,
        "ordinary_push_only": True,
        "force_push": False,
        "history_rewrite": False,
        "errors": errors,
        **{key: False for key in FALSE_AUTHORITY},
        "qualified": qualified,
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(paired, indent=2, sort_keys=True) + "\n")
    print(json.dumps(paired, indent=2, sort_keys=True))
    github_output = os.environ.get("GITHUB_OUTPUT")
    if github_output:
        with open(github_output, "a", encoding="utf-8") as stream:
            stream.write(f"qualified={'true' if qualified else 'false'}\n")
            stream.write(f"head={expected_head}\n")
            stream.write(f"tree={paired['tree'] or ''}\n")
            stream.write(f"bundle_sha256={observed_bundle_sha or ''}\n")
    return 0 if qualified else 1


if __name__ == "__main__":
    raise SystemExit(main())
