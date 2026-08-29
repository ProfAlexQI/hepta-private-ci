#!/usr/bin/env python3
"""Fail-closed source verifier for the selected Hepta architecture V5 plan."""

from __future__ import annotations

import json
import pathlib
import re
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
ARCH = ROOT / "docs" / "architecture"

CURRENT = ARCH / "HEPTA_CURRENT_PLAN.json"
MODEL = ARCH / "HEPTA_ARCHITECTURE_MODEL_V2.json"
PLAN = ARCH / "HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V5.md"
LEDGER = ARCH / "HEPTA_ARCHITECTURE_GAP_LEDGER_V5.json"
INDEX = ARCH / "HEPTA_DOCUMENT_AUTHORITY_INDEX_V4.json"
STATUS = ARCH / "HEPTA_QUALIFICATION_STATUS_V4.json"
PACKAGE_CONTRACT = ARCH / "HEPTA_PACKAGE_EXECUTION_CONTRACT_V1.md"
DEPENDENCY_POLICY = ARCH / "HEPTA_DEPENDENCY_POLICY_V1.json"
V5_WORKFLOW = ROOT / ".github" / "workflows" / "hepta-architecture-plan-v5.yml"
P07A_WORKFLOW = ROOT / ".github" / "workflows" / "hepta-runtime-bootstrap-p0-7a.yml"
TEMP_SOURCE_WRITER = ROOT / ".github" / "workflows" / "hepta-p0-7a-scope-repair.yml"

EXPECTED_PARENT_COMMIT = "b1bdd44ee6cbedbcc249150968448e25f5ce859c"
EXPECTED_PARENT_TREE = "14f6db9aeb747aa0c3c468799d945a83a661e1d1"
EXPECTED_BRANCH = "codex/architecture-v2-blocker-closure-20260830"

ALL_FALSE_KEYS = {
    "runtime",
    "productionCaller",
    "productionWriter",
    "modelInvocation",
    "providerDispatch",
    "toolExecution",
    "networkConnect",
    "externalEffect",
    "fleetMutation",
    "operatorAcceptance",
    "promotion",
    "release",
}

REQUIRED_PACKAGES = {
    "P0.7a_signedRuntimeBootstrapClosure",
    "P0.7b_verifiedUsePhysicalBoundaries",
    "P0.7c_memoryBoundedContextExtraction",
    "P0.7d_commonDurableFaultMatrix",
    "P0.7e_dependencyInversionAndWireIsolation",
    "P0.8a_astAuthorityCallerRatchet",
    "P0.8b_fleetRuntimeInstanceProjection",
    "P0.8c_runtimeResourceBudgetEnforcement",
    "P0.8d_realProcessVerticalSlice",
}

REQUIRED_PLAN_HEADINGS = {
    "## 1. Completion model",
    "## 2. Non-negotiable invariants",
    "## 3. Package operating contract",
    "## 5. P0.7a — signed runtime-bootstrap closure",
    "## 6. P0.7b — per-use physical capability boundaries",
    "## 7. P0.7c — Memory bounded-context extraction",
    "## 8. P0.7d — common durable fault matrix",
    "## 9. P0.7e — dependency inversion and wire isolation",
    "## 10. P0.8a — AST/compiler authority caller ratchet",
    "## 11. P0.8b — fleet runtime-instance projection",
    "## 12. P0.8c — resource-budget enforcement",
    "## 13. P0.8d — exact real-process vertical slice",
    "## 18. P0.9 external gates",
    "## 20. Definition of done",
}


class VerificationError(RuntimeError):
    """A deterministic source-contract failure."""


