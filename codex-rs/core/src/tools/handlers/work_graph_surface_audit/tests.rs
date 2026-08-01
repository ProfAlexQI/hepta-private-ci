use super::*;
use crate::tools::handlers::work_graph_admission::WorkGraphAdmissionShadowCheck;
use crate::tools::handlers::work_graph_promotion_readiness::WorkGraphPromotionReadinessShadowMatrix;
use serde_json::json;

fn ready_readback() -> codex_state::AgentJobWorkGraphAuditChainReadback {
    let segments = work_graph_surface_audit_chain_segment_specs()
        .iter()
        .map(|spec| {
            let latest_decision = format!("{}_recorded_shadow_no_live_cutover", spec.segment_id);
            codex_state::AgentJobWorkGraphAuditChainSegmentReadback {
                segment_id: spec.segment_id.to_string(),
                event_type: spec.event_type.to_string(),
                event_count: 1,
                latest_payload: Some(json!({
                    "decision": latest_decision,
                    "replayConsistent": true
                })),
                latest_decision,
                readback_ready: true,
                replay_consistent: true,
                no_live_guardrail_ready: true,
                ready: true,
            }
        })
        .collect::<Vec<_>>();
    codex_state::AgentJobWorkGraphAuditChainReadback {
        job_id: "job-1".to_string(),
        segments,
        live_blocking_event_count: 0,
        live_cutover_event_count: 0,
        chain_readback_ready: true,
        chain_replay_consistent: true,
        no_live_guardrails_ready: true,
        chain_ready: true,
    }
}

