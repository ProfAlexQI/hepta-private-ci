use serde::Deserialize;
use serde::Serialize;

use super::MemoryRecord;

/// Query contract for memory retrieval.
///
/// Implementations may apply ranking or indexing internally, but they should
/// treat `text` as the caller's retrieval hint and must honor `limit`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub text: String,
    pub limit: usize,
}

/// Compact returned-vs-matched counts derived from a query report.
///
/// This keeps only the aggregate coverage counts that automation and tests
/// often need when they want to reason about clipping without carrying the
/// full memory or transcript hit payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct QueryReportCoverage {
    pub returned_count: usize,
    pub matched_count: usize,
}

impl QueryReportCoverage {
    pub fn omitted_count(&self) -> usize {
        self.matched_count.saturating_sub(self.returned_count)
    }

    pub fn is_complete(&self) -> bool {
        self.returned_count == self.matched_count
    }

    pub fn is_empty(&self) -> bool {
        self.matched_count == 0
    }

    pub fn is_truncated(&self) -> bool {
        self.returned_count < self.matched_count
    }
}

/// Compact omission-focused summary derived from a query report.
///
/// Unlike [`QueryReportCoverage`], this focuses on limit pressure only: whether
/// the result was truncated and how many matched hits were left behind.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct QueryReportLimitPressure {
    pub truncated: bool,
    pub omitted_count: usize,
}

impl QueryReportLimitPressure {
    pub fn from_coverage(coverage: &QueryReportCoverage) -> Self {
        Self {
            truncated: coverage.is_truncated(),
            omitted_count: coverage.omitted_count(),
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.truncated && self.omitted_count == 0
    }

    pub fn is_empty(&self) -> bool {
        self.is_complete()
    }
}

/// Portable top-level report for memory retrieval.
///
/// This gives callers a stable machine-readable envelope for search results
/// without requiring them to infer truncation or total match counts from an
/// implementation-specific backend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryQueryReport {
    pub query: MemoryQuery,
    pub matched_count: usize,
    pub returned_count: usize,
    pub truncated: bool,
    #[serde(default, skip_serializing_if = "is_zero_usize")]
    pub omitted_control_count: usize,
    #[serde(default)]
    pub hits: Vec<MemoryRecord>,
}

impl MemoryQueryReport {
    pub fn from_hits(query: MemoryQuery, matched_count: usize, hits: Vec<MemoryRecord>) -> Self {
        Self::from_hits_with_omitted_control_count(query, matched_count, hits, 0)
    }

    pub fn from_hits_with_omitted_control_count(
        query: MemoryQuery,
        matched_count: usize,
        hits: Vec<MemoryRecord>,
        omitted_control_count: usize,
    ) -> Self {
        let returned_count = hits.len();

        Self {
            query,
            matched_count,
            returned_count,
            truncated: returned_count < matched_count,
            omitted_control_count,
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

fn is_zero_usize(value: &usize) -> bool {
    *value == 0
}
