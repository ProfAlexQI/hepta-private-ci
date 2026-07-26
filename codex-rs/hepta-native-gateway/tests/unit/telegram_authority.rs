use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use super::*;

const KEY: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
];

fn authority() -> (tempfile::TempDir, TelegramAuthority) {
    let root = tempfile::tempdir().expect("tempdir");
    let config = TelegramAuthorityConfig::for_test(root.path()).expect("config");
    let authority = TelegramAuthority::open(config).expect("authority");
    (root, authority)
}

fn plan_body(
    request_id: &str,
    cursor: Option<i64>,
    request_binding: &str,
    session_binding: &str,
) -> String {
    serde_json::json!({
        "request_id": request_id,
        "cursor": cursor,
        "proof": proof(
            &KEY,
            PLAN_PROOF_DOMAIN,
            &[
                request_id,
                &cursor_binding(cursor),
                request_binding,
                session_binding,
            ],
        ),
    })
    .to_string()
}

fn commit_body(
    request_id: &str,
    plan_hash: &str,
    plan_request_binding: &str,
    commit_request_binding: &str,
    session_binding: &str,
) -> String {
    serde_json::json!({
        "request_id": request_id,
        "plan_hash": plan_hash,
        "plan_request_binding_hash": plan_request_binding,
        "proof": proof(
            &KEY,
            COMMIT_PROOF_DOMAIN,
            &[
                request_id,
                plan_hash,
                plan_request_binding,
                commit_request_binding,
                session_binding,
            ],
        ),
    })
    .to_string()
}

fn authorized<'a>(
    authority: &'a TelegramAuthority,
    request_id: &str,
    cursor: Option<i64>,
) -> (
    TelegramAuthorityPlanReceipt,
    TelegramAuthorityCommitReceipt,
    TelegramPipelinePermit<'a>,
) {
    let plan_binding = "a".repeat(64);
    let commit_binding = "b".repeat(64);
    let session = "c".repeat(64);
    let plan = authority
        .plan(
            Some(&plan_body(request_id, cursor, &plan_binding, &session)),
            &plan_binding,
            &session,
            cursor,
        )
        .expect("plan");
    let (commit, permit) = authority
        .authorize(
            Some(&commit_body(
                request_id,
                &plan.plan_hash,
                &plan_binding,
                &commit_binding,
                &session,
            )),
            &commit_binding,
            &session,
            cursor,
        )
        .expect("authorize");
    (plan, commit, permit)
}

#[derive(Clone, Copy)]
enum ReconciliationCrashStage {
    AfterSend,
    AfterProviderAck,
    AfterDeliveryAckBeforeCursor,
    AfterCursorBeforeTerminal,
}

struct ReconciliationFixture {
    _root: tempfile::TempDir,
    authority: TelegramAuthority,
    request: TelegramReconciliationRequest,
    request_binding_hash: String,
    delivery_ledger: PathBuf,
    cursor: PathBuf,
}