fn sample_matrix() -> WorkGraphPromotionReadinessShadowMatrix {
    WorkGraphPromotionReadinessShadowMatrix {
        decision: "promotion_matrix_not_ready_shadow_no_live_cutover",
        promotion_stage: "shadow_only",
        expected_source_surface_count: 8,
        observed_source_surface_count: 0,
        promotion_ready_count: 0,
        promotion_not_ready_count: 0,
        coverage_ready: false,
        all_promotion_ready: false,
        ready_source_surface_ids: Vec::new(),
        not_ready_source_surface_ids: Vec::new(),
        missing_source_surface_ids: Vec::new(),
        unexpected_source_surface_ids: Vec::new(),
        duplicate_source_surface_ids: Vec::new(),
        entries: Vec::new(),
        checks: vec![WorkGraphAdmissionShadowCheck {
            name: "sample",
            passed: true,
            detail: "sample".to_string(),
        }],
        feature_flag_id: "work_graph_promotion_readiness_matrix_shadow_only",
        feature_flag_enabled: false,
        canary_stage: "off",
        canary_traffic_ppm: 0,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

#[test]
fn surface_audit_records_generic_chain_without_live_cutover() {
    let matrix = sample_matrix();
    let readback = ready_readback();

    let packet = build_work_graph_surface_audit_packet(WorkGraphSurfaceAuditPacketInput {
        job_id: "job-1",
        promotion_readiness_shadow_matrix: &matrix,
        role_manifest_shadow_decisions: &[],
        audit_chain_readback: &readback,
    });

    assert_eq!(
        packet.decision,
        "work_graph_surface_audit_recorded_shadow_no_live_cutover"
    );
    assert!(packet.audit_packet_ready);
    assert!(packet.audit_chain.chain_ready);
    assert_eq!(work_graph_surface_audit_chain_segment_specs().len(), 12);
    assert_eq!(packet.audit_chain.segment_count, 12);
    assert_eq!(packet.audit_chain.ready_segment_count, 12);
    assert_eq!(packet.governed_source_surface_count, 8);
    assert_eq!(packet.planning_source_surface_count, 2);
    assert_eq!(packet.runtime_source_surface_count, 4);
    assert_eq!(packet.canonical_write_enabled_count, 0);
    assert!(packet.canonical_readiness_failed);
    assert!(!packet.feature_flag_enabled);
    assert!(!packet.promotion_allowed);
    assert!(!packet.live_cutover_enabled);
    assert!(
        packet
            .optimization_blockers
            .iter()
            .any(|blocker| blocker.contains("canonical WorkGraph nodes"))
    );
    let summary = summarize_work_graph_surface_audit_packet(&packet);
    assert_eq!(
        summary.operator_matrix_row_count,
        packet.source_surface_count
    );
    assert_eq!(
        summary.operator_matrix_blocked_row_count,
        packet.source_surface_count
    );
    assert_eq!(summary.operator_matrix_ready_row_count, 0);
    let direct_spawn_row = summary
        .operator_matrix_rows
        .iter()
        .find(|row| row.source_surface_id == "spawn_agent_v2")
        .expect("operator matrix should include direct subagent spawn row");
    assert_eq!(
        direct_spawn_row.readiness_status,
        "blocked_missing_result_contract"
    );
    assert_eq!(direct_spawn_row.next_blocker, "missing_result_contract");
    assert!(direct_spawn_row.row_auditable);
    assert!(!direct_spawn_row.canonical_promotion_ready);
    assert_eq!(
        direct_spawn_row
            .task_result_contract_plan_decision
            .as_deref(),
        Some("task_result_contract_plan_blocked_shadow_no_live_cutover")
    );
    assert_eq!(
        direct_spawn_row.task_result_contract_plan_ready,
        Some(false)
    );
    assert_eq!(
        direct_spawn_row.task_result_contract_id.as_deref(),
        Some("subagent_task_result_contract_v1")
    );
    assert_eq!(
        direct_spawn_row.terminal_delivery_surface.as_deref(),
        Some("wait_agent(result_required=true)")
    );
    assert_eq!(
        direct_spawn_row.missing_task_result_contract_parts,
        vec![
            "task_result_contract".to_string(),
            "verifier".to_string(),
            "reducer".to_string()
        ]
    );
    assert!(
        direct_spawn_row
            .task_result_contract_next_action
            .as_deref()
            .is_some_and(|action| action.contains("TaskResultEnvelope"))
    );
    assert_eq!(
        direct_spawn_row.task_result_contract_next_action_count,
        Some(3)
    );
    let agent_jobs_row = summary
        .operator_matrix_rows
        .iter()
        .find(|row| row.source_surface_id == "spawn_agents_on_csv")
        .expect("operator matrix should include agent job worker row");
    assert_eq!(agent_jobs_row.task_result_contract_plan_ready, Some(true));
    assert_eq!(
        agent_jobs_row.task_result_contract_id.as_deref(),
        Some("agent_job_task_result_contract_v1")
    );
    let projection_receipt = build_work_graph_canonical_projection_shadow_receipt(&summary);
    assert_eq!(
        projection_receipt.decision,
        "work_graph_canonical_projection_recorded_shadow_no_live_cutover"
    );
    assert!(projection_receipt.read_projection_ready);
    assert!(!projection_receipt.write_projection_ready);
    assert!(!projection_receipt.canonical_write_enabled);
    assert_eq!(projection_receipt.projected_work_node_count, 5);
    assert!(
        projection_receipt
            .projection_rows
            .iter()
            .any(|row| row.source_surface_id == "wait_agent" && row.node_kind == "wait_barrier")
    );
    let projection_receipt_payload =
        serde_json::to_value(&projection_receipt).expect("projection receipt serializes");
    let replay_decision = build_work_graph_canonical_projection_replay_consistency_decision(
        WorkGraphCanonicalProjectionReplayConsistencyInput {
            source_surface_id: "agent_jobs",
            projection_receipt: &projection_receipt,
            projection_receipt_payload: &projection_receipt_payload,
            latest_projection_receipt_payload: Some(&projection_receipt_payload),
            projection_receipt_events: 1,
            projection_receipt_readback_ready: true,
            prior_projection_replay_consistency_events: 0,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
        },
    );
    assert_eq!(
        replay_decision.decision,
        "work_graph_canonical_projection_replay_consistent_shadow_no_live_cutover"
    );
    assert!(replay_decision.replay_consistent);
    assert!(replay_decision.projection_receipt_matches_readback);
    assert!(!replay_decision.shadow_readiness_failed);
    let closeout_receipt = build_work_graph_canonical_projection_closeout_receipt(
        WorkGraphCanonicalProjectionCloseoutReceiptInput {
            source_surface_id: "agent_jobs",
            projection_receipt: &projection_receipt,
            replay_consistency_decision: &replay_decision,
            projection_receipt_events: 1,
            projection_replay_consistency_events: 1,
            prior_projection_closeout_receipt_events: 0,
            projection_receipt_readback_ready: true,
            projection_replay_consistency_ready: true,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
        },
    );
    assert_eq!(
        closeout_receipt.decision,
        "work_graph_canonical_projection_closeout_recorded_shadow_no_live_cutover"
    );
    assert!(closeout_receipt.closeout_ready);
    assert!(closeout_receipt.no_cutover_terminal_receipt);
    assert!(!closeout_receipt.shadow_readiness_failed);
    assert!(!closeout_receipt.canonical_write_enabled);
    assert!(!closeout_receipt.canonical_read_enabled);
    let closeout_receipt_payload =
        serde_json::to_value(&closeout_receipt).expect("closeout receipt serializes");
    let closeout_replay_decision =
        build_work_graph_canonical_projection_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionCloseoutReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                closeout_receipt: &closeout_receipt,
                closeout_receipt_payload: &closeout_receipt_payload,
                latest_closeout_receipt_payload: Some(&closeout_receipt_payload),
                closeout_receipt_events: 1,
                closeout_receipt_readback_ready: true,
                prior_closeout_replay_consistency_events: 0,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        closeout_replay_decision.decision,
        "work_graph_canonical_projection_closeout_replay_consistent_shadow_no_live_cutover"
    );
    assert!(closeout_replay_decision.replay_consistent);
    assert!(closeout_replay_decision.closeout_receipt_matches_readback);
    assert!(!closeout_replay_decision.shadow_readiness_failed);
    let audit_chain_closeout_receipt =
        build_work_graph_canonical_projection_audit_chain_closeout_receipt(
            WorkGraphCanonicalProjectionAuditChainCloseoutReceiptInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &projection_receipt,
                projection_replay_consistency_decision: &replay_decision,
                closeout_receipt: &closeout_receipt,
                closeout_replay_consistency_decision: &closeout_replay_decision,
                projection_receipt_events: 1,
                projection_replay_consistency_events: 1,
                closeout_receipt_events: 1,
                closeout_replay_consistency_events: 1,
                prior_audit_chain_closeout_receipt_events: 0,
                projection_receipt_readback_ready: true,
                projection_replay_consistency_ready: true,
                closeout_receipt_readback_ready: true,
                closeout_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        audit_chain_closeout_receipt.decision,
        "work_graph_canonical_projection_audit_chain_closeout_recorded_shadow_no_live_cutover"
    );
    assert!(audit_chain_closeout_receipt.audit_chain_closeout_ready);
    assert!(audit_chain_closeout_receipt.no_cutover_terminal_receipt);
    assert!(!audit_chain_closeout_receipt.shadow_readiness_failed);
    assert!(!audit_chain_closeout_receipt.canonical_write_enabled);
    assert!(!audit_chain_closeout_receipt.canonical_read_enabled);
    let audit_chain_closeout_receipt_payload = serde_json::to_value(&audit_chain_closeout_receipt)
        .expect("audit-chain closeout receipt serializes");
    let audit_chain_closeout_replay_decision =
        build_work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                audit_chain_closeout_receipt: &audit_chain_closeout_receipt,
                audit_chain_closeout_receipt_payload: &audit_chain_closeout_receipt_payload,
                latest_audit_chain_closeout_receipt_payload: Some(
                    &audit_chain_closeout_receipt_payload,
                ),
                audit_chain_closeout_receipt_events: 1,
                audit_chain_closeout_receipt_readback_ready: true,
                prior_audit_chain_closeout_replay_consistency_events: 0,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        audit_chain_closeout_replay_decision.decision,
        "work_graph_canonical_projection_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
    );
    assert!(audit_chain_closeout_replay_decision.replay_consistent);
    assert!(audit_chain_closeout_replay_decision.audit_chain_closeout_receipt_matches_readback);
    assert!(!audit_chain_closeout_replay_decision.shadow_readiness_failed);
    let enablement_operator_review_packet =
        build_work_graph_canonical_projection_enablement_operator_review_packet(
            WorkGraphCanonicalProjectionEnablementOperatorReviewPacketInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &projection_receipt,
                projection_replay_consistency_decision: &replay_decision,
                closeout_receipt: &closeout_receipt,
                closeout_replay_consistency_decision: &closeout_replay_decision,
                audit_chain_closeout_receipt: &audit_chain_closeout_receipt,
                audit_chain_closeout_replay_consistency_decision:
                    &audit_chain_closeout_replay_decision,
                projection_receipt_events: 1,
                projection_replay_consistency_events: 1,
                closeout_receipt_events: 1,
                closeout_replay_consistency_events: 1,
                audit_chain_closeout_receipt_events: 1,
                audit_chain_closeout_replay_consistency_events: 1,
                prior_enablement_operator_review_packet_events: 0,
                projection_receipt_readback_ready: true,
                projection_replay_consistency_ready: true,
                closeout_receipt_readback_ready: true,
                closeout_replay_consistency_ready: true,
                audit_chain_closeout_receipt_readback_ready: true,
                audit_chain_closeout_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        enablement_operator_review_packet.decision,
        "work_graph_canonical_projection_enablement_operator_review_ready_shadow_no_live_cutover"
    );
    assert!(enablement_operator_review_packet.enablement_operator_review_ready);
    assert!(enablement_operator_review_packet.no_live_enablement_rehearsal_ready);
    assert!(!enablement_operator_review_packet.shadow_readiness_failed);
    assert!(!enablement_operator_review_packet.enablement_allowed);
    assert!(!enablement_operator_review_packet.operator_approval_recorded);
    assert!(!enablement_operator_review_packet.reviewed_flag_enabled);
    assert!(!enablement_operator_review_packet.canonical_write_enabled);
    assert!(!enablement_operator_review_packet.live_cutover_enabled);
    let enablement_operator_review_packet_payload =
        serde_json::to_value(&enablement_operator_review_packet)
            .expect("enablement operator-review packet serializes");
    let enablement_operator_review_replay_decision =
            build_work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    enablement_operator_review_packet: &enablement_operator_review_packet,
                    enablement_operator_review_packet_payload: &enablement_operator_review_packet_payload,
                    latest_enablement_operator_review_packet_payload: Some(
                        &enablement_operator_review_packet_payload,
                    ),
                    enablement_operator_review_packet_events: 1,
                    enablement_operator_review_packet_readback_ready: true,
                    prior_enablement_operator_review_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        enablement_operator_review_replay_decision.decision,
        "work_graph_canonical_projection_enablement_operator_review_replay_consistent_shadow_no_live_cutover"
    );
    assert!(enablement_operator_review_replay_decision.replay_consistent);
    assert!(
        enablement_operator_review_replay_decision
            .enablement_operator_review_packet_matches_readback
    );
    assert!(!enablement_operator_review_replay_decision.shadow_readiness_failed);
    assert!(!enablement_operator_review_replay_decision.enablement_allowed);
    assert!(!enablement_operator_review_replay_decision.operator_approval_recorded);
    assert!(!enablement_operator_review_replay_decision.reviewed_flag_enabled);
    let enablement_no_live_rehearsal_closeout_receipt =
        build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput {
                source_surface_id: "agent_jobs",
                enablement_operator_review_packet: &enablement_operator_review_packet,
                enablement_operator_review_replay_consistency_decision:
                    &enablement_operator_review_replay_decision,
                enablement_operator_review_packet_events: 1,
                enablement_operator_review_replay_consistency_events: 1,
                prior_enablement_no_live_rehearsal_closeout_events: 0,
                enablement_operator_review_packet_readback_ready: true,
                enablement_operator_review_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        enablement_no_live_rehearsal_closeout_receipt.decision,
        "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_recorded_shadow_no_live_cutover"
    );
    assert!(
        enablement_no_live_rehearsal_closeout_receipt.no_live_enablement_rehearsal_closeout_ready
    );
    assert!(!enablement_no_live_rehearsal_closeout_receipt.shadow_readiness_failed);
    assert!(!enablement_no_live_rehearsal_closeout_receipt.enablement_allowed);
    assert!(!enablement_no_live_rehearsal_closeout_receipt.operator_approval_recorded);
    assert!(!enablement_no_live_rehearsal_closeout_receipt.reviewed_flag_enabled);
    assert!(!enablement_no_live_rehearsal_closeout_receipt.canonical_write_enabled);
    let enablement_no_live_rehearsal_closeout_receipt_payload =
        serde_json::to_value(&enablement_no_live_rehearsal_closeout_receipt)
            .expect("enablement no-live rehearsal closeout receipt serializes");
    let enablement_no_live_rehearsal_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    no_live_rehearsal_closeout_receipt:
                        &enablement_no_live_rehearsal_closeout_receipt,
                    no_live_rehearsal_closeout_receipt_payload:
                        &enablement_no_live_rehearsal_closeout_receipt_payload,
                    latest_no_live_rehearsal_closeout_receipt_payload: Some(
                        &enablement_no_live_rehearsal_closeout_receipt_payload,
                    ),
                    no_live_rehearsal_closeout_events: 1,
                    no_live_rehearsal_closeout_readback_ready: true,
                    prior_no_live_rehearsal_closeout_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        enablement_no_live_rehearsal_closeout_replay_decision.decision,
        "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistent_shadow_no_live_cutover"
    );
    assert!(enablement_no_live_rehearsal_closeout_replay_decision.replay_consistent);
    assert!(
        enablement_no_live_rehearsal_closeout_replay_decision
            .no_live_rehearsal_closeout_matches_readback
    );
    assert!(
        enablement_no_live_rehearsal_closeout_replay_decision
            .no_live_enablement_rehearsal_closeout_ready
    );
    assert!(!enablement_no_live_rehearsal_closeout_replay_decision.shadow_readiness_failed);
    assert!(!enablement_no_live_rehearsal_closeout_replay_decision.enablement_allowed);
    assert!(!enablement_no_live_rehearsal_closeout_replay_decision.reviewed_flag_enabled);
    let enablement_audit_chain_closeout_receipt =
        build_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementAuditChainCloseoutInput {
                source_surface_id: "agent_jobs",
                enablement_operator_review_packet: &enablement_operator_review_packet,
                enablement_operator_review_replay_consistency_decision:
                    &enablement_operator_review_replay_decision,
                no_live_rehearsal_closeout_receipt: &enablement_no_live_rehearsal_closeout_receipt,
                no_live_rehearsal_closeout_replay_consistency_decision:
                    &enablement_no_live_rehearsal_closeout_replay_decision,
                enablement_operator_review_packet_events: 1,
                enablement_operator_review_replay_consistency_events: 1,
                no_live_rehearsal_closeout_events: 1,
                no_live_rehearsal_closeout_replay_consistency_events: 1,
                prior_enablement_audit_chain_closeout_events: 0,
                enablement_operator_review_packet_readback_ready: true,
                enablement_operator_review_replay_consistency_ready: true,
                no_live_rehearsal_closeout_readback_ready: true,
                no_live_rehearsal_closeout_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        enablement_audit_chain_closeout_receipt.decision,
        "work_graph_canonical_projection_enablement_audit_chain_closeout_recorded_shadow_no_live_cutover"
    );
    assert!(enablement_audit_chain_closeout_receipt.enablement_audit_chain_closeout_ready);
    assert!(!enablement_audit_chain_closeout_receipt.shadow_readiness_failed);
    assert!(!enablement_audit_chain_closeout_receipt.enablement_allowed);
    assert!(!enablement_audit_chain_closeout_receipt.operator_approval_recorded);
    assert!(!enablement_audit_chain_closeout_receipt.reviewed_flag_enabled);
    let enablement_audit_chain_closeout_receipt_payload =
        serde_json::to_value(&enablement_audit_chain_closeout_receipt)
            .expect("enablement audit-chain closeout receipt serializes");
    let enablement_audit_chain_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    enablement_audit_chain_closeout_receipt:
                        &enablement_audit_chain_closeout_receipt,
                    enablement_audit_chain_closeout_receipt_payload:
                        &enablement_audit_chain_closeout_receipt_payload,
                    latest_enablement_audit_chain_closeout_receipt_payload: Some(
                        &enablement_audit_chain_closeout_receipt_payload,
                    ),
                    enablement_audit_chain_closeout_events: 1,
                    enablement_audit_chain_closeout_readback_ready: true,
                    prior_enablement_audit_chain_closeout_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        enablement_audit_chain_closeout_replay_decision.decision,
        "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
    );
    assert!(enablement_audit_chain_closeout_replay_decision.replay_consistent);
    assert!(
        enablement_audit_chain_closeout_replay_decision
            .enablement_audit_chain_closeout_matches_readback
    );
    assert!(enablement_audit_chain_closeout_replay_decision.enablement_audit_chain_closeout_ready);
    assert!(!enablement_audit_chain_closeout_replay_decision.shadow_readiness_failed);
    assert!(!enablement_audit_chain_closeout_replay_decision.enablement_allowed);
    assert!(!enablement_audit_chain_closeout_replay_decision.reviewed_flag_enabled);
    let activation_precondition_packet =
        build_work_graph_canonical_projection_enablement_activation_precondition_operator_packet(
            WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput {
                source_surface_id: "agent_jobs",
                enablement_audit_chain_closeout_receipt: &enablement_audit_chain_closeout_receipt,
                enablement_audit_chain_closeout_replay_consistency_decision:
                    &enablement_audit_chain_closeout_replay_decision,
                enablement_audit_chain_closeout_events: 1,
                enablement_audit_chain_closeout_replay_consistency_events: 1,
                prior_enablement_activation_precondition_operator_packet_events: 0,
                enablement_audit_chain_closeout_readback_ready: true,
                enablement_audit_chain_closeout_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        activation_precondition_packet.decision,
        "work_graph_canonical_projection_enablement_activation_precondition_ready_shadow_no_live_cutover"
    );
    assert!(activation_precondition_packet.activation_precondition_ready);
    assert!(!activation_precondition_packet.activation_allowed);
    assert!(!activation_precondition_packet.shadow_readiness_failed);
    assert!(!activation_precondition_packet.enablement_allowed);
    assert!(!activation_precondition_packet.operator_approval_recorded);
    assert!(!activation_precondition_packet.reviewed_flag_enabled);
    assert!(!activation_precondition_packet.canonical_write_enabled);
    assert!(activation_precondition_packet.approval_record_required_before_activation);
    assert!(activation_precondition_packet.reviewed_flag_required_before_activation);
    let activation_precondition_packet_payload =
        serde_json::to_value(&activation_precondition_packet)
            .expect("activation precondition packet serializes");
    let activation_precondition_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_precondition_operator_packet: &activation_precondition_packet,
                    activation_precondition_operator_packet_payload:
                        &activation_precondition_packet_payload,
                    latest_activation_precondition_operator_packet_payload: Some(
                        &activation_precondition_packet_payload,
                    ),
                    activation_precondition_operator_packet_events: 1,
                    activation_precondition_operator_packet_readback_ready: true,
                    prior_activation_precondition_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        activation_precondition_replay_decision.decision,
        "work_graph_canonical_projection_enablement_activation_precondition_replay_consistent_shadow_no_live_cutover"
    );
    assert!(activation_precondition_replay_decision.replay_consistent);
    assert!(
        activation_precondition_replay_decision
            .activation_precondition_operator_packet_matches_readback
    );
    assert!(activation_precondition_replay_decision.activation_precondition_ready);
    assert!(!activation_precondition_replay_decision.activation_allowed);
    assert!(!activation_precondition_replay_decision.shadow_readiness_failed);
    let activation_no_live_closeout_receipt =
        build_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutInput {
                source_surface_id: "agent_jobs",
                activation_precondition_operator_packet: &activation_precondition_packet,
                activation_precondition_replay_consistency_decision:
                    &activation_precondition_replay_decision,
                activation_precondition_operator_packet_events: 1,
                activation_precondition_replay_consistency_events: 1,
                prior_activation_no_live_closeout_events: 0,
                activation_precondition_operator_packet_readback_ready: true,
                activation_precondition_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        activation_no_live_closeout_receipt.decision,
        "work_graph_canonical_projection_enablement_activation_no_live_closeout_recorded_shadow_no_live_cutover"
    );
    assert!(activation_no_live_closeout_receipt.activation_no_live_closeout_ready);
    assert!(activation_no_live_closeout_receipt.activation_precondition_ready);
    assert!(activation_no_live_closeout_receipt.activation_precondition_replay_consistent);
    assert!(activation_no_live_closeout_receipt.no_live_enablement_rehearsal_ready);
    assert!(!activation_no_live_closeout_receipt.activation_allowed);
    assert!(!activation_no_live_closeout_receipt.enablement_allowed);
    assert!(!activation_no_live_closeout_receipt.operator_approval_recorded);
    assert!(!activation_no_live_closeout_receipt.approval_record_mutation_enabled);
    assert!(!activation_no_live_closeout_receipt.reviewed_flag_enabled);
    assert!(activation_no_live_closeout_receipt.approval_record_required_before_activation);
    assert!(activation_no_live_closeout_receipt.reviewed_flag_required_before_activation);
    assert!(!activation_no_live_closeout_receipt.canonical_write_enabled);
    assert!(!activation_no_live_closeout_receipt.canonical_read_enabled);
    assert!(!activation_no_live_closeout_receipt.canonical_projection_persistence_enabled);
    assert!(!activation_no_live_closeout_receipt.feature_flag_enabled);
    assert_eq!(activation_no_live_closeout_receipt.canary_stage, "off");
    assert_eq!(activation_no_live_closeout_receipt.canary_traffic_ppm, 0);
    assert!(!activation_no_live_closeout_receipt.live_blocking_enabled);
    assert!(!activation_no_live_closeout_receipt.live_cutover_enabled);
    assert!(
        activation_no_live_closeout_receipt
            .closeout_blockers
            .is_empty()
    );
    let activation_no_live_closeout_receipt_payload =
        serde_json::to_value(&activation_no_live_closeout_receipt)
            .expect("activation no-live closeout receipt serializes");
    let activation_no_live_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_no_live_closeout_receipt: &activation_no_live_closeout_receipt,
                    activation_no_live_closeout_receipt_payload:
                        &activation_no_live_closeout_receipt_payload,
                    latest_activation_no_live_closeout_receipt_payload: Some(
                        &activation_no_live_closeout_receipt_payload,
                    ),
                    activation_no_live_closeout_events: 1,
                    activation_no_live_closeout_readback_ready: true,
                    prior_activation_no_live_closeout_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        activation_no_live_closeout_replay_decision.decision,
        "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistent_shadow_no_live_cutover"
    );
    assert!(activation_no_live_closeout_replay_decision.replay_consistent);
    assert!(
        activation_no_live_closeout_replay_decision.activation_no_live_closeout_matches_readback
    );
    assert!(activation_no_live_closeout_replay_decision.activation_no_live_closeout_ready);
    assert!(!activation_no_live_closeout_replay_decision.activation_allowed);
    assert!(!activation_no_live_closeout_replay_decision.reviewed_flag_enabled);
    assert!(!activation_no_live_closeout_replay_decision.canonical_write_enabled);
    assert!(!activation_no_live_closeout_replay_decision.shadow_readiness_failed);
    let activation_audit_chain_closeout_receipt =
        build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutInput {
                source_surface_id: "agent_jobs",
                activation_precondition_operator_packet: &activation_precondition_packet,
                activation_precondition_replay_consistency_decision:
                    &activation_precondition_replay_decision,
                activation_no_live_closeout_receipt: &activation_no_live_closeout_receipt,
                activation_no_live_closeout_replay_consistency_decision:
                    &activation_no_live_closeout_replay_decision,
                activation_precondition_operator_packet_events: 1,
                activation_precondition_replay_consistency_events: 1,
                activation_no_live_closeout_events: 1,
                activation_no_live_closeout_replay_consistency_events: 1,
                prior_activation_audit_chain_closeout_events: 0,
                activation_precondition_operator_packet_readback_ready: true,
                activation_precondition_replay_consistency_ready: true,
                activation_no_live_closeout_readback_ready: true,
                activation_no_live_closeout_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        activation_audit_chain_closeout_receipt.decision,
        "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_recorded_shadow_no_live_cutover"
    );
    assert!(activation_audit_chain_closeout_receipt.activation_audit_chain_closeout_ready);
    assert!(activation_audit_chain_closeout_receipt.activation_precondition_ready);
    assert!(activation_audit_chain_closeout_receipt.activation_precondition_replay_consistent);
    assert!(activation_audit_chain_closeout_receipt.activation_no_live_closeout_ready);
    assert!(activation_audit_chain_closeout_receipt.activation_no_live_closeout_replay_consistent);
    assert!(activation_audit_chain_closeout_receipt.no_live_guardrails_ready);
    assert!(!activation_audit_chain_closeout_receipt.activation_allowed);
    assert!(!activation_audit_chain_closeout_receipt.enablement_allowed);
    assert!(!activation_audit_chain_closeout_receipt.operator_approval_recorded);
    assert!(!activation_audit_chain_closeout_receipt.approval_record_mutation_enabled);
    assert!(!activation_audit_chain_closeout_receipt.reviewed_flag_enabled);
    assert!(activation_audit_chain_closeout_receipt.approval_record_required_before_activation);
    assert!(activation_audit_chain_closeout_receipt.reviewed_flag_required_before_activation);
    assert!(!activation_audit_chain_closeout_receipt.canonical_write_enabled);
    assert!(!activation_audit_chain_closeout_receipt.canonical_read_enabled);
    assert!(!activation_audit_chain_closeout_receipt.canonical_projection_persistence_enabled);
    assert!(!activation_audit_chain_closeout_receipt.live_blocking_enabled);
    assert!(!activation_audit_chain_closeout_receipt.live_cutover_enabled);
    assert!(!activation_audit_chain_closeout_receipt.shadow_readiness_failed);
    assert!(
        activation_audit_chain_closeout_receipt
            .closeout_blockers
            .is_empty()
    );
    let activation_audit_chain_closeout_payload =
        serde_json::to_value(&activation_audit_chain_closeout_receipt)
            .expect("activation audit-chain closeout receipt serializes");
    let activation_audit_chain_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_audit_chain_closeout_receipt:
                        &activation_audit_chain_closeout_receipt,
                    activation_audit_chain_closeout_receipt_payload:
                        &activation_audit_chain_closeout_payload,
                    latest_activation_audit_chain_closeout_receipt_payload: Some(
                        &activation_audit_chain_closeout_payload,
                    ),
                    activation_audit_chain_closeout_events: 1,
                    activation_audit_chain_closeout_readback_ready: true,
                    prior_activation_audit_chain_closeout_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        activation_audit_chain_closeout_replay_decision.decision,
        "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistent_shadow_no_live_cutover"
    );
    assert!(activation_audit_chain_closeout_replay_decision.replay_consistent);
    assert!(
        activation_audit_chain_closeout_replay_decision
            .activation_audit_chain_closeout_matches_readback
    );
    assert!(activation_audit_chain_closeout_replay_decision.activation_audit_chain_closeout_ready);
    assert!(!activation_audit_chain_closeout_replay_decision.activation_allowed);
    assert!(!activation_audit_chain_closeout_replay_decision.reviewed_flag_enabled);
    assert!(!activation_audit_chain_closeout_replay_decision.canonical_write_enabled);
    assert!(!activation_audit_chain_closeout_replay_decision.shadow_readiness_failed);
    let activation_operator_approval_readiness_preflight_packet =
            build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet(
                WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacketInput {
                    source_surface_id: "agent_jobs",
                    activation_audit_chain_closeout_receipt:
                        &activation_audit_chain_closeout_receipt,
                    activation_audit_chain_closeout_replay_consistency_decision:
                        &activation_audit_chain_closeout_replay_decision,
                    activation_audit_chain_closeout_events: 1,
                    activation_audit_chain_closeout_replay_consistency_events: 1,
                    prior_activation_operator_approval_readiness_preflight_packet_events: 0,
                    activation_audit_chain_closeout_readback_ready: true,
                    activation_audit_chain_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        activation_operator_approval_readiness_preflight_packet.decision,
        "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_ready_shadow_no_live_cutover"
    );
    assert!(
        activation_operator_approval_readiness_preflight_packet
            .activation_operator_approval_readiness_preflight_ready
    );
    assert!(
        activation_operator_approval_readiness_preflight_packet
            .activation_audit_chain_closeout_ready
    );
    assert!(
        activation_operator_approval_readiness_preflight_packet
            .activation_audit_chain_closeout_replay_consistent
    );
    assert!(
        activation_operator_approval_readiness_preflight_packet
            .operator_approval_required_before_activation
    );
    assert!(
        activation_operator_approval_readiness_preflight_packet
            .approval_record_required_before_activation
    );
    assert!(
        activation_operator_approval_readiness_preflight_packet
            .reviewed_flag_required_before_activation
    );
    assert!(!activation_operator_approval_readiness_preflight_packet.activation_allowed);
    assert!(!activation_operator_approval_readiness_preflight_packet.operator_approval_recorded);
    assert!(
        !activation_operator_approval_readiness_preflight_packet.approval_record_mutation_enabled
    );
    assert!(!activation_operator_approval_readiness_preflight_packet.reviewed_flag_enabled);
    assert!(
        !activation_operator_approval_readiness_preflight_packet.reviewed_flag_mutation_enabled
    );
    assert!(!activation_operator_approval_readiness_preflight_packet.canonical_write_enabled);
    assert!(!activation_operator_approval_readiness_preflight_packet.shadow_readiness_failed);
    assert!(
        activation_operator_approval_readiness_preflight_packet
            .preflight_blockers
            .is_empty()
    );
    let activation_operator_approval_readiness_preflight_payload =
        serde_json::to_value(&activation_operator_approval_readiness_preflight_packet)
            .expect("preflight packet should serialize");
    let activation_operator_approval_readiness_preflight_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_operator_approval_readiness_preflight_packet:
                        &activation_operator_approval_readiness_preflight_packet,
                    activation_operator_approval_readiness_preflight_packet_payload:
                        &activation_operator_approval_readiness_preflight_payload,
                    latest_activation_operator_approval_readiness_preflight_packet_payload:
                        Some(&activation_operator_approval_readiness_preflight_payload),
                    activation_operator_approval_readiness_preflight_packet_events: 1,
                    activation_operator_approval_readiness_preflight_packet_readback_ready: true,
                    prior_activation_operator_approval_readiness_preflight_replay_consistency_events: 0,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        activation_operator_approval_readiness_preflight_replay_decision.decision,
        "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistent_shadow_no_live_cutover"
    );
    assert!(activation_operator_approval_readiness_preflight_replay_decision.replay_consistent);
    assert!(
        activation_operator_approval_readiness_preflight_replay_decision
            .activation_operator_approval_readiness_preflight_packet_matches_readback
    );
    assert!(
        activation_operator_approval_readiness_preflight_replay_decision
            .activation_operator_approval_readiness_preflight_ready
    );
    assert!(
        activation_operator_approval_readiness_preflight_replay_decision
            .operator_approval_required_before_activation
    );
    assert!(
        activation_operator_approval_readiness_preflight_replay_decision
            .reviewed_flag_required_before_activation
    );
    assert!(!activation_operator_approval_readiness_preflight_replay_decision.activation_allowed);
    assert!(
        !activation_operator_approval_readiness_preflight_replay_decision
            .approval_record_mutation_enabled
    );
    assert!(
        !activation_operator_approval_readiness_preflight_replay_decision
            .reviewed_flag_mutation_enabled
    );
    assert!(
        !activation_operator_approval_readiness_preflight_replay_decision.canonical_write_enabled
    );
    assert!(
        !activation_operator_approval_readiness_preflight_replay_decision.shadow_readiness_failed
    );
}

