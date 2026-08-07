use crate::MemoryMutationOperation;
use crate::MemoryMutationProposal;
use crate::MemoryMutationProposalId;
use crate::Sha256Digest;
use crate::canonical::length_delimited_sha256;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::de::Error as _;

pub const MEMORY_MUTATION_DRY_RUN_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct MemoryMutationDryRunId(String);

impl MemoryMutationDryRunId {
    pub fn parse(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        let Some(digest) = value.strip_prefix("memory-mutation-dry-run:v1:") else {
            return Err("memory mutation dry-run id has an invalid prefix".to_string());
        };
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(
                "memory mutation dry-run id requires a lowercase SHA-256 digest".to_string(),
            );
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for MemoryMutationDryRunId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::parse(String::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryMutationDryRunDisposition {
    WouldCreate,
    WouldSupersede,
    WouldTombstone,
    NoOp,
    Blocked,
}

impl MemoryMutationDryRunDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WouldCreate => "would_create",
            Self::WouldSupersede => "would_supersede",
            Self::WouldTombstone => "would_tombstone",
            Self::NoOp => "no_op",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryMutationDryRunReason {
    Ready,
    ExactRevisionAlreadyPresent,
    ProposalInvalid,
    UnexpectedExistingRevision,
    ExpectedRevisionMissing,
    ScopeMismatch,
    RevisionMismatch,
    CurrentRevisionInvalid,
    CurrentRevisionInactive,
    SourceBindingMismatch,
    SourceRevisionNotNewer,
}

impl MemoryMutationDryRunReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::ExactRevisionAlreadyPresent => "exact_revision_already_present",
            Self::ProposalInvalid => "proposal_invalid",
            Self::UnexpectedExistingRevision => "unexpected_existing_revision",
            Self::ExpectedRevisionMissing => "expected_revision_missing",
            Self::ScopeMismatch => "scope_mismatch",
            Self::RevisionMismatch => "revision_mismatch",
            Self::CurrentRevisionInvalid => "current_revision_invalid",
            Self::CurrentRevisionInactive => "current_revision_inactive",
            Self::SourceBindingMismatch => "source_binding_mismatch",
            Self::SourceRevisionNotNewer => "source_revision_not_newer",
        }
    }
}

/// Digest-only projection of one mutation simulation.
///
/// Projected Memory writes describe the future transactional shape. This
/// contract contains no writer, provider, network, or KG execution handle.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MemoryMutationDryRun {
    pub schema_version: u32,
    pub dry_run_id: MemoryMutationDryRunId,
    pub proposal_id: MemoryMutationProposalId,
    pub snapshot_sha256: Sha256Digest,
    pub disposition: MemoryMutationDryRunDisposition,
    pub reason: MemoryMutationDryRunReason,
    pub projected_memory_writes: u32,
}

impl MemoryMutationDryRun {
    pub fn has_integrity(&self) -> bool {
        let expected_projected_writes = match self.disposition {
            MemoryMutationDryRunDisposition::WouldCreate
            | MemoryMutationDryRunDisposition::WouldTombstone => 1,
            MemoryMutationDryRunDisposition::WouldSupersede => 2,
            MemoryMutationDryRunDisposition::NoOp | MemoryMutationDryRunDisposition::Blocked => 0,
        };
        self.schema_version == MEMORY_MUTATION_DRY_RUN_SCHEMA_VERSION
            && self.projected_memory_writes == expected_projected_writes
            && self.dry_run_id
                == expected_dry_run_id(
                    &self.proposal_id,
                    &self.snapshot_sha256,
                    self.disposition,
                    self.reason,
                    self.projected_memory_writes,
                )
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.has_integrity() {
            Ok(())
        } else {
            Err("memory mutation dry-run integrity check failed".to_string())
        }
    }

