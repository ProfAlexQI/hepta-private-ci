use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use codex_hepta_authbus_p1_qualification::P11EvidenceState;
use codex_hepta_authbus_p1_qualification::P11KeyPurpose;
use codex_hepta_authbus_p1_qualification::P11ManualEvidenceDisposition;
use codex_hepta_authbus_p1_qualification::P11ManualEvidenceReceipt;
use codex_hepta_authbus_p1_qualification::P11OperationEvidenceBinding;
use codex_hepta_authbus_p1_qualification::P11ProviderEvidenceDisposition;
use codex_hepta_authbus_p1_qualification::P11ProviderStatusReceipt;
use codex_hepta_authbus_p1_qualification::P11SignedManualEvidence;
use codex_hepta_authbus_p1_qualification::P11SignedProviderStatusEvidence;
use codex_hepta_authbus_p1_qualification::P11VerificationKeyRecord;
use codex_hepta_contracts::Sha256Digest;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use serde::Serialize;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::Transaction;
use sqlx::sqlite::SqliteRow;

use crate::AUTHBUS_P1_2_AUTHORITY;
use crate::AUTHBUS_P1_2_EFFECT_AUTHORITY;
use crate::AUTHBUS_P1_2_EXECUTE_ALLOWED;
use crate::AUTHBUS_P1_2_G5_ALLOWED;
use crate::AUTHBUS_P1_2_LISTENER_ENABLED;
use crate::AUTHBUS_P1_2_OPENBAO_ENABLED;
use crate::AUTHBUS_P1_2_OPERATOR_ACCEPTANCE;
use crate::AUTHBUS_P1_2_PARENT_WORKSPACE_WIRED;
use crate::AUTHBUS_P1_2_PRIVATE_KEY_STORAGE;
use crate::AUTHBUS_P1_2_PRODUCTION_CALLER;
use crate::AUTHBUS_P1_2_PRODUCTION_WRITER;
use crate::AUTHBUS_P1_2_PROMOTION;
use crate::AUTHBUS_P1_2_PROVIDER_CALL_ENABLED;
use crate::AUTHBUS_P1_2_QUALIFICATION_ONLY;
use crate::AUTHBUS_P1_2_RAW_SIGNATURE_STORAGE;
use crate::AUTHBUS_P1_2_SCHEMA_VERSION;
use crate::AUTHBUS_P1_2_SECRET_STORAGE;
use crate::P12Error;
use crate::P12Failpoint;
use crate::P12GcReport;
use crate::P12GcRequest;
use crate::P12IntegrityReport;
use crate::P12ManualObservation;
use crate::P12NonceClaim;
use crate::P12OperationSnapshot;
use crate::P12Policy;
use crate::P12ProviderObservation;
use crate::P12Result;
use crate::P12WriteDisposition;
use crate::P12WriterIdentity;
use crate::digest_serializable;
use crate::evidence_state_name;
use crate::key_purpose_name;
use crate::parse_evidence_state;
use crate::parse_key_purpose;
use crate::validate_digest;
use crate::validate_key_record;
use crate::validate_positive_i64;
use crate::validate_text;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
const DATABASE_FILE: &str = "authbus-p1-2-qualification.sqlite3";

#[derive(Clone)]
pub struct P12Store {
    pool: SqlitePool,
    database_path: PathBuf,
    writer: P12WriterIdentity,
    policy: P12Policy,
    policy_sha256: Sha256Digest,
    failpoints: Arc<AtomicU64>,
}

impl P12Store {
    pub async fn open(
        root: impl AsRef<Path>,
        writer: P12WriterIdentity,
        policy: P12Policy,
        now_unix_seconds: u64,
    ) -> P12Result<Self> {
        writer.validate()?;
        policy.validate()?;
        validate_positive_i64(now_unix_seconds)?;
        let policy_sha256 = policy.digest()?;

        let root = root.as_ref();
        create_private_directory(root)?;
        let root = fs::canonicalize(root).map_err(|_| P12Error::StorageUnavailable)?;
        let absolute_root =
            AbsolutePathBuf::try_from(root.clone()).map_err(|_| P12Error::StorageUnavailable)?;
        let database_path = root.join(DATABASE_FILE);
        let pool = SqliteConfig::from_sqlite_home(absolute_root)
            .open_durable_evidence_pool(&database_path)
            .await
            .map_err(map_sqlx)?;

        sqlx::query("PRAGMA secure_delete = ON")
            .execute(&pool)
            .await
            .map_err(map_sqlx)?;
        sqlx::query("PRAGMA trusted_schema = OFF")
            .execute(&pool)
            .await
            .map_err(map_sqlx)?;
        MIGRATOR
            .run(&pool)
            .await
            .map_err(|_| P12Error::StorageUnavailable)?;
        protect_private_file(&database_path)?;
        initialize_or_rebind_meta(&pool, &writer, policy, &policy_sha256, now_unix_seconds).await?;

        let store = Self {
            pool,
            database_path,
            writer,
            policy,
            policy_sha256,
            failpoints: Arc::new(AtomicU64::new(0)),
        };
        store.verify_integrity().await?;
        Ok(store)
    }

    pub fn database_path(&self) -> &Path {
        &self.database_path
    }

    pub fn writer(&self) -> &P12WriterIdentity {
        &self.writer
    }

    pub fn policy(&self) -> P12Policy {
        self.policy
    }

    pub fn enable_failpoint(&self, failpoint: P12Failpoint) {
        self.failpoints.fetch_or(failpoint.bit(), Ordering::SeqCst);
    }

    pub fn clear_failpoints(&self) {
        self.failpoints.store(0, Ordering::SeqCst);
    }

    pub async fn close(self) {
        self.pool.close().await;
    }

    fn maybe_fail(&self, failpoint: P12Failpoint) -> P12Result<()> {
        if self.failpoints.load(Ordering::SeqCst) & failpoint.bit() == 0 {
            return Ok(());
        }
        if failpoint == P12Failpoint::StorageUnavailableBeforeCommit {
            Err(P12Error::StorageUnavailable)
        } else {
            Err(P12Error::InjectedFailure)
        }
    }

    async fn commit_or_rollback(
        &self,
        transaction: Transaction<'_, Sqlite>,
        failpoint: P12Failpoint,
    ) -> P12Result<()> {
        let failure = self
            .maybe_fail(failpoint)
            .and_then(|_| self.maybe_fail(P12Failpoint::StorageUnavailableBeforeCommit))
            .err();
        if let Some(error) = failure {
            transaction.rollback().await.map_err(map_sqlx)?;
            return Err(error);
        }
        transaction.commit().await.map_err(map_sqlx)
    }

