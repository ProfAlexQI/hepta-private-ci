use pretty_assertions::assert_eq;
use sha2::Digest as _;

use super::*;
use crate::RootStateIdentityV8;

fn digest(character: char) -> String {
    character.to_string().repeat(64)
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

fn authority_trust() -> VerifiedTrustPolicyBindingV8 {
    crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::InstallEpochAuthorityV1)
}

fn lease_trust() -> VerifiedTrustPolicyBindingV8 {
    crate::test_only_trust_binding_v8(SshsigTrustPurposeV8::ExternalWatermarkLeaseV1)
}

fn signer(trust: &VerifiedTrustPolicyBindingV8) -> AuthoritySignerBindingV8 {
    AuthoritySignerBindingV8 {
        allowed_signers_sha256: trust.allowed_signers_sha256().to_string(),
        key_fingerprint: trust.key_fingerprint().to_string(),
        principal: trust.principal().to_string(),
        signature_algorithm: trust.signature_algorithm(),
    }
}

fn root_file(path: &str, character: char, mode: u32) -> RootFileInstallIdentityV8 {
    RootFileInstallIdentityV8 {
        content_sha256: digest(character),
        gid: 0,
        mode,
        path: path.to_string(),
        size_bytes: 4096,
        uid: 0,
    }
}

fn inventory(state: &StateRootProfileBindingV1) -> ExactRootInstallInventoryV8 {
    ExactRootInstallInventoryV8 {
        admissiond_binary: root_file(crate::ADMISSIOND_INSTALL_PATH_V8, '1', 0o555),
        admissiond_unit: root_file(crate::ADMISSIOND_UNIT_PATH_V8, '2', 0o444),
        recovery_binary: root_file(crate::RECOVERY_INSTALL_PATH_V8, '3', 0o555),
        recovery_unit: root_file(crate::RECOVERY_UNIT_PATH_V8, '4', 0o444),
        state_root: RootStateIdentityV8 {
            gid: state.gid,
            layout_manifest_sha256: state.layout_manifest_sha256.clone(),
            mode: state.mode,
            path: state.path.clone(),
            uid: state.uid,
        },
    }
}

fn host() -> TargetHostBindingV8 {
    TargetHostBindingV8 {
        machine_id_sha256: digest('c'),
    }
}

fn genesis_predecessor(
    state: &StateRootProfileBindingV1,
    provider: &ExternalWatermarkProviderProfileV1,
    host: &TargetHostBindingV8,
) -> ExternalWatermarkPredecessorV1 {
    ExternalWatermarkPredecessorV1::GenesisPinnedSentinel {
        genesis_epoch_binding_sha256: provider.genesis_epoch_binding_sha256.clone(),
        provider_profile_sha256: provider.profile_sha256.clone(),
        revision: 0,
        stream_id_sha256: external_watermark_stream_id_sha256(
            &host.machine_id_sha256,
            &state.profile_sha256,
            &provider.profile_sha256,
        )
        .unwrap(),
        tip_sha256: provider.genesis_tip_sha256.clone(),
    }
}

fn successor_predecessor(
    state: &StateRootProfileBindingV1,
    provider: &ExternalWatermarkProviderProfileV1,
    host: &TargetHostBindingV8,
    sequence: u64,
) -> ExternalWatermarkPredecessorV1 {
    ExternalWatermarkPredecessorV1::Successor {
        installed_epoch_binding_sha256: digest('7'),
        installed_epoch_sequence: sequence,
        provider_profile_sha256: provider.profile_sha256.clone(),
        revision: sequence,
        stream_id_sha256: external_watermark_stream_id_sha256(
            &host.machine_id_sha256,
            &state.profile_sha256,
            &provider.profile_sha256,
        )
        .unwrap(),
        tip_sha256: digest('9'),
    }
}

fn authority_challenge(
    predecessor: ExternalWatermarkPredecessorV1,
    epoch_sequence: u64,
) -> InstallEpochAuthorityChallengeV1 {
    let state = test_only_state_root_profile_v1();
    InstallEpochAuthorityChallengeV1 {
        activation:
            InstallEpochActivationV1::InstallFilesAndCreateInertStateRootOnlyNoReloadEnableStartOrExecution,
        authority_nonce: digest('a'),
        epoch: InstallEpochBindingV1 {
            epoch_nonce_sha256: digest('b'),
            epoch_sequence,
        },
        expires_at_unix_seconds: 1_800,
        install_inventory: inventory(&state),
        issued_at_unix_seconds: 1_000,
        namespace: INSTALL_EPOCH_AUTHORITY_NAMESPACE_V1.to_string(),
        predecessor,
        schema: INSTALL_EPOCH_AUTHORITY_SCHEMA_V1.to_string(),
        signer: signer(&authority_trust()),
        state_root_profile: state,
        target_host: host(),
    }
}

