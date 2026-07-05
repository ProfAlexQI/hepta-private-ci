use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::runtime_types::SessionId;

use super::super::TranscriptEntry;
use super::super::TranscriptEntryKind;
use super::duplicate_non_blank_values;

/// This stays intentionally storage-agnostic so diagnostics, export/import
/// tooling, and contract tests can reason about transcript state without
/// binding to a concrete backend implementation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranscriptSnapshotStats {
    pub total_entry_count: usize,
    pub session_count: usize,
    pub message_count: usize,
    pub tool_call_count: usize,
    pub tool_result_count: usize,
    pub approval_count: usize,
    pub summary_count: usize,
    pub event_count: usize,
}

impl TranscriptSnapshotStats {
    pub fn from_entries(entries: &[TranscriptEntry]) -> Self {
        let mut stats = Self::default();
        let mut session_ids = BTreeSet::new();

        for entry in entries {
            stats.total_entry_count += 1;

            let session_id = entry.session_id.0.trim();
            if !session_id.is_empty() {
                session_ids.insert(session_id.to_string());
            }

            match entry.kind {
                TranscriptEntryKind::Message => stats.message_count += 1,
                TranscriptEntryKind::ToolCall => stats.tool_call_count += 1,
                TranscriptEntryKind::ToolResult => stats.tool_result_count += 1,
                TranscriptEntryKind::Approval => stats.approval_count += 1,
                TranscriptEntryKind::Summary => stats.summary_count += 1,
                TranscriptEntryKind::Event => stats.event_count += 1,
            }
        }

        stats.session_count = session_ids.len();
        stats
    }

    pub fn is_empty(&self) -> bool {
        self.total_entry_count == 0
    }
}

/// Compact transcript metadata for snapshot manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotTranscriptDescriptor {
    pub entry_id: String,
    pub session_id: SessionId,
    pub sequence: u64,
    pub kind: TranscriptEntryKind,
    pub content_bytes: usize,
}

/// Portable manifest that summarizes a transcript snapshot without embedding
/// the full entry contents.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranscriptSnapshotManifest {
    pub stats: TranscriptSnapshotStats,
    #[serde(default)]
    pub entries: Vec<SnapshotTranscriptDescriptor>,
}

impl TranscriptSnapshotManifest {
    pub fn from_entries(entries: &[TranscriptEntry]) -> Self {
        let mut entry_descriptors = entries
            .iter()
            .map(|entry| SnapshotTranscriptDescriptor {
                entry_id: entry.entry_id.clone(),
                session_id: entry.session_id.clone(),
                sequence: entry.sequence,
                kind: entry.kind,
                content_bytes: entry.content.len(),
            })
            .collect::<Vec<_>>();
        entry_descriptors.sort_by(|left, right| {
            left.session_id
                .0
                .cmp(&right.session_id.0)
                .then(left.sequence.cmp(&right.sequence))
                .then(left.entry_id.cmp(&right.entry_id))
        });

        Self {
            stats: TranscriptSnapshotStats::from_entries(entries),
            entries: entry_descriptors,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stats.is_empty()
    }
}

/// Compact per-session transcript inventory derived from a portable transcript
/// snapshot.
///
/// This gives doctor, export/import, and CLI tooling a payload-light way to
/// inspect transcript occupancy by session without needing to load or diff the
/// full entry list.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranscriptSessionInventory {
    pub total_entry_count: usize,
    pub blank_session_id_entry_count: usize,
    #[serde(default)]
    pub sessions: Vec<TranscriptSessionDescriptor>,
}

/// Per-session transcript inventory row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptSessionDescriptor {
    pub session_id: SessionId,
    pub entry_count: usize,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub message_count: usize,
    pub tool_call_count: usize,
    pub tool_result_count: usize,
    pub approval_count: usize,
    pub summary_count: usize,
    pub event_count: usize,
}

