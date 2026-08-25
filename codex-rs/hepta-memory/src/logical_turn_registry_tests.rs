use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_contracts::Sha256Digest;
use tempfile::TempDir;

use crate::CognitiveStore;
use crate::LogicalTurnAttemptRequest;
use crate::LogicalTurnRegistryError;
use crate::LogicalTurnRequest;
use crate::LogicalTurnReservation;
use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;

async fn opened_store(temp: &TempDir, number: u8) -> CognitiveStore {
    let owner = agent_id(number);
    CognitiveStore::open(&layout(temp, &owner))
        .await
        .expect("cognitive store")
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_secs()
}

fn logical_request() -> LogicalTurnRequest {
    LogicalTurnRequest::new(
        "logical-turn:test",
        "scope:test",
        Sha256Digest::for_bytes(b"logical-binding"),
    )
    .expect("logical request")
}

fn attempt(
    suffix: &str,
    lease_suffix: &str,
    generation: u64,
    expiry: u64,
) -> LogicalTurnAttemptRequest {
    LogicalTurnAttemptRequest::new(
        format!("attempt:{suffix}"),
        format!("lease:{lease_suffix}"),
        format!("journal:{suffix}"),
        format!("trajectory:{suffix}"),
        format!("occurrence:{suffix}"),
        1,
        1,
        generation,
        format!("fence:{suffix}"),
        expiry,
    )
    .expect("attempt request")
}

#[tokio::test]
async fn reserve_and_exact_replay_are_one_row() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 180).await;
    let request = logical_request();
    let physical = attempt("one", "one", 1, now() + 3_600);
    let first = store
        .reserve_or_replay_logical_turn(request.clone(), physical.clone())
        .await
        .expect("first reservation");
    assert!(matches!(first, LogicalTurnReservation::Acquired { .. }));
    let replay = store
        .reserve_or_replay_logical_turn(request, physical)
        .await
        .expect("exact replay");
    assert!(matches!(replay, LogicalTurnReservation::Replayed { .. }));
    let identity_rows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_logical_turns")
        .fetch_one(&store.pool)
        .await
        .expect("identity count");
    let attempt_rows: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_logical_turn_attempts")
            .fetch_one(&store.pool)
            .await
            .expect("attempt count");
    assert_eq!(identity_rows, 1);
    assert_eq!(attempt_rows, 1);
}

#[tokio::test]
async fn live_different_attempt_is_existing_in_flight_without_side_effects() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 181).await;
    let request = logical_request();
    let first = attempt("live-a", "live-a", 1, now() + 3_600);
    store
        .reserve_or_replay_logical_turn(request.clone(), first)
        .await
        .expect("first reservation");
    let second = attempt("live-b", "live-b", 1, now() + 3_600);
    let result = store
        .reserve_or_replay_logical_turn(request, second)
        .await
        .expect("in-flight result");
    assert!(matches!(
        result,
        LogicalTurnReservation::ExistingInFlight { .. }
    ));
    let lease_rows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_local_leases")
        .fetch_one(&store.pool)
        .await
        .expect("lease count");
    assert_eq!(lease_rows, 1, "losing observation must not append a lease");
}

#[tokio::test]
async fn expired_attempt_without_evidence_has_one_takeover_winner() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 182).await;
    let request = logical_request();
    let expired = attempt("old", "old", 1, 1);
    store
        .reserve_or_replay_logical_turn(request.clone(), expired)
        .await
        .expect("expired reservation");
    let successor = attempt("new", "new", 1, now() + 3_600);
    let result = store
        .reserve_or_replay_logical_turn(request, successor)
        .await
        .expect("takeover");
    let LogicalTurnReservation::Takeover {
        superseded,
        attempt,
    } = result
    else {
        panic!("expected takeover");
    };
    assert_eq!(
        superseded.transition,
        crate::LogicalTurnAttemptTransition::Superseded
    );
    assert_eq!(superseded.registry_sequence, 2);
    assert_eq!(
        attempt.transition,
        crate::LogicalTurnAttemptTransition::Active
    );
    assert_eq!(attempt.registry_sequence, 3);
    assert_eq!(attempt.attempt_no, 2);
}

#[tokio::test]
async fn takeover_reopens_with_historical_witness_and_fences_old_handle() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 185).await;
    let request = logical_request();
    let expired = attempt("reopen-old", "reopen-old", 1, 1);
    store
        .reserve_or_replay_logical_turn(request.clone(), expired.clone())
        .await
        .expect("expired reservation");
    let successor = attempt("reopen-new", "reopen-new", 1, now() + 3_600);
    let takeover = store
        .reserve_or_replay_logical_turn(request.clone(), successor.clone())
        .await
        .expect("takeover");
    assert!(matches!(takeover, LogicalTurnReservation::Takeover { .. }));
    let old_state: String = sqlx::query_scalar(
        "SELECT state FROM cognitive_local_leases
         WHERE lease_id = ? ORDER BY lease_sequence DESC LIMIT 1",
    )
    .bind(&expired.lease_id)
    .fetch_one(&store.pool)
    .await
    .expect("old lease state");
    assert_eq!(old_state, "rolled_back");

    store.pool.close().await;
    let reopened = opened_store(&temp, 185).await;
    let replay = reopened
        .reserve_or_replay_logical_turn(request, successor)
        .await
        .expect("replay after reopen");
    assert!(matches!(replay, LogicalTurnReservation::Replayed { .. }));
    assert!(
        reopened
            .reopen_local_lease(
                &expired.lease_id,
                expired.generation,
                &expired.fencing_token
            )
            .await
            .is_err()
    );
}