    async fn ensure_current_writer(
        &self,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> P12Result<()> {
        let row = sqlx::query(
            "SELECT writer_boot_id, writer_generation, policy_sha256 \
             FROM authbus_p1_2_meta WHERE singleton = 1",
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(map_sqlx)?
        .ok_or(P12Error::CorruptState)?;
        let boot_id: String = row.try_get("writer_boot_id").map_err(map_sqlx)?;
        let generation = read_u64(&row, "writer_generation")?;
        let policy_sha256: String = row.try_get("policy_sha256").map_err(map_sqlx)?;
        if boot_id != self.writer.boot_id || generation != self.writer.generation {
            return Err(P12Error::StaleWriter);
        }
        if policy_sha256 != self.policy_sha256.as_str() {
            return Err(P12Error::PolicyConflict);
        }
        Ok(())
    }

    pub async fn register_key(
        &self,
        record: P11VerificationKeyRecord,
        now_unix_seconds: u64,
    ) -> P12Result<P12WriteDisposition> {
        validate_key_record(&record)?;
        validate_positive_i64(now_unix_seconds)?;
        let record_sha256 = record.registration_digest()?;
        let record_json = serde_json::to_string(&record).map_err(|_| P12Error::InvalidInput)?;
        let purpose = key_purpose_name(record.purpose);
        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;

        if let Some(existing) = load_key_optional(
            &mut transaction,
            &record.issuer_id,
            &record.key_id,
            record.key_epoch,
        )
        .await?
        {
            existing.verify()?;
            if existing.record_sha256 == record_sha256.as_str()
                && existing.record_json == record_json
            {
                return Ok(P12WriteDisposition::AlreadyPresent);
            }
            return Err(P12Error::KeyConflict);
        }

        if let Some(head) =
            load_key_head_optional(&mut transaction, &record.issuer_id, purpose).await?
        {
            head.verify()?;
            if record.key_epoch < head.current_key_epoch {
                return Err(P12Error::StaleKeyEpoch);
            }
            if record.key_epoch == head.current_key_epoch && record.key_id != head.current_key_id {
                return Err(P12Error::KeyConflict);
            }
        }

        let count = count_rows(&mut transaction, "p12_key_registrations").await?;
        if count >= self.policy.max_key_entries {
            return Err(P12Error::KeyCapacity);
        }

        let row = KeyRow::new(record.clone(), record_json, record_sha256, now_unix_seconds)?;
        insert_key(&mut transaction, &row).await?;
        let head = KeyHeadRow::new(
            record.issuer_id.clone(),
            record.purpose,
            record.key_id.clone(),
            record.key_epoch,
            now_unix_seconds,
        )?;
        upsert_key_head(&mut transaction, &head).await?;
        append_receipt(
            &mut transaction,
            "KEY_REGISTERED",
            &format!(
                "{}:{}:{}",
                record.issuer_id, record.key_id, record.key_epoch
            ),
            record.key_epoch,
            &row.record_sha256_digest()?,
            &self.writer,
            now_unix_seconds,
        )
        .await?;
        self.commit_or_rollback(transaction, P12Failpoint::KeyBeforeCommit)
            .await?;
        Ok(P12WriteDisposition::Applied)
    }

    pub async fn revoke_key(
        &self,
        issuer_id: &str,
        key_id: &str,
        key_epoch: u64,
        revoked_at_unix_seconds: u64,
    ) -> P12Result<P12WriteDisposition> {
        validate_text(issuer_id)?;
        validate_text(key_id)?;
        validate_positive_i64(key_epoch)?;
        validate_positive_i64(revoked_at_unix_seconds)?;
        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;
        let current = load_key_optional(&mut transaction, issuer_id, key_id, key_epoch)
            .await?
            .ok_or(P12Error::UnknownKey)?;
        current.verify()?;
        let mut record = current.record()?;
        if revoked_at_unix_seconds < record.valid_from_unix_seconds {
            return Err(P12Error::InvalidInput);
        }
        if let Some(existing) = record.revoked_at_unix_seconds {
            if existing == revoked_at_unix_seconds {
                return Ok(P12WriteDisposition::AlreadyPresent);
            }
            return Err(P12Error::KeyConflict);
        }
        record.revoked_at_unix_seconds = Some(revoked_at_unix_seconds);
        validate_key_record(&record)?;
        let next = KeyRow::from_existing(record, &current, revoked_at_unix_seconds)?;
        update_key(&mut transaction, &next).await?;
        append_receipt(
            &mut transaction,
            "KEY_REVOKED",
            &format!("{issuer_id}:{key_id}:{key_epoch}"),
            key_epoch,
            &next.record_sha256_digest()?,
            &self.writer,
            revoked_at_unix_seconds,
        )
        .await?;
        self.commit_or_rollback(transaction, P12Failpoint::KeyBeforeCommit)
            .await?;
        Ok(P12WriteDisposition::Applied)
    }

    pub async fn key_record(
        &self,
        issuer_id: &str,
        key_id: &str,
        key_epoch: u64,
    ) -> P12Result<P11VerificationKeyRecord> {
        validate_text(issuer_id)?;
        validate_text(key_id)?;
        validate_positive_i64(key_epoch)?;
        let row = sqlx::query(
            "SELECT * FROM p12_key_registrations \
             WHERE issuer_id = ? AND key_id = ? AND key_epoch = ?",
        )
        .bind(issuer_id)
        .bind(key_id)
        .bind(to_i64(key_epoch)?)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?
        .ok_or(P12Error::UnknownKey)?;
        let row = KeyRow::from_sqlite(&row)?;
        row.verify()?;
        row.record()
    }

    pub async fn claim_nonce(&self, claim: P12NonceClaim) -> P12Result<P12WriteDisposition> {
        claim.validate()?;
        let claim_sha256 = claim.digest()?;
        let claim_json = serde_json::to_string(&claim).map_err(|_| P12Error::InvalidInput)?;
        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;
        require_durable_key(
            &mut transaction,
            &claim.issuer_id,
            &claim.key_id,
            claim.key_epoch,
            P11KeyPurpose::IdentityIssuer,
            claim.claimed_at_unix_seconds,
        )
        .await?;
        let exists: Option<i64> =
            sqlx::query_scalar("SELECT 1 FROM p12_nonce_claims WHERE nonce_key_sha256 = ?")
                .bind(claim.nonce_key_sha256.as_str())
                .fetch_optional(&mut *transaction)
                .await
                .map_err(map_sqlx)?;
        if exists.is_some() {
            return Err(P12Error::NonceReplay);
        }
        let live_count = count_live_nonces(&mut transaction, claim.claimed_at_unix_seconds).await?;
        if live_count >= self.policy.max_nonce_entries {
            return Err(P12Error::NonceCapacity);
        }
        let row = NonceRow::new(claim, claim_json, claim_sha256)?;
        insert_nonce(&mut transaction, &row).await?;
        append_receipt(
            &mut transaction,
            "NONCE_CLAIMED",
            &row.nonce_key_sha256,
            1,
            &row.claim_sha256_digest()?,
            &self.writer,
            row.claimed_at_unix_seconds,
        )
        .await?;
        self.commit_or_rollback(transaction, P12Failpoint::NonceBeforeCommit)
            .await?;
        Ok(P12WriteDisposition::Applied)
    }

    pub async fn key_count(&self) -> P12Result<u64> {
        count_rows_pool(&self.pool, "p12_key_registrations").await
    }

    pub async fn nonce_count(&self) -> P12Result<u64> {
        count_rows_pool(&self.pool, "p12_nonce_claims").await
    }

    pub async fn register_operation(
        &self,
        binding: P11OperationEvidenceBinding,
        now_unix_seconds: u64,
    ) -> P12Result<P12WriteDisposition> {
        binding.validate()?;
        validate_positive_i64(now_unix_seconds)?;
        let binding_sha256 = binding.digest()?;
        let binding_json = serde_json::to_string(&binding).map_err(|_| P12Error::InvalidInput)?;
        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;
        if let Some(existing) =
            load_operation_optional(&mut transaction, &binding.operation_id).await?
        {
            existing.verify()?;
            if existing.binding_sha256 == binding_sha256.as_str()
                && existing.binding_json == binding_json
            {
                return Ok(P12WriteDisposition::AlreadyPresent);
            }
            return Err(P12Error::OperationConflict);
        }
        let count = count_rows(&mut transaction, "p12_operations").await?;
        if count >= self.policy.max_operation_entries {
            return Err(P12Error::OperationCapacity);
        }
        let row = OperationRow::new(binding, binding_json, binding_sha256, now_unix_seconds)?;
        insert_operation(&mut transaction, &row).await?;
        append_receipt(
            &mut transaction,
            "OPERATION_REGISTERED",
            &row.operation_id,
            row.record_revision,
            &row.binding_sha256_digest()?,
            &self.writer,
            now_unix_seconds,
        )
        .await?;
        self.commit_or_rollback(transaction, P12Failpoint::OperationBeforeCommit)
            .await?;
        Ok(P12WriteDisposition::Applied)
    }

    pub async fn append_provider_status(
        &self,
        evidence: &P11SignedProviderStatusEvidence,
        receipt: &P11ProviderStatusReceipt,
        now_unix_seconds: u64,
    ) -> P12Result<P11ProviderEvidenceDisposition> {
        validate_positive_i64(now_unix_seconds)?;
        let observation = P12ProviderObservation::from_verified(evidence, receipt)?;
        if observation.observed_at_unix_seconds > now_unix_seconds {
            return Err(P12Error::InvalidInput);
        }
        let observation_sha256 = observation.digest()?;
        let observation_json =
            serde_json::to_string(&observation).map_err(|_| P12Error::InvalidInput)?;
        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;
        require_durable_key(
            &mut transaction,
            &observation.issuer_id,
            &observation.key_id,
            observation.key_epoch,
            P11KeyPurpose::ProviderStatusIssuer,
            observation.observed_at_unix_seconds,
        )
        .await?;
        let current = load_operation_optional(&mut transaction, &observation.operation_id)
            .await?
            .ok_or(P12Error::UnknownOperation)?;
        current.verify()?;
        let binding = current.binding()?;
        verify_provider_binding(&binding, &observation)?;

        if let Some(existing) = load_status_optional(
            &mut transaction,
            &observation.operation_id,
            observation.status_revision,
        )
        .await?
        {
            existing.verify()?;
            if existing.evidence_sha256 == observation.evidence_sha256.as_str()
                && existing.observation_sha256 == observation_sha256.as_str()
            {
                return Ok(P11ProviderEvidenceDisposition::AlreadyPresent(
                    existing.provider_receipt()?,
                ));
            }
            return Err(P12Error::EvidenceConflict);
        }
        if tombstone_exists(&mut transaction, &observation.operation_id).await? {
            return Err(P12Error::TerminalImmutable);
        }
        if current.state.is_terminal() {
            return Err(P12Error::TerminalImmutable);
        }
        if current.state == P11EvidenceState::ManualRequired {
            return Err(P12Error::ManualEvidenceRequired);
        }
        match current.last_status_revision {
            Some(revision) if observation.status_revision <= revision => {
                return Err(P12Error::StaleObservation);
            }
            None if observation.status_revision != 1 => {
                return Err(P12Error::StaleObservation);
            }
            _ => {}
        }
        if current
            .last_observed_at_unix_seconds
            .is_some_and(|observed_at| observation.observed_at_unix_seconds < observed_at)
        {
            return Err(P12Error::StaleObservation);
        }

        let status_row = StatusRow::new(observation, observation_json, observation_sha256)?;
        insert_status(&mut transaction, &status_row).await?;
        let status_head = StatusHeadRow::from_status(&status_row)?;
        upsert_status_head(&mut transaction, &status_head).await?;
        let next = current.transition_provider(&status_row, now_unix_seconds)?;
        update_operation(&mut transaction, &next, current.record_revision).await?;
        if next.state.is_terminal() {
            let tombstone = TombstoneRow::new(
                next.operation_id.clone(),
                "PROVIDER_STATUS",
                next.state,
                status_row.evidence_sha256_digest()?,
                status_row.observed_at_unix_seconds,
                status_row
                    .observed_at_unix_seconds
                    .checked_add(self.policy.terminal_retention_seconds)
                    .ok_or(P12Error::InvalidInput)?,
            )?;
            insert_tombstone(&mut transaction, &tombstone).await?;
        }
        append_receipt(
            &mut transaction,
            "STATUS_APPENDED",
            &status_row.operation_id,
            status_row.status_revision,
            &status_row.evidence_sha256_digest()?,
            &self.writer,
            now_unix_seconds,
        )
        .await?;
        self.commit_or_rollback(transaction, P12Failpoint::StatusBeforeCommit)
            .await?;
        Ok(P11ProviderEvidenceDisposition::Applied(
            status_row.provider_receipt()?,
        ))
    }

    pub async fn append_manual_evidence(
        &self,
        evidence: &P11SignedManualEvidence,
        receipt: &P11ManualEvidenceReceipt,
        now_unix_seconds: u64,
    ) -> P12Result<P11ManualEvidenceDisposition> {
        validate_positive_i64(now_unix_seconds)?;
        let observation = P12ManualObservation::from_verified(evidence, receipt)?;
        if observation.observed_at_unix_seconds > now_unix_seconds {
            return Err(P12Error::InvalidInput);
        }
        let observation_sha256 = observation.digest()?;
        let observation_json =
            serde_json::to_string(&observation).map_err(|_| P12Error::InvalidInput)?;
        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;
        require_durable_key(
            &mut transaction,
            &observation.issuer_id,
            &observation.key_id,
            observation.key_epoch,
            P11KeyPurpose::OperatorEvidenceIssuer,
            observation.observed_at_unix_seconds,
        )
        .await?;
        let current = load_operation_optional(&mut transaction, &observation.operation_id)
            .await?
            .ok_or(P12Error::UnknownOperation)?;
        current.verify()?;
        let binding = current.binding()?;
        verify_manual_binding(&binding, &observation)?;

        if let Some(existing) = load_manual_optional(
            &mut transaction,
            &observation.operation_id,
            observation.manual_revision,
        )
        .await?
        {
            existing.verify()?;
            if existing.evidence_sha256 == observation.evidence_sha256.as_str()
                && existing.observation_sha256 == observation_sha256.as_str()
            {
                return Ok(P11ManualEvidenceDisposition::AlreadyPresent(
                    existing.manual_receipt()?,
                ));
            }
            return Err(P12Error::EvidenceConflict);
        }
        if tombstone_exists(&mut transaction, &observation.operation_id).await? {
            return Err(P12Error::TerminalImmutable);
        }
        if current.state.is_terminal() {
            return Err(P12Error::TerminalImmutable);
        }
        if current.state != P11EvidenceState::ManualRequired {
            return Err(P12Error::InvalidManualTransition);
        }
        if current
            .last_manual_revision
            .is_some_and(|revision| observation.manual_revision <= revision)
        {
            return Err(P12Error::StaleObservation);
        }
        if current
            .last_observed_at_unix_seconds
            .is_some_and(|observed_at| observation.observed_at_unix_seconds < observed_at)
        {
            return Err(P12Error::StaleObservation);
        }

        let manual_row = ManualRow::new(observation, observation_json, observation_sha256)?;
        insert_manual(&mut transaction, &manual_row).await?;
        let manual_head = ManualHeadRow::from_manual(&manual_row)?;
        upsert_manual_head(&mut transaction, &manual_head).await?;
        let next = current.transition_manual(&manual_row, now_unix_seconds)?;
        update_operation(&mut transaction, &next, current.record_revision).await?;
        if next.state.is_terminal() {
            let tombstone = TombstoneRow::new(
                next.operation_id.clone(),
                "MANUAL_EVIDENCE",
                next.state,
                manual_row.evidence_sha256_digest()?,
                manual_row.observed_at_unix_seconds,
                manual_row
                    .observed_at_unix_seconds
                    .checked_add(self.policy.terminal_retention_seconds)
                    .ok_or(P12Error::InvalidInput)?,
            )?;
            insert_tombstone(&mut transaction, &tombstone).await?;
        }
        append_receipt(
            &mut transaction,
            "MANUAL_APPENDED",
            &manual_row.operation_id,
            manual_row.manual_revision,
            &manual_row.evidence_sha256_digest()?,
            &self.writer,
            now_unix_seconds,
        )
        .await?;
        self.commit_or_rollback(transaction, P12Failpoint::ManualBeforeCommit)
            .await?;
        Ok(P11ManualEvidenceDisposition::Applied(
            manual_row.manual_receipt()?,
        ))
    }

    pub async fn operation_snapshot(&self, operation_id: &str) -> P12Result<P12OperationSnapshot> {
        validate_text(operation_id)?;
        let row = sqlx::query("SELECT * FROM p12_operations WHERE operation_id = ?")
            .bind(operation_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx)?
            .ok_or(P12Error::UnknownOperation)?;
        let operation = OperationRow::from_sqlite(&row)?;
        operation.verify()?;
        let tombstone = sqlx::query(
            "SELECT retain_until_unix_seconds FROM p12_terminal_tombstones \
             WHERE operation_id = ?",
        )
        .bind(operation_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx)?;
        let terminal_retain_until_unix_seconds = tombstone
            .as_ref()
            .map(|row| read_u64(row, "retain_until_unix_seconds"))
            .transpose()?;
        operation.snapshot(terminal_retain_until_unix_seconds)
    }

    pub async fn operation_count(&self) -> P12Result<u64> {
        count_rows_pool(&self.pool, "p12_operations").await
    }

    pub async fn status_evidence_count(&self) -> P12Result<u64> {
        count_rows_pool(&self.pool, "p12_status_evidence").await
    }

    pub async fn manual_evidence_count(&self) -> P12Result<u64> {
        count_rows_pool(&self.pool, "p12_manual_evidence").await
    }

    pub async fn gc_revision(&self) -> P12Result<u64> {
        let row = sqlx::query("SELECT revision FROM p12_gc_cursor WHERE singleton = 1")
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx)?
            .ok_or(P12Error::CorruptState)?;
        read_u64(&row, "revision")
    }

    pub async fn collect_garbage(&self, request: P12GcRequest) -> P12Result<P12GcReport> {
        request.validate()?;
        let mut transaction = self.pool.begin().await.map_err(map_sqlx)?;
        self.ensure_current_writer(&mut transaction).await?;
        let cursor = load_gc_cursor(&mut transaction).await?;
        cursor.verify()?;
        if cursor.revision != request.expected_revision {
            return Err(P12Error::GcConflict);
        }
        let evidence_cutoff = request
            .now_unix_seconds
            .saturating_sub(self.policy.evidence_retention_seconds);
        let key_cutoff = request
            .now_unix_seconds
            .saturating_sub(self.policy.key_retention_seconds);
        let limit = to_i64(request.max_rows)?;

        let nonce_rows_deleted = sqlx::query(
            "DELETE FROM p12_nonce_claims WHERE nonce_key_sha256 IN (\
                 SELECT nonce_key_sha256 FROM p12_nonce_claims \
                 WHERE expires_at_unix_seconds <= ? \
                 ORDER BY expires_at_unix_seconds, nonce_key_sha256 LIMIT ?\
             )",
        )
        .bind(to_i64(request.now_unix_seconds)?)
        .bind(limit)
        .execute(&mut *transaction)
        .await
        .map_err(map_sqlx)?
        .rows_affected();

        let status_rows_deleted = sqlx::query(
            "DELETE FROM p12_status_evidence WHERE (operation_id, status_revision) IN (\
                 SELECT e.operation_id, e.status_revision FROM p12_status_evidence e \
                 LEFT JOIN p12_status_heads h \
                   ON h.operation_id = e.operation_id AND h.status_revision = e.status_revision \
                 WHERE e.observed_at_unix_seconds < ? AND h.operation_id IS NULL \
                 ORDER BY e.observed_at_unix_seconds, e.operation_id, e.status_revision LIMIT ?\
             )",
        )
        .bind(to_i64(evidence_cutoff)?)
        .bind(limit)
        .execute(&mut *transaction)
        .await
        .map_err(map_sqlx)?
        .rows_affected();

        let manual_rows_deleted = sqlx::query(
            "DELETE FROM p12_manual_evidence WHERE (operation_id, manual_revision) IN (\
                 SELECT e.operation_id, e.manual_revision FROM p12_manual_evidence e \
                 LEFT JOIN p12_manual_heads h \
                   ON h.operation_id = e.operation_id AND h.manual_revision = e.manual_revision \
                 WHERE e.observed_at_unix_seconds < ? AND h.operation_id IS NULL \
                 ORDER BY e.observed_at_unix_seconds, e.operation_id, e.manual_revision LIMIT ?\
             )",
        )
        .bind(to_i64(evidence_cutoff)?)
        .bind(limit)
        .execute(&mut *transaction)
        .await
        .map_err(map_sqlx)?
        .rows_affected();

        let key_rows_deleted = sqlx::query(
            "DELETE FROM p12_key_registrations WHERE (issuer_id, key_id, key_epoch) IN (\
                 SELECT k.issuer_id, k.key_id, k.key_epoch \
                 FROM p12_key_registrations k \
                 LEFT JOIN p12_key_heads h \
                   ON h.issuer_id = k.issuer_id AND h.current_key_id = k.key_id \
                  AND h.current_key_epoch = k.key_epoch \
                 WHERE k.revoked_at_unix_seconds IS NOT NULL \
                   AND k.revoked_at_unix_seconds <= ? AND h.issuer_id IS NULL \
                 ORDER BY k.revoked_at_unix_seconds, k.issuer_id, k.key_epoch LIMIT ?\
             )",
        )
        .bind(to_i64(key_cutoff)?)
        .bind(limit)
        .execute(&mut *transaction)
        .await
        .map_err(map_sqlx)?
        .rows_affected();

        let receipt_rows_deleted = sqlx::query(
            "DELETE FROM p12_durable_receipts WHERE sequence IN (\
                 SELECT sequence FROM p12_durable_receipts \
                 WHERE recorded_at_unix_seconds < ? \
                 ORDER BY recorded_at_unix_seconds, sequence LIMIT ?\
             )",
        )
        .bind(to_i64(evidence_cutoff)?)
        .bind(limit)
        .execute(&mut *transaction)
        .await
        .map_err(map_sqlx)?
        .rows_affected();

        let terminal_operation_ids: Vec<String> = sqlx::query_scalar(
            "SELECT operation_id FROM p12_terminal_tombstones \
             WHERE retain_until_unix_seconds <= ? \
             ORDER BY retain_until_unix_seconds, operation_id LIMIT ?",
        )
        .bind(to_i64(request.now_unix_seconds)?)
        .bind(limit)
        .fetch_all(&mut *transaction)
        .await
        .map_err(map_sqlx)?;
        let mut terminal_operations_deleted = 0_u64;
        for operation_id in terminal_operation_ids {
            terminal_operations_deleted = terminal_operations_deleted
                .checked_add(
                    sqlx::query("DELETE FROM p12_operations WHERE operation_id = ?")
                        .bind(operation_id)
                        .execute(&mut *transaction)
                        .await
                        .map_err(map_sqlx)?
                        .rows_affected(),
                )
                .ok_or(P12Error::CorruptState)?;
        }

        let after_revision = cursor
            .revision
            .checked_add(1)
            .ok_or(P12Error::InvalidInput)?;
        let next_cursor = GcCursorRow::new(after_revision, request.now_unix_seconds)?;
        update_gc_cursor(&mut transaction, &next_cursor, cursor.revision).await?;
        let report = P12GcReport {
            before_revision: cursor.revision,
            after_revision,
            nonce_rows_deleted,
            status_rows_deleted,
            manual_rows_deleted,
            key_rows_deleted,
            receipt_rows_deleted,
            terminal_operations_deleted,
            authority: false,
        };
        let report_sha256 = digest_serializable(
            "hepta.authbus.p1.2.gc-report.v1",
            &GcReportWitness::from(&report),
        )?;
        append_receipt(
            &mut transaction,
            "GC_COMMITTED",
            "gc-cursor",
            after_revision,
            &report_sha256,
            &self.writer,
            request.now_unix_seconds,
        )
        .await?;
        self.commit_or_rollback(transaction, P12Failpoint::GcBeforeCommit)
            .await?;
        Ok(report)
    }

    pub async fn verify_integrity(&self) -> P12Result<P12IntegrityReport> {
        let quick_check: String = sqlx::query_scalar("PRAGMA quick_check")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx)?;
        if quick_check != "ok" {
            return Err(P12Error::CorruptState);
        }
        let journal_mode: String = sqlx::query_scalar("PRAGMA journal_mode")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx)?;
        if !journal_mode.eq_ignore_ascii_case("wal") {
            return Err(P12Error::CorruptState);
        }
        let synchronous: i64 = sqlx::query_scalar("PRAGMA synchronous")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx)?;
        if synchronous != 2 {
            return Err(P12Error::CorruptState);
        }
        verify_meta(&self.pool, &self.policy_sha256).await?;

        let key_rows = verify_all_keys(&self.pool).await?;
        verify_all_key_heads(&self.pool).await?;
        let nonce_rows = verify_all_nonces(&self.pool).await?;
        let operation_rows = verify_all_operations(&self.pool).await?;
        let status_rows = verify_all_status(&self.pool).await?;
        verify_all_status_heads(&self.pool).await?;
        let manual_rows = verify_all_manual(&self.pool).await?;
        verify_all_manual_heads(&self.pool).await?;
        let tombstone_rows = verify_all_tombstones(&self.pool).await?;
        verify_operation_head_consistency(&self.pool).await?;
        let receipt_rows = verify_all_receipts(&self.pool).await?;
        let cursor = load_gc_cursor_pool(&self.pool).await?;
        cursor.verify()?;

        Ok(P12IntegrityReport {
            key_rows,
            nonce_rows,
            operation_rows,
            status_rows,
            manual_rows,
            tombstone_rows,
            receipt_rows,
            gc_revision: cursor.revision,
            authority: false,
        })
    }

    #[doc(hidden)]
    pub async fn qualification_corrupt_nonce_row(&self) -> P12Result<()> {
        sqlx::query(
            "UPDATE p12_nonce_claims SET claim_sha256 = ? \
             WHERE nonce_key_sha256 = (SELECT nonce_key_sha256 FROM p12_nonce_claims LIMIT 1)",
        )
        .bind(Sha256Digest::for_bytes(b"qualification-corruption").as_str())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }
}