    pub fn validate_for_proposal(&self, proposal: &MemoryMutationProposal) -> Result<(), String> {
        self.validate()?;
        proposal.validate_binding()?;
        if self.proposal_id != proposal.proposal_id {
            return Err("memory mutation dry-run does not bind its proposal".to_string());
        }
        let valid_semantics = matches!(
            (&proposal.operation, self.disposition, self.reason,),
            (
                MemoryMutationOperation::Create,
                MemoryMutationDryRunDisposition::WouldCreate,
                MemoryMutationDryRunReason::Ready,
            ) | (
                MemoryMutationOperation::Supersede { .. },
                MemoryMutationDryRunDisposition::WouldSupersede,
                MemoryMutationDryRunReason::Ready,
            ) | (
                MemoryMutationOperation::Tombstone { .. },
                MemoryMutationDryRunDisposition::WouldTombstone,
                MemoryMutationDryRunReason::Ready,
            ) | (
                MemoryMutationOperation::Create,
                MemoryMutationDryRunDisposition::NoOp,
                MemoryMutationDryRunReason::ExactRevisionAlreadyPresent,
            ) | (
                MemoryMutationOperation::Create,
                MemoryMutationDryRunDisposition::Blocked,
                MemoryMutationDryRunReason::ProposalInvalid
                    | MemoryMutationDryRunReason::CurrentRevisionInvalid
                    | MemoryMutationDryRunReason::UnexpectedExistingRevision,
            ) | (
                MemoryMutationOperation::Supersede { .. },
                MemoryMutationDryRunDisposition::Blocked,
                MemoryMutationDryRunReason::ProposalInvalid
                    | MemoryMutationDryRunReason::CurrentRevisionInvalid
                    | MemoryMutationDryRunReason::ExpectedRevisionMissing
                    | MemoryMutationDryRunReason::ScopeMismatch
                    | MemoryMutationDryRunReason::RevisionMismatch
                    | MemoryMutationDryRunReason::CurrentRevisionInactive
                    | MemoryMutationDryRunReason::SourceBindingMismatch
                    | MemoryMutationDryRunReason::SourceRevisionNotNewer,
            ) | (
                MemoryMutationOperation::Tombstone { .. },
                MemoryMutationDryRunDisposition::Blocked,
                MemoryMutationDryRunReason::ProposalInvalid
                    | MemoryMutationDryRunReason::CurrentRevisionInvalid
                    | MemoryMutationDryRunReason::ExpectedRevisionMissing
                    | MemoryMutationDryRunReason::ScopeMismatch
                    | MemoryMutationDryRunReason::RevisionMismatch
                    | MemoryMutationDryRunReason::CurrentRevisionInactive,
            )
        );
        if valid_semantics {
            Ok(())
        } else {
            Err("memory mutation dry-run semantics do not match its proposal".to_string())
        }
    }
}

#[derive(Deserialize)]
struct MemoryMutationDryRunWire {
    schema_version: u32,
    dry_run_id: MemoryMutationDryRunId,
    proposal_id: MemoryMutationProposalId,
    snapshot_sha256: Sha256Digest,
    disposition: MemoryMutationDryRunDisposition,
    reason: MemoryMutationDryRunReason,
    projected_memory_writes: u32,
}

impl<'de> Deserialize<'de> for MemoryMutationDryRun {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = MemoryMutationDryRunWire::deserialize(deserializer)?;
        let dry_run = Self {
            schema_version: wire.schema_version,
            dry_run_id: wire.dry_run_id,
            proposal_id: wire.proposal_id,
            snapshot_sha256: wire.snapshot_sha256,
            disposition: wire.disposition,
            reason: wire.reason,
            projected_memory_writes: wire.projected_memory_writes,
        };
        dry_run.validate().map_err(D::Error::custom)?;
        Ok(dry_run)
    }
}

/// Builds the canonical, zero-effect evidence projection selected by the Memory evaluator.
pub fn canonical_memory_mutation_dry_run(
    proposal_id: MemoryMutationProposalId,
    snapshot_sha256: Sha256Digest,
    disposition: MemoryMutationDryRunDisposition,
    reason: MemoryMutationDryRunReason,
    projected_memory_writes: u32,
) -> MemoryMutationDryRun {
    let dry_run_id = expected_dry_run_id(
        &proposal_id,
        &snapshot_sha256,
        disposition,
        reason,
        projected_memory_writes,
    );
    MemoryMutationDryRun {
        schema_version: MEMORY_MUTATION_DRY_RUN_SCHEMA_VERSION,
        dry_run_id,
        proposal_id,
        snapshot_sha256,
        disposition,
        reason,
        projected_memory_writes,
    }
}