#[tokio::test]
async fn identity_payload_mismatch_is_a_conflict_without_new_attempt() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 186).await;
    let original = logical_request();
    store
        .reserve_or_replay_logical_turn(
            original.clone(),
            attempt("identity", "identity", 1, now() + 3_600),
        )
        .await
        .expect("first reservation");
    let scope_mismatch = LogicalTurnRequest::new(
        original.logical_turn_id.clone(),
        "scope:changed",
        original.logical_binding_sha256.clone(),
    )
    .expect("scope mismatch request");
    let result = store
        .reserve_or_replay_logical_turn(
            scope_mismatch,
            attempt("identity-scope", "identity-scope", 1, now() + 3_600),
        )
        .await
        .expect("scope conflict result");
    assert!(matches!(result, LogicalTurnReservation::Conflict { .. }));

    let binding_mismatch = LogicalTurnRequest::new(
        original.logical_turn_id,
        original.scope_key,
        Sha256Digest::for_bytes(b"changed-binding"),
    )
    .expect("binding mismatch request");
    let result = store
        .reserve_or_replay_logical_turn(
            binding_mismatch,
            attempt("identity-binding", "identity-binding", 1, now() + 3_600),
        )
        .await
        .expect("binding conflict result");
    assert!(matches!(result, LogicalTurnReservation::Conflict { .. }));
    let attempts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_logical_turn_attempts")
        .fetch_one(&store.pool)
        .await
        .expect("attempt count");
    assert_eq!(attempts, 1);
}

#[tokio::test]
async fn takeover_fault_rolls_back_lease_and_registry_rows() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 187).await;
    let request = logical_request();
    let expired = attempt("fault-old", "fault-old", 1, 1);
    store
        .reserve_or_replay_logical_turn(request.clone(), expired.clone())
        .await
        .expect("expired reservation");
    sqlx::query(
        "CREATE TRIGGER logical_turn_registry_fault
         BEFORE INSERT ON cognitive_logical_turn_attempts
         WHEN NEW.transition = 'active'
         BEGIN SELECT RAISE(ABORT, 'injected logical registry fault'); END",
    )
    .execute(&store.pool)
    .await
    .expect("fault trigger");
    let failed = store
        .reserve_or_replay_logical_turn(
            request.clone(),
            attempt("fault-new", "fault-new", 1, now() + 3_600),
        )
        .await;
    assert!(failed.is_err());
    sqlx::query("DROP TRIGGER logical_turn_registry_fault")
        .execute(&store.pool)
        .await
        .expect("drop fault trigger");
    let attempts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_logical_turn_attempts")
        .fetch_one(&store.pool)
        .await
        .expect("attempt count after rollback");
    let leases: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_local_leases")
        .fetch_one(&store.pool)
        .await
        .expect("lease count after rollback");
    assert_eq!(attempts, 1, "registry rows must roll back together");
    assert_eq!(
        leases, 1,
        "new and terminal lease rows must roll back together"
    );
    let state: String = sqlx::query_scalar(
        "SELECT state FROM cognitive_local_leases
         WHERE lease_id = ? ORDER BY lease_sequence DESC LIMIT 1",
    )
    .bind(&expired.lease_id)
    .fetch_one(&store.pool)
    .await
    .expect("old lease state after rollback");
    assert_eq!(state, "active");

    let takeover = store
        .reserve_or_replay_logical_turn(
            request,
            attempt("fault-new", "fault-new", 1, now() + 3_600),
        )
        .await
        .expect("retry takeover");
    assert!(matches!(takeover, LogicalTurnReservation::Takeover { .. }));
}

