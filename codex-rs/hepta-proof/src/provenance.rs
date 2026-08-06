use codex_hepta_contracts::MigrationFamilySnapshot;
use codex_hepta_contracts::MigrationSnapshotId;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_evidence::EvidenceError;
use codex_hepta_evidence::HISTORICAL_EVIDENCE_SCHEMA_VERSION;
use codex_hepta_evidence::HeptaEvidenceStore;
use codex_hepta_evidence::HistoricalEvidenceFamily;
use codex_hepta_evidence::HistoricalEvidenceRecord;
use codex_hepta_evidence::HistoricalEvidenceSelector;
use codex_hepta_evidence::HistoricalEvidenceState;
use codex_hepta_evidence::historical_record_sha256;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

use crate::ProofCommandSpec;
use crate::ProofError;
use crate::ProofExecutionResult;
use crate::ProofHarness;
use crate::ProofInvocation;
use crate::ProofSubject;

pub const PROOF_PROVENANCE_SCHEMA_VERSION: u32 = 1;

const PROOF_PROVENANCE_CONTEXT_DOMAIN: &str = "hepta.proof-provenance-context.v1";

/// A successful historical state proposed as proof context.
///
/// Only the three positive terminal states supported by the clean evidence
/// series are admitted. Pending, rejected, blocked, failed, aborted, and
/// indeterminate records cannot be used as proof provenance context.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceEvidenceKind {
    GovernanceHandlerCompletedSuccess,
    ProviderCompleted,
    ChannelIngressAccepted,
}

impl ProvenanceEvidenceKind {
    pub const fn as_wire_str(self) -> &'static str {
        match self {
            Self::GovernanceHandlerCompletedSuccess => "governance_handler_completed_success",
            Self::ProviderCompleted => "provider_completed",
            Self::ChannelIngressAccepted => "channel_ingress_accepted",
        }
    }

    pub const fn historical_family(self) -> HistoricalEvidenceFamily {
        match self {
            Self::GovernanceHandlerCompletedSuccess => HistoricalEvidenceFamily::GovernanceAction,
            Self::ProviderCompleted => HistoricalEvidenceFamily::ProviderAttempt,
            Self::ChannelIngressAccepted => HistoricalEvidenceFamily::ChannelIngress,
        }
    }

    pub const fn historical_state(self) -> HistoricalEvidenceState {
        match self {
            Self::GovernanceHandlerCompletedSuccess => {
                HistoricalEvidenceState::HandlerCompletedSuccess
            }
            Self::ProviderCompleted => HistoricalEvidenceState::Completed,
            Self::ChannelIngressAccepted => HistoricalEvidenceState::Accepted,
        }
    }

    fn for_record(record: &HistoricalEvidenceRecord) -> Result<Self, String> {
        match (record.family(), record.state()) {
            (
                HistoricalEvidenceFamily::GovernanceAction,
                HistoricalEvidenceState::HandlerCompletedSuccess,
            ) => Ok(Self::GovernanceHandlerCompletedSuccess),
            (HistoricalEvidenceFamily::ProviderAttempt, HistoricalEvidenceState::Completed) => {
                Ok(Self::ProviderCompleted)
            }
            (HistoricalEvidenceFamily::ChannelIngress, HistoricalEvidenceState::Accepted) => {
                Ok(Self::ChannelIngressAccepted)
            }
            _ => Err(
                "proof provenance requires a supported positive terminal historical record"
                    .to_string(),
            ),
        }
    }
}

/// Exact proposed execution context for one proof invocation.
///
/// The execution entry point constructs this from a self-validating migration
/// snapshot and a store-produced positive historical record. Deserialization
/// proves only canonical self-consistency. The value is not authority: it may
/// be stale, caller-selected, or sourced from a rollbackable local store. A
/// lineage observation must reread both stores after execution and require an
/// exact match.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProofProvenanceContext {
    schema_version: u32,
    snapshot_id: MigrationSnapshotId,
    candidate_sha256: Sha256Digest,
    historical_schema_version: u32,
    evidence_kind: ProvenanceEvidenceKind,
    historical_record_id: String,
    historical_evidence_sha256: Sha256Digest,
    historical_record_sha256: Sha256Digest,
    context_sha256: Sha256Digest,
}