def object_pairs_no_duplicates(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise VerificationError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def load_json(path: pathlib.Path) -> dict[str, Any]:
    if not path.is_file():
        raise VerificationError(f"missing required JSON: {path.relative_to(ROOT)}")
    try:
        value = json.loads(
            path.read_text(encoding="utf-8"),
            object_pairs_hook=object_pairs_no_duplicates,
        )
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise VerificationError(f"invalid JSON {path.relative_to(ROOT)}: {error}") from error
    if not isinstance(value, dict):
        raise VerificationError(f"JSON root must be an object: {path.relative_to(ROOT)}")
    return value


def require_file(path: pathlib.Path) -> None:
    if not path.is_file():
        raise VerificationError(f"missing required file: {path.relative_to(ROOT)}")


def require_all_false(value: dict[str, Any], label: str) -> None:
    missing = sorted(ALL_FALSE_KEYS - set(value))
    if missing:
        raise VerificationError(f"{label} is missing authority keys: {missing}")
    enabled = sorted(key for key in ALL_FALSE_KEYS if value.get(key) is not False)
    if enabled:
        raise VerificationError(f"{label} grants authority: {enabled}")


def verify_current_pointer(current: dict[str, Any]) -> None:
    expected = {
        "schema": "hepta.current-plan.v2",
        "schemaVersion": 2,
        "currentPlan": "docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V5.md",
        "currentGapLedger": "docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V5.json",
        "documentAuthorityIndex": "docs/architecture/HEPTA_DOCUMENT_AUTHORITY_INDEX_V4.json",
        "qualificationStatus": "docs/architecture/HEPTA_QUALIFICATION_STATUS_V4.json",
        "packageExecutionContract": "docs/architecture/HEPTA_PACKAGE_EXECUTION_CONTRACT_V1.md",
        "dependencyPolicy": "docs/architecture/HEPTA_DEPENDENCY_POLICY_V1.json",
    }
    for key, value in expected.items():
        if current.get(key) != value:
            raise VerificationError(f"current plan pointer mismatch for {key}")
    binding = current.get("candidateBinding")
    if not isinstance(binding, dict):
        raise VerificationError("current plan candidateBinding is missing")
    if binding.get("parentCommit") != EXPECTED_PARENT_COMMIT:
        raise VerificationError("current plan parent commit drift")
    if binding.get("parentTree") != EXPECTED_PARENT_TREE:
        raise VerificationError("current plan parent tree drift")
    if binding.get("implementationBranch") != EXPECTED_BRANCH:
        raise VerificationError("current plan implementation branch drift")
    claims = current.get("claims")
    if not isinstance(claims, dict) or claims.get("sourcePlanCurrent") is not True:
        raise VerificationError("current plan is not selected")
    for key in (
        "allSourceGapsClosed",
        "exactHeadQualified",
        "mergeCandidateQualified",
        "operatorAccepted",
        "promoted",
        "released",
    ):
        if claims.get(key) is not False:
            raise VerificationError(f"unearned current-plan claim: {key}")
    require_all_false(current.get("authority", {}), "current plan")


def verify_plan() -> None:
    require_file(PLAN)
    text = PLAN.read_text(encoding="utf-8")
    missing = sorted(REQUIRED_PLAN_HEADINGS - set(text.splitlines()))
    if missing:
        raise VerificationError(f"V5 plan missing headings: {missing}")
    for phrase in (
        "source_implemented never means a test passed",
        "No qualification workflow commits, pushes",
        "All gaps closed",
        "blocked_external",
    ):
        if phrase.lower() not in text.lower():
            raise VerificationError(f"V5 plan missing fail-closed phrase: {phrase}")


def verify_ledger(ledger: dict[str, Any]) -> None:
    if ledger.get("schema") != "hepta.architecture-gap-ledger.v5":
        raise VerificationError("wrong V5 gap-ledger schema")
    if ledger.get("schemaVersion") != 5:
        raise VerificationError("wrong V5 gap-ledger version")
    subject = ledger.get("subject")
    if not isinstance(subject, dict):
        raise VerificationError("gap ledger subject is missing")
    if subject.get("parentCommit") != EXPECTED_PARENT_COMMIT:
        raise VerificationError("gap ledger parent commit drift")
    if subject.get("parentTree") != EXPECTED_PARENT_TREE:
        raise VerificationError("gap ledger parent tree drift")
    if subject.get("implementationBranch") != EXPECTED_BRANCH:
        raise VerificationError("gap ledger branch drift")
    require_all_false(ledger.get("authorityBoundary", {}), "gap ledger")
    packages = ledger.get("sourcePackages")
    if not isinstance(packages, dict):
        raise VerificationError("sourcePackages must be an object")
    missing = sorted(REQUIRED_PACKAGES - set(packages))
    if missing:
        raise VerificationError(f"gap ledger missing packages: {missing}")
    allowed_states = set(ledger.get("stateVocabulary", []))
    for package_id, package in packages.items():
        if not isinstance(package, dict):
            raise VerificationError(f"package is not an object: {package_id}")
        if package.get("state") not in allowed_states:
            raise VerificationError(f"unknown package state: {package_id}")
        if package.get("authorityDelta") != "none":
            raise VerificationError(f"package has an authority delta: {package_id}")
        for key in ("owner", "prerequisites", "subpackages", "touchedPathFamilies", "exitCriteria"):
            if key not in package:
                raise VerificationError(f"package {package_id} missing {key}")
    direct = ledger.get("directGaps")
    if not isinstance(direct, dict):
        raise VerificationError("directGaps must be an object")
    if direct.get("A-LOCK-01", {}).get("state") != "source_implemented":
        raise VerificationError("committed lock source is not represented honestly")
    if direct.get("A-CI-01", {}).get("state") != "blocked_external":
        raise VerificationError("runner gap must remain blocked_external")
    completion = ledger.get("completion")
    if not isinstance(completion, dict):
        raise VerificationError("completion block is missing")
    if any(value is not False for value in completion.values()):
        raise VerificationError("gap ledger contains an unearned completion claim")


def verify_index(index: dict[str, Any], current: dict[str, Any]) -> None:
    if index.get("schema") != "hepta.document-authority-index.v4":
        raise VerificationError("wrong document-authority schema")
    if index.get("schemaVersion") != 4:
        raise VerificationError("wrong document-authority version")
    expected_order = [
        "docs/architecture/HEPTA_CURRENT_PLAN.json",
        "docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json",
        current["currentPlan"],
        current["currentGapLedger"],
        current["packageExecutionContract"],
        current["dependencyPolicy"],
        current["qualificationStatus"],
    ]
    if index.get("resolutionOrder") != expected_order:
        raise VerificationError("document authority resolution order drift")
    documents = index.get("documents")
    if not isinstance(documents, list):
        raise VerificationError("document authority documents must be a list")
    ids: set[str] = set()
    paths: set[str] = set()
    for entry in documents:
        if not isinstance(entry, dict):
            raise VerificationError("document index entry must be an object")
        document_id = entry.get("id")
        path = entry.get("path")
        if not isinstance(document_id, str) or document_id in ids:
            raise VerificationError(f"duplicate or invalid document id: {document_id}")
        if not isinstance(path, str) or path in paths:
            raise VerificationError(f"duplicate or invalid document path: {path}")
        ids.add(document_id)
        paths.add(path)
        require_file(ROOT / path)
    rules = index.get("rules")
    if not isinstance(rules, dict):
        raise VerificationError("document rules are missing")
    required_false = (
        "draftPrCanSelectCurrentPlan",
        "receiptCanRewritePlan",
        "qualificationWorkflowCanWriteSource",
        "sourceCanSelfIssueOperatorAcceptance",
        "sourceCanSelfIssuePromotion",
        "sourceCanSelfIssueRelease",
        "buildLockDriftCanPass",
    )
    for key in required_false:
        if rules.get(key) is not False:
            raise VerificationError(f"unsafe document rule: {key}")


def verify_status(status: dict[str, Any]) -> None:
    if status.get("schema") != "hepta.qualification-status.v4":
        raise VerificationError("wrong qualification-status schema")
    if status.get("schemaVersion") != 4:
        raise VerificationError("wrong qualification-status version")
    require_all_false(status.get("authorityBoundary", {}), "qualification status")
    observed = status.get("observedParentExecution")
    if not isinstance(observed, dict) or observed.get("qualificationClaim") != "not_run":
        raise VerificationError("runner-zero parent evidence must remain not_run")
    external = status.get("externalDecisions")
    if not isinstance(external, dict):
        raise VerificationError("external decision block is missing")
    for decision, value in external.items():
        if not isinstance(value, dict) or value.get("state") != "not_issued":
            raise VerificationError(f"unearned external decision: {decision}")


def verify_dependency_policy(policy: dict[str, Any]) -> None:
    if policy.get("schema") != "hepta.dependency-policy.v1":
        raise VerificationError("wrong dependency-policy schema")
    if policy.get("schemaVersion") != 1:
        raise VerificationError("wrong dependency-policy version")
    require_all_false(
        {
            **{key: False for key in ALL_FALSE_KEYS},
            **policy.get("authority", {}),
        },
        "dependency policy",
    )
    rules = policy.get("rules")
    debts = policy.get("baselineDebts")
    if not isinstance(rules, list) or not isinstance(debts, list):
        raise VerificationError("dependency rules and debts must be lists")
    rule_ids = {rule.get("id") for rule in rules if isinstance(rule, dict)}
    if len(rule_ids) != len(rules) or None in rule_ids:
        raise VerificationError("dependency rule IDs are invalid")
    debt_ids: set[str] = set()
    debt_edges: set[tuple[str, str]] = set()
    for debt in debts:
        if not isinstance(debt, dict):
            raise VerificationError("dependency debt must be an object")
        debt_id = debt.get("id")
        if not isinstance(debt_id, str) or debt_id in debt_ids:
            raise VerificationError(f"duplicate dependency debt: {debt_id}")
        debt_ids.add(debt_id)
        if debt.get("rule") not in rule_ids:
            raise VerificationError(f"dependency debt references unknown rule: {debt_id}")
        if debt.get("state") != "open":
            raise VerificationError(f"closed dependency debt must be removed: {debt_id}")
        edge = debt.get("edge")
        if edge is not None:
            if not (
                isinstance(edge, list)
                and len(edge) == 2
                and all(isinstance(part, str) for part in edge)
            ):
                raise VerificationError(f"invalid debt edge: {debt_id}")
            pair = (edge[0], edge[1])
            if pair in debt_edges:
                raise VerificationError(f"duplicate dependency edge debt: {pair}")
            debt_edges.add(pair)


def verify_workflow_read_only(path: pathlib.Path) -> None:
    require_file(path)
    text = path.read_text(encoding="utf-8")
    if re.search(r"(?m)^\s*contents:\s*write\s*$", text):
        raise VerificationError(f"workflow writes contents: {path.relative_to(ROOT)}")
    for forbidden in ("git push", "git commit", "git update-ref", "gh api --method PATCH"):
        if forbidden in text:
            raise VerificationError(
                f"workflow mutates reviewed source ({forbidden}): {path.relative_to(ROOT)}"
            )


def verify_source_hygiene() -> None:
    if TEMP_SOURCE_WRITER.exists():
        raise VerificationError(
            "temporary source-mutating P0.7a workflow must be deleted"
        )
    verify_workflow_read_only(V5_WORKFLOW)
    verify_workflow_read_only(P07A_WORKFLOW)
    for lock in (ROOT / "codex-rs" / "Cargo.lock", ROOT / "MODULE.bazel.lock"):
        require_file(lock)
        if lock.stat().st_size == 0:
            raise VerificationError(f"empty lock file: {lock.relative_to(ROOT)}")


def main() -> int:
    for path in (
        CURRENT,
        MODEL,
        PLAN,
        LEDGER,
        INDEX,
        STATUS,
        PACKAGE_CONTRACT,
        DEPENDENCY_POLICY,
        V5_WORKFLOW,
        P07A_WORKFLOW,
    ):
        require_file(path)

    current = load_json(CURRENT)
    ledger = load_json(LEDGER)
    index = load_json(INDEX)
    status = load_json(STATUS)
    policy = load_json(DEPENDENCY_POLICY)

    verify_current_pointer(current)
    verify_plan()
    verify_ledger(ledger)
    verify_index(index, current)
    verify_status(status)
    verify_dependency_policy(policy)
    verify_source_hygiene()

    print("PASS: Hepta architecture V5 source contract")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except VerificationError as error:
        print(f"FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
