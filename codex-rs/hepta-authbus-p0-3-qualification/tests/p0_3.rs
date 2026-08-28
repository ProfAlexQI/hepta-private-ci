use codex_hepta_authbus_p0_3_qualification::AUTHBUS_B4_P0_3_AUTHORITY;
use codex_hepta_authbus_p0_3_qualification::AUTHBUS_B4_P0_3_EFFECT_AUTHORITY;
use codex_hepta_authbus_p0_3_qualification::AUTHBUS_B4_P0_3_EXECUTE_ALLOWED;
use codex_hepta_authbus_p0_3_qualification::AUTHBUS_B4_P0_3_G5_ALLOWED;
use codex_hepta_authbus_p0_3_qualification::AUTHBUS_B4_P0_3_OPERATOR_ACCEPTANCE;
use codex_hepta_authbus_p0_3_qualification::AUTHBUS_B4_P0_3_PRODUCTION_CALLER;
use codex_hepta_authbus_p0_3_qualification::AUTHBUS_B4_P0_3_PRODUCTION_WRITER;
use codex_hepta_authbus_p0_3_qualification::AUTHBUS_B4_P0_3_PROMOTION;
use codex_hepta_authbus_p0_3_qualification::AUTHBUS_B4_P0_3_QUALIFICATION_ONLY;
use codex_hepta_authbus_p0_3_qualification::CanonicalQuotaLimits;
use codex_hepta_authbus_p0_3_qualification::CanonicalQuotaVector;
use codex_hepta_authbus_p0_3_qualification::P03AdmissionDisposition;
use codex_hepta_authbus_p0_3_qualification::P03Fence;
use codex_hepta_authbus_p0_3_qualification::P03LocalScheduler;
use codex_hepta_authbus_p0_3_qualification::P03OldPermitReconcileRequest;
use codex_hepta_authbus_p0_3_qualification::P03ReconcileDisposition;
use codex_hepta_authbus_p0_3_qualification::P03ReconcileOutcome;
use codex_hepta_authbus_p0_3_qualification::P03ReconcileResolution;
use codex_hepta_authbus_p0_3_qualification::P03ReservationRequest;
use codex_hepta_authbus_p0_3_qualification::P03ReservationState;
use codex_hepta_authbus_p0_3_qualification::P03ResourceState;
use codex_hepta_authbus_p0_3_qualification::P03SchedulerError;
use codex_hepta_authbus_p0_3_qualification::P03SchedulerResource;
use codex_hepta_authbus_p0_3_qualification::P03WriteDisposition;
use codex_hepta_contracts::QuotaVector;
use codex_hepta_contracts::Sha256Digest;
use pretty_assertions::assert_eq;

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

fn capacity() -> CanonicalQuotaVector {
    CanonicalQuotaVector::new(100, 100, 10_000, 8, 100_000, 100_000)
}

fn resource(quota: CanonicalQuotaLimits) -> P03SchedulerResource {
    P03SchedulerResource {
        resource_id: "resource-a".to_string(),
        resource_sha256: digest("resource-a"),
        fence: fence(7, 11, "fence-old"),
        quota,
        state: P03ResourceState::Available,
    }
}

fn request(
    scheduler: &P03LocalScheduler,
    label: &str,
    created_at_ms: u64,
    expires_at_ms: u64,
) -> P03ReservationRequest {
    P03ReservationRequest {
        request_id: format!("request-{label}"),
        command_id: format!("command-{label}"),
        idempotency_key: format!("idempotency-{label}"),
        payload_sha256: digest(&format!("payload-{label}")),
        policy_sha256: digest("policy"),
        resource_id: scheduler.resource().resource_id.clone(),
        resource_sha256: scheduler.resource().resource_sha256.clone(),
        estimate: CanonicalQuotaVector::new(1, 1, 100, 1, 100, 256),
        safety_margin: CanonicalQuotaVector::new(0, 0, 20, 0, 20, 64),
        fence: scheduler.resource().fence.clone(),
        expected_revision: scheduler.revision(),
        created_at_ms,
        expires_at_ms,
    }
}