fn reconciliation_fixture(stage: ReconciliationCrashStage) -> ReconciliationFixture {
    let (root, authority) = authority();
    let delivery_ledger = root.path().join("delivery.jsonl");
    let cursor = root.path().join("cursor.json");
    let request_id = "a".repeat(64);
    let (plan, commit, mut permit) = authorized(&authority, &request_id, Some(42));
    let read_result_hash = "1".repeat(64);
    let model_result_hash = "2".repeat(64);
    let effect_plan_hash = "e".repeat(64);
    let provider_ack_hash = "f".repeat(64);
    let read_evidence = PhaseEvidence {
        update_id: Some(42),
        next_update_offset: Some(43),
        read_result_hash: Some(read_result_hash.clone()),
        ..PhaseEvidence::default()
    };
    authority
        .append_transition(
            &permit.binding,
            &permit.owner_nonce,
            Phase::ReadIntent,
            PhaseEvidence::default(),
        )
        .expect("read intent");
    authority
        .append_transition(
            &permit.binding,
            &permit.owner_nonce,
            Phase::ReadCompleted,
            read_evidence,
        )
        .expect("read completed");
    authority
        .append_transition(
            &permit.binding,
            &permit.owner_nonce,
            Phase::ModelIntent,
            PhaseEvidence {
                update_id: Some(42),
                next_update_offset: Some(43),
                read_result_hash: Some(read_result_hash.clone()),
                ..PhaseEvidence::default()
            },
        )
        .expect("model intent");
    authority
        .append_transition(
            &permit.binding,
            &permit.owner_nonce,
            Phase::ModelCompleted,
            PhaseEvidence {
                update_id: Some(42),
                next_update_offset: Some(43),
                read_result_hash: Some(read_result_hash.clone()),
                model_result_hash: Some(model_result_hash.clone()),
                ..PhaseEvidence::default()
            },
        )
        .expect("model completed");
    authority
        .append_transition(
            &permit.binding,
            &permit.owner_nonce,
            Phase::SendIntent,
            PhaseEvidence {
                update_id: Some(42),
                next_update_offset: Some(43),
                read_result_hash: Some(read_result_hash.clone()),
                model_result_hash: Some(model_result_hash.clone()),
                effect_plan_hash: Some(effect_plan_hash.clone()),
                ..PhaseEvidence::default()
            },
        )
        .expect("send intent");
    if !matches!(stage, ReconciliationCrashStage::AfterSend) {
        authority
            .append_transition(
                &permit.binding,
                &permit.owner_nonce,
                Phase::SendAcknowledged,
                PhaseEvidence {
                    update_id: Some(42),
                    next_update_offset: Some(43),
                    read_result_hash: Some(read_result_hash),
                    model_result_hash: Some(model_result_hash),
                    effect_plan_hash: Some(effect_plan_hash.clone()),
                    provider_ack_hash: Some(provider_ack_hash.clone()),
                    ..PhaseEvidence::default()
                },
            )
            .expect("send acknowledged");
    }
    if matches!(
        stage,
        ReconciliationCrashStage::AfterDeliveryAckBeforeCursor
            | ReconciliationCrashStage::AfterCursorBeforeTerminal
    ) {
        append_authenticated_delivery_ack_record(
            &delivery_ledger,
            &cursor,
            &permit.binding,
            43,
            42,
            &effect_plan_hash,
            &provider_ack_hash,
            &KEY,
        )
        .expect("authenticated delivery ACK");
    }
    if matches!(stage, ReconciliationCrashStage::AfterCursorBeforeTerminal) {
        crate::telegram_durable_files::write_cursor_next_update_offset(&cursor, 43)
            .expect("cursor");
    }
    permit.finished = true;
    drop(permit);
    ReconciliationFixture {
        _root: root,
        authority,
        request: TelegramReconciliationRequest {
            request_id,
            plan_hash: plan.plan_hash,
            plan_request_binding_hash: plan.plan_request_binding_hash,
            commit_request_binding_hash: commit.commit_request_binding_hash,
            session_binding_hash: commit.session_binding_hash,
            cursor: Some(42),
            update_id: 42,
            next_update_offset: 43,
            effect_plan_hash,
            provider_ack_hash,
            decision: None,
            proof: String::new(),
        },
        request_binding_hash: "9".repeat(64),
        delivery_ledger,
        cursor,
    }
}

fn reconciliation_body(
    fixture: &ReconciliationFixture,
    path: &str,
    decision: Option<TelegramReconciliationDecision>,
    proof_path: &str,
) -> String {
    let mut request = fixture.request.clone();
    request.decision = decision;
    let cursor = cursor_binding(request.cursor);
    let update_id = request.update_id.to_string();
    let next_update_offset = request.next_update_offset.to_string();
    request.proof = proof(
        &KEY,
        RECONCILIATION_PROOF_DOMAIN,
        &[
            "POST",
            proof_path,
            &fixture.request_binding_hash,
            &request.request_id,
            &request.plan_hash,
            &request.plan_request_binding_hash,
            &request.commit_request_binding_hash,
            &request.session_binding_hash,
            &cursor,
            &update_id,
            &next_update_offset,
            &request.effect_plan_hash,
            &request.provider_ack_hash,
            if decision.is_some() {
                "complete_terminal_receipt_only"
            } else {
                "inspect_only"
            },
        ],
    );
    debug_assert!(matches!(
        path,
        TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT | TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT
    ));
    serde_json::to_string(&request).expect("reconciliation body")
}

