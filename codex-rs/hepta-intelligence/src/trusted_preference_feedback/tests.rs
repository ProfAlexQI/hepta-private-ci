use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use hepta_contracts::CapabilityId;
use hepta_contracts::CapabilityManifestRef;
use hepta_contracts::ContentHash;
use hepta_contracts::PreferenceEvidenceId;
use hepta_contracts::PreferenceId;
use hepta_contracts::PreferenceState;
use hepta_contracts::PreferenceTransitionId;
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptId;
use hepta_contracts::Revision;
use hepta_contracts::RevisionStamp;
use hepta_memory::PreferenceCasError;
use hepta_memory::PreferenceGenesisOutcome;

use super::*;
use crate::explicit_preference_genesis;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn durable_integrity_key(byte: u8) -> DurableIntegrityKey {
    DurableIntegrityKey::from_bytes([byte; 32])
}

#[test]
fn trusted_explicit_feedback_binds_source_target_prior_reducer_and_one_cas() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let target = target("echo");
    let genesis = explicit_preference_genesis(
        PrincipalId::new("subject:alice"),
        PreferenceId::new("preference:tool-choice"),
        target.clone(),
    );
    let genesis_document = PreferenceStateDocument::new(
        genesis.state().clone(),
        EXPLICIT_PREFERENCE_REDUCER_VERSION,
        genesis.canonical_payload(),
    );
    assert_eq!(
        store.get_or_init_genesis(
            genesis.preference().clone(),
            genesis.subject().clone(),
            genesis_document.clone(),
        )?,
        PreferenceGenesisOutcome::Initialized
    );
    let source = ExactTrustedSource::new(
        source_ref("human-auth"),
        genesis.subject().clone(),
        genesis.preference().clone(),
        target.clone(),
        genesis.state().clone(),
    );
    let feedback_input = input("trusted", &genesis, target.clone());

    let outcome = advance_trusted_explicit_preference(&store, &source, feedback_input)?;
    assert!(outcome.committed_now());
    assert_eq!(outcome.source(), &source_ref("human-auth"));
    assert_eq!(
        outcome.reducer(),
        TrustedExplicitPreferenceReducer::try_new()?.binding()
    );
    assert_eq!(outcome.expected_previous(), genesis.state());
    assert_eq!(outcome.evidence().subject(), genesis.subject());
    assert_eq!(outcome.evidence().preference(), genesis.preference());
    assert_eq!(
        outcome.evidence().target_binding_hash(),
        &target.binding_hash()
    );
    assert_eq!(
        outcome.evidence().signal(),
        ExplicitPreferenceSignal::Accepted
    );

    let expected = reduce_explicit_preference(
        genesis.state(),
        genesis.canonical_payload(),
        outcome.evidence(),
    )?;
    assert_eq!(
        store.read_document(genesis.preference(), genesis.subject())?,
        Some(PreferenceStateDocument::new(
            expected.next_state().clone(),
            EXPLICIT_PREFERENCE_REDUCER_VERSION,
            expected.canonical_payload(),
        ))
    );

    assert!(matches!(
        advance_trusted_explicit_preference(&store, &source, input("trusted", &genesis, target),)
            .expect_err("the exact prior request must not advance twice"),
        PreferenceAuthorityError::Cas(PreferenceCasError::StateConflict { .. })
    ));
    assert_eq!(source.calls.load(Ordering::SeqCst), 1);
    Ok(())
}

