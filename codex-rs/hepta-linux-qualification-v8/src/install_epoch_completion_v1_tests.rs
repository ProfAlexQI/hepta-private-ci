use sha2::Digest as _;

use super::*;

const COMMIT_NONCE_CHARACTER: char = 'e';
const INITIAL_QUERY_NONCE_CHARACTER: char = '7';

fn digest(character: char) -> String {
    character.to_string().repeat(64)
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

fn commit_trust() -> VerifiedTrustPolicyBindingV8 {
    crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCommitV1)
}

fn current_tip_trust() -> VerifiedTrustPolicyBindingV8 {
    crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1)
}

fn signer(trust: &VerifiedTrustPolicyBindingV8) -> AuthoritySignerBindingV8 {
    AuthoritySignerBindingV8 {
        allowed_signers_sha256: trust.allowed_signers_sha256().to_string(),
        key_fingerprint: trust.key_fingerprint().to_string(),
        principal: trust.principal().to_string(),
        signature_algorithm: trust.signature_algorithm(),
    }
}

fn commit_challenge(
    intent: &IssuedExternalWatermarkCasIntentV1,
    committed_at: u64,
) -> ExternalWatermarkCommitChallengeV1 {
    let preparation = intent.preparation();
    ExternalWatermarkCommitChallengeV1 {
        authority_nonce: preparation.authority_nonce().to_string(),
        capability:
            ExternalWatermarkCommitCapabilityV1::ExactIdempotentCasByCommitNonceAndConsumeLeaseOnce,
        commit_nonce: intent.commit_nonce().to_string(),
        committed_at_unix_seconds: committed_at,
        completion_operation_binding_sha256: intent
            .completion_operation_binding_sha256()
            .to_string(),
        completion_profile_sha256: intent.0.profile.profile_sha256.clone(),
        lease_nonce: preparation.lease_nonce().to_string(),
        lease_signature_sha256: preparation.lease_signature_sha256().to_string(),
        lease_statement_sha256: preparation.lease_statement_sha256().to_string(),
        lease_trust_policy_sha256: preparation.lease_trust_policy_sha256().to_string(),
        namespace: EXTERNAL_WATERMARK_COMMIT_NAMESPACE_V1.to_string(),
        predecessor: preparation.predecessor().clone(),
        preparation_binding_sha256: install_epoch_preparation_binding_sha256_v1(preparation),
        provider_transaction_sha256: digest('f'),
        schema: EXTERNAL_WATERMARK_COMMIT_SCHEMA_V1.to_string(),
        signer: signer(&commit_trust()),
        successor_record: intent.0.successor_record.clone(),
        successor_tip_sha256: intent.0.successor_tip_sha256.clone(),
    }
}

struct SignedCommit {
    envelope: SignedExternalWatermarkCommitV1,
    observation: CryptographicSignatureObservation,
}

fn sign_commit(
    intent: &IssuedExternalWatermarkCasIntentV1,
    challenge: ExternalWatermarkCommitChallengeV1,
) -> SignedCommit {
    let commit_trust = commit_trust();
    let current_tip_trust = current_tip_trust();
    let statement = canonical_commit_statement_with_profile_v1(
        &intent.0,
        &challenge,
        &commit_trust,
        &current_tip_trust,
        &intent.0.profile,
    )
    .unwrap();
    let statement_sha256 = sha256(&statement);
    let signature_bytes = b"test-provider-cas-commit-signature".to_vec();
    let signature_sha256 = sha256(&signature_bytes);
    let observation = CryptographicSignatureObservation::for_test_only(
        signature_sha256.clone(),
        statement_sha256.clone(),
        SshsigTrustPurposeV8::ExternalWatermarkCommitV1,
    );
    SignedCommit {
        envelope: SignedExternalWatermarkCommitV1 {
            canonical_statement_sha256: statement_sha256,
            challenge,
            detached_signature_bytes: signature_bytes,
            detached_signature_sha256: signature_sha256,
        },
        observation,
    }
}

fn exact_signed_commit(
    intent: &IssuedExternalWatermarkCasIntentV1,
    committed_at: u64,
) -> SignedCommit {
    sign_commit(intent, commit_challenge(intent, committed_at))
}

fn current_tip_challenge(
    issued: &IssuedExternalWatermarkCurrentTipQueryV1,
    commit: &SignedExternalWatermarkCommitV1,
    issued_at: u64,
    expires_at: u64,
) -> ExternalWatermarkCurrentTipChallengeV1 {
    let pending = &issued.0;
    let preparation = pending.preparation();
    ExternalWatermarkCurrentTipChallengeV1 {
        capability: ExternalWatermarkCurrentTipCapabilityV1::
            ExactIdempotentAuthenticatedCurrentTipByQueryNonceAfterCommit,
        commit_signature_sha256: sha256(&commit.detached_signature_bytes),
        commit_statement_sha256: commit.canonical_statement_sha256.clone(),
        commit_trust_policy_sha256: commit_trust().policy_sha256().to_string(),
        completion_profile_sha256: pending.intent.profile.profile_sha256.clone(),
        current_record: pending.intent.successor_record.clone(),
        current_revision: pending.intent.successor_record.successor_revision,
        current_tip_sha256: pending.intent.successor_tip_sha256.clone(),
        expires_at_unix_seconds: expires_at,
        issued_at_unix_seconds: issued_at,
        namespace: EXTERNAL_WATERMARK_CURRENT_TIP_NAMESPACE_V1.to_string(),
        preparation_binding_sha256: install_epoch_preparation_binding_sha256_v1(preparation),
        provider_transaction_sha256: commit.challenge.provider_transaction_sha256.clone(),
        query_nonce: pending.active_query_nonce().to_string(),
        schema: EXTERNAL_WATERMARK_CURRENT_TIP_SCHEMA_V1.to_string(),
        signer: signer(&current_tip_trust()),
        stream_id_sha256: pending.intent.successor_record.stream_id_sha256.clone(),
        target_host: preparation.target_host().clone(),
    }
}

struct SignedCurrentTip {
    envelope: SignedExternalWatermarkCurrentTipV1,
    observation: CryptographicSignatureObservation,
}

fn sign_current_tip(
    issued: &IssuedExternalWatermarkCurrentTipQueryV1,
    challenge: ExternalWatermarkCurrentTipChallengeV1,
) -> SignedCurrentTip {
    let pending = &issued.0;
    let commit_trust = commit_trust();
    let current_tip_trust = current_tip_trust();
    let statement = canonical_current_tip_statement_with_profile_v1(
        pending,
        &challenge,
        &commit_trust,
        &current_tip_trust,
        &pending.intent.profile,
    )
    .unwrap();
    let statement_sha256 = sha256(&statement);
    let signature_bytes = b"test-provider-current-tip-signature".to_vec();
    let signature_sha256 = sha256(&signature_bytes);
    let observation = CryptographicSignatureObservation::for_test_only(
        signature_sha256.clone(),
        statement_sha256.clone(),
        SshsigTrustPurposeV8::ExternalWatermarkCurrentTipV1,
    );
    SignedCurrentTip {
        envelope: SignedExternalWatermarkCurrentTipV1 {
            canonical_statement_sha256: statement_sha256,
            challenge,
            detached_signature_bytes: signature_bytes,
            detached_signature_sha256: signature_sha256,
        },
        observation,
    }
}

fn exact_signed_current_tip(
    issued: &IssuedExternalWatermarkCurrentTipQueryV1,
    commit: &SignedExternalWatermarkCommitV1,
    issued_at: u64,
    expires_at: u64,
) -> SignedCurrentTip {
    sign_current_tip(
        issued,
        current_tip_challenge(issued, commit, issued_at, expires_at),
    )
}

