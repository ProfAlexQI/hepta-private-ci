#!/usr/bin/env python3
"""Apply deterministic P0.3 qualification hardening before lock/bootstrap CI.

This script is intentionally branch-local and one-shot. It changes only the
isolated P0.3 qualification crate, and the bootstrap workflow deletes it after
committing the qualified outputs.
"""

from pathlib import Path

ROOT = Path("codex-rs/hepta-authbus-p0-3-qualification")
SCHEDULER = ROOT / "src" / "scheduler.rs"
P03_TEST = ROOT / "tests" / "p0_3.rs"


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one match, found {count}")
    return text.replace(old, new, 1)


def harden_scheduler() -> None:
    text = SCHEDULER.read_text(encoding="utf-8")

    text = replace_once(
        text,
        """struct ActivePermit {
    permit: P03SchedulerPermit,
    state: P03ReservationState,
    last_unknown_evidence_sha256: Option<Sha256Digest>,
}
""",
        """struct ActivePermit {
    permit: P03SchedulerPermit,
    state: P03ReservationState,
    dispatch_marker_sha256: Option<Sha256Digest>,
    unknown_marker_sha256: Option<Sha256Digest>,
}
""",
        "active permit marker fields",
    )

    text = replace_once(
        text,
        """            ActivePermit {
                permit,
                state: P03ReservationState::ActiveReserved,
                last_unknown_evidence_sha256: None,
            },
""",
        """            ActivePermit {
                permit,
                state: P03ReservationState::ActiveReserved,
                dispatch_marker_sha256: None,
                unknown_marker_sha256: None,
            },
""",
        "active permit initialization",
    )

    text = replace_once(
        text,
        """        let active = self
            .active
            .get(permit_id)
            .cloned()
            .ok_or(P03SchedulerError::UnknownPermit)?;
        if active.state == P03ReservationState::DispatchStarted
            || active.state == P03ReservationState::OutcomeUnknown
        {
            return Ok((
                P03WriteDisposition::AlreadyPresent,
                self.snapshot_for_permit(&active.permit)?,
            ));
        }
        if active.state != P03ReservationState::ActiveReserved {
            return Err(P03SchedulerError::InvalidTransition);
        }
        self.validate_current_callback(&active.permit, current_fence, expected_revision)?;
        let next_revision = self.next_revision()?;
        let mut next = active;
        next.state = P03ReservationState::DispatchStarted;
""",
        """        let active = self
            .active
            .get(permit_id)
            .cloned()
            .ok_or(P03SchedulerError::UnknownPermit)?;
        let marker_sha256 = dispatch_marker_digest(permit_id, current_fence, observed_at_ms);
        if active.state == P03ReservationState::DispatchStarted
            || active.state == P03ReservationState::OutcomeUnknown
        {
            self.validate_replay_fence(&active.permit, current_fence)?;
            if active.dispatch_marker_sha256.as_ref() != Some(&marker_sha256) {
                return Err(P03SchedulerError::ObservationConflict);
            }
            return Ok((
                P03WriteDisposition::AlreadyPresent,
                self.snapshot_for_permit(&active.permit)?,
            ));
        }
        if active.state != P03ReservationState::ActiveReserved {
            return Err(P03SchedulerError::InvalidTransition);
        }
        self.validate_current_callback(&active.permit, current_fence, expected_revision)?;
        let next_revision = self.next_revision()?;
        let mut next = active;
        next.state = P03ReservationState::DispatchStarted;
        next.dispatch_marker_sha256 = Some(marker_sha256);
""",
        "dispatch replay binding",
    )

    text = replace_once(
        text,
        """        let active = self
            .active
            .get(permit_id)
            .cloned()
            .ok_or(P03SchedulerError::UnknownPermit)?;
        if active.state == P03ReservationState::OutcomeUnknown {
            if active.last_unknown_evidence_sha256.as_ref() == Some(&evidence_sha256) {
                return Ok((
                    P03WriteDisposition::AlreadyPresent,
                    self.snapshot_for_permit(&active.permit)?,
                ));
            }
            return Err(P03SchedulerError::ObservationConflict);
        }
        if active.state != P03ReservationState::DispatchStarted {
            return Err(P03SchedulerError::InvalidTransition);
        }
        self.validate_current_callback(&active.permit, current_fence, expected_revision)?;
        let next_revision = self.next_revision()?;
        let mut next = active;
        next.state = P03ReservationState::OutcomeUnknown;
        next.last_unknown_evidence_sha256 = Some(evidence_sha256);
""",
        """        let active = self
            .active
            .get(permit_id)
            .cloned()
            .ok_or(P03SchedulerError::UnknownPermit)?;
        let marker_sha256 = unknown_marker_digest(
            permit_id,
            current_fence,
            &evidence_sha256,
            observed_at_ms,
        );
        if active.state == P03ReservationState::OutcomeUnknown {
            self.validate_replay_fence(&active.permit, current_fence)?;
            if active.unknown_marker_sha256.as_ref() == Some(&marker_sha256) {
                return Ok((
                    P03WriteDisposition::AlreadyPresent,
                    self.snapshot_for_permit(&active.permit)?,
                ));
            }
            return Err(P03SchedulerError::ObservationConflict);
        }
        if active.state != P03ReservationState::DispatchStarted {
            return Err(P03SchedulerError::InvalidTransition);
        }
        self.validate_current_callback(&active.permit, current_fence, expected_revision)?;
        let next_revision = self.next_revision()?;
        let mut next = active;
        next.state = P03ReservationState::OutcomeUnknown;
        next.unknown_marker_sha256 = Some(marker_sha256);
""",
        "unknown replay binding",
    )

    text = replace_once(
        text,
        """        if let Some(terminal_request) = self
            .terminal_reconcile_by_permit
            .get(&request.permit_id)
        {
            if terminal_request == &request_key {
                let receipt = self
                    .reconcile_history
                    .get(&request_key)
                    .cloned()
                    .ok_or(P03SchedulerError::CorruptState)?;
                return Ok(P03ReconcileDisposition::AlreadyPresent(receipt));
            }
            return Err(P03SchedulerError::TerminalImmutable);
        }
        if let Some(receipt) = self.reconcile_history.get(&request_key) {
            return Ok(P03ReconcileDisposition::AlreadyPresent(receipt.clone()));
        }
""",
        """        if let Some(receipt) = self.reconcile_history.get(&request_key) {
            return Ok(P03ReconcileDisposition::AlreadyPresent(receipt.clone()));
        }
        if self
            .terminal_reconcile_by_permit
            .contains_key(&request.permit_id)
        {
            return Err(P03SchedulerError::TerminalImmutable);
        }
""",
        "reconcile history order",
    )

    text = replace_once(
        text,
        """            if !active.state.is_active() || active.permit.authority {
                return Err(P03SchedulerError::CorruptState);
            }
            recomputed_held = recomputed_held
""",
        """            if !active.state.is_active() || active.permit.authority {
                return Err(P03SchedulerError::CorruptState);
            }
            let marker_shape_valid = match active.state {
                P03ReservationState::ActiveReserved => {
                    active.dispatch_marker_sha256.is_none()
                        && active.unknown_marker_sha256.is_none()
                }
                P03ReservationState::DispatchStarted => {
                    active.dispatch_marker_sha256.is_some()
                        && active.unknown_marker_sha256.is_none()
                }
                P03ReservationState::OutcomeUnknown => {
                    active.dispatch_marker_sha256.is_some()
                }
                P03ReservationState::Completed
                | P03ReservationState::Released
                | P03ReservationState::ExpiredPreDispatch => false,
            };
            if !marker_shape_valid {
                return Err(P03SchedulerError::CorruptState);
            }
            recomputed_held = recomputed_held
""",
        "marker invariant",
    )

    text = replace_once(
        text,
        """        if !self.resource.quota.can_hold(
            CanonicalQuotaVector::default(),
            CanonicalQuotaVector::default(),
            self.used,
        ) {
""",
        """        if !self.resource.quota.can_hold(
            self.used,
            self.held,
            CanonicalQuotaVector::default(),
        ) {
""",
        "combined quota invariant",
    )

    text = replace_once(
        text,
        """    fn next_revision(&self) -> Result<u64, P03SchedulerError> {
""",
        """    fn validate_replay_fence(
        &self,
        permit: &P03SchedulerPermit,
        current_fence: &P03Fence,
    ) -> Result<(), P03SchedulerError> {
        if current_fence != &self.resource.fence
            || permit.fence != self.resource.fence
            || permit.resource_id != self.resource.resource_id
            || permit.resource_sha256 != self.resource.resource_sha256
        {
            return Err(P03SchedulerError::StaleFence);
        }
        Ok(())
    }

    fn next_revision(&self) -> Result<u64, P03SchedulerError> {
""",
        "replay fence validator",
    )

    text = replace_once(
        text,
        """fn validate_text(value: &str) -> Result<(), P03SchedulerError> {
""",
        """fn dispatch_marker_digest(
    permit_id: &str,
    current_fence: &P03Fence,
    observed_at_ms: u64,
) -> Sha256Digest {
    let mut bytes = Vec::new();
    push_text(&mut bytes, "hepta.authbus.b4.p0.3.dispatch-marker.v1");
    push_text(&mut bytes, permit_id);
    push_fence(&mut bytes, current_fence);
    push_u64(&mut bytes, observed_at_ms);
    Sha256Digest::for_bytes(&bytes)
}

fn unknown_marker_digest(
    permit_id: &str,
    current_fence: &P03Fence,
    evidence_sha256: &Sha256Digest,
    observed_at_ms: u64,
) -> Sha256Digest {
    let mut bytes = Vec::new();
    push_text(&mut bytes, "hepta.authbus.b4.p0.3.unknown-marker.v1");
    push_text(&mut bytes, permit_id);
    push_fence(&mut bytes, current_fence);
    push_digest(&mut bytes, evidence_sha256);
    push_u64(&mut bytes, observed_at_ms);
    Sha256Digest::for_bytes(&bytes)
}

fn validate_text(value: &str) -> Result<(), P03SchedulerError> {
""",
        "marker digest helpers",
    )

    SCHEDULER.write_text(text, encoding="utf-8")


