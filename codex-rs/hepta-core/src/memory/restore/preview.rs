use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::runtime_types::SessionId;

use super::super::MemoryRecord;
use super::super::SessionRecord;
use super::super::SnapshotAuditReport;
use super::super::TranscriptEntry;
use super::delta::MemoryRestoreDelta;
use super::delta::RestoreDeltaCounts;
use super::delta::SessionRestoreDelta;
use super::delta::TranscriptRestoreDelta;
use super::domain::SnapshotRestoreDomain;
use super::domain::SnapshotRestoreDomainImpact;
use super::planning::SnapshotRestoreImpact;
use super::planning::SnapshotRestoreMutationProfile;
use super::planning::SnapshotRestoreReadiness;
use super::planning::SnapshotRestoreSafety;

/// Portable preflight view of what a full snapshot restore would change.
///
/// This complements [`SnapshotInspectionBundle`] and [`SnapshotAuditReport`]
/// with an additive diff surface so automation can preview replace-style
/// restores before mutating a concrete store.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SnapshotRestorePreview {
    pub current_audit: SnapshotAuditReport,
    pub incoming_audit: SnapshotAuditReport,
    pub session_delta: SessionRestoreDelta,
    pub memory_delta: MemoryRestoreDelta,
    pub transcript_delta: TranscriptRestoreDelta,
}

impl SnapshotRestorePreview {
    pub fn from_records_and_entries(
        current_sessions: &[SessionRecord],
        current_memories: &[MemoryRecord],
        current_transcripts: &[TranscriptEntry],
        incoming_sessions: &[SessionRecord],
        incoming_memories: &[MemoryRecord],
        incoming_transcripts: &[TranscriptEntry],
    ) -> Self {
        let current_session_map = current_sessions
            .iter()
            .map(|record| (record.session_id.0.clone(), record.clone()))
            .collect::<BTreeMap<_, _>>();
        let incoming_session_map = incoming_sessions
            .iter()
            .map(|record| (record.session_id.0.clone(), record.clone()))
            .collect::<BTreeMap<_, _>>();
        let (added_session_ids, removed_session_ids, updated_session_ids, session_unchanged_count) =
            keyed_restore_delta(current_session_map, incoming_session_map);

        let current_memory_map = current_memories
            .iter()
            .map(|record| (record.id.clone(), record.clone()))
            .collect::<BTreeMap<_, _>>();
        let incoming_memory_map = incoming_memories
            .iter()
            .map(|record| (record.id.clone(), record.clone()))
            .collect::<BTreeMap<_, _>>();
        let (added_memory_ids, removed_memory_ids, updated_memory_ids, memory_unchanged_count) =
            keyed_restore_delta(current_memory_map, incoming_memory_map);

        let current_transcript_map = current_transcripts
            .iter()
            .map(|entry| (entry.entry_id.clone(), entry.clone()))
            .collect::<BTreeMap<_, _>>();
        let incoming_transcript_map = incoming_transcripts
            .iter()
            .map(|entry| (entry.entry_id.clone(), entry.clone()))
            .collect::<BTreeMap<_, _>>();
        let (added_entry_ids, removed_entry_ids, updated_entry_ids, transcript_unchanged_count) =
            keyed_restore_delta(current_transcript_map, incoming_transcript_map);

        Self {
            current_audit: SnapshotAuditReport::from_records_and_entries(
                current_sessions,
                current_memories,
                current_transcripts,
            ),
            incoming_audit: SnapshotAuditReport::from_records_and_entries(
                incoming_sessions,
                incoming_memories,
                incoming_transcripts,
            ),
            session_delta: SessionRestoreDelta {
                added_session_ids: added_session_ids.into_iter().map(SessionId).collect(),
                removed_session_ids: removed_session_ids.into_iter().map(SessionId).collect(),
                updated_session_ids: updated_session_ids.into_iter().map(SessionId).collect(),
                unchanged_count: session_unchanged_count,
            },
            memory_delta: MemoryRestoreDelta {
                added_memory_ids,
                removed_memory_ids,
                updated_memory_ids,
                unchanged_count: memory_unchanged_count,
            },
            transcript_delta: TranscriptRestoreDelta {
                added_entry_ids,
                removed_entry_ids,
                updated_entry_ids,
                unchanged_count: transcript_unchanged_count,
            },
        }
    }

