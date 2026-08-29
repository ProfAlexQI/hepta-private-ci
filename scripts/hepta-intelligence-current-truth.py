#!/usr/bin/env python3
"""Strict, deterministic Hepta Intelligence current-truth validator/emitter."""
from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "plans" / "hepta-intelligence"
REPOSITORY = 'ProfHepta/hepta-private-ci'
Q0_HEAD = 'c768bcbeb4c1168088d2499828c24da521a2a73a'
Q0_TREE = 'ca455a9ef797cd95164c880c7b8faba80b305589'
Q0_PARENT = 'aeb8ac0bfb30d570a16c4914b6e4b31ce035dd62'
A0_BRANCH = 'codex/hepta-intelligence-a0-authority-gap-closure-20260829'
PLAN_VERSION = '4.5.1'
SPEC_VERSION = '1.3.0'
ALLOWED_PATHS = ['.github/workflows/hepta-intelligence-a0-authority.yml', '.github/workflows/hepta-intelligence-execution-spec.yml', '.github/workflows/hepta-intelligence-master-plan.yml', 'plans/hepta-intelligence/AGENTS.md', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json', 'scripts/hepta-intelligence-current-truth.py', 'scripts/verify-hepta-intelligence-a0-authority.py', 'scripts/verify-hepta-intelligence-document-authority.py', 'scripts/verify-hepta-intelligence-master-plan.py']
READ_ORDER = ['plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md', 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md']
LIFECYCLE = ['specified', 'implemented', 'candidate_qualified', 'selected', 'wired', 'runtime_qualified', 'efficacy_proven', 'operator_accepted', 'promoted', 'released']
GAP_CLASSES = set(['CLOSED_SOURCE_CONTROLLED', 'OPEN_SOURCE_CONTROLLED', 'BLOCKED_EXTERNAL_EVIDENCE', 'BLOCKED_UPSTREAM', 'STOP_CONDITION'])
CHECK_CLASSES = ['PASS', 'INTRODUCED_BY_CANDIDATE', 'PRE_EXISTING_ON_BASE', 'MERGE_INTERACTION', 'RUNNER_OR_PLATFORM_INFRA', 'CANCELLED_OR_SUPERSEDED', 'NOT_REQUIRED_BY_SELECTED_POLICY', 'UNKNOWN_FAIL_CLOSED']
GLOBAL_GAP_IDS = ['A0-DOC-001', 'A0-TRIGGER-001', 'A0-EVIDENCE-001', 'A0-REVIEW-001', 'A0-MERGE-001', 'B0-BOUNDARY-001', 'C0-LEDGER-001', 'M0-COORDINATOR-001', 'J0-LIFECYCLE-001', 'R1-RETRIEVAL-001', 'R1-CORPUS-001', 'N1-NEURON-001', 'I1-INTUITION-001', 'L1-LEARNING-001', 'C1-CLOSED-LOOP-001', 'EXT-HARDWARE-001', 'EXT-OPERATOR-001']
AUTHORITY_FALSE = {'callers_ratchet': False, 'default_open_wired': False, 'external_effects': False, 'learning_write_authority': False, 'memory_write_authority': False, 'model_runtime_authority': False, 'operator_acceptance': False, 'outbox_dispatch_authority': False, 'product_module_registered': False, 'production_authority': False, 'projection_write_authority': False, 'promotion': False, 'provider_dispatch_authority': False, 'release_authority': False, 'runtime_wired': False, 'tool_registered': False}

PATHS = {
    "current": PLAN / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
    "document": PLAN / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "evidence": PLAN / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "capability": PLAN / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "pr_stack": PLAN / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "integration": PLAN / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "master": PLAN / "HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    "spec": PLAN / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
    "q0": PLAN / "HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json",
    "claims": PLAN / "HEPTA_INTELLIGENCE_CLAIM_LADDER_V1.json",
}


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_CURRENT_TRUTH: {message}")


def pairs(items: list[tuple[str, Any]]) -> dict[str, Any]:
    out: dict[str, Any] = {}
    for key, value in items:
        if key in out:
            fail(f"duplicate JSON key: {key}")
        out[key] = value
    return out


def load(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=pairs)
    except SystemExit:
        raise
    except Exception as exc:
        fail(f"cannot parse {path.relative_to(ROOT)}: {exc}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain an object")
    return value


def canonical(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(value: bool, message: str) -> None:
    if not value:
        fail(message)


def exact_keys(value: dict[str, Any], required: set[str], allowed: set[str], label: str) -> None:
    missing = required - set(value)
    unknown = set(value) - allowed
    require(not missing, f"{label} missing fields: {sorted(missing)}")
    require(not unknown, f"{label} unknown fields: {sorted(unknown)}")


def all_false(value: Any) -> bool:
    return isinstance(value, dict) and value and all(item is False for item in value.values())


def verify_binding(doc: dict[str, Any], field: str, label: str) -> None:
    work = copy.deepcopy(doc)
    observed = work.pop(field, None)
    require(isinstance(observed, str) and re.fullmatch(r"[0-9a-f]{64}", observed) is not None, f"{label} binding missing")
    require(hashlib.sha256(canonical(work)).hexdigest() == observed, f"{label} binding mismatch")


def validate_gap_entries(entries: Any, exact_ids: list[str] | None, label: str) -> list[dict[str, Any]]:
    require(isinstance(entries, list) and entries, f"{label} entries missing")
    required = {"gap_id", "title", "classification", "status", "owner_class", "blocked_by", "acceptance_tests", "receipt_type", "closure_evidence", "next_action", "authority_effect", "rollback_pointer", "resume_predicate"}
    allowed = required | {"payload_status"}
    seen: dict[str, dict[str, Any]] = {}
    for raw in entries:
        require(isinstance(raw, dict), f"{label} entry must be object")
        exact_keys(raw, required, allowed, f"{label} entry")
        gap_id = raw["gap_id"]
        require(isinstance(gap_id, str) and gap_id, f"{label} gap_id invalid")
        require(gap_id not in seen, f"{label} duplicate gap: {gap_id}")
        require(raw["classification"] in GAP_CLASSES, f"{gap_id} invalid classification")
        require(isinstance(raw["blocked_by"], list), f"{gap_id} blocked_by must be list")
        require(isinstance(raw["acceptance_tests"], list) and raw["acceptance_tests"], f"{gap_id} acceptance tests missing")
        require(isinstance(raw["closure_evidence"], list) and raw["closure_evidence"], f"{gap_id} closure evidence missing")
        require(raw["title"] != gap_id and raw["next_action"] != "DoD" and raw["resume_predicate"] != "deps", f"{gap_id} placeholder semantics")
        seen[gap_id] = raw
    if exact_ids is not None:
        require(list(seen) == exact_ids, f"{label} exact ID/order drift: {list(seen)}")
    for gap_id, raw in seen.items():
        for dep in raw["blocked_by"]:
            require(dep in seen or label.startswith("multimodal"), f"{gap_id} unknown dependency {dep}")
    # Acyclic graph over dependencies defined in this ledger.
    visiting: set[str] = set()
    visited: set[str] = set()
    def walk(node: str) -> None:
        if node in visited:
            return
        require(node not in visiting, f"{label} dependency cycle at {node}")
        visiting.add(node)
        for dep in seen[node]["blocked_by"]:
            if dep in seen:
                walk(dep)
        visiting.remove(node)
        visited.add(node)
    for node in seen:
        walk(node)
    return list(seen.values())


def validate_and_build() -> dict[str, Any]:
    for path in PATHS.values():
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")
    current = load(PATHS["current"])
    document = load(PATHS["document"])
    evidence = load(PATHS["evidence"])
    capability = load(PATHS["capability"])
    pr_stack = load(PATHS["pr_stack"])
    integration = load(PATHS["integration"])
    q0 = load(PATHS["q0"])
    claims = load(PATHS["claims"])
    master_text = PATHS["master"].read_text(encoding="utf-8")
    spec_text = PATHS["spec"].read_text(encoding="utf-8")

    require(current.get("schema") == "hepta_intelligence_current_plan_v2", "current schema")
    require(document.get("schema") == "hepta_intelligence_document_authority_registry_v1", "document schema")
    require(evidence.get("schema") == "hepta_intelligence_evidence_index_v1", "evidence schema")
    require(capability.get("schema") == "hepta_intelligence_capability_registry_v1", "capability schema")
    require(pr_stack.get("schema") == "hepta_intelligence_pr_stack_registry_v1", "PR stack schema")
    require(integration.get("schema") == "hepta_intelligence_integration_candidate_v1", "integration schema")
    for label, doc in (("current", current), ("document", document), ("evidence", evidence), ("capability", capability), ("pr_stack", pr_stack), ("integration", integration)):
        require(doc.get("repository") == REPOSITORY, f"{label} repository")
    for label, mapping in (("current", current.get("authority")), ("document", document.get("authority")), ("evidence", evidence.get("authority")), ("capability", capability.get("authority")), ("pr_stack", pr_stack.get("authority")), ("integration", integration.get("authority")), ("q0", q0.get("authority")), ("claims", claims.get("authority"))):
        require(all_false(mapping), f"{label} authority must remain all false")

    canonical_doc = current.get("canonical", {})
    require(canonical_doc.get("plan_version") == PLAN_VERSION, "plan version drift")
    require(canonical_doc.get("content_sha256") == sha(PATHS["master"]), "master digest drift")
    operational = current.get("operational_execution", {})
    require(operational.get("execution_spec_version") == SPEC_VERSION, "spec version drift")
    require(operational.get("execution_spec_sha256") == sha(PATHS["spec"]), "spec digest drift")
    require(current.get("session_bootstrap", {}).get("read_order") == READ_ORDER, "read order drift")
    require(current.get("active_phase", {}).get("active_task") == "A0.3_V4_5_SOURCE_CONTRACT_CLOSURE", "active task drift")
    actions = current.get("next_actions")
    require(isinstance(actions, list) and actions and actions[0].get("id") == "A0.3a", "next action identity drift")
    require(actions[0].get("action") == "Publish V4.5.1 as one exact-parent replacement", "next action version drift")
    require(current.get("stack_budget", {}).get("reason") == "A0 V4.5.1 exact-parent replacement requires executable evidence, independent review, canonical selection and merge-candidate admission", "stack budget reason drift")
    require(current.get("a0_previous_exact_head_provenance") == ["241daa85253a6224a9473a1a2b2967f71e9b46af", "88b355931456603bd7a1dd9e0cdfd4d0d035dc0e", "0b80caff91010f40a79c795c20487ff9d773d229", "0a93b172e575670d20c30e2efa7f312097466fe6", "d93050c48e2943bb2be46e5f610c1bc109498194", "1146f0290fbbff7e6a26f6b91c80ec1e1daa0b60"], "A0 provenance drift")
    require("V4.4" not in json.dumps(current, sort_keys=True), "stale V4.4 current-plan semantics")
    require(f"Version: `{PLAN_VERSION}`" in master_text, "master version marker")
    require(f"Version: `{SPEC_VERSION}`" in spec_text, "spec version marker")
    require(f"canonical parent plan: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4` version `{PLAN_VERSION}`".lower() in spec_text.lower(), "spec parent version drift")
    require("PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_5_SOURCE_ONLY" in spec_text, "spec expected plan marker drift")

    precedence = document.get("precedence", {})
    require(precedence.get("read_order") == READ_ORDER, "document read order drift")
    inputs = document.get("registered_canonical_inputs")
    require(isinstance(inputs, list) and len(inputs) == len(READ_ORDER), "registered input count")
    require([item.get("path") for item in inputs] == READ_ORDER, "registered input order")
    input_required = {"path", "role", "schema", "content_sha256", "digest_scope", "mutation_policy", "conflict_rule", "current_plan_authority"}
    for item in inputs:
        require(isinstance(item, dict), "registered input object")
        exact_keys(item, input_required, input_required, f"input {item.get('path')}")
        path = ROOT / item["path"]
        require(path.is_file(), f"registered input missing: {item['path']}")
        require(isinstance(item["content_sha256"], str) and re.fullmatch(r"[0-9a-f]{64}", item["content_sha256"]) is not None, f"input digest invalid: {item['path']}")
        if item["digest_scope"] == "RAW_FILE_BYTES":
            require(item["content_sha256"] == sha(path), f"input digest drift: {item['path']}")
        elif item["digest_scope"] == "CANONICAL_JSON_WITH_SELF_DIGEST_NULL":
            work = copy.deepcopy(document)
            self_item = next(x for x in work["registered_canonical_inputs"] if x["path"] == item["path"])
            observed = self_item["content_sha256"]
            self_item["content_sha256"] = None
            require(hashlib.sha256(canonical(work)).hexdigest() == observed, "document self digest drift")
        else:
            fail(f"unknown digest scope: {item['digest_scope']}")
    require(sum(bool(item["current_plan_authority"]) for item in inputs) == 1, "exactly one aggregate machine authority required")
    require(next(item for item in inputs if item["current_plan_authority"])["path"] == READ_ORDER[0], "wrong aggregate authority")

    compatibility = document.get("compatibility_contracts")
    require(isinstance(compatibility, list) and len(compatibility) == 2, "compatibility contracts")
    for item in compatibility:
        required = {"path", "schema", "content_sha256", "registered_consumers", "schema_preserved", "current_authority", "mutation_policy"}
        exact_keys(item, required, required, f"compatibility {item.get('path')}")
        path = ROOT / item["path"]
        doc = load(path)
        require(doc.get("schema") == item["schema"], f"compatibility schema drift: {item['path']}")
        require(sha(path) == item["content_sha256"], f"compatibility digest drift: {item['path']}")
        require(item["schema_preserved"] is True and item["current_authority"] is False, "compatibility authority/schema")
        require(isinstance(item["registered_consumers"], list) and item["registered_consumers"], "compatibility consumers")
        for consumer in item["registered_consumers"]:
            require((ROOT / consumer).exists(), f"registered consumer missing: {consumer}")
    snapshots = document.get("immutable_tranche_snapshots")
    require(isinstance(snapshots, list) and len(snapshots) >= 5, "tranche snapshots")
    seen_snapshots: set[str] = set()
    for item in snapshots:
        path = ROOT / item["path"]
        snap = load(path)
        require(item["path"] not in seen_snapshots, "duplicate snapshot")
        seen_snapshots.add(item["path"])
        require(sha(path) == item["content_sha256"], f"snapshot digest drift: {item['path']}")
        require(snap.get("schema") == item["schema"] and snap.get("snapshot_id") == item["snapshot_id"], f"snapshot identity drift: {item['path']}")
        require(snap.get("current_authority") is False and item["current_authority"] is False, f"snapshot authority: {item['path']}")
        if item.get("source_ref") is not None:
            require(snap.get("source_ref") == item["source_ref"], f"snapshot source ref: {item['path']}")
        if item.get("source_status_blob_sha") is not None:
            require(snap.get("source_status_blob_sha") == item["source_status_blob_sha"], f"snapshot blob: {item['path']}")
    claim_policy = document.get("claim_ladder_policy", {})
    require(claim_policy.get("path") == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CLAIM_LADDER_V1.json", "claim policy path")
    require(claim_policy.get("content_sha256") == sha(PATHS["claims"]), "claim policy digest")
    require(claim_policy.get("current_state_authority") is False, "claim policy authority")

    verify_binding(q0, "receipt_binding_sha256", "Q0 receipt")
    q0_obs = q0.get("evidence_observation", {})
    require(q0_obs.get("head") == Q0_HEAD and q0_obs.get("tree") == Q0_TREE and q0_obs.get("parent") == Q0_PARENT, "Q0 identity")
    require(q0.get("conclusion", {}).get("qualified_candidate") is True, "Q0 candidate qualification")
    q0_index = evidence.get("q0_current_evidence", {})
    require(q0_index.get("summary_file_sha256") == sha(PATHS["q0"]), "Q0 index cross hash")
    entries = evidence.get("entries")
    require(isinstance(entries, list) and entries, "evidence entries")
    ids = [item.get("id") for item in entries if isinstance(item, dict)]
    require(len(ids) == len(set(ids)), "duplicate evidence ID")
    required_history = {"PLAN_V4_HOSTED_SOURCE_1099821C", "PLAN_V4_HOSTED_SOURCE_80BE14B1", "DOC_COMPAT_REGRESSION_80BE14B1", "P0_4B_RUST_FMT_80BE14B1"}
    require(required_history.issubset(set(ids)), "historical PASS/failure evidence was deleted")
    require(evidence.get("append_only_policy", {}).get("existing_entry_deletion_forbidden") is True, "append-only policy")

    require(capability.get("lifecycle") == LIFECYCLE, "capability lifecycle drift")
    capability_entries = capability.get("capabilities")
    require(isinstance(capability_entries, list) and capability_entries, "capability entries")
    cap_ids: set[str] = set()
    for entry in capability_entries:
        cap_id = entry.get("capability_id")
        require(isinstance(cap_id, str) and cap_id not in cap_ids, "capability ID duplicate/missing")
        cap_ids.add(cap_id)
        for field in LIFECYCLE:
            require(type(entry.get(field)) is bool, f"{cap_id}.{field} must be bool")
        receipts = entry.get("lifecycle_receipts")
        require(isinstance(receipts, dict) and set(receipts) == set(LIFECYCLE), f"{cap_id} receipt slots")
        implications = [
            ("implemented", "specified"),
            ("candidate_qualified", "implemented"),
            ("selected", "candidate_qualified"),
            ("wired", "selected"),
            ("runtime_qualified", "wired"),
            ("efficacy_proven", "runtime_qualified"),
            ("operator_accepted", "efficacy_proven"),
            ("promoted", "operator_accepted"),
            ("released", "promoted"),
        ]
        for child, parent in implications:
            require(not entry[child] or entry[parent], f"{cap_id} invalid lifecycle {child} without {parent}")
        for field in ("candidate_qualified", "selected", "wired", "runtime_qualified", "efficacy_proven", "operator_accepted", "promoted", "released"):
            require(not entry[field] or isinstance(receipts[field], str), f"{cap_id} {field} lacks receipt")
        require(all_false(entry.get("authority")), f"{cap_id} authority")

    require(integration.get("allowed_changed_paths") == ALLOWED_PATHS, "A0 path manifest drift")
    require(integration.get("expected_changed_path_count") == len(ALLOWED_PATHS), "path count drift")
    require(integration.get("expected_parent") == Q0_HEAD, "A0 expected parent")
    require(integration.get("repository_check_classifications") == CHECK_CLASSES, "check classification vocabulary")
    global_entries = validate_gap_entries(integration.get("gap_closure_ledger", {}).get("entries"), GLOBAL_GAP_IDS, "global")
    mm = integration.get("multimodal_memory_gap_ledger", {})
    mm_entries = validate_gap_entries(mm.get("entries"), None, "multimodal")
    global_ids = {item["gap_id"] for item in global_entries}
    mm_ids = {item["gap_id"] for item in mm_entries}
    inherited = mm.get("inherited_global_gap_ids")
    require(isinstance(inherited, list), "multimodal inherited refs")
    require(not (global_ids & mm_ids), "shared gap IDs redefined in multimodal ledger")
    require(set(inherited).issubset(global_ids), "unknown inherited global gap")
    require(mm.get("projected_gap_count") == len(mm_ids) + len(inherited), "multimodal projected count")
    registry = integration.get("gap_identity_registry", {})
    require(registry.get("duplicate_definition_count") == 0, "gap duplicate definitions")
    require(registry.get("global_gap_ids") == sorted(global_ids), "global identity registry")
    require(registry.get("multimodal_gap_ids") == sorted(mm_ids), "multimodal identity registry")

    # One exact trigger surface across all three read-only workflows.
    workflow_paths = [
        ROOT / ".github/workflows/hepta-intelligence-a0-authority.yml",
        ROOT / ".github/workflows/hepta-intelligence-execution-spec.yml",
        ROOT / ".github/workflows/hepta-intelligence-master-plan.yml",
    ]
    for path in workflow_paths:
        text = path.read_text(encoding="utf-8")
        for rel in ALLOWED_PATHS:
            require(rel in text, f"workflow trigger missing {rel} in {path.name}")
        for forbidden in ("contents: write", "git push", "git commit", "git update-ref", "create-pull-request"):
            require(forbidden not in text, f"source-write capability in {path.name}: {forbidden}")
        require("a0_candidate_qualified = true" not in text.lower(), f"workflow self qualification in {path.name}")
        require("a0_candidate_qualified: true" not in text.lower(), f"workflow self qualification in {path.name}")
    require(operational.get("canonical_trigger_path_manifest_sha256") == hashlib.sha256(("\n".join(ALLOWED_PATHS) + "\n").encode()).hexdigest(), "path manifest digest")

    source_times = [current.get("generated_at_utc"), document.get("generated_at_utc"), integration.get("gap_closure_ledger", {}).get("as_of_utc")]
    require(all(value is None for value in source_times), "checked-in source snapshot contains live/future timestamp")

    truth = {
        "schema": "hepta_intelligence_current_truth_v1",
        "repository": REPOSITORY,
        "source_snapshot": {"as_of_utc": None, "binding": "COMMIT_AND_EXTERNAL_EVIDENCE_RECEIPT"},
        "plan": {"version": PLAN_VERSION, "sha256": sha(PATHS["master"])},
        "execution_spec": {"version": SPEC_VERSION, "sha256": sha(PATHS["spec"])},
        "read_order": READ_ORDER,
        "active_phase": current.get("active_phase"),
        "q0": {"head": Q0_HEAD, "tree": Q0_TREE, "parent": Q0_PARENT, "candidate_qualified": True, "runtime_qualified": False},
        "a0": {
            "branch": A0_BRANCH,
            "expected_parent": Q0_HEAD,
            "source_evidence_pending": True,
            "candidate_qualified": False,
            "selected": False,
            "full_repository_merge_green": False,
        },
        "capabilities": {entry["capability_id"]: {field: entry[field] for field in LIFECYCLE} for entry in capability_entries},
        "gap_counts": {"global": len(global_entries), "multimodal_unique": len(mm_entries), "multimodal_inherited": len(inherited), "multimodal_projected": mm.get("projected_gap_count")},
        "evidence_history_count": len(entries),
        "authority": copy.deepcopy(AUTHORITY_FALSE),
        "input_sha256": {key: sha(path) for key, path in PATHS.items()},
    }
    return truth


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify", action="store_true")
    parser.add_argument("--compact", action="store_true")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    truth = validate_and_build()
    if args.verify:
        print("PASS_HEPTA_INTELLIGENCE_CURRENT_TRUTH_V1")
        return 0
    encoded = (canonical(truth) if args.compact else json.dumps(truth, indent=2, sort_keys=True, ensure_ascii=False).encode()) + b"\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_bytes(encoded)
    else:
        sys.stdout.buffer.write(encoded)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