impl ProofProvenanceContext {
    fn from_record(
        snapshot: &MigrationFamilySnapshot,
        record: &HistoricalEvidenceRecord,
    ) -> Result<Self, ProofError> {
        snapshot.validate().map_err(ProofError::InvalidInput)?;
        record.validate().map_err(ProofError::InvalidInput)?;
        let evidence_kind =
            ProvenanceEvidenceKind::for_record(record).map_err(ProofError::InvalidInput)?;
        let mut context = Self {
            schema_version: PROOF_PROVENANCE_SCHEMA_VERSION,
            snapshot_id: snapshot.snapshot_id().clone(),
            candidate_sha256: snapshot.candidate_sha256().clone(),
            historical_schema_version: record.schema_version(),
            evidence_kind,
            historical_record_id: record.record_id().to_string(),
            historical_evidence_sha256: record.evidence_sha256().clone(),
            historical_record_sha256: record.record_sha256().clone(),
            context_sha256: Sha256Digest::for_bytes(b"pending-proof-provenance-context"),
        };
        context.context_sha256 = context.expected_context_sha256();
        context.validate().map_err(ProofError::InvalidInput)?;
        Ok(context)
    }

    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub const fn snapshot_id(&self) -> &MigrationSnapshotId {
        &self.snapshot_id
    }

    pub const fn candidate_sha256(&self) -> &Sha256Digest {
        &self.candidate_sha256
    }

    pub const fn historical_schema_version(&self) -> u32 {
        self.historical_schema_version
    }

    pub const fn evidence_kind(&self) -> ProvenanceEvidenceKind {
        self.evidence_kind
    }

    pub fn historical_record_id(&self) -> &str {
        &self.historical_record_id
    }

    pub const fn historical_evidence_sha256(&self) -> &Sha256Digest {
        &self.historical_evidence_sha256
    }

    pub const fn historical_record_sha256(&self) -> &Sha256Digest {
        &self.historical_record_sha256
    }

    pub const fn context_sha256(&self) -> &Sha256Digest {
        &self.context_sha256
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != PROOF_PROVENANCE_SCHEMA_VERSION {
            return Err("unsupported proof provenance context schema version".to_string());
        }
        if self.historical_schema_version != HISTORICAL_EVIDENCE_SCHEMA_VERSION {
            return Err("unsupported historical evidence schema version".to_string());
        }
        MigrationSnapshotId::parse(self.snapshot_id.as_str())?;
        Sha256Digest::parse(self.candidate_sha256.as_str())?;
        Sha256Digest::parse(self.historical_evidence_sha256.as_str())?;
        Sha256Digest::parse(self.historical_record_sha256.as_str())?;
        Sha256Digest::parse(self.context_sha256.as_str())?;
        let selector = HistoricalEvidenceSelector::new(
            self.evidence_kind.historical_family(),
            self.historical_record_id.clone(),
        )?;
        if selector.record_id() != self.historical_record_id {
            return Err("proof provenance historical record ID is not canonical".to_string());
        }
        let expected_record_sha256 = historical_record_sha256(
            self.historical_schema_version,
            self.evidence_kind.historical_family(),
            &self.historical_record_id,
            self.evidence_kind.historical_state(),
            &self.historical_evidence_sha256,
        )
        .map_err(|error| error.to_string())?;
        if self.historical_record_sha256 != expected_record_sha256 {
            return Err(
                "proof provenance historical record digest does not match its bindings".to_string(),
            );
        }
        if self.context_sha256 != self.expected_context_sha256() {
            return Err("proof provenance context digest does not match its bindings".to_string());
        }
        Ok(())
    }

