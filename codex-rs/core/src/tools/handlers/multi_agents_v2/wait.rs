use super::*;
use crate::agent::status::is_final;
use crate::session::session::Session;
use crate::session::turn_context::TurnContext;
use crate::tools::handlers::multi_agents_spec::WaitAgentTimeoutOptions;
use crate::tools::handlers::multi_agents_spec::create_wait_agent_tool_v2;
use crate::tools::handlers::work_graph_admission::WorkGraphAdmissionShadowCheck;
use crate::tools::handlers::work_graph_admission::WorkGraphAgentCardManifestObservation;
use crate::tools::handlers::work_graph_admission::WorkGraphRoleManifestShadowDecision;
use crate::tools::handlers::work_graph_admission::build_agent_card_manifest_shadow_decision;
use crate::tools::handlers::work_graph_admission::configured_agent_role_manifest_source;
use crate::tools::handlers::work_graph_admission::subagent_lifecycle_agent_card_manifest;
use crate::tools::handlers::work_graph_surface_audit::DirectWaitWorkGraphSurfaceAuditPacketInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphAuditChainSegment;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphAuditChainSummary;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionAuditChainCloseoutReceipt;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionAuditChainCloseoutReceiptInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyDecision;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionCloseoutReceipt;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionCloseoutReceiptInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionCloseoutReplayConsistencyDecision;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionCloseoutReplayConsistencyInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacket;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacketInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyDecision;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReceipt;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyDecision;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReceipt;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyDecision;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacket;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacketInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyDecision;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyDecision;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementAuditChainCloseoutInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReceipt;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyDecision;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReceipt;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyDecision;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementOperatorReviewPacket;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementOperatorReviewPacketInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyDecision;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionReplayConsistencyDecision;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionReplayConsistencyInput;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphCanonicalProjectionShadowReceipt;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphOperatorMatrixRow;
use crate::tools::handlers::work_graph_surface_audit::WorkGraphSurfaceAuditPacketSummary;
use crate::tools::handlers::work_graph_surface_audit::build_direct_wait_work_graph_surface_audit_packet;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_audit_chain_closeout_receipt;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_closeout_receipt;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_closeout_replay_consistency_decision;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_activation_precondition_operator_packet;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_operator_review_packet;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_replay_consistency_decision;
use crate::tools::handlers::work_graph_surface_audit::build_work_graph_canonical_projection_shadow_receipt;
use crate::tools::handlers::work_graph_surface_audit::summarize_work_graph_surface_audit_packet;
use crate::turn_timing::now_unix_timestamp_ms;
use codex_protocol::protocol::CollabAgentRef;
use codex_tools::ToolSpec;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use tokio::time::sleep_until;
use tokio::time::timeout_at;

#[derive(Default)]
pub(crate) struct Handler {
    options: WaitAgentTimeoutOptions,
}

impl Handler {
    pub(crate) fn new(options: WaitAgentTimeoutOptions) -> Self {
        Self { options }
    }
}

