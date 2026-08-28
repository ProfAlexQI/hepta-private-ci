use codex_hepta_authbus_p1_qualification::*;
use codex_hepta_contracts::IdentityBinding;
use codex_hepta_contracts::IdentityPeerEvidence;
use codex_hepta_contracts::Sha256Digest;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use pretty_assertions::assert_eq;

const NOW: u64 = 1_800_000_000;

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

fn signing_key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

fn key_record(
    issuer_id: &str,
    key_id: &str,
    epoch: u64,
    purpose: P11KeyPurpose,
    signing_key: &SigningKey,
) -> P11VerificationKeyRecord {
    let public_key = signing_key.verifying_key().to_bytes().to_vec();
    P11VerificationKeyRecord {
        schema_version: AUTHBUS_P1_1_SCHEMA_VERSION,
        issuer_id: issuer_id.to_owned(),
        key_id: key_id.to_owned(),
        key_epoch: epoch,
        purpose,
        usage_domain: purpose.usage_domain().to_owned(),
        public_key_sha256: Sha256Digest::for_bytes(&public_key),
        public_key,
        backend_binding_sha256: digest(&format!("backend:{issuer_id}:{key_id}:{epoch}")),
        valid_from_unix_seconds: NOW - 1_000,
        valid_until_unix_seconds: NOW + 10_000,
        revoked_at_unix_seconds: None,
        authority: false,
    }
}

fn identity_binding(
    key_id: &str,
    epoch: u64,
    nonce_label: &str,
    issued_at: u64,
    not_before: u64,
    expires_at: u64,
) -> IdentityBinding {
    let tenant_id = "tenant-a";
    let workspace_id = "workspace-a";
    let agent_id = "agent-a";
    let service_id = "authbus-client";
    let node_id = "node-a";
    let generation = 7;
    let launch_nonce_sha256 = digest("launch-a");
    let subject_digest = p11_identity_subject_digest(
        tenant_id,
        workspace_id,
        agent_id,
        service_id,
        node_id,
        generation,
    )
    .expect("subject digest");
    IdentityBinding {
        schema_version: 1,
        binding_id: format!("binding:{nonce_label}"),
        tenant_id: tenant_id.to_owned(),
        workspace_id: workspace_id.to_owned(),
        agent_id: agent_id.to_owned(),
        service_id: service_id.to_owned(),
        node_id: node_id.to_owned(),
        generation,
        launch_nonce_sha256: launch_nonce_sha256.clone(),
        session_id: "session-a".to_owned(),
        operation: "refresh".to_owned(),
        secret_ref_allowlist_digest: digest("allowlist"),
        epoch,
        nonce_sha256: digest(nonce_label),
        key_id: key_id.to_owned(),
        capability_digest: digest("capability"),
        intent_digest: digest("intent"),
        transcript_digest: digest("transcript"),
        audience: "hepta-authbus".to_owned(),
        issued_at_unix_seconds: issued_at,
        not_before_unix_seconds: not_before,
        expires_at_unix_seconds: expires_at,
        policy_digest: digest("policy"),
        hnl_attestation_digest: digest("hnl"),
        service_identity_digest: digest("service-identity"),
        subject_digest,
        authority_epoch: 3,
        owner_epoch: 5,
        fencing_token_sha256: digest("identity-fence"),
        peer: IdentityPeerEvidence::LinuxPeer {
            peer_uid: 1_001,
            peer_gid: 1_001,
            peer_pid: 42,
            agentd_generation: generation,
            launch_nonce_sha256,
            pid_start_time_ticks: 44_001,
            pidfd_bound: true,
        },
        authority: false,
    }
}

fn signed_identity(
    signing_key: &SigningKey,
    issuer_id: &str,
    key_id: &str,
    epoch: u64,
    nonce_label: &str,
) -> P11SignedIdentityEvidence {
    let mut evidence = P11SignedIdentityEvidence {
        schema_version: AUTHBUS_P1_1_SCHEMA_VERSION,
        issuer_id: issuer_id.to_owned(),
        key_id: key_id.to_owned(),
        key_epoch: epoch,
        binding: identity_binding(key_id, epoch, nonce_label, NOW - 2, NOW - 5, NOW + 120),
        signature: Vec::new(),
        authority: false,
    };
    evidence.signature = signing_key
        .sign(&evidence.signing_bytes().expect("identity bytes"))
        .to_bytes()
        .to_vec();
    evidence
}

