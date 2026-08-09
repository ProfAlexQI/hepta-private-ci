use std::num::NonZeroU64;
use std::path::Path;

use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use sqlx::Executor;
use sqlx::SqlitePool;
use tempfile::TempDir;

use super::ConsumptionRecord;
use super::HistoryPoint;
use super::RatchetRecord;
use super::SqlitePromotionReceiptReplayStore;
use super::verify_promotion_replay_integrity;
use crate::EvidenceError;
use crate::HeptaEvidenceStore;
use crate::schema_validation::verify_schema_manifest;
use codex_hepta_contracts::MAX_PROMOTION_REVOCATIONS_PER_KIND;
use codex_hepta_contracts::PromotionReplayStoreError;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

struct TestDatabase {
    directory: TempDir,
    path: std::path::PathBuf,
    pool: SqlitePool,
    store: SqlitePromotionReceiptReplayStore,
}

impl TestDatabase {
    async fn new() -> Self {
        let directory = tempfile::tempdir().expect("create test database directory");
        let path = directory.path().join("promotion-replay.sqlite");
        let pool = open_pool(&path).await;
        MIGRATOR
            .run(&pool)
            .await
            .expect("apply evidence migrations");
        let store = SqlitePromotionReceiptReplayStore::new(
            pool.clone(),
            NonZeroU64::new(1_000).expect("nonzero forward-jump policy"),
        );
        Self {
            directory,
            path,
            pool,
            store,
        }
    }

    async fn reopen(&mut self) {
        self.pool.close().await;
        self.pool = open_pool(&self.path).await;
        self.store = SqlitePromotionReceiptReplayStore::new(
            self.pool.clone(),
            NonZeroU64::new(1_000).expect("nonzero forward-jump policy"),
        );
    }
}

async fn open_pool(path: &Path) -> SqlitePool {
    let home = path.parent().expect("promotion replay database parent");
    let sqlite = SqliteConfig::new_for_testing(
        AbsolutePathBuf::try_from(home.to_path_buf()).expect("absolute test database parent"),
    );
    sqlite
        .open_durable_evidence_pool(path)
        .await
        .expect("open promotion replay test database")
}

fn digest(byte: u8) -> String {
    format!("{byte:02x}").repeat(32)
}

fn history_point(record: &RatchetRecord) -> HistoryPoint {
    HistoryPoint {
        trust_root_revision: record.trust_root_revision,
        trust_root_sha256: record.trust_root_sha256.clone(),
        revocation_revision: record.revocation_revision,
        revocations_sha256: record.revocations_sha256.clone(),
        history_chain_sha256: record.history_chain_sha256.clone(),
    }
}

fn initial_ratchet(observed_at_unix_seconds: u64) -> RatchetRecord {
    let mut record = RatchetRecord {
        checkpoint_source_json_sha256: digest(1),
        checkpoint_sha256: digest(2),
        checkpoint_trust_root_id: "root-a".to_string(),
        genesis_trust_root_sha256: digest(3),
        checkpoint_terminal_history_chain_sha256: digest(6),
        checkpoint_terminal_trust_root_revision: 1,
        checkpoint_terminal_trust_root_sha256: digest(4),
        checkpoint_terminal_revocation_revision: 1,
        checkpoint_terminal_revocations_sha256: digest(5),
        history: Vec::new(),
        trust_root_id: "root-a".to_string(),
        trust_root_revision: 1,
        trust_root_sha256: digest(4),
        revocation_trust_root_id: "root-a".to_string(),
        revocation_revision: 1,
        revocations_sha256: digest(5),
        history_chain_sha256: digest(6),
        revoked_key_ids: Vec::new(),
        revoked_receipt_sha256: Vec::new(),
        revoked_nonces: Vec::new(),
        observed_at_unix_seconds,
    };
    record.history.push(history_point(&record));
    record
}

fn synchronize_checkpoint(record: &mut RatchetRecord) {
    record.checkpoint_terminal_history_chain_sha256 = record.history_chain_sha256.clone();
    record.checkpoint_terminal_trust_root_revision = record.trust_root_revision;
    record.checkpoint_terminal_trust_root_sha256 = record.trust_root_sha256.clone();
    record.checkpoint_terminal_revocation_revision = record.revocation_revision;
    record.checkpoint_terminal_revocations_sha256 = record.revocations_sha256.clone();
}