#[async_trait::async_trait]
impl ToolExecutor<ToolInvocation> for Handler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain("wait_agent")
    }

    fn spec(&self) -> Option<ToolSpec> {
        Some(create_wait_agent_tool_v2(self.options))
    }

    async fn handle(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn crate::tools::context::ToolOutput>, FunctionCallError> {
        let ToolInvocation {
            session,
            turn,
            payload,
            call_id,
            ..
        } = invocation;
        let arguments = function_arguments(payload)?;
        let args: WaitArgs = parse_arguments(&arguments)?;
        let task_id = args.task_id;
        let task_name = args.task_name;
        let barrier_id = args
            .barrier_id
            .unwrap_or_else(|| format!("wait-agent:{}:{call_id}", session.conversation_id));
        let result_required = args.result_required.unwrap_or(false);
        let min_timeout_ms = turn.config.multi_agent_v2.min_wait_timeout_ms;
        let max_timeout_ms = turn.config.multi_agent_v2.max_wait_timeout_ms;
        let default_timeout_ms = turn.config.multi_agent_v2.default_wait_timeout_ms;
        let timeout_ms = match args.timeout_ms {
            Some(ms) if ms < min_timeout_ms => {
                return Err(FunctionCallError::RespondToModel(format!(
                    "timeout_ms must be at least {min_timeout_ms}"
                )));
            }
            Some(ms) if ms > max_timeout_ms => {
                return Err(FunctionCallError::RespondToModel(format!(
                    "timeout_ms must be at most {max_timeout_ms}"
                )));
            }
            Some(ms) => ms,
            None => default_timeout_ms,
        };
        let use_task_result_evidence =
            result_required && task_id.is_some() && session.state_db().is_some();
        let mut wait_target = if result_required && !use_task_result_evidence {
            match task_name.as_deref() {
                Some(task_name) => {
                    Some(resolve_wait_task_target(&session, &turn, task_name).await?)
                }
                None => None,
            }
        } else {
            None
        };
        let deadline_at_ms = now_unix_timestamp_ms().saturating_add(timeout_ms);
        let opened_event_recorded = record_wait_barrier_event(
            session.state_db(),
            codex_state::InterAgentMailboxBarrierEventParams {
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                event_type: "wait_barrier_opened",
                status: "opened",
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                result_required,
                deadline_at_ms: Some(deadline_at_ms),
                trace_id: turn.trace_id.as_deref(),
            },
        )
        .await;

        let mut mailbox_seq_rx = session.subscribe_mailbox_seq();
        let receiver_thread_ids = wait_target
            .as_ref()
            .map(|target| vec![target.thread_id])
            .unwrap_or_default();
        let receiver_agents = wait_target
            .as_ref()
            .map(|target| vec![target.agent_ref.clone()])
            .unwrap_or_default();

        session
            .send_event(
                &turn,
                CollabWaitingBeginEvent {
                    started_at_ms: now_unix_timestamp_ms(),
                    sender_thread_id: session.conversation_id,
                    receiver_thread_ids: receiver_thread_ids.clone(),
                    receiver_agents: receiver_agents.clone(),
                    call_id: call_id.clone(),
                }
                .into(),
            )
            .await;

        let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);
        let (timed_out, task_status, task_result, wait_condition) = if use_task_result_evidence {
            let task_result =
                wait_for_task_result_evidence(session.clone(), task_id.as_deref(), deadline).await;
            (
                task_result.is_none(),
                None,
                task_result,
                "task_result_evidence",
            )
        } else if let Some(target) = wait_target.as_mut() {
            let task_status = wait_for_task_terminal_status(
                session.clone(),
                target.thread_id,
                &mut target.status_rx,
                deadline,
            )
            .await
            .map(redact_agent_status_content);
            (
                task_status.is_none(),
                task_status,
                None,
                "task_terminal_status",
            )
        } else if session.has_pending_mailbox_items().await {
            (false, None, None, "mailbox_change")
        } else {
            (
                !wait_for_mailbox_change(&mut mailbox_seq_rx, deadline).await,
                None,
                None,
                "mailbox_change",
            )
        };
        let task_result_delivery_shadow = build_task_result_delivery_shadow_decision(
            result_required,
            task_id.as_deref(),
            task_result.as_ref(),
            wait_condition,
            timed_out,
        );
        let parent_reducer_shadow_receipt = build_parent_reducer_shadow_receipt(
            result_required,
            task_id.as_deref(),
            task_result_delivery_shadow.as_ref(),
        );
        let task_result_delivery_shadow_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type: "wait_task_result_delivery_shadow",
                status: task_result_delivery_shadow
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: task_result_delivery_shadow
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let parent_reducer_shadow_receipt_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type: "wait_parent_reducer_shadow_receipt",
                status: parent_reducer_shadow_receipt
                    .as_ref()
                    .map(|receipt| receipt.decision),
                payload_json: parent_reducer_shadow_receipt
                    .as_ref()
                    .and_then(|receipt| serde_json::to_value(receipt).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_initial_readback = load_wait_task_result_readback(
            session.state_db(),
            session.conversation_id,
            &barrier_id,
        )
        .await;
        let work_graph_wait_task_result_replay_consistency_decision =
            build_wait_task_result_replay_consistency_decision(
                result_required,
                session.conversation_id,
                &barrier_id,
                task_result_delivery_shadow.as_ref(),
                parent_reducer_shadow_receipt.as_ref(),
                wait_task_result_initial_readback.as_ref(),
            );
        let task_result_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type: "wait_task_result_replay_consistency",
                status: work_graph_wait_task_result_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_wait_task_result_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let terminal_event_recorded = record_wait_barrier_event(
            session.state_db(),
            codex_state::InterAgentMailboxBarrierEventParams {
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                event_type: if timed_out {
                    "wait_barrier_timed_out"
                } else {
                    "wait_barrier_satisfied"
                },
                status: if timed_out { "timed_out" } else { "satisfied" },
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                result_required,
                deadline_at_ms: Some(deadline_at_ms),
                trace_id: turn.trace_id.as_deref(),
            },
        )
        .await;
        let wait_task_result_final_readback = load_wait_task_result_readback(
            session.state_db(),
            session.conversation_id,
            &barrier_id,
        )
        .await;
        let work_graph_wait_task_result_readback =
            summarize_wait_task_result_readback(WaitTaskResultReadbackSummaryInput {
                readback: wait_task_result_final_readback.as_ref(),
                task_result_delivery_shadow_event_recorded,
                parent_reducer_shadow_receipt_event_recorded,
                task_result_replay_consistency_event_recorded,
                wait_surface_audit_packet_event_recorded: false,
                wait_surface_audit_replay_consistency_event_recorded: false,
                wait_canonical_projection_receipt_event_recorded: false,
                wait_canonical_projection_replay_consistency_event_recorded: false,
                wait_canonical_projection_closeout_receipt_event_recorded: false,
                wait_canonical_projection_closeout_replay_consistency_event_recorded: false,
                wait_canonical_projection_audit_chain_closeout_receipt_event_recorded: false,
                wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_operator_review_packet_event_recorded: false,
                wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded:
                    false,
                wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_audit_chain_closeout_event_recorded: false,
                wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded:
                    false,
            });
        let work_graph_lifecycle_shadow_decision =
            build_wait_lifecycle_role_manifest_shadow_decision(
                turn.as_ref(),
                wait_target
                    .as_ref()
                    .and_then(|target| target.agent_ref.agent_role.as_ref()),
            );
        let work_graph_wait_operator_matrix_row = build_wait_operator_matrix_row(
            result_required,
            work_graph_wait_task_result_readback.as_ref(),
        );
        let work_graph_wait_surface_audit_packet = build_wait_surface_audit_packet(
            result_required,
            session.conversation_id,
            &barrier_id,
            wait_task_result_final_readback.as_ref(),
            work_graph_wait_operator_matrix_row.as_ref(),
        );
        let wait_surface_audit_packet_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type: "wait_surface_audit_packet",
                status: work_graph_wait_surface_audit_packet
                    .as_ref()
                    .map(|packet| packet.decision),
                payload_json: work_graph_wait_surface_audit_packet
                    .as_ref()
                    .and_then(|packet| serde_json::to_value(packet).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_surface_audit_packet_readback = load_wait_task_result_readback(
            session.state_db(),
            session.conversation_id,
            &barrier_id,
        )
        .await;
        let work_graph_wait_surface_audit_replay_consistency_decision =
            build_wait_surface_audit_replay_consistency_decision(
                result_required,
                session.conversation_id,
                &barrier_id,
                work_graph_wait_surface_audit_packet.as_ref(),
                wait_surface_audit_packet_readback.as_ref(),
            );
        let wait_surface_audit_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type: "wait_surface_audit_replay_consistency",
                status: work_graph_wait_surface_audit_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_wait_surface_audit_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_surface_audit_readback = load_wait_task_result_readback(
            session.state_db(),
            session.conversation_id,
            &barrier_id,
        )
        .await;
        let work_graph_wait_task_result_readback =
            summarize_wait_task_result_readback(WaitTaskResultReadbackSummaryInput {
                readback: wait_task_result_surface_audit_readback.as_ref(),
                task_result_delivery_shadow_event_recorded,
                parent_reducer_shadow_receipt_event_recorded,
                task_result_replay_consistency_event_recorded,
                wait_surface_audit_packet_event_recorded,
                wait_surface_audit_replay_consistency_event_recorded,
                wait_canonical_projection_receipt_event_recorded: false,
                wait_canonical_projection_replay_consistency_event_recorded: false,
                wait_canonical_projection_closeout_receipt_event_recorded: false,
                wait_canonical_projection_closeout_replay_consistency_event_recorded: false,
                wait_canonical_projection_audit_chain_closeout_receipt_event_recorded: false,
                wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_operator_review_packet_event_recorded: false,
                wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded:
                    false,
                wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_audit_chain_closeout_event_recorded: false,
                wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded:
                    false,
                wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded:
                    false,
            });
        let work_graph_wait_operator_matrix_row = build_wait_operator_matrix_row(
            result_required,
            work_graph_wait_task_result_readback.as_ref(),
        );
        let work_graph_global_surface_audit_packet = if result_required {
            let packet = build_direct_wait_work_graph_surface_audit_packet(
                DirectWaitWorkGraphSurfaceAuditPacketInput {
                    thread_id: session.conversation_id.to_string(),
                    barrier_id: &barrier_id,
                    wait_task_result_readback: wait_task_result_surface_audit_readback.as_ref(),
                    wait_operator_matrix_row: work_graph_wait_operator_matrix_row.as_ref(),
                },
            );
            Some(summarize_work_graph_surface_audit_packet(&packet))
        } else {
            None
        };
        let work_graph_canonical_projection_receipt = work_graph_global_surface_audit_packet
            .as_ref()
            .map(build_work_graph_canonical_projection_shadow_receipt);
        let work_graph_canonical_projection_receipt_payload =
            work_graph_canonical_projection_receipt
                .as_ref()
                .and_then(|receipt| serde_json::to_value(receipt).ok());
        let wait_canonical_projection_receipt_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type: "wait_work_graph_canonical_projection_receipt",
                status: work_graph_canonical_projection_receipt
                    .as_ref()
                    .map(|receipt| receipt.decision),
                payload_json: work_graph_canonical_projection_receipt_payload.clone(),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_readback = load_wait_task_result_readback(
            session.state_db(),
            session.conversation_id,
            &barrier_id,
        )
        .await;
        let work_graph_canonical_projection_replay_consistency_decision =
            build_wait_canonical_projection_replay_consistency_decision(
                result_required,
                work_graph_canonical_projection_receipt.as_ref(),
                work_graph_canonical_projection_receipt_payload.as_ref(),
                wait_task_result_canonical_projection_readback.as_ref(),
            );
        let wait_canonical_projection_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type: "wait_work_graph_canonical_projection_replay_consistency",
                status: work_graph_canonical_projection_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_canonical_projection_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_replay_readback = load_wait_task_result_readback(
            session.state_db(),
            session.conversation_id,
            &barrier_id,
        )
        .await;
        let work_graph_canonical_projection_closeout_receipt =
            build_wait_canonical_projection_closeout_receipt(
                result_required,
                work_graph_canonical_projection_receipt.as_ref(),
                work_graph_canonical_projection_replay_consistency_decision.as_ref(),
                wait_task_result_canonical_projection_replay_readback.as_ref(),
            );
        let work_graph_canonical_projection_closeout_receipt_payload =
            work_graph_canonical_projection_closeout_receipt
                .as_ref()
                .and_then(|receipt| serde_json::to_value(receipt).ok());
        let wait_canonical_projection_closeout_receipt_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type: "wait_work_graph_canonical_projection_closeout_receipt",
                status: work_graph_canonical_projection_closeout_receipt
                    .as_ref()
                    .map(|receipt| receipt.decision),
                payload_json: work_graph_canonical_projection_closeout_receipt_payload.clone(),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_closeout_readback =
            load_wait_task_result_readback(
                session.state_db(),
                session.conversation_id,
                &barrier_id,
            )
            .await;
        let work_graph_canonical_projection_closeout_replay_consistency_decision =
            build_wait_canonical_projection_closeout_replay_consistency_decision(
                result_required,
                work_graph_canonical_projection_closeout_receipt.as_ref(),
                work_graph_canonical_projection_closeout_receipt_payload.as_ref(),
                wait_task_result_canonical_projection_closeout_readback.as_ref(),
            );
        let wait_canonical_projection_closeout_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type: "wait_work_graph_canonical_projection_closeout_replay_consistency",
                status: work_graph_canonical_projection_closeout_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_canonical_projection_closeout_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_closeout_replay_readback =
            load_wait_task_result_readback(
                session.state_db(),
                session.conversation_id,
                &barrier_id,
            )
            .await;
        let work_graph_canonical_projection_audit_chain_closeout_receipt =
            build_wait_canonical_projection_audit_chain_closeout_receipt(
                result_required,
                work_graph_canonical_projection_receipt.as_ref(),
                work_graph_canonical_projection_replay_consistency_decision.as_ref(),
                work_graph_canonical_projection_closeout_receipt.as_ref(),
                work_graph_canonical_projection_closeout_replay_consistency_decision.as_ref(),
                wait_task_result_canonical_projection_closeout_replay_readback.as_ref(),
            );
        let work_graph_canonical_projection_audit_chain_closeout_receipt_payload =
            work_graph_canonical_projection_audit_chain_closeout_receipt
                .as_ref()
                .and_then(|receipt| serde_json::to_value(receipt).ok());
        let wait_canonical_projection_audit_chain_closeout_receipt_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type: "wait_work_graph_canonical_projection_audit_chain_closeout_receipt",
                status: work_graph_canonical_projection_audit_chain_closeout_receipt
                    .as_ref()
                    .map(|receipt| receipt.decision),
                payload_json: work_graph_canonical_projection_audit_chain_closeout_receipt_payload
                    .clone(),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_audit_chain_closeout_readback =
            load_wait_task_result_readback(
                session.state_db(),
                session.conversation_id,
                &barrier_id,
            )
            .await;
        let work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision =
            build_wait_canonical_projection_audit_chain_closeout_replay_consistency_decision(
                result_required,
                work_graph_canonical_projection_audit_chain_closeout_receipt.as_ref(),
                work_graph_canonical_projection_audit_chain_closeout_receipt_payload.as_ref(),
                wait_task_result_canonical_projection_audit_chain_closeout_readback.as_ref(),
            );
        let wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
                status: work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_audit_chain_closeout_replay_readback =
            load_wait_task_result_readback(
                session.state_db(),
                session.conversation_id,
                &barrier_id,
            )
            .await;
        let work_graph_canonical_projection_enablement_operator_review_packet =
            build_wait_canonical_projection_enablement_operator_review_packet(
                WaitCanonicalProjectionEnablementOperatorReviewPacketInput {
                result_required,
                    projection_receipt: work_graph_canonical_projection_receipt.as_ref(),
                    projection_replay_consistency_decision:
                        work_graph_canonical_projection_replay_consistency_decision.as_ref(),
                    closeout_receipt: work_graph_canonical_projection_closeout_receipt.as_ref(),
                    closeout_replay_consistency_decision:
                        work_graph_canonical_projection_closeout_replay_consistency_decision
                            .as_ref(),
                    audit_chain_closeout_receipt:
                        work_graph_canonical_projection_audit_chain_closeout_receipt.as_ref(),
                    audit_chain_closeout_replay_consistency_decision:
                        work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_audit_chain_closeout_replay_readback
                            .as_ref(),
                },
            );
        let wait_canonical_projection_enablement_operator_review_packet_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type: "wait_work_graph_canonical_projection_enablement_operator_review_packet",
                status: work_graph_canonical_projection_enablement_operator_review_packet
                    .as_ref()
                    .map(|packet| packet.decision),
                payload_json: work_graph_canonical_projection_enablement_operator_review_packet
                    .as_ref()
                    .and_then(|packet| serde_json::to_value(packet).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_review_readback =
            load_wait_task_result_readback(
                session.state_db(),
                session.conversation_id,
                &barrier_id,
            )
            .await;
        let work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision =
            build_wait_canonical_projection_enablement_operator_review_replay_consistency_decision(
                WaitCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput {
                    result_required,
                    enablement_operator_review_packet:
                        work_graph_canonical_projection_enablement_operator_review_packet.as_ref(),
                    readback: wait_task_result_canonical_projection_enablement_review_readback
                        .as_ref(),
                },
            );
        let wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
                status: work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_review_replay_readback =
            load_wait_task_result_readback(
                session.state_db(),
                session.conversation_id,
                &barrier_id,
            )
            .await;
        let work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt =
            build_wait_canonical_projection_enablement_no_live_rehearsal_closeout_receipt(
                WaitCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput {
                    result_required,
                    enablement_operator_review_packet:
                        work_graph_canonical_projection_enablement_operator_review_packet.as_ref(),
                    enablement_operator_review_replay_consistency_decision:
                        work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_review_replay_readback
                            .as_ref(),
                },
            );
        let work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt_payload =
            work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt
                .as_ref()
                .and_then(|receipt| serde_json::to_value(receipt).ok());
        let wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
                status: work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt
                    .as_ref()
                    .map(|receipt| receipt.decision),
                payload_json:
                    work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt_payload
                        .clone(),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_no_live_closeout_readback =
            load_wait_task_result_readback(
                session.state_db(),
                session.conversation_id,
                &barrier_id,
            )
            .await;
        let work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision =
            build_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision(
                WaitCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput {
                    result_required,
                    no_live_rehearsal_closeout_receipt:
                        work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt
                            .as_ref(),
                    no_live_rehearsal_closeout_receipt_payload:
                        work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt_payload
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_no_live_closeout_readback
                            .as_ref(),
                },
            );
        let wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
                status: work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_no_live_closeout_replay_readback =
            load_wait_task_result_readback(
                session.state_db(),
                session.conversation_id,
                &barrier_id,
            )
            .await;
        let work_graph_canonical_projection_enablement_audit_chain_closeout_receipt =
            build_wait_canonical_projection_enablement_audit_chain_closeout_receipt(
                WaitCanonicalProjectionEnablementAuditChainCloseoutInput {
                    result_required,
                    enablement_operator_review_packet:
                        work_graph_canonical_projection_enablement_operator_review_packet.as_ref(),
                    enablement_operator_review_replay_consistency_decision:
                        work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision
                            .as_ref(),
                    no_live_rehearsal_closeout_receipt:
                        work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt
                            .as_ref(),
                    no_live_rehearsal_closeout_replay_consistency_decision:
                        work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_no_live_closeout_replay_readback
                            .as_ref(),
                },
            );
        let work_graph_canonical_projection_enablement_audit_chain_closeout_receipt_payload =
            work_graph_canonical_projection_enablement_audit_chain_closeout_receipt
                .as_ref()
                .and_then(|receipt| serde_json::to_value(receipt).ok());
        let wait_canonical_projection_enablement_audit_chain_closeout_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
                status: work_graph_canonical_projection_enablement_audit_chain_closeout_receipt
                    .as_ref()
                    .map(|receipt| receipt.decision),
                payload_json:
                    work_graph_canonical_projection_enablement_audit_chain_closeout_receipt_payload
                        .clone(),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_audit_chain_closeout_readback =
            load_wait_task_result_readback(
                session.state_db(),
                session.conversation_id,
                &barrier_id,
            )
            .await;
        let work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision =
            build_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision(
                WaitCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput {
                    result_required,
                    enablement_audit_chain_closeout_receipt:
                        work_graph_canonical_projection_enablement_audit_chain_closeout_receipt
                            .as_ref(),
                    enablement_audit_chain_closeout_receipt_payload:
                        work_graph_canonical_projection_enablement_audit_chain_closeout_receipt_payload
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_audit_chain_closeout_readback
                            .as_ref(),
                },
            );
        let wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
                status: work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_audit_chain_closeout_replay_readback =
            load_wait_task_result_readback(session.state_db(), session.conversation_id, &barrier_id)
                .await;
        let work_graph_canonical_projection_enablement_activation_precondition_operator_packet =
            build_wait_canonical_projection_enablement_activation_precondition_operator_packet(
                WaitCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput {
                    result_required,
                    enablement_audit_chain_closeout_receipt:
                        work_graph_canonical_projection_enablement_audit_chain_closeout_receipt
                            .as_ref(),
                    enablement_audit_chain_closeout_replay_consistency_decision:
                        work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_audit_chain_closeout_replay_readback
                        .as_ref(),
                },
            );
        let work_graph_canonical_projection_enablement_activation_precondition_operator_packet_payload =
            work_graph_canonical_projection_enablement_activation_precondition_operator_packet
                .as_ref()
                .and_then(|packet| serde_json::to_value(packet).ok());
        let wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
                status: work_graph_canonical_projection_enablement_activation_precondition_operator_packet
                    .as_ref()
                    .map(|packet| packet.decision),
                payload_json:
                    work_graph_canonical_projection_enablement_activation_precondition_operator_packet_payload
                        .clone(),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_activation_precondition_readback =
            load_wait_task_result_readback(
                session.state_db(),
                session.conversation_id,
                &barrier_id,
            )
            .await;
        let work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision =
            build_wait_canonical_projection_enablement_activation_precondition_replay_consistency_decision(
                WaitCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput {
                    result_required,
                    activation_precondition_operator_packet:
                        work_graph_canonical_projection_enablement_activation_precondition_operator_packet
                            .as_ref(),
                    activation_precondition_operator_packet_payload:
                        work_graph_canonical_projection_enablement_activation_precondition_operator_packet_payload
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_activation_precondition_readback
                            .as_ref(),
                },
            );
        let wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
                status: work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_activation_precondition_replay_readback =
            load_wait_task_result_readback(session.state_db(), session.conversation_id, &barrier_id)
                .await;
        let work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt =
            build_wait_canonical_projection_enablement_activation_no_live_closeout_receipt(
                WaitCanonicalProjectionEnablementActivationNoLiveCloseoutInput {
                    result_required,
                    activation_precondition_operator_packet:
                        work_graph_canonical_projection_enablement_activation_precondition_operator_packet
                            .as_ref(),
                    activation_precondition_replay_consistency_decision:
                        work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_activation_precondition_replay_readback
                            .as_ref(),
                },
            );
        let work_graph_canonical_projection_enablement_activation_no_live_closeout_payload =
            work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt
                .as_ref()
                .and_then(|receipt| serde_json::to_value(receipt).ok());
        let wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
                status: work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt
                    .as_ref()
                    .map(|receipt| receipt.decision),
                payload_json:
                    work_graph_canonical_projection_enablement_activation_no_live_closeout_payload
                        .clone(),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_activation_no_live_closeout_readback =
            load_wait_task_result_readback(session.state_db(), session.conversation_id, &barrier_id)
                .await;
        let work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision =
            build_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision(
                WaitCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput {
                    result_required,
                    activation_no_live_closeout_receipt:
                        work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt
                            .as_ref(),
                    activation_no_live_closeout_receipt_payload:
                        work_graph_canonical_projection_enablement_activation_no_live_closeout_payload
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_activation_no_live_closeout_readback
                            .as_ref(),
                },
            );
        let wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
                status: work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_activation_no_live_closeout_replay_readback =
            load_wait_task_result_readback(session.state_db(), session.conversation_id, &barrier_id)
                .await;
        let work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt =
            build_wait_canonical_projection_enablement_activation_audit_chain_closeout_receipt(
                WaitCanonicalProjectionEnablementActivationAuditChainCloseoutInput {
                    result_required,
                    activation_precondition_operator_packet:
                        work_graph_canonical_projection_enablement_activation_precondition_operator_packet
                            .as_ref(),
                    activation_precondition_replay_consistency_decision:
                        work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision
                            .as_ref(),
                    activation_no_live_closeout_receipt:
                        work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt
                            .as_ref(),
                    activation_no_live_closeout_replay_consistency_decision:
                        work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_activation_no_live_closeout_replay_readback
                            .as_ref(),
                },
            );
        let work_graph_canonical_projection_enablement_activation_audit_chain_closeout_payload =
            work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt
                .as_ref()
                .and_then(|receipt| serde_json::to_value(receipt).ok());
        let wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
                status: work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt
                    .as_ref()
                    .map(|receipt| receipt.decision),
                payload_json:
                    work_graph_canonical_projection_enablement_activation_audit_chain_closeout_payload
                        .clone(),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_activation_audit_chain_closeout_readback =
            load_wait_task_result_readback(session.state_db(), session.conversation_id, &barrier_id)
                .await;
        let work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision =
            build_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision(
                WaitCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput {
                    result_required,
                    activation_audit_chain_closeout_receipt:
                        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt
                            .as_ref(),
                    activation_audit_chain_closeout_receipt_payload:
                        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_payload
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_activation_audit_chain_closeout_readback
                            .as_ref(),
                },
            );
        let wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
                status: work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_activation_audit_chain_closeout_replay_readback =
            load_wait_task_result_readback(session.state_db(), session.conversation_id, &barrier_id)
                .await;
        let work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet =
            build_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet(
                WaitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightInput {
                    result_required,
                    activation_audit_chain_closeout_receipt:
                        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt
                            .as_ref(),
                    activation_audit_chain_closeout_replay_consistency_decision:
                        work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_activation_audit_chain_closeout_replay_readback
                            .as_ref(),
                },
            );
        let work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_payload =
            work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet
                .as_ref()
                .and_then(|packet| serde_json::to_value(packet).ok());
        let wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
                status: work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet
                    .as_ref()
                    .map(|packet| packet.decision),
                payload_json: work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_payload
                    .clone(),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_activation_operator_approval_readiness_preflight_readback =
            load_wait_task_result_readback(session.state_db(), session.conversation_id, &barrier_id)
                .await;
        let work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision =
            build_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision(
                WaitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput {
                    result_required,
                    activation_operator_approval_readiness_preflight_packet:
                        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet
                            .as_ref(),
                    activation_operator_approval_readiness_preflight_packet_payload:
                        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_payload
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_activation_operator_approval_readiness_preflight_readback
                            .as_ref(),
                },
            );
        let wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency",
                status: work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_readback =
            load_wait_task_result_readback(session.state_db(), session.conversation_id, &barrier_id)
                .await;
        let work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet =
            build_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet(
                WaitCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutInput {
                    result_required,
                    activation_operator_approval_readiness_preflight_packet:
                        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet
                            .as_ref(),
                    activation_operator_approval_readiness_preflight_replay_consistency_decision:
                        work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_readback
                            .as_ref(),
                },
            );
        let work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_payload =
            work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet
                .as_ref()
                .and_then(|packet| serde_json::to_value(packet).ok());
        let wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet",
                status: work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet
                    .as_ref()
                    .map(|packet| packet.decision),
                payload_json: work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_payload
                    .clone(),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_readback =
            load_wait_task_result_readback(session.state_db(), session.conversation_id, &barrier_id)
                .await;
        let work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision =
            build_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision(
                WaitCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyInput {
                    result_required,
                    activation_approval_review_side_effect_lock_closeout_packet:
                        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet
                            .as_ref(),
                    activation_approval_review_side_effect_lock_closeout_packet_payload:
                        work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_payload
                            .as_ref(),
                    readback:
                        wait_task_result_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_readback
                            .as_ref(),
                },
            );
        let wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded =
            record_wait_work_graph_shadow_event(WaitWorkGraphShadowRecordInput {
                state_db: session.state_db(),
                thread_id: session.conversation_id,
                barrier_id: &barrier_id,
                task_id: task_id.as_deref(),
                task_name: task_name.as_deref(),
                event_type:
                    "wait_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency",
                status: work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision
                    .as_ref()
                    .map(|decision| decision.decision),
                payload_json: work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision
                    .as_ref()
                    .and_then(|decision| serde_json::to_value(decision).ok()),
                trace_id: turn.trace_id.as_deref(),
            })
            .await;
        let wait_task_result_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_readback =
            load_wait_task_result_readback(session.state_db(), session.conversation_id, &barrier_id)
                .await;
        let work_graph_wait_task_result_readback = summarize_wait_task_result_readback(
            WaitTaskResultReadbackSummaryInput {
                readback:
                    wait_task_result_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_readback
                        .as_ref(),
                task_result_delivery_shadow_event_recorded,
                parent_reducer_shadow_receipt_event_recorded,
                task_result_replay_consistency_event_recorded,
                wait_surface_audit_packet_event_recorded,
                wait_surface_audit_replay_consistency_event_recorded,
                wait_canonical_projection_receipt_event_recorded,
                wait_canonical_projection_replay_consistency_event_recorded,
                wait_canonical_projection_closeout_receipt_event_recorded,
                wait_canonical_projection_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_audit_chain_closeout_receipt_event_recorded,
                wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_operator_review_packet_event_recorded,
                wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded,
                wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_audit_chain_closeout_event_recorded,
                wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded,
                wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded,
                wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded,
                wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded,
                wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded,
                wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded,
            },
        );
        let result = WaitAgentToolOutput {
            result: WaitAgentResult::from_timed_out(timed_out),
            barrier_id,
            task_id,
            task_name,
            task_thread_id: wait_target
                .as_ref()
                .map(|target| target.thread_id.to_string()),
            task_status: task_status.clone(),
            task_result,
            result_required,
            wait_condition: wait_condition.to_string(),
            task_result_delivery_shadow,
            parent_reducer_shadow_receipt,
            work_graph_wait_task_result_replay_consistency_decision,
            work_graph_wait_surface_audit_replay_consistency_decision,
            work_graph_canonical_projection_replay_consistency_decision,
            work_graph_wait_task_result_readback,
            work_graph_wait_operator_matrix_row,
            work_graph_wait_surface_audit_packet,
            work_graph_global_surface_audit_packet,
            work_graph_canonical_projection_receipt,
            work_graph_canonical_projection_closeout_receipt,
            work_graph_canonical_projection_closeout_replay_consistency_decision,
            work_graph_canonical_projection_audit_chain_closeout_receipt,
            work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision,
            work_graph_canonical_projection_enablement_operator_review_packet,
            work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision,
            work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt,
            work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision,
            work_graph_canonical_projection_enablement_audit_chain_closeout_receipt,
            work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision,
            work_graph_canonical_projection_enablement_activation_precondition_operator_packet,
            work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision,
            work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt,
            work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision,
            work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt,
            work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision,
            work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet,
            work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision,
            work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet,
            work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision,
            durable_mailbox: DurableMailboxWaitMetadata {
                opened_event_recorded,
                terminal_event_recorded,
                task_result_delivery_shadow_event_recorded,
                parent_reducer_shadow_receipt_event_recorded,
                task_result_replay_consistency_event_recorded,
                wait_surface_audit_packet_event_recorded,
                wait_surface_audit_replay_consistency_event_recorded,
                wait_canonical_projection_receipt_event_recorded,
                wait_canonical_projection_replay_consistency_event_recorded,
                wait_canonical_projection_closeout_receipt_event_recorded,
                wait_canonical_projection_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_audit_chain_closeout_receipt_event_recorded,
                wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_operator_review_packet_event_recorded,
                wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded,
                wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_audit_chain_closeout_event_recorded,
                wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded,
                wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded,
                wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded,
                wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded,
                wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded,
                wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded,
                wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded,
                live_blocking_enabled: false,
                live_cutover_enabled: false,
            },
            work_graph_lifecycle_shadow_decision,
        };

        session
            .send_event(
                &turn,
                CollabWaitingEndEvent {
                    sender_thread_id: session.conversation_id,
                    call_id,
                    completed_at_ms: now_unix_timestamp_ms(),
                    agent_statuses: build_wait_agent_statuses(
                        &status_map_for_wait_target(wait_target.as_ref(), task_status.as_ref()),
                        &receiver_agents,
                    ),
                    statuses: status_map_for_wait_target(
                        wait_target.as_ref(),
                        task_status.as_ref(),
                    ),
                }
                .into(),
            )
            .await;

        Ok(boxed_tool_output(result))
    }
}

impl CoreToolRuntime for Handler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct WaitArgs {
    timeout_ms: Option<i64>,
    task_name: Option<String>,
    task_id: Option<String>,
    barrier_id: Option<String>,
    result_required: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct WaitAgentResult {
    pub(crate) message: String,
    pub(crate) timed_out: bool,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct WaitAgentToolOutput {
    #[serde(flatten)]
    result: WaitAgentResult,
    barrier_id: String,
    task_id: Option<String>,
    task_name: Option<String>,
    task_thread_id: Option<String>,
    task_status: Option<AgentStatus>,
    task_result: Option<Value>,
    result_required: bool,
    wait_condition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_result_delivery_shadow: Option<WorkGraphTaskResultDeliveryShadowDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_reducer_shadow_receipt: Option<WorkGraphParentReducerShadowReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_wait_task_result_replay_consistency_decision:
        Option<WorkGraphWaitTaskResultReplayConsistencyDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_wait_surface_audit_replay_consistency_decision:
        Option<WorkGraphWaitSurfaceAuditReplayConsistencyDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_replay_consistency_decision:
        Option<WorkGraphCanonicalProjectionReplayConsistencyDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_wait_task_result_readback: Option<WorkGraphWaitTaskResultReadbackSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_wait_operator_matrix_row: Option<WorkGraphOperatorMatrixRow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_wait_surface_audit_packet: Option<WorkGraphWaitSurfaceAuditPacketSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_global_surface_audit_packet: Option<WorkGraphSurfaceAuditPacketSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_receipt: Option<WorkGraphCanonicalProjectionShadowReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_closeout_receipt:
        Option<WorkGraphCanonicalProjectionCloseoutReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_closeout_replay_consistency_decision:
        Option<WorkGraphCanonicalProjectionCloseoutReplayConsistencyDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_audit_chain_closeout_receipt:
        Option<WorkGraphCanonicalProjectionAuditChainCloseoutReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision:
        Option<WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_operator_review_packet:
        Option<WorkGraphCanonicalProjectionEnablementOperatorReviewPacket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision:
        Option<WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt:
        Option<WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision:
        Option<
            WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyDecision,
        >,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_audit_chain_closeout_receipt:
        Option<WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision:
        Option<WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_activation_precondition_operator_packet:
        Option<WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision:
        Option<
            WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyDecision,
        >,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt:
        Option<WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision:
        Option<
            WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyDecision,
        >,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt:
        Option<WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision:
        Option<
            WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyDecision,
        >,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet:
        Option<WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision:
        Option<
            WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyDecision,
        >,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet:
        Option<
            WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacket,
        >,
    #[serde(skip_serializing_if = "Option::is_none")]
    work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision:
        Option<
            WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyDecision,
        >,
    durable_mailbox: DurableMailboxWaitMetadata,
    work_graph_lifecycle_shadow_decision: WorkGraphRoleManifestShadowDecision,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct DurableMailboxWaitMetadata {
    opened_event_recorded: bool,
    terminal_event_recorded: bool,
    task_result_delivery_shadow_event_recorded: bool,
    parent_reducer_shadow_receipt_event_recorded: bool,
    task_result_replay_consistency_event_recorded: bool,
    wait_surface_audit_packet_event_recorded: bool,
    wait_surface_audit_replay_consistency_event_recorded: bool,
    wait_canonical_projection_receipt_event_recorded: bool,
    wait_canonical_projection_replay_consistency_event_recorded: bool,
    wait_canonical_projection_closeout_receipt_event_recorded: bool,
    wait_canonical_projection_closeout_replay_consistency_event_recorded: bool,
    wait_canonical_projection_audit_chain_closeout_receipt_event_recorded: bool,
    wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded: bool,
    wait_canonical_projection_enablement_operator_review_packet_event_recorded: bool,
    wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded: bool,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_audit_chain_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded:
        bool,
    live_blocking_enabled: bool,
    live_cutover_enabled: bool,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct WorkGraphTaskResultDeliveryShadowDecision {
    source_surface_id: &'static str,
    decision: &'static str,
    task_id: Option<String>,
    result_required: bool,
    task_result_envelope_present: bool,
    task_result_status: Option<String>,
    task_result_contract_id: &'static str,
    result_envelope_schema: &'static str,
    terminal_delivery_surface: &'static str,
    verifier_id: &'static str,
    shadow_delivery_ready: bool,
    wait_condition: String,
    timed_out: bool,
    checks: Vec<WorkGraphAdmissionShadowCheck>,
    feature_flag_enabled: bool,
    canary_stage: &'static str,
    live_blocking_enabled: bool,
    live_cutover_enabled: bool,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct WorkGraphParentReducerShadowReceipt {
    source_surface_id: &'static str,
    decision: &'static str,
    task_id: Option<String>,
    reducer_id: &'static str,
    task_result_envelope_observed: bool,
    parent_reducer_receipt_ready: bool,
    reduced_into_parent_work_graph: bool,
    checks: Vec<WorkGraphAdmissionShadowCheck>,
    feature_flag_enabled: bool,
    canary_stage: &'static str,
    live_blocking_enabled: bool,
    live_cutover_enabled: bool,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct WorkGraphWaitTaskResultReplayConsistencyDecision {
    source_surface_id: &'static str,
    decision: &'static str,
    replay_stage: &'static str,
    thread_id: String,
    barrier_id: String,
    readback_ready: bool,
    task_result_delivery_shadow_events: usize,
    parent_reducer_shadow_receipt_events: usize,
    prior_replay_consistency_events: usize,
    task_result_delivery_matches_readback: bool,
    parent_reducer_receipt_matches_readback: bool,
    no_live_guardrails_ready: bool,
    replay_consistent: bool,
    shadow_readiness_failed: bool,
    consistency_blockers: Vec<String>,
    checks: Vec<WorkGraphAdmissionShadowCheck>,
    feature_flag_enabled: bool,
    canary_stage: &'static str,
    live_blocking_enabled: bool,
    live_cutover_enabled: bool,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct WorkGraphWaitSurfaceAuditReplayConsistencyDecision {
    source_surface_id: &'static str,
    decision: &'static str,
    replay_stage: &'static str,
    thread_id: String,
    barrier_id: String,
    readback_ready: bool,
    wait_surface_audit_packet_events: usize,
    prior_wait_surface_audit_replay_consistency_events: usize,
    wait_surface_audit_packet_matches_readback: bool,
    no_live_guardrails_ready: bool,
    replay_consistent: bool,
    shadow_readiness_failed: bool,
    consistency_blockers: Vec<String>,
    checks: Vec<WorkGraphAdmissionShadowCheck>,
    feature_flag_enabled: bool,
    canary_stage: &'static str,
    live_blocking_enabled: bool,
    live_cutover_enabled: bool,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct WorkGraphWaitTaskResultReadbackSummary {
    thread_id: String,
    barrier_id: String,
    task_result_delivery_shadow_events: usize,
    parent_reducer_shadow_receipt_events: usize,
    task_result_replay_consistency_events: usize,
    wait_surface_audit_packet_events: usize,
    wait_surface_audit_replay_consistency_events: usize,
    wait_canonical_projection_receipt_events: usize,
    wait_canonical_projection_replay_consistency_events: usize,
    wait_canonical_projection_closeout_receipt_events: usize,
    wait_canonical_projection_closeout_replay_consistency_events: usize,
    wait_canonical_projection_audit_chain_closeout_receipt_events: usize,
    wait_canonical_projection_audit_chain_closeout_replay_consistency_events: usize,
    wait_canonical_projection_enablement_operator_review_packet_events: usize,
    wait_canonical_projection_enablement_operator_review_replay_consistency_events: usize,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_events: usize,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_events:
        usize,
    wait_canonical_projection_enablement_audit_chain_closeout_events: usize,
    wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_events: usize,
    wait_canonical_projection_enablement_activation_precondition_operator_packet_events: usize,
    wait_canonical_projection_enablement_activation_precondition_replay_consistency_events: usize,
    wait_canonical_projection_enablement_activation_no_live_closeout_events: usize,
    wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_events:
        usize,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_events: usize,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_events:
        usize,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_events:
        usize,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_events:
        usize,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_events:
        usize,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_events:
        usize,
    latest_task_result_delivery_decision: String,
    latest_parent_reducer_decision: String,
    latest_task_result_replay_consistency_decision: String,
    latest_wait_surface_audit_decision: String,
    latest_wait_surface_audit_replay_consistency_decision: String,
    latest_wait_canonical_projection_decision: String,
    latest_wait_canonical_projection_replay_consistency_decision: String,
    latest_wait_canonical_projection_closeout_decision: String,
    latest_wait_canonical_projection_closeout_replay_consistency_decision: String,
    latest_wait_canonical_projection_audit_chain_closeout_decision: String,
    latest_wait_canonical_projection_audit_chain_closeout_replay_consistency_decision: String,
    latest_wait_canonical_projection_enablement_operator_review_decision: String,
    latest_wait_canonical_projection_enablement_operator_review_replay_consistency_decision: String,
    latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_decision: String,
    latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision:
        String,
    latest_wait_canonical_projection_enablement_audit_chain_closeout_decision: String,
    latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision:
        String,
    latest_wait_canonical_projection_enablement_activation_precondition_operator_decision: String,
    latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency_decision:
        String,
    latest_wait_canonical_projection_enablement_activation_no_live_closeout_decision: String,
    latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision:
        String,
    latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_decision: String,
    latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision:
        String,
    latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_decision:
        String,
    latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision:
        String,
    latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_decision:
        String,
    latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision:
        String,
    task_result_delivery_readback_ready: bool,
    parent_reducer_readback_ready: bool,
    replay_consistency_ready: bool,
    wait_surface_audit_packet_readback_ready: bool,
    wait_surface_audit_replay_consistency_ready: bool,
    wait_canonical_projection_receipt_readback_ready: bool,
    wait_canonical_projection_replay_consistency_ready: bool,
    wait_canonical_projection_closeout_receipt_readback_ready: bool,
    wait_canonical_projection_closeout_replay_consistency_ready: bool,
    wait_canonical_projection_audit_chain_closeout_receipt_readback_ready: bool,
    wait_canonical_projection_audit_chain_closeout_replay_consistency_ready: bool,
    wait_canonical_projection_enablement_operator_review_packet_readback_ready: bool,
    wait_canonical_projection_enablement_operator_review_replay_consistency_ready: bool,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready: bool,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_ready: bool,
    wait_canonical_projection_enablement_audit_chain_closeout_readback_ready: bool,
    wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_ready: bool,
    wait_canonical_projection_enablement_activation_precondition_operator_packet_readback_ready:
        bool,
    wait_canonical_projection_enablement_activation_precondition_replay_consistency_ready: bool,
    wait_canonical_projection_enablement_activation_no_live_closeout_readback_ready: bool,
    wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_ready: bool,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_readback_ready: bool,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_ready:
        bool,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready:
        bool,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_ready:
        bool,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_readback_ready:
        bool,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_ready:
        bool,
    task_result_delivery_ready: bool,
    parent_reducer_receipt_ready: bool,
    replay_consistent: bool,
    wait_surface_audit_packet_ready: bool,
    wait_surface_audit_replay_consistent: bool,
    wait_canonical_projection_receipt_ready: bool,
    wait_canonical_projection_replay_consistent: bool,
    wait_canonical_projection_closeout_receipt_ready: bool,
    wait_canonical_projection_closeout_replay_consistent: bool,
    wait_canonical_projection_audit_chain_closeout_receipt_ready: bool,
    wait_canonical_projection_audit_chain_closeout_replay_consistent: bool,
    wait_canonical_projection_enablement_operator_review_ready: bool,
    wait_canonical_projection_enablement_operator_review_replay_consistent: bool,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready: bool,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent: bool,
    wait_canonical_projection_enablement_audit_chain_closeout_ready: bool,
    wait_canonical_projection_enablement_audit_chain_closeout_replay_consistent: bool,
    wait_canonical_projection_enablement_activation_precondition_ready: bool,
    wait_canonical_projection_enablement_activation_precondition_replay_consistent: bool,
    wait_canonical_projection_enablement_activation_no_live_closeout_ready: bool,
    wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistent: bool,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_ready: bool,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent: bool,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready:
        bool,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent:
        bool,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready:
        bool,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent:
        bool,
    no_live_guardrails_ready: bool,
    readback_ready: bool,
    direct_wait_task_result_ready: bool,
    direct_wait_surface_audit_ready: bool,
    direct_wait_canonical_projection_ready: bool,
    direct_wait_canonical_projection_closeout_ready: bool,
    direct_wait_canonical_projection_audit_chain_closeout_ready: bool,
    direct_wait_canonical_projection_audit_chain_closeout_replay_ready: bool,
    direct_wait_canonical_projection_enablement_operator_review_ready: bool,
    direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready: bool,
    direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_ready: bool,
    direct_wait_canonical_projection_enablement_audit_chain_closeout_ready: bool,
    direct_wait_canonical_projection_enablement_audit_chain_closeout_replay_ready: bool,
    direct_wait_canonical_projection_enablement_activation_precondition_ready: bool,
    direct_wait_canonical_projection_enablement_activation_precondition_replay_ready: bool,
    direct_wait_canonical_projection_enablement_activation_no_live_closeout_ready: bool,
    direct_wait_canonical_projection_enablement_activation_no_live_closeout_replay_ready: bool,
    direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_ready: bool,
    direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_ready: bool,
    direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready:
        bool,
    direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_ready:
        bool,
    direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready:
        bool,
    direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_ready:
        bool,
    task_result_delivery_shadow_event_recorded: bool,
    parent_reducer_shadow_receipt_event_recorded: bool,
    task_result_replay_consistency_event_recorded: bool,
    wait_surface_audit_packet_event_recorded: bool,
    wait_surface_audit_replay_consistency_event_recorded: bool,
    wait_canonical_projection_receipt_event_recorded: bool,
    wait_canonical_projection_replay_consistency_event_recorded: bool,
    wait_canonical_projection_closeout_receipt_event_recorded: bool,
    wait_canonical_projection_closeout_replay_consistency_event_recorded: bool,
    wait_canonical_projection_audit_chain_closeout_receipt_event_recorded: bool,
    wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded: bool,
    wait_canonical_projection_enablement_operator_review_packet_event_recorded: bool,
    wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded: bool,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_audit_chain_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded:
        bool,
    live_blocking_event_count: usize,
    live_cutover_event_count: usize,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct WorkGraphWaitSurfaceAuditPacketSummary {
    decision: &'static str,
    audit_stage: &'static str,
    source_surface_id: &'static str,
    thread_id: String,
    barrier_id: String,
    audit_chain_segment_count: usize,
    audit_chain_ready_segment_count: usize,
    audit_chain_missing_segment_ids: Vec<String>,
    audit_chain_inconsistent_segment_ids: Vec<String>,
    audit_chain_ready: bool,
    audit_chain: WorkGraphAuditChainSummary,
    operator_matrix_row_count: usize,
    operator_matrix_ready_row_count: usize,
    operator_matrix_blocked_row_count: usize,
    operator_matrix_rows: Vec<WorkGraphOperatorMatrixRow>,
    audit_packet_ready: bool,
    audit_blockers: Vec<String>,
    recommended_next_action: &'static str,
    feature_flag_enabled: bool,
    canary_stage: &'static str,
    live_blocking_enabled: bool,
    live_cutover_enabled: bool,
}

struct WaitWorkGraphShadowRecordInput<'a> {
    state_db: Option<crate::StateDbHandle>,
    thread_id: codex_protocol::ThreadId,
    barrier_id: &'a str,
    task_id: Option<&'a str>,
    task_name: Option<&'a str>,
    event_type: &'static str,
    status: Option<&'static str>,
    payload_json: Option<Value>,
    trace_id: Option<&'a str>,
}

impl WaitAgentResult {
    fn from_timed_out(timed_out: bool) -> Self {
        let message = if timed_out {
            "Wait timed out."
        } else {
            "Wait completed."
        };
        Self {
            message: message.to_string(),
            timed_out,
        }
    }
}

fn build_task_result_delivery_shadow_decision(
    result_required: bool,
    task_id: Option<&str>,
    task_result: Option<&Value>,
    wait_condition: &str,
    timed_out: bool,
) -> Option<WorkGraphTaskResultDeliveryShadowDecision> {
    if !result_required {
        return None;
    }

    let task_result_envelope_present = task_result.is_some();
    let shadow_delivery_ready = task_result_envelope_present;
    let task_result_status = task_result
        .and_then(|result| result.get("status"))
        .and_then(Value::as_str)
        .map(str::to_string);
    let decision = if shadow_delivery_ready {
        "task_result_delivery_recorded_shadow_no_live_cutover"
    } else {
        "task_result_delivery_blocked_shadow_no_live_cutover"
    };
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "result_required",
            passed: result_required,
            detail: "wait_agent was invoked with result_required=true".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "task_id_present",
            passed: task_id.is_some(),
            detail: task_id
                .map(|task_id| format!("task_id={task_id}"))
                .unwrap_or_else(|| "task_id missing".to_string()),
        },
        WorkGraphAdmissionShadowCheck {
            name: "task_result_envelope_present",
            passed: task_result_envelope_present,
            detail: if task_result_envelope_present {
                "durable TaskResultEnvelope evidence was read back".to_string()
            } else {
                "durable TaskResultEnvelope evidence is not available yet".to_string()
            },
        },
        WorkGraphAdmissionShadowCheck {
            name: "terminal_delivery_surface_shadow_only",
            passed: true,
            detail: "terminal delivery surface is wait_agent(result_required=true) in shadow only"
                .to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_delivery_mutation",
            passed: true,
            detail: "TaskResult delivery does not mutate live parent state".to_string(),
        },
    ];

    Some(WorkGraphTaskResultDeliveryShadowDecision {
        source_surface_id: "wait_agent",
        decision,
        task_id: task_id.map(str::to_string),
        result_required,
        task_result_envelope_present,
        task_result_status,
        task_result_contract_id: "subagent_task_result_contract_v1",
        result_envelope_schema: "hepta.task_result.v1",
        terminal_delivery_surface: "wait_agent(result_required=true)",
        verifier_id: "subagent_task_result_verifier_v1",
        shadow_delivery_ready,
        wait_condition: wait_condition.to_string(),
        timed_out,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    })
}

fn build_parent_reducer_shadow_receipt(
    result_required: bool,
    task_id: Option<&str>,
    delivery: Option<&WorkGraphTaskResultDeliveryShadowDecision>,
) -> Option<WorkGraphParentReducerShadowReceipt> {
    if !result_required {
        return None;
    }

    let task_result_envelope_observed = delivery.is_some_and(|delivery| {
        delivery.task_result_envelope_present && delivery.shadow_delivery_ready
    });
    let parent_reducer_receipt_ready = task_result_envelope_observed;
    let decision = if parent_reducer_receipt_ready {
        "parent_reducer_shadow_receipt_recorded_shadow_no_live_cutover"
    } else {
        "parent_reducer_shadow_receipt_blocked_shadow_no_live_cutover"
    };
    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "task_result_delivery_ready",
            passed: task_result_envelope_observed,
            detail: if task_result_envelope_observed {
                "TaskResultEnvelope delivery shadow evidence is ready".to_string()
            } else {
                "parent reducer is waiting for TaskResultEnvelope delivery shadow evidence"
                    .to_string()
            },
        },
        WorkGraphAdmissionShadowCheck {
            name: "parent_reducer_shadow_only",
            passed: true,
            detail: "parent reducer receipt is recorded as shadow evidence only".to_string(),
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_parent_graph_mutation",
            passed: true,
            detail: "parent WorkGraph state is not mutated by this receipt".to_string(),
        },
    ];

    Some(WorkGraphParentReducerShadowReceipt {
        source_surface_id: "wait_agent",
        decision,
        task_id: task_id.map(str::to_string),
        reducer_id: "subagent_parent_reducer_v1",
        task_result_envelope_observed,
        parent_reducer_receipt_ready,
        reduced_into_parent_work_graph: false,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    })
}

fn build_wait_task_result_replay_consistency_decision(
    result_required: bool,
    thread_id: codex_protocol::ThreadId,
    barrier_id: &str,
    task_result_delivery_shadow: Option<&WorkGraphTaskResultDeliveryShadowDecision>,
    parent_reducer_shadow_receipt: Option<&WorkGraphParentReducerShadowReceipt>,
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
) -> Option<WorkGraphWaitTaskResultReplayConsistencyDecision> {
    if !result_required {
        return None;
    }

    let task_result_delivery_shadow_json =
        task_result_delivery_shadow.and_then(|decision| serde_json::to_value(decision).ok());
    let parent_reducer_shadow_receipt_json =
        parent_reducer_shadow_receipt.and_then(|receipt| serde_json::to_value(receipt).ok());
    let readback_ready = readback.is_some_and(|readback| readback.readback_ready);
    let no_live_guardrails_ready =
        readback.is_some_and(|readback| readback.no_live_guardrails_ready);
    let task_result_delivery_matches_readback = readback
        .and_then(|readback| readback.latest_task_result_delivery_shadow.as_ref())
        == task_result_delivery_shadow_json.as_ref();
    let parent_reducer_receipt_matches_readback = readback
        .and_then(|readback| readback.latest_parent_reducer_shadow_receipt.as_ref())
        == parent_reducer_shadow_receipt_json.as_ref();
    let task_result_delivery_shadow_events = readback
        .map(|readback| readback.task_result_delivery_shadow_events)
        .unwrap_or_default();
    let parent_reducer_shadow_receipt_events = readback
        .map(|readback| readback.parent_reducer_shadow_receipt_events)
        .unwrap_or_default();
    let prior_replay_consistency_events = readback
        .map(|readback| readback.task_result_replay_consistency_events)
        .unwrap_or_default();
    let replay_consistent = readback_ready
        && task_result_delivery_matches_readback
        && parent_reducer_receipt_matches_readback
        && no_live_guardrails_ready;
    let shadow_readiness_failed = !replay_consistent;
    let decision = if replay_consistent {
        "wait_task_result_replay_consistent_shadow_no_live_cutover"
    } else {
        "wait_task_result_replay_mismatch_shadow_no_live_cutover"
    };
    let mut consistency_blockers = Vec::new();
    if readback.is_none() {
        consistency_blockers.push("wait_task_result_readback_missing".to_string());
    }
    if !readback_ready {
        consistency_blockers.push("wait_task_result_readback_not_ready".to_string());
    }
    if task_result_delivery_shadow_json.is_none() {
        consistency_blockers.push("task_result_delivery_shadow_missing".to_string());
    }
    if parent_reducer_shadow_receipt_json.is_none() {
        consistency_blockers.push("parent_reducer_shadow_receipt_missing".to_string());
    }
    if !task_result_delivery_matches_readback {
        consistency_blockers.push("task_result_delivery_shadow_replay_mismatch".to_string());
    }
    if !parent_reducer_receipt_matches_readback {
        consistency_blockers.push("parent_reducer_shadow_receipt_replay_mismatch".to_string());
    }
    if !no_live_guardrails_ready {
        consistency_blockers.push("wait_task_result_live_guardrail_event_present".to_string());
    }
    consistency_blockers.sort();
    consistency_blockers.dedup();

    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "readback_ready",
            passed: readback_ready,
            detail: if readback_ready {
                "delivery and parent reducer shadow events were read back".to_string()
            } else {
                "delivery and parent reducer shadow readback is not complete".to_string()
            },
        },
        WorkGraphAdmissionShadowCheck {
            name: "task_result_delivery_matches_readback",
            passed: task_result_delivery_matches_readback,
            detail: if task_result_delivery_matches_readback {
                "tool-result delivery shadow matches durable latest payload".to_string()
            } else {
                "tool-result delivery shadow does not match durable latest payload".to_string()
            },
        },
        WorkGraphAdmissionShadowCheck {
            name: "parent_reducer_matches_readback",
            passed: parent_reducer_receipt_matches_readback,
            detail: if parent_reducer_receipt_matches_readback {
                "tool-result parent reducer receipt matches durable latest payload".to_string()
            } else {
                "tool-result parent reducer receipt does not match durable latest payload"
                    .to_string()
            },
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_guardrails_ready",
            passed: no_live_guardrails_ready,
            detail: "direct wait replay gate remains shadow-only with no live cutover".to_string(),
        },
    ];

    Some(WorkGraphWaitTaskResultReplayConsistencyDecision {
        source_surface_id: "wait_agent",
        decision,
        replay_stage: "direct_wait_task_result_delivery",
        thread_id: thread_id.to_string(),
        barrier_id: barrier_id.to_string(),
        readback_ready,
        task_result_delivery_shadow_events,
        parent_reducer_shadow_receipt_events,
        prior_replay_consistency_events,
        task_result_delivery_matches_readback,
        parent_reducer_receipt_matches_readback,
        no_live_guardrails_ready,
        replay_consistent,
        shadow_readiness_failed,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    })
}

fn build_wait_surface_audit_replay_consistency_decision(
    result_required: bool,
    thread_id: codex_protocol::ThreadId,
    barrier_id: &str,
    wait_surface_audit_packet: Option<&WorkGraphWaitSurfaceAuditPacketSummary>,
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
) -> Option<WorkGraphWaitSurfaceAuditReplayConsistencyDecision> {
    if !result_required {
        return None;
    }

    let wait_surface_audit_packet_json =
        wait_surface_audit_packet.and_then(|packet| serde_json::to_value(packet).ok());
    let readback_ready =
        readback.is_some_and(|readback| readback.wait_surface_audit_packet_readback_ready);
    let no_live_guardrails_ready =
        readback.is_some_and(|readback| readback.no_live_guardrails_ready);
    let wait_surface_audit_packet_matches_readback = readback
        .and_then(|readback| readback.latest_wait_surface_audit_packet.as_ref())
        == wait_surface_audit_packet_json.as_ref();
    let wait_surface_audit_packet_events = readback
        .map(|readback| readback.wait_surface_audit_packet_events)
        .unwrap_or_default();
    let prior_wait_surface_audit_replay_consistency_events = readback
        .map(|readback| readback.wait_surface_audit_replay_consistency_events)
        .unwrap_or_default();
    let replay_consistent =
        readback_ready && wait_surface_audit_packet_matches_readback && no_live_guardrails_ready;
    let shadow_readiness_failed = !replay_consistent;
    let decision = if replay_consistent {
        "wait_surface_audit_replay_consistent_shadow_no_live_cutover"
    } else {
        "wait_surface_audit_replay_mismatch_shadow_no_live_cutover"
    };
    let mut consistency_blockers = Vec::new();
    if readback.is_none() {
        consistency_blockers.push("wait_surface_audit_readback_missing".to_string());
    }
    if !readback_ready {
        consistency_blockers.push("wait_surface_audit_packet_readback_not_ready".to_string());
    }
    if wait_surface_audit_packet_json.is_none() {
        consistency_blockers.push("wait_surface_audit_packet_missing".to_string());
    }
    if !wait_surface_audit_packet_matches_readback {
        consistency_blockers.push("wait_surface_audit_packet_replay_mismatch".to_string());
    }
    if !no_live_guardrails_ready {
        consistency_blockers.push("wait_surface_audit_live_guardrail_event_present".to_string());
    }
    consistency_blockers.sort();
    consistency_blockers.dedup();

    let checks = vec![
        WorkGraphAdmissionShadowCheck {
            name: "surface_audit_packet_readback_ready",
            passed: readback_ready,
            detail: if readback_ready {
                "surface-audit packet event was read back from durable mailbox stream".to_string()
            } else {
                "surface-audit packet durable readback is not complete".to_string()
            },
        },
        WorkGraphAdmissionShadowCheck {
            name: "surface_audit_packet_matches_readback",
            passed: wait_surface_audit_packet_matches_readback,
            detail: if wait_surface_audit_packet_matches_readback {
                "tool-result surface-audit packet matches durable latest payload".to_string()
            } else {
                "tool-result surface-audit packet does not match durable latest payload".to_string()
            },
        },
        WorkGraphAdmissionShadowCheck {
            name: "no_live_guardrails_ready",
            passed: no_live_guardrails_ready,
            detail:
                "direct wait surface-audit replay gate remains shadow-only with no live cutover"
                    .to_string(),
        },
    ];

    Some(WorkGraphWaitSurfaceAuditReplayConsistencyDecision {
        source_surface_id: "wait_agent",
        decision,
        replay_stage: "direct_wait_surface_audit_packet",
        thread_id: thread_id.to_string(),
        barrier_id: barrier_id.to_string(),
        readback_ready,
        wait_surface_audit_packet_events,
        prior_wait_surface_audit_replay_consistency_events,
        wait_surface_audit_packet_matches_readback,
        no_live_guardrails_ready,
        replay_consistent,
        shadow_readiness_failed,
        consistency_blockers,
        checks,
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    })
}

fn build_wait_canonical_projection_replay_consistency_decision(
    result_required: bool,
    projection_receipt: Option<&WorkGraphCanonicalProjectionShadowReceipt>,
    projection_receipt_payload: Option<&Value>,
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
) -> Option<WorkGraphCanonicalProjectionReplayConsistencyDecision> {
    if !result_required {
        return None;
    }

    let (Some(projection_receipt), Some(projection_receipt_payload)) =
        (projection_receipt, projection_receipt_payload)
    else {
        return None;
    };

    Some(
        build_work_graph_canonical_projection_replay_consistency_decision(
            WorkGraphCanonicalProjectionReplayConsistencyInput {
                source_surface_id: "wait_agent",
                projection_receipt,
                projection_receipt_payload,
                latest_projection_receipt_payload: readback.and_then(|readback| {
                    readback.latest_wait_canonical_projection_receipt.as_ref()
                }),
                projection_receipt_events: readback
                    .map(|readback| readback.wait_canonical_projection_receipt_events)
                    .unwrap_or_default(),
                projection_receipt_readback_ready: readback.is_some_and(|readback| {
                    readback.wait_canonical_projection_receipt_readback_ready
                }),
                prior_projection_replay_consistency_events: readback
                    .map(|readback| readback.wait_canonical_projection_replay_consistency_events)
                    .unwrap_or_default(),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

fn build_wait_canonical_projection_closeout_receipt(
    result_required: bool,
    projection_receipt: Option<&WorkGraphCanonicalProjectionShadowReceipt>,
    replay_consistency_decision: Option<&WorkGraphCanonicalProjectionReplayConsistencyDecision>,
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
) -> Option<WorkGraphCanonicalProjectionCloseoutReceipt> {
    if !result_required {
        return None;
    }

    let (Some(projection_receipt), Some(replay_consistency_decision)) =
        (projection_receipt, replay_consistency_decision)
    else {
        return None;
    };

    Some(build_work_graph_canonical_projection_closeout_receipt(
        WorkGraphCanonicalProjectionCloseoutReceiptInput {
            source_surface_id: "wait_agent",
            projection_receipt,
            replay_consistency_decision,
            projection_receipt_events: readback
                .map(|readback| readback.wait_canonical_projection_receipt_events)
                .unwrap_or_default(),
            projection_replay_consistency_events: readback
                .map(|readback| readback.wait_canonical_projection_replay_consistency_events)
                .unwrap_or_default(),
            prior_projection_closeout_receipt_events: readback
                .map(|readback| readback.wait_canonical_projection_closeout_receipt_events)
                .unwrap_or_default(),
            projection_receipt_readback_ready: readback
                .is_some_and(|readback| readback.wait_canonical_projection_receipt_readback_ready),
            projection_replay_consistency_ready: readback.is_some_and(|readback| {
                readback.wait_canonical_projection_replay_consistency_ready
            }),
            live_blocking_event_count: readback
                .map(|readback| readback.live_blocking_event_count)
                .unwrap_or_default(),
            live_cutover_event_count: readback
                .map(|readback| readback.live_cutover_event_count)
                .unwrap_or_default(),
        },
    ))
}

fn build_wait_canonical_projection_closeout_replay_consistency_decision(
    result_required: bool,
    closeout_receipt: Option<&WorkGraphCanonicalProjectionCloseoutReceipt>,
    closeout_receipt_payload: Option<&Value>,
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
) -> Option<WorkGraphCanonicalProjectionCloseoutReplayConsistencyDecision> {
    if !result_required {
        return None;
    }

    let (Some(closeout_receipt), Some(closeout_receipt_payload)) =
        (closeout_receipt, closeout_receipt_payload)
    else {
        return None;
    };

    Some(
        build_work_graph_canonical_projection_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionCloseoutReplayConsistencyInput {
                source_surface_id: "wait_agent",
                closeout_receipt,
                closeout_receipt_payload,
                latest_closeout_receipt_payload: readback.and_then(|readback| {
                    readback
                        .latest_wait_canonical_projection_closeout_receipt
                        .as_ref()
                }),
                closeout_receipt_events: readback
                    .map(|readback| readback.wait_canonical_projection_closeout_receipt_events)
                    .unwrap_or_default(),
                closeout_receipt_readback_ready: readback.is_some_and(|readback| {
                    readback.wait_canonical_projection_closeout_receipt_readback_ready
                }),
                prior_closeout_replay_consistency_events: readback
                    .map(|readback| {
                        readback.wait_canonical_projection_closeout_replay_consistency_events
                    })
                    .unwrap_or_default(),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

fn build_wait_canonical_projection_audit_chain_closeout_receipt(
    result_required: bool,
    projection_receipt: Option<&WorkGraphCanonicalProjectionShadowReceipt>,
    projection_replay_consistency_decision: Option<
        &WorkGraphCanonicalProjectionReplayConsistencyDecision,
    >,
    closeout_receipt: Option<&WorkGraphCanonicalProjectionCloseoutReceipt>,
    closeout_replay_consistency_decision: Option<
        &WorkGraphCanonicalProjectionCloseoutReplayConsistencyDecision,
    >,
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
) -> Option<WorkGraphCanonicalProjectionAuditChainCloseoutReceipt> {
    if !result_required {
        return None;
    }

    let (
        Some(projection_receipt),
        Some(projection_replay_consistency_decision),
        Some(closeout_receipt),
        Some(closeout_replay_consistency_decision),
    ) = (
        projection_receipt,
        projection_replay_consistency_decision,
        closeout_receipt,
        closeout_replay_consistency_decision,
    )
    else {
        return None;
    };

    Some(
        build_work_graph_canonical_projection_audit_chain_closeout_receipt(
            WorkGraphCanonicalProjectionAuditChainCloseoutReceiptInput {
                source_surface_id: "wait_agent",
                projection_receipt,
                projection_replay_consistency_decision,
                closeout_receipt,
                closeout_replay_consistency_decision,
                projection_receipt_events: readback
                    .map(|readback| readback.wait_canonical_projection_receipt_events)
                    .unwrap_or_default(),
                projection_replay_consistency_events: readback
                    .map(|readback| readback.wait_canonical_projection_replay_consistency_events)
                    .unwrap_or_default(),
                closeout_receipt_events: readback
                    .map(|readback| readback.wait_canonical_projection_closeout_receipt_events)
                    .unwrap_or_default(),
                closeout_replay_consistency_events: readback
                    .map(|readback| {
                        readback.wait_canonical_projection_closeout_replay_consistency_events
                    })
                    .unwrap_or_default(),
                prior_audit_chain_closeout_receipt_events: readback
                    .map(|readback| {
                        readback.wait_canonical_projection_audit_chain_closeout_receipt_events
                    })
                    .unwrap_or_default(),
                projection_receipt_readback_ready: readback.is_some_and(|readback| {
                    readback.wait_canonical_projection_receipt_readback_ready
                }),
                projection_replay_consistency_ready: readback.is_some_and(|readback| {
                    readback.wait_canonical_projection_replay_consistency_ready
                }),
                closeout_receipt_readback_ready: readback.is_some_and(|readback| {
                    readback.wait_canonical_projection_closeout_receipt_readback_ready
                }),
                closeout_replay_consistency_ready: readback.is_some_and(|readback| {
                    readback.wait_canonical_projection_closeout_replay_consistency_ready
                }),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

fn build_wait_canonical_projection_audit_chain_closeout_replay_consistency_decision(
    result_required: bool,
    audit_chain_closeout_receipt: Option<&WorkGraphCanonicalProjectionAuditChainCloseoutReceipt>,
    audit_chain_closeout_receipt_payload: Option<&Value>,
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
) -> Option<WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyDecision> {
    if !result_required {
        return None;
    }

    let (Some(audit_chain_closeout_receipt), Some(audit_chain_closeout_receipt_payload)) = (
        audit_chain_closeout_receipt,
        audit_chain_closeout_receipt_payload,
    ) else {
        return None;
    };

    Some(
        build_work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyInput {
                source_surface_id: "wait_agent",
                audit_chain_closeout_receipt,
                audit_chain_closeout_receipt_payload,
                latest_audit_chain_closeout_receipt_payload: readback.and_then(|readback| {
                    readback
                        .latest_wait_canonical_projection_audit_chain_closeout_receipt
                        .as_ref()
                }),
                audit_chain_closeout_receipt_events: readback
                    .map(|readback| {
                        readback.wait_canonical_projection_audit_chain_closeout_receipt_events
                    })
                    .unwrap_or_default(),
                audit_chain_closeout_receipt_readback_ready: readback.is_some_and(|readback| {
                    readback
                        .wait_canonical_projection_audit_chain_closeout_receipt_readback_ready
                }),
                prior_audit_chain_closeout_replay_consistency_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_audit_chain_closeout_replay_consistency_events
                    })
                    .unwrap_or_default(),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementOperatorReviewPacketInput<'a> {
    result_required: bool,
    projection_receipt: Option<&'a WorkGraphCanonicalProjectionShadowReceipt>,
    projection_replay_consistency_decision:
        Option<&'a WorkGraphCanonicalProjectionReplayConsistencyDecision>,
    closeout_receipt: Option<&'a WorkGraphCanonicalProjectionCloseoutReceipt>,
    closeout_replay_consistency_decision:
        Option<&'a WorkGraphCanonicalProjectionCloseoutReplayConsistencyDecision>,
    audit_chain_closeout_receipt: Option<&'a WorkGraphCanonicalProjectionAuditChainCloseoutReceipt>,
    audit_chain_closeout_replay_consistency_decision:
        Option<&'a WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyDecision>,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_operator_review_packet(
    input: WaitCanonicalProjectionEnablementOperatorReviewPacketInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementOperatorReviewPacket> {
    if !input.result_required {
        return None;
    }

    let (
        Some(projection_receipt),
        Some(projection_replay_consistency_decision),
        Some(closeout_receipt),
        Some(closeout_replay_consistency_decision),
        Some(audit_chain_closeout_receipt),
        Some(audit_chain_closeout_replay_consistency_decision),
    ) = (
        input.projection_receipt,
        input.projection_replay_consistency_decision,
        input.closeout_receipt,
        input.closeout_replay_consistency_decision,
        input.audit_chain_closeout_receipt,
        input.audit_chain_closeout_replay_consistency_decision,
    )
    else {
        return None;
    };

    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_operator_review_packet(
            WorkGraphCanonicalProjectionEnablementOperatorReviewPacketInput {
                source_surface_id: "wait_agent",
                projection_receipt,
                projection_replay_consistency_decision,
                closeout_receipt,
                closeout_replay_consistency_decision,
                audit_chain_closeout_receipt,
                audit_chain_closeout_replay_consistency_decision,
                projection_receipt_events: readback
                    .map(|readback| readback.wait_canonical_projection_receipt_events)
                    .unwrap_or_default(),
                projection_replay_consistency_events: readback
                    .map(|readback| readback.wait_canonical_projection_replay_consistency_events)
                    .unwrap_or_default(),
                closeout_receipt_events: readback
                    .map(|readback| readback.wait_canonical_projection_closeout_receipt_events)
                    .unwrap_or_default(),
                closeout_replay_consistency_events: readback
                    .map(|readback| {
                        readback.wait_canonical_projection_closeout_replay_consistency_events
                    })
                    .unwrap_or_default(),
                audit_chain_closeout_receipt_events: readback
                    .map(|readback| {
                        readback.wait_canonical_projection_audit_chain_closeout_receipt_events
                    })
                    .unwrap_or_default(),
                audit_chain_closeout_replay_consistency_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_audit_chain_closeout_replay_consistency_events
                    })
                    .unwrap_or_default(),
                prior_enablement_operator_review_packet_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_operator_review_packet_events
                    })
                    .unwrap_or_default(),
                projection_receipt_readback_ready: readback.is_some_and(|readback| {
                    readback.wait_canonical_projection_receipt_readback_ready
                }),
                projection_replay_consistency_ready: readback.is_some_and(|readback| {
                    readback.wait_canonical_projection_replay_consistency_ready
                }),
                closeout_receipt_readback_ready: readback.is_some_and(|readback| {
                    readback.wait_canonical_projection_closeout_receipt_readback_ready
                }),
                closeout_replay_consistency_ready: readback.is_some_and(|readback| {
                    readback.wait_canonical_projection_closeout_replay_consistency_ready
                }),
                audit_chain_closeout_receipt_readback_ready: readback.is_some_and(|readback| {
                    readback
                        .wait_canonical_projection_audit_chain_closeout_receipt_readback_ready
                }),
                audit_chain_closeout_replay_consistency_ready: readback.is_some_and(|readback| {
                    readback
                        .wait_canonical_projection_audit_chain_closeout_replay_consistency_ready
                }),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput<'a> {
    result_required: bool,
    enablement_operator_review_packet:
        Option<&'a WorkGraphCanonicalProjectionEnablementOperatorReviewPacket>,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_operator_review_replay_consistency_decision(
    input: WaitCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyDecision> {
    if !input.result_required {
        return None;
    }

    let enablement_operator_review_packet = input.enablement_operator_review_packet?;
    let enablement_operator_review_packet_payload =
        serde_json::to_value(enablement_operator_review_packet).ok()?;
    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput {
                source_surface_id: "wait_agent",
                enablement_operator_review_packet,
                enablement_operator_review_packet_payload: &enablement_operator_review_packet_payload,
                latest_enablement_operator_review_packet_payload: readback.and_then(|readback| {
                    readback
                        .latest_wait_canonical_projection_enablement_operator_review_packet
                        .as_ref()
                }),
                enablement_operator_review_packet_events: readback
                    .map(|readback| {
                        readback.wait_canonical_projection_enablement_operator_review_packet_events
                    })
                    .unwrap_or_default(),
                enablement_operator_review_packet_readback_ready: readback.is_some_and(
                    |readback| {
                        readback
                            .wait_canonical_projection_enablement_operator_review_packet_readback_ready
                    },
                ),
                prior_enablement_operator_review_replay_consistency_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_operator_review_replay_consistency_events
                    })
                    .unwrap_or_default(),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput<'a> {
    result_required: bool,
    enablement_operator_review_packet:
        Option<&'a WorkGraphCanonicalProjectionEnablementOperatorReviewPacket>,
    enablement_operator_review_replay_consistency_decision:
        Option<&'a WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyDecision>,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_no_live_rehearsal_closeout_receipt(
    input: WaitCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReceipt> {
    if !input.result_required {
        return None;
    }

    let enablement_operator_review_packet = input.enablement_operator_review_packet?;
    let enablement_operator_review_replay_consistency_decision =
        input.enablement_operator_review_replay_consistency_decision?;
    let readback = input.readback;

    Some(build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt(
        WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput {
            source_surface_id: "wait_agent",
            enablement_operator_review_packet,
            enablement_operator_review_replay_consistency_decision,
            enablement_operator_review_packet_events: readback
                .map(|readback| {
                    readback.wait_canonical_projection_enablement_operator_review_packet_events
                })
                .unwrap_or_default(),
            enablement_operator_review_replay_consistency_events: readback
                .map(|readback| {
                    readback
                        .wait_canonical_projection_enablement_operator_review_replay_consistency_events
                })
                .unwrap_or_default(),
            prior_enablement_no_live_rehearsal_closeout_events: readback
                .map(|readback| {
                    readback
                        .wait_canonical_projection_enablement_no_live_rehearsal_closeout_events
                })
                .unwrap_or_default(),
            enablement_operator_review_packet_readback_ready: readback.is_some_and(|readback| {
                readback.wait_canonical_projection_enablement_operator_review_packet_readback_ready
            }),
            enablement_operator_review_replay_consistency_ready: readback.is_some_and(
                |readback| {
                    readback
                        .wait_canonical_projection_enablement_operator_review_replay_consistency_ready
                },
            ),
            live_blocking_event_count: readback
                .map(|readback| readback.live_blocking_event_count)
                .unwrap_or_default(),
            live_cutover_event_count: readback
                .map(|readback| readback.live_cutover_event_count)
                .unwrap_or_default(),
        },
    ))
}

struct WaitCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput<'a> {
    result_required: bool,
    no_live_rehearsal_closeout_receipt:
        Option<&'a WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReceipt>,
    no_live_rehearsal_closeout_receipt_payload: Option<&'a Value>,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision(
    input: WaitCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyDecision>
{
    if !input.result_required {
        return None;
    }

    let no_live_rehearsal_closeout_receipt = input.no_live_rehearsal_closeout_receipt?;
    let no_live_rehearsal_closeout_receipt_payload =
        input.no_live_rehearsal_closeout_receipt_payload?;
    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput {
                source_surface_id: "wait_agent",
                no_live_rehearsal_closeout_receipt,
                no_live_rehearsal_closeout_receipt_payload,
                latest_no_live_rehearsal_closeout_receipt_payload: readback.and_then(|readback| {
                    readback
                        .latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout
                        .as_ref()
                }),
                no_live_rehearsal_closeout_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_no_live_rehearsal_closeout_events
                    })
                    .unwrap_or_default(),
                no_live_rehearsal_closeout_readback_ready: readback.is_some_and(|readback| {
                    readback
                        .wait_canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready
                }),
                prior_no_live_rehearsal_closeout_replay_consistency_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_events
                    })
                    .unwrap_or_default(),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementAuditChainCloseoutInput<'a> {
    result_required: bool,
    enablement_operator_review_packet:
        Option<&'a WorkGraphCanonicalProjectionEnablementOperatorReviewPacket>,
    enablement_operator_review_replay_consistency_decision:
        Option<&'a WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyDecision>,
    no_live_rehearsal_closeout_receipt:
        Option<&'a WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReceipt>,
    no_live_rehearsal_closeout_replay_consistency_decision: Option<
        &'a WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyDecision,
    >,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_audit_chain_closeout_receipt(
    input: WaitCanonicalProjectionEnablementAuditChainCloseoutInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReceipt> {
    if !input.result_required {
        return None;
    }

    let enablement_operator_review_packet = input.enablement_operator_review_packet?;
    let enablement_operator_review_replay_consistency_decision =
        input.enablement_operator_review_replay_consistency_decision?;
    let no_live_rehearsal_closeout_receipt = input.no_live_rehearsal_closeout_receipt?;
    let no_live_rehearsal_closeout_replay_consistency_decision =
        input.no_live_rehearsal_closeout_replay_consistency_decision?;
    let readback = input.readback;

    Some(build_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt(
        WorkGraphCanonicalProjectionEnablementAuditChainCloseoutInput {
            source_surface_id: "wait_agent",
            enablement_operator_review_packet,
            enablement_operator_review_replay_consistency_decision,
            no_live_rehearsal_closeout_receipt,
            no_live_rehearsal_closeout_replay_consistency_decision,
            enablement_operator_review_packet_events: readback
                .map(|readback| {
                    readback.wait_canonical_projection_enablement_operator_review_packet_events
                })
                .unwrap_or_default(),
            enablement_operator_review_replay_consistency_events: readback
                .map(|readback| {
                    readback
                        .wait_canonical_projection_enablement_operator_review_replay_consistency_events
                })
                .unwrap_or_default(),
            no_live_rehearsal_closeout_events: readback
                .map(|readback| {
                    readback.wait_canonical_projection_enablement_no_live_rehearsal_closeout_events
                })
                .unwrap_or_default(),
            no_live_rehearsal_closeout_replay_consistency_events: readback
                .map(|readback| {
                    readback
                        .wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_events
                })
                .unwrap_or_default(),
            prior_enablement_audit_chain_closeout_events: readback
                .map(|readback| {
                    readback.wait_canonical_projection_enablement_audit_chain_closeout_events
                })
                .unwrap_or_default(),
            enablement_operator_review_packet_readback_ready: readback.is_some_and(|readback| {
                readback.wait_canonical_projection_enablement_operator_review_packet_readback_ready
            }),
            enablement_operator_review_replay_consistency_ready: readback.is_some_and(
                |readback| {
                    readback
                        .wait_canonical_projection_enablement_operator_review_replay_consistency_ready
                },
            ),
            no_live_rehearsal_closeout_readback_ready: readback.is_some_and(|readback| {
                readback.wait_canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready
            }),
            no_live_rehearsal_closeout_replay_consistency_ready: readback.is_some_and(
                |readback| {
                    readback
                        .wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_ready
                },
            ),
            live_blocking_event_count: readback
                .map(|readback| readback.live_blocking_event_count)
                .unwrap_or_default(),
            live_cutover_event_count: readback
                .map(|readback| readback.live_cutover_event_count)
                .unwrap_or_default(),
        },
    ))
}

struct WaitCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput<'a> {
    result_required: bool,
    enablement_audit_chain_closeout_receipt:
        Option<&'a WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReceipt>,
    enablement_audit_chain_closeout_receipt_payload: Option<&'a Value>,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision(
    input: WaitCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyDecision> {
    if !input.result_required {
        return None;
    }

    let enablement_audit_chain_closeout_receipt = input.enablement_audit_chain_closeout_receipt?;
    let enablement_audit_chain_closeout_receipt_payload =
        input.enablement_audit_chain_closeout_receipt_payload?;
    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput {
                source_surface_id: "wait_agent",
                enablement_audit_chain_closeout_receipt,
                enablement_audit_chain_closeout_receipt_payload,
                latest_enablement_audit_chain_closeout_receipt_payload: readback.and_then(|readback| {
                    readback
                        .latest_wait_canonical_projection_enablement_audit_chain_closeout
                        .as_ref()
                }),
                enablement_audit_chain_closeout_events: readback
                    .map(|readback| {
                        readback.wait_canonical_projection_enablement_audit_chain_closeout_events
                    })
                    .unwrap_or_default(),
                enablement_audit_chain_closeout_readback_ready: readback.is_some_and(|readback| {
                    readback
                        .wait_canonical_projection_enablement_audit_chain_closeout_readback_ready
                }),
                prior_enablement_audit_chain_closeout_replay_consistency_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_events
                    })
                    .unwrap_or_default(),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput<'a> {
    result_required: bool,
    enablement_audit_chain_closeout_receipt:
        Option<&'a WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReceipt>,
    enablement_audit_chain_closeout_replay_consistency_decision: Option<
        &'a WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyDecision,
    >,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_activation_precondition_operator_packet(
    input: WaitCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket> {
    if !input.result_required {
        return None;
    }

    let enablement_audit_chain_closeout_receipt = input.enablement_audit_chain_closeout_receipt?;
    let enablement_audit_chain_closeout_replay_consistency_decision =
        input.enablement_audit_chain_closeout_replay_consistency_decision?;
    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_activation_precondition_operator_packet(
            WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput {
                source_surface_id: "wait_agent",
                enablement_audit_chain_closeout_receipt,
                enablement_audit_chain_closeout_replay_consistency_decision,
                enablement_audit_chain_closeout_events: readback
                    .map(|readback| {
                        readback.wait_canonical_projection_enablement_audit_chain_closeout_events
                    })
                    .unwrap_or_default(),
                enablement_audit_chain_closeout_replay_consistency_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_events
                    })
                    .unwrap_or_default(),
                prior_enablement_activation_precondition_operator_packet_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_precondition_operator_packet_events
                    })
                    .unwrap_or_default(),
                enablement_audit_chain_closeout_readback_ready: readback.is_some_and(|readback| {
                    readback
                        .wait_canonical_projection_enablement_audit_chain_closeout_readback_ready
                }),
                enablement_audit_chain_closeout_replay_consistency_ready: readback
                    .is_some_and(|readback| {
                        readback
                            .wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_ready
                    }),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput<'a> {
    result_required: bool,
    activation_precondition_operator_packet:
        Option<&'a WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket>,
    activation_precondition_operator_packet_payload: Option<&'a Value>,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_activation_precondition_replay_consistency_decision(
    input: WaitCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyDecision> {
    if !input.result_required {
        return None;
    }

    let activation_precondition_operator_packet = input.activation_precondition_operator_packet?;
    let activation_precondition_operator_packet_payload =
        input.activation_precondition_operator_packet_payload?;
    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput {
                source_surface_id: "wait_agent",
                activation_precondition_operator_packet,
                activation_precondition_operator_packet_payload,
                latest_activation_precondition_operator_packet_payload: readback.and_then(|readback| {
                    readback
                        .latest_wait_canonical_projection_enablement_activation_precondition_operator_packet
                        .as_ref()
                }),
                activation_precondition_operator_packet_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_precondition_operator_packet_events
                    })
                    .unwrap_or_default(),
                activation_precondition_operator_packet_readback_ready: readback.is_some_and(|readback| {
                    readback
                        .wait_canonical_projection_enablement_activation_precondition_operator_packet_readback_ready
                }),
                prior_activation_precondition_replay_consistency_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_precondition_replay_consistency_events
                    })
                    .unwrap_or_default(),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementActivationNoLiveCloseoutInput<'a> {
    result_required: bool,
    activation_precondition_operator_packet:
        Option<&'a WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket>,
    activation_precondition_replay_consistency_decision: Option<
        &'a WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyDecision,
    >,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_activation_no_live_closeout_receipt(
    input: WaitCanonicalProjectionEnablementActivationNoLiveCloseoutInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReceipt> {
    if !input.result_required {
        return None;
    }

    Some(
        build_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutInput {
                source_surface_id: "wait_agent",
                activation_precondition_operator_packet: input
                    .activation_precondition_operator_packet?,
                activation_precondition_replay_consistency_decision: input
                    .activation_precondition_replay_consistency_decision?,
                activation_precondition_operator_packet_events: input
                    .readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_precondition_operator_packet_events
                    })
                    .unwrap_or_default(),
                activation_precondition_replay_consistency_events: input
                    .readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_precondition_replay_consistency_events
                    })
                    .unwrap_or_default(),
                prior_activation_no_live_closeout_events: input
                    .readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_no_live_closeout_events
                    })
                    .unwrap_or_default(),
                activation_precondition_operator_packet_readback_ready: input
                    .readback
                    .is_some_and(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_precondition_operator_packet_readback_ready
                    }),
                activation_precondition_replay_consistency_ready: input
                    .readback
                    .is_some_and(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_precondition_replay_consistency_ready
                    }),
                live_blocking_event_count: input
                    .readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: input
                    .readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput<'a> {
    result_required: bool,
    activation_no_live_closeout_receipt:
        Option<&'a WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReceipt>,
    activation_no_live_closeout_receipt_payload: Option<&'a Value>,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision(
    input: WaitCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyDecision>
{
    if !input.result_required {
        return None;
    }

    let activation_no_live_closeout_receipt = input.activation_no_live_closeout_receipt?;
    let activation_no_live_closeout_receipt_payload =
        input.activation_no_live_closeout_receipt_payload?;
    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput {
                source_surface_id: "wait_agent",
                activation_no_live_closeout_receipt,
                activation_no_live_closeout_receipt_payload,
                latest_activation_no_live_closeout_receipt_payload: readback.and_then(|readback| {
                    readback
                        .latest_wait_canonical_projection_enablement_activation_no_live_closeout
                        .as_ref()
                }),
                activation_no_live_closeout_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_no_live_closeout_events
                    })
                    .unwrap_or_default(),
                activation_no_live_closeout_readback_ready: readback.is_some_and(|readback| {
                    readback
                        .wait_canonical_projection_enablement_activation_no_live_closeout_readback_ready
                }),
                prior_activation_no_live_closeout_replay_consistency_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_events
                    })
                    .unwrap_or_default(),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementActivationAuditChainCloseoutInput<'a> {
    result_required: bool,
    activation_precondition_operator_packet:
        Option<&'a WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacket>,
    activation_precondition_replay_consistency_decision: Option<
        &'a WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyDecision,
    >,
    activation_no_live_closeout_receipt:
        Option<&'a WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReceipt>,
    activation_no_live_closeout_replay_consistency_decision: Option<
        &'a WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyDecision,
    >,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_activation_audit_chain_closeout_receipt(
    input: WaitCanonicalProjectionEnablementActivationAuditChainCloseoutInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReceipt> {
    if !input.result_required {
        return None;
    }

    let activation_precondition_operator_packet = input.activation_precondition_operator_packet?;
    let activation_precondition_replay_consistency_decision =
        input.activation_precondition_replay_consistency_decision?;
    let activation_no_live_closeout_receipt = input.activation_no_live_closeout_receipt?;
    let activation_no_live_closeout_replay_consistency_decision =
        input.activation_no_live_closeout_replay_consistency_decision?;
    let readback = input.readback;

    Some(build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt(
        WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutInput {
            source_surface_id: "wait_agent",
            activation_precondition_operator_packet,
            activation_precondition_replay_consistency_decision,
            activation_no_live_closeout_receipt,
            activation_no_live_closeout_replay_consistency_decision,
            activation_precondition_operator_packet_events: readback
                .map(|readback| {
                    readback
                        .wait_canonical_projection_enablement_activation_precondition_operator_packet_events
                })
                .unwrap_or_default(),
            activation_precondition_replay_consistency_events: readback
                .map(|readback| {
                    readback
                        .wait_canonical_projection_enablement_activation_precondition_replay_consistency_events
                })
                .unwrap_or_default(),
            activation_no_live_closeout_events: readback
                .map(|readback| {
                    readback
                        .wait_canonical_projection_enablement_activation_no_live_closeout_events
                })
                .unwrap_or_default(),
            activation_no_live_closeout_replay_consistency_events: readback
                .map(|readback| {
                    readback
                        .wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_events
                })
                .unwrap_or_default(),
            prior_activation_audit_chain_closeout_events: readback
                .map(|readback| {
                    readback
                        .wait_canonical_projection_enablement_activation_audit_chain_closeout_events
                })
                .unwrap_or_default(),
            activation_precondition_operator_packet_readback_ready: readback.is_some_and(
                |readback| {
                    readback
                        .wait_canonical_projection_enablement_activation_precondition_operator_packet_readback_ready
                },
            ),
            activation_precondition_replay_consistency_ready: readback.is_some_and(|readback| {
                readback
                    .wait_canonical_projection_enablement_activation_precondition_replay_consistency_ready
            }),
            activation_no_live_closeout_readback_ready: readback.is_some_and(|readback| {
                readback
                    .wait_canonical_projection_enablement_activation_no_live_closeout_readback_ready
            }),
            activation_no_live_closeout_replay_consistency_ready: readback.is_some_and(
                |readback| {
                    readback
                        .wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_ready
                },
            ),
            live_blocking_event_count: readback
                .map(|readback| readback.live_blocking_event_count)
                .unwrap_or_default(),
            live_cutover_event_count: readback
                .map(|readback| readback.live_cutover_event_count)
                .unwrap_or_default(),
        },
    ))
}

struct WaitCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput<'a> {
    result_required: bool,
    activation_audit_chain_closeout_receipt:
        Option<&'a WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReceipt>,
    activation_audit_chain_closeout_receipt_payload: Option<&'a Value>,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision(
    input: WaitCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput<'_>,
) -> Option<
    WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyDecision,
> {
    if !input.result_required {
        return None;
    }

    let activation_audit_chain_closeout_receipt = input.activation_audit_chain_closeout_receipt?;
    let activation_audit_chain_closeout_receipt_payload =
        input.activation_audit_chain_closeout_receipt_payload?;
    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput {
                source_surface_id: "wait_agent",
                activation_audit_chain_closeout_receipt,
                activation_audit_chain_closeout_receipt_payload,
                latest_activation_audit_chain_closeout_receipt_payload: readback.and_then(
                    |readback| {
                        readback
                            .latest_wait_canonical_projection_enablement_activation_audit_chain_closeout
                            .as_ref()
                    },
                ),
                activation_audit_chain_closeout_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_audit_chain_closeout_events
                    })
                    .unwrap_or_default(),
                activation_audit_chain_closeout_readback_ready: readback.is_some_and(|readback| {
                    readback
                        .wait_canonical_projection_enablement_activation_audit_chain_closeout_readback_ready
                }),
                prior_activation_audit_chain_closeout_replay_consistency_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_events
                    })
                    .unwrap_or_default(),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightInput<'a> {
    result_required: bool,
    activation_audit_chain_closeout_receipt:
        Option<&'a WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReceipt>,
    activation_audit_chain_closeout_replay_consistency_decision: Option<
        &'a WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyDecision,
    >,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet(
    input: WaitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightInput<'_>,
) -> Option<WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacket>
{
    if !input.result_required {
        return None;
    }

    let activation_audit_chain_closeout_receipt = input.activation_audit_chain_closeout_receipt?;
    let activation_audit_chain_closeout_replay_consistency_decision =
        input.activation_audit_chain_closeout_replay_consistency_decision?;
    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet(
            WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacketInput {
                source_surface_id: "wait_agent",
                activation_audit_chain_closeout_receipt,
                activation_audit_chain_closeout_replay_consistency_decision,
                activation_audit_chain_closeout_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_audit_chain_closeout_events
                    })
                    .unwrap_or_default(),
                activation_audit_chain_closeout_replay_consistency_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_events
                    })
                    .unwrap_or_default(),
                prior_activation_operator_approval_readiness_preflight_packet_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_events
                    })
                    .unwrap_or_default(),
                activation_audit_chain_closeout_readback_ready: readback.is_some_and(|readback| {
                    readback
                        .wait_canonical_projection_enablement_activation_audit_chain_closeout_readback_ready
                }),
                activation_audit_chain_closeout_replay_consistency_ready: readback.is_some_and(
                    |readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_ready
                    },
                ),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput<'a> {
    result_required: bool,
    activation_operator_approval_readiness_preflight_packet:
        Option<&'a WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacket>,
    activation_operator_approval_readiness_preflight_packet_payload: Option<&'a Value>,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision(
    input: WaitCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput<
        '_,
    >,
) -> Option<WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyDecision>{
    if !input.result_required {
        return None;
    }

    let (
        Some(activation_operator_approval_readiness_preflight_packet),
        Some(activation_operator_approval_readiness_preflight_packet_payload),
    ) = (
        input.activation_operator_approval_readiness_preflight_packet,
        input.activation_operator_approval_readiness_preflight_packet_payload,
    )
    else {
        return None;
    };
    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput {
                source_surface_id: "wait_agent",
                activation_operator_approval_readiness_preflight_packet,
                activation_operator_approval_readiness_preflight_packet_payload,
                latest_activation_operator_approval_readiness_preflight_packet_payload:
                    readback.and_then(|readback| {
                        readback
                            .latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet
                            .as_ref()
                    }),
                activation_operator_approval_readiness_preflight_packet_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_events
                    })
                    .unwrap_or_default(),
                activation_operator_approval_readiness_preflight_packet_readback_ready: readback
                    .is_some_and(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready
                    }),
                prior_activation_operator_approval_readiness_preflight_replay_consistency_events:
                    readback
                        .map(|readback| {
                            readback
                                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_events
                        })
                        .unwrap_or_default(),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutInput<'a> {
    result_required: bool,
    activation_operator_approval_readiness_preflight_packet:
        Option<&'a WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacket>,
    activation_operator_approval_readiness_preflight_replay_consistency_decision: Option<
        &'a WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyDecision,
    >,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet(
    input: WaitCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutInput<'_>,
) -> Option<
    WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacket,
