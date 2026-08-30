#!/usr/bin/env python3
"""Apply the bounded V5/B0 trusted-boundary source-contract update."""

from __future__ import annotations

import json
from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected one anchor, found {count}")
    return text.replace(old, new, 1)


def update_exports() -> None:
    path = Path("codex-rs/hepta-contracts/src/lib.rs")
    source = path.read_text()
    old = """pub use verified_use::PhysicalCapabilityKind;
pub use verified_use::PhysicalUseFinalCheck;
pub use verified_use::PhysicalUseVerification;
pub use verified_use::PhysicalUseVerificationRequest;
pub use verified_use::PhysicalUseVerifier;
pub use verified_use::PhysicalUseWindow;
pub use verified_use::RevocationRevision;
pub use verified_use::VERIFIED_USE_SCHEMA_VERSION;
pub use verified_use::VerifiedUseError;
pub use verified_use::VerifiedUseToken;
pub use verified_use::VerifiedUseWitness;
pub use verified_use::verify_physical_capability_use;"""
    new = """pub use verified_use::PhysicalCapabilityKind;
pub use verified_use::PhysicalUseClaimKey;
pub use verified_use::PhysicalUseClaimReceipt;
pub use verified_use::PhysicalUseClaimRequest;
pub use verified_use::PhysicalUseClaimStore;
pub use verified_use::PhysicalUseClaimStoreError;
pub use verified_use::PhysicalUseFinalCheck;
pub use verified_use::PhysicalUseVerification;
pub use verified_use::PhysicalUseVerificationRequest;
pub use verified_use::PhysicalUseVerifier;
pub use verified_use::PhysicalUseWindow;
pub use verified_use::RevocationRevision;
pub use verified_use::TrustedPhysicalClock;
pub use verified_use::VERIFIED_USE_SCHEMA_VERSION;
pub use verified_use::VerifiedUseBoundaryPermit;
pub use verified_use::VerifiedUseError;
pub use verified_use::VerifiedUseToken;
pub use verified_use::VerifiedUseWitness;
pub use verified_use::verify_physical_capability_use;"""
    path.write_text(replace_once(source, old, new, "lib exports"))


def update_verifier() -> None:
    path = Path("scripts/verify-hepta-architecture-plan-v5.py")
    source = path.read_text()
    source = replace_once(
        source,
        '"pub const VERIFIED_USE_SCHEMA_VERSION: u32 = 1",',
        '"pub const VERIFIED_USE_SCHEMA_VERSION: u32 = 2",',
        "schema marker",
    )
    source = replace_once(
        source,
        '                "pub trait PhysicalUseVerifier: CapabilityUseVerifier",\n',
        '                "pub trait TrustedPhysicalClock",\n'
        '                "pub trait PhysicalUseVerifier: CapabilityUseVerifier",\n'
        '                "pub struct PhysicalUseClaimKey",\n'
        '                "pub struct PhysicalUseClaimRequest",\n'
        '                "pub struct PhysicalUseClaimReceipt",\n'
        '                "pub trait PhysicalUseClaimStore",\n'
        '                "pub struct VerifiedUseBoundaryPermit<C>",\n'
        '                "pub fn consume_at_boundary",\n'
        '                "claim_once(",\n',
        "v2 source markers",
    )
    source = replace_once(
        source,
        '    if not re.search(r"pub fn consume\\(\\s*self,", source):\n'
        '        fail("VerifiedUseToken consumption must take self by value")\n',
        '    if not re.search(\n'
        '        r"pub fn consume_at_boundary<[^>]+>\\(\\s*self,",\n'
        '        source,\n'
        '        flags=re.DOTALL,\n'
        '    ):\n'
        '        fail("VerifiedUseToken boundary consumption must take self by value")\n'
        '    final_check = re.search(\n'
        '        r"pub struct PhysicalUseFinalCheck<[^>]+> \\{(.*?)\\n\\}",\n'
        '        source,\n'
        '        flags=re.DOTALL,\n'
        '    )\n'
        '    if not final_check:\n'
        '        fail("cannot locate PhysicalUseFinalCheck")\n'
        '    for forbidden in ("current_revocation_revision", "crossed_at_unix_seconds"):\n'
        '        if forbidden in final_check.group(1):\n'
        '            fail(f"PhysicalUseFinalCheck trusts caller-supplied {forbidden}")\n'
        '    token_impl = source[source.find("impl<C> VerifiedUseToken<C>") :]\n'
        '    for required in (\n'
        '        "trusted_now(clock)",\n'
        '        "verifier.verify_physical_use",\n'
        '        "claim_store.claim_once",\n'
        '        "VerifiedUseBoundaryPermit",\n'
        '    ):\n'
        '        if required not in token_impl:\n'
        '            fail(f"verified-use boundary is missing {required}")\n',
        "consume verifier",
    )
    old_tests = """            "exact_final_payload_issues_and_consumes_one_stable_token",
            "kind_action_mismatch_is_rejected_before_any_verifier_call",
            "local_broad_capability_cannot_cross_a_physical_write_boundary",
            "broad_authority_context_drift_and_expiry_fail_before_physical_verification",
            "requested_window_and_current_revocation_revision_are_fail_closed",
            "verifier_denial_and_expired_verifier_window_are_distinct",
            "final_operation_payload_context_kind_and_revision_drift_are_rejected",
            "final_crossing_time_must_be_inside_the_verified_window",
            "revocation_revision_and_window_reject_zero_or_empty_values","""
    new_tests = """            "trusted_final_verification_and_durable_claim_issue_one_permit",
            "durable_store_rejects_replay_and_same_operation_payload_conflict",
            "final_revision_and_clock_are_rechecked_before_claim",
            "caller_drift_local_authority_and_claim_failures_are_closed",
            "invalid_windows_revisions_and_trusted_clock_fail_closed","""
    source = replace_once(source, old_tests, new_tests, "test markers")
    source = replace_once(
        source,
        '        "pub use verified_use::PhysicalUseFinalCheck;",\n',
        '        "pub use verified_use::PhysicalUseClaimKey;",\n'
        '        "pub use verified_use::PhysicalUseClaimReceipt;",\n'
        '        "pub use verified_use::PhysicalUseClaimRequest;",\n'
        '        "pub use verified_use::PhysicalUseClaimStore;",\n'
        '        "pub use verified_use::PhysicalUseClaimStoreError;",\n'
        '        "pub use verified_use::PhysicalUseFinalCheck;",\n',
        "claim exports",
    )
    source = replace_once(
        source,
        '        "pub use verified_use::RevocationRevision;",\n',
        '        "pub use verified_use::RevocationRevision;",\n'
        '        "pub use verified_use::TrustedPhysicalClock;",\n'
        '        "pub use verified_use::VerifiedUseBoundaryPermit;",\n',
        "clock permit exports",
    )
    source = replace_once(
        source,
        '            "consumed by value",\n',
        '            "consumed by value",\n'
        '            "trusted clock",\n'
        '            "final boundary re-verification",\n'
        '            "durable single-use claim",\n'
        '            "PhysicalUseClaimStore",\n'
        '            "VerifiedUseBoundaryPermit<C>",\n'
        '            "effect completion",\n',
        "delivery markers",
    )
    source = replace_once(
        source,
        '        "current_revision_verifier",\n'
        '        "nonclone_nonserializable_private_token",\n',
        '        "current_revision_verifier",\n'
        '        "trusted_boundary_clock",\n'
        '        "final_boundary_reverification",\n'
        '        "durable_single_use_claim",\n'
        '        "boundary_permit_after_claim",\n'
        '        "witness_not_effect_completion",\n'
        '        "nonclone_nonserializable_private_token",\n',
        "ledger facts",
    )
    path.write_text(source)