fn advance(
    predecessor: &RatchetRecord,
    trust_root_revision: u64,
    trust_root_digest_byte: u8,
    revocation_revision: u64,
    revocations_digest_byte: u8,
    history_chain_byte: u8,
    observed_at_unix_seconds: u64,
) -> RatchetRecord {
    let mut record = predecessor.clone();
    record.checkpoint_source_json_sha256 = digest(history_chain_byte.wrapping_add(20));
    record.checkpoint_sha256 = digest(history_chain_byte.wrapping_add(40));
    record.trust_root_revision = trust_root_revision;
    record.trust_root_sha256 = digest(trust_root_digest_byte);
    record.revocation_revision = revocation_revision;
    record.revocations_sha256 = digest(revocations_digest_byte);
    record.history_chain_sha256 = digest(history_chain_byte);
    record.observed_at_unix_seconds = observed_at_unix_seconds;
    synchronize_checkpoint(&mut record);
    record.history.push(history_point(&record));
    record
}

fn consumption(
    ratchet: &RatchetRecord,
    nonce_digest_byte: u8,
    receipt_digest_byte: u8,
) -> ConsumptionRecord {
    ConsumptionRecord {
        trust_root_id: ratchet.trust_root_id.clone(),
        checkpoint_sha256: ratchet.checkpoint_sha256.clone(),
        trust_root_revision: ratchet.trust_root_revision,
        trust_root_sha256: ratchet.trust_root_sha256.clone(),
        revocation_revision: ratchet.revocation_revision,
        revocations_sha256: ratchet.revocations_sha256.clone(),
        history_chain_sha256: ratchet.history_chain_sha256.clone(),
        observed_at_unix_seconds: ratchet.observed_at_unix_seconds,
        nonce: digest(nonce_digest_byte),
        receipt_sha256: digest(receipt_digest_byte),
        expires_at_unix_seconds: ratchet.observed_at_unix_seconds + 100,
    }
}

async fn scalar_count(pool: &SqlitePool, table: &str) -> i64 {
    let sql = match table {
        "promotion_trust_watermarks" => "SELECT COUNT(*) FROM promotion_trust_watermarks",
        "promotion_revoked_key_tombstones" => {
            "SELECT COUNT(*) FROM promotion_revoked_key_tombstones"
        }
        "promotion_revoked_receipt_tombstones" => {
            "SELECT COUNT(*) FROM promotion_revoked_receipt_tombstones"
        }
        "promotion_revoked_nonce_tombstones" => {
            "SELECT COUNT(*) FROM promotion_revoked_nonce_tombstones"
        }
        "promotion_receipt_consumptions" => "SELECT COUNT(*) FROM promotion_receipt_consumptions",
        _ => panic!("unexpected promotion replay test table: {table}"),
    };
    sqlx::query_scalar(sql)
        .fetch_one(pool)
        .await
        .expect("count test rows")
}

#[tokio::test]
async fn fresh_bootstrap_requires_nonempty_exact_terminal_history() {
    let database = TestDatabase::new().await;
    let mut empty = initial_ratchet(100);
    empty.history.clear();
    assert_eq!(
        database.store.ratchet_record(empty).await,
        Err(PromotionReplayStoreError::TrustRootHistoryMismatch)
    );

    let mut wrong_terminal = initial_ratchet(100);
    wrong_terminal.history_chain_sha256 = digest(77);
    synchronize_checkpoint(&mut wrong_terminal);
    assert_eq!(
        database.store.ratchet_record(wrong_terminal).await,
        Err(PromotionReplayStoreError::TrustRootHistoryMismatch)
    );
    assert_eq!(
        scalar_count(&database.pool, "promotion_trust_watermarks").await,
        0
    );
}