> {
    if !input.result_required {
        return None;
    }

    let activation_operator_approval_readiness_preflight_packet =
        input.activation_operator_approval_readiness_preflight_packet?;
    let activation_operator_approval_readiness_preflight_replay_consistency_decision =
        input.activation_operator_approval_readiness_preflight_replay_consistency_decision?;
    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet(
            WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacketInput {
                source_surface_id: "wait_agent",
                activation_operator_approval_readiness_preflight_packet,
                activation_operator_approval_readiness_preflight_replay_consistency_decision,
                activation_operator_approval_readiness_preflight_packet_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_events
                    })
                    .unwrap_or_default(),
                activation_operator_approval_readiness_preflight_replay_consistency_events:
                    readback
                        .map(|readback| {
                            readback
                                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_events
                        })
                        .unwrap_or_default(),
                prior_activation_approval_review_side_effect_lock_closeout_packet_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_events
                    })
                    .unwrap_or_default(),
                activation_operator_approval_readiness_preflight_packet_readback_ready: readback
                    .is_some_and(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready
                    }),
                activation_operator_approval_readiness_preflight_replay_consistency_ready:
                    readback.is_some_and(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_ready
                    }),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

struct WaitCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyInput<
    'a,
> {
    result_required: bool,
    activation_approval_review_side_effect_lock_closeout_packet:
        Option<&'a WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutPacket>,
    activation_approval_review_side_effect_lock_closeout_packet_payload: Option<&'a Value>,
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
}

