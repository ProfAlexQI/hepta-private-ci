use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentJobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl AgentJobStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            AgentJobStatus::Pending => "pending",
            AgentJobStatus::Running => "running",
            AgentJobStatus::Completed => "completed",
            AgentJobStatus::Failed => "failed",
            AgentJobStatus::Cancelled => "cancelled",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "pending" => Ok(Self::Pending),
            "running" => Ok(Self::Running),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(anyhow::anyhow!("invalid agent job status: {value}")),
        }
    }

    pub fn is_final(self) -> bool {
        matches!(
            self,
            AgentJobStatus::Completed | AgentJobStatus::Failed | AgentJobStatus::Cancelled
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentJobItemStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl AgentJobItemStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            AgentJobItemStatus::Pending => "pending",
            AgentJobItemStatus::Running => "running",
            AgentJobItemStatus::Completed => "completed",
            AgentJobItemStatus::Failed => "failed",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "pending" => Ok(Self::Pending),
            "running" => Ok(Self::Running),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            _ => Err(anyhow::anyhow!("invalid agent job item status: {value}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentJob {
    pub id: String,
    pub name: String,
    pub status: AgentJobStatus,
    pub instruction: String,
    pub auto_export: bool,
    pub max_runtime_seconds: Option<u64>,
    // TODO(jif-oai): Convert to JSON Schema and enforce structured outputs.
    pub output_schema_json: Option<Value>,
    pub input_headers: Vec<String>,
    pub input_csv_path: String,
    pub output_csv_path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentJobItem {
    pub job_id: String,
    pub item_id: String,
    pub row_index: i64,
    pub source_id: Option<String>,
    pub row_json: Value,
    pub status: AgentJobItemStatus,
    pub assigned_thread_id: Option<String>,
    pub attempt_count: i64,
    pub result_json: Option<Value>,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub reported_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentJobProgress {
    pub total_items: usize,
    pub pending_items: usize,
    pub running_items: usize,
    pub completed_items: usize,
    pub failed_items: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentJobWorkGraphShadowEvent {
    pub sequence_id: i64,
    pub job_id: String,
    pub item_id: Option<String>,
    pub event_type: String,
    pub task_id: String,
    pub status: String,
    pub summary: String,
    pub payload_json: Value,
    pub trace_id: Option<String>,
    pub span_id: String,
    pub source_surface_id: String,
    pub created_at: DateTime<Utc>,
    pub live_blocking_enabled: bool,
    pub live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentJobWorkGraphShadowProjection {
    pub job_id: String,
    pub total_events: usize,
    pub distinct_tasks: usize,
    pub latest_sequence_id: Option<i64>,
    pub item_started_events: usize,
    pub item_completed_events: usize,
    pub item_failed_events: usize,
    pub job_terminal_events: usize,
    pub live_blocking_event_count: usize,
    pub live_cutover_event_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentJobWorkGraphShadowProjectionDiff {
    pub job_id: String,
    pub progress: AgentJobProgress,
    pub projection: AgentJobWorkGraphShadowProjection,
    pub completed_item_delta: isize,
    pub failed_item_delta: isize,
    pub projection_matches_items: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentJobWorkGraphPromotionReviewReadback {
    pub job_id: String,
    pub admission_shadow_decision_events: usize,
    pub promotion_readiness_matrix_events: usize,
    pub operator_review_promotion_packet_events: usize,
    pub promotion_review_replay_consistency_events: usize,
    pub promotion_closeout_receipt_events: usize,
    pub promotion_closeout_replay_consistency_events: usize,
    pub promotion_review_audit_chain_receipt_events: usize,
    pub reviewed_flag_precondition_plan_events: usize,
    pub reviewed_flag_precondition_plan_replay_consistency_events: usize,
    pub reviewed_flag_readiness_closeout_receipt_events: usize,
    pub reviewed_flag_readiness_closeout_replay_consistency_events: usize,
    pub reviewed_flag_audit_chain_closeout_receipt_events: usize,
    pub live_blocking_event_count: usize,
    pub live_cutover_event_count: usize,
    pub latest_admission_shadow_decision: Option<Value>,
    pub latest_promotion_readiness_matrix: Option<Value>,
    pub latest_operator_review_promotion_packet: Option<Value>,
    pub latest_promotion_review_replay_consistency: Option<Value>,
    pub latest_promotion_closeout_receipt: Option<Value>,
    pub latest_promotion_closeout_replay_consistency: Option<Value>,
    pub latest_promotion_review_audit_chain_receipt: Option<Value>,
    pub latest_reviewed_flag_precondition_plan: Option<Value>,
    pub latest_reviewed_flag_precondition_plan_replay_consistency: Option<Value>,
    pub latest_reviewed_flag_readiness_closeout_receipt: Option<Value>,
    pub latest_reviewed_flag_readiness_closeout_replay_consistency: Option<Value>,
    pub latest_reviewed_flag_audit_chain_closeout_receipt: Option<Value>,
    pub readback_ready: bool,
    pub replay_consistency_ready: bool,
    pub closeout_receipt_ready: bool,
    pub closeout_replay_consistency_ready: bool,
    pub audit_chain_receipt_ready: bool,
    pub reviewed_flag_precondition_plan_ready: bool,
    pub reviewed_flag_precondition_plan_replay_consistency_ready: bool,
    pub reviewed_flag_readiness_closeout_receipt_ready: bool,
    pub reviewed_flag_readiness_closeout_replay_consistency_ready: bool,
    pub reviewed_flag_audit_chain_closeout_receipt_ready: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentJobWorkGraphAuditChainSegmentSpec {
    pub segment_id: &'static str,
    pub event_type: &'static str,
    pub replay_consistency_field: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentJobWorkGraphAuditChainReadback {
    pub job_id: String,
    pub segments: Vec<AgentJobWorkGraphAuditChainSegmentReadback>,
    pub live_blocking_event_count: usize,
    pub live_cutover_event_count: usize,
    pub chain_readback_ready: bool,
    pub chain_replay_consistent: bool,
    pub no_live_guardrails_ready: bool,
    pub chain_ready: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentJobWorkGraphAuditChainSegmentReadback {
    pub segment_id: String,
    pub event_type: String,
    pub event_count: usize,
    pub latest_payload: Option<Value>,
    pub latest_decision: String,
    pub readback_ready: bool,
    pub replay_consistent: bool,
    pub no_live_guardrail_ready: bool,
    pub ready: bool,
}

#[derive(Debug, Clone)]
pub struct AgentJobCreateParams {
    pub id: String,
    pub name: String,
    pub instruction: String,
    pub auto_export: bool,
    pub max_runtime_seconds: Option<u64>,
    pub output_schema_json: Option<Value>,
    pub input_headers: Vec<String>,
    pub input_csv_path: String,
    pub output_csv_path: String,
}

#[derive(Debug, Clone)]
pub struct AgentJobItemCreateParams {
    pub item_id: String,
    pub row_index: i64,
    pub source_id: Option<String>,
    pub row_json: Value,
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct AgentJobRow {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) status: String,
    pub(crate) instruction: String,
    pub(crate) auto_export: i64,
    pub(crate) max_runtime_seconds: Option<i64>,
    pub(crate) output_schema_json: Option<String>,
    pub(crate) input_headers_json: String,
    pub(crate) input_csv_path: String,
    pub(crate) output_csv_path: String,
    pub(crate) created_at: i64,
    pub(crate) updated_at: i64,
    pub(crate) started_at: Option<i64>,
    pub(crate) completed_at: Option<i64>,
    pub(crate) last_error: Option<String>,
}

impl TryFrom<AgentJobRow> for AgentJob {
    type Error = anyhow::Error;

    fn try_from(value: AgentJobRow) -> Result<Self, Self::Error> {
        let output_schema_json = value
            .output_schema_json
            .as_deref()
            .map(serde_json::from_str)
            .transpose()?;
        let input_headers = serde_json::from_str(value.input_headers_json.as_str())?;
        let max_runtime_seconds = value
            .max_runtime_seconds
            .map(u64::try_from)
            .transpose()
            .map_err(|_| anyhow::anyhow!("invalid max_runtime_seconds value"))?;
        Ok(Self {
            id: value.id,
            name: value.name,
            status: AgentJobStatus::parse(value.status.as_str())?,
            instruction: value.instruction,
            auto_export: value.auto_export != 0,
            max_runtime_seconds,
            output_schema_json,
            input_headers,
            input_csv_path: value.input_csv_path,
            output_csv_path: value.output_csv_path,
            created_at: epoch_seconds_to_datetime(value.created_at)?,
            updated_at: epoch_seconds_to_datetime(value.updated_at)?,
            started_at: value
                .started_at
                .map(epoch_seconds_to_datetime)
                .transpose()?,
            completed_at: value
                .completed_at
                .map(epoch_seconds_to_datetime)
                .transpose()?,
            last_error: value.last_error,
        })
    }
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct AgentJobItemRow {
    pub(crate) job_id: String,
    pub(crate) item_id: String,
    pub(crate) row_index: i64,
    pub(crate) source_id: Option<String>,
    pub(crate) row_json: String,
    pub(crate) status: String,
    pub(crate) assigned_thread_id: Option<String>,
    pub(crate) attempt_count: i64,
    pub(crate) result_json: Option<String>,
    pub(crate) last_error: Option<String>,
    pub(crate) created_at: i64,
    pub(crate) updated_at: i64,
    pub(crate) completed_at: Option<i64>,
    pub(crate) reported_at: Option<i64>,
}

impl TryFrom<AgentJobItemRow> for AgentJobItem {
    type Error = anyhow::Error;

    fn try_from(value: AgentJobItemRow) -> Result<Self, Self::Error> {
        Ok(Self {
            job_id: value.job_id,
            item_id: value.item_id,
            row_index: value.row_index,
            source_id: value.source_id,
            row_json: serde_json::from_str(value.row_json.as_str())?,
            status: AgentJobItemStatus::parse(value.status.as_str())?,
            assigned_thread_id: value.assigned_thread_id,
            attempt_count: value.attempt_count,
            result_json: value
                .result_json
                .as_deref()
                .map(serde_json::from_str)
                .transpose()?,
            last_error: value.last_error,
            created_at: epoch_seconds_to_datetime(value.created_at)?,
            updated_at: epoch_seconds_to_datetime(value.updated_at)?,
            completed_at: value
                .completed_at
                .map(epoch_seconds_to_datetime)
                .transpose()?,
            reported_at: value
                .reported_at
                .map(epoch_seconds_to_datetime)
                .transpose()?,
        })
    }
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct AgentJobWorkGraphShadowEventRow {
    pub(crate) sequence_id: i64,
    pub(crate) job_id: String,
    pub(crate) item_id: Option<String>,
    pub(crate) event_type: String,
    pub(crate) task_id: String,
    pub(crate) status: String,
    pub(crate) summary: String,
    pub(crate) payload_json: String,
    pub(crate) trace_id: Option<String>,
    pub(crate) span_id: String,
    pub(crate) source_surface_id: String,
    pub(crate) created_at: i64,
    pub(crate) live_blocking_enabled: i64,
    pub(crate) live_cutover_enabled: i64,
}

impl TryFrom<AgentJobWorkGraphShadowEventRow> for AgentJobWorkGraphShadowEvent {
    type Error = anyhow::Error;

    fn try_from(value: AgentJobWorkGraphShadowEventRow) -> Result<Self, Self::Error> {
        Ok(Self {
            sequence_id: value.sequence_id,
            job_id: value.job_id,
            item_id: value.item_id,
            event_type: value.event_type,
            task_id: value.task_id,
            status: value.status,
            summary: value.summary,
            payload_json: serde_json::from_str(value.payload_json.as_str())?,
            trace_id: value.trace_id,
            span_id: value.span_id,
            source_surface_id: value.source_surface_id,
            created_at: epoch_seconds_to_datetime(value.created_at)?,
            live_blocking_enabled: value.live_blocking_enabled != 0,
            live_cutover_enabled: value.live_cutover_enabled != 0,
        })
    }
}

fn epoch_seconds_to_datetime(secs: i64) -> Result<DateTime<Utc>> {
    DateTime::<Utc>::from_timestamp(secs, 0)
        .ok_or_else(|| anyhow::anyhow!("invalid unix timestamp: {secs}"))
}