fn prepare_fresh_begin(
    preparation: VerifiedInstallEpochPreparationV1,
    guard: &mut InstallEpochReplayGuardV1,
) -> FreshExternalWatermarkCasIntentV1 {
    let outcome = prepare_cas_intent_for_test_v1(
        preparation,
        digest(COMMIT_NONCE_CHARACTER),
        digest(INITIAL_QUERY_NONCE_CHARACTER),
        guard,
    )
    .unwrap();
    match outcome {
        ExternalWatermarkCasIntentOutcomeV1::Fresh(fresh) => fresh,
        ExternalWatermarkCasIntentOutcomeV1::Recovered(_) => {
            panic!("first exact begin unexpectedly recovered an old intent")
        }
    }
}

fn begin_fresh(
    preparation: VerifiedInstallEpochPreparationV1,
    guard: &mut InstallEpochReplayGuardV1,
) -> IssuedExternalWatermarkCasIntentV1 {
    let fresh = prepare_fresh_begin(preparation, guard);
    match fresh.reserve_provider_cas_model(guard).unwrap() {
        ExternalWatermarkCasReservationOutcomeV1::Fresh(reserved) => {
            reserved.into_pending_after_provider_call()
        }
        ExternalWatermarkCasReservationOutcomeV1::Recovered(_) => {
            panic!("first CAS issue reservation unexpectedly recovered an old edge")
        }
    }
}

fn begin_recovered(
    preparation: VerifiedInstallEpochPreparationV1,
    guard: &mut InstallEpochReplayGuardV1,
) -> IssuedExternalWatermarkCasIntentV1 {
    let outcome = prepare_cas_intent_for_test_v1(
        preparation,
        digest(COMMIT_NONCE_CHARACTER),
        digest(INITIAL_QUERY_NONCE_CHARACTER),
        guard,
    )
    .unwrap();
    match outcome {
        ExternalWatermarkCasIntentOutcomeV1::Recovered(recovered) => {
            match recovered.reserve_provider_cas_model(guard).unwrap() {
                ExternalWatermarkCasReservationOutcomeV1::Recovered(reserved) => {
                    reserved.into_pending_for_receipt_reconciliation()
                }
                ExternalWatermarkCasReservationOutcomeV1::Fresh(_) => {
                    panic!("recovered begin unexpectedly won an already-issued CAS edge")
                }
            }
        }
        ExternalWatermarkCasIntentOutcomeV1::Fresh(_) => {
            panic!("exact begin replay unexpectedly authorized a fresh CAS")
        }
    }
}

fn commit_pending(
    intent: IssuedExternalWatermarkCasIntentV1,
    committed_at: u64,
    guard: &mut InstallEpochReplayGuardV1,
) -> (VerifiedCasCommittedPendingTipV1, SignedCommit) {
    let signed = exact_signed_commit(&intent, committed_at);
    let pending =
        verify_cas_commit_for_test_v1(intent, &signed.envelope, &signed.observation, guard)
            .unwrap();
    (pending, signed)
}

fn reserve_query(
    pending: VerifiedCasCommittedPendingTipV1,
    guard: &mut InstallEpochReplayGuardV1,
) -> (IssuedExternalWatermarkCurrentTipQueryV1, bool) {
    match pending.reserve_current_tip_query_model(guard).unwrap() {
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Fresh(reserved) => {
            (reserved.into_pending_after_provider_call(), true)
        }
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Recovered(reserved) => {
            (reserved.into_pending_for_receipt_reconciliation(), false)
        }
    }
}

fn query_closure(
    issued: &IssuedExternalWatermarkCurrentTipQueryV1,
    evidence_character: char,
) -> VerifiedExternalWatermarkCurrentTipQueryClosureV1 {
    verified_query_closure_for_test_v1(issued, digest(evidence_character))
}

fn duplicate_consumed_initial_query_phase() -> (
    IssuedExternalWatermarkCurrentTipQueryV1,
    IssuedExternalWatermarkCurrentTipQueryV1,
    SignedCommit,
    InstallEpochReplayGuardV1,
) {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let fresh_intent = begin_fresh(preparation, &mut guard);
    let (same_preparation, _) = crate::test_only_genesis_install_epoch_preparation_v1();
    let recovered_intent = begin_recovered(same_preparation, &mut guard);
    let signed = exact_signed_commit(&fresh_intent, 1_060);
    let fresh_pending = verify_cas_commit_for_test_v1(
        fresh_intent,
        &signed.envelope,
        &signed.observation,
        &mut guard,
    )
    .unwrap();
    assert!(fresh_pending.active_query_may_issue_model());
    let recovered_pending = verify_cas_commit_for_test_v1(
        recovered_intent,
        &signed.envelope,
        &signed.observation,
        &mut guard,
    )
    .unwrap();
    assert!(recovered_pending.active_query_may_issue_model());
    let (fresh_pending, fresh) = reserve_query(fresh_pending, &mut guard);
    let (recovered_pending, recovered_fresh) = reserve_query(recovered_pending, &mut guard);
    assert!(fresh);
    assert!(!recovered_fresh);
    assert_eq!(fresh_pending.0.phase_revision(), 4);
    assert_eq!(recovered_pending.0.phase_revision(), 4);
    assert_eq!(
        fresh_pending.0.active_query_state_sha256(),
        recovered_pending.0.active_query_state_sha256()
    );
    (fresh_pending, recovered_pending, signed, guard)
}

pub(super) fn complete_genesis(
    model_now: u64,
) -> (
    VerifiedCommittedCurrentTipPreparationV1,
    InstallEpochReplayGuardV1,
) {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let (pending, commit) = commit_pending(intent, 1_060, &mut guard);
    let (issued, fresh) = reserve_query(pending, &mut guard);
    assert!(fresh);
    let current_tip = exact_signed_current_tip(&issued, &commit.envelope, 1_060, 1_100);
    let verified = verify_current_tip_for_test_v1(
        issued,
        &current_tip.envelope,
        &current_tip.observation,
        model_now,
        &mut guard,
    )
    .unwrap();
    (verified, guard)
}

pub(super) fn complete_genesis_after_one_retry(
    model_now: u64,
) -> VerifiedCommittedCurrentTipPreparationV1 {
    complete_genesis_after_retries(model_now, 1)
}

pub(super) fn complete_genesis_after_retries(
    model_now: u64,
    retry_count: u64,
) -> VerifiedCommittedCurrentTipPreparationV1 {
    assert!(retry_count <= MAX_EXTERNAL_WATERMARK_CURRENT_TIP_RETRY_COUNT_V1);
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let (pending, commit) = commit_pending(intent, 1_060, &mut guard);
    let (mut issued, fresh) = reserve_query(pending, &mut guard);
    assert!(fresh);
    for retry_index in 1..=retry_count {
        let closure = verified_query_closure_for_test_v1(
            &issued,
            sha256(format!("projection-retry-closure-{retry_index}").as_bytes()),
        );
        let outcome = prepare_external_watermark_current_tip_retry_v1(
            issued,
            closure,
            sha256(format!("projection-retry-query-{retry_index}").as_bytes()),
            &mut guard,
        )
        .unwrap();
        issued = match outcome {
            ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Fresh(reserved) => {
                reserved.into_pending_after_provider_call()
            }
            ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Recovered(_) => {
                panic!("fresh retry unexpectedly recovered an existing query")
            }
        };
    }
    let current_tip = exact_signed_current_tip(&issued, &commit.envelope, 1_060, 1_100);
    verify_current_tip_for_test_v1(
        issued,
        &current_tip.envelope,
        &current_tip.observation,
        model_now,
        &mut guard,
    )
    .unwrap()
}

