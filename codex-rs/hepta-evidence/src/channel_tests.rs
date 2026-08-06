use codex_hepta_contracts::CHANNEL_EVIDENCE_SCHEMA_VERSION;
use codex_hepta_contracts::ChannelAdapterId;
use codex_hepta_contracts::ChannelIngressEvent;
use codex_hepta_contracts::ChannelIngressReceipt;
use codex_hepta_contracts::ChannelIngressTerminal;
use codex_hepta_contracts::ChannelScope;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::channel_target_thread_sha256;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use tempfile::TempDir;

use crate::AppendDisposition;
use crate::ChannelIngressClaimDisposition;
use crate::ChannelIngressState;
use crate::EvidenceError;
use crate::HeptaEvidenceStore;

fn sqlite_config(temp: &TempDir) -> SqliteConfig {
    SqliteConfig::new_for_testing(
        AbsolutePathBuf::try_from(temp.path().to_path_buf()).expect("absolute temp path"),
    )
}

fn digest(value: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(value.as_bytes())
}

fn scope() -> ChannelScope {
    ChannelScope {
        adapter_id: ChannelAdapterId::new("fixture-adapter").expect("valid adapter"),
        installation_sha256: digest("installation"),
        account_sha256: digest("account"),
        conversation_sha256: digest("conversation"),
        principal_sha256: digest("principal"),
    }
}

fn ingress(
    event_number: u64,
    predecessor_cursor_sha256: Option<Sha256Digest>,
) -> ChannelIngressEvent {
    ChannelIngressEvent::new(
        scope(),
        digest(&format!("source-event-{event_number}")),
        digest(&format!("payload-{event_number}")),
        channel_target_thread_sha256("thread-1").expect("target thread"),
        predecessor_cursor_sha256,
        digest(&format!("cursor-{event_number}")),
        10_000 + event_number,
    )
    .expect("valid ingress event")
}

fn accepted(event: ChannelIngressEvent) -> ChannelIngressReceipt {
    ChannelIngressReceipt::new(
        event,
        ChannelIngressTerminal::Accepted {
            thread_id: "thread-1".to_string(),
            turn_id: "turn-1".to_string(),
        },
    )
}

fn rejected(event: ChannelIngressEvent) -> ChannelIngressReceipt {
    ChannelIngressReceipt::new(
        event,
        ChannelIngressTerminal::Rejected {
            reason_code: "unsupported_event".to_string(),
        },
    )
}