#[derive(Serialize)]
struct GcReportWitness {
    before_revision: u64,
    after_revision: u64,
    nonce_rows_deleted: u64,
    status_rows_deleted: u64,
    manual_rows_deleted: u64,
    key_rows_deleted: u64,
    receipt_rows_deleted: u64,
    terminal_operations_deleted: u64,
}

impl From<&P12GcReport> for GcReportWitness {
    fn from(value: &P12GcReport) -> Self {
        Self {
            before_revision: value.before_revision,
            after_revision: value.after_revision,
            nonce_rows_deleted: value.nonce_rows_deleted,
            status_rows_deleted: value.status_rows_deleted,
            manual_rows_deleted: value.manual_rows_deleted,
            key_rows_deleted: value.key_rows_deleted,
            receipt_rows_deleted: value.receipt_rows_deleted,
            terminal_operations_deleted: value.terminal_operations_deleted,
        }
    }
}

async fn initialize_or_rebind_meta(
    pool: &SqlitePool,
    writer: &P12WriterIdentity,
    policy: P12Policy,
    policy_sha256: &Sha256Digest,
    now_unix_seconds: u64,
) -> P12Result<()> {
    let policy_json = serde_json::to_string(&policy).map_err(|_| P12Error::InvalidInput)?;
    let mut transaction = pool.begin().await.map_err(map_sqlx)?;
    let existing = sqlx::query(
        "SELECT writer_boot_id, writer_generation, policy_sha256 \
         FROM authbus_p1_2_meta WHERE singleton = 1",
    )
    .fetch_optional(&mut *transaction)
    .await
    .map_err(map_sqlx)?;
    match existing {
        None => {
            sqlx::query(
                "INSERT INTO authbus_p1_2_meta (\
                     singleton, schema_version, qualification_only, authority, effect_authority, \
                     production_caller, production_writer, operator_acceptance, promotion, \
                     g5_allowed, execute_allowed, listener_enabled, provider_call_enabled, \
                     openbao_enabled, private_key_storage, raw_signature_storage, secret_storage, \
                     parent_workspace_wired, writer_boot_id, writer_generation, policy_json, \
                     policy_sha256, created_at_unix_seconds, updated_at_unix_seconds\
                 ) VALUES (1, ?, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ?, ?, ?, ?, ?, ?)",
            )
            .bind(i64::from(AUTHBUS_P1_2_SCHEMA_VERSION))
            .bind(&writer.boot_id)
            .bind(to_i64(writer.generation)?)
            .bind(&policy_json)
            .bind(policy_sha256.as_str())
            .bind(to_i64(now_unix_seconds)?)
            .bind(to_i64(now_unix_seconds)?)
            .execute(&mut *transaction)
            .await
            .map_err(map_sqlx)?;
            let cursor = GcCursorRow::new(0, now_unix_seconds)?;
            insert_gc_cursor(&mut transaction, &cursor).await?;
        }
        Some(row) => {
            let current_boot: String = row.try_get("writer_boot_id").map_err(map_sqlx)?;
            let current_generation = read_u64(&row, "writer_generation")?;
            let current_policy: String = row.try_get("policy_sha256").map_err(map_sqlx)?;
            if current_policy != policy_sha256.as_str() {
                return Err(P12Error::PolicyConflict);
            }
            if writer.generation < current_generation
                || (writer.generation == current_generation && writer.boot_id != current_boot)
            {
                return Err(P12Error::StaleWriter);
            }
            if writer.generation > current_generation {
                sqlx::query(
                    "UPDATE authbus_p1_2_meta SET writer_boot_id = ?, writer_generation = ?, \
                     updated_at_unix_seconds = ? WHERE singleton = 1 AND writer_generation = ?",
                )
                .bind(&writer.boot_id)
                .bind(to_i64(writer.generation)?)
                .bind(to_i64(now_unix_seconds)?)
                .bind(to_i64(current_generation)?)
                .execute(&mut *transaction)
                .await
                .map_err(map_sqlx)?;
                append_receipt(
                    &mut transaction,
                    "WRITER_REBOUND",
                    "meta",
                    writer.generation,
                    policy_sha256,
                    writer,
                    now_unix_seconds,
                )
                .await?;
            }
        }
    }
    transaction.commit().await.map_err(map_sqlx)
}

fn bool_u64(value: bool) -> u64 {
    if value { 1 } else { 0 }
}

async fn verify_meta(pool: &SqlitePool, expected_policy_sha256: &Sha256Digest) -> P12Result<()> {
    let row = sqlx::query("SELECT * FROM authbus_p1_2_meta WHERE singleton = 1")
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?
        .ok_or(P12Error::CorruptState)?;
    if read_u64(&row, "schema_version")? != u64::from(AUTHBUS_P1_2_SCHEMA_VERSION)
        || read_u64(&row, "qualification_only")? != bool_u64(AUTHBUS_P1_2_QUALIFICATION_ONLY)
        || read_u64(&row, "authority")? != bool_u64(AUTHBUS_P1_2_AUTHORITY)
        || read_u64(&row, "effect_authority")? != bool_u64(AUTHBUS_P1_2_EFFECT_AUTHORITY)
        || read_u64(&row, "production_caller")? != bool_u64(AUTHBUS_P1_2_PRODUCTION_CALLER)
        || read_u64(&row, "production_writer")? != bool_u64(AUTHBUS_P1_2_PRODUCTION_WRITER)
        || read_u64(&row, "operator_acceptance")? != bool_u64(AUTHBUS_P1_2_OPERATOR_ACCEPTANCE)
        || read_u64(&row, "promotion")? != bool_u64(AUTHBUS_P1_2_PROMOTION)
        || read_u64(&row, "g5_allowed")? != bool_u64(AUTHBUS_P1_2_G5_ALLOWED)
        || read_u64(&row, "execute_allowed")? != bool_u64(AUTHBUS_P1_2_EXECUTE_ALLOWED)
        || read_u64(&row, "listener_enabled")? != bool_u64(AUTHBUS_P1_2_LISTENER_ENABLED)
        || read_u64(&row, "provider_call_enabled")? != bool_u64(AUTHBUS_P1_2_PROVIDER_CALL_ENABLED)
        || read_u64(&row, "openbao_enabled")? != bool_u64(AUTHBUS_P1_2_OPENBAO_ENABLED)
        || read_u64(&row, "private_key_storage")? != bool_u64(AUTHBUS_P1_2_PRIVATE_KEY_STORAGE)
        || read_u64(&row, "raw_signature_storage")? != bool_u64(AUTHBUS_P1_2_RAW_SIGNATURE_STORAGE)
        || read_u64(&row, "secret_storage")? != bool_u64(AUTHBUS_P1_2_SECRET_STORAGE)
        || read_u64(&row, "parent_workspace_wired")?
            != bool_u64(AUTHBUS_P1_2_PARENT_WORKSPACE_WIRED)
    {
        return Err(P12Error::CorruptState);
    }
    let policy_json: String = row.try_get("policy_json").map_err(map_sqlx)?;
    let policy: P12Policy =
        serde_json::from_str(&policy_json).map_err(|_| P12Error::CorruptState)?;
    policy.validate().map_err(|_| P12Error::CorruptState)?;
    let policy_sha256: String = row.try_get("policy_sha256").map_err(map_sqlx)?;
    if policy
        .digest()
        .map_err(|_| P12Error::CorruptState)?
        .as_str()
        != policy_sha256
        || policy_sha256 != expected_policy_sha256.as_str()
    {
        return Err(P12Error::CorruptState);
    }
    let writer = P12WriterIdentity {
        boot_id: row.try_get("writer_boot_id").map_err(map_sqlx)?,
        generation: read_u64(&row, "writer_generation")?,
    };
    writer.validate().map_err(|_| P12Error::CorruptState)
}