#[test]
fn one_guard_retains_preparation_claims_and_atomically_adds_completion_pair() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    assert_eq!(guard.claim_count(), 2);
    assert_eq!(guard.bundle_count(), 1);
    assert!(guard.nonce_is_consumed(preparation.authority_nonce()));
    assert!(guard.nonce_is_consumed(preparation.lease_nonce()));

    let intent = begin_fresh(preparation, &mut guard);
    assert_eq!(guard.claim_count(), 4);
    assert_eq!(guard.bundle_count(), 2);
    assert!(guard.nonce_is_consumed(intent.commit_nonce()));
    assert!(guard.nonce_is_consumed(intent.initial_query_nonce()));
}

#[test]
fn partially_persisted_completion_bundle_is_never_recovered_as_exact() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let bundle_id = intent.completion_slot_id_sha256().to_string();
    assert_eq!(guard.claim_count(), 4);
    assert_eq!(guard.bundle_count(), 2);
    guard.remove_bundle_for_test(&bundle_id);
    assert_eq!(guard.bundle_count(), 1);

    let (same_preparation, _) = crate::test_only_genesis_install_epoch_preparation_v1();
    assert!(
        prepare_cas_intent_for_test_v1(
            same_preparation,
            digest(COMMIT_NONCE_CHARACTER),
            digest(INITIAL_QUERY_NONCE_CHARACTER),
            &mut guard,
        )
        .is_err()
    );
    assert_eq!(guard.claim_count(), 4);
    assert_eq!(guard.bundle_count(), 1);
}

#[test]
fn begin_requires_exact_existing_authority_and_lease_claims() {
    let (preparation, _) = crate::test_only_genesis_install_epoch_preparation_v1();
    let mut missing = InstallEpochReplayGuardV1::default();
    assert!(
        prepare_cas_intent_for_test_v1(
            preparation,
            digest(COMMIT_NONCE_CHARACTER),
            digest(INITIAL_QUERY_NONCE_CHARACTER),
            &mut missing,
        )
        .is_err()
    );
    assert_eq!(missing.claim_count(), 0);

    let (preparation, _) = crate::test_only_genesis_install_epoch_preparation_v1();
    let mut wrong = InstallEpochReplayGuardV1::from_consumed_nonces([
        preparation.authority_nonce().to_string(),
        preparation.lease_nonce().to_string(),
    ])
    .unwrap();
    assert!(
        prepare_cas_intent_for_test_v1(
            preparation,
            digest(COMMIT_NONCE_CHARACTER),
            digest(INITIAL_QUERY_NONCE_CHARACTER),
            &mut wrong,
        )
        .is_err()
    );
    assert_eq!(wrong.claim_count(), 2);
}

#[test]
fn begin_error_returns_the_owned_preparation() {
    let (preparation, _) = crate::test_only_genesis_install_epoch_preparation_v1();
    let authority_nonce = preparation.authority_nonce().to_string();
    let lease_nonce = preparation.lease_nonce().to_string();
    let preparation_binding = install_epoch_preparation_binding_sha256_v1(&preparation);
    let mut missing = InstallEpochReplayGuardV1::default();

    let error = prepare_cas_intent_for_test_v1(
        preparation,
        digest(COMMIT_NONCE_CHARACTER),
        digest(INITIAL_QUERY_NONCE_CHARACTER),
        &mut missing,
    )
    .unwrap_err();
    assert_eq!(error.pending().authority_nonce(), authority_nonce);
    assert_eq!(error.pending().lease_nonce(), lease_nonce);
    let returned = error.into_pending();
    assert_eq!(
        install_epoch_preparation_binding_sha256_v1(&returned),
        preparation_binding
    );
    assert_eq!(missing.claim_count(), 0);
    assert_eq!(missing.bundle_count(), 0);
    assert_eq!(missing.phase_head_count(), 0);
}

#[test]
fn deleting_the_preparation_bundle_makes_begin_fail_without_partial_completion_state() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let authority_nonce = preparation.authority_nonce().to_string();
    let preparation_bundle_id = preparation.preparation_bundle_id_sha256();
    guard.remove_bundle_for_test(&preparation_bundle_id);
    assert_eq!(guard.claim_count(), 2);
    assert_eq!(guard.bundle_count(), 0);

    let error = prepare_cas_intent_for_test_v1(
        preparation,
        digest(COMMIT_NONCE_CHARACTER),
        digest(INITIAL_QUERY_NONCE_CHARACTER),
        &mut guard,
    )
    .unwrap_err();
    assert_eq!(error.into_pending().authority_nonce(), authority_nonce);
    assert_eq!(guard.claim_count(), 2);
    assert_eq!(guard.bundle_count(), 0);
    assert_eq!(guard.phase_head_count(), 0);
    assert!(!guard.nonce_is_consumed(&digest(COMMIT_NONCE_CHARACTER)));
    assert!(!guard.nonce_is_consumed(&digest(INITIAL_QUERY_NONCE_CHARACTER)));
}

#[test]
fn commit_or_query_collision_never_partially_writes_the_pair() {
    for (colliding_nonce, untouched_nonce) in [
        (
            digest(COMMIT_NONCE_CHARACTER),
            digest(INITIAL_QUERY_NONCE_CHARACTER),
        ),
        (
            digest(INITIAL_QUERY_NONCE_CHARACTER),
            digest(COMMIT_NONCE_CHARACTER),
        ),
    ] {
        let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
        guard
            .claim_exact_or_replay(&colliding_nonce, "unrelated-prior-scope", &digest('9'))
            .unwrap();
        assert_eq!(guard.claim_count(), 3);
        assert!(
            prepare_cas_intent_for_test_v1(
                preparation,
                digest(COMMIT_NONCE_CHARACTER),
                digest(INITIAL_QUERY_NONCE_CHARACTER),
                &mut guard,
            )
            .is_err()
        );
        assert_eq!(guard.claim_count(), 3);
        assert!(guard.nonce_is_consumed(&colliding_nonce));
        assert!(!guard.nonce_is_consumed(&untouched_nonce));
    }
}

#[test]
fn rev1_begin_requires_a_durable_reservation_before_commit_typestate_exists() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let fresh = prepare_fresh_begin(preparation, &mut guard);
    assert_eq!(guard.claim_count(), 4);
    assert_eq!(fresh.0.phase_revision(), 1);
    assert_eq!(
        guard.phase_head_for_test(fresh.0.phase_head_id_sha256()),
        Some((1, fresh.0.phase_state_sha256()))
    );

    let issued = match fresh.reserve_provider_cas_model(&mut guard).unwrap() {
        ExternalWatermarkCasReservationOutcomeV1::Fresh(reserved) => {
            reserved.into_pending_after_provider_call()
        }
        ExternalWatermarkCasReservationOutcomeV1::Recovered(_) => {
            panic!("first rev1 reservation unexpectedly recovered")
        }
    };
    assert_eq!(issued.phase_revision(), 2);
    assert_ne!(
        issued.phase_state_sha256(),
        issued.cas_intent_state_sha256()
    );
}

