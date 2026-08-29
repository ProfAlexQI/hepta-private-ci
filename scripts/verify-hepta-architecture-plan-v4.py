#!/usr/bin/env python3
"""Fail-closed static verifier for the selected Hepta V4 architecture package."""

from __future__ import annotations

import json
import pathlib
import re
import sys
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
POINTER = "docs/architecture/HEPTA_CURRENT_PLAN.json"
MODEL = "docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json"
INDEX = "docs/architecture/HEPTA_DOCUMENT_AUTHORITY_INDEX_V3.json"
PLAN = "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V4.md"
LEDGER = "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V4.json"
STATUS = "docs/architecture/HEPTA_QUALIFICATION_STATUS_V3.json"
BOOTSTRAP = "docs/architecture/HEPTA_RUNTIME_GRANT_BOOTSTRAP_V1.md"
BOUNDARY = "docs/architecture/HEPTA_PHYSICAL_CAPABILITY_BOUNDARY_MATRIX_V1.md"
FAULTS = "docs/architecture/HEPTA_COMMON_DURABLE_FAULT_MATRIX_V1.md"
BUDGETS = "docs/architecture/HEPTA_RESOURCE_BUDGETS_V1.md"
README = "README.md"
V4_WORKFLOW = ".github/workflows/hepta-architecture-plan-v4.yml"
P07A_WORKFLOW = ".github/workflows/hepta-runtime-bootstrap-p0-7a.yml"
CARGO_LOCK = "codex-rs/Cargo.lock"

ADVANCED_SOURCE_STATES = {
    "source_implemented",
    "source_verified",
    "qualified_exact",
    "merge_candidate_qualified",
}


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_ARCHITECTURE_PLAN_V4: {message}")


def text(relative: str) -> str:
    path = ROOT / relative
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(f"cannot read {relative}: {error}")


