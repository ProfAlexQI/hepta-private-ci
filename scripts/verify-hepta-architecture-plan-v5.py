#!/usr/bin/env python3
"""Fail-closed source verifier for Hepta Architecture Convergence Plan V5/B0."""

from __future__ import annotations

import json
import pathlib
import re
from typing import Any, NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
POINTER = "docs/architecture/HEPTA_CURRENT_PLAN.json"
MODEL = "docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json"
PLAN = "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V5.md"
LEDGER = "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V5.json"
INDEX = "docs/architecture/HEPTA_DOCUMENT_AUTHORITY_INDEX_V4.json"
STATUS = "docs/architecture/HEPTA_QUALIFICATION_STATUS_V4.json"
DELIVERY = "docs/architecture/HEPTA_P0_7B_VERIFIED_USE_DELIVERY_CONTRACT_V1.md"
BOUNDARY = "docs/architecture/HEPTA_PHYSICAL_CAPABILITY_BOUNDARY_MATRIX_V1.md"
FAULTS = "docs/architecture/HEPTA_COMMON_DURABLE_FAULT_MATRIX_V1.md"
BUDGETS = "docs/architecture/HEPTA_RESOURCE_BUDGETS_V1.md"
SOURCE = "codex-rs/hepta-contracts/src/verified_use.rs"
TESTS = "codex-rs/hepta-contracts/src/verified_use_tests.rs"
LIB = "codex-rs/hepta-contracts/src/lib.rs"
ROUTER = "scripts/verify-hepta-selected-architecture-plan.py"
V4_VERIFIER = "scripts/verify-hepta-architecture-plan-v4.py"
V5_VERIFIER = "scripts/verify-hepta-architecture-plan-v5.py"
FORMAT = "scripts/verify-hepta-v5-b0-format.py"
V4_WORKFLOW = ".github/workflows/hepta-architecture-plan-v4.yml"
P07A_WORKFLOW = ".github/workflows/hepta-runtime-bootstrap-p0-7a.yml"
P07A_ARM_WORKFLOW = ".github/workflows/hepta-p0-7a-direct-arm.yml"
WORKFLOW = ".github/workflows/hepta-architecture-v5-b0-verified-use.yml"

PACKAGE_IDS = [
    "P0.7a_signed_runtime_bootstrap_closure",
    "P0.7b/B0_verified_use_kernel",
    "P0.7b/B1_model_provider_boundaries",
    "P0.7b/B2_tool_network_filesystem_boundaries",
    "P0.7b/B3_secret_matrix_fleet_operator_release_boundaries",
    "P0.7b/B4_negative_callsite_proof",
    "P0.7c_memory_bounded_context_extraction",
    "P0.7d_common_durable_fault_matrix",
    "P0.7e_dependency_inversion_and_wire_isolation",
    "P0.8a_ast_authority_caller_ratchet",
    "P0.8b_runtime_instance_readiness_projection",
    "P0.8c_resource_budget_enforcement",
    "P0.8d_real_process_vertical_slice",
    "P0.9_external_and_administrative_gates",
]

PHYSICAL_KINDS = [
    "CognitiveStateWrite",
    "ModelInvocation",
    "ProviderDispatch",
    "ExternalEffect",
    "ToolProcessSpawn",
    "OutboundNetworkConnect",
    "ExternalFilesystemMutation",
    "SecretOperation",
    "MatrixSend",
    "FleetMutation",
    "OperatorAcceptance",
    "ReleasePromotion",
]

B0_OWNED_PATHS = {
    SOURCE,
    TESTS,
    LIB,
    DELIVERY,
    PLAN,
    LEDGER,
    POINTER,
    INDEX,
    STATUS,
    ROUTER,
    V5_VERIFIER,
    FORMAT,
    V4_WORKFLOW,
    P07A_WORKFLOW,
    P07A_ARM_WORKFLOW,
    WORKFLOW,
}