    fn expected_context_sha256(&self) -> Sha256Digest {
        let schema_version = self.schema_version.to_string();
        let historical_schema_version = self.historical_schema_version.to_string();
        digest_parts([
            PROOF_PROVENANCE_CONTEXT_DOMAIN,
            schema_version.as_str(),
            self.snapshot_id.as_str(),
            self.candidate_sha256.as_str(),
            historical_schema_version.as_str(),
            self.evidence_kind.as_wire_str(),
            self.historical_record_id.as_str(),
            self.historical_evidence_sha256.as_str(),
            self.historical_record_sha256.as_str(),
        ])
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProofProvenanceContextWire {
    schema_version: u32,
    snapshot_id: MigrationSnapshotId,
    candidate_sha256: Sha256Digest,
    historical_schema_version: u32,
    evidence_kind: ProvenanceEvidenceKind,
    historical_record_id: String,
    historical_evidence_sha256: Sha256Digest,
    historical_record_sha256: Sha256Digest,
    context_sha256: Sha256Digest,
}

impl<'de> Deserialize<'de> for ProofProvenanceContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ProofProvenanceContextWire::deserialize(deserializer)?;
        let context = Self {
            schema_version: wire.schema_version,
            snapshot_id: wire.snapshot_id,
            candidate_sha256: wire.candidate_sha256,
            historical_schema_version: wire.historical_schema_version,
            evidence_kind: wire.evidence_kind,
            historical_record_id: wire.historical_record_id,
            historical_evidence_sha256: wire.historical_evidence_sha256,
            historical_record_sha256: wire.historical_record_sha256,
            context_sha256: wire.context_sha256,
        };
        context.validate().map_err(serde::de::Error::custom)?;
        Ok(context)
    }
}

impl ProofInvocation {
    /// Binds a proof invocation to one exact proposed provenance context.
    ///
    /// This does not make the context authoritative. A later lineage builder
    /// must reread the exact proof receipt and historical record.
    pub(crate) fn new_with_provenance(
        context: &ProofProvenanceContext,
        nonce: [u8; 16],
        command: ProofCommandSpec,
    ) -> Result<Self, ProofError> {
        context.validate().map_err(ProofError::InvalidInput)?;
        let subject = ProofSubject::new(
            context.candidate_sha256.clone(),
            context.context_sha256.clone(),
        )
        .map_err(ProofError::InvalidInput)?;
        Ok(Self::new(subject, nonce, command))
    }
}

/// Resolves one exact positive historical record immediately before running a
/// bounded local observation and binds that record into the proof subject.
///
/// The evidence store read is authoritative only relative to the current
/// rollbackable local SQLite root. This function does not mint authority, and
/// a later lineage builder must independently reread both stores.
pub async fn run_historical_observation(
    harness: &ProofHarness,
    evidence_store: &HeptaEvidenceStore,
    snapshot: &MigrationFamilySnapshot,
    selector: &HistoricalEvidenceSelector,
    nonce: [u8; 16],
    command: ProofCommandSpec,
) -> Result<ProofExecutionResult, ProofError> {
    snapshot.validate().map_err(ProofError::InvalidInput)?;
    let record = evidence_store
        .historical_record(selector)
        .await
        .map_err(map_evidence_error)?
        .ok_or_else(|| {
            ProofError::InvalidInput(
                "proof provenance historical record is not present".to_string(),
            )
        })?;
    if record.family() != selector.family() || record.record_id() != selector.record_id() {
        return Err(ProofError::Corrupt(
            "proof provenance historical read returned the wrong selector".to_string(),
        ));
    }
    let context = ProofProvenanceContext::from_record(snapshot, &record)?;
    let invocation = ProofInvocation::new_with_provenance(&context, nonce, command)?;
    harness.run(invocation).await
}

fn map_evidence_error(error: EvidenceError) -> ProofError {
    match error {
        EvidenceError::Unavailable(detail) => ProofError::StoreUnavailable(detail),
        EvidenceError::InvalidRecord(detail) => ProofError::InvalidInput(detail),
        EvidenceError::Corrupt(detail) => ProofError::Corrupt(detail),
        EvidenceError::Serialization(detail) => ProofError::Corrupt(detail),
        EvidenceError::IdempotencyConflict { record_id } => ProofError::Corrupt(format!(
            "unexpected historical evidence identity conflict for {record_id}"
        )),
    }
}

fn digest_parts<'a>(parts: impl IntoIterator<Item = &'a str>) -> Sha256Digest {
    let mut canonical = Vec::new();
    for part in parts {
        canonical.extend_from_slice(&(part.len() as u64).to_be_bytes());
        canonical.extend_from_slice(part.as_bytes());
    }
    Sha256Digest::for_bytes(&canonical)
}

#[cfg(test)]
#[path = "provenance_tests.rs"]
mod tests;