fn identity_context(binding: &IdentityBinding, now: u64) -> P11IdentityVerificationContext {
    P11IdentityVerificationContext {
        expected_audience: binding.audience.clone(),
        expected_service_identity_sha256: binding.service_identity_digest.clone(),
        expected_policy_sha256: binding.policy_digest.clone(),
        expected_peer: binding.peer.clone(),
        now_unix_seconds: now,
    }
}

fn fence() -> P11Fence {
    P11Fence {
        authority_epoch: 3,
        owner_epoch: 5,
        generation: 7,
        fencing_token_sha256: digest("operation-fence"),
    }
}

fn operation_binding(operation_id: &str) -> P11OperationEvidenceBinding {
    P11OperationEvidenceBinding {
        schema_version: AUTHBUS_P1_1_SCHEMA_VERSION,
        operation_id: operation_id.to_owned(),
        provider_id: "provider-a".to_owned(),
        profile_id: "profile-a".to_owned(),
        token_family_id: "family-a".to_owned(),
        status_binding_sha256: digest(&format!("status-binding:{operation_id}")),
        fence: fence(),
        authority: false,
    }
}

fn signed_status(
    signing_key: &SigningKey,
    key_id: &str,
    operation: &P11OperationEvidenceBinding,
    revision: u64,
    observed_at: u64,
    outcome: P11ProviderEvidenceOutcome,
) -> P11SignedProviderStatusEvidence {
    let mut evidence = P11SignedProviderStatusEvidence {
        schema_version: AUTHBUS_P1_1_SCHEMA_VERSION,
        issuer_id: "provider-status-issuer".to_owned(),
        key_id: key_id.to_owned(),
        key_epoch: 1,
        operation_id: operation.operation_id.clone(),
        provider_id: operation.provider_id.clone(),
        profile_id: operation.profile_id.clone(),
        token_family_id: operation.token_family_id.clone(),
        status_binding_sha256: operation.status_binding_sha256.clone(),
        fence: operation.fence.clone(),
        status_revision: revision,
        observed_at_unix_seconds: observed_at,
        outcome,
        signature: Vec::new(),
        authority: false,
    };
    evidence.signature = signing_key
        .sign(&evidence.signing_bytes().expect("status bytes"))
        .to_bytes()
        .to_vec();
    evidence
}

fn signed_manual(
    signing_key: &SigningKey,
    key_id: &str,
    operation: &P11OperationEvidenceBinding,
    revision: u64,
    observed_at: u64,
    decision: P11ManualDecision,
) -> P11SignedManualEvidence {
    let mut evidence = P11SignedManualEvidence {
        schema_version: AUTHBUS_P1_1_SCHEMA_VERSION,
        issuer_id: "operator-issuer".to_owned(),
        key_id: key_id.to_owned(),
        key_epoch: 1,
        operator_id: "operator-a".to_owned(),
        case_id: "case-a".to_owned(),
        operation_id: operation.operation_id.clone(),
        status_binding_sha256: operation.status_binding_sha256.clone(),
        fence: operation.fence.clone(),
        manual_revision: revision,
        observed_at_unix_seconds: observed_at,
        decision,
        reason_sha256: digest("manual-reason"),
        signature: Vec::new(),
        authority: false,
    };
    evidence.signature = signing_key
        .sign(&evidence.signing_bytes().expect("manual bytes"))
        .to_bytes()
        .to_vec();
    evidence
}

