use codex_hepta_authbus_p0_3_qualification::CanonicalQuotaLimits;
use codex_hepta_authbus_p0_3_qualification::CanonicalQuotaVector;
use codex_hepta_authbus_p0_3_qualification::P03AdmissionDisposition;
use codex_hepta_authbus_p0_3_qualification::P03Fence;
use codex_hepta_authbus_p0_3_qualification::P03LocalScheduler;
use codex_hepta_authbus_p0_3_qualification::P03ReservationRequest;
use codex_hepta_authbus_p0_3_qualification::P03ResourceState;
use codex_hepta_authbus_p0_3_qualification::P03SchedulerError;
use codex_hepta_authbus_p0_3_qualification::P03SchedulerResource;
use codex_hepta_authbus_p0_3_qualification::P03WriteDisposition;
use codex_hepta_contracts::Sha256Digest;

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

fn fence(owner_epoch: u64, generation: u64, label: &str) -> P03Fence {
    P03Fence {
        authority_epoch: 5,
        owner_epoch,
        generation,
        fencing_token_sha256: digest(label),
    }
}

#[test]
fn marker_replay_is_bound_to_exact_fence_evidence_and_observed_time() {
    let original_fence = fence(7, 11, "original-fence");
    let resource = P03SchedulerResource {
        resource_id: "resource-replay-fence".to_string(),
        resource_sha256: digest("resource-replay-fence"),
        fence: original_fence.clone(),
        quota: CanonicalQuotaLimits::known(CanonicalQuotaVector::new(
            10, 20, 1_000, 10, 1_000, 2_000,
        )),
        state: P03ResourceState::Available,
    };
    let mut scheduler = P03LocalScheduler::new(resource.clone()).expect("scheduler");
    let request = P03ReservationRequest {
        request_id: "request-replay-fence".to_string(),
        command_id: "command-replay-fence".to_string(),
        idempotency_key: "idempotency-replay-fence".to_string(),
        payload_sha256: digest("payload-replay-fence"),
        policy_sha256: digest("policy-replay-fence"),
        resource_id: resource.resource_id,
        resource_sha256: resource.resource_sha256,
        estimate: CanonicalQuotaVector::new(1, 2, 100, 1, 100, 200),
        safety_margin: CanonicalQuotaVector::default(),
        fence: original_fence.clone(),
        expected_revision: scheduler.revision(),
        created_at_ms: 1_000,
        expires_at_ms: 2_000,
    };
    let permit = match scheduler.reserve(request).expect("reserve") {
        P03AdmissionDisposition::Inserted(snapshot) => snapshot.permit,
        P03AdmissionDisposition::AlreadyPresent(_) => panic!("first reservation"),
    };
    let dispatch_revision = scheduler.revision();
    let (_, dispatch_snapshot) = scheduler
        .mark_dispatch_started(&permit.permit_id, &original_fence, dispatch_revision, 1_100)
        .expect("dispatch");
    assert_eq!(
        scheduler.mark_dispatch_started(
            &permit.permit_id,
            &original_fence,
            dispatch_revision,
            1_100,
        ),
        Ok((P03WriteDisposition::AlreadyPresent, dispatch_snapshot))
    );

    let before_changed_time = scheduler.clone();
    assert_eq!(
        scheduler.mark_dispatch_started(
            &permit.permit_id,
            &original_fence,
            dispatch_revision,
            1_101,
        ),
        Err(P03SchedulerError::ObservationConflict)
    );
    assert_eq!(scheduler, before_changed_time);

    let forged_fence = fence(7, 11, "forged-fence");
    assert_eq!(
        scheduler
            .mark_dispatch_started(&permit.permit_id, &forged_fence, dispatch_revision, 1_100,),
        Err(P03SchedulerError::StaleFence)
    );

    let unknown_revision = scheduler.revision();
    let evidence = digest("unknown-evidence");
    let (_, unknown_snapshot) = scheduler
        .mark_outcome_unknown(
            &permit.permit_id,
            &original_fence,
            unknown_revision,
            evidence.clone(),
            1_200,
        )
        .expect("unknown");
    assert_eq!(
        scheduler.mark_outcome_unknown(
            &permit.permit_id,
            &original_fence,
            unknown_revision,
            evidence.clone(),
            1_200,
        ),
        Ok((P03WriteDisposition::AlreadyPresent, unknown_snapshot))
    );
    let before_unknown_conflict = scheduler.clone();
    assert_eq!(
        scheduler.mark_outcome_unknown(
            &permit.permit_id,
            &original_fence,
            unknown_revision,
            evidence.clone(),
            1_201,
        ),
        Err(P03SchedulerError::ObservationConflict)
    );
    assert_eq!(scheduler, before_unknown_conflict);
    assert_eq!(
        scheduler.mark_outcome_unknown(
            &permit.permit_id,
            &forged_fence,
            unknown_revision,
            evidence,
            1_200,
        ),
        Err(P03SchedulerError::StaleFence)
    );

    let current_fence = fence(8, 12, "current-fence");
    scheduler
        .rebind(current_fence, scheduler.revision(), 1_300)
        .expect("rebind");
    assert_eq!(
        scheduler.mark_dispatch_started(
            &permit.permit_id,
            &original_fence,
            dispatch_revision,
            1_100,
        ),
        Err(P03SchedulerError::StaleFence)
    );
    scheduler.verify_invariants().expect("invariants");
}