#[test]
fn recovered_genesis_can_win_cas_reservation_and_exact_issue_is_idempotent() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let original = prepare_fresh_begin(preparation, &mut guard);

    let (same_preparation, _) = crate::test_only_genesis_install_epoch_preparation_v1();
    let recovered = prepare_cas_intent_for_test_v1(
        same_preparation,
        digest(COMMIT_NONCE_CHARACTER),
        digest(INITIAL_QUERY_NONCE_CHARACTER),
        &mut guard,
    )
    .unwrap();
    let recovered = match recovered {
        ExternalWatermarkCasIntentOutcomeV1::Recovered(recovered) => recovered,
        ExternalWatermarkCasIntentOutcomeV1::Fresh(_) => {
            panic!("exact genesis replay was not recovered")
        }
    };
    let recovered_winner = match recovered.reserve_provider_cas_model(&mut guard).unwrap() {
        ExternalWatermarkCasReservationOutcomeV1::Fresh(reserved) => {
            reserved.into_pending_after_provider_call()
        }
        ExternalWatermarkCasReservationOutcomeV1::Recovered(_) => {
            panic!("recovered genesis did not win the still-open CAS issue edge")
        }
    };
    let original_replay = match original.reserve_provider_cas_model(&mut guard).unwrap() {
        ExternalWatermarkCasReservationOutcomeV1::Recovered(reserved) => {
            reserved.into_pending_for_receipt_reconciliation()
        }
        ExternalWatermarkCasReservationOutcomeV1::Fresh(_) => {
            panic!("exact issue replay incorrectly minted a second fresh permit")
        }
    };
    assert_eq!(recovered_winner.phase_revision(), 2);
    assert_eq!(original_replay.phase_revision(), 2);
    assert_eq!(
        recovered_winner.phase_state_sha256(),
        original_replay.phase_state_sha256()
    );
    assert_eq!(guard.claim_count(), 4);
    assert_eq!(guard.phase_head_count(), 1);
    assert_eq!(
        guard.phase_head_for_test(recovered_winner.phase_head_id_sha256()),
        Some((2, recovered_winner.phase_state_sha256()))
    );
}

#[test]
fn commit_verification_error_returns_the_exact_intent() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let commit_nonce = intent.commit_nonce().to_string();
    let operation = intent.completion_operation_binding_sha256().to_string();
    let mut signed = exact_signed_commit(&intent, 1_060);
    signed.envelope.detached_signature_bytes.push(0);

    let error =
        verify_cas_commit_for_test_v1(intent, &signed.envelope, &signed.observation, &mut guard)
            .unwrap_err();
    assert!(!error.error().to_string().is_empty());
    assert_eq!(error.pending().commit_nonce(), commit_nonce);
    let returned = error.into_pending();
    assert_eq!(returned.completion_operation_binding_sha256(), operation);
    assert_eq!(guard.claim_count(), 4);
}

#[test]
fn missing_phase_head_makes_commit_fail_and_returns_the_intent_without_writes() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let phase_head_id = intent.phase_head_id_sha256().to_string();
    let operation = intent.completion_operation_binding_sha256().to_string();
    let signed = exact_signed_commit(&intent, 1_060);
    guard.remove_phase_head_for_test(&phase_head_id);
    let claims_before = guard.claim_count();
    let bundles_before = guard.bundle_count();

    let error =
        verify_cas_commit_for_test_v1(intent, &signed.envelope, &signed.observation, &mut guard)
            .unwrap_err();
    assert_eq!(
        error.pending().completion_operation_binding_sha256(),
        operation
    );
    assert_eq!(
        error.into_pending().commit_nonce(),
        digest(COMMIT_NONCE_CHARACTER)
    );
    assert_eq!(guard.claim_count(), claims_before);
    assert_eq!(guard.bundle_count(), bundles_before);
    assert_eq!(guard.phase_head_count(), 0);
    assert_eq!(guard.phase_head_for_test(&phase_head_id), None);
}

#[test]
fn exact_cas_receipt_recovery_reuses_one_receipt_edge() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let fresh_intent = begin_fresh(preparation, &mut guard);
    let issue_state = fresh_intent.phase_state_sha256().to_string();
    let phase_head_id = fresh_intent.phase_head_id_sha256().to_string();
    let (same_preparation, _) = crate::test_only_genesis_install_epoch_preparation_v1();
    let recovered_intent = begin_recovered(same_preparation, &mut guard);
    let signed = exact_signed_commit(&fresh_intent, 1_060);

    let first = verify_cas_commit_for_test_v1(
        fresh_intent,
        &signed.envelope,
        &signed.observation,
        &mut guard,
    )
    .unwrap();
    assert!(first.active_query_may_issue_model());
    let receipt_state = first.cas_receipt_state_sha256().to_string();

    let recovered = verify_cas_commit_for_test_v1(
        recovered_intent,
        &signed.envelope,
        &signed.observation,
        &mut guard,
    )
    .unwrap();
    assert!(recovered.active_query_may_issue_model());
    assert_eq!(recovered.phase_revision(), 3);
    assert_eq!(recovered.cas_receipt_state_sha256(), receipt_state);
    assert_eq!(
        guard.phase_head_edge_for_test(&phase_head_id),
        Some((
            Some(2),
            Some(issue_state.as_str()),
            3,
            receipt_state.as_str()
        ))
    );
    assert_eq!(guard.claim_count(), 4);
    assert_eq!(guard.bundle_count(), 2);
    assert_eq!(guard.phase_head_count(), 1);
}

#[test]
fn alternate_valid_cas_receipt_is_rejected_as_a_phase_fork() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let fresh_intent = begin_fresh(preparation, &mut guard);
    let phase_head_id = fresh_intent.phase_head_id_sha256().to_string();
    let (same_preparation, _) = crate::test_only_genesis_install_epoch_preparation_v1();
    let recovered_intent = begin_recovered(same_preparation, &mut guard);
    let first_signed = exact_signed_commit(&fresh_intent, 1_060);
    let mut alternate_challenge = commit_challenge(&recovered_intent, 1_061);
    alternate_challenge.provider_transaction_sha256 = digest('8');
    let alternate_signed = sign_commit(&recovered_intent, alternate_challenge);
    assert_ne!(
        first_signed.envelope.canonical_statement_sha256,
        alternate_signed.envelope.canonical_statement_sha256
    );

    let first = verify_cas_commit_for_test_v1(
        fresh_intent,
        &first_signed.envelope,
        &first_signed.observation,
        &mut guard,
    )
    .unwrap();
    let first_receipt_state = first.cas_receipt_state_sha256().to_string();
    let claims_before_fork = guard.claim_count();
    let bundles_before_fork = guard.bundle_count();

    let error = verify_cas_commit_for_test_v1(
        recovered_intent,
        &alternate_signed.envelope,
        &alternate_signed.observation,
        &mut guard,
    )
    .unwrap_err();
    assert_eq!(
        error.pending().commit_nonce(),
        digest(COMMIT_NONCE_CHARACTER)
    );
    assert_eq!(
        guard.phase_head_for_test(&phase_head_id),
        Some((3, first_receipt_state.as_str()))
    );
    assert_eq!(guard.claim_count(), claims_before_fork);
    assert_eq!(guard.bundle_count(), bundles_before_fork);
    assert_eq!(guard.phase_head_count(), 1);
}