async fn insert_raw_event(pool: &sqlx::SqlitePool, event: &ChannelIngressEvent) {
    let payload = crate::canonical::canonical_json(event).expect("canonical event");
    let payload_json = String::from_utf8(payload).expect("UTF-8 event");
    let scope_sha256 = event.scope.binding_sha256();
    sqlx::query(
        "INSERT INTO channel_ingress_events (
            event_id, scope_sha256, adapter_id, source_event_sha256,
            event_payload_sha256, target_thread_sha256,
            predecessor_cursor_sha256, next_cursor_sha256,
            received_at_unix_ms, schema_version, payload_json, evidence_sha256,
            recorded_at_ms
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(event.event_id.as_str())
    .bind(scope_sha256.as_str())
    .bind(event.scope.adapter_id.as_str())
    .bind(event.source_event_sha256.as_str())
    .bind(event.payload_sha256.as_str())
    .bind(event.target_thread_sha256.as_str())
    .bind(
        event
            .predecessor_cursor_sha256
            .as_ref()
            .map(Sha256Digest::as_str),
    )
    .bind(event.next_cursor_sha256.as_str())
    .bind(i64::try_from(event.received_at_unix_ms).expect("event time"))
    .bind(i64::from(CHANNEL_EVIDENCE_SCHEMA_VERSION))
    .bind(&payload_json)
    .bind(Sha256Digest::for_bytes(payload_json.as_bytes()).as_str())
    .bind(1_i64)
    .execute(pool)
    .await
    .expect("insert raw event");
}

#[tokio::test]
async fn exact_claim_receipt_and_restart_preserve_one_authoritative_record() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let event = ingress(1, None);

    assert_eq!(
        store
            .claim_channel_ingress_event(&event)
            .await
            .expect("claim event"),
        ChannelIngressClaimDisposition::Inserted,
    );
    assert_eq!(
        store
            .claim_channel_ingress_event(&event)
            .await
            .expect("replay event"),
        ChannelIngressClaimDisposition::ExactReplay(ChannelIngressState::Pending),
    );
    let receipt = accepted(event.clone());
    assert_eq!(
        store
            .append_channel_ingress_receipt(&receipt)
            .await
            .expect("append receipt"),
        AppendDisposition::Inserted,
    );
    assert_eq!(
        store
            .append_channel_ingress_receipt(&receipt)
            .await
            .expect("replay receipt"),
        AppendDisposition::AlreadyPresent,
    );
    assert_eq!(
        store
            .current_channel_cursor(&scope())
            .await
            .expect("cursor"),
        Some(event.next_cursor_sha256.clone()),
    );
    drop(store);

    let reopened = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("reopen evidence");
    let stored = reopened
        .get_channel_ingress_event(&event.event_id)
        .await
        .expect("read event")
        .expect("stored event");
    assert_eq!(stored.event.event, event);
    assert_eq!(stored.receipt.expect("stored receipt").receipt, receipt);
    assert_eq!(
        reopened
            .claim_channel_ingress_event(&stored.event.event)
            .await
            .expect("terminal replay"),
        ChannelIngressClaimDisposition::ExactReplay(ChannelIngressState::Accepted),
    );
}

#[tokio::test]
async fn one_scope_serializes_unresolved_events_and_enforces_cursor_cas() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let first = ingress(1, None);
    let second = ingress(2, Some(first.next_cursor_sha256.clone()));
    store
        .claim_channel_ingress_event(&first)
        .await
        .expect("claim first");
    let mut other_scope = scope();
    other_scope.conversation_sha256 = digest("other-conversation");
    let independent = ChannelIngressEvent::new(
        other_scope,
        digest("independent-source"),
        digest("independent-payload"),
        channel_target_thread_sha256("thread-1").expect("target thread"),
        None,
        digest("independent-cursor"),
        20_000,
    )
    .expect("independent event");
    assert_eq!(
        store
            .claim_channel_ingress_event(&independent)
            .await
            .expect("independent scope"),
        ChannelIngressClaimDisposition::Inserted,
    );
    assert_eq!(
        store
            .claim_channel_ingress_event(&second)
            .await
            .expect("blocked second"),
        ChannelIngressClaimDisposition::BlockedByUnresolved {
            event_id: first.event_id.clone(),
            state: ChannelIngressState::Pending,
        },
    );
    let first_receipt = rejected(first.clone());
    store
        .append_channel_ingress_receipt(&first_receipt)
        .await
        .expect("reject first");
    assert_eq!(
        store
            .claim_channel_ingress_event(&second)
            .await
            .expect("claim second"),
        ChannelIngressClaimDisposition::Inserted,
    );
    store
        .append_channel_ingress_receipt(&rejected(second.clone()))
        .await
        .expect("reject second");
    assert_eq!(
        store
            .append_channel_ingress_receipt(&first_receipt)
            .await
            .expect("old definitive replay"),
        AppendDisposition::AlreadyPresent,
    );

    let stale = ingress(3, None);
    assert_eq!(
        store
            .claim_channel_ingress_event(&stale)
            .await
            .expect("stale cursor"),
        ChannelIngressClaimDisposition::CursorMismatch {
            expected: None,
            observed: Some(second.next_cursor_sha256),
        },
    );
}

