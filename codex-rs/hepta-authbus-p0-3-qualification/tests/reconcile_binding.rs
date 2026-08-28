use codex_hepta_authbus_p0_3_qualification::CanonicalQuotaVector;
use codex_hepta_authbus_p0_3_qualification::P03Fence;
use codex_hepta_authbus_p0_3_qualification::P03OldPermitReconcileRequest;
use codex_hepta_authbus_p0_3_qualification::P03ReconcileOutcome;
use codex_hepta_contracts::Sha256Digest;

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

fn fence(owner_epoch: u64, generation: u64, label: &str) -> P03Fence {
    P03Fence {
        authority_epoch: 3,
        owner_epoch,
        generation,
        fencing_token_sha256: digest(label),
    }
}

fn reconcile_request(expected_revision: u64) -> P03OldPermitReconcileRequest {
    P03OldPermitReconcileRequest {
        permit_id: "permit-binding".to_string(),
        old_fence: fence(7, 11, "old-fence"),
        current_fence: fence(8, 12, "current-fence"),
        provider_status_receipt_sha256: digest("provider-status"),
        owner_evidence_sha256: digest("owner-evidence"),
        expected_revision,
        observed_at_ms: 1_700,
        outcome: P03ReconcileOutcome::VerifiedConsumed {
            actual: CanonicalQuotaVector::new(1, 1, 64, 1, 64, 128),
        },
    }
}

#[test]
fn reconcile_digest_binds_expected_revision() {
    let first = reconcile_request(17).digest().expect("first digest");
    let second = reconcile_request(18).digest().expect("second digest");
    assert_ne!(first, second);
}
