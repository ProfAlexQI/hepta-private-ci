#![cfg(feature = "p1-3-qualification")]

use codex_hepta_authbus_p1_3_qualification::QuotaWindowBindings;
use codex_hepta_authbus_p1_3_qualification::QuotaWindowKey;
use codex_hepta_authbus_p1_3_qualification::QuotaWindowKind;
use codex_hepta_authbus_p1_3_qualification::SemanticAdmissionDisposition;
use codex_hepta_authbus_p1_3_qualification::SemanticQuotaError;
use codex_hepta_authbus_p1_3_qualification::SemanticReservationRequest;
use codex_hepta_authbus_p1_3_qualification::SemanticReservationState;
use codex_hepta_authbus_p1_3_qualification::WindowedQuotaLedger;
use codex_hepta_authbus_p1_3_qualification::authority_posture_is_closed;
use codex_hepta_authbus_p1_3_qualification::verify_transition_chain;
use codex_hepta_contracts::CanonicalQuotaLimits;
use codex_hepta_contracts::CanonicalQuotaVector;
use codex_hepta_contracts::QuotaDimension;
use codex_hepta_contracts::Sha256Digest;

fn limits(
    request_count: u64,
    rpm: u64,
    tpm: u64,
    concurrency: u64,
    day_budget: u64,
    context: u64,
) -> CanonicalQuotaLimits {
    CanonicalQuotaLimits::known(CanonicalQuotaVector::new(
        request_count,
        rpm,
        tpm,
        concurrency,
        day_budget,
        context,
    ))
}

fn windows(domain: &str, observed_at_s: u64, policy_revision: u64) -> QuotaWindowBindings {
    let minute = observed_at_s / 60 * 60;
    let day = observed_at_s / 86_400 * 86_400;
    QuotaWindowBindings {
        rpm: QuotaWindowKey {
            quota_domain: domain.to_string(),
            dimension: QuotaDimension::Rpm,
            kind: QuotaWindowKind::MinuteUtc,
            starts_at_s: minute,
            ends_at_s: minute + 60,
            policy_revision,
        },
        tpm: QuotaWindowKey {
            quota_domain: domain.to_string(),
            dimension: QuotaDimension::Tpm,
            kind: QuotaWindowKind::MinuteUtc,
            starts_at_s: minute,
            ends_at_s: minute + 60,
            policy_revision,
        },
        day_budget: QuotaWindowKey {
            quota_domain: domain.to_string(),
            dimension: QuotaDimension::DayBudget,
            kind: QuotaWindowKind::DayUtc,
            starts_at_s: day,
            ends_at_s: day + 86_400,
            policy_revision,
        },
    }
}

fn request(
    ledger: &WindowedQuotaLedger,
    id: &str,
    observed_at_s: u64,
    estimated: CanonicalQuotaVector,
) -> SemanticReservationRequest {
    let domain = "provider/openbao/profile/default";
    SemanticReservationRequest {
        reservation_id: id.to_string(),
        idempotency_key: format!("idem-{id}"),
        quota_domain: domain.to_string(),
        payload_sha256: Sha256Digest::for_bytes(format!("payload-{id}").as_bytes()),
        policy_sha256: Sha256Digest::for_bytes(b"policy-v12"),
        policy_revision: 12,
        estimated,
        safety_margin: CanonicalQuotaVector::default(),
        windows: windows(domain, observed_at_s, 12),
        issued_at_s: observed_at_s,
        expires_at_s: observed_at_s + 30,
        expected_revision: ledger.revision(),
    }
}

fn inserted(
    disposition: SemanticAdmissionDisposition,
) -> codex_hepta_authbus_p1_3_qualification::SemanticReservationRecord {
    match disposition {
        SemanticAdmissionDisposition::Inserted(record) => record,
        SemanticAdmissionDisposition::AlreadyPresent(_) => panic!("expected a new reservation"),
    }
}

fn one_request(context: u64, day_budget: u64) -> CanonicalQuotaVector {
    CanonicalQuotaVector::new(1, 1, 100, 1, day_budget, context)
}

