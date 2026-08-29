#!/usr/bin/env python3
"""Fail-closed static verifier for the Hepta V3 architecture plan package."""

from __future__ import annotations

import json
import pathlib
import re
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
POINTER = "docs/architecture/HEPTA_CURRENT_PLAN.json"
INDEX = "docs/architecture/HEPTA_DOCUMENT_AUTHORITY_INDEX_V2.json"
PLAN = "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V3.md"
LEDGER = "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V3.json"
BOUNDARY = "docs/architecture/HEPTA_PHYSICAL_CAPABILITY_BOUNDARY_MATRIX_V1.md"
FAULTS = "docs/architecture/HEPTA_COMMON_DURABLE_FAULT_MATRIX_V1.md"
BUDGETS = "docs/architecture/HEPTA_RESOURCE_BUDGETS_V1.md"
BOOTSTRAP = "docs/architecture/HEPTA_RUNTIME_GRANT_BOOTSTRAP_V1.md"
README = "README.md"
WORKFLOW = ".github/workflows/hepta-architecture-plan-v3.yml"


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_ARCHITECTURE_PLAN_V3: {message}")


def text(relative: str) -> str:
    path = ROOT / relative
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot read {relative}: {error}")


def obj(relative: str) -> dict[str, Any]:
    def hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        output: dict[str, Any] = {}
        for key, value in pairs:
            if key in output:
                fail(f"duplicate JSON key {key!r} in {relative}")
            output[key] = value
        return output

    try:
        value = json.loads(text(relative), object_pairs_hook=hook)
    except json.JSONDecodeError as error:
        fail(f"invalid JSON in {relative}: {error}")
    if not isinstance(value, dict):
        fail(f"{relative} must contain one JSON object")
    return value


def require_markers(relative: str, markers: tuple[str, ...]) -> str:
    source = text(relative)
    for marker in markers:
        if marker not in source:
            fail(f"{relative} is missing {marker!r}")
    return source


def require_false_authority(value: Any, location: str) -> None:
    if not isinstance(value, dict) or not value:
        fail(f"{location} must be a non-empty authority object")
    enabled = sorted(key for key, state in value.items() if state is not False)
    if enabled:
        fail(f"{location} contains non-false authority fields: {enabled}")


def verify_pointer() -> None:
    pointer = obj(POINTER)
    if pointer.get("schema") != "hepta.current-plan.v1":
        fail("current plan pointer schema drifted")
    if pointer.get("schemaVersion") != 1:
        fail("current plan pointer version drifted")
    expected = {
        "architectureModel": "docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json",
        "currentPlan": PLAN,
        "currentGapLedger": LEDGER,
        "documentAuthorityIndex": INDEX,
        "qualificationStatus": "docs/architecture/HEPTA_QUALIFICATION_STATUS_V2.json",
    }
    for key, path in expected.items():
        if pointer.get(key) != path:
            fail(f"current plan pointer {key} drifted")
        if not (ROOT / path).is_file():
            fail(f"current plan pointer target is missing: {path}")
    claims = pointer.get("claims")
    if not isinstance(claims, dict):
        fail("current plan claims must be an object")
    if claims.get("sourcePlanCurrent") is not True:
        fail("current V3 plan must be selected")
    for key in (
        "allSourceGapsClosed",
        "exactHeadQualified",
        "mergeCandidateQualified",
        "operatorAccepted",
        "promoted",
        "released",
    ):
        if claims.get(key) is not False:
            fail(f"current pointer must keep {key}=false")
    require_false_authority(pointer.get("authority"), "current pointer authority")


def verify_document_index() -> None:
    index = obj(INDEX)
    if index.get("schema") != "hepta.document-authority-index.v2":
        fail("document authority index schema drifted")
    order = index.get("resolutionOrder")
    if not isinstance(order, list) or order[:4] != [
        POINTER,
        "docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json",
        PLAN,
        LEDGER,
    ]:
        fail("document resolution order drifted")
    documents = index.get("documents")
    if not isinstance(documents, list):
        fail("document index entries must be a list")
    by_path: dict[str, dict[str, Any]] = {}
    for entry in documents:
        if not isinstance(entry, dict) or not isinstance(entry.get("path"), str):
            fail("document index contains a malformed entry")
        path = entry["path"]
        if path in by_path:
            fail(f"document index repeats {path}")
        by_path[path] = entry
        if not (ROOT / path).is_file():
            fail(f"indexed document is missing: {path}")
    for required in (POINTER, PLAN, LEDGER, BOOTSTRAP, BOUNDARY, FAULTS, BUDGETS):
        if required not in by_path:
            fail(f"document authority index is missing {required}")
    generated = {
        "ARCHITECTURE.md": "scripts/generate-hepta-architecture-projections.py",
        "docs/architecture/DATA_AUTHORITY_MAP.md": "scripts/generate-hepta-architecture-projections.py",
        "docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json": "scripts/generate-hepta-architecture-projections.py",
    }
    for path, generator in generated.items():
        entry = by_path.get(path)
        if not entry or entry.get("editable") is not False or entry.get("generator") != generator:
            fail(f"generated projection contract drifted for {path}")
    rules = index.get("rules")
    if not isinstance(rules, dict):
        fail("document index rules must be an object")
    for key in (
        "draftPrCanSelectCurrentPlan",
        "receiptCanRewritePlan",
        "sourceCanSelfIssueOperatorAcceptance",
        "sourceCanSelfIssuePromotion",
        "sourceCanSelfIssueRelease",
    ):
        if rules.get(key) is not False:
            fail(f"document authority rule {key} must remain false")
    if rules.get("unknownDocumentClassFailsClosed") is not True:
        fail("unknown document classes must fail closed")


