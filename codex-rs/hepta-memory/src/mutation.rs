use codex_hepta_contracts::MEMORY_CONTRACT_SCHEMA_VERSION;
use codex_hepta_contracts::MemoryLifecycle;
use codex_hepta_contracts::MemoryMutationDryRun;
use codex_hepta_contracts::MemoryMutationDryRunDisposition;
use codex_hepta_contracts::MemoryMutationDryRunReason;
use codex_hepta_contracts::MemoryMutationOperation;
use codex_hepta_contracts::MemoryMutationProposal;
use codex_hepta_contracts::MemoryRevision;
use codex_hepta_contracts::MemorySourceKind;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::canonical_memory_mutation_dry_run;
use sha2::Digest as _;
use sha2::Sha256;

use crate::framing::frame_part;

/// Simulates one exact-CAS memory mutation without owning a writer.
pub fn dry_run_memory_mutation(
    proposal: &MemoryMutationProposal,
    canonical_candidate_content: Option<&[u8]>,
    current: Option<&MemoryRevision>,
) -> MemoryMutationDryRun {
    let snapshot_sha256 = snapshot_sha256(current);
    let (disposition, reason, projected_memory_writes) =
        if proposal.validate(canonical_candidate_content).is_err() {
            (
                MemoryMutationDryRunDisposition::Blocked,
                MemoryMutationDryRunReason::ProposalInvalid,
                0,
            )
        } else {
            evaluate_valid_proposal(proposal, current)
        };
    canonical_memory_mutation_dry_run(
        proposal.proposal_id.clone(),
        snapshot_sha256,
        disposition,
        reason,
        projected_memory_writes,
    )
}

fn evaluate_valid_proposal(
    proposal: &MemoryMutationProposal,
    current: Option<&MemoryRevision>,
) -> (
    MemoryMutationDryRunDisposition,
    MemoryMutationDryRunReason,
    u32,
) {
    if current.is_some_and(|revision| {
        revision.schema_version != MEMORY_CONTRACT_SCHEMA_VERSION
            || revision.revision.revision == 0
            || revision.provenance.source_revision.revision == 0
    }) {
        return (
            MemoryMutationDryRunDisposition::Blocked,
            MemoryMutationDryRunReason::CurrentRevisionInvalid,
            0,
        );
    }
    match &proposal.operation {
        MemoryMutationOperation::Create => match current {
            None => (
                MemoryMutationDryRunDisposition::WouldCreate,
                MemoryMutationDryRunReason::Ready,
                1,
            ),
            Some(current) if proposal.candidate.as_ref() == Some(current) => (
                MemoryMutationDryRunDisposition::NoOp,
                MemoryMutationDryRunReason::ExactRevisionAlreadyPresent,
                0,
            ),
            Some(_) => (
                MemoryMutationDryRunDisposition::Blocked,
                MemoryMutationDryRunReason::UnexpectedExistingRevision,
                0,
            ),
        },
        MemoryMutationOperation::Supersede {
            expected_memory_id,
            expected_revision,
        } => {
            let Some(current) = current else {
                return (
                    MemoryMutationDryRunDisposition::Blocked,
                    MemoryMutationDryRunReason::ExpectedRevisionMissing,
                    0,
                );
            };
            if current.scope != proposal.scope {
                return (
                    MemoryMutationDryRunDisposition::Blocked,
                    MemoryMutationDryRunReason::ScopeMismatch,
                    0,
                );
            }
            if &current.memory_id != expected_memory_id || &current.revision != expected_revision {
                return (
                    MemoryMutationDryRunDisposition::Blocked,
                    MemoryMutationDryRunReason::RevisionMismatch,
                    0,
                );
            }
            if !matches!(current.lifecycle, MemoryLifecycle::Active) {
                return (
                    MemoryMutationDryRunDisposition::Blocked,
                    MemoryMutationDryRunReason::CurrentRevisionInactive,
                    0,
                );
            }
            let Some(candidate) = proposal.candidate.as_ref() else {
                return (
                    MemoryMutationDryRunDisposition::Blocked,
                    MemoryMutationDryRunReason::ProposalInvalid,
                    0,
                );
            };
            if candidate.provenance.source_kind != current.provenance.source_kind
                || candidate.provenance.source_id_sha256 != current.provenance.source_id_sha256
            {
                return (
                    MemoryMutationDryRunDisposition::Blocked,
                    MemoryMutationDryRunReason::SourceBindingMismatch,
                    0,
                );
            }
            if candidate.provenance.source_revision.revision
                <= current.provenance.source_revision.revision
            {
                return (
                    MemoryMutationDryRunDisposition::Blocked,
                    MemoryMutationDryRunReason::SourceRevisionNotNewer,
                    0,
                );
            }
            (
                MemoryMutationDryRunDisposition::WouldSupersede,
                MemoryMutationDryRunReason::Ready,
                2,
            )
        }
        MemoryMutationOperation::Tombstone {
            expected_memory_id,
            expected_revision,
            ..
        } => {
            let Some(current) = current else {
                return (
                    MemoryMutationDryRunDisposition::Blocked,
                    MemoryMutationDryRunReason::ExpectedRevisionMissing,
                    0,
                );
            };
            if current.scope != proposal.scope {
                return (
                    MemoryMutationDryRunDisposition::Blocked,
                    MemoryMutationDryRunReason::ScopeMismatch,
                    0,
                );
            }
            if &current.memory_id != expected_memory_id || &current.revision != expected_revision {
                return (
                    MemoryMutationDryRunDisposition::Blocked,
                    MemoryMutationDryRunReason::RevisionMismatch,
                    0,
                );
            }
            if !matches!(current.lifecycle, MemoryLifecycle::Active) {
                return (
                    MemoryMutationDryRunDisposition::Blocked,
                    MemoryMutationDryRunReason::CurrentRevisionInactive,
                    0,
                );
            }
            (
                MemoryMutationDryRunDisposition::WouldTombstone,
                MemoryMutationDryRunReason::Ready,
                1,
            )
        }
    }
}

