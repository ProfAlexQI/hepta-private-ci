use super::*;
use serde_json::json;

pub(super) struct AgentJobWorkGraphShadowEventInsert<'a> {
    pub(super) job_id: &'a str,
    pub(super) item_id: Option<&'a str>,
    pub(super) event_type: &'static str,
    pub(super) task_id: String,
    pub(super) status: &'static str,
    pub(super) summary: String,
    pub(super) payload_json: Value,
    pub(super) trace_id: Option<&'a str>,
    pub(super) span_id: String,
}

pub(super) struct AgentJobWorkGraphShadowEventSpec {
    pub(super) event_type: &'static str,
    pub(super) status: &'static str,
    pub(super) summary: &'static str,
    pub(super) action: &'static str,
}

impl StateRuntime {
    pub async fn append_agent_job_admission_shadow_decision(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_admission_shadow_decision",
            task_id: task_id.to_string(),
            status: decision,
            summary: "agent job admission shadow decision recorded without live blocking"
                .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "admission-shadow"),
        })
        .await
    }

    pub async fn append_agent_job_promotion_readiness_matrix_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_promotion_readiness_matrix",
            task_id: task_id.to_string(),
            status: decision,
            summary: "agent job promotion readiness matrix recorded without live cutover"
                .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "promotion-matrix-shadow"),
        })
        .await
    }

    pub async fn append_agent_job_operator_review_promotion_packet_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_operator_review_promotion_packet",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job operator review promotion packet recorded without approval mutation"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "operator-review-promotion"),
        })
        .await
    }

    pub async fn append_agent_job_promotion_review_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_promotion_review_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job promotion review replay consistency recorded without reviewed flag mutation"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "promotion-review-replay"),
        })
        .await
    }

    pub async fn append_agent_job_promotion_closeout_receipt_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_promotion_closeout_receipt",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job terminal no-cutover promotion closeout receipt recorded without promotion"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "promotion-closeout-receipt"),
        })
        .await
    }

    pub async fn append_agent_job_promotion_closeout_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_promotion_closeout_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary: "agent job closeout receipt replay consistency recorded without live cutover"
                .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "promotion-closeout-replay"),
        })
        .await
    }

    pub async fn append_agent_job_promotion_review_audit_chain_receipt_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_promotion_review_audit_chain_receipt",
            task_id: task_id.to_string(),
            status: decision,
            summary: "agent job promotion review audit chain receipt recorded without live cutover"
                .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "promotion-review-audit-chain"),
        })
        .await
    }

    pub async fn append_agent_job_reviewed_flag_precondition_plan_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_reviewed_flag_precondition_plan",
            task_id: task_id.to_string(),
            status: decision,
            summary: "agent job reviewed flag precondition plan recorded without mutation"
                .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "reviewed-flag-plan"),
        })
        .await
    }

    pub async fn append_agent_job_reviewed_flag_precondition_plan_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_reviewed_flag_precondition_plan_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job reviewed flag precondition plan replay consistency recorded without mutation"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "reviewed-flag-plan-replay"),
        })
        .await
    }

    pub async fn append_agent_job_reviewed_flag_readiness_closeout_receipt_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_reviewed_flag_readiness_closeout_receipt",
            task_id: task_id.to_string(),
            status: decision,
            summary: "agent job reviewed flag readiness closeout recorded without mutation"
                .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "reviewed-flag-closeout"),
        })
        .await
    }

    pub async fn append_agent_job_reviewed_flag_readiness_closeout_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_reviewed_flag_readiness_closeout_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job reviewed flag readiness closeout replay consistency recorded without mutation"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "reviewed-flag-closeout-replay"),
        })
        .await
    }

    pub async fn append_agent_job_reviewed_flag_audit_chain_closeout_receipt_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_reviewed_flag_audit_chain_closeout_receipt",
            task_id: task_id.to_string(),
            status: decision,
            summary: "agent job reviewed flag audit-chain closeout recorded without mutation"
                .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "reviewed-flag-audit-closeout"),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_surface_audit_packet_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_work_graph_surface_audit_packet",
            task_id: task_id.to_string(),
            status: decision,
            summary: "agent job WorkGraph surface audit packet recorded without live cutover"
                .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "work-graph-surface-audit"),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_receipt_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_work_graph_canonical_projection_receipt",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection receipt recorded without canonical writes"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(job_id, None, "work-graph-canonical-projection"),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_work_graph_canonical_projection_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection replay consistency recorded without canonical writes"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-replay",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_closeout_receipt_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_work_graph_canonical_projection_closeout_receipt",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection closeout recorded without canonical writes"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-closeout",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_closeout_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_work_graph_canonical_projection_closeout_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection closeout replay consistency recorded without canonical writes"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-closeout-replay",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_work_graph_canonical_projection_audit_chain_closeout_receipt",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection audit-chain closeout recorded without canonical writes"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-audit-closeout",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_audit_chain_closeout_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection audit-chain closeout replay consistency recorded without canonical writes"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-audit-closeout-replay",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_operator_review_packet_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: "agent_job_work_graph_canonical_projection_enablement_operator_review_packet",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement operator-review packet recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-review",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_operator_review_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement operator-review replay consistency recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-review-replay",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement no-live rehearsal closeout recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-no-live-closeout",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement no-live rehearsal closeout replay consistency recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-no-live-closeout-replay",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement audit-chain closeout recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-audit-chain-closeout",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement audit-chain closeout replay consistency recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-audit-chain-closeout-replay",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_activation_precondition_operator_packet",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement activation precondition packet recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-activation-precondition",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement activation precondition replay consistency recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-activation-precondition-replay",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement activation no-live closeout recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-activation-no-live-closeout",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement activation no-live closeout replay consistency recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-activation-no-live-closeout-replay",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement activation audit-chain closeout recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-activation-audit-chain-closeout",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement activation audit-chain closeout replay consistency recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-activation-audit-chain-closeout-replay",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-activation-operator-approval-readiness-preflight",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement activation operator-approval readiness preflight replay consistency recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-activation-operator-approval-readiness-preflight-replay",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_packet",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-activation-approval-review-side-effect-lock-closeout",
            ),
        })
        .await
    }

    pub async fn append_agent_job_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency_shadow(
        &self,
        job_id: &str,
        task_id: &str,
        decision: &'static str,
        payload_json: Value,
        trace_id: Option<&str>,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type:
                "agent_job_work_graph_canonical_projection_enablement_activation_approval_review_side_effect_lock_closeout_replay_consistency",
            task_id: task_id.to_string(),
            status: decision,
            summary:
                "agent job canonical WorkGraph projection enablement activation approval/review side-effect lock closeout replay consistency recorded without approval or cutover"
                    .to_string(),
            payload_json,
            trace_id,
            span_id: agent_job_work_graph_span_id(
                job_id,
                None,
                "work-graph-canonical-projection-enablement-activation-approval-review-side-effect-lock-closeout-replay",
            ),
        })
        .await
    }

    pub(super) async fn append_agent_job_status_shadow_event(
        &self,
        job_id: &str,
        spec: AgentJobWorkGraphShadowEventSpec,
        payload_json: Value,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: None,
            event_type: spec.event_type,
            task_id: agent_job_work_graph_task_id(job_id),
            status: spec.status,
            summary: spec.summary.to_string(),
            payload_json,
            trace_id: None,
            span_id: agent_job_work_graph_span_id(job_id, None, spec.action),
        })
        .await
    }

    pub(super) async fn append_agent_job_item_status_shadow_event(
        &self,
        job_id: &str,
        item_id: &str,
        spec: AgentJobWorkGraphShadowEventSpec,
        payload_json: Value,
    ) -> anyhow::Result<()> {
        self.append_agent_job_work_graph_shadow_event(AgentJobWorkGraphShadowEventInsert {
            job_id,
            item_id: Some(item_id),
            event_type: spec.event_type,
            task_id: agent_job_work_graph_item_task_id(job_id, item_id),
            status: spec.status,
            summary: spec.summary.to_string(),
            payload_json,
            trace_id: None,
            span_id: agent_job_work_graph_span_id(job_id, Some(item_id), spec.action),
        })
        .await
    }

    pub(super) async fn append_agent_job_work_graph_shadow_event(
        &self,
        event: AgentJobWorkGraphShadowEventInsert<'_>,
    ) -> anyhow::Result<()> {
        let now = Utc::now().timestamp();
        let payload_json = serde_json::to_string(&event.payload_json)?;
        sqlx::query(
            r#"
INSERT INTO agent_job_work_graph_shadow_events (
    job_id,
    item_id,
    event_type,
    task_id,
    status,
    summary,
    payload_json,
    trace_id,
    span_id,
    source_surface_id,
    created_at,
    live_blocking_enabled,
    live_cutover_enabled
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0)
            "#,
        )
        .bind(event.job_id)
        .bind(event.item_id)
        .bind(event.event_type)
        .bind(event.task_id)
        .bind(event.status)
        .bind(event.summary)
        .bind(payload_json)
        .bind(event.trace_id)
        .bind(event.span_id)
        .bind("agent_jobs_state_runtime")
        .bind(now)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn list_agent_job_work_graph_shadow_events(
        &self,
        job_id: &str,
    ) -> anyhow::Result<Vec<AgentJobWorkGraphShadowEvent>> {
        let rows: Vec<AgentJobWorkGraphShadowEventRow> =
            sqlx::query_as::<_, AgentJobWorkGraphShadowEventRow>(
                r#"
SELECT
    sequence_id,
    job_id,
    item_id,
    event_type,
    task_id,
    status,
    summary,
    payload_json,
    trace_id,
    span_id,
    source_surface_id,
    created_at,
    live_blocking_enabled,
    live_cutover_enabled
FROM agent_job_work_graph_shadow_events
WHERE job_id = ?
ORDER BY sequence_id ASC
            "#,
            )
            .bind(job_id)
            .fetch_all(self.pool.as_ref())
            .await?;
        rows.into_iter()
            .map(AgentJobWorkGraphShadowEvent::try_from)
            .collect()
    }

    pub async fn get_agent_job_task_result_envelope_by_task_id(
        &self,
        task_id: &str,
    ) -> anyhow::Result<Option<Value>> {
        let payloads: Vec<String> = sqlx::query_scalar(
            r#"
SELECT payload_json
FROM agent_job_work_graph_shadow_events
WHERE
    task_id = ?
    AND event_type = 'agent_job_item_result_accepted'
    AND status = 'completed'
ORDER BY sequence_id DESC
LIMIT 8
            "#,
        )
        .bind(task_id)
        .fetch_all(self.pool.as_ref())
        .await?;

        for payload in payloads {
            let payload_json: Value = serde_json::from_str(&payload)?;
            if let Some(envelope) = payload_json.get("taskResultEnvelope") {
                return Ok(Some(envelope.clone()));
            }
        }

        Ok(None)
    }

    pub async fn get_agent_job_work_graph_shadow_projection(
        &self,
        job_id: &str,
    ) -> anyhow::Result<AgentJobWorkGraphShadowProjection> {
        let row = sqlx::query(
            r#"
SELECT
    COUNT(*) AS total_events,
    COUNT(DISTINCT task_id) AS distinct_tasks,
    MAX(sequence_id) AS latest_sequence_id,
    SUM(CASE WHEN event_type = 'agent_job_item_started' THEN 1 ELSE 0 END) AS item_started_events,
    SUM(CASE WHEN event_type IN ('agent_job_item_result_accepted', 'agent_job_item_completed') THEN 1 ELSE 0 END) AS item_completed_events,
    SUM(CASE WHEN event_type = 'agent_job_item_failed' THEN 1 ELSE 0 END) AS item_failed_events,
    SUM(CASE WHEN event_type IN ('agent_job_completed', 'agent_job_failed', 'agent_job_cancelled') THEN 1 ELSE 0 END) AS job_terminal_events,
    SUM(CASE WHEN live_blocking_enabled != 0 THEN 1 ELSE 0 END) AS live_blocking_event_count,
    SUM(CASE WHEN live_cutover_enabled != 0 THEN 1 ELSE 0 END) AS live_cutover_event_count
FROM agent_job_work_graph_shadow_events
WHERE job_id = ?
            "#,
        )
        .bind(job_id)
        .fetch_one(self.pool.as_ref())
        .await?;

        let total_events: i64 = row.try_get("total_events")?;
        let distinct_tasks: i64 = row.try_get("distinct_tasks")?;
        let latest_sequence_id: Option<i64> = row.try_get("latest_sequence_id")?;
        let item_started_events: Option<i64> = row.try_get("item_started_events")?;
        let item_completed_events: Option<i64> = row.try_get("item_completed_events")?;
        let item_failed_events: Option<i64> = row.try_get("item_failed_events")?;
        let job_terminal_events: Option<i64> = row.try_get("job_terminal_events")?;
        let live_blocking_event_count: Option<i64> = row.try_get("live_blocking_event_count")?;
        let live_cutover_event_count: Option<i64> = row.try_get("live_cutover_event_count")?;

        Ok(AgentJobWorkGraphShadowProjection {
            job_id: job_id.to_string(),
            total_events: sql_count_to_usize(total_events),
            distinct_tasks: sql_count_to_usize(distinct_tasks),
            latest_sequence_id,
            item_started_events: optional_sql_count_to_usize(item_started_events),
            item_completed_events: optional_sql_count_to_usize(item_completed_events),
            item_failed_events: optional_sql_count_to_usize(item_failed_events),
            job_terminal_events: optional_sql_count_to_usize(job_terminal_events),
            live_blocking_event_count: optional_sql_count_to_usize(live_blocking_event_count),
            live_cutover_event_count: optional_sql_count_to_usize(live_cutover_event_count),
        })
    }

    pub async fn get_agent_job_work_graph_shadow_projection_diff(
        &self,
        job_id: &str,
    ) -> anyhow::Result<AgentJobWorkGraphShadowProjectionDiff> {
        let progress = self.get_agent_job_progress(job_id).await?;
        let projection = self
            .get_agent_job_work_graph_shadow_projection(job_id)
            .await?;
        let completed_item_delta =
            count_delta(progress.completed_items, projection.item_completed_events);
        let failed_item_delta = count_delta(progress.failed_items, projection.item_failed_events);
        Ok(AgentJobWorkGraphShadowProjectionDiff {
            job_id: job_id.to_string(),
            progress,
            projection,
            completed_item_delta,
            failed_item_delta,
            projection_matches_items: completed_item_delta == 0 && failed_item_delta == 0,
        })
    }

    pub async fn get_agent_job_work_graph_promotion_review_readback(
        &self,
        job_id: &str,
    ) -> anyhow::Result<AgentJobWorkGraphPromotionReviewReadback> {
        let row = sqlx::query(
            r#"
SELECT
    SUM(CASE WHEN event_type = 'agent_job_admission_shadow_decision' THEN 1 ELSE 0 END) AS admission_shadow_decision_events,
    SUM(CASE WHEN event_type = 'agent_job_promotion_readiness_matrix' THEN 1 ELSE 0 END) AS promotion_readiness_matrix_events,
    SUM(CASE WHEN event_type = 'agent_job_operator_review_promotion_packet' THEN 1 ELSE 0 END) AS operator_review_promotion_packet_events,
    SUM(CASE WHEN event_type = 'agent_job_promotion_review_replay_consistency' THEN 1 ELSE 0 END) AS promotion_review_replay_consistency_events,
    SUM(CASE WHEN event_type = 'agent_job_promotion_closeout_receipt' THEN 1 ELSE 0 END) AS promotion_closeout_receipt_events,
    SUM(CASE WHEN event_type = 'agent_job_promotion_closeout_replay_consistency' THEN 1 ELSE 0 END) AS promotion_closeout_replay_consistency_events,
    SUM(CASE WHEN event_type = 'agent_job_promotion_review_audit_chain_receipt' THEN 1 ELSE 0 END) AS promotion_review_audit_chain_receipt_events,
    SUM(CASE WHEN event_type = 'agent_job_reviewed_flag_precondition_plan' THEN 1 ELSE 0 END) AS reviewed_flag_precondition_plan_events,
    SUM(CASE WHEN event_type = 'agent_job_reviewed_flag_precondition_plan_replay_consistency' THEN 1 ELSE 0 END) AS reviewed_flag_precondition_plan_replay_consistency_events,
    SUM(CASE WHEN event_type = 'agent_job_reviewed_flag_readiness_closeout_receipt' THEN 1 ELSE 0 END) AS reviewed_flag_readiness_closeout_receipt_events,
    SUM(CASE WHEN event_type = 'agent_job_reviewed_flag_readiness_closeout_replay_consistency' THEN 1 ELSE 0 END) AS reviewed_flag_readiness_closeout_replay_consistency_events,
    SUM(CASE WHEN event_type = 'agent_job_reviewed_flag_audit_chain_closeout_receipt' THEN 1 ELSE 0 END) AS reviewed_flag_audit_chain_closeout_receipt_events,
    SUM(CASE WHEN live_blocking_enabled != 0 THEN 1 ELSE 0 END) AS live_blocking_event_count,
    SUM(CASE WHEN live_cutover_enabled != 0 THEN 1 ELSE 0 END) AS live_cutover_event_count
FROM agent_job_work_graph_shadow_events
WHERE job_id = ?
            "#,
        )
        .bind(job_id)
        .fetch_one(self.pool.as_ref())
        .await?;

        let admission_shadow_decision_events: Option<i64> =
            row.try_get("admission_shadow_decision_events")?;
        let promotion_readiness_matrix_events: Option<i64> =
            row.try_get("promotion_readiness_matrix_events")?;
        let operator_review_promotion_packet_events: Option<i64> =
            row.try_get("operator_review_promotion_packet_events")?;
        let promotion_review_replay_consistency_events: Option<i64> =
            row.try_get("promotion_review_replay_consistency_events")?;
        let promotion_closeout_receipt_events: Option<i64> =
            row.try_get("promotion_closeout_receipt_events")?;
        let promotion_closeout_replay_consistency_events: Option<i64> =
            row.try_get("promotion_closeout_replay_consistency_events")?;
        let promotion_review_audit_chain_receipt_events: Option<i64> =
            row.try_get("promotion_review_audit_chain_receipt_events")?;
        let reviewed_flag_precondition_plan_events: Option<i64> =
            row.try_get("reviewed_flag_precondition_plan_events")?;
        let reviewed_flag_precondition_plan_replay_consistency_events: Option<i64> =
            row.try_get("reviewed_flag_precondition_plan_replay_consistency_events")?;
        let reviewed_flag_readiness_closeout_receipt_events: Option<i64> =
            row.try_get("reviewed_flag_readiness_closeout_receipt_events")?;
        let reviewed_flag_readiness_closeout_replay_consistency_events: Option<i64> =
            row.try_get("reviewed_flag_readiness_closeout_replay_consistency_events")?;
        let reviewed_flag_audit_chain_closeout_receipt_events: Option<i64> =
            row.try_get("reviewed_flag_audit_chain_closeout_receipt_events")?;
        let live_blocking_event_count: Option<i64> = row.try_get("live_blocking_event_count")?;
        let live_cutover_event_count: Option<i64> = row.try_get("live_cutover_event_count")?;

        let admission_shadow_decision_events =
            optional_sql_count_to_usize(admission_shadow_decision_events);
        let promotion_readiness_matrix_events =
            optional_sql_count_to_usize(promotion_readiness_matrix_events);
        let operator_review_promotion_packet_events =
            optional_sql_count_to_usize(operator_review_promotion_packet_events);
        let promotion_review_replay_consistency_events =
            optional_sql_count_to_usize(promotion_review_replay_consistency_events);
        let promotion_closeout_receipt_events =
            optional_sql_count_to_usize(promotion_closeout_receipt_events);
        let promotion_closeout_replay_consistency_events =
            optional_sql_count_to_usize(promotion_closeout_replay_consistency_events);
        let promotion_review_audit_chain_receipt_events =
            optional_sql_count_to_usize(promotion_review_audit_chain_receipt_events);
        let reviewed_flag_precondition_plan_events =
            optional_sql_count_to_usize(reviewed_flag_precondition_plan_events);
        let reviewed_flag_precondition_plan_replay_consistency_events =
            optional_sql_count_to_usize(reviewed_flag_precondition_plan_replay_consistency_events);
        let reviewed_flag_readiness_closeout_receipt_events =
            optional_sql_count_to_usize(reviewed_flag_readiness_closeout_receipt_events);
        let reviewed_flag_readiness_closeout_replay_consistency_events =
            optional_sql_count_to_usize(reviewed_flag_readiness_closeout_replay_consistency_events);
        let reviewed_flag_audit_chain_closeout_receipt_events =
            optional_sql_count_to_usize(reviewed_flag_audit_chain_closeout_receipt_events);
        let live_blocking_event_count = optional_sql_count_to_usize(live_blocking_event_count);
        let live_cutover_event_count = optional_sql_count_to_usize(live_cutover_event_count);
        let latest_admission_shadow_decision = self
            .latest_agent_job_shadow_event_payload(job_id, "agent_job_admission_shadow_decision")
            .await?;
        let latest_promotion_readiness_matrix = self
            .latest_agent_job_shadow_event_payload(job_id, "agent_job_promotion_readiness_matrix")
            .await?;
        let latest_operator_review_promotion_packet = self
            .latest_agent_job_shadow_event_payload(
                job_id,
                "agent_job_operator_review_promotion_packet",
            )
            .await?;
        let latest_promotion_review_replay_consistency = self
            .latest_agent_job_shadow_event_payload(
                job_id,
                "agent_job_promotion_review_replay_consistency",
            )
            .await?;
        let latest_promotion_closeout_receipt = self
            .latest_agent_job_shadow_event_payload(job_id, "agent_job_promotion_closeout_receipt")
            .await?;
        let latest_promotion_closeout_replay_consistency = self
            .latest_agent_job_shadow_event_payload(
                job_id,
                "agent_job_promotion_closeout_replay_consistency",
            )
            .await?;
        let latest_promotion_review_audit_chain_receipt = self
            .latest_agent_job_shadow_event_payload(
                job_id,
                "agent_job_promotion_review_audit_chain_receipt",
            )
            .await?;
        let latest_reviewed_flag_precondition_plan = self
            .latest_agent_job_shadow_event_payload(
                job_id,
                "agent_job_reviewed_flag_precondition_plan",
            )
            .await?;
        let latest_reviewed_flag_precondition_plan_replay_consistency = self
            .latest_agent_job_shadow_event_payload(
                job_id,
                "agent_job_reviewed_flag_precondition_plan_replay_consistency",
            )
            .await?;
        let latest_reviewed_flag_readiness_closeout_receipt = self
            .latest_agent_job_shadow_event_payload(
                job_id,
                "agent_job_reviewed_flag_readiness_closeout_receipt",
            )
            .await?;
        let latest_reviewed_flag_readiness_closeout_replay_consistency = self
            .latest_agent_job_shadow_event_payload(
                job_id,
                "agent_job_reviewed_flag_readiness_closeout_replay_consistency",
            )
            .await?;
        let latest_reviewed_flag_audit_chain_closeout_receipt = self
            .latest_agent_job_shadow_event_payload(
                job_id,
                "agent_job_reviewed_flag_audit_chain_closeout_receipt",
            )
            .await?;
        let readback_ready = admission_shadow_decision_events > 0
            && promotion_readiness_matrix_events > 0
            && operator_review_promotion_packet_events > 0
            && live_blocking_event_count == 0
            && live_cutover_event_count == 0
            && latest_admission_shadow_decision.is_some()
            && latest_promotion_readiness_matrix.is_some()
            && latest_operator_review_promotion_packet.is_some();
        let replay_consistency_ready = readback_ready
            && promotion_review_replay_consistency_events > 0
            && latest_promotion_review_replay_consistency.is_some();
        let closeout_receipt_ready = replay_consistency_ready
            && promotion_closeout_receipt_events > 0
            && latest_promotion_closeout_receipt.is_some();
        let closeout_replay_consistency_ready = closeout_receipt_ready
            && promotion_closeout_replay_consistency_events > 0
            && latest_promotion_closeout_replay_consistency.is_some();
        let audit_chain_receipt_ready = closeout_replay_consistency_ready
            && promotion_review_audit_chain_receipt_events > 0
            && latest_promotion_review_audit_chain_receipt.is_some();
        let reviewed_flag_precondition_plan_ready = audit_chain_receipt_ready
            && reviewed_flag_precondition_plan_events > 0
            && latest_reviewed_flag_precondition_plan.is_some();
        let reviewed_flag_precondition_plan_replay_consistency_ready =
            reviewed_flag_precondition_plan_ready
                && reviewed_flag_precondition_plan_replay_consistency_events > 0
                && latest_reviewed_flag_precondition_plan_replay_consistency.is_some();
        let reviewed_flag_readiness_closeout_receipt_ready =
            reviewed_flag_precondition_plan_replay_consistency_ready
                && reviewed_flag_readiness_closeout_receipt_events > 0
                && latest_reviewed_flag_readiness_closeout_receipt.is_some();
        let reviewed_flag_readiness_closeout_replay_consistency_ready =
            reviewed_flag_readiness_closeout_receipt_ready
                && reviewed_flag_readiness_closeout_replay_consistency_events > 0
                && latest_reviewed_flag_readiness_closeout_replay_consistency.is_some();
        let reviewed_flag_audit_chain_closeout_receipt_ready =
            reviewed_flag_readiness_closeout_replay_consistency_ready
                && reviewed_flag_audit_chain_closeout_receipt_events > 0
                && latest_reviewed_flag_audit_chain_closeout_receipt.is_some();

        Ok(AgentJobWorkGraphPromotionReviewReadback {
            job_id: job_id.to_string(),
            admission_shadow_decision_events,
            promotion_readiness_matrix_events,
            operator_review_promotion_packet_events,
            promotion_review_replay_consistency_events,
            promotion_closeout_receipt_events,
            promotion_closeout_replay_consistency_events,
            promotion_review_audit_chain_receipt_events,
            reviewed_flag_precondition_plan_events,
            reviewed_flag_precondition_plan_replay_consistency_events,
            reviewed_flag_readiness_closeout_receipt_events,
            reviewed_flag_readiness_closeout_replay_consistency_events,
            reviewed_flag_audit_chain_closeout_receipt_events,
            live_blocking_event_count,
            live_cutover_event_count,
            latest_admission_shadow_decision,
            latest_promotion_readiness_matrix,
            latest_operator_review_promotion_packet,
            latest_promotion_review_replay_consistency,
            latest_promotion_closeout_receipt,
            latest_promotion_closeout_replay_consistency,
            latest_promotion_review_audit_chain_receipt,
            latest_reviewed_flag_precondition_plan,
            latest_reviewed_flag_precondition_plan_replay_consistency,
            latest_reviewed_flag_readiness_closeout_receipt,
            latest_reviewed_flag_readiness_closeout_replay_consistency,
            latest_reviewed_flag_audit_chain_closeout_receipt,
            readback_ready,
            replay_consistency_ready,
            closeout_receipt_ready,
            closeout_replay_consistency_ready,
            audit_chain_receipt_ready,
            reviewed_flag_precondition_plan_ready,
            reviewed_flag_precondition_plan_replay_consistency_ready,
            reviewed_flag_readiness_closeout_receipt_ready,
            reviewed_flag_readiness_closeout_replay_consistency_ready,
            reviewed_flag_audit_chain_closeout_receipt_ready,
        })
    }

    pub async fn get_agent_job_work_graph_audit_chain_readback(
        &self,
        job_id: &str,
        segment_specs: &[AgentJobWorkGraphAuditChainSegmentSpec],
    ) -> anyhow::Result<AgentJobWorkGraphAuditChainReadback> {
        let row = sqlx::query(
            r#"
SELECT
    SUM(CASE WHEN live_blocking_enabled != 0 THEN 1 ELSE 0 END) AS live_blocking_event_count,
    SUM(CASE WHEN live_cutover_enabled != 0 THEN 1 ELSE 0 END) AS live_cutover_event_count
FROM agent_job_work_graph_shadow_events
WHERE job_id = ?
            "#,
        )
        .bind(job_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        let live_blocking_event_count: Option<i64> = row.try_get("live_blocking_event_count")?;
        let live_cutover_event_count: Option<i64> = row.try_get("live_cutover_event_count")?;
        let live_blocking_event_count = optional_sql_count_to_usize(live_blocking_event_count);
        let live_cutover_event_count = optional_sql_count_to_usize(live_cutover_event_count);
        let no_live_guardrails_ready =
            live_blocking_event_count == 0 && live_cutover_event_count == 0;
        let mut segments = Vec::with_capacity(segment_specs.len());
        for spec in segment_specs {
            let event_count: i64 = sqlx::query_scalar(
                r#"
SELECT COUNT(*)
FROM agent_job_work_graph_shadow_events
WHERE job_id = ? AND event_type = ?
                "#,
            )
            .bind(job_id)
            .bind(spec.event_type)
            .fetch_one(self.pool.as_ref())
            .await?;
            let event_count = sql_count_to_usize(event_count);
            let latest_payload = self
                .latest_agent_job_shadow_event_payload(job_id, spec.event_type)
                .await?;
            let latest_decision = latest_payload
                .as_ref()
                .and_then(|value| value.get("decision"))
                .and_then(Value::as_str)
                .unwrap_or("missing")
                .to_string();
            let readback_ready = event_count > 0 && latest_payload.is_some();
            let replay_consistent = spec.replay_consistency_field.is_none_or(|field| {
                latest_payload
                    .as_ref()
                    .and_then(|value| value.get(field))
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
            });
            let ready = readback_ready && replay_consistent && no_live_guardrails_ready;
            segments.push(AgentJobWorkGraphAuditChainSegmentReadback {
                segment_id: spec.segment_id.to_string(),
                event_type: spec.event_type.to_string(),
                event_count,
                latest_payload,
                latest_decision,
                readback_ready,
                replay_consistent,
                no_live_guardrail_ready: no_live_guardrails_ready,
                ready,
            });
        }
        let chain_readback_ready = segments.iter().all(|segment| segment.readback_ready);
        let chain_replay_consistent = segments.iter().all(|segment| segment.replay_consistent);
        let chain_ready =
            chain_readback_ready && chain_replay_consistent && no_live_guardrails_ready;

        Ok(AgentJobWorkGraphAuditChainReadback {
            job_id: job_id.to_string(),
            segments,
            live_blocking_event_count,
            live_cutover_event_count,
            chain_readback_ready,
            chain_replay_consistent,
            no_live_guardrails_ready,
            chain_ready,
        })
    }

    async fn latest_agent_job_shadow_event_payload(
        &self,
        job_id: &str,
        event_type: &str,
    ) -> anyhow::Result<Option<Value>> {
        let payload: Option<String> = sqlx::query_scalar(
            r#"
SELECT payload_json
FROM agent_job_work_graph_shadow_events
WHERE job_id = ? AND event_type = ?
ORDER BY sequence_id DESC
LIMIT 1
            "#,
        )
        .bind(job_id)
        .bind(event_type)
        .fetch_optional(self.pool.as_ref())
        .await?;

        payload
            .as_deref()
            .map(serde_json::from_str)
            .transpose()
            .map_err(Into::into)
    }
}

pub(super) async fn append_agent_job_created_shadow_events_tx(
    tx: &mut sqlx::Transaction<'_, Sqlite>,
    params: &AgentJobCreateParams,
    items: &[AgentJobItemCreateParams],
    created_at: i64,
) -> anyhow::Result<()> {
    append_agent_job_work_graph_shadow_event_tx(
        tx,
        AgentJobWorkGraphShadowEventInsert {
            job_id: params.id.as_str(),
            item_id: None,
            event_type: "agent_job_created",
            task_id: agent_job_work_graph_task_id(params.id.as_str()),
            status: AgentJobStatus::Pending.as_str(),
            summary: "agent job created in shadow WorkGraph event stream".to_string(),
            payload_json: json!({
                "autoExport": params.auto_export,
                "hasOutputSchema": params.output_schema_json.is_some(),
                "inputHeaderCount": params.input_headers.len(),
                "itemCount": items.len(),
                "maxRuntimeSeconds": params.max_runtime_seconds,
                "shadowOnly": true,
            }),
            trace_id: None,
            span_id: agent_job_work_graph_span_id(params.id.as_str(), None, "created"),
        },
        created_at,
    )
    .await?;

    for item in items {
        append_agent_job_work_graph_shadow_event_tx(
            tx,
            AgentJobWorkGraphShadowEventInsert {
                job_id: params.id.as_str(),
                item_id: Some(item.item_id.as_str()),
                event_type: "agent_job_item_created",
                task_id: agent_job_work_graph_item_task_id(
                    params.id.as_str(),
                    item.item_id.as_str(),
                ),
                status: AgentJobItemStatus::Pending.as_str(),
                summary: "agent job item created in shadow WorkGraph event stream".to_string(),
                payload_json: json!({
                    "hasSourceId": item.source_id.is_some(),
                    "rowIndex": item.row_index,
                    "shadowOnly": true,
                }),
                trace_id: None,
                span_id: agent_job_work_graph_span_id(
                    params.id.as_str(),
                    Some(item.item_id.as_str()),
                    "created",
                ),
            },
            created_at,
        )
        .await?;
    }
    Ok(())
}

pub(super) async fn append_agent_job_work_graph_shadow_event_tx(
    tx: &mut sqlx::Transaction<'_, Sqlite>,
    event: AgentJobWorkGraphShadowEventInsert<'_>,
    created_at: i64,
) -> anyhow::Result<()> {
    let payload_json = serde_json::to_string(&event.payload_json)?;
    sqlx::query(
        r#"
INSERT INTO agent_job_work_graph_shadow_events (
    job_id,
    item_id,
    event_type,
    task_id,
    status,
    summary,
    payload_json,
    trace_id,
    span_id,
    source_surface_id,
    created_at,
    live_blocking_enabled,
    live_cutover_enabled
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0)
            "#,
    )
    .bind(event.job_id)
    .bind(event.item_id)
    .bind(event.event_type)
    .bind(event.task_id)
    .bind(event.status)
    .bind(event.summary)
    .bind(payload_json)
    .bind(event.trace_id)
    .bind(event.span_id)
    .bind("agent_jobs_state_runtime")
    .bind(created_at)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub(super) fn agent_job_work_graph_task_id(job_id: &str) -> String {
    format!("agent-job:{job_id}")
}

pub(super) fn agent_job_work_graph_item_task_id(job_id: &str, item_id: &str) -> String {
    format!("agent-job:{job_id}:{item_id}")
}

pub(super) fn agent_job_work_graph_span_id(
    job_id: &str,
    item_id: Option<&str>,
    action: &str,
) -> String {
    match item_id {
        Some(item_id) => format!("span-agent-job-{job_id}-{item_id}-{action}"),
        None => format!("span-agent-job-{job_id}-{action}"),
    }
}

pub(super) fn json_value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn sql_count_to_usize(value: i64) -> usize {
    usize::try_from(value).unwrap_or_default()
}

fn optional_sql_count_to_usize(value: Option<i64>) -> usize {
    value.map(sql_count_to_usize).unwrap_or_default()
}

fn count_delta(current_count: usize, projection_count: usize) -> isize {
    let current_count = isize::try_from(current_count).unwrap_or(isize::MAX);
    let projection_count = isize::try_from(projection_count).unwrap_or(isize::MAX);
    current_count.saturating_sub(projection_count)
}