INDEXED_V5_PATHS = {
    POINTER,
    MODEL,
    PLAN,
    LEDGER,
    STATUS,
    DELIVERY,
    BOUNDARY,
    FAULTS,
    BUDGETS,
    ROUTER,
    V4_VERIFIER,
    V5_VERIFIER,
    FORMAT,
    V4_WORKFLOW,
    P07A_WORKFLOW,
    P07A_ARM_WORKFLOW,
    WORKFLOW,
}


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_ARCHITECTURE_PLAN_V5: {message}")


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


def verify_required_files() -> None:
    for relative in sorted(
        INDEXED_V5_PATHS
        | {
            SOURCE,
            TESTS,
            LIB,
        }
    ):
        if not (ROOT / relative).is_file():
            fail(f"required V5/B0 file is missing: {relative}")


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
        "currentPackage": "P0.7b/B0_verified_use_kernel",
        "supersedesPlan": "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V4.md",
    }
    for key, expected_value in expected.items():
        if pointer.get(key) != expected_value:
            fail(f"current plan pointer {key} drifted")
    claims = pointer.get("claims")
    if not isinstance(claims, dict):
        fail("current plan claims must be an object")
    if claims.get("sourcePlanCurrentOnCandidateBranch") is not True:
        fail("V5 must be selected on its candidate branch")
    if claims.get("predecessorExecutableQualificationPending") is not True:
        fail("P0.7a executable qualification must remain pending in this source snapshot")
    for key in (
        "allSourceGapsClosed",
        "exactHeadQualified",
        "mergeCandidateQualified",
        "independentReviewIssued",
        "operatorAccepted",
        "promoted",
        "released",
        "allGapsClosed",
    ):
        if claims.get(key) is not False:
            fail(f"current plan claim {key} must remain false")
    binding = pointer.get("candidateBinding")
    if not isinstance(binding, dict):
        fail("current plan candidate binding must be an object")
    if binding.get("stackParentCommit") != "f69e5a4a5068a2657f1470da43c26b1410d53c6f":
        fail("current plan stack-parent commit drifted")
    if binding.get("stackParentTree") != "532307507d2b02a479d3c76042d42cc948b499df":
        fail("current plan stack-parent tree drifted")
    require_false_authority(pointer.get("authority"), "current plan authority")


def verify_document_index() -> None:
    index = obj(INDEX)
    if index.get("schema") != "hepta.document-authority-index.v4":
        fail("document authority index schema drifted")
    if index.get("schemaVersion") != 4 or index.get("status") != "normative_document_authority":
        fail("V4 document authority index is not normative")
    expected_order = [POINTER, MODEL, PLAN, LEDGER, STATUS, DELIVERY, BOUNDARY]
    if index.get("resolutionOrder") != expected_order:
        fail("document resolution order drifted")
    documents = index.get("documents")
    if not isinstance(documents, list) or not documents:
        fail("document authority entries must be a non-empty list")
    by_path: dict[str, dict[str, Any]] = {}
    for entry in documents:
        if not isinstance(entry, dict) or not isinstance(entry.get("path"), str):
            fail("document authority index contains a malformed entry")
        path = entry["path"]
        if path in by_path:
            fail(f"document authority index repeats {path}")
        by_path[path] = entry
        if not (ROOT / path).exists():
            fail(f"indexed path is missing: {path}")
    missing = sorted(INDEXED_V5_PATHS - set(by_path))
    if missing:
        fail(f"document authority index is missing V5 paths: {missing}")
    rules = index.get("rules")
    if not isinstance(rules, dict):
        fail("document authority rules must be an object")
    if rules.get("draftPrCanSelectCurrentPlanOnItsOwnBranch") is not True:
        fail("candidate branch plan-selection rule drifted")
    for key in (
        "draftPrCanSelectCurrentPlanOnDefaultBranch",
        "receiptCanRewritePlan",
        "sourceCanSelfIssueIndependentReview",
        "sourceCanSelfIssueOperatorAcceptance",
        "sourceCanSelfIssuePromotion",
        "sourceCanSelfIssueRelease",
        "buildLockDriftCanPass",
        "queuedOrEmptyExecutionCanPass",
        "selfReferentialDocumentDigestAllowed",
    ):
        if rules.get(key) is not False:
            fail(f"document authority rule {key} must remain false")
    if rules.get("unknownDocumentClassFailsClosed") is not True:
        fail("unknown document classes must fail closed")
    require_false_authority(index.get("authority"), "document authority index")