#[tokio::test]
async fn ratchet_persists_exact_watermark_and_all_tombstone_kinds() {
    let database = TestDatabase::new().await;
    let mut record = initial_ratchet(100);
    record.revoked_key_ids = vec!["key-a".to_string(), "key-b".to_string()];
    record.revoked_receipt_sha256 = vec![digest(70), digest(71)];
    record.revoked_nonces = vec![digest(72), digest(73)];
    database
        .store
        .ratchet_record(record.clone())
        .await
        .expect("initialize durable promotion ratchet");

    let row = sqlx::query(
        "SELECT checkpoint_source_json_sha256, checkpoint_sha256,
                genesis_trust_root_sha256, trust_root_revision,
                trust_root_sha256, revocation_revision, revocations_sha256,
                history_chain_sha256, max_observed_time_unix_seconds
         FROM promotion_trust_watermarks WHERE trust_root_id = ?",
    )
    .bind(&record.trust_root_id)
    .fetch_one(&database.pool)
    .await
    .expect("load stored promotion watermark");
    use sqlx::Row as _;
    assert_eq!(
        row.get::<String, _>("checkpoint_source_json_sha256"),
        record.checkpoint_source_json_sha256
    );
    assert_eq!(
        row.get::<String, _>("checkpoint_sha256"),
        record.checkpoint_sha256
    );
    assert_eq!(
        row.get::<String, _>("genesis_trust_root_sha256"),
        record.genesis_trust_root_sha256
    );
    assert_eq!(row.get::<i64, _>("trust_root_revision"), 1);
    assert_eq!(
        row.get::<String, _>("trust_root_sha256"),
        record.trust_root_sha256
    );
    assert_eq!(row.get::<i64, _>("revocation_revision"), 1);
    assert_eq!(
        row.get::<String, _>("revocations_sha256"),
        record.revocations_sha256
    );
    assert_eq!(
        row.get::<String, _>("history_chain_sha256"),
        record.history_chain_sha256
    );
    assert_eq!(row.get::<i64, _>("max_observed_time_unix_seconds"), 100);
    assert_eq!(
        scalar_count(&database.pool, "promotion_revoked_key_tombstones").await,
        2
    );
    assert_eq!(
        scalar_count(&database.pool, "promotion_revoked_receipt_tombstones").await,
        2
    );
    assert_eq!(
        scalar_count(&database.pool, "promotion_revoked_nonce_tombstones").await,
        2
    );
}

#[tokio::test]
async fn ratchet_enforces_clock_genesis_revision_and_chain_ancestry() {
    let database = TestDatabase::new().await;
    let initial = initial_ratchet(100);
    database
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize ratchet");

    let mut clock_rollback = initial.clone();
    clock_rollback.observed_at_unix_seconds = 99;
    assert_eq!(
        database.store.ratchet_record(clock_rollback).await,
        Err(PromotionReplayStoreError::ClockRollback {
            highest_seen: 100,
            found: 99,
        })
    );

    let mut excessive_forward_jump = initial.clone();
    excessive_forward_jump.observed_at_unix_seconds = 1_101;
    let jump_result = database.store.ratchet_record(excessive_forward_jump).await;
    assert!(matches!(
        jump_result,
        Err(PromotionReplayStoreError::Storage(detail))
            if detail.contains("authenticated store recovery")
    ));

    let mut genesis_conflict = initial.clone();
    genesis_conflict.genesis_trust_root_sha256 = digest(90);
    assert_eq!(
        database.store.ratchet_record(genesis_conflict).await,
        Err(PromotionReplayStoreError::GenesisAnchorConflict)
    );

    let mut root_conflict = initial.clone();
    root_conflict.trust_root_sha256 = digest(91);
    synchronize_checkpoint(&mut root_conflict);
    root_conflict.history = vec![history_point(&root_conflict)];
    assert_eq!(
        database.store.ratchet_record(root_conflict).await,
        Err(PromotionReplayStoreError::TrustRootRevisionConflict { revision: 1 })
    );

    let mut revocation_conflict = initial.clone();
    revocation_conflict.revocations_sha256 = digest(92);
    synchronize_checkpoint(&mut revocation_conflict);
    revocation_conflict.history = vec![history_point(&revocation_conflict)];
    assert_eq!(
        database.store.ratchet_record(revocation_conflict).await,
        Err(PromotionReplayStoreError::RevocationRevisionConflict { revision: 1 })
    );

    let advanced = advance(&initial, 2, 10, 2, 11, 12, 100);
    database
        .store
        .ratchet_record(advanced.clone())
        .await
        .expect("advance exact ancestor chain");

    let mut root_rollback = initial.clone();
    root_rollback.observed_at_unix_seconds = 101;
    assert_eq!(
        database.store.ratchet_record(root_rollback).await,
        Err(PromotionReplayStoreError::TrustRootRevisionRollback {
            highest_seen: 2,
            found: 1,
        })
    );

    let mut revocation_rollback = advanced.clone();
    revocation_rollback.trust_root_revision = 3;
    revocation_rollback.trust_root_sha256 = digest(13);
    revocation_rollback.revocation_revision = 1;
    revocation_rollback.revocations_sha256 = digest(5);
    revocation_rollback.history_chain_sha256 = digest(14);
    synchronize_checkpoint(&mut revocation_rollback);
    revocation_rollback
        .history
        .push(history_point(&revocation_rollback));
    assert_eq!(
        database.store.ratchet_record(revocation_rollback).await,
        Err(PromotionReplayStoreError::RevocationRevisionRollback {
            highest_seen: 2,
            found: 1,
        })
    );

    // Same current state digests reached through a different cumulative chain
    // must not erase the durable history semantics by convergence.
    let mut convergent_branch = advanced.clone();
    convergent_branch.history_chain_sha256 = digest(99);
    synchronize_checkpoint(&mut convergent_branch);
    convergent_branch.history = vec![history_point(&convergent_branch)];
    assert_eq!(
        database.store.ratchet_record(convergent_branch).await,
        Err(PromotionReplayStoreError::TrustRootHistoryMismatch)
    );
}