fn inserted_permit(
    disposition: P03AdmissionDisposition,
) -> codex_hepta_authbus_p0_3_qualification::P03SchedulerPermit {
    match disposition {
        P03AdmissionDisposition::Inserted(snapshot) => snapshot.permit,
        P03AdmissionDisposition::AlreadyPresent(_) => panic!("expected inserted permit"),
    }
}

fn prepare_old_fence_unknown(
    label: &str,
) -> (
    P03LocalScheduler,
    codex_hepta_authbus_p0_3_qualification::P03SchedulerPermit,
    P03Fence,
    P03Fence,
) {
    let mut scheduler =
        P03LocalScheduler::new(resource(CanonicalQuotaLimits::known(capacity())))
            .expect("scheduler");
    let permit = inserted_permit(
        scheduler
            .reserve(request(&scheduler, label, 1_000, 10_000))
            .expect("reserve"),
    );
    let old_fence = permit.fence.clone();
    let dispatch_revision = scheduler.revision();
    scheduler
        .mark_dispatch_started(
            &permit.permit_id,
            &old_fence,
            dispatch_revision,
            1_100,
        )
        .expect("dispatch started");
    let unknown_revision = scheduler.revision();
    scheduler
        .mark_outcome_unknown(
            &permit.permit_id,
            &old_fence,
            unknown_revision,
            digest(&format!("unknown-{label}")),
            1_200,
        )
        .expect("unknown");
    let current_fence = fence(8, 12, &format!("fence-current-{label}"));
    let rebind_revision = scheduler.revision();
    scheduler
        .rebind(current_fence.clone(), rebind_revision, 1_300)
        .expect("rebind");
    (scheduler, permit, old_fence, current_fence)
}

#[test]
fn authority_posture_is_statically_closed() {
    assert!(AUTHBUS_B4_P0_3_QUALIFICATION_ONLY);
    assert!(!AUTHBUS_B4_P0_3_AUTHORITY);
    assert!(!AUTHBUS_B4_P0_3_EFFECT_AUTHORITY);
    assert!(!AUTHBUS_B4_P0_3_PRODUCTION_CALLER);
    assert!(!AUTHBUS_B4_P0_3_PRODUCTION_WRITER);
    assert!(!AUTHBUS_B4_P0_3_OPERATOR_ACCEPTANCE);
    assert!(!AUTHBUS_B4_P0_3_PROMOTION);
    assert!(!AUTHBUS_B4_P0_3_G5_ALLOWED);
    assert!(!AUTHBUS_B4_P0_3_EXECUTE_ALLOWED);
}

#[test]
fn six_dimensional_quota_has_stable_legacy_projection() {
    let legacy = QuotaVector::new(2, 300, 1, 400, 512);
    let canonical = CanonicalQuotaVector::from_legacy_b4(legacy);
    assert_eq!(canonical.request_count, 1);
    assert_eq!(canonical.to_legacy_b4(), legacy);
    assert_eq!(canonical.digest(), canonical.digest());
    assert_ne!(
        canonical.digest(),
        CanonicalQuotaVector::new(2, 2, 300, 1, 400, 512).digest()
    );
}

#[test]
fn unknown_quota_denies_without_mutation() {
    let mut scheduler = P03LocalScheduler::new(resource(
        CanonicalQuotaLimits::unknown_request_count(capacity()),
    ))
    .expect("scheduler");
    let before = scheduler.clone();
    assert_eq!(
        scheduler.reserve(request(&scheduler, "unknown", 1_000, 2_000)),
        Err(P03SchedulerError::UnknownQuota)
    );
    assert_eq!(scheduler, before);
}