fn sign_authority(
    challenge: InstallEpochAuthorityChallengeV1,
) -> (
    SignedInstallEpochAuthorityV1,
    CryptographicSignatureObservation,
) {
    let authority_trust = authority_trust();
    let lease_trust = lease_trust();
    let state = test_only_state_root_profile_v1();
    let provider = test_only_external_watermark_provider_profile_v1(&lease_trust);
    let statement = canonical_install_epoch_authority_statement_with_trust_v1(
        &challenge,
        &authority_trust,
        &state,
        &provider,
    )
    .unwrap();
    let statement_sha256 = sha256(&statement);
    let signature_bytes = b"test-install-epoch-authority-signature".to_vec();
    let signature_sha256 = sha256(&signature_bytes);
    let observation = CryptographicSignatureObservation::for_test_only(
        signature_sha256.clone(),
        statement_sha256.clone(),
        SshsigTrustPurposeV8::InstallEpochAuthorityV1,
    );
    (
        SignedInstallEpochAuthorityV1 {
            canonical_statement_sha256: statement_sha256,
            challenge,
            detached_signature_bytes: signature_bytes,
            detached_signature_sha256: signature_sha256,
        },
        observation,
    )
}

fn lease_challenge(authority: &SignedInstallEpochAuthorityV1) -> ExternalWatermarkLeaseChallengeV1 {
    let lease_trust = lease_trust();
    ExternalWatermarkLeaseChallengeV1 {
        authority_nonce: authority.challenge.authority_nonce.clone(),
        epoch: authority.challenge.epoch.clone(),
        expires_at_unix_seconds: 1_110,
        install_authority_statement_sha256: authority.canonical_statement_sha256.clone(),
        install_authority_trust_policy_sha256: authority_trust().policy_sha256().to_string(),
        issued_at_unix_seconds: 1_000,
        lease_nonce: digest('d'),
        namespace: EXTERNAL_WATERMARK_LEASE_NAMESPACE_V1.to_string(),
        predecessor: authority.challenge.predecessor.clone(),
        provider_trust_policy_sha256: lease_trust.policy_sha256().to_string(),
        reserved_successor_revision: authority.challenge.predecessor.revision() + 1,
        schema: EXTERNAL_WATERMARK_LEASE_SCHEMA_V1.to_string(),
        signer: signer(&lease_trust),
        state_root_profile_sha256: authority
            .challenge
            .state_root_profile
            .profile_sha256
            .clone(),
        target_host: authority.challenge.target_host.clone(),
    }
}

fn sign_lease(
    challenge: ExternalWatermarkLeaseChallengeV1,
) -> (
    SignedExternalWatermarkLeaseV1,
    CryptographicSignatureObservation,
) {
    let trust = lease_trust();
    let state = test_only_state_root_profile_v1();
    let provider = test_only_external_watermark_provider_profile_v1(&trust);
    let statement = canonical_external_watermark_lease_statement_with_trust_v1(
        &challenge, &trust, &state, &provider,
    )
    .unwrap();
    let statement_sha256 = sha256(&statement);
    let signature_bytes = b"test-external-watermark-lease-signature".to_vec();
    let signature_sha256 = sha256(&signature_bytes);
    let observation = CryptographicSignatureObservation::for_test_only(
        signature_sha256.clone(),
        statement_sha256.clone(),
        SshsigTrustPurposeV8::ExternalWatermarkLeaseV1,
    );
    (
        SignedExternalWatermarkLeaseV1 {
            canonical_statement_sha256: statement_sha256,
            challenge,
            detached_signature_bytes: signature_bytes,
            detached_signature_sha256: signature_sha256,
        },
        observation,
    )
}

#[derive(Clone)]
struct SignedPair {
    authority: SignedInstallEpochAuthorityV1,
    authority_observation: CryptographicSignatureObservation,
    lease: SignedExternalWatermarkLeaseV1,
    lease_observation: CryptographicSignatureObservation,
}