#[tokio::test]
async fn removed_tombstone_rolls_back_the_entire_ratchet() {
    for removed_kind in ["key-id", "receipt", "nonce"] {
        let database = TestDatabase::new().await;
        let mut initial = initial_ratchet(100);
        initial.revoked_key_ids = vec!["key-a".to_string()];
        initial.revoked_receipt_sha256 = vec![digest(70)];
        initial.revoked_nonces = vec![digest(72)];
        database
            .store
            .ratchet_record(initial.clone())
            .await
            .expect("initialize tombstones");

        let mut successor = advance(&initial, 2, 10, 2, 11, 12, 101);
        successor.revoked_key_ids.push("key-b".to_string());
        successor.revoked_receipt_sha256.push(digest(71));
        successor.revoked_nonces.push(digest(73));
        match removed_kind {
            "key-id" => successor.revoked_key_ids.remove(0),
            "receipt" => successor.revoked_receipt_sha256.remove(0),
            "nonce" => successor.revoked_nonces.remove(0),
            _ => unreachable!(),
        };

        assert_eq!(
            database.store.ratchet_record(successor).await,
            Err(PromotionReplayStoreError::RevocationTombstoneRemoved(
                removed_kind
            ))
        );
        let revision: i64 =
            sqlx::query_scalar("SELECT trust_root_revision FROM promotion_trust_watermarks")
                .fetch_one(&database.pool)
                .await
                .expect("read unchanged watermark");
        assert_eq!(revision, 1);
        assert_eq!(
            scalar_count(&database.pool, "promotion_revoked_key_tombstones").await,
            1
        );
        assert_eq!(
            scalar_count(&database.pool, "promotion_revoked_receipt_tombstones").await,
            1
        );
        assert_eq!(
            scalar_count(&database.pool, "promotion_revoked_nonce_tombstones").await,
            1
        );
    }
}

#[tokio::test]
async fn consumption_error_priority_is_fail_closed_and_atomic() {
    let database = TestDatabase::new().await;
    let initial = initial_ratchet(100);
    let exact = consumption(&initial, 80, 40);

    assert_eq!(
        database.store.consume_record(exact.clone()).await,
        Err(PromotionReplayStoreError::TrustedConfigurationNotInitialized)
    );
    database
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize ratchet");
    database
        .store
        .consume_record(exact.clone())
        .await
        .expect("consume exact receipt");

    let mut rollback_replay = exact.clone();
    rollback_replay.observed_at_unix_seconds = 99;
    assert_eq!(
        database.store.consume_record(rollback_replay).await,
        Err(PromotionReplayStoreError::ClockRollback {
            highest_seen: 100,
            found: 99,
        })
    );

    let mut future_replay = exact.clone();
    future_replay.observed_at_unix_seconds = 101;
    assert_eq!(
        database.store.consume_record(future_replay).await,
        Err(PromotionReplayStoreError::TrustedConfigurationNotCurrent)
    );

    let mut stale_state_replay = exact.clone();
    stale_state_replay.history_chain_sha256 = digest(99);
    assert_eq!(
        database.store.consume_record(stale_state_replay).await,
        Err(PromotionReplayStoreError::TrustedConfigurationNotCurrent)
    );

    assert_eq!(
        database.store.consume_record(exact.clone()).await,
        Err(PromotionReplayStoreError::ReceiptReplay)
    );

    let mut nonce_replay = exact;
    nonce_replay.receipt_sha256 = digest(41);
    assert_eq!(
        database.store.consume_record(nonce_replay).await,
        Err(PromotionReplayStoreError::NonceReplay)
    );
    assert_eq!(
        scalar_count(&database.pool, "promotion_receipt_consumptions").await,
        1
    );
}

