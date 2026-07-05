mod memory;
mod transcript;

use std::collections::BTreeMap;

pub use memory::MemorySnapshotIntegrityReport;
pub use memory::MemorySnapshotManifest;
pub use memory::MemorySnapshotStats;
pub use memory::SessionAgentDescriptor;
pub use memory::SessionAgentInventory;
pub use memory::SnapshotMemoryDescriptor;
pub use memory::SnapshotSessionDescriptor;
pub use transcript::SnapshotTranscriptDescriptor;
pub use transcript::TranscriptSequenceCollision;
pub use transcript::TranscriptSessionDescriptor;
pub use transcript::TranscriptSessionInventory;
pub use transcript::TranscriptSnapshotIntegrityReport;
pub use transcript::TranscriptSnapshotManifest;
pub use transcript::TranscriptSnapshotStats;

fn duplicate_non_blank_values(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut counts = BTreeMap::<String, usize>::new();

    for value in values {
        if value.is_empty() {
            continue;
        }
        *counts.entry(value).or_default() += 1;
    }

    counts
        .into_iter()
        .filter_map(|(value, count)| (count > 1).then_some(value))
        .collect()
}