fn build_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision(
    input: WaitCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyInput<'_>,
) -> Option<
    WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyDecision,
>{
    if !input.result_required {
        return None;
    }

    let activation_approval_review_side_effect_lock_closeout_packet =
        input.activation_approval_review_side_effect_lock_closeout_packet?;
    let activation_approval_review_side_effect_lock_closeout_packet_payload =
        input.activation_approval_review_side_effect_lock_closeout_packet_payload?;
    let readback = input.readback;

    Some(
        build_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionEnablementActivationApprovalReviewSideEffectLockCloseoutReplayConsistencyInput {
                source_surface_id: "wait_agent",
                activation_approval_review_side_effect_lock_closeout_packet,
                activation_approval_review_side_effect_lock_closeout_packet_payload,
                latest_activation_approval_review_side_effect_lock_closeout_packet_payload: readback
                    .and_then(|readback| {
                        readback
                            .latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet
                            .as_ref()
                    }),
                activation_approval_review_side_effect_lock_closeout_packet_events: readback
                    .map(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_events
                    })
                    .unwrap_or_default(),
                activation_approval_review_side_effect_lock_closeout_packet_readback_ready:
                    readback.is_some_and(|readback| {
                        readback
                            .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_readback_ready
                    }),
                prior_activation_approval_review_side_effect_lock_closeout_replay_consistency_events:
                    readback
                        .map(|readback| {
                            readback
                                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_events
                        })
                        .unwrap_or_default(),
                live_blocking_event_count: readback
                    .map(|readback| readback.live_blocking_event_count)
                    .unwrap_or_default(),
                live_cutover_event_count: readback
                    .map(|readback| readback.live_cutover_event_count)
                    .unwrap_or_default(),
            },
        ),
    )
}

