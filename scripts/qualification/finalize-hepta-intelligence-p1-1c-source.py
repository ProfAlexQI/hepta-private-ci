#!/usr/bin/env python3
"""Finalize P1.1c source qualification after executable gates pass.

This helper lives only on an isolated qualification wrapper branch. The output
commit contains the reviewed source formatting plus governance/evidence files,
never this helper.
"""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path

ROOT = Path.cwd()
STATUS_PATH = ROOT / "plans/hepta-intelligence/P1-1C_EXECUTION_STATUS.json"
EVIDENCE_PATH = ROOT / "plans/hepta-intelligence/P1-1C_SOURCE_QUALIFICATION_EVIDENCE.json"
VERIFIER_PATH = ROOT / "scripts/verify-hepta-intelligence-p1-1c-offline-efficacy.py"
WORKFLOW_PATH = ROOT / ".github/workflows/hepta-intelligence-p1-1c-offline-efficacy.yml"


def replace_once(path: Path, old: str, new: str, label: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected one anchor in {path}, found {count}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def required_env(name: str) -> str:
    value = os.environ.get(name, "").strip()
    if not value:
        raise SystemExit(f"missing required environment variable {name}")
    return value


def update_status(source_parent: str, run_id: int, receipt_sha256: str) -> None:
    status = json.loads(STATUS_PATH.read_text(encoding="utf-8"))
    if status.get("schema") != "hepta.intelligence.p1_1c.execution_status.v1":
        raise SystemExit("unexpected P1.1c execution status schema")
    if status.get("status") != "IMPLEMENTED_PENDING_EXECUTABLE_QUALIFICATION":
        raise SystemExit("P1.1c execution status is not in the expected pending state")
    qualification = status["qualification"]
    for key in [
        "source_qualified",
        "executable_gates_passed",
        "seed_pipeline_reproducible",
        "efficacy_validation",
        "operator_acceptance",
    ]:
        if qualification[key] is not False:
            raise SystemExit(f"unexpected pre-finalization value qualification.{key}")
    status["status"] = "SOURCE_QUALIFIED_SEED_PIPELINE"
    qualification["source_qualified"] = True
    qualification["executable_gates_passed"] = True
    qualification["seed_pipeline_reproducible"] = True
    qualification["evidence_file"] = str(EVIDENCE_PATH.relative_to(ROOT))
    qualification["evidence_run_id"] = run_id
    qualification["evidence_source_parent"] = source_parent
    qualification["evaluation_receipt_sha256"] = receipt_sha256
    STATUS_PATH.write_text(
        json.dumps(status, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def write_evidence(source_parent: str, run_id: int, run_attempt: int, receipt_sha256: str) -> None:
    evidence = {
        "schema": "hepta.intelligence.p1_1c.source_qualification_evidence.v1",
        "status": "PASS_P1_1C_SOURCE_QUALIFICATION",
        "source_parent_commit": source_parent,
        "qualification_run_id": run_id,
        "qualification_run_attempt": run_attempt,
        "rust_toolchain": "1.95.0",
        "runner_os": required_env("RUNNER_OS"),
        "runner_arch": required_env("RUNNER_ARCH"),
        "tests_passed": 13,
        "evaluation_receipt_sha256": receipt_sha256,
        "corpus": {
            "provenance": "synthetic_seed",
            "reviewed": False,
            "locale_count": 8,
            "case_count": 8,
            "candidate_count": 48,
        },
        "gates": {
            "stack_base": "success",
            "p1b_bounded_decode_prerequisite": "success",
            "format_scope": "success",
            "source_gate": "success",
            "rustfmt": "success",
            "tests": "success",
            "check": "success",
            "clippy": "success",
            "receipt_reproducibility": "success",
            "receipt_json_and_redaction": "success",
        },
        "source_qualified": True,
        "qualified": False,
        "efficacy_validation": False,
        "efficacy_claim": False,
        "product_workspace_member": False,
        "product_module_registered": False,
        "runtime_wired": False,
        "default_recall_changed": False,
        "federation_recall_changed": False,
        "context_attachment": False,
        "physical_send": False,
        "network_access": False,
        "model_download": False,
        "external_effects": False,
        "production_authority": False,
        "operator_acceptance": False,
        "promotion": False,
        "callers_ratchet": False,
    }
    EVIDENCE_PATH.write_text(
        json.dumps(evidence, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def patch_verifier() -> None:
    replace_once(
        VERIFIER_PATH,
        'STATUS = ROOT / "plans/hepta-intelligence/P1-1C_EXECUTION_STATUS.json"\n',
        'STATUS = ROOT / "plans/hepta-intelligence/P1-1C_EXECUTION_STATUS.json"\n'
        'EVIDENCE = ROOT / "plans/hepta-intelligence/P1-1C_SOURCE_QUALIFICATION_EVIDENCE.json"\n',
        "verifier-evidence-path",
    )
    replace_once(
        VERIFIER_PATH,
        '''    false_paths = [
        ("qualification", "source_qualified"),
        ("qualification", "efficacy_validation"),
        ("authority", "runtime_wired"),
        ("authority", "network_access"),
        ("authority", "production_authority"),
        ("authority", "promotion"),
    ]
    for group, key in false_paths:
        if status[group][key] is not False:
            fail(f"status field {group}.{key} must remain false before qualification")
    if status["corpus"]["provenance"] != "synthetic_seed":
        fail("status corpus provenance must remain synthetic_seed")
''',
        '''    false_paths = [
        ("qualification", "efficacy_validation"),
        ("qualification", "operator_acceptance"),
        ("authority", "runtime_wired"),
        ("authority", "network_access"),
        ("authority", "production_authority"),
        ("authority", "promotion"),
    ]
    for group, key in false_paths:
        if status[group][key] is not False:
            fail(f"status field {group}.{key} must remain false")
    if status["corpus"]["provenance"] != "synthetic_seed":
        fail("status corpus provenance must remain synthetic_seed")

    source_qualified = status["qualification"]["source_qualified"]
    if source_qualified:
        if status["status"] != "SOURCE_QUALIFIED_SEED_PIPELINE":
            fail("source-qualified status must use SOURCE_QUALIFIED_SEED_PIPELINE")
        evidence = json.loads(read(EVIDENCE))
        if evidence.get("schema") != "hepta.intelligence.p1_1c.source_qualification_evidence.v1":
            fail("unexpected P1.1c source qualification evidence schema")
        if evidence.get("status") != "PASS_P1_1C_SOURCE_QUALIFICATION":
            fail("P1.1c source qualification evidence is not PASS")
        if evidence.get("tests_passed") != 13:
            fail("P1.1c source qualification evidence must bind 13 tests")
        if evidence.get("source_parent_commit") != status["qualification"].get("evidence_source_parent"):
            fail("P1.1c evidence source parent does not match execution status")
        if evidence.get("qualification_run_id") != status["qualification"].get("evidence_run_id"):
            fail("P1.1c evidence run ID does not match execution status")
        if evidence.get("evaluation_receipt_sha256") != status["qualification"].get("evaluation_receipt_sha256"):
            fail("P1.1c evaluation receipt digest does not match execution status")
        required_success = {
            "stack_base",
            "p1b_bounded_decode_prerequisite",
            "format_scope",
            "source_gate",
            "rustfmt",
            "tests",
            "check",
            "clippy",
            "receipt_reproducibility",
            "receipt_json_and_redaction",
        }
        if {key for key, value in evidence.get("gates", {}).items() if value == "success"} != required_success:
            fail("P1.1c evidence gate set is incomplete or contains unexpected entries")
        for key in [
            "qualified",
            "efficacy_validation",
            "efficacy_claim",
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
        ]:
            if evidence.get(key) is not False:
                fail(f"P1.1c evidence field {key} must remain false")
    else:
        if status["status"] != "IMPLEMENTED_PENDING_EXECUTABLE_QUALIFICATION":
            fail("pending P1.1c source must keep pending execution status")
        if EVIDENCE.exists():
            fail("pending P1.1c source must not carry qualification evidence")
''',
        "verifier-governance-transition",
    )
    replace_once(
        VERIFIER_PATH,
        '        "source_qualified": False,\n',
        '        "source_qualified": status["qualification"]["source_qualified"],\n',
        "verifier-source-qualified-receipt",
    )


def patch_workflow_allowlist() -> None:
    anchor = '              "plans/hepta-intelligence/P1-1C_EXECUTION_STATUS.json",\n'
    addition = '              "plans/hepta-intelligence/P1-1C_SOURCE_QUALIFICATION_EVIDENCE.json",\n'
    text = WORKFLOW_PATH.read_text(encoding="utf-8")
    if addition in text:
        raise SystemExit("dedicated workflow already contains evidence allowlist path")
    if text.count(anchor) != 1:
        raise SystemExit("dedicated workflow status allowlist anchor is not unique")
    WORKFLOW_PATH.write_text(text.replace(anchor, anchor + addition, 1), encoding="utf-8")


def main() -> None:
    source_parent = required_env("SOURCE_PARENT")
    run_id = int(required_env("RUN_ID"))
    run_attempt = int(required_env("RUN_ATTEMPT"))
    receipt_path = Path(required_env("EVALUATION_RECEIPT_PATH"))
    receipt_bytes = receipt_path.read_bytes()
    receipt_sha256 = hashlib.sha256(receipt_bytes).hexdigest()
    expected_sha256 = required_env("EVALUATION_RECEIPT_SHA256")
    if receipt_sha256 != expected_sha256:
        raise SystemExit("evaluation receipt digest changed before finalization")

    update_status(source_parent, run_id, receipt_sha256)
    write_evidence(source_parent, run_id, run_attempt, receipt_sha256)
    patch_verifier()
    patch_workflow_allowlist()

    print(
        json.dumps(
            {
                "status": "PASS_P1_1C_SOURCE_FINALIZATION_PATCH",
                "source_parent": source_parent,
                "qualification_run_id": run_id,
                "evaluation_receipt_sha256": receipt_sha256,
                "governance_paths": [
                    str(STATUS_PATH.relative_to(ROOT)),
                    str(EVIDENCE_PATH.relative_to(ROOT)),
                    str(VERIFIER_PATH.relative_to(ROOT)),
                    str(WORKFLOW_PATH.relative_to(ROOT)),
                ],
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