def verify_plan() -> None:
    source = require_markers(
        PLAN,
        (
            "HEPTA-ARCHITECTURE-CONVERGENCE-V5",
            "P0.7a — signed runtime bootstrap closure",
            "P0.7b — verified physical capability closure",
            "B0 common verified-use kernel",
            "B1 model/provider boundary",
            "B2 tool/network/filesystem boundary",
            "B3 secret, Matrix, fleet, operator and release boundaries",
            "B4 negative call-site proof",
            "P0.7c — Memory bounded-context extraction",
            "P0.7d — common durable fault matrix",
            "P0.7e — dependency inversion and wire isolation",
            "P0.8a — compiler/AST authority ratchet",
            "P0.8b — runtime-instance and readiness graph",
            "P0.8c — executable resource budgets",
            "P0.8d — real-process vertical slice",
            "P0.9 — external and administrative gates",
            "F01",
            "F18",
            "all_gaps_closed=false",
            "non-serializable verified-use token",
            "No adapter may mint the capability or verified-use token that it consumes",
        ),
    )
    ordered = (
        "## 6. P0.7a",
        "## 7. P0.7b",
        "## 8. P0.7c",
        "## 9. P0.7d",
        "## 10. P0.7e",
        "## 11. P0.8a",
        "## 12. P0.8b",
        "## 13. P0.8c",
        "## 14. P0.8d",
        "## 15. P0.9",
    )
    positions = [source.find(marker) for marker in ordered]
    if any(position < 0 for position in positions) or positions != sorted(positions):
        fail("V5 package order drifted")
    if "production authority enabled" in source.lower():
        fail("V5 contains a self-issued production claim")


def verify_ledger() -> None:
    ledger = obj(LEDGER)
    if ledger.get("schema") != "hepta.architecture-gap-ledger.v5":
        fail("V5 gap ledger schema drifted")
    if ledger.get("planId") != "HEPTA-ARCHITECTURE-CONVERGENCE-V5":
        fail("V5 gap ledger plan binding drifted")
    if ledger.get("overallState") != "source_execution_in_progress_external_gates_open":
        fail("V5 gap ledger overall state overclaims progress")
    if ledger.get("currentPackage") != "P0.7b/B0_verified_use_kernel":
        fail("V5 current package drifted")
    if ledger.get("allGapsClosed") is not False:
        fail("V5 cannot claim all gaps closed")
    stack_parent = ledger.get("stackParent")
    if not isinstance(stack_parent, dict):
        fail("V5 gap ledger stack parent must be an object")
    if stack_parent.get("commit") != "f69e5a4a5068a2657f1470da43c26b1410d53c6f":
        fail("V5 gap ledger stack-parent commit drifted")
    if stack_parent.get("tree") != "532307507d2b02a479d3c76042d42cc948b499df":
        fail("V5 gap ledger stack-parent tree drifted")
    identity_policy = ledger.get("candidateIdentityPolicy")
    if not isinstance(identity_policy, dict):
        fail("candidate identity policy must be an object")
    for key in ("queuedIsEvidence", "emptyStepsAreEvidence", "sourceVerifierIsExecutableEvidence"):
        if identity_policy.get(key) is not False:
            fail(f"candidate identity rule {key} must remain false")

    packages = ledger.get("packages")
    if not isinstance(packages, list) or len(packages) != len(PACKAGE_IDS):
        fail("V5 package inventory is incomplete")
    observed_ids: list[str] = []
    for expected_order, package in enumerate(packages, start=1):
        if not isinstance(package, dict):
            fail("V5 package entry must be an object")
        observed_ids.append(str(package.get("id")))
        if package.get("order") != expected_order:
            fail(f"V5 package order drifted at {package.get('id')}")
    if observed_ids != PACKAGE_IDS:
        fail("V5 package IDs or order drifted")
    by_id = {package["id"]: package for package in packages}
    if by_id[PACKAGE_IDS[0]].get("state") != "source_implemented_executable_qualification_pending":
        fail("P0.7a state must remain executable-qualification pending")
    b0 = by_id[PACKAGE_IDS[1]]
    if b0.get("state") not in {"source_in_progress", "source_implemented", "source_verified"}:
        fail("B0 source state is invalid for this candidate")
    owned_paths = b0.get("ownedPaths")
    if not isinstance(owned_paths, list) or set(owned_paths) != B0_OWNED_PATHS:
        fail("B0 owned-path allowlist drifted")
    required_facts = b0.get("requiredFacts")
    if not isinstance(required_facts, list):
        fail("B0 required facts must be a list")
    for fact in (
        "closed_physical_capability_kind",
        "final_operation_and_payload_binding",
        "current_revision_verifier",
        "nonclone_nonserializable_private_token",
        "consume_by_value",
        "selected_plan_verifier_routing",
        "no_product_caller_change",
    ):
        if fact not in required_facts:
            fail(f"B0 required fact is missing: {fact}")
    for package_id in PACKAGE_IDS[2:]:
        if by_id[package_id].get("state") != "open":
            fail(f"future package {package_id} overclaims progress")
    fault_rows = by_id["P0.7d_common_durable_fault_matrix"].get("requiredRows")
    if not isinstance(fault_rows, list) or len(fault_rows) != 18:
        fail("V5 durable fault matrix must retain all 18 rows")
    if not fault_rows[0].startswith("F01_") or not fault_rows[-1].startswith("F18_"):
        fail("V5 durable fault matrix boundary rows drifted")

    external = ledger.get("externalGates")
    if not isinstance(external, list) or len(external) != 8:
        fail("external gate inventory must contain eight issuer-bound rows")
    for gate in external:
        if not isinstance(gate, dict) or gate.get("state") != "open" or not gate.get("issuer"):
            fail("external gate overclaims issuance or lacks an issuer")
    require_false_authority(ledger.get("authorityFlags"), "V5 gap ledger authority")
    expected_stops = {
        "PACKAGE_CLOSED_CANDIDATE",
        "BASE_DRIFT",
        "BLOCKED_UPSTREAM",
        "BLOCKED_EXTERNAL",
        "STOP_CONDITION",
        "RESUME_REQUIRED",
    }
    if set(ledger.get("validStopOutcomes", [])) != expected_stops:
        fail("V5 stop-outcome vocabulary drifted")


