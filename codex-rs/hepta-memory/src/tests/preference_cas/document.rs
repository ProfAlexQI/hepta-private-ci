#[test]
fn v2_genesis_is_revision_zero_exact_and_isolated_from_legacy_state() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("prefer-local-builds-v2");
    let subject = PrincipalId::new("operator-v2");
    let genesis = preference_document(
        0,
        "sha256:genesis",
        "preference.reducer.v1",
        r#"{"accepted":0,"rejected":0}"#,
    );

    store.seed(
        preference.clone(),
        subject.clone(),
        preference_state(99, "sha256:legacy-only"),
    )?;
    assert_eq!(
        store.get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone())?,
        PreferenceGenesisOutcome::Initialized
    );
    assert_eq!(
        store.get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone())?,
        PreferenceGenesisOutcome::AlreadyInitialized
    );
    assert_eq!(
        store.read_document(&preference, &subject)?,
        Some(genesis.clone())
    );
    assert_eq!(genesis.reducer_version(), "preference.reducer.v1");
    assert_eq!(
        genesis.canonical_payload(),
        r#"{"accepted":0,"rejected":0}"#
    );
    assert_eq!(
        store.read(&preference, &subject)?,
        Some(preference_state(99, "sha256:legacy-only"))
    );
    Ok(())
}

#[test]
fn v2_genesis_rejects_nonzero_revision_and_payload_or_version_drift() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("preference-genesis");
    let subject = PrincipalId::new("subject-genesis");
    let nonzero = preference_document(1, "sha256:nonzero", "reducer.v1", "{}");
    assert_eq!(
        store
            .get_or_init_genesis(preference.clone(), subject.clone(), nonzero)
            .expect_err("nonzero genesis must fail"),
        PreferenceCasError::NonZeroGenesis {
            attempted: Revision::new(1)
        }
    );

    let genesis = preference_document(0, "sha256:genesis", "reducer.v1", "{\"value\":0}");
    store.get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone())?;
    for conflicting in [
        preference_document(0, "sha256:genesis", "reducer.v1", "{\"value\":1}"),
        preference_document(0, "sha256:genesis", "reducer.v2", "{\"value\":0}"),
    ] {
        assert_eq!(
            store
                .get_or_init_genesis(preference.clone(), subject.clone(), conflicting.clone(),)
                .expect_err("genesis drift must fail"),
            PreferenceCasError::GenesisConflict {
                existing: Box::new(genesis.clone()),
                attempted: Box::new(conflicting),
            }
        );
    }
    Ok(())
}

#[test]
fn evidenced_document_commit_and_historical_replay_are_exact() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("preference-evidenced");
    let subject = PrincipalId::new("subject-evidenced");
    let genesis = preference_document(0, "sha256:g", "reducer.v1", "{\"score\":0}");
    store.get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone())?;

    let first_receipt = outcome_receipt("receipt-evidenced-1", "sha256:receipt-evidenced-1")?;
    let first_document = preference_document(1, "sha256:d1", "reducer.v1", "{\"score\":1}");
    let first = transition(
        "transition-evidenced-1",
        preference.clone(),
        subject.clone(),
        genesis.state().clone(),
        first_document.state().clone(),
        &first_receipt,
    )?;
    assert_eq!(
        store.commit_evidenced(first.clone(), first_document.clone())?,
        PreferenceDocumentCommitOutcome::Committed {
            document: first_document.clone()
        }
    );

    let second_receipt = outcome_receipt("receipt-evidenced-2", "sha256:receipt-evidenced-2")?;
    let second_document = preference_document(2, "sha256:d2", "reducer.v1", "{\"score\":2}");
    let second = transition(
        "transition-evidenced-2",
        preference.clone(),
        subject.clone(),
        first_document.state().clone(),
        second_document.state().clone(),
        &second_receipt,
    )?;
    assert!(
        store
            .commit_evidenced(second, second_document.clone())?
            .committed_now()
    );
    assert_eq!(
        store.commit_evidenced(first, first_document.clone())?,
        PreferenceDocumentCommitOutcome::AlreadyCommitted {
            document: first_document
        }
    );
    assert_eq!(
        store.read_document(&preference, &subject)?,
        Some(second_document)
    );
    Ok(())
}