#[derive(Clone, Debug)]
struct KeyRow {
    issuer_id: String,
    key_id: String,
    key_epoch: u64,
    purpose: P11KeyPurpose,
    record_json: String,
    record_sha256: String,
    revoked_at_unix_seconds: Option<u64>,
    created_at_unix_seconds: u64,
    updated_at_unix_seconds: u64,
    row_sha256: String,
}

impl KeyRow {
    fn new(
        record: P11VerificationKeyRecord,
        record_json: String,
        record_sha256: Sha256Digest,
        now_unix_seconds: u64,
    ) -> P12Result<Self> {
        let mut row = Self {
            issuer_id: record.issuer_id.clone(),
            key_id: record.key_id.clone(),
            key_epoch: record.key_epoch,
            purpose: record.purpose,
            record_json,
            record_sha256: record_sha256.as_str().to_owned(),
            revoked_at_unix_seconds: record.revoked_at_unix_seconds,
            created_at_unix_seconds: now_unix_seconds,
            updated_at_unix_seconds: now_unix_seconds,
            row_sha256: String::new(),
        };
        row.row_sha256 = row.digest()?.as_str().to_owned();
        Ok(row)
    }

    fn from_existing(
        record: P11VerificationKeyRecord,
        current: &Self,
        now_unix_seconds: u64,
    ) -> P12Result<Self> {
        let record_sha256 = record.registration_digest()?;
        let record_json = serde_json::to_string(&record).map_err(|_| P12Error::InvalidInput)?;
        let mut row = Self {
            issuer_id: current.issuer_id.clone(),
            key_id: current.key_id.clone(),
            key_epoch: current.key_epoch,
            purpose: current.purpose,
            record_json,
            record_sha256: record_sha256.as_str().to_owned(),
            revoked_at_unix_seconds: record.revoked_at_unix_seconds,
            created_at_unix_seconds: current.created_at_unix_seconds,
            updated_at_unix_seconds: now_unix_seconds,
            row_sha256: String::new(),
        };
        row.row_sha256 = row.digest()?.as_str().to_owned();
        Ok(row)
    }

    fn from_sqlite(row: &SqliteRow) -> P12Result<Self> {
        Ok(Self {
            issuer_id: row.try_get("issuer_id").map_err(map_sqlx)?,
            key_id: row.try_get("key_id").map_err(map_sqlx)?,
            key_epoch: read_u64(row, "key_epoch")?,
            purpose: parse_key_purpose(&row.try_get::<String, _>("purpose").map_err(map_sqlx)?)?,
            record_json: row.try_get("record_json").map_err(map_sqlx)?,
            record_sha256: row.try_get("record_sha256").map_err(map_sqlx)?,
            revoked_at_unix_seconds: read_optional_u64(row, "revoked_at_unix_seconds")?,
            created_at_unix_seconds: read_u64(row, "created_at_unix_seconds")?,
            updated_at_unix_seconds: read_u64(row, "updated_at_unix_seconds")?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn record(&self) -> P12Result<P11VerificationKeyRecord> {
        let record: P11VerificationKeyRecord =
            serde_json::from_str(&self.record_json).map_err(|_| P12Error::CorruptState)?;
        validate_key_record(&record).map_err(|_| P12Error::CorruptState)?;
        if record.issuer_id != self.issuer_id
            || record.key_id != self.key_id
            || record.key_epoch != self.key_epoch
            || record.purpose != self.purpose
            || record.revoked_at_unix_seconds != self.revoked_at_unix_seconds
            || record
                .registration_digest()
                .map_err(|_| P12Error::CorruptState)?
                .as_str()
                != self.record_sha256
        {
            return Err(P12Error::CorruptState);
        }
        Ok(record)
    }

    fn digest(&self) -> P12Result<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            issuer_id: &'a str,
            key_id: &'a str,
            key_epoch: u64,
            purpose: &'a str,
            record_sha256: &'a str,
            revoked_at_unix_seconds: Option<u64>,
            created_at_unix_seconds: u64,
            updated_at_unix_seconds: u64,
        }
        digest_serializable(
            "hepta.authbus.p1.2.key-row.v1",
            &Witness {
                issuer_id: &self.issuer_id,
                key_id: &self.key_id,
                key_epoch: self.key_epoch,
                purpose: key_purpose_name(self.purpose),
                record_sha256: &self.record_sha256,
                revoked_at_unix_seconds: self.revoked_at_unix_seconds,
                created_at_unix_seconds: self.created_at_unix_seconds,
                updated_at_unix_seconds: self.updated_at_unix_seconds,
            },
        )
    }

    fn verify(&self) -> P12Result<()> {
        self.record()?;
        if self.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.row_sha256 {
            return Err(P12Error::CorruptState);
        }
        Ok(())
    }

    fn record_sha256_digest(&self) -> P12Result<Sha256Digest> {
        parse_digest(&self.record_sha256)
    }
}

#[derive(Clone, Debug)]
struct KeyHeadRow {
    issuer_id: String,
    purpose: P11KeyPurpose,
    current_key_id: String,
    current_key_epoch: u64,
    updated_at_unix_seconds: u64,
    row_sha256: String,
}

impl KeyHeadRow {
    fn new(
        issuer_id: String,
        purpose: P11KeyPurpose,
        current_key_id: String,
        current_key_epoch: u64,
        updated_at_unix_seconds: u64,
    ) -> P12Result<Self> {
        let mut row = Self {
            issuer_id,
            purpose,
            current_key_id,
            current_key_epoch,
            updated_at_unix_seconds,
            row_sha256: String::new(),
        };
        row.row_sha256 = row.digest()?.as_str().to_owned();
        Ok(row)
    }