def verify_status() -> None:
    status = obj(STATUS)
    if status.get("schema") != "hepta.qualification-status.v4":
        fail("V4 qualification status schema drifted")
    if status.get("planId") != "HEPTA-ARCHITECTURE-CONVERGENCE-V5":
        fail("qualification status plan binding drifted")
    if status.get("claimLevel") != "source_execution_in_progress_unqualified":
        fail("qualification status overclaims executable evidence")
    execution_states = set(status.get("executionStateVocabulary", []))
    expected_execution_states = {
        "not_run",
        "queued",
        "running",
        "passed",
        "failed",
        "blocked",
        "superseded",
    }
    if execution_states != expected_execution_states:
        fail("execution-state vocabulary drifted")
    package_states = set(status.get("packageStateVocabulary", []))
    expected_package_states = {
        "open",
        "source_in_progress",
        "source_implemented",
        "source_verified",
        "qualified_exact",
        "merge_candidate_qualified",
        "blocked_external",
        "closed",
    }
    if package_states != expected_package_states:
        fail("package-state vocabulary drifted")
    observed = status.get("observedExecution")
    if not isinstance(observed, dict):
        fail("qualification status lacks observed execution")
    for name in ("b0ExactHead", "b0MergeCandidate"):
        row = observed.get(name)
        if not isinstance(row, dict) or row.get("state") not in execution_states:
            fail(f"{name} contains an invalid execution state")
        if row.get("state") == "passed":
            for id_key in ("runId", "jobId", "runnerId", "stepCount"):
                value = row.get(id_key)
                if not isinstance(value, int) or value <= 0:
                    fail(f"{name} passed without positive {id_key}")
        elif row.get("qualificationClaim") not in {"not_run", "failed", "blocked"}:
            fail(f"{name} overclaims qualification before a passed run")
    decisions = status.get("externalDecisions")
    if not isinstance(decisions, dict) or not decisions:
        fail("external decision status is missing")
    for name, decision in decisions.items():
        if not isinstance(decision, dict) or decision.get("state") != "not_issued":
            fail(f"external decision {name} was self-issued")
    rules = status.get("rules")
    if not isinstance(rules, dict) or any(value is not False for value in rules.values()):
        fail("qualification source rules must all deny self-promotion")
    require_false_authority(status.get("authorityBoundary"), "qualification authority")