def update_ledger() -> None:
    path = Path("docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V5.json")
    ledger = json.loads(path.read_text())
    b0 = next(
        row
        for row in ledger["packages"]
        if row["id"] == "P0.7b/B0_verified_use_kernel"
    )
    for fact in (
        "trusted_boundary_clock",
        "final_boundary_reverification",
        "durable_single_use_claim",
        "boundary_permit_after_claim",
        "witness_not_effect_completion",
    ):
        if fact not in b0["requiredFacts"]:
            b0["requiredFacts"].append(fact)
    path.write_text(json.dumps(ledger, indent=2) + "\n")


def update_plan() -> None:
    path = Path("docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V5.md")
    source = path.read_text()
    old = """- verifier response carrying current revision, verifier receipt digest and validity bound;
- `verify_physical_capability_use` composes the existing per-use verifier and returns a private-constructor token;
- `VerifiedUseToken<C>` has no `Clone`, `Copy`, `Serialize` or `Deserialize` implementation;
- boundary entry consumes the token by value and emits a digest-bound witness;
- final operation, payload, context, revision, kind and time are rechecked during consumption."""
    new = """- verifier response carrying current revision, verifier receipt digest and validity bound;
- issuance and crossing time are read from a trusted boundary clock rather than caller-supplied values;
- `verify_physical_capability_use` composes the existing per-use verifier and returns a private-constructor token;
- `VerifiedUseToken<C>` has no `Clone`, `Copy`, `Serialize` or `Deserialize` implementation;
- the final boundary re-verifies the current revision and exact final request;
- an atomic durable `PhysicalUseClaimStore::claim_once` prevents replay and same-operation/different-payload reuse across processes and crashes;
- only a successful durable claim yields a non-serializable `VerifiedUseBoundaryPermit<C>`;
- the permit is consumed by the adapter and emits a digest-bound witness that explicitly does not prove effect completion;
- final operation, payload, context, revision, kind and trusted time are rechecked during consumption."""
    path.write_text(replace_once(source, old, new, "plan B0 facts"))


def main() -> None:
    update_exports()
    update_verifier()
    update_ledger()
    update_plan()


if __name__ == "__main__":
    main()
