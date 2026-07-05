use serde::Deserialize;
use serde::Serialize;

use crate::model::MessageRole;
use crate::runtime_types::SessionId;

use super::QueryReportCoverage;
use super::QueryReportLimitPressure;

/// Stable transcript entry kind for full-session content storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptEntryKind {
    Message,
    ToolCall,
    ToolResult,
    Approval,
    Summary,
    Event,
}

/// Ordered range inside a single session transcript.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptRange {
    pub start_sequence: u64,
    pub end_sequence: u64,
}

impl TranscriptRange {
    pub fn contains(&self, sequence: u64) -> bool {
        sequence >= self.start_sequence && sequence <= self.end_sequence
    }
}

/// Portable full-fidelity transcript entry used for exact session recall.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptEntry {
    pub entry_id: String,
    pub session_id: SessionId,
    pub sequence: u64,
    pub kind: TranscriptEntryKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<MessageRole>,
    pub content: String,
    pub created_at_unix_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_of_range: Option<TranscriptRange>,
}

impl TranscriptEntry {
    pub fn matches_query(&self, query: &TranscriptQuery) -> bool {
        if let Some(session_id) = &query.session_id
            && &self.session_id != session_id
        {
            return false;
        }

        self.content.contains(&query.text)
    }
}

/// Portable recalled transcript span.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptSpan {
    pub session_id: SessionId,
    pub range: TranscriptRange,
    pub entry_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excerpt: Option<String>,
    #[serde(default)]
    pub entries: Vec<TranscriptEntry>,
}

impl TranscriptSpan {
    pub fn from_entry(entry: TranscriptEntry) -> Self {
        Self {
            session_id: entry.session_id.clone(),
            range: TranscriptRange {
                start_sequence: entry.sequence,
                end_sequence: entry.sequence,
            },
            entry_count: 1,
            excerpt: Some(entry.content.clone()),
            entries: vec![entry],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entry_count == 0
    }
}

/// Query contract for transcript retrieval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<SessionId>,
    pub text: String,
    pub limit: usize,
}

/// Machine-readable transcript retrieval report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptQueryReport {
    pub query: TranscriptQuery,
    pub matched_count: usize,
    pub returned_count: usize,
    pub truncated: bool,
    #[serde(default)]
    pub hits: Vec<TranscriptSpan>,
}

impl TranscriptQueryReport {
    pub fn from_hits(
        query: TranscriptQuery,
        matched_count: usize,
        hits: Vec<TranscriptSpan>,
    ) -> Self {
        let returned_count = hits.len();

        Self {
            query,
            matched_count,
            returned_count,
            truncated: returned_count < matched_count,
            hits,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.matched_count == 0
    }

    pub fn has_hits(&self) -> bool {
        !self.is_empty()
    }

    pub fn omitted_count(&self) -> usize {
        self.coverage().omitted_count()
    }

    pub fn is_complete(&self) -> bool {
        self.coverage().is_complete()
    }

    pub fn coverage(&self) -> QueryReportCoverage {
        QueryReportCoverage {
            returned_count: self.returned_count,
            matched_count: self.matched_count,
        }
    }

    pub fn limit_pressure(&self) -> QueryReportLimitPressure {
        QueryReportLimitPressure::from_coverage(&self.coverage())
    }
}