#[test]
fn negative_authority_and_key_registration_are_fail_closed() {
    assert!(AUTHBUS_P1_1_QUALIFICATION_ONLY);
    assert!(!AUTHBUS_P1_1_AUTHORITY);
    assert!(!AUTHBUS_P1_1_EFFECT_AUTHORITY);
    assert!(!AUTHBUS_P1_1_PRODUCTION_CALLER);
    assert!(!AUTHBUS_P1_1_PRODUCTION_WRITER);
    assert!(!AUTHBUS_P1_1_OPERATOR_ACCEPTANCE);
    assert!(!AUTHBUS_P1_1_PROMOTION);
    assert!(!AUTHBUS_P1_1_G5_ALLOWED);
    assert!(!AUTHBUS_P1_1_EXECUTE_ALLOWED);
    assert!(!AUTHBUS_P1_1_PRIVATE_KEY_STORAGE);

    let signing = signing_key(1);
    let record = key_record(
        "identity-issuer",
        "identity-key-1",
        1,
        P11KeyPurpose::IdentityIssuer,
        &signing,
    );
    let mut verifier = P11Verifier::new(P11Policy::default()).expect("verifier");
    assert_eq!(
        verifier.register_key(record.clone()),
        Ok(P11WriteDisposition::Applied)
    );
    assert_eq!(
        verifier.register_key(record.clone()),
        Ok(P11WriteDisposition::AlreadyPresent)
    );

    let mut changed = record;
    changed.backend_binding_sha256 = digest("changed-backend");
    assert_eq!(
        verifier.register_key(changed),
        Err(P11Error::KeyConflict)
    );
}

#[test]
fn signed_identity_verifies_once_and_replay_is_rejected() {
    let signing = signing_key(2);
    let mut verifier = P11Verifier::new(P11Policy::default()).expect("verifier");
    verifier
        .register_key(key_record(
            "identity-issuer",
            "identity-key-1",
            1,
            P11KeyPurpose::IdentityIssuer,
            &signing,
        ))
        .expect("key");

    let evidence = signed_identity(
        &signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "nonce-a",
    );
    let context = identity_context(&evidence.binding, NOW);
    let receipt = verifier
        .verify_identity(&evidence, &context)
        .expect("identity verified");
    assert_eq!(receipt.binding_sha256, evidence.binding.digest().expect("digest"));
    assert!(!receipt.authority);
    assert_eq!(
        verifier.verify_identity(&evidence, &context),
        Err(P11Error::NonceReplay)
    );
}

#[test]
fn identity_signature_audience_peer_and_time_bindings_are_enforced() {
    let signing = signing_key(3);
    let wrong_signing = signing_key(4);
    let mut verifier = P11Verifier::new(P11Policy::default()).expect("verifier");
    verifier
        .register_key(key_record(
            "identity-issuer",
            "identity-key-1",
            1,
            P11KeyPurpose::IdentityIssuer,
            &signing,
        ))
        .expect("key");

    let evidence = signed_identity(
        &wrong_signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "nonce-bad-signature",
    );
    let context = identity_context(&evidence.binding, NOW);
    assert_eq!(
        verifier.verify_identity(&evidence, &context),
        Err(P11Error::SignatureInvalid)
    );

    let evidence = signed_identity(
        &signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "nonce-wrong-audience",
    );
    let mut context = identity_context(&evidence.binding, NOW);
    context.expected_audience = "other-audience".to_owned();
    assert_eq!(
        verifier.verify_identity(&evidence, &context),
        Err(P11Error::AudienceMismatch)
    );

    let evidence = signed_identity(
        &signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "nonce-wrong-peer",
    );
    let mut context = identity_context(&evidence.binding, NOW);
    context.expected_peer = IdentityPeerEvidence::MacAuditToken {
        token_sha256: digest("audit-token"),
    };
    assert_eq!(
        verifier.verify_identity(&evidence, &context),
        Err(P11Error::BindingMismatch)
    );

    let mut long_lived = signed_identity(
        &signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "nonce-long-lived",
    );
    long_lived.binding.expires_at_unix_seconds = NOW + 1_000;
    long_lived.signature = signing
        .sign(&long_lived.signing_bytes().expect("long bytes"))
        .to_bytes()
        .to_vec();
    let context = identity_context(&long_lived.binding, NOW);
    assert_eq!(
        verifier.verify_identity(&long_lived, &context),
        Err(P11Error::TtlExceeded)
    );

    let expired = signed_identity(
        &signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "nonce-expired",
    );
    let context = identity_context(&expired.binding, NOW + 1_000);
    assert_eq!(
        verifier.verify_identity(&expired, &context),
        Err(P11Error::Expired)
    );
}