#[tokio::test]
async fn consumption_and_fresh_initialization_are_atomic_under_race() {
    let database = TestDatabase::new().await;
    let initial = initial_ratchet(100);
    database
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize ratchet");
    let exact = consumption(&initial, 81, 50);
    let (left, right) = tokio::join!(
        database.store.consume_record(exact.clone()),
        database.store.consume_record(exact)
    );
    assert_eq!(usize::from(left.is_ok()) + usize::from(right.is_ok()), 1);
    let loser = if left.is_err() { left } else { right };
    assert_eq!(loser, Err(PromotionReplayStoreError::ReceiptReplay));

    let same_nonce_left = consumption(&initial, 82, 51);
    let same_nonce_right = consumption(&initial, 82, 52);
    let (left, right) = tokio::join!(
        database.store.consume_record(same_nonce_left),
        database.store.consume_record(same_nonce_right)
    );
    assert_eq!(usize::from(left.is_ok()) + usize::from(right.is_ok()), 1);
    let loser = if left.is_err() { left } else { right };
    assert_eq!(loser, Err(PromotionReplayStoreError::NonceReplay));

    let other_database = TestDatabase::new().await;
    let branch_a = initial_ratchet(100);
    let mut branch_b = branch_a.clone();
    branch_b.history_chain_sha256 = digest(88);
    branch_b.checkpoint_sha256 = digest(89);
    branch_b.checkpoint_source_json_sha256 = digest(90);
    synchronize_checkpoint(&mut branch_b);
    branch_b.history = vec![history_point(&branch_b)];
    let (left, right) = tokio::join!(
        other_database.store.ratchet_record(branch_a),
        other_database.store.ratchet_record(branch_b)
    );
    assert_eq!(usize::from(left.is_ok()) + usize::from(right.is_ok()), 1);
    let loser = if left.is_err() { left } else { right };
    assert_eq!(
        loser,
        Err(PromotionReplayStoreError::TrustRootHistoryMismatch)
    );
    assert_eq!(
        scalar_count(&other_database.pool, "promotion_trust_watermarks").await,
        1
    );
}

#[tokio::test]
async fn stale_consume_cannot_cross_or_follow_a_higher_ratchet() {
    let database = TestDatabase::new().await;
    let initial = initial_ratchet(100);
    database
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize ratchet");
    let advanced = advance(&initial, 2, 10, 2, 11, 12, 100);
    let stale = consumption(&initial, 84, 53);
    let (ratchet_result, consume_result) = tokio::join!(
        database.store.ratchet_record(advanced.clone()),
        database.store.consume_record(stale)
    );
    ratchet_result.expect("higher ratchet wins or follows serialized consume");
    assert!(matches!(
        consume_result,
        Ok(()) | Err(PromotionReplayStoreError::TrustedConfigurationNotCurrent)
    ));
    assert_eq!(
        database
            .store
            .consume_record(consumption(&initial, 85, 54))
            .await,
        Err(PromotionReplayStoreError::TrustedConfigurationNotCurrent)
    );

    let ratchet_first = TestDatabase::new().await;
    ratchet_first
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize second ratchet");
    ratchet_first
        .store
        .ratchet_record(advanced)
        .await
        .expect("advance second ratchet first");
    assert_eq!(
        ratchet_first
            .store
            .consume_record(consumption(&initial, 86, 55))
            .await,
        Err(PromotionReplayStoreError::TrustedConfigurationNotCurrent)
    );
}

