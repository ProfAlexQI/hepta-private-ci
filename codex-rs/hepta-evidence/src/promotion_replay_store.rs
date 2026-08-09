use std::num::NonZeroU64;

use codex_hepta_contracts::MAX_PROMOTION_REVOCATIONS_PER_KIND;
use codex_hepta_contracts::MAX_PROMOTION_TRUST_HISTORY_UPDATES;
use codex_hepta_contracts::PromotionReceiptReplayStore;
use codex_hepta_contracts::PromotionReplayConsumption;
use codex_hepta_contracts::PromotionReplayFuture;
use codex_hepta_contracts::PromotionReplayStoreError;
use codex_hepta_contracts::PromotionTrustedConfigRatchet;
use codex_hepta_contracts::Sha256Digest;
use futures::TryStreamExt;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::Transaction;
use sqlx::sqlite::SqliteRow;

use crate::EvidenceError;
use crate::schema_validation::classify_sqlx_error;

const PROMOTION_REPLAY_SCHEMA_VERSION: i64 = 1;
const WATERMARK_INTEGRITY_PAGE_SIZE: i64 = 128;

const WATERMARK_SELECT: &str = "SELECT checkpoint_source_json_sha256, checkpoint_sha256,
            schema_version, genesis_trust_root_sha256,
            trust_root_revision, trust_root_sha256,
            revocation_revision, revocations_sha256, history_chain_sha256,
            max_observed_time_unix_seconds
     FROM promotion_trust_watermarks
     WHERE trust_root_id = ?";

const KEY_TOMBSTONE_SELECT: &str = "SELECT revoked_key_id
     FROM promotion_revoked_key_tombstones
     WHERE trust_root_id = ?
     ORDER BY revoked_key_id";

const RECEIPT_TOMBSTONE_SELECT: &str = "SELECT revoked_receipt_sha256
     FROM promotion_revoked_receipt_tombstones
     WHERE trust_root_id = ?
     ORDER BY revoked_receipt_sha256";

const NONCE_TOMBSTONE_SELECT: &str = "SELECT revoked_nonce
     FROM promotion_revoked_nonce_tombstones
     WHERE trust_root_id = ?
     ORDER BY revoked_nonce";

/// SQLite-backed replay protection for promotion evidence verification.
///
/// This store durably ratchets signed trust configuration, trusted time, and
/// permanent replay/revocation tombstones. It validates evidence identity; it
/// does not record operator acceptance and never grants promotion authority.
#[derive(Clone)]
pub struct SqlitePromotionReceiptReplayStore {
    pool: SqlitePool,
    max_forward_jump_seconds: NonZeroU64,
}

impl SqlitePromotionReceiptReplayStore {
    pub(crate) fn new(pool: SqlitePool, max_forward_jump_seconds: NonZeroU64) -> Self {
        Self {
            pool,
            max_forward_jump_seconds,
        }
    }

    async fn ratchet_record(&self, record: RatchetRecord) -> Result<(), PromotionReplayStoreError> {
        record.validate_internal_consistency()?;
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(storage_error)?;
        let current = load_watermark(&mut transaction, &record.trust_root_id).await?;

        if let Some(current) = current.as_ref() {
            validate_existing_ratchet(current, &record, self.max_forward_jump_seconds)?;
            ensure_tombstones_preserved(
                &mut transaction,
                KEY_TOMBSTONE_SELECT,
                &record.trust_root_id,
                &record.revoked_key_ids,
                "key-id",
            )
            .await?;
            ensure_tombstones_preserved(
                &mut transaction,
                RECEIPT_TOMBSTONE_SELECT,
                &record.trust_root_id,
                &record.revoked_receipt_sha256,
                "receipt",
            )
            .await?;
            ensure_tombstones_preserved(
                &mut transaction,
                NONCE_TOMBSTONE_SELECT,
                &record.trust_root_id,
                &record.revoked_nonces,
                "nonce",
            )
            .await?;
            update_watermark(&mut transaction, current, &record).await?;
        } else {
            // A verified complete history always contains at least the
            // revision-one revocation state, and its last point is the exact
            // independently checkpointed head. Requiring both here prevents
            // an empty or truncated fresh-store bootstrap.
            if record.history.is_empty() || !record.last_history_point_is_head() {
                return Err(PromotionReplayStoreError::TrustRootHistoryMismatch);
            }
            insert_watermark(&mut transaction, &record).await?;
        }

        insert_tombstones(
            &mut transaction,
            TombstoneKind::Key,
            &record.trust_root_id,
            &record.revoked_key_ids,
            record.revocation_revision,
            &record.history_chain_sha256,
        )
        .await?;
        insert_tombstones(
            &mut transaction,
            TombstoneKind::Receipt,
            &record.trust_root_id,
            &record.revoked_receipt_sha256,
            record.revocation_revision,
            &record.history_chain_sha256,
        )
        .await?;
        insert_tombstones(
            &mut transaction,
            TombstoneKind::Nonce,
            &record.trust_root_id,
            &record.revoked_nonces,
            record.revocation_revision,
            &record.history_chain_sha256,
        )
        .await?;

        transaction.commit().await.map_err(storage_error)
    }