fn route_reconciliation(
    fixture: &ReconciliationFixture,
    path: &str,
    body: &str,
) -> TelegramReconciliationHttpResponse {
    fixture
        .authority
        .route_reconciliation_http(
            "POST",
            path,
            Some(body),
            &fixture.request_binding_hash,
            &fixture.request.session_binding_hash,
            &fixture.delivery_ledger,
            &fixture.cursor,
        )
        .expect("reconciliation route")
}

fn durable_snapshot(fixture: &ReconciliationFixture) -> (usize, Vec<u8>, Vec<u8>) {
    (
        fixture.authority.inspect_events().expect("events").len(),
        fs::read(&fixture.delivery_ledger).unwrap_or_default(),
        fs::read(&fixture.cursor).unwrap_or_default(),
    )
}

#[test]
fn exact_pipeline_records_each_intent_ack_and_terminal_without_real_side_effects() {
    let (root, authority) = authority();
    let delivery_ledger = root.path().join("delivery.jsonl");
    let cursor = root.path().join("cursor.json");
    let calls = Arc::new(AtomicUsize::new(0));
    let (_plan, commit, permit) = authorized(&authority, &"1".repeat(64), Some(42));
    assert!(!commit.external_effect_started);
    let read_calls = Arc::clone(&calls);
    let model_calls = Arc::clone(&calls);
    let send_calls = Arc::clone(&calls);
    let read_authority = &authority;
    let model_authority = &authority;
    let send_authority = &authority;
    let receipt = permit
        .execute_with(
            &delivery_ledger,
            &cursor,
            move |request| {
                assert_eq!(read_calls.fetch_add(1, Ordering::SeqCst), 0);
                assert_eq!(
                    read_authority
                        .inspect_events()
                        .expect("read intent")
                        .last()
                        .unwrap()
                        .phase,
                    Phase::ReadIntent
                );
                assert_eq!(request.cursor, Some(42));
                Ok(TelegramReadResult {
                    update_id: 42,
                    chat_id: 6476198178,
                    reply_to_message_id: Some(9),
                    prompt: "injected private prompt".into(),
                })
            },
            move |request| {
                assert_eq!(model_calls.fetch_add(1, Ordering::SeqCst), 1);
                assert_eq!(
                    model_authority
                        .inspect_events()
                        .expect("model intent")
                        .last()
                        .unwrap()
                        .phase,
                    Phase::ModelIntent
                );
                assert_eq!(request.update_id, 42);
                assert_eq!(request.prompt, "injected private prompt");
                Ok("injected model response".into())
            },
            move |plan| {
                assert_eq!(send_calls.fetch_add(1, Ordering::SeqCst), 2);
                assert_eq!(
                    send_authority
                        .inspect_events()
                        .expect("send intent")
                        .last()
                        .unwrap()
                        .phase,
                    Phase::SendIntent
                );
                assert_eq!(plan.chat_id, 6476198178);
                assert_eq!(plan.message_text, "injected model response");
                Ok(TelegramProviderAck {
                    provider: "telegram-bot-api".into(),
                    provider_message_id: 10,
                    chat_id: plan.chat_id,
                    raw_response_hash: "d".repeat(64),
                })
            },
        )
        .expect("pipeline");
    assert_eq!(calls.load(Ordering::SeqCst), 3);
    assert!(receipt.durable_intent_recorded);
    assert!(receipt.provider_effect_ack_recorded);
    assert!(receipt.delivery_ack_recorded);
    assert!(receipt.cursor_written);
    assert!(receipt.terminal_receipt_recorded);
    assert_eq!(
        receipt.durable_intent_owner,
        TELEGRAM_PIPELINE_AUTHORITY_OWNER
    );
    assert_eq!(receipt.update_id, 42);
    assert_eq!(receipt.next_update_offset, 43);
    assert_eq!(
        hepta_gateway::telegram_cursor_status_from_path(&cursor).next_update_offset,
        Some(43)
    );
    let delivery = fs::read_to_string(&delivery_ledger).expect("delivery ledger");
    assert!(delivery.contains("\"stage\":\"enqueued\""));
    assert!(delivery.contains("\"stage\":\"acked\""));
    let raw = fs::read_to_string(&authority.journal_file).expect("journal");
    assert!(!raw.contains("injected private prompt"));
    assert!(!raw.contains("injected model response"));
    let phases = authority
        .inspect_events()
        .expect("events")
        .into_iter()
        .map(|event| event.phase)
        .collect::<Vec<_>>();
    assert_eq!(
        phases,
        [
            Phase::Planned,
            Phase::Authorized,
            Phase::ReadIntent,
            Phase::ReadCompleted,
            Phase::ModelIntent,
            Phase::ModelCompleted,
            Phase::SendIntent,
            Phase::SendAcknowledged,
            Phase::TerminalSucceeded,
        ]
    );
    let monotonic = authority.monotonic_state().expect("monotonic state");
    assert_eq!(monotonic.journal_sequence, 9);
    assert!(monotonic.latest_event_hash.starts_with("sha256:"));
    assert_eq!(
        monotonic.latest_event_mac.as_deref().map(str::len),
        Some(64)
    );
}