    fn from_sqlite(row: &SqliteRow) -> P12Result<Self> {
        Ok(Self {
            issuer_id: row.try_get("issuer_id").map_err(map_sqlx)?,
            purpose: parse_key_purpose(&row.try_get::<String, _>("purpose").map_err(map_sqlx)?)?,
            current_key_id: row.try_get("current_key_id").map_err(map_sqlx)?,
            current_key_epoch: read_u64(row, "current_key_epoch")?,
            updated_at_unix_seconds: read_u64(row, "updated_at_unix_seconds")?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn digest(&self) -> P12Result<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            issuer_id: &'a str,
            purpose: &'a str,
            current_key_id: &'a str,
            current_key_epoch: u64,
            updated_at_unix_seconds: u64,
        }
        digest_serializable(
            "hepta.authbus.p1.2.key-head.v1",
            &Witness {
                issuer_id: &self.issuer_id,
                purpose: key_purpose_name(self.purpose),
                current_key_id: &self.current_key_id,
                current_key_epoch: self.current_key_epoch,
                updated_at_unix_seconds: self.updated_at_unix_seconds,
            },
        )
    }

    fn verify(&self) -> P12Result<()> {
        validate_text(&self.issuer_id).map_err(|_| P12Error::CorruptState)?;
        validate_text(&self.current_key_id).map_err(|_| P12Error::CorruptState)?;
        validate_positive_i64(self.current_key_epoch).map_err(|_| P12Error::CorruptState)?;
        if self.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.row_sha256 {
            return Err(P12Error::CorruptState);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct NonceRow {
    nonce_key_sha256: String,
    claim_json: String,
    claim_sha256: String,
    evidence_sha256: String,
    binding_sha256: String,
    subject_sha256: String,
    nonce_sha256: String,
    launch_nonce_sha256: String,
    expires_at_unix_seconds: u64,
    claimed_at_unix_seconds: u64,
    row_sha256: String,
}

impl NonceRow {
    fn new(
        claim: P12NonceClaim,
        claim_json: String,
        claim_sha256: Sha256Digest,
    ) -> P12Result<Self> {
        let mut row = Self {
            nonce_key_sha256: claim.nonce_key_sha256.as_str().to_owned(),
            claim_json,
            claim_sha256: claim_sha256.as_str().to_owned(),
            evidence_sha256: claim.evidence_sha256.as_str().to_owned(),
            binding_sha256: claim.binding_sha256.as_str().to_owned(),
            subject_sha256: claim.subject_sha256.as_str().to_owned(),
            nonce_sha256: claim.nonce_sha256.as_str().to_owned(),
            launch_nonce_sha256: claim.launch_nonce_sha256.as_str().to_owned(),
            expires_at_unix_seconds: claim.expires_at_unix_seconds,
            claimed_at_unix_seconds: claim.claimed_at_unix_seconds,
            row_sha256: String::new(),
        };
        row.row_sha256 = row.digest()?.as_str().to_owned();
        Ok(row)
    }

    fn from_sqlite(row: &SqliteRow) -> P12Result<Self> {
        Ok(Self {
            nonce_key_sha256: row.try_get("nonce_key_sha256").map_err(map_sqlx)?,
            claim_json: row.try_get("claim_json").map_err(map_sqlx)?,
            claim_sha256: row.try_get("claim_sha256").map_err(map_sqlx)?,
            evidence_sha256: row.try_get("evidence_sha256").map_err(map_sqlx)?,
            binding_sha256: row.try_get("binding_sha256").map_err(map_sqlx)?,
            subject_sha256: row.try_get("subject_sha256").map_err(map_sqlx)?,
            nonce_sha256: row.try_get("nonce_sha256").map_err(map_sqlx)?,
            launch_nonce_sha256: row.try_get("launch_nonce_sha256").map_err(map_sqlx)?,
            expires_at_unix_seconds: read_u64(row, "expires_at_unix_seconds")?,
            claimed_at_unix_seconds: read_u64(row, "claimed_at_unix_seconds")?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn claim(&self) -> P12Result<P12NonceClaim> {
        let claim: P12NonceClaim =
            serde_json::from_str(&self.claim_json).map_err(|_| P12Error::CorruptState)?;
        claim.validate().map_err(|_| P12Error::CorruptState)?;
        if claim.nonce_key_sha256.as_str() != self.nonce_key_sha256
            || claim.evidence_sha256.as_str() != self.evidence_sha256
            || claim.binding_sha256.as_str() != self.binding_sha256
            || claim.subject_sha256.as_str() != self.subject_sha256
            || claim.nonce_sha256.as_str() != self.nonce_sha256
            || claim.launch_nonce_sha256.as_str() != self.launch_nonce_sha256
            || claim.expires_at_unix_seconds != self.expires_at_unix_seconds
            || claim.claimed_at_unix_seconds != self.claimed_at_unix_seconds
            || claim.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.claim_sha256
        {
            return Err(P12Error::CorruptState);
        }
        Ok(claim)
    }

    fn digest(&self) -> P12Result<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            nonce_key_sha256: &'a str,
            claim_sha256: &'a str,
            evidence_sha256: &'a str,
            binding_sha256: &'a str,
            subject_sha256: &'a str,
            nonce_sha256: &'a str,
            launch_nonce_sha256: &'a str,
            expires_at_unix_seconds: u64,
            claimed_at_unix_seconds: u64,
        }
        digest_serializable(
            "hepta.authbus.p1.2.nonce-row.v1",
            &Witness {
                nonce_key_sha256: &self.nonce_key_sha256,
                claim_sha256: &self.claim_sha256,
                evidence_sha256: &self.evidence_sha256,
                binding_sha256: &self.binding_sha256,
                subject_sha256: &self.subject_sha256,
                nonce_sha256: &self.nonce_sha256,
                launch_nonce_sha256: &self.launch_nonce_sha256,
                expires_at_unix_seconds: self.expires_at_unix_seconds,
                claimed_at_unix_seconds: self.claimed_at_unix_seconds,
            },
        )
    }

    fn verify(&self) -> P12Result<()> {
        self.claim()?;
        if self.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.row_sha256 {
            return Err(P12Error::CorruptState);
        }
        Ok(())
    }

    fn claim_sha256_digest(&self) -> P12Result<Sha256Digest> {
        parse_digest(&self.claim_sha256)
    }
}

#[derive(Clone, Debug)]
struct OperationRow {
    operation_id: String,
    binding_json: String,
    binding_sha256: String,
    state: P11EvidenceState,
    last_status_revision: Option<u64>,
    last_manual_revision: Option<u64>,
    last_status_sha256: Option<String>,
    last_manual_sha256: Option<String>,
    last_observed_at_unix_seconds: Option<u64>,
    record_revision: u64,
    created_at_unix_seconds: u64,
    updated_at_unix_seconds: u64,
    row_sha256: String,
}

impl OperationRow {
    fn new(
        binding: P11OperationEvidenceBinding,
        binding_json: String,
        binding_sha256: Sha256Digest,
        now_unix_seconds: u64,
    ) -> P12Result<Self> {
        let mut row = Self {
            operation_id: binding.operation_id,
            binding_json,
            binding_sha256: binding_sha256.as_str().to_owned(),
            state: P11EvidenceState::Pending,
            last_status_revision: None,
            last_manual_revision: None,
            last_status_sha256: None,
            last_manual_sha256: None,
            last_observed_at_unix_seconds: None,
            record_revision: 1,
            created_at_unix_seconds: now_unix_seconds,
            updated_at_unix_seconds: now_unix_seconds,
            row_sha256: String::new(),
        };
        row.row_sha256 = row.digest()?.as_str().to_owned();
        Ok(row)
    }

    fn from_sqlite(row: &SqliteRow) -> P12Result<Self> {
        Ok(Self {
            operation_id: row.try_get("operation_id").map_err(map_sqlx)?,
            binding_json: row.try_get("binding_json").map_err(map_sqlx)?,
            binding_sha256: row.try_get("binding_sha256").map_err(map_sqlx)?,
            state: parse_evidence_state(&row.try_get::<String, _>("state").map_err(map_sqlx)?)?,
            last_status_revision: read_optional_u64(row, "last_status_revision")?,
            last_manual_revision: read_optional_u64(row, "last_manual_revision")?,
            last_status_sha256: row.try_get("last_status_sha256").map_err(map_sqlx)?,
            last_manual_sha256: row.try_get("last_manual_sha256").map_err(map_sqlx)?,
            last_observed_at_unix_seconds: read_optional_u64(row, "last_observed_at_unix_seconds")?,
            record_revision: read_u64(row, "record_revision")?,
            created_at_unix_seconds: read_u64(row, "created_at_unix_seconds")?,
            updated_at_unix_seconds: read_u64(row, "updated_at_unix_seconds")?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn binding(&self) -> P12Result<P11OperationEvidenceBinding> {
        let binding: P11OperationEvidenceBinding =
            serde_json::from_str(&self.binding_json).map_err(|_| P12Error::CorruptState)?;
        binding.validate().map_err(|_| P12Error::CorruptState)?;
        if binding.operation_id != self.operation_id
            || binding
                .digest()
                .map_err(|_| P12Error::CorruptState)?
                .as_str()
                != self.binding_sha256
        {
            return Err(P12Error::CorruptState);
        }
        Ok(binding)
    }

    fn transition_provider(&self, status: &StatusRow, now_unix_seconds: u64) -> P12Result<Self> {
        let mut next = self.clone();
        next.state = status.state;
        next.last_status_revision = Some(status.status_revision);
        next.last_status_sha256 = Some(status.evidence_sha256.clone());
        next.last_observed_at_unix_seconds = Some(status.observed_at_unix_seconds);
        next.record_revision = self
            .record_revision
            .checked_add(1)
            .ok_or(P12Error::InvalidInput)?;
        next.updated_at_unix_seconds = now_unix_seconds;
        next.row_sha256 = next.digest()?.as_str().to_owned();
        Ok(next)
    }

    fn transition_manual(&self, manual: &ManualRow, now_unix_seconds: u64) -> P12Result<Self> {
        let mut next = self.clone();
        next.state = manual.state;
        next.last_manual_revision = Some(manual.manual_revision);
        next.last_manual_sha256 = Some(manual.evidence_sha256.clone());
        next.last_observed_at_unix_seconds = Some(manual.observed_at_unix_seconds);
        next.record_revision = self
            .record_revision
            .checked_add(1)
            .ok_or(P12Error::InvalidInput)?;
        next.updated_at_unix_seconds = now_unix_seconds;
        next.row_sha256 = next.digest()?.as_str().to_owned();
        Ok(next)
    }

    fn digest(&self) -> P12Result<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            operation_id: &'a str,
            binding_sha256: &'a str,
            state: &'a str,
            last_status_revision: Option<u64>,
            last_manual_revision: Option<u64>,
            last_status_sha256: &'a Option<String>,
            last_manual_sha256: &'a Option<String>,
            last_observed_at_unix_seconds: Option<u64>,
            record_revision: u64,
            created_at_unix_seconds: u64,
            updated_at_unix_seconds: u64,
        }
        digest_serializable(
            "hepta.authbus.p1.2.operation-row.v1",
            &Witness {
                operation_id: &self.operation_id,
                binding_sha256: &self.binding_sha256,
                state: evidence_state_name(self.state),
                last_status_revision: self.last_status_revision,
                last_manual_revision: self.last_manual_revision,
                last_status_sha256: &self.last_status_sha256,
                last_manual_sha256: &self.last_manual_sha256,
                last_observed_at_unix_seconds: self.last_observed_at_unix_seconds,
                record_revision: self.record_revision,
                created_at_unix_seconds: self.created_at_unix_seconds,
                updated_at_unix_seconds: self.updated_at_unix_seconds,
            },
        )
    }

    fn verify(&self) -> P12Result<()> {
        self.binding()?;
        validate_positive_i64(self.record_revision).map_err(|_| P12Error::CorruptState)?;
        if self.last_status_revision.is_some() != self.last_status_sha256.is_some()
            || self.last_manual_revision.is_some() != self.last_manual_sha256.is_some()
            || self
                .last_status_sha256
                .as_ref()
                .is_some_and(|value| parse_digest(value).is_err())
            || self
                .last_manual_sha256
                .as_ref()
                .is_some_and(|value| parse_digest(value).is_err())
            || self.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.row_sha256
        {
            return Err(P12Error::CorruptState);
        }
        Ok(())
    }

    fn binding_sha256_digest(&self) -> P12Result<Sha256Digest> {
        parse_digest(&self.binding_sha256)
    }

    fn snapshot(
        &self,
        terminal_retain_until_unix_seconds: Option<u64>,
    ) -> P12Result<P12OperationSnapshot> {
        Ok(P12OperationSnapshot {
            binding: self.binding()?,
            state: self.state,
            last_status_revision: self.last_status_revision,
            last_manual_revision: self.last_manual_revision,
            last_status_sha256: self
                .last_status_sha256
                .as_deref()
                .map(parse_digest)
                .transpose()?,
            last_manual_sha256: self
                .last_manual_sha256
                .as_deref()
                .map(parse_digest)
                .transpose()?,
            last_observed_at_unix_seconds: self.last_observed_at_unix_seconds,
            record_revision: self.record_revision,
            terminal_retain_until_unix_seconds,
            authority: false,
        })
    }
}

#[derive(Clone, Debug)]
struct StatusRow {
    operation_id: String,
    status_revision: u64,
    observation_json: String,
    observation_sha256: String,
    evidence_sha256: String,
    state: P11EvidenceState,
    observed_at_unix_seconds: u64,
    row_sha256: String,
}

impl StatusRow {
    fn new(
        observation: P12ProviderObservation,
        observation_json: String,
        observation_sha256: Sha256Digest,
    ) -> P12Result<Self> {
        let mut row = Self {
            operation_id: observation.operation_id.clone(),
            status_revision: observation.status_revision,
            observation_json,
            observation_sha256: observation_sha256.as_str().to_owned(),
            evidence_sha256: observation.evidence_sha256.as_str().to_owned(),
            state: observation.state,
            observed_at_unix_seconds: observation.observed_at_unix_seconds,
            row_sha256: String::new(),
        };
        row.row_sha256 = row.digest()?.as_str().to_owned();
        Ok(row)
    }

    fn from_sqlite(row: &SqliteRow) -> P12Result<Self> {
        Ok(Self {
            operation_id: row.try_get("operation_id").map_err(map_sqlx)?,
            status_revision: read_u64(row, "status_revision")?,
            observation_json: row.try_get("observation_json").map_err(map_sqlx)?,
            observation_sha256: row.try_get("observation_sha256").map_err(map_sqlx)?,
            evidence_sha256: row.try_get("evidence_sha256").map_err(map_sqlx)?,
            state: parse_evidence_state(&row.try_get::<String, _>("state").map_err(map_sqlx)?)?,
            observed_at_unix_seconds: read_u64(row, "observed_at_unix_seconds")?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn observation(&self) -> P12Result<P12ProviderObservation> {
        let observation: P12ProviderObservation =
            serde_json::from_str(&self.observation_json).map_err(|_| P12Error::CorruptState)?;
        observation.validate().map_err(|_| P12Error::CorruptState)?;
        if observation.operation_id != self.operation_id
            || observation.status_revision != self.status_revision
            || observation.observed_at_unix_seconds != self.observed_at_unix_seconds
            || observation.state != self.state
            || observation.evidence_sha256.as_str() != self.evidence_sha256
            || observation
                .digest()
                .map_err(|_| P12Error::CorruptState)?
                .as_str()
                != self.observation_sha256
        {
            return Err(P12Error::CorruptState);
        }
        Ok(observation)
    }

    fn digest(&self) -> P12Result<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            operation_id: &'a str,
            status_revision: u64,
            observation_sha256: &'a str,
            evidence_sha256: &'a str,
            state: &'a str,
            observed_at_unix_seconds: u64,
        }
        digest_serializable(
            "hepta.authbus.p1.2.status-row.v1",
            &Witness {
                operation_id: &self.operation_id,
                status_revision: self.status_revision,
                observation_sha256: &self.observation_sha256,
                evidence_sha256: &self.evidence_sha256,
                state: evidence_state_name(self.state),
                observed_at_unix_seconds: self.observed_at_unix_seconds,
            },
        )
    }

    fn verify(&self) -> P12Result<()> {
        self.observation()?;
        if self.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.row_sha256 {
            return Err(P12Error::CorruptState);
        }
        Ok(())
    }

    fn evidence_sha256_digest(&self) -> P12Result<Sha256Digest> {
        parse_digest(&self.evidence_sha256)
    }

    fn provider_receipt(&self) -> P12Result<P11ProviderStatusReceipt> {
        Ok(P11ProviderStatusReceipt {
            evidence_sha256: self.evidence_sha256_digest()?,
            operation_id: self.operation_id.clone(),
            status_revision: self.status_revision,
            observed_at_unix_seconds: self.observed_at_unix_seconds,
            state: self.state,
            authority: false,
        })
    }
}

#[derive(Clone, Debug)]
struct StatusHeadRow {
    operation_id: String,
    status_revision: u64,
    evidence_sha256: String,
    state: P11EvidenceState,
    observed_at_unix_seconds: u64,
    row_sha256: String,
}

impl StatusHeadRow {
    fn from_status(status: &StatusRow) -> P12Result<Self> {
        let mut row = Self {
            operation_id: status.operation_id.clone(),
            status_revision: status.status_revision,
            evidence_sha256: status.evidence_sha256.clone(),
            state: status.state,
            observed_at_unix_seconds: status.observed_at_unix_seconds,
            row_sha256: String::new(),
        };
        row.row_sha256 = row.digest()?.as_str().to_owned();
        Ok(row)
    }

    fn from_sqlite(row: &SqliteRow) -> P12Result<Self> {
        Ok(Self {
            operation_id: row.try_get("operation_id").map_err(map_sqlx)?,
            status_revision: read_u64(row, "status_revision")?,
            evidence_sha256: row.try_get("evidence_sha256").map_err(map_sqlx)?,
            state: parse_evidence_state(&row.try_get::<String, _>("state").map_err(map_sqlx)?)?,
            observed_at_unix_seconds: read_u64(row, "observed_at_unix_seconds")?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn digest(&self) -> P12Result<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            operation_id: &'a str,
            status_revision: u64,
            evidence_sha256: &'a str,
            state: &'a str,
            observed_at_unix_seconds: u64,
        }
        digest_serializable(
            "hepta.authbus.p1.2.status-head.v1",
            &Witness {
                operation_id: &self.operation_id,
                status_revision: self.status_revision,
                evidence_sha256: &self.evidence_sha256,
                state: evidence_state_name(self.state),
                observed_at_unix_seconds: self.observed_at_unix_seconds,
            },
        )
    }

    fn verify(&self) -> P12Result<()> {
        parse_digest(&self.evidence_sha256).map_err(|_| P12Error::CorruptState)?;
        if self.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.row_sha256 {
            return Err(P12Error::CorruptState);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct ManualRow {
    operation_id: String,
    manual_revision: u64,
    observation_json: String,
    observation_sha256: String,
    evidence_sha256: String,
    state: P11EvidenceState,
    observed_at_unix_seconds: u64,
    row_sha256: String,
}

impl ManualRow {
    fn new(
        observation: P12ManualObservation,
        observation_json: String,
        observation_sha256: Sha256Digest,
    ) -> P12Result<Self> {
        let mut row = Self {
            operation_id: observation.operation_id.clone(),
            manual_revision: observation.manual_revision,
            observation_json,
            observation_sha256: observation_sha256.as_str().to_owned(),
            evidence_sha256: observation.evidence_sha256.as_str().to_owned(),
            state: observation.state,
            observed_at_unix_seconds: observation.observed_at_unix_seconds,
            row_sha256: String::new(),
        };
        row.row_sha256 = row.digest()?.as_str().to_owned();
        Ok(row)
    }

    fn from_sqlite(row: &SqliteRow) -> P12Result<Self> {
        Ok(Self {
            operation_id: row.try_get("operation_id").map_err(map_sqlx)?,
            manual_revision: read_u64(row, "manual_revision")?,
            observation_json: row.try_get("observation_json").map_err(map_sqlx)?,
            observation_sha256: row.try_get("observation_sha256").map_err(map_sqlx)?,
            evidence_sha256: row.try_get("evidence_sha256").map_err(map_sqlx)?,
            state: parse_evidence_state(&row.try_get::<String, _>("state").map_err(map_sqlx)?)?,
            observed_at_unix_seconds: read_u64(row, "observed_at_unix_seconds")?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn observation(&self) -> P12Result<P12ManualObservation> {
        let observation: P12ManualObservation =
            serde_json::from_str(&self.observation_json).map_err(|_| P12Error::CorruptState)?;
        observation.validate().map_err(|_| P12Error::CorruptState)?;
        if observation.operation_id != self.operation_id
            || observation.manual_revision != self.manual_revision
            || observation.observed_at_unix_seconds != self.observed_at_unix_seconds
            || observation.state != self.state
            || observation.evidence_sha256.as_str() != self.evidence_sha256
            || observation
                .digest()
                .map_err(|_| P12Error::CorruptState)?
                .as_str()
                != self.observation_sha256
        {
            return Err(P12Error::CorruptState);
        }
        Ok(observation)
    }

    fn digest(&self) -> P12Result<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            operation_id: &'a str,
            manual_revision: u64,
            observation_sha256: &'a str,
            evidence_sha256: &'a str,
            state: &'a str,
            observed_at_unix_seconds: u64,
        }
        digest_serializable(
            "hepta.authbus.p1.2.manual-row.v1",
            &Witness {
                operation_id: &self.operation_id,
                manual_revision: self.manual_revision,
                observation_sha256: &self.observation_sha256,
                evidence_sha256: &self.evidence_sha256,
                state: evidence_state_name(self.state),
                observed_at_unix_seconds: self.observed_at_unix_seconds,
            },
        )
    }

    fn verify(&self) -> P12Result<()> {
        self.observation()?;
        if self.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.row_sha256 {
            return Err(P12Error::CorruptState);
        }
        Ok(())
    }

    fn evidence_sha256_digest(&self) -> P12Result<Sha256Digest> {
        parse_digest(&self.evidence_sha256)
    }

    fn manual_receipt(&self) -> P12Result<P11ManualEvidenceReceipt> {
        Ok(P11ManualEvidenceReceipt {
            evidence_sha256: self.evidence_sha256_digest()?,
            operation_id: self.operation_id.clone(),
            manual_revision: self.manual_revision,
            observed_at_unix_seconds: self.observed_at_unix_seconds,
            state: self.state,
            authority: false,
        })
    }
}

#[derive(Clone, Debug)]
struct ManualHeadRow {
    operation_id: String,
    manual_revision: u64,
    evidence_sha256: String,
    state: P11EvidenceState,
    observed_at_unix_seconds: u64,
    row_sha256: String,
}

impl ManualHeadRow {
    fn from_manual(manual: &ManualRow) -> P12Result<Self> {
        let mut row = Self {
            operation_id: manual.operation_id.clone(),
            manual_revision: manual.manual_revision,
            evidence_sha256: manual.evidence_sha256.clone(),
            state: manual.state,
            observed_at_unix_seconds: manual.observed_at_unix_seconds,
            row_sha256: String::new(),
        };
        row.row_sha256 = row.digest()?.as_str().to_owned();
        Ok(row)
    }

    fn from_sqlite(row: &SqliteRow) -> P12Result<Self> {
        Ok(Self {
            operation_id: row.try_get("operation_id").map_err(map_sqlx)?,
            manual_revision: read_u64(row, "manual_revision")?,
            evidence_sha256: row.try_get("evidence_sha256").map_err(map_sqlx)?,
            state: parse_evidence_state(&row.try_get::<String, _>("state").map_err(map_sqlx)?)?,
            observed_at_unix_seconds: read_u64(row, "observed_at_unix_seconds")?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn digest(&self) -> P12Result<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            operation_id: &'a str,
            manual_revision: u64,
            evidence_sha256: &'a str,
            state: &'a str,
            observed_at_unix_seconds: u64,
        }
        digest_serializable(
            "hepta.authbus.p1.2.manual-head.v1",
            &Witness {
                operation_id: &self.operation_id,
                manual_revision: self.manual_revision,
                evidence_sha256: &self.evidence_sha256,
                state: evidence_state_name(self.state),
                observed_at_unix_seconds: self.observed_at_unix_seconds,
            },
        )
    }

    fn verify(&self) -> P12Result<()> {
        parse_digest(&self.evidence_sha256).map_err(|_| P12Error::CorruptState)?;
        if self.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.row_sha256 {
            return Err(P12Error::CorruptState);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct TombstoneRow {
    operation_id: String,
    source_kind: String,
    terminal_state: P11EvidenceState,
    evidence_sha256: String,
    terminal_at_unix_seconds: u64,
    retain_until_unix_seconds: u64,
    row_sha256: String,
}

impl TombstoneRow {
    fn new(
        operation_id: String,
        source_kind: &str,
        terminal_state: P11EvidenceState,
        evidence_sha256: Sha256Digest,
        terminal_at_unix_seconds: u64,
        retain_until_unix_seconds: u64,
    ) -> P12Result<Self> {
        if !terminal_state.is_terminal()
            || !matches!(source_kind, "PROVIDER_STATUS" | "MANUAL_EVIDENCE")
            || retain_until_unix_seconds < terminal_at_unix_seconds
        {
            return Err(P12Error::InvalidInput);
        }
        let mut row = Self {
            operation_id,
            source_kind: source_kind.to_owned(),
            terminal_state,
            evidence_sha256: evidence_sha256.as_str().to_owned(),
            terminal_at_unix_seconds,
            retain_until_unix_seconds,
            row_sha256: String::new(),
        };
        row.row_sha256 = row.digest()?.as_str().to_owned();
        Ok(row)
    }

    fn from_sqlite(row: &SqliteRow) -> P12Result<Self> {
        Ok(Self {
            operation_id: row.try_get("operation_id").map_err(map_sqlx)?,
            source_kind: row.try_get("source_kind").map_err(map_sqlx)?,
            terminal_state: parse_evidence_state(
                &row.try_get::<String, _>("terminal_state")
                    .map_err(map_sqlx)?,
            )?,
            evidence_sha256: row.try_get("evidence_sha256").map_err(map_sqlx)?,
            terminal_at_unix_seconds: read_u64(row, "terminal_at_unix_seconds")?,
            retain_until_unix_seconds: read_u64(row, "retain_until_unix_seconds")?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn digest(&self) -> P12Result<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            operation_id: &'a str,
            source_kind: &'a str,
            terminal_state: &'a str,
            evidence_sha256: &'a str,
            terminal_at_unix_seconds: u64,
            retain_until_unix_seconds: u64,
        }
        digest_serializable(
            "hepta.authbus.p1.2.terminal-tombstone.v1",
            &Witness {
                operation_id: &self.operation_id,
                source_kind: &self.source_kind,
                terminal_state: evidence_state_name(self.terminal_state),
                evidence_sha256: &self.evidence_sha256,
                terminal_at_unix_seconds: self.terminal_at_unix_seconds,
                retain_until_unix_seconds: self.retain_until_unix_seconds,
            },
        )
    }

    fn verify(&self) -> P12Result<()> {
        if !self.terminal_state.is_terminal()
            || !matches!(
                self.source_kind.as_str(),
                "PROVIDER_STATUS" | "MANUAL_EVIDENCE"
            )
            || self.retain_until_unix_seconds < self.terminal_at_unix_seconds
            || parse_digest(&self.evidence_sha256).is_err()
            || self.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.row_sha256
        {
            return Err(P12Error::CorruptState);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct ReceiptRow {
    event_kind: String,
    subject_id: String,
    subject_revision: u64,
    payload_sha256: String,
    writer: P12WriterIdentity,
    recorded_at_unix_seconds: u64,
    witness_sha256: String,
}

impl ReceiptRow {
    fn from_sqlite(row: &SqliteRow) -> P12Result<Self> {
        Ok(Self {
            event_kind: row.try_get("event_kind").map_err(map_sqlx)?,
            subject_id: row.try_get("subject_id").map_err(map_sqlx)?,
            subject_revision: read_u64(row, "subject_revision")?,
            payload_sha256: row.try_get("payload_sha256").map_err(map_sqlx)?,
            writer: P12WriterIdentity {
                boot_id: row.try_get("writer_boot_id").map_err(map_sqlx)?,
                generation: read_u64(row, "writer_generation")?,
            },
            recorded_at_unix_seconds: read_u64(row, "recorded_at_unix_seconds")?,
            witness_sha256: row.try_get("witness_sha256").map_err(map_sqlx)?,
        })
    }

    fn digest(&self) -> P12Result<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness<'a> {
            event_kind: &'a str,
            subject_id: &'a str,
            subject_revision: u64,
            payload_sha256: &'a str,
            writer: &'a P12WriterIdentity,
            recorded_at_unix_seconds: u64,
        }
        digest_serializable(
            "hepta.authbus.p1.2.durable-receipt.v1",
            &Witness {
                event_kind: &self.event_kind,
                subject_id: &self.subject_id,
                subject_revision: self.subject_revision,
                payload_sha256: &self.payload_sha256,
                writer: &self.writer,
                recorded_at_unix_seconds: self.recorded_at_unix_seconds,
            },
        )
    }

    fn verify(&self) -> P12Result<()> {
        validate_text(&self.event_kind).map_err(|_| P12Error::CorruptState)?;
        validate_text(&self.subject_id).map_err(|_| P12Error::CorruptState)?;
        self.writer.validate().map_err(|_| P12Error::CorruptState)?;
        parse_digest(&self.payload_sha256).map_err(|_| P12Error::CorruptState)?;
        if self.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.witness_sha256 {
            return Err(P12Error::CorruptState);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct GcCursorRow {
    revision: u64,
    last_run_at_unix_seconds: u64,
    row_sha256: String,
}

impl GcCursorRow {
    fn new(revision: u64, last_run_at_unix_seconds: u64) -> P12Result<Self> {
        let mut row = Self {
            revision,
            last_run_at_unix_seconds,
            row_sha256: String::new(),
        };
        row.row_sha256 = row.digest()?.as_str().to_owned();
        Ok(row)
    }

    fn from_sqlite(row: &SqliteRow) -> P12Result<Self> {
        Ok(Self {
            revision: read_u64(row, "revision")?,
            last_run_at_unix_seconds: read_u64(row, "last_run_at_unix_seconds")?,
            row_sha256: row.try_get("row_sha256").map_err(map_sqlx)?,
        })
    }

    fn digest(&self) -> P12Result<Sha256Digest> {
        #[derive(Serialize)]
        struct Witness {
            revision: u64,
            last_run_at_unix_seconds: u64,
        }
        digest_serializable(
            "hepta.authbus.p1.2.gc-cursor.v1",
            &Witness {
                revision: self.revision,
                last_run_at_unix_seconds: self.last_run_at_unix_seconds,
            },
        )
    }

    fn verify(&self) -> P12Result<()> {
        validate_positive_i64(self.last_run_at_unix_seconds).map_err(|_| P12Error::CorruptState)?;
        if self.digest().map_err(|_| P12Error::CorruptState)?.as_str() != self.row_sha256 {
            return Err(P12Error::CorruptState);
        }
        Ok(())
    }
}

async fn require_durable_key(
    transaction: &mut Transaction<'_, Sqlite>,
    issuer_id: &str,
    key_id: &str,
    key_epoch: u64,
    purpose: P11KeyPurpose,
    observed_at_unix_seconds: u64,
) -> P12Result<KeyRow> {
    let row = load_key_optional(transaction, issuer_id, key_id, key_epoch)
        .await?
        .ok_or(P12Error::UnknownKey)?;
    row.verify()?;
    let record = row.record()?;
    if row.purpose != purpose {
        return Err(P12Error::BindingMismatch);
    }
    if observed_at_unix_seconds < record.valid_from_unix_seconds
        || observed_at_unix_seconds >= record.valid_until_unix_seconds
        || record
            .revoked_at_unix_seconds
            .is_some_and(|revoked_at| revoked_at <= observed_at_unix_seconds)
    {
        return Err(P12Error::UnknownKey);
    }
    Ok(row)
}

async fn load_key_optional(
    transaction: &mut Transaction<'_, Sqlite>,
    issuer_id: &str,
    key_id: &str,
    key_epoch: u64,
) -> P12Result<Option<KeyRow>> {
    sqlx::query(
        "SELECT * FROM p12_key_registrations \
         WHERE issuer_id = ? AND key_id = ? AND key_epoch = ?",
    )
    .bind(issuer_id)
    .bind(key_id)
    .bind(to_i64(key_epoch)?)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(map_sqlx)?
    .as_ref()
    .map(KeyRow::from_sqlite)
    .transpose()
}

async fn insert_key(transaction: &mut Transaction<'_, Sqlite>, row: &KeyRow) -> P12Result<()> {
    sqlx::query(
        "INSERT INTO p12_key_registrations (\
             issuer_id, key_id, key_epoch, purpose, record_json, record_sha256, \
             revoked_at_unix_seconds, created_at_unix_seconds, updated_at_unix_seconds, row_sha256\
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&row.issuer_id)
    .bind(&row.key_id)
    .bind(to_i64(row.key_epoch)?)
    .bind(key_purpose_name(row.purpose))
    .bind(&row.record_json)
    .bind(&row.record_sha256)
    .bind(optional_i64(row.revoked_at_unix_seconds)?)
    .bind(to_i64(row.created_at_unix_seconds)?)
    .bind(to_i64(row.updated_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn update_key(transaction: &mut Transaction<'_, Sqlite>, row: &KeyRow) -> P12Result<()> {
    let result = sqlx::query(
        "UPDATE p12_key_registrations SET record_json = ?, record_sha256 = ?, \
         revoked_at_unix_seconds = ?, updated_at_unix_seconds = ?, row_sha256 = ? \
         WHERE issuer_id = ? AND key_id = ? AND key_epoch = ?",
    )
    .bind(&row.record_json)
    .bind(&row.record_sha256)
    .bind(optional_i64(row.revoked_at_unix_seconds)?)
    .bind(to_i64(row.updated_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .bind(&row.issuer_id)
    .bind(&row.key_id)
    .bind(to_i64(row.key_epoch)?)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    if result.rows_affected() != 1 {
        return Err(P12Error::UnknownKey);
    }
    Ok(())
}

async fn load_key_head_optional(
    transaction: &mut Transaction<'_, Sqlite>,
    issuer_id: &str,
    purpose: &str,
) -> P12Result<Option<KeyHeadRow>> {
    sqlx::query("SELECT * FROM p12_key_heads WHERE issuer_id = ? AND purpose = ?")
        .bind(issuer_id)
        .bind(purpose)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(map_sqlx)?
        .as_ref()
        .map(KeyHeadRow::from_sqlite)
        .transpose()
}

async fn upsert_key_head(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &KeyHeadRow,
) -> P12Result<()> {
    sqlx::query(
        "INSERT INTO p12_key_heads (\
             issuer_id, purpose, current_key_id, current_key_epoch, updated_at_unix_seconds, row_sha256\
         ) VALUES (?, ?, ?, ?, ?, ?) \
         ON CONFLICT(issuer_id, purpose) DO UPDATE SET \
             current_key_id = excluded.current_key_id, \
             current_key_epoch = excluded.current_key_epoch, \
             updated_at_unix_seconds = excluded.updated_at_unix_seconds, \
             row_sha256 = excluded.row_sha256",
    )
    .bind(&row.issuer_id)
    .bind(key_purpose_name(row.purpose))
    .bind(&row.current_key_id)
    .bind(to_i64(row.current_key_epoch)?)
    .bind(to_i64(row.updated_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn insert_nonce(transaction: &mut Transaction<'_, Sqlite>, row: &NonceRow) -> P12Result<()> {
    sqlx::query(
        "INSERT INTO p12_nonce_claims (\
             nonce_key_sha256, claim_json, claim_sha256, evidence_sha256, binding_sha256, \
             subject_sha256, nonce_sha256, launch_nonce_sha256, expires_at_unix_seconds, \
             claimed_at_unix_seconds, row_sha256\
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&row.nonce_key_sha256)
    .bind(&row.claim_json)
    .bind(&row.claim_sha256)
    .bind(&row.evidence_sha256)
    .bind(&row.binding_sha256)
    .bind(&row.subject_sha256)
    .bind(&row.nonce_sha256)
    .bind(&row.launch_nonce_sha256)
    .bind(to_i64(row.expires_at_unix_seconds)?)
    .bind(to_i64(row.claimed_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn count_live_nonces(
    transaction: &mut Transaction<'_, Sqlite>,
    now_unix_seconds: u64,
) -> P12Result<u64> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM p12_nonce_claims WHERE expires_at_unix_seconds > ?",
    )
    .bind(to_i64(now_unix_seconds)?)
    .fetch_one(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    u64::try_from(count).map_err(|_| P12Error::CorruptState)
}

async fn load_operation_optional(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_id: &str,
) -> P12Result<Option<OperationRow>> {
    sqlx::query("SELECT * FROM p12_operations WHERE operation_id = ?")
        .bind(operation_id)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(map_sqlx)?
        .as_ref()
        .map(OperationRow::from_sqlite)
        .transpose()
}

async fn insert_operation(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &OperationRow,
) -> P12Result<()> {
    sqlx::query(
        "INSERT INTO p12_operations (\
             operation_id, binding_json, binding_sha256, state, last_status_revision, \
             last_manual_revision, last_status_sha256, last_manual_sha256, \
             last_observed_at_unix_seconds, record_revision, created_at_unix_seconds, \
             updated_at_unix_seconds, row_sha256\
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&row.operation_id)
    .bind(&row.binding_json)
    .bind(&row.binding_sha256)
    .bind(evidence_state_name(row.state))
    .bind(optional_i64(row.last_status_revision)?)
    .bind(optional_i64(row.last_manual_revision)?)
    .bind(&row.last_status_sha256)
    .bind(&row.last_manual_sha256)
    .bind(optional_i64(row.last_observed_at_unix_seconds)?)
    .bind(to_i64(row.record_revision)?)
    .bind(to_i64(row.created_at_unix_seconds)?)
    .bind(to_i64(row.updated_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn update_operation(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &OperationRow,
    expected_revision: u64,
) -> P12Result<()> {
    let result = sqlx::query(
        "UPDATE p12_operations SET state = ?, last_status_revision = ?, \
         last_manual_revision = ?, last_status_sha256 = ?, last_manual_sha256 = ?, \
         last_observed_at_unix_seconds = ?, record_revision = ?, \
         updated_at_unix_seconds = ?, row_sha256 = ? \
         WHERE operation_id = ? AND record_revision = ?",
    )
    .bind(evidence_state_name(row.state))
    .bind(optional_i64(row.last_status_revision)?)
    .bind(optional_i64(row.last_manual_revision)?)
    .bind(&row.last_status_sha256)
    .bind(&row.last_manual_sha256)
    .bind(optional_i64(row.last_observed_at_unix_seconds)?)
    .bind(to_i64(row.record_revision)?)
    .bind(to_i64(row.updated_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .bind(&row.operation_id)
    .bind(to_i64(expected_revision)?)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    if result.rows_affected() != 1 {
        return Err(P12Error::StaleObservation);
    }
    Ok(())
}

async fn load_status_optional(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_id: &str,
    status_revision: u64,
) -> P12Result<Option<StatusRow>> {
    sqlx::query("SELECT * FROM p12_status_evidence WHERE operation_id = ? AND status_revision = ?")
        .bind(operation_id)
        .bind(to_i64(status_revision)?)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(map_sqlx)?
        .as_ref()
        .map(StatusRow::from_sqlite)
        .transpose()
}

async fn insert_status(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &StatusRow,
) -> P12Result<()> {
    sqlx::query(
        "INSERT INTO p12_status_evidence (\
             operation_id, status_revision, observation_json, observation_sha256, \
             evidence_sha256, state, observed_at_unix_seconds, row_sha256\
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&row.operation_id)
    .bind(to_i64(row.status_revision)?)
    .bind(&row.observation_json)
    .bind(&row.observation_sha256)
    .bind(&row.evidence_sha256)
    .bind(evidence_state_name(row.state))
    .bind(to_i64(row.observed_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn upsert_status_head(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &StatusHeadRow,
) -> P12Result<()> {
    sqlx::query(
        "INSERT INTO p12_status_heads (\
             operation_id, status_revision, evidence_sha256, state, observed_at_unix_seconds, row_sha256\
         ) VALUES (?, ?, ?, ?, ?, ?) \
         ON CONFLICT(operation_id) DO UPDATE SET \
             status_revision = excluded.status_revision, \
             evidence_sha256 = excluded.evidence_sha256, \
             state = excluded.state, \
             observed_at_unix_seconds = excluded.observed_at_unix_seconds, \
             row_sha256 = excluded.row_sha256",
    )
    .bind(&row.operation_id)
    .bind(to_i64(row.status_revision)?)
    .bind(&row.evidence_sha256)
    .bind(evidence_state_name(row.state))
    .bind(to_i64(row.observed_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn load_manual_optional(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_id: &str,
    manual_revision: u64,
) -> P12Result<Option<ManualRow>> {
    sqlx::query("SELECT * FROM p12_manual_evidence WHERE operation_id = ? AND manual_revision = ?")
        .bind(operation_id)
        .bind(to_i64(manual_revision)?)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(map_sqlx)?
        .as_ref()
        .map(ManualRow::from_sqlite)
        .transpose()
}

async fn insert_manual(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &ManualRow,
) -> P12Result<()> {
    sqlx::query(
        "INSERT INTO p12_manual_evidence (\
             operation_id, manual_revision, observation_json, observation_sha256, \
             evidence_sha256, state, observed_at_unix_seconds, row_sha256\
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&row.operation_id)
    .bind(to_i64(row.manual_revision)?)
    .bind(&row.observation_json)
    .bind(&row.observation_sha256)
    .bind(&row.evidence_sha256)
    .bind(evidence_state_name(row.state))
    .bind(to_i64(row.observed_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn upsert_manual_head(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &ManualHeadRow,
) -> P12Result<()> {
    sqlx::query(
        "INSERT INTO p12_manual_heads (\
             operation_id, manual_revision, evidence_sha256, state, observed_at_unix_seconds, row_sha256\
         ) VALUES (?, ?, ?, ?, ?, ?) \
         ON CONFLICT(operation_id) DO UPDATE SET \
             manual_revision = excluded.manual_revision, \
             evidence_sha256 = excluded.evidence_sha256, \
             state = excluded.state, \
             observed_at_unix_seconds = excluded.observed_at_unix_seconds, \
             row_sha256 = excluded.row_sha256",
    )
    .bind(&row.operation_id)
    .bind(to_i64(row.manual_revision)?)
    .bind(&row.evidence_sha256)
    .bind(evidence_state_name(row.state))
    .bind(to_i64(row.observed_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn tombstone_exists(
    transaction: &mut Transaction<'_, Sqlite>,
    operation_id: &str,
) -> P12Result<bool> {
    let value: Option<i64> =
        sqlx::query_scalar("SELECT 1 FROM p12_terminal_tombstones WHERE operation_id = ?")
            .bind(operation_id)
            .fetch_optional(&mut **transaction)
            .await
            .map_err(map_sqlx)?;
    Ok(value.is_some())
}

async fn insert_tombstone(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &TombstoneRow,
) -> P12Result<()> {
    sqlx::query(
        "INSERT INTO p12_terminal_tombstones (\
             operation_id, source_kind, terminal_state, evidence_sha256, \
             terminal_at_unix_seconds, retain_until_unix_seconds, row_sha256\
         ) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&row.operation_id)
    .bind(&row.source_kind)
    .bind(evidence_state_name(row.terminal_state))
    .bind(&row.evidence_sha256)
    .bind(to_i64(row.terminal_at_unix_seconds)?)
    .bind(to_i64(row.retain_until_unix_seconds)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn append_receipt(
    transaction: &mut Transaction<'_, Sqlite>,
    event_kind: &str,
    subject_id: &str,
    subject_revision: u64,
    payload_sha256: &Sha256Digest,
    writer: &P12WriterIdentity,
    recorded_at_unix_seconds: u64,
) -> P12Result<()> {
    let receipt = ReceiptRow {
        event_kind: event_kind.to_owned(),
        subject_id: subject_id.to_owned(),
        subject_revision,
        payload_sha256: payload_sha256.as_str().to_owned(),
        writer: writer.clone(),
        recorded_at_unix_seconds,
        witness_sha256: String::new(),
    };
    let witness_sha256 = receipt.digest()?;
    sqlx::query(
        "INSERT INTO p12_durable_receipts (\
             event_kind, subject_id, subject_revision, payload_sha256, writer_boot_id, \
             writer_generation, recorded_at_unix_seconds, witness_sha256\
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(event_kind)
    .bind(subject_id)
    .bind(to_i64(subject_revision)?)
    .bind(payload_sha256.as_str())
    .bind(&writer.boot_id)
    .bind(to_i64(writer.generation)?)
    .bind(to_i64(recorded_at_unix_seconds)?)
    .bind(witness_sha256.as_str())
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn insert_gc_cursor(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &GcCursorRow,
) -> P12Result<()> {
    sqlx::query(
        "INSERT INTO p12_gc_cursor (singleton, revision, last_run_at_unix_seconds, row_sha256) \
         VALUES (1, ?, ?, ?)",
    )
    .bind(to_i64(row.revision)?)
    .bind(to_i64(row.last_run_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    Ok(())
}

async fn load_gc_cursor(transaction: &mut Transaction<'_, Sqlite>) -> P12Result<GcCursorRow> {
    let row = sqlx::query("SELECT * FROM p12_gc_cursor WHERE singleton = 1")
        .fetch_optional(&mut **transaction)
        .await
        .map_err(map_sqlx)?
        .ok_or(P12Error::CorruptState)?;
    GcCursorRow::from_sqlite(&row)
}

async fn load_gc_cursor_pool(pool: &SqlitePool) -> P12Result<GcCursorRow> {
    let row = sqlx::query("SELECT * FROM p12_gc_cursor WHERE singleton = 1")
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?
        .ok_or(P12Error::CorruptState)?;
    GcCursorRow::from_sqlite(&row)
}

async fn update_gc_cursor(
    transaction: &mut Transaction<'_, Sqlite>,
    row: &GcCursorRow,
    expected_revision: u64,
) -> P12Result<()> {
    let result = sqlx::query(
        "UPDATE p12_gc_cursor SET revision = ?, last_run_at_unix_seconds = ?, row_sha256 = ? \
         WHERE singleton = 1 AND revision = ?",
    )
    .bind(to_i64(row.revision)?)
    .bind(to_i64(row.last_run_at_unix_seconds)?)
    .bind(&row.row_sha256)
    .bind(to_i64(expected_revision)?)
    .execute(&mut **transaction)
    .await
    .map_err(map_sqlx)?;
    if result.rows_affected() != 1 {
        return Err(P12Error::GcConflict);
    }
    Ok(())
}

async fn verify_all_keys(pool: &SqlitePool) -> P12Result<u64> {
    let rows =
        sqlx::query("SELECT * FROM p12_key_registrations ORDER BY issuer_id, key_epoch, key_id")
            .fetch_all(pool)
            .await
            .map_err(map_sqlx)?;
    for row in &rows {
        KeyRow::from_sqlite(row)?.verify()?;
    }
    u64::try_from(rows.len()).map_err(|_| P12Error::CorruptState)
}

async fn verify_all_key_heads(pool: &SqlitePool) -> P12Result<()> {
    let rows = sqlx::query("SELECT * FROM p12_key_heads ORDER BY issuer_id, purpose")
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    for row in rows {
        let head = KeyHeadRow::from_sqlite(&row)?;
        head.verify()?;
        let key = sqlx::query(
            "SELECT * FROM p12_key_registrations \
             WHERE issuer_id = ? AND key_id = ? AND key_epoch = ?",
        )
        .bind(&head.issuer_id)
        .bind(&head.current_key_id)
        .bind(to_i64(head.current_key_epoch)?)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?
        .ok_or(P12Error::CorruptState)?;
        let key = KeyRow::from_sqlite(&key)?;
        key.verify()?;
        if key.purpose != head.purpose {
            return Err(P12Error::CorruptState);
        }
    }
    Ok(())
}

async fn verify_all_nonces(pool: &SqlitePool) -> P12Result<u64> {
    let rows = sqlx::query("SELECT * FROM p12_nonce_claims ORDER BY nonce_key_sha256")
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    for row in &rows {
        NonceRow::from_sqlite(row)?.verify()?;
    }
    u64::try_from(rows.len()).map_err(|_| P12Error::CorruptState)
}

async fn verify_all_operations(pool: &SqlitePool) -> P12Result<u64> {
    let rows = sqlx::query("SELECT * FROM p12_operations ORDER BY operation_id")
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    for row in &rows {
        OperationRow::from_sqlite(row)?.verify()?;
    }
    u64::try_from(rows.len()).map_err(|_| P12Error::CorruptState)
}

async fn verify_all_status(pool: &SqlitePool) -> P12Result<u64> {
    let rows =
        sqlx::query("SELECT * FROM p12_status_evidence ORDER BY operation_id, status_revision")
            .fetch_all(pool)
            .await
            .map_err(map_sqlx)?;
    for row in &rows {
        StatusRow::from_sqlite(row)?.verify()?;
    }
    u64::try_from(rows.len()).map_err(|_| P12Error::CorruptState)
}

async fn verify_all_status_heads(pool: &SqlitePool) -> P12Result<()> {
    let rows = sqlx::query("SELECT * FROM p12_status_heads ORDER BY operation_id")
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    for row in rows {
        let head = StatusHeadRow::from_sqlite(&row)?;
        head.verify()?;
        let evidence = sqlx::query(
            "SELECT * FROM p12_status_evidence \
             WHERE operation_id = ? AND status_revision = ?",
        )
        .bind(&head.operation_id)
        .bind(to_i64(head.status_revision)?)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?
        .ok_or(P12Error::CorruptState)?;
        let evidence = StatusRow::from_sqlite(&evidence)?;
        evidence.verify()?;
        if evidence.evidence_sha256 != head.evidence_sha256
            || evidence.state != head.state
            || evidence.observed_at_unix_seconds != head.observed_at_unix_seconds
        {
            return Err(P12Error::CorruptState);
        }
    }
    Ok(())
}

async fn verify_all_manual(pool: &SqlitePool) -> P12Result<u64> {
    let rows =
        sqlx::query("SELECT * FROM p12_manual_evidence ORDER BY operation_id, manual_revision")
            .fetch_all(pool)
            .await
            .map_err(map_sqlx)?;
    for row in &rows {
        ManualRow::from_sqlite(row)?.verify()?;
    }
    u64::try_from(rows.len()).map_err(|_| P12Error::CorruptState)
}

async fn verify_all_manual_heads(pool: &SqlitePool) -> P12Result<()> {
    let rows = sqlx::query("SELECT * FROM p12_manual_heads ORDER BY operation_id")
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    for row in rows {
        let head = ManualHeadRow::from_sqlite(&row)?;
        head.verify()?;
        let evidence = sqlx::query(
            "SELECT * FROM p12_manual_evidence \
             WHERE operation_id = ? AND manual_revision = ?",
        )
        .bind(&head.operation_id)
        .bind(to_i64(head.manual_revision)?)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?
        .ok_or(P12Error::CorruptState)?;
        let evidence = ManualRow::from_sqlite(&evidence)?;
        evidence.verify()?;
        if evidence.evidence_sha256 != head.evidence_sha256
            || evidence.state != head.state
            || evidence.observed_at_unix_seconds != head.observed_at_unix_seconds
        {
            return Err(P12Error::CorruptState);
        }
    }
    Ok(())
}

async fn verify_all_tombstones(pool: &SqlitePool) -> P12Result<u64> {
    let rows = sqlx::query("SELECT * FROM p12_terminal_tombstones ORDER BY operation_id")
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    for row in &rows {
        let tombstone = TombstoneRow::from_sqlite(row)?;
        tombstone.verify()?;
        let operation = sqlx::query("SELECT * FROM p12_operations WHERE operation_id = ?")
            .bind(&tombstone.operation_id)
            .fetch_optional(pool)
            .await
            .map_err(map_sqlx)?
            .ok_or(P12Error::CorruptState)?;
        let operation = OperationRow::from_sqlite(&operation)?;
        operation.verify()?;
        if operation.state != tombstone.terminal_state {
            return Err(P12Error::CorruptState);
        }
        let head_digest: Option<String> = match tombstone.source_kind.as_str() {
            "PROVIDER_STATUS" => sqlx::query_scalar(
                "SELECT evidence_sha256 FROM p12_status_heads WHERE operation_id = ?",
            )
            .bind(&tombstone.operation_id)
            .fetch_optional(pool)
            .await
            .map_err(map_sqlx)?,
            "MANUAL_EVIDENCE" => sqlx::query_scalar(
                "SELECT evidence_sha256 FROM p12_manual_heads WHERE operation_id = ?",
            )
            .bind(&tombstone.operation_id)
            .fetch_optional(pool)
            .await
            .map_err(map_sqlx)?,
            _ => return Err(P12Error::CorruptState),
        };
        if head_digest.as_deref() != Some(tombstone.evidence_sha256.as_str()) {
            return Err(P12Error::CorruptState);
        }
    }
    u64::try_from(rows.len()).map_err(|_| P12Error::CorruptState)
}

async fn verify_operation_head_consistency(pool: &SqlitePool) -> P12Result<()> {
    let rows = sqlx::query("SELECT * FROM p12_operations ORDER BY operation_id")
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    for row in rows {
        let operation = OperationRow::from_sqlite(&row)?;
        operation.verify()?;
        let status_head = sqlx::query("SELECT * FROM p12_status_heads WHERE operation_id = ?")
            .bind(&operation.operation_id)
            .fetch_optional(pool)
            .await
            .map_err(map_sqlx)?;
        match (operation.last_status_revision, status_head) {
            (None, None) => {}
            (Some(revision), Some(row)) => {
                let head = StatusHeadRow::from_sqlite(&row)?;
                head.verify()?;
                if head.status_revision != revision
                    || Some(head.evidence_sha256) != operation.last_status_sha256
                {
                    return Err(P12Error::CorruptState);
                }
            }
            _ => return Err(P12Error::CorruptState),
        }
        let manual_head = sqlx::query("SELECT * FROM p12_manual_heads WHERE operation_id = ?")
            .bind(&operation.operation_id)
            .fetch_optional(pool)
            .await
            .map_err(map_sqlx)?;
        match (operation.last_manual_revision, manual_head) {
            (None, None) => {}
            (Some(revision), Some(row)) => {
                let head = ManualHeadRow::from_sqlite(&row)?;
                head.verify()?;
                if head.manual_revision != revision
                    || Some(head.evidence_sha256) != operation.last_manual_sha256
                {
                    return Err(P12Error::CorruptState);
                }
            }
            _ => return Err(P12Error::CorruptState),
        }
        let has_tombstone: Option<i64> =
            sqlx::query_scalar("SELECT 1 FROM p12_terminal_tombstones WHERE operation_id = ?")
                .bind(&operation.operation_id)
                .fetch_optional(pool)
                .await
                .map_err(map_sqlx)?;
        if operation.state.is_terminal() != has_tombstone.is_some() {
            return Err(P12Error::CorruptState);
        }
    }
    Ok(())
}

async fn verify_all_receipts(pool: &SqlitePool) -> P12Result<u64> {
    let rows = sqlx::query("SELECT * FROM p12_durable_receipts ORDER BY sequence")
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    for row in &rows {
        ReceiptRow::from_sqlite(row)?.verify()?;
    }
    u64::try_from(rows.len()).map_err(|_| P12Error::CorruptState)
}

fn verify_provider_binding(
    binding: &P11OperationEvidenceBinding,
    observation: &P12ProviderObservation,
) -> P12Result<()> {
    if binding.operation_id != observation.operation_id
        || binding.provider_id != observation.provider_id
        || binding.profile_id != observation.profile_id
        || binding.token_family_id != observation.token_family_id
        || binding.status_binding_sha256 != observation.status_binding_sha256
        || binding.fence != observation.fence
    {
        return Err(P12Error::BindingMismatch);
    }
    Ok(())
}

fn verify_manual_binding(
    binding: &P11OperationEvidenceBinding,
    observation: &P12ManualObservation,
) -> P12Result<()> {
    if binding.operation_id != observation.operation_id
        || binding.status_binding_sha256 != observation.status_binding_sha256
        || binding.fence != observation.fence
    {
        return Err(P12Error::BindingMismatch);
    }
    Ok(())
}

async fn count_rows(transaction: &mut Transaction<'_, Sqlite>, table: &str) -> P12Result<u64> {
    let query = match table {
        "p12_key_registrations" => "SELECT COUNT(*) FROM p12_key_registrations",
        "p12_operations" => "SELECT COUNT(*) FROM p12_operations",
        _ => return Err(P12Error::InvalidInput),
    };
    let count: i64 = sqlx::query_scalar(query)
        .fetch_one(&mut **transaction)
        .await
        .map_err(map_sqlx)?;
    u64::try_from(count).map_err(|_| P12Error::CorruptState)
}

async fn count_rows_pool(pool: &SqlitePool, table: &str) -> P12Result<u64> {
    let query = match table {
        "p12_key_registrations" => "SELECT COUNT(*) FROM p12_key_registrations",
        "p12_nonce_claims" => "SELECT COUNT(*) FROM p12_nonce_claims",
        "p12_operations" => "SELECT COUNT(*) FROM p12_operations",
        "p12_status_evidence" => "SELECT COUNT(*) FROM p12_status_evidence",
        "p12_manual_evidence" => "SELECT COUNT(*) FROM p12_manual_evidence",
        _ => return Err(P12Error::InvalidInput),
    };
    let count: i64 = sqlx::query_scalar(query)
        .fetch_one(pool)
        .await
        .map_err(map_sqlx)?;
    u64::try_from(count).map_err(|_| P12Error::CorruptState)
}

fn to_i64(value: u64) -> P12Result<i64> {
    i64::try_from(value).map_err(|_| P12Error::InvalidInput)
}

fn optional_i64(value: Option<u64>) -> P12Result<Option<i64>> {
    value.map(to_i64).transpose()
}

fn read_u64(row: &SqliteRow, column: &str) -> P12Result<u64> {
    let value: i64 = row.try_get(column).map_err(map_sqlx)?;
    u64::try_from(value).map_err(|_| P12Error::CorruptState)
}

fn read_optional_u64(row: &SqliteRow, column: &str) -> P12Result<Option<u64>> {
    let value: Option<i64> = row.try_get(column).map_err(map_sqlx)?;
    value
        .map(|value| u64::try_from(value).map_err(|_| P12Error::CorruptState))
        .transpose()
}

fn parse_digest(value: &str) -> P12Result<Sha256Digest> {
    let digest = Sha256Digest::parse(value.to_owned()).map_err(|_| P12Error::CorruptState)?;
    validate_digest(&digest).map_err(|_| P12Error::CorruptState)?;
    Ok(digest)
}

fn map_sqlx(error: sqlx::Error) -> P12Error {
    if let sqlx::Error::Database(database) = &error {
        let message = database.message().to_ascii_lowercase();
        if message.contains("database or disk is full")
            || message.contains("disk i/o error")
            || message.contains("readonly")
        {
            return P12Error::StorageUnavailable;
        }
    }
    P12Error::StorageUnavailable
}

fn create_private_directory(path: &Path) -> P12Result<()> {
    fs::create_dir_all(path).map_err(|_| P12Error::StorageUnavailable)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .map_err(|_| P12Error::StorageUnavailable)?;
    }
    Ok(())
}

fn protect_private_file(path: &Path) -> P12Result<()> {
    if !path.is_file() {
        return Err(P12Error::StorageUnavailable);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .map_err(|_| P12Error::StorageUnavailable)?;
    }
    Ok(())
}
