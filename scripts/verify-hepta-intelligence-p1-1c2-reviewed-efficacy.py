#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
P1_CRATE = ROOT / "codex-rs/hepta-memory-p1-1c-qualification"
P1_CARGO = P1_CRATE / "Cargo.toml"
CRATE = ROOT / "codex-rs/hepta-memory-p1-1c2-qualification"
CARGO = CRATE / "Cargo.toml"
LIB = CRATE / "src/lib.rs"
DIGEST = CRATE / "src/digest.rs"
PROJECTION = CRATE / "src/projection.rs"
EVALUATION = CRATE / "src/evaluation.rs"
BINARY = CRATE / "src/bin/p1_1c2_receipt.rs"
TESTS = CRATE / "tests/p1_1c2.rs"
FIXTURE = CRATE / "fixtures/p1_1c2_projection_seed.tsv"
PLAN = ROOT / "plans/hepta-intelligence/P1-1C2_REVIEWED_CORPUS_EFFICACY_PLAN.md"
STATUS = ROOT / "plans/hepta-intelligence/P1-1C2_EXECUTION_STATUS.json"
RECEIPT = ROOT / "plans/hepta-intelligence/P1-1C2_IMPLEMENTATION_RECEIPT.json"
PARENT_PLAN = ROOT / "plans/hepta-intelligence/P1-1C1_REVIEWED_CORPUS_ACCEPTANCE_PLAN.md"
PARENT_STATUS = ROOT / "plans/hepta-intelligence/P1-1C1_EXECUTION_STATUS.json"
WORKFLOW = ROOT / ".github/workflows/hepta-intelligence-p1-1c2-reviewed-efficacy.yml"
PRODUCT_CARGO = ROOT / "codex-rs/Cargo.toml"

AUTHORITY_KEYS = (
    "product_workspace_member",
    "product_module_registered",
    "runtime_wired",
    "default_recall_changed",
    "federation_recall_changed",
    "context_attachment",
    "physical_send",
    "network_access",
    "model_download",
    "external_effects",
    "production_authority",
    "operator_acceptance",
    "promotion",
    "callers_ratchet",
)


def contains_all(text: str, markers: tuple[str, ...]) -> bool:
    return all(marker in text for marker in markers)


