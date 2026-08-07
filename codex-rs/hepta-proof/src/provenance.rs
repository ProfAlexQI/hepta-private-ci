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

use crate::PROOF_SCHEMA_VERSION;
use crate::ProofCommandSpec;
use crate::ProofContextOrigin;
use crate::ProofError;
use crate::ProofExecutionResult;
use crate::ProofHarness;
use crate::ProofInvocation;
use crate::ProofInvocationId;
use crate::ProofReceipt;
use crate::ProofReceiptId;
use crate::ProofStore;
use crate::ProofSubject;
use crate::ProofTerminal;
use crate::framing::length_delimited_sha256;

pub const PROOF_PROVENANCE_SCHEMA_VERSION: u32 = 1;
pub const LOCAL_PROOF_PROVENANCE_LINEAGE_SCHEMA_VERSION: u32 = 1;

const PROOF_PROVENANCE_CONTEXT_DOMAIN: &str = "hepta.proof-provenance-context.v1";
const LOCAL_PROOF_PROVENANCE_LINEAGE_DOMAIN: &str = "hepta.local-proof-provenance-lineage.v1";

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
        length_delimited_sha256([
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

/// Exact mixed-time association observed by rereading two local stores.
///
/// The builder accepts only typed identities, resolves the historical record
/// from SQLite, and rereads the proof receipt plus its intent from the proof
/// filesystem. Both roots remain independently replaceable and rollbackable;
/// this value is not an attestation, authority, or anti-rollback lineage.
/// Deserialization proves only unkeyed wire self-consistency; it cannot replay
/// the two store reads or independently prove the successful terminal.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LocalProofProvenanceLineage {
    schema_version: u32,
    context: ProofProvenanceContext,
    proof_schema_version: u32,
    proof_context_origin: ProofContextOrigin,
    proof_invocation_id: ProofInvocationId,
    proof_receipt_id: ProofReceiptId,
    command_binding_sha256: Sha256Digest,
    proof_receipt_sha256: Sha256Digest,
    proof_terminal: ProofTerminal,
    lineage_sha256: Sha256Digest,
}

impl LocalProofProvenanceLineage {
    fn from_context_and_receipt(
        context: ProofProvenanceContext,
        receipt: &ProofReceipt,
    ) -> Result<Self, ProofError> {
        context.validate().map_err(ProofError::InvalidInput)?;
        if receipt.subject().context_origin() != ProofContextOrigin::HistoricalStoreResolved {
            return Err(ProofError::InvalidInput(
                "proof receipt was not produced by the historical store resolver".to_string(),
            ));
        }
        if !successful_terminal(receipt.terminal()) {
            return Err(ProofError::InvalidInput(
                "proof provenance lineage requires a successful completed receipt".to_string(),
            ));
        }
        if receipt.subject().candidate_sha256() != context.candidate_sha256()
            || receipt.subject().context_sha256() != context.context_sha256()
        {
            return Err(ProofError::InvalidInput(
                "proof receipt does not match the resolved historical context".to_string(),
            ));
        }
        let mut lineage = Self {
            schema_version: LOCAL_PROOF_PROVENANCE_LINEAGE_SCHEMA_VERSION,
            context,
            proof_schema_version: receipt.schema_version(),
            proof_context_origin: receipt.subject().context_origin(),
            proof_invocation_id: receipt.invocation_id().clone(),
            proof_receipt_id: receipt.receipt_id().clone(),
            command_binding_sha256: receipt.command_binding_sha256().clone(),
            proof_receipt_sha256: receipt.receipt_sha256().clone(),
            proof_terminal: receipt.terminal().clone(),
            lineage_sha256: Sha256Digest::for_bytes(b"pending-local-proof-provenance-lineage"),
        };
        lineage.lineage_sha256 = lineage.expected_lineage_sha256();
        lineage.validate().map_err(ProofError::InvalidInput)?;
        Ok(lineage)
    }

    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub const fn context(&self) -> &ProofProvenanceContext {
        &self.context
    }

    pub const fn proof_schema_version(&self) -> u32 {
        self.proof_schema_version
    }

    pub const fn proof_context_origin(&self) -> ProofContextOrigin {
        self.proof_context_origin
    }

    pub const fn proof_invocation_id(&self) -> &ProofInvocationId {
        &self.proof_invocation_id
    }

    pub const fn proof_receipt_id(&self) -> &ProofReceiptId {
        &self.proof_receipt_id
    }

    pub const fn command_binding_sha256(&self) -> &Sha256Digest {
        &self.command_binding_sha256
    }

    pub const fn proof_receipt_sha256(&self) -> &Sha256Digest {
        &self.proof_receipt_sha256
    }

    pub const fn proof_terminal(&self) -> &ProofTerminal {
        &self.proof_terminal
    }

    /// Returns an unkeyed self-consistency digest, not an external anchor.
    pub const fn lineage_sha256(&self) -> &Sha256Digest {
        &self.lineage_sha256
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != LOCAL_PROOF_PROVENANCE_LINEAGE_SCHEMA_VERSION {
            return Err("unsupported local proof provenance lineage schema version".to_string());
        }
        self.context.validate()?;
        if self.proof_schema_version != PROOF_SCHEMA_VERSION {
            return Err("unsupported proof receipt schema version in lineage".to_string());
        }
        if self.proof_context_origin != ProofContextOrigin::HistoricalStoreResolved {
            return Err(
                "local proof provenance requires store-resolved context origin".to_string(),
            );
        }
        ProofInvocationId::parse(self.proof_invocation_id.as_str())?;
        ProofReceiptId::parse(self.proof_receipt_id.as_str())?;
        Sha256Digest::parse(self.command_binding_sha256.as_str())?;
        Sha256Digest::parse(self.proof_receipt_sha256.as_str())?;
        Sha256Digest::parse(self.lineage_sha256.as_str())?;
        if !successful_terminal(&self.proof_terminal) {
            return Err("proof lineage requires a successful completed terminal".to_string());
        }
        if self.proof_receipt_id != ProofReceiptId::for_invocation(&self.proof_invocation_id) {
            return Err("proof lineage receipt ID does not match its invocation".to_string());
        }
        if self.lineage_sha256 != self.expected_lineage_sha256() {
            return Err("proof lineage digest does not match its bindings".to_string());
        }
        Ok(())
    }

    fn expected_lineage_sha256(&self) -> Sha256Digest {
        let schema_version = self.schema_version.to_string();
        let proof_schema_version = self.proof_schema_version.to_string();
        length_delimited_sha256([
            LOCAL_PROOF_PROVENANCE_LINEAGE_DOMAIN,
            schema_version.as_str(),
            self.context.context_sha256().as_str(),
            proof_schema_version.as_str(),
            self.proof_context_origin.as_wire_str(),
            self.proof_invocation_id.as_str(),
            self.proof_receipt_id.as_str(),
            self.command_binding_sha256.as_str(),
            self.proof_receipt_sha256.as_str(),
            "completed_success",
        ])
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LocalProofProvenanceLineageWire {
    schema_version: u32,
    context: ProofProvenanceContext,
    proof_schema_version: u32,
    proof_context_origin: ProofContextOrigin,
    proof_invocation_id: ProofInvocationId,
    proof_receipt_id: ProofReceiptId,
    command_binding_sha256: Sha256Digest,
    proof_receipt_sha256: Sha256Digest,
    proof_terminal: ProofTerminal,
    lineage_sha256: Sha256Digest,
}

impl<'de> Deserialize<'de> for LocalProofProvenanceLineage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = LocalProofProvenanceLineageWire::deserialize(deserializer)?;
        let lineage = Self {
            schema_version: wire.schema_version,
            context: wire.context,
            proof_schema_version: wire.proof_schema_version,
            proof_context_origin: wire.proof_context_origin,
            proof_invocation_id: wire.proof_invocation_id,
            proof_receipt_id: wire.proof_receipt_id,
            command_binding_sha256: wire.command_binding_sha256,
            proof_receipt_sha256: wire.proof_receipt_sha256,
            proof_terminal: wire.proof_terminal,
            lineage_sha256: wire.lineage_sha256,
        };
        lineage.validate().map_err(serde::de::Error::custom)?;
        Ok(lineage)
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
        let subject = ProofSubject::new_historical_store_resolved(
            context.candidate_sha256.clone(),
            context.context_sha256.clone(),
        )
        .map_err(ProofError::InvalidInput)?;
        Ok(Self::new_historical_store_resolved(subject, nonce, command))
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
    let context = resolve_historical_context(evidence_store, snapshot, selector).await?;
    let invocation = ProofInvocation::new_with_provenance(&context, nonce, command)?;
    harness.run(invocation).await
}

/// Rereads one positive historical record and one successful proof receipt to
/// construct an exact local association.
///
/// The reads are not one cross-store transaction and neither local root is an
/// anti-rollback anchor. The result also does not prove that the candidate
/// caused the historical evidence or that execution was hermetic.
pub async fn build_local_proof_provenance_lineage(
    proof_store: &ProofStore,
    evidence_store: &HeptaEvidenceStore,
    snapshot: &MigrationFamilySnapshot,
    selector: &HistoricalEvidenceSelector,
    receipt_id: &ProofReceiptId,
) -> Result<LocalProofProvenanceLineage, ProofError> {
    let context = resolve_historical_context(evidence_store, snapshot, selector).await?;
    let receipt = proof_store.get_receipt(receipt_id)?.ok_or_else(|| {
        ProofError::InvalidInput("proof provenance receipt is not present".to_string())
    })?;
    LocalProofProvenanceLineage::from_context_and_receipt(context, &receipt)
}

async fn resolve_historical_context(
    evidence_store: &HeptaEvidenceStore,
    snapshot: &MigrationFamilySnapshot,
    selector: &HistoricalEvidenceSelector,
) -> Result<ProofProvenanceContext, ProofError> {
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
    ProofProvenanceContext::from_record(snapshot, &record)
}

fn successful_terminal(terminal: &ProofTerminal) -> bool {
    match terminal {
        ProofTerminal::Completed {
            success: true,
            exit_code: Some(0),
        } => true,
        ProofTerminal::Completed { .. }
        | ProofTerminal::TimedOut
        | ProofTerminal::OutputLimitExceeded { .. }
        | ProofTerminal::NotStarted { .. }
        | ProofTerminal::Indeterminate { .. } => false,
    }
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

#[cfg(test)]
#[path = "provenance_tests.rs"]
mod tests;
