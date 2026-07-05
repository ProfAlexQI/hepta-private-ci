use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use crate::runtime_types::AgentId;
use crate::runtime_types::SessionId;

use super::super::MemoryRecord;
use super::super::MemoryScope;
use super::super::SessionRecord;
use super::duplicate_non_blank_values;

/// Aggregate counts that describe a portable session+memory snapshot.
///
/// This stays intentionally storage-agnostic so doctor reports, export/import
/// flows, and lightweight tooling can reason about memory state without binding
/// to a concrete backend implementation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MemorySnapshotStats {
    pub session_count: usize,
    pub active_session_count: usize,
    pub archived_session_count: usize,
    pub total_memory_count: usize,
    pub session_memory_count: usize,
    pub long_term_memory_count: usize,
}

impl MemorySnapshotStats {
    pub fn from_records(sessions: &[SessionRecord], memories: &[MemoryRecord]) -> Self {
        let archived_session_count = sessions
            .iter()
            .filter(|record| record.archived_at_unix_ms.is_some())
            .count();
        let session_memory_count = memories
            .iter()
            .filter(|record| record.scope == MemoryScope::Session)
            .count();
        let long_term_memory_count = memories
            .iter()
            .filter(|record| record.scope == MemoryScope::LongTerm)
            .count();

        Self {
            session_count: sessions.len(),
            active_session_count: sessions.len().saturating_sub(archived_session_count),
            archived_session_count,
            total_memory_count: memories.len(),
            session_memory_count,
            long_term_memory_count,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.session_count == 0 && self.total_memory_count == 0
    }
}

/// Compact per-agent session inventory derived from portable session records.
///
/// This gives doctor, audit, and export/import tooling a storage-agnostic way
/// to inspect session occupancy by agent without loading the full session
/// payload set into a custom report shape.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SessionAgentInventory {
    pub total_session_count: usize,
    pub blank_agent_id_session_count: usize,
    #[serde(default)]
    pub agents: Vec<SessionAgentDescriptor>,
}

/// Per-agent session inventory row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionAgentDescriptor {
    pub agent_id: AgentId,
    pub session_count: usize,
    pub active_session_count: usize,
    pub archived_session_count: usize,
    pub latest_activity_unix_ms: u64,
}

impl SessionAgentInventory {
    pub fn from_records(sessions: &[SessionRecord]) -> Self {
        #[derive(Default)]
        struct AgentAccumulator {
            session_count: usize,
            active_session_count: usize,
            archived_session_count: usize,
            latest_activity_unix_ms: u64,
        }

        let mut blank_agent_id_session_count = 0;
        let mut by_agent = BTreeMap::<String, AgentAccumulator>::new();

        for record in sessions {
            let agent_id = record.agent_id.0.trim();
            if agent_id.is_empty() {
                blank_agent_id_session_count += 1;
                continue;
            }

            let accumulator = by_agent.entry(agent_id.to_string()).or_default();
            accumulator.session_count += 1;
            if record.archived_at_unix_ms.is_some() {
                accumulator.archived_session_count += 1;
            } else {
                accumulator.active_session_count += 1;
            }
            accumulator.latest_activity_unix_ms = accumulator
                .latest_activity_unix_ms
                .max(record.last_active_unix_ms);
        }

        let agents = by_agent
            .into_iter()
            .map(|(agent_id, accumulator)| SessionAgentDescriptor {
                agent_id: AgentId(agent_id),
                session_count: accumulator.session_count,
                active_session_count: accumulator.active_session_count,
                archived_session_count: accumulator.archived_session_count,
                latest_activity_unix_ms: accumulator.latest_activity_unix_ms,
            })
            .collect();

        Self {
            total_session_count: sessions.len(),
            blank_agent_id_session_count,
            agents,
        }
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn inventoried_session_count(&self) -> usize {
        self.agents.iter().map(|agent| agent.session_count).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.total_session_count == 0
    }
}

/// Compact session metadata for snapshot manifests.
///
/// Manifests are intended for diagnostics, audit trails, and export/import
/// planning, where callers need to inspect the shape of a snapshot without
/// loading the full session history or memory payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotSessionDescriptor {
    pub session_id: SessionId,
    pub title: String,
    pub archived: bool,
}

