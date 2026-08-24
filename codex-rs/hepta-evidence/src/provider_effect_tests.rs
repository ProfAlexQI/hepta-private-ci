use codex_hepta_contracts::PROVIDER_EVIDENCE_SCHEMA_VERSION;
use codex_hepta_contracts::ProviderEffectAck;
use codex_hepta_contracts::ProviderEffectAckStatus;
use codex_hepta_contracts::ProviderEffectIdempotencyCapability;
use codex_hepta_contracts::ProviderEffectIntent;
use codex_hepta_contracts::ProviderEffectKey;
use codex_hepta_contracts::ProviderEffectLookup;
use codex_hepta_contracts::ProviderEffectState;
use codex_hepta_contracts::ProviderEffectUncertainty;
use codex_hepta_contracts::ProviderRequestBinding;
use codex_hepta_contracts::ProviderRequestKind;
use codex_hepta_contracts::ProviderTransport;
use codex_hepta_contracts::RequestBindingId;
use codex_hepta_contracts::Sha256Digest;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use tempfile::TempDir;

use crate::AppendDisposition;
use crate::EvidenceError;
use crate::HeptaEvidenceStore;

fn sqlite_config(temp: &TempDir) -> SqliteConfig {
    SqliteConfig::new_for_testing(
        AbsolutePathBuf::try_from(temp.path().to_path_buf()).expect("absolute temp path"),
    )
}

fn request_binding_id() -> RequestBindingId {
    RequestBindingId::for_request(&ProviderRequestBinding {
        schema_version: PROVIDER_EVIDENCE_SCHEMA_VERSION,
        thread_id: "thread-effect".to_string(),
        turn_id: "turn-effect".to_string(),
        host_request_binding_id_sha256: Sha256Digest::for_bytes(b"host-effect"),
        request_kind: ProviderRequestKind::Turn,
        provider_id: "provider-effect-fixture".to_string(),
        provider_config_sha256: Sha256Digest::for_bytes(b"config-effect"),
        model: "effect-model".to_string(),
        transport: ProviderTransport::Http,
        endpoint_sha256: Sha256Digest::for_bytes(b"/effect"),
        logical_request_sha256: Sha256Digest::for_bytes(b"logical-effect"),
        wire_semantic_sha256: Sha256Digest::for_bytes(b"wire-effect"),
        ephemeral_input_sha256: None,
        ephemeral_input_witness_sha256: None,
        previous_response_id_sha256: None,
        generate: true,
    })
}

fn effect_intent(payload: &[u8]) -> ProviderEffectIntent {
    let key = ProviderEffectKey::for_occurrence(
        "provider-effect-fixture/config-v1",
        "automation:agent-a:occurrence-1",
        &request_binding_id(),
    )
    .expect("effect key");
    ProviderEffectIntent::new(key, Sha256Digest::for_bytes(payload))
}

fn completed_ack(intent: &ProviderEffectIntent, operation: &[u8]) -> ProviderEffectAck {
    ProviderEffectAck::new(
        intent.key.clone(),
        intent.payload_sha256.clone(),
        Sha256Digest::for_bytes(operation),
        ProviderEffectAckStatus::Completed,
    )
}

#[tokio::test]
async fn effect_journal_quarantines_unknown_and_reconciles_after_restart() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let intent = effect_intent(b"effect-payload");
    let key = intent.key.clone();

    assert_eq!(
        store
            .append_provider_effect_intent(&intent)
            .await
            .expect("intent"),
        AppendDisposition::Inserted
    );
    assert_eq!(
        store
            .append_provider_effect_intent(&intent)
            .await
            .expect("intent replay"),
        AppendDisposition::AlreadyPresent
    );
    assert_eq!(
        store
            .reconcile_provider_effect_lookup(
                ProviderEffectIdempotencyCapability::KeyAndStatusLookup,
                &key,
                ProviderEffectLookup::Unknown,
            )
            .await
            .expect("unknown is quarantined"),
        ProviderEffectState::Indeterminate
    );
    let quarantined = store
        .get_provider_effect(&key)
        .await
        .expect("read quarantine")
        .expect("effect");
    assert_eq!(quarantined.state(), ProviderEffectState::Indeterminate);
    assert_eq!(quarantined.uncertainties.len(), 1);
    assert!(quarantined.acknowledgements.is_empty());

    assert_eq!(
        store
            .append_provider_effect_ack(&completed_ack(&intent, b"operation-after-unknown"))
            .await
            .expect("reconciled completion"),
        AppendDisposition::Inserted
    );
    drop(store);

    let reopened = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("reopen evidence");
    let completed = reopened
        .get_provider_effect(&key)
        .await
        .expect("read completed")
        .expect("effect");
    assert_eq!(completed.state(), ProviderEffectState::Completed);
    assert_eq!(completed.uncertainties.len(), 1);
    assert_eq!(completed.acknowledgements.len(), 1);
}

