use super::*;
use codex_protocol::protocol::InterAgentCommunication;

pub struct InterAgentMailboxBarrierEventParams<'a> {
    pub thread_id: ThreadId,
    pub barrier_id: &'a str,
    pub event_type: &'static str,
    pub status: &'static str,
    pub task_id: Option<&'a str>,
    pub task_name: Option<&'a str>,
    pub result_required: bool,
    pub deadline_at_ms: Option<i64>,
    pub trace_id: Option<&'a str>,
}

pub struct InterAgentWaitWorkGraphShadowEventParams<'a> {
    pub thread_id: ThreadId,
    pub barrier_id: &'a str,
    pub task_id: Option<&'a str>,
    pub task_name: Option<&'a str>,
    pub event_type: &'static str,
    pub status: &'static str,
    pub payload_json: Value,
    pub trace_id: Option<&'a str>,
}

struct InterAgentMailboxEventInsert<'a> {
    thread_id: ThreadId,
    event_type: &'static str,
    mailbox_seq: Option<i64>,
    barrier_id: Option<&'a str>,
    task_id: Option<&'a str>,
    task_name: Option<&'a str>,
    author_path: Option<String>,
    recipient_path: Option<String>,
    other_recipients_json: Option<Value>,
    trigger_turn: Option<bool>,
    content_json: Option<Value>,
    status: &'static str,
    deadline_at_ms: Option<i64>,
    trace_id: Option<&'a str>,
}

impl StateRuntime {
    pub async fn record_inter_agent_mailbox_queued(
        &self,
        thread_id: ThreadId,
        mailbox_seq: u64,
        communication: &InterAgentCommunication,
        trace_id: Option<&str>,
    ) -> anyhow::Result<InterAgentMailboxEvent> {
        self.append_inter_agent_mailbox_event(InterAgentMailboxEventInsert {
            thread_id,
            event_type: "inter_agent_message_queued",
            mailbox_seq: Some(u64_to_i64(mailbox_seq)),
            barrier_id: None,
            task_id: None,
            task_name: None,
            author_path: Some(communication.author.to_string()),
            recipient_path: Some(communication.recipient.to_string()),
            other_recipients_json: Some(serde_json::to_value(&communication.other_recipients)?),
            trigger_turn: Some(communication.trigger_turn),
            content_json: Some(serde_json::to_value(communication)?),
            status: "queued",
            deadline_at_ms: None,
            trace_id,
        })
        .await
    }

    pub async fn record_inter_agent_mailbox_delivered(
        &self,
        thread_id: ThreadId,
        mailbox_seq: u64,
        communication: &InterAgentCommunication,
        trace_id: Option<&str>,
    ) -> anyhow::Result<InterAgentMailboxEvent> {
        self.append_inter_agent_mailbox_event(InterAgentMailboxEventInsert {
            thread_id,
            event_type: "inter_agent_message_delivered",
            mailbox_seq: Some(u64_to_i64(mailbox_seq)),
            barrier_id: None,
            task_id: None,
            task_name: None,
            author_path: Some(communication.author.to_string()),
            recipient_path: Some(communication.recipient.to_string()),
            other_recipients_json: Some(serde_json::to_value(&communication.other_recipients)?),
            trigger_turn: Some(communication.trigger_turn),
            content_json: Some(serde_json::to_value(communication)?),
            status: "delivered",
            deadline_at_ms: None,
            trace_id,
        })
        .await
    }

    pub async fn record_inter_agent_wait_barrier_event(
        &self,
        params: InterAgentMailboxBarrierEventParams<'_>,
    ) -> anyhow::Result<InterAgentMailboxEvent> {
        let content_json = Some(serde_json::json!({
            "resultRequired": params.result_required,
            "shadowOnly": true,
        }));
        self.append_inter_agent_mailbox_event(InterAgentMailboxEventInsert {
            thread_id: params.thread_id,
            event_type: params.event_type,
            mailbox_seq: None,
            barrier_id: Some(params.barrier_id),
            task_id: params.task_id,
            task_name: params.task_name,
            author_path: None,
            recipient_path: None,
            other_recipients_json: None,
            trigger_turn: None,
            content_json,
            status: params.status,
            deadline_at_ms: params.deadline_at_ms,
            trace_id: params.trace_id,
        })
        .await
    }

    pub async fn record_inter_agent_wait_work_graph_shadow_event(
        &self,
        params: InterAgentWaitWorkGraphShadowEventParams<'_>,
    ) -> anyhow::Result<InterAgentMailboxEvent> {
        self.append_inter_agent_mailbox_event(InterAgentMailboxEventInsert {
            thread_id: params.thread_id,
            event_type: params.event_type,
            mailbox_seq: None,
            barrier_id: Some(params.barrier_id),
            task_id: params.task_id,
            task_name: params.task_name,
            author_path: None,
            recipient_path: None,
            other_recipients_json: None,
            trigger_turn: None,
            content_json: Some(params.payload_json),
            status: params.status,
            deadline_at_ms: None,
            trace_id: params.trace_id,
        })
        .await
    }