/// Compact memory metadata for snapshot manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotMemoryDescriptor {
    pub id: String,
    pub scope: MemoryScope,
    pub content_bytes: usize,
}

/// Portable manifest that summarizes a session+memory snapshot without
/// embedding the full memory contents.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MemorySnapshotManifest {
    pub stats: MemorySnapshotStats,
    #[serde(default)]
    pub sessions: Vec<SnapshotSessionDescriptor>,
    #[serde(default)]
    pub memories: Vec<SnapshotMemoryDescriptor>,
}

impl MemorySnapshotManifest {
    pub fn from_records(sessions: &[SessionRecord], memories: &[MemoryRecord]) -> Self {
        let mut session_descriptors = sessions
            .iter()
            .map(|record| SnapshotSessionDescriptor {
                session_id: record.session_id.clone(),
                title: record.title.clone(),
                archived: record.archived_at_unix_ms.is_some(),
            })
            .collect::<Vec<_>>();
        session_descriptors.sort_by(|left, right| left.session_id.0.cmp(&right.session_id.0));

        let mut memory_descriptors = memories
            .iter()
            .map(|record| SnapshotMemoryDescriptor {
                id: record.id.clone(),
                scope: record.scope,
                content_bytes: record.content.len(),
            })
            .collect::<Vec<_>>();
        memory_descriptors.sort_by(|left, right| left.id.cmp(&right.id));

        Self {
            stats: MemorySnapshotStats::from_records(sessions, memories),
            sessions: session_descriptors,
            memories: memory_descriptors,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stats.is_empty()
    }
}

/// Integrity-focused summary of a portable session+memory snapshot.
///
/// This is intentionally additive to the manifest/stats layer: callers can use
/// it for doctor checks, export/import preflight validation, or audit tooling
/// without binding to a concrete store implementation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MemorySnapshotIntegrityReport {
    #[serde(default)]
    pub duplicate_session_ids: Vec<SessionId>,
    #[serde(default)]
    pub duplicate_memory_ids: Vec<String>,
    pub blank_session_id_count: usize,
    pub blank_memory_id_count: usize,
    pub blank_session_title_count: usize,
    pub blank_memory_content_count: usize,
}

impl MemorySnapshotIntegrityReport {
    pub fn from_records(sessions: &[SessionRecord], memories: &[MemoryRecord]) -> Self {
        let duplicate_session_ids = duplicate_non_blank_values(
            sessions
                .iter()
                .map(|record| record.session_id.0.trim().to_string()),
        )
        .into_iter()
        .map(SessionId)
        .collect();
        let duplicate_memory_ids =
            duplicate_non_blank_values(memories.iter().map(|record| record.id.trim().to_string()));

        Self {
            duplicate_session_ids,
            duplicate_memory_ids,
            blank_session_id_count: sessions
                .iter()
                .filter(|record| record.session_id.0.trim().is_empty())
                .count(),
            blank_memory_id_count: memories
                .iter()
                .filter(|record| record.id.trim().is_empty())
                .count(),
            blank_session_title_count: sessions
                .iter()
                .filter(|record| record.title.trim().is_empty())
                .count(),
            blank_memory_content_count: memories
                .iter()
                .filter(|record| record.content.trim().is_empty())
                .count(),
        }
    }

    pub fn issue_count(&self) -> usize {
        self.duplicate_session_ids.len()
            + self.duplicate_memory_ids.len()
            + self.blank_session_id_count
            + self.blank_memory_id_count
            + self.blank_session_title_count
            + self.blank_memory_content_count
    }

    pub fn is_clean(&self) -> bool {
        self.issue_count() == 0
    }
}
