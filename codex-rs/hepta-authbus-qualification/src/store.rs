use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use codex_hepta_contracts::Sha256Digest;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use serde::Serialize;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::Transaction;
use sqlx::sqlite::SqliteRow;

use crate::AUTHBUS_P0_2_AUTHORITY;
use crate::AUTHBUS_P0_2_EFFECT_AUTHORITY;
use crate::AUTHBUS_P0_2_EXECUTE_ALLOWED;
use crate::AUTHBUS_P0_2_G5_ALLOWED;
use crate::AUTHBUS_P0_2_OPERATOR_ACCEPTANCE;
use crate::AUTHBUS_P0_2_PRODUCTION_CALLER;
use crate::AUTHBUS_P0_2_PRODUCTION_WRITER;
use crate::AUTHBUS_P0_2_PROMOTION;
use crate::AUTHBUS_P0_2_QUALIFICATION_ONLY;
use crate::AdmissionDisposition;
use crate::DispatchObservation;
use crate::DispatchTicket;
use crate::IntegrityReport;
use crate::LookupOutcome;
use crate::OperationSnapshot;
use crate::OperationState;
use crate::OutboxRecord;
use crate::QualificationAdmission;
use crate::QualificationError;
use crate::QualificationFailpoint;
use crate::QualificationFence;
use crate::QualificationQuota;
use crate::QualificationResult;
use crate::QuotaReservationState;
use crate::QuotaSnapshot;
use crate::RecoveryAction;
use crate::StatusObservation;
use crate::VerifiedNoEffectTerminal;
use crate::WriteDisposition;
use crate::WriterIdentity;
use crate::digest_length_delimited;
use crate::digest_serializable;
use crate::validate_identifier;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
const DATABASE_FILE: &str = "authbus-p0-2-qualification.sqlite3";

#[derive(Clone)]
pub struct QualificationStore {
    pool: SqlitePool,
    database_path: PathBuf,
    writer: WriterIdentity,
    failpoints: Arc<AtomicU64>,
}

impl QualificationStore {
    pub async fn open(
        root: impl AsRef<Path>,
        writer: WriterIdentity,
        now_ms: u64,
    ) -> QualificationResult<Self> {
        writer.validate()?;
        if now_ms == 0 {
            return Err(QualificationError::InvalidInput);
        }

        let root = root.as_ref();
        create_private_directory(root)?;
        let root = fs::canonicalize(root).map_err(|_| QualificationError::StorageUnavailable)?;
        let absolute_root = AbsolutePathBuf::try_from(root.clone())
            .map_err(|_| QualificationError::StorageUnavailable)?;
        let database_path = root.join(DATABASE_FILE);
        let pool = SqliteConfig::from_sqlite_home(absolute_root)
            .open_durable_evidence_pool(&database_path)
            .await
            .map_err(map_sqlx)?;
        MIGRATOR
            .run(&pool)
            .await
            .map_err(|_| QualificationError::StorageUnavailable)?;
        protect_private_file(&database_path)?;
        initialize_or_rebind_meta(&pool, &writer, now_ms).await?;

        let store = Self {
            pool,
            database_path,
            writer,
            failpoints: Arc::new(AtomicU64::new(0)),
        };
        store.verify_integrity().await?;
        Ok(store)
    }

    pub fn database_path(&self) -> &Path {
        &self.database_path
    }

    pub fn writer(&self) -> &WriterIdentity {
        &self.writer
    }

    pub async fn close(self) {
        self.pool.close().await;
    }

    pub async fn admit(
        &self,
        admission: QualificationAdmission,
    ) -> QualificationResult<(AdmissionDisposition, OperationSnapshot)> {
        admission.validate()?;
        let admission_sha256 = admission.intent_sha256()?;
        let intent_json =
            serde_json::to_string(&admission).map_err(|_| QualificationError::InvalidInput)?;
        let claim_sha256 = admission.intent.claim_key_sha256();
        let now_ms = admission.intent.created_at_ms;
        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;

        if let Some(existing) = load_operation_optional(
            &mut transaction,
            admission.intent.operation_id.as_str(),
        )
        .await?
        {
            verify_operation_row(&existing)?;
            if existing.intent_sha256 == admission_sha256.to_string() {
                let snapshot = existing.snapshot()?;
                return Ok((AdmissionDisposition::AlreadyPresent, snapshot));
            }
            return Err(QualificationError::Conflict);
        }

        if operation_binding_exists(
            &mut transaction,
            admission.intent.operation_key.as_str(),
            admission.intent.effect_key.as_str(),
            admission.intent.idempotency_key.as_str(),
        )
        .await?
        {
            return Err(QualificationError::Conflict);
        }

        let claim_key = claim_sha256.to_string();
        if let Some((existing_operation, active)) = load_claim(&mut transaction, &claim_key).await?
        {
            if active && existing_operation != admission.intent.operation_id {
                return Err(QualificationError::ActiveClaim);
            }
        }

        let row = OperationRow {
            operation_id: admission.intent.operation_id.clone(),
            operation_key: admission.intent.operation_key.clone(),
            effect_key: admission.intent.effect_key.clone(),
            idempotency_key: admission.intent.idempotency_key.clone(),
            operation_kind: admission.intent.kind.as_db().to_string(),
            provider_id: admission.intent.provider_id.clone(),
            profile_id: admission.intent.profile_id.clone(),
            token_family_id: admission.intent.token_family_id.clone(),
            intent_json,
            intent_sha256: admission_sha256.to_string(),
            state: OperationState::IntentDurable,
            revision: 1,
            attempt: 0,
            last_status_revision: None,
            last_observed_at_ms: None,
            fence: admission.intent.fence.clone(),
            writer: self.writer.clone(),
            created_at_ms: now_ms,
            updated_at_ms: now_ms,
            row_sha256: String::new(),
        }
        .with_digest()?;

        insert_operation(&mut transaction, &row).await?;
        insert_quota_reservation(&mut transaction, &admission, now_ms).await?;
        upsert_active_claim(
            &mut transaction,
            &claim_key,
            admission.intent.operation_id.as_str(),
            &admission.intent.fence,
            now_ms,
        )
        .await?;
        append_fsync_receipt(
            &mut transaction,
            row.operation_id.as_str(),
            "INTENT_DURABLE",
            row.revision,
            &admission_sha256,
            &self.writer,
            now_ms,
        )
        .await?;
        self.maybe_fail(QualificationFailpoint::AdmissionBeforeCommit)?;
        transaction.commit().await.map_err(map_sqlx)?;
        Ok((AdmissionDisposition::Inserted, row.snapshot()?))
    }

    pub async fn begin_dispatch(
        &self,
        operation_id: &str,
        expected_revision: u64,
        started_at_ms: u64,
    ) -> QualificationResult<DispatchTicket> {
        validate_identifier(operation_id)?;
        if expected_revision == 0 || started_at_ms == 0 {
            return Err(QualificationError::InvalidInput);
        }
        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;
        let current = load_operation(&mut transaction, operation_id).await?;
        verify_operation_row(&current)?;
        if current.revision != expected_revision {
            return Err(QualificationError::StaleRevision);
        }
        if current.state.is_terminal() {
            return Err(QualificationError::TerminalImmutable);
        }
        if current.state != OperationState::IntentDurable {
            return Err(QualificationError::InvalidTransition);
        }

        let attempt = current
            .attempt
            .checked_add(1)
            .ok_or(QualificationError::InvalidInput)?;
        let revision = current
            .revision
            .checked_add(1)
            .ok_or(QualificationError::InvalidInput)?;
        let next = current
            .clone()
            .transition(
                OperationState::AttemptStarted,
                revision,
                attempt,
                current.last_status_revision,
                current.last_observed_at_ms,
                self.writer.clone(),
                started_at_ms,
            )?;
        update_operation(&mut transaction, &next, current.revision).await?;

        let attempt_row = DispatchAttemptRow {
            operation_id: operation_id.to_string(),
            attempt,
            operation_revision: revision,
            writer: self.writer.clone(),
            fence: next.fence.clone(),
            started_at_ms,
            marker_kind: None,
            marker_json: None,
            marker_sha256: None,
            marked_at_ms: None,
            row_sha256: String::new(),
        }
        .with_digest()?;
        insert_attempt(&mut transaction, &attempt_row).await?;
        let attempt_payload = attempt_row.digest()?;
        let (witness_sequence, witness_sha256) = append_fsync_receipt(
            &mut transaction,
            operation_id,
            "DISPATCH_ATTEMPT_DURABLE",
            revision,
            &attempt_payload,
            &self.writer,
            started_at_ms,
        )
        .await?;
        self.maybe_fail(QualificationFailpoint::DispatchAttemptBeforeCommit)?;
        transaction.commit().await.map_err(map_sqlx)?;

        Ok(DispatchTicket {
            operation_id: operation_id.to_string(),
            attempt: to_u32(attempt)?,
            operation_revision: revision,
            writer: self.writer.clone(),
            fence: next.fence,
            witness_sequence,
            witness_sha256,
        })
    }