impl TranscriptSessionInventory {
    pub fn from_entries(entries: &[TranscriptEntry]) -> Self {
        #[derive(Default)]
        struct SessionAccumulator {
            entry_count: usize,
            first_sequence: Option<u64>,
            last_sequence: Option<u64>,
            message_count: usize,
            tool_call_count: usize,
            tool_result_count: usize,
            approval_count: usize,
            summary_count: usize,
            event_count: usize,
        }

        let mut blank_session_id_entry_count = 0;
        let mut by_session = BTreeMap::<String, SessionAccumulator>::new();

        for entry in entries {
            let session_id = entry.session_id.0.trim();
            if session_id.is_empty() {
                blank_session_id_entry_count += 1;
                continue;
            }

            let accumulator = by_session.entry(session_id.to_string()).or_default();
            accumulator.entry_count += 1;
            accumulator.first_sequence = Some(
                accumulator
                    .first_sequence
                    .map_or(entry.sequence, |current| current.min(entry.sequence)),
            );
            accumulator.last_sequence = Some(
                accumulator
                    .last_sequence
                    .map_or(entry.sequence, |current| current.max(entry.sequence)),
            );

            match entry.kind {
                TranscriptEntryKind::Message => accumulator.message_count += 1,
                TranscriptEntryKind::ToolCall => accumulator.tool_call_count += 1,
                TranscriptEntryKind::ToolResult => accumulator.tool_result_count += 1,
                TranscriptEntryKind::Approval => accumulator.approval_count += 1,
                TranscriptEntryKind::Summary => accumulator.summary_count += 1,
                TranscriptEntryKind::Event => accumulator.event_count += 1,
            }
        }

        let sessions = by_session
            .into_iter()
            .map(|(session_id, accumulator)| TranscriptSessionDescriptor {
                session_id: SessionId(session_id),
                entry_count: accumulator.entry_count,
                first_sequence: accumulator.first_sequence.unwrap_or_default(),
                last_sequence: accumulator.last_sequence.unwrap_or_default(),
                message_count: accumulator.message_count,
                tool_call_count: accumulator.tool_call_count,
                tool_result_count: accumulator.tool_result_count,
                approval_count: accumulator.approval_count,
                summary_count: accumulator.summary_count,
                event_count: accumulator.event_count,
            })
            .collect();

        Self {
            total_entry_count: entries.len(),
            blank_session_id_entry_count,
            sessions,
        }
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    pub fn inventoried_entry_count(&self) -> usize {
        self.sessions
            .iter()
            .map(|session| session.entry_count)
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.total_entry_count == 0
    }
}

/// Duplicate transcript sequence occupancy inside a single session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptSequenceCollision {
    pub session_id: SessionId,
    pub sequence: u64,
    #[serde(default)]
    pub entry_ids: Vec<String>,
}

/// Integrity-focused summary of a portable transcript snapshot.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranscriptSnapshotIntegrityReport {
    #[serde(default)]
    pub duplicate_entry_ids: Vec<String>,
    #[serde(default)]
    pub duplicate_sequence_collisions: Vec<TranscriptSequenceCollision>,
    pub blank_entry_id_count: usize,
    pub blank_session_id_count: usize,
    pub blank_content_count: usize,
}

impl TranscriptSnapshotIntegrityReport {
    pub fn from_entries(entries: &[TranscriptEntry]) -> Self {
        let duplicate_entry_ids = duplicate_non_blank_values(
            entries
                .iter()
                .map(|entry| entry.entry_id.trim().to_string()),
        );

        let mut sequence_collisions = BTreeMap::<(String, u64), Vec<String>>::new();
        for entry in entries {
            let session_id = entry.session_id.0.trim();
            if session_id.is_empty() {
                continue;
            }

            sequence_collisions
                .entry((session_id.to_string(), entry.sequence))
                .or_default()
                .push(entry.entry_id.trim().to_string());
        }

        let duplicate_sequence_collisions = sequence_collisions
            .into_iter()
            .filter_map(|((session_id, sequence), entry_ids)| {
                (entry_ids.len() > 1).then_some(TranscriptSequenceCollision {
                    session_id: SessionId(session_id),
                    sequence,
                    entry_ids,
                })
            })
            .collect();

        Self {
            duplicate_entry_ids,
            duplicate_sequence_collisions,
            blank_entry_id_count: entries
                .iter()
                .filter(|entry| entry.entry_id.trim().is_empty())
                .count(),
            blank_session_id_count: entries
                .iter()
                .filter(|entry| entry.session_id.0.trim().is_empty())
                .count(),
            blank_content_count: entries
                .iter()
                .filter(|entry| entry.content.trim().is_empty())
                .count(),
        }
    }

    pub fn issue_count(&self) -> usize {
        self.duplicate_entry_ids.len()
            + self.duplicate_sequence_collisions.len()
            + self.blank_entry_id_count
            + self.blank_session_id_count
            + self.blank_content_count
    }

    pub fn is_clean(&self) -> bool {
        self.issue_count() == 0
    }
}
