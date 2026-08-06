use codex_hepta_contracts::PROVIDER_EVIDENCE_SCHEMA_VERSION;
use codex_hepta_contracts::ProviderInvocationIntent;
use codex_hepta_contracts::ProviderInvocationReceipt;
use codex_hepta_contracts::ProviderRequestBinding;
use codex_hepta_contracts::ProviderRequestKind;
use codex_hepta_contracts::ProviderTerminal;
use codex_hepta_contracts::ProviderTransport;
use codex_hepta_contracts::Sha256Digest;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use tempfile::TempDir;

use crate::AppendDisposition;
use crate::EvidenceError;
use crate::HeptaEvidenceStore;
use crate::ProviderBindingState;
use crate::ProviderIntentClaimDisposition;

fn sqlite_config(temp: &TempDir) -> SqliteConfig {
    SqliteConfig::new_for_testing(
        AbsolutePathBuf::try_from(temp.path().to_path_buf()).expect("absolute temp path"),
    )
}

fn binding(thread_id: &str, logical: &[u8], wire: &[u8]) -> ProviderRequestBinding {
    ProviderRequestBinding {
        schema_version: PROVIDER_EVIDENCE_SCHEMA_VERSION,
        thread_id: thread_id.to_string(),
        turn_id: "turn-1".to_string(),
        host_request_binding_id_sha256: Sha256Digest::for_bytes(b"host-request-binding-1"),
        request_kind: ProviderRequestKind::Turn,
        provider_id: "provider-fixture".to_string(),
        provider_config_sha256: Sha256Digest::for_bytes(b"provider-config"),
        model: "model-fixture".to_string(),
        transport: ProviderTransport::Http,
        endpoint_sha256: Sha256Digest::for_bytes(b"/responses"),
        logical_request_sha256: Sha256Digest::for_bytes(logical),
        ephemeral_input_sha256: None,
        ephemeral_input_witness_sha256: None,
        wire_semantic_sha256: Sha256Digest::for_bytes(wire),
        previous_response_id_sha256: None,
        generate: true,
    }
}

fn intent(nonce: u8) -> ProviderInvocationIntent {
    ProviderInvocationIntent::new([nonce; 16], binding("thread-1", b"logical", b"wire"))
}

fn completed(intent: ProviderInvocationIntent, output: &[u8]) -> ProviderInvocationReceipt {
    ProviderInvocationReceipt::new(
        intent,
        ProviderTerminal::Completed {
            response_id_sha256: Sha256Digest::for_bytes(b"response-id"),
            response_items_sha256: Sha256Digest::for_bytes(output),
            token_usage_sha256: Sha256Digest::for_bytes(b"token-usage"),
            end_turn: Some(true),
        },
    )
}

#[tokio::test]
async fn provider_intent_requires_exact_prompt_only_input_witness_pair() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    for witness_only in [false, true] {
        let mut binding = binding("thread-1", b"logical", b"wire");
        if witness_only {
            binding.ephemeral_input_witness_sha256 = Some(Sha256Digest::for_bytes(b"witness"));
        } else {
            binding.ephemeral_input_sha256 = Some(Sha256Digest::for_bytes(b"input"));
        }
        let intent = ProviderInvocationIntent::new([u8::from(witness_only); 16], binding);
        assert!(matches!(
            store
                .append_provider_intent(&intent)
                .await
                .expect_err("partial prompt-only authority must fail closed"),
            EvidenceError::InvalidRecord(_)
        ));
    }
    assert_eq!(
        store
            .pending_provider_attempt_count()
            .await
            .expect("pending count"),
        0
    );
}