async fn record_wait_work_graph_shadow_event(input: WaitWorkGraphShadowRecordInput<'_>) -> bool {
    let (Some(state_db), Some(status), Some(payload_json)) =
        (input.state_db, input.status, input.payload_json)
    else {
        return false;
    };
    state_db
        .record_inter_agent_wait_work_graph_shadow_event(
            codex_state::InterAgentWaitWorkGraphShadowEventParams {
                thread_id: input.thread_id,
                barrier_id: input.barrier_id,
                task_id: input.task_id,
                task_name: input.task_name,
                event_type: input.event_type,
                status,
                payload_json,
                trace_id: input.trace_id,
            },
        )
        .await
        .is_ok()
}

async fn load_wait_task_result_readback(
    state_db: Option<crate::StateDbHandle>,
    thread_id: codex_protocol::ThreadId,
    barrier_id: &str,
) -> Option<codex_state::InterAgentWaitTaskResultReadback> {
    let state_db = state_db?;
    state_db
        .get_inter_agent_wait_task_result_readback(thread_id, barrier_id)
        .await
        .ok()
}

struct WaitTaskResultReadbackSummaryInput<'a> {
    readback: Option<&'a codex_state::InterAgentWaitTaskResultReadback>,
    task_result_delivery_shadow_event_recorded: bool,
    parent_reducer_shadow_receipt_event_recorded: bool,
    task_result_replay_consistency_event_recorded: bool,
    wait_surface_audit_packet_event_recorded: bool,
    wait_surface_audit_replay_consistency_event_recorded: bool,
    wait_canonical_projection_receipt_event_recorded: bool,
    wait_canonical_projection_replay_consistency_event_recorded: bool,
    wait_canonical_projection_closeout_receipt_event_recorded: bool,
    wait_canonical_projection_closeout_replay_consistency_event_recorded: bool,
    wait_canonical_projection_audit_chain_closeout_receipt_event_recorded: bool,
    wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded: bool,
    wait_canonical_projection_enablement_operator_review_packet_event_recorded: bool,
    wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded: bool,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_audit_chain_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded: bool,
    wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded:
        bool,
    wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded:
        bool,
}