#[test]
fn source_denial_or_binding_mismatch_leaves_preference_unchanged() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let target = target("echo");
    let genesis = explicit_preference_genesis(
        PrincipalId::new("subject:alice"),
        PreferenceId::new("preference:tool-choice"),
        target.clone(),
    );
    let genesis_document = PreferenceStateDocument::new(
        genesis.state().clone(),
        EXPLICIT_PREFERENCE_REDUCER_VERSION,
        genesis.canonical_payload(),
    );
    store.get_or_init_genesis(
        genesis.preference().clone(),
        genesis.subject().clone(),
        genesis_document.clone(),
    )?;
    let source = ExactTrustedSource::new(
        source_ref("strict"),
        PrincipalId::new("subject:bob"),
        genesis.preference().clone(),
        target.clone(),
        genesis.state().clone(),
    );

    assert_eq!(
        advance_trusted_explicit_preference(
            &store,
            &source,
            input("wrong-subject", &genesis, target),
        )
        .expect_err("source must reject a subject it did not authenticate"),
        PreferenceAuthorityError::Authentication(PreferenceFeedbackAuthenticationError::new(
            "trusted_source.challenge_mismatch"
        ))
    );
    assert_eq!(
        store.read_document(genesis.preference(), genesis.subject())?,
        Some(genesis_document)
    );
    Ok(())
}

#[test]
fn hmac_ingress_proof_authenticates_the_memory_owned_challenge() -> TestResult {
    let store = InMemoryPreferenceStore::default();
    let target = target("authenticated-ingress");
    let genesis = explicit_preference_genesis(
        PrincipalId::new("subject:alice"),
        PreferenceId::new("preference:tool-choice"),
        target.clone(),
    );
    store.get_or_init_genesis(
        genesis.preference().clone(),
        genesis.subject().clone(),
        PreferenceStateDocument::new(
            genesis.state().clone(),
            EXPLICIT_PREFERENCE_REDUCER_VERSION,
            genesis.canonical_payload(),
        ),
    )?;
    let input = input("authenticated-ingress", &genesis, target);
    let source = source_ref("authenticated-ingress");
    let client_key = PreferenceIngressAuthenticationKey::from_bytes([0x31; 32]);
    let challenge_hash = explicit_preference_feedback_challenge_hash(&input, source.clone())?;
    let proof = sign_preference_ingress_challenge(&client_key, &challenge_hash)?;
    let ingress = HmacTrustedPreferenceFeedbackSource::new(
        source.clone(),
        PreferenceIngressAuthenticationKey::from_bytes([0x31; 32]),
        PreferenceIngressProof::from_hex(&proof.to_hex())?,
    );

    let outcome = advance_trusted_explicit_preference(&store, &ingress, input)?;

    assert!(outcome.committed_now());
    assert_eq!(outcome.source(), &source);
    Ok(())
}

#[test]
fn hmac_ingress_rejects_wrong_key_tampering_and_noncanonical_proofs() -> TestResult {
    assert_eq!(
        PreferenceIngressProof::from_hex(&"A".repeat(64)),
        Err(PreferenceFeedbackAuthenticationError::new(
            "trusted_preference_ingress.proof_encoding_invalid"
        ))
    );

    let store = InMemoryPreferenceStore::default();
    let target = target("authenticated-ingress-denial");
    let genesis = explicit_preference_genesis(
        PrincipalId::new("subject:alice"),
        PreferenceId::new("preference:tool-choice"),
        target.clone(),
    );
    let genesis_document = PreferenceStateDocument::new(
        genesis.state().clone(),
        EXPLICIT_PREFERENCE_REDUCER_VERSION,
        genesis.canonical_payload(),
    );
    store.get_or_init_genesis(
        genesis.preference().clone(),
        genesis.subject().clone(),
        genesis_document.clone(),
    )?;
    let signed_input = input("signed-ingress", &genesis, target.clone());
    let source = source_ref("authenticated-ingress-denial");
    let challenge_hash =
        explicit_preference_feedback_challenge_hash(&signed_input, source.clone())?;
    let proof = sign_preference_ingress_challenge(
        &PreferenceIngressAuthenticationKey::from_bytes([0x41; 32]),
        &challenge_hash,
    )?;
    let wrong_key_source = HmacTrustedPreferenceFeedbackSource::new(
        source.clone(),
        PreferenceIngressAuthenticationKey::from_bytes([0x42; 32]),
        proof.clone(),
    );
    assert_eq!(
        advance_trusted_explicit_preference(&store, &wrong_key_source, signed_input)
            .expect_err("a wrong ingress key must fail closed"),
        PreferenceAuthorityError::Authentication(PreferenceFeedbackAuthenticationError::new(
            "trusted_preference_ingress.proof_verification_failed"
        ))
    );

    let exact_key_source = HmacTrustedPreferenceFeedbackSource::new(
        source,
        PreferenceIngressAuthenticationKey::from_bytes([0x41; 32]),
        proof,
    );
    assert_eq!(
        advance_trusted_explicit_preference(
            &store,
            &exact_key_source,
            input("tampered-ingress", &genesis, target),
        )
        .expect_err("a proof for a different challenge must fail closed"),
        PreferenceAuthorityError::Authentication(PreferenceFeedbackAuthenticationError::new(
            "trusted_preference_ingress.proof_verification_failed"
        ))
    );
    assert_eq!(
        store.read_document(genesis.preference(), genesis.subject())?,
        Some(genesis_document)
    );
    Ok(())
}

