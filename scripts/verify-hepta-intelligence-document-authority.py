#!/usr/bin/env python3
"""Verify Hepta Intelligence document authority, registries and compatibility."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys
from typing import Any, NoReturn

ROOT = Path(__file__).resolve().parents[1]
PLAN_DIR = ROOT / "plans" / "hepta-intelligence"
CURRENT = PLAN_DIR / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
MASTER = PLAN_DIR / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
SPEC = PLAN_DIR / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
REGISTRY = PLAN_DIR / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
EVIDENCE = PLAN_DIR / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json"
CAPABILITIES = PLAN_DIR / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json"
PR_STACK = PLAN_DIR / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json"
INTEGRATION = PLAN_DIR / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
Q0 = PLAN_DIR / "HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json"
CLAIMS = PLAN_DIR / "HEPTA_INTELLIGENCE_CLAIM_LADDER_V1.json"
LEGACY_V2 = PLAN_DIR / "HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json"
LEGACY_V3 = PLAN_DIR / "HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json"
SNAPSHOTS = {
    "P0.2": PLAN_DIR
    / "status-snapshots/HEPTA_INTELLIGENCE_P0_2_STATUS_SNAPSHOT_V1.json",
    "P0.3": PLAN_DIR
    / "status-snapshots/HEPTA_INTELLIGENCE_P0_3_STATUS_SNAPSHOT_V1.json",
    "P0.4a": PLAN_DIR
    / "status-snapshots/HEPTA_INTELLIGENCE_P0_4A_STATUS_SNAPSHOT_V1.json",
    "P0.4b": PLAN_DIR
    / "status-snapshots/HEPTA_INTELLIGENCE_P0_4B_STATUS_SNAPSHOT_V1.json",
    "P0.4c": PLAN_DIR
    / "status-snapshots/HEPTA_INTELLIGENCE_P0_4C_STATUS_SNAPSHOT_V1.json",
}


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY: {message}")


def load(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        fail(f"cannot parse {path.relative_to(ROOT)}: {exc}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain an object")
    return value


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def all_false(mapping: Any) -> bool:
    return (
        isinstance(mapping, dict)
        and bool(mapping)
        and all(value is False for value in mapping.values())
    )


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    for path in [
        CURRENT,
        MASTER,
        SPEC,
        REGISTRY,
        EVIDENCE,
        CAPABILITIES,
        PR_STACK,
        INTEGRATION,
        Q0,
        CLAIMS,
        LEGACY_V2,
        LEGACY_V3,
        *SNAPSHOTS.values(),
    ]:
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")

    current = load(CURRENT)
    registry = load(REGISTRY)
    evidence = load(EVIDENCE)
    capabilities = load(CAPABILITIES)
    pr_stack = load(PR_STACK)
    integration = load(INTEGRATION)
    q0 = load(Q0)
    claims = load(CLAIMS)
    legacy_v2 = load(LEGACY_V2)
    legacy_v3 = load(LEGACY_V3)

    require(
        current.get("schema") == "hepta_intelligence_current_plan_v2",
        "current schema drift",
    )
    require(
        current.get("repository") == "ProfHepta/hepta-private-ci",
        "current repository drift",
    )
    require(
        current.get("canonical", {}).get("human_document")
        == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md",
        "master pointer drift",
    )
    require(
        current.get("canonical", {}).get("plan_version") == "4.2.0",
        "master version drift",
    )
    require(
        current.get("canonical", {}).get("content_sha256") == sha256(MASTER),
        "master digest drift",
    )
    operational = current.get("operational_execution", {})
    require(
        operational.get("execution_spec_version") == "1.1.0",
        "execution spec version drift",
    )
    require(
        operational.get("execution_spec_sha256") == sha256(SPEC),
        "execution spec digest drift",
    )
    require(
        operational.get("no_ci_source_writeback") is True,
        "CI writeback prohibition missing",
    )
    require(
        current.get("active_phase", {}).get("active_task")
        == "A0.3_EXACT_PARENT_DOCUMENT_DEEPENING_AND_EXECUTABLE_EVIDENCE",
        "active A0 task drift",
    )
    require(
        current.get("active_phase", {}).get("current_work_unit")
        == "A0.3_REPLACE_BOT_HEAD_AND_OBTAIN_EXACT_HEAD_EXECUTABLE_EVIDENCE",
        "A0 work unit drift",
    )
    require(all_false(current.get("authority")), "current authority must be false")

    bootstrap = current.get("session_bootstrap", {})
    pointers = {
        "document_authority_registry": "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
        "time_bounded_evidence_index": "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
        "capability_registry": "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
        "pr_stack_registry": "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
        "integration_candidate": "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
        "subordinate_execution_spec": "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
    }
    for key, path in pointers.items():
        require(bootstrap.get(key) == path, f"{key} pointer drift")
    require(
        bootstrap.get("legacy_machine_contracts_are_current_authority") is False,
        "legacy contracts gained current authority",
    )
    require(
        bootstrap.get("legacy_machine_contracts_must_remain_compatible_until_migrated")
        is True,
        "compatibility preservation disabled",
    )

    source_policy = current.get("source_snapshot_policy", {})
    require(
        source_policy.get("classification") == "SOURCE_SNAPSHOT_NOT_LIVE_CI",
        "source snapshot policy missing",
    )
    require(
        source_policy.get("live_evidence_may_directly_mutate_source") is False,
        "live evidence may mutate source",
    )
    require(
        source_policy.get("queued_or_incomplete_is_pass") is False,
        "queued/incomplete interpreted as pass",
    )

    require(
        registry.get("schema") == "hepta_intelligence_document_authority_registry_v1",
        "registry schema drift",
    )
    require(
        registry.get("repository") == "ProfHepta/hepta-private-ci",
        "registry repository drift",
    )
    rules = registry.get("rules", {})
    required_true_rules = [
        "single_current_machine_authority",
        "single_current_human_authority",
        "breaking_a_registered_consumer_requires_migration_receipt",
        "unknown_document_fails_closed",
    ]
    for key in required_true_rules:
        require(rules.get(key) is True, f"required registry rule disabled: {key}")
    required_false_rules = [
        "registered_inputs_may_grant_production_authority",
        "compatibility_contracts_are_current_authority",
        "subordinate_operational_document_may_grant_authority",
        "source_snapshot_is_live_evidence",
        "live_evidence_may_directly_mutate_source",
        "pr_body_is_current_truth",
        "workflow_log_is_current_truth",
    ]
    for key in required_false_rules:
        require(rules.get(key) is False, f"registry rule must be false: {key}")

    human = registry.get("current_authority", {}).get("human", {})
    require(
        human.get("path")
        == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md",
        "registry human pointer drift",
    )
    require(human.get("plan_version") == "4.2.0", "registry plan version drift")
    require(human.get("content_sha256") == sha256(MASTER), "registry master digest drift")
    require(
        registry.get("current_authority", {}).get("machine", {}).get("path")
        == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
        "registry machine pointer drift",
    )
    require(all_false(registry.get("authority")), "registry authority must be false")

    operational_docs = registry.get("registered_operational_documents")
    require(
        isinstance(operational_docs, list) and len(operational_docs) == 1,
        "operational document registry mismatch",
    )
    op = operational_docs[0]
    require(
        op.get("path")
        == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
        "execution spec registry path drift",
    )
    require(op.get("version") == "1.1.0", "execution spec registry version drift")
    require(op.get("content_sha256") == sha256(SPEC), "execution spec registry digest drift")
    require(op.get("current_plan_authority") is False, "execution spec gained plan authority")
    require(op.get("promotion_authority") is False, "execution spec gained promotion authority")

    registered_inputs = registry.get("registered_canonical_inputs")
    require(
        isinstance(registered_inputs, list) and len(registered_inputs) == 4,
        "canonical input registry incomplete",
    )
    expected_inputs = {
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json": "hepta_intelligence_evidence_index_v1",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json": "hepta_intelligence_capability_registry_v1",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json": "hepta_intelligence_pr_stack_registry_v1",
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json": "hepta_intelligence_integration_candidate_v1",
    }
    observed_inputs = {
        item.get("path"): item.get("schema")
        for item in registered_inputs
        if isinstance(item, dict)
    }
    require(observed_inputs == expected_inputs, "canonical input surface drift")
    require(
        all(item.get("current_plan_authority") is False for item in registered_inputs),
        "registered input gained plan authority",
    )

    compatibility = registry.get("compatibility_contracts")
    require(
        isinstance(compatibility, list) and len(compatibility) >= 2,
        "compatibility registry incomplete",
    )
    registered_paths = {
        item.get("path") for item in compatibility if isinstance(item, dict)
    }
    require(
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json"
        in registered_paths,
        "legacy V2 missing",
    )
    require(
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json"
        in registered_paths,
        "legacy V3 missing",
    )
    for item in compatibility:
        require(isinstance(item, dict), "compatibility entry malformed")
        require(item.get("current_authority") is False, "legacy gained authority")
        require(item.get("schema_preserved") is True, "legacy schema not preserved")
        consumers = item.get("registered_consumers")
        require(isinstance(consumers, list) and consumers, "legacy consumers missing")
        for consumer in consumers:
            require((ROOT / consumer).exists(), f"registered consumer missing: {consumer}")

    claim_entry = next(
        (
            item
            for item in registry.get("policy_documents", [])
            if isinstance(item, dict)
            and item.get("path")
            == "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CLAIM_LADDER_V1.json"
        ),
        None,
    )
    require(isinstance(claim_entry, dict), "claim ladder not registered")
    require(
        claim_entry.get("current_state_authority") is False,
        "claim ladder gained state authority",
    )
    require(
        claims.get("schema") == "hepta_intelligence_claim_ladder_v1",
        "claim ladder schema drift",
    )
    require(
        claims.get("plan_id") == "HEPTA_INTELLIGENCE_MASTER_PLAN_V4",
        "claim ladder plan binding drift",
    )
    require(all_false(claims.get("authority")), "claim policy authority must be false")

    require(
        legacy_v2.get("schema") == "hepta_intelligence_execution_status_v2",
        "legacy V2 schema replaced",
    )
    require(
        legacy_v3.get("schema") == "hepta_intelligence_execution_status_v3",
        "legacy V3 schema replaced",
    )
    require(all_false(legacy_v2.get("authority")), "legacy V2 authority must be false")
    require(all_false(legacy_v3.get("authority")), "legacy V3 authority must be false")

    require(
        evidence.get("schema") == "hepta_intelligence_evidence_index_v1",
        "evidence schema drift",
    )
    require(evidence.get("not_current_plan_authority") is True, "evidence gained authority")
    require(
        evidence.get("q0_current_evidence", {}).get("summary_file_sha256")
        == sha256(Q0),
        "Q0 evidence digest drift",
    )
    require(all_false(evidence.get("authority")), "evidence authority must be false")

    require(
        capabilities.get("schema") == "hepta_intelligence_capability_registry_v1",
        "capability schema drift",
    )
    require(
        capabilities.get("current_plan_authority") is False,
        "capability registry gained authority",
    )
    require(all_false(capabilities.get("authority")), "capability authority must be false")

    require(
        pr_stack.get("schema") == "hepta_intelligence_pr_stack_registry_v1",
        "PR stack schema drift",
    )
    require(
        pr_stack.get("canonical_stack", {}).get("candidate_head")
        == "c768bcbeb4c1168088d2499828c24da521a2a73a",
        "canonical stack head drift",
    )
    require(all_false(pr_stack.get("authority")), "PR stack authority must be false")

    require(
        integration.get("schema") == "hepta_intelligence_integration_candidate_v1",
        "integration schema drift",
    )
    require(
        integration.get("expected_parent")
        == "c768bcbeb4c1168088d2499828c24da521a2a73a",
        "integration parent drift",
    )
    require(
        integration.get("operational_documents", [])[0].get("content_sha256")
        == sha256(SPEC),
        "integration spec digest drift",
    )
    require(
        integration.get("source_freeze", {}).get("rust_runtime_changes_allowed")
        is False,
        "runtime freeze disabled",
    )
    require(all_false(integration.get("authority")), "integration authority must be false")

    require(
        q0.get("schema") == "hepta_intelligence_q0_external_evidence_summary_v1",
        "Q0 receipt schema drift",
    )
    bound = dict(q0)
    observed = bound.pop("receipt_binding_sha256", None)
    encoded = json.dumps(
        bound, sort_keys=True, separators=(",", ":"), ensure_ascii=False
    ).encode()
    require(observed == hashlib.sha256(encoded).hexdigest(), "Q0 receipt binding drift")
    require(
        q0.get("conclusion", {}).get("qualified_candidate") is True,
        "Q0 qualified candidate missing",
    )
    require(
        q0.get("conclusion", {}).get("runtime_capability_qualified") is False,
        "Q0 gained runtime qualification",
    )
    require(all_false(q0.get("authority")), "Q0 authority must be false")

    for tranche, path in SNAPSHOTS.items():
        snapshot = load(path)
        require(
            snapshot.get("schema") == "hepta_intelligence_tranche_status_snapshot_v1",
            f"snapshot schema drift: {tranche}",
        )
        require(snapshot.get("snapshot_id") == tranche, f"snapshot ID drift: {tranche}")
        require(
            snapshot.get("classification")
            == "IMMUTABLE_QUALIFICATION_COMPATIBILITY_SNAPSHOT",
            f"snapshot class drift: {tranche}",
        )
        require(snapshot.get("current_authority") is False, "snapshot gained authority")
        require(
            snapshot.get("current_tranche", {}).get("qualified") is False,
            f"historical snapshot changed: {tranche}",
        )
        require(all_false(snapshot.get("authority")), "snapshot authority must be false")

    for consumer in registry.get("registered_consumers", []):
        require((ROOT / consumer).exists(), f"current consumer missing: {consumer}")

    print("PASS_HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRIES_AND_COMPATIBILITY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