fn summarize_wait_task_result_readback(
    input: WaitTaskResultReadbackSummaryInput<'_>,
) -> Option<WorkGraphWaitTaskResultReadbackSummary> {
    let readback = input.readback?;
    Some(WorkGraphWaitTaskResultReadbackSummary {
        thread_id: readback.thread_id.to_string(),
        barrier_id: readback.barrier_id.clone(),
        task_result_delivery_shadow_events: readback.task_result_delivery_shadow_events,
        parent_reducer_shadow_receipt_events: readback.parent_reducer_shadow_receipt_events,
        task_result_replay_consistency_events: readback.task_result_replay_consistency_events,
        wait_surface_audit_packet_events: readback.wait_surface_audit_packet_events,
        wait_surface_audit_replay_consistency_events: readback
            .wait_surface_audit_replay_consistency_events,
        wait_canonical_projection_receipt_events: readback.wait_canonical_projection_receipt_events,
        wait_canonical_projection_replay_consistency_events: readback
            .wait_canonical_projection_replay_consistency_events,
        wait_canonical_projection_closeout_receipt_events: readback
            .wait_canonical_projection_closeout_receipt_events,
        wait_canonical_projection_closeout_replay_consistency_events: readback
            .wait_canonical_projection_closeout_replay_consistency_events,
        wait_canonical_projection_audit_chain_closeout_receipt_events: readback
            .wait_canonical_projection_audit_chain_closeout_receipt_events,
        wait_canonical_projection_audit_chain_closeout_replay_consistency_events: readback
            .wait_canonical_projection_audit_chain_closeout_replay_consistency_events,
        wait_canonical_projection_enablement_operator_review_packet_events: readback
            .wait_canonical_projection_enablement_operator_review_packet_events,
        wait_canonical_projection_enablement_operator_review_replay_consistency_events: readback
            .wait_canonical_projection_enablement_operator_review_replay_consistency_events,
        wait_canonical_projection_enablement_no_live_rehearsal_closeout_events: readback
            .wait_canonical_projection_enablement_no_live_rehearsal_closeout_events,
        wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_events:
            readback
                .wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_events,
        wait_canonical_projection_enablement_audit_chain_closeout_events: readback
            .wait_canonical_projection_enablement_audit_chain_closeout_events,
        wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_events:
            readback
                .wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_events,
        wait_canonical_projection_enablement_activation_precondition_operator_packet_events:
            readback
                .wait_canonical_projection_enablement_activation_precondition_operator_packet_events,
        wait_canonical_projection_enablement_activation_precondition_replay_consistency_events:
            readback
                .wait_canonical_projection_enablement_activation_precondition_replay_consistency_events,
        wait_canonical_projection_enablement_activation_no_live_closeout_events: readback
            .wait_canonical_projection_enablement_activation_no_live_closeout_events,
        wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_events:
            readback
                .wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_events,
        wait_canonical_projection_enablement_activation_audit_chain_closeout_events: readback
            .wait_canonical_projection_enablement_activation_audit_chain_closeout_events,
        wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_events:
            readback
                .wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_events,
        wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_events:
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_events,
        wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_events:
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_events,
        wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_events:
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_events,
        wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_events:
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_events,
        latest_task_result_delivery_decision: readback.latest_task_result_delivery_decision.clone(),
        latest_parent_reducer_decision: readback.latest_parent_reducer_decision.clone(),
        latest_task_result_replay_consistency_decision: readback
            .latest_task_result_replay_consistency_decision
            .clone(),
        latest_wait_surface_audit_decision: readback.latest_wait_surface_audit_decision.clone(),
        latest_wait_surface_audit_replay_consistency_decision: readback
            .latest_wait_surface_audit_replay_consistency_decision
            .clone(),
        latest_wait_canonical_projection_decision: readback
            .latest_wait_canonical_projection_decision
            .clone(),
        latest_wait_canonical_projection_replay_consistency_decision: readback
            .latest_wait_canonical_projection_replay_consistency_decision
            .clone(),
        latest_wait_canonical_projection_closeout_decision: readback
            .latest_wait_canonical_projection_closeout_decision
            .clone(),
        latest_wait_canonical_projection_closeout_replay_consistency_decision: readback
            .latest_wait_canonical_projection_closeout_replay_consistency_decision
            .clone(),
        latest_wait_canonical_projection_audit_chain_closeout_decision: readback
            .latest_wait_canonical_projection_audit_chain_closeout_decision
            .clone(),
        latest_wait_canonical_projection_audit_chain_closeout_replay_consistency_decision: readback
            .latest_wait_canonical_projection_audit_chain_closeout_replay_consistency_decision
            .clone(),
        latest_wait_canonical_projection_enablement_operator_review_decision: readback
            .latest_wait_canonical_projection_enablement_operator_review_decision
            .clone(),
        latest_wait_canonical_projection_enablement_operator_review_replay_consistency_decision:
            readback
                .latest_wait_canonical_projection_enablement_operator_review_replay_consistency_decision
                .clone(),
        latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_decision: readback
            .latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_decision
            .clone(),
        latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision:
            readback
                .latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision
                .clone(),
        latest_wait_canonical_projection_enablement_audit_chain_closeout_decision: readback
            .latest_wait_canonical_projection_enablement_audit_chain_closeout_decision
            .clone(),
        latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision:
            readback
                .latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision
                .clone(),
        latest_wait_canonical_projection_enablement_activation_precondition_operator_decision:
            readback
                .latest_wait_canonical_projection_enablement_activation_precondition_operator_decision
                .clone(),
        latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency_decision:
            readback
                .latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency_decision
                .clone(),
        latest_wait_canonical_projection_enablement_activation_no_live_closeout_decision: readback
            .latest_wait_canonical_projection_enablement_activation_no_live_closeout_decision
            .clone(),
        latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision:
            readback
                .latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision
                .clone(),
        latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_decision:
            readback
                .latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_decision
                .clone(),
        latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision:
            readback
                .latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision
                .clone(),
        latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_decision:
            readback
                .latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_decision
                .clone(),
        latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision:
            readback
                .latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision
                .clone(),
        latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_decision:
            readback
                .latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_decision
                .clone(),
        latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision:
            readback
                .latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision
                .clone(),
        task_result_delivery_readback_ready: readback.task_result_delivery_readback_ready,
        parent_reducer_readback_ready: readback.parent_reducer_readback_ready,
        replay_consistency_ready: readback.replay_consistency_ready,
        wait_surface_audit_packet_readback_ready: readback.wait_surface_audit_packet_readback_ready,
        wait_surface_audit_replay_consistency_ready: readback
            .wait_surface_audit_replay_consistency_ready,
        wait_canonical_projection_receipt_readback_ready: readback
            .wait_canonical_projection_receipt_readback_ready,
        wait_canonical_projection_replay_consistency_ready: readback
            .wait_canonical_projection_replay_consistency_ready,
        wait_canonical_projection_closeout_receipt_readback_ready: readback
            .wait_canonical_projection_closeout_receipt_readback_ready,
        wait_canonical_projection_closeout_replay_consistency_ready: readback
            .wait_canonical_projection_closeout_replay_consistency_ready,
        wait_canonical_projection_audit_chain_closeout_receipt_readback_ready: readback
            .wait_canonical_projection_audit_chain_closeout_receipt_readback_ready,
        wait_canonical_projection_audit_chain_closeout_replay_consistency_ready: readback
            .wait_canonical_projection_audit_chain_closeout_replay_consistency_ready,
        wait_canonical_projection_enablement_operator_review_packet_readback_ready: readback
            .wait_canonical_projection_enablement_operator_review_packet_readback_ready,
        wait_canonical_projection_enablement_operator_review_replay_consistency_ready: readback
            .wait_canonical_projection_enablement_operator_review_replay_consistency_ready,
        wait_canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready: readback
            .wait_canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready,
        wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_ready:
            readback
                .wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_ready,
        wait_canonical_projection_enablement_audit_chain_closeout_readback_ready: readback
            .wait_canonical_projection_enablement_audit_chain_closeout_readback_ready,
        wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_ready:
            readback
                .wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_ready,
        wait_canonical_projection_enablement_activation_precondition_operator_packet_readback_ready:
            readback
                .wait_canonical_projection_enablement_activation_precondition_operator_packet_readback_ready,
        wait_canonical_projection_enablement_activation_precondition_replay_consistency_ready:
            readback
                .wait_canonical_projection_enablement_activation_precondition_replay_consistency_ready,
        wait_canonical_projection_enablement_activation_no_live_closeout_readback_ready: readback
            .wait_canonical_projection_enablement_activation_no_live_closeout_readback_ready,
        wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_ready:
            readback
                .wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_ready,
        wait_canonical_projection_enablement_activation_audit_chain_closeout_readback_ready:
            readback
                .wait_canonical_projection_enablement_activation_audit_chain_closeout_readback_ready,
        wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_ready:
            readback
                .wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_ready,
        wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready:
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready,
        wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_ready:
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_ready,
        wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_readback_ready:
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_readback_ready,
        wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_ready:
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_ready,
        task_result_delivery_ready: readback.task_result_delivery_ready,
        parent_reducer_receipt_ready: readback.parent_reducer_receipt_ready,
        replay_consistent: readback.replay_consistent,
        wait_surface_audit_packet_ready: readback.wait_surface_audit_packet_ready,
        wait_surface_audit_replay_consistent: readback.wait_surface_audit_replay_consistent,
        wait_canonical_projection_receipt_ready: readback.wait_canonical_projection_receipt_ready,
        wait_canonical_projection_replay_consistent: readback
            .wait_canonical_projection_replay_consistent,
        wait_canonical_projection_closeout_receipt_ready: readback
            .wait_canonical_projection_closeout_receipt_ready,
        wait_canonical_projection_closeout_replay_consistent: readback
            .wait_canonical_projection_closeout_replay_consistent,
        wait_canonical_projection_audit_chain_closeout_receipt_ready: readback
            .wait_canonical_projection_audit_chain_closeout_receipt_ready,
        wait_canonical_projection_audit_chain_closeout_replay_consistent: readback
            .wait_canonical_projection_audit_chain_closeout_replay_consistent,
        wait_canonical_projection_enablement_operator_review_ready: readback
            .wait_canonical_projection_enablement_operator_review_ready,
        wait_canonical_projection_enablement_operator_review_replay_consistent: readback
            .wait_canonical_projection_enablement_operator_review_replay_consistent,
        wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready: readback
            .wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready,
        wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent:
            readback
                .wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent,
        wait_canonical_projection_enablement_audit_chain_closeout_ready: readback
            .wait_canonical_projection_enablement_audit_chain_closeout_ready,
        wait_canonical_projection_enablement_audit_chain_closeout_replay_consistent: readback
            .wait_canonical_projection_enablement_audit_chain_closeout_replay_consistent,
        wait_canonical_projection_enablement_activation_precondition_ready: readback
            .wait_canonical_projection_enablement_activation_precondition_ready,
        wait_canonical_projection_enablement_activation_precondition_replay_consistent: readback
            .wait_canonical_projection_enablement_activation_precondition_replay_consistent,
        wait_canonical_projection_enablement_activation_no_live_closeout_ready: readback
            .wait_canonical_projection_enablement_activation_no_live_closeout_ready,
        wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistent:
            readback
                .wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistent,
        wait_canonical_projection_enablement_activation_audit_chain_closeout_ready: readback
            .wait_canonical_projection_enablement_activation_audit_chain_closeout_ready,
        wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent:
            readback
                .wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent,
        wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready:
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready,
        wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent:
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent,
        wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready:
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready,
        wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent:
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent,
        no_live_guardrails_ready: readback.no_live_guardrails_ready,
        readback_ready: readback.readback_ready,
        direct_wait_task_result_ready: readback.direct_wait_task_result_ready,
        direct_wait_surface_audit_ready: readback.direct_wait_surface_audit_ready,
        direct_wait_canonical_projection_ready: readback.direct_wait_canonical_projection_ready,
        direct_wait_canonical_projection_closeout_ready: readback
            .direct_wait_canonical_projection_closeout_ready,
        direct_wait_canonical_projection_audit_chain_closeout_ready: readback
            .direct_wait_canonical_projection_audit_chain_closeout_ready,
        direct_wait_canonical_projection_audit_chain_closeout_replay_ready: readback
            .direct_wait_canonical_projection_audit_chain_closeout_replay_ready,
        direct_wait_canonical_projection_enablement_operator_review_ready: readback
            .direct_wait_canonical_projection_enablement_operator_review_ready,
        direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready: readback
            .direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready,
        direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_ready:
            readback
                .direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_ready,
        direct_wait_canonical_projection_enablement_audit_chain_closeout_ready: readback
            .direct_wait_canonical_projection_enablement_audit_chain_closeout_ready,
        direct_wait_canonical_projection_enablement_audit_chain_closeout_replay_ready: readback
            .direct_wait_canonical_projection_enablement_audit_chain_closeout_replay_ready,
        direct_wait_canonical_projection_enablement_activation_precondition_ready: readback
            .direct_wait_canonical_projection_enablement_activation_precondition_ready,
        direct_wait_canonical_projection_enablement_activation_precondition_replay_ready: readback
            .direct_wait_canonical_projection_enablement_activation_precondition_replay_ready,
        direct_wait_canonical_projection_enablement_activation_no_live_closeout_ready: readback
            .direct_wait_canonical_projection_enablement_activation_no_live_closeout_ready,
        direct_wait_canonical_projection_enablement_activation_no_live_closeout_replay_ready:
            readback
                .direct_wait_canonical_projection_enablement_activation_no_live_closeout_replay_ready,
        direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_ready: readback
            .direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_ready,
        direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_ready:
            readback
                .direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_ready,
        direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready:
            readback
                .direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready,
        direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_ready:
            readback
                .direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_ready,
        direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready:
            readback
                .direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready,
        direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_ready:
            readback
                .direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_ready,
        task_result_delivery_shadow_event_recorded: input
            .task_result_delivery_shadow_event_recorded,
        parent_reducer_shadow_receipt_event_recorded: input
            .parent_reducer_shadow_receipt_event_recorded,
        task_result_replay_consistency_event_recorded: input
            .task_result_replay_consistency_event_recorded,
        wait_surface_audit_packet_event_recorded: input.wait_surface_audit_packet_event_recorded,
        wait_surface_audit_replay_consistency_event_recorded: input
            .wait_surface_audit_replay_consistency_event_recorded,
        wait_canonical_projection_receipt_event_recorded: input
            .wait_canonical_projection_receipt_event_recorded,
        wait_canonical_projection_replay_consistency_event_recorded: input
            .wait_canonical_projection_replay_consistency_event_recorded,
        wait_canonical_projection_closeout_receipt_event_recorded: input
            .wait_canonical_projection_closeout_receipt_event_recorded,
        wait_canonical_projection_closeout_replay_consistency_event_recorded: input
            .wait_canonical_projection_closeout_replay_consistency_event_recorded,
        wait_canonical_projection_audit_chain_closeout_receipt_event_recorded: input
            .wait_canonical_projection_audit_chain_closeout_receipt_event_recorded,
        wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded: input
            .wait_canonical_projection_audit_chain_closeout_replay_consistency_event_recorded,
        wait_canonical_projection_enablement_operator_review_packet_event_recorded: input
            .wait_canonical_projection_enablement_operator_review_packet_event_recorded,
        wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded:
            input
                .wait_canonical_projection_enablement_operator_review_replay_consistency_event_recorded,
        wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded:
            input.wait_canonical_projection_enablement_no_live_rehearsal_closeout_event_recorded,
        wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded:
            input
                .wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_event_recorded,
        wait_canonical_projection_enablement_audit_chain_closeout_event_recorded: input
            .wait_canonical_projection_enablement_audit_chain_closeout_event_recorded,
        wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded:
            input
                .wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_event_recorded,
        wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded:
            input
                .wait_canonical_projection_enablement_activation_precondition_operator_packet_event_recorded,
        wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded:
            input
                .wait_canonical_projection_enablement_activation_precondition_replay_consistency_event_recorded,
        wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded:
            input.wait_canonical_projection_enablement_activation_no_live_closeout_event_recorded,
        wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded:
            input
                .wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_event_recorded,
        wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded:
            input
                .wait_canonical_projection_enablement_activation_audit_chain_closeout_event_recorded,
        wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded:
            input
                .wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_event_recorded,
        wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded:
            input
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_event_recorded,
        wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded:
            input
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_event_recorded,
        wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded:
            input
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_event_recorded,
        wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded:
            input
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_event_recorded,
        live_blocking_event_count: readback.live_blocking_event_count,
        live_cutover_event_count: readback.live_cutover_event_count,
    })
}