fn signed_pair_with_predecessor(
    predecessor: ExternalWatermarkPredecessorV1,
    epoch_sequence: u64,
) -> SignedPair {
    let (authority, authority_observation) =
        sign_authority(authority_challenge(predecessor, epoch_sequence));
    let (lease, lease_observation) = sign_lease(lease_challenge(&authority));
    SignedPair {
        authority,
        authority_observation,
        lease,
        lease_observation,
    }
}

fn genesis_pair() -> SignedPair {
    let state = test_only_state_root_profile_v1();
    let lease_trust = lease_trust();
    let provider = test_only_external_watermark_provider_profile_v1(&lease_trust);
    signed_pair_with_predecessor(genesis_predecessor(&state, &provider, &host()), 1)
}

fn successor_pair() -> SignedPair {
    let state = test_only_state_root_profile_v1();
    let lease_trust = lease_trust();
    let provider = test_only_external_watermark_provider_profile_v1(&lease_trust);
    signed_pair_with_predecessor(successor_predecessor(&state, &provider, &host(), 3), 4)
}

fn verify(
    pair: &SignedPair,
    now: u64,
    guard: &mut InstallEpochReplayGuardV1,
) -> Result<VerifiedInstallEpochPreparationV1, QualificationError> {
    verify_install_epoch_preparation_for_test_v1(
        &pair.authority,
        &pair.lease,
        &pair.authority_observation,
        &pair.lease_observation,
        now,
        guard,
    )
}

#[test]
fn exact_genesis_dual_trust_binding_only_grants_inert_install_preparation() {
    let pair = genesis_pair();
    let mut guard = InstallEpochReplayGuardV1::default();
    let verified = verify(&pair, 1_050, &mut guard).unwrap();
    assert!(verified.model_preparation_verified());
    assert!(!verified.root_install_execution_allowed());
    assert!(!verified.daemon_reload_enable_or_start_allowed());
    assert!(!verified.trusted_state_root_established());
    assert!(!verified.fresh_attempt_allowed());
    assert_eq!(verified.epoch().epoch_sequence, 1);
    assert_eq!(verified.reserved_successor_revision(), 1);
    assert!(guard.nonce_is_consumed(&pair.authority.challenge.authority_nonce));
    assert!(guard.nonce_is_consumed(&pair.lease.challenge.lease_nonce));
}

#[test]
fn exact_successor_requires_one_shared_monotonic_epoch_and_external_revision() {
    let pair = successor_pair();
    let verified = verify(&pair, 1_050, &mut InstallEpochReplayGuardV1::default()).unwrap();
    assert_eq!(verified.epoch().epoch_sequence, 4);
    assert_eq!(verified.predecessor().installed_epoch_sequence(), 3);
    assert_eq!(verified.predecessor().revision(), 3);
    assert_eq!(verified.reserved_successor_revision(), 4);
}

#[test]
fn unpublished_production_profiles_fail_closed_without_consuming_nonces() {
    let pair = genesis_pair();
    let mut guard = InstallEpochReplayGuardV1::default();
    assert!(
        verify_install_epoch_preparation_v1(&pair.authority, &pair.lease, 1_050, &mut guard)
            .is_err()
    );
    assert!(!guard.nonce_is_consumed(&pair.authority.challenge.authority_nonce));
    assert!(!guard.nonce_is_consumed(&pair.lease.challenge.lease_nonce));
}

#[test]
fn state_root_provider_and_stream_bindings_cannot_be_self_described() {
    let pair = genesis_pair();
    let state = test_only_state_root_profile_v1();
    let provider = test_only_external_watermark_provider_profile_v1(&lease_trust());
    let trust = authority_trust();

    let mut challenge = pair.authority.challenge.clone();
    challenge.state_root_profile.profile_revision += 1;
    challenge.state_root_profile.profile_sha256 =
        state_root_profile_sha256(&challenge.state_root_profile);
    assert!(
        canonical_install_epoch_authority_statement_with_trust_v1(
            &challenge, &trust, &state, &provider,
        )
        .is_err()
    );

    let mut challenge = pair.authority.challenge.clone();
    if let ExternalWatermarkPredecessorV1::GenesisPinnedSentinel {
        provider_profile_sha256,
        ..
    } = &mut challenge.predecessor
    {
        *provider_profile_sha256 = digest('e');
    }
    assert!(
        canonical_install_epoch_authority_statement_with_trust_v1(
            &challenge, &trust, &state, &provider,
        )
        .is_err()
    );

    let mut challenge = pair.authority.challenge;
    if let ExternalWatermarkPredecessorV1::GenesisPinnedSentinel {
        stream_id_sha256, ..
    } = &mut challenge.predecessor
    {
        *stream_id_sha256 = digest('e');
    }
    assert!(
        canonical_install_epoch_authority_statement_with_trust_v1(
            &challenge, &trust, &state, &provider,
        )
        .is_err()
    );
}

