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
    "document_authority": PLAN_DIR
    / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
    "evidence": PLAN_DIR / "HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json",
    "capabilities": PLAN_DIR / "HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json",
    "pr_stack": PLAN_DIR / "HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json",
    "integration_candidate": PLAN_DIR
    / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
    "master_plan": PLAN_DIR / "HEPTA_INTELLIGENCE_MASTER_PLAN.md",
    "execution_spec": PLAN_DIR
    / "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
    "q0_receipt": PLAN_DIR
    / "HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json",
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
    return json.dumps(
        value, sort_keys=True, separators=(",", ":"), ensure_ascii=False
    ).encode()


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def all_false(value: Any) -> bool:
    return (
        isinstance(value, dict)
        and bool(value)
        and all(item is False for item in value.values())
    )


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
    require(
        not entry["runtime_qualified"] or entry["wired"],
        f"{capability_id}: runtime qualification without wiring",
    )
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
    require(all_false(entry.get("authority")), f"{capability_id}: authority must be false")


def validate_gap_ledger(integration: dict[str, Any]) -> list[dict[str, Any]]:
    ledger = integration.get("gap_closure_ledger")
    require(isinstance(ledger, dict), "gap ledger missing")
    require(
        ledger.get("schema") == "hepta_intelligence_gap_closure_ledger_v1",
        "gap ledger schema mismatch",
    )
    invariants = ledger.get("invariants")
    require(isinstance(invariants, dict), "gap invariants missing")
    for key in (
        "external_evidence_may_be_synthesized",
        "fixture_may_close_external_gap",
        "positive_authority_allowed",
        "live_evidence_may_directly_mutate_source",
        "source_snapshot_is_live_evidence",
    ):
        require(invariants.get(key) is False, f"gap invariant must be false: {key}")
    for key in (
        "source_closed_requires_exact_head_executable_evidence",
        "unknown_gap_fails_closed",
    ):
        require(invariants.get(key) is True, f"gap invariant must be true: {key}")

    entries = ledger.get("entries")
    require(isinstance(entries, list) and len(entries) == 17, "gap surface mismatch")
    ids = [entry.get("gap_id") for entry in entries if isinstance(entry, dict)]
    require(len(ids) == len(entries) == len(set(ids)), "duplicate/malformed gap ID")
    allowed_classes = {
        "CLOSED_SOURCE_CONTROLLED",
        "OPEN_SOURCE_CONTROLLED",
        "BLOCKED_EXTERNAL_EVIDENCE",
        "BLOCKED_UPSTREAM",
        "STOP_CONDITION",
    }
    for entry in entries:
        gap_id = entry["gap_id"]
        require(entry.get("classification") in allowed_classes, f"{gap_id}: invalid class")
        for field in (
            "title",
            "owner_class",
            "status",
            "closure_evidence",
            "next_action",
            "authority_effect",
            "resume_predicate",
        ):
            require(field in entry, f"{gap_id}: missing {field}")
        require(
            isinstance(entry["closure_evidence"], list)
            and bool(entry["closure_evidence"]),
            f"{gap_id}: closure evidence missing",
        )
    return entries


