#!/usr/bin/env python3
"""Emit and verify deterministic Hepta Intelligence current truth."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import sys
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN_DIR = ROOT / "plans" / "hepta-intelligence"

PATHS = {
    "current": PLAN_DIR / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
    "document_authority": PLAN_DIR / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "evidence": PLAN_DIR / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "capabilities": PLAN_DIR / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "pr_stack": PLAN_DIR / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "integration_candidate": PLAN_DIR / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "master_plan": PLAN_DIR / "HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    "q0_receipt": PLAN_DIR / "HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json",
}

SCHEMAS = {
    "current": "hepta_intelligence_current_plan_v2",
    "document_authority": "hepta_intelligence_document_authority_registry_v1",
    "evidence": "hepta_intelligence_evidence_index_v1",
    "capabilities": "hepta_intelligence_capability_registry_v1",
    "pr_stack": "hepta_intelligence_pr_stack_registry_v1",
    "integration_candidate": "hepta_intelligence_integration_candidate_v1",
    "q0_receipt": "hepta_intelligence_q0_external_evidence_summary_v1",
}


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_CURRENT_TRUTH: {message}")


def load(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        fail(f"cannot parse {path.relative_to(ROOT)}: {exc}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain a JSON object")
    return value


def canonical(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def all_false(value: Any) -> bool:
    return isinstance(value, dict) and bool(value) and all(item is False for item in value.values())


def validate_capability(entry: Any) -> None:
    require(isinstance(entry, dict), "capability entry must be an object")
    capability_id = entry.get("capability_id")
    require(isinstance(capability_id, str) and capability_id, "capability_id missing")
    flags = [
        "implemented",
        "candidate_qualified",
        "wired",
        "runtime_qualified",
        "efficacy_proven",
        "operator_accepted",
        "promoted",
    ]
    for flag in flags:
        require(type(entry.get(flag)) is bool, f"{capability_id}.{flag} must be bool")
    require(not entry["runtime_qualified"] or entry["wired"], f"{capability_id}: runtime qualification without wiring")
    require(
        not entry["efficacy_proven"] or entry["runtime_qualified"],
        f"{capability_id}: efficacy without runtime qualification",
    )
    require(
        not entry["operator_accepted"] or entry["efficacy_proven"],
        f"{capability_id}: operator acceptance without efficacy",
    )
    require(
        not entry["promoted"] or entry["operator_accepted"],
        f"{capability_id}: promotion without operator acceptance",
    )
    require(all_false(entry.get("authority")), f"{capability_id}: authority must remain false")


def validate_and_build() -> dict[str, Any]:
    for path in PATHS.values():
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")

    current = load(PATHS["current"])
    document = load(PATHS["document_authority"])
    evidence = load(PATHS["evidence"])
    capabilities = load(PATHS["capabilities"])
    pr_stack = load(PATHS["pr_stack"])
    integration = load(PATHS["integration_candidate"])
    q0 = load(PATHS["q0_receipt"])

    for key, schema in SCHEMAS.items():
        require(load(PATHS[key]).get("schema") == schema, f"{key} schema mismatch")

    repository = current.get("repository")
    require(repository == "ProfHepta/hepta-private-ci", "repository identity mismatch")
    for label, value in (
        ("document authority", document),
        ("evidence", evidence),
        ("capabilities", capabilities),
        ("pr stack", pr_stack),
        ("integration candidate", integration),
    ):
        require(value.get("repository") == repository, f"{label} repository mismatch")

    canonical_plan = current.get("canonical")
    require(isinstance(canonical_plan, dict), "current canonical plan missing")
    require(canonical_plan.get("plan_id") == "HEPTA_INTELLIGENCE_MASTER_PLAN_V4", "plan id mismatch")
    require(canonical_plan.get("plan_version") == "4.1.0", "plan version mismatch")
    require(canonical_plan.get("content_sha256") == sha256(PATHS["master_plan"]), "master plan digest mismatch")

    read_order = current.get("session_bootstrap", {}).get("read_order")
    expected_read_order = [
        PATHS["current"].relative_to(ROOT).as_posix(),
        PATHS["document_authority"].relative_to(ROOT).as_posix(),
        PATHS["evidence"].relative_to(ROOT).as_posix(),
        PATHS["capabilities"].relative_to(ROOT).as_posix(),
        PATHS["pr_stack"].relative_to(ROOT).as_posix(),
        PATHS["integration_candidate"].relative_to(ROOT).as_posix(),
        PATHS["master_plan"].relative_to(ROOT).as_posix(),
    ]
    require(read_order == expected_read_order, "mandatory read order mismatch")

    q0_binding = dict(q0)
    observed_q0_binding = q0_binding.pop("receipt_binding_sha256", None)
    require(
        observed_q0_binding == hashlib.sha256(canonical(q0_binding)).hexdigest(),
        "Q0 evidence summary binding mismatch",
    )
    q0_observation = q0.get("evidence_observation", {})
    q0_candidate = q0_observation.get("branch")
    require(q0_candidate == "codex/hepta-intelligence-plan-v3-20260828", "Q0 branch mismatch")
    require(q0_observation.get("head") == "c768bcbeb4c1168088d2499828c24da521a2a73a", "Q0 head mismatch")
    require(q0_observation.get("tree") == "ca455a9ef797cd95164c880c7b8faba80b305589", "Q0 tree mismatch")
    require(q0_observation.get("parent") == "aeb8ac0bfb30d570a16c4914b6e4b31ce035dd62", "Q0 parent mismatch")
    require(q0.get("conclusion", {}).get("q0_executable_qualified") is True, "Q0 is not executable-qualified")
    require(q0.get("conclusion", {}).get("qualified_candidate") is True, "Q0 candidate is not qualified")
    require(q0.get("conclusion", {}).get("runtime_capability_qualified") is False, "Q0 gained runtime qualification")
    require(all_false(q0.get("authority")), "Q0 authority must remain false")

    capability_entries = capabilities.get("capabilities")
    require(isinstance(capability_entries, list) and capability_entries, "capability registry empty")
    for entry in capability_entries:
        validate_capability(entry)
    capability_ids = [entry["capability_id"] for entry in capability_entries]
    require(capability_ids == list(dict.fromkeys(capability_ids)), "duplicate capability ID")
    capability_map = {entry["capability_id"]: entry for entry in capability_entries}
    p0 = capability_map.get("P0_GROUNDED_MUTATION_FOUNDATION")
    require(isinstance(p0, dict) and p0.get("candidate_qualified") is True, "P0 candidate qualification missing")
    require(p0.get("wired") is False and p0.get("runtime_qualified") is False, "P0 crossed runtime boundary")
    a0 = capability_map.get("A0_CANONICAL_CAPABILITY_AND_EVIDENCE_AUTHORITY")
    require(isinstance(a0, dict) and a0.get("implemented") is True, "A0 source is not implemented")
    require(a0.get("candidate_qualified") is False, "A0 prematurely qualified itself")
    require(all_false(capabilities.get("authority")), "capability registry authority must remain false")

    canonical_stack = pr_stack.get("canonical_stack")
    require(isinstance(canonical_stack, dict), "canonical PR stack missing")
    require(canonical_stack.get("candidate_head") == q0_observation.get("head"), "PR stack/Q0 head mismatch")
    require(canonical_stack.get("candidate_tree") == q0_observation.get("tree"), "PR stack/Q0 tree mismatch")
    canonical_entries = canonical_stack.get("entries")
    require(isinstance(canonical_entries, list), "canonical PR entries missing")
    require([entry.get("pr_number") for entry in canonical_entries] == [7, 13, 14, 16, 21, 23, 29], "canonical PR order mismatch")
    require(all(entry.get("canonical_inclusion") is True for entry in canonical_entries), "canonical stack exclusion found")
    require(all(entry.get("merged") is False for entry in canonical_entries), "unapproved merged PR recorded")

    side_stacks = pr_stack.get("side_stacks")
    require(isinstance(side_stacks, list) and side_stacks, "side stack registry missing")
    require(all(stack.get("canonical") is False for stack in side_stacks), "side stack gained canonical status")
    external = pr_stack.get("external_unmerged_dependencies")
    require(isinstance(external, list) and external, "external dependency registry missing")
    require(all(item.get("canonical") is False and item.get("authority") is False for item in external), "external PR gained authority")
    require(all_false(pr_stack.get("authority")), "PR stack registry authority must remain false")

    require(integration.get("expected_parent") == q0_observation.get("head"), "A0 expected parent mismatch")
    require(integration.get("branch") == "codex/hepta-intelligence-a0-authority-gap-closure-20260829", "A0 branch mismatch")
    allowed = integration.get("allowed_changed_paths")
    require(isinstance(allowed, list) and allowed == sorted(set(allowed)), "A0 changed-path allowlist invalid")
    freeze = integration.get("source_freeze", {})
    require(freeze.get("rust_runtime_changes_allowed") is False, "runtime source freeze disabled")
    require(freeze.get("sql_migrations_allowed") is False, "migration freeze disabled")
    require(freeze.get("product_callers_allowed") is False, "caller freeze disabled")
    require(all_false(integration.get("authority")), "integration candidate authority must remain false")

    require(evidence.get("not_current_plan_authority") is True, "evidence index gained plan authority")
    indexed_q0 = evidence.get("q0_current_evidence", {})
    require(indexed_q0.get("summary_file_sha256") == sha256(PATHS["q0_receipt"]), "Q0 evidence file digest mismatch")
    require(indexed_q0.get("q0_executable_qualified") is True, "evidence index lost Q0 qualification")
    require(indexed_q0.get("runtime_wired") is False, "evidence index crossed runtime boundary")
    require(all_false(evidence.get("authority")), "evidence authority must remain false")
    require(all_false(document.get("authority")), "document authority registry crossed product authority")
    require(all_false(current.get("authority")), "current plan authority flags must remain false")

    truth = {
        "schema": "hepta_intelligence_current_truth_v1",
        "repository": repository,
        "as_of_utc": current.get("generated_at_utc"),
        "plan": {
            "id": canonical_plan.get("plan_id"),
            "version": canonical_plan.get("plan_version"),
            "content_sha256": canonical_plan.get("content_sha256"),
        },
        "active_phase": current.get("active_phase"),
        "q0": {
            "candidate": {
                "branch": q0_observation.get("branch"),
                "head": q0_observation.get("head"),
                "tree": q0_observation.get("tree"),
                "parent": q0_observation.get("parent"),
            },
            "run_id": q0_observation.get("run_id"),
            "qualified_candidate": True,
            "runtime_capability_qualified": False,
            "full_repository_merge_green": False,
        },
        "capabilities": capability_map,
        "canonical_stack": canonical_stack,
        "side_stacks": side_stacks,
        "external_unmerged_dependencies": external,
        "integration_candidate": {
            "candidate_id": integration.get("candidate_id"),
            "branch": integration.get("branch"),
            "expected_parent": integration.get("expected_parent"),
            "allowed_changed_paths": allowed,
            "source_freeze": freeze,
            "merge_policy": integration.get("merge_policy"),
        },
        "claims": current.get("claim_levels"),
        "blockers": [
            "A0 exact-head executable qualification",
            "independent review",
            "full-repository merge-green before default-branch merge",
            "B0 and later phases remain dependency-blocked",
        ],
        "input_sha256": {
            key: sha256(path)
            for key, path in PATHS.items()
        },
        "authority": current.get("authority"),
    }
    return truth


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify", action="store_true", help="validate only")
    parser.add_argument("--compact", action="store_true", help="emit canonical compact JSON")
    parser.add_argument("--output", type=Path, help="write JSON to a file")
    args = parser.parse_args()

    truth = validate_and_build()
    if args.verify:
        print("PASS_HEPTA_INTELLIGENCE_CURRENT_TRUTH_V1")
        return 0

    if args.compact:
        encoded = canonical(truth) + b"\n"
    else:
        encoded = (json.dumps(truth, indent=2, sort_keys=True, ensure_ascii=False) + "\n").encode()

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_bytes(encoded)
    else:
        sys.stdout.buffer.write(encoded)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
