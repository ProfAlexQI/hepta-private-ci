use std::collections::BTreeMap;

use super::*;

fn sample_transcript_entry(sequence: u64, content: &str) -> TranscriptEntry {
    TranscriptEntry {
        entry_id: format!("entry-{}", sequence),
        session_id: SessionId("session-42".into()),
        sequence,
        kind: TranscriptEntryKind::Message,
        role: Some(MessageRole::User),
        content: content.into(),
        created_at_unix_ms: 100 + sequence,
        tool_name: None,
        correlation_id: Some("corr-1".into()),
        summary_of_range: None,
    }
}

mod context_plane_activation;
mod context_plane_operator_packet;
mod context_plane_status;
mod provider_plane;
mod query;
mod recall_core;
mod recall_inspection;
mod recall_quality;
mod recall_summary;
mod restore_impact;
mod restore_planning;
mod restore_preview;
mod restore_readiness;
mod session;
mod snapshot;
mod snapshot_inspection;
mod store;
mod transcript;
