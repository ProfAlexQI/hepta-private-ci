use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

pub const TURN_CONTEXT_MANIFEST_VERSION: u32 = 1;
pub const TURN_CONTEXT_DECISION_SCHEMA_VERSION: u32 = 1;
pub const TURN_CONTEXT_COMPRESSION_CANDIDATE_SCHEMA_VERSION: u32 = 1;
pub const TURN_CONTEXT_COMPRESSION_STAGE_SCHEMA_VERSION: u32 = 2;
pub const TURN_CONTEXT_ADAPTIVE_BUDGET_ALLOCATION_SCHEMA_VERSION: u32 = 1;
pub const TURN_CONTEXT_MEMORY_TAXONOMY_SCHEMA_VERSION: u32 = 1;
pub const TURN_CONTEXT_MEMORY_FORMATION_RECEIPT_SCHEMA_VERSION: u32 = 1;
pub const TURN_CONTEXT_MEMORY_TEMPORAL_FACT_SCHEMA_VERSION: u32 = 1;

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextTier {
    System,
    Developer,
    User,
    Tool,
    Runtime,
    SessionState,
    CrossSessionMemory,
    RetrievedSnippets,
    Summary,
    #[default]
    Unknown,
}

impl TurnContextTier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Developer => "developer",
            Self::User => "user",
            Self::Tool => "tool",
            Self::Runtime => "runtime",
            Self::SessionState => "session_state",
            Self::CrossSessionMemory => "cross_session_memory",
            Self::RetrievedSnippets => "retrieved_snippets",
            Self::Summary => "summary",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn is_zero_u32(value: &u32) -> bool {
    *value == 0
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(super) fn is_false(value: &bool) -> bool {
    !*value
}

pub(in crate::protocol) fn is_stable_manifest_replay_hash(value: &str) -> bool {
    value.len() == 16 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub(super) fn compression_candidate_source_id_is_payload_light(value: &str) -> bool {
    const FORBIDDEN_SUBSTRINGS: &[&str] = &[
        "memory_id",
        "neuron_id",
        "prompt_text",
        "query",
        "replay_key",
        "snippet_text",
        "text_hash",
        "topic_id",
    ];

    !value.is_empty()
        && value.len() <= 96
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && !FORBIDDEN_SUBSTRINGS
            .iter()
            .any(|forbidden| value.contains(forbidden))
}
