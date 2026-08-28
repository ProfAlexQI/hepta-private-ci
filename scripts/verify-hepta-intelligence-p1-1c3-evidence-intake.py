#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CRATE = ROOT / "codex-rs/hepta-memory-p1-1c3-qualification"
FILES = {
    "cargo": CRATE / "Cargo.toml",
    "lib": CRATE / "src/lib.rs",
    "digest": CRATE / "src/digest.rs",
    "trust": CRATE / "src/trust.rs",
    "qualification": CRATE / "src/qualification.rs",
    "review_trust": CRATE / "src/review_trust.rs",
    "intake": CRATE / "src/intake.rs",
    "binary": CRATE / "src/bin/p1_1c3_receipt.rs",
    "tests": CRATE / "tests/p1_1c3.rs",
    "plan": ROOT / "plans/hepta-intelligence/P1-1C3_TRUSTED_EVIDENCE_INTAKE_PLAN.md",
    "status": ROOT / "plans/hepta-intelligence/P1-1C3_EXECUTION_STATUS.json",
    "receipt": ROOT / "plans/hepta-intelligence/P1-1C3_IMPLEMENTATION_RECEIPT.json",
    "workflow": ROOT / ".github/workflows/hepta-intelligence-p1-1c3-evidence-intake.yml",
}


def contains_all(text: str, markers: tuple[str, ...]) -> bool:
    return all(marker in text for marker in markers)