#[tokio::test]
async fn accepted_effect_stays_indeterminate_when_reconcile_reports_rejected() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let intent = effect_intent(b"accepted-then-rejected");
    let key = intent.key.clone();
    store
        .append_provider_effect_intent(&intent)
        .await
        .expect("intent");
    let accepted = ProviderEffectAck::new(
        key.clone(),
        intent.payload_sha256.clone(),
        Sha256Digest::for_bytes(b"operation-accepted"),
        ProviderEffectAckStatus::Accepted,
    );
    store
        .append_provider_effect_ack(&accepted)
        .await
        .expect("accepted");
    assert_eq!(
        store
            .reconcile_provider_effect_lookup(
                ProviderEffectIdempotencyCapability::KeyAndStatusLookup,
                &key,
                ProviderEffectLookup::Unknown,
            )
            .await
            .expect("unknown quarantine"),
        ProviderEffectState::Indeterminate
    );
    let rejected = ProviderEffectAck::new(
        key.clone(),
        intent.payload_sha256.clone(),
        Sha256Digest::for_bytes(b"operation-authoritative"),
        ProviderEffectAckStatus::Rejected,
    );
    let error = store
        .append_provider_effect_ack(&rejected)
        .await
        .expect_err("accepted -> rejected must remain fail-closed");
    assert!(matches!(error, EvidenceError::IdempotencyConflict { .. }));
    let effect = store
        .get_provider_effect(&key)
        .await
        .expect("read effect")
        .expect("effect");
    assert_eq!(effect.state(), ProviderEffectState::Indeterminate);
    assert_eq!(effect.acknowledgements.len(), 1);
    assert_eq!(effect.uncertainties.len(), 1);
}

#[tokio::test]
async fn unsupported_provider_capability_is_durable_indeterminate() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let intent = effect_intent(b"unsupported-payload");
    let key = intent.key.clone();
    store
        .append_provider_effect_intent(&intent)
        .await
        .expect("intent");

    let error = store
        .reconcile_provider_effect_lookup(
            ProviderEffectIdempotencyCapability::Unsupported,
            &key,
            ProviderEffectLookup::Unknown,
        )
        .await
        .expect_err("unsupported lookup must fail closed");
    assert!(matches!(error, EvidenceError::InvalidRecord(_)));
    let effect = store
        .get_provider_effect(&key)
        .await
        .expect("read effect")
        .expect("effect");
    assert_eq!(effect.state(), ProviderEffectState::Indeterminate);
    assert_eq!(
        effect.uncertainties[0].uncertainty.reason_code,
        "provider_capability_unsupported"
    );
}

#[tokio::test]
async fn terminal_effect_cannot_be_quarantined_or_replaced() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let intent = effect_intent(b"terminal-payload");
    let key = intent.key.clone();
    store
        .append_provider_effect_intent(&intent)
        .await
        .expect("intent");
    let completion = completed_ack(&intent, b"terminal-operation");
    store
        .append_provider_effect_ack(&completion)
        .await
        .expect("completion");

    let quarantine = store
        .mark_provider_effect_indeterminate(&key, "late_network_unknown")
        .await
        .expect_err("terminal cannot be quarantined");
    assert!(matches!(
        quarantine,
        EvidenceError::IdempotencyConflict { .. }
    ));
    let replacement = ProviderEffectAck::new(
        key,
        intent.payload_sha256,
        Sha256Digest::for_bytes(b"other-operation"),
        ProviderEffectAckStatus::Rejected,
    );
    let error = store
        .append_provider_effect_ack(&replacement)
        .await
        .expect_err("terminal cannot be replaced");
    assert!(matches!(error, EvidenceError::IdempotencyConflict { .. }));
}