    pub fn change_count(&self) -> usize {
        self.session_delta.change_count()
            + self.memory_delta.change_count()
            + self.transcript_delta.change_count()
    }

    /// Returns a compact automation-friendly summary of restore impact.
    pub fn impact(&self) -> SnapshotRestoreImpact {
        SnapshotRestoreImpact::from_preview(self)
    }

    /// Returns a payload-light readiness summary for restore planning.
    pub fn readiness(&self) -> SnapshotRestoreReadiness {
        SnapshotRestoreReadiness::from_preview(self)
    }

    /// Returns a compact safety summary for restore planning.
    pub fn safety(&self) -> SnapshotRestoreSafety {
        SnapshotRestoreSafety::from_preview(self)
    }

    /// Returns a compact domain-shape summary for restore planning.
    pub fn mutation_profile(&self) -> SnapshotRestoreMutationProfile {
        SnapshotRestoreMutationProfile::from_preview(self)
    }

    /// Returns the restore domains whose delta counts include real changes.
    pub fn changed_domains(&self) -> Vec<SnapshotRestoreDomain> {
        self.domain_impacts()
            .into_iter()
            .filter_map(|impact| (!impact.counts.is_empty()).then_some(impact.domain))
            .collect()
    }

    /// Returns aggregate counts for each restore domain in stable domain order.
    pub fn domain_impacts(&self) -> Vec<SnapshotRestoreDomainImpact> {
        vec![
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: self.session_delta.counts(),
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: self.memory_delta.counts(),
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: self.transcript_delta.counts(),
            },
        ]
    }

    /// Returns the number of restore domains touched by this preview.
    pub fn changed_domain_count(&self) -> usize {
        self.changed_domains().len()
    }

    /// Returns `true` when this preview includes changes for `domain`.
    pub fn touches(&self, domain: SnapshotRestoreDomain) -> bool {
        self.changed_domains().contains(&domain)
    }

    /// Returns the compact delta counts for one restore domain.
    pub fn impact_for(&self, domain: SnapshotRestoreDomain) -> Option<SnapshotRestoreDomainImpact> {
        self.domain_impacts()
            .into_iter()
            .find(|impact| impact.domain == domain)
    }

    /// Returns aggregate added/removed/updated/unchanged counts across all
    /// restore domains.
    pub fn change_totals(&self) -> RestoreDeltaCounts {
        let session = self.session_delta.counts();
        let memory = self.memory_delta.counts();
        let transcript = self.transcript_delta.counts();

        RestoreDeltaCounts {
            added_count: session.added_count + memory.added_count + transcript.added_count,
            removed_count: session.removed_count + memory.removed_count + transcript.removed_count,
            updated_count: session.updated_count + memory.updated_count + transcript.updated_count,
            unchanged_count: session.unchanged_count
                + memory.unchanged_count
                + transcript.unchanged_count,
        }
    }

    pub fn has_integrity_issues(&self) -> bool {
        !self.current_audit.is_clean() || !self.incoming_audit.is_clean()
    }

    pub fn is_noop(&self) -> bool {
        self.change_count() == 0
    }
}

fn keyed_restore_delta<V: PartialEq>(
    current: BTreeMap<String, V>,
    incoming: BTreeMap<String, V>,
) -> (Vec<String>, Vec<String>, Vec<String>, usize) {
    let mut keys = current.keys().cloned().collect::<BTreeSet<_>>();
    keys.extend(incoming.keys().cloned());

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut updated = Vec::new();
    let mut unchanged_count = 0;

    for key in keys {
        match (current.get(&key), incoming.get(&key)) {
            (None, Some(_)) => added.push(key),
            (Some(_), None) => removed.push(key),
            (Some(current_value), Some(incoming_value)) => {
                if current_value == incoming_value {
                    unchanged_count += 1;
                } else {
                    updated.push(key);
                }
            }
            (None, None) => {}
        }
    }

    (added, removed, updated, unchanged_count)
}
