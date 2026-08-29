#!/usr/bin/env python3
"""Emit and verify deterministic Hepta Intelligence current truth v1 (V4.4 source)."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import sys
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "plans" / "hepta-intelligence"
PATHS = {
    "current": PLAN / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
    "document": PLAN / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "evidence": PLAN / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "capabilities": PLAN / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "pr_stack": PLAN / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "integration": PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "master": PLAN / "HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    "spec": PLAN / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
    "q0": PLAN / "HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json",
}
EXPECTED_REPOSITORY = "ProfHepta/hepta-private-ci"
EXPECTED_Q0_HEAD = "c768bcbeb4c1168088d2499828c24da521a2a73a"
EXPECTED_Q0_TREE = "ca455a9ef797cd95164c880c7b8faba80b305589"
EXPECTED_A0_BRANCH = "codex/hepta-intelligence-a0-authority-gap-closure-20260829"
EXPECTED_READ_ORDER = [
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
]
ALLOWED_GAP_CLASSES = {
    "CLOSED_SOURCE_CONTROLLED", "OPEN_SOURCE_CONTROLLED",
    "BLOCKED_EXTERNAL_EVIDENCE", "BLOCKED_UPSTREAM", "STOP_CONDITION",
}


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_CURRENT_TRUTH: {message}")


def require(value: bool, message: str) -> None:
    if not value:
        fail(message)


def load(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        fail(f"cannot parse {path.relative_to(ROOT)}: {exc}")
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must contain an object")
    return value


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")


def all_false(value: Any) -> bool:
    return isinstance(value, dict) and bool(value) and all(item is False for item in value.values())


def validate() -> dict[str, Any]:
    for path in PATHS.values():
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")
    current = load(PATHS["current"])
    document = load(PATHS["document"])
    evidence = load(PATHS["evidence"])
    capabilities = load(PATHS["capabilities"])
    pr_stack = load(PATHS["pr_stack"])
    integration = load(PATHS["integration"])
    q0 = load(PATHS["q0"])

    require(current.get("schema") == "hepta_intelligence_current_plan_v2", "current schema")
    require(current.get("repository") == EXPECTED_REPOSITORY, "repository")
    require(current.get("canonical", {}).get("plan_version") == "4.4.0", "plan version")
    require(current.get("canonical", {}).get("content_sha256") == sha256(PATHS["master"]), "master digest")
    operational = current.get("operational_execution", {})
    require(operational.get("execution_spec_version") == "1.2.0", "spec version")
    require(operational.get("execution_spec_sha256") == sha256(PATHS["spec"]), "spec digest")
    require(operational.get("no_ci_source_writeback") is True, "CI writeback")
    require(operational.get("source_publisher_separate_from_evidence_workflow") is True, "publisher/evidence separation")
    require(current.get("session_bootstrap", {}).get("read_order") == EXPECTED_READ_ORDER, "read order")
    require(current.get("generated_at_utc") is None, "source timestamp")
    require(current.get("source_snapshot_policy", {}).get("generated_at_policy") == "COMMIT_BOUND_AT_EXECUTABLE_RECEIPT", "timestamp binding")
    require(current.get("active_phase", {}).get("id") == "A0", "A0 phase")
    require(current.get("active_phase", {}).get("allowed_change_class") == "DOCUMENTATION_REGISTRY_VERIFIER_WORKFLOW_ONLY", "A0 change class")
    require(current.get("stack_budget", {}).get("runtime_source_freeze") is True, "runtime freeze")
    require(current.get("stack_budget", {}).get("expected_parent") == EXPECTED_Q0_HEAD, "A0 parent")
    require(all_false(current.get("authority")), "current authority")

    require(document.get("schema") == "hepta_intelligence_document_authority_registry_v1", "document schema")
    require(document.get("repository") == EXPECTED_REPOSITORY, "document repository")
    require(document.get("current_plan_authority", {}).get("human_plan_version") == "4.4.0", "document plan version")
    require(document.get("current_plan_authority", {}).get("human_plan_content_sha256") == sha256(PATHS["master"]), "document master digest")
    op_docs = document.get("registered_operational_documents")
    require(isinstance(op_docs, list) and len(op_docs) == 1, "operational docs")
    require(op_docs[0].get("version") == "1.2.0", "document spec version")
    require(op_docs[0].get("parent_plan_version") == "4.4.0", "document spec parent")
    require(op_docs[0].get("content_sha256") == sha256(PATHS["spec"]), "document spec digest")
    require(all_false(document.get("authority")), "document authority")

    require(evidence.get("schema") == "hepta_intelligence_evidence_index_v1", "evidence schema")
    require(evidence.get("q0_current_evidence", {}).get("q0_executable_qualified") is True, "Q0 evidence")
    require(evidence.get("q0_current_evidence", {}).get("full_repository_merge_green") is False, "fabricated merge green")
    require(all_false(evidence.get("authority")), "evidence authority")

    require(capabilities.get("schema") == "hepta_intelligence_capability_registry_v1", "capability schema")
    entries = capabilities.get("capabilities")
    require(isinstance(entries, list) and entries, "capabilities")
    by_id = {entry.get("capability_id"): entry for entry in entries if isinstance(entry, dict)}
    require(len(by_id) == len(entries), "duplicate capability")
    require(by_id.get("P0_GROUNDED_MUTATION_FOUNDATION", {}).get("candidate_qualified") is True, "P0 qualification")
    require(by_id.get("A0_CANONICAL_CAPABILITY_AND_EVIDENCE_AUTHORITY", {}).get("candidate_qualified") is False, "A0 source self-qualified")
    for mm_id in (
        "MM0_MULTIMODAL_MEMORY_CONTRACTS", "MM1_ASSET_DERIVATION_LEDGER",
        "MM2_SANDBOXED_EXTRACTION", "MM3_MULTIMODAL_EMBEDDING_INDEX",
        "MM4_CROSS_MODAL_RETRIEVAL", "MM5_MULTIMODAL_CONSOLIDATION_CONTEXT",
        "MM6_MULTIMODAL_SECURITY_EFFICACY",
    ):
        require(mm_id in by_id, f"missing capability {mm_id}")
        require(by_id[mm_id].get("wired") is False and by_id[mm_id].get("runtime_qualified") is False, f"{mm_id} crossed runtime boundary")
    for name, entry in by_id.items():
        require(all_false(entry.get("authority")), f"{name} authority")
    require(all_false(capabilities.get("authority")), "capability registry authority")

    require(pr_stack.get("schema") == "hepta_intelligence_pr_stack_registry_v1", "PR stack schema")
    canonical_stack = pr_stack.get("canonical_stack", {})
    require(canonical_stack.get("candidate_head") == EXPECTED_Q0_HEAD, "stack Q0 head")
    require(canonical_stack.get("candidate_tree") == EXPECTED_Q0_TREE, "stack Q0 tree")
    require([entry.get("pr_number") for entry in canonical_stack.get("entries", [])] == [7, 13, 14, 16, 21, 23, 29], "canonical stack order")
    require(all_false(pr_stack.get("authority")), "PR stack authority")

    require(integration.get("schema") == "hepta_intelligence_integration_candidate_v1", "integration schema")
    require(integration.get("repository") == EXPECTED_REPOSITORY, "integration repository")
    require(integration.get("branch") == EXPECTED_A0_BRANCH, "integration branch")
    require(integration.get("expected_parent") == EXPECTED_Q0_HEAD, "integration parent")
    allowed = integration.get("allowed_changed_paths")
    require(isinstance(allowed, list) and allowed == sorted(set(allowed)), "allowlist")
    require(len(allowed) == integration.get("expected_changed_path_count") == 17, "allowlist count")
    freeze = integration.get("source_freeze", {})
    for key in ("rust_runtime_changes_allowed", "sql_migrations_allowed", "product_callers_allowed", "h5_h6_h7_runtime_allowed", "model_provider_effects_allowed", "multimodal_runtime_allowed"):
        require(freeze.get(key) is False, f"source freeze {key}")
    gaps = integration.get("gap_closure_ledger", {}).get("entries")
    require(isinstance(gaps, list) and len(gaps) == 17, "global gap ledger")
    global_ids = [entry.get("gap_id") for entry in gaps if isinstance(entry, dict)]
    require(len(set(global_ids)) == 17, "duplicate global gap")
    for entry in gaps:
        require(entry.get("classification") in ALLOWED_GAP_CLASSES, f"{entry.get('gap_id')}: global classification")
    mm = integration.get("multimodal_memory_gap_ledger", {})
    require(mm.get("schema") == "hepta_multimodal_memory_gap_ledger_v2", "multimodal schema")
    mm_entries = mm.get("entries")
    require(isinstance(mm_entries, list) and len(mm_entries) == 42, "multimodal gap count")
    mm_ids = [entry.get("gap_id") for entry in mm_entries if isinstance(entry, dict)]
    require(len(mm_ids) == len(set(mm_ids)) == 42, "multimodal gap identities")
    for entry in mm_entries:
        require(entry.get("repository_status") in ALLOWED_GAP_CLASSES, f"{entry.get('gap_id')}: multimodal status")
    boundary = mm.get("claim_boundary", {})
    require(boundary.get("multimodal_memory") == "MM0_SPECIFIED_ONLY", "multimodal claim")
    require(boundary.get("native_media_memory_wired") is False, "native media wiring")
    require(boundary.get("cross_modal_retrieval_qualified") is False, "cross-modal qualification")
    require(boundary.get("multimodal_efficacy_proven") is False, "multimodal efficacy")
    require(all(value is False for value in mm.get("authority", {}).values()), "multimodal authority")
    require(all_false(integration.get("authority")), "integration authority")

    observation = q0.get("evidence_observation", {})
    require(observation.get("head") == EXPECTED_Q0_HEAD, "Q0 receipt head")
    require(observation.get("tree") == EXPECTED_Q0_TREE, "Q0 receipt tree")
    require(q0.get("conclusion", {}).get("q0_executable_qualified") is True, "Q0 receipt")
    require(q0.get("conclusion", {}).get("runtime_capability_qualified") is False, "Q0 runtime")
    require(all_false(q0.get("authority")), "Q0 authority")

    open_global = [{"gap_id": item.get("gap_id"), "classification": item.get("classification"), "status": item.get("status"), "next_action": item.get("next_action")} for item in gaps if item.get("classification") != "CLOSED_SOURCE_CONTROLLED"]
    open_mm = [{"gap_id": item.get("gap_id"), "classification": item.get("repository_status"), "status": item.get("payload_status"), "next_action": item.get("next_action")} for item in mm_entries if item.get("repository_status") != "CLOSED_SOURCE_CONTROLLED"]
    return {
        "schema": "hepta_intelligence_current_truth_v1",
        "repository": EXPECTED_REPOSITORY,
        "source_snapshot": {"as_of_utc": None, "classification": "SOURCE_SNAPSHOT_NOT_LIVE_CI", "timestamp_policy": "COMMIT_BOUND_AT_EXECUTABLE_RECEIPT", "live_evidence_embedded": False},
        "plan": {"id": current["canonical"]["plan_id"], "version": current["canonical"]["plan_version"], "content_sha256": current["canonical"]["content_sha256"], "execution_spec_version": operational["execution_spec_version"], "execution_spec_sha256": operational["execution_spec_sha256"]},
        "active_phase": current["active_phase"],
        "q0": {"candidate": {"branch": observation.get("branch"), "head": observation.get("head"), "tree": observation.get("tree"), "parent": observation.get("parent")}, "run_id": observation.get("run_id"), "qualified_candidate": True, "runtime_capability_qualified": False, "full_repository_merge_green": False},
        "integration_candidate": {"candidate_id": integration.get("candidate_id"), "branch": integration.get("branch"), "expected_parent": integration.get("expected_parent"), "allowed_changed_paths": allowed, "source_freeze": freeze},
        "claims": current.get("claim_levels"),
        "open_gaps": open_global,
        "open_multimodal_memory_gaps": open_mm,
        "gap_counts": {"global": len(gaps), "multimodal_memory": len(mm_entries)},
        "input_sha256": {name: sha256(path) for name, path in PATHS.items()},
        "authority": current["authority"],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify", action="store_true")
    parser.add_argument("--compact", action="store_true")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    truth = validate()
    if args.verify:
        print("PASS_HEPTA_INTELLIGENCE_CURRENT_TRUTH_V1")
        return 0
    encoded = canonical(truth) + b"\n" if args.compact else (json.dumps(truth, indent=2, sort_keys=True, ensure_ascii=False) + "\n").encode()
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_bytes(encoded)
    else:
        sys.stdout.buffer.write(encoded)
    return 0


if __name__ == "__main__":
    sys.exit(main())