fn build_wait_surface_audit_packet(
    result_required: bool,
    thread_id: codex_protocol::ThreadId,
    barrier_id: &str,
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
    operator_row: Option<&WorkGraphOperatorMatrixRow>,
) -> Option<WorkGraphWaitSurfaceAuditPacketSummary> {
    if !result_required {
        return None;
    }

    let audit_chain = build_wait_task_result_audit_chain_summary(readback);
    let operator_matrix_rows = operator_row.cloned().into_iter().collect::<Vec<_>>();
    let operator_matrix_row_count = operator_matrix_rows.len();
    let operator_matrix_ready_row_count = operator_matrix_rows
        .iter()
        .filter(|row| row.canonical_promotion_ready)
        .count();
    let audit_packet_ready = audit_chain.chain_ready;
    let mut audit_blockers = Vec::new();
    if !audit_chain.chain_readback_ready {
        audit_blockers.push("wait_task_result_audit_chain_readback_not_ready".to_string());
    }
    if !audit_chain.chain_replay_consistent {
        audit_blockers.push("wait_task_result_audit_chain_replay_inconsistent".to_string());
    }
    if !audit_chain.no_live_guardrails_ready {
        audit_blockers.push("wait_task_result_live_guardrail_event_present".to_string());
    }
    let decision = if audit_packet_ready {
        "wait_task_result_surface_audit_recorded_shadow_no_live_cutover"
    } else {
        "wait_task_result_surface_audit_blocked_shadow_no_live_cutover"
    };

    Some(WorkGraphWaitSurfaceAuditPacketSummary {
        decision,
        audit_stage: "direct_wait_surface_audit_shadow_only",
        source_surface_id: "wait_agent",
        thread_id: thread_id.to_string(),
        barrier_id: barrier_id.to_string(),
        audit_chain_segment_count: audit_chain.segment_count,
        audit_chain_ready_segment_count: audit_chain.ready_segment_count,
        audit_chain_missing_segment_ids: audit_chain.missing_segment_ids.clone(),
        audit_chain_inconsistent_segment_ids: audit_chain.inconsistent_segment_ids.clone(),
        audit_chain_ready: audit_chain.chain_ready,
        audit_chain,
        operator_matrix_row_count,
        operator_matrix_ready_row_count,
        operator_matrix_blocked_row_count: operator_matrix_row_count
            .saturating_sub(operator_matrix_ready_row_count),
        operator_matrix_rows,
        audit_packet_ready,
        audit_blockers,
        recommended_next_action: "feed direct wait delivery, reducer, and replay readback into the global WorkGraph surface audit bundle",
        feature_flag_enabled: false,
        canary_stage: "off",
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    })
}

