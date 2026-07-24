#[test]
fn commit_requires_the_exact_seeded_key_and_state() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("prefer-local-builds");
    let subject = PrincipalId::new("operator");
    let previous = preference_state(10, "sha256:preference-before");
    let next = preference_state(11, "sha256:preference-after");
    let receipt = outcome_receipt("receipt-1", "sha256:receipt-1")?;

    assert_eq!(
        store.seed(preference.clone(), subject.clone(), previous.clone())?,
        PreferenceSeedOutcome::Seeded
    );
    assert_eq!(
        store.seed(preference.clone(), subject.clone(), previous.clone())?,
        PreferenceSeedOutcome::AlreadySeeded
    );

    let outcome = store.commit(transition(
        "transition-1",
        preference.clone(),
        subject.clone(),
        previous,
        next.clone(),
        &receipt,
    )?)?;

    assert_eq!(
        outcome,
        PreferenceCommitOutcome::Committed {
            state: next.clone()
        }
    );
    assert!(outcome.committed_now());
    assert_eq!(outcome.state(), &next);
    assert_eq!(store.read(&preference, &subject)?, Some(next));
    Ok(())
}

#[test]
fn stale_expected_state_is_a_typed_conflict_and_does_not_mutate() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("prefer-local-builds");
    let subject = PrincipalId::new("operator");
    let current = preference_state(11, "sha256:preference-current");
    let stale = preference_state(10, "sha256:preference-stale");
    store.seed(preference.clone(), subject.clone(), current.clone())?;

    let receipt = outcome_receipt("receipt-stale", "sha256:receipt-stale")?;
    let error = store
        .commit(transition(
            "transition-stale",
            preference.clone(),
            subject.clone(),
            stale.clone(),
            preference_state(11, "sha256:preference-attempted"),
            &receipt,
        )?)
        .expect_err("a stale exact state must fail CAS");

    assert_eq!(
        error,
        PreferenceCasError::StateConflict {
            preference: preference.clone(),
            subject: subject.clone(),
            expected: stale,
            actual: current.clone(),
        }
    );
    assert_eq!(store.read(&preference, &subject)?, Some(current));
    Ok(())
}

#[test]
fn unseeded_subject_is_distinct_even_when_preference_and_state_match() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("prefer-local-builds");
    let seeded_subject = PrincipalId::new("operator-a");
    let attempted_subject = PrincipalId::new("operator-b");
    let previous = preference_state(1, "sha256:before");
    store.seed(preference.clone(), seeded_subject.clone(), previous.clone())?;

    let receipt = outcome_receipt("receipt-wrong-subject", "sha256:receipt-wrong-subject")?;
    let error = store
        .commit(transition(
            "transition-wrong-subject",
            preference.clone(),
            attempted_subject.clone(),
            previous.clone(),
            preference_state(2, "sha256:after"),
            &receipt,
        )?)
        .expect_err("the same state under a different subject must not match");

    assert_eq!(
        error,
        PreferenceCasError::PreferenceNotSeeded {
            preference: preference.clone(),
            subject: attempted_subject,
        }
    );
    assert_eq!(store.read(&preference, &seeded_subject)?, Some(previous));
    Ok(())
}

#[test]
fn competing_commits_are_serialized_by_one_cas_critical_section() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("prefer-local-builds");
    let subject = PrincipalId::new("operator");
    let previous = preference_state(1, "sha256:before");
    store.seed(preference.clone(), subject.clone(), previous.clone())?;

    let first_receipt = outcome_receipt("receipt-race-a", "sha256:receipt-race-a")?;
    let second_receipt = outcome_receipt("receipt-race-b", "sha256:receipt-race-b")?;
    let first = transition(
        "transition-race-a",
        preference.clone(),
        subject.clone(),
        previous.clone(),
        preference_state(2, "sha256:after-a"),
        &first_receipt,
    )?;
    let second = transition(
        "transition-race-b",
        preference.clone(),
        subject.clone(),
        previous,
        preference_state(2, "sha256:after-b"),
        &second_receipt,
    )?;
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));

    let first_handle = {
        let store = store.clone();
        let barrier = barrier.clone();
        std::thread::spawn(move || {
            barrier.wait();
            store.commit(first)
        })
    };
    let second_handle = {
        let store = store.clone();
        let barrier = barrier.clone();
        std::thread::spawn(move || {
            barrier.wait();
            store.commit(second)
        })
    };
    barrier.wait();

    let outcomes = [
        first_handle.join().expect("first CAS thread should join"),
        second_handle.join().expect("second CAS thread should join"),
    ];
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(outcome, Ok(PreferenceCommitOutcome::Committed { .. })))
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(outcome, Err(PreferenceCasError::StateConflict { .. })))
            .count(),
        1
    );
    assert_eq!(
        store
            .read(&preference, &subject)?
            .expect("one state must be committed")
            .revision(),
        Revision::new(2)
    );
    Ok(())
}