#[test]
fn exact_idempotency_replays_original_permit_and_changed_payload_conflicts() {
    let mut scheduler =
        P03LocalScheduler::new(resource(CanonicalQuotaLimits::known(capacity())))
            .expect("scheduler");
    let original = request(&scheduler, "replay", 1_000, 2_000);
    let first = scheduler.reserve(original.clone()).expect("reserve");
    let snapshot = match first {
        P03AdmissionDisposition::Inserted(snapshot) => snapshot,
        P03AdmissionDisposition::AlreadyPresent(_) => panic!("first insert"),
    };
    let revision_after_first = scheduler.revision();
    let held_after_first = scheduler.held();

    assert_eq!(
        scheduler.reserve(original.clone()),
        Ok(P03AdmissionDisposition::AlreadyPresent(snapshot.clone()))
    );
    assert_eq!(scheduler.revision(), revision_after_first);
    assert_eq!(scheduler.held(), held_after_first);
    assert_eq!(scheduler.active_permit_count(), 1);

    let mut changed_payload = original.clone();
    changed_payload.payload_sha256 = digest("changed-payload");
    assert_eq!(
        scheduler.reserve(changed_payload),
        Err(P03SchedulerError::IdempotencyConflict)
    );

    let mut changed_binding = original;
    changed_binding.command_id = "changed-command".to_string();
    assert_eq!(
        scheduler.reserve(changed_binding),
        Err(P03SchedulerError::BindingConflict)
    );
    assert_eq!(scheduler.revision(), revision_after_first);
    assert_eq!(scheduler.held(), held_after_first);
    scheduler.verify_invariants().expect("invariants");
}

#[test]
fn old_fence_consumed_reconcile_completes_once_and_replays_receipt() {
    let (mut scheduler, permit, old_fence, current_fence) =
        prepare_old_fence_unknown("consumed");
    let held_before = scheduler.held();
    let request = P03OldPermitReconcileRequest {
        permit_id: permit.permit_id.clone(),
        old_fence,
        current_fence,
        provider_status_receipt_sha256: digest("provider-consumed"),
        owner_evidence_sha256: digest("owner-evidence"),
        expected_revision: scheduler.revision(),
        observed_at_ms: 1_400,
        outcome: P03ReconcileOutcome::VerifiedConsumed {
            actual: CanonicalQuotaVector::new(1, 1, 70, 1, 70, 128),
        },
    };
    let applied = scheduler
        .reconcile_old_permit(request.clone())
        .expect("reconcile");
    let receipt = match applied {
        P03ReconcileDisposition::Applied(receipt) => receipt,
        P03ReconcileDisposition::AlreadyPresent(_) => panic!("first reconcile"),
    };
    assert_eq!(receipt.resolution, P03ReconcileResolution::Consumed);
    assert!(!receipt.authority);
    assert_eq!(scheduler.held(), CanonicalQuotaVector::default());
    assert_ne!(held_before, scheduler.held());
    assert_eq!(scheduler.used().request_count, 1);
    assert_eq!(scheduler.used().concurrency, 0);
    assert_eq!(scheduler.active_permit_count(), 0);
    assert_eq!(
        scheduler
            .reservation_by_idempotency(&permit.idempotency_key)
            .expect("reservation")
            .state,
        P03ReservationState::Completed
    );

    assert_eq!(
        scheduler.reconcile_old_permit(request.clone()),
        Ok(P03ReconcileDisposition::AlreadyPresent(receipt))
    );
    let mut changed = request;
    changed.provider_status_receipt_sha256 = digest("changed-terminal-evidence");
    changed.expected_revision = scheduler.revision();
    assert_eq!(
        scheduler.reconcile_old_permit(changed),
        Err(P03SchedulerError::TerminalImmutable)
    );
    scheduler.verify_invariants().expect("invariants");
}

#[test]
fn verified_no_effect_releases_old_permit_without_usage() {
    let (mut scheduler, permit, old_fence, current_fence) =
        prepare_old_fence_unknown("no-effect");
    let request = P03OldPermitReconcileRequest {
        permit_id: permit.permit_id.clone(),
        old_fence,
        current_fence,
        provider_status_receipt_sha256: digest("provider-no-effect"),
        owner_evidence_sha256: digest("owner-no-effect"),
        expected_revision: scheduler.revision(),
        observed_at_ms: 1_400,
        outcome: P03ReconcileOutcome::VerifiedNoEffect,
    };
    let result = scheduler
        .reconcile_old_permit(request)
        .expect("release reconcile");
    assert!(matches!(
        result,
        P03ReconcileDisposition::Applied(
            codex_hepta_authbus_p0_3_qualification::P03OldPermitReconcileReceipt {
                resolution: P03ReconcileResolution::NoEffect,
                ..
            }
        )
    ));
    assert_eq!(scheduler.held(), CanonicalQuotaVector::default());
    assert_eq!(scheduler.used(), CanonicalQuotaVector::default());
    assert_eq!(scheduler.active_permit_count(), 0);
    assert_eq!(
        scheduler
            .reservation_by_idempotency(&permit.idempotency_key)
            .expect("reservation")
            .state,
        P03ReservationState::Released
    );
    scheduler.verify_invariants().expect("invariants");
}

