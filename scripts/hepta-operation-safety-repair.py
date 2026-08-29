#!/usr/bin/env python3
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def replace_once(path: Path, old: str, new: str, label: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected one match, found {count}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def apply_automation() -> None:
    generated = ROOT / "scripts/hepta-automation-operation-repair-once.py"
    parts = sorted((ROOT / "scripts").glob("hepta-automation-operation-repair-once.py.part-*"))
    if len(parts) != 4:
        raise SystemExit(f"expected four Automation repair parts, found {len(parts)}")
    generated.write_text("".join(part.read_text(encoding="utf-8") for part in parts), encoding="utf-8")
    template = ROOT / "scripts/hepta-automation-model-v4.rs"
    template.write_text(template.read_text(encoding="utf-8").replace("occurrrence", "occurrence"), encoding="utf-8")
    subprocess.run(["python3", str(generated)], cwd=ROOT, check=True)


def apply_matrix() -> None:
    path = ROOT / "codex-rs/hepta-matrix-store/src/operation.rs"
    replace_once(
        path,
        """            OperationPhase::DeliveryClaimed
            | OperationPhase::Indeterminate
            | OperationPhase::Acknowledged
            | OperationPhase::ReconciledApplied
            | OperationPhase::ReconciledNotApplied
            | OperationPhase::Quarantined => Ok(current),
            _ => Err(MatrixDurableError::Corrupt),
""",
        """            // HEPTA_MATRIX_SINGLE_WINNER_CLAIM_V1: once the durable
            // boundary is claimed, all re-entry is lookup/reconcile only.
            OperationPhase::DeliveryClaimed
            | OperationPhase::Indeterminate
            | OperationPhase::Acknowledged
            | OperationPhase::ReconciledApplied
            | OperationPhase::ReconciledNotApplied
            | OperationPhase::Quarantined => Err(MatrixDurableError::Conflict),
            _ => Err(MatrixDurableError::Corrupt),
""",
        "Matrix claim state",
    )
    replace_once(
        path,
        """        let claimed = journal
            .claim_delivery(&event_id, &begun.record.envelope, 5)
            .await
            .expect("claim");
        assert_eq!(claimed.phase, OperationPhase::DeliveryClaimed);
""",
        """        let (first_claim, second_claim) = tokio::join!(
            journal.claim_delivery(&event_id, &begun.record.envelope, 5),
            journal.claim_delivery(&event_id, &begun.record.envelope, 5),
        );
        let claimed = match (first_claim, second_claim) {
            (Ok(claimed), Err(MatrixDurableError::Conflict)) => claimed,
            (Err(MatrixDurableError::Conflict), Ok(claimed)) => claimed,
            other => panic!("two concurrent Matrix delivery claims must have one winner: {other:?}"),
        };
        assert_eq!(claimed.phase, OperationPhase::DeliveryClaimed);
""",
        "Matrix concurrent claim test",
    )


def apply_provider() -> None:
    path = ROOT / "codex-rs/hepta-contracts/src/provider_operation.rs"
    replace_once(
        path,
        """            OperationPhase::DeliveryClaimed
            | OperationPhase::Indeterminate
            | OperationPhase::Acknowledged
            | OperationPhase::ReconciledApplied
            | OperationPhase::ReconciledNotApplied
            | OperationPhase::Quarantined => Ok(()),
            _ => Err(ProviderOperationError::BindingDrift),
""",
        """            // HEPTA_PROVIDER_SINGLE_WINNER_CLAIM_V1: a claimed or
            // settled provider operation may only use status lookup/reconcile.
            OperationPhase::DeliveryClaimed
            | OperationPhase::Indeterminate
            | OperationPhase::Acknowledged
            | OperationPhase::ReconciledApplied
            | OperationPhase::ReconciledNotApplied
            | OperationPhase::Quarantined => {
                Err(ProviderOperationError::DeliveryAlreadyClaimed)
            }
            _ => Err(ProviderOperationError::BindingDrift),
""",
        "Provider claim state",
    )
    replace_once(
        path,
        """    ExternalAuthorityRequired,
    LookupBeforeBoundary,
""",
        """    ExternalAuthorityRequired,
    DeliveryAlreadyClaimed,
    LookupBeforeBoundary,
""",
        "Provider error variant",
    )
    replace_once(
        path,
        """            Self::ExternalAuthorityRequired => formatter.write_str(
                "provider operation requires externally verified effect authority",
            ),
            Self::LookupBeforeBoundary => formatter.write_str(
""",
        """            Self::ExternalAuthorityRequired => formatter.write_str(
                "provider operation requires externally verified effect authority",
            ),
            Self::DeliveryAlreadyClaimed => formatter.write_str(
                "provider delivery boundary was already claimed; reconcile instead",
            ),
            Self::LookupBeforeBoundary => formatter.write_str(
""",
        "Provider display variant",
    )
    replace_once(
        path,
        """    if binding.is_expired_at(observed_at_unix_seconds) {
""",
        """    if binding.authority_epoch() != operation.envelope.binding.authority_epoch
        || binding.owner_epoch() != operation.envelope.binding.owner_epoch
        || binding.fencing_token_sha256()
            != &operation.envelope.binding.fencing_token_sha256
    {
        return Err(ProviderOperationError::ExternalAuthorityRequired);
    }
    if binding.is_expired_at(observed_at_unix_seconds) {
""",
        "Provider exact lease binding",
    )
    replace_once(
        path,
        'Sha256Digest::for_bytes(b"provider-fence")',
        'Sha256Digest::for_bytes(b"effect-fence")',
        "Provider test fence",
    )
    replace_once(
        path,
        """        let receipt = coordinator
            .dispatch_once(intent, 101)
            .await
            .unwrap_or_else(|error| panic!("dispatch must settle: {error}"));
        assert!(receipt.provider.physical_dispatch_attempted);
        assert_eq!(receipt.provider.state, ProviderEffectState::Completed);
        assert_eq!(receipt.operation_phase, OperationPhase::Acknowledged);
        assert_eq!(coordinator.operation().recovery_decision(), RecoveryDecision::Terminal);
""",
        """        let receipt = coordinator
            .dispatch_once(intent.clone(), 101)
            .await
            .unwrap_or_else(|error| panic!("dispatch must settle: {error}"));
        assert!(receipt.provider.physical_dispatch_attempted);
        assert_eq!(receipt.provider.state, ProviderEffectState::Completed);
        assert_eq!(receipt.operation_phase, OperationPhase::Acknowledged);
        assert_eq!(coordinator.operation().recovery_decision(), RecoveryDecision::Terminal);
        assert_eq!(
            coordinator.dispatch_once(intent, 102).await,
            Err(ProviderOperationError::DeliveryAlreadyClaimed),
        );
""",
        "Provider duplicate dispatch test",
    )


def apply_verifier() -> None:
    path = ROOT / "scripts/verify-hepta-cross-owner-operation-wiring.py"
    replace_once(
        path,
        """            "pub async fn claim_delivery",
            "pub async fn mark_indeterminate",
""",
        """            "pub async fn claim_delivery",
            "HEPTA_MATRIX_SINGLE_WINNER_CLAIM_V1",
            "two concurrent Matrix delivery claims must have one winner",
            "pub async fn mark_indeterminate",
""",
        "Matrix verifier markers",
    )
    replace_once(
        path,
        """            "external_effect.is_external()",
            "binding.is_expired_at(observed_at_unix_seconds)",
""",
        """            "external_effect.is_external()",
            "HEPTA_PROVIDER_SINGLE_WINNER_CLAIM_V1",
            "DeliveryAlreadyClaimed",
            "binding.authority_epoch()",
            "binding.owner_epoch()",
            "binding.fencing_token_sha256()",
            "binding.is_expired_at(observed_at_unix_seconds)",
""",
        "Provider verifier markers",
    )


def main() -> None:
    apply_automation()
    apply_matrix()
    apply_provider()
    apply_verifier()
    print("PASS_HEPTA_OPERATION_SAFETY_REPAIR_SOURCE")


if __name__ == "__main__":
    main()