    async fn consume_record(
        &self,
        record: ConsumptionRecord,
    ) -> Result<(), PromotionReplayStoreError> {
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(storage_error)?;
        let current = load_watermark(&mut transaction, &record.trust_root_id)
            .await?
            .ok_or(PromotionReplayStoreError::TrustedConfigurationNotInitialized)?;

        // Error priority is part of the replay-store contract. In particular,
        // a stale caller cannot probe the receipt or nonce sets until its exact
        // trust/time capability is current.
        if record.observed_at_unix_seconds < current.max_observed_time_unix_seconds {
            return Err(PromotionReplayStoreError::ClockRollback {
                highest_seen: current.max_observed_time_unix_seconds,
                found: record.observed_at_unix_seconds,
            });
        }
        if record.observed_at_unix_seconds != current.max_observed_time_unix_seconds
            || record.checkpoint_sha256 != current.checkpoint_sha256
            || record.trust_root_revision != current.trust_root_revision
            || record.trust_root_sha256 != current.trust_root_sha256
            || record.revocation_revision != current.revocation_revision
            || record.revocations_sha256 != current.revocations_sha256
            || record.history_chain_sha256 != current.history_chain_sha256
        {
            return Err(PromotionReplayStoreError::TrustedConfigurationNotCurrent);
        }

        let receipt_exists = sqlx::query_scalar::<_, i64>(
            "SELECT 1
             FROM promotion_receipt_consumptions
             WHERE trust_root_id = ? AND receipt_sha256 = ?",
        )
        .bind(&record.trust_root_id)
        .bind(&record.receipt_sha256)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(storage_error)?
        .is_some();
        if receipt_exists {
            return Err(PromotionReplayStoreError::ReceiptReplay);
        }

        let nonce_exists = sqlx::query_scalar::<_, i64>(
            "SELECT 1
             FROM promotion_receipt_consumptions
             WHERE trust_root_id = ? AND nonce = ?",
        )
        .bind(&record.trust_root_id)
        .bind(&record.nonce)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(storage_error)?
        .is_some();
        if nonce_exists {
            return Err(PromotionReplayStoreError::NonceReplay);
        }

        let insert = sqlx::query(
            "INSERT INTO promotion_receipt_consumptions (
                trust_root_id, schema_version, checkpoint_sha256,
                trust_root_revision, trust_root_sha256,
                revocation_revision, revocations_sha256, history_chain_sha256,
                observed_at_unix_seconds, nonce, receipt_sha256,
                expires_at_unix_seconds
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&record.trust_root_id)
        .bind(PROMOTION_REPLAY_SCHEMA_VERSION)
        .bind(&record.checkpoint_sha256)
        .bind(sqlite_integer(
            record.trust_root_revision,
            "trust-root revision",
        )?)
        .bind(&record.trust_root_sha256)
        .bind(sqlite_integer(
            record.revocation_revision,
            "revocation revision",
        )?)
        .bind(&record.revocations_sha256)
        .bind(&record.history_chain_sha256)
        .bind(sqlite_integer(
            record.observed_at_unix_seconds,
            "consume observation time",
        )?)
        .bind(&record.nonce)
        .bind(&record.receipt_sha256)
        .bind(sqlite_integer(
            record.expires_at_unix_seconds,
            "receipt expiry",
        )?)
        .execute(&mut *transaction)
        .await
        .map_err(storage_error)?;

        if insert.rows_affected() != 1 {
            return Err(PromotionReplayStoreError::Storage(
                "promotion receipt consumption insert did not persist exactly one row".to_string(),
            ));
        }
        verify_consumption_readback(&mut transaction, &record).await?;

        transaction.commit().await.map_err(storage_error)
    }
}

impl PromotionReceiptReplayStore for SqlitePromotionReceiptReplayStore {
    fn ratchet_trusted_config<'a>(
        &'a self,
        ratchet: PromotionTrustedConfigRatchet<'a>,
    ) -> PromotionReplayFuture<'a> {
        let record = RatchetRecord::capture(ratchet);
        Box::pin(async move { self.ratchet_record(record).await })
    }

    fn check_and_consume<'a>(
        &'a self,
        consumption: PromotionReplayConsumption<'a>,
    ) -> PromotionReplayFuture<'a> {
        let record = ConsumptionRecord::capture(consumption);
        Box::pin(async move { self.consume_record(record).await })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HistoryPoint {
    trust_root_revision: u64,
    trust_root_sha256: String,
    revocation_revision: u64,
    revocations_sha256: String,
    history_chain_sha256: String,
}

#[derive(Clone, Debug)]
struct RatchetRecord {
    checkpoint_source_json_sha256: String,
    checkpoint_sha256: String,
    checkpoint_trust_root_id: String,
    genesis_trust_root_sha256: String,
    checkpoint_terminal_history_chain_sha256: String,
    checkpoint_terminal_trust_root_revision: u64,
    checkpoint_terminal_trust_root_sha256: String,
    checkpoint_terminal_revocation_revision: u64,
    checkpoint_terminal_revocations_sha256: String,
    history: Vec<HistoryPoint>,
    trust_root_id: String,
    trust_root_revision: u64,
    trust_root_sha256: String,
    revocation_trust_root_id: String,
    revocation_revision: u64,
    revocations_sha256: String,
    history_chain_sha256: String,
    revoked_key_ids: Vec<String>,
    revoked_receipt_sha256: Vec<String>,
    revoked_nonces: Vec<String>,
    observed_at_unix_seconds: u64,
}

impl RatchetRecord {
    fn capture(ratchet: PromotionTrustedConfigRatchet<'_>) -> Self {
        let checkpoint = ratchet.checkpoint();
        let trust_root = ratchet.trust_root();
        let revocations = ratchet.revocations();
        Self {
            checkpoint_source_json_sha256: checkpoint.source_json_sha256().as_str().to_owned(),
            checkpoint_sha256: checkpoint.checkpoint_sha256().as_str().to_owned(),
            checkpoint_trust_root_id: checkpoint.trust_root_id().to_owned(),
            genesis_trust_root_sha256: ratchet.genesis_trust_root_sha256().as_str().to_owned(),
            checkpoint_terminal_history_chain_sha256: checkpoint
                .terminal_history_chain_sha256()
                .as_str()
                .to_owned(),
            checkpoint_terminal_trust_root_revision: checkpoint.terminal_trust_root_revision(),
            checkpoint_terminal_trust_root_sha256: checkpoint
                .terminal_trust_root_sha256()
                .as_str()
                .to_owned(),
            checkpoint_terminal_revocation_revision: checkpoint.terminal_revocation_revision(),
            checkpoint_terminal_revocations_sha256: checkpoint
                .terminal_revocations_sha256()
                .as_str()
                .to_owned(),
            history: ratchet
                .history()
                .iter()
                .map(|entry| HistoryPoint {
                    trust_root_revision: entry.trust_root_revision(),
                    trust_root_sha256: entry.trust_root_sha256().as_str().to_owned(),
                    revocation_revision: entry.revocation_revision(),
                    revocations_sha256: entry.revocations_sha256().as_str().to_owned(),
                    history_chain_sha256: entry.history_chain_sha256().as_str().to_owned(),
                })
                .collect(),
            trust_root_id: trust_root.trust_root_id.clone(),
            trust_root_revision: trust_root.revision,
            trust_root_sha256: ratchet.trust_root_sha256().as_str().to_owned(),
            revocation_trust_root_id: revocations.trust_root_id.clone(),
            revocation_revision: revocations.revision,
            revocations_sha256: ratchet.revocations_sha256().as_str().to_owned(),
            history_chain_sha256: ratchet.history_chain_sha256().as_str().to_owned(),
            revoked_key_ids: revocations.revoked_key_ids.clone(),
            revoked_receipt_sha256: revocations
                .revoked_receipt_sha256
                .iter()
                .map(|digest| digest.as_str().to_owned())
                .collect(),
            revoked_nonces: revocations.revoked_nonces.clone(),
            observed_at_unix_seconds: ratchet.observed_at_unix_seconds(),
        }
    }