#[test]
fn reducer_version_and_payload_fail_before_any_cas_mutation() -> TestResult {
    let target = target("echo");
    let genesis = explicit_preference_genesis(
        PrincipalId::new("subject:alice"),
        PreferenceId::new("preference:tool-choice"),
        target.clone(),
    );
    let source = ExactTrustedSource::new(
        source_ref("strict"),
        genesis.subject().clone(),
        genesis.preference().clone(),
        target.clone(),
        genesis.state().clone(),
    );

    let wrong_version_store = InMemoryPreferenceStore::default();
    let wrong_version_document =
        PreferenceStateDocument::new(genesis.state().clone(), "reducer.v2", "{}");
    wrong_version_store.get_or_init_genesis(
        genesis.preference().clone(),
        genesis.subject().clone(),
        wrong_version_document.clone(),
    )?;
    assert_eq!(
        advance_trusted_explicit_preference(
            &wrong_version_store,
            &source,
            input("wrong-version", &genesis, target.clone()),
        )
        .expect_err("reducer version drift must fail before authentication"),
        PreferenceAuthorityError::ReducerVersionConflict {
            current: "reducer.v2".into(),
            authority: EXPLICIT_PREFERENCE_REDUCER_VERSION.into(),
        }
    );
    assert_eq!(source.calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        wrong_version_store.read_document(genesis.preference(), genesis.subject())?,
        Some(wrong_version_document)
    );

    let corrupt_store = InMemoryPreferenceStore::default();
    let corrupt_document = PreferenceStateDocument::new(
        genesis.state().clone(),
        EXPLICIT_PREFERENCE_REDUCER_VERSION,
        "not-a-canonical-payload",
    );
    corrupt_store.get_or_init_genesis(
        genesis.preference().clone(),
        genesis.subject().clone(),
        corrupt_document.clone(),
    )?;
    assert_eq!(
        advance_trusted_explicit_preference(
            &corrupt_store,
            &source,
            input("corrupt-payload", &genesis, target),
        )
        .expect_err("malformed reducer state must fail closed"),
        PreferenceAuthorityError::Reduction(PreferenceDomainReducerError::new(
            "explicit_preference.malformed_previous_payload"
        ))
    );
    assert_eq!(
        corrupt_store.read_document(genesis.preference(), genesis.subject())?,
        Some(corrupt_document)
    );
    Ok(())
}