fn build_wait_task_result_audit_chain_summary(
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
) -> WorkGraphAuditChainSummary {
    let no_live_guardrails_ready = readback.is_none_or(|readback| {
        readback.live_blocking_event_count == 0 && readback.live_cutover_event_count == 0
    });
    let segments = vec![
        wait_task_result_audit_chain_segment(WaitTaskResultAuditChainSegmentInput {
            segment_id: "wait_task_result_delivery_shadow",
            event_type: "wait_task_result_delivery_shadow",
            event_count: readback
                .map(|readback| readback.task_result_delivery_shadow_events)
                .unwrap_or_default(),
            latest_payload: readback
                .and_then(|readback| readback.latest_task_result_delivery_shadow.as_ref()),
            latest_decision: readback
                .map(|readback| readback.latest_task_result_delivery_decision.as_str())
                .unwrap_or("missing"),
            readback_ready: readback
                .is_some_and(|readback| readback.task_result_delivery_readback_ready),
            replay_consistent: true,
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
        wait_task_result_audit_chain_segment(WaitTaskResultAuditChainSegmentInput {
            segment_id: "wait_parent_reducer_shadow_receipt",
            event_type: "wait_parent_reducer_shadow_receipt",
            event_count: readback
                .map(|readback| readback.parent_reducer_shadow_receipt_events)
                .unwrap_or_default(),
            latest_payload: readback
                .and_then(|readback| readback.latest_parent_reducer_shadow_receipt.as_ref()),
            latest_decision: readback
                .map(|readback| readback.latest_parent_reducer_decision.as_str())
                .unwrap_or("missing"),
            readback_ready: readback.is_some_and(|readback| readback.parent_reducer_readback_ready),
            replay_consistent: true,
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
        wait_task_result_audit_chain_segment(WaitTaskResultAuditChainSegmentInput {
            segment_id: "wait_task_result_replay_consistency",
            event_type: "wait_task_result_replay_consistency",
            event_count: readback
                .map(|readback| readback.task_result_replay_consistency_events)
                .unwrap_or_default(),
            latest_payload: readback
                .and_then(|readback| readback.latest_task_result_replay_consistency.as_ref()),
            latest_decision: readback
                .map(|readback| {
                    readback
                        .latest_task_result_replay_consistency_decision
                        .as_str()
                })
                .unwrap_or("missing"),
            readback_ready: readback.is_some_and(|readback| readback.replay_consistency_ready),
            replay_consistent: readback.is_some_and(|readback| readback.replay_consistent),
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
    ];
    let ready_segment_count = segments.iter().filter(|segment| segment.ready).count();
    let missing_segment_ids = segments
        .iter()
        .filter(|segment| !segment.readback_ready)
        .map(|segment| segment.segment_id.clone())
        .collect::<Vec<_>>();
    let inconsistent_segment_ids = segments
        .iter()
        .filter(|segment| segment.readback_ready && !segment.replay_consistent)
        .map(|segment| segment.segment_id.clone())
        .collect::<Vec<_>>();
    let chain_readback_ready = segments.iter().all(|segment| segment.readback_ready);
    let chain_replay_consistent = segments.iter().all(|segment| segment.replay_consistent);
    let chain_ready = chain_readback_ready && chain_replay_consistent && no_live_guardrails_ready;

    WorkGraphAuditChainSummary {
        segment_count: segments.len(),
        ready_segment_count,
        missing_segment_ids,
        inconsistent_segment_ids,
        chain_readback_ready,
        chain_replay_consistent,
        no_live_guardrails_ready,
        chain_ready,
        segments,
    }
}

struct WaitTaskResultAuditChainSegmentInput<'a> {
    segment_id: &'a str,
    event_type: &'a str,
    event_count: usize,
    latest_payload: Option<&'a Value>,
    latest_decision: &'a str,
    readback_ready: bool,
    replay_consistent: bool,
    no_live_guardrail_ready: bool,
}

fn wait_task_result_audit_chain_segment(
    input: WaitTaskResultAuditChainSegmentInput<'_>,
) -> WorkGraphAuditChainSegment {
    WorkGraphAuditChainSegment {
        segment_id: input.segment_id.to_string(),
        event_type: input.event_type.to_string(),
        event_count: input.event_count,
        latest_payload_present: input.latest_payload.is_some(),
        latest_decision: input.latest_decision.to_string(),
        readback_ready: input.readback_ready,
        replay_consistent: input.replay_consistent,
        no_live_guardrail_ready: input.no_live_guardrail_ready,
        ready: input.readback_ready && input.replay_consistent && input.no_live_guardrail_ready,
    }
}

fn build_wait_operator_matrix_row(
    result_required: bool,
    readback: Option<&WorkGraphWaitTaskResultReadbackSummary>,
) -> Option<WorkGraphOperatorMatrixRow> {
    if !result_required {
        return None;
    }

    let durable_fact_source_present = readback.is_some_and(|readback| readback.readback_ready);
    let no_live_guardrail_ready =
        readback.is_some_and(|readback| readback.no_live_guardrails_ready);
    let row_auditable = durable_fact_source_present && no_live_guardrail_ready;
    let result_contract_ready =
        readback.is_some_and(|readback| readback.task_result_delivery_ready);
    let verifier_reducer_ready =
        readback.is_some_and(|readback| readback.parent_reducer_receipt_ready);
    let task_result_replay_consistent = readback
        .is_some_and(|readback| readback.replay_consistency_ready && readback.replay_consistent);
    let surface_audit_replay_consistent = readback.is_some_and(|readback| {
        readback.wait_surface_audit_packet_readback_ready
            && readback.wait_surface_audit_packet_ready
            && readback.wait_surface_audit_replay_consistency_ready
            && readback.wait_surface_audit_replay_consistent
    });
    let replay_consistent = task_result_replay_consistent && surface_audit_replay_consistent;
    let (readiness_status, next_blocker, next_action) = if !row_auditable {
        (
            "blocked_wait_task_result_readback_not_ready",
            "wait_task_result_readback_not_ready",
            "persist direct wait delivery and parent reducer shadow evidence",
        )
    } else if !result_contract_ready {
        (
            "blocked_wait_task_result_delivery_not_ready",
            "wait_task_result_delivery_not_ready",
            "produce a TaskResultEnvelope before parent reducer promotion",
        )
    } else if !verifier_reducer_ready {
        (
            "blocked_parent_reducer_receipt_not_ready",
            "parent_reducer_receipt_not_ready",
            "record parent reducer shadow receipt from TaskResultEnvelope evidence",
        )
    } else if !task_result_replay_consistent {
        (
            "blocked_wait_task_result_replay_consistency_not_ready",
            "wait_task_result_replay_consistency_not_ready",
            "record direct wait replay/readback consistency before canonical WorkGraph write",
        )
    } else if !surface_audit_replay_consistent {
        (
            "blocked_wait_surface_audit_replay_consistency_not_ready",
            "wait_surface_audit_replay_consistency_not_ready",
            "record direct wait surface-audit replay/readback consistency before canonical WorkGraph write",
        )
    } else {
        (
            "blocked_canonical_work_graph_write_disabled",
            "canonical_work_graph_write_disabled",
            "project direct wait delivery and reducer readback into canonical WorkGraph edges",
        )
    };
    let mut missing_task_result_contract_parts = Vec::new();
    if !result_contract_ready {
        missing_task_result_contract_parts.push("task_result_delivery_shadow".to_string());
    }
    if !verifier_reducer_ready {
        missing_task_result_contract_parts.push("parent_reducer_shadow_receipt".to_string());
    }
    let task_result_contract_plan_ready = result_contract_ready && verifier_reducer_ready;
    let task_result_contract_plan_decision = if task_result_contract_plan_ready {
        "task_result_delivery_readback_ready_shadow_no_live_cutover"
    } else {
        "task_result_delivery_readback_blocked_shadow_no_live_cutover"
    };

    Some(WorkGraphOperatorMatrixRow {
        source_surface_id: "wait_agent".to_string(),
        family: "subagent_lifecycle",
        owner_lane: "subagent_lifecycle",
        observed_this_run: true,
        durable_fact_source_present,
        canonical_work_graph_write_enabled: false,
        row_auditable,
        result_contract_ready,
        verifier_reducer_ready,
        promotion_ready: false,
        replay_consistent,
        no_live_guardrail_ready,
        canonical_promotion_ready: false,
        readiness_status,
        next_blocker,
        task_result_contract_plan_decision: Some(task_result_contract_plan_decision.to_string()),
        task_result_contract_plan_ready: Some(task_result_contract_plan_ready),
        task_result_contract_id: Some("subagent_task_result_contract_v1".to_string()),
        terminal_delivery_surface: Some("wait_agent(result_required=true)".to_string()),
        missing_task_result_contract_parts,
        task_result_contract_next_action: Some(next_action.to_string()),
        task_result_contract_next_action_count: Some(1),
        next_action,
    })
}

impl ToolOutput for WaitAgentToolOutput {
    fn log_preview(&self) -> String {
        tool_output_json_text(self, "wait_agent")
    }

    fn success_for_logging(&self) -> bool {
        true
    }

    fn to_response_item(&self, call_id: &str, payload: &ToolPayload) -> ResponseInputItem {
        tool_output_response_item(call_id, payload, self, /*success*/ None, "wait_agent")
    }

    fn code_mode_result(&self, _payload: &ToolPayload) -> JsonValue {
        tool_output_code_mode_result(self, "wait_agent")
    }
}

async fn record_wait_barrier_event(
    state_db: Option<crate::StateDbHandle>,
    params: codex_state::InterAgentMailboxBarrierEventParams<'_>,
) -> bool {
    let Some(state_db) = state_db else {
        return false;
    };
    state_db
        .record_inter_agent_wait_barrier_event(params)
        .await
        .is_ok()
}

struct WaitTaskTarget {
    thread_id: codex_protocol::ThreadId,
    agent_ref: CollabAgentRef,
    status_rx: tokio::sync::watch::Receiver<AgentStatus>,
}

fn build_wait_lifecycle_role_manifest_shadow_decision(
    turn: &TurnContext,
    receiver_agent_role: Option<&String>,
) -> WorkGraphRoleManifestShadowDecision {
    let requested_role = receiver_agent_role
        .map(String::as_str)
        .map(str::trim)
        .filter(|role| !role.is_empty());
    let configured_role = requested_role.and_then(|role| turn.config.agent_roles.get(role));
    let role_declared = requested_role.is_none() || configured_role.is_some();
    let role_description_present = requested_role.is_none()
        || configured_role
            .and_then(|role| role.description.as_deref())
            .is_some_and(|description| !description.trim().is_empty());

    build_agent_card_manifest_shadow_decision(
        subagent_lifecycle_agent_card_manifest("wait_agent"),
        WorkGraphAgentCardManifestObservation {
            role_name: requested_role.map(str::to_string),
            role_declared,
            role_description_present,
            configured_manifest_source: configured_agent_role_manifest_source(
                requested_role,
                configured_role.is_some(),
                configured_role.is_some_and(|role| role.config_file.is_some()),
                configured_role.and_then(|role| role.agent_card_manifest_source.as_deref()),
            ),
            configured_manifest_version: configured_role
                .and_then(|role| role.agent_card_manifest_version.clone()),
            configured_manifest_overlay: configured_role
                .and_then(|role| role.agent_card_manifest.clone()),
            budget_present: turn
                .config
                .agent_max_threads
                .is_none_or(|max_threads| max_threads > 0),
            output_contract_present: None,
            result_contract_present: None,
            verifier_present: None,
            reducer_present: None,
            attempted_tool: Some("wait_agent"),
            observed_lane: Some("subagent_lifecycle"),
        },
    )
}

async fn resolve_wait_task_target(
    session: &Arc<Session>,
    turn: &Arc<TurnContext>,
    task_name: &str,
) -> Result<WaitTaskTarget, FunctionCallError> {
    let thread_id = session
        .services
        .agent_control
        .resolve_agent_reference(session.conversation_id, &turn.session_source, task_name)
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "task_name `{task_name}` could not be resolved to a live agent: {err}"
            ))
        })?;
    let metadata = session
        .services
        .agent_control
        .get_agent_metadata(thread_id)
        .unwrap_or_default();
    let status_rx = session
        .services
        .agent_control
        .subscribe_status(thread_id)
        .await
        .map_err(|err| collab_agent_error(thread_id, err))?;
    Ok(WaitTaskTarget {
        thread_id,
        agent_ref: CollabAgentRef {
            thread_id,
            agent_nickname: metadata.agent_nickname,
            agent_role: metadata.agent_role,
        },
        status_rx,
    })
}

async fn wait_for_task_terminal_status(
    session: Arc<Session>,
    thread_id: codex_protocol::ThreadId,
    status_rx: &mut tokio::sync::watch::Receiver<AgentStatus>,
    deadline: Instant,
) -> Option<AgentStatus> {
    let mut status = status_rx.borrow().clone();
    if is_final(&status) {
        return Some(status);
    }

    loop {
        match timeout_at(deadline, status_rx.changed()).await {
            Ok(Ok(())) => {
                status = status_rx.borrow().clone();
                if is_final(&status) {
                    return Some(status);
                }
            }
            Ok(Err(_)) => {
                let latest = session.services.agent_control.get_status(thread_id).await;
                return is_final(&latest).then_some(latest);
            }
            Err(_) => return None,
        }
    }
}

async fn wait_for_task_result_evidence(
    session: Arc<Session>,
    task_id: Option<&str>,
    deadline: Instant,
) -> Option<Value> {
    let state_db = session.state_db()?;
    let task_id = task_id?;
    loop {
        if let Ok(Some(task_result)) = state_db
            .get_agent_job_task_result_envelope_by_task_id(task_id)
            .await
        {
            return Some(task_result);
        }
        if Instant::now() >= deadline {
            return None;
        }
        let next_probe = Instant::now() + Duration::from_millis(250);
        sleep_until(if next_probe < deadline {
            next_probe
        } else {
            deadline
        })
        .await;
    }
}

fn status_map_for_wait_target(
    target: Option<&WaitTaskTarget>,
    status: Option<&AgentStatus>,
) -> HashMap<codex_protocol::ThreadId, AgentStatus> {
    match (target, status) {
        (Some(target), Some(status)) => HashMap::from([(target.thread_id, status.clone())]),
        _ => HashMap::new(),
    }
}

fn redact_agent_status_content(status: AgentStatus) -> AgentStatus {
    match status {
        AgentStatus::Completed(_) => AgentStatus::Completed(None),
        status => status,
    }
}

async fn wait_for_mailbox_change(
    mailbox_seq_rx: &mut tokio::sync::watch::Receiver<u64>,
    deadline: Instant,
) -> bool {
    match timeout_at(deadline, mailbox_seq_rx.changed()).await {
        Ok(Ok(())) => true,
        Ok(Err(_)) | Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_status_redaction_removes_completed_content() {
        let status = redact_agent_status_content(AgentStatus::Completed(Some(
            "sensitive child output".to_string(),
        )));

        assert_eq!(status, AgentStatus::Completed(None));
    }
}