    pub async fn record_dispatch_observation(
        &self,
        ticket: &DispatchTicket,
        observation: DispatchObservation,
    ) -> QualificationResult<(WriteDisposition, OperationSnapshot)> {
        validate_identifier(&ticket.operation_id)?;
        ticket.writer.validate()?;
        ticket.fence.validate()?;
        let marker_json =
            serde_json::to_string(&observation).map_err(|_| QualificationError::InvalidInput)?;
        let marker_sha256 = digest_serializable(
            "hepta.authbus.p0.2.dispatch-observation.v1",
            &observation,
        )?;
        let marked_at_ms = observation.observed_at_ms();

        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;
        if ticket.writer != self.writer {
            return Err(QualificationError::StaleWriter);
        }
        let current = load_operation(&mut transaction, &ticket.operation_id).await?;
        verify_operation_row(&current)?;
        if current.fence != ticket.fence {
            return Err(QualificationError::StaleFence);
        }
        let quota = load_quota(&mut transaction, &ticket.operation_id).await?;
        verify_quota_row(&quota)?;
        observation.validate(quota.reserved)?;

        let attempt = load_attempt(
            &mut transaction,
            &ticket.operation_id,
            u64::from(ticket.attempt),
        )
        .await?;
        verify_attempt_row(&attempt)?;
        if attempt.writer != ticket.writer
            || attempt.fence != ticket.fence
            || attempt.operation_revision != ticket.operation_revision
        {
            return Err(QualificationError::Conflict);
        }
        if let Some(existing) = attempt.marker_sha256.as_deref() {
            if existing == marker_sha256.to_string() {
                return Ok((WriteDisposition::AlreadyPresent, current.snapshot()?));
            }
            return Err(QualificationError::Conflict);
        }

        if current.state != OperationState::AttemptStarted
            || current.revision != ticket.operation_revision
            || current.attempt != u64::from(ticket.attempt)
        {
            return Err(QualificationError::InvalidTransition);
        }

        let target = observation.target_state();
        let revision = current
            .revision
            .checked_add(1)
            .ok_or(QualificationError::InvalidInput)?;
        if target.is_terminal() {
            match &observation {
                DispatchObservation::Completed { actual, .. } => {
                    complete_quota(
                        &mut transaction,
                        &quota,
                        actual.terminal_usage(),
                        marked_at_ms,
                    )
                    .await?;
                }
                DispatchObservation::VerifiedNoEffect { .. } => {
                    release_quota(&mut transaction, &quota, marked_at_ms).await?;
                }
                DispatchObservation::Accepted { .. } | DispatchObservation::Unknown { .. } => {
                    return Err(QualificationError::InvalidTransition);
                }
            }
            release_claim(&mut transaction, &ticket.operation_id, marked_at_ms).await?;
        }

        let next = current
            .clone()
            .transition(
                target,
                revision,
                current.attempt,
                current.last_status_revision,
                Some(marked_at_ms),
                self.writer.clone(),
                marked_at_ms,
            )?;
        update_operation(&mut transaction, &next, current.revision).await?;

        let marked_attempt = attempt.with_marker(
            observation.event_kind().to_string(),
            marker_json,
            marker_sha256.clone(),
            marked_at_ms,
        )?;
        update_attempt_marker(&mut transaction, &marked_attempt).await?;
        insert_outbox(
            &mut transaction,
            &next,
            observation.event_kind(),
            &marker_sha256,
            marked_at_ms,
        )
        .await?;
        append_fsync_receipt(
            &mut transaction,
            &ticket.operation_id,
            "DISPATCH_MARKER_DURABLE",
            revision,
            &marker_sha256,
            &self.writer,
            marked_at_ms,
        )
        .await?;
        self.maybe_fail(QualificationFailpoint::DispatchMarkerBeforeCommit)?;
        transaction.commit().await.map_err(map_sqlx)?;
        Ok((WriteDisposition::Applied, next.snapshot()?))
    }

    pub async fn record_status_observation(
        &self,
        observation: StatusObservation,
    ) -> QualificationResult<(WriteDisposition, OperationSnapshot)> {
        validate_identifier(&observation.operation_id)?;
        let observation_sha256 = observation.digest()?;
        let observation_json =
            serde_json::to_string(&observation).map_err(|_| QualificationError::InvalidInput)?;
        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;
        let current = load_operation(&mut transaction, &observation.operation_id).await?;
        verify_operation_row(&current)?;
        let quota = load_quota(&mut transaction, &observation.operation_id).await?;
        verify_quota_row(&quota)?;
        observation.validate(quota.reserved)?;

        if let Some(existing_sha256) = load_status_observation_sha(
            &mut transaction,
            &observation.operation_id,
            observation.status_revision,
        )
        .await?
        {
            if existing_sha256 == observation_sha256.to_string() {
                return Ok((WriteDisposition::AlreadyPresent, current.snapshot()?));
            }
            return Err(QualificationError::ObservationConflict);
        }

        if current.state.is_terminal() {
            return Err(QualificationError::TerminalImmutable);
        }
        if !current.state.requires_lookup_only() {
            return Err(QualificationError::InvalidTransition);
        }
        if current.fence != observation.fence {
            return Err(QualificationError::StaleFence);
        }
        let admission: QualificationAdmission = serde_json::from_str(&current.intent_json)
            .map_err(|_| QualificationError::Corrupt)?;
        let expected_binding = admission.status_binding_sha256()?;
        if observation.binding_sha256 != expected_binding {
            return Err(QualificationError::ObservationConflict);
        }
        if current
            .last_status_revision
            .is_some_and(|revision| observation.status_revision <= revision)
        {
            return Err(QualificationError::StaleObservation);
        }
        if current
            .last_observed_at_ms
            .is_some_and(|observed_at_ms| observation.observed_at_ms < observed_at_ms)
        {
            return Err(QualificationError::StaleObservation);
        }

        insert_status_observation(
            &mut transaction,
            &observation,
            &observation_json,
            &observation_sha256,
            &self.writer,
        )
        .await?;
        let target = observation.outcome.target_state();
        let revision = current
            .revision
            .checked_add(1)
            .ok_or(QualificationError::InvalidInput)?;
        if target.is_terminal() {
            match &observation.outcome {
                LookupOutcome::Completed { actual, .. } => {
                    complete_quota(
                        &mut transaction,
                        &quota,
                        actual.terminal_usage(),
                        observation.observed_at_ms,
                    )
                    .await?;
                }
                LookupOutcome::VerifiedNoEffect { .. } => {
                    release_quota(&mut transaction, &quota, observation.observed_at_ms).await?;
                }
                LookupOutcome::Unknown { .. } | LookupOutcome::Indeterminate { .. } => {
                    return Err(QualificationError::InvalidTransition);
                }
            }
            release_claim(
                &mut transaction,
                &observation.operation_id,
                observation.observed_at_ms,
            )
            .await?;
        }

        let next = current
            .clone()
            .transition(
                target,
                revision,
                current.attempt,
                Some(observation.status_revision),
                Some(observation.observed_at_ms),
                self.writer.clone(),
                observation.observed_at_ms,
            )?;
        update_operation(&mut transaction, &next, current.revision).await?;
        insert_outbox(
            &mut transaction,
            &next,
            observation.outcome.event_kind(),
            &observation_sha256,
            observation.observed_at_ms,
        )
        .await?;
        append_fsync_receipt(
            &mut transaction,
            &observation.operation_id,
            "STATUS_OBSERVATION_DURABLE",
            revision,
            &observation_sha256,
            &self.writer,
            observation.observed_at_ms,
        )
        .await?;
        self.maybe_fail(QualificationFailpoint::StatusObservationBeforeCommit)?;
        transaction.commit().await.map_err(map_sqlx)?;
        Ok((WriteDisposition::Applied, next.snapshot()?))
    }

    pub async fn snapshot(&self, operation_id: &str) -> QualificationResult<OperationSnapshot> {
        validate_identifier(operation_id)?;
        let row = load_operation_pool(&self.pool, operation_id).await?;
        verify_operation_row(&row)?;
        row.snapshot()
    }

    pub async fn quota_snapshot(&self, operation_id: &str) -> QualificationResult<QuotaSnapshot> {
        validate_identifier(operation_id)?;
        let row = load_quota_pool(&self.pool, operation_id).await?;
        verify_quota_row(&row)?;
        row.snapshot()
    }

    pub async fn status_binding_sha256(
        &self,
        operation_id: &str,
    ) -> QualificationResult<Sha256Digest> {
        let row = load_operation_pool(&self.pool, operation_id).await?;
        verify_operation_row(&row)?;
        let admission: QualificationAdmission =
            serde_json::from_str(&row.intent_json).map_err(|_| QualificationError::Corrupt)?;
        admission.status_binding_sha256()
    }

    pub async fn recover_operation(
        &self,
        operation_id: &str,
    ) -> QualificationResult<RecoveryAction> {
        let snapshot = self.snapshot(operation_id).await?;
        Ok(match snapshot.state {
            OperationState::IntentDurable => RecoveryAction::DispatchEligible {
                operation_id: snapshot.operation_id,
                revision: snapshot.revision,
            },
            state if state.requires_lookup_only() => RecoveryAction::LookupOnly {
                operation_id: snapshot.operation_id,
                attempt: snapshot.attempt,
                revision: snapshot.revision,
            },
            state if state.is_terminal() => RecoveryAction::Terminal {
                operation_id: snapshot.operation_id,
                state,
                revision: snapshot.revision,
            },
            state => RecoveryAction::SafeStop {
                operation_id: snapshot.operation_id,
                state,
                revision: snapshot.revision,
            },
        })
    }