#[tokio::test]
async fn composed_keyed_authority_advances_once_and_reopens_for_audit() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("trusted-preference.sqlite3");
    let target_binding = target("durable-authority");
    let genesis = explicit_preference_genesis(
        PrincipalId::new("subject:durable-authority"),
        PreferenceId::new("preference:durable-authority"),
        target_binding.clone(),
    );
    let genesis_document = PreferenceStateDocument::new(
        genesis.state().clone(),
        EXPLICIT_PREFERENCE_REDUCER_VERSION,
        genesis.canonical_payload(),
    );
    let source = ExactTrustedSource::new(
        source_ref("durable-authority"),
        genesis.subject().clone(),
        genesis.preference().clone(),
        target_binding.clone(),
        genesis.state().clone(),
    );
    let authority = DurableTrustedPreferenceFeedbackAuthority::bootstrap_new(
        &database_path,
        durable_integrity_key(0x71),
        source,
    )
    .await?;
    assert_eq!(
        authority
            .get_or_init_genesis(
                genesis.preference().clone(),
                genesis.subject().clone(),
                genesis_document,
            )
            .await?,
        PreferenceGenesisOutcome::Initialized
    );
    let outcome = authority
        .advance(input("composed-durable", &genesis, target_binding))
        .await?;
    assert!(outcome.committed_now());
    assert_eq!(outcome.source(), authority.source_binding());
    let expected_document = outcome.commit().document().clone();
    drop(authority);

    let audit_source = ExactTrustedSource::new(
        source_ref("durable-authority"),
        genesis.subject().clone(),
        genesis.preference().clone(),
        target("durable-authority"),
        expected_document.state().clone(),
    );
    let reopened = DurableTrustedPreferenceFeedbackAuthority::open_existing(
        &database_path,
        durable_integrity_key(0x71),
        audit_source,
    )
    .await?;
    assert_eq!(
        reopened
            .read_document(genesis.preference(), genesis.subject())
            .await?,
        Some(expected_document)
    );
    drop(reopened);

    let wrong_key_source = ExactTrustedSource::new(
        source_ref("durable-authority"),
        genesis.subject().clone(),
        genesis.preference().clone(),
        target("durable-authority"),
        genesis.state().clone(),
    );
    let error = match DurableTrustedPreferenceFeedbackAuthority::open_existing(
        &database_path,
        durable_integrity_key(0x72),
        wrong_key_source,
    )
    .await
    {
        Ok(_) => panic!("wrong preference integrity key must fail closed"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        PreferenceAuthorityError::Cas(PreferenceCasError::Corrupt { detail })
            if detail.contains("integrity key or algorithm")
    ));
    Ok(())
}

#[tokio::test]
async fn composed_authority_rejects_source_drift_before_authentication_or_cas() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("trusted-source-drift.sqlite3");
    let target = target("source-drift");
    let genesis = explicit_preference_genesis(
        PrincipalId::new("subject:source-drift"),
        PreferenceId::new("preference:source-drift"),
        target.clone(),
    );
    let genesis_document = PreferenceStateDocument::new(
        genesis.state().clone(),
        EXPLICIT_PREFERENCE_REDUCER_VERSION,
        genesis.canonical_payload(),
    );
    {
        let store = DurablePreferenceStore::bootstrap_new_keyed(
            &database_path,
            durable_integrity_key(0x73),
        )
        .await?;
        store
            .get_or_init_genesis(
                genesis.preference().clone(),
                genesis.subject().clone(),
                genesis_document.clone(),
            )
            .await?;
    }
    let source = DriftingComposedSource {
        first: source_ref("source-drift-a"),
        second: source_ref("source-drift-b"),
        reads: AtomicUsize::new(0),
    };
    let authority = DurableTrustedPreferenceFeedbackAuthority::open_existing(
        &database_path,
        durable_integrity_key(0x73),
        source,
    )
    .await?;
    assert!(matches!(
        authority
            .advance(input("source-drift", &genesis, target))
            .await
            .expect_err("source drift must fail before authentication"),
        PreferenceAuthorityError::Authentication(error)
            if error == PreferenceFeedbackAuthenticationError::new(
                "trusted_preference_feedback.source_binding_changed"
            )
    ));
    drop(authority);

    let store =
        DurablePreferenceStore::open_existing_keyed(&database_path, durable_integrity_key(0x73))
            .await?;
    assert_eq!(
        store
            .read_document(genesis.preference(), genesis.subject())
            .await?,
        Some(genesis_document)
    );
    Ok(())
}