#[tokio::test]
async fn same_source_identity_rejects_every_full_record_substitution() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let original = ingress(1, None);
    store
        .claim_channel_ingress_event(&original)
        .await
        .expect("claim original");

    let mut substitutions = Vec::new();
    let mut payload = original.clone();
    payload.payload_sha256 = digest("substituted-payload");
    substitutions.push(payload);
    let mut target = original.clone();
    target.target_thread_sha256 =
        channel_target_thread_sha256("thread-2").expect("substituted target");
    substitutions.push(target);
    let mut predecessor = original.clone();
    predecessor.predecessor_cursor_sha256 = Some(digest("substituted-predecessor"));
    substitutions.push(predecessor);
    let mut next = original.clone();
    next.next_cursor_sha256 = digest("substituted-next");
    substitutions.push(next);
    let mut received_at = original.clone();
    received_at.received_at_unix_ms += 1;
    substitutions.push(received_at);

    for substituted in substitutions {
        assert_eq!(substituted.event_id, original.event_id);
        assert!(matches!(
            store
                .claim_channel_ingress_event(&substituted)
                .await
                .expect_err("substitution must conflict"),
            EvidenceError::IdempotencyConflict { .. },
        ));
    }
    assert_eq!(
        store
            .get_channel_ingress_event(&original.event_id)
            .await
            .expect("read event")
            .expect("stored event")
            .event
            .event,
        original,
    );
}

#[tokio::test]
async fn concurrent_exact_claim_has_one_insert_and_one_replay() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let first = HeptaEvidenceStore::open(&sqlite).await.expect("first pool");
    let second = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("second pool");
    let event = ingress(1, None);

    let (left, right) = tokio::join!(
        first.claim_channel_ingress_event(&event),
        second.claim_channel_ingress_event(&event),
    );
    let dispositions = [left.expect("left claim"), right.expect("right claim")];
    assert!(dispositions.contains(&ChannelIngressClaimDisposition::Inserted));
    assert!(
        dispositions.contains(&ChannelIngressClaimDisposition::ExactReplay(
            ChannelIngressState::Pending,
        ))
    );
}

#[tokio::test]
async fn concurrent_distinct_claims_allow_only_one_unresolved_event_per_scope() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let first = HeptaEvidenceStore::open(&sqlite).await.expect("first pool");
    let second = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("second pool");
    let left_event = ingress(1, None);
    let right_event = ingress(2, None);

    let (left, right) = tokio::join!(
        first.claim_channel_ingress_event(&left_event),
        second.claim_channel_ingress_event(&right_event),
    );
    let dispositions = [left.expect("left claim"), right.expect("right claim")];
    assert_eq!(
        dispositions
            .iter()
            .filter(|value| **value == ChannelIngressClaimDisposition::Inserted)
            .count(),
        1,
    );
    assert_eq!(
        dispositions
            .iter()
            .filter(|value| matches!(
                value,
                ChannelIngressClaimDisposition::BlockedByUnresolved {
                    state: ChannelIngressState::Pending,
                    ..
                }
            ))
            .count(),
        1,
    );
}

#[tokio::test]
async fn concurrent_conflicting_receipts_leave_one_terminal() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let first = HeptaEvidenceStore::open(&sqlite).await.expect("first pool");
    let second = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("second pool");
    let event = ingress(1, None);
    first
        .claim_channel_ingress_event(&event)
        .await
        .expect("claim event");
    let acceptance = accepted(event.clone());
    let rejection = rejected(event.clone());

    let (left, right) = tokio::join!(
        first.append_channel_ingress_receipt(&acceptance),
        second.append_channel_ingress_receipt(&rejection),
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
    assert_eq!(inserted, 1);
    assert_eq!(conflicts, 1);

    let stored = first
        .get_channel_ingress_event(&event.event_id)
        .await
        .expect("read evidence")
        .expect("stored evidence")
        .receipt
        .expect("stored receipt")
        .receipt;
    assert!(stored == acceptance || stored == rejection);
}