#[test]
fn evidenced_commit_requires_v2_genesis_even_if_legacy_state_exists() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("preference-uninitialized");
    let subject = PrincipalId::new("subject-uninitialized");
    let previous = preference_state(0, "sha256:g");
    store.seed(preference.clone(), subject.clone(), previous.clone())?;
    let receipt = outcome_receipt("receipt-uninitialized", "sha256:receipt-uninitialized")?;
    let next_document = preference_document(1, "sha256:d1", "reducer.v1", "{\"v\":1}");
    let error = store
        .commit_evidenced(
            transition(
                "transition-uninitialized",
                preference.clone(),
                subject.clone(),
                previous,
                next_document.state().clone(),
                &receipt,
            )?,
            next_document,
        )
        .expect_err("legacy state must not stand in for a V2 genesis document");

    assert_eq!(
        error,
        PreferenceCasError::PreferenceDocumentNotInitialized {
            preference,
            subject,
        }
    );
    Ok(())
}

#[test]
fn evidenced_commit_rejects_document_mismatch_reducer_drift_and_stale_state() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("preference-validation");
    let subject = PrincipalId::new("subject-validation");
    let genesis = preference_document(0, "sha256:g", "reducer.v1", "{}");
    store.get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone())?;

    let receipt = outcome_receipt("receipt-validation", "sha256:receipt-validation")?;
    let next = preference_state(1, "sha256:next");
    let valid_transition = transition(
        "transition-validation",
        preference.clone(),
        subject.clone(),
        genesis.state().clone(),
        next.clone(),
        &receipt,
    )?;
    let mismatch = preference_document(1, "sha256:other", "reducer.v1", "{\"x\":1}");
    assert_eq!(
        store
            .commit_evidenced(valid_transition.clone(), mismatch.clone())
            .expect_err("document state must match transition"),
        PreferenceCasError::CommittedDocumentStateMismatch {
            expected: next.clone(),
            attempted: mismatch.state().clone(),
        }
    );

    let reducer_drift = PreferenceStateDocument::new(next.clone(), "reducer.v2", "{\"x\":1}");
    assert_eq!(
        store
            .commit_evidenced(valid_transition, reducer_drift)
            .expect_err("implicit reducer migration must fail"),
        PreferenceCasError::ReducerVersionConflict {
            existing: "reducer.v1".into(),
            attempted: "reducer.v2".into(),
        }
    );

    let stale_receipt = outcome_receipt("receipt-stale-v2", "sha256:receipt-stale-v2")?;
    let stale = preference_state(9, "sha256:stale");
    let stale_next = preference_state(10, "sha256:stale-next");
    let stale_transition = transition(
        "transition-stale-v2",
        preference.clone(),
        subject.clone(),
        stale.clone(),
        stale_next.clone(),
        &stale_receipt,
    )?;
    assert!(matches!(
        store
            .commit_evidenced(
                stale_transition,
                PreferenceStateDocument::new(stale_next, "reducer.v1", "{\"x\":10}"),
            )
            .expect_err("stale state must fail"),
        PreferenceCasError::StateConflict { expected, actual, .. }
            if expected == stale && actual == *genesis.state()
    ));
    assert_eq!(store.read_document(&preference, &subject)?, Some(genesis));
    Ok(())
}