#[tokio::test]
async fn reopen_rejects_late_uncertainty_after_terminal_ack() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let intent = effect_intent(b"late-uncertainty-payload");
    let key = intent.key.clone();
    store
        .append_provider_effect_intent(&intent)
        .await
        .expect("intent");
    store
        .append_provider_effect_ack(&completed_ack(&intent, b"terminal-operation"))
        .await
        .expect("terminal ACK");

    // Model a damaged/imported store that bypassed the public append guard.
    // A late quarantine row must not downgrade a terminal result to
    // Indeterminate when the evidence store is reopened.
    let uncertainty = ProviderEffectUncertainty::new(
        key.clone(),
        intent.payload_sha256.clone(),
        "late_network_unknown",
    );
    let uncertainty_bytes =
        crate::canonical::canonical_json(&uncertainty).expect("canonical uncertainty");
    let uncertainty_json = String::from_utf8(uncertainty_bytes.clone()).expect("uncertainty JSON");
    let uncertainty_record_sha = Sha256Digest::for_bytes(&uncertainty_bytes);
    sqlx::query(
        "INSERT INTO provider_effect_uncertainties (
            effect_key, payload_sha256, reason_code, schema_version,
            payload_json, record_sha256, recorded_at_ms
         ) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(key.as_str())
    .bind(uncertainty.payload_sha256.as_str())
    .bind(&uncertainty.reason_code)
    .bind(i64::from(uncertainty.schema_version))
    .bind(&uncertainty_json)
    .bind(uncertainty_record_sha.as_str())
    .bind(i64::MAX)
    .execute(&store.pool)
    .await
    .expect("insert damaged late uncertainty");
    drop(store);

    let reopen = HeptaEvidenceStore::open(&sqlite).await;
    assert!(matches!(
        reopen,
        Err(EvidenceError::Corrupt(detail))
            if detail.contains("uncertainty follows terminal ACK")
    ));
}

#[tokio::test]
async fn same_key_payload_conflict_and_ack_binding_fail_closed() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let intent = effect_intent(b"authoritative-payload");
    let key = intent.key.clone();
    assert_eq!(
        store
            .append_provider_effect_intent(&intent)
            .await
            .expect("intent"),
        AppendDisposition::Inserted
    );

    // A provider/client retry may replay the exact occurrence, but it must
    // not reuse the durable key for a different payload.
    let conflicting_intent = effect_intent(b"different-payload");
    let conflict = store
        .append_provider_effect_intent(&conflicting_intent)
        .await
        .expect_err("same-key/different-payload must be rejected");
    assert!(matches!(
        conflict,
        EvidenceError::IdempotencyConflict { .. }
    ));

    // An ACK is independently bound to both the occurrence key and the
    // exact payload digest; a provider response for another payload cannot
    // close this intent even when the occurrence key matches.
    let mismatched_ack = ProviderEffectAck::new(
        key.clone(),
        Sha256Digest::for_bytes(b"different-payload"),
        Sha256Digest::for_bytes(b"provider-operation"),
        ProviderEffectAckStatus::Completed,
    );
    let binding_error = store
        .append_provider_effect_ack(&mismatched_ack)
        .await
        .expect_err("payload-mismatched ACK must be rejected");
    assert!(matches!(binding_error, EvidenceError::InvalidRecord(_)));

    let valid_ack = completed_ack(&intent, b"provider-operation");
    assert_eq!(
        store
            .append_provider_effect_ack(&valid_ack)
            .await
            .expect("valid ACK"),
        AppendDisposition::Inserted
    );
    assert_eq!(
        store
            .append_provider_effect_ack(&valid_ack)
            .await
            .expect("exact ACK replay"),
        AppendDisposition::AlreadyPresent
    );
}