#[tokio::test]
async fn rotating_prompt_only_witness_cannot_bypass_host_request_replay_state() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let mut first_binding = binding("thread-1", b"logical", b"wire");
    first_binding.ephemeral_input_sha256 = Some(Sha256Digest::for_bytes(b"input"));
    first_binding.ephemeral_input_witness_sha256 = Some(Sha256Digest::for_bytes(b"witness-1"));
    let first = ProviderInvocationIntent::new([11; 16], first_binding.clone());
    let mut retry_binding = first_binding;
    retry_binding.ephemeral_input_witness_sha256 = Some(Sha256Digest::for_bytes(b"witness-2"));
    let retry = ProviderInvocationIntent::new([12; 16], retry_binding);
    assert_ne!(first.request_binding_id, retry.request_binding_id);
    assert_eq!(
        first.binding.host_request_binding_id_sha256,
        retry.binding.host_request_binding_id_sha256
    );

    assert_eq!(
        store
            .claim_provider_intent(&first)
            .await
            .expect("first claim"),
        ProviderIntentClaimDisposition::Inserted
    );
    store
        .append_provider_receipt(&completed(first, b"output"))
        .await
        .expect("completed terminal");
    assert_eq!(
        store
            .claim_provider_intent(&retry)
            .await
            .expect("retry claim"),
        ProviderIntentClaimDisposition::BlockedByBinding(ProviderBindingState::Completed)
    );
}

#[tokio::test]
async fn provider_records_are_idempotent_pending_and_persistent() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let intent = intent(1);
    let receipt = completed(intent.clone(), b"output");

    assert_eq!(
        store.append_provider_intent(&intent).await.expect("intent"),
        AppendDisposition::Inserted
    );
    assert_eq!(
        store
            .append_provider_intent(&intent)
            .await
            .expect("intent replay"),
        AppendDisposition::AlreadyPresent
    );
    assert_eq!(
        store
            .pending_provider_attempt_count()
            .await
            .expect("pending count"),
        1
    );
    let pending = store
        .list_pending_provider_intents("thread-1", 0, 10)
        .await
        .expect("pending intents");
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].intent, intent);

    assert_eq!(
        store
            .append_provider_receipt(&receipt)
            .await
            .expect("terminal"),
        AppendDisposition::Inserted
    );
    assert_eq!(
        store
            .append_provider_receipt(&receipt)
            .await
            .expect("terminal replay"),
        AppendDisposition::AlreadyPresent
    );
    assert_eq!(
        store
            .pending_provider_attempt_count()
            .await
            .expect("terminal count"),
        0
    );
    drop(store);

    let reopened = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("reopen evidence");
    let stored = reopened
        .get_provider_attempt(&receipt.attempt_id)
        .await
        .expect("read provider attempt")
        .expect("provider attempt");
    assert_eq!(stored.intent.intent, intent);
    assert_eq!(stored.receipt.expect("provider terminal").receipt, receipt);
}

#[tokio::test]
async fn provider_terminal_conflict_never_overwrites_original() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let intent = intent(2);
    let completed = completed(intent.clone(), b"first-output");
    let rejected = ProviderInvocationReceipt::new(
        intent.clone(),
        ProviderTerminal::Rejected {
            reason_code: "provider_unauthorized".to_string(),
        },
    );
    store.append_provider_intent(&intent).await.expect("intent");
    store
        .append_provider_receipt(&completed)
        .await
        .expect("first terminal");

    assert!(matches!(
        store
            .append_provider_receipt(&rejected)
            .await
            .expect_err("different terminal for one attempt must conflict"),
        EvidenceError::IdempotencyConflict { .. }
    ));
    let stored = store
        .get_provider_receipt(&completed.receipt_id)
        .await
        .expect("read terminal")
        .expect("stored terminal");
    assert_eq!(stored.receipt, completed);
}

#[tokio::test]
async fn concurrent_provider_intent_replay_inserts_exactly_once() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let first = HeptaEvidenceStore::open(&sqlite).await.expect("first pool");
    let second = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("second pool");
    let intent = intent(3);

    let (left, right) = tokio::join!(
        first.append_provider_intent(&intent),
        second.append_provider_intent(&intent)
    );
    let dispositions = [left.expect("left append"), right.expect("right append")];
    assert!(dispositions.contains(&AppendDisposition::Inserted));
    assert!(dispositions.contains(&AppendDisposition::AlreadyPresent));
    assert_eq!(
        first
            .pending_provider_attempt_count()
            .await
            .expect("pending count"),
        1
    );
}

