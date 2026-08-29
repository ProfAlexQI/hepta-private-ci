use std::fs;

use codex_hepta_authbus_p1_2_qualification::*;
use codex_hepta_authbus_p1_qualification::*;
use codex_hepta_contracts::IdentityBinding;
use codex_hepta_contracts::IdentityPeerEvidence;
use codex_hepta_contracts::Sha256Digest;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

const NOW: u64 = 1_800_000_000;

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

fn signing_key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

fn writer(boot_id: &str, generation: u64) -> P12WriterIdentity {
    P12WriterIdentity {
        boot_id: boot_id.to_owned(),
        generation,
    }
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
        issued_at_unix_seconds: NOW - 2,
        not_before_unix_seconds: NOW - 5,
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
    expires_at: u64,
) -> P11SignedIdentityEvidence {
    let mut evidence = P11SignedIdentityEvidence {
        schema_version: AUTHBUS_P1_1_SCHEMA_VERSION,
        issuer_id: issuer_id.to_owned(),
        key_id: key_id.to_owned(),
        key_epoch: epoch,
        binding: identity_binding(key_id, epoch, nonce_label, expires_at),
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

fn verified_identity(
    record: &P11VerificationKeyRecord,
    evidence: &P11SignedIdentityEvidence,
) -> P11IdentityVerificationReceipt {
    let mut verifier = P11Verifier::new(P11Policy::default()).expect("verifier");
    verifier.register_key(record.clone()).expect("key");
    verifier
        .verify_identity(evidence, &identity_context(&evidence.binding, NOW))
        .expect("identity receipt")
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

fn status_verifier(
    status_record: &P11VerificationKeyRecord,
    operator_record: Option<&P11VerificationKeyRecord>,
    operation: &P11OperationEvidenceBinding,
) -> P11Verifier {
    let mut verifier = P11Verifier::new(P11Policy::default()).expect("verifier");
    verifier
        .register_key(status_record.clone())
        .expect("status key");
    if let Some(record) = operator_record {
        verifier.register_key(record.clone()).expect("operator key");
    }
    verifier
        .register_operation(operation.clone())
        .expect("operation");
    verifier
}

async fn open_store(
    root: &TempDir,
    writer_identity: P12WriterIdentity,
    policy: P12Policy,
) -> P12Store {
    P12Store::open(root.path(), writer_identity, policy, NOW)
        .await
        .expect("open P1.2 store")
}

#[tokio::test]
async fn default_off_authority_and_private_file_posture_are_enforced() {
    const {
        assert!(AUTHBUS_P1_2_QUALIFICATION_ONLY);
        assert!(!AUTHBUS_P1_2_AUTHORITY);
        assert!(!AUTHBUS_P1_2_EFFECT_AUTHORITY);
        assert!(!AUTHBUS_P1_2_PRODUCTION_CALLER);
        assert!(!AUTHBUS_P1_2_PRODUCTION_WRITER);
        assert!(!AUTHBUS_P1_2_OPERATOR_ACCEPTANCE);
        assert!(!AUTHBUS_P1_2_PROMOTION);
        assert!(!AUTHBUS_P1_2_G5_ALLOWED);
        assert!(!AUTHBUS_P1_2_EXECUTE_ALLOWED);
        assert!(!AUTHBUS_P1_2_LISTENER_ENABLED);
        assert!(!AUTHBUS_P1_2_PROVIDER_CALL_ENABLED);
        assert!(!AUTHBUS_P1_2_OPENBAO_ENABLED);
        assert!(!AUTHBUS_P1_2_PRIVATE_KEY_STORAGE);
        assert!(!AUTHBUS_P1_2_RAW_SIGNATURE_STORAGE);
        assert!(!AUTHBUS_P1_2_SECRET_STORAGE);
        assert!(!AUTHBUS_P1_2_PARENT_WORKSPACE_WIRED);
    }

    let root = TempDir::new().expect("tempdir");
    let store = open_store(&root, writer("boot-a", 1), P12Policy::default()).await;
    let report = store.verify_integrity().await.expect("integrity");
    assert_eq!(report.key_rows, 0);
    assert_eq!(report.nonce_rows, 0);
    assert_eq!(report.operation_rows, 0);
    assert!(!report.authority);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let directory_mode = fs::metadata(root.path())
            .expect("directory metadata")
            .permissions()
            .mode()
            & 0o777;
        let file_mode = fs::metadata(store.database_path())
            .expect("database metadata")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(directory_mode, 0o700);
        assert_eq!(file_mode, 0o600);
    }
}

#[tokio::test]
async fn key_rotation_revocation_and_reopen_are_durable_and_monotonic() {
    let root = TempDir::new().expect("tempdir");
    let identity_one = signing_key(1);
    let identity_two = signing_key(2);
    let key_one = key_record(
        "identity-issuer",
        "identity-key-1",
        1,
        P11KeyPurpose::IdentityIssuer,
        &identity_one,
    );
    let key_two = key_record(
        "identity-issuer",
        "identity-key-2",
        2,
        P11KeyPurpose::IdentityIssuer,
        &identity_two,
    );
    let store = open_store(&root, writer("boot-a", 1), P12Policy::default()).await;
    assert_eq!(
        store.register_key(key_one.clone(), NOW).await,
        Ok(P12WriteDisposition::Applied)
    );
    assert_eq!(
        store.register_key(key_one.clone(), NOW).await,
        Ok(P12WriteDisposition::AlreadyPresent)
    );
    let mut conflict = key_one.clone();
    conflict.backend_binding_sha256 = digest("changed-backend");
    assert_eq!(
        store.register_key(conflict, NOW).await,
        Err(P12Error::KeyConflict)
    );
    assert_eq!(
        store.register_key(key_two.clone(), NOW + 1).await,
        Ok(P12WriteDisposition::Applied)
    );
    let stale = key_record(
        "identity-issuer",
        "identity-key-stale",
        1,
        P11KeyPurpose::IdentityIssuer,
        &identity_one,
    );
    assert_eq!(
        store.register_key(stale, NOW + 1).await,
        Err(P12Error::StaleKeyEpoch)
    );
    assert_eq!(
        store
            .revoke_key(
                "identity-issuer",
                P11KeyPurpose::IdentityIssuer,
                "identity-key-2",
                2,
                NOW + 2,
            )
            .await,
        Ok(P12WriteDisposition::Applied)
    );
    assert_eq!(
        store
            .revoke_key(
                "identity-issuer",
                P11KeyPurpose::IdentityIssuer,
                "identity-key-2",
                2,
                NOW + 2,
            )
            .await,
        Ok(P12WriteDisposition::AlreadyPresent)
    );
    store.close().await;

    let reopened = open_store(&root, writer("boot-a", 1), P12Policy::default()).await;
    let durable = reopened
        .key_record(
            "identity-issuer",
            P11KeyPurpose::IdentityIssuer,
            "identity-key-2",
            2,
        )
        .await
        .expect("durable key");
    assert_eq!(durable.revoked_at_unix_seconds, Some(NOW + 2));
    assert_eq!(reopened.key_count().await, Ok(2));
    reopened.verify_integrity().await.expect("integrity");
}

#[tokio::test]
async fn key_identity_is_namespaced_by_purpose_across_reopen_and_revocation() {
    let root = TempDir::new().expect("tempdir");
    let identity_signing = signing_key(21);
    let status_signing = signing_key(22);
    let identity_record = key_record(
        "shared-issuer",
        "shared-key",
        1,
        P11KeyPurpose::IdentityIssuer,
        &identity_signing,
    );
    let status_record = key_record(
        "shared-issuer",
        "shared-key",
        1,
        P11KeyPurpose::ProviderStatusIssuer,
        &status_signing,
    );

    let store = open_store(&root, writer("boot-a", 1), P12Policy::default()).await;
    assert_eq!(
        store.register_key(identity_record.clone(), NOW).await,
        Ok(P12WriteDisposition::Applied)
    );
    assert_eq!(
        store.register_key(status_record.clone(), NOW + 1).await,
        Ok(P12WriteDisposition::Applied)
    );
    assert_eq!(store.key_count().await, Ok(2));
    assert_eq!(
        store
            .key_record(
                "shared-issuer",
                P11KeyPurpose::OperatorEvidenceIssuer,
                "shared-key",
                1,
            )
            .await,
        Err(P12Error::UnknownKey)
    );
    assert_eq!(
        store
            .revoke_key(
                "shared-issuer",
                P11KeyPurpose::IdentityIssuer,
                "shared-key",
                1,
                NOW + 2,
            )
            .await,
        Ok(P12WriteDisposition::Applied)
    );
    store.close().await;

    let reopened = open_store(&root, writer("boot-a", 1), P12Policy::default()).await;
    let durable_identity = reopened
        .key_record(
            "shared-issuer",
            P11KeyPurpose::IdentityIssuer,
            "shared-key",
            1,
        )
        .await
        .expect("durable identity key");
    let durable_status = reopened
        .key_record(
            "shared-issuer",
            P11KeyPurpose::ProviderStatusIssuer,
            "shared-key",
            1,
        )
        .await
        .expect("durable status key");
    assert_eq!(durable_identity.revoked_at_unix_seconds, Some(NOW + 2));
    assert_eq!(durable_status.revoked_at_unix_seconds, None);
    assert_ne!(durable_identity.public_key, durable_status.public_key);

    let database_url = format!("sqlite://{}", reopened.database_path().display());
    let inspection = sqlx::SqlitePool::connect(&database_url)
        .await
        .expect("inspection pool");
    let mut connection = inspection.acquire().await.expect("inspection connection");
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&mut *connection)
        .await
        .expect("foreign keys on");
    let receipt_subjects: Vec<String> = sqlx::query_scalar(
        "SELECT subject_id FROM p12_durable_receipts \
         WHERE event_kind IN ('KEY_REGISTERED', 'KEY_REVOKED') ORDER BY sequence",
    )
    .fetch_all(&mut *connection)
    .await
    .expect("key receipt subjects");
    assert_eq!(
        receipt_subjects,
        vec![
            "shared-issuer:IDENTITY_ISSUER:shared-key:1".to_owned(),
            "shared-issuer:PROVIDER_STATUS_ISSUER:shared-key:1".to_owned(),
            "shared-issuer:IDENTITY_ISSUER:shared-key:1".to_owned(),
        ]
    );
    let wrong_purpose_head = sqlx::query(
        "UPDATE p12_key_heads SET purpose = 'OPERATOR_EVIDENCE_ISSUER' \
         WHERE issuer_id = 'shared-issuer' AND purpose = 'PROVIDER_STATUS_ISSUER'",
    )
    .execute(&mut *connection)
    .await;
    assert!(wrong_purpose_head.is_err());
    drop(connection);
    inspection.close().await;
    reopened.verify_integrity().await.expect("integrity");
}

#[tokio::test]
async fn nonce_replay_survives_reopen_and_capacity_fails_closed() {
    let root = TempDir::new().expect("tempdir");
    let signing = signing_key(3);
    let record = key_record(
        "identity-issuer",
        "identity-key-1",
        1,
        P11KeyPurpose::IdentityIssuer,
        &signing,
    );
    let first = signed_identity(
        &signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "nonce-one",
        NOW + 120,
    );
    let second = signed_identity(
        &signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "nonce-two",
        // `issued_at` is `NOW - 2`; this exercises the exact 300-second TTL limit.
        NOW + 298,
    );
    let first_receipt = verified_identity(&record, &first);
    let second_receipt = verified_identity(&record, &second);
    let policy = P12Policy {
        max_nonce_entries: 1,
        ..P12Policy::default()
    };
    let store = open_store(&root, writer("boot-a", 1), policy).await;
    store
        .register_key(record.clone(), NOW)
        .await
        .expect("durable identity key");
    store
        .claim_nonce(P12NonceClaim::from_verified(&first, &first_receipt, NOW).expect("claim"))
        .await
        .expect("first claim");
    store.close().await;

    let reopened = open_store(&root, writer("boot-a", 1), policy).await;
    assert_eq!(
        reopened
            .claim_nonce(
                P12NonceClaim::from_verified(&first, &first_receipt, NOW + 1)
                    .expect("replay claim")
            )
            .await,
        Err(P12Error::NonceReplay)
    );
    assert_eq!(
        reopened
            .claim_nonce(
                P12NonceClaim::from_verified(&second, &second_receipt, NOW + 1)
                    .expect("second claim")
            )
            .await,
        Err(P12Error::NonceCapacity)
    );
    let gc = reopened
        .collect_garbage(P12GcRequest {
            expected_revision: 0,
            now_unix_seconds: NOW + 121,
            max_rows: 10,
        })
        .await
        .expect("GC expired nonce");
    assert_eq!(gc.nonce_rows_deleted, 1);
    assert_eq!(
        reopened
            .claim_nonce(
                P12NonceClaim::from_verified(&second, &second_receipt, NOW + 121)
                    .expect("second post-GC claim")
            )
            .await,
        Ok(P12WriteDisposition::Applied)
    );
    assert_eq!(reopened.nonce_count().await, Ok(1));
}

#[tokio::test]
async fn provider_status_replay_conflict_and_terminal_tombstone_survive_reopen() {
    let root = TempDir::new().expect("tempdir");
    let status_signing = signing_key(4);
    let status_record = key_record(
        "provider-status-issuer",
        "status-key-1",
        1,
        P11KeyPurpose::ProviderStatusIssuer,
        &status_signing,
    );
    let operation = operation_binding("operation-terminal");
    let mut verifier = status_verifier(&status_record, None, &operation);
    let status_one = signed_status(
        &status_signing,
        "status-key-1",
        &operation,
        1,
        NOW,
        P11ProviderEvidenceOutcome::Unknown {
            reason_sha256: digest("unknown-one"),
        },
    );
    let receipt_one = match verifier
        .verify_provider_status(&status_one, NOW)
        .expect("status one")
    {
        P11ProviderEvidenceDisposition::Applied(receipt) => receipt,
        P11ProviderEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };
    let status_two = signed_status(
        &status_signing,
        "status-key-1",
        &operation,
        2,
        NOW + 1,
        P11ProviderEvidenceOutcome::Completed {
            result_sha256: digest("terminal-result"),
        },
    );
    let receipt_two = match verifier
        .verify_provider_status(&status_two, NOW + 1)
        .expect("status two")
    {
        P11ProviderEvidenceDisposition::Applied(receipt) => receipt,
        P11ProviderEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };

    let store = open_store(&root, writer("boot-a", 1), P12Policy::default()).await;
    store
        .register_key(status_record.clone(), NOW)
        .await
        .expect("durable status key");
    store
        .register_operation(operation.clone(), NOW)
        .await
        .expect("register operation");
    assert!(matches!(
        store
            .append_provider_status(&status_one, &receipt_one, NOW)
            .await,
        Ok(P11ProviderEvidenceDisposition::Applied(_))
    ));
    assert!(matches!(
        store
            .append_provider_status(&status_one, &receipt_one, NOW)
            .await,
        Ok(P11ProviderEvidenceDisposition::AlreadyPresent(_))
    ));

    let changed_one = signed_status(
        &status_signing,
        "status-key-1",
        &operation,
        1,
        NOW,
        P11ProviderEvidenceOutcome::Unknown {
            reason_sha256: digest("changed-same-revision"),
        },
    );
    let mut conflict_verifier = status_verifier(&status_record, None, &operation);
    let changed_receipt = match conflict_verifier
        .verify_provider_status(&changed_one, NOW)
        .expect("independently valid changed observation")
    {
        P11ProviderEvidenceDisposition::Applied(receipt) => receipt,
        P11ProviderEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };
    assert_eq!(
        store
            .append_provider_status(&changed_one, &changed_receipt, NOW)
            .await,
        Err(P12Error::EvidenceConflict)
    );
    store
        .append_provider_status(&status_two, &receipt_two, NOW + 1)
        .await
        .expect("terminal status");
    let snapshot = store
        .operation_snapshot(&operation.operation_id)
        .await
        .expect("snapshot");
    assert_eq!(snapshot.state, P11EvidenceState::Completed);
    assert!(snapshot.terminal_retain_until_unix_seconds.is_some());
    store.close().await;

    let reopened = open_store(&root, writer("boot-a", 1), P12Policy::default()).await;
    let mut later_verifier = status_verifier(&status_record, None, &operation);
    for (revision, outcome) in [
        (
            1,
            P11ProviderEvidenceOutcome::Unknown {
                reason_sha256: digest("later-one"),
            },
        ),
        (
            2,
            P11ProviderEvidenceOutcome::Indeterminate {
                reason_sha256: digest("later-two"),
            },
        ),
    ] {
        let evidence = signed_status(
            &status_signing,
            "status-key-1",
            &operation,
            revision,
            NOW + revision,
            outcome,
        );
        later_verifier
            .verify_provider_status(&evidence, NOW + revision)
            .expect("advance later verifier");
    }
    let later = signed_status(
        &status_signing,
        "status-key-1",
        &operation,
        3,
        NOW + 3,
        P11ProviderEvidenceOutcome::Unknown {
            reason_sha256: digest("later-three"),
        },
    );
    let later_receipt = match later_verifier
        .verify_provider_status(&later, NOW + 3)
        .expect("later receipt")
    {
        P11ProviderEvidenceDisposition::Applied(receipt) => receipt,
        P11ProviderEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };
    assert_eq!(
        reopened
            .append_provider_status(&later, &later_receipt, NOW + 3)
            .await,
        Err(P12Error::TerminalImmutable)
    );
    reopened.verify_integrity().await.expect("integrity");
}

#[tokio::test]
async fn manual_evidence_uses_an_independent_revision_ledger_and_lookup_only_resume() {
    let root = TempDir::new().expect("tempdir");
    let status_signing = signing_key(5);
    let operator_signing = signing_key(6);
    let status_record = key_record(
        "provider-status-issuer",
        "status-key-1",
        1,
        P11KeyPurpose::ProviderStatusIssuer,
        &status_signing,
    );
    let operator_record = key_record(
        "operator-issuer",
        "operator-key-1",
        1,
        P11KeyPurpose::OperatorEvidenceIssuer,
        &operator_signing,
    );
    let operation = operation_binding("operation-manual");
    let mut verifier = status_verifier(&status_record, Some(&operator_record), &operation);
    let manual_required = signed_status(
        &status_signing,
        "status-key-1",
        &operation,
        1,
        NOW,
        P11ProviderEvidenceOutcome::ManualRequired {
            reason_sha256: digest("manual-required"),
        },
    );
    let manual_required_receipt = match verifier
        .verify_provider_status(&manual_required, NOW)
        .expect("manual required")
    {
        P11ProviderEvidenceDisposition::Applied(receipt) => receipt,
        P11ProviderEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };
    let manual = signed_manual(
        &operator_signing,
        "operator-key-1",
        &operation,
        1,
        NOW + 1,
        P11ManualDecision::ResumeLookupOnly,
    );
    let manual_receipt = match verifier
        .verify_manual_evidence(&manual, NOW + 1)
        .expect("manual evidence")
    {
        P11ManualEvidenceDisposition::Applied(receipt) => receipt,
        P11ManualEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };
    let resumed = signed_status(
        &status_signing,
        "status-key-1",
        &operation,
        2,
        NOW + 2,
        P11ProviderEvidenceOutcome::Unknown {
            reason_sha256: digest("resumed-lookup"),
        },
    );
    let resumed_receipt = match verifier
        .verify_provider_status(&resumed, NOW + 2)
        .expect("resumed provider status")
    {
        P11ProviderEvidenceDisposition::Applied(receipt) => receipt,
        P11ProviderEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };

    let store = open_store(&root, writer("boot-a", 1), P12Policy::default()).await;
    store
        .register_key(status_record.clone(), NOW)
        .await
        .expect("durable status key");
    store
        .register_key(operator_record.clone(), NOW)
        .await
        .expect("durable operator key");
    store
        .register_operation(operation.clone(), NOW)
        .await
        .expect("operation");
    store
        .append_provider_status(&manual_required, &manual_required_receipt, NOW)
        .await
        .expect("manual required durable");
    store
        .append_manual_evidence(&manual, &manual_receipt, NOW + 1)
        .await
        .expect("manual durable");
    assert!(matches!(
        store
            .append_manual_evidence(&manual, &manual_receipt, NOW + 1)
            .await,
        Ok(P11ManualEvidenceDisposition::AlreadyPresent(_))
    ));
    let resumed_snapshot = store
        .operation_snapshot(&operation.operation_id)
        .await
        .expect("lookup-only snapshot");
    assert_eq!(resumed_snapshot.state, P11EvidenceState::LookupOnly);
    assert_eq!(resumed_snapshot.last_status_revision, Some(1));
    assert_eq!(resumed_snapshot.last_manual_revision, Some(1));
    store
        .append_provider_status(&resumed, &resumed_receipt, NOW + 2)
        .await
        .expect("resumed durable status");
    let final_snapshot = store
        .operation_snapshot(&operation.operation_id)
        .await
        .expect("final snapshot");
    assert_eq!(final_snapshot.state, P11EvidenceState::Unknown);
    assert_eq!(final_snapshot.last_status_revision, Some(2));
    assert_eq!(final_snapshot.last_manual_revision, Some(1));
    assert_eq!(store.status_evidence_count().await, Ok(2));
    assert_eq!(store.manual_evidence_count().await, Ok(1));
}

#[tokio::test]
async fn writer_generation_rebind_fences_every_stale_store_instance() {
    let root = TempDir::new().expect("tempdir");
    let first = open_store(&root, writer("boot-one", 1), P12Policy::default()).await;
    first
        .register_operation(operation_binding("operation-one"), NOW)
        .await
        .expect("first operation");

    let second = open_store(&root, writer("boot-two", 2), P12Policy::default()).await;
    assert_eq!(
        first
            .register_operation(operation_binding("operation-stale"), NOW + 1)
            .await,
        Err(P12Error::StaleWriter)
    );
    second
        .register_operation(operation_binding("operation-two"), NOW + 1)
        .await
        .expect("second writer operation");
    assert_eq!(second.operation_count().await, Ok(2));
    assert!(matches!(
        P12Store::open(
            root.path(),
            writer("different-boot-same-generation", 2),
            P12Policy::default(),
            NOW + 2,
        )
        .await,
        Err(P12Error::StaleWriter)
    ));
    assert!(matches!(
        P12Store::open(
            root.path(),
            writer("old-generation", 1),
            P12Policy::default(),
            NOW + 2,
        )
        .await,
        Err(P12Error::StaleWriter)
    ));
    second.verify_integrity().await.expect("integrity");
}

#[tokio::test]
async fn every_precommit_failpoint_rolls_back_without_partial_replay_state() {
    let root = TempDir::new().expect("tempdir");
    let store = open_store(&root, writer("boot-a", 1), P12Policy::default()).await;
    let identity_signing = signing_key(7);
    let identity_record = key_record(
        "identity-issuer",
        "identity-key-1",
        1,
        P11KeyPurpose::IdentityIssuer,
        &identity_signing,
    );

    store.enable_failpoint(P12Failpoint::KeyBeforeCommit);
    assert_eq!(
        store.register_key(identity_record.clone(), NOW).await,
        Err(P12Error::InjectedFailure)
    );
    store.clear_failpoints();
    assert_eq!(
        store
            .key_record(
                "identity-issuer",
                P11KeyPurpose::IdentityIssuer,
                "identity-key-1",
                1,
            )
            .await,
        Err(P12Error::UnknownKey)
    );
    store
        .register_key(identity_record.clone(), NOW)
        .await
        .expect("key after rollback");

    let identity = signed_identity(
        &identity_signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "failpoint-nonce",
        NOW + 120,
    );
    let identity_receipt = verified_identity(&identity_record, &identity);
    let nonce_claim =
        P12NonceClaim::from_verified(&identity, &identity_receipt, NOW).expect("nonce claim");
    store.enable_failpoint(P12Failpoint::NonceBeforeCommit);
    assert_eq!(
        store.claim_nonce(nonce_claim.clone()).await,
        Err(P12Error::InjectedFailure)
    );
    store.clear_failpoints();
    assert_eq!(store.nonce_count().await, Ok(0));
    store
        .claim_nonce(nonce_claim)
        .await
        .expect("nonce after rollback");

    let operation = operation_binding("operation-failpoint");
    let status_signing = signing_key(8);
    let status_record = key_record(
        "provider-status-issuer",
        "status-key-1",
        1,
        P11KeyPurpose::ProviderStatusIssuer,
        &status_signing,
    );
    store
        .register_key(status_record.clone(), NOW)
        .await
        .expect("durable status key");
    store.enable_failpoint(P12Failpoint::OperationBeforeCommit);
    assert_eq!(
        store.register_operation(operation.clone(), NOW).await,
        Err(P12Error::InjectedFailure)
    );
    store.clear_failpoints();
    assert_eq!(store.operation_count().await, Ok(0));
    store
        .register_operation(operation.clone(), NOW)
        .await
        .expect("operation after rollback");

    let mut verifier = status_verifier(&status_record, None, &operation);
    let status = signed_status(
        &status_signing,
        "status-key-1",
        &operation,
        1,
        NOW,
        P11ProviderEvidenceOutcome::Unknown {
            reason_sha256: digest("failpoint-status"),
        },
    );
    let status_receipt = match verifier
        .verify_provider_status(&status, NOW)
        .expect("status receipt")
    {
        P11ProviderEvidenceDisposition::Applied(receipt) => receipt,
        P11ProviderEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };
    store.enable_failpoint(P12Failpoint::StatusBeforeCommit);
    assert_eq!(
        store
            .append_provider_status(&status, &status_receipt, NOW)
            .await,
        Err(P12Error::InjectedFailure)
    );
    store.clear_failpoints();
    assert_eq!(store.status_evidence_count().await, Ok(0));
    assert_eq!(
        store
            .operation_snapshot(&operation.operation_id)
            .await
            .expect("pending snapshot")
            .state,
        P11EvidenceState::Pending
    );

    store.enable_failpoint(P12Failpoint::StorageUnavailableBeforeCommit);
    assert_eq!(
        store
            .append_provider_status(&status, &status_receipt, NOW)
            .await,
        Err(P12Error::StorageUnavailable)
    );
    store.clear_failpoints();
    assert_eq!(store.status_evidence_count().await, Ok(0));
    store
        .append_provider_status(&status, &status_receipt, NOW)
        .await
        .expect("status after storage rollback");

    let manual_required_status = signed_status(
        &status_signing,
        "status-key-1",
        &operation,
        2,
        NOW + 1,
        P11ProviderEvidenceOutcome::ManualRequired {
            reason_sha256: digest("failpoint-manual-required"),
        },
    );
    let manual_required_receipt = match verifier
        .verify_provider_status(&manual_required_status, NOW + 1)
        .expect("manual-required receipt")
    {
        P11ProviderEvidenceDisposition::Applied(receipt) => receipt,
        P11ProviderEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };
    store
        .append_provider_status(&manual_required_status, &manual_required_receipt, NOW + 1)
        .await
        .expect("manual-required status");

    let operator_signing = signing_key(9);
    let operator_record = key_record(
        "operator-issuer",
        "operator-key-1",
        1,
        P11KeyPurpose::OperatorEvidenceIssuer,
        &operator_signing,
    );
    store
        .register_key(operator_record.clone(), NOW + 1)
        .await
        .expect("durable operator key");
    verifier
        .register_key(operator_record)
        .expect("register operator verifier key");
    let manual = signed_manual(
        &operator_signing,
        "operator-key-1",
        &operation,
        1,
        NOW + 2,
        P11ManualDecision::KeepManualRequired,
    );
    let manual_receipt = match verifier
        .verify_manual_evidence(&manual, NOW + 2)
        .expect("manual receipt")
    {
        P11ManualEvidenceDisposition::Applied(receipt) => receipt,
        P11ManualEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };
    store.enable_failpoint(P12Failpoint::ManualBeforeCommit);
    assert_eq!(
        store
            .append_manual_evidence(&manual, &manual_receipt, NOW + 2)
            .await,
        Err(P12Error::InjectedFailure)
    );
    store.clear_failpoints();
    assert_eq!(store.manual_evidence_count().await, Ok(0));
    store
        .append_manual_evidence(&manual, &manual_receipt, NOW + 2)
        .await
        .expect("manual after rollback");

    store.enable_failpoint(P12Failpoint::GcBeforeCommit);
    assert_eq!(
        store
            .collect_garbage(P12GcRequest {
                expected_revision: 0,
                now_unix_seconds: NOW + 121,
                max_rows: 10,
            })
            .await,
        Err(P12Error::InjectedFailure)
    );
    store.clear_failpoints();
    assert_eq!(store.gc_revision().await, Ok(0));
    assert_eq!(store.nonce_count().await, Ok(1));
    store.verify_integrity().await.expect("integrity");
}

#[tokio::test]
async fn bounded_gc_preserves_live_heads_and_terminal_tombstones_until_deadline() {
    let root = TempDir::new().expect("tempdir");
    let policy = P12Policy {
        evidence_retention_seconds: 10,
        terminal_retention_seconds: 20,
        key_retention_seconds: 10,
        ..P12Policy::default()
    };
    let store = open_store(&root, writer("boot-a", 1), policy).await;

    let old_signing = signing_key(9);
    let current_signing = signing_key(10);
    let old_key = key_record(
        "identity-issuer",
        "identity-key-1",
        1,
        P11KeyPurpose::IdentityIssuer,
        &old_signing,
    );
    let current_key = key_record(
        "identity-issuer",
        "identity-key-2",
        2,
        P11KeyPurpose::IdentityIssuer,
        &current_signing,
    );
    store.register_key(old_key, NOW).await.expect("old key");
    store
        .register_key(current_key, NOW)
        .await
        .expect("current key");
    store
        .revoke_key(
            "identity-issuer",
            P11KeyPurpose::IdentityIssuer,
            "identity-key-1",
            1,
            NOW,
        )
        .await
        .expect("revoke old key");

    let identity_record = key_record(
        "nonce-issuer",
        "nonce-key-1",
        1,
        P11KeyPurpose::IdentityIssuer,
        &old_signing,
    );
    let identity = signed_identity(
        &old_signing,
        "nonce-issuer",
        "nonce-key-1",
        1,
        "gc-nonce",
        NOW + 5,
    );
    let identity_receipt = verified_identity(&identity_record, &identity);
    store
        .register_key(identity_record.clone(), NOW)
        .await
        .expect("durable nonce key");
    store
        .claim_nonce(
            P12NonceClaim::from_verified(&identity, &identity_receipt, NOW).expect("claim"),
        )
        .await
        .expect("nonce");

    let status_signing = signing_key(11);
    let status_record = key_record(
        "provider-status-issuer",
        "status-key-1",
        1,
        P11KeyPurpose::ProviderStatusIssuer,
        &status_signing,
    );
    store
        .register_key(status_record.clone(), NOW)
        .await
        .expect("durable status key");
    let operation = operation_binding("operation-gc-terminal");
    let mut verifier = status_verifier(&status_record, None, &operation);
    let first = signed_status(
        &status_signing,
        "status-key-1",
        &operation,
        1,
        NOW,
        P11ProviderEvidenceOutcome::Unknown {
            reason_sha256: digest("gc-first"),
        },
    );
    let first_receipt = match verifier.verify_provider_status(&first, NOW).expect("first") {
        P11ProviderEvidenceDisposition::Applied(receipt) => receipt,
        P11ProviderEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };
    let terminal = signed_status(
        &status_signing,
        "status-key-1",
        &operation,
        2,
        NOW + 1,
        P11ProviderEvidenceOutcome::Completed {
            result_sha256: digest("gc-terminal"),
        },
    );
    let terminal_receipt = match verifier
        .verify_provider_status(&terminal, NOW + 1)
        .expect("terminal")
    {
        P11ProviderEvidenceDisposition::Applied(receipt) => receipt,
        P11ProviderEvidenceDisposition::AlreadyPresent(_) => panic!("unexpected replay"),
    };
    store
        .register_operation(operation.clone(), NOW)
        .await
        .expect("operation");
    store
        .append_provider_status(&first, &first_receipt, NOW)
        .await
        .expect("first durable");
    store
        .append_provider_status(&terminal, &terminal_receipt, NOW + 1)
        .await
        .expect("terminal durable");

    let first_gc = store
        .collect_garbage(P12GcRequest {
            expected_revision: 0,
            now_unix_seconds: NOW + 15,
            max_rows: 100,
        })
        .await
        .expect("first GC");
    assert_eq!(first_gc.nonce_rows_deleted, 1);
    assert_eq!(first_gc.status_rows_deleted, 1);
    assert_eq!(first_gc.key_rows_deleted, 1);
    assert_eq!(first_gc.terminal_operations_deleted, 0);
    assert_eq!(store.key_count().await, Ok(3));
    assert_eq!(store.status_evidence_count().await, Ok(1));
    assert_eq!(store.operation_count().await, Ok(1));
    assert_eq!(
        store
            .collect_garbage(P12GcRequest {
                expected_revision: 0,
                now_unix_seconds: NOW + 16,
                max_rows: 100,
            })
            .await,
        Err(P12Error::GcConflict)
    );

    let second_gc = store
        .collect_garbage(P12GcRequest {
            expected_revision: 1,
            now_unix_seconds: NOW + 22,
            max_rows: 100,
        })
        .await
        .expect("second GC");
    assert_eq!(second_gc.terminal_operations_deleted, 1);
    assert_eq!(store.operation_count().await, Ok(0));
    assert_eq!(store.status_evidence_count().await, Ok(0));
    store.verify_integrity().await.expect("integrity");
}

#[tokio::test]
async fn row_digest_corruption_fails_integrity_and_reopen_closed() {
    let root = TempDir::new().expect("tempdir");
    let identity_signing = signing_key(12);
    let identity_record = key_record(
        "identity-issuer",
        "identity-key-1",
        1,
        P11KeyPurpose::IdentityIssuer,
        &identity_signing,
    );
    let identity = signed_identity(
        &identity_signing,
        "identity-issuer",
        "identity-key-1",
        1,
        "corrupt-nonce",
        NOW + 120,
    );
    let receipt = verified_identity(&identity_record, &identity);
    let store = open_store(&root, writer("boot-a", 1), P12Policy::default()).await;
    store
        .register_key(identity_record.clone(), NOW)
        .await
        .expect("durable identity key");
    store
        .claim_nonce(P12NonceClaim::from_verified(&identity, &receipt, NOW).expect("claim"))
        .await
        .expect("claim durable");
    store
        .qualification_corrupt_nonce_row()
        .await
        .expect("inject corruption");
    assert_eq!(store.verify_integrity().await, Err(P12Error::CorruptState));
    store.close().await;
    assert!(matches!(
        P12Store::open(
            root.path(),
            writer("boot-a", 1),
            P12Policy::default(),
            NOW + 1,
        )
        .await,
        Err(P12Error::CorruptState)
    ));
}