#[test]
fn current_tip_error_returns_pending_and_allows_a_fresh_query_retry() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let (pending, commit) = commit_pending(intent, 1_060, &mut guard);
    let (issued, fresh) = reserve_query(pending, &mut guard);
    assert!(fresh);
    let closure = query_closure(&issued, 'c');
    let mut bad_tip = exact_signed_current_tip(&issued, &commit.envelope, 1_060, 1_100);
    bad_tip.observation = CryptographicSignatureObservation::for_test_only(
        bad_tip.envelope.detached_signature_sha256.clone(),
        bad_tip.envelope.canonical_statement_sha256.clone(),
        SshsigTrustPurposeV8::ExternalWatermarkCommitV1,
    );

    let error = verify_current_tip_for_test_v1(
        issued,
        &bad_tip.envelope,
        &bad_tip.observation,
        1_070,
        &mut guard,
    )
    .unwrap_err();
    assert_eq!(
        error.pending().0.commit_nonce(),
        digest(COMMIT_NONCE_CHARACTER)
    );
    assert!(!error.pending().0.active_query_may_issue_model());
    assert_eq!(guard.claim_count(), 4);

    let retry_nonce = digest('9');
    let retry = prepare_external_watermark_current_tip_retry_v1(
        error.into_pending(),
        closure,
        retry_nonce.clone(),
        &mut guard,
    )
    .unwrap();
    let retry = match retry {
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Fresh(reserved) => {
            reserved.into_pending_after_provider_call()
        }
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Recovered(_) => {
            panic!("fresh retry unexpectedly recovered an old query edge")
        }
    };
    assert_eq!(retry.0.active_query_nonce(), retry_nonce);
    assert!(!retry.0.active_query_may_issue_model());
    assert_eq!(guard.claim_count(), 5);

    let retry_tip = exact_signed_current_tip(&retry, &commit.envelope, 1_071, 1_100);
    let verified = verify_current_tip_for_test_v1(
        retry,
        &retry_tip.envelope,
        &retry_tip.observation,
        1_080,
        &mut guard,
    )
    .unwrap();
    assert_eq!(verified.query_nonce(), retry_nonce);
}

#[test]
fn exact_query_replay_is_reconciliation_only() {
    let (issued, replay_issued, _, mut guard) = duplicate_consumed_initial_query_phase();
    let retry_nonce = digest('9');
    let fresh_closure = query_closure(&issued, 'c');
    let replay_closure = query_closure(&replay_issued, 'c');
    let fresh_retry = prepare_external_watermark_current_tip_retry_v1(
        issued,
        fresh_closure,
        retry_nonce.clone(),
        &mut guard,
    )
    .unwrap();
    let fresh_retry = match fresh_retry {
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Fresh(reserved) => {
            reserved.into_pending_after_provider_call()
        }
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Recovered(_) => {
            panic!("first retry reservation unexpectedly recovered")
        }
    };
    assert!(!fresh_retry.0.active_query_may_issue_model());
    assert_eq!(guard.claim_count(), 5);

    let reconciled = prepare_external_watermark_current_tip_retry_v1(
        replay_issued,
        replay_closure,
        retry_nonce.clone(),
        &mut guard,
    )
    .unwrap();
    let reconciled = match reconciled {
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Recovered(reserved) => {
            reserved.into_pending_for_receipt_reconciliation()
        }
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Fresh(_) => {
            panic!("exact retry replay minted a second fresh query permit")
        }
    };
    assert_eq!(reconciled.0.active_query_nonce(), retry_nonce);
    assert!(!reconciled.0.active_query_may_issue_model());
    assert_eq!(
        reconciled.0.active_query_state_sha256(),
        fresh_retry.0.active_query_state_sha256()
    );
    assert_eq!(guard.claim_count(), 5);
}

#[test]
fn recovered_query_cannot_enter_retry_without_a_typed_closure_argument() {
    type RetryApi = fn(
        IssuedExternalWatermarkCurrentTipQueryV1,
        VerifiedExternalWatermarkCurrentTipQueryClosureV1,
        String,
        &mut InstallEpochReplayGuardV1,
    ) -> Result<
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1,
        CompletionTransitionErrorV1<ExternalWatermarkCurrentTipRetryAttemptV1>,
    >;
    let _: RetryApi = prepare_external_watermark_current_tip_retry_v1;

    let (_, recovered_issued, _, guard) = duplicate_consumed_initial_query_phase();
    assert!(!recovered_issued.0.active_query_may_issue_model());
    assert_eq!(recovered_issued.0.phase_revision(), 4);
    assert_eq!(guard.claim_count(), 4);
    assert_eq!(guard.bundle_count(), 2);
}

#[test]
fn query_closure_mismatch_returns_the_issued_query_without_any_write() {
    let (issued, _, _, mut guard) = duplicate_consumed_initial_query_phase();
    let phase_head_id = issued.0.intent.phase_head_id_sha256().to_string();
    let query_state = issued.0.active_query_state_sha256().to_string();
    let mut closure = query_closure(&issued, 'c');
    closure.query_sequence += 1;
    let closure_evidence = closure.closure_evidence_sha256.clone();
    let claims_before = guard.claim_count();
    let bundles_before = guard.bundle_count();

    let error =
        prepare_external_watermark_current_tip_retry_v1(issued, closure, digest('9'), &mut guard)
            .unwrap_err();
    assert_eq!(error.pending().issued().0.phase_revision(), 4);
    assert_eq!(
        error.pending().issued().0.active_query_state_sha256(),
        query_state
    );
    assert_eq!(error.pending().query_nonce(), digest('9'));
    let (cause, attempt) = error.into_parts();
    assert!(!cause.to_string().is_empty());
    let (returned, returned_closure, returned_nonce) = attempt.into_parts();
    assert_eq!(
        returned.0.active_query_nonce(),
        digest(INITIAL_QUERY_NONCE_CHARACTER)
    );
    assert_eq!(returned_closure.closure_evidence_sha256, closure_evidence);
    assert_eq!(returned_nonce, digest('9'));
    assert_eq!(guard.claim_count(), claims_before);
    assert_eq!(guard.bundle_count(), bundles_before);
    assert!(!guard.nonce_is_consumed(&digest('9')));
    assert_eq!(
        guard.phase_head_for_test(&phase_head_id),
        Some((4, query_state.as_str()))
    );
}