#[tokio::test]
async fn concurrent_enforced_claims_for_one_request_binding_have_one_owner() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let first = HeptaEvidenceStore::open(&sqlite).await.expect("first pool");
    let second = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("second pool");
    let left_intent = intent(31);
    let right_intent = intent(32);
    assert_eq!(
        left_intent.request_binding_id,
        right_intent.request_binding_id
    );

    let (left, right) = tokio::join!(
        first.claim_provider_intent(&left_intent),
        second.claim_provider_intent(&right_intent)
    );
    let dispositions = [left.expect("left claim"), right.expect("right claim")];
    assert_eq!(
        dispositions
            .iter()
            .filter(|disposition| { **disposition == ProviderIntentClaimDisposition::Inserted })
            .count(),
        1
    );
    assert_eq!(
        dispositions
            .iter()
            .filter(|disposition| {
                **disposition
                    == ProviderIntentClaimDisposition::BlockedByBinding(
                        ProviderBindingState::Pending,
                    )
            })
            .count(),
        1
    );
    assert_eq!(
        first
            .pending_provider_attempt_count()
            .await
            .expect("pending count"),
        1
    );
}

#[tokio::test]
async fn enforced_claim_blocks_same_host_request_across_transport_encoding_changes() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let first = intent(41);
    let mut retry_binding = binding("thread-1", b"logical", b"incremental-wire");
    retry_binding.transport = ProviderTransport::WebSocket;
    retry_binding.previous_response_id_sha256 = Some(Sha256Digest::for_bytes(b"previous-response"));
    let retry = ProviderInvocationIntent::new([42; 16], retry_binding);
    assert_ne!(first.request_binding_id, retry.request_binding_id);
    assert_eq!(
        first.binding.host_request_binding_id_sha256,
        retry.binding.host_request_binding_id_sha256
    );

    assert_eq!(
        store
            .claim_provider_intent(&first)
            .await
            .expect("first claim"),
        ProviderIntentClaimDisposition::Inserted
    );
    assert_eq!(
        store
            .claim_provider_intent(&retry)
            .await
            .expect("retry claim"),
        ProviderIntentClaimDisposition::BlockedByBinding(ProviderBindingState::Pending)
    );

    let not_dispatched = ProviderInvocationReceipt::new(
        first,
        ProviderTerminal::NotDispatched {
            reason_code: "transport_not_entered".to_string(),
        },
    );
    store
        .append_provider_receipt(&not_dispatched)
        .await
        .expect("not-dispatched terminal");
    assert_eq!(
        store
            .claim_provider_intent(&retry)
            .await
            .expect("retry after not-dispatched"),
        ProviderIntentClaimDisposition::Inserted
    );
}

#[tokio::test]
async fn concurrent_provider_terminal_conflict_preserves_one_terminal() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let first = HeptaEvidenceStore::open(&sqlite).await.expect("first pool");
    let second = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("second pool");
    let intent = intent(4);
    first.append_provider_intent(&intent).await.expect("intent");
    let completed = completed(intent.clone(), b"completed-output");
    let indeterminate = ProviderInvocationReceipt::new(
        intent,
        ProviderTerminal::Indeterminate {
            reason_code: "stream_eof_before_completed".to_string(),
            partial_response_sha256: Some(Sha256Digest::for_bytes(b"partial")),
        },
    );

    let (left, right) = tokio::join!(
        first.append_provider_receipt(&completed),
        second.append_provider_receipt(&indeterminate)
    );
    let inserted = usize::from(matches!(left, Ok(AppendDisposition::Inserted)))
        + usize::from(matches!(right, Ok(AppendDisposition::Inserted)));
    let conflicts = usize::from(matches!(
        left,
        Err(EvidenceError::IdempotencyConflict { .. })
    )) + usize::from(matches!(
        right,
        Err(EvidenceError::IdempotencyConflict { .. })
    ));
    assert_eq!(inserted, 1, "left={left:?}, right={right:?}");
    assert_eq!(conflicts, 1, "left={left:?}, right={right:?}");
    assert_eq!(
        first
            .pending_provider_attempt_count()
            .await
            .expect("pending count"),
        0
    );
}