fn expected_dry_run_id(
    proposal_id: &MemoryMutationProposalId,
    snapshot_sha256: &Sha256Digest,
    disposition: MemoryMutationDryRunDisposition,
    reason: MemoryMutationDryRunReason,
    projected_memory_writes: u32,
) -> MemoryMutationDryRunId {
    let projected_memory_writes = projected_memory_writes.to_string();
    MemoryMutationDryRunId(format!(
        "memory-mutation-dry-run:v1:{}",
        length_delimited_sha256([
            proposal_id.as_str(),
            snapshot_sha256.as_str(),
            disposition.as_str(),
            reason.as_str(),
            projected_memory_writes.as_str(),
        ])
        .as_str()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MEMORY_CONTRACT_SCHEMA_VERSION;
    use crate::MemoryId;
    use crate::MemoryLifecycle;
    use crate::MemoryProvenance;
    use crate::MemoryRevision;
    use crate::MemoryScope;
    use crate::MemorySourceKind;
    use crate::RevisionStamp;

    fn digest(value: &str) -> Sha256Digest {
        Sha256Digest::for_bytes(value.as_bytes())
    }

    fn create_proposal() -> MemoryMutationProposal {
        let content = b"private reviewed memory";
        let scope = MemoryScope {
            installation_sha256: digest("installation"),
            workspace_sha256: digest("workspace"),
            thread_sha256: digest("thread"),
            principal_sha256: digest("principal"),
        };
        let candidate = MemoryRevision {
            schema_version: MEMORY_CONTRACT_SCHEMA_VERSION,
            memory_id: MemoryId::for_content(&scope, content),
            revision: RevisionStamp::new(1, content),
            scope,
            provenance: MemoryProvenance {
                source_kind: MemorySourceKind::ReviewedHeptaMemory,
                source_id_sha256: digest("source"),
                source_revision: RevisionStamp {
                    revision: 1,
                    content_sha256: digest("source-1"),
                },
                observed_at_unix_seconds: 100,
            },
            lifecycle: MemoryLifecycle::Active,
            valid_until_unix_seconds: None,
        };
        MemoryMutationProposal::create("turn-1", digest("proposer"), candidate, content)
            .expect("valid create proposal")
    }

    #[test]
    fn dry_run_id_has_a_fixed_canonical_oracle() {
        let proposal_id =
            MemoryMutationProposalId::parse(format!("memory-mutation:v1:{}", "a".repeat(64)))
                .expect("proposal id");
        let dry_run = canonical_memory_mutation_dry_run(
            proposal_id,
            digest("snapshot"),
            MemoryMutationDryRunDisposition::WouldCreate,
            MemoryMutationDryRunReason::Ready,
            1,
        );

        assert_eq!(
            dry_run.dry_run_id.as_str(),
            "memory-mutation-dry-run:v1:66341eee89a0b537bfe6c796db43121c401ece4c37278d523096b3eb0819d33c"
        );
        assert!(dry_run.validate().is_ok());
    }

    #[test]
    fn dry_run_deserialization_rejects_projection_and_schema_substitution() {
        let proposal = create_proposal();
        let dry_run = canonical_memory_mutation_dry_run(
            proposal.proposal_id,
            digest("snapshot"),
            MemoryMutationDryRunDisposition::WouldCreate,
            MemoryMutationDryRunReason::Ready,
            1,
        );
        let mut serialized = serde_json::to_value(&dry_run).expect("serialize dry-run");
        serialized["projected_memory_writes"] = serde_json::json!(2);
        assert!(serde_json::from_value::<MemoryMutationDryRun>(serialized).is_err());

        let mut serialized = serde_json::to_value(&dry_run).expect("serialize dry-run");
        serialized["schema_version"] = serde_json::json!(2);
        assert!(serde_json::from_value::<MemoryMutationDryRun>(serialized).is_err());
    }

    #[test]
    fn self_consistent_dry_run_with_impossible_operation_semantics_is_rejected() {
        let proposal = create_proposal();
        let dry_run = canonical_memory_mutation_dry_run(
            proposal.proposal_id.clone(),
            digest("snapshot"),
            MemoryMutationDryRunDisposition::WouldTombstone,
            MemoryMutationDryRunReason::Ready,
            1,
        );

        assert!(dry_run.validate().is_ok());
        assert!(dry_run.validate_for_proposal(&proposal).is_err());
    }
}