#[test]
fn retry_budget_preserves_the_eighth_query_for_terminal_finalization() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let phase_head_id = intent.phase_head_id_sha256().to_string();
    let (pending, commit) = commit_pending(intent, 1_060, &mut guard);
    let (mut issued, fresh) = reserve_query(pending, &mut guard);
    assert!(fresh);

    for retry_index in 1..=MAX_EXTERNAL_WATERMARK_CURRENT_TIP_RETRY_COUNT_V1 {
        let closure = verified_query_closure_for_test_v1(
            &issued,
            sha256(format!("retry-closure-evidence-{retry_index}").as_bytes()),
        );
        let query_nonce = sha256(format!("retry-query-nonce-{retry_index}").as_bytes());
        let outcome = prepare_external_watermark_current_tip_retry_v1(
            issued,
            closure,
            query_nonce.clone(),
            &mut guard,
        )
        .unwrap();
        issued = match outcome {
            ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Fresh(reserved) => {
                reserved.into_pending_after_provider_call()
            }
            ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Recovered(_) => {
                panic!("fresh bounded retry unexpectedly recovered an old query edge")
            }
        };
        assert_eq!(issued.active_query_sequence(), retry_index + 1);
        assert_eq!(issued.active_query_nonce(), query_nonce);
    }

    let terminal_query_nonce = issued.active_query_nonce().to_string();
    let terminal_query_state = issued.active_query_state_sha256().to_string();
    let terminal_phase_revision = issued.phase_revision();
    let claims_before_ninth = guard.claim_count();
    let bundles_before_ninth = guard.bundle_count();
    let ninth_closure =
        verified_query_closure_for_test_v1(&issued, sha256(b"retry-closure-evidence-over-budget"));
    let ninth_closure_binding = ninth_closure.closure_binding_sha256.clone();
    let ninth_nonce = sha256(b"retry-query-nonce-over-budget");
    let error = prepare_external_watermark_current_tip_retry_v1(
        issued,
        ninth_closure,
        ninth_nonce.clone(),
        &mut guard,
    )
    .unwrap_err();
    assert_eq!(
        error.pending().issued().active_query_nonce(),
        terminal_query_nonce
    );
    assert_eq!(
        error.pending().issued().active_query_state_sha256(),
        terminal_query_state
    );
    assert_eq!(
        error.pending().issued().phase_revision(),
        terminal_phase_revision
    );
    assert_eq!(error.pending().query_nonce(), ninth_nonce);
    assert_eq!(guard.claim_count(), claims_before_ninth);
    assert_eq!(guard.bundle_count(), bundles_before_ninth);
    assert!(!guard.nonce_is_consumed(&ninth_nonce));
    assert_eq!(
        guard.phase_head_for_test(&phase_head_id),
        Some((terminal_phase_revision, terminal_query_state.as_str()))
    );

    let (_, attempt) = error.into_parts();
    let (issued, returned_closure, returned_nonce) = attempt.into_parts();
    assert_eq!(
        returned_closure.closure_binding_sha256,
        ninth_closure_binding
    );
    assert_eq!(returned_nonce, ninth_nonce);
    let tip = exact_signed_current_tip(&issued, &commit.envelope, 1_080, 1_100);
    let verified =
        verify_current_tip_for_test_v1(issued, &tip.envelope, &tip.observation, 1_090, &mut guard)
            .unwrap();
    assert_eq!(verified.query_nonce(), terminal_query_nonce);
    assert_eq!(verified.final_phase_revision(), terminal_phase_revision + 1);
    assert!(verified.final_phase_was_fresh_model());
}

#[test]
fn exact_genesis_completion_produces_only_an_inert_model_token() {
    let (verified, guard) = complete_genesis(1_070);
    assert!(verified.provider_exact_cas_committed_model());
    assert!(verified.provider_current_tip_attested_model());
    assert_eq!(verified.successor_record().successor_revision, 1);
    assert_eq!(verified.commit_nonce(), digest(COMMIT_NONCE_CHARACTER));
    assert_eq!(
        verified.query_nonce(),
        digest(INITIAL_QUERY_NONCE_CHARACTER)
    );
    assert_eq!(verified.current_tip_issued_at_unix_seconds(), 1_060);
    assert_eq!(verified.current_tip_expires_at_unix_seconds(), 1_100);
    assert_eq!(verified.model_completed_at_unix_seconds(), 1_070);
    assert_eq!(
        verified.successor_tip_sha256(),
        external_watermark_record_sha256_v1(verified.successor_record())
    );
    assert!(!verified.actual_host_verified());
    assert!(!verified.trusted_time_verified());
    assert!(!verified.durable_global_nonce_claimed());
    assert!(!verified.root_install_execution_allowed());
    assert!(!verified.daemon_reload_enable_or_start_allowed());
    assert!(!verified.trusted_state_root_established());
    assert!(!verified.fresh_attempt_allowed());
    assert_eq!(guard.claim_count(), 4);
}

#[test]
fn durable_phase_digests_form_three_distinct_nonzero_states() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let intent_state = intent.cas_intent_state_sha256().to_string();
    let (pending, commit) = commit_pending(intent, 1_060, &mut guard);
    let receipt_state = pending.cas_receipt_state_sha256().to_string();
    let (issued, fresh) = reserve_query(pending, &mut guard);
    assert!(fresh);
    let tip = exact_signed_current_tip(&issued, &commit.envelope, 1_060, 1_100);
    let verified =
        verify_current_tip_for_test_v1(issued, &tip.envelope, &tip.observation, 1_070, &mut guard)
            .unwrap();
    let finalized_state = verified.finalized_state_sha256().to_string();

    for state in [&intent_state, &receipt_state, &finalized_state] {
        assert_eq!(state.len(), 64);
        assert!(state.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_ne!(state, &"0".repeat(64));
    }
    assert_ne!(intent_state, receipt_state);
    assert_ne!(receipt_state, finalized_state);
    assert_ne!(intent_state, finalized_state);
}

#[test]
fn phase_head_advances_intent_receipt_query_retry_and_final_in_order() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let phase_head_id = intent.phase_head_id_sha256().to_string();
    let intent_state = intent.cas_intent_state_sha256().to_string();
    let issue_state = intent.phase_state_sha256().to_string();
    assert_eq!(
        guard.phase_head_edge_for_test(&phase_head_id),
        Some((
            Some(1),
            Some(intent_state.as_str()),
            2,
            issue_state.as_str()
        ))
    );

    let (pending, commit) = commit_pending(intent, 1_060, &mut guard);
    let receipt_state = pending.cas_receipt_state_sha256().to_string();
    assert_eq!(pending.phase_revision(), 3);
    assert_eq!(
        guard.phase_head_edge_for_test(&phase_head_id),
        Some((
            Some(2),
            Some(issue_state.as_str()),
            3,
            receipt_state.as_str()
        ))
    );

    let (issued, fresh) = reserve_query(pending, &mut guard);
    assert!(fresh);
    let initial_query_state = issued.0.active_query_state_sha256().to_string();
    assert_eq!(issued.0.phase_revision(), 4);
    assert_eq!(
        guard.phase_head_edge_for_test(&phase_head_id),
        Some((
            Some(3),
            Some(receipt_state.as_str()),
            4,
            initial_query_state.as_str(),
        ))
    );

    let closure = query_closure(&issued, 'c');
    let retry =
        prepare_external_watermark_current_tip_retry_v1(issued, closure, digest('9'), &mut guard)
            .unwrap();
    let retry = match retry {
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Fresh(reserved) => {
            reserved.into_pending_after_provider_call()
        }
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Recovered(_) => {
            panic!("first retry edge unexpectedly recovered")
        }
    };
    let retry_state = retry.0.active_query_state_sha256().to_string();
    assert_eq!(retry.0.phase_revision(), 5);
    assert!(!retry.0.active_query_may_issue_model());
    assert_eq!(
        guard.phase_head_edge_for_test(&phase_head_id),
        Some((
            Some(4),
            Some(initial_query_state.as_str()),
            5,
            retry_state.as_str(),
        ))
    );
    let tip = exact_signed_current_tip(&retry, &commit.envelope, 1_071, 1_100);
    let verified =
        verify_current_tip_for_test_v1(retry, &tip.envelope, &tip.observation, 1_080, &mut guard)
            .unwrap();
    assert_eq!(verified.final_phase_revision(), 6);
    assert!(verified.final_phase_was_fresh_model());
    assert_eq!(
        guard.phase_head_edge_for_test(&phase_head_id),
        Some((
            Some(5),
            Some(retry_state.as_str()),
            6,
            verified.finalized_state_sha256(),
        ))
    );
}