    fn validate_internal_consistency(&self) -> Result<(), PromotionReplayStoreError> {
        if self.history.len() > MAX_PROMOTION_TRUST_HISTORY_UPDATES
            || self.revoked_key_ids.len() > MAX_PROMOTION_REVOCATIONS_PER_KIND
            || self.revoked_receipt_sha256.len() > MAX_PROMOTION_REVOCATIONS_PER_KIND
            || self.revoked_nonces.len() > MAX_PROMOTION_REVOCATIONS_PER_KIND
        {
            return Err(PromotionReplayStoreError::Storage(
                "verified promotion replay request exceeded its contract bounds".to_string(),
            ));
        }
        if self.checkpoint_trust_root_id != self.trust_root_id
            || self.revocation_trust_root_id != self.trust_root_id
            || self.checkpoint_terminal_history_chain_sha256 != self.history_chain_sha256
            || self.checkpoint_terminal_trust_root_revision != self.trust_root_revision
            || self.checkpoint_terminal_trust_root_sha256 != self.trust_root_sha256
            || self.checkpoint_terminal_revocation_revision != self.revocation_revision
            || self.checkpoint_terminal_revocations_sha256 != self.revocations_sha256
            || !self.last_history_point_is_head()
        {
            return Err(PromotionReplayStoreError::TrustRootHistoryMismatch);
        }
        Ok(())
    }

    fn last_history_point_is_head(&self) -> bool {
        self.history.last().is_some_and(|point| {
            point.trust_root_revision == self.trust_root_revision
                && point.trust_root_sha256 == self.trust_root_sha256
                && point.revocation_revision == self.revocation_revision
                && point.revocations_sha256 == self.revocations_sha256
                && point.history_chain_sha256 == self.history_chain_sha256
        })
    }
}

#[derive(Clone, Debug)]
struct ConsumptionRecord {
    trust_root_id: String,
    checkpoint_sha256: String,
    trust_root_revision: u64,
    trust_root_sha256: String,
    revocation_revision: u64,
    revocations_sha256: String,
    history_chain_sha256: String,
    observed_at_unix_seconds: u64,
    nonce: String,
    receipt_sha256: String,
    expires_at_unix_seconds: u64,
}

impl ConsumptionRecord {
    fn capture(consumption: PromotionReplayConsumption<'_>) -> Self {
        Self {
            trust_root_id: consumption.trust_root_id().to_owned(),
            checkpoint_sha256: consumption.checkpoint_sha256().as_str().to_owned(),
            trust_root_revision: consumption.trust_root_revision(),
            trust_root_sha256: consumption.trust_root_sha256().as_str().to_owned(),
            revocation_revision: consumption.revocation_revision(),
            revocations_sha256: consumption.revocations_sha256().as_str().to_owned(),
            history_chain_sha256: consumption.history_chain_sha256().as_str().to_owned(),
            observed_at_unix_seconds: consumption.observed_at_unix_seconds(),
            nonce: consumption.nonce().to_owned(),
            receipt_sha256: consumption.receipt_sha256().as_str().to_owned(),
            expires_at_unix_seconds: consumption.expires_at_unix_seconds(),
        }
    }
}

#[derive(Debug)]
struct StoredWatermark {
    schema_version: i64,
    checkpoint_source_json_sha256: String,
    checkpoint_sha256: String,
    genesis_trust_root_sha256: String,
    trust_root_revision: u64,
    trust_root_sha256: String,
    revocation_revision: u64,
    revocations_sha256: String,
    history_chain_sha256: String,
    max_observed_time_unix_seconds: u64,
}

async fn load_watermark(
    transaction: &mut Transaction<'_, Sqlite>,
    trust_root_id: &str,
) -> Result<Option<StoredWatermark>, PromotionReplayStoreError> {
    sqlx::query(WATERMARK_SELECT)
        .bind(trust_root_id)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(storage_error)?
        .map(decode_watermark)
        .transpose()
}

fn decode_watermark(row: SqliteRow) -> Result<StoredWatermark, PromotionReplayStoreError> {
    Ok(StoredWatermark {
        schema_version: row.try_get("schema_version").map_err(storage_error)?,
        checkpoint_source_json_sha256: row
            .try_get("checkpoint_source_json_sha256")
            .map_err(storage_error)?,
        checkpoint_sha256: row.try_get("checkpoint_sha256").map_err(storage_error)?,
        genesis_trust_root_sha256: row
            .try_get("genesis_trust_root_sha256")
            .map_err(storage_error)?,
        trust_root_revision: unsigned_sqlite_integer(
            row.try_get("trust_root_revision").map_err(storage_error)?,
            "stored trust-root revision",
        )?,
        trust_root_sha256: row.try_get("trust_root_sha256").map_err(storage_error)?,
        revocation_revision: unsigned_sqlite_integer(
            row.try_get("revocation_revision").map_err(storage_error)?,
            "stored revocation revision",
        )?,
        revocations_sha256: row.try_get("revocations_sha256").map_err(storage_error)?,
        history_chain_sha256: row.try_get("history_chain_sha256").map_err(storage_error)?,
        max_observed_time_unix_seconds: unsigned_sqlite_integer(
            row.try_get("max_observed_time_unix_seconds")
                .map_err(storage_error)?,
            "stored trusted-clock watermark",
        )?,
    })
}