#[tokio::test]
async fn canonical_forked_cursor_chain_fails_closed() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let first = ingress(1, None);
    store
        .claim_channel_ingress_event(&first)
        .await
        .expect("claim first");
    let fork = ingress(2, None);
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    insert_raw_event(&raw, &fork).await;

    assert!(matches!(
        store
            .current_channel_cursor(&scope())
            .await
            .expect_err("forked chain must fail closed"),
        EvidenceError::Corrupt(_),
    ));
    raw.close().await;
}

#[tokio::test]
async fn indeterminate_receipt_permanently_freezes_scope_across_restart() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let first = ingress(1, None);
    store
        .claim_channel_ingress_event(&first)
        .await
        .expect("claim first");
    let receipt = ChannelIngressReceipt::new(
        first.clone(),
        ChannelIngressTerminal::Indeterminate {
            reason_code: "app_server_timeout".to_string(),
        },
    );
    store
        .append_channel_ingress_receipt(&receipt)
        .await
        .expect("append indeterminate");
    assert_eq!(
        store
            .current_channel_cursor(&scope())
            .await
            .expect("cursor"),
        None,
    );
    drop(store);

    let reopened = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("reopen evidence");
    assert_eq!(
        reopened
            .claim_channel_ingress_event(&ingress(2, Some(first.next_cursor_sha256.clone())))
            .await
            .expect("blocked next event"),
        ChannelIngressClaimDisposition::BlockedByUnresolved {
            event_id: first.event_id.clone(),
            state: ChannelIngressState::Indeterminate,
        },
    );
    assert_eq!(
        reopened
            .append_channel_ingress_receipt(&receipt)
            .await
            .expect("exact receipt replay"),
        AppendDisposition::AlreadyPresent,
    );
}

#[tokio::test]
async fn receipt_requires_the_claimed_exact_event_and_target_thread() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let event = ingress(1, None);
    assert!(matches!(
        store
            .append_channel_ingress_receipt(&rejected(event.clone()))
            .await
            .expect_err("missing event must fail"),
        EvidenceError::InvalidRecord(_),
    ));
    store
        .claim_channel_ingress_event(&event)
        .await
        .expect("claim event");

    let mut rebound = event.clone();
    rebound.payload_sha256 = digest("rebound-payload");
    assert!(matches!(
        store
            .append_channel_ingress_receipt(&rejected(rebound))
            .await
            .expect_err("rebound event must fail"),
        EvidenceError::IdempotencyConflict { .. },
    ));
    let wrong_target = ChannelIngressReceipt::new(
        event.clone(),
        ChannelIngressTerminal::Accepted {
            thread_id: "thread-2".to_string(),
            turn_id: "turn-1".to_string(),
        },
    );
    assert!(matches!(
        store
            .append_channel_ingress_receipt(&wrong_target)
            .await
            .expect_err("wrong target must fail"),
        EvidenceError::InvalidRecord(_),
    ));

    let terminal = rejected(event.clone());
    store
        .append_channel_ingress_receipt(&terminal)
        .await
        .expect("append terminal");
    let conflict = ChannelIngressReceipt::new(
        event.clone(),
        ChannelIngressTerminal::Indeterminate {
            reason_code: "response_lost".to_string(),
        },
    );
    assert!(matches!(
        store
            .append_channel_ingress_receipt(&conflict)
            .await
            .expect_err("terminal substitution must fail"),
        EvidenceError::IdempotencyConflict { .. },
    ));
    assert_eq!(
        store
            .get_channel_ingress_event(&event.event_id)
            .await
            .expect("read evidence")
            .expect("stored evidence")
            .receipt
            .expect("stored receipt")
            .receipt,
        terminal,
    );
}