#[test]
fn genesis_is_exact_provider_sentinel_and_local_absence_has_no_encoding() {
    let pair = genesis_pair();
    let state = test_only_state_root_profile_v1();
    let provider = test_only_external_watermark_provider_profile_v1(&lease_trust());
    let trust = authority_trust();
    for mutate in [
        |predecessor: &mut ExternalWatermarkPredecessorV1| {
            if let ExternalWatermarkPredecessorV1::GenesisPinnedSentinel { revision, .. } =
                predecessor
            {
                *revision = 1;
            }
        },
        |predecessor: &mut ExternalWatermarkPredecessorV1| {
            if let ExternalWatermarkPredecessorV1::GenesisPinnedSentinel { tip_sha256, .. } =
                predecessor
            {
                *tip_sha256 = digest('e');
            }
        },
        |predecessor: &mut ExternalWatermarkPredecessorV1| {
            if let ExternalWatermarkPredecessorV1::GenesisPinnedSentinel {
                genesis_epoch_binding_sha256,
                ..
            } = predecessor
            {
                *genesis_epoch_binding_sha256 = digest('e');
            }
        },
    ] as [fn(&mut ExternalWatermarkPredecessorV1); 3]
    {
        let mut challenge = pair.authority.challenge.clone();
        mutate(&mut challenge.predecessor);
        assert!(
            canonical_install_epoch_authority_statement_with_trust_v1(
                &challenge, &trust, &state, &provider,
            )
            .is_err()
        );
    }
}

#[test]
fn epoch_sequence_cannot_repeat_regress_or_diverge_from_external_revision() {
    let genesis = genesis_pair();
    let mut challenge = genesis.authority.challenge;
    challenge.epoch.epoch_sequence = 2;
    assert_authority_challenge_rejected(challenge);

    let successor = successor_pair();
    let mut challenge = successor.authority.challenge.clone();
    challenge.epoch.epoch_sequence = 3;
    assert_authority_challenge_rejected(challenge);

    let mut challenge = successor.authority.challenge;
    if let ExternalWatermarkPredecessorV1::Successor { revision, .. } = &mut challenge.predecessor {
        *revision = 2;
    }
    assert_authority_challenge_rejected(challenge);
}

fn assert_authority_challenge_rejected(challenge: InstallEpochAuthorityChallengeV1) {
    let state = test_only_state_root_profile_v1();
    let provider = test_only_external_watermark_provider_profile_v1(&lease_trust());
    assert!(
        canonical_install_epoch_authority_statement_with_trust_v1(
            &challenge,
            &authority_trust(),
            &state,
            &provider,
        )
        .is_err()
    );
}

#[test]
fn one_shared_successor_nonce_domain_rejects_exact_and_resigned_cross_scope_replay() {
    let pair = genesis_pair();
    let mut guard = InstallEpochReplayGuardV1::default();
    verify(&pair, 1_050, &mut guard).unwrap();
    assert!(verify(&pair, 1_050, &mut guard).is_err());

    let mut same_nonce = genesis_pair();
    same_nonce.lease.challenge.lease_nonce = same_nonce.authority.challenge.authority_nonce.clone();
    let (lease, lease_observation) = sign_lease(same_nonce.lease.challenge.clone());
    same_nonce.lease = lease;
    same_nonce.lease_observation = lease_observation;
    assert!(
        verify(
            &same_nonce,
            1_050,
            &mut InstallEpochReplayGuardV1::default()
        )
        .is_err()
    );

    let pair = genesis_pair();
    for consumed in [
        pair.authority.challenge.authority_nonce.clone(),
        pair.lease.challenge.lease_nonce.clone(),
    ] {
        let mut guard = InstallEpochReplayGuardV1::from_consumed_nonces([consumed]).unwrap();
        assert!(verify(&pair, 1_050, &mut guard).is_err());
    }

    let original = genesis_pair();
    let mut guard = InstallEpochReplayGuardV1::default();
    verify(&original, 1_050, &mut guard).unwrap();
    let mut swapped_authority_challenge = original.authority.challenge.clone();
    swapped_authority_challenge.authority_nonce = original.lease.challenge.lease_nonce.clone();
    let (swapped_authority, swapped_authority_observation) =
        sign_authority(swapped_authority_challenge);
    let mut swapped_lease_challenge = lease_challenge(&swapped_authority);
    swapped_lease_challenge.lease_nonce = original.authority.challenge.authority_nonce.clone();
    let (swapped_lease, swapped_lease_observation) = sign_lease(swapped_lease_challenge);
    let resigned_swapped = SignedPair {
        authority: swapped_authority,
        authority_observation: swapped_authority_observation,
        lease: swapped_lease,
        lease_observation: swapped_lease_observation,
    };
    assert!(verify(&resigned_swapped, 1_050, &mut guard).is_err());
}

