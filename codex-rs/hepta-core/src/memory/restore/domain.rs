use serde::Deserialize;
use serde::Serialize;

use super::delta::RestoreDeltaCounts;

/// Named snapshot restore domain used by compact impact summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotRestoreDomain {
    Sessions,
    Memories,
    Transcripts,
}

/// Compact per-domain restore impact summary.
///
/// Unlike the identifier-bearing restore deltas on [`SnapshotRestorePreview`],
/// this struct keeps only aggregate counts for one restore domain so automation
/// can present stable summaries without loading the full changed-id lists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotRestoreDomainImpact {
    pub domain: SnapshotRestoreDomain,
    pub counts: RestoreDeltaCounts,
}

impl SnapshotRestoreDomainImpact {
    pub fn change_count(&self) -> usize {
        self.counts.change_count()
    }

    pub fn has_changes(&self) -> bool {
        self.counts.has_changes()
    }

    pub fn has_additions(&self) -> bool {
        self.counts.has_additions()
    }

    pub fn has_removals(&self) -> bool {
        self.counts.has_removals()
    }

    pub fn has_updates(&self) -> bool {
        self.counts.has_updates()
    }

    pub fn touches_existing_records(&self) -> bool {
        self.counts.touches_existing_records()
    }

    pub fn is_additive_only(&self) -> bool {
        self.counts.is_additive_only()
    }
}
