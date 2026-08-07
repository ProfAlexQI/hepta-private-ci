use codex_hepta_contracts::MemoryMutationDryRun;
use codex_hepta_contracts::MemoryMutationDryRunId;
use codex_hepta_contracts::MemoryMutationProposal;
use codex_hepta_contracts::MemoryMutationProposalId;
use codex_hepta_contracts::Sha256Digest;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::de::Error as _;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;
use sqlx::sqlite::SqliteRow;

use crate::AppendDisposition;
use crate::EvidenceError;
use crate::HeptaEvidenceStore;
use crate::canonical::canonical_json;
use crate::canonical::canonical_storage_payload;
use crate::store::classify_sqlx_error;
use crate::store::now_millis;

pub const MEMORY_MUTATION_SHADOW_SCHEMA_VERSION: u32 = 1;

/// Append-only, digest-only evidence for one memory mutation simulation.
///
/// This record deliberately has no authority field. Persisting it proves only
/// that the bound proposal and evaluator result were recorded.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MemoryMutationShadowObservation {
    pub schema_version: u32,
    pub proposal: MemoryMutationProposal,
    pub dry_run: MemoryMutationDryRun,
}

impl MemoryMutationShadowObservation {
    pub fn new(
        proposal: MemoryMutationProposal,
        dry_run: MemoryMutationDryRun,
    ) -> Result<Self, String> {
        let observation = Self {
            schema_version: MEMORY_MUTATION_SHADOW_SCHEMA_VERSION,
            proposal,
            dry_run,
        };
        observation.validate()?;
        Ok(observation)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != MEMORY_MUTATION_SHADOW_SCHEMA_VERSION {
            return Err("unsupported memory mutation shadow schema version".to_string());
        }
        self.dry_run.validate_for_proposal(&self.proposal)?;
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MemoryMutationShadowObservationWire {
    schema_version: u32,
    proposal: MemoryMutationProposal,
    dry_run: MemoryMutationDryRun,
}

impl<'de> Deserialize<'de> for MemoryMutationShadowObservation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = MemoryMutationShadowObservationWire::deserialize(deserializer)?;
        let observation = Self {
            schema_version: wire.schema_version,
            proposal: wire.proposal,
            dry_run: wire.dry_run,
        };
        observation.validate().map_err(D::Error::custom)?;
        Ok(observation)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredMemoryMutationShadowObservation {
    pub seq: i64,
    pub observation: MemoryMutationShadowObservation,
}

impl HeptaEvidenceStore {
    pub async fn append_memory_mutation_shadow_observation(
        &self,
        observation: &MemoryMutationShadowObservation,
    ) -> Result<AppendDisposition, EvidenceError> {
        validate_observation(observation)?;
        let (payload_json, evidence_sha256) = canonical_storage_payload(observation)?;
        let scope_sha256 = observation.proposal.scope.binding_sha256();
        let dry_run = &observation.dry_run;
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        let insert = sqlx::query(
            "INSERT INTO memory_mutation_shadow_observations (
                dry_run_id, proposal_id, turn_sha256, scope_sha256, snapshot_sha256,
                disposition, reason, projected_memory_writes, schema_version,
                payload_json, evidence_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(dry_run.dry_run_id.as_str())
        .bind(observation.proposal.proposal_id.as_str())
        .bind(observation.proposal.turn_sha256.as_str())
        .bind(scope_sha256.as_str())
        .bind(dry_run.snapshot_sha256.as_str())
        .bind(dry_run.disposition.as_str())
        .bind(dry_run.reason.as_str())
        .bind(i64::from(dry_run.projected_memory_writes))
        .bind(i64::from(observation.schema_version))
        .bind(&payload_json)
        .bind(evidence_sha256.as_str())
        .bind(now_millis()?)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let disposition = verify_observation(
            &mut transaction,
            observation,
            &payload_json,
            evidence_sha256.as_str(),
            insert.rows_affected() == 1,
        )
        .await?;
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(disposition)
    }

    pub async fn get_memory_mutation_shadow_observation(
        &self,
        dry_run_id: &MemoryMutationDryRunId,
    ) -> Result<Option<StoredMemoryMutationShadowObservation>, EvidenceError> {
        let row = sqlx::query(
            "SELECT seq, dry_run_id, proposal_id, turn_sha256, scope_sha256,
                    snapshot_sha256, disposition, reason, projected_memory_writes,
                    schema_version, payload_json, evidence_sha256
             FROM memory_mutation_shadow_observations WHERE dry_run_id = ?",
        )
        .bind(dry_run_id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(classify_sqlx_error)?;
        row.map(|row| stored_observation_from_row(&row)).transpose()
    }

    pub async fn list_memory_mutation_shadow_observations(
        &self,
        proposal_id: &MemoryMutationProposalId,
        after_seq: i64,
        limit: u32,
    ) -> Result<Vec<StoredMemoryMutationShadowObservation>, EvidenceError> {
        if after_seq < 0 {
            return invalid("memory mutation shadow query requires a non-negative cursor");
        }
        if !(1..=1_000).contains(&limit) {
            return invalid("memory mutation shadow query limit must be between 1 and 1000");
        }
        let rows = sqlx::query(
            "SELECT seq, dry_run_id, proposal_id, turn_sha256, scope_sha256,
                    snapshot_sha256, disposition, reason, projected_memory_writes,
                    schema_version, payload_json, evidence_sha256
             FROM memory_mutation_shadow_observations
             WHERE proposal_id = ? AND seq > ? ORDER BY seq ASC LIMIT ?",
        )
        .bind(proposal_id.as_str())
        .bind(after_seq)
        .bind(i64::from(limit))
        .fetch_all(&self.pool)
        .await
        .map_err(classify_sqlx_error)?;
        rows.iter().map(stored_observation_from_row).collect()
    }
}

async fn verify_observation(
    transaction: &mut Transaction<'_, Sqlite>,
    observation: &MemoryMutationShadowObservation,
    payload_json: &str,
    evidence_sha256: &str,
    inserted: bool,
) -> Result<AppendDisposition, EvidenceError> {
    let rows = sqlx::query(
        "SELECT seq, dry_run_id, proposal_id, turn_sha256, scope_sha256,
                snapshot_sha256, disposition, reason, projected_memory_writes,
                schema_version, payload_json, evidence_sha256
         FROM memory_mutation_shadow_observations
         WHERE dry_run_id = ? OR (proposal_id = ? AND snapshot_sha256 = ?)",
    )
    .bind(observation.dry_run.dry_run_id.as_str())
    .bind(observation.proposal.proposal_id.as_str())
    .bind(observation.dry_run.snapshot_sha256.as_str())
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    if rows.len() != 1 {
        return Err(EvidenceError::Corrupt(format!(
            "expected one memory mutation shadow row for {} but found {}",
            observation.dry_run.dry_run_id.as_str(),
            rows.len()
        )));
    }
    let stored = decode_observation_row(&rows[0])?;
    let exact = rows[0].get::<String, _>("payload_json") == payload_json
        && rows[0].get::<String, _>("evidence_sha256") == evidence_sha256
        && stored == *observation;
    if !exact {
        return Err(EvidenceError::IdempotencyConflict {
            record_id: observation.dry_run.dry_run_id.as_str().to_string(),
        });
    }
    Ok(if inserted {
        AppendDisposition::Inserted
    } else {
        AppendDisposition::AlreadyPresent
    })
}

fn stored_observation_from_row(
    row: &SqliteRow,
) -> Result<StoredMemoryMutationShadowObservation, EvidenceError> {
    Ok(StoredMemoryMutationShadowObservation {
        seq: row.get("seq"),
        observation: decode_observation_row(row)?,
    })
}

fn decode_observation_row(
    row: &SqliteRow,
) -> Result<MemoryMutationShadowObservation, EvidenceError> {
    let payload_json: String = row.get("payload_json");
    verify_stored_digest(
        &payload_json,
        row.get("evidence_sha256"),
        "memory mutation shadow observation",
    )?;
    let observation: MemoryMutationShadowObservation = serde_json::from_str(&payload_json)
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    validate_observation(&observation).map_err(invalid_as_corrupt)?;
    verify_canonical_payload(
        &observation,
        &payload_json,
        "memory mutation shadow observation",
    )?;
    let proposal = &observation.proposal;
    let dry_run = &observation.dry_run;
    let scope_sha256 = proposal.scope.binding_sha256();
    if row.get::<String, _>("dry_run_id") != dry_run.dry_run_id.as_str()
        || row.get::<String, _>("proposal_id") != proposal.proposal_id.as_str()
        || row.get::<String, _>("turn_sha256") != proposal.turn_sha256.as_str()
        || row.get::<String, _>("scope_sha256") != scope_sha256.as_str()
        || row.get::<String, _>("snapshot_sha256") != dry_run.snapshot_sha256.as_str()
        || row.get::<String, _>("disposition") != dry_run.disposition.as_str()
        || row.get::<String, _>("reason") != dry_run.reason.as_str()
        || row.get::<i64, _>("projected_memory_writes")
            != i64::from(dry_run.projected_memory_writes)
        || row.get::<i64, _>("schema_version") != i64::from(observation.schema_version)
    {
        return Err(EvidenceError::Corrupt(
            "memory mutation shadow columns do not match canonical payload".to_string(),
        ));
    }
    Ok(observation)
}

fn validate_observation(
    observation: &MemoryMutationShadowObservation,
) -> Result<(), EvidenceError> {
    observation.validate().map_err(EvidenceError::InvalidRecord)
}

fn verify_stored_digest(
    payload_json: &str,
    expected: &str,
    record_kind: &str,
) -> Result<(), EvidenceError> {
    let actual = Sha256Digest::for_bytes(payload_json.as_bytes());
    if actual.as_str() == expected {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(format!(
            "stored {record_kind} payload digest mismatch"
        )))
    }
}

fn verify_canonical_payload<T: Serialize>(
    value: &T,
    stored: &str,
    record_kind: &str,
) -> Result<(), EvidenceError> {
    let canonical = canonical_json(value)?;
    if canonical == stored.as_bytes() {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(format!(
            "stored {record_kind} JSON is not canonical"
        )))
    }
}

fn invalid_as_corrupt(error: EvidenceError) -> EvidenceError {
    match error {
        EvidenceError::InvalidRecord(detail) => EvidenceError::Corrupt(detail),
        other => other,
    }
}

fn invalid<T>(detail: impl Into<String>) -> Result<T, EvidenceError> {
    Err(EvidenceError::InvalidRecord(detail.into()))
}