#[test]
fn dropped_owner_becomes_in_doubt_and_cannot_replay() {
    let (_root, authority) = authority();
    let request_id = "2".repeat(64);
    let (plan, _, permit) = authorized(&authority, &request_id, None);
    drop(permit);
    assert_eq!(
        authority
            .inspect_events()
            .expect("events")
            .last()
            .unwrap()
            .phase,
        Phase::InDoubt
    );
    let commit_binding = "b".repeat(64);
    assert!(
        authority
            .authorize(
                Some(&commit_body(
                    &request_id,
                    &plan.plan_hash,
                    &plan.plan_request_binding_hash,
                    &commit_binding,
                    &plan.session_binding_hash,
                )),
                &commit_binding,
                &plan.session_binding_hash,
                None,
            )
            .is_err()
    );
}

#[test]
fn stale_session_cursor_tampering_and_phase_or_owner_substitution_fail_closed() {
    let (_root, authority) = authority();
    let request_id = "3".repeat(64);
    let plan_binding = "a".repeat(64);
    let commit_binding = "b".repeat(64);
    let session = "c".repeat(64);
    let mut tampered: serde_json::Value = serde_json::from_str(&plan_body(
        &"f".repeat(64),
        Some(5),
        &plan_binding,
        &session,
    ))
    .expect("plan JSON");
    tampered["proof"] = serde_json::Value::String("0".repeat(64));
    assert!(
        authority
            .plan(
                Some(&tampered.to_string()),
                &plan_binding,
                &session,
                Some(5),
            )
            .is_err()
    );
    let plan = authority
        .plan(
            Some(&plan_body(&request_id, Some(5), &plan_binding, &session)),
            &plan_binding,
            &session,
            Some(5),
        )
        .expect("plan");
    let body = commit_body(
        &request_id,
        &plan.plan_hash,
        &plan_binding,
        &commit_binding,
        &session,
    );
    assert!(
        authority
            .authorize(Some(&body), &commit_binding, &"d".repeat(64), Some(5))
            .is_err()
    );
    assert!(
        authority
            .authorize(Some(&body), &commit_binding, &session, Some(6))
            .is_err()
    );
    let (_, permit) = authority
        .authorize(Some(&body), &commit_binding, &session, Some(5))
        .expect("exact authorization");
    assert!(
        authority
            .append_transition(
                &permit.binding,
                "wrong-owner",
                Phase::ModelIntent,
                PhaseEvidence::default(),
            )
            .is_err()
    );
    assert!(
        authority
            .append_transition(
                &permit.binding,
                &permit.owner_nonce,
                Phase::SendIntent,
                PhaseEvidence::default(),
            )
            .is_err()
    );
}

#[test]
fn mismatched_provider_ack_never_publishes_terminal_receipt() {
    let (root, authority) = authority();
    let delivery_ledger = root.path().join("delivery.jsonl");
    let cursor = root.path().join("cursor.json");
    let (_, _, permit) = authorized(&authority, &"4".repeat(64), Some(7));
    let result = permit.execute_with(
        &delivery_ledger,
        &cursor,
        |_| {
            Ok(TelegramReadResult {
                update_id: 7,
                chat_id: 88,
                reply_to_message_id: None,
                prompt: "prompt".into(),
            })
        },
        |_| Ok("response".into()),
        |_| {
            Ok(TelegramProviderAck {
                provider: "telegram-bot-api".into(),
                provider_message_id: 9,
                chat_id: 99,
                raw_response_hash: "e".repeat(64),
            })
        },
    );
    assert!(result.is_err());
    assert!(!cursor.exists());
    let events = authority.inspect_events().expect("events");
    assert_eq!(events.last().unwrap().phase, Phase::SendIntent);
    assert!(
        !events
            .iter()
            .any(|event| event.phase == Phase::TerminalSucceeded)
    );
}