fn snapshot_sha256(current: Option<&MemoryRevision>) -> Sha256Digest {
    let binding = current.map_or_else(
        || "memory-snapshot:absent".to_string(),
        memory_revision_binding,
    );
    Sha256Digest::for_bytes(binding.as_bytes())
}

fn memory_revision_binding(revision: &MemoryRevision) -> String {
    let valid_until = revision
        .valid_until_unix_seconds
        .map_or_else(|| "none".to_string(), |value| value.to_string());
    let revision_number = revision.revision.revision.to_string();
    let source_revision = revision.provenance.source_revision.revision.to_string();
    let observed_at = revision.provenance.observed_at_unix_seconds.to_string();
    let schema_version = revision.schema_version.to_string();
    let lifecycle = lifecycle(&revision.lifecycle);
    let mut hasher = Sha256::new();
    for part in [
        schema_version.as_str(),
        revision.memory_id.as_str(),
        revision_number.as_str(),
        revision.revision.content_sha256.as_str(),
        revision.scope.binding_sha256().as_str(),
        source_kind(revision.provenance.source_kind),
        revision.provenance.source_id_sha256.as_str(),
        source_revision.as_str(),
        revision.provenance.source_revision.content_sha256.as_str(),
        observed_at.as_str(),
        lifecycle.as_str(),
        valid_until.as_str(),
    ] {
        frame_part(&mut hasher, part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn source_kind(source: MemorySourceKind) -> &'static str {
    match source {
        MemorySourceKind::CodexStage1Summary => "codex_stage1_summary",
        MemorySourceKind::ReviewedHeptaMemory => "reviewed_hepta_memory",
        MemorySourceKind::LocalKgEpisode => "local_kg_episode",
    }
}

fn lifecycle(lifecycle: &MemoryLifecycle) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use codex_hepta_contracts::MemoryId;
    use codex_hepta_contracts::MemoryMutationReasonCode;
    use codex_hepta_contracts::MemoryProvenance;
    use codex_hepta_contracts::MemoryScope;
    use codex_hepta_contracts::RevisionStamp;

    fn digest(value: &str) -> Sha256Digest {
        Sha256Digest::for_bytes(value.as_bytes())
    }

    fn scope(thread: &str) -> MemoryScope {
        MemoryScope {
            installation_sha256: digest("installation"),
            workspace_sha256: digest("workspace"),
            thread_sha256: digest(thread),
            principal_sha256: digest("principal"),
        }
    }

    fn revision(content: &[u8], source_revision: u64) -> MemoryRevision {
        let scope = scope("thread-1");
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
                observed_at_unix_seconds: 100 + source_revision as i64,
            },
            lifecycle: MemoryLifecycle::Active,
            valid_until_unix_seconds: None,
        }
    }

    #[test]
    fn create_is_projected_without_any_actual_effect() {
        let proposal = MemoryMutationProposal::create(
            "turn-1",
            digest("proposer"),
            revision(b"private memory", 1),
            b"private memory",
        )
        .expect("create proposal");
        let dry_run = dry_run_memory_mutation(&proposal, Some(b"private memory"), None);
        assert_eq!(
            proposal.proposal_id.as_str(),
            "memory-mutation:v1:18a119a1efadaff89696b210e461b5dcfd545f251d9232477b9f9b9e3c0f2e40"
        );
        assert_eq!(
            dry_run.snapshot_sha256.as_str(),
            "20e9074456ad7830746fce7ecab2cafdc8925f8e2cf6f627457a56a77af5af88"
        );
        assert_eq!(
            dry_run.dry_run_id.as_str(),
            "memory-mutation-dry-run:v1:3dac2af7e8abeb6df3ef40db9200ccbfa1abb2ab7ed69b83f1cf0bc63d5232bf"
        );

        assert_eq!(
            dry_run.disposition,
            MemoryMutationDryRunDisposition::WouldCreate
        );
        assert_eq!(dry_run.projected_memory_writes, 1);
        assert!(dry_run.has_integrity());
        let serialized = serde_json::to_string(&dry_run).expect("serialize dry run");
        assert!(!serialized.contains("private memory"));
        assert_eq!(
            serde_json::from_str::<MemoryMutationDryRun>(&serialized)
                .expect("deserialize exact dry run"),
            dry_run,
        );
        let mut tampered = serde_json::from_str::<serde_json::Value>(&serialized)
            .expect("deserialize dry run JSON");
        tampered["projected_memory_writes"] = serde_json::Value::from(2);
        assert!(serde_json::from_value::<MemoryMutationDryRun>(tampered).is_err());
    }

    #[test]
    fn exact_create_replay_is_noop_and_substitution_is_blocked() {
        let current = revision(b"private memory", 1);
        let proposal = MemoryMutationProposal::create(
            "turn-1",
            digest("proposer"),
            current.clone(),
            b"private memory",
        )
        .expect("create proposal");
        let replay = dry_run_memory_mutation(&proposal, Some(b"private memory"), Some(&current));
        assert_eq!(
            replay.snapshot_sha256.as_str(),
            "367c6e7416afb9e1b13ac306f8c67f1d7e674bed5be4fc405a7932959a007e4e"
        );
        assert_eq!(replay.disposition, MemoryMutationDryRunDisposition::NoOp);
        let substituted = dry_run_memory_mutation(&proposal, Some(b"substituted"), None);
        assert_eq!(
            substituted.reason,
            MemoryMutationDryRunReason::ProposalInvalid
        );
        assert!(substituted.has_integrity());
    }

    #[test]
    fn self_consistent_but_operation_impossible_results_are_rejected() {
        let proposal = MemoryMutationProposal::create(
            "turn-1",
            digest("proposer"),
            revision(b"private memory", 1),
            b"private memory",
        )
        .expect("create proposal");
        let authoritative = dry_run_memory_mutation(&proposal, Some(b"private memory"), None);
        let forged = canonical_memory_mutation_dry_run(
            authoritative.proposal_id.clone(),
            authoritative.snapshot_sha256.clone(),
            MemoryMutationDryRunDisposition::WouldTombstone,
            authoritative.reason,
            authoritative.projected_memory_writes,
        );
        assert!(forged.validate().is_ok());
        assert!(forged.validate_for_proposal(&proposal).is_err());

        let forged = canonical_memory_mutation_dry_run(
            authoritative.proposal_id,
            authoritative.snapshot_sha256,
            MemoryMutationDryRunDisposition::Blocked,
            authoritative.reason,
            0,
        );
        assert!(forged.validate().is_ok());
        assert!(forged.validate_for_proposal(&proposal).is_err());
    }

    #[test]
    fn supersede_requires_exact_active_cas_and_newer_source_revision() {
        let current = revision(b"current", 1);
        let successor = revision(b"successor", 2);
        let proposal = MemoryMutationProposal::supersede(
            "turn-1",
            digest("proposer"),
            current.memory_id.clone(),
            current.revision.clone(),
            successor,
            b"successor",
        )
        .expect("supersede proposal");
        let ready = dry_run_memory_mutation(&proposal, Some(b"successor"), Some(&current));
        assert_eq!(
            ready.disposition,
            MemoryMutationDryRunDisposition::WouldSupersede
        );
        assert_eq!(ready.projected_memory_writes, 2);

        let mut stale = current;
        stale.revision.revision += 1;
        assert_eq!(
            dry_run_memory_mutation(&proposal, Some(b"successor"), Some(&stale)).reason,
            MemoryMutationDryRunReason::RevisionMismatch
        );
    }

    #[test]
    fn supersede_rejects_source_or_revision_rotation() {
        let current = revision(b"current", 2);
        let stale_successor = revision(b"successor", 2);
        let stale = MemoryMutationProposal::supersede(
            "turn-1",
            digest("proposer"),
            current.memory_id.clone(),
            current.revision.clone(),
            stale_successor,
            b"successor",
        )
        .expect("structurally valid stale proposal");
        assert_eq!(
            dry_run_memory_mutation(&stale, Some(b"successor"), Some(&current)).reason,
            MemoryMutationDryRunReason::SourceRevisionNotNewer
        );

        let mut different_source = revision(b"different", 3);
        different_source.provenance.source_id_sha256 = digest("different-source");
        let different = MemoryMutationProposal::supersede(
            "turn-1",
            digest("proposer"),
            current.memory_id.clone(),
            current.revision.clone(),
            different_source,
            b"different",
        )
        .expect("different source proposal");
        assert_eq!(
            dry_run_memory_mutation(&different, Some(b"different"), Some(&current)).reason,
            MemoryMutationDryRunReason::SourceBindingMismatch
        );
    }

    #[test]
    fn tombstone_is_exact_cas_and_never_executes() {
        let current = revision(b"current", 1);
        let proposal = MemoryMutationProposal::tombstone(
            "turn-1",
            digest("proposer"),
            current.scope.clone(),
            current.memory_id.clone(),
            current.revision.clone(),
            MemoryMutationReasonCode::parse("operator_delete").expect("reason"),
        )
        .expect("tombstone proposal");
        let ready = dry_run_memory_mutation(&proposal, None, Some(&current));
        assert_eq!(
            ready.disposition,
            MemoryMutationDryRunDisposition::WouldTombstone
        );
        assert_eq!(ready.projected_memory_writes, 1);
        assert!(ready.has_integrity());

        let mut inactive = current;
        inactive.lifecycle = MemoryLifecycle::Tombstoned {
            reason_code: "already_deleted".to_string(),
        };
        assert_eq!(
            dry_run_memory_mutation(&proposal, None, Some(&inactive)).reason,
            MemoryMutationDryRunReason::CurrentRevisionInactive
        );

        inactive.schema_version += 1;
        assert_eq!(
            dry_run_memory_mutation(&proposal, None, Some(&inactive)).reason,
            MemoryMutationDryRunReason::CurrentRevisionInvalid
        );
    }
}