fn validate_existing_ratchet(
    current: &StoredWatermark,
    record: &RatchetRecord,
    max_forward_jump_seconds: NonZeroU64,
) -> Result<(), PromotionReplayStoreError> {
    if record.observed_at_unix_seconds < current.max_observed_time_unix_seconds {
        return Err(PromotionReplayStoreError::ClockRollback {
            highest_seen: current.max_observed_time_unix_seconds,
            found: record.observed_at_unix_seconds,
        });
    }
    if record
        .observed_at_unix_seconds
        .saturating_sub(current.max_observed_time_unix_seconds)
        > max_forward_jump_seconds.get()
    {
        return Err(PromotionReplayStoreError::Storage(format!(
            "trusted system clock advanced by more than the configured {} second limit; authenticated store recovery is required",
            max_forward_jump_seconds.get()
        )));
    }
    if record.genesis_trust_root_sha256 != current.genesis_trust_root_sha256 {
        return Err(PromotionReplayStoreError::GenesisAnchorConflict);
    }
    validate_revision(
        current.trust_root_revision,
        &current.trust_root_sha256,
        record.trust_root_revision,
        &record.trust_root_sha256,
        true,
    )?;
    validate_revision(
        current.revocation_revision,
        &current.revocations_sha256,
        record.revocation_revision,
        &record.revocations_sha256,
        false,
    )?;

    let exact_ancestor = record.history.iter().any(|point| {
        point.trust_root_revision == current.trust_root_revision
            && point.trust_root_sha256 == current.trust_root_sha256
            && point.revocation_revision == current.revocation_revision
            && point.revocations_sha256 == current.revocations_sha256
            && point.history_chain_sha256 == current.history_chain_sha256
    });
    if !exact_ancestor {
        return Err(PromotionReplayStoreError::TrustRootHistoryMismatch);
    }
    Ok(())
}

fn validate_revision(
    highest_revision: u64,
    highest_digest: &str,
    found_revision: u64,
    found_digest: &str,
    trust_root: bool,
) -> Result<(), PromotionReplayStoreError> {
    if found_revision < highest_revision {
        return if trust_root {
            Err(PromotionReplayStoreError::TrustRootRevisionRollback {
                highest_seen: highest_revision,
                found: found_revision,
            })
        } else {
            Err(PromotionReplayStoreError::RevocationRevisionRollback {
                highest_seen: highest_revision,
                found: found_revision,
            })
        };
    }
    if found_revision == highest_revision && found_digest != highest_digest {
        return if trust_root {
            Err(PromotionReplayStoreError::TrustRootRevisionConflict {
                revision: found_revision,
            })
        } else {
            Err(PromotionReplayStoreError::RevocationRevisionConflict {
                revision: found_revision,
            })
        };
    }
    Ok(())
}

async fn insert_watermark(
    transaction: &mut Transaction<'_, Sqlite>,
    record: &RatchetRecord,
) -> Result<(), PromotionReplayStoreError> {
    let insert = sqlx::query(
        "INSERT INTO promotion_trust_watermarks (
            trust_root_id, schema_version, checkpoint_source_json_sha256,
            checkpoint_sha256, genesis_trust_root_sha256,
            trust_root_revision, trust_root_sha256, revocation_revision,
            revocations_sha256, history_chain_sha256,
            max_observed_time_unix_seconds
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&record.trust_root_id)
    .bind(PROMOTION_REPLAY_SCHEMA_VERSION)
    .bind(&record.checkpoint_source_json_sha256)
    .bind(&record.checkpoint_sha256)
    .bind(&record.genesis_trust_root_sha256)
    .bind(sqlite_integer(
        record.trust_root_revision,
        "trust-root revision",
    )?)
    .bind(&record.trust_root_sha256)
    .bind(sqlite_integer(
        record.revocation_revision,
        "revocation revision",
    )?)
    .bind(&record.revocations_sha256)
    .bind(&record.history_chain_sha256)
    .bind(sqlite_integer(
        record.observed_at_unix_seconds,
        "trusted-clock watermark",
    )?)
    .execute(&mut **transaction)
    .await
    .map_err(storage_error)?;
    if insert.rows_affected() != 1 {
        return Err(PromotionReplayStoreError::Storage(
            "fresh promotion trust watermark insert did not persist exactly one row".to_string(),
        ));
    }
    verify_watermark_readback(transaction, record).await?;
    Ok(())
}

async fn update_watermark(
    transaction: &mut Transaction<'_, Sqlite>,
    current: &StoredWatermark,
    record: &RatchetRecord,
) -> Result<(), PromotionReplayStoreError> {
    let result = sqlx::query(
        "UPDATE promotion_trust_watermarks
         SET checkpoint_source_json_sha256 = ?, checkpoint_sha256 = ?,
             trust_root_revision = ?, trust_root_sha256 = ?,
             revocation_revision = ?, revocations_sha256 = ?,
             history_chain_sha256 = ?, max_observed_time_unix_seconds = ?
         WHERE trust_root_id = ?
           AND schema_version = ?
           AND checkpoint_source_json_sha256 = ?
           AND checkpoint_sha256 = ?
           AND genesis_trust_root_sha256 = ?
           AND trust_root_revision = ?
           AND trust_root_sha256 = ?
           AND revocation_revision = ?
           AND revocations_sha256 = ?
           AND history_chain_sha256 = ?
           AND max_observed_time_unix_seconds = ?",
    )
    .bind(&record.checkpoint_source_json_sha256)
    .bind(&record.checkpoint_sha256)
    .bind(sqlite_integer(
        record.trust_root_revision,
        "trust-root revision",
    )?)
    .bind(&record.trust_root_sha256)
    .bind(sqlite_integer(
        record.revocation_revision,
        "revocation revision",
    )?)
    .bind(&record.revocations_sha256)
    .bind(&record.history_chain_sha256)
    .bind(sqlite_integer(
        record.observed_at_unix_seconds,
        "trusted-clock watermark",
    )?)
    .bind(&record.trust_root_id)
    .bind(PROMOTION_REPLAY_SCHEMA_VERSION)
    .bind(&current.checkpoint_source_json_sha256)
    .bind(&current.checkpoint_sha256)
    .bind(&current.genesis_trust_root_sha256)
    .bind(sqlite_integer(
        current.trust_root_revision,
        "current trust-root revision",
    )?)
    .bind(&current.trust_root_sha256)
    .bind(sqlite_integer(
        current.revocation_revision,
        "current revocation revision",
    )?)
    .bind(&current.revocations_sha256)
    .bind(&current.history_chain_sha256)
    .bind(sqlite_integer(
        current.max_observed_time_unix_seconds,
        "current trusted-clock watermark",
    )?)
    .execute(&mut **transaction)
    .await
    .map_err(storage_error)?;
    if result.rows_affected() != 1 {
        return Err(PromotionReplayStoreError::Storage(
            "promotion trust watermark disappeared during ratchet".to_string(),
        ));
    }
    verify_watermark_readback(transaction, record).await?;
    Ok(())
}