    async fn append_inter_agent_mailbox_event(
        &self,
        event: InterAgentMailboxEventInsert<'_>,
    ) -> anyhow::Result<InterAgentMailboxEvent> {
        let created_at_ms = datetime_to_epoch_millis(Utc::now());
        let other_recipients_json = event
            .other_recipients_json
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        let content_json = event
            .content_json
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        let trigger_turn = event
            .trigger_turn
            .map(|value| if value { 1_i64 } else { 0_i64 });
        let row = sqlx::query_as::<_, InterAgentMailboxEventRow>(
            r#"
INSERT INTO inter_agent_mailbox_events (
    thread_id,
    event_type,
    mailbox_seq,
    barrier_id,
    task_id,
    task_name,
    author_path,
    recipient_path,
    other_recipients_json,
    trigger_turn,
    content_json,
    status,
    created_at_ms,
    deadline_at_ms,
    trace_id,
    live_blocking_enabled,
    live_cutover_enabled
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0)
RETURNING
    sequence_id,
    thread_id,
    event_type,
    mailbox_seq,
    barrier_id,
    task_id,
    task_name,
    author_path,
    recipient_path,
    other_recipients_json,
    trigger_turn,
    content_json,
    status,
    created_at_ms,
    deadline_at_ms,
    trace_id,
    live_blocking_enabled,
    live_cutover_enabled
            "#,
        )
        .bind(event.thread_id.to_string())
        .bind(event.event_type)
        .bind(event.mailbox_seq)
        .bind(event.barrier_id)
        .bind(event.task_id)
        .bind(event.task_name)
        .bind(event.author_path)
        .bind(event.recipient_path)
        .bind(other_recipients_json)
        .bind(trigger_turn)
        .bind(content_json)
        .bind(event.status)
        .bind(created_at_ms)
        .bind(event.deadline_at_ms)
        .bind(event.trace_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        InterAgentMailboxEvent::try_from(row)
    }

    pub async fn list_inter_agent_mailbox_events(
        &self,
        thread_id: ThreadId,
    ) -> anyhow::Result<Vec<InterAgentMailboxEvent>> {
        let rows: Vec<InterAgentMailboxEventRow> = sqlx::query_as::<_, InterAgentMailboxEventRow>(
            r#"
SELECT
    sequence_id,
    thread_id,
    event_type,
    mailbox_seq,
    barrier_id,
    task_id,
    task_name,
    author_path,
    recipient_path,
    other_recipients_json,
    trigger_turn,
    content_json,
    status,
    created_at_ms,
    deadline_at_ms,
    trace_id,
    live_blocking_enabled,
    live_cutover_enabled
FROM inter_agent_mailbox_events
WHERE thread_id = ?
ORDER BY sequence_id ASC
            "#,
        )
        .bind(thread_id.to_string())
        .fetch_all(self.pool.as_ref())
        .await?;
        rows.into_iter()
            .map(InterAgentMailboxEvent::try_from)
            .collect()
    }

    pub async fn get_inter_agent_mailbox_projection(
        &self,
        thread_id: ThreadId,
    ) -> anyhow::Result<InterAgentMailboxProjection> {
        let row = sqlx::query(
            r#"
SELECT
    COUNT(*) AS total_events,
    SUM(CASE WHEN event_type = 'inter_agent_message_queued' THEN 1 ELSE 0 END) AS queued_events,
    SUM(CASE WHEN event_type = 'inter_agent_message_delivered' THEN 1 ELSE 0 END) AS delivered_events,
    SUM(CASE WHEN event_type = 'wait_barrier_opened' THEN 1 ELSE 0 END) AS barrier_opened_events,
    SUM(CASE WHEN event_type = 'wait_barrier_satisfied' THEN 1 ELSE 0 END) AS barrier_satisfied_events,
    SUM(CASE WHEN event_type = 'wait_barrier_timed_out' THEN 1 ELSE 0 END) AS barrier_timed_out_events,
    SUM(CASE WHEN live_blocking_enabled != 0 THEN 1 ELSE 0 END) AS live_blocking_event_count,
    SUM(CASE WHEN live_cutover_enabled != 0 THEN 1 ELSE 0 END) AS live_cutover_event_count
FROM inter_agent_mailbox_events
WHERE thread_id = ?
            "#,
        )
        .bind(thread_id.to_string())
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(InterAgentMailboxProjection {
            thread_id,
            total_events: sql_count_to_usize(row.try_get("total_events")?),
            queued_events: optional_sql_count_to_usize(row.try_get("queued_events")?),
            delivered_events: optional_sql_count_to_usize(row.try_get("delivered_events")?),
            barrier_opened_events: optional_sql_count_to_usize(
                row.try_get("barrier_opened_events")?,
            ),
            barrier_satisfied_events: optional_sql_count_to_usize(
                row.try_get("barrier_satisfied_events")?,
            ),
            barrier_timed_out_events: optional_sql_count_to_usize(
                row.try_get("barrier_timed_out_events")?,
            ),
            live_blocking_event_count: optional_sql_count_to_usize(
                row.try_get("live_blocking_event_count")?,
            ),
            live_cutover_event_count: optional_sql_count_to_usize(
                row.try_get("live_cutover_event_count")?,
            ),
        })
    }

    pub async fn get_inter_agent_wait_task_result_readback(
        &self,
        thread_id: ThreadId,
        barrier_id: &str,
    ) -> anyhow::Result<InterAgentWaitTaskResultReadback> {
        let rows: Vec<InterAgentMailboxEventRow> = sqlx::query_as::<_, InterAgentMailboxEventRow>(
            r#"
SELECT
    sequence_id,
    thread_id,
    event_type,
    mailbox_seq,
    barrier_id,
    task_id,
    task_name,
    author_path,
    recipient_path,
    other_recipients_json,
    trigger_turn,
    content_json,
    status,
    created_at_ms,
    deadline_at_ms,
    trace_id,
    live_blocking_enabled,
    live_cutover_enabled
FROM inter_agent_mailbox_events
WHERE thread_id = ?
  AND barrier_id = ?
  AND event_type IN (
      'wait_task_result_delivery_shadow',
      'wait_parent_reducer_shadow_receipt',
      'wait_task_result_replay_consistency',
      'wait_surface_audit_packet',
      'wait_surface_audit_replay_consistency',
      'wait_work_graph_canonical_projection_receipt',
      'wait_work_graph_canonical_projection_replay_consistency',
      'wait_work_graph_canonical_projection_closeout_receipt',
      'wait_work_graph_canonical_projection_closeout_replay_consistency',
	      'wait_work_graph_canonical_projection_audit_chain_closeout_receipt',
	      'wait_work_graph_canonical_projection_audit_chain_closeout_replay_consistency',
	      'wait_work_graph_canonical_projection_enablement_operator_review_packet',
	      'wait_work_graph_canonical_projection_enablement_operator_review_replay_consistency',
	      'wait_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt',
	      'wait_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency',
	      'wait_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt',
	      'wait_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency',
	      'wait_work_graph_canonical_projection_enablement_activation_precondition_operator_packet',
		      'wait_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency',
		      'wait_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt',
		      'wait_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency',
			      'wait_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt',
				      'wait_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency',
				      'wait_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet',
				      'wait_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency',
					      'wait_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet',
					      'wait_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency'
				  )
ORDER BY sequence_id ASC
            "#,
        )
        .bind(thread_id.to_string())
        .bind(barrier_id)
        .fetch_all(self.pool.as_ref())
        .await?;

        let events = rows
            .into_iter()
            .map(InterAgentMailboxEvent::try_from)
            .collect::<anyhow::Result<Vec<_>>>()?;
        let task_result_delivery_shadow_events = events
            .iter()
            .filter(|event| event.event_type == "wait_task_result_delivery_shadow")
            .count();
        let parent_reducer_shadow_receipt_events = events
            .iter()
            .filter(|event| event.event_type == "wait_parent_reducer_shadow_receipt")
            .count();
        let task_result_replay_consistency_events = events
            .iter()
            .filter(|event| event.event_type == "wait_task_result_replay_consistency")
            .count();
        let wait_surface_audit_packet_events = events
            .iter()
            .filter(|event| event.event_type == "wait_surface_audit_packet")
            .count();
        let wait_surface_audit_replay_consistency_events = events
            .iter()
            .filter(|event| event.event_type == "wait_surface_audit_replay_consistency")
            .count();
        let wait_canonical_projection_receipt_events = events
            .iter()
            .filter(|event| event.event_type == "wait_work_graph_canonical_projection_receipt")
            .count();
        let wait_canonical_projection_replay_consistency_events = events
            .iter()
            .filter(|event| {
                event.event_type == "wait_work_graph_canonical_projection_replay_consistency"
            })
            .count();
        let wait_canonical_projection_closeout_receipt_events = events
            .iter()
            .filter(|event| {
                event.event_type == "wait_work_graph_canonical_projection_closeout_receipt"
            })
            .count();
        let wait_canonical_projection_closeout_replay_consistency_events = events
            .iter()
            .filter(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_closeout_replay_consistency"
            })
            .count();
        let wait_canonical_projection_audit_chain_closeout_receipt_events = events
            .iter()
            .filter(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_audit_chain_closeout_receipt"
            })
            .count();
        let wait_canonical_projection_audit_chain_closeout_replay_consistency_events = events
            .iter()
            .filter(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_audit_chain_closeout_replay_consistency"
            })
            .count();
        let wait_canonical_projection_enablement_operator_review_packet_events = events
            .iter()
            .filter(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_operator_review_packet"
            })
            .count();
        let wait_canonical_projection_enablement_operator_review_replay_consistency_events = events
            .iter()
            .filter(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_operator_review_replay_consistency"
            })
            .count();
        let wait_canonical_projection_enablement_no_live_rehearsal_closeout_events = events
            .iter()
            .filter(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt"
            })
            .count();
        let wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_events =
            events
                .iter()
                .filter(|event| {
                    event.event_type
                        == "wait_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency"
                })
                .count();
        let wait_canonical_projection_enablement_audit_chain_closeout_events = events
            .iter()
            .filter(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt"
            })
            .count();
        let wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_events =
            events
                .iter()
                .filter(|event| {
                    event.event_type
                        == "wait_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency"
                })
                .count();
        let wait_canonical_projection_enablement_activation_precondition_operator_packet_events =
            events
                .iter()
                .filter(|event| {
                    event.event_type
                        == "wait_work_graph_canonical_projection_enablement_activation_precondition_operator_packet"
                })
                .count();
        let wait_canonical_projection_enablement_activation_precondition_replay_consistency_events =
            events
                .iter()
                .filter(|event| {
                    event.event_type
                        == "wait_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency"
                })
                .count();
        let wait_canonical_projection_enablement_activation_no_live_closeout_events = events
            .iter()
            .filter(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt"
            })
            .count();
        let wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_events =
            events
                .iter()
                .filter(|event| {
                    event.event_type
                        == "wait_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency"
                })
                .count();
        let wait_canonical_projection_enablement_activation_audit_chain_closeout_events = events
            .iter()
            .filter(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"
            })
            .count();
        let wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_events =
            events
                .iter()
                .filter(|event| {
                    event.event_type
                        == "wait_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency"
                })
                .count();
        let wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_events =
            events
                .iter()
                .filter(|event| {
                    event.event_type
                        == "wait_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"
                })
                .count();
        let wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_events =
            events
                .iter()
                .filter(|event| {
                    event.event_type
                        == "wait_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency"
                })
                .count();
        let wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_events =
            events
                .iter()
                .filter(|event| {
                    event.event_type
                        == "wait_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"
                })
                .count();
        let wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_events =
            events
                .iter()
                .filter(|event| {
                    event.event_type
                        == "wait_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency"
                })
                .count();
        let latest_task_result_delivery = events
            .iter()
            .rev()
            .find(|event| event.event_type == "wait_task_result_delivery_shadow");
        let latest_parent_reducer = events
            .iter()
            .rev()
            .find(|event| event.event_type == "wait_parent_reducer_shadow_receipt");
        let latest_task_result_replay_consistency = events
            .iter()
            .rev()
            .find(|event| event.event_type == "wait_task_result_replay_consistency");
        let latest_wait_surface_audit_packet = events
            .iter()
            .rev()
            .find(|event| event.event_type == "wait_surface_audit_packet");
        let latest_wait_surface_audit_replay_consistency = events
            .iter()
            .rev()
            .find(|event| event.event_type == "wait_surface_audit_replay_consistency");
        let latest_wait_canonical_projection_receipt = events
            .iter()
            .rev()
            .find(|event| event.event_type == "wait_work_graph_canonical_projection_receipt");
        let latest_wait_canonical_projection_replay_consistency =
            events.iter().rev().find(|event| {
                event.event_type == "wait_work_graph_canonical_projection_replay_consistency"
            });
        let latest_wait_canonical_projection_closeout_receipt = events.iter().rev().find(|event| {
            event.event_type == "wait_work_graph_canonical_projection_closeout_receipt"
        });
        let latest_wait_canonical_projection_closeout_replay_consistency =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_closeout_replay_consistency"
            });
        let latest_wait_canonical_projection_audit_chain_closeout_receipt =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_audit_chain_closeout_receipt"
            });
        let latest_wait_canonical_projection_audit_chain_closeout_replay_consistency =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_audit_chain_closeout_replay_consistency"
            });
        let latest_wait_canonical_projection_enablement_operator_review_packet =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_operator_review_packet"
            });
        let latest_wait_canonical_projection_enablement_operator_review_replay_consistency =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_operator_review_replay_consistency"
            });
        let latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt"
            });
        let latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency"
            });
        let latest_wait_canonical_projection_enablement_audit_chain_closeout =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt"
            });
        let latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency"
            });
        let latest_wait_canonical_projection_enablement_activation_precondition_operator_packet =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_precondition_operator_packet"
            });
        let latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency"
            });
        let latest_wait_canonical_projection_enablement_activation_no_live_closeout =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt"
            });
        let latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency"
            });
        let latest_wait_canonical_projection_enablement_activation_audit_chain_closeout =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt"
            });
        let latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency"
            });
        let latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet"
            });
        let latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency"
            });
        let latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet"
            });
        let latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency =
            events.iter().rev().find(|event| {
                event.event_type
                    == "wait_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency"
            });
        let latest_task_result_delivery_shadow =
            latest_task_result_delivery.and_then(|event| event.content_json.clone());
        let latest_parent_reducer_shadow_receipt =
            latest_parent_reducer.and_then(|event| event.content_json.clone());
        let latest_task_result_replay_consistency_payload =
            latest_task_result_replay_consistency.and_then(|event| event.content_json.clone());
        let latest_wait_surface_audit_packet_payload =
            latest_wait_surface_audit_packet.and_then(|event| event.content_json.clone());
        let latest_wait_surface_audit_replay_consistency_payload =
            latest_wait_surface_audit_replay_consistency
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_receipt_payload =
            latest_wait_canonical_projection_receipt.and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_replay_consistency_payload =
            latest_wait_canonical_projection_replay_consistency
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_closeout_receipt_payload =
            latest_wait_canonical_projection_closeout_receipt
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_closeout_replay_consistency_payload =
            latest_wait_canonical_projection_closeout_replay_consistency
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_audit_chain_closeout_payload =
            latest_wait_canonical_projection_audit_chain_closeout_receipt
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_audit_chain_closeout_replay_consistency_payload =
            latest_wait_canonical_projection_audit_chain_closeout_replay_consistency
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_operator_review_packet_payload =
            latest_wait_canonical_projection_enablement_operator_review_packet
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_operator_review_replay_consistency_payload =
            latest_wait_canonical_projection_enablement_operator_review_replay_consistency
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_payload =
            latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_payload =
            latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_audit_chain_closeout_payload =
            latest_wait_canonical_projection_enablement_audit_chain_closeout
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_payload =
            latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_activation_precondition_operator_packet_payload =
            latest_wait_canonical_projection_enablement_activation_precondition_operator_packet
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency_payload =
            latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_activation_no_live_closeout_payload =
            latest_wait_canonical_projection_enablement_activation_no_live_closeout
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_payload =
            latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_payload =
            latest_wait_canonical_projection_enablement_activation_audit_chain_closeout
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_payload =
            latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_payload =
            latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_payload =
            latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_payload =
            latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet
                .and_then(|event| event.content_json.clone());
        let latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_payload =
            latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency
                .and_then(|event| event.content_json.clone());
        let task_result_delivery_ready = latest_task_result_delivery_shadow
            .as_ref()
            .and_then(|payload| payload.get("shadowDeliveryReady"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let parent_reducer_receipt_ready = latest_parent_reducer_shadow_receipt
            .as_ref()
            .and_then(|payload| payload.get("parentReducerReceiptReady"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let replay_consistent = latest_task_result_replay_consistency_payload
            .as_ref()
            .and_then(|payload| payload.get("replayConsistent"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let wait_surface_audit_packet_ready = latest_wait_surface_audit_packet_payload
            .as_ref()
            .and_then(|payload| payload.get("auditPacketReady"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let wait_surface_audit_replay_consistent =
            latest_wait_surface_audit_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_receipt_ready =
            latest_wait_canonical_projection_receipt_payload
                .as_ref()
                .and_then(|payload| payload.get("projectionReceiptReady"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_replay_consistent =
            latest_wait_canonical_projection_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_closeout_receipt_ready =
            latest_wait_canonical_projection_closeout_receipt_payload
                .as_ref()
                .and_then(|payload| payload.get("closeoutReady"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_closeout_replay_consistent =
            latest_wait_canonical_projection_closeout_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_audit_chain_closeout_receipt_ready =
            latest_wait_canonical_projection_audit_chain_closeout_payload
                .as_ref()
                .and_then(|payload| payload.get("auditChainCloseoutReady"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_audit_chain_closeout_replay_consistent =
            latest_wait_canonical_projection_audit_chain_closeout_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_operator_review_ready =
            latest_wait_canonical_projection_enablement_operator_review_packet_payload
                .as_ref()
                .and_then(|payload| payload.get("enablementOperatorReviewReady"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_operator_review_replay_consistent =
            latest_wait_canonical_projection_enablement_operator_review_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready =
            latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_payload
                .as_ref()
                .and_then(|payload| payload.get("noLiveEnablementRehearsalCloseoutReady"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent =
            latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_audit_chain_closeout_ready =
            latest_wait_canonical_projection_enablement_audit_chain_closeout_payload
                .as_ref()
                .and_then(|payload| payload.get("enablementAuditChainCloseoutReady"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_audit_chain_closeout_replay_consistent =
            latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_activation_precondition_ready =
            latest_wait_canonical_projection_enablement_activation_precondition_operator_packet_payload
                .as_ref()
                .and_then(|payload| payload.get("activationPreconditionReady"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_activation_precondition_replay_consistent =
            latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_activation_no_live_closeout_ready =
            latest_wait_canonical_projection_enablement_activation_no_live_closeout_payload
                .as_ref()
                .and_then(|payload| payload.get("activationNoLiveCloseoutReady"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistent =
            latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_activation_audit_chain_closeout_ready =
            latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_payload
                .as_ref()
                .and_then(|payload| payload.get("activationAuditChainCloseoutReady"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent =
            latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready =
            latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_payload
                .as_ref()
                .and_then(|payload| payload.get("activationOperatorApprovalReadinessPreflightReady"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent =
            latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready =
            latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_payload
                .as_ref()
                .and_then(|payload| payload.get("approvalReviewSideEffectLockCloseoutReady"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent =
            latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_payload
                .as_ref()
                .and_then(|payload| payload.get("replayConsistent"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let live_blocking_event_count = events
            .iter()
            .filter(|event| event.live_blocking_enabled)
            .count();
        let live_cutover_event_count = events
            .iter()
            .filter(|event| event.live_cutover_enabled)
            .count();
        let task_result_delivery_readback_ready = task_result_delivery_shadow_events > 0;
        let parent_reducer_readback_ready = parent_reducer_shadow_receipt_events > 0;
        let replay_consistency_ready = task_result_replay_consistency_events > 0
            && latest_task_result_replay_consistency_payload.is_some();
        let wait_surface_audit_packet_readback_ready = wait_surface_audit_packet_events > 0
            && latest_wait_surface_audit_packet_payload.is_some();
        let wait_surface_audit_replay_consistency_ready =
            wait_surface_audit_replay_consistency_events > 0
                && latest_wait_surface_audit_replay_consistency_payload.is_some();
        let wait_canonical_projection_receipt_readback_ready =
            wait_canonical_projection_receipt_events > 0
                && latest_wait_canonical_projection_receipt_payload.is_some();
        let wait_canonical_projection_replay_consistency_ready =
            wait_canonical_projection_replay_consistency_events > 0
                && latest_wait_canonical_projection_replay_consistency_payload.is_some();
        let wait_canonical_projection_closeout_receipt_readback_ready =
            wait_canonical_projection_closeout_receipt_events > 0
                && latest_wait_canonical_projection_closeout_receipt_payload.is_some();
        let wait_canonical_projection_closeout_replay_consistency_ready =
            wait_canonical_projection_closeout_replay_consistency_events > 0
                && latest_wait_canonical_projection_closeout_replay_consistency_payload.is_some();
        let wait_canonical_projection_audit_chain_closeout_receipt_readback_ready =
            wait_canonical_projection_audit_chain_closeout_receipt_events > 0
                && latest_wait_canonical_projection_audit_chain_closeout_payload.is_some();
        let wait_canonical_projection_audit_chain_closeout_replay_consistency_ready =
            wait_canonical_projection_audit_chain_closeout_replay_consistency_events > 0
                && latest_wait_canonical_projection_audit_chain_closeout_replay_consistency_payload
                    .is_some();
        let wait_canonical_projection_enablement_operator_review_packet_readback_ready =
            wait_canonical_projection_enablement_operator_review_packet_events > 0
                && latest_wait_canonical_projection_enablement_operator_review_packet_payload
                    .is_some();
        let wait_canonical_projection_enablement_operator_review_replay_consistency_ready =
            wait_canonical_projection_enablement_operator_review_replay_consistency_events > 0
                && latest_wait_canonical_projection_enablement_operator_review_replay_consistency_payload
                    .is_some();
        let wait_canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready =
            wait_canonical_projection_enablement_no_live_rehearsal_closeout_events > 0
                && latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_payload
                    .is_some();
        let wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_ready =
            wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_events > 0
                && latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_payload
                    .is_some();
        let wait_canonical_projection_enablement_audit_chain_closeout_readback_ready =
            wait_canonical_projection_enablement_audit_chain_closeout_events > 0
                && latest_wait_canonical_projection_enablement_audit_chain_closeout_payload
                    .is_some();
        let wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_ready =
            wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_events > 0
                && latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_payload
                    .is_some();
        let wait_canonical_projection_enablement_activation_precondition_operator_packet_readback_ready =
            wait_canonical_projection_enablement_activation_precondition_operator_packet_events > 0
                && latest_wait_canonical_projection_enablement_activation_precondition_operator_packet_payload
                    .is_some();
        let wait_canonical_projection_enablement_activation_precondition_replay_consistency_ready =
            wait_canonical_projection_enablement_activation_precondition_replay_consistency_events > 0
                && latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency_payload
                    .is_some();
        let wait_canonical_projection_enablement_activation_no_live_closeout_readback_ready =
            wait_canonical_projection_enablement_activation_no_live_closeout_events > 0
                && latest_wait_canonical_projection_enablement_activation_no_live_closeout_payload
                    .is_some();
        let wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_ready =
            wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_events > 0
                && latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_payload
                    .is_some();
        let wait_canonical_projection_enablement_activation_audit_chain_closeout_readback_ready =
            wait_canonical_projection_enablement_activation_audit_chain_closeout_events > 0
                && latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_payload
                    .is_some();
        let wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_ready =
            wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_events > 0
                && latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_payload
                    .is_some();
        let wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready =
            wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_events > 0
                && latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_payload
                    .is_some();
        let wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_ready =
            wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_events > 0
                && latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_payload
                    .is_some();
        let wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_readback_ready =
            wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_events > 0
                && latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_payload
                    .is_some();
        let wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_ready =
            wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_events > 0
                && latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_payload
                    .is_some();
        let no_live_guardrails_ready =
            live_blocking_event_count == 0 && live_cutover_event_count == 0;
        let readback_ready = task_result_delivery_readback_ready && parent_reducer_readback_ready;
        let direct_wait_task_result_ready = readback_ready
            && task_result_delivery_ready
            && parent_reducer_receipt_ready
            && replay_consistency_ready
            && replay_consistent
            && no_live_guardrails_ready;
        let direct_wait_surface_audit_ready = direct_wait_task_result_ready
            && wait_surface_audit_packet_readback_ready
            && wait_surface_audit_packet_ready
            && wait_surface_audit_replay_consistency_ready
            && wait_surface_audit_replay_consistent;
        let direct_wait_canonical_projection_ready = direct_wait_surface_audit_ready
            && wait_canonical_projection_receipt_readback_ready
            && wait_canonical_projection_receipt_ready
            && wait_canonical_projection_replay_consistency_ready
            && wait_canonical_projection_replay_consistent;
        let direct_wait_canonical_projection_closeout_ready = direct_wait_canonical_projection_ready
            && wait_canonical_projection_closeout_receipt_readback_ready
            && wait_canonical_projection_closeout_receipt_ready
            && wait_canonical_projection_closeout_replay_consistency_ready
            && wait_canonical_projection_closeout_replay_consistent;
        let direct_wait_canonical_projection_audit_chain_closeout_receipt_ready =
            direct_wait_canonical_projection_closeout_ready
                && wait_canonical_projection_audit_chain_closeout_receipt_readback_ready
                && wait_canonical_projection_audit_chain_closeout_receipt_ready;
        let direct_wait_canonical_projection_audit_chain_closeout_ready =
            direct_wait_canonical_projection_audit_chain_closeout_receipt_ready
                && wait_canonical_projection_audit_chain_closeout_replay_consistency_ready
                && wait_canonical_projection_audit_chain_closeout_replay_consistent;
        let direct_wait_canonical_projection_audit_chain_closeout_replay_ready =
            direct_wait_canonical_projection_audit_chain_closeout_ready;
        let direct_wait_canonical_projection_enablement_operator_review_ready =
            direct_wait_canonical_projection_audit_chain_closeout_replay_ready
                && wait_canonical_projection_enablement_operator_review_packet_readback_ready
                && wait_canonical_projection_enablement_operator_review_ready
                && wait_canonical_projection_enablement_operator_review_replay_consistency_ready
                && wait_canonical_projection_enablement_operator_review_replay_consistent;
        let direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready =
            direct_wait_canonical_projection_enablement_operator_review_ready
                && wait_canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready
                && wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready
                && wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_ready
                && wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent;
        let direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_ready =
            direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready;
        let direct_wait_canonical_projection_enablement_audit_chain_closeout_ready =
            direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_ready
                && wait_canonical_projection_enablement_audit_chain_closeout_readback_ready
                && wait_canonical_projection_enablement_audit_chain_closeout_ready
                && wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_ready
                && wait_canonical_projection_enablement_audit_chain_closeout_replay_consistent;
        let direct_wait_canonical_projection_enablement_audit_chain_closeout_replay_ready =
            direct_wait_canonical_projection_enablement_audit_chain_closeout_ready;
        let direct_wait_canonical_projection_enablement_activation_precondition_ready =
            direct_wait_canonical_projection_enablement_audit_chain_closeout_replay_ready
                && wait_canonical_projection_enablement_activation_precondition_operator_packet_readback_ready
                && wait_canonical_projection_enablement_activation_precondition_ready
                && wait_canonical_projection_enablement_activation_precondition_replay_consistency_ready
                && wait_canonical_projection_enablement_activation_precondition_replay_consistent;
        let direct_wait_canonical_projection_enablement_activation_precondition_replay_ready =
            direct_wait_canonical_projection_enablement_activation_precondition_ready;
        let direct_wait_canonical_projection_enablement_activation_no_live_closeout_ready =
            direct_wait_canonical_projection_enablement_activation_precondition_replay_ready
                && wait_canonical_projection_enablement_activation_no_live_closeout_readback_ready
                && wait_canonical_projection_enablement_activation_no_live_closeout_ready;
        let direct_wait_canonical_projection_enablement_activation_no_live_closeout_replay_ready =
            direct_wait_canonical_projection_enablement_activation_no_live_closeout_ready
                && wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_ready
                && wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistent;
        let direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_ready =
            direct_wait_canonical_projection_enablement_activation_no_live_closeout_replay_ready
                && wait_canonical_projection_enablement_activation_audit_chain_closeout_readback_ready
                && wait_canonical_projection_enablement_activation_audit_chain_closeout_ready;
        let direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_ready =
            direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_ready
                && wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_ready
                && wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent;
        let direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready =
            direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_ready
                && wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready
                && wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready;
        let direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_ready =
            direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready
                && wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_ready
                && wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent;
        let direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready =
            direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_ready
                && wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_readback_ready
                && wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready;
        let direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_ready =
            direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready
                && wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_ready
                && wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent;

        Ok(InterAgentWaitTaskResultReadback {
            thread_id,
            barrier_id: barrier_id.to_string(),
            task_result_delivery_shadow_events,
            parent_reducer_shadow_receipt_events,
            task_result_replay_consistency_events,
            wait_surface_audit_packet_events,
            wait_surface_audit_replay_consistency_events,
            wait_canonical_projection_receipt_events,
            wait_canonical_projection_replay_consistency_events,
            wait_canonical_projection_closeout_receipt_events,
            wait_canonical_projection_closeout_replay_consistency_events,
            wait_canonical_projection_audit_chain_closeout_receipt_events,
            wait_canonical_projection_audit_chain_closeout_replay_consistency_events,
            wait_canonical_projection_enablement_operator_review_packet_events,
            wait_canonical_projection_enablement_operator_review_replay_consistency_events,
            wait_canonical_projection_enablement_no_live_rehearsal_closeout_events,
            wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_events,
            wait_canonical_projection_enablement_audit_chain_closeout_events,
            wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_events,
            wait_canonical_projection_enablement_activation_precondition_operator_packet_events,
            wait_canonical_projection_enablement_activation_precondition_replay_consistency_events,
            wait_canonical_projection_enablement_activation_no_live_closeout_events,
            wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_events,
            wait_canonical_projection_enablement_activation_audit_chain_closeout_events,
            wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_events,
            wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_events,
            wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_events,
            wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_events,
            wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_events,
            live_blocking_event_count,
            live_cutover_event_count,
            latest_task_result_delivery_shadow,
            latest_parent_reducer_shadow_receipt,
            latest_task_result_replay_consistency: latest_task_result_replay_consistency_payload,
            latest_wait_surface_audit_packet: latest_wait_surface_audit_packet_payload,
            latest_wait_surface_audit_replay_consistency:
                latest_wait_surface_audit_replay_consistency_payload,
            latest_wait_canonical_projection_receipt:
                latest_wait_canonical_projection_receipt_payload,
            latest_wait_canonical_projection_replay_consistency:
                latest_wait_canonical_projection_replay_consistency_payload,
            latest_wait_canonical_projection_closeout_receipt:
                latest_wait_canonical_projection_closeout_receipt_payload,
            latest_wait_canonical_projection_closeout_replay_consistency:
                latest_wait_canonical_projection_closeout_replay_consistency_payload,
            latest_wait_canonical_projection_audit_chain_closeout_receipt:
                latest_wait_canonical_projection_audit_chain_closeout_payload,
            latest_wait_canonical_projection_audit_chain_closeout_replay_consistency:
                latest_wait_canonical_projection_audit_chain_closeout_replay_consistency_payload,
            latest_wait_canonical_projection_enablement_operator_review_packet:
                latest_wait_canonical_projection_enablement_operator_review_packet_payload,
            latest_wait_canonical_projection_enablement_operator_review_replay_consistency:
                latest_wait_canonical_projection_enablement_operator_review_replay_consistency_payload,
            latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout:
                latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_payload,
            latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency:
                latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_payload,
            latest_wait_canonical_projection_enablement_audit_chain_closeout:
                latest_wait_canonical_projection_enablement_audit_chain_closeout_payload,
            latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency:
                latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_payload,
            latest_wait_canonical_projection_enablement_activation_precondition_operator_packet:
                latest_wait_canonical_projection_enablement_activation_precondition_operator_packet_payload,
            latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency:
                latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency_payload,
            latest_wait_canonical_projection_enablement_activation_no_live_closeout:
                latest_wait_canonical_projection_enablement_activation_no_live_closeout_payload,
            latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency:
                latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_payload,
            latest_wait_canonical_projection_enablement_activation_audit_chain_closeout:
                latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_payload,
            latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency:
                latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_payload,
            latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet:
                latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_payload,
            latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency:
                latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_payload,
            latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet:
                latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_payload,
            latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency:
                latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_payload,
            latest_task_result_delivery_decision: latest_task_result_delivery
                .map(|event| event.status.clone())
                .unwrap_or_else(|| "missing".to_string()),
            latest_parent_reducer_decision: latest_parent_reducer
                .map(|event| event.status.clone())
                .unwrap_or_else(|| "missing".to_string()),
            latest_task_result_replay_consistency_decision: latest_task_result_replay_consistency
                .map(|event| event.status.clone())
                .unwrap_or_else(|| "missing".to_string()),
            latest_wait_surface_audit_decision: latest_wait_surface_audit_packet
                .map(|event| event.status.clone())
                .unwrap_or_else(|| "missing".to_string()),
            latest_wait_surface_audit_replay_consistency_decision:
                latest_wait_surface_audit_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_decision: latest_wait_canonical_projection_receipt
                .map(|event| event.status.clone())
                .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_replay_consistency_decision:
                latest_wait_canonical_projection_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_closeout_decision:
                latest_wait_canonical_projection_closeout_receipt
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_closeout_replay_consistency_decision:
                latest_wait_canonical_projection_closeout_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_audit_chain_closeout_decision:
                latest_wait_canonical_projection_audit_chain_closeout_receipt
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_audit_chain_closeout_replay_consistency_decision:
                latest_wait_canonical_projection_audit_chain_closeout_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_operator_review_decision:
                latest_wait_canonical_projection_enablement_operator_review_packet
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_operator_review_replay_consistency_decision:
                latest_wait_canonical_projection_enablement_operator_review_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_decision:
                latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision:
                latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_audit_chain_closeout_decision:
                latest_wait_canonical_projection_enablement_audit_chain_closeout
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision:
                latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_activation_precondition_operator_decision:
                latest_wait_canonical_projection_enablement_activation_precondition_operator_packet
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency_decision:
                latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_activation_no_live_closeout_decision:
                latest_wait_canonical_projection_enablement_activation_no_live_closeout
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision:
                latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_decision:
                latest_wait_canonical_projection_enablement_activation_audit_chain_closeout
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision:
                latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_decision:
                latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision:
                latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_decision:
                latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision:
                latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency
                    .map(|event| event.status.clone())
                    .unwrap_or_else(|| "missing".to_string()),
            task_result_delivery_readback_ready,
            parent_reducer_readback_ready,
            replay_consistency_ready,
            wait_surface_audit_packet_readback_ready,
            wait_surface_audit_replay_consistency_ready,
            wait_canonical_projection_receipt_readback_ready,
            wait_canonical_projection_replay_consistency_ready,
            wait_canonical_projection_closeout_receipt_readback_ready,
            wait_canonical_projection_closeout_replay_consistency_ready,
            wait_canonical_projection_audit_chain_closeout_receipt_readback_ready,
            wait_canonical_projection_audit_chain_closeout_replay_consistency_ready,
            wait_canonical_projection_enablement_operator_review_packet_readback_ready,
            wait_canonical_projection_enablement_operator_review_replay_consistency_ready,
            wait_canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready,
            wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_ready,
            wait_canonical_projection_enablement_audit_chain_closeout_readback_ready,
            wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_ready,
            wait_canonical_projection_enablement_activation_precondition_operator_packet_readback_ready,
            wait_canonical_projection_enablement_activation_precondition_replay_consistency_ready,
            wait_canonical_projection_enablement_activation_no_live_closeout_readback_ready,
            wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_ready,
            wait_canonical_projection_enablement_activation_audit_chain_closeout_readback_ready,
            wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_ready,
            wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready,
            wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_ready,
            wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_readback_ready,
            wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_ready,
            task_result_delivery_ready,
            parent_reducer_receipt_ready,
            replay_consistent,
            wait_surface_audit_packet_ready,
            wait_surface_audit_replay_consistent,
            wait_canonical_projection_receipt_ready,
            wait_canonical_projection_replay_consistent,
            wait_canonical_projection_closeout_receipt_ready,
            wait_canonical_projection_closeout_replay_consistent,
            wait_canonical_projection_audit_chain_closeout_receipt_ready,
            wait_canonical_projection_audit_chain_closeout_replay_consistent,
            wait_canonical_projection_enablement_operator_review_ready,
            wait_canonical_projection_enablement_operator_review_replay_consistent,
            wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready,
            wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent,
            wait_canonical_projection_enablement_audit_chain_closeout_ready,
            wait_canonical_projection_enablement_audit_chain_closeout_replay_consistent,
            wait_canonical_projection_enablement_activation_precondition_ready,
            wait_canonical_projection_enablement_activation_precondition_replay_consistent,
            wait_canonical_projection_enablement_activation_no_live_closeout_ready,
            wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistent,
            wait_canonical_projection_enablement_activation_audit_chain_closeout_ready,
            wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent,
            wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready,
            wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent,
            wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready,
            wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent,
            no_live_guardrails_ready,
            readback_ready,
            direct_wait_task_result_ready,
            direct_wait_surface_audit_ready,
            direct_wait_canonical_projection_ready,
            direct_wait_canonical_projection_closeout_ready,
            direct_wait_canonical_projection_audit_chain_closeout_ready,
            direct_wait_canonical_projection_audit_chain_closeout_replay_ready,
            direct_wait_canonical_projection_enablement_operator_review_ready,
            direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready,
            direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_ready,
            direct_wait_canonical_projection_enablement_audit_chain_closeout_ready,
            direct_wait_canonical_projection_enablement_audit_chain_closeout_replay_ready,
            direct_wait_canonical_projection_enablement_activation_precondition_ready,
            direct_wait_canonical_projection_enablement_activation_precondition_replay_ready,
            direct_wait_canonical_projection_enablement_activation_no_live_closeout_ready,
            direct_wait_canonical_projection_enablement_activation_no_live_closeout_replay_ready,
            direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_ready,
            direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_ready,
            direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready,
            direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_ready,
            direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready,
            direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_ready,
        })
    }
}

fn u64_to_i64(value: u64) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}

fn sql_count_to_usize(value: i64) -> usize {
    usize::try_from(value).unwrap_or_default()
}

fn optional_sql_count_to_usize(value: Option<i64>) -> usize {
    value.map(sql_count_to_usize).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_protocol::AgentPath;
    use pretty_assertions::assert_eq;

    fn unique_temp_dir() -> PathBuf {
        let suffix = uuid::Uuid::new_v4();
        std::env::temp_dir().join(format!("codex-state-mailbox-test-{suffix}"))
    }

    fn test_thread_id() -> ThreadId {
        ThreadId::from_string("00000000-0000-0000-0000-000000000777")
            .expect("test thread id should parse")
    }

    fn test_communication() -> InterAgentCommunication {
        InterAgentCommunication::new(
            AgentPath::try_from("/root/worker").expect("author path should parse"),
            AgentPath::root(),
            Vec::new(),
            "done".to_string(),
            /*trigger_turn*/ true,
        )
    }

    #[tokio::test]
    async fn mailbox_events_record_queue_delivery_and_barrier_projection() -> anyhow::Result<()> {
        let codex_home = unique_temp_dir();
        let runtime = StateRuntime::init(codex_home, "test-provider".to_string()).await?;
        let thread_id = test_thread_id();
        let communication = test_communication();

        runtime
            .record_inter_agent_mailbox_queued(thread_id, 1, &communication, Some("trace-1"))
            .await?;
        runtime
            .record_inter_agent_mailbox_delivered(thread_id, 1, &communication, Some("trace-1"))
            .await?;
        runtime
            .record_inter_agent_wait_barrier_event(InterAgentMailboxBarrierEventParams {
                thread_id,
                barrier_id: "barrier-1",
                event_type: "wait_barrier_opened",
                status: "opened",
                task_id: Some("task-1"),
                task_name: Some("/root/worker"),
                result_required: true,
                deadline_at_ms: Some(1_700_000_000_000),
                trace_id: Some("trace-1"),
            })
            .await?;
        runtime
            .record_inter_agent_wait_barrier_event(InterAgentMailboxBarrierEventParams {
                thread_id,
                barrier_id: "barrier-1",
                event_type: "wait_barrier_satisfied",
                status: "satisfied",
                task_id: Some("task-1"),
                task_name: Some("/root/worker"),
                result_required: true,
                deadline_at_ms: Some(1_700_000_000_000),
                trace_id: Some("trace-1"),
            })
            .await?;

        let events = runtime.list_inter_agent_mailbox_events(thread_id).await?;
        assert_eq!(
            events
                .iter()
                .map(|event| event.event_type.as_str())
                .collect::<Vec<_>>(),
            vec![
                "inter_agent_message_queued",
                "inter_agent_message_delivered",
                "wait_barrier_opened",
                "wait_barrier_satisfied",
            ]
        );
        assert!(
            events
                .iter()
                .all(|event| !event.live_blocking_enabled && !event.live_cutover_enabled)
        );
        assert_eq!(
            events[0]
                .content_json
                .as_ref()
                .and_then(|value| value.get("content")),
            Some(&serde_json::json!("done"))
        );

        let projection = runtime
            .get_inter_agent_mailbox_projection(thread_id)
            .await?;
        assert_eq!(
            projection,
            InterAgentMailboxProjection {
                thread_id,
                total_events: 4,
                queued_events: 1,
                delivered_events: 1,
                barrier_opened_events: 1,
                barrier_satisfied_events: 1,
                barrier_timed_out_events: 0,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            }
        );
        Ok(())
    }

    #[tokio::test]
    async fn wait_task_result_readback_tracks_delivery_and_reducer_shadow_events()
    -> anyhow::Result<()> {
        let codex_home = unique_temp_dir();
        let runtime = StateRuntime::init(codex_home, "test-provider".to_string()).await?;
        let thread_id = test_thread_id();

        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type: "wait_task_result_delivery_shadow",
                    status: "task_result_delivery_recorded_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "shadowDeliveryReady": true,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type: "wait_work_graph_canonical_projection_audit_chain_closeout_receipt",
                    status: "work_graph_canonical_projection_audit_chain_closeout_recorded_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "auditChainCloseoutReady": true,
                        "noCutoverTerminalReceipt": true,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type: "wait_parent_reducer_shadow_receipt",
                    status: "parent_reducer_shadow_receipt_recorded_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "parentReducerReceiptReady": true,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type: "wait_task_result_replay_consistency",
                    status: "wait_task_result_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type: "wait_surface_audit_packet",
                    status: "wait_task_result_surface_audit_recorded_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "auditPacketReady": true,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type: "wait_surface_audit_replay_consistency",
                    status: "wait_surface_audit_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type: "wait_work_graph_canonical_projection_receipt",
                    status: "work_graph_canonical_projection_recorded_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "projectionReceiptReady": true,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type: "wait_work_graph_canonical_projection_replay_consistency",
                    status: "work_graph_canonical_projection_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type: "wait_work_graph_canonical_projection_closeout_receipt",
                    status: "work_graph_canonical_projection_closeout_recorded_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "closeoutReady": true,
                        "noCutoverTerminalReceipt": true,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type: "wait_work_graph_canonical_projection_closeout_replay_consistency",
                    status: "work_graph_canonical_projection_closeout_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
                    status: "work_graph_canonical_projection_audit_chain_closeout_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_operator_review_packet",
                    status: "work_graph_canonical_projection_enablement_operator_review_ready_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "enablementOperatorReviewReady": true,
                        "noLiveEnablementRehearsalReady": true,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
                    status: "work_graph_canonical_projection_enablement_operator_review_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
                    status: "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_recorded_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "noLiveEnablementRehearsalCloseoutReady": true,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
                    status: "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
                    status: "work_graph_canonical_projection_enablement_audit_chain_closeout_recorded_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "enablementAuditChainCloseoutReady": true,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
                    status: "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
                    status: "work_graph_canonical_projection_enablement_activation_precondition_ready_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "activationPreconditionReady": true,
                        "activationAllowed": false,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
                    status: "work_graph_canonical_projection_enablement_activation_precondition_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "activationAllowed": false,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
                    status: "work_graph_canonical_projection_enablement_activation_no_live_closeout_recorded_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "activationNoLiveCloseoutReady": true,
                        "activationPreconditionReady": true,
                        "activationPreconditionReplayConsistent": true,
                        "activationAllowed": false,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
                    status: "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "activationNoLiveCloseoutReady": true,
                        "activationAllowed": false,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
                    status: "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_recorded_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "activationAuditChainCloseoutReady": true,
                        "activationNoLiveCloseoutReady": true,
                        "activationNoLiveCloseoutReplayConsistent": true,
                        "activationAllowed": false,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
                    status: "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "activationAuditChainCloseoutReady": true,
                        "activationAllowed": false,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "reviewedFlagEnabled": false,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
                    status: "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "activationOperatorApprovalReadinessPreflightReady": true,
                        "activationAllowed": false,
                        "enablementAllowed": false,
                        "operatorApprovalRequiredBeforeActivation": true,
                        "operatorApprovalRecorded": false,
                        "approvalRecordMutationEnabled": false,
                        "reviewedFlagRequiredBeforeActivation": true,
                        "reviewedFlagEnabled": false,
                        "reviewedFlagMutationEnabled": false,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency",
                    status: "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "activationOperatorApprovalReadinessPreflightReady": true,
                        "activationAllowed": false,
                        "enablementAllowed": false,
                        "operatorApprovalRequiredBeforeActivation": true,
                        "operatorApprovalRecorded": false,
                        "approvalRecordMutationEnabled": false,
                        "reviewedFlagRequiredBeforeActivation": true,
                        "reviewedFlagEnabled": false,
                        "reviewedFlagMutationEnabled": false,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet",
                    status: "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_recorded_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "approvalReviewSideEffectLockCloseoutReady": true,
                        "activationOperatorApprovalReadinessPreflightReady": true,
                        "activationOperatorApprovalReadinessPreflightReplayConsistent": true,
                        "activationOperatorApprovalReadinessPreflightPacketMatchesReadback": true,
                        "activationAllowed": false,
                        "enablementAllowed": false,
                        "operatorApprovalRequiredBeforeActivation": true,
                        "operatorApprovalRecorded": false,
                        "approvalRecordRequiredBeforeActivation": true,
                        "approvalRecordMutationEnabled": false,
                        "reviewedFlagRequiredBeforeActivation": true,
                        "reviewedFlagEnabled": false,
                        "reviewedFlagMutationEnabled": false,
                        "approvalReviewSideEffectsLocked": true,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;
        runtime
            .record_inter_agent_wait_work_graph_shadow_event(
                InterAgentWaitWorkGraphShadowEventParams {
                    thread_id,
                    barrier_id: "barrier-task-result",
                    task_id: Some("task-1"),
                    task_name: Some("worker"),
                    event_type:
                        "wait_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency",
                    status: "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent_shadow_no_live_cutover",
                    payload_json: serde_json::json!({
                        "replayConsistent": true,
                        "approvalReviewSideEffectLockCloseoutReady": true,
                        "activationApprovalReviewSideEffectLockCloseoutPacketMatchesReadback": true,
                        "approvalReviewSideEffectsLocked": true,
                        "activationAllowed": false,
                        "enablementAllowed": false,
                        "operatorApprovalRecorded": false,
                        "approvalRecordMutationEnabled": false,
                        "reviewedFlagEnabled": false,
                        "reviewedFlagMutationEnabled": false,
                        "canonicalWriteEnabled": false,
                        "liveBlockingEnabled": false,
                        "liveCutoverEnabled": false,
                    }),
                    trace_id: Some("trace-1"),
                },
            )
            .await?;

        let readback = runtime
            .get_inter_agent_wait_task_result_readback(thread_id, "barrier-task-result")
            .await?;
        assert_eq!(readback.task_result_delivery_shadow_events, 1);
        assert_eq!(readback.parent_reducer_shadow_receipt_events, 1);
        assert_eq!(readback.task_result_replay_consistency_events, 1);
        assert_eq!(readback.wait_surface_audit_packet_events, 1);
        assert_eq!(readback.wait_surface_audit_replay_consistency_events, 1);
        assert_eq!(readback.wait_canonical_projection_receipt_events, 1);
        assert_eq!(
            readback.wait_canonical_projection_replay_consistency_events,
            1
        );
        assert_eq!(
            readback.wait_canonical_projection_closeout_receipt_events,
            1
        );
        assert_eq!(
            readback.wait_canonical_projection_closeout_replay_consistency_events,
            1
        );
        assert_eq!(
            readback.wait_canonical_projection_audit_chain_closeout_receipt_events,
            1
        );
        assert_eq!(
            readback.wait_canonical_projection_audit_chain_closeout_replay_consistency_events,
            1
        );
        assert_eq!(
            readback.wait_canonical_projection_enablement_operator_review_packet_events,
            1
        );
        assert_eq!(
            readback.wait_canonical_projection_enablement_operator_review_replay_consistency_events,
            1
        );
        assert_eq!(
            readback.wait_canonical_projection_enablement_no_live_rehearsal_closeout_events,
            1
        );
        assert_eq!(
            readback
                .wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_events,
            1
        );
        assert_eq!(
            readback.wait_canonical_projection_enablement_audit_chain_closeout_events,
            1
        );
        assert_eq!(
            readback
                .wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_events,
            1
        );
        assert_eq!(
            readback
                .wait_canonical_projection_enablement_activation_precondition_operator_packet_events,
            1
        );
        assert_eq!(
            readback
                .wait_canonical_projection_enablement_activation_precondition_replay_consistency_events,
            1
        );
        assert_eq!(
            readback.wait_canonical_projection_enablement_activation_no_live_closeout_events,
            1
        );
        assert_eq!(
            readback
                .wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_events,
            1
        );
        assert_eq!(
            readback.wait_canonical_projection_enablement_activation_audit_chain_closeout_events,
            1
        );
        assert_eq!(
            readback
                .wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_events,
            1
        );
        assert_eq!(
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_events,
            1
        );
        assert_eq!(
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_events,
            1
        );
        assert_eq!(
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_events,
            1
        );
        assert_eq!(
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_events,
            1
        );
        assert_eq!(
            readback.latest_task_result_delivery_decision,
            "task_result_delivery_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            readback.latest_parent_reducer_decision,
            "parent_reducer_shadow_receipt_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            readback.latest_task_result_replay_consistency_decision,
            "wait_task_result_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback.latest_wait_surface_audit_decision,
            "wait_task_result_surface_audit_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            readback.latest_wait_surface_audit_replay_consistency_decision,
            "wait_surface_audit_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback.latest_wait_canonical_projection_decision,
            "work_graph_canonical_projection_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            readback.latest_wait_canonical_projection_replay_consistency_decision,
            "work_graph_canonical_projection_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback.latest_wait_canonical_projection_closeout_decision,
            "work_graph_canonical_projection_closeout_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            readback.latest_wait_canonical_projection_closeout_replay_consistency_decision,
            "work_graph_canonical_projection_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback.latest_wait_canonical_projection_audit_chain_closeout_decision,
            "work_graph_canonical_projection_audit_chain_closeout_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_audit_chain_closeout_replay_consistency_decision,
            "work_graph_canonical_projection_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback.latest_wait_canonical_projection_enablement_operator_review_decision,
            "work_graph_canonical_projection_enablement_operator_review_ready_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_operator_review_replay_consistency_decision,
            "work_graph_canonical_projection_enablement_operator_review_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_decision,
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision,
            "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback.latest_wait_canonical_projection_enablement_audit_chain_closeout_decision,
            "work_graph_canonical_projection_enablement_audit_chain_closeout_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision,
            "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_activation_precondition_operator_decision,
            "work_graph_canonical_projection_enablement_activation_precondition_ready_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_activation_precondition_replay_consistency_decision,
            "work_graph_canonical_projection_enablement_activation_precondition_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_activation_no_live_closeout_decision,
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision,
            "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_decision,
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision,
            "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_decision,
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision,
            "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_decision,
            "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_recorded_shadow_no_live_cutover"
        );
        assert_eq!(
            readback
                .latest_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_decision,
            "work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent_shadow_no_live_cutover"
        );
        assert!(readback.task_result_delivery_readback_ready);
        assert!(readback.parent_reducer_readback_ready);
        assert!(readback.replay_consistency_ready);
        assert!(readback.wait_surface_audit_packet_readback_ready);
        assert!(readback.wait_surface_audit_replay_consistency_ready);
        assert!(readback.wait_canonical_projection_receipt_readback_ready);
        assert!(readback.wait_canonical_projection_replay_consistency_ready);
        assert!(readback.wait_canonical_projection_closeout_receipt_readback_ready);
        assert!(readback.wait_canonical_projection_closeout_replay_consistency_ready);
        assert!(readback.wait_canonical_projection_audit_chain_closeout_receipt_readback_ready);
        assert!(readback.wait_canonical_projection_audit_chain_closeout_replay_consistency_ready);
        assert!(
            readback.wait_canonical_projection_enablement_operator_review_packet_readback_ready
        );
        assert!(
            readback.wait_canonical_projection_enablement_operator_review_replay_consistency_ready
        );
        assert!(
            readback.wait_canonical_projection_enablement_no_live_rehearsal_closeout_readback_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_ready
        );
        assert!(readback.wait_canonical_projection_enablement_audit_chain_closeout_readback_ready);
        assert!(
            readback
                .wait_canonical_projection_enablement_audit_chain_closeout_replay_consistency_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_precondition_operator_packet_readback_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_precondition_replay_consistency_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_no_live_closeout_readback_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_audit_chain_closeout_readback_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_readback_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_readback_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_ready
        );
        assert!(readback.task_result_delivery_ready);
        assert!(readback.parent_reducer_receipt_ready);
        assert!(readback.replay_consistent);
        assert!(readback.wait_surface_audit_packet_ready);
        assert!(readback.wait_surface_audit_replay_consistent);
        assert!(readback.wait_canonical_projection_receipt_ready);
        assert!(readback.wait_canonical_projection_replay_consistent);
        assert!(readback.wait_canonical_projection_closeout_receipt_ready);
        assert!(readback.wait_canonical_projection_closeout_replay_consistent);
        assert!(readback.wait_canonical_projection_audit_chain_closeout_receipt_ready);
        assert!(readback.wait_canonical_projection_audit_chain_closeout_replay_consistent);
        assert!(readback.wait_canonical_projection_enablement_operator_review_ready);
        assert!(readback.wait_canonical_projection_enablement_operator_review_replay_consistent);
        assert!(readback.wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready);
        assert!(
            readback
                .wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent
        );
        assert!(readback.wait_canonical_projection_enablement_audit_chain_closeout_ready);
        assert!(
            readback.wait_canonical_projection_enablement_audit_chain_closeout_replay_consistent
        );
        assert!(readback.wait_canonical_projection_enablement_activation_precondition_ready);
        assert!(
            readback.wait_canonical_projection_enablement_activation_precondition_replay_consistent
        );
        assert!(readback.wait_canonical_projection_enablement_activation_no_live_closeout_ready);
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_no_live_closeout_replay_consistent
        );
        assert!(
            readback.wait_canonical_projection_enablement_activation_audit_chain_closeout_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready
        );
        assert!(
            readback
                .wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistent
        );
        assert!(readback.no_live_guardrails_ready);
        assert!(readback.readback_ready);
        assert!(readback.direct_wait_task_result_ready);
        assert!(readback.direct_wait_surface_audit_ready);
        assert!(readback.direct_wait_canonical_projection_ready);
        assert!(readback.direct_wait_canonical_projection_closeout_ready);
        assert!(readback.direct_wait_canonical_projection_audit_chain_closeout_ready);
        assert!(readback.direct_wait_canonical_projection_audit_chain_closeout_replay_ready);
        assert!(readback.direct_wait_canonical_projection_enablement_operator_review_ready);
        assert!(
            readback.direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_ready
        );
        assert!(
            readback
                .direct_wait_canonical_projection_enablement_no_live_rehearsal_closeout_replay_ready
        );
        assert!(readback.direct_wait_canonical_projection_enablement_audit_chain_closeout_ready);
        assert!(
            readback.direct_wait_canonical_projection_enablement_audit_chain_closeout_replay_ready
        );
        assert!(readback.direct_wait_canonical_projection_enablement_activation_precondition_ready);
        assert!(
            readback
                .direct_wait_canonical_projection_enablement_activation_precondition_replay_ready
        );
        assert!(
            readback.direct_wait_canonical_projection_enablement_activation_no_live_closeout_ready
        );
        assert!(
            readback
                .direct_wait_canonical_projection_enablement_activation_no_live_closeout_replay_ready
        );
        assert!(
            readback
                .direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_ready
        );
        assert!(
            readback
                .direct_wait_canonical_projection_enablement_activation_audit_chain_closeout_replay_ready
        );
        assert!(
            readback
                .direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready
        );
        assert!(
            readback
                .direct_wait_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_ready
        );
        assert!(
            readback
                .direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_ready
        );
        assert!(
            readback
                .direct_wait_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_ready
        );
        Ok(())
    }
}