#[test]
fn cursor_failure_preserves_send_acknowledged_for_exact_reconciliation() {
    let (root, authority) = authority();
    let delivery_ledger = root.path().join("delivery.jsonl");
    let cursor = root.path().join("cursor-is-directory");
    fs::create_dir(&cursor).expect("cursor failure fixture");
    let (_, _, permit) = authorized(&authority, &"5".repeat(64), Some(11));
    let result = permit.execute_with(
        &delivery_ledger,
        &cursor,
        |_| {
            Ok(TelegramReadResult {
                update_id: 11,
                chat_id: 88,
                reply_to_message_id: None,
                prompt: "prompt".into(),
            })
        },
        |_| Ok("response".into()),
        |plan| {
            Ok(TelegramProviderAck {
                provider: "telegram-bot-api".into(),
                provider_message_id: 12,
                chat_id: plan.chat_id,
                raw_response_hash: "f".repeat(64),
            })
        },
    );
    assert!(result.is_err());
    let delivery = fs::read_to_string(&delivery_ledger).expect("delivery ledger");
    let enqueued = delivery.find("\"stage\":\"enqueued\"").expect("enqueued");
    let acked = delivery.find("\"stage\":\"acked\"").expect("acked");
    assert!(enqueued < acked);
    assert!(cursor.is_dir());
    let events = authority.inspect_events().expect("events");
    assert_eq!(events.last().unwrap().phase, Phase::SendAcknowledged);
    assert!(
        !events
            .iter()
            .any(|event| event.phase == Phase::TerminalSucceeded)
    );
}

#[test]
fn terminal_reconciliation_rejects_each_incomplete_crash_window_without_state_change() {
    for stage in [
        ReconciliationCrashStage::AfterSend,
        ReconciliationCrashStage::AfterProviderAck,
        ReconciliationCrashStage::AfterDeliveryAckBeforeCursor,
    ] {
        let fixture = reconciliation_fixture(stage);
        let body = reconciliation_body(
            &fixture,
            TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
            Some(TelegramReconciliationDecision::CompleteTerminalReceiptOnly),
            TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
        );
        let before = durable_snapshot(&fixture);
        let response =
            route_reconciliation(&fixture, TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT, &body);
        assert_eq!(response.status, "409 Conflict");
        assert!(!response.outcome_state_changed);
        assert_eq!(durable_snapshot(&fixture), before);
        assert!(
            !fixture
                .authority
                .inspect_events()
                .expect("events")
                .iter()
                .any(|event| event.phase == Phase::ReconciledTerminalSucceeded)
        );
    }
}

#[test]
fn terminal_reconciliation_is_terminal_only_and_duplicate_resolve_is_idempotent() {
    let fixture = reconciliation_fixture(ReconciliationCrashStage::AfterCursorBeforeTerminal);
    let inspect_body = reconciliation_body(
        &fixture,
        TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT,
        None,
        TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT,
    );
    let before = durable_snapshot(&fixture);
    let inspect = route_reconciliation(
        &fixture,
        TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT,
        &inspect_body,
    );
    assert_eq!(inspect.status, "200 OK");
    assert!(!inspect.outcome_state_changed);
    assert!(inspect.body.contains("\"result\":\"eligible\""));
    assert_eq!(durable_snapshot(&fixture), before);

    let resolve_body = reconciliation_body(
        &fixture,
        TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
        Some(TelegramReconciliationDecision::CompleteTerminalReceiptOnly),
        TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
    );
    let resolve = route_reconciliation(
        &fixture,
        TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
        &resolve_body,
    );
    assert_eq!(resolve.status, "200 OK");
    assert!(resolve.outcome_state_changed);
    let receipt: serde_json::Value = serde_json::from_str(&resolve.body).expect("resolve receipt");
    assert_eq!(receipt["result"], "recorded");
    assert_eq!(receipt["provider_replayed"], false);
    assert_eq!(receipt["read_replayed"], false);
    assert_eq!(receipt["model_replayed"], false);
    assert_eq!(receipt["cursor_written"], false);
    assert_eq!(receipt["cursor_advanced"], false);
    assert_eq!(receipt["terminal_receipt_recorded"], true);
    let after_first = durable_snapshot(&fixture);
    assert_eq!(after_first.1, before.1);
    assert_eq!(after_first.2, before.2);
    assert_eq!(
        fixture
            .authority
            .inspect_events()
            .expect("events")
            .last()
            .expect("terminal")
            .phase,
        Phase::ReconciledTerminalSucceeded
    );

    let duplicate = route_reconciliation(
        &fixture,
        TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
        &resolve_body,
    );
    assert_eq!(duplicate.status, "200 OK");
    assert!(!duplicate.outcome_state_changed);
    assert!(duplicate.body.contains("\"result\":\"already_recorded\""));
    assert_eq!(durable_snapshot(&fixture), after_first);
}