#[test]
fn surface_audit_blocks_when_audit_chain_missing_segment() {
    let matrix = sample_matrix();
    let mut readback = ready_readback();
    let missing_segment = readback
        .segments
        .iter_mut()
        .find(|segment| segment.segment_id == "reviewed_flag_audit_chain_closeout_receipt")
        .expect("reviewed flag audit-chain closeout segment should exist");
    missing_segment.event_count = 0;
    missing_segment.latest_payload = None;
    missing_segment.latest_decision = "missing".to_string();
    missing_segment.readback_ready = false;
    missing_segment.ready = false;
    readback.chain_readback_ready = false;
    readback.chain_ready = false;

    let packet = build_work_graph_surface_audit_packet(WorkGraphSurfaceAuditPacketInput {
        job_id: "job-1",
        promotion_readiness_shadow_matrix: &matrix,
        role_manifest_shadow_decisions: &[],
        audit_chain_readback: &readback,
    });

    assert_eq!(
        packet.decision,
        "work_graph_surface_audit_blocked_shadow_no_live_cutover"
    );
    assert!(!packet.audit_packet_ready);
    assert!(!packet.audit_chain.chain_ready);
    assert!(
        packet
            .audit_chain
            .missing_segment_ids
            .contains(&"reviewed_flag_audit_chain_closeout_receipt".to_string())
    );
    let summary = summarize_work_graph_surface_audit_packet(&packet);
    assert!(
        summary
            .operator_matrix_rows
            .iter()
            .all(|row| row.next_blocker == "audit_chain_or_no_live_guardrail_not_ready")
    );
    let projection_receipt = build_work_graph_canonical_projection_shadow_receipt(&summary);
    assert_eq!(
        projection_receipt.decision,
        "work_graph_canonical_projection_blocked_shadow_no_live_cutover"
    );
    assert!(!projection_receipt.read_projection_ready);
    assert!(!projection_receipt.canonical_write_enabled);
    assert!(!packet.live_cutover_enabled);
    let projection_receipt_payload =
        serde_json::to_value(&projection_receipt).expect("projection receipt serializes");
    let replay_decision = build_work_graph_canonical_projection_replay_consistency_decision(
        WorkGraphCanonicalProjectionReplayConsistencyInput {
            source_surface_id: "agent_jobs",
            projection_receipt: &projection_receipt,
            projection_receipt_payload: &projection_receipt_payload,
            latest_projection_receipt_payload: Some(&json!({
                "decision": "stale_projection_receipt"
            })),
            projection_receipt_events: 1,
            projection_receipt_readback_ready: true,
            prior_projection_replay_consistency_events: 1,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
        },
    );
    assert_eq!(
        replay_decision.decision,
        "work_graph_canonical_projection_replay_mismatch_shadow_no_live_cutover"
    );
    assert!(!replay_decision.replay_consistent);
    assert!(!replay_decision.projection_receipt_matches_readback);
    assert!(replay_decision.shadow_readiness_failed);
    let closeout_receipt = build_work_graph_canonical_projection_closeout_receipt(
        WorkGraphCanonicalProjectionCloseoutReceiptInput {
            source_surface_id: "agent_jobs",
            projection_receipt: &projection_receipt,
            replay_consistency_decision: &replay_decision,
            projection_receipt_events: 1,
            projection_replay_consistency_events: 1,
            prior_projection_closeout_receipt_events: 1,
            projection_receipt_readback_ready: true,
            projection_replay_consistency_ready: true,
            live_blocking_event_count: 0,
            live_cutover_event_count: 0,
        },
    );
    assert_eq!(
        closeout_receipt.decision,
        "work_graph_canonical_projection_closeout_blocked_shadow_no_live_cutover"
    );
    assert!(!closeout_receipt.closeout_ready);
    assert!(closeout_receipt.shadow_readiness_failed);
    assert!(!closeout_receipt.no_cutover_terminal_receipt);
    assert!(
        closeout_receipt
            .closeout_blockers
            .iter()
            .any(|blocker| { blocker.starts_with("canonical_projection_replay_consistent") })
    );
    let closeout_receipt_payload =
        serde_json::to_value(&closeout_receipt).expect("closeout receipt serializes");
    let closeout_replay_decision =
        build_work_graph_canonical_projection_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionCloseoutReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                closeout_receipt: &closeout_receipt,
                closeout_receipt_payload: &closeout_receipt_payload,
                latest_closeout_receipt_payload: Some(&json!({
                    "decision": "stale_closeout_receipt"
                })),
                closeout_receipt_events: 1,
                closeout_receipt_readback_ready: true,
                prior_closeout_replay_consistency_events: 1,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        closeout_replay_decision.decision,
        "work_graph_canonical_projection_closeout_replay_mismatch_shadow_no_live_cutover"
    );
    assert!(!closeout_replay_decision.replay_consistent);
    assert!(!closeout_replay_decision.closeout_receipt_matches_readback);
    assert!(closeout_replay_decision.shadow_readiness_failed);
    let audit_chain_closeout_receipt =
        build_work_graph_canonical_projection_audit_chain_closeout_receipt(
            WorkGraphCanonicalProjectionAuditChainCloseoutReceiptInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &projection_receipt,
                projection_replay_consistency_decision: &replay_decision,
                closeout_receipt: &closeout_receipt,
                closeout_replay_consistency_decision: &closeout_replay_decision,
                projection_receipt_events: 1,
                projection_replay_consistency_events: 1,
                closeout_receipt_events: 1,
                closeout_replay_consistency_events: 1,
                prior_audit_chain_closeout_receipt_events: 1,
                projection_receipt_readback_ready: true,
                projection_replay_consistency_ready: true,
                closeout_receipt_readback_ready: true,
                closeout_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        audit_chain_closeout_receipt.decision,
        "work_graph_canonical_projection_audit_chain_closeout_blocked_shadow_no_live_cutover"
    );
    assert!(!audit_chain_closeout_receipt.audit_chain_closeout_ready);
    assert!(!audit_chain_closeout_receipt.no_cutover_terminal_receipt);
    assert!(audit_chain_closeout_receipt.shadow_readiness_failed);
    assert!(
        audit_chain_closeout_receipt
            .audit_chain_blockers
            .iter()
            .any(|blocker| blocker.starts_with("canonical_projection_closeout_replay_consistent"))
    );
    let audit_chain_closeout_receipt_payload = serde_json::to_value(&audit_chain_closeout_receipt)
        .expect("audit-chain closeout receipt serializes");
    let audit_chain_closeout_replay_decision =
        build_work_graph_canonical_projection_audit_chain_closeout_replay_consistency_decision(
            WorkGraphCanonicalProjectionAuditChainCloseoutReplayConsistencyInput {
                source_surface_id: "agent_jobs",
                audit_chain_closeout_receipt: &audit_chain_closeout_receipt,
                audit_chain_closeout_receipt_payload: &audit_chain_closeout_receipt_payload,
                latest_audit_chain_closeout_receipt_payload: Some(&json!({
                    "decision": "stale_audit_chain_closeout_receipt"
                })),
                audit_chain_closeout_receipt_events: 1,
                audit_chain_closeout_receipt_readback_ready: true,
                prior_audit_chain_closeout_replay_consistency_events: 1,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        audit_chain_closeout_replay_decision.decision,
        "work_graph_canonical_projection_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover"
    );
    assert!(!audit_chain_closeout_replay_decision.replay_consistent);
    assert!(!audit_chain_closeout_replay_decision.audit_chain_closeout_receipt_matches_readback);
    assert!(audit_chain_closeout_replay_decision.shadow_readiness_failed);
    let enablement_operator_review_packet =
        build_work_graph_canonical_projection_enablement_operator_review_packet(
            WorkGraphCanonicalProjectionEnablementOperatorReviewPacketInput {
                source_surface_id: "agent_jobs",
                projection_receipt: &projection_receipt,
                projection_replay_consistency_decision: &replay_decision,
                closeout_receipt: &closeout_receipt,
                closeout_replay_consistency_decision: &closeout_replay_decision,
                audit_chain_closeout_receipt: &audit_chain_closeout_receipt,
                audit_chain_closeout_replay_consistency_decision:
                    &audit_chain_closeout_replay_decision,
                projection_receipt_events: 1,
                projection_replay_consistency_events: 1,
                closeout_receipt_events: 1,
                closeout_replay_consistency_events: 1,
                audit_chain_closeout_receipt_events: 1,
                audit_chain_closeout_replay_consistency_events: 1,
                prior_enablement_operator_review_packet_events: 1,
                projection_receipt_readback_ready: true,
                projection_replay_consistency_ready: true,
                closeout_receipt_readback_ready: true,
                closeout_replay_consistency_ready: true,
                audit_chain_closeout_receipt_readback_ready: true,
                audit_chain_closeout_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        enablement_operator_review_packet.decision,
        "work_graph_canonical_projection_enablement_operator_review_blocked_shadow_no_live_cutover"
    );
    assert!(!enablement_operator_review_packet.enablement_operator_review_ready);
    assert!(!enablement_operator_review_packet.no_cutover_terminal_receipt);
    assert!(enablement_operator_review_packet.shadow_readiness_failed);
    assert!(!enablement_operator_review_packet.enablement_allowed);
    assert!(
        enablement_operator_review_packet
            .enablement_blockers
            .iter()
            .any(|blocker| blocker
                .starts_with("canonical_projection_audit_chain_closeout_replay_ready"))
    );
    let enablement_operator_review_packet_payload =
        serde_json::to_value(&enablement_operator_review_packet)
            .expect("enablement operator-review packet serializes");
    let enablement_operator_review_replay_decision =
            build_work_graph_canonical_projection_enablement_operator_review_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementOperatorReviewReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    enablement_operator_review_packet: &enablement_operator_review_packet,
                    enablement_operator_review_packet_payload: &enablement_operator_review_packet_payload,
                    latest_enablement_operator_review_packet_payload: Some(&json!({
                        "decision": "stale_enablement_operator_review_packet"
                    })),
                    enablement_operator_review_packet_events: 1,
                    enablement_operator_review_packet_readback_ready: true,
                    prior_enablement_operator_review_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        enablement_operator_review_replay_decision.decision,
        "work_graph_canonical_projection_enablement_operator_review_replay_mismatch_shadow_no_live_cutover"
    );
    assert!(!enablement_operator_review_replay_decision.replay_consistent);
    assert!(
        !enablement_operator_review_replay_decision
            .enablement_operator_review_packet_matches_readback
    );
    assert!(enablement_operator_review_replay_decision.shadow_readiness_failed);
    assert!(!enablement_operator_review_replay_decision.enablement_allowed);
    let enablement_no_live_rehearsal_closeout_receipt =
        build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutInput {
                source_surface_id: "agent_jobs",
                enablement_operator_review_packet: &enablement_operator_review_packet,
                enablement_operator_review_replay_consistency_decision:
                    &enablement_operator_review_replay_decision,
                enablement_operator_review_packet_events: 1,
                enablement_operator_review_replay_consistency_events: 1,
                prior_enablement_no_live_rehearsal_closeout_events: 1,
                enablement_operator_review_packet_readback_ready: true,
                enablement_operator_review_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        enablement_no_live_rehearsal_closeout_receipt.decision,
        "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_blocked_shadow_no_live_cutover"
    );
    assert!(
        !enablement_no_live_rehearsal_closeout_receipt.no_live_enablement_rehearsal_closeout_ready
    );
    assert!(enablement_no_live_rehearsal_closeout_receipt.shadow_readiness_failed);
    assert!(!enablement_no_live_rehearsal_closeout_receipt.enablement_allowed);
    let enablement_no_live_rehearsal_closeout_receipt_payload =
        serde_json::to_value(&enablement_no_live_rehearsal_closeout_receipt)
            .expect("enablement no-live rehearsal closeout receipt serializes");
    let enablement_no_live_rehearsal_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementNoLiveRehearsalCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    no_live_rehearsal_closeout_receipt:
                        &enablement_no_live_rehearsal_closeout_receipt,
                    no_live_rehearsal_closeout_receipt_payload:
                        &enablement_no_live_rehearsal_closeout_receipt_payload,
                    latest_no_live_rehearsal_closeout_receipt_payload: Some(&json!({
                        "decision": "stale_no_live_rehearsal_closeout_receipt"
                    })),
                    no_live_rehearsal_closeout_events: 1,
                    no_live_rehearsal_closeout_readback_ready: true,
                    prior_no_live_rehearsal_closeout_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        enablement_no_live_rehearsal_closeout_replay_decision.decision,
        "work_graph_canonical_projection_enablement_no_live_rehearsal_closeout_replay_mismatch_shadow_no_live_cutover"
    );
    assert!(!enablement_no_live_rehearsal_closeout_replay_decision.replay_consistent);
    assert!(
        !enablement_no_live_rehearsal_closeout_replay_decision
            .no_live_rehearsal_closeout_matches_readback
    );
    assert!(enablement_no_live_rehearsal_closeout_replay_decision.shadow_readiness_failed);
    assert!(!enablement_no_live_rehearsal_closeout_replay_decision.enablement_allowed);
    let enablement_audit_chain_closeout_receipt =
        build_work_graph_canonical_projection_enablement_audit_chain_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementAuditChainCloseoutInput {
                source_surface_id: "agent_jobs",
                enablement_operator_review_packet: &enablement_operator_review_packet,
                enablement_operator_review_replay_consistency_decision:
                    &enablement_operator_review_replay_decision,
                no_live_rehearsal_closeout_receipt: &enablement_no_live_rehearsal_closeout_receipt,
                no_live_rehearsal_closeout_replay_consistency_decision:
                    &enablement_no_live_rehearsal_closeout_replay_decision,
                enablement_operator_review_packet_events: 1,
                enablement_operator_review_replay_consistency_events: 1,
                no_live_rehearsal_closeout_events: 1,
                no_live_rehearsal_closeout_replay_consistency_events: 1,
                prior_enablement_audit_chain_closeout_events: 1,
                enablement_operator_review_packet_readback_ready: true,
                enablement_operator_review_replay_consistency_ready: true,
                no_live_rehearsal_closeout_readback_ready: true,
                no_live_rehearsal_closeout_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        enablement_audit_chain_closeout_receipt.decision,
        "work_graph_canonical_projection_enablement_audit_chain_closeout_blocked_shadow_no_live_cutover"
    );
    assert!(!enablement_audit_chain_closeout_receipt.enablement_audit_chain_closeout_ready);
    assert!(enablement_audit_chain_closeout_receipt.shadow_readiness_failed);
    assert!(
        enablement_audit_chain_closeout_receipt
            .closeout_blockers
            .iter()
            .any(|blocker| blocker.starts_with(
                "canonical_projection_enablement_no_live_rehearsal_closeout_replay_ready"
            ))
    );
    let enablement_audit_chain_closeout_receipt_payload =
        serde_json::to_value(&enablement_audit_chain_closeout_receipt)
            .expect("enablement audit-chain closeout receipt serializes");
    let enablement_audit_chain_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_audit_chain_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementAuditChainCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    enablement_audit_chain_closeout_receipt:
                        &enablement_audit_chain_closeout_receipt,
                    enablement_audit_chain_closeout_receipt_payload:
                        &enablement_audit_chain_closeout_receipt_payload,
                    latest_enablement_audit_chain_closeout_receipt_payload: Some(&json!({
                        "decision": "stale_enablement_audit_chain_closeout_receipt"
                    })),
                    enablement_audit_chain_closeout_events: 1,
                    enablement_audit_chain_closeout_readback_ready: true,
                    prior_enablement_audit_chain_closeout_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        enablement_audit_chain_closeout_replay_decision.decision,
        "work_graph_canonical_projection_enablement_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover"
    );
    assert!(!enablement_audit_chain_closeout_replay_decision.replay_consistent);
    assert!(
        !enablement_audit_chain_closeout_replay_decision
            .enablement_audit_chain_closeout_matches_readback
    );
    assert!(enablement_audit_chain_closeout_replay_decision.shadow_readiness_failed);
    assert!(!enablement_audit_chain_closeout_replay_decision.enablement_allowed);
    let activation_precondition_packet =
        build_work_graph_canonical_projection_enablement_activation_precondition_operator_packet(
            WorkGraphCanonicalProjectionEnablementActivationPreconditionOperatorPacketInput {
                source_surface_id: "agent_jobs",
                enablement_audit_chain_closeout_receipt: &enablement_audit_chain_closeout_receipt,
                enablement_audit_chain_closeout_replay_consistency_decision:
                    &enablement_audit_chain_closeout_replay_decision,
                enablement_audit_chain_closeout_events: 1,
                enablement_audit_chain_closeout_replay_consistency_events: 1,
                prior_enablement_activation_precondition_operator_packet_events: 1,
                enablement_audit_chain_closeout_readback_ready: true,
                enablement_audit_chain_closeout_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        activation_precondition_packet.decision,
        "work_graph_canonical_projection_enablement_activation_precondition_blocked_shadow_no_live_cutover"
    );
    assert!(!activation_precondition_packet.activation_precondition_ready);
    assert!(activation_precondition_packet.shadow_readiness_failed);
    assert!(!activation_precondition_packet.activation_allowed);
    assert!(
        activation_precondition_packet
            .activation_blockers
            .iter()
            .any(|blocker| blocker.starts_with(
                "canonical_projection_enablement_audit_chain_closeout_replay_consistent"
            ))
    );
    let activation_precondition_packet_payload =
        serde_json::to_value(&activation_precondition_packet)
            .expect("activation precondition packet serializes");
    let activation_precondition_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_precondition_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationPreconditionReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_precondition_operator_packet: &activation_precondition_packet,
                    activation_precondition_operator_packet_payload:
                        &activation_precondition_packet_payload,
                    latest_activation_precondition_operator_packet_payload: Some(&json!({
                        "decision": "stale_activation_precondition_packet"
                    })),
                    activation_precondition_operator_packet_events: 1,
                    activation_precondition_operator_packet_readback_ready: true,
                    prior_activation_precondition_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        activation_precondition_replay_decision.decision,
        "work_graph_canonical_projection_enablement_activation_precondition_replay_mismatch_shadow_no_live_cutover"
    );
    assert!(!activation_precondition_replay_decision.replay_consistent);
    assert!(
        !activation_precondition_replay_decision
            .activation_precondition_operator_packet_matches_readback
    );
    assert!(activation_precondition_replay_decision.shadow_readiness_failed);
    assert!(!activation_precondition_replay_decision.activation_allowed);
    let activation_no_live_closeout_receipt =
        build_work_graph_canonical_projection_enablement_activation_no_live_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutInput {
                source_surface_id: "agent_jobs",
                activation_precondition_operator_packet: &activation_precondition_packet,
                activation_precondition_replay_consistency_decision:
                    &activation_precondition_replay_decision,
                activation_precondition_operator_packet_events: 1,
                activation_precondition_replay_consistency_events: 1,
                prior_activation_no_live_closeout_events: 1,
                activation_precondition_operator_packet_readback_ready: true,
                activation_precondition_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        activation_no_live_closeout_receipt.decision,
        "work_graph_canonical_projection_enablement_activation_no_live_closeout_blocked_shadow_no_live_cutover"
    );
    assert!(!activation_no_live_closeout_receipt.activation_no_live_closeout_ready);
    assert!(!activation_no_live_closeout_receipt.activation_precondition_ready);
    assert!(!activation_no_live_closeout_receipt.activation_precondition_replay_consistent);
    assert!(!activation_no_live_closeout_receipt.activation_allowed);
    assert!(!activation_no_live_closeout_receipt.enablement_allowed);
    assert!(!activation_no_live_closeout_receipt.operator_approval_recorded);
    assert!(!activation_no_live_closeout_receipt.reviewed_flag_enabled);
    assert!(
        activation_no_live_closeout_receipt
            .closeout_blockers
            .iter()
            .any(|blocker| blocker.starts_with(
                "canonical_projection_enablement_activation_precondition_replay_ready"
            ))
    );
    let activation_no_live_closeout_receipt_payload =
        serde_json::to_value(&activation_no_live_closeout_receipt)
            .expect("activation no-live closeout receipt serializes");
    let activation_no_live_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationNoLiveCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_no_live_closeout_receipt: &activation_no_live_closeout_receipt,
                    activation_no_live_closeout_receipt_payload:
                        &activation_no_live_closeout_receipt_payload,
                    latest_activation_no_live_closeout_receipt_payload: Some(&json!({
                        "decision": "stale_activation_no_live_closeout_receipt"
                    })),
                    activation_no_live_closeout_events: 1,
                    activation_no_live_closeout_readback_ready: true,
                    prior_activation_no_live_closeout_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        activation_no_live_closeout_replay_decision.decision,
        "work_graph_canonical_projection_enablement_activation_no_live_closeout_replay_mismatch_shadow_no_live_cutover"
    );
    assert!(!activation_no_live_closeout_replay_decision.replay_consistent);
    assert!(
        !activation_no_live_closeout_replay_decision.activation_no_live_closeout_matches_readback
    );
    assert!(activation_no_live_closeout_replay_decision.shadow_readiness_failed);
    assert!(!activation_no_live_closeout_replay_decision.activation_allowed);
    assert!(
        activation_no_live_closeout_replay_decision
            .consistency_blockers
            .iter()
            .any(|blocker| blocker.starts_with(
                "canonical_projection_enablement_activation_no_live_closeout_latest_payload_matches"
            ))
    );
    let activation_audit_chain_closeout_receipt =
        build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_receipt(
            WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutInput {
                source_surface_id: "agent_jobs",
                activation_precondition_operator_packet: &activation_precondition_packet,
                activation_precondition_replay_consistency_decision:
                    &activation_precondition_replay_decision,
                activation_no_live_closeout_receipt: &activation_no_live_closeout_receipt,
                activation_no_live_closeout_replay_consistency_decision:
                    &activation_no_live_closeout_replay_decision,
                activation_precondition_operator_packet_events: 1,
                activation_precondition_replay_consistency_events: 1,
                activation_no_live_closeout_events: 1,
                activation_no_live_closeout_replay_consistency_events: 1,
                prior_activation_audit_chain_closeout_events: 1,
                activation_precondition_operator_packet_readback_ready: true,
                activation_precondition_replay_consistency_ready: true,
                activation_no_live_closeout_readback_ready: true,
                activation_no_live_closeout_replay_consistency_ready: true,
                live_blocking_event_count: 0,
                live_cutover_event_count: 0,
            },
        );
    assert_eq!(
        activation_audit_chain_closeout_receipt.decision,
        "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_blocked_shadow_no_live_cutover"
    );
    assert!(!activation_audit_chain_closeout_receipt.activation_audit_chain_closeout_ready);
    assert!(!activation_audit_chain_closeout_receipt.activation_precondition_ready);
    assert!(!activation_audit_chain_closeout_receipt.activation_precondition_replay_consistent);
    assert!(!activation_audit_chain_closeout_receipt.activation_no_live_closeout_ready);
    assert!(!activation_audit_chain_closeout_receipt.activation_no_live_closeout_replay_consistent);
    assert!(!activation_audit_chain_closeout_receipt.activation_allowed);
    assert!(!activation_audit_chain_closeout_receipt.enablement_allowed);
    assert!(!activation_audit_chain_closeout_receipt.operator_approval_recorded);
    assert!(!activation_audit_chain_closeout_receipt.reviewed_flag_enabled);
    assert!(activation_audit_chain_closeout_receipt.shadow_readiness_failed);
    assert!(
        activation_audit_chain_closeout_receipt
            .closeout_blockers
            .iter()
            .any(|blocker| blocker.starts_with(
                "canonical_projection_enablement_activation_no_live_closeout_replay_ready"
            ))
    );
    let activation_audit_chain_closeout_payload =
        serde_json::to_value(&activation_audit_chain_closeout_receipt)
            .expect("activation audit-chain closeout receipt serializes");
    let activation_audit_chain_closeout_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationAuditChainCloseoutReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_audit_chain_closeout_receipt:
                        &activation_audit_chain_closeout_receipt,
                    activation_audit_chain_closeout_receipt_payload:
                        &activation_audit_chain_closeout_payload,
                    latest_activation_audit_chain_closeout_receipt_payload: Some(&json!({
                        "decision": "stale_activation_audit_chain_closeout_receipt"
                    })),
                    activation_audit_chain_closeout_events: 1,
                    activation_audit_chain_closeout_readback_ready: true,
                    prior_activation_audit_chain_closeout_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        activation_audit_chain_closeout_replay_decision.decision,
        "work_graph_canonical_projection_enablement_activation_audit_chain_closeout_replay_mismatch_shadow_no_live_cutover"
    );
    assert!(!activation_audit_chain_closeout_replay_decision.replay_consistent);
    assert!(
        !activation_audit_chain_closeout_replay_decision
            .activation_audit_chain_closeout_matches_readback
    );
    assert!(activation_audit_chain_closeout_replay_decision.shadow_readiness_failed);
    assert!(!activation_audit_chain_closeout_replay_decision.activation_allowed);
    assert!(
            activation_audit_chain_closeout_replay_decision
                .consistency_blockers
                .iter()
                .any(|blocker| blocker.starts_with(
                    "canonical_projection_enablement_activation_audit_chain_closeout_latest_payload_matches"
                ))
        );
    let activation_operator_approval_readiness_preflight_packet =
            build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_packet(
                WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightPacketInput {
                    source_surface_id: "agent_jobs",
                    activation_audit_chain_closeout_receipt:
                        &activation_audit_chain_closeout_receipt,
                    activation_audit_chain_closeout_replay_consistency_decision:
                        &activation_audit_chain_closeout_replay_decision,
                    activation_audit_chain_closeout_events: 1,
                    activation_audit_chain_closeout_replay_consistency_events: 1,
                    prior_activation_operator_approval_readiness_preflight_packet_events: 1,
                    activation_audit_chain_closeout_readback_ready: true,
                    activation_audit_chain_closeout_replay_consistency_ready: true,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        activation_operator_approval_readiness_preflight_packet.decision,
        "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_blocked_shadow_no_live_cutover"
    );
    assert!(
        !activation_operator_approval_readiness_preflight_packet
            .activation_operator_approval_readiness_preflight_ready
    );
    assert!(
        !activation_operator_approval_readiness_preflight_packet
            .activation_audit_chain_closeout_ready
    );
    assert!(
        !activation_operator_approval_readiness_preflight_packet
            .activation_audit_chain_closeout_replay_consistent
    );
    assert!(!activation_operator_approval_readiness_preflight_packet.activation_allowed);
    assert!(!activation_operator_approval_readiness_preflight_packet.operator_approval_recorded);
    assert!(!activation_operator_approval_readiness_preflight_packet.reviewed_flag_enabled);
    assert!(activation_operator_approval_readiness_preflight_packet.shadow_readiness_failed);
    assert!(
        activation_operator_approval_readiness_preflight_packet
            .preflight_blockers
            .iter()
            .any(|blocker| blocker.starts_with(
                "canonical_projection_enablement_activation_audit_chain_closeout_replay_ready"
            ))
    );
    let activation_operator_approval_readiness_preflight_payload =
        serde_json::to_value(&activation_operator_approval_readiness_preflight_packet)
            .expect("preflight packet should serialize");
    let mismatched_activation_operator_approval_readiness_preflight_payload =
        serde_json::json!({"mismatch": true});
    let activation_operator_approval_readiness_preflight_replay_decision =
            build_work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_consistency_decision(
                WorkGraphCanonicalProjectionEnablementActivationOperatorApprovalReadinessPreflightReplayConsistencyInput {
                    source_surface_id: "agent_jobs",
                    activation_operator_approval_readiness_preflight_packet:
                        &activation_operator_approval_readiness_preflight_packet,
                    activation_operator_approval_readiness_preflight_packet_payload:
                        &activation_operator_approval_readiness_preflight_payload,
                    latest_activation_operator_approval_readiness_preflight_packet_payload:
                        Some(&mismatched_activation_operator_approval_readiness_preflight_payload),
                    activation_operator_approval_readiness_preflight_packet_events: 1,
                    activation_operator_approval_readiness_preflight_packet_readback_ready: true,
                    prior_activation_operator_approval_readiness_preflight_replay_consistency_events: 1,
                    live_blocking_event_count: 0,
                    live_cutover_event_count: 0,
                },
            );
    assert_eq!(
        activation_operator_approval_readiness_preflight_replay_decision.decision,
        "work_graph_canonical_projection_enablement_activation_operator_approval_readiness_preflight_replay_mismatch_shadow_no_live_cutover"
    );
    assert!(!activation_operator_approval_readiness_preflight_replay_decision.replay_consistent);
    assert!(
        !activation_operator_approval_readiness_preflight_replay_decision
            .activation_operator_approval_readiness_preflight_packet_matches_readback
    );
    assert!(
        activation_operator_approval_readiness_preflight_replay_decision.shadow_readiness_failed
    );
    assert!(
            activation_operator_approval_readiness_preflight_replay_decision
                .consistency_blockers
                .iter()
                .any(|blocker| blocker.starts_with(
                    "canonical_projection_enablement_activation_operator_approval_readiness_preflight_latest_payload_matches"
                ))
        );
}