def validate_and_build() -> dict[str, Any]:
    for path in PATHS.values():
        require(path.is_file(), f"missing {path.relative_to(ROOT)}")

    docs = {key: load(path) for key, path in PATHS.items() if key in SCHEMAS}
    for key, schema in SCHEMAS.items():
        require(docs[key].get("schema") == schema, f"{key} schema mismatch")

    current = docs["current"]
    document = docs["document_authority"]
    evidence = docs["evidence"]
    capabilities = docs["capabilities"]
    pr_stack = docs["pr_stack"]
    integration = docs["integration_candidate"]
    q0 = docs["q0_receipt"]

    repository = current.get("repository")
    require(repository == "ProfHepta/hepta-private-ci", "repository identity mismatch")
    for label, value in (
        ("document", document),
        ("evidence", evidence),
        ("capabilities", capabilities),
        ("PR stack", pr_stack),
        ("integration", integration),
    ):
        require(value.get("repository") == repository, f"{label} repository mismatch")

    canonical_plan = current.get("canonical")
    require(isinstance(canonical_plan, dict), "canonical plan missing")
    require(
        canonical_plan.get("plan_id") == "HEPTA_INTELLIGENCE_MASTER_PLAN_V4",
        "plan ID mismatch",
    )
    require(canonical_plan.get("plan_version") == "4.2.0", "plan version mismatch")
    require(
        canonical_plan.get("content_sha256") == sha256(PATHS["master_plan"]),
        "master plan digest mismatch",
    )

    operational = current.get("operational_execution")
    require(isinstance(operational, dict), "operational execution missing")
    require(
        operational.get("execution_spec_version") == "1.1.0",
        "execution spec version mismatch",
    )
    require(
        operational.get("execution_spec_sha256") == sha256(PATHS["execution_spec"]),
        "execution spec digest mismatch",
    )
    require(operational.get("no_ci_source_writeback") is True, "CI writeback enabled")
    require(
        operational.get("source_publisher_separate_from_evidence_workflow") is True,
        "publisher/evidence identity separation disabled",
    )

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

    source_policy = current.get("source_snapshot_policy")
    require(isinstance(source_policy, dict), "source snapshot policy missing")
    require(
        source_policy.get("classification") == "SOURCE_SNAPSHOT_NOT_LIVE_CI",
        "source snapshot classification mismatch",
    )
    require(
        source_policy.get("live_evidence_may_directly_mutate_source") is False,
        "live evidence may mutate source",
    )
    require(
        source_policy.get("live_observation_requires_exact_receipt") is True,
        "live evidence receipt gate disabled",
    )
    require(
        source_policy.get("queued_or_incomplete_is_pass") is False,
        "queued/incomplete interpreted as pass",
    )

    q0_binding = dict(q0)
    observed_q0_binding = q0_binding.pop("receipt_binding_sha256", None)
    require(
        observed_q0_binding == hashlib.sha256(canonical(q0_binding)).hexdigest(),
        "Q0 summary binding mismatch",
    )
    q0_observation = q0.get("evidence_observation", {})
    require(
        q0_observation.get("branch") == "codex/hepta-intelligence-plan-v3-20260828",
        "Q0 branch mismatch",
    )
    require(
        q0_observation.get("head") == "c768bcbeb4c1168088d2499828c24da521a2a73a",
        "Q0 head mismatch",
    )
    require(
        q0_observation.get("tree") == "ca455a9ef797cd95164c880c7b8faba80b305589",
        "Q0 tree mismatch",
    )
    require(
        q0_observation.get("parent") == "aeb8ac0bfb30d570a16c4914b6e4b31ce035dd62",
        "Q0 parent mismatch",
    )
    require(
        q0.get("conclusion", {}).get("q0_executable_qualified") is True,
        "Q0 not executable-qualified",
    )
    require(
        q0.get("conclusion", {}).get("qualified_candidate") is True,
        "Q0 candidate not qualified",
    )
    require(
        q0.get("conclusion", {}).get("runtime_capability_qualified") is False,
        "Q0 gained runtime qualification",
    )
    require(all_false(q0.get("authority")), "Q0 authority must remain false")

    capability_entries = capabilities.get("capabilities")
    require(
        isinstance(capability_entries, list) and capability_entries,
        "capability registry empty",
    )
    for entry in capability_entries:
        validate_capability(entry)
    capability_ids = [entry["capability_id"] for entry in capability_entries]
    require(len(capability_ids) == len(set(capability_ids)), "duplicate capability ID")
    capability_map = {entry["capability_id"]: entry for entry in capability_entries}
    p0 = capability_map.get("P0_GROUNDED_MUTATION_FOUNDATION")
    require(
        isinstance(p0, dict) and p0.get("candidate_qualified") is True,
        "P0 candidate qualification missing",
    )
    require(
        p0.get("wired") is False and p0.get("runtime_qualified") is False,
        "P0 crossed runtime boundary",
    )
    a0 = capability_map.get("A0_CANONICAL_CAPABILITY_AND_EVIDENCE_AUTHORITY")
    require(isinstance(a0, dict) and a0.get("implemented") is True, "A0 source missing")
    require(a0.get("candidate_qualified") is False, "A0 self-qualified in source")

    canonical_stack = pr_stack.get("canonical_stack")
    require(isinstance(canonical_stack, dict), "canonical PR stack missing")
    require(
        canonical_stack.get("candidate_head") == q0_observation.get("head"),
        "PR stack/Q0 head mismatch",
    )
    require(
        canonical_stack.get("candidate_tree") == q0_observation.get("tree"),
        "PR stack/Q0 tree mismatch",
    )
    canonical_entries = canonical_stack.get("entries")
    require(isinstance(canonical_entries, list), "canonical PR entries missing")
    require(
        [entry.get("pr_number") for entry in canonical_entries]
        == [7, 13, 14, 16, 21, 23, 29],
        "canonical PR order mismatch",
    )
    require(
        all(entry.get("canonical_inclusion") is True for entry in canonical_entries),
        "canonical stack exclusion found",
    )
    require(
        all(entry.get("merged") is False for entry in canonical_entries),
        "unapproved merged PR recorded",
    )
    side_stacks = pr_stack.get("side_stacks")
    require(isinstance(side_stacks, list) and side_stacks, "side stacks missing")
    require(
        all(stack.get("canonical") is False for stack in side_stacks),
        "side stack gained canonical status",
    )

    require(
        integration.get("expected_parent") == q0_observation.get("head"),
        "A0 expected parent mismatch",
    )
    require(
        integration.get("branch")
        == "codex/hepta-intelligence-a0-authority-gap-closure-20260829",
        "A0 branch mismatch",
    )
    allowed = integration.get("allowed_changed_paths")
    require(
        isinstance(allowed, list) and allowed == sorted(set(allowed)),
        "A0 allowlist invalid",
    )
    require(
        integration.get("expected_changed_path_count") == len(allowed) == 17,
        "A0 changed-path count mismatch",
    )
    freeze = integration.get("source_freeze", {})
    for key in (
        "rust_runtime_changes_allowed",
        "sql_migrations_allowed",
        "product_callers_allowed",
        "h5_h6_h7_runtime_allowed",
        "model_provider_effects_allowed",
    ):
        require(freeze.get(key) is False, f"source freeze disabled: {key}")
    operational_docs = integration.get("operational_documents")
    require(
        isinstance(operational_docs, list) and len(operational_docs) == 1,
        "integration operational document surface mismatch",
    )
    require(
        operational_docs[0].get("content_sha256") == sha256(PATHS["execution_spec"]),
        "integration execution spec digest mismatch",
    )
    gaps = validate_gap_ledger(integration)

    require(
        evidence.get("not_current_plan_authority") is True,
        "evidence index gained plan authority",
    )
    require(
        evidence.get("q0_current_evidence", {}).get("summary_file_sha256")
        == sha256(PATHS["q0_receipt"]),
        "Q0 evidence file digest mismatch",
    )
    require(all_false(evidence.get("authority")), "evidence authority must be false")
    require(all_false(document.get("authority")), "document authority must be false")
    require(all_false(capabilities.get("authority")), "capability authority must be false")
    require(all_false(pr_stack.get("authority")), "PR stack authority must be false")
    require(all_false(integration.get("authority")), "integration authority must be false")
    require(all_false(current.get("authority")), "current authority must be false")

    open_gaps = [
        {
            "gap_id": entry["gap_id"],
            "classification": entry["classification"],
            "status": entry["status"],
            "next_action": entry["next_action"],
        }
        for entry in gaps
        if entry["classification"] != "CLOSED_SOURCE_CONTROLLED"
    ]

    truth = {
        "schema": "hepta_intelligence_current_truth_v1",
        "repository": repository,
        "source_snapshot": {
            "as_of_utc": current.get("generated_at_utc"),
            "classification": "SOURCE_SNAPSHOT_NOT_LIVE_CI",
            "live_evidence_embedded": False,
        },
        "plan": {
            "id": canonical_plan.get("plan_id"),
            "version": canonical_plan.get("plan_version"),
            "content_sha256": canonical_plan.get("content_sha256"),
            "execution_spec_version": operational.get("execution_spec_version"),
            "execution_spec_sha256": operational.get("execution_spec_sha256"),
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
        "external_unmerged_dependencies": pr_stack.get(
            "external_unmerged_dependencies"
        ),
        "integration_candidate": {
            "candidate_id": integration.get("candidate_id"),
            "branch": integration.get("branch"),
            "expected_parent": integration.get("expected_parent"),
            "allowed_changed_paths": allowed,
            "source_freeze": freeze,
            "merge_policy": integration.get("merge_policy"),
            "side_stack_decisions": integration.get("side_stack_decisions"),
        },
        "claims": current.get("claim_levels"),
        "open_gaps": open_gaps,
        "input_sha256": {key: sha256(path) for key, path in PATHS.items()},
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
        encoded = (
            json.dumps(truth, indent=2, sort_keys=True, ensure_ascii=False) + "\n"
        ).encode()

    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_bytes(encoded)
    else:
        sys.stdout.buffer.write(encoded)
    return 0


if __name__ == "__main__":
    sys.exit(main())