#[test]
fn rpm_and_tpm_are_isolated_by_exact_minute_window() {
    let mut ledger = WindowedQuotaLedger::open(limits(100, 1, 100, 4, 1_000, 8_192))
        .expect("known limits");
    let first = inserted(
        ledger
            .reserve(request(&ledger, "r1", 120, one_request(1_024, 10)))
            .expect("first minute admission"),
    );
    ledger
        .mark_dispatch_attempted("r1", first.revision)
        .expect("dispatch transition");
    let completed = ledger
        .finalize_consumed(
            "r1",
            ledger.revision(),
            CanonicalQuotaVector::new(1, 1, 100, 0, 10, 900),
        )
        .expect("terminal usage");
    assert_eq!(completed.state, SemanticReservationState::Completed);

    let same_minute = request(&ledger, "r2", 150, one_request(1_024, 10));
    assert_eq!(
        ledger.reserve(same_minute),
        Err(SemanticQuotaError::QuotaExceeded)
    );

    let next_minute = inserted(
        ledger
            .reserve(request(&ledger, "r3", 180, one_request(1_024, 10)))
            .expect("new minute has independent counters"),
    );
    assert_eq!(next_minute.state, SemanticReservationState::Held);
    let first_minute_key = windows("provider/openbao/profile/default", 120, 12).rpm;
    let next_minute_key = windows("provider/openbao/profile/default", 180, 12).rpm;
    assert_eq!(ledger.window_usage(&first_minute_key), (1, 0));
    assert_eq!(ledger.window_usage(&next_minute_key), (0, 1));
    ledger.verify_invariants().expect("consistent ledger");
}

#[test]
fn daily_budget_rolls_only_at_exact_utc_day_boundary() {
    let mut ledger = WindowedQuotaLedger::open(limits(100, 100, 10_000, 4, 10, 8_192))
        .expect("known limits");
    let first = inserted(
        ledger
            .reserve(request(&ledger, "day-1", 86_410, one_request(512, 10)))
            .expect("first day admission"),
    );
    ledger
        .mark_dispatch_attempted("day-1", first.revision)
        .expect("dispatch transition");
    ledger
        .finalize_consumed(
            "day-1",
            ledger.revision(),
            CanonicalQuotaVector::new(1, 1, 100, 0, 10, 500),
        )
        .expect("day budget consumed");

    assert_eq!(
        ledger.reserve(request(
            &ledger,
            "same-day",
            86_500,
            one_request(512, 1),
        )),
        Err(SemanticQuotaError::QuotaExceeded)
    );
    inserted(
        ledger
            .reserve(request(
                &ledger,
                "next-day",
                172_810,
                one_request(512, 10),
            ))
            .expect("next UTC day has an independent daily counter"),
    );
    ledger.verify_invariants().expect("consistent ledger");
}

#[test]
fn max_context_is_per_request_and_never_aggregate_spend() {
    let mut ledger = WindowedQuotaLedger::open(limits(100, 100, 100_000, 2, 10_000, 4_096))
        .expect("known limits");
    let first = inserted(
        ledger
            .reserve(request(&ledger, "ctx-1", 600, one_request(4_096, 1)))
            .expect("first maximum-context request"),
    );
    let second = inserted(
        ledger
            .reserve(request(&ledger, "ctx-2", 601, one_request(4_096, 1)))
            .expect("second maximum-context request is independently bounded"),
    );
    assert_eq!(ledger.active_concurrency(), 2);

    ledger
        .mark_dispatch_attempted("ctx-1", ledger.revision())
        .expect("dispatch first");
    ledger
        .finalize_consumed(
            "ctx-1",
            ledger.revision(),
            CanonicalQuotaVector::new(1, 1, 90, 0, 1, 4_096),
        )
        .expect("complete first");
    assert_eq!(ledger.active_concurrency(), 1);
    ledger
        .finalize_no_effect("ctx-2", ledger.revision())
        .expect("release second");
    assert_eq!(ledger.active_concurrency(), 0);

    let third = inserted(
        ledger
            .reserve(request(&ledger, "ctx-3", 602, one_request(4_096, 1)))
            .expect("prior context did not consume a global context budget"),
    );
    assert_eq!(third.held.context, 4_096);
    assert_eq!(first.held.context, second.held.context);
    ledger.verify_invariants().expect("consistent ledger");
}