#[test]
fn terminal_reconciliation_rejects_binding_replay_and_ledger_tamper() {
    let fixture = reconciliation_fixture(ReconciliationCrashStage::AfterCursorBeforeTerminal);
    let replay_body = reconciliation_body(
        &fixture,
        TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
        Some(TelegramReconciliationDecision::CompleteTerminalReceiptOnly),
        TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT,
    );
    let before = durable_snapshot(&fixture);
    let replay = route_reconciliation(
        &fixture,
        TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
        &replay_body,
    );
    assert_eq!(replay.status, "403 Forbidden");
    assert!(!replay.outcome_state_changed);
    assert_eq!(durable_snapshot(&fixture), before);

    let valid_body = reconciliation_body(
        &fixture,
        TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
        Some(TelegramReconciliationDecision::CompleteTerminalReceiptOnly),
        TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
    );
    let mut substituted: serde_json::Value =
        serde_json::from_str(&valid_body).expect("valid reconciliation body");
    substituted["provider_ack_hash"] = serde_json::Value::String("0".repeat(64));
    let substituted = route_reconciliation(
        &fixture,
        TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
        &substituted.to_string(),
    );
    assert_eq!(substituted.status, "403 Forbidden");
    assert_eq!(durable_snapshot(&fixture), before);

    let mut records = fs::read_to_string(&fixture.delivery_ledger)
        .expect("authenticated delivery ledger")
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("record"))
        .collect::<Vec<_>>();
    records[0]["acked"] = serde_json::Value::Bool(false);
    let mut tampered = records
        .iter()
        .map(serde_json::to_string)
        .collect::<std::result::Result<Vec<_>, _>>()
        .expect("records")
        .join("\n");
    tampered.push('\n');
    fs::write(&fixture.delivery_ledger, tampered).expect("tamper ledger");
    let after_tamper = durable_snapshot(&fixture);
    let tamper = route_reconciliation(
        &fixture,
        TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT,
        &valid_body,
    );
    assert_eq!(tamper.status, "409 Conflict");
    assert!(!tamper.outcome_state_changed);
    assert_eq!(durable_snapshot(&fixture), after_tamper);
}

#[test]
fn send_failure_records_no_cursor_and_never_publishes_terminal_receipt() {
    let (root, authority) = authority();
    let delivery_ledger = root.path().join("delivery.jsonl");
    let cursor = root.path().join("cursor.json");
    let (_, _, permit) = authorized(&authority, &"6".repeat(64), Some(13));
    let result = permit.execute_with(
        &delivery_ledger,
        &cursor,
        |_| {
            Ok(TelegramReadResult {
                update_id: 13,
                chat_id: 88,
                reply_to_message_id: None,
                prompt: "prompt".into(),
            })
        },
        |_| Ok("response".into()),
        |_| anyhow::bail!("injected sender failure"),
    );
    assert!(result.is_err());
    assert!(!cursor.exists());
    let delivery = fs::read_to_string(&delivery_ledger).expect("delivery ledger");
    assert!(delivery.contains("\"stage\":\"enqueued\""));
    assert!(delivery.contains("\"stage\":\"failed\""));
    assert!(!delivery.contains("\"stage\":\"acked\""));
    let events = authority.inspect_events().expect("events");
    assert_eq!(events.last().unwrap().phase, Phase::SendIntent);
    assert!(
        !events
            .iter()
            .any(|event| event.phase == Phase::TerminalSucceeded)
    );
}