#[test]
fn key_epoch_rotation_and_revocation_are_monotonic() {
    let key_one = signing_key(5);
    let key_two = signing_key(6);
    let mut verifier = P11Verifier::new(P11Policy::default()).expect("verifier");
    verifier
        .register_key(key_record(
            "identity-issuer",
            "identity-key-1",
            1,
            P11KeyPurpose::IdentityIssuer,
            &key_one,
        ))
        .expect("key one");
    verifier
        .register_key(key_record(
            "identity-issuer",
            "identity-key-2",
            2,
            P11KeyPurpose::IdentityIssuer,
            &key_two,
        ))
        .expect("key two");

    let old = signed_identity(
        &key_one,
        "identity-issuer",
        "identity-key-1",
        1,
        "nonce-old-epoch",
    );
    let context = identity_context(&old.binding, NOW);
    assert_eq!(
        verifier.verify_identity(&old, &context),
        Err(P11Error::StaleKeyEpoch)
    );

    let current = signed_identity(
        &key_two,
        "identity-issuer",
        "identity-key-2",
        2,
        "nonce-current-epoch",
    );
    let context = identity_context(&current.binding, NOW);
    verifier
        .verify_identity(&current, &context)
        .expect("current epoch");

    verifier
        .revoke_key("identity-issuer", "identity-key-2", 2, NOW)
        .expect("revoke");
    let revoked = signed_identity(
        &key_two,
        "identity-issuer",
        "identity-key-2",
        2,
        "nonce-revoked",
    );
    let context = identity_context(&revoked.binding, NOW);
    assert_eq!(
        verifier.verify_identity(&revoked, &context),
        Err(P11Error::KeyRevoked)
    );
}

#[test]
fn nonce_cache_capacity_denies_without_evicting_live_evidence() {
    let signing = signing_key(7);
    let policy = P11Policy {
        max_nonce_entries: 1,
        ..P11Policy::default()
    };
    let mut verifier = P11Verifier::new(policy).expect("verifier");
    verifier
        .register_key(key_record(
            "identity-issuer",
            "identity-key-1",
            1,
            P11KeyPurpose::IdentityIssuer,
            &signing,
        ))
        .expect("key");

    let first = signed_identity(
        &signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "nonce-capacity-1",
    );
    verifier
        .verify_identity(&first, &identity_context(&first.binding, NOW))
        .expect("first");
    let second = signed_identity(
        &signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "nonce-capacity-2",
    );
    assert_eq!(
        verifier.verify_identity(&second, &identity_context(&second.binding, NOW)),
        Err(P11Error::NonceCapacity)
    );
    assert_eq!(verifier.nonce_entry_count(), 1);
}

#[test]
fn provider_status_is_signed_monotonic_and_exactly_replayable() {
    let signing = signing_key(8);
    let operation = operation_binding("operation-a");
    let mut verifier = P11Verifier::new(P11Policy::default()).expect("verifier");
    verifier
        .register_key(key_record(
            "provider-status-issuer",
            "status-key-1",
            1,
            P11KeyPurpose::ProviderStatusIssuer,
            &signing,
        ))
        .expect("key");
    verifier
        .register_operation(operation.clone())
        .expect("operation");

    let unknown = signed_status(
        &signing,
        "status-key-1",
        &operation,
        1,
        NOW,
        P11ProviderEvidenceOutcome::Unknown {
            reason_sha256: digest("unknown"),
        },
    );
    let first = verifier
        .verify_provider_status(&unknown, NOW)
        .expect("unknown");
    assert!(matches!(
        first,
        P11ProviderEvidenceDisposition::Applied(P11ProviderStatusReceipt {
            state: P11EvidenceState::Unknown,
            ..
        })
    ));
    assert!(matches!(
        verifier
            .verify_provider_status(&unknown, NOW)
            .expect("replay"),
        P11ProviderEvidenceDisposition::AlreadyPresent(_)
    ));

    let mut changed = unknown.clone();
    changed.outcome = P11ProviderEvidenceOutcome::Indeterminate {
        reason_sha256: digest("changed"),
    };
    changed.signature = signing
        .sign(&changed.signing_bytes().expect("changed bytes"))
        .to_bytes()
        .to_vec();
    assert_eq!(
        verifier.verify_provider_status(&changed, NOW),
        Err(P11Error::EvidenceConflict)
    );

    let completed = signed_status(
        &signing,
        "status-key-1",
        &operation,
        2,
        NOW + 1,
        P11ProviderEvidenceOutcome::Completed {
            result_sha256: digest("result"),
        },
    );
    verifier
        .verify_provider_status(&completed, NOW + 1)
        .expect("completed");
    assert_eq!(
        verifier.operation_state(&operation.operation_id),
        Ok(P11EvidenceState::Completed)
    );

    let later = signed_status(
        &signing,
        "status-key-1",
        &operation,
        3,
        NOW + 2,
        P11ProviderEvidenceOutcome::Unknown {
            reason_sha256: digest("late"),
        },
    );
    assert_eq!(
        verifier.verify_provider_status(&later, NOW + 2),
        Err(P11Error::TerminalImmutable)
    );
}