def obj(relative: str) -> dict[str, Any]:
    def hook(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                fail(f"duplicate JSON key {key!r} in {relative}")
            result[key] = value
        return result

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
        fail(f"{location} must be a non-empty object")
    enabled = sorted(key for key, state in value.items() if state is not False)
    if enabled:
        fail(f"{location} contains non-false authority fields: {enabled}")


def verify_pointer() -> None:
    pointer = obj(POINTER)
    if pointer.get("schema") != "hepta.current-plan.v1" or pointer.get("schemaVersion") != 1:
        fail("current plan pointer schema drifted")
    expected = {
        "architectureModel": MODEL,
        "currentPlan": PLAN,
        "currentGapLedger": LEDGER,
        "documentAuthorityIndex": INDEX,
        "qualificationStatus": STATUS,
    }
    for key, path in expected.items():
        if pointer.get(key) != path:
            fail(f"current plan pointer {key} drifted")
        if not (ROOT / path).is_file():
            fail(f"current plan pointer target is missing: {path}")
    if pointer.get("supersedesPlan") != "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V3.md":
        fail("V4 pointer must supersede V3")
    claims = pointer.get("claims")
    if not isinstance(claims, dict) or claims.get("sourcePlanCurrent") is not True:
        fail("V4 must be the selected source plan")
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
    if index.get("schema") != "hepta.document-authority-index.v3":
        fail("document authority index schema drifted")
    if index.get("schemaVersion") != 3 or index.get("status") != "normative_document_authority":
        fail("V3 document index is not selected as normative")
    expected_order = [POINTER, MODEL, PLAN, LEDGER, STATUS]
    if index.get("resolutionOrder") != expected_order:
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
    for required in (
        POINTER,
        MODEL,
        PLAN,
        LEDGER,
        STATUS,
        BOOTSTRAP,
        BOUNDARY,
        FAULTS,
        BUDGETS,
        "scripts/verify-hepta-architecture-plan-v4.py",
        V4_WORKFLOW,
        P07A_WORKFLOW,
    ):
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
    historical = {
        "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V3.md": PLAN,
        "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V3.json": LEDGER,
    }
    for path, replacement in historical.items():
        entry = by_path.get(path)
        if (
            not entry
            or entry.get("class") not in {"historical_plan", "historical_gap_ledger"}
            or entry.get("editable") is not False
            or entry.get("supersededBy") != replacement
        ):
            fail(f"historical authority contract drifted for {path}")
    rules = index.get("rules")
    if not isinstance(rules, dict):
        fail("document index rules must be an object")
    for key in (
        "draftPrCanSelectCurrentPlan",
        "receiptCanRewritePlan",
        "sourceCanSelfIssueOperatorAcceptance",
        "sourceCanSelfIssuePromotion",
        "sourceCanSelfIssueRelease",
        "buildLockDriftCanPass",
    ):
        if rules.get(key) is not False:
            fail(f"document authority rule {key} must remain false")
    if rules.get("unknownDocumentClassFailsClosed") is not True:
        fail("unknown document classes must fail closed")


def verify_plan() -> None:
    source = require_markers(
        PLAN,
        (
            "A-LOCK-01",
            "A-TRANSPORT-01",
            "A-RECOVERY-01",
            "P0.7a — signed runtime bootstrap closure",
            "P0.7b — per-use physical capability boundaries",
            "P0.7c — Memory bounded-context extraction",
            "P0.7d — common durable fault matrix execution",
            "P0.7e — dependency inversion and wire isolation",
            "P0.8a — AST/compiler authority caller ratchet",
            "P0.8b — fleet runtime-instance projection",
            "P0.8c — resource-budget enforcement",
            "P0.8d — exact real-process vertical slice",
            "P0.9 — independently issued external gates",
            "Source presence is not executable qualification",
            "No component may mint the capability it consumes",
        ),
    )
    ordered = (
        "P0.7a —",
        "P0.7b —",
        "P0.7c —",
        "P0.7d —",
        "P0.7e —",
        "P0.8a —",
        "P0.8b —",
        "P0.8c —",
        "P0.8d —",
        "P0.9 —",
    )
    positions = [source.find(marker) for marker in ordered]
    if any(position < 0 for position in positions) or positions != sorted(positions):
        fail("V4 package order drifted")
    if "production authority enabled" in source.lower():
        fail("plan contains a self-issued production claim")


def verify_ledger() -> dict[str, Any]:
    ledger = obj(LEDGER)
    if ledger.get("schema") != "hepta.architecture-gap-ledger.v4":
        fail("gap ledger schema drifted")
    if ledger.get("overallState") != "source_execution_in_progress_external_gates_open":
        fail("gap ledger overall state overclaims progress")
    expected_vocabulary = {
        "open",
        "source_in_progress",
        "source_implemented",
        "source_verified",
        "qualified_exact",
        "merge_candidate_qualified",
        "blocked_external",
        "not_run",
        "base_drift",
        "rejected",
        "superseded",
    }
    if set(ledger.get("stateVocabulary", [])) != expected_vocabulary:
        fail("gap ledger state vocabulary drifted")
    expected_delivery = [
        "P0.7a_signed_runtime_bootstrap_closure",
        "P0.7b_all_physical_boundaries_checked_per_use",
        "P0.7c_memory_bounded_context_extraction",
        "P0.7d_common_durable_fault_matrix_execution",
        "P0.7e_dependency_inversion_and_wire_isolation",
        "P0.8a_ast_authority_caller_ratchet",
        "P0.8b_fleet_runtime_instance_projection",
        "P0.8c_runtime_resource_budget_enforcement",
        "P0.8d_real_process_vertical_slice",
        "P0.9_repository_operator_promotion_release",
    ]
    if ledger.get("sourceDeliveryGraph") != expected_delivery:
        fail("V4 delivery graph drifted")
    blockers = ledger.get("sourceControlledBlockers")
    if not isinstance(blockers, dict) or set(blockers) != {
        "A-LOCK-01",
        "A-TRANSPORT-01",
        "A-RECOVERY-01",
        "A-CI-01",
    }:
        fail("P0.7a blocker inventory drifted")
    packages = ledger.get("sourcePackages")
    if not isinstance(packages, dict) or len(packages) != 9:
        fail("source package inventory must contain P0.7a-P0.8d")
    for name, package in packages.items():
        if not isinstance(package, dict) or package.get("state") not in expected_vocabulary:
            fail(f"malformed package state for {name}")
        if package.get("authorityDelta") != "none":
            fail(f"source package {name} attempts an authority delta")
    external = ledger.get("externalGates")
    if not isinstance(external, dict) or len(external) != 7:
        fail("external gate inventory is incomplete")
    for name, gate in external.items():
        if not isinstance(gate, dict) or gate.get("state") not in {"blocked_external", "not_run"}:
            fail(f"external gate {name} overclaims completion")
    completion = ledger.get("completion")
    if not isinstance(completion, dict) or any(value is not False for value in completion.values()):
        fail("V4 completion flags must remain false")
    require_false_authority(ledger.get("authorityBoundary"), "V4 gap ledger authority")
    return ledger


def verify_status() -> None:
    status = obj(STATUS)
    if status.get("schema") != "hepta.qualification-status.v3":
        fail("qualification status schema drifted")
    if status.get("claimLevel") != "source_present_unqualified":
        fail("qualification status overclaims executable evidence")
    observed = status.get("observedExternalExecution")
    if not isinstance(observed, dict):
        fail("observed external execution is missing")
    if observed.get("jobCount") == 0 and observed.get("qualificationClaim") != "not_run":
        fail("zero-job Actions evidence must remain not_run")
    qualification = status.get("qualification")
    if not isinstance(qualification, dict) or not qualification:
        fail("qualification matrix is missing")
    for name, result in qualification.items():
        if not isinstance(result, dict) or result.get("state") not in {
            "not_run",
            "queued",
            "running",
            "passed",
            "failed",
            "blocked",
            "superseded",
        }:
            fail(f"invalid qualification state for {name}")
        if result.get("state") == "passed":
            if not result.get("runnerId") or not result.get("stepCount"):
                fail(f"passed qualification {name} lacks executable runner evidence")
    external = status.get("externalDecisions")
    if not isinstance(external, dict) or any(
        not isinstance(value, dict) or value.get("state") != "not_issued"
        for value in external.values()
    ):
        fail("qualification status self-issued an external decision")
    require_false_authority(status.get("authorityBoundary"), "qualification authority")


def package_dependencies(lock_source: str, package: str) -> set[str]:
    match = re.search(
        rf'(?ms)^\[\[package\]\]\nname = "{re.escape(package)}"\n.*?(?=^\[\[package\]\]|\Z)',
        lock_source,
    )
    if not match:
        fail(f"Cargo.lock is missing package {package}")
    block = match.group(0)
    dependency_match = re.search(r"(?ms)^dependencies = \[\n(.*?)^\]\n", block)
    if not dependency_match:
        return set()
    return set(re.findall(r'^ "([^"]+)",$', dependency_match.group(1), flags=re.MULTILINE))


def dependency_present(dependencies: set[str], expected: str) -> bool:
    return any(value == expected or value.startswith(f"{expected} ") for value in dependencies)


def verify_lock_alignment(ledger: dict[str, Any]) -> None:
    lock_source = text(CARGO_LOCK)
    expected = {
        "codex-hepta-contracts": {"base64", "ed25519-dalek"},
        "codex-hepta-fleet": {"libc"},
        "codex-hepta-agentd": {"ed25519-dalek"},
    }
    missing: list[str] = []
    for package, dependencies in expected.items():
        actual = package_dependencies(lock_source, package)
        for dependency in dependencies:
            if not dependency_present(actual, dependency):
                missing.append(f"{package}->{dependency}")
    state = ledger["sourceControlledBlockers"]["A-LOCK-01"]["state"]
    if missing and state in ADVANCED_SOURCE_STATES:
        fail(f"A-LOCK-01 advanced while Cargo.lock is stale: {missing}")
    if not missing and state in {"open", "source_in_progress"}:
        fail("A-LOCK-01 remains open after Cargo.lock became coherent")


def verify_conditional_source(ledger: dict[str, Any]) -> None:
    blockers = ledger["sourceControlledBlockers"]
    transport_state = blockers["A-TRANSPORT-01"]["state"]
    recovery_state = blockers["A-RECOVERY-01"]["state"]
    agentd_source = text("codex-rs/hepta-agentd/src/runtime_bootstrap.rs")
    fleet_source = text("codex-rs/hepta-fleet/src/runtime_bootstrap_registry.rs")
    agentd_tests = text("codex-rs/hepta-agentd/src/runtime_bootstrap_tests.rs")
    supervisor_tests = text("codex-rs/hepta-supervisor/src/runtime_bootstrap_tests.rs")
    if transport_state in ADVANCED_SOURCE_STATES:
        for marker in ("O_NOFOLLOW", ".uid()", ".nlink()", "symlink", "hardlink", "wrong mode"):
            haystack = "\n".join((agentd_source, fleet_source, agentd_tests, supervisor_tests))
            if marker not in haystack:
                fail(f"A-TRANSPORT-01 advanced without source marker {marker!r}")
    if recovery_state in ADVANCED_SOURCE_STATES:
        for marker in (
            "claim",
            "fresh generation",
            "partial reservation",
            "published handoff",
            "replay",
        ):
            haystack = "\n".join((agentd_tests, supervisor_tests))
            if marker not in haystack:
                fail(f"A-RECOVERY-01 advanced without test marker {marker!r}")


def verify_component_contracts() -> None:
    require_markers(
        BOOTSTRAP,
        (
            "hepta.runtime-authority-bootstrap.v1",
            "Owner-only bootstrap file",
            "durable nonce compare-and-claim",
            "P0.7a never self-issues operator acceptance, promotion or release",
        ),
    )
    require_markers(
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
            "No adapter may mint the capability it consumes",
        ),
    )
    faults = text(FAULTS)
    expected_faults = {f"F{number:02d}" for number in range(1, 19)}
    if set(re.findall(r"\| (F\d{2}) \|", faults)) != expected_faults:
        fail("durable fault matrix lost one or more F01-F18 rows")
    require_markers(
        BUDGETS,
        (
            "concurrent turns per Agent",
            "concurrent tool processes",
            "concurrent model requests",
            "concurrent provider effects",
            "N+1 tests exist for every bound",
        ),
    )