async fn ensure_tombstones_preserved(
    transaction: &mut Transaction<'_, Sqlite>,
    select_sql: &'static str,
    trust_root_id: &str,
    successor: &[String],
    kind: &'static str,
) -> Result<(), PromotionReplayStoreError> {
    let mut rows = sqlx::query_scalar::<_, String>(select_sql)
        .bind(trust_root_id)
        .fetch(&mut **transaction);
    let mut count = 0usize;
    while let Some(value) = rows.try_next().await.map_err(storage_error)? {
        count = count.checked_add(1).ok_or_else(|| {
            PromotionReplayStoreError::Storage(
                "promotion tombstone stream count overflowed".to_string(),
            )
        })?;
        if count > MAX_PROMOTION_REVOCATIONS_PER_KIND {
            return Err(PromotionReplayStoreError::Storage(format!(
                "stored promotion {kind} tombstones exceed the contract bound"
            )));
        }
        if successor.binary_search(&value).is_err() {
            return Err(PromotionReplayStoreError::RevocationTombstoneRemoved(kind));
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum TombstoneKind {
    Key,
    Receipt,
    Nonce,
}

async fn insert_tombstones(
    transaction: &mut Transaction<'_, Sqlite>,
    kind: TombstoneKind,
    trust_root_id: &str,
    values: &[String],
    revocation_revision: u64,
    history_chain_sha256: &str,
) -> Result<(), PromotionReplayStoreError> {
    let insert_sql = match kind {
        TombstoneKind::Key => {
            "INSERT INTO promotion_revoked_key_tombstones (
                trust_root_id, revoked_key_id,
                durably_observed_revocation_revision,
                durably_observed_history_chain_sha256
             ) VALUES (?, ?, ?, ?) ON CONFLICT DO NOTHING"
        }
        TombstoneKind::Receipt => {
            "INSERT INTO promotion_revoked_receipt_tombstones (
                trust_root_id, revoked_receipt_sha256,
                durably_observed_revocation_revision,
                durably_observed_history_chain_sha256
             ) VALUES (?, ?, ?, ?) ON CONFLICT DO NOTHING"
        }
        TombstoneKind::Nonce => {
            "INSERT INTO promotion_revoked_nonce_tombstones (
                trust_root_id, revoked_nonce,
                durably_observed_revocation_revision,
                durably_observed_history_chain_sha256
             ) VALUES (?, ?, ?, ?) ON CONFLICT DO NOTHING"
        }
    };
    for value in values {
        let insert = sqlx::query(insert_sql)
            .bind(trust_root_id)
            .bind(value)
            .bind(sqlite_integer(
                revocation_revision,
                "tombstone observation revocation revision",
            )?)
            .bind(history_chain_sha256)
            .execute(&mut **transaction)
            .await
            .map_err(storage_error)?;
        if insert.rows_affected() > 1 {
            return Err(PromotionReplayStoreError::Storage(
                "promotion tombstone insert affected more than one row".to_string(),
            ));
        }
        verify_tombstone_readback(
            transaction,
            kind,
            trust_root_id,
            value,
            revocation_revision,
            history_chain_sha256,
            insert.rows_affected(),
        )
        .await?;
    }
    Ok(())
}

async fn verify_watermark_readback(
    transaction: &mut Transaction<'_, Sqlite>,
    record: &RatchetRecord,
) -> Result<(), PromotionReplayStoreError> {
    let stored = load_watermark(transaction, &record.trust_root_id)
        .await?
        .ok_or_else(|| {
            PromotionReplayStoreError::Storage(
                "promotion trust watermark disappeared after write".to_string(),
            )
        })?;
    if stored.schema_version != PROMOTION_REPLAY_SCHEMA_VERSION
        || stored.checkpoint_source_json_sha256 != record.checkpoint_source_json_sha256
        || stored.checkpoint_sha256 != record.checkpoint_sha256
        || stored.genesis_trust_root_sha256 != record.genesis_trust_root_sha256
        || stored.trust_root_revision != record.trust_root_revision
        || stored.trust_root_sha256 != record.trust_root_sha256
        || stored.revocation_revision != record.revocation_revision
        || stored.revocations_sha256 != record.revocations_sha256
        || stored.history_chain_sha256 != record.history_chain_sha256
        || stored.max_observed_time_unix_seconds != record.observed_at_unix_seconds
    {
        return Err(PromotionReplayStoreError::Storage(
            "promotion trust watermark exact readback differed after write".to_string(),
        ));
    }
    Ok(())
}