#[tokio::test]
async fn ingress_rows_are_append_only() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let event = ingress(1, None);
    store
        .claim_channel_ingress_event(&event)
        .await
        .expect("claim event");
    store
        .append_channel_ingress_receipt(&rejected(event))
        .await
        .expect("append receipt");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    for statement in [
        "UPDATE channel_ingress_events SET recorded_at_ms = recorded_at_ms + 1",
        "DELETE FROM channel_ingress_events",
        "UPDATE channel_ingress_receipts SET recorded_at_ms = recorded_at_ms + 1",
        "DELETE FROM channel_ingress_receipts",
    ] {
        sqlx::query(statement)
            .execute(&raw)
            .await
            .expect_err("append-only operation must fail");
    }
    raw.close().await;
}

#[tokio::test]
async fn typed_readback_rejects_canonical_payload_substitution() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let event = ingress(1, None);
    store
        .claim_channel_ingress_event(&event)
        .await
        .expect("claim event");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    sqlx::query("DROP TRIGGER channel_ingress_events_no_update")
        .execute(&raw)
        .await
        .expect("drop trigger");
    let mut substituted = event.clone();
    substituted.target_thread_sha256 =
        channel_target_thread_sha256("thread-2").expect("substituted target");
    let payload_json = serde_json::to_string(&substituted).expect("serialize substituted event");
    sqlx::query(
        "UPDATE channel_ingress_events
         SET payload_json = ?, evidence_sha256 = ? WHERE event_id = ?",
    )
    .bind(&payload_json)
    .bind(Sha256Digest::for_bytes(payload_json.as_bytes()).as_str())
    .bind(event.event_id.as_str())
    .execute(&raw)
    .await
    .expect("substitute payload");

    assert!(matches!(
        store
            .get_channel_ingress_event(&event.event_id)
            .await
            .expect_err("payload substitution must fail"),
        EvidenceError::Corrupt(_),
    ));
    raw.close().await;
}

#[tokio::test]
async fn typed_readback_rejects_receipt_projection_substitution() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let event = ingress(1, None);
    store
        .claim_channel_ingress_event(&event)
        .await
        .expect("claim event");
    store
        .append_channel_ingress_receipt(&rejected(event.clone()))
        .await
        .expect("append receipt");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    sqlx::query("DROP TRIGGER channel_ingress_receipts_no_update")
        .execute(&raw)
        .await
        .expect("drop trigger");
    sqlx::query(
        "UPDATE channel_ingress_receipts SET terminal_kind = 'indeterminate' WHERE event_id = ?",
    )
    .bind(event.event_id.as_str())
    .execute(&raw)
    .await
    .expect("substitute projection");

    assert!(matches!(
        store
            .get_channel_ingress_event(&event.event_id)
            .await
            .expect_err("projection substitution must fail"),
        EvidenceError::Corrupt(_),
    ));
    raw.close().await;
}

#[tokio::test]
async fn cursor_and_admission_control_reject_corrupt_projection_columns() {
    {
        let temp = TempDir::new().expect("temp dir");
        let sqlite = sqlite_config(&temp);
        let store = HeptaEvidenceStore::open(&sqlite)
            .await
            .expect("open evidence");
        let event = ingress(1, None);
        store
            .claim_channel_ingress_event(&event)
            .await
            .expect("claim event");
        store
            .append_channel_ingress_receipt(&accepted(event.clone()))
            .await
            .expect("accept event");
        let raw = sqlite
            .open_durable_evidence_pool(store.path())
            .await
            .expect("raw evidence pool");
        sqlx::query("DROP TRIGGER channel_ingress_events_no_update")
            .execute(&raw)
            .await
            .expect("drop event trigger");
        sqlx::query("UPDATE channel_ingress_events SET next_cursor_sha256 = ? WHERE event_id = ?")
            .bind(digest("forged-cursor").as_str())
            .bind(event.event_id.as_str())
            .execute(&raw)
            .await
            .expect("forge cursor projection");
        assert!(matches!(
            store
                .current_channel_cursor(&scope())
                .await
                .expect_err("cursor control must validate canonical rows"),
            EvidenceError::Corrupt(_),
        ));
        raw.close().await;
    }

    {
        let temp = TempDir::new().expect("temp dir");
        let sqlite = sqlite_config(&temp);
        let store = HeptaEvidenceStore::open(&sqlite)
            .await
            .expect("open evidence");
        let event = ingress(1, None);
        store
            .claim_channel_ingress_event(&event)
            .await
            .expect("claim event");
        store
            .append_channel_ingress_receipt(&ChannelIngressReceipt::new(
                event.clone(),
                ChannelIngressTerminal::Indeterminate {
                    reason_code: "response_lost".to_string(),
                },
            ))
            .await
            .expect("append indeterminate");
        let raw = sqlite
            .open_durable_evidence_pool(store.path())
            .await
            .expect("raw evidence pool");
        sqlx::query("DROP TRIGGER channel_ingress_receipts_no_update")
            .execute(&raw)
            .await
            .expect("drop receipt trigger");
        sqlx::query(
            "UPDATE channel_ingress_receipts SET terminal_kind = 'rejected' WHERE event_id = ?",
        )
        .bind(event.event_id.as_str())
        .execute(&raw)
        .await
        .expect("forge terminal projection");
        assert!(matches!(
            store
                .claim_channel_ingress_event(&ingress(2, Some(event.next_cursor_sha256.clone()),))
                .await
                .expect_err("admission control must validate canonical rows"),
            EvidenceError::Corrupt(_),
        ));
        raw.close().await;
    }
}

