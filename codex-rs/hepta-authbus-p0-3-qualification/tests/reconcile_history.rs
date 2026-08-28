use codex_hepta_authbus_p0_3_qualification::CanonicalQuotaLimits;
use codex_hepta_authbus_p0_3_qualification::CanonicalQuotaVector;
use codex_hepta_authbus_p0_3_qualification::P03AdmissionDisposition;
use codex_hepta_authbus_p0_3_qualification::P03Fence;
use codex_hepta_authbus_p0_3_qualification::P03LocalScheduler;
use codex_hepta_authbus_p0_3_qualification::P03OldPermitReconcileRequest;
use codex_hepta_authbus_p0_3_qualification::P03ReconcileDisposition;
use codex_hepta_authbus_p0_3_qualification::P03ReconcileOutcome;
use codex_hepta_authbus_p0_3_qualification::P03ReservationRequest;
use codex_hepta_authbus_p0_3_qualification::P03ResourceState;
use codex_hepta_authbus_p0_3_qualification::P03SchedulerResource;
use codex_hepta_contracts::Sha256Digest;

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

fn fence(owner_epoch: u64, generation: u64, label: &str) -> P03Fence {
    P03Fence {
        authority_epoch: 7,
        owner_epoch,
        generation,
        fencing_token_sha256: digest(label),
    }
}

fn quota() -> CanonicalQuotaVector {
    CanonicalQuotaVector::new(1, 2, 100, 1, 100, 200)
}

#[test]
fn exact_unknown_reconcile_remains_replayable_after_terminal_settlement() {
    let old_fence = fence(11, 21, "old");
    let current_fence = fence(12, 22, "current");
    let resource = P03SchedulerResource {
        resource_id: "resource-replay-history".to_string(),
        resource_sha256: digest("resource-replay-history"),
        fence: old_fence.clone(),
        quota: CanonicalQuotaLimits::known(CanonicalQuotaVector::new(
            10, 20, 1_000, 10, 1_000, 2_000,
        )),
        state: P03ResourceState::Available,
    };
    let mut scheduler = P03LocalScheduler::new(resource.clone()).expect("scheduler");
    let request = P03ReservationRequest {
        request_id: "request-replay-history".to_string(),
        command_id: "command-replay-history".to_string(),
        idempotency_key: "idempotency-replay-history".to_string(),
        payload_sha256: digest("payload-replay-history"),
        policy_sha256: digest("policy-replay-history"),
        resource_id: resource.resource_id,
        resource_sha256: resource.resource_sha256,
        estimate: quota(),
        safety_margin: CanonicalQuotaVector::default(),
        fence: old_fence.clone(),
        expected_revision: scheduler.revision(),
        created_at_ms: 1_000,
        expires_at_ms: 2_000,
    };
    let permit = match scheduler.reserve(request).expect("reserve") {
        P03AdmissionDisposition::Inserted(snapshot) => snapshot.permit,
        P03AdmissionDisposition::AlreadyPresent(_) => panic!("first reservation"),
    };
    scheduler
        .mark_dispatch_started(&permit.permit_id, &old_fence, scheduler.revision(), 1_050)
        .expect("dispatch");
    scheduler
        .rebind(current_fence.clone(), scheduler.revision(), 1_100)
        .expect("rebind");

    let unknown = P03OldPermitReconcileRequest {
        permit_id: permit.permit_id.clone(),
        old_fence: old_fence.clone(),
        current_fence: current_fence.clone(),
        provider_status_receipt_sha256: digest("provider-unknown"),
        owner_evidence_sha256: digest("owner-unknown"),
        expected_revision: scheduler.revision(),
        observed_at_ms: 1_200,
        outcome: P03ReconcileOutcome::Unknown,
    };
    let unknown_receipt = match scheduler
        .reconcile_old_permit(unknown.clone())
        .expect("unknown")
    {
        P03ReconcileDisposition::Applied(receipt) => receipt,
        P03ReconcileDisposition::AlreadyPresent(_) => panic!("first unknown"),
    };

    let terminal = P03OldPermitReconcileRequest {
        permit_id: permit.permit_id,
        old_fence,
        current_fence,
        provider_status_receipt_sha256: digest("provider-terminal"),
        owner_evidence_sha256: digest("owner-terminal"),
        expected_revision: scheduler.revision(),
        observed_at_ms: 1_300,
        outcome: P03ReconcileOutcome::VerifiedConsumed { actual: quota() },
    };
    scheduler
        .reconcile_old_permit(terminal)
        .expect("terminal settlement");

    assert_eq!(
        scheduler.reconcile_old_permit(unknown),
        Ok(P03ReconcileDisposition::AlreadyPresent(unknown_receipt))
    );
    scheduler.verify_invariants().expect("invariants");
}