async fn verify_consumption_readback(
    transaction: &mut Transaction<'_, Sqlite>,
    record: &ConsumptionRecord,
) -> Result<(), PromotionReplayStoreError> {
    let row = sqlx::query(
        "SELECT schema_version, checkpoint_sha256, trust_root_revision,
                trust_root_sha256, revocation_revision, revocations_sha256,
                history_chain_sha256, observed_at_unix_seconds, nonce,
                expires_at_unix_seconds
         FROM promotion_receipt_consumptions
         WHERE trust_root_id = ? AND receipt_sha256 = ?",
    )
    .bind(&record.trust_root_id)
    .bind(&record.receipt_sha256)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(storage_error)?
    .ok_or_else(|| {
        PromotionReplayStoreError::Storage(
            "promotion receipt consumption disappeared after insert".to_string(),
        )
    })?;
    let schema_version: i64 = row.try_get("schema_version").map_err(storage_error)?;
    let checkpoint_sha256: String = row.try_get("checkpoint_sha256").map_err(storage_error)?;
    let trust_root_revision = unsigned_sqlite_integer(
        row.try_get("trust_root_revision").map_err(storage_error)?,
        "consumption readback trust-root revision",
    )?;
    let trust_root_sha256: String = row.try_get("trust_root_sha256").map_err(storage_error)?;
    let revocation_revision = unsigned_sqlite_integer(
        row.try_get("revocation_revision").map_err(storage_error)?,
        "consumption readback revocation revision",
    )?;
    let revocations_sha256: String = row.try_get("revocations_sha256").map_err(storage_error)?;
    let history_chain_sha256: String =
        row.try_get("history_chain_sha256").map_err(storage_error)?;
    let observed_at_unix_seconds = unsigned_sqlite_integer(
        row.try_get("observed_at_unix_seconds")
            .map_err(storage_error)?,
        "consumption readback observation time",
    )?;
    let nonce: String = row.try_get("nonce").map_err(storage_error)?;
    let expires_at_unix_seconds = unsigned_sqlite_integer(
        row.try_get("expires_at_unix_seconds")
            .map_err(storage_error)?,
        "consumption readback expiry",
    )?;
    if schema_version != PROMOTION_REPLAY_SCHEMA_VERSION
        || checkpoint_sha256 != record.checkpoint_sha256
        || trust_root_revision != record.trust_root_revision
        || trust_root_sha256 != record.trust_root_sha256
        || revocation_revision != record.revocation_revision
        || revocations_sha256 != record.revocations_sha256
        || history_chain_sha256 != record.history_chain_sha256
        || observed_at_unix_seconds != record.observed_at_unix_seconds
        || nonce != record.nonce
        || expires_at_unix_seconds != record.expires_at_unix_seconds
    {
        return Err(PromotionReplayStoreError::Storage(
            "promotion receipt consumption exact readback differed after insert".to_string(),
        ));
    }
    Ok(())
}

async fn verify_tombstone_readback(
    transaction: &mut Transaction<'_, Sqlite>,
    kind: TombstoneKind,
    trust_root_id: &str,
    value: &str,
    current_revocation_revision: u64,
    current_history_chain_sha256: &str,
    rows_affected: u64,
) -> Result<(), PromotionReplayStoreError> {
    let select_sql = match kind {
        TombstoneKind::Key => {
            "SELECT durably_observed_revocation_revision,
                    durably_observed_history_chain_sha256
             FROM promotion_revoked_key_tombstones
             WHERE trust_root_id = ? AND revoked_key_id = ?"
        }
        TombstoneKind::Receipt => {
            "SELECT durably_observed_revocation_revision,
                    durably_observed_history_chain_sha256
             FROM promotion_revoked_receipt_tombstones
             WHERE trust_root_id = ? AND revoked_receipt_sha256 = ?"
        }
        TombstoneKind::Nonce => {
            "SELECT durably_observed_revocation_revision,
                    durably_observed_history_chain_sha256
             FROM promotion_revoked_nonce_tombstones
             WHERE trust_root_id = ? AND revoked_nonce = ?"
        }
    };
    let row = sqlx::query(select_sql)
        .bind(trust_root_id)
        .bind(value)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(storage_error)?
        .ok_or_else(|| {
            PromotionReplayStoreError::Storage(
                "promotion tombstone insert reported success/conflict without a durable row"
                    .to_string(),
            )
        })?;
    let observed_revision = unsigned_sqlite_integer(
        row.try_get("durably_observed_revocation_revision")
            .map_err(storage_error)?,
        "tombstone readback revocation revision",
    )?;
    let observed_chain: String = row
        .try_get("durably_observed_history_chain_sha256")
        .map_err(storage_error)?;
    if observed_revision == 0
        || observed_revision > current_revocation_revision
        || Sha256Digest::parse(observed_chain.clone()).is_err()
        || (rows_affected == 1
            && (observed_revision != current_revocation_revision
                || observed_chain != current_history_chain_sha256))
    {
        return Err(PromotionReplayStoreError::Storage(
            "promotion tombstone durable provenance failed exact readback validation".to_string(),
        ));
    }
    Ok(())
}

fn sqlite_integer(value: u64, field: &str) -> Result<i64, PromotionReplayStoreError> {
    i64::try_from(value).map_err(|_| {
        PromotionReplayStoreError::Storage(format!(
            "verified {field} exceeds SQLite's signed integer range"
        ))
    })
}

fn unsigned_sqlite_integer(value: i64, field: &str) -> Result<u64, PromotionReplayStoreError> {
    u64::try_from(value).map_err(|_| {
        PromotionReplayStoreError::Storage(format!("{field} is negative in durable storage"))
    })
}

fn storage_error(error: sqlx::Error) -> PromotionReplayStoreError {
    PromotionReplayStoreError::Storage(format!(
        "SQLite promotion replay store operation failed: {error}"
    ))
}