    pub async fn recover_all(&self) -> QualificationResult<Vec<RecoveryAction>> {
        self.verify_integrity().await?;
        let operation_ids: Vec<String> = sqlx::query_scalar(
            "SELECT operation_id FROM operations ORDER BY operation_id ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;
        let mut actions = Vec::with_capacity(operation_ids.len());
        for operation_id in operation_ids {
            actions.push(self.recover_operation(&operation_id).await?);
        }
        Ok(actions)
    }

    pub async fn pending_outbox(&self, limit: u32) -> QualificationResult<Vec<OutboxRecord>> {
        if limit == 0 || limit > 1_000 {
            return Err(QualificationError::InvalidInput);
        }
        let rows = sqlx::query(
            "SELECT sequence, outbox_id, operation_id, operation_revision, event_kind, \
                    idempotency_key, payload_sha256, payload_json, ack_sha256, \
                    created_at_ms, acked_at_ms, state, row_sha256 \
             FROM outbox WHERE state = 'PENDING' ORDER BY sequence ASC LIMIT ?",
        )
        .bind(i64::from(limit))
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;
        rows.into_iter()
            .map(|row| OutboxRow::from_sqlite(&row))
            .map(|row| row.and_then(|value| value.record()))
            .collect()
    }

    pub async fn outbox_cursor_revision(&self) -> QualificationResult<u64> {
        let value: i64 = sqlx::query_scalar(
            "SELECT revision FROM outbox_cursor WHERE singleton = 1",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx)?;
        to_u64(value)
    }

    pub async fn ack_outbox(
        &self,
        outbox_id: &str,
        ack_sha256: Sha256Digest,
        expected_cursor_revision: u64,
        acked_at_ms: u64,
    ) -> QualificationResult<WriteDisposition> {
        validate_identifier(outbox_id)?;
        if acked_at_ms == 0 {
            return Err(QualificationError::InvalidInput);
        }
        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;
        let current = load_outbox(&mut transaction, outbox_id).await?;
        verify_outbox_row(&current)?;
        if current.state == "ACKED" {
            if current.ack_sha256.as_deref() == Some(ack_sha256.to_string().as_str()) {
                return Ok(WriteDisposition::AlreadyPresent);
            }
            return Err(QualificationError::Conflict);
        }

        let cursor: SqliteRow = sqlx::query(
            "SELECT revision, last_sequence FROM outbox_cursor WHERE singleton = 1",
        )
        .fetch_one(&mut *transaction)
        .await
        .map_err(map_sqlx)?;
        let cursor_revision = to_u64(cursor.try_get::<i64, _>("revision").map_err(map_sqlx)?)?;
        let last_sequence =
            to_u64(cursor.try_get::<i64, _>("last_sequence").map_err(map_sqlx)?)?;
        if cursor_revision != expected_cursor_revision {
            return Err(QualificationError::CursorConflict);
        }
        let next_cursor_revision = cursor_revision
            .checked_add(1)
            .ok_or(QualificationError::InvalidInput)?;
        let next = current.with_ack(ack_sha256.clone(), acked_at_ms)?;
        sqlx::query(
            "UPDATE outbox SET state = 'ACKED', ack_sha256 = ?, acked_at_ms = ?, row_sha256 = ? \
             WHERE outbox_id = ? AND state = 'PENDING'",
        )
        .bind(ack_sha256.to_string())
        .bind(to_i64(acked_at_ms)?)
        .bind(&next.row_sha256)
        .bind(outbox_id)
        .execute(&mut *transaction)
        .await
        .map_err(map_sqlx)?;
        sqlx::query(
            "UPDATE outbox_cursor SET revision = ?, last_sequence = ?, updated_at_ms = ? \
             WHERE singleton = 1 AND revision = ?",
        )
        .bind(to_i64(next_cursor_revision)?)
        .bind(to_i64(last_sequence.max(current.sequence))?)
        .bind(to_i64(acked_at_ms)?)
        .bind(to_i64(cursor_revision)?)
        .execute(&mut *transaction)
        .await
        .map_err(map_sqlx)?;
        append_fsync_receipt(
            &mut transaction,
            &current.operation_id,
            "OUTBOX_ACK_DURABLE",
            current.operation_revision,
            &ack_sha256,
            &self.writer,
            acked_at_ms,
        )
        .await?;
        self.maybe_fail(QualificationFailpoint::OutboxAckBeforeCommit)?;
        transaction.commit().await.map_err(map_sqlx)?;
        Ok(WriteDisposition::Applied)
    }

    pub async fn operation_count(&self) -> QualificationResult<u64> {
        count_query(&self.pool, "SELECT COUNT(*) FROM operations").await
    }

    pub async fn active_claim_count(&self) -> QualificationResult<u64> {
        count_query(
            &self.pool,
            "SELECT COUNT(*) FROM token_family_claims WHERE active = 1",
        )
        .await
    }

    pub async fn sqlite_runtime_version(&self) -> QualificationResult<String> {
        sqlx::query_scalar("SELECT sqlite_version()")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx)
    }