#[tokio::test]
async fn provider_terminal_requires_exact_durable_intent() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let receipt = completed(intent(5), b"output");

    assert!(matches!(
        store
            .append_provider_receipt(&receipt)
            .await
            .expect_err("terminal without durable intent must fail"),
        EvidenceError::Corrupt(_)
    ));
    assert_eq!(
        store
            .pending_provider_attempt_count()
            .await
            .expect("pending count"),
        0
    );
}

#[tokio::test]
async fn provider_tables_are_immutable_and_foreign_key_bound() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let intent = intent(6);
    let receipt = completed(intent.clone(), b"output");
    store.append_provider_intent(&intent).await.expect("intent");
    store
        .append_provider_receipt(&receipt)
        .await
        .expect("terminal");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");

    for statement in [
        "UPDATE provider_invocation_intents SET recorded_at_ms = recorded_at_ms + 1",
        "DELETE FROM provider_invocation_intents",
        "UPDATE provider_invocation_terminals SET recorded_at_ms = recorded_at_ms + 1",
        "DELETE FROM provider_invocation_terminals",
    ] {
        let error = sqlx::query(statement)
            .execute(&raw)
            .await
            .expect_err("provider evidence mutation must fail");
        assert!(error.to_string().contains("immutable"));
    }

    let error = sqlx::query(
        "INSERT INTO provider_invocation_terminals (
            receipt_id, attempt_id, request_binding_id, thread_id, turn_id,
            terminal_kind, schema_version, payload_json, payload_sha256, recorded_at_ms
         ) VALUES (?, ?, ?, ?, ?, 'indeterminate', 1, '{}', ?, 0)",
    )
    .bind(format!("provider-receipt:v1:{}", "a".repeat(64)))
    .bind(format!("provider-attempt:v1:{}", "b".repeat(64)))
    .bind(format!("provider-request:v1:{}", "c".repeat(64)))
    .bind("thread-1")
    .bind("turn-1")
    .bind("0".repeat(64))
    .execute(&raw)
    .await
    .expect_err("terminal must reference its exact intent");
    assert!(error.to_string().contains("FOREIGN KEY constraint failed"));
}

#[tokio::test]
async fn provider_evidence_never_persists_prompt_token_header_or_output_plaintext() {
    const PROMPT: &[u8] = b"fixture-prompt-ultra-private-871";
    const TOKEN: &[u8] = b"fixture-bearer-token-ultra-private-872";
    const HEADER: &[u8] = b"fixture-auth-header-ultra-private-873";
    const OUTPUT: &[u8] = b"fixture-provider-output-ultra-private-874";
    const ENDPOINT: &[u8] = b"https://provider.invalid/responses?secret=875";

    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let mut binding = binding("thread-secret-test", PROMPT, HEADER);
    binding.provider_config_sha256 = Sha256Digest::for_bytes(TOKEN);
    binding.endpoint_sha256 = Sha256Digest::for_bytes(ENDPOINT);
    let intent = ProviderInvocationIntent::new([7; 16], binding);
    let receipt = completed(intent.clone(), OUTPUT);
    store.append_provider_intent(&intent).await.expect("intent");
    store
        .append_provider_receipt(&receipt)
        .await
        .expect("terminal");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    let rows: Vec<String> = sqlx::query_scalar(
        "SELECT payload_json FROM provider_invocation_intents
         UNION ALL
         SELECT payload_json FROM provider_invocation_terminals",
    )
    .fetch_all(&raw)
    .await
    .expect("provider JSON rows");
    let joined = rows.join("\n");
    for forbidden in [PROMPT, TOKEN, HEADER, OUTPUT, ENDPOINT] {
        assert!(
            !joined
                .as_bytes()
                .windows(forbidden.len())
                .any(|w| w == forbidden)
        );
    }
}

