use serde::Deserialize;
use serde::Serialize;

use crate::runtime_types::SessionId;

use super::super::MemoryQuery;
use super::super::TranscriptQuery;

/// Bounded request for blended transcript + memory recall.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextRecallRequest {
    pub session_id: SessionId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    pub recent_window_limit: usize,
    pub transcript_limit: usize,
    pub memory_limit: usize,
    /// Advisory widening flag for memory sources.
    ///
    /// Recent entries and transcript hits remain anchored to `session_id`
    /// regardless of this value. Portable `MemoryRecord` payloads do not carry
    /// session ownership, so adapters that cannot distinguish session-local
    /// memory may legitimately return the same memory hits whether this is
    /// enabled or not.
    pub allow_cross_session: bool,
}

/// Global budget used when several recall sources compete for a bounded prompt
/// or routing frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextBudget {
    pub max_items: usize,
    pub max_tokens_estimate: usize,
    pub min_source_diversity: usize,
    pub max_per_source: usize,
}

impl Default for ContextBudget {
    fn default() -> Self {
        Self {
            max_items: 24,
            max_tokens_estimate: 4096,
            min_source_diversity: 3,
            max_per_source: 8,
        }
    }
}

impl ContextBudget {
    pub fn from_request(request: &ContextRecallRequest) -> Self {
        let max_items = request
            .recent_window_limit
            .saturating_add(request.transcript_limit)
            .saturating_add(request.memory_limit)
            .max(1);
        Self {
            max_items,
            max_tokens_estimate: max_items.saturating_mul(256),
            min_source_diversity: 3,
            max_per_source: request
                .recent_window_limit
                .max(request.transcript_limit)
                .max(request.memory_limit)
                .max(1),
        }
    }
}

impl ContextRecallRequest {
    /// Returns the trimmed query text when the request carries a non-blank
    /// search hint.
    pub fn normalized_query_text(&self) -> Option<&str> {
        self.query_text
            .as_deref()
            .map(str::trim)
            .filter(|text| !text.is_empty())
    }

    pub fn has_query_text(&self) -> bool {
        self.normalized_query_text().is_some()
    }

    /// Builds the session-scoped transcript query that corresponds to this
    /// recall request.
    pub fn transcript_query(&self) -> TranscriptQuery {
        TranscriptQuery {
            session_id: Some(self.session_id.clone()),
            text: self.normalized_query_text().unwrap_or_default().to_string(),
            limit: self.transcript_limit,
        }
    }

    /// Builds the memory query that corresponds to this recall request.
    pub fn memory_query(&self) -> MemoryQuery {
        MemoryQuery {
            text: self.normalized_query_text().unwrap_or_default().to_string(),
            limit: self.memory_limit,
        }
    }
}