#[test]
fn authority_and_provider_require_distinct_trust_roots_and_keys() {
    let authority = authority_trust();
    let lease = lease_trust();
    assert_ne!(authority.policy_sha256(), lease.policy_sha256());
    assert_ne!(authority.trust_root_id(), lease.trust_root_id());
    assert_ne!(authority.key_fingerprint(), lease.key_fingerprint());
    validate_independent_trust_bindings_v1(&authority, &lease).unwrap();

    let same_root = crate::test_only_trust_binding_with_identity_v8(
        SshsigTrustPurposeV8::ExternalWatermarkLeaseV1,
        authority.trust_root_id(),
        lease.key_fingerprint(),
    );
    assert!(validate_independent_trust_bindings_v1(&authority, &same_root).is_err());

    let same_key = crate::test_only_trust_binding_with_identity_v8(
        SshsigTrustPurposeV8::ExternalWatermarkLeaseV1,
        lease.trust_root_id(),
        authority.key_fingerprint(),
    );
    assert!(validate_independent_trust_bindings_v1(&authority, &same_key).is_err());
}

#[test]
fn lease_cross_binds_authority_trust_epoch_predecessor_profile_host_and_next_revision() {
    let base = genesis_pair();
    for mutate in [
        |lease: &mut ExternalWatermarkLeaseChallengeV1| {
            lease.install_authority_statement_sha256 = digest('e')
        },
        |lease: &mut ExternalWatermarkLeaseChallengeV1| {
            lease.install_authority_trust_policy_sha256 = digest('e')
        },
        |lease: &mut ExternalWatermarkLeaseChallengeV1| lease.epoch.epoch_sequence += 1,
        |lease: &mut ExternalWatermarkLeaseChallengeV1| {
            lease.epoch.epoch_nonce_sha256 = digest('e')
        },
        |lease: &mut ExternalWatermarkLeaseChallengeV1| {
            lease.state_root_profile_sha256 = digest('e')
        },
        |lease: &mut ExternalWatermarkLeaseChallengeV1| {
            lease.provider_trust_policy_sha256 = digest('e')
        },
        |lease: &mut ExternalWatermarkLeaseChallengeV1| {
            lease.target_host.machine_id_sha256 = digest('e')
        },
        |lease: &mut ExternalWatermarkLeaseChallengeV1| lease.reserved_successor_revision += 1,
    ] as [fn(&mut ExternalWatermarkLeaseChallengeV1); 8]
    {
        let mut pair = base.clone();
        let mut challenge = pair.lease.challenge.clone();
        mutate(&mut challenge);
        let state = test_only_state_root_profile_v1();
        let provider = test_only_external_watermark_provider_profile_v1(&lease_trust());
        let result = canonical_external_watermark_lease_statement_with_trust_v1(
            &challenge,
            &lease_trust(),
            &state,
            &provider,
        );
        if let Ok(statement) = result {
            let signature_bytes = b"test-external-watermark-lease-signature".to_vec();
            let signature_sha256 = sha256(&signature_bytes);
            pair.lease = SignedExternalWatermarkLeaseV1 {
                canonical_statement_sha256: sha256(&statement),
                challenge,
                detached_signature_bytes: signature_bytes,
                detached_signature_sha256: signature_sha256.clone(),
            };
            pair.lease_observation = CryptographicSignatureObservation::for_test_only(
                signature_sha256,
                pair.lease.canonical_statement_sha256.clone(),
                SshsigTrustPurposeV8::ExternalWatermarkLeaseV1,
            );
            assert!(verify(&pair, 1_050, &mut InstallEpochReplayGuardV1::default()).is_err());
        }
    }
}