def main() -> int:
    checks: dict[str, bool] = {
        "files.present": all(path.is_file() and path.stat().st_size > 0 for path in FILES.values())
    }
    if not checks["files.present"]:
        print(json.dumps({"status": "FAIL_P1_1C3_SOURCE", "checks": checks}, indent=2, sort_keys=True))
        return 1

    texts = {name: path.read_text(encoding="utf-8") for name, path in FILES.items() if path.suffix not in {".json"}}
    status = json.loads(FILES["status"].read_text(encoding="utf-8"))
    receipt = json.loads(FILES["receipt"].read_text(encoding="utf-8"))
    root_cargo = (ROOT / "codex-rs/Cargo.toml").read_text(encoding="utf-8")

    checks["crate.isolated"] = (
        "[workspace]" in texts["cargo"]
        and "hepta-memory-p1-1c3-qualification" not in root_cargo
        and 'publish = false' in texts["cargo"]
    )
    checks["crate.crypto_dependencies"] = contains_all(
        texts["cargo"],
        (
            'ed25519-dalek = "=2.2.0"',
            'sha2 = "=0.10.9"',
            'hepta-memory-p1-1c1-qualification',
        ),
    )
    checks["trust.real_ed25519"] = contains_all(
        texts["trust"],
        (
            "VerifyingKey::from_bytes",
            ".verify(",
            "signature key is revoked",
            "validity window",
            "ExternalAttested",
            "QualificationFixture",
            "allowed_locales",
            "affiliation_id",
        ),
    )
    checks["qualification.exact_executable_evidence"] = contains_all(
        texts["qualification"],
        (
            "source_commit",
            "source_tree",
            "workflow_run_id",
            "job_id",
            "runner_id",
            "step_count",
            "commands_executed",
            "artifact_sha256",
            "gates.all_pass()",
            "runner, artifact and step identities must be bounded and non-zero",
            "verify_signed_digest",
        ),
    )
    checks["review.signed_and_independent"] = contains_all(
        texts["review_trust"],
        (
            "ReviewAttestation",
            "AdjudicationAttestation",
            "review_record_digest",
            "adjudication_record_digest",
            "review pair affiliations are not independent",
            "adjudicator affiliation is not independent",
            "TrustRole::Reviewer",
            "TrustRole::Adjudicator",
        ),
    )
    checks["intake.full_governance_chain"] = contains_all(
        texts["intake"],
        (
            "LicenseEvidence",
            "ProvenanceEvidence",
            "PrivacyEvidence",
            "OperatorApprovalEvidence",
            "qualification.p1c_missing",
            "qualification.p1c1_missing",
            "review.trust_receipt_missing",
            "license.evidence_missing",
            "provenance.evidence_missing",
            "privacy.evidence_missing",
            "operator.approval_missing",
            "PASS_P1_1C3_TRUSTED_CORPUS_INTAKE",
            "BLOCKED_P1_1C3_TRUSTED_CORPUS_INTAKE",
        ),
    )
    checks["intake.authority_frozen_false"] = contains_all(
        texts["lib"],
        (
            "P1_1C3_RUNTIME_WIRED: bool = false",
            "P1_1C3_DEFAULT_RECALL_CHANGED: bool = false",
            "P1_1C3_CONTEXT_ATTACHMENT: bool = false",
            "P1_1C3_PHYSICAL_SEND: bool = false",
            "P1_1C3_PRODUCTION_AUTHORITY: bool = false",
            "P1_1C3_EFFICACY_CLAIM: bool = false",
            "P1_1C3_PROMOTION: bool = false",
        ),
    )
    checks["tests.negative_matrix"] = contains_all(
        texts["tests"],
        (
            "fully_signed_external_mechanics_can_pass_without_product_authority",
            "qualification_rejects_zero_runner_or_steps",
            "qualification_rejects_exact_source_drift",
            "tampered_ed25519_signature_is_rejected",
            "external_policy_rejects_qualification_fixture_key",
            "same_affiliation_reviewers_are_rejected",
            "missing_review_signature_is_rejected",
            "locale_unauthorized_reviewer_is_rejected",
            "disallowed_license_blocks_intake",
            "failed_privacy_assessment_blocks_intake",
            "dataset_digest_drift_blocks_intake",
            "operator_subject_drift_blocks_intake",
            "receipt_rejects_duplicate_or_noncanonical_blockers",
        ),
    )
    checks["binary.blocked_fixture_only"] = contains_all(
        texts["binary"],
        (
            "QualificationFixture",
            "p1c_qualification: None",
            "license: None",
            "operator_approval: None",
            "assert!(!receipt.mechanically_accepted)",
        ),
    )
    checks["source.no_product_or_network_api"] = not any(
        marker in "\n".join(texts.values())
        for marker in (
            "std::net",
            "reqwest",
            "tokio::net",
            "CognitiveStore",
            "cognitive_retrieval",
            "physical_send(",
            "ProductionAuthority",
        )
    )
    checks["plan.external_evidence_honesty"] = contains_all(
        texts["plan"],
        (
            "external evidence absent",
            "does not fabricate it",
            "real reviewer and adjudicator",
            "production_authority=false",
        ),
    )
    authority = status.get("authority", {})
    checks["status.boundary"] = (
        status.get("status") == "SOURCE_IMPLEMENTED_EXTERNAL_EVIDENCE_ABSENT"
        and status.get("implemented") is True
        and status.get("source_qualified") is False
        and status.get("external_evidence_present") is False
        and status.get("trusted_corpus_accepted") is False
        and all(value is False for value in authority.values())
    )
    checks["receipt.boundary"] = (
        receipt.get("status") == "SOURCE_IMPLEMENTED_EXTERNAL_EVIDENCE_ABSENT"
        and receipt.get("claims", {}).get("real_external_evidence_present") is False
        and receipt.get("claims", {}).get("trusted_corpus_accepted") is False
        and all(value is False for value in receipt.get("authority", {}).values())
    )
    checks["workflow.exact_matrix"] = contains_all(
        texts["workflow"],
        (
            'toolchain: "1.95.0"',
            "verify-hepta-intelligence-p1-1c3-evidence-intake.py",
            "cargo fmt --manifest-path",
            "cargo test --manifest-path",
            "expected 16 P1.1c.3 tests",
            "cargo check --manifest-path",
            "cargo clippy --manifest-path",
            "--all-targets -- -D warnings",
            "p1_1c3_receipt",
            "cmp",
            "BLOCKED_P1_1C3_TRUSTED_CORPUS_INTAKE",
        ),
    )

    failures = sorted(name for name, passed in checks.items() if not passed)
    output = {
        "schema": "hepta.intelligence.p1_1c3.source_gate.v1",
        "status": "PASS_P1_1C3_SOURCE_ONLY" if not failures else "FAIL_P1_1C3_SOURCE",
        "implemented": not failures,
        "source_qualified": False,
        "external_evidence_present": False,
        "trusted_corpus_accepted": False,
        "runtime_wired": False,
        "production_authority": False,
        "checks": checks,
        "failures": failures,
    }
    print(json.dumps(output, indent=2, sort_keys=True))
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())
