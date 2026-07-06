use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use codex_protocol::ThreadId;
use serde_json::Value;

use super::epoch_millis_to_datetime;

#[derive(Debug, Clone, PartialEq)]
pub struct InterAgentMailboxEvent {
    pub sequence_id: i64,
    pub thread_id: ThreadId,
    pub event_type: String,
    pub mailbox_seq: Option<i64>,
    pub barrier_id: Option<String>,
    pub task_id: Option<String>,
    pub task_name: Option<String>,
    pub author_path: Option<String>,
    pub recipient_path: Option<String>,
    pub other_recipients_json: Option<Value>,
    pub trigger_turn: Option<bool>,
    pub content_json: Option<Value>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub deadline_at: Option<DateTime<Utc>>,
    pub trace_id: Option<String>,
    pub live_blocking_enabled: bool,
    pub live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterAgentMailboxProjection {
    pub thread_id: ThreadId,
    pub total_events: usize,
    pub queued_events: usize,
    pub delivered_events: usize,
    pub barrier_opened_events: usize,
    pub barrier_satisfied_events: usize,
    pub barrier_timed_out_events: usize,
    pub live_blocking_event_count: usize,
    pub live_cutover_event_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterAgentWaitTaskResultReadback {
    pub thread_id: ThreadId,
    pub barrier_id: String,
    pub task_result_delivery_shadow_events: usize,
    pub parent_reducer_shadow_receipt_events: usize,
    pub task_result_replay_consistency_events: usize,
    pub wait_surface_audit_packet_events: usize,
    pub wait_surface_audit_replay_consistency_events: usize,
    pub wait_canonical_projection_receipt_events: usize,
    pub wait_canonical_projection_replay_consistency_events: usize,
    pub wait_canonical_projection_closeout_receipt_events: usize,
    pub wait_canonical_projection_closeout_replay_consistency_events: usize,
    pub wait_canonical_projection_audit_chain_closeout_receipt_events: usize,
    pub wait_canonical_projection_audit_chain_closeout_replay_consistency_events: usize,
    pub wait_canonical_projection_enablement_operator_review_packet_events: usize,
    pub wait_canonical_projection_enablement_operator_review_replay_consistency_events: usize,
    pub wait_canonical_projection_enablement_no_live_rehearsal_closeout_events: usize,
    pub wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_events:
        usize,
    pub wait_canonical_projection_enablement_audit_chain_closeout_events: usize,
    pub wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_events: usize,
    pub wait_canonical_projection_enablement_activation_precondition_operator_packet_events: usize,
    pub wait_canonical_projection_enablement_activation_precondition_replay_consistency_events:
        usize,
    pub wait_canonical_projection_enablement_activation_no_live_closeout_events: usize,
    pub wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_events:
        usize,
    pub wait_canonical_projection_enablement_activation_audit_chain_closeout_events: usize,
    pub wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_events:
        usize,
    pub wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_events:
        usize,
    pub wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_events:
        usize,
    pub wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_events:
        usize,
    pub wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_events:
        usize,
    pub live_blocking_event_count: usize,
    pub live_cutover_event_count: usize,
    pub latest_task_result_delivery_shadow: Option<Value>,
    pub latest_parent_reducer_shadow_receipt: Option<Value>,
    pub latest_task_result_replay_consistency: Option<Value>,
    pub latest_wait_surface_audit_packet: Option<Value>,
    pub latest_wait_surface_audit_replay_consistency: Option<Value>,
    pub latest_wait_canonical_projection_receipt: Option<Value>,
    pub latest_wait_canonical_projection_replay_consistency: Option<Value>,
    pub latest_wait_canonical_projection_closeout_receipt: Option<Value>,
    pub latest_wait_canonical_projection_closeout_replay_consistency: Option<Value>,
    pub latest_wait_canonical_projection_audit_chain_closeout_receipt: Option<Value>,
    pub latest_wait_canonical_projection_audit_chain_closeout_replay_consistency: Option<Value>,
    pub latest_wait_canonical_projection_enablement_operator_review_packet: Option<Value>,
    pub latest_wait_canonical_projection_enablement_operator_review_replay_consistency:
        Option<Value>,
    pub latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout: Option<Value>,
    pub latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency:
        Option<Value>,
    pub latest_wait_canonical_projection_enablement_audit_chain_closeout: Option<Value>,
    pub latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency:
        Option<Value>,
    pub latest_wait_canonical_projection_enablement_activation_precondition_operator_packet:
        Option<Value>,
    pub latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency:
        Option<Value>,
    pub latest_wait_canonical_projection_enablement_activation_no_live_closeout: Option<Value>,
    pub latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency:
        Option<Value>,
    pub latest_wait_canonical_projection_enablement_activation_audit_chain_closeout: Option<Value>,
    pub latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency:
        Option<Value>,
    pub latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet:
        Option<Value>,
    pub latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency:
        Option<Value>,
    pub latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet:
        Option<Value>,
    pub latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency:
        Option<Value>,
    pub latest_task_result_delivery_decision: String,
    pub latest_parent_reducer_decision: String,
    pub latest_task_result_replay_consistency_decision: String,
    pub latest_wait_surface_audit_decision: String,
    pub latest_wait_surface_audit_replay_consistency_decision: String,
    pub latest_wait_canonical_projection_decision: String,
    pub latest_wait_canonical_projection_replay_consistency_decision: String,
    pub latest_wait_canonical_projection_closeout_decision: String,
    pub latest_wait_canonical_projection_closeout_replay_consistency_decision: String,
    pub latest_wait_canonical_projection_audit_chain_closeout_decision: String,
    pub latest_wait_canonical_projection_audit_chain_closeout_replay_consistency_decision: String,
    pub latest_wait_canonical_projection_enablement_operator_review_decision: String,
    pub latest_wait_canonical_projection_enablement_operator_review_replay_consistency_decision:
        String,
    pub latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_decision: String,
    pub latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision:
        String,
    pub latest_wait_canonical_projection_enablement_audit_chain_closeout_decision: String,
    pub latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision:
        String,
    pub latest_wait_canonical_projection_enablement_activation_precondition_operator_decision:
        String,
    pub latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency_decision:
        String,
    pub latest_wait_canonical_projection_enablement_activation_no_live_closeout_decision: String,
    pub latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision:
        String,
    pub latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_decision:
        String,
    pub latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision:
        String,
    pub latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_decision:
        String,
    pub latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision:
        String,
    pub latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_decision:
        String,
    pub latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision:
        String,
    pub task_result_delivery_readback_ready: bool,
    pub parent_reducer_readback_ready: bool,
    pub replay_consistency_ready: bool,
    pub wait_surface_audit_packet_readback_ready: bool,
    pub wait_surface_audit_replay_consistency_ready: bool,
    pub wait_canonical_projection_receipt_readback_ready: bool,
    pub wait_canonical_projection_replay_consistency_ready: bool,
    pub wait_canonical_projection_closeout_receipt_readback_ready: bool,
    pub wait_canonical_projection_closeout_replay_consistency_ready: bool,
    pub wait_canonical_projection_audit_chain_closeout_receipt_readback_ready: bool,
    pub wait_canonical_projection_audit_chain_closeout_replay_consistency_ready: bool,
    pub wait_canonical_projection_enablement_operator_review_packet_readback_ready: bool,
    pub wait_canonical_projection_enablement_operator_review_replay_consistency_ready: bool,
    pub wait_canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready: bool,
    pub wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_ready:
        bool,
    pub wait_canonical_projection_enablement_audit_chain_closeout_readback_ready: bool,
    pub wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_ready: bool,
    pub wait_canonical_projection_enablement_activation_precondition_operator_packet_readback_ready:
        bool,
    pub wait_canonical_projection_enablement_activation_precondition_replay_consistency_ready: bool,
    pub wait_canonical_projection_enablement_activation_no_live_closeout_readback_ready: bool,
    pub wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_ready:
        bool,
    pub wait_canonical_projection_enablement_activation_audit_chain_closeout_readback_ready: bool,
    pub wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_ready:
        bool,
    pub wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready:
        bool,
    pub wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_ready:
        bool,
    pub wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_readback_ready:
        bool,
    pub wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_ready:
        bool,
    pub task_result_delivery_ready: bool,
    pub parent_reducer_receipt_ready: bool,
    pub replay_consistent: bool,
    pub wait_surface_audit_packet_ready: bool,
    pub wait_surface_audit_replay_consistent: bool,
    pub wait_canonical_projection_receipt_ready: bool,
    pub wait_canonical_projection_replay_consistent: bool,
    pub wait_canonical_projection_closeout_receipt_ready: bool,
    pub wait_canonical_projection_closeout_replay_consistent: bool,
    pub wait_canonical_projection_audit_chain_closeout_receipt_ready: bool,
    pub wait_canonical_projection_audit_chain_closeout_replay_consistent: bool,
    pub wait_canonical_projection_enablement_operator_review_ready: bool,
    pub wait_canonical_projection_enablement_operator_review_replay_consistent: bool,
    pub wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready: bool,
    pub wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent: bool,
    pub wait_canonical_projection_enablement_audit_chain_closeout_ready: bool,
    pub wait_canonical_projection_enablement_audit_chain_closeout_replay_consistent: bool,
    pub wait_canonical_projection_enablement_activation_precondition_ready: bool,
    pub wait_canonical_projection_enablement_activation_precondition_replay_consistent: bool,
    pub wait_canonical_projection_enablement_activation_no_live_closeout_ready: bool,
    pub wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistent: bool,
    pub wait_canonical_projection_enablement_activation_audit_chain_closeout_ready: bool,
    pub wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent:
        bool,
    pub wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready:
        bool,
    pub wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent:
        bool,
    pub wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready:
        bool,
    pub wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent:
        bool,
    pub no_live_guardrails_ready: bool,
    pub readback_ready: bool,
    pub direct_wait_task_result_ready: bool,
    pub direct_wait_surface_audit_ready: bool,
    pub direct_wait_canonical_projection_ready: bool,
    pub direct_wait_canonical_projection_closeout_ready: bool,
    pub direct_wait_canonical_projection_audit_chain_closeout_ready: bool,
    pub direct_wait_canonical_projection_audit_chain_closeout_replay_ready: bool,
    pub direct_wait_canonical_projection_enablement_operator_review_ready: bool,
    pub direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready: bool,
    pub direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_ready: bool,
    pub direct_wait_canonical_projection_enablement_audit_chain_closeout_ready: bool,
    pub direct_wait_canonical_projection_enablement_audit_chain_closeout_replay_ready: bool,
    pub direct_wait_canonical_projection_enablement_activation_precondition_ready: bool,
    pub direct_wait_canonical_projection_enablement_activation_precondition_replay_ready: bool,
    pub direct_wait_canonical_projection_enablement_activation_no_live_closeout_ready: bool,
    pub direct_wait_canonical_projection_enablement_activation_no_live_closeout_replay_ready: bool,
    pub direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_ready: bool,
    pub direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_ready:
        bool,
    pub direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready:
        bool,
    pub direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_ready:
        bool,
    pub direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready:
        bool,
    pub direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_ready:
        bool,
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct InterAgentMailboxEventRow {
    pub(crate) sequence_id: i64,
    pub(crate) thread_id: String,
    pub(crate) event_type: String,
    pub(crate) mailbox_seq: Option<i64>,
    pub(crate) barrier_id: Option<String>,
    pub(crate) task_id: Option<String>,
    pub(crate) task_name: Option<String>,
    pub(crate) author_path: Option<String>,
    pub(crate) recipient_path: Option<String>,
    pub(crate) other_recipients_json: Option<String>,
    pub(crate) trigger_turn: Option<i64>,
    pub(crate) content_json: Option<String>,
    pub(crate) status: String,
    pub(crate) created_at_ms: i64,
    pub(crate) deadline_at_ms: Option<i64>,
    pub(crate) trace_id: Option<String>,
    pub(crate) live_blocking_enabled: i64,
    pub(crate) live_cutover_enabled: i64,
}

impl TryFrom<InterAgentMailboxEventRow> for InterAgentMailboxEvent {
    type Error = anyhow::Error;

    fn try_from(value: InterAgentMailboxEventRow) -> Result<Self, Self::Error> {
        Ok(Self {
            sequence_id: value.sequence_id,
            thread_id: ThreadId::try_from(value.thread_id)?,
            event_type: value.event_type,
            mailbox_seq: value.mailbox_seq,
            barrier_id: value.barrier_id,
            task_id: value.task_id,
            task_name: value.task_name,
            author_path: value.author_path,
            recipient_path: value.recipient_path,
            other_recipients_json: value
                .other_recipients_json
                .as_deref()
                .map(serde_json::from_str)
                .transpose()?,
            trigger_turn: value.trigger_turn.map(|trigger_turn| trigger_turn != 0),
            content_json: value
                .content_json
                .as_deref()
                .map(serde_json::from_str)
                .transpose()?,
            status: value.status,
            created_at: epoch_millis_to_datetime(value.created_at_ms)?,
            deadline_at: value
                .deadline_at_ms
                .map(epoch_millis_to_datetime)
                .transpose()?,
            trace_id: value.trace_id,
            live_blocking_enabled: value.live_blocking_enabled != 0,
            live_cutover_enabled: value.live_cutover_enabled != 0,
        })
    }
}