#[test]
fn unknown_reconcile_keeps_hold_and_later_terminal_evidence_can_settle() {
    let (mut scheduler, permit, old_fence, current_fence) =
        prepare_old_fence_unknown("held");
    let held_before = scheduler.held();
    let unknown = P03OldPermitReconcileRequest {
        permit_id: permit.permit_id.clone(),
        old_fence: old_fence.clone(),
        current_fence: current_fence.clone(),
        provider_status_receipt_sha256: digest("provider-still-unknown"),
        owner_evidence_sha256: digest("owner-hold"),
        expected_revision: scheduler.revision(),
        observed_at_ms: 1_400,
        outcome: P03ReconcileOutcome::Unknown,
    };
    let applied = scheduler
        .reconcile_old_permit(unknown.clone())
        .expect("unknown reconcile");
    let receipt = match applied {
        P03ReconcileDisposition::Applied(receipt) => receipt,
        P03ReconcileDisposition::AlreadyPresent(_) => panic!("first unknown"),
    };
    assert_eq!(receipt.resolution, P03ReconcileResolution::HeldUnknown);
    assert_eq!(scheduler.held(), held_before);
    assert_eq!(scheduler.active_permit_count(), 1);
    assert_eq!(
        scheduler.reconcile_old_permit(unknown),
        Ok(P03ReconcileDisposition::AlreadyPresent(receipt))
    );

    let terminal = P03OldPermitReconcileRequest {
        permit_id: permit.permit_id,
        old_fence,
        current_fence,
        provider_status_receipt_sha256: digest("provider-later-consumed"),
        owner_evidence_sha256: digest("owner-later-consumed"),
        expected_revision: scheduler.revision(),
        observed_at_ms: 1_500,
        outcome: P03ReconcileOutcome::VerifiedConsumed {
            actual: CanonicalQuotaVector::new(1, 1, 80, 1, 80, 128),
        },
    };
    assert!(matches!(
        scheduler.reconcile_old_permit(terminal),
        Ok(P03ReconcileDisposition::Applied(
            codex_hepta_authbus_p0_3_qualification::P03OldPermitReconcileReceipt {
                resolution: P03ReconcileResolution::Consumed,
                ..
            }
        ))
    ));
    assert_eq!(scheduler.held(), CanonicalQuotaVector::default());
    assert_eq!(scheduler.active_permit_count(), 0);
    scheduler.verify_invariants().expect("invariants");
}

#[test]
fn stale_or_forged_reconcile_evidence_is_non_mutating() {
    let (mut scheduler, permit, old_fence, current_fence) =
        prepare_old_fence_unknown("forged");
    let before = scheduler.clone();
    let forged = P03OldPermitReconcileRequest {
        permit_id: permit.permit_id,
        old_fence: fence(99, 99, "forged-old"),
        current_fence,
        provider_status_receipt_sha256: digest("provider-forged"),
        owner_evidence_sha256: digest("owner-forged"),
        expected_revision: scheduler.revision(),
        observed_at_ms: 1_400,
        outcome: P03ReconcileOutcome::VerifiedNoEffect,
    };
    assert_eq!(
        scheduler.reconcile_old_permit(forged),
        Err(P03SchedulerError::StaleFence)
    );
    assert_eq!(scheduler, before);

    let stale_revision = P03OldPermitReconcileRequest {
        permit_id: before
            .reservation_by_idempotency("idempotency-forged")
            .expect("reservation")
            .permit
            .permit_id
            .clone(),
        old_fence,
        current_fence: before.resource().fence.clone(),
        provider_status_receipt_sha256: digest("provider-valid"),
        owner_evidence_sha256: digest("owner-valid"),
        expected_revision: before.revision() - 1,
        observed_at_ms: 1_500,
        outcome: P03ReconcileOutcome::VerifiedNoEffect,
    };
    assert_eq!(
        scheduler.reconcile_old_permit(stale_revision),
        Err(P03SchedulerError::StaleRevision)
    );
    assert_eq!(scheduler, before);
}