#[test]
fn exact_transition_replay_is_idempotent() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("prefer-local-builds");
    let subject = PrincipalId::new("operator");
    let previous = preference_state(1, "sha256:before");
    let next = preference_state(2, "sha256:after");
    store.seed(preference.clone(), subject.clone(), previous.clone())?;

    let receipt = outcome_receipt("receipt-replay", "sha256:receipt-replay")?;
    let transition = transition(
        "transition-replay",
        preference.clone(),
        subject.clone(),
        previous,
        next.clone(),
        &receipt,
    )?;
    assert!(store.commit(transition.clone())?.committed_now());
    assert_eq!(
        store.commit(transition)?,
        PreferenceCommitOutcome::AlreadyCommitted {
            state: next.clone()
        }
    );
    assert_eq!(store.read(&preference, &subject)?, Some(next));
    Ok(())
}

#[test]
fn receipt_identity_cannot_back_a_different_transition() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let subject = PrincipalId::new("operator");
    let first_preference = PreferenceId::new("prefer-local-builds");
    let second_preference = PreferenceId::new("prefer-brief-output");
    let previous = preference_state(1, "sha256:before");
    store.seed(first_preference.clone(), subject.clone(), previous.clone())?;
    store.seed(second_preference.clone(), subject.clone(), previous.clone())?;

    let receipt = outcome_receipt("receipt-single-use", "sha256:receipt-single-use")?;
    store.commit(transition(
        "transition-first",
        first_preference,
        subject.clone(),
        previous.clone(),
        preference_state(2, "sha256:first-after"),
        &receipt,
    )?)?;
    let error = store
        .commit(transition(
            "transition-second",
            second_preference.clone(),
            subject.clone(),
            previous.clone(),
            preference_state(2, "sha256:second-after"),
            &receipt,
        )?)
        .expect_err("one receipt identity must not back a different transition");

    assert!(matches!(
        error,
        PreferenceCasError::ReceiptReuseConflict {
            receipt,
            existing_transition,
            attempted_transition,
            ..
        } if receipt == ReceiptId::new("receipt-single-use")
            && existing_transition == PreferenceTransitionId::new("transition-first")
            && attempted_transition == PreferenceTransitionId::new("transition-second")
    ));
    assert_eq!(store.read(&second_preference, &subject)?, Some(previous));
    Ok(())
}

#[test]
fn transition_identity_reuse_with_changed_content_is_rejected() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("prefer-local-builds");
    let subject = PrincipalId::new("operator");
    let previous = preference_state(1, "sha256:before");
    store.seed(preference.clone(), subject.clone(), previous.clone())?;

    let first_receipt = outcome_receipt("receipt-first", "sha256:receipt-first")?;
    store.commit(transition(
        "transition-reused",
        preference.clone(),
        subject.clone(),
        previous.clone(),
        preference_state(2, "sha256:after"),
        &first_receipt,
    )?)?;

    let rebound_receipt = outcome_receipt("receipt-rebound", "sha256:receipt-rebound")?;
    let error = store
        .commit(transition(
            "transition-reused",
            preference,
            subject,
            previous,
            preference_state(2, "sha256:different-after"),
            &rebound_receipt,
        )?)
        .expect_err("transition identity reuse must require exact replay");
    assert_eq!(
        error,
        PreferenceCasError::TransitionReuseConflict {
            transition: PreferenceTransitionId::new("transition-reused")
        }
    );
    Ok(())
}
#[test]
fn contracts_reject_non_advancing_state_before_cas() -> TestResult {
    let receipt = outcome_receipt("receipt-invalid", "sha256:receipt-invalid")?;
    let expected = Revision::new(10);
    let evidence = preference_evidence(
        "evidence-invalid",
        "sha256:evidence-invalid",
        PreferenceId::new("prefer-local-builds"),
        PrincipalId::new("operator"),
        &receipt,
    );
    let error = PreferenceTransition::try_new(
        PreferenceTransitionId::new("transition-invalid"),
        &evidence,
        PreferenceState::new(expected, ContentHash::new("sha256:before")),
        PreferenceState::new(Revision::new(9), ContentHash::new("sha256:older")),
    )
    .expect_err("contracts must reject a non-advancing revision");

    assert_eq!(
        error,
        ContractError::PreferenceRevisionNotAdvanced {
            expected,
            committed: Revision::new(9),
        }
    );
    Ok(())
}