#[tokio::test]
async fn effect_tables_are_append_only() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let intent = effect_intent(b"immutable-payload");
    let key = intent.key.clone();
    store
        .append_provider_effect_intent(&intent)
        .await
        .expect("intent");
    store
        .mark_provider_effect_indeterminate(&key, "network_unknown")
        .await
        .expect("quarantine");

    let update = sqlx::query(
        "UPDATE provider_effect_uncertainties SET reason_code = 'tampered' WHERE effect_key = ?",
    )
    .bind(key.as_str())
    .execute(&store.pool)
    .await;
    assert!(update.is_err());
    let delete = sqlx::query("DELETE FROM provider_effect_uncertainties WHERE effect_key = ?")
        .bind(key.as_str())
        .execute(&store.pool)
        .await;
    assert!(delete.is_err());
}

#[tokio::test]
async fn reopen_rejects_illegal_ack_even_when_late_uncertainty_exists() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let intent = effect_intent(b"corrupt-transition-payload");
    let key = intent.key.clone();
    store
        .append_provider_effect_intent(&intent)
        .await
        .expect("intent");
    let accepted = ProviderEffectAck::new(
        key.clone(),
        intent.payload_sha256.clone(),
        Sha256Digest::for_bytes(b"operation-one"),
        ProviderEffectAckStatus::Accepted,
    );
    store
        .append_provider_effect_ack(&accepted)
        .await
        .expect("accepted");

    // Bypass the public transition guard to model a damaged/imported store.
    // The reopen verifier must reject it rather than allowing a later
    // uncertainty row to retroactively legalize the operation-id change.
    let illegal = ProviderEffectAck::new(
        key.clone(),
        intent.payload_sha256.clone(),
        Sha256Digest::for_bytes(b"operation-two"),
        ProviderEffectAckStatus::Completed,
    );
    let illegal_bytes = crate::canonical::canonical_json(&illegal).expect("canonical ACK");
    let illegal_json = String::from_utf8(illegal_bytes.clone()).expect("ACK JSON");
    let illegal_record_sha = Sha256Digest::for_bytes(&illegal_bytes);
    sqlx::query(
        "INSERT INTO provider_effect_acknowledgements (
            effect_key, payload_sha256, provider_operation_id_sha256,
            status, schema_version, payload_json, record_sha256, recorded_at_ms
         ) VALUES (?, ?, ?, 'completed', ?, ?, ?, 200)",
    )
    .bind(key.as_str())
    .bind(illegal.payload_sha256.as_str())
    .bind(illegal.provider_operation_id_sha256.as_str())
    .bind(i64::from(illegal.schema_version))
    .bind(&illegal_json)
    .bind(illegal_record_sha.as_str())
    .execute(&store.pool)
    .await
    .expect("insert damaged ACK");

    let uncertainty = ProviderEffectUncertainty::new(
        key.clone(),
        intent.payload_sha256.clone(),
        "late_uncertainty",
    );
    let uncertainty_bytes =
        crate::canonical::canonical_json(&uncertainty).expect("canonical uncertainty");
    let uncertainty_json = String::from_utf8(uncertainty_bytes.clone()).expect("uncertainty JSON");
    let uncertainty_record_sha = Sha256Digest::for_bytes(&uncertainty_bytes);
    sqlx::query(
        "INSERT INTO provider_effect_uncertainties (
            effect_key, payload_sha256, reason_code, schema_version,
            payload_json, record_sha256, recorded_at_ms
         ) VALUES (?, ?, ?, ?, ?, ?, 300)",
    )
    .bind(key.as_str())
    .bind(uncertainty.payload_sha256.as_str())
    .bind(&uncertainty.reason_code)
    .bind(i64::from(uncertainty.schema_version))
    .bind(&uncertainty_json)
    .bind(uncertainty_record_sha.as_str())
    .execute(&store.pool)
    .await
    .expect("insert late uncertainty");
    drop(store);

    let reopen = HeptaEvidenceStore::open(&sqlite).await;
    assert!(matches!(reopen, Err(EvidenceError::Corrupt(_))));
}