#[tokio::test]
async fn tombstone_insert_failure_rolls_back_a_prior_watermark_update() {
    let database = TestDatabase::new().await;
    let initial = initial_ratchet(100);
    database
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize ratchet");
    let mut successor = advance(&initial, 2, 10, 2, 11, 12, 101);
    successor
        .revoked_nonces
        .push("not-a-canonical-nonce".to_string());
    assert!(matches!(
        database.store.ratchet_record(successor).await,
        Err(PromotionReplayStoreError::Storage(_))
    ));

    let row: (i64, String, i64, String, String, i64) = sqlx::query_as(
        "SELECT trust_root_revision, trust_root_sha256,
                revocation_revision, revocations_sha256,
                history_chain_sha256, max_observed_time_unix_seconds
         FROM promotion_trust_watermarks WHERE trust_root_id = 'root-a'",
    )
    .fetch_one(&database.pool)
    .await
    .expect("read watermark after failed tombstone insert");
    assert_eq!(
        row,
        (
            1,
            initial.trust_root_sha256,
            1,
            initial.revocations_sha256,
            initial.history_chain_sha256,
            100,
        )
    );
    assert_eq!(
        scalar_count(&database.pool, "promotion_revoked_nonce_tombstones").await,
        0
    );
}

#[tokio::test]
async fn consumed_receipts_and_nonces_survive_restart() {
    let mut database = TestDatabase::new().await;
    assert!(database.directory.path().exists());
    let initial = initial_ratchet(100);
    let exact = consumption(&initial, 82, 60);
    database
        .store
        .ratchet_record(initial)
        .await
        .expect("initialize ratchet");
    database
        .store
        .consume_record(exact.clone())
        .await
        .expect("consume before restart");
    database.reopen().await;

    assert_eq!(
        database.store.consume_record(exact.clone()).await,
        Err(PromotionReplayStoreError::ReceiptReplay)
    );
    let mut same_nonce = exact;
    same_nonce.receipt_sha256 = digest(61);
    assert_eq!(
        database.store.consume_record(same_nonce).await,
        Err(PromotionReplayStoreError::NonceReplay)
    );
}

#[tokio::test]
async fn schema_guards_watermark_rollback_and_permanent_rows() {
    let database = TestDatabase::new().await;
    let mut initial = initial_ratchet(100);
    initial.revoked_key_ids = vec!["key-a".to_string()];
    initial.revoked_receipt_sha256 = vec![digest(70)];
    initial.revoked_nonces = vec![digest(72)];
    database
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize ratchet");
    database
        .store
        .consume_record(consumption(&initial, 83, 71))
        .await
        .expect("consume receipt");

    for statement in [
        "UPDATE promotion_trust_watermarks SET max_observed_time_unix_seconds = 99",
        "DELETE FROM promotion_trust_watermarks",
        "UPDATE promotion_revoked_key_tombstones SET revoked_key_id = 'key-z'",
        "DELETE FROM promotion_revoked_key_tombstones",
        "UPDATE promotion_revoked_receipt_tombstones SET revoked_receipt_sha256 = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'",
        "DELETE FROM promotion_revoked_receipt_tombstones",
        "UPDATE promotion_revoked_nonce_tombstones SET revoked_nonce = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'",
        "DELETE FROM promotion_revoked_nonce_tombstones",
        "UPDATE promotion_receipt_consumptions SET nonce = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'",
        "DELETE FROM promotion_receipt_consumptions",
    ] {
        assert!(
            database.pool.execute(statement).await.is_err(),
            "schema unexpectedly permitted: {statement}"
        );
    }
}

#[tokio::test]
async fn tombstone_readback_is_streamed_and_hard_bounded() {
    let database = TestDatabase::new().await;
    let initial = initial_ratchet(100);
    database
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize ratchet");

    sqlx::query(
        "WITH RECURSIVE generated(value) AS (
             SELECT 0
             UNION ALL
             SELECT value + 1 FROM generated WHERE value < 4096
         )
         INSERT INTO promotion_revoked_key_tombstones (
             trust_root_id, revoked_key_id,
             durably_observed_revocation_revision,
             durably_observed_history_chain_sha256
         )
         SELECT 'root-a', printf('key-%04d', value), 1, ? FROM generated",
    )
    .bind(&initial.history_chain_sha256)
    .execute(&database.pool)
    .await
    .expect("inject externally oversized tombstone set");

    let mut replay = initial;
    replay.revoked_key_ids = (0..MAX_PROMOTION_REVOCATIONS_PER_KIND)
        .map(|value| format!("key-{value:04}"))
        .collect();
    let result = database.store.ratchet_record(replay).await;
    assert!(matches!(result, Err(PromotionReplayStoreError::Storage(_))));
    assert!(matches!(
        verify_promotion_replay_integrity(&database.pool).await,
        Err(EvidenceError::Corrupt(detail)) if detail.contains("contract bound")
    ));
}