    pub async fn qualification_cap_database_pages(&self) -> QualificationResult<u64> {
        let page_count: i64 = sqlx::query_scalar("PRAGMA page_count")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx)?;
        if page_count <= 0 {
            return Err(QualificationError::Corrupt);
        }
        let statement = format!("PRAGMA max_page_count = {page_count}");
        let applied: i64 = sqlx::query_scalar(&statement)
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx)?;
        to_u64(applied)
    }

    pub fn qualification_set_failpoint(&self, failpoint: QualificationFailpoint) {
        self.failpoints.fetch_or(failpoint.bit(), Ordering::SeqCst);
    }

    pub fn qualification_clear_failpoint(&self, failpoint: QualificationFailpoint) {
        self.failpoints.fetch_and(!failpoint.bit(), Ordering::SeqCst);
    }

    pub async fn qualification_inject_corrupt_operation_digest(
        &self,
        operation_id: &str,
    ) -> QualificationResult<()> {
        validate_identifier(operation_id)?;
        let result = sqlx::query(
            "UPDATE operations SET row_sha256 = 'sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff' \
             WHERE operation_id = ?",
        )
        .bind(operation_id)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        if result.rows_affected() != 1 {
            return Err(QualificationError::NotFound);
        }
        Ok(())
    }

    pub async fn qualification_schema_columns(&self) -> QualificationResult<Vec<String>> {
        let tables = [
            "operations",
            "token_family_claims",
            "quota_reservations",
            "dispatch_attempts",
            "status_observations",
            "outbox",
            "fsync_receipts",
        ];
        let mut columns = Vec::new();
        for table in tables {
            let statement = format!("PRAGMA table_info({table})");
            let rows = sqlx::query(&statement)
                .fetch_all(&self.pool)
                .await
                .map_err(map_sqlx)?;
            for row in rows {
                columns.push(row.try_get::<String, _>("name").map_err(map_sqlx)?);
            }
        }
        Ok(columns)
    }

    pub async fn verify_integrity(&self) -> QualificationResult<IntegrityReport> {
        let checks: Vec<String> = sqlx::query_scalar("PRAGMA quick_check")
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx)?;
        if checks.len() != 1 || checks.first().map(String::as_str) != Some("ok") {
            return Err(QualificationError::Corrupt);
        }
        verify_meta(&self.pool).await?;

        let rows = sqlx::query(OPERATION_SELECT_ALL)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx)?;
        for sqlite_row in rows {
            let operation = OperationRow::from_sqlite(&sqlite_row)?;
            verify_operation_row(&operation)?;
            let quota = load_quota_pool(&self.pool, &operation.operation_id).await?;
            verify_quota_row(&quota)?;
            verify_state_resources(&self.pool, &operation, &quota).await?;
        }

        let attempt_rows = sqlx::query(ATTEMPT_SELECT_ALL)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx)?;
        for sqlite_row in attempt_rows {
            verify_attempt_row(&DispatchAttemptRow::from_sqlite(&sqlite_row)?)?;
        }

        let status_rows = sqlx::query(
            "SELECT observation_json, observation_sha256 FROM status_observations",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;
        for row in status_rows {
            let json = row
                .try_get::<String, _>("observation_json")
                .map_err(map_sqlx)?;
            let stored = row
                .try_get::<String, _>("observation_sha256")
                .map_err(map_sqlx)?;
            let observation: StatusObservation =
                serde_json::from_str(&json).map_err(|_| QualificationError::Corrupt)?;
            if observation.digest()?.to_string() != stored {
                return Err(QualificationError::Corrupt);
            }
        }

        let outbox_rows = sqlx::query(OUTBOX_SELECT_ALL)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx)?;
        for sqlite_row in outbox_rows {
            verify_outbox_row(&OutboxRow::from_sqlite(&sqlite_row)?)?;
        }

        let receipt_rows = sqlx::query(RECEIPT_SELECT_ALL)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx)?;
        for sqlite_row in receipt_rows {
            verify_receipt_row(&FsyncReceiptRow::from_sqlite(&sqlite_row)?)?;
        }

        Ok(IntegrityReport {
            operations: self.operation_count().await?,
            active_claims: self.active_claim_count().await?,
            held_reservations: count_query(
                &self.pool,
                "SELECT COUNT(*) FROM quota_reservations WHERE state = 'HELD'",
            )
            .await?,
            pending_outbox: count_query(
                &self.pool,
                "SELECT COUNT(*) FROM outbox WHERE state = 'PENDING'",
            )
            .await?,
            fsync_receipts: count_query(&self.pool, "SELECT COUNT(*) FROM fsync_receipts")
                .await?,
        })
    }

    async fn ensure_current_writer(
        &self,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> QualificationResult<()> {
        let row = sqlx::query(
            "SELECT writer_boot_id, writer_generation FROM authbus_p0_2_meta WHERE singleton = 1",
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(map_sqlx)?;
        let boot_id = row
            .try_get::<String, _>("writer_boot_id")
            .map_err(map_sqlx)?;
        let generation =
            to_u64(row.try_get::<i64, _>("writer_generation").map_err(map_sqlx)?)?;
        if boot_id != self.writer.boot_id || generation != self.writer.generation {
            return Err(QualificationError::StaleWriter);
        }
        Ok(())
    }

    fn maybe_fail(&self, failpoint: QualificationFailpoint) -> QualificationResult<()> {
        if self.failpoints.load(Ordering::SeqCst) & failpoint.bit() != 0 {
            return Err(QualificationError::InjectedDiskFull);
        }
        Ok(())
    }
}

const OPERATION_SELECT: &str =
    "SELECT operation_id, operation_key, effect_key, idempotency_key, operation_kind, \
            provider_id, profile_id, token_family_id, intent_json, intent_sha256, state, \
            revision, attempt, last_status_revision, last_observed_at_ms, authority_epoch, \
            owner_epoch, generation, fencing_token_sha256, writer_boot_id, writer_generation, \
            created_at_ms, updated_at_ms, row_sha256 \
     FROM operations WHERE operation_id = ?";
const OPERATION_SELECT_ALL: &str =
    "SELECT operation_id, operation_key, effect_key, idempotency_key, operation_kind, \
            provider_id, profile_id, token_family_id, intent_json, intent_sha256, state, \
            revision, attempt, last_status_revision, last_observed_at_ms, authority_epoch, \
            owner_epoch, generation, fencing_token_sha256, writer_boot_id, writer_generation, \
            created_at_ms, updated_at_ms, row_sha256 \
     FROM operations ORDER BY operation_id ASC";
const ATTEMPT_SELECT: &str =
    "SELECT operation_id, attempt, operation_revision, writer_boot_id, writer_generation, \
            authority_epoch, owner_epoch, generation, fencing_token_sha256, started_at_ms, \
            marker_kind, marker_json, marker_sha256, marked_at_ms, row_sha256 \
     FROM dispatch_attempts WHERE operation_id = ? AND attempt = ?";
const ATTEMPT_SELECT_ALL: &str =
    "SELECT operation_id, attempt, operation_revision, writer_boot_id, writer_generation, \
            authority_epoch, owner_epoch, generation, fencing_token_sha256, started_at_ms, \
            marker_kind, marker_json, marker_sha256, marked_at_ms, row_sha256 \
     FROM dispatch_attempts ORDER BY operation_id ASC, attempt ASC";
const OUTBOX_SELECT: &str =
    "SELECT sequence, outbox_id, operation_id, operation_revision, event_kind, idempotency_key, \
            payload_sha256, payload_json, ack_sha256, created_at_ms, acked_at_ms, state, row_sha256 \
     FROM outbox WHERE outbox_id = ?";
const OUTBOX_SELECT_ALL: &str =
    "SELECT sequence, outbox_id, operation_id, operation_revision, event_kind, idempotency_key, \
            payload_sha256, payload_json, ack_sha256, created_at_ms, acked_at_ms, state, row_sha256 \
     FROM outbox ORDER BY sequence ASC";
const RECEIPT_SELECT_ALL: &str =
    "SELECT sequence, operation_id, phase, operation_revision, payload_sha256, writer_boot_id, \
            writer_generation, recorded_at_ms, witness_sha256 \
     FROM fsync_receipts ORDER BY sequence ASC";

#[derive(Clone, Debug)]
struct OperationRow {
    operation_id: String,
    operation_key: String,
    effect_key: String,
    idempotency_key: String,
    operation_kind: String,
    provider_id: String,
    profile_id: String,
    token_family_id: String,
    intent_json: String,
    intent_sha256: String,
    state: OperationState,
    revision: u64,
    attempt: u64,
    last_status_revision: Option<u64>,
    last_observed_at_ms: Option<u64>,
    fence: QualificationFence,
    writer: WriterIdentity,
    created_at_ms: u64,
    updated_at_ms: u64,
    row_sha256: String,
}

impl OperationRow {
    fn from_sqlite(row: &SqliteRow) -> QualificationResult<Self> {
        Ok(Self {
            operation_id: row.try_get("operation_id").map_err(map_sqlx)?,
            operation_key: row.try_get("operation_key").map_err(map_sqlx)?,
            effect_key: row.try_get("effect_key").map_err(map_sqlx)?,
            idempotency_key: row.try_get("idempotency_key").map_err(map_sqlx)?,
            operation_kind: row.try_get("operation_kind").map_err(map_sqlx)?,
            provider_id: row.try_get("provider_id").map_err(map_sqlx)?,
            profile_id: row.try_get("profile_id").map_err(map_sqlx)?,
            token_family_id: row.try_get("token_family_id").map_err(map_sqlx)?,
            intent_json: row.try_get("intent_json").map_err(map_sqlx)?,
            intent_sha256: row.try_get("intent_sha256").map_err(map_sqlx)?,
            state: OperationState::from_db(
                row.try_get::<String, _>("state")
                    .map_err(map_sqlx)?
                    .as_str(),
            )?,
            revision: to_u64(row.try_get::<i64, _>("revision").map_err(map_sqlx)?)?,
            attempt: to_u64(row.try_get::<i64, _>("attempt").map_err(map_sqlx)?)?,
            last_status_revision: optional_u64(
                row.try_get::<Option<i64>, _>("last_status_revision")
                    .map_err(map_sqlx)?,
            )?,
            last_observed_at_ms: optional_u64(
                row.try_get::<Option<i64>, _>("last_observed_at_ms")
                    .map_err(map_sqlx)?,
            )?,
            fence: QualificationFence {
                authority_epoch: to_u64(
                    row.try_get::<i64, _>("authority_epoch")
                        .map_err(map_sqlx)?,
                )?,
                owner_epoch: to_u64(
                    row.try_get::<i64, _>("owner_epoch")
                        .map_err(map_sqlx)?,
                )?,
                generation: to_u64(
                    row.try_get::<i64, _>("generation")
                        .map_err(map_sqlx)?,
                )?,
                fencing_token_sha256: parse_digest(
                    row.try_get::<String, _>("fencing_token_sha256")
                        .map_err(map_sqlx)?,
                )?,
            },
            writer: WriterIdentity {
                boot_id: row.try_get("writer_boot_id").map_err(map_sqlx)?,
                generation: to_u64(
                    row.try_get::<i64, _>("writer_generation")
                        .map_err(map_sqlx)?,
                )?,
            },
            created_at_ms: to_u64(
                row.try_get::<i64, _>("created_at_ms")
                    .map_err(map_sqlx)?,
            )?,
            updated_at_ms: to_u64(
                row.try_get::<i64, _>("updated_at_ms")
                    .map_err(map_sqlx)?,
            )?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn with_digest(mut self) -> QualificationResult<Self> {
        self.row_sha256 = self.digest()?.to_string();
        Ok(self)
    }

    fn digest(&self) -> QualificationResult<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            operation_id: &'a str,
            operation_key: &'a str,
            effect_key: &'a str,
            idempotency_key: &'a str,
            operation_kind: &'a str,
            provider_id: &'a str,
            profile_id: &'a str,
            token_family_id: &'a str,
            intent_sha256: &'a str,
            state: &'a str,
            revision: u64,
            attempt: u64,
            last_status_revision: Option<u64>,
            last_observed_at_ms: Option<u64>,
            fence: &'a QualificationFence,
            writer: &'a WriterIdentity,
            created_at_ms: u64,
            updated_at_ms: u64,
        }
        digest_serializable(
            "hepta.authbus.p0.2.operation-row.v1",
            &Witness {
                operation_id: &self.operation_id,
                operation_key: &self.operation_key,
                effect_key: &self.effect_key,
                idempotency_key: &self.idempotency_key,
                operation_kind: &self.operation_kind,
                provider_id: &self.provider_id,
                profile_id: &self.profile_id,
                token_family_id: &self.token_family_id,
                intent_sha256: &self.intent_sha256,
                state: self.state.as_db(),
                revision: self.revision,
                attempt: self.attempt,
                last_status_revision: self.last_status_revision,
                last_observed_at_ms: self.last_observed_at_ms,
                fence: &self.fence,
                writer: &self.writer,
                created_at_ms: self.created_at_ms,
                updated_at_ms: self.updated_at_ms,
            },
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn transition(
        mut self,
        state: OperationState,
        revision: u64,
        attempt: u64,
        last_status_revision: Option<u64>,
        last_observed_at_ms: Option<u64>,
        writer: WriterIdentity,
        updated_at_ms: u64,
    ) -> QualificationResult<Self> {
        if revision <= self.revision || updated_at_ms < self.updated_at_ms {
            return Err(QualificationError::InvalidTransition);
        }
        self.state = state;
        self.revision = revision;
        self.attempt = attempt;
        self.last_status_revision = last_status_revision;
        self.last_observed_at_ms = last_observed_at_ms;
        self.writer = writer;
        self.updated_at_ms = updated_at_ms;
        self.with_digest()
    }

    fn snapshot(&self) -> QualificationResult<OperationSnapshot> {
        Ok(OperationSnapshot {
            operation_id: self.operation_id.clone(),
            state: self.state,
            revision: self.revision,
            attempt: to_u32(self.attempt)?,
            last_status_revision: self.last_status_revision,
            last_observed_at_ms: self.last_observed_at_ms,
            writer: self.writer.clone(),
            fence: self.fence.clone(),
        })
    }
}

#[derive(Clone, Debug)]
struct QuotaRow {
    operation_id: String,
    permit_id: String,
    resource_id: String,
    resource_sha256: String,
    reserved: QualificationQuota,
    used: QualificationQuota,
    state: String,
    revision: u64,
    updated_at_ms: u64,
    row_sha256: String,
}

impl QuotaRow {
    fn from_sqlite(row: &SqliteRow) -> QualificationResult<Self> {
        Ok(Self {
            operation_id: row.try_get("operation_id").map_err(map_sqlx)?,
            permit_id: row.try_get("permit_id").map_err(map_sqlx)?,
            resource_id: row.try_get("resource_id").map_err(map_sqlx)?,
            resource_sha256: row.try_get("resource_sha256").map_err(map_sqlx)?,
            reserved: QualificationQuota {
                rpm: to_u64(row.try_get::<i64, _>("reserved_rpm").map_err(map_sqlx)?)?,
                tpm: to_u64(row.try_get::<i64, _>("reserved_tpm").map_err(map_sqlx)?)?,
                concurrency: to_u64(
                    row.try_get::<i64, _>("reserved_concurrency")
                        .map_err(map_sqlx)?,
                )?,
                day_budget: to_u64(
                    row.try_get::<i64, _>("reserved_day_budget")
                        .map_err(map_sqlx)?,
                )?,
                context: to_u64(
                    row.try_get::<i64, _>("reserved_context")
                        .map_err(map_sqlx)?,
                )?,
            },
            used: QualificationQuota {
                rpm: to_u64(row.try_get::<i64, _>("used_rpm").map_err(map_sqlx)?)?,
                tpm: to_u64(row.try_get::<i64, _>("used_tpm").map_err(map_sqlx)?)?,
                concurrency: to_u64(
                    row.try_get::<i64, _>("used_concurrency")
                        .map_err(map_sqlx)?,
                )?,
                day_budget: to_u64(
                    row.try_get::<i64, _>("used_day_budget")
                        .map_err(map_sqlx)?,
                )?,
                context: to_u64(row.try_get::<i64, _>("used_context").map_err(map_sqlx)?)?,
            },
            state: row.try_get("state").map_err(map_sqlx)?,
            revision: to_u64(row.try_get::<i64, _>("revision").map_err(map_sqlx)?)?,
            updated_at_ms: to_u64(
                row.try_get::<i64, _>("updated_at_ms")
                    .map_err(map_sqlx)?,
            )?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn digest(&self) -> QualificationResult<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            operation_id: &'a str,
            permit_id: &'a str,
            resource_id: &'a str,
            resource_sha256: &'a str,
            reserved: QualificationQuota,
            used: QualificationQuota,
            state: &'a str,
            revision: u64,
            updated_at_ms: u64,
        }
        digest_serializable(
            "hepta.authbus.p0.2.quota-row.v1",
            &Witness {
                operation_id: &self.operation_id,
                permit_id: &self.permit_id,
                resource_id: &self.resource_id,
                resource_sha256: &self.resource_sha256,
                reserved: self.reserved,
                used: self.used,
                state: &self.state,
                revision: self.revision,
                updated_at_ms: self.updated_at_ms,
            },
        )
    }

    fn snapshot(&self) -> QualificationResult<QuotaSnapshot> {
        let state = match self.state.as_str() {
            "HELD" => QuotaReservationState::Held,
            "COMPLETED" => QuotaReservationState::Completed,
            "RELEASED" => QuotaReservationState::Released,
            _ => return Err(QualificationError::Corrupt),
        };
        Ok(QuotaSnapshot {
            operation_id: self.operation_id.clone(),
            reserved: self.reserved,
            used: self.used,
            state,
            revision: self.revision,
        })
    }
}

#[derive(Clone, Debug)]
struct DispatchAttemptRow {
    operation_id: String,
    attempt: u64,
    operation_revision: u64,
    writer: WriterIdentity,
    fence: QualificationFence,
    started_at_ms: u64,
    marker_kind: Option<String>,
    marker_json: Option<String>,
    marker_sha256: Option<String>,
    marked_at_ms: Option<u64>,
    row_sha256: String,
}

impl DispatchAttemptRow {
    fn from_sqlite(row: &SqliteRow) -> QualificationResult<Self> {
        Ok(Self {
            operation_id: row.try_get("operation_id").map_err(map_sqlx)?,
            attempt: to_u64(row.try_get::<i64, _>("attempt").map_err(map_sqlx)?)?,
            operation_revision: to_u64(
                row.try_get::<i64, _>("operation_revision")
                    .map_err(map_sqlx)?,
            )?,
            writer: WriterIdentity {
                boot_id: row.try_get("writer_boot_id").map_err(map_sqlx)?,
                generation: to_u64(
                    row.try_get::<i64, _>("writer_generation")
                        .map_err(map_sqlx)?,
                )?,
            },
            fence: QualificationFence {
                authority_epoch: to_u64(
                    row.try_get::<i64, _>("authority_epoch")
                        .map_err(map_sqlx)?,
                )?,
                owner_epoch: to_u64(
                    row.try_get::<i64, _>("owner_epoch")
                        .map_err(map_sqlx)?,
                )?,
                generation: to_u64(
                    row.try_get::<i64, _>("generation")
                        .map_err(map_sqlx)?,
                )?,
                fencing_token_sha256: parse_digest(
                    row.try_get::<String, _>("fencing_token_sha256")
                        .map_err(map_sqlx)?,
                )?,
            },
            started_at_ms: to_u64(
                row.try_get::<i64, _>("started_at_ms")
                    .map_err(map_sqlx)?,
            )?,
            marker_kind: row.try_get("marker_kind").map_err(map_sqlx)?,
            marker_json: row.try_get("marker_json").map_err(map_sqlx)?,
            marker_sha256: row.try_get("marker_sha256").map_err(map_sqlx)?,
            marked_at_ms: optional_u64(
                row.try_get::<Option<i64>, _>("marked_at_ms")
                    .map_err(map_sqlx)?,
            )?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn with_digest(mut self) -> QualificationResult<Self> {
        self.row_sha256 = self.digest()?.to_string();
        Ok(self)
    }

    fn with_marker(
        mut self,
        marker_kind: String,
        marker_json: String,
        marker_sha256: Sha256Digest,
        marked_at_ms: u64,
    ) -> QualificationResult<Self> {
        self.marker_kind = Some(marker_kind);
        self.marker_json = Some(marker_json);
        self.marker_sha256 = Some(marker_sha256.to_string());
        self.marked_at_ms = Some(marked_at_ms);
        self.with_digest()
    }

    fn digest(&self) -> QualificationResult<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            operation_id: &'a str,
            attempt: u64,
            operation_revision: u64,
            writer: &'a WriterIdentity,
            fence: &'a QualificationFence,
            started_at_ms: u64,
            marker_kind: &'a Option<String>,
            marker_sha256: &'a Option<String>,
            marked_at_ms: Option<u64>,
        }
        digest_serializable(
            "hepta.authbus.p0.2.dispatch-attempt-row.v1",
            &Witness {
                operation_id: &self.operation_id,
                attempt: self.attempt,
                operation_revision: self.operation_revision,
                writer: &self.writer,
                fence: &self.fence,
                started_at_ms: self.started_at_ms,
                marker_kind: &self.marker_kind,
                marker_sha256: &self.marker_sha256,
                marked_at_ms: self.marked_at_ms,
            },
        )
    }
}

#[derive(Clone, Debug)]
struct OutboxRow {
    sequence: u64,
    outbox_id: String,
    operation_id: String,
    operation_revision: u64,
    event_kind: String,
    idempotency_key: String,
    payload_sha256: String,
    payload_json: String,
    ack_sha256: Option<String>,
    created_at_ms: u64,
    acked_at_ms: Option<u64>,
    state: String,
    row_sha256: String,
}

impl OutboxRow {
    fn from_sqlite(row: &SqliteRow) -> QualificationResult<Self> {
        Ok(Self {
            sequence: to_u64(row.try_get::<i64, _>("sequence").map_err(map_sqlx)?)?,
            outbox_id: row.try_get("outbox_id").map_err(map_sqlx)?,
            operation_id: row.try_get("operation_id").map_err(map_sqlx)?,
            operation_revision: to_u64(
                row.try_get::<i64, _>("operation_revision")
                    .map_err(map_sqlx)?,
            )?,
            event_kind: row.try_get("event_kind").map_err(map_sqlx)?,
            idempotency_key: row.try_get("idempotency_key").map_err(map_sqlx)?,
            payload_sha256: row.try_get("payload_sha256").map_err(map_sqlx)?,
            payload_json: row.try_get("payload_json").map_err(map_sqlx)?,
            ack_sha256: row.try_get("ack_sha256").map_err(map_sqlx)?,
            created_at_ms: to_u64(
                row.try_get::<i64, _>("created_at_ms")
                    .map_err(map_sqlx)?,
            )?,
            acked_at_ms: optional_u64(
                row.try_get::<Option<i64>, _>("acked_at_ms")
                    .map_err(map_sqlx)?,
            )?,
            state: row.try_get("state").map_err(map_sqlx)?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn digest(&self) -> QualificationResult<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            sequence: u64,
            outbox_id: &'a str,
            operation_id: &'a str,
            operation_revision: u64,
            event_kind: &'a str,
            idempotency_key: &'a str,
            payload_sha256: &'a str,
            ack_sha256: &'a Option<String>,
            created_at_ms: u64,
            acked_at_ms: Option<u64>,
            state: &'a str,
        }
        digest_serializable(
            "hepta.authbus.p0.2.outbox-row.v1",
            &Witness {
                sequence: self.sequence,
                outbox_id: &self.outbox_id,
                operation_id: &self.operation_id,
                operation_revision: self.operation_revision,
                event_kind: &self.event_kind,
                idempotency_key: &self.idempotency_key,
                payload_sha256: &self.payload_sha256,
                ack_sha256: &self.ack_sha256,
                created_at_ms: self.created_at_ms,
                acked_at_ms: self.acked_at_ms,
                state: &self.state,
            },
        )
    }

    fn with_ack(mut self, ack_sha256: Sha256Digest, acked_at_ms: u64) -> QualificationResult<Self> {
        self.ack_sha256 = Some(ack_sha256.to_string());
        self.acked_at_ms = Some(acked_at_ms);
        self.state = "ACKED".to_string();
        self.row_sha256 = self.digest()?.to_string();
        Ok(self)
    }

    fn record(self) -> QualificationResult<OutboxRecord> {
        verify_outbox_row(&self)?;
        Ok(OutboxRecord {
            sequence: self.sequence,
            outbox_id: self.outbox_id,
            operation_id: self.operation_id,
            operation_revision: self.operation_revision,
            event_kind: self.event_kind,
            idempotency_key: self.idempotency_key,
            payload_sha256: parse_digest(self.payload_sha256)?,
            payload_json: self.payload_json,
            ack_sha256: self.ack_sha256.map(parse_digest).transpose()?,
        })
    }
}

#[derive(Clone, Debug)]
struct FsyncReceiptRow {
    sequence: u64,
    operation_id: String,
    phase: String,
    operation_revision: u64,
    payload_sha256: String,
    writer: WriterIdentity,
    recorded_at_ms: u64,
    witness_sha256: String,
}

impl FsyncReceiptRow {
    fn from_sqlite(row: &SqliteRow) -> QualificationResult<Self> {
        Ok(Self {
            sequence: to_u64(row.try_get::<i64, _>("sequence").map_err(map_sqlx)?)?,
            operation_id: row.try_get("operation_id").map_err(map_sqlx)?,
            phase: row.try_get("phase").map_err(map_sqlx)?,
            operation_revision: to_u64(
                row.try_get::<i64, _>("operation_revision")
                    .map_err(map_sqlx)?,
            )?,
            payload_sha256: row.try_get("payload_sha256").map_err(map_sqlx)?,
            writer: WriterIdentity {
                boot_id: row.try_get("writer_boot_id").map_err(map_sqlx)?,
                generation: to_u64(
                    row.try_get::<i64, _>("writer_generation")
                        .map_err(map_sqlx)?,
                )?,
            },
            recorded_at_ms: to_u64(
                row.try_get::<i64, _>("recorded_at_ms")
                    .map_err(map_sqlx)?,
            )?,
            witness_sha256: row.try_get("witness_sha256").map_err(map_sqlx)?,
        })
    }

    fn digest(&self) -> QualificationResult<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            operation_id: &'a str,
            phase: &'a str,
            operation_revision: u64,
            payload_sha256: &'a str,
            writer: &'a WriterIdentity,
            recorded_at_ms: u64,
        }
        digest_serializable(
            "hepta.authbus.p0.2.fsync-receipt.v1",
            &Witness {
                operation_id: &self.operation_id,
                phase: &self.phase,
                operation_revision: self.operation_revision,
                payload_sha256: &self.payload_sha256,
                writer: &self.writer,
                recorded_at_ms: self.recorded_at_ms,
            },
        )
    }
}

async fn initialize_or_rebind_meta(
    pool: &SqlitePool,
    writer: &WriterIdentity,
    now_ms: u64,
) -> QualificationResult<()> {
    let mut transaction = pool.begin().await.map_err(map_sqlx)?;
    let existing = sqlx::query(
        "SELECT writer_boot_id, writer_generation, writer_epoch FROM authbus_p0_2_meta \
         WHERE singleton = 1",
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(map_sqlx)?;
    if let Some(row) = existing {
        let boot_id = row
            .try_get::<String, _>("writer_boot_id")
            .map_err(map_sqlx)?;
        let generation =
            to_u64(row.try_get::<i64, _>("writer_generation").map_err(map_sqlx)?)?;
        let epoch = to_u64(row.try_get::<i64, _>("writer_epoch").map_err(map_sqlx)?)?;
        if generation > writer.generation
            || (generation == writer.generation && boot_id != writer.boot_id)
        {
            return Err(QualificationError::StaleWriter);
        }
        if generation < writer.generation {
            let next_epoch = epoch
                .checked_add(1)
                .ok_or(QualificationError::InvalidInput)?;
            sqlx::query(
                "UPDATE authbus_p0_2_meta SET writer_boot_id = ?, writer_generation = ?, \
                        writer_epoch = ?, updated_at_ms = ? WHERE singleton = 1",
            )
            .bind(&writer.boot_id)
            .bind(to_i64(writer.generation)?)
            .bind(to_i64(next_epoch)?)
            .bind(to_i64(now_ms)?)
            .execute(&mut *transaction)
            .await
            .map_err(map_sqlx)?;
        }
    } else {
        sqlx::query(
            "INSERT INTO authbus_p0_2_meta (singleton, schema_version, qualification_only, \
                authority, effect_authority, production_caller, production_writer, \
                operator_acceptance, promotion, g5_allowed, execute_allowed, writer_boot_id, \
                writer_generation, writer_epoch, created_at_ms, updated_at_ms) \
             VALUES (1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, ?, ?, 1, ?, ?)",
        )
        .bind(&writer.boot_id)
        .bind(to_i64(writer.generation)?)
        .bind(to_i64(now_ms)?)
        .bind(to_i64(now_ms)?)
        .execute(&mut *transaction)
        .await
        .map_err(map_sqlx)?;
        sqlx::query(
            "INSERT INTO outbox_cursor (singleton, revision, last_sequence, updated_at_ms) \
             VALUES (1, 0, 0, ?)",
        )
        .bind(to_i64(now_ms)?)
        .execute(&mut *transaction)
        .await
        .map_err(map_sqlx)?;
    }
    transaction.commit().await.map_err(map_sqlx)
}

async fn verify_meta(pool: &SqlitePool) -> QualificationResult<()> {
    let row = sqlx::query(
        "SELECT schema_version, qualification_only, authority, effect_authority, \
                production_caller, production_writer, operator_acceptance, promotion, \
                g5_allowed, execute_allowed FROM authbus_p0_2_meta WHERE singleton = 1",
    )
    .fetch_one(pool)
    .await
    .map_err(map_sqlx)?;
    let values = [
        row.try_get::<i64, _>("schema_version").map_err(map_sqlx)?,
        row.try_get::<i64, _>("qualification_only")
            .map_err(map_sqlx)?,
        row.try_get::<i64, _>("authority").map_err(map_sqlx)?,
        row.try_get::<i64, _>("effect_authority")
            .map_err(map_sqlx)?,
        row.try_get::<i64, _>("production_caller")
            .map_err(map_sqlx)?,
        row.try_get::<i64, _>("production_writer")
            .map_err(map_sqlx)?,
        row.try_get::<i64, _>("operator_acceptance")
            .map_err(map_sqlx)?,
        row.try_get::<i64, _>("promotion").map_err(map_sqlx)?,
        row.try_get::<i64, _>("g5_allowed").map_err(map_sqlx)?,
        row.try_get::<i64, _>("execute_allowed")
            .map_err(map_sqlx)?,
    ];
    if values != [1, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        || !AUTHBUS_P0_2_QUALIFICATION_ONLY
        || AUTHBUS_P0_2_AUTHORITY
        || AUTHBUS_P0_2_EFFECT_AUTHORITY
        || AUTHBUS_P0_2_PRODUCTION_CALLER
        || AUTHBUS_P0_2_PRODUCTION_WRITER
        || AUTHBUS_P0_2_OPERATOR_ACCEPTANCE
        || AUTHBUS_P0_2_PROMOTION
        || AUTHBUS_P0_2_G5_ALLOWED
        || AUTHBUS_P0_2_EXECUTE_ALLOWED
    {
        return Err(QualificationError::Corrupt);
    }
    Ok(())
}

async fn insert_operation(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &OperationRow,
) -> QualificationResult<()> {
    sqlx::query(
        "INSERT INTO operations (operation_id, operation_key, effect_key, idempotency_key, \
            operation_kind, provider_id, profile_id, token_family_id, intent_json, intent_sha256, \
            state, revision, attempt, last_status_revision, last_observed_at_ms, authority_epoch, \
            owner_epoch, generation, fencing_token_sha256, writer_boot_id, writer_generation, \
            created_at_ms, updated_at_ms, row_sha256) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&row.operation_id)
    .bind(&row.operation_key)
    .bind(&row.effect_key)
    .bind(&row.idempotency_key)
    .bind(&row.operation_kind)
    .bind(&row.provider_id)
    .bind(&row.profile_id)
    .bind(&row.token_family_id)
    .bind(&row.intent_json)
    .bind(&row.intent_sha256)
    .bind(row.state.as_db())
    .bind(to_i64(row.revision)?)
    .bind(to_i64(row.attempt)?)
    .bind(optional_i64(row.last_status_revision)?)
    .bind(optional_i64(row.last_observed_at_ms)?)
    .bind(to_i64(row.fence.authority_epoch)?)
    .bind(to_i64(row.fence.owner_epoch)?)
    .bind(to_i64(row.fence.generation)?)
    .bind(row.fence.fencing_token_sha256.to_string())
    .bind(&row.writer.boot_id)
    .bind(to_i64(row.writer.generation)?)
    .bind(to_i64(row.created_at_ms)?)
    .bind(to_i64(row.updated_at_ms)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn update_operation(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &OperationRow,
    previous_revision: u64,
) -> QualificationResult<()> {
    let result = sqlx::query(
        "UPDATE operations SET state = ?, revision = ?, attempt = ?, last_status_revision = ?, \
            last_observed_at_ms = ?, writer_boot_id = ?, writer_generation = ?, updated_at_ms = ?, \
            row_sha256 = ? WHERE operation_id = ? AND revision = ?",
    )
    .bind(row.state.as_db())
    .bind(to_i64(row.revision)?)
    .bind(to_i64(row.attempt)?)
    .bind(optional_i64(row.last_status_revision)?)
    .bind(optional_i64(row.last_observed_at_ms)?)
    .bind(&row.writer.boot_id)
    .bind(to_i64(row.writer.generation)?)
    .bind(to_i64(row.updated_at_ms)?)
    .bind(&row.row_sha256)
    .bind(&row.operation_id)
    .bind(to_i64(previous_revision)?)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    if result.rows_affected() != 1 {
        return Err(QualificationError::StaleRevision);
    }
    Ok(())
}

async fn load_operation(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_id: &str,
) -> QualificationResult<OperationRow> {
    let row = sqlx::query(OPERATION_SELECT)
        .bind(operation_id)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(map_sqlx)?
        .ok_or(QualificationError::NotFound)?;
    OperationRow::from_sqlite(&row)
}

async fn load_operation_optional(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_id: &str,
) -> QualificationResult<Option<OperationRow>> {
    sqlx::query(OPERATION_SELECT)
        .bind(operation_id)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(map_sqlx)?
        .map(|row| OperationRow::from_sqlite(&row))
        .transpose()
}

async fn load_operation_pool(
    pool: &SqlitePool,
    operation_id: &str,
) -> QualificationResult<OperationRow> {
    let row = sqlx::query(OPERATION_SELECT)
        .bind(operation_id)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?
        .ok_or(QualificationError::NotFound)?;
    OperationRow::from_sqlite(&row)
}

async fn operation_binding_exists(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_key: &str,
    effect_key: &str,
    idempotency_key: &str,
) -> QualificationResult<bool> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM operations WHERE operation_key = ? OR effect_key = ? OR idempotency_key = ?",
    )
    .bind(operation_key)
    .bind(effect_key)
    .bind(idempotency_key)
    .fetch_one(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(count > 0)
}

async fn load_claim(
    transaction: &mut Transaction<'_, Sqlite>,
    claim_sha256: &str,
) -> QualificationResult<Option<(String, bool)>> {
    let row = sqlx::query(
        "SELECT operation_id, active FROM token_family_claims WHERE claim_sha256 = ?",
    )
    .bind(claim_sha256)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    row.map(|value| {
        let operation_id = value.try_get("operation_id").map_err(map_sqlx)?;
        let active = value.try_get::<i64, _>("active").map_err(map_sqlx)? == 1;
        Ok((operation_id, active))
    })
    .transpose()
}

async fn upsert_active_claim(
    transaction: &mut Transaction<'_, Sqlite>,
    claim_sha256: &str,
    operation_id: &str,
    fence: &QualificationFence,
    acquired_at_ms: u64,
) -> QualificationResult<()> {
    sqlx::query(
        "INSERT INTO token_family_claims (claim_sha256, operation_id, active, authority_epoch, \
            owner_epoch, generation, fencing_token_sha256, acquired_at_ms, released_at_ms) \
         VALUES (?, ?, 1, ?, ?, ?, ?, ?, NULL) \
         ON CONFLICT(claim_sha256) DO UPDATE SET operation_id = excluded.operation_id, active = 1, \
            authority_epoch = excluded.authority_epoch, owner_epoch = excluded.owner_epoch, \
            generation = excluded.generation, fencing_token_sha256 = excluded.fencing_token_sha256, \
            acquired_at_ms = excluded.acquired_at_ms, released_at_ms = NULL \
         WHERE token_family_claims.active = 0",
    )
    .bind(claim_sha256)
    .bind(operation_id)
    .bind(to_i64(fence.authority_epoch)?)
    .bind(to_i64(fence.owner_epoch)?)
    .bind(to_i64(fence.generation)?)
    .bind(fence.fencing_token_sha256.to_string())
    .bind(to_i64(acquired_at_ms)?)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn release_claim(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_id: &str,
    released_at_ms: u64,
) -> QualificationResult<()> {
    let result = sqlx::query(
        "UPDATE token_family_claims SET active = 0, released_at_ms = ? \
         WHERE operation_id = ? AND active = 1",
    )
    .bind(to_i64(released_at_ms)?)
    .bind(operation_id)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    if result.rows_affected() != 1 {
        return Err(QualificationError::Corrupt);
    }
    Ok(())
}

async fn insert_quota_reservation(
    transaction: &mut Transaction<'_, Sqlite>,
    admission: &QualificationAdmission,
    now_ms: u64,
) -> QualificationResult<()> {
    let row = QuotaRow {
        operation_id: admission.intent.operation_id.clone(),
        permit_id: admission.permit.permit_id.clone(),
        resource_id: admission.permit.resource_id.clone(),
        resource_sha256: admission.permit.resource_sha256.to_string(),
        reserved: admission.permit.reserved,
        used: QualificationQuota::default(),
        state: "HELD".to_string(),
        revision: 1,
        updated_at_ms: now_ms,
        row_sha256: String::new(),
    };
    let digest = row.digest()?.to_string();
    sqlx::query(
        "INSERT INTO quota_reservations (operation_id, permit_id, resource_id, resource_sha256, \
            reserved_rpm, reserved_tpm, reserved_concurrency, reserved_day_budget, reserved_context, \
            used_rpm, used_tpm, used_concurrency, used_day_budget, used_context, state, revision, \
            updated_at_ms, row_sha256) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0, 0, 0, 0, \
            'HELD', 1, ?, ?)",
    )
    .bind(&row.operation_id)
    .bind(&row.permit_id)
    .bind(&row.resource_id)
    .bind(&row.resource_sha256)
    .bind(to_i64(row.reserved.rpm)?)
    .bind(to_i64(row.reserved.tpm)?)
    .bind(to_i64(row.reserved.concurrency)?)
    .bind(to_i64(row.reserved.day_budget)?)
    .bind(to_i64(row.reserved.context)?)
    .bind(to_i64(now_ms)?)
    .bind(digest)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

const QUOTA_SELECT: &str =
    "SELECT operation_id, permit_id, resource_id, resource_sha256, reserved_rpm, reserved_tpm, \
            reserved_concurrency, reserved_day_budget, reserved_context, used_rpm, used_tpm, \
            used_concurrency, used_day_budget, used_context, state, revision, updated_at_ms, row_sha256 \
     FROM quota_reservations WHERE operation_id = ?";

async fn load_quota(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_id: &str,
) -> QualificationResult<QuotaRow> {
    let row = sqlx::query(QUOTA_SELECT)
        .bind(operation_id)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(map_sqlx)?
        .ok_or(QualificationError::Corrupt)?;
    QuotaRow::from_sqlite(&row)
}

async fn load_quota_pool(
    pool: &SqlitePool,
    operation_id: &str,
) -> QualificationResult<QuotaRow> {
    let row = sqlx::query(QUOTA_SELECT)
        .bind(operation_id)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?
        .ok_or(QualificationError::Corrupt)?;
    QuotaRow::from_sqlite(&row)
}

async fn complete_quota(
    transaction: &mut Transaction<'_, Sqlite>,
    current: &QuotaRow,
    used: QualificationQuota,
    now_ms: u64,
) -> QualificationResult<()> {
    if current.state != "HELD" || !used.fits_within(current.reserved) {
        return Err(QualificationError::InvalidTransition);
    }
    let next = QuotaRow {
        used,
        state: "COMPLETED".to_string(),
        revision: current
            .revision
            .checked_add(1)
            .ok_or(QualificationError::InvalidInput)?,
        updated_at_ms: now_ms,
        row_sha256: String::new(),
        ..current.clone()
    };
    update_quota(transaction, current, &next).await
}

async fn release_quota(
    transaction: &mut Transaction<'_, Sqlite>,
    current: &QuotaRow,
    now_ms: u64,
) -> QualificationResult<()> {
    if current.state != "HELD" {
        return Err(QualificationError::InvalidTransition);
    }
    let next = QuotaRow {
        used: QualificationQuota::default(),
        state: "RELEASED".to_string(),
        revision: current
            .revision
            .checked_add(1)
            .ok_or(QualificationError::InvalidInput)?,
        updated_at_ms: now_ms,
        row_sha256: String::new(),
        ..current.clone()
    };
    update_quota(transaction, current, &next).await
}

async fn update_quota(
    transaction: &mut Transaction<'_, Sqlite>,
    current: &QuotaRow,
    next: &QuotaRow,
) -> QualificationResult<()> {
    let digest = next.digest()?.to_string();
    let result = sqlx::query(
        "UPDATE quota_reservations SET used_rpm = ?, used_tpm = ?, used_concurrency = ?, \
            used_day_budget = ?, used_context = ?, state = ?, revision = ?, updated_at_ms = ?, \
            row_sha256 = ? WHERE operation_id = ? AND revision = ? AND state = 'HELD'",
    )
    .bind(to_i64(next.used.rpm)?)
    .bind(to_i64(next.used.tpm)?)
    .bind(to_i64(next.used.concurrency)?)
    .bind(to_i64(next.used.day_budget)?)
    .bind(to_i64(next.used.context)?)
    .bind(&next.state)
    .bind(to_i64(next.revision)?)
    .bind(to_i64(next.updated_at_ms)?)
    .bind(digest)
    .bind(&next.operation_id)
    .bind(to_i64(current.revision)?)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    if result.rows_affected() != 1 {
        return Err(QualificationError::StaleRevision);
    }
    Ok(())
}

async fn insert_attempt(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &DispatchAttemptRow,
) -> QualificationResult<()> {
    sqlx::query(
        "INSERT INTO dispatch_attempts (operation_id, attempt, operation_revision, writer_boot_id, \
            writer_generation, authority_epoch, owner_epoch, generation, fencing_token_sha256, \
            started_at_ms, marker_kind, marker_json, marker_sha256, marked_at_ms, row_sha256) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, NULL, NULL, ?)",
    )
    .bind(&row.operation_id)
    .bind(to_i64(row.attempt)?)
    .bind(to_i64(row.operation_revision)?)
    .bind(&row.writer.boot_id)
    .bind(to_i64(row.writer.generation)?)
    .bind(to_i64(row.fence.authority_epoch)?)
    .bind(to_i64(row.fence.owner_epoch)?)
    .bind(to_i64(row.fence.generation)?)
    .bind(row.fence.fencing_token_sha256.to_string())
    .bind(to_i64(row.started_at_ms)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn load_attempt(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_id: &str,
    attempt: u64,
) -> QualificationResult<DispatchAttemptRow> {
    let row = sqlx::query(ATTEMPT_SELECT)
        .bind(operation_id)
        .bind(to_i64(attempt)?)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(map_sqlx)?
        .ok_or(QualificationError::Corrupt)?;
    DispatchAttemptRow::from_sqlite(&row)
}

async fn update_attempt_marker(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &DispatchAttemptRow,
) -> QualificationResult<()> {
    let result = sqlx::query(
        "UPDATE dispatch_attempts SET marker_kind = ?, marker_json = ?, marker_sha256 = ?, \
            marked_at_ms = ?, row_sha256 = ? WHERE operation_id = ? AND attempt = ? \
            AND marker_sha256 IS NULL",
    )
    .bind(&row.marker_kind)
    .bind(&row.marker_json)
    .bind(&row.marker_sha256)
    .bind(optional_i64(row.marked_at_ms)?)
    .bind(&row.row_sha256)
    .bind(&row.operation_id)
    .bind(to_i64(row.attempt)?)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    if result.rows_affected() != 1 {
        return Err(QualificationError::Conflict);
    }
    Ok(())
}

async fn load_status_observation_sha(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_id: &str,
    status_revision: u64,
) -> QualificationResult<Option<String>> {
    sqlx::query_scalar(
        "SELECT observation_sha256 FROM status_observations \
         WHERE operation_id = ? AND status_revision = ?",
    )
    .bind(operation_id)
    .bind(to_i64(status_revision)?)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(map_sqlx)
}

async fn insert_status_observation(
    transaction: &mut Transaction<'_, Sqlite>,
    observation: &StatusObservation,
    observation_json: &str,
    observation_sha256: &Sha256Digest,
    writer: &WriterIdentity,
) -> QualificationResult<()> {
    sqlx::query(
        "INSERT INTO status_observations (operation_id, status_revision, observed_at_ms, \
            binding_sha256, observation_json, observation_sha256, writer_boot_id, \
            writer_generation, created_at_ms) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&observation.operation_id)
    .bind(to_i64(observation.status_revision)?)
    .bind(to_i64(observation.observed_at_ms)?)
    .bind(observation.binding_sha256.to_string())
    .bind(observation_json)
    .bind(observation_sha256.to_string())
    .bind(&writer.boot_id)
    .bind(to_i64(writer.generation)?)
    .bind(to_i64(observation.observed_at_ms)?)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn insert_outbox(
    transaction: &mut Transaction<'_, Sqlite>,
    operation: &OperationRow,
    event_kind: &str,
    evidence_sha256: &Sha256Digest,
    now_ms: u64,
) -> QualificationResult<()> {
    #[derive(Serialize)]
    struct EventPayload<'a> {
        schema_version: u32,
        qualification_only: bool,
        authority: bool,
        effect_authority: bool,
        production_caller: bool,
        production_writer: bool,
        operator_acceptance: bool,
        promotion: bool,
        g5_allowed: bool,
        execute_allowed: bool,
        operation_id: &'a str,
        state: &'a str,
        revision: u64,
        event_kind: &'a str,
        evidence_sha256: &'a Sha256Digest,
    }
    let payload = EventPayload {
        schema_version: 1,
        qualification_only: true,
        authority: false,
        effect_authority: false,
        production_caller: false,
        production_writer: false,
        operator_acceptance: false,
        promotion: false,
        g5_allowed: false,
        execute_allowed: false,
        operation_id: &operation.operation_id,
        state: operation.state.as_db(),
        revision: operation.revision,
        event_kind,
        evidence_sha256,
    };
    let payload_json =
        serde_json::to_string(&payload).map_err(|_| QualificationError::InvalidInput)?;
    let payload_sha256 = digest_serializable("hepta.authbus.p0.2.outbox-payload.v1", &payload)?;
    let outbox_id = format!("authbus-outbox:v1:{payload_sha256}");
    let idempotency_key = format!(
        "authbus-p0.2:{}:{}:{}",
        operation.operation_id, operation.revision, event_kind
    );

    sqlx::query(
        "INSERT INTO outbox (outbox_id, operation_id, operation_revision, event_kind, \
            idempotency_key, payload_sha256, payload_json, state, ack_sha256, created_at_ms, \
            acked_at_ms, row_sha256) VALUES (?, ?, ?, ?, ?, ?, ?, 'PENDING', NULL, ?, NULL, '')",
    )
    .bind(&outbox_id)
    .bind(&operation.operation_id)
    .bind(to_i64(operation.revision)?)
    .bind(event_kind)
    .bind(&idempotency_key)
    .bind(payload_sha256.to_string())
    .bind(&payload_json)
    .bind(to_i64(now_ms)?)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    let sequence: i64 = sqlx::query_scalar("SELECT last_insert_rowid()")
        .fetch_one(&mut **transaction)
        .await
        .map_err(map_sqlx)?;
    let row = OutboxRow {
        sequence: to_u64(sequence)?,
        outbox_id,
        operation_id: operation.operation_id.clone(),
        operation_revision: operation.revision,
        event_kind: event_kind.to_string(),
        idempotency_key,
        payload_sha256: payload_sha256.to_string(),
        payload_json,
        ack_sha256: None,
        created_at_ms: now_ms,
        acked_at_ms: None,
        state: "PENDING".to_string(),
        row_sha256: String::new(),
    };
    let digest = row.digest()?.to_string();
    sqlx::query("UPDATE outbox SET row_sha256 = ? WHERE sequence = ?")
        .bind(digest)
        .bind(sequence)
        .execute(&mut **transaction)
        .await
        .map_err(map_sqlx)?;
    Ok(())
}

async fn load_outbox(
    transaction: &mut Transaction<'_, Sqlite>,
    outbox_id: &str,
) -> QualificationResult<OutboxRow> {
    let row = sqlx::query(OUTBOX_SELECT)
        .bind(outbox_id)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(map_sqlx)?
        .ok_or(QualificationError::NotFound)?;
    OutboxRow::from_sqlite(&row)
}

async fn append_fsync_receipt(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_id: &str,
    phase: &str,
    operation_revision: u64,
    payload_sha256: &Sha256Digest,
    writer: &WriterIdentity,
    recorded_at_ms: u64,
) -> QualificationResult<(u64, Sha256Digest)> {
    let witness = FsyncReceiptRow {
        sequence: 0,
        operation_id: operation_id.to_string(),
        phase: phase.to_string(),
        operation_revision,
        payload_sha256: payload_sha256.to_string(),
        writer: writer.clone(),
        recorded_at_ms,
        witness_sha256: String::new(),
    };
    let witness_sha256 = witness.digest()?;
    sqlx::query(
        "INSERT INTO fsync_receipts (operation_id, phase, operation_revision, payload_sha256, \
            writer_boot_id, writer_generation, recorded_at_ms, witness_sha256) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(operation_id)
    .bind(phase)
    .bind(to_i64(operation_revision)?)
    .bind(payload_sha256.to_string())
    .bind(&writer.boot_id)
    .bind(to_i64(writer.generation)?)
    .bind(to_i64(recorded_at_ms)?)
    .bind(witness_sha256.to_string())
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    let sequence: i64 = sqlx::query_scalar("SELECT last_insert_rowid()")
        .fetch_one(&mut **transaction)
        .await
        .map_err(map_sqlx)?;
    Ok((to_u64(sequence)?, witness_sha256))
}

async fn verify_state_resources(
    pool: &SqlitePool,
    operation: &OperationRow,
    quota: &QuotaRow,
) -> QualificationResult<()> {
    let claim: Option<i64> = sqlx::query_scalar(
        "SELECT active FROM token_family_claims WHERE operation_id = ?",
    )
    .bind(&operation.operation_id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx)?;
    let active = claim.ok_or(QualificationError::Corrupt)? == 1;
    match operation.state {
        OperationState::Completed => {
            if quota.state != "COMPLETED" || active {
                return Err(QualificationError::Corrupt);
            }
        }
        OperationState::Rejected | OperationState::Quarantined => {
            if quota.state != "RELEASED" || active {
                return Err(QualificationError::Corrupt);
            }
        }
        _ => {
            if quota.state != "HELD" || !active {
                return Err(QualificationError::Corrupt);
            }
        }
    }
    if operation.state != OperationState::IntentDurable {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM dispatch_attempts WHERE operation_id = ?",
        )
        .bind(&operation.operation_id)
        .fetch_one(pool)
        .await
        .map_err(map_sqlx)?;
        if count == 0 {
            return Err(QualificationError::Corrupt);
        }
    }
    Ok(())
}

fn verify_operation_row(row: &OperationRow) -> QualificationResult<()> {
    row.fence.validate()?;
    row.writer.validate()?;
    if row.revision == 0
        || row.created_at_ms == 0
        || row.updated_at_ms < row.created_at_ms
        || row.digest()?.to_string() != row.row_sha256
    {
        return Err(QualificationError::Corrupt);
    }
    let admission: QualificationAdmission =
        serde_json::from_str(&row.intent_json).map_err(|_| QualificationError::Corrupt)?;
    if admission.intent_sha256()?.to_string() != row.intent_sha256
        || admission.intent.operation_id != row.operation_id
        || admission.intent.operation_key != row.operation_key
        || admission.intent.effect_key != row.effect_key
        || admission.intent.idempotency_key != row.idempotency_key
        || admission.intent.provider_id != row.provider_id
        || admission.intent.profile_id != row.profile_id
        || admission.intent.token_family_id != row.token_family_id
        || admission.intent.fence != row.fence
    {
        return Err(QualificationError::Corrupt);
    }
    Ok(())
}

fn verify_quota_row(row: &QuotaRow) -> QualificationResult<()> {
    if !row.used.fits_within(row.reserved)
        || row.revision == 0
        || row.digest()?.to_string() != row.row_sha256
    {
        return Err(QualificationError::Corrupt);
    }
    Ok(())
}

fn verify_attempt_row(row: &DispatchAttemptRow) -> QualificationResult<()> {
    if row.attempt == 0
        || row.operation_revision == 0
        || row.started_at_ms == 0
        || row.digest()?.to_string() != row.row_sha256
    {
        return Err(QualificationError::Corrupt);
    }
    let marker_fields = [
        row.marker_kind.is_some(),
        row.marker_json.is_some(),
        row.marker_sha256.is_some(),
        row.marked_at_ms.is_some(),
    ];
    if marker_fields.iter().any(|value| *value) && marker_fields.iter().any(|value| !*value) {
        return Err(QualificationError::Corrupt);
    }
    Ok(())
}

fn verify_outbox_row(row: &OutboxRow) -> QualificationResult<()> {
    if row.sequence == 0
        || row.operation_revision == 0
        || row.created_at_ms == 0
        || row.digest()?.to_string() != row.row_sha256
    {
        return Err(QualificationError::Corrupt);
    }
    match row.state.as_str() {
        "PENDING" if row.ack_sha256.is_none() && row.acked_at_ms.is_none() => Ok(()),
        "ACKED" if row.ack_sha256.is_some() && row.acked_at_ms.is_some() => Ok(()),
        _ => Err(QualificationError::Corrupt),
    }
}

fn verify_receipt_row(row: &FsyncReceiptRow) -> QualificationResult<()> {
    if row.sequence == 0
        || row.operation_revision == 0
        || row.recorded_at_ms == 0
        || row.digest()?.to_string() != row.witness_sha256
    {
        return Err(QualificationError::Corrupt);
    }
    Ok(())
}

async fn count_query(pool: &SqlitePool, statement: &str) -> QualificationResult<u64> {
    let count: i64 = sqlx::query_scalar(statement)
        .fetch_one(pool)
        .await
        .map_err(map_sqlx)?;
    to_u64(count)
}

fn parse_digest(value: String) -> QualificationResult<Sha256Digest> {
    Sha256Digest::parse(value).map_err(|_| QualificationError::Corrupt)
}

fn to_i64(value: u64) -> QualificationResult<i64> {
    i64::try_from(value).map_err(|_| QualificationError::InvalidInput)
}

fn to_u64(value: i64) -> QualificationResult<u64> {
    u64::try_from(value).map_err(|_| QualificationError::Corrupt)
}

fn to_u32(value: u64) -> QualificationResult<u32> {
    u32::try_from(value).map_err(|_| QualificationError::Corrupt)
}

fn optional_i64(value: Option<u64>) -> QualificationResult<Option<i64>> {
    value.map(to_i64).transpose()
}

fn optional_u64(value: Option<i64>) -> QualificationResult<Option<u64>> {
    value.map(to_u64).transpose()
}

fn map_sqlx(error: sqlx::Error) -> QualificationError {
    if let sqlx::Error::Database(database_error) = &error {
        let code = database_error.code();
        if code.as_deref() == Some("13")
            || database_error.message().contains("database or disk is full")
        {
            return QualificationError::StorageFull;
        }
    }
    QualificationError::StorageUnavailable
}

fn create_private_directory(path: &Path) -> QualificationResult<()> {
    fs::create_dir_all(path).map_err(|_| QualificationError::StorageUnavailable)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .map_err(|_| QualificationError::StorageUnavailable)?;
    }
    Ok(())
}

fn protect_private_file(path: &Path) -> QualificationResult<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .map_err(|_| QualificationError::StorageUnavailable)?;
    }
    Ok(())
}