#[tokio::test]
async fn open_rejects_missing_ingress_immutability_trigger() {
    for (trigger, statement) in [
        (
            "channel_ingress_events_no_update",
            "DROP TRIGGER channel_ingress_events_no_update",
        ),
        (
            "channel_ingress_events_no_delete",
            "DROP TRIGGER channel_ingress_events_no_delete",
        ),
        (
            "channel_ingress_receipts_no_update",
            "DROP TRIGGER channel_ingress_receipts_no_update",
        ),
        (
            "channel_ingress_receipts_no_delete",
            "DROP TRIGGER channel_ingress_receipts_no_delete",
        ),
    ] {
        let temp = TempDir::new().expect("temp dir");
        let sqlite = sqlite_config(&temp);
        let store = HeptaEvidenceStore::open(&sqlite)
            .await
            .expect("open evidence");
        let raw = sqlite
            .open_durable_evidence_pool(store.path())
            .await
            .expect("raw evidence pool");
        sqlx::query(statement)
            .execute(&raw)
            .await
            .expect("drop trigger");
        raw.close().await;
        drop(store);

        let error = match HeptaEvidenceStore::open(&sqlite).await {
            Ok(_) => panic!("missing trigger {trigger} must fail closed"),
            Err(error) => error,
        };
        assert!(matches!(error, EvidenceError::Corrupt(_)));
    }
}

#[tokio::test]
async fn open_rejects_weakened_source_identity_constraint() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    let mut connection = raw.acquire().await.expect("raw evidence connection");
    sqlx::query("PRAGMA writable_schema = ON")
        .execute(&mut *connection)
        .await
        .expect("enable writable schema");
    let update = sqlx::query(
        "UPDATE sqlite_schema
         SET sql = replace(
             sql,
             'UNIQUE(scope_sha256, source_event_sha256)',
             'UNIQUE(scope_sha256, source_event_sha256, received_at_unix_ms)'
         )
         WHERE type = 'table' AND name = 'channel_ingress_events'",
    )
    .execute(&mut *connection)
    .await
    .expect("weaken source identity");
    assert_eq!(update.rows_affected(), 1);
    sqlx::query("PRAGMA writable_schema = OFF")
        .execute(&mut *connection)
        .await
        .expect("disable writable schema");
    drop(connection);
    raw.close().await;
    drop(store);

    let error = match HeptaEvidenceStore::open(&sqlite).await {
        Ok(_) => panic!("weakened source identity must fail closed"),
        Err(error) => error,
    };
    assert!(matches!(error, EvidenceError::Corrupt(_)));
}