#[test]
fn evidence_identity_drift_is_typed_and_does_not_mutate() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("preference-evidence-reuse");
    let subject = PrincipalId::new("subject-evidence-reuse");
    let genesis = preference_document(0, "sha256:g", "reducer.v1", "{}");
    store.get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone())?;

    let first_receipt = outcome_receipt("receipt-evidence-a", "sha256:receipt-evidence-a")?;
    let first_document = preference_document(1, "sha256:d1", "reducer.v1", "{\"v\":1}");
    let first = transition_with_evidence(
        "transition-evidence-a",
        "shared-evidence",
        "sha256:evidence-a",
        preference.clone(),
        subject.clone(),
        genesis.state().clone(),
        first_document.state().clone(),
        &first_receipt,
    )?;
    store.commit_evidenced(first, first_document.clone())?;

    let second_receipt = outcome_receipt("receipt-evidence-b", "sha256:receipt-evidence-b")?;
    let attempted_document = preference_document(2, "sha256:d2", "reducer.v1", "{\"v\":2}");
    let attempted = transition_with_evidence(
        "transition-evidence-b",
        "shared-evidence",
        "sha256:evidence-drifted",
        preference.clone(),
        subject.clone(),
        first_document.state().clone(),
        attempted_document.state().clone(),
        &second_receipt,
    )?;
    assert!(matches!(
        store
            .commit_evidenced(attempted, attempted_document)
            .expect_err("evidence identity drift must fail"),
        PreferenceCasError::EvidenceReuseConflict { evidence, .. }
            if evidence == PreferenceEvidenceId::new("shared-evidence")
    ));
    assert_eq!(
        store.read_document(&preference, &subject)?,
        Some(first_document)
    );
    Ok(())
}

#[test]
fn one_receipt_cannot_cross_preference_keys_in_v2() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let subject = PrincipalId::new("subject-receipt");
    let first_preference = PreferenceId::new("preference-receipt-a");
    let second_preference = PreferenceId::new("preference-receipt-b");
    let genesis = preference_document(0, "sha256:g", "reducer.v1", "{}");
    store.get_or_init_genesis(first_preference.clone(), subject.clone(), genesis.clone())?;
    store.get_or_init_genesis(second_preference.clone(), subject.clone(), genesis.clone())?;
    let receipt = outcome_receipt("receipt-cross-key-v2", "sha256:receipt-cross-key-v2")?;

    let first_document = preference_document(1, "sha256:a", "reducer.v1", "{\"a\":1}");
    store.commit_evidenced(
        transition(
            "transition-cross-key-a",
            first_preference,
            subject.clone(),
            genesis.state().clone(),
            first_document.state().clone(),
            &receipt,
        )?,
        first_document,
    )?;

    let second_document = preference_document(1, "sha256:b", "reducer.v1", "{\"b\":1}");
    let error = store
        .commit_evidenced(
            transition(
                "transition-cross-key-b",
                second_preference.clone(),
                subject.clone(),
                genesis.state().clone(),
                second_document.state().clone(),
                &receipt,
            )?,
            second_document,
        )
        .expect_err("receipt identity must be single-use across keys");
    assert!(matches!(
        error,
        PreferenceCasError::ReceiptReuseConflict { receipt, .. }
            if receipt == ReceiptId::new("receipt-cross-key-v2")
    ));
    assert_eq!(
        store.read_document(&second_preference, &subject)?,
        Some(genesis)
    );
    Ok(())
}

#[test]
fn competing_evidenced_commits_share_one_atomic_cas_section() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let preference = PreferenceId::new("preference-race-v2");
    let subject = PrincipalId::new("subject-race-v2");
    let genesis = preference_document(0, "sha256:g", "reducer.v1", "{}");
    store.get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone())?;
    let first_receipt = outcome_receipt("receipt-race-v2-a", "sha256:receipt-race-v2-a")?;
    let second_receipt = outcome_receipt("receipt-race-v2-b", "sha256:receipt-race-v2-b")?;
    let first_document = preference_document(1, "sha256:a", "reducer.v1", "{\"v\":1}");
    let second_document = preference_document(1, "sha256:b", "reducer.v1", "{\"v\":2}");
    let first = transition(
        "transition-race-v2-a",
        preference.clone(),
        subject.clone(),
        genesis.state().clone(),
        first_document.state().clone(),
        &first_receipt,
    )?;
    let second = transition(
        "transition-race-v2-b",
        preference,
        subject,
        genesis.state().clone(),
        second_document.state().clone(),
        &second_receipt,
    )?;
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
    let handles = [
        {
            let store = store.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                store.commit_evidenced(first, first_document)
            })
        },
        {
            let store = store.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                store.commit_evidenced(second, second_document)
            })
        },
    ];
    barrier.wait();
    let outcomes = handles.map(|handle| handle.join().expect("V2 CAS thread should join"));
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(
                outcome,
                Ok(PreferenceDocumentCommitOutcome::Committed { .. })
            ))
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
    Ok(())
}