#[test]
fn context_above_per_request_limit_fails_before_any_counter_changes() {
    let mut ledger = WindowedQuotaLedger::open(limits(100, 100, 100_000, 4, 10_000, 4_096))
        .expect("known limits");
    let revision = ledger.revision();
    assert_eq!(
        ledger.reserve(request(&ledger, "too-large", 700, one_request(4_097, 1))),
        Err(SemanticQuotaError::QuotaExceeded)
    );
    assert_eq!(ledger.revision(), revision);
    assert_eq!(ledger.active_concurrency(), 0);
}

#[test]
fn stale_or_wrong_window_binding_fails_closed() {
    let mut ledger = WindowedQuotaLedger::open(limits(100, 100, 100_000, 4, 10_000, 4_096))
        .expect("known limits");
    let mut stale = request(&ledger, "stale-window", 720, one_request(512, 1));
    stale.windows.rpm.starts_at_s = 660;
    stale.windows.rpm.ends_at_s = 720;
    assert_eq!(
        ledger.reserve(stale),
        Err(SemanticQuotaError::InvalidWindow)
    );

    let mut wrong_kind = request(&ledger, "wrong-kind", 720, one_request(512, 1));
    wrong_kind.windows.tpm.kind = QuotaWindowKind::DayUtc;
    assert_eq!(
        ledger.reserve(wrong_kind),
        Err(SemanticQuotaError::InvalidWindow)
    );

    let mut wrong_policy = request(&ledger, "wrong-policy", 720, one_request(512, 1));
    wrong_policy.windows.day_budget.policy_revision = 11;
    assert_eq!(
        ledger.reserve(wrong_policy),
        Err(SemanticQuotaError::InvalidWindow)
    );
}

#[test]
fn stale_revision_and_changed_idempotency_binding_fail_closed() {
    let mut ledger = WindowedQuotaLedger::open(limits(100, 100, 100_000, 4, 10_000, 4_096))
        .expect("known limits");
    let exact_request = request(&ledger, "idempotent", 800, one_request(512, 1));
    let first = inserted(
        ledger
            .reserve(exact_request.clone())
            .expect("initial admission"),
    );
    let revision = ledger.revision();
    match ledger.reserve(exact_request).expect("exact replay") {
        SemanticAdmissionDisposition::AlreadyPresent(replayed) => assert_eq!(replayed, first),
        SemanticAdmissionDisposition::Inserted(_) => panic!("exact replay inserted twice"),
    }
    assert_eq!(ledger.revision(), revision);

    let mut changed = request(&ledger, "idempotent", 800, one_request(512, 1));
    changed.idempotency_key = "idem-idempotent".to_string();
    changed.expected_revision = first.revision;
    changed.payload_sha256 = Sha256Digest::for_bytes(b"changed-payload");
    assert_eq!(
        ledger.reserve(changed),
        Err(SemanticQuotaError::BindingConflict)
    );

    let mut stale_revision = request(&ledger, "stale-revision", 801, one_request(512, 1));
    stale_revision.expected_revision = 1;
    assert_eq!(
        ledger.reserve(stale_revision),
        Err(SemanticQuotaError::StaleRevision)
    );
}

#[test]
fn completed_transition_conserves_hold_consumed_and_remaining() {
    let mut ledger = WindowedQuotaLedger::open(limits(100, 100, 100_000, 4, 10_000, 4_096))
        .expect("known limits");
    let mut req = request(&ledger, "conservation", 900, one_request(1_024, 10));
    req.safety_margin = CanonicalQuotaVector::new(0, 0, 20, 0, 2, 100);
    let held = inserted(ledger.reserve(req).expect("admission"));
    ledger
        .mark_dispatch_attempted("conservation", held.revision)
        .expect("dispatch transition");
    let completed = ledger
        .finalize_consumed(
            "conservation",
            ledger.revision(),
            CanonicalQuotaVector::new(1, 1, 90, 0, 8, 900),
        )
        .expect("terminal usage");
    completed
        .validate_semantics()
        .expect("valid conservation projection");
    assert_eq!(
        completed
            .consumed
            .checked_add(completed.remaining)
            .expect("bounded addition"),
        completed.held
    );

    let mut tampered = completed;
    tampered.remaining.context -= 1;
    assert_eq!(
        tampered.validate_semantics(),
        Err(SemanticQuotaError::ConservationViolation)
    );
}