def update_existing_marker_test() -> None:
    text = P03_TEST.read_text(encoding="utf-8")
    text = replace_once(text, "                1_101,\n", "                1_100,\n", "dispatch exact replay timestamp")
    text = replace_once(text, "                1_201,\n", "                1_200,\n", "unknown exact replay timestamp")
    P03_TEST.write_text(text, encoding="utf-8")


def write_reconcile_history_test() -> None:
    (ROOT / "tests" / "reconcile_history.rs").write_text(
        r'''use codex_hepta_authbus_p0_3_qualification::CanonicalQuotaLimits;
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
        .mark_dispatch_started(
            &permit.permit_id,
            &old_fence,
            scheduler.revision(),
            1_050,
        )
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
''',
        encoding="utf-8",
    )


def write_replay_fence_test() -> None:
    (ROOT / "tests" / "replay_fence.rs").write_text(
        r'''use codex_hepta_authbus_p0_3_qualification::CanonicalQuotaLimits;
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
        .mark_dispatch_started(
            &permit.permit_id,
            &original_fence,
            dispatch_revision,
            1_100,
        )
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
        scheduler.mark_dispatch_started(
            &permit.permit_id,
            &forged_fence,
            dispatch_revision,
            1_100,
        ),
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
''',
        encoding="utf-8",
    )


if __name__ == "__main__":
    harden_scheduler()
    update_existing_marker_test()
    write_reconcile_history_test()
    write_replay_fence_test()