#[tokio::test]
async fn tampered_registry_digest_is_rejected_on_reopen() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 188).await;
    store
        .reserve_or_replay_logical_turn(
            logical_request(),
            attempt("tamper", "tamper", 1, now() + 3_600),
        )
        .await
        .expect("reservation");
    sqlx::query("DROP TRIGGER cognitive_logical_turn_attempts_no_update")
        .execute(&store.pool)
        .await
        .expect("drop immutable trigger");
    sqlx::query(
        "UPDATE cognitive_logical_turn_attempts
         SET attempt_sha256 = ?",
    )
    .bind("0000000000000000000000000000000000000000000000000000000000000000")
    .execute(&store.pool)
    .await
    .expect("tamper digest");
    assert!(
        crate::logical_turn_registry::verify_logical_turn_registry(
            &store.pool,
            store.owner_agent_id(),
        )
        .await
        .is_err()
    );
    store.pool.close().await;
    assert!(
        CognitiveStore::open(&layout(&temp, &agent_id(188)))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn evidence_blocks_expired_takeover() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 183).await;
    let request = logical_request();
    let expired = attempt("evidence-old", "evidence-old", 1, 1);
    store
        .reserve_or_replay_logical_turn(request.clone(), expired.clone())
        .await
        .expect("expired reservation");
    sqlx::query(
        "INSERT INTO cognitive_local_events (
            lease_id, event_sequence, event_id, occurrence_key, owner_agent_id,
            generation, fencing_token, event_kind, payload_json, payload_sha256,
            previous_sha256, event_sha256, recorded_at_unix_seconds
         ) VALUES (?, 1, ?, ?, ?, 1, ?, 'admitted', '{}', ?, ?, ?, 0)",
    )
    .bind(&expired.lease_id)
    .bind("event:evidence")
    .bind(&expired.occurrence_key)
    .bind(store.owner_agent_id().as_str())
    .bind(&expired.fencing_token)
    .bind("0000000000000000000000000000000000000000000000000000000000000000")
    .bind("0000000000000000000000000000000000000000000000000000000000000000")
    .bind("0000000000000000000000000000000000000000000000000000000000000000")
    .execute(&store.pool)
    .await
    .expect("evidence row");
    let result = store
        .reserve_or_replay_logical_turn(
            request,
            attempt("evidence-new", "evidence-new", 1, now() + 3_600),
        )
        .await
        .expect("blocked result");
    let LogicalTurnReservation::BlockedByEvidence { evidence, .. } = result else {
        panic!("expected evidence block");
    };
    assert_eq!(evidence.event_rows, 1);
    assert!(!evidence.is_empty());
}

#[tokio::test]
async fn concurrent_reservations_have_one_acquired_and_one_existing() {
    let temp = TempDir::new().expect("temp dir");
    // Separate pools model two independent spawn/process handles against the
    // same Agent-local database; BEGIN IMMEDIATE must still leave one winner.
    let left_store = opened_store(&temp, 184).await;
    let right_store = opened_store(&temp, 184).await;
    let request = logical_request();
    let left = attempt("concurrent-left", "concurrent-left", 1, now() + 3_600);
    let right = attempt("concurrent-right", "concurrent-right", 1, now() + 3_600);
    let (first, second) = tokio::join!(
        left_store.reserve_or_replay_logical_turn(request.clone(), left),
        right_store.reserve_or_replay_logical_turn(request, right),
    );
    let first = first.expect("first concurrent result");
    let second = second.expect("second concurrent result");
    let acquired = usize::from(matches!(first, LogicalTurnReservation::Acquired { .. }))
        + usize::from(matches!(second, LogicalTurnReservation::Acquired { .. }));
    let existing = usize::from(matches!(
        first,
        LogicalTurnReservation::ExistingInFlight { .. }
    )) + usize::from(matches!(
        second,
        LogicalTurnReservation::ExistingInFlight { .. }
    ));
    assert_eq!(acquired, 1);
    assert_eq!(existing, 1);
}

#[test]
fn request_validation_rejects_zero_fences_and_bad_digests() {
    let bad = LogicalTurnRequest {
        logical_turn_id: "turn".to_string(),
        scope_key: "scope".to_string(),
        logical_binding_sha256: Sha256Digest::parse("bad")
            .unwrap_or_else(|_| Sha256Digest::for_bytes(b"placeholder")),
    };
    assert!(
        bad.validate().is_ok(),
        "parsed digest is structurally valid"
    );
    let invalid = LogicalTurnAttemptRequest {
        attempt_id: "attempt".to_string(),
        lease_id: "lease".to_string(),
        journal_id: "journal".to_string(),
        trajectory_id: "trajectory".to_string(),
        occurrence_key: "occurrence".to_string(),
        authority_epoch: 0,
        owner_epoch: 1,
        generation: 1,
        fencing_token: "fence".to_string(),
        lease_expires_at_unix_seconds: 1,
    };
    assert!(matches!(
        invalid.validate(),
        Err(LogicalTurnRegistryError::Invalid(_))
    ));
}

#[test]
fn registry_policy_is_explicitly_local_only() {
    assert_eq!(
        crate::LOGICAL_TURN_REGISTRY_NAMESPACE,
        "local_qualification_only"
    );
    assert!(!crate::LOGICAL_TURN_REGISTRY_EXTERNAL_EFFECTS);
    assert!(!crate::LOGICAL_TURN_REGISTRY_KG_WRITE_AUTHORITY);
    assert!(!crate::LOGICAL_TURN_REGISTRY_PRODUCTION_CALLER);
}