#[test]
fn duplicate_update_is_rejected_before_model_or_send_and_does_not_move_cursor() {
    let (root, authority) = authority();
    let delivery_ledger = root.path().join("delivery.jsonl");
    let cursor = root.path().join("cursor.json");
    let (_, _, first) = authorized(&authority, &"7".repeat(64), Some(20));
    first
        .execute_with(
            &delivery_ledger,
            &cursor,
            |_| {
                Ok(TelegramReadResult {
                    update_id: 20,
                    chat_id: 88,
                    reply_to_message_id: None,
                    prompt: "first prompt".into(),
                })
            },
            |_| Ok("first response".into()),
            |plan| {
                Ok(TelegramProviderAck {
                    provider: "telegram-bot-api".into(),
                    provider_message_id: 21,
                    chat_id: plan.chat_id,
                    raw_response_hash: "1".repeat(64),
                })
            },
        )
        .expect("first delivery");
    assert_eq!(
        hepta_gateway::telegram_cursor_status_from_path(&cursor).next_update_offset,
        Some(21)
    );
    let ledger_before = fs::read_to_string(&delivery_ledger).expect("delivery ledger");

    // Even if a caller supplies a stale cursor equal to the old update,
    // terminal journal history refuses to mint a second pipeline owner
    // before credentialed read can start.
    let replay_request_id = "8".repeat(64);
    let replay_plan_binding = "d".repeat(64);
    let replay_commit_binding = "e".repeat(64);
    let replay_session = "f".repeat(64);
    let replay_plan = authority
        .plan(
            Some(&plan_body(
                &replay_request_id,
                Some(20),
                &replay_plan_binding,
                &replay_session,
            )),
            &replay_plan_binding,
            &replay_session,
            Some(20),
        )
        .expect("record replay plan");
    let result = authority.authorize(
        Some(&commit_body(
            &replay_request_id,
            &replay_plan.plan_hash,
            &replay_plan_binding,
            &replay_commit_binding,
            &replay_session,
        )),
        &replay_commit_binding,
        &replay_session,
        Some(20),
    );
    assert!(result.is_err());
    assert_eq!(
        hepta_gateway::telegram_cursor_status_from_path(&cursor).next_update_offset,
        Some(21)
    );
    assert_eq!(
        fs::read_to_string(&delivery_ledger).expect("delivery ledger"),
        ledger_before
    );
    assert_eq!(
        authority
            .inspect_events()
            .expect("events")
            .last()
            .unwrap()
            .phase,
        Phase::Planned
    );
}