#[tokio::test]
async fn provider_projection_and_digest_corruption_fail_closed() {
    for (nonce, corruption) in [(8, "projection"), (9, "digest")] {
        let temp = TempDir::new().expect("temp dir");
        let sqlite = sqlite_config(&temp);
        let store = HeptaEvidenceStore::open(&sqlite)
            .await
            .expect("open evidence");
        let intent = intent(nonce);
        store.append_provider_intent(&intent).await.expect("intent");
        let raw = sqlite
            .open_durable_evidence_pool(store.path())
            .await
            .expect("raw evidence pool");
        sqlx::query("DROP TRIGGER provider_invocation_intents_no_update")
            .execute(&raw)
            .await
            .expect("disable immutable trigger for corruption simulation");
        if corruption == "projection" {
            sqlx::query(
                "UPDATE provider_invocation_intents SET provider_id = 'drifted-provider'
                 WHERE attempt_id = ?",
            )
            .bind(intent.attempt_id.as_str())
            .execute(&raw)
            .await
            .expect("corrupt provider projection");
        } else {
            sqlx::query(
                "UPDATE provider_invocation_intents SET payload_sha256 = ? WHERE attempt_id = ?",
            )
            .bind("0".repeat(64))
            .bind(intent.attempt_id.as_str())
            .execute(&raw)
            .await
            .expect("corrupt provider digest");
        }

        assert!(matches!(
            store
                .get_provider_attempt(&intent.attempt_id)
                .await
                .expect_err("provider corruption must fail closed"),
            EvidenceError::Corrupt(_)
        ));
    }
}

#[tokio::test]
async fn open_rejects_missing_provider_schema_object() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    sqlx::query("DROP TRIGGER provider_invocation_terminals_no_delete")
        .execute(&raw)
        .await
        .expect("drop provider immutable trigger");
    raw.close().await;
    drop(store);

    let error = match HeptaEvidenceStore::open(&sqlite).await {
        Ok(_) => panic!("missing provider schema object must be rejected"),
        Err(error) => error,
    };
    assert!(matches!(error, EvidenceError::Corrupt(_)));
}

#[tokio::test]
async fn open_rejects_legacy_provider_rows_without_host_binding_digest() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    sqlx::query("DROP TRIGGER provider_invocation_intents_host_binding_required")
        .execute(&raw)
        .await
        .expect("drop host binding trigger");
    sqlx::query(
        "INSERT INTO provider_invocation_intents (
            attempt_id, request_binding_id, attempt_nonce_sha256, thread_id, turn_id,
            request_kind, provider_id, provider_config_sha256, model, transport,
            endpoint_sha256, logical_request_sha256, wire_semantic_sha256,
            previous_response_id_sha256, generate, schema_version,
            payload_json, payload_sha256, recorded_at_ms
         ) VALUES (?, ?, ?, ?, ?, 'turn', ?, ?, ?, 'http', ?, ?, ?, NULL, 1, 1, '{}', ?, 0)",
    )
    .bind(format!("provider-attempt:v1:{}", "1".repeat(64)))
    .bind(format!("provider-request:v1:{}", "2".repeat(64)))
    .bind("3".repeat(64))
    .bind("thread-legacy")
    .bind("turn-legacy")
    .bind("provider-legacy")
    .bind("4".repeat(64))
    .bind("model-legacy")
    .bind("5".repeat(64))
    .bind("6".repeat(64))
    .bind("7".repeat(64))
    .bind(Sha256Digest::for_bytes(b"{}").as_str())
    .execute(&raw)
    .await
    .expect("insert legacy row");
    sqlx::query(
        "CREATE TRIGGER provider_invocation_intents_host_binding_required
         BEFORE INSERT ON provider_invocation_intents
         WHEN NEW.host_request_binding_id_sha256 IS NULL
         BEGIN
             SELECT RAISE(ABORT, 'provider invocation intent requires host request binding digest');
         END",
    )
    .execute(&raw)
    .await
    .expect("restore host binding trigger");
    raw.close().await;
    drop(store);

    let error = match HeptaEvidenceStore::open(&sqlite).await {
        Ok(_) => panic!("legacy unbound row must require explicit migration"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        EvidenceError::Corrupt(detail)
            if detail.contains("predate host request binding evidence")
    ));
}