def verify_delivery_contract() -> None:
    require_markers(
        DELIVERY,
        (
            "B0_verified_use_kernel",
            "CognitiveStateWrite",
            "ModelInvocation",
            "ProviderDispatch",
            "ToolProcessSpawn",
            "OutboundNetworkConnect",
            "ExternalFilesystemMutation",
            "SecretOperation",
            "MatrixSend",
            "FleetMutation",
            "OperatorAcceptance",
            "ReleasePromotion",
            "non-cloneable",
            "non-serializable",
            "consumed by value",
            "operation ID drift",
            "final payload drift",
            "revocation revision drift",
            "exact-head and merge-candidate",
            "runtime, effect, production, operator, promotion and release authority remain false",
        ),
    )
    require_markers(
        BOUNDARY,
        (
            "short-lived",
            "operation-bound verified-use token",
            "final operation payload",
            "cannot be reused",
            "No adapter may mint the capability it consumes",
        ),
    )


def verify_verified_use_source() -> None:
    source = require_markers(
        SOURCE,
        tuple(
            [
                "pub const VERIFIED_USE_SCHEMA_VERSION: u32 = 1",
                "pub enum PhysicalCapabilityKind",
                "pub struct RevocationRevision",
                "pub struct PhysicalUseWindow",
                "pub struct PhysicalUseVerificationRequest",
                "pub struct PhysicalUseVerification",
                "pub trait PhysicalUseVerifier: CapabilityUseVerifier",
                "pub fn verify_physical_capability_use",
                "pub struct PhysicalUseFinalCheck",
                "pub struct VerifiedUseToken<C>",
                "pub struct VerifiedUseWitness",
                "external_lease_binding()",
                "verify_capability_use(",
                "expected_revocation_revision",
                "final_payload_sha256",
                "runtime_authority_context_sha256",
                "verifier_receipt_sha256",
                "token_sha256",
                "witness_sha256",
            ]
            + PHYSICAL_KINDS
        ),
    )
    if not re.search(r"pub fn consume\(\s*self,", source):
        fail("VerifiedUseToken consumption must take self by value")
    token_declaration = source.find("pub struct VerifiedUseToken<C>")
    if token_declaration < 0:
        fail("VerifiedUseToken declaration is missing")
    preceding = source[max(0, token_declaration - 120) : token_declaration]
    if "#[derive" in preceding:
        fail("VerifiedUseToken must not derive Clone, Copy or Serde traits")
    for marker in (
        "impl<C> Clone for VerifiedUseToken",
        "impl<C> Copy for VerifiedUseToken",
        "impl<C> Serialize for VerifiedUseToken",
        "impl<'de, C> Deserialize<'de> for VerifiedUseToken",
    ):
        if marker in source:
            fail(f"VerifiedUseToken contains forbidden implementation {marker}")
    token_impl_match = re.search(
        r"impl<C> VerifiedUseToken<C>.*?impl<C> fmt::Debug for VerifiedUseToken<C>",
        source,
        flags=re.DOTALL,
    )
    if not token_impl_match:
        fail("cannot locate VerifiedUseToken implementation boundary")
    if "pub fn new" in token_impl_match.group(0):
        fail("VerifiedUseToken exposes a public constructor")

    construction_sites: list[str] = []
    for path in (ROOT / "codex-rs").rglob("*.rs"):
        candidate = path.read_text(encoding="utf-8")
        if "VerifiedUseToken {" in candidate:
            construction_sites.append(path.relative_to(ROOT).as_posix())
    if construction_sites != [SOURCE]:
        fail(f"VerifiedUseToken construction escaped the kernel: {construction_sites}")

    callsites: set[str] = set()
    for path in (ROOT / "codex-rs").rglob("*.rs"):
        candidate = path.read_text(encoding="utf-8")
        if "verify_physical_capability_use" in candidate:
            callsites.add(path.relative_to(ROOT).as_posix())
    expected_callsites = {SOURCE, TESTS, LIB}
    if callsites != expected_callsites:
        fail(f"B0 changed or missed product verified-use call sites: {sorted(callsites)}")