#[test]
fn provider_binding_freshness_and_key_purpose_are_enforced() {
    let provider_signing = signing_key(9);
    let identity_signing = signing_key(10);
    let operation = operation_binding("operation-b");
    let mut verifier = P11Verifier::new(P11Policy::default()).expect("verifier");
    verifier
        .register_key(key_record(
            "provider-status-issuer",
            "status-key-1",
            1,
            P11KeyPurpose::ProviderStatusIssuer,
            &provider_signing,
        ))
        .expect("provider key");
    verifier
        .register_key(key_record(
            "identity-issuer",
            "identity-key-1",
            1,
            P11KeyPurpose::IdentityIssuer,
            &identity_signing,
        ))
        .expect("identity key");
    verifier
        .register_operation(operation.clone())
        .expect("operation");

    let mut wrong_binding = signed_status(
        &provider_signing,
        "status-key-1",
        &operation,
        1,
        NOW,
        P11ProviderEvidenceOutcome::Unknown {
            reason_sha256: digest("unknown"),
        },
    );
    wrong_binding.provider_id = "provider-b".to_owned();
    wrong_binding.signature = provider_signing
        .sign(&wrong_binding.signing_bytes().expect("wrong binding bytes"))
        .to_bytes()
        .to_vec();
    assert_eq!(
        verifier.verify_provider_status(&wrong_binding, NOW),
        Err(P11Error::BindingMismatch)
    );

    let too_old = signed_status(
        &provider_signing,
        "status-key-1",
        &operation,
        1,
        NOW - 1_000,
        P11ProviderEvidenceOutcome::Unknown {
            reason_sha256: digest("old"),
        },
    );
    assert_eq!(
        verifier.verify_provider_status(&too_old, NOW),
        Err(P11Error::EvidenceTooOld)
    );

    let mut wrong_purpose = signed_status(
        &identity_signing,
        "identity-key-1",
        &operation,
        1,
        NOW,
        P11ProviderEvidenceOutcome::Unknown {
            reason_sha256: digest("purpose"),
        },
    );
    wrong_purpose.issuer_id = "identity-issuer".to_owned();
    wrong_purpose.signature = identity_signing
        .sign(&wrong_purpose.signing_bytes().expect("purpose bytes"))
        .to_bytes()
        .to_vec();
    assert_eq!(
        verifier.verify_provider_status(&wrong_purpose, NOW),
        Err(P11Error::KeyPurposeMismatch)
    );
}

