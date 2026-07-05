use serde::Deserialize;
use serde::Serialize;

use crate::runtime_types::SessionId;

/// Machine-readable restore delta counts used by preview automation and CLI
/// summaries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct RestoreDeltaCounts {
    pub added_count: usize,
    pub removed_count: usize,
    pub updated_count: usize,
    pub unchanged_count: usize,
}

impl RestoreDeltaCounts {
    pub fn change_count(&self) -> usize {
        self.added_count + self.removed_count + self.updated_count
    }

    pub fn has_additions(&self) -> bool {
        self.added_count > 0
    }

    pub fn has_removals(&self) -> bool {
        self.removed_count > 0
    }

    pub fn has_updates(&self) -> bool {
        self.updated_count > 0
    }

    pub fn has_changes(&self) -> bool {
        self.change_count() > 0
    }

    /// Returns `true` when this delta modifies or removes existing records.
    pub fn touches_existing_records(&self) -> bool {
        self.has_removals() || self.has_updates()
    }

    /// Returns `true` when this delta adds new records without updating or
    /// removing any existing records.
    pub fn is_additive_only(&self) -> bool {
        self.has_additions() && !self.touches_existing_records()
    }

    pub fn is_empty(&self) -> bool {
        self.change_count() == 0
    }
}

/// Restore-preview change summary for session records keyed by `session_id`.
///
/// When either side contains duplicate session ids, this remains a best-effort
/// diff and callers should consult the paired integrity reports before using
/// the identifier lists as an exhaustive inventory.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SessionRestoreDelta {
    #[serde(default)]
    pub added_session_ids: Vec<SessionId>,
    #[serde(default)]
    pub removed_session_ids: Vec<SessionId>,
    #[serde(default)]
    pub updated_session_ids: Vec<SessionId>,
    pub unchanged_count: usize,
}

impl SessionRestoreDelta {
    pub fn change_count(&self) -> usize {
        self.added_session_ids.len()
            + self.removed_session_ids.len()
            + self.updated_session_ids.len()
    }

    pub fn counts(&self) -> RestoreDeltaCounts {
        RestoreDeltaCounts {
            added_count: self.added_session_ids.len(),
            removed_count: self.removed_session_ids.len(),
            updated_count: self.updated_session_ids.len(),
            unchanged_count: self.unchanged_count,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.change_count() == 0
    }
}

/// Restore-preview change summary for memory records keyed by `id`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MemoryRestoreDelta {
    #[serde(default)]
    pub added_memory_ids: Vec<String>,
    #[serde(default)]
    pub removed_memory_ids: Vec<String>,
    #[serde(default)]
    pub updated_memory_ids: Vec<String>,
    pub unchanged_count: usize,
}

impl MemoryRestoreDelta {
    pub fn change_count(&self) -> usize {
        self.added_memory_ids.len() + self.removed_memory_ids.len() + self.updated_memory_ids.len()
    }

    pub fn counts(&self) -> RestoreDeltaCounts {
        RestoreDeltaCounts {
            added_count: self.added_memory_ids.len(),
            removed_count: self.removed_memory_ids.len(),
            updated_count: self.updated_memory_ids.len(),
            unchanged_count: self.unchanged_count,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.change_count() == 0
    }
}

/// Restore-preview change summary for transcript entries keyed by `entry_id`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TranscriptRestoreDelta {
    #[serde(default)]
    pub added_entry_ids: Vec<String>,
    #[serde(default)]
    pub removed_entry_ids: Vec<String>,
    #[serde(default)]
    pub updated_entry_ids: Vec<String>,
    pub unchanged_count: usize,
}

impl TranscriptRestoreDelta {
    pub fn change_count(&self) -> usize {
        self.added_entry_ids.len() + self.removed_entry_ids.len() + self.updated_entry_ids.len()
    }

    pub fn counts(&self) -> RestoreDeltaCounts {
        RestoreDeltaCounts {
            added_count: self.added_entry_ids.len(),
            removed_count: self.removed_entry_ids.len(),
            updated_count: self.updated_entry_ids.len(),
            unchanged_count: self.unchanged_count,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.change_count() == 0
    }
}