def verify_readme() -> None:
    require_markers(
        README,
        (
            "HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V4.md",
            "HEPTA_ARCHITECTURE_GAP_LEDGER_V4.json",
            "HEPTA_DOCUMENT_AUTHORITY_INDEX_V3.json",
            "HEPTA_QUALIFICATION_STATUS_V3.json",
            "verify-hepta-architecture-plan-v4.py",
            "neither source presence nor qualification activates authority",
        ),
    )


def verify_workflow(relative: str, required: tuple[str, ...]) -> str:
    source = text(relative)
    for marker in required:
        if marker not in source:
            fail(f"{relative} is missing {marker!r}")
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
            fail(f"{relative} contains forbidden mutation marker {marker!r}")
    return source


def verify_workflows() -> None:
    verify_workflow(
        V4_WORKFLOW,
        (
            "permissions:\n  contents: read",
            "pull_request:",
            "workflow_dispatch:",
            "push:",
            "python3 scripts/verify-hepta-architecture-plan-v4.py",
            "python3 scripts/generate-hepta-architecture-projections.py --check",
            "Hepta V4 architecture plan required",
            "if: ${{ always() && !cancelled() }}",
        ),
    )
    p07a = verify_workflow(
        P07A_WORKFLOW,
        (
            "cargo metadata --locked",
            "just bazel-lock-check",
            "just bazel-lock-update",
            "git diff --exit-code -- codex-rs/Cargo.lock MODULE.bazel.lock",
            "python3 scripts/verify-hepta-architecture-plan-v4.py",
            "Hepta P0.7a required",
        ),
    )
    if "V3 architecture source verifier" in p07a:
        fail("P0.7a workflow still selects the superseded V3 verifier")


def main() -> int:
    verify_pointer()
    verify_document_index()
    verify_plan()
    ledger = verify_ledger()
    verify_status()
    verify_lock_alignment(ledger)
    verify_conditional_source(ledger)
    verify_component_contracts()
    verify_readme()
    verify_workflows()
    print("PASS_HEPTA_ARCHITECTURE_PLAN_V4_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    sys.exit(main())