def verify_plan() -> None:
    source = require_markers(
        PLAN,
        (
            "P0.7a — Supervisor-signed runtime-grant bootstrap",
            "P0.7b — all physical boundaries checked per use",
            "P0.7c — physical Memory bounded-context extraction",
            "P0.7d — common durable fault matrix",
            "P0.8a — AST/compiler authority caller ratchet",
            "P0.8b — fleet-queryable runtime-instance projection",
            "P0.8c — runtime resource-budget enforcement",
            "P0.8d — exact real-process vertical slice",
            "P0.9 — repository, operator, promotion and release gates",
            "All source gaps closed",
            "presence never equals executable qualification",
        ),
    )
    headings = [
        source.find("P0.7a —"),
        source.find("P0.7b —"),
        source.find("P0.7c —"),
        source.find("P0.7d —"),
        source.find("P0.8a —"),
        source.find("P0.8b —"),
        source.find("P0.8c —"),
        source.find("P0.8d —"),
        source.find("P0.9 —"),
    ]
    if any(position < 0 for position in headings) or headings != sorted(headings):
        fail("V3 package order drifted")
    forbidden_claims = (
        "all gaps are closed",
        "production authority enabled",
        "operator acceptance issued",
        "promotion issued",
        "release issued",
    )
    lowered = source.lower()
    for phrase in forbidden_claims:
        if phrase in lowered:
            fail(f"plan contains a self-issued claim: {phrase}")


def verify_ledger() -> None:
    ledger = obj(LEDGER)
    if ledger.get("schema") != "hepta.architecture-gap-ledger.v3":
        fail("gap ledger schema drifted")
    if ledger.get("overallState") != "source_execution_in_progress_external_gates_open":
        fail("gap ledger overall state overclaims progress")
    vocabulary = set(ledger.get("stateVocabulary", []))
    required_states = {
        "source_implemented",
        "source_in_progress",
        "open",
        "blocked_external",
        "not_run",
        "qualified_exact",
        "rejected",
    }
    if vocabulary != required_states:
        fail("gap ledger state vocabulary drifted")
    delivery = ledger.get("sourceDeliveryGraph")
    expected_delivery = [
        "P0.7a_supervisor_signed_runtime_grant_bootstrap",
        "P0.7b_all_physical_boundaries_checked_per_use",
        "P0.7c_memory_bounded_context_extraction",
        "P0.7d_common_durable_fault_matrix_execution",
        "P0.8a_ast_authority_caller_ratchet",
        "P0.8b_fleet_runtime_instance_projection",
        "P0.8c_runtime_resource_budget_enforcement",
        "P0.8d_real_process_vertical_slice",
        "P0.9_repository_operator_promotion_release",
    ]
    if delivery != expected_delivery:
        fail("gap ledger delivery graph drifted")
    gaps = ledger.get("openSourceGaps")
    if not isinstance(gaps, dict) or len(gaps) != 8:
        fail("gap ledger must enumerate eight P0.7/P0.8 source gaps")
    if gaps.get("P0.7a_supervisorSignedRuntimeGrantBootstrap", {}).get("state") != "source_in_progress":
        fail("P0.7a must remain source_in_progress until implementation lands")
    for name, gap in gaps.items():
        if not isinstance(gap, dict):
            fail(f"malformed source gap {name}")
        if gap.get("state") not in vocabulary:
            fail(f"unknown state for source gap {name}")
        criteria = gap.get("closureCriteria")
        if not isinstance(criteria, list) or not criteria:
            fail(f"source gap {name} has no closure criteria")
    external = ledger.get("externalGates")
    if not isinstance(external, dict) or len(external) != 7:
        fail("external gates are incomplete")
    for name, gate in external.items():
        if not isinstance(gate, dict) or gate.get("state") not in {
            "blocked_external",
            "not_run",
        }:
            fail(f"external gate {name} overclaims completion")
    require_false_authority(ledger.get("authorityBoundary"), "V3 gap ledger authority")