#[test]
fn invalid_state_transitions_and_post_dispatch_expiry_are_rejected() {
    let mut ledger = WindowedQuotaLedger::open(limits(100, 100, 100_000, 4, 10_000, 4_096))
        .expect("known limits");
    let held = inserted(
        ledger
            .reserve(request(&ledger, "state", 1_000, one_request(512, 1)))
            .expect("admission"),
    );
    assert_eq!(
        ledger.finalize_consumed(
            "state",
            held.revision,
            CanonicalQuotaVector::new(1, 1, 80, 0, 1, 400),
        ),
        Err(SemanticQuotaError::InvalidTransition)
    );
    ledger
        .mark_dispatch_attempted("state", held.revision)
        .expect("dispatch transition");
    assert_eq!(
        ledger.expire_pre_dispatch("state", ledger.revision(), 2_000),
        Err(SemanticQuotaError::InvalidTransition)
    );
    ledger
        .mark_indeterminate("state", ledger.revision())
        .expect("indeterminate transition");
    ledger
        .finalize_no_effect("state", ledger.revision())
        .expect("lookup proved no effect");
    ledger.verify_invariants().expect("consistent ledger");
}

#[test]
fn transition_chain_detects_digest_tampering() {
    let mut ledger = WindowedQuotaLedger::open(limits(100, 100, 100_000, 4, 10_000, 4_096))
        .expect("known limits");
    let held = inserted(
        ledger
            .reserve(request(&ledger, "chain", 1_100, one_request(512, 1)))
            .expect("admission"),
    );
    ledger
        .mark_dispatch_attempted("chain", held.revision)
        .expect("dispatch transition");
    ledger
        .mark_indeterminate("chain", ledger.revision())
        .expect("indeterminate transition");
    verify_transition_chain(ledger.transition_receipts()).expect("valid chain");

    let mut tampered = ledger.transition_receipts().to_vec();
    tampered[1].policy_sha256 = Sha256Digest::for_bytes(b"tampered-policy");
    assert_eq!(
        verify_transition_chain(&tampered),
        Err(SemanticQuotaError::DigestChainMismatch)
    );
}

#[test]
fn request_count_is_cumulative_across_window_rollover() {
    let mut ledger = WindowedQuotaLedger::open(limits(2, 100, 100_000, 4, 10_000, 4_096))
        .expect("known limits");
    for (id, at) in [("count-1", 1_200), ("count-2", 87_600)] {
        let held = inserted(
            ledger
                .reserve(request(&ledger, id, at, one_request(512, 1)))
                .expect("admission within cumulative request-count budget"),
        );
        ledger
            .mark_dispatch_attempted(id, held.revision)
            .expect("dispatch transition");
        ledger
            .finalize_consumed(
                id,
                ledger.revision(),
                CanonicalQuotaVector::new(1, 1, 80, 0, 1, 400),
            )
            .expect("terminal usage");
    }
    assert_eq!(
        ledger.reserve(request(
            &ledger,
            "count-3",
            174_000,
            one_request(512, 1),
        )),
        Err(SemanticQuotaError::QuotaExceeded)
    );
}

#[test]
fn unknown_limit_and_authority_escape_are_rejected() {
    let unknown = CanonicalQuotaLimits {
        request_count: Some(10),
        rpm: Some(10),
        tpm: Some(1_000),
        concurrency: Some(2),
        day_budget: Some(100),
        context: None,
    };
    assert!(matches!(
        WindowedQuotaLedger::open(unknown),
        Err(SemanticQuotaError::UnknownLimit)
    ));
    assert!(authority_posture_is_closed());
}
