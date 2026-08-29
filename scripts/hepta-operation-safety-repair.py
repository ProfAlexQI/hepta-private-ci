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


def apply_matrix_fault_injection() -> None:
    path = ROOT / "codex-rs/hepta-matrix-store/src/store.rs"
    replace_once(
        path,
        """use sqlx::Row;
use sqlx::Sqlite;
""",
        """#[cfg(feature = "qualification-fault-injection")]
use sqlx::AssertSqlSafe;
#[cfg(feature = "qualification-fault-injection")]
use sqlx::Connection;
use sqlx::Row;
use sqlx::Sqlite;
""",
        "Matrix qualification fault imports",
    )
    replace_once(
        path,
        """    pub async fn close(&self) {
        self.pool.close().await;
    }

    pub async fn bind_room(
""",
        """    pub async fn close(&self) {
        self.pool.close().await;
    }

    /// Execute one normal inbox transaction on a connection whose SQLite
    /// page-growth limit is temporarily pinned to the supplied value.
    ///
    /// This is compiled only for qualification. The limit and the product
    /// transaction share the same pooled connection, so the test cannot pass
    /// by constraining an unrelated control connection. The connection is
    /// reset before it returns to the pool, including after a failed write.
    #[cfg(feature = "qualification-fault-injection")]
    pub async fn ingest_inbox_with_max_page_count_for_qualification(
        &self,
        draft: &InboxDraft,
        max_page_count: u64,
    ) -> Result<InboxDisposition, MatrixDurableError> {
        if max_page_count == 0 {
            return Err(MatrixDurableError::Invalid);
        }
        let expected_limit = to_i64(max_page_count)?;
        let mut connection = self.pool.acquire().await.map_err(unavailable)?;
        let pragma = format!("PRAGMA max_page_count = {max_page_count}");
        let applied_limit: i64 = sqlx::query_scalar(AssertSqlSafe(pragma))
            .fetch_one(&mut *connection)
            .await
            .map_err(unavailable)?;
        if applied_limit != expected_limit {
            return Err(MatrixDurableError::Unavailable);
        }

        let write_result = async {
            let mut transaction = (&mut *connection).begin().await.map_err(unavailable)?;
            match self.ingest_inbox_tx(&mut transaction, draft).await {
                Ok(disposition) => {
                    transaction.commit().await.map_err(unavailable)?;
                    Ok(disposition)
                }
                Err(error) => {
                    transaction.rollback().await.map_err(unavailable)?;
                    Err(error)
                }
            }
        }
        .await;

        let reset_limit: i64 = sqlx::query_scalar("PRAGMA max_page_count = 2147483646")
            .fetch_one(&mut *connection)
            .await
            .map_err(unavailable)?;
        if reset_limit < applied_limit {
            return Err(MatrixDurableError::Unavailable);
        }
        write_result
    }

    pub async fn bind_room(
""",
        "Matrix same-connection SQLITE_FULL fault seam",
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
    replace_once(
        path,
        """def verify_real_store_fault() -> None:
    require_markers(
        "codex-rs/hepta-matrix-store/tests/sqlite_full.rs",
        (
            "real_matrix_sqlite_full_rolls_back_failed_inbox_and_preserves_operation_reopen",
            "PRAGMA max_page_count",
            "real SQLite growth must reach SQLITE_FULL",
            "the failed product transaction must not leave a partial inbox row",
            "MatrixDurableStore::open(&layout",
            "reopened_operation",
        ),
    )
""",
        """def verify_real_store_fault() -> None:
    require_markers(
        "codex-rs/hepta-matrix-store/src/store.rs",
        (
            "ingest_inbox_with_max_page_count_for_qualification",
            "PRAGMA max_page_count",
            "AssertSqlSafe",
            "transaction.rollback()",
            "PRAGMA max_page_count = 2147483646",
        ),
    )
    require_markers(
        "codex-rs/hepta-matrix-store/tests/sqlite_full.rs",
        (
            "real_matrix_sqlite_full_rolls_back_failed_inbox_and_preserves_operation_reopen",
            "ingest_inbox_with_max_page_count_for_qualification",
            "the product write transaction must observe SQLITE_FULL on its own connection",
            "the failed product transaction must not leave a partial inbox row",
            "MatrixDurableStore::open(&layout",
            "reopened_operation",
        ),
    )
""",
        "Matrix same-connection fault verifier",
    )
    replace_once(
        path,
        '        "cargo test --locked -p codex-hepta-matrix-store --test sqlite_full",\n',
        '        "cargo test --locked -p codex-hepta-matrix-store --features qualification-fault-injection --test sqlite_full",\n',
        "Matrix qualification workflow marker",
    )


def main() -> None:
    apply_automation()
    apply_matrix()
    apply_matrix_fault_injection()
    apply_provider()
    apply_verifier()
    print("PASS_HEPTA_OPERATION_SAFETY_REPAIR_SOURCE")


if __name__ == "__main__":
    main()