#[tokio::test]
async fn exact_schema_fingerprint_rejects_fragment_spoofing_trigger() {
    let database = TestDatabase::new().await;
    sqlx::raw_sql(
        "DROP TRIGGER promotion_receipt_consumptions_no_delete;
         CREATE TRIGGER promotion_receipt_consumptions_no_delete
             BEFORE DELETE ON promotion_receipt_consumptions
             BEGIN
                 SELECT 'raise(abort';
             END;",
    )
    .execute(&database.pool)
    .await
    .expect("install fragment-spoofing trigger");
    assert!(matches!(
        verify_schema_manifest(&database.pool).await,
        Err(EvidenceError::Corrupt(detail))
            if detail.contains("fingerprinted SQLite trigger")
    ));
}

#[tokio::test]
async fn reopen_rejects_any_extra_trigger_on_promotion_tables() {
    let directory = tempfile::tempdir().expect("create evidence home");
    let sqlite = SqliteConfig::new_for_testing(
        AbsolutePathBuf::try_from(directory.path().to_path_buf()).expect("absolute evidence home"),
    );
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("create evidence database");
    let database_path = store.path().to_path_buf();
    store.pool.close().await;
    drop(store);

    let pool = sqlite
        .open_durable_evidence_pool(&database_path)
        .await
        .expect("open database for trigger injection");
    pool.execute(
        "CREATE TRIGGER arbitrary_promotion_consumption_ignore
         BEFORE INSERT ON promotion_receipt_consumptions
         BEGIN
             SELECT RAISE(IGNORE);
         END",
    )
    .await
    .expect("inject arbitrary promotion trigger");
    pool.close().await;

    assert!(matches!(
        HeptaEvidenceStore::open(&sqlite).await,
        Err(EvidenceError::Corrupt(detail))
            if detail.contains("promotion replay trigger")
    ));
}

#[tokio::test]
async fn post_open_ignore_trigger_cannot_report_a_consumption_without_durable_row() {
    let database = TestDatabase::new().await;
    let initial = initial_ratchet(100);
    database
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize ratchet");
    database
        .pool
        .execute(
            "CREATE TRIGGER arbitrary_promotion_consumption_ignore
             BEFORE INSERT ON promotion_receipt_consumptions
             BEGIN
                 SELECT RAISE(IGNORE);
             END",
        )
        .await
        .expect("inject post-open ignore trigger");
    let exact = consumption(&initial, 93, 94);
    for _ in 0..2 {
        assert!(matches!(
            database.store.consume_record(exact.clone()).await,
            Err(PromotionReplayStoreError::Storage(detail))
                if detail.contains("exactly one row")
        ));
    }
    assert_eq!(
        scalar_count(&database.pool, "promotion_receipt_consumptions").await,
        0
    );

    database
        .pool
        .execute("DROP TRIGGER arbitrary_promotion_consumption_ignore")
        .await
        .expect("remove post-open ignore trigger");
    database
        .store
        .consume_record(exact.clone())
        .await
        .expect("consume once after restoring schema");
    assert_eq!(
        database.store.consume_record(exact).await,
        Err(PromotionReplayStoreError::ReceiptReplay)
    );
}