#[test]
fn expiry_releases_only_pre_dispatch_and_never_unknown_effects() {
    let mut scheduler =
        P03LocalScheduler::new(resource(CanonicalQuotaLimits::known(capacity())))
            .expect("scheduler");
    let pre_dispatch = inserted_permit(
        scheduler
            .reserve(request(&scheduler, "pre", 1_000, 1_100))
            .expect("reserve pre"),
    );
    let post_dispatch = inserted_permit(
        scheduler
            .reserve(request(&scheduler, "post", 1_000, 1_100))
            .expect("reserve post"),
    );
    let current_fence = scheduler.resource().fence.clone();
    let dispatch_revision = scheduler.revision();
    scheduler
        .mark_dispatch_started(
            &post_dispatch.permit_id,
            &current_fence,
            dispatch_revision,
            1_050,
        )
        .expect("dispatch post");
    let unknown_revision = scheduler.revision();
    scheduler
        .mark_outcome_unknown(
            &post_dispatch.permit_id,
            &current_fence,
            unknown_revision,
            digest("post-unknown"),
            1_060,
        )
        .expect("unknown post");

    let expiry_revision = scheduler.revision();
    let report = scheduler
        .expire_active_permits(1_200, expiry_revision)
        .expect("expiry scan");
    assert_eq!(report.released_pre_dispatch.len(), 1);
    assert_eq!(report.held_for_reconcile.len(), 1);
    assert_eq!(
        report.released_pre_dispatch[0].permit.permit_id,
        pre_dispatch.permit_id
    );
    assert_eq!(
        report.released_pre_dispatch[0].state,
        P03ReservationState::ExpiredPreDispatch
    );
    assert_eq!(
        report.held_for_reconcile[0].permit.permit_id,
        post_dispatch.permit_id
    );
    assert_eq!(
        report.held_for_reconcile[0].state,
        P03ReservationState::OutcomeUnknown
    );
    assert_eq!(scheduler.active_permit_count(), 1);
    assert_eq!(scheduler.held(), post_dispatch.reserved);
    scheduler.verify_invariants().expect("invariants");
}

#[test]
fn dispatch_and_unknown_markers_are_exactly_idempotent() {
    let mut scheduler =
        P03LocalScheduler::new(resource(CanonicalQuotaLimits::known(capacity())))
            .expect("scheduler");
    let permit = inserted_permit(
        scheduler
            .reserve(request(&scheduler, "markers", 1_000, 2_000))
            .expect("reserve"),
    );
    let current_fence = scheduler.resource().fence.clone();
    let dispatch_revision = scheduler.revision();
    let (_, dispatch_snapshot) = scheduler
        .mark_dispatch_started(
            &permit.permit_id,
            &current_fence,
            dispatch_revision,
            1_100,
        )
        .expect("dispatch");
    assert_eq!(
        scheduler
            .mark_dispatch_started(
                &permit.permit_id,
                &current_fence,
                dispatch_revision,
                1_101,
            )
            .expect("dispatch replay"),
        (P03WriteDisposition::AlreadyPresent, dispatch_snapshot)
    );

    let unknown_revision = scheduler.revision();
    let evidence = digest("unknown-evidence");
    let (_, unknown_snapshot) = scheduler
        .mark_outcome_unknown(
            &permit.permit_id,
            &current_fence,
            unknown_revision,
            evidence.clone(),
            1_200,
        )
        .expect("unknown");
    assert_eq!(
        scheduler
            .mark_outcome_unknown(
                &permit.permit_id,
                &current_fence,
                unknown_revision,
                evidence,
                1_201,
            )
            .expect("unknown replay"),
        (P03WriteDisposition::AlreadyPresent, unknown_snapshot)
    );
    assert_eq!(
        scheduler.mark_outcome_unknown(
            &permit.permit_id,
            &current_fence,
            scheduler.revision(),
            digest("changed-unknown"),
            1_202,
        ),
        Err(P03SchedulerError::ObservationConflict)
    );
}
