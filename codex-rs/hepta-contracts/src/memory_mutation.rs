use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::de::Error as _;
use sha2::Digest as _;
use sha2::Sha256;

use crate::MEMORY_CONTRACT_SCHEMA_VERSION;
use crate::MemoryId;
use crate::MemoryLifecycle;
use crate::MemoryRevision;
use crate::MemoryScope;
use crate::MemorySourceKind;
use crate::RevisionStamp;
use crate::Sha256Digest;

pub const MEMORY_MUTATION_SCHEMA_VERSION: u32 = 1;

const MAX_REASON_CODE_BYTES: usize = 64;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct MemoryMutationProposalId(String);

impl MemoryMutationProposalId {
    pub fn parse(value: impl Into<String>) -> Result<Self, String> {
        crate::stable_id::parse_prefixed_sha256_id(
            value,
            "memory-mutation:v1:",
            "memory mutation proposal",
        )
        .map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for MemoryMutationProposalId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::parse(String::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct MemoryMutationReasonCode(String);

impl MemoryMutationReasonCode {
    pub fn parse(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        let mut bytes = value.bytes();
        let Some(first) = bytes.next() else {
            return Err(invalid_reason_code());
        };
        if value.len() > MAX_REASON_CODE_BYTES
            || !first.is_ascii_lowercase()
            || !bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        {
            return Err(invalid_reason_code());
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for MemoryMutationReasonCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::parse(String::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum MemoryMutationOperation {
    Create,
    Supersede {
        expected_memory_id: MemoryId,
        expected_revision: RevisionStamp,
    },
    Tombstone {
        expected_memory_id: MemoryId,
        expected_revision: RevisionStamp,
        reason_code: MemoryMutationReasonCode,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MemoryMutationProposal {
    pub schema_version: u32,
    pub proposal_id: MemoryMutationProposalId,
    pub turn_sha256: Sha256Digest,
    /// Host attribution binding only; this digest is not executable authority.
    pub proposer_binding_sha256: Sha256Digest,
    pub scope: MemoryScope,
    pub operation: MemoryMutationOperation,
    pub candidate: Option<MemoryRevision>,
}

impl MemoryMutationProposal {
    pub fn create(
        turn_id: &str,
        proposer_binding_sha256: Sha256Digest,
        candidate: MemoryRevision,
        canonical_content: &[u8],
    ) -> Result<Self, String> {
        Self::new(
            turn_id,
            proposer_binding_sha256,
            candidate.scope.clone(),
            MemoryMutationOperation::Create,
            Some(candidate),
            Some(canonical_content),
        )
    }

    pub fn supersede(
        turn_id: &str,
        proposer_binding_sha256: Sha256Digest,
        expected_memory_id: MemoryId,
        expected_revision: RevisionStamp,
        candidate: MemoryRevision,
        canonical_content: &[u8],
    ) -> Result<Self, String> {
        Self::new(
            turn_id,
            proposer_binding_sha256,
            candidate.scope.clone(),
            MemoryMutationOperation::Supersede {
                expected_memory_id,
                expected_revision,
            },
            Some(candidate),
            Some(canonical_content),
        )
    }

    pub fn tombstone(
        turn_id: &str,
        proposer_binding_sha256: Sha256Digest,
        scope: MemoryScope,
        expected_memory_id: MemoryId,
        expected_revision: RevisionStamp,
        reason_code: MemoryMutationReasonCode,
    ) -> Result<Self, String> {
        Self::new(
            turn_id,
            proposer_binding_sha256,
            scope,
            MemoryMutationOperation::Tombstone {
                expected_memory_id,
                expected_revision,
                reason_code,
            },
            None,
            None,
        )
    }

    pub fn validate(&self, canonical_content: Option<&[u8]>) -> Result<(), String> {
        self.validate_structure()?;
        match (&self.operation, self.candidate.as_ref(), canonical_content) {
            (
                MemoryMutationOperation::Create | MemoryMutationOperation::Supersede { .. },
                Some(candidate),
                Some(content),
            ) => candidate.validate_content_binding(content),
            (MemoryMutationOperation::Tombstone { .. }, None, None) => Ok(()),
            _ => Err("memory mutation candidate content does not match its operation".to_string()),
        }
    }

    /// Validates the serialized proposal binding without materializing private content.
    ///
    /// This is sufficient for digest-only evidence readback. It is not sufficient to
    /// authorize or execute a mutation: create and supersede execution must still call
    /// [`Self::validate`] with the trusted canonical candidate content.
    pub fn validate_binding(&self) -> Result<(), String> {
        self.validate_structure()
    }

    fn new(
        turn_id: &str,
        proposer_binding_sha256: Sha256Digest,
        scope: MemoryScope,
        operation: MemoryMutationOperation,
        candidate: Option<MemoryRevision>,
        canonical_content: Option<&[u8]>,
    ) -> Result<Self, String> {
        if turn_id.trim().is_empty() {
            return Err("memory mutation requires a non-empty turn identity".to_string());
        }
        let mut proposal = Self {
            schema_version: MEMORY_MUTATION_SCHEMA_VERSION,
            proposal_id: MemoryMutationProposalId(String::new()),
            turn_sha256: Sha256Digest::for_bytes(turn_id.as_bytes()),
            proposer_binding_sha256,
            scope,
            operation,
            candidate,
        };
        proposal.proposal_id = proposal.expected_proposal_id();
        proposal.validate(canonical_content)?;
        Ok(proposal)
    }

    fn validate_structure(&self) -> Result<(), String> {
        if self.schema_version != MEMORY_MUTATION_SCHEMA_VERSION {
            return Err("unsupported memory mutation schema version".to_string());
        }
        match (&self.operation, self.candidate.as_ref()) {
            (MemoryMutationOperation::Create, Some(candidate)) => {
                validate_candidate(&self.scope, candidate)?;
            }
            (
                MemoryMutationOperation::Supersede {
                    expected_memory_id,
                    expected_revision,
                },
                Some(candidate),
            ) => {
                validate_expected_revision(expected_revision)?;
                validate_candidate(&self.scope, candidate)?;
                if expected_memory_id == &candidate.memory_id {
                    return Err(
                        "memory supersession requires a distinct successor identity".to_string()
                    );
                }
            }
            (
                MemoryMutationOperation::Tombstone {
                    expected_revision, ..
                },
                None,
            ) => validate_expected_revision(expected_revision)?,
            _ => {
                return Err(
                    "memory mutation candidate presence does not match its operation".to_string(),
                );
            }
        }
        if self.proposal_id != self.expected_proposal_id() {
            return Err("memory mutation proposal identity does not match its binding".to_string());
        }
        Ok(())
    }

    fn expected_proposal_id(&self) -> MemoryMutationProposalId {
        MemoryMutationProposalId(format!(
            "memory-mutation:v1:{}",
            digest_parts([
                self.schema_version.to_string(),
                self.turn_sha256.as_str().to_string(),
                self.proposer_binding_sha256.as_str().to_string(),
                self.scope.binding_sha256().as_str().to_string(),
                operation_binding(&self.operation),
                self.candidate
                    .as_ref()
                    .map_or_else(|| "candidate:absent".to_string(), revision_binding),
            ])
        ))
    }
}

#[derive(Deserialize)]
struct MemoryMutationProposalWire {
    schema_version: u32,
    proposal_id: MemoryMutationProposalId,
    turn_sha256: Sha256Digest,
    proposer_binding_sha256: Sha256Digest,
    scope: MemoryScope,
    operation: MemoryMutationOperation,
    candidate: Option<MemoryRevision>,
}

impl<'de> Deserialize<'de> for MemoryMutationProposal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = MemoryMutationProposalWire::deserialize(deserializer)?;
        let proposal = Self {
            schema_version: wire.schema_version,
            proposal_id: wire.proposal_id,
            turn_sha256: wire.turn_sha256,
            proposer_binding_sha256: wire.proposer_binding_sha256,
            scope: wire.scope,
            operation: wire.operation,
            candidate: wire.candidate,
        };
        proposal.validate_structure().map_err(D::Error::custom)?;
        Ok(proposal)
    }
}

fn validate_candidate(scope: &MemoryScope, candidate: &MemoryRevision) -> Result<(), String> {
    if candidate.schema_version != MEMORY_CONTRACT_SCHEMA_VERSION {
        return Err("memory mutation candidate uses an unsupported memory schema".to_string());
    }
    if &candidate.scope != scope {
        return Err("memory mutation candidate scope does not match the proposal".to_string());
    }
    if !matches!(candidate.lifecycle, MemoryLifecycle::Active) {
        return Err("memory mutation candidate must be active".to_string());
    }
    validate_expected_revision(&candidate.revision)?;
    validate_expected_revision(&candidate.provenance.source_revision)
}

fn validate_expected_revision(revision: &RevisionStamp) -> Result<(), String> {
    if revision.revision == 0 {
        return Err("memory mutation revisions must be positive".to_string());
    }
    Ok(())
}

fn operation_binding(operation: &MemoryMutationOperation) -> String {
    match operation {
        MemoryMutationOperation::Create => "create".to_string(),
        MemoryMutationOperation::Supersede {
            expected_memory_id,
            expected_revision,
        } => digest_parts([
            "supersede".to_string(),
            expected_memory_id.as_str().to_string(),
            revision_stamp_binding(expected_revision),
        ]),
        MemoryMutationOperation::Tombstone {
            expected_memory_id,
            expected_revision,
            reason_code,
        } => digest_parts([
            "tombstone".to_string(),
            expected_memory_id.as_str().to_string(),
            revision_stamp_binding(expected_revision),
            reason_code.as_str().to_string(),
        ]),
    }
}

pub(crate) fn revision_binding(revision: &MemoryRevision) -> String {
    let valid_until = revision
        .valid_until_unix_seconds
        .map_or_else(|| "none".to_string(), |value| value.to_string());
    digest_parts([
        revision.schema_version.to_string(),
        revision.memory_id.as_str().to_string(),
        revision_stamp_binding(&revision.revision),
        revision.scope.binding_sha256().as_str().to_string(),
        memory_source_kind(revision.provenance.source_kind).to_string(),
        revision.provenance.source_id_sha256.as_str().to_string(),
        revision_stamp_binding(&revision.provenance.source_revision),
        revision.provenance.observed_at_unix_seconds.to_string(),
        lifecycle_binding(&revision.lifecycle),
        valid_until,
    ])
}

fn revision_stamp_binding(revision: &RevisionStamp) -> String {
    digest_parts([
        revision.revision.to_string(),
        revision.content_sha256.as_str().to_string(),
    ])
}

fn memory_source_kind(source: MemorySourceKind) -> &'static str {
    match source {
        MemorySourceKind::CodexStage1Summary => "codex_stage1_summary",
        MemorySourceKind::ReviewedHeptaMemory => "reviewed_hepta_memory",
        MemorySourceKind::LocalKgEpisode => "local_kg_episode",
    }
}

fn lifecycle_binding(lifecycle: &MemoryLifecycle) -> String {
    match lifecycle {
        MemoryLifecycle::Active => "active".to_string(),
        MemoryLifecycle::Superseded {
            successor_memory_id,
        } => format!("superseded:{}", successor_memory_id.as_str()),
        MemoryLifecycle::Tombstoned { reason_code } => format!("tombstoned:{reason_code}"),
        MemoryLifecycle::Expired {
            expired_at_unix_seconds,
        } => format!("expired:{expired_at_unix_seconds}"),
    }
}

fn digest_parts(parts: impl IntoIterator<Item = String>) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn invalid_reason_code() -> String {
    format!(
        "memory mutation reason code must match [a-z][a-z0-9_]{{0,{}}}",
        MAX_REASON_CODE_BYTES - 1
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryProvenance;

    fn digest(value: &str) -> Sha256Digest {
        Sha256Digest::for_bytes(value.as_bytes())
    }

    fn scope() -> MemoryScope {
        MemoryScope {
            installation_sha256: digest("installation"),
            workspace_sha256: digest("workspace"),
            thread_sha256: digest("thread"),
            principal_sha256: digest("principal"),
        }
    }

    fn candidate(content: &[u8], source_revision: u64) -> MemoryRevision {
        let scope = scope();
        MemoryRevision {
            schema_version: MEMORY_CONTRACT_SCHEMA_VERSION,
            memory_id: MemoryId::for_content(&scope, content),
            revision: RevisionStamp::new(1, content),
            scope,
            provenance: MemoryProvenance {
                source_kind: MemorySourceKind::ReviewedHeptaMemory,
                source_id_sha256: digest("source"),
                source_revision: RevisionStamp {
                    revision: source_revision,
                    content_sha256: digest(format!("source-{source_revision}").as_str()),
                },
                observed_at_unix_seconds: 100,
            },
            lifecycle: MemoryLifecycle::Active,
            valid_until_unix_seconds: None,
        }
    }

    #[test]
    fn create_proposal_is_exact_bound_without_serializing_content() {
        let proposal = MemoryMutationProposal::create(
            "turn-1",
            digest("proposer"),
            candidate(b"private reviewed memory", 1),
            b"private reviewed memory",
        )
        .expect("valid create proposal");

        assert!(proposal.validate(Some(b"private reviewed memory")).is_ok());
        assert!(
            proposal
                .proposal_id
                .as_str()
                .starts_with("memory-mutation:v1:")
        );
        let serialized = serde_json::to_string(&proposal).expect("serialize proposal");
        assert!(!serialized.contains("private reviewed memory"));
    }

    #[test]
    fn proposal_substitution_and_invalid_candidate_fail_closed() {
        let proposal = MemoryMutationProposal::create(
            "turn-1",
            digest("proposer"),
            candidate(b"candidate", 1),
            b"candidate",
        )
        .expect("valid create proposal");
        assert!(proposal.validate(Some(b"substituted")).is_err());

        let mut serialized = serde_json::to_value(proposal).expect("serialize proposal");
        serialized["turn_sha256"] = serde_json::Value::String("0".repeat(64));
        assert!(serde_json::from_value::<MemoryMutationProposal>(serialized).is_err());
    }

    #[test]
    fn supersede_and_tombstone_require_exact_positive_preconditions() {
        let current = candidate(b"current", 1);
        let successor = candidate(b"successor", 2);
        assert!(
            MemoryMutationProposal::supersede(
                "turn-1",
                digest("proposer"),
                current.memory_id.clone(),
                current.revision.clone(),
                successor,
                b"successor",
            )
            .is_ok()
        );
        assert!(
            MemoryMutationProposal::tombstone(
                "turn-1",
                digest("proposer"),
                current.scope.clone(),
                current.memory_id,
                current.revision,
                MemoryMutationReasonCode::parse("operator_delete").expect("reason"),
            )
            .is_ok()
        );
        assert!(MemoryMutationReasonCode::parse("Operator Delete").is_err());
    }
}