def verify_bootstrap_contract() -> None:
    source = require_markers(
        BOOTSTRAP,
        (
            "hepta.runtime-authority-bootstrap.v1",
            "Inherited read-only descriptor",
            "Owner-only bootstrap file",
            "durable nonce compare-and-claim",
            "background task may start",
            "P0.7a never self-issues operator acceptance, promotion or release",
        ),
    )
    fields = (
        "subject_agent_id",
        "release_id",
        "source_commit",
        "source_tree",
        "binary_sha256",
        "runtime_profile_sha256",
        "authority_grant_sha256",
        "product_graph_sha256",
        "authority_epoch",
        "owner_epoch",
        "generation",
        "fencing_token_sha256",
        "signer_key_id",
        "signer_epoch",
        "not_before_unix_seconds",
        "expires_at_unix_seconds",
        "nonce_sha256",
    )
    for field in fields:
        if field not in source:
            fail(f"bootstrap contract is missing field {field}")


def verify_boundary_matrix() -> None:
    source = require_markers(
        BOUNDARY,
        (
            "model request submission",
            "provider dispatch",
            "tool process spawn",
            "outbound network connect",
            "filesystem mutation outside Agent root",
            "secret read/refresh/rotate",
            "Matrix send",
            "fleet lifecycle mutation",
            "operator acceptance",
            "release promotion",
            "No adapter may mint the capability it consumes",
        ),
    )
    table_rows = len(re.findall(r"^\| [^|-].*\|$", source, flags=re.MULTILINE))
    if table_rows < 10:
        fail("physical boundary matrix lost required rows")


def verify_fault_matrix() -> None:
    source = text(FAULTS)
    ids = {f"F{value:02d}" for value in range(1, 19)}
    present = set(re.findall(r"\| (F\d{2}) \|", source))
    if present != ids:
        fail(f"durable fault matrix drifted: expected {sorted(ids)}, found {sorted(present)}")
    for store in ("Memory", "Automation", "Matrix", "Evidence", "Fleet/Supervisor", "TaskFlow"):
        if store not in source:
            fail(f"durable fault matrix is missing store {store}")
    if "steps=[]" not in source or "runner ID zero" not in source:
        fail("fault matrix must reject empty/non-running evidence")


def verify_budgets() -> None:
    source = require_markers(
        BUDGETS,
        (
            "concurrent turns per Agent",
            "concurrent tool processes",
            "concurrent model requests",
            "concurrent provider effects",
            "Automation runnable leases",
            "TaskFlow active steps",
            "Matrix in-flight deliveries",
            "Agent cognitive DB",
            "total outbox pending rows",
            "N+1 tests exist for every bound",
        ),
    )
    if source.count("| ") < 15:
        fail("resource budget table is unexpectedly small")


def verify_readme() -> None:
    require_markers(
        README,
        (
            "HEPTA_CURRENT_PLAN.json",
            "HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V3.md",
            "HEPTA_ARCHITECTURE_GAP_LEDGER_V3.json",
            "verify-hepta-architecture-plan-v3.py",
            "neither source presence nor qualification activates authority",
        ),
    )


def verify_workflow() -> None:
    source = text(WORKFLOW)
    required = (
        "permissions:\n  contents: read",
        "pull_request:",
        "workflow_dispatch:",
        "python3 scripts/verify-hepta-architecture-plan-v3.py",
        "python3 scripts/generate-hepta-architecture-projections.py --check",
        "git diff --exit-code",
        "git diff --cached --exit-code",
        "Hepta V3 architecture plan required",
        "if: ${{ always() && !cancelled() }}",
    )
    for marker in required:
        if marker not in source:
            fail(f"V3 workflow is missing {marker!r}")
    forbidden = (
        "git push",
        "git commit",
        "update-ref",
        "contents: write",
        "pull-requests: write",
        "actions/checkout@v",
    )
    lowered = source.lower()
    for marker in forbidden:
        if marker.lower() in lowered:
            fail(f"V3 qualification workflow contains forbidden mutation marker {marker!r}")


def main() -> int:
    verify_pointer()
    verify_document_index()
    verify_plan()
    verify_ledger()
    verify_bootstrap_contract()
    verify_boundary_matrix()
    verify_fault_matrix()
    verify_budgets()
    verify_readme()
    verify_workflow()
    print("PASS_HEPTA_ARCHITECTURE_PLAN_V3_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