def main() -> int:
    files = (
        P1_CARGO,
        CARGO,
        LIB,
        DIGEST,
        PROJECTION,
        EVALUATION,
        BINARY,
        TESTS,
        FIXTURE,
        PLAN,
        STATUS,
        RECEIPT,
        PARENT_PLAN,
        PARENT_STATUS,
        WORKFLOW,
        PRODUCT_CARGO,
    )
    checks: dict[str, bool] = {
        "files.present": all(path.is_file() and path.stat().st_size > 0 for path in files)
    }
    if not checks["files.present"]:
        missing = [str(path.relative_to(ROOT)) for path in files if not path.is_file()]
        print(
            json.dumps(
                {
                    "schema": "hepta.intelligence.p1_1c2.source_gate.v1",
                    "status": "FAIL_P1_1C2_SOURCE",
                    "checks": checks,
                    "missing_files": missing,
                },
                indent=2,
                sort_keys=True,
            )
        )
        return 1

    p1_cargo = P1_CARGO.read_text(encoding="utf-8")
    cargo = CARGO.read_text(encoding="utf-8")
    lib = LIB.read_text(encoding="utf-8")
    digest = DIGEST.read_text(encoding="utf-8")
    projection = PROJECTION.read_text(encoding="utf-8")
    evaluation = EVALUATION.read_text(encoding="utf-8")
    binary = BINARY.read_text(encoding="utf-8")
    tests = TESTS.read_text(encoding="utf-8")
    fixture = FIXTURE.read_text(encoding="utf-8")
    plan = PLAN.read_text(encoding="utf-8")
    parent_plan = PARENT_PLAN.read_text(encoding="utf-8")
    workflow = WORKFLOW.read_text(encoding="utf-8")
    product_cargo = PRODUCT_CARGO.read_text(encoding="utf-8")
    status = json.loads(STATUS.read_text(encoding="utf-8"))
    receipt = json.loads(RECEIPT.read_text(encoding="utf-8"))
    parent_status = json.loads(PARENT_STATUS.read_text(encoding="utf-8"))

    checks["dependency.p1c_isolated_workspace"] = contains_all(
        p1_cargo,
        (
            "[workspace]",
            "publish = false",
            'unsafe_code = "forbid"',
            'all = "deny"',
        ),
    )
    checks["crate.isolated_local_dependencies"] = contains_all(
        cargo,
        (
            "[workspace]",
            "publish = false",
            'path = "../hepta-memory-p1-1c-qualification"',
            'path = "../hepta-memory-p1-1c1-qualification"',
            'unsafe_code = "forbid"',
            'all = "deny"',
        ),
    )
    checks["crate.not_product_workspace_member"] = all(
        name not in product_cargo
        for name in (
            "hepta-memory-p1-1c-qualification",
            "hepta-memory-p1-1c2-qualification",
        )
    )
    checks["digest.real_sha256"] = contains_all(
        digest,
        (
            "ROUND_CONSTANTS: [u32; 64]",
            "Digest32",
            "sha256_known_answer_matches",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
    )
    checks["projection.complete_candidate_binding"] = contains_all(
        projection,
        (
            "review_item_count",
            "evaluation_candidate_count",
            "candidate_coverage_incomplete",
            "review_coverage_incomplete",
            "review_item_set_mismatch",
            "query_digest_mismatch",
            "candidate_digest_mismatch",
            "locale_mismatch",
            "fixture_only",
            "candidate_projection_digest",
        ),
    )
    checks["evaluation.recomputes_acceptance"] = contains_all(
        evaluation,
        (
            "evaluate_review_batch",
            "acceptance_receipt_matches",
            "receipt_recomputation_mismatch",
            "reviewed_corpus_not_accepted",
            "human_review_not_attested",
            "seed_pipeline_not_reproducible",
        ),
    )
    checks["evaluation.seven_lanes_and_baseline_deltas"] = contains_all(
        evaluation,
        (
            "AblationLane::ALL",
            "run_seven_lanes",
            "build_lane_deltas",
            "recall_delta_ppm",
            "ndcg_delta_ppm",
            "citation_delta_ppm",
            "p95_latency_delta_micros",
            "mean_token_cost_delta",
        ),
    )
    checks["evaluation.source_policy"] = contains_all(
        evaluation,
        (
            "minimum_full_recall_at_4_ppm: 750_000",
            "minimum_full_ndcg_at_4_ppm: 700_000",
            "minimum_full_citation_precision_ppm: 400_000",
            "maximum_full_p95_latency_micros: 1_000",
            "maximum_full_mean_token_cost: 512",
            "production_calibrated: false",
        ),
    )
    checks["evaluation.authority_frozen_false"] = contains_all(
        lib,
        (
            "P1_1C2_SOURCE_QUALIFIED: bool = false",
            "P1_1C2_EFFICACY_CLAIM: bool = false",
            "P1_1C2_PRODUCT_WORKSPACE_MEMBER: bool = false",
            "P1_1C2_RUNTIME_WIRED: bool = false",
            "P1_1C2_DEFAULT_RECALL_CHANGED: bool = false",
            "P1_1C2_CONTEXT_ATTACHMENT: bool = false",
            "P1_1C2_PHYSICAL_SEND: bool = false",
            "P1_1C2_PRODUCTION_AUTHORITY: bool = false",
            "P1_1C2_PROMOTION: bool = false",
        ),
    )
    checks["fixture.fail_closed_8_of_48"] = contains_all(
        fixture,
        (
            "fixture_only=true",
            "en-case-001",
            "zh-case-001",
            "pt-case-001",
        ),
    ) and fixture.count("\n") >= 12
    checks["binary.emits_blocked_receipt"] = contains_all(
        binary,
        (
            "DependencyState::blocked_seed",
            "p1_1c2_projection_seed.tsv",
            "evaluate_reviewed_corpus",
            "to_json_pretty",
        ),
    )
    checks["tests.positive_and_negative_paths"] = contains_all(
        tests,
        (
            "reviewed_corpus_with_complete_projection_runs_all_seven_lanes",
            "checked_in_seed_is_blocked_without_lane_evidence",
            "fixture_only_projection_cannot_activate_reviewed_evaluation",
            "incomplete_candidate_coverage_is_blocked",
            "query_digest_drift_is_blocked",
            "candidate_digest_drift_is_blocked",
            "blocked_acceptance_receipt_cannot_be_reused_with_qualified_dependency",
            "reviewed_evaluation_receipt_is_deterministic",
            "machine_receipt_redacts_queries_candidates_and_reviewers",
            "authority_boundary_remains_frozen_false",
        ),
    )
    checks["plan.matches_parent_exit"] = contains_all(
        parent_plan,
        (
            "P1.1c.2",
            "rerun the seven-lane offline evaluation",
            "accepted immutable corpus digest",
        ),
    ) and contains_all(
        plan,
        (
            "P1.1c.2",
            "review item count == evaluation candidate count",
            "projection rows=8",
            "evaluation candidates=48",
            "lanes=[]",
            "source_qualified=false",
            "production_authority=false",
            "nested, publish-disabled workspace",
        ),
    )
    checks["parent.remains_unaccepted"] = (
        parent_status["current_tranche"]["reviewed_corpus_accepted"] is False
        and parent_status["current_tranche"]["corpus_reviewed"] is False
        and parent_status["current_tranche"]["human_review_attested"] is False
    )
    authority = status.get("authority", {})
    checks["status.valid_blocked_boundary"] = (
        status.get("current_tranche", {}).get("implemented") is True
        and status.get("current_tranche", {}).get("source_qualified") is False
        and status.get("current_tranche", {}).get("reviewed_corpus_present") is False
        and status.get("current_tranche", {}).get("reviewed_corpus_evaluated") is False
        and status.get("current_tranche", {}).get("efficacy_validation") is False
        and status.get("implementation", {}).get("transitive_workspace_isolation") is True
        and status.get("checked_in_fixture", {}).get("review_item_count") == 8
        and status.get("checked_in_fixture", {}).get("evaluation_candidate_count") == 48
        and all(authority.get(key) is False for key in AUTHORITY_KEYS)
    )
    receipt_authority = receipt.get("authority", {})
    checks["implementation_receipt.valid_boundary"] = (
        receipt.get("status") == "IMPLEMENTED_PENDING_EXECUTABLE_QUALIFICATION"
        and receipt.get("claims", {}).get("seven_lane_rerun") is True
        and receipt.get("claims", {}).get("transitive_workspace_isolation") is True
        and receipt.get("claims", {}).get("real_human_reviewed_corpus") is False
        and receipt.get("claims", {}).get("real_efficacy_validation") is False
        and receipt.get("fixture", {}).get("expected_to_emit_lanes") is False
        and all(receipt_authority.get(key) is False for key in AUTHORITY_KEYS)
    )
    checks["source.no_network_or_product_wiring"] = not any(
        marker in "\n".join((lib, projection, evaluation, binary, tests))
        for marker in (
            "reqwest::",
            "hyper::",
            "tokio::net",
            "std::net::",
            "codex_hepta_memory::",
            "codex_hepta_agentd::",
            "physical_send(",
            "context_attachment(",
        )
    )
    checks["workflow.exact_matrix"] = contains_all(
        workflow,
        (
            'toolchain: "1.95.0"',
            "Verify local workspace isolation",
            "verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py",
            "cargo fmt --manifest-path",
            "cargo test --manifest-path",
            "cargo check --manifest-path",
            "cargo clippy --manifest-path",
            "--all-targets -- -D warnings",
            "p1_1c2_receipt",
            "cmp",
            "BLOCKED_P1_1C2_REVIEWED_CORPUS_DEPENDENCY",
            "steps.candidate",
        ),
    )

    failures = sorted(name for name, passed in checks.items() if not passed)
    output = {
        "schema": "hepta.intelligence.p1_1c2.source_gate.v1",
        "status": "PASS_P1_1C2_SOURCE_ONLY" if not failures else "FAIL_P1_1C2_SOURCE",
        "implemented": not failures,
        "source_qualified": False,
        "reviewed_corpus_present": False,
        "reviewed_corpus_evaluated": False,
        "efficacy_validation": False,
        "efficacy_claim": False,
        "runtime_wired": False,
        "production_authority": False,
        "checks": checks,
        "failures": failures,
    }
    print(json.dumps(output, indent=2, sort_keys=True))
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())