def verify_tests_and_exports() -> None:
    require_markers(
        TESTS,
        (
            "exact_final_payload_issues_and_consumes_one_stable_token",
            "kind_action_mismatch_is_rejected_before_any_verifier_call",
            "local_broad_capability_cannot_cross_a_physical_write_boundary",
            "broad_authority_context_drift_and_expiry_fail_before_physical_verification",
            "requested_window_and_current_revocation_revision_are_fail_closed",
            "verifier_denial_and_expired_verifier_window_are_distinct",
            "final_operation_payload_context_kind_and_revision_drift_are_rejected",
            "final_crossing_time_must_be_inside_the_verified_window",
            "revocation_revision_and_window_reject_zero_or_empty_values",
        ),
    )
    library = text(LIB)
    if "mod verified_use;" not in library:
        fail("hepta-contracts does not compile the verified-use module")
    for marker in (
        "pub use verified_use::PhysicalCapabilityKind;",
        "pub use verified_use::PhysicalUseFinalCheck;",
        "pub use verified_use::PhysicalUseVerificationRequest;",
        "pub use verified_use::PhysicalUseVerifier;",
        "pub use verified_use::RevocationRevision;",
        "pub use verified_use::VerifiedUseToken;",
        "pub use verified_use::VerifiedUseWitness;",
        "pub use verified_use::verify_physical_capability_use;",
    ):
        if marker not in library:
            fail(f"hepta-contracts public verified-use surface is missing {marker}")


def verify_selected_plan_routing() -> None:
    require_markers(
        ROUTER,
        (
            "HEPTA_CURRENT_PLAN.json",
            "HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V4.md",
            "HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V5.md",
            "object_pairs_hook=hook",
            "no allowlisted verifier for selected plan",
            "subprocess.run",
        ),
    )
    for relative in (V4_WORKFLOW, P07A_WORKFLOW, P07A_ARM_WORKFLOW, WORKFLOW):
        require_markers(
            relative,
            (
                "scripts/verify-hepta-selected-architecture-plan.py",
                "git diff --exit-code",
            ),
        )
    require_markers(
        WORKFLOW,
        (
            "V5 B0 source qualification",
            "scripts/verify-hepta-v5-b0-format.py --check",
            "just test -p codex-hepta-contracts verified_use",
            "just test -p codex-hepta-contracts",
            "cargo check --locked --all-targets -p codex-hepta-contracts",
            "cargo clippy --locked --all-targets -p codex-hepta-contracts -- -D warnings",
            "just bazel-lock-check",
            "Upload exact-candidate evidence",
            "Hepta V5 B0 required",
        ),
    )


def verify_scoped_formatter() -> None:
    formatter = require_markers(
        FORMAT,
        (
            "codex-rs/hepta-contracts/src/lib.rs",
            "codex-rs/hepta-contracts/src/verified_use.rs",
            "codex-rs/hepta-contracts/src/verified_use_tests.rs",
            '"rustfmt"',
            '"--edition"',
            '"2024"',
            '"--config-path"',
            "rustfmt.toml",
            '"--check"',
        ),
    )
    if "rglob" in formatter or "cargo fmt --all" in formatter:
        fail("B0 scoped formatter expanded beyond its source-owned Rust files")


def main() -> None:
    verify_required_files()
    verify_pointer()
    verify_document_index()
    verify_plan()
    verify_ledger()
    verify_status()
    verify_delivery_contract()
    verify_verified_use_source()
    verify_tests_and_exports()
    verify_selected_plan_routing()
    verify_scoped_formatter()
    print("PASS_HEPTA_ARCHITECTURE_PLAN_V5_B0_SOURCE")


if __name__ == "__main__":
    main()