#[test]
fn query_retry_fork_rolls_back_the_token_and_writes_nothing_partial() {
    let (issued, replay_issued, _, mut guard) = duplicate_consumed_initial_query_phase();
    let phase_head_id = issued.0.intent.phase_head_id_sha256().to_string();
    let stale_query_state = issued.0.active_query_state_sha256().to_string();
    let fresh_closure = query_closure(&issued, 'c');
    let fork_closure = query_closure(&replay_issued, 'c');
    let fresh_retry = prepare_external_watermark_current_tip_retry_v1(
        issued,
        fresh_closure,
        digest('9'),
        &mut guard,
    )
    .unwrap();
    let fresh_retry = match fresh_retry {
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Fresh(reserved) => {
            reserved.into_pending_after_provider_call()
        }
        ExternalWatermarkCurrentTipQueryReservationOutcomeV1::Recovered(_) => {
            panic!("first retry edge unexpectedly recovered")
        }
    };
    let retry_state = fresh_retry.0.active_query_state_sha256().to_string();
    let claims_before_fork = guard.claim_count();
    let bundles_before_fork = guard.bundle_count();

    let error = prepare_external_watermark_current_tip_retry_v1(
        replay_issued,
        fork_closure,
        digest('8'),
        &mut guard,
    )
    .unwrap_err();
    assert_eq!(error.pending().issued().0.phase_revision(), 4);
    assert_eq!(
        error.pending().issued().0.active_query_state_sha256(),
        stale_query_state
    );
    assert_eq!(error.pending().query_nonce(), digest('8'));
    let (_, attempt) = error.into_parts();
    let (returned, _closure, returned_nonce) = attempt.into_parts();
    assert_eq!(
        returned.0.active_query_nonce(),
        digest(INITIAL_QUERY_NONCE_CHARACTER)
    );
    assert_eq!(returned_nonce, digest('8'));
    assert_eq!(guard.claim_count(), claims_before_fork);
    assert_eq!(guard.bundle_count(), bundles_before_fork);
    assert!(!guard.nonce_is_consumed(&digest('8')));
    assert_eq!(
        guard.phase_head_for_test(&phase_head_id),
        Some((5, retry_state.as_str()))
    );
}

#[test]
fn same_tip_with_different_valid_model_times_exactly_recovers_one_final_state() {
    let (issued, replay_issued, commit, mut guard) = duplicate_consumed_initial_query_phase();
    let phase_head_id = issued.0.intent.phase_head_id_sha256().to_string();
    let tip = exact_signed_current_tip(&issued, &commit.envelope, 1_060, 1_100);
    let first =
        verify_current_tip_for_test_v1(issued, &tip.envelope, &tip.observation, 1_070, &mut guard)
            .unwrap();
    assert!(first.final_phase_was_fresh_model());
    let finalized_state = first.finalized_state_sha256().to_string();

    let recovered = verify_current_tip_for_test_v1(
        replay_issued,
        &tip.envelope,
        &tip.observation,
        1_071,
        &mut guard,
    )
    .unwrap();
    assert!(!recovered.final_phase_was_fresh_model());
    assert!(recovered.requires_read_only_reconciliation_model());
    assert_eq!(first.model_completed_at_unix_seconds(), 1_070);
    assert_eq!(recovered.model_completed_at_unix_seconds(), 1_071);
    assert_eq!(recovered.finalized_state_sha256(), finalized_state);
    assert_eq!(
        guard.phase_head_for_test(&phase_head_id),
        Some((5, finalized_state.as_str()))
    );
}

#[test]
fn exact_successor_advances_n_to_n_plus_one() {
    let (preparation, mut guard) = crate::test_only_successor_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let (pending, commit) = commit_pending(intent, 1_060, &mut guard);
    let (issued, fresh) = reserve_query(pending, &mut guard);
    assert!(fresh);
    let tip = exact_signed_current_tip(&issued, &commit.envelope, 1_061, 1_100);
    let verified =
        verify_current_tip_for_test_v1(issued, &tip.envelope, &tip.observation, 1_070, &mut guard)
            .unwrap();
    assert_eq!(
        predecessor_revision(&verified.successor_record().predecessor),
        3
    );
    assert_eq!(verified.successor_record().successor_revision, 4);
}

#[test]
fn unpublished_production_profile_fails_closed_before_new_claims() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    assert_eq!(guard.claim_count(), 2);
    assert!(
        prepare_external_watermark_cas_intent_v1(
            preparation,
            digest(COMMIT_NONCE_CHARACTER),
            digest(INITIAL_QUERY_NONCE_CHARACTER),
            &mut guard,
        )
        .is_err()
    );
    assert_eq!(guard.claim_count(), 2);
    assert!(!guard.nonce_is_consumed(&digest(COMMIT_NONCE_CHARACTER)));
    assert!(!guard.nonce_is_consumed(&digest(INITIAL_QUERY_NONCE_CHARACTER)));
}

#[test]
fn caller_supplied_historical_model_now_remains_explicitly_model_only() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_at_v1(1_050);
    let intent = begin_fresh(preparation, &mut guard);
    let (pending, commit) = commit_pending(intent, 1_060, &mut guard);
    let (issued, fresh) = reserve_query(pending, &mut guard);
    assert!(fresh);
    let tip = exact_signed_current_tip(&issued, &commit.envelope, 1_060, 1_100);
    let verified =
        verify_current_tip_for_test_v1(issued, &tip.envelope, &tip.observation, 1_065, &mut guard)
            .unwrap();
    assert_eq!(
        verified.preparation().model_verified_at_unix_seconds(),
        1_050
    );
    assert_eq!(verified.model_completed_at_unix_seconds(), 1_065);
    assert!(!verified.trusted_time_verified());
    assert!(!verified.actual_host_verified());
}

#[test]
fn time_windows_are_half_open_and_transition_failures_retain_state() {
    for committed_at in [1_049, 1_110] {
        let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
        let intent = begin_fresh(preparation, &mut guard);
        let challenge = commit_challenge(&intent, committed_at);
        assert!(
            canonical_commit_statement_with_profile_v1(
                &intent.0,
                &challenge,
                &commit_trust(),
                &current_tip_trust(),
                &intent.0.profile,
            )
            .is_err()
        );
    }

    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let (pending, commit) = commit_pending(intent, 1_060, &mut guard);
    let (issued, fresh) = reserve_query(pending, &mut guard);
    assert!(fresh);
    let tip = exact_signed_current_tip(&issued, &commit.envelope, 1_060, 1_100);
    let error =
        verify_current_tip_for_test_v1(issued, &tip.envelope, &tip.observation, 1_100, &mut guard)
            .unwrap_err();
    assert_eq!(error.pending().0.committed_at_unix_seconds(), 1_060);
    assert_eq!(error.into_pending().0.active_query_nonce(), digest('7'));
}

#[test]
fn deterministic_record_hash_changes_with_every_record_field() {
    let (preparation, _) = crate::test_only_genesis_install_epoch_preparation_v1();
    let profile =
        test_only_completion_profile_v1(&preparation, &commit_trust(), &current_tip_trust());
    let base = expected_external_watermark_record_v1(&preparation, &profile);
    let base_hash = external_watermark_record_sha256_v1(&base);
    let mut variants = Vec::new();

    let mut value = base.clone();
    value.completion_profile_sha256 = digest('8');
    variants.push(value);
    let mut value = base.clone();
    value.prepared_epoch_binding_sha256 = digest('8');
    variants.push(value);
    let mut value = base.clone();
    value.machine_id_sha256 = digest('8');
    variants.push(value);
    let mut value = base.clone();
    value.preparation_binding_sha256 = digest('8');
    variants.push(value);
    let mut value = base.clone();
    value.provider_profile_sha256 = digest('8');
    variants.push(value);
    let mut value = base.clone();
    value.state_root_profile_sha256 = digest('8');
    variants.push(value);
    let mut value = base.clone();
    value.stream_id_sha256 = digest('8');
    variants.push(value);
    let mut value = base.clone();
    value.successor_revision += 1;
    variants.push(value);
    let mut value = base;
    if let ExternalWatermarkPredecessorV1::GenesisPinnedSentinel { tip_sha256, .. } =
        &mut value.predecessor
    {
        *tip_sha256 = digest('8');
    }
    variants.push(value);

    for variant in variants {
        assert_ne!(external_watermark_record_sha256_v1(&variant), base_hash);
    }
}