#[test]
fn lease_validity_must_be_exactly_contained_by_authority_validity() {
    let base = genesis_pair();
    assert!(verify(&base, 1_050, &mut InstallEpochReplayGuardV1::default()).is_ok());

    let mut starts_early = base.clone();
    let mut challenge = starts_early.lease.challenge.clone();
    challenge.issued_at_unix_seconds = 999;
    challenge.expires_at_unix_seconds = 1_050;
    let (lease, observation) = sign_lease(challenge);
    starts_early.lease = lease;
    starts_early.lease_observation = observation;
    assert!(
        verify(
            &starts_early,
            1_025,
            &mut InstallEpochReplayGuardV1::default()
        )
        .is_err()
    );

    let mut authority_challenge = base.authority.challenge;
    authority_challenge.expires_at_unix_seconds = 1_050;
    let (authority, authority_observation) = sign_authority(authority_challenge);
    let (lease, lease_observation) = sign_lease(lease_challenge(&authority));
    let ends_late = SignedPair {
        authority,
        authority_observation,
        lease,
        lease_observation,
    };
    assert!(verify(&ends_late, 1_025, &mut InstallEpochReplayGuardV1::default()).is_err());
}

#[test]
fn validity_is_half_open_and_old_family_or_wrong_purpose_evidence_is_rejected() {
    let pair = genesis_pair();
    assert!(verify(&pair, 999, &mut InstallEpochReplayGuardV1::default()).is_err());
    assert!(verify(&pair, 1_000, &mut InstallEpochReplayGuardV1::default()).is_ok());
    assert!(verify(&pair, 1_109, &mut InstallEpochReplayGuardV1::default()).is_ok());
    assert!(verify(&pair, 1_110, &mut InstallEpochReplayGuardV1::default()).is_err());

    let mut wrong = pair.clone();
    wrong.authority_observation = CryptographicSignatureObservation::for_test_only(
        wrong.authority.detached_signature_sha256.clone(),
        wrong.authority.canonical_statement_sha256.clone(),
        SshsigTrustPurposeV8::InstallAuthority,
    );
    assert!(verify(&wrong, 1_050, &mut InstallEpochReplayGuardV1::default()).is_err());

    let mut wrong = pair.clone();
    wrong.lease_observation = CryptographicSignatureObservation::for_test_only(
        wrong.lease.detached_signature_sha256.clone(),
        wrong.lease.canonical_statement_sha256.clone(),
        SshsigTrustPurposeV8::InstallEpochAuthorityV1,
    );
    assert!(verify(&wrong, 1_050, &mut InstallEpochReplayGuardV1::default()).is_err());

    let mut old_family = pair.authority.challenge;
    old_family.namespace = crate::INSTALL_NAMESPACE_V8.to_string();
    assert_authority_challenge_rejected(old_family);
}

#[test]
fn statement_signature_and_nested_schema_tampering_fail_closed() {
    let pair = genesis_pair();
    let mut tampered = pair.clone();
    tampered.authority.canonical_statement_sha256 = digest('e');
    assert!(verify(&tampered, 1_050, &mut InstallEpochReplayGuardV1::default()).is_err());

    let mut tampered = pair.clone();
    tampered.lease.detached_signature_bytes.push(0);
    assert!(verify(&tampered, 1_050, &mut InstallEpochReplayGuardV1::default()).is_err());

    let mut value = serde_json::to_value(&pair.authority.challenge).unwrap();
    value["state_root_profile"]
        .as_object_mut()
        .unwrap()
        .insert("self_trusted".to_string(), serde_json::json!(true));
    assert!(serde_json::from_value::<InstallEpochAuthorityChallengeV1>(value).is_err());

    let mut value = serde_json::to_value(&pair.lease.challenge).unwrap();
    value["predecessor"].as_object_mut().unwrap().insert(
        "local_missing_means_genesis".to_string(),
        serde_json::json!(true),
    );
    assert!(serde_json::from_value::<ExternalWatermarkLeaseChallengeV1>(value).is_err());
}