#[tokio::test]
async fn post_open_ignore_triggers_cannot_fake_fresh_watermark_or_tombstone_writes() {
    let database = TestDatabase::new().await;
    database
        .pool
        .execute(
            "CREATE TRIGGER arbitrary_fresh_watermark_ignore
             BEFORE INSERT ON promotion_trust_watermarks
             BEGIN
                 SELECT RAISE(IGNORE);
             END",
        )
        .await
        .expect("inject fresh watermark ignore trigger");
    let initial = initial_ratchet(100);
    assert!(matches!(
        database.store.ratchet_record(initial.clone()).await,
        Err(PromotionReplayStoreError::Storage(detail))
            if detail.contains("exactly one row")
    ));
    assert_eq!(
        scalar_count(&database.pool, "promotion_trust_watermarks").await,
        0
    );
    database
        .pool
        .execute("DROP TRIGGER arbitrary_fresh_watermark_ignore")
        .await
        .expect("remove fresh watermark ignore trigger");
    database
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize after restoring watermark schema");

    database
        .pool
        .execute(
            "CREATE TRIGGER arbitrary_key_tombstone_ignore
             BEFORE INSERT ON promotion_revoked_key_tombstones
             BEGIN
                 SELECT RAISE(IGNORE);
             END",
        )
        .await
        .expect("inject tombstone ignore trigger");
    let mut successor = advance(&initial, 1, 4, 2, 11, 12, 101);
    successor.revoked_key_ids.push("key-new".to_string());
    assert!(matches!(
        database.store.ratchet_record(successor).await,
        Err(PromotionReplayStoreError::Storage(detail))
            if detail.contains("without a durable row")
    ));
    let revision: i64 = sqlx::query_scalar(
        "SELECT revocation_revision FROM promotion_trust_watermarks WHERE trust_root_id = 'root-a'",
    )
    .fetch_one(&database.pool)
    .await
    .expect("read watermark after ignored tombstone");
    assert_eq!(revision, 1);
    assert_eq!(
        scalar_count(&database.pool, "promotion_revoked_key_tombstones").await,
        0
    );
}

#[tokio::test]
async fn fresh_watermark_exact_readback_rejects_after_insert_mutation() {
    let database = TestDatabase::new().await;
    database
        .pool
        .execute(
            "CREATE TRIGGER arbitrary_fresh_watermark_mutation
             AFTER INSERT ON promotion_trust_watermarks
             BEGIN
                 UPDATE promotion_trust_watermarks
                 SET checkpoint_sha256 = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
                 WHERE trust_root_id = NEW.trust_root_id;
             END",
        )
        .await
        .expect("inject fresh watermark mutation trigger");
    assert!(matches!(
        database.store.ratchet_record(initial_ratchet(100)).await,
        Err(PromotionReplayStoreError::Storage(detail))
            if detail.contains("exact readback differed")
    ));
    assert_eq!(
        scalar_count(&database.pool, "promotion_trust_watermarks").await,
        0
    );
}

#[tokio::test]
async fn open_integrity_rejects_late_tombstone_provenance_and_consumption_state() {
    let database = TestDatabase::new().await;
    let initial = initial_ratchet(100);
    database
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize ratchet");
    sqlx::query(
        "INSERT INTO promotion_revoked_key_tombstones (
            trust_root_id, revoked_key_id,
            durably_observed_revocation_revision,
            durably_observed_history_chain_sha256
         ) VALUES (?, 'key-late', 2, ?)",
    )
    .bind(&initial.trust_root_id)
    .bind(&initial.history_chain_sha256)
    .execute(&database.pool)
    .await
    .expect("inject late tombstone provenance");
    assert!(matches!(
        verify_promotion_replay_integrity(&database.pool).await,
        Err(EvidenceError::Corrupt(detail)) if detail.contains("later than")
    ));

    let other = TestDatabase::new().await;
    other
        .store
        .ratchet_record(initial.clone())
        .await
        .expect("initialize second ratchet");
    sqlx::query(
        "INSERT INTO promotion_receipt_consumptions (
            trust_root_id, schema_version, checkpoint_sha256,
            trust_root_revision, trust_root_sha256,
            revocation_revision, revocations_sha256, history_chain_sha256,
            observed_at_unix_seconds, nonce, receipt_sha256,
            expires_at_unix_seconds
         ) VALUES (?, 1, ?, 2, ?, 1, ?, ?, 100, ?, ?, 101)",
    )
    .bind(&initial.trust_root_id)
    .bind(&initial.checkpoint_sha256)
    .bind(digest(90))
    .bind(&initial.revocations_sha256)
    .bind(&initial.history_chain_sha256)
    .bind(digest(91))
    .bind(digest(92))
    .execute(&other.pool)
    .await
    .expect("inject consumption later than watermark");
    assert!(matches!(
        verify_promotion_replay_integrity(&other.pool).await,
        Err(EvidenceError::Corrupt(detail)) if detail.contains("later than")
    ));
}