#[test]
fn preparation_and_prepared_epoch_bindings_include_time_and_predecessor() {
    let (first, _) = crate::test_only_genesis_install_epoch_preparation_at_v1(1_050);
    let (later, _) = crate::test_only_genesis_install_epoch_preparation_at_v1(1_051);
    let (successor, _) = crate::test_only_successor_install_epoch_preparation_v1();
    assert_ne!(
        install_epoch_preparation_binding_sha256_v1(&first),
        install_epoch_preparation_binding_sha256_v1(&later)
    );
    assert_ne!(
        prepared_epoch_binding_sha256_v1(&first),
        prepared_epoch_binding_sha256_v1(&later)
    );
    assert_ne!(
        install_epoch_preparation_binding_sha256_v1(&first),
        install_epoch_preparation_binding_sha256_v1(&successor)
    );
}

#[test]
fn commit_bound_field_mutations_fail_and_transaction_changes_the_statement() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    for mutate in [
        |challenge: &mut ExternalWatermarkCommitChallengeV1| {
            challenge.completion_operation_binding_sha256 = digest('8')
        },
        |challenge: &mut ExternalWatermarkCommitChallengeV1| {
            challenge.successor_tip_sha256 = digest('8')
        },
        |challenge: &mut ExternalWatermarkCommitChallengeV1| {
            challenge.successor_record.prepared_epoch_binding_sha256 = digest('8')
        },
        |challenge: &mut ExternalWatermarkCommitChallengeV1| {
            challenge.successor_record.successor_revision += 1
        },
        |challenge: &mut ExternalWatermarkCommitChallengeV1| {
            challenge.lease_signature_sha256 = digest('8')
        },
    ] as [fn(&mut ExternalWatermarkCommitChallengeV1); 5]
    {
        let mut challenge = commit_challenge(&intent, 1_060);
        mutate(&mut challenge);
        assert!(
            canonical_commit_statement_with_profile_v1(
                &intent.0,
                &challenge,
                &commit_trust(),
                &current_tip_trust(),
                &intent.0.profile,
            )
            .is_err()
        );
    }

    let original = commit_challenge(&intent, 1_060);
    let original_statement = canonical_commit_statement_with_profile_v1(
        &intent.0,
        &original,
        &commit_trust(),
        &current_tip_trust(),
        &intent.0.profile,
    )
    .unwrap();
    let mut changed_transaction = original;
    changed_transaction.provider_transaction_sha256 = digest('8');
    let changed_statement = canonical_commit_statement_with_profile_v1(
        &intent.0,
        &changed_transaction,
        &commit_trust(),
        &current_tip_trust(),
        &intent.0.profile,
    )
    .unwrap();
    assert_ne!(sha256(&original_statement), sha256(&changed_statement));
}

#[test]
fn current_tip_must_equal_the_exact_commit_operation_and_host() {
    for mutate in [
        |challenge: &mut ExternalWatermarkCurrentTipChallengeV1| {
            challenge.current_tip_sha256 = digest('8')
        },
        |challenge: &mut ExternalWatermarkCurrentTipChallengeV1| challenge.current_revision += 1,
        |challenge: &mut ExternalWatermarkCurrentTipChallengeV1| {
            challenge.current_record.prepared_epoch_binding_sha256 = digest('8')
        },
        |challenge: &mut ExternalWatermarkCurrentTipChallengeV1| {
            challenge.commit_statement_sha256 = digest('8')
        },
        |challenge: &mut ExternalWatermarkCurrentTipChallengeV1| {
            challenge.provider_transaction_sha256 = digest('8')
        },
        |challenge: &mut ExternalWatermarkCurrentTipChallengeV1| {
            challenge.target_host.machine_id_sha256 = digest('8')
        },
    ] as [fn(&mut ExternalWatermarkCurrentTipChallengeV1); 6]
    {
        let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
        let intent = begin_fresh(preparation, &mut guard);
        let (pending, commit) = commit_pending(intent, 1_060, &mut guard);
        let (issued, fresh) = reserve_query(pending, &mut guard);
        assert!(fresh);
        let mut challenge = current_tip_challenge(&issued, &commit.envelope, 1_060, 1_100);
        mutate(&mut challenge);
        assert!(
            canonical_current_tip_statement_with_profile_v1(
                &issued.0,
                &challenge,
                &commit_trust(),
                &current_tip_trust(),
                &issued.0.intent.profile,
            )
            .is_err()
        );
    }
}

#[test]
fn policy_digests_are_pairwise_distinct() {
    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let challenge = commit_challenge(&intent, 1_060);
    let commit_trust = commit_trust();
    let tip_trust = current_tip_trust();
    let base = intent.0.profile.clone();

    let mut same_as_lease = base.clone();
    same_as_lease.commit_trust_policy_sha256 = same_as_lease.lease_trust_policy_sha256.clone();
    same_as_lease.profile_sha256 = completion_profile_sha256(&same_as_lease);
    assert!(
        canonical_commit_statement_with_profile_v1(
            &intent.0,
            &challenge,
            &commit_trust,
            &tip_trust,
            &same_as_lease,
        )
        .is_err()
    );

    let mut same_as_commit = base;
    same_as_commit.current_tip_trust_policy_sha256 =
        same_as_commit.commit_trust_policy_sha256.clone();
    same_as_commit.profile_sha256 = completion_profile_sha256(&same_as_commit);
    assert!(
        canonical_commit_statement_with_profile_v1(
            &intent.0,
            &challenge,
            &commit_trust,
            &tip_trust,
            &same_as_commit,
        )
        .is_err()
    );
}

#[test]
fn pairwise_nonce_domain_and_unknown_boolean_claims_fail_closed() {
    for (commit_nonce, query_nonce) in [
        (digest('a'), digest(INITIAL_QUERY_NONCE_CHARACTER)),
        (digest('b'), digest(INITIAL_QUERY_NONCE_CHARACTER)),
        (digest(COMMIT_NONCE_CHARACTER), digest('a')),
        (digest(COMMIT_NONCE_CHARACTER), digest('b')),
        (
            digest(COMMIT_NONCE_CHARACTER),
            digest(COMMIT_NONCE_CHARACTER),
        ),
    ] {
        let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
        let authority = preparation.authority_nonce().to_string();
        let lease = preparation.lease_nonce().to_string();
        let (commit_nonce, query_nonce) = match (commit_nonce.as_str(), query_nonce.as_str()) {
            (value, _) if value == digest('a') => (authority, query_nonce),
            (value, _) if value == digest('b') => (lease, query_nonce),
            (_, value) if value == digest('a') => (commit_nonce, authority),
            (_, value) if value == digest('b') => (commit_nonce, lease),
            _ => (commit_nonce, query_nonce),
        };
        assert!(
            prepare_cas_intent_for_test_v1(preparation, commit_nonce, query_nonce, &mut guard)
                .is_err()
        );
        assert_eq!(guard.claim_count(), 2);
    }

    let (preparation, mut guard) = crate::test_only_genesis_install_epoch_preparation_v1();
    let intent = begin_fresh(preparation, &mut guard);
    let signed = exact_signed_commit(&intent, 1_060);
    let mut value = serde_json::to_value(&signed.envelope.challenge).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("cas_succeeded".to_string(), serde_json::json!(true));
    assert!(serde_json::from_value::<ExternalWatermarkCommitChallengeV1>(value).is_err());
}