pub(crate) async fn verify_promotion_replay_integrity(
    pool: &SqlitePool,
) -> Result<(), EvidenceError> {
    let mut transaction = pool.begin().await.map_err(classify_sqlx_error)?;
    let mut last_trust_root_id = String::new();
    loop {
        let rows = sqlx::query(
            "SELECT trust_root_id, schema_version,
                    checkpoint_source_json_sha256, checkpoint_sha256,
                    genesis_trust_root_sha256,
                    trust_root_revision, trust_root_sha256,
                    revocation_revision, revocations_sha256,
                    history_chain_sha256, max_observed_time_unix_seconds
             FROM promotion_trust_watermarks
             WHERE trust_root_id > ?
             ORDER BY trust_root_id ASC
             LIMIT ?",
        )
        .bind(&last_trust_root_id)
        .bind(WATERMARK_INTEGRITY_PAGE_SIZE)
        .fetch_all(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        if rows.is_empty() {
            break;
        }
        for row in rows {
            let trust_root_id: String = row.try_get("trust_root_id").map_err(|error| {
                EvidenceError::Corrupt(format!(
                    "promotion trust watermark has an invalid trust-root id column: {error}"
                ))
            })?;
            validate_portable_identifier("promotion trust-root id", &trust_root_id)?;
            let watermark = decode_watermark(row).map_err(replay_error_as_corrupt)?;
            validate_watermark_digests(&watermark)?;
            verify_tombstone_integrity(
                &mut transaction,
                KEY_TOMBSTONE_INTEGRITY_SELECT,
                &trust_root_id,
                &watermark,
                TombstoneValueShape::Identifier,
                "key-id",
            )
            .await?;
            verify_tombstone_integrity(
                &mut transaction,
                RECEIPT_TOMBSTONE_INTEGRITY_SELECT,
                &trust_root_id,
                &watermark,
                TombstoneValueShape::Digest,
                "receipt",
            )
            .await?;
            verify_tombstone_integrity(
                &mut transaction,
                NONCE_TOMBSTONE_INTEGRITY_SELECT,
                &trust_root_id,
                &watermark,
                TombstoneValueShape::Digest,
                "nonce",
            )
            .await?;
            verify_consumption_integrity(&mut transaction, &trust_root_id, &watermark).await?;
            last_trust_root_id = trust_root_id;
        }
    }
    transaction.commit().await.map_err(classify_sqlx_error)
}

const KEY_TOMBSTONE_INTEGRITY_SELECT: &str = "SELECT revoked_key_id AS tombstone_value,
            durably_observed_revocation_revision,
            durably_observed_history_chain_sha256
     FROM promotion_revoked_key_tombstones
     WHERE trust_root_id = ?
     ORDER BY revoked_key_id";

const RECEIPT_TOMBSTONE_INTEGRITY_SELECT: &str = "SELECT revoked_receipt_sha256 AS tombstone_value,
            durably_observed_revocation_revision,
            durably_observed_history_chain_sha256
     FROM promotion_revoked_receipt_tombstones
     WHERE trust_root_id = ?
     ORDER BY revoked_receipt_sha256";

const NONCE_TOMBSTONE_INTEGRITY_SELECT: &str = "SELECT revoked_nonce AS tombstone_value,
            durably_observed_revocation_revision,
            durably_observed_history_chain_sha256
     FROM promotion_revoked_nonce_tombstones
     WHERE trust_root_id = ?
     ORDER BY revoked_nonce";

#[derive(Clone, Copy)]
enum TombstoneValueShape {
    Identifier,
    Digest,
}

async fn verify_tombstone_integrity(
    transaction: &mut Transaction<'_, Sqlite>,
    select_sql: &'static str,
    trust_root_id: &str,
    watermark: &StoredWatermark,
    value_shape: TombstoneValueShape,
    kind: &'static str,
) -> Result<(), EvidenceError> {
    let mut rows = sqlx::query(select_sql)
        .bind(trust_root_id)
        .fetch(&mut **transaction);
    let mut count = 0usize;
    while let Some(row) = rows.try_next().await.map_err(classify_sqlx_error)? {
        count = count.checked_add(1).ok_or_else(|| {
            EvidenceError::Corrupt(format!(
                "promotion {kind} tombstone count overflowed during integrity verification"
            ))
        })?;
        if count > MAX_PROMOTION_REVOCATIONS_PER_KIND {
            return Err(EvidenceError::Corrupt(format!(
                "promotion {kind} tombstones exceed the contract bound"
            )));
        }
        let value: String = row.try_get("tombstone_value").map_err(|error| {
            EvidenceError::Corrupt(format!(
                "promotion {kind} tombstone has an invalid value column: {error}"
            ))
        })?;
        match value_shape {
            TombstoneValueShape::Identifier => {
                validate_portable_identifier("promotion revoked key id", &value)?
            }
            TombstoneValueShape::Digest => {
                validate_digest_text("promotion replay tombstone", &value)?
            }
        }
        let observed_revision = integrity_u64(
            row.try_get("durably_observed_revocation_revision")
                .map_err(|error| {
                    EvidenceError::Corrupt(format!(
                        "promotion {kind} tombstone has an invalid revision column: {error}"
                    ))
                })?,
            "promotion tombstone observation revision",
        )?;
        if observed_revision == 0 || observed_revision > watermark.revocation_revision {
            return Err(EvidenceError::Corrupt(format!(
                "promotion {kind} tombstone provenance is later than its durable watermark"
            )));
        }
        let observed_chain: String = row
            .try_get("durably_observed_history_chain_sha256")
            .map_err(|error| {
                EvidenceError::Corrupt(format!(
                    "promotion {kind} tombstone has an invalid chain column: {error}"
                ))
            })?;
        validate_digest_text("promotion tombstone provenance chain", &observed_chain)?;
    }
    Ok(())
}

async fn verify_consumption_integrity(
    transaction: &mut Transaction<'_, Sqlite>,
    trust_root_id: &str,
    watermark: &StoredWatermark,
) -> Result<(), EvidenceError> {
    let mut rows = sqlx::query(
        "SELECT schema_version, checkpoint_sha256, trust_root_revision,
                trust_root_sha256, revocation_revision, revocations_sha256,
                history_chain_sha256, observed_at_unix_seconds, nonce,
                receipt_sha256, expires_at_unix_seconds
         FROM promotion_receipt_consumptions
         WHERE trust_root_id = ?
         ORDER BY receipt_sha256",
    )
    .bind(trust_root_id)
    .fetch(&mut **transaction);
    while let Some(row) = rows.try_next().await.map_err(classify_sqlx_error)? {
        let schema_version: i64 = row.try_get("schema_version").map_err(|error| {
            EvidenceError::Corrupt(format!(
                "promotion consumption has an invalid schema version: {error}"
            ))
        })?;
        if schema_version != PROMOTION_REPLAY_SCHEMA_VERSION {
            return Err(EvidenceError::Corrupt(
                "promotion consumption has an unsupported schema version".to_string(),
            ));
        }
        let checkpoint_sha256: String = row.try_get("checkpoint_sha256").map_err(|error| {
            EvidenceError::Corrupt(format!(
                "promotion consumption has an invalid checkpoint digest: {error}"
            ))
        })?;
        let trust_root_sha256: String = row.try_get("trust_root_sha256").map_err(|error| {
            EvidenceError::Corrupt(format!(
                "promotion consumption has an invalid trust-root digest: {error}"
            ))
        })?;
        let revocations_sha256: String = row.try_get("revocations_sha256").map_err(|error| {
            EvidenceError::Corrupt(format!(
                "promotion consumption has an invalid revocations digest: {error}"
            ))
        })?;
        let history_chain_sha256: String =
            row.try_get("history_chain_sha256").map_err(|error| {
                EvidenceError::Corrupt(format!(
                    "promotion consumption has an invalid history-chain digest: {error}"
                ))
            })?;
        let nonce: String = row.try_get("nonce").map_err(|error| {
            EvidenceError::Corrupt(format!(
                "promotion consumption has an invalid nonce: {error}"
            ))
        })?;
        let receipt_sha256: String = row.try_get("receipt_sha256").map_err(|error| {
            EvidenceError::Corrupt(format!(
                "promotion consumption has an invalid receipt digest: {error}"
            ))
        })?;
        for (label, value) in [
            ("promotion consumption checkpoint", &checkpoint_sha256),
            ("promotion consumption trust root", &trust_root_sha256),
            ("promotion consumption revocations", &revocations_sha256),
            ("promotion consumption history chain", &history_chain_sha256),
            ("promotion consumption nonce", &nonce),
            ("promotion consumption receipt", &receipt_sha256),
        ] {
            validate_digest_text(label, value)?;
        }
        let trust_root_revision = integrity_u64(
            row.try_get("trust_root_revision").map_err(|error| {
                EvidenceError::Corrupt(format!(
                    "promotion consumption has an invalid trust-root revision: {error}"
                ))
            })?,
            "promotion consumption trust-root revision",
        )?;
        let revocation_revision = integrity_u64(
            row.try_get("revocation_revision").map_err(|error| {
                EvidenceError::Corrupt(format!(
                    "promotion consumption has an invalid revocation revision: {error}"
                ))
            })?,
            "promotion consumption revocation revision",
        )?;
        let observed_at = integrity_u64(
            row.try_get("observed_at_unix_seconds").map_err(|error| {
                EvidenceError::Corrupt(format!(
                    "promotion consumption has an invalid observation time: {error}"
                ))
            })?,
            "promotion consumption observation time",
        )?;
        let expires_at = integrity_u64(
            row.try_get("expires_at_unix_seconds").map_err(|error| {
                EvidenceError::Corrupt(format!(
                    "promotion consumption has an invalid expiry: {error}"
                ))
            })?,
            "promotion consumption expiry",
        )?;
        if trust_root_revision == 0
            || trust_root_revision > watermark.trust_root_revision
            || revocation_revision == 0
            || revocation_revision > watermark.revocation_revision
            || observed_at == 0
            || observed_at > watermark.max_observed_time_unix_seconds
            || expires_at <= observed_at
        {
            return Err(EvidenceError::Corrupt(
                "promotion consumption is later than its durable watermark or has an invalid time window"
                    .to_string(),
            ));
        }
        if (trust_root_revision == watermark.trust_root_revision
            && trust_root_sha256 != watermark.trust_root_sha256)
            || (revocation_revision == watermark.revocation_revision
                && revocations_sha256 != watermark.revocations_sha256)
            || (trust_root_revision == watermark.trust_root_revision
                && revocation_revision == watermark.revocation_revision
                && history_chain_sha256 != watermark.history_chain_sha256)
        {
            return Err(EvidenceError::Corrupt(
                "promotion consumption conflicts with its durable revision watermark".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_watermark_digests(watermark: &StoredWatermark) -> Result<(), EvidenceError> {
    for (label, value) in [
        (
            "promotion checkpoint source JSON",
            &watermark.checkpoint_source_json_sha256,
        ),
        ("promotion checkpoint", &watermark.checkpoint_sha256),
        (
            "promotion genesis trust root",
            &watermark.genesis_trust_root_sha256,
        ),
        ("promotion current trust root", &watermark.trust_root_sha256),
        (
            "promotion current revocations",
            &watermark.revocations_sha256,
        ),
        (
            "promotion current history chain",
            &watermark.history_chain_sha256,
        ),
    ] {
        validate_digest_text(label, value)?;
    }
    if watermark.schema_version != PROMOTION_REPLAY_SCHEMA_VERSION
        || watermark.trust_root_revision == 0
        || watermark.revocation_revision == 0
        || watermark.max_observed_time_unix_seconds == 0
    {
        return Err(EvidenceError::Corrupt(
            "promotion trust watermark contains an invalid schema, revision, or time".to_string(),
        ));
    }
    Ok(())
}

fn validate_digest_text(label: &str, value: &str) -> Result<(), EvidenceError> {
    Sha256Digest::parse(value.to_string())
        .map(|_| ())
        .map_err(|error| EvidenceError::Corrupt(format!("{label}: {error}")))
}

fn validate_portable_identifier(label: &str, value: &str) -> Result<(), EvidenceError> {
    if value.is_empty()
        || value.len() > 128
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b'/')
        })
    {
        return Err(EvidenceError::Corrupt(format!(
            "{label} is not a canonical portable identifier"
        )));
    }
    Ok(())
}

fn integrity_u64(value: i64, label: &str) -> Result<u64, EvidenceError> {
    u64::try_from(value)
        .map_err(|_| EvidenceError::Corrupt(format!("{label} is negative in durable storage")))
}

fn replay_error_as_corrupt(error: PromotionReplayStoreError) -> EvidenceError {
    EvidenceError::Corrupt(format!(
        "promotion replay watermark could not be decoded: {error:?}"
    ))
}

#[cfg(test)]
#[path = "promotion_replay_store_tests.rs"]
mod tests;