#[test]
fn concurrent_plans_atomically_exclude_the_second_update_claim() {
    let (root, authority) = authority();
    let delivery_ledger = root.path().join("delivery.jsonl");
    let cursor = root.path().join("cursor.json");
    let (_, _, first_permit) = authorized(&authority, &"9".repeat(64), Some(30));
    let second_request_id = "a".repeat(64);
    let plan_binding = "d".repeat(64);
    let commit_binding = "e".repeat(64);
    let session = "c".repeat(64);
    let second_plan = authority
        .plan(
            Some(&plan_body(
                &second_request_id,
                Some(30),
                &plan_binding,
                &session,
            )),
            &plan_binding,
            &session,
            Some(30),
        )
        .expect("second plan may be prepared without owning the cursor");
    let second_commit_body = commit_body(
        &second_request_id,
        &second_plan.plan_hash,
        &plan_binding,
        &commit_binding,
        &session,
    );
    assert!(
        authority
            .authorize(
                Some(&second_commit_body),
                &commit_binding,
                &session,
                Some(30),
            )
            .is_err(),
        "the cursor-scoped owner must be chosen before any credentialed read"
    );
    let read_calls = Arc::new(AtomicUsize::new(0));
    let model_calls = Arc::new(AtomicUsize::new(0));
    let send_calls = Arc::new(AtomicUsize::new(0));
    let read_counter = Arc::clone(&read_calls);
    let model_counter = Arc::clone(&model_calls);
    let send_counter = Arc::clone(&send_calls);
    let first_result = first_permit.execute_with(
        &delivery_ledger,
        &cursor,
        |_| {
            read_counter.fetch_add(1, Ordering::SeqCst);
            Ok(TelegramReadResult {
                update_id: 30,
                chat_id: 88,
                reply_to_message_id: None,
                prompt: "first prompt".into(),
            })
        },
        |_| {
            model_counter.fetch_add(1, Ordering::SeqCst);
            Ok("first response".into())
        },
        |plan| {
            send_counter.fetch_add(1, Ordering::SeqCst);
            Ok(TelegramProviderAck {
                provider: "telegram-bot-api".into(),
                provider_message_id: 31,
                chat_id: plan.chat_id,
                raw_response_hash: "2".repeat(64),
            })
        },
    );
    assert!(first_result.is_ok());
    assert_eq!(read_calls.load(Ordering::SeqCst), 1);
    assert_eq!(model_calls.load(Ordering::SeqCst), 1);
    assert_eq!(send_calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        hepta_gateway::telegram_cursor_status_from_path(&cursor).next_update_offset,
        Some(31)
    );
    let delivery = fs::read_to_string(&delivery_ledger).expect("delivery ledger");
    assert_eq!(delivery.matches("\"stage\":\"enqueued\"").count(), 1);
    assert_eq!(delivery.matches("\"stage\":\"acked\"").count(), 1);
    let events = authority.inspect_events().expect("events");
    assert_eq!(
        events
            .iter()
            .filter(|event| event.phase == Phase::TerminalSucceeded)
            .count(),
        1
    );
    assert!(
        events.iter().all(|event| {
            event.plan_hash != second_plan.plan_hash || matches!(event.phase, Phase::Planned)
        }),
        "losing plan must not acquire a permit or reach credentialed read"
    );
}

#[test]
fn terminal_checkpoint_preserves_replay_identity_and_in_doubt_history() -> Result<()> {
    let mut snapshot = JournalSnapshot {
        checkpoint: None,
        events: Vec::new(),
    };
    for (request, plan, phase) in [
        ('1', '2', Phase::TerminalSucceeded),
        ('3', '4', Phase::InDoubt),
        ('5', '6', Phase::ReconciledTerminalSucceeded),
    ] {
        let binding = PlanBinding {
            request_id: request.to_string().repeat(64),
            plan_hash: plan.to_string().repeat(64),
            plan_request_binding_hash: "7".repeat(64),
            commit_request_binding_hash: Some("8".repeat(64)),
            session_binding_hash: "9".repeat(64),
            cursor: None,
        };
        let event = event_for(
            &snapshot,
            &binding,
            Some(&"a".repeat(64)),
            phase,
            PhaseEvidence::default(),
        );
        append_event(&mut snapshot, event, &KEY)?;
    }
    let before = snapshot.monotonic_binding();

    compact_terminal_pipelines(&mut snapshot, &KEY, 1)?;

    let checkpoint = snapshot.checkpoint.as_ref().context("checkpoint")?;
    assert_eq!(checkpoint.compacted_events, 1);
    assert_eq!(checkpoint.consumed_authorities.len(), 1);
    assert!(authority_consumed(
        &snapshot,
        &"1".repeat(64),
        &"2".repeat(64)
    ));
    assert!(
        snapshot
            .events
            .iter()
            .any(|event| { event.plan_hash == "4".repeat(64) && event.phase == Phase::InDoubt })
    );
    assert!(snapshot.events.iter().any(|event| {
        event.plan_hash == "6".repeat(64) && event.phase == Phase::ReconciledTerminalSucceeded
    }));
    assert!(snapshot.monotonic_binding().0 > before.0);

    let encoded = encode_journal_snapshot(&snapshot)?;
    let decoded = read_journal_snapshot(&encoded, &KEY)?;
    assert_eq!(decoded.checkpoint, snapshot.checkpoint);
    assert_eq!(decoded.events, snapshot.events);

    let mut tampered = encoded;
    let checkpoint_mac = checkpoint.mac.as_bytes();
    let offset = tampered
        .windows(checkpoint_mac.len())
        .position(|window| window == checkpoint_mac)
        .context("checkpoint MAC")?;
    tampered[offset] = if tampered[offset] == b'a' { b'b' } else { b'a' };
    assert!(read_journal_snapshot(&tampered, &KEY).is_err());
    Ok(())
}