#[test]
fn manual_required_needs_independent_operator_evidence_and_resumes_lookup_only() {
    let provider_signing = signing_key(11);
    let operator_signing = signing_key(12);
    let operation = operation_binding("operation-manual");
    let mut verifier = P11Verifier::new(P11Policy::default()).expect("verifier");
    verifier
        .register_key(key_record(
            "provider-status-issuer",
            "status-key-1",
            1,
            P11KeyPurpose::ProviderStatusIssuer,
            &provider_signing,
        ))
        .expect("provider key");
    verifier
        .register_key(key_record(
            "operator-issuer",
            "operator-key-1",
            1,
            P11KeyPurpose::OperatorEvidenceIssuer,
            &operator_signing,
        ))
        .expect("operator key");
    verifier
        .register_operation(operation.clone())
        .expect("operation");

    let manual_required = signed_status(
        &provider_signing,
        "status-key-1",
        &operation,
        1,
        NOW,
        P11ProviderEvidenceOutcome::ManualRequired {
            reason_sha256: digest("manual-required"),
        },
    );
    verifier
        .verify_provider_status(&manual_required, NOW)
        .expect("manual required");

    let bypass = signed_status(
        &provider_signing,
        "status-key-1",
        &operation,
        2,
        NOW + 1,
        P11ProviderEvidenceOutcome::Completed {
            result_sha256: digest("unsafe-result"),
        },
    );
    assert_eq!(
        verifier.verify_provider_status(&bypass, NOW + 1),
        Err(P11Error::ManualEvidenceRequired)
    );

    let resume = signed_manual(
        &operator_signing,
        "operator-key-1",
        &operation,
        1,
        NOW + 1,
        P11ManualDecision::ResumeLookupOnly,
    );
    let applied = verifier
        .verify_manual_evidence(&resume, NOW + 1)
        .expect("manual resume");
    assert!(matches!(
        applied,
        P11ManualEvidenceDisposition::Applied(P11ManualEvidenceReceipt {
            state: P11EvidenceState::LookupOnly,
            authority: false,
            ..
        })
    ));
    assert!(matches!(
        verifier
            .verify_manual_evidence(&resume, NOW + 1)
            .expect("manual replay"),
        P11ManualEvidenceDisposition::AlreadyPresent(_)
    ));

    let lookup_result = signed_status(
        &provider_signing,
        "status-key-1",
        &operation,
        2,
        NOW + 2,
        P11ProviderEvidenceOutcome::VerifiedNoEffect {
            provider_receipt_sha256: digest("no-effect"),
        },
    );
    verifier
        .verify_provider_status(&lookup_result, NOW + 2)
        .expect("lookup result");
    assert_eq!(
        verifier.operation_state(&operation.operation_id),
        Ok(P11EvidenceState::NoEffect)
    );
}

#[test]
fn manual_evidence_cannot_use_provider_key_or_mutate_non_manual_state() {
    let provider_signing = signing_key(13);
    let operation = operation_binding("operation-not-manual");
    let mut verifier = P11Verifier::new(P11Policy::default()).expect("verifier");
    verifier
        .register_key(key_record(
            "provider-status-issuer",
            "status-key-1",
            1,
            P11KeyPurpose::ProviderStatusIssuer,
            &provider_signing,
        ))
        .expect("provider key");
    verifier
        .register_operation(operation.clone())
        .expect("operation");

    let mut evidence = signed_manual(
        &provider_signing,
        "status-key-1",
        &operation,
        1,
        NOW,
        P11ManualDecision::Quarantine,
    );
    evidence.issuer_id = "provider-status-issuer".to_owned();
    evidence.signature = provider_signing
        .sign(&evidence.signing_bytes().expect("manual provider bytes"))
        .to_bytes()
        .to_vec();
    assert_eq!(
        verifier.verify_manual_evidence(&evidence, NOW),
        Err(P11Error::KeyPurposeMismatch)
    );
    assert_eq!(
        verifier.operation_state(&operation.operation_id),
        Ok(P11EvidenceState::Pending)
    );
}

#[test]
fn operation_ledger_capacity_and_registration_conflicts_fail_closed() {
    let policy = P11Policy {
        max_operation_entries: 1,
        ..P11Policy::default()
    };
    let mut verifier = P11Verifier::new(policy).expect("verifier");
    let first = operation_binding("operation-capacity-1");
    assert_eq!(
        verifier.register_operation(first.clone()),
        Ok(P11WriteDisposition::Applied)
    );
    assert_eq!(
        verifier.register_operation(first.clone()),
        Ok(P11WriteDisposition::AlreadyPresent)
    );

    let mut changed = first;
    changed.provider_id = "changed-provider".to_owned();
    assert_eq!(
        verifier.register_operation(changed),
        Err(P11Error::OperationConflict)
    );
    assert_eq!(
        verifier.register_operation(operation_binding("operation-capacity-2")),
        Err(P11Error::OperationCapacity)
    );
}