fn input(
    label: &str,
    genesis: &crate::PreferenceAccumulator,
    target: ExplicitPreferenceTarget,
) -> ExplicitPreferenceFeedbackInput {
    ExplicitPreferenceFeedbackInput::try_new(ExplicitPreferenceFeedbackInputParts {
        transition_id: PreferenceTransitionId::new(format!("transition:{label}")),
        evidence_id: PreferenceEvidenceId::new(format!("evidence:{label}")),
        signal: ExplicitPreferenceSignal::Accepted,
        receipt: hepta_contracts::ReceiptRef::new(
            ReceiptId::new(format!("receipt:{label}")),
            ContentHash::new(format!("sha256:receipt-{label}")),
        ),
        session_binding_hash: ContentHash::new(format!("sha256:session-{label}")),
        subject: genesis.subject().clone(),
        preference: genesis.preference().clone(),
        target,
        expected_previous: genesis.state().clone(),
    })
    .expect("test trusted feedback input must be valid")
}

fn target(label: &str) -> ExplicitPreferenceTarget {
    ExplicitPreferenceTarget::Capability(CapabilityManifestRef::new(
        CapabilityId::new(format!("tool:{label}")),
        Revision::new(7),
        ContentHash::new(format!("sha256:manifest-{label}")),
        RevisionStamp::new(Revision::new(11), ContentHash::new("sha256:catalog-main")),
    ))
}

fn source_ref(label: &str) -> PreferenceFeedbackSourceRef {
    PreferenceFeedbackSourceRef::try_new(
        PrincipalId::new(format!("trusted-source:{label}")),
        Revision::new(3),
        ContentHash::new(format!("sha256:trusted-source-{label}")),
    )
    .expect("test trusted source reference must be valid")
}

struct ExactTrustedSource {
    binding: PreferenceFeedbackSourceRef,
    subject: PrincipalId,
    preference: PreferenceId,
    target: ExplicitPreferenceTarget,
    previous: PreferenceState,
    calls: AtomicUsize,
}

struct DriftingComposedSource {
    first: PreferenceFeedbackSourceRef,
    second: PreferenceFeedbackSourceRef,
    reads: AtomicUsize,
}

impl TrustedPreferenceFeedbackSource for DriftingComposedSource {
    fn source(&self) -> PreferenceFeedbackSourceRef {
        if self.reads.fetch_add(1, Ordering::SeqCst) < 2 {
            self.first.clone()
        } else {
            self.second.clone()
        }
    }

    fn authenticate(
        &self,
        _challenge: &TrustedPreferenceFeedbackChallenge<'_>,
    ) -> Result<(), PreferenceFeedbackAuthenticationError> {
        Err(PreferenceFeedbackAuthenticationError::new(
            "drifting_composed_source.must_not_authenticate",
        ))
    }
}

impl ExactTrustedSource {
    fn new(
        binding: PreferenceFeedbackSourceRef,
        subject: PrincipalId,
        preference: PreferenceId,
        target: ExplicitPreferenceTarget,
        previous: PreferenceState,
    ) -> Self {
        Self {
            binding,
            subject,
            preference,
            target,
            previous,
            calls: AtomicUsize::new(0),
        }
    }
}

impl TrustedPreferenceFeedbackSource for ExactTrustedSource {
    fn source(&self) -> PreferenceFeedbackSourceRef {
        self.binding.clone()
    }

    fn authenticate(
        &self,
        challenge: &TrustedPreferenceFeedbackChallenge<'_>,
    ) -> Result<(), PreferenceFeedbackAuthenticationError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let authority = challenge.authority();
        let request = authority.request();
        if authority.source() == &self.binding
            && authority.reducer().identity() == EXPLICIT_PREFERENCE_REDUCER_ID
            && authority.reducer().version() == EXPLICIT_PREFERENCE_REDUCER_VERSION
            && request.subject() == &self.subject
            && request.preference() == &self.preference
            && request.expected_previous() == &self.previous
            && request.target_binding_hash() == &self.target.binding_hash()
            && challenge.target() == &self.target
        {
            Ok(())
        } else {
            Err(PreferenceFeedbackAuthenticationError::new(
                "trusted_source.challenge_mismatch",
            ))
        }
    }
}
