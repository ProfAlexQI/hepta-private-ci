use super::*;

pub(super) fn build_canonical_projection_row(
    row: &WorkGraphOperatorMatrixRow,
) -> WorkGraphCanonicalProjectionRow {
    let read_projection_ready = row.row_auditable
        && row.durable_fact_source_present
        && row.replay_consistent
        && row.no_live_guardrail_ready;
    WorkGraphCanonicalProjectionRow {
        source_surface_id: row.source_surface_id.clone(),
        family: row.family,
        node_kind: canonical_projection_node_kind(row),
        row_auditable: row.row_auditable,
        durable_fact_source_present: row.durable_fact_source_present,
        result_contract_ready: row.result_contract_ready,
        verifier_reducer_ready: row.verifier_reducer_ready,
        replay_consistent: row.replay_consistent,
        no_live_guardrail_ready: row.no_live_guardrail_ready,
        read_projection_ready,
        write_projection_ready: false,
        canonical_write_enabled: false,
        next_blocker: if read_projection_ready {
            "canonical_work_graph_write_disabled"
        } else {
            row.next_blocker
        },
        next_action: row.next_action,
    }
}

pub(super) fn canonical_projection_node_kind(row: &WorkGraphOperatorMatrixRow) -> &'static str {
    match row.source_surface_id.as_str() {
        "spawn_agents_on_csv" => "work_batch",
        "report_agent_job_result" => "task_result",
        "spawn_agent" | "spawn_agent_v2" => "subagent_task",
        "send_message" | "followup_task" => "handoff_edge",
        "close_agent" => "lifecycle_close",
        "wait_agent" => "wait_barrier",
        "update_plan_tool" => "plan_step",
        "plan_mode_proposed_plan" => "plan_proposal",
        "hepta_runtime_task_board" => "runtime_task",
        "hepta_runtime_worker_tasks" => "runtime_worker_task",
        "hepta_runtime_multi_agent_reducer" => "runtime_reducer",
        "hepta_runtime_scheduler_store" => "scheduler_admission",
        _ => "work_node",
    }
}

pub(super) fn build_operator_matrix_rows(
    packet: &WorkGraphSurfaceAuditPacket,
) -> Vec<WorkGraphOperatorMatrixRow> {
    packet
        .surface_entries
        .iter()
        .map(|entry| build_operator_matrix_row(packet, entry))
        .collect()
}

pub(super) fn build_operator_matrix_row(
    packet: &WorkGraphSurfaceAuditPacket,
    entry: &WorkGraphSurfaceAuditEntry,
) -> WorkGraphOperatorMatrixRow {
    let row_auditable = packet.audit_chain.chain_ready
        && packet.audit_chain.chain_replay_consistent
        && packet.audit_chain.no_live_guardrails_ready
        && !packet.feature_flag_enabled
        && packet.canary_stage == "off"
        && packet.canary_traffic_ppm == 0
        && !packet.live_blocking_enabled
        && !packet.live_cutover_enabled;
    let result_contract_ready = !entry.result_contract_required || entry.result_contract_present;
    let verifier_reducer_ready =
        !entry.result_contract_required || (entry.verifier_present && entry.reducer_present);
    let canonical_promotion_ready = row_auditable
        && entry.durable_fact_source_present
        && entry.canonical_work_graph_write_enabled
        && result_contract_ready
        && verifier_reducer_ready
        && entry.promotion_ready;
    let (readiness_status, next_blocker) = operator_matrix_row_readiness(
        row_auditable,
        entry.durable_fact_source_present,
        entry.canonical_work_graph_write_enabled,
        result_contract_ready,
        verifier_reducer_ready,
        entry.promotion_ready,
    );
    let include_task_result_plan = entry.result_contract_required
        || entry.task_result_contract_id != "task_result_contract_not_required";
    let (
        task_result_contract_plan_decision,
        task_result_contract_plan_ready,
        task_result_contract_id,
        terminal_delivery_surface,
        missing_task_result_contract_parts,
        task_result_contract_next_action,
        task_result_contract_next_action_count,
    ) = if include_task_result_plan {
        (
            Some(entry.task_result_contract_plan_decision.clone()),
            Some(entry.task_result_contract_plan_ready),
            Some(entry.task_result_contract_id.clone()),
            Some(entry.terminal_delivery_surface.clone()),
            entry.missing_task_result_contract_parts.clone(),
            entry.task_result_contract_next_actions.first().cloned(),
            Some(entry.task_result_contract_next_actions.len()),
        )
    } else {
        (None, None, None, None, Vec::new(), None, None)
    };

    WorkGraphOperatorMatrixRow {
        source_surface_id: entry.source_surface_id.clone(),
        family: entry.family,
        owner_lane: entry.owner_lane,
        observed_this_run: entry.observed_this_run,
        durable_fact_source_present: entry.durable_fact_source_present,
        canonical_work_graph_write_enabled: entry.canonical_work_graph_write_enabled,
        row_auditable,
        result_contract_ready,
        verifier_reducer_ready,
        promotion_ready: entry.promotion_ready,
        replay_consistent: packet.audit_chain.chain_replay_consistent,
        no_live_guardrail_ready: packet.audit_chain.no_live_guardrails_ready,
        canonical_promotion_ready,
        readiness_status,
        next_blocker,
        task_result_contract_plan_decision,
        task_result_contract_plan_ready,
        task_result_contract_id,
        terminal_delivery_surface,
        missing_task_result_contract_parts,
        task_result_contract_next_action,
        task_result_contract_next_action_count,
        next_action: entry.next_action,
    }
}

pub(super) fn operator_matrix_row_readiness(
    row_auditable: bool,
    durable_fact_source_present: bool,
    canonical_work_graph_write_enabled: bool,
    result_contract_ready: bool,
    verifier_reducer_ready: bool,
    promotion_ready: bool,
) -> (&'static str, &'static str) {
    if !row_auditable {
        return (
            "blocked_audit_chain_or_no_live_guardrail_not_ready",
            "audit_chain_or_no_live_guardrail_not_ready",
        );
    }
    if !result_contract_ready {
        return ("blocked_missing_result_contract", "missing_result_contract");
    }
    if !verifier_reducer_ready {
        return (
            "blocked_missing_verifier_reducer",
            "missing_verifier_reducer",
        );
    }
    if !durable_fact_source_present {
        return (
            "blocked_missing_durable_fact_source",
            "missing_durable_fact_source",
        );
    }
    if !promotion_ready {
        return ("blocked_promotion_not_ready", "promotion_not_ready");
    }
    if !canonical_work_graph_write_enabled {
        return (
            "blocked_canonical_work_graph_write_disabled",
            "canonical_work_graph_write_disabled",
        );
    }
    (
        "ready_shadow_operator_matrix_no_live_cutover",
        "none_shadow_only",
    )
}

pub(super) fn build_surface_entries(
    matrix: &WorkGraphPromotionReadinessShadowMatrix,
    role_decisions: &[WorkGraphRoleManifestShadowDecision],
) -> Vec<WorkGraphSurfaceAuditEntry> {
    let observed_surface_ids = role_decisions
        .iter()
        .map(|decision| decision.source_surface_id)
        .collect::<BTreeSet<_>>();
    let mut entries = default_agent_card_manifest_registry()
        .entries()
        .iter()
        .map(|registry_entry| {
            let role_decision = role_decisions
                .iter()
                .find(|decision| decision.source_surface_id == registry_entry.source_surface_id);
            let task_result_contract_shadow_plan = role_decision
                .map(|decision| decision.task_result_contract_shadow_plan.clone())
                .unwrap_or_else(|| {
                    build_default_task_result_contract_shadow_plan(registry_entry.manifest)
                });
            let matrix_entry = matrix
                .entries
                .iter()
                .find(|entry| entry.source_surface_id == registry_entry.source_surface_id);
            WorkGraphSurfaceAuditEntry {
                source_surface_id: registry_entry.source_surface_id.to_string(),
                family: governed_surface_family(registry_entry.source_surface_id),
                owner_lane: registry_entry.manifest.lane,
                present_in_current_head: true,
                observed_this_run: observed_surface_ids.contains(registry_entry.source_surface_id),
                durable_fact_source_present: governed_surface_durable_fact_source_present(
                    registry_entry.source_surface_id,
                ),
                canonical_work_graph_write_enabled: false,
                shadow_only: true,
                result_contract_required: registry_entry.manifest.result_contract_required,
                result_contract_present: role_decision.map_or(
                    registry_entry.manifest.result_contract_present,
                    |decision| decision.result_contract_present,
                ),
                verifier_present: role_decision
                    .map_or(registry_entry.manifest.verifier_present, |decision| {
                        decision.verifier_present
                    }),
                reducer_present: role_decision
                    .map_or(registry_entry.manifest.reducer_present, |decision| {
                        decision.reducer_present
                    }),
                role_manifest_decision: role_decision
                    .map(|decision| decision.decision.to_string())
                    .unwrap_or_else(|| "not_observed_this_run".to_string()),
                promotion_readiness_decision: matrix_entry
                    .map(|entry| entry.promotion_readiness_decision.to_string())
                    .unwrap_or_else(|| "not_observed_this_run".to_string()),
                promotion_ready: matrix_entry.is_some_and(|entry| entry.promotion_ready),
                task_result_contract_plan_decision: task_result_contract_shadow_plan
                    .decision
                    .to_string(),
                task_result_contract_plan_ready: task_result_contract_shadow_plan
                    .contract_plan_ready,
                task_result_contract_id: task_result_contract_shadow_plan
                    .task_result_contract_id
                    .to_string(),
                terminal_delivery_surface: task_result_contract_shadow_plan
                    .terminal_delivery_surface
                    .to_string(),
                missing_task_result_contract_parts: task_result_contract_shadow_plan
                    .missing_contract_parts,
                task_result_contract_next_actions: task_result_contract_shadow_plan.next_actions,
                next_action: governed_surface_next_action(registry_entry.source_surface_id),
            }
        })
        .collect::<Vec<_>>();
    entries.extend(non_governed_surface_entries());
    entries
}

pub(super) fn build_audit_chain_summary(
    readback: &AgentJobWorkGraphAuditChainReadback,
) -> WorkGraphAuditChainSummary {
    let segments = readback
        .segments
        .iter()
        .map(|segment| WorkGraphAuditChainSegment {
            segment_id: segment.segment_id.clone(),
            event_type: segment.event_type.clone(),
            event_count: segment.event_count,
            latest_payload_present: segment.latest_payload.is_some(),
            latest_decision: segment.latest_decision.clone(),
            readback_ready: segment.readback_ready,
            replay_consistent: segment.replay_consistent,
            no_live_guardrail_ready: segment.no_live_guardrail_ready,
            ready: segment.ready,
        })
        .collect::<Vec<_>>();
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

    WorkGraphAuditChainSummary {
        segment_count: segments.len(),
        ready_segment_count,
        missing_segment_ids,
        inconsistent_segment_ids,
        chain_readback_ready: readback.chain_readback_ready,
        chain_replay_consistent: readback.chain_replay_consistent,
        no_live_guardrails_ready: readback.no_live_guardrails_ready,
        chain_ready: readback.chain_ready,
        segments,
    }
}

pub(super) fn build_direct_wait_surface_audit_entry(
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
    operator_row: Option<&WorkGraphOperatorMatrixRow>,
) -> WorkGraphSurfaceAuditEntry {
    let result_contract_present =
        readback.is_some_and(|readback| readback.task_result_delivery_ready);
    let verifier_reducer_present =
        readback.is_some_and(|readback| readback.parent_reducer_receipt_ready);
    let direct_wait_surface_audit_ready =
        readback.is_some_and(|readback| readback.direct_wait_surface_audit_ready);
    let task_result_contract_plan_ready = operator_row
        .and_then(|row| row.task_result_contract_plan_ready)
        .unwrap_or(result_contract_present && verifier_reducer_present);
    let mut missing_task_result_contract_parts = operator_row
        .map(|row| row.missing_task_result_contract_parts.clone())
        .unwrap_or_default();
    if operator_row.is_none() {
        if !result_contract_present {
            missing_task_result_contract_parts.push("task_result_delivery_shadow".to_string());
        }
        if !verifier_reducer_present {
            missing_task_result_contract_parts.push("parent_reducer_shadow_receipt".to_string());
        }
    }
    missing_task_result_contract_parts.sort();
    missing_task_result_contract_parts.dedup();
    let next_action = if direct_wait_surface_audit_ready {
        "project direct wait surface-audit evidence into canonical WorkGraph write/read projection"
    } else {
        "complete direct wait delivery, reducer, replay, and surface-audit shadow readback"
    };
    let task_result_contract_next_actions = operator_row
        .and_then(|row| row.task_result_contract_next_action.clone())
        .map_or_else(|| vec![next_action.to_string()], |action| vec![action]);

    WorkGraphSurfaceAuditEntry {
        source_surface_id: "wait_agent".to_string(),
        family: "subagent_lifecycle",
        owner_lane: "subagent_lifecycle",
        present_in_current_head: true,
        observed_this_run: readback.is_some(),
        durable_fact_source_present: readback.is_some_and(|readback| readback.readback_ready),
        canonical_work_graph_write_enabled: false,
        shadow_only: true,
        result_contract_required: true,
        result_contract_present,
        verifier_present: verifier_reducer_present,
        reducer_present: verifier_reducer_present,
        role_manifest_decision: if readback.is_some() {
            "direct_wait_task_result_readback_observed_shadow_no_live_cutover".to_string()
        } else {
            "direct_wait_task_result_readback_missing_shadow_no_live_cutover".to_string()
        },
        promotion_readiness_decision: if direct_wait_surface_audit_ready {
            "direct_wait_surface_audit_ready_shadow_no_live_cutover".to_string()
        } else {
            "direct_wait_surface_audit_blocked_shadow_no_live_cutover".to_string()
        },
        promotion_ready: direct_wait_surface_audit_ready,
        task_result_contract_plan_decision: operator_row
            .and_then(|row| row.task_result_contract_plan_decision.clone())
            .unwrap_or_else(|| {
                if task_result_contract_plan_ready {
                    "task_result_delivery_readback_ready_shadow_no_live_cutover".to_string()
                } else {
                    "task_result_delivery_readback_blocked_shadow_no_live_cutover".to_string()
                }
            }),
        task_result_contract_plan_ready,
        task_result_contract_id: operator_row
            .and_then(|row| row.task_result_contract_id.clone())
            .unwrap_or_else(|| "subagent_task_result_contract_v1".to_string()),
        terminal_delivery_surface: operator_row
            .and_then(|row| row.terminal_delivery_surface.clone())
            .unwrap_or_else(|| "wait_agent(result_required=true)".to_string()),
        missing_task_result_contract_parts,
        task_result_contract_next_actions,
        next_action,
    }
}

pub(super) fn build_direct_wait_global_audit_chain_summary(
    readback: Option<&codex_state::InterAgentWaitTaskResultReadback>,
) -> WorkGraphAuditChainSummary {
    let no_live_guardrails_ready = readback.is_none_or(|readback| {
        readback.live_blocking_event_count == 0 && readback.live_cutover_event_count == 0
    });
    let segments = vec![
        direct_wait_global_audit_chain_segment(DirectWaitGlobalAuditChainSegmentInput {
            segment_id: "wait_task_result_delivery_shadow",
            event_type: "wait_task_result_delivery_shadow",
            event_count: readback
                .map(|readback| readback.task_result_delivery_shadow_events)
                .unwrap_or_default(),
            latest_payload_present: readback
                .is_some_and(|readback| readback.latest_task_result_delivery_shadow.is_some()),
            latest_decision: readback
                .map(|readback| readback.latest_task_result_delivery_decision.clone())
                .unwrap_or_else(|| "missing".to_string()),
            readback_ready: readback
                .is_some_and(|readback| readback.task_result_delivery_readback_ready),
            replay_consistent: true,
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
        direct_wait_global_audit_chain_segment(DirectWaitGlobalAuditChainSegmentInput {
            segment_id: "wait_parent_reducer_shadow_receipt",
            event_type: "wait_parent_reducer_shadow_receipt",
            event_count: readback
                .map(|readback| readback.parent_reducer_shadow_receipt_events)
                .unwrap_or_default(),
            latest_payload_present: readback
                .is_some_and(|readback| readback.latest_parent_reducer_shadow_receipt.is_some()),
            latest_decision: readback
                .map(|readback| readback.latest_parent_reducer_decision.clone())
                .unwrap_or_else(|| "missing".to_string()),
            readback_ready: readback.is_some_and(|readback| readback.parent_reducer_readback_ready),
            replay_consistent: true,
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
        direct_wait_global_audit_chain_segment(DirectWaitGlobalAuditChainSegmentInput {
            segment_id: "wait_task_result_replay_consistency",
            event_type: "wait_task_result_replay_consistency",
            event_count: readback
                .map(|readback| readback.task_result_replay_consistency_events)
                .unwrap_or_default(),
            latest_payload_present: readback
                .is_some_and(|readback| readback.latest_task_result_replay_consistency.is_some()),
            latest_decision: readback
                .map(|readback| {
                    readback
                        .latest_task_result_replay_consistency_decision
                        .clone()
                })
                .unwrap_or_else(|| "missing".to_string()),
            readback_ready: readback.is_some_and(|readback| readback.replay_consistency_ready),
            replay_consistent: readback.is_some_and(|readback| readback.replay_consistent),
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
        direct_wait_global_audit_chain_segment(DirectWaitGlobalAuditChainSegmentInput {
            segment_id: "wait_surface_audit_packet",
            event_type: "wait_surface_audit_packet",
            event_count: readback
                .map(|readback| readback.wait_surface_audit_packet_events)
                .unwrap_or_default(),
            latest_payload_present: readback
                .is_some_and(|readback| readback.latest_wait_surface_audit_packet.is_some()),
            latest_decision: readback
                .map(|readback| readback.latest_wait_surface_audit_decision.clone())
                .unwrap_or_else(|| "missing".to_string()),
            readback_ready: readback
                .is_some_and(|readback| readback.wait_surface_audit_packet_readback_ready),
            replay_consistent: readback
                .is_some_and(|readback| readback.wait_surface_audit_packet_ready),
            no_live_guardrail_ready: no_live_guardrails_ready,
        }),
        direct_wait_global_audit_chain_segment(DirectWaitGlobalAuditChainSegmentInput {
            segment_id: "wait_surface_audit_replay_consistency",
            event_type: "wait_surface_audit_replay_consistency",
            event_count: readback
                .map(|readback| readback.wait_surface_audit_replay_consistency_events)
                .unwrap_or_default(),
            latest_payload_present: readback.is_some_and(|readback| {
                readback
                    .latest_wait_surface_audit_replay_consistency
                    .is_some()
            }),
            latest_decision: readback
                .map(|readback| {
                    readback
                        .latest_wait_surface_audit_replay_consistency_decision
                        .clone()
                })
                .unwrap_or_else(|| "missing".to_string()),
            readback_ready: readback
                .is_some_and(|readback| readback.wait_surface_audit_replay_consistency_ready),
            replay_consistent: readback
                .is_some_and(|readback| readback.wait_surface_audit_replay_consistent),
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

struct DirectWaitGlobalAuditChainSegmentInput<'a> {
    segment_id: &'a str,
    event_type: &'a str,
    event_count: usize,
    latest_payload_present: bool,
    latest_decision: String,
    readback_ready: bool,
    replay_consistent: bool,
    no_live_guardrail_ready: bool,
}

fn direct_wait_global_audit_chain_segment(
    input: DirectWaitGlobalAuditChainSegmentInput<'_>,
) -> WorkGraphAuditChainSegment {
    WorkGraphAuditChainSegment {
        segment_id: input.segment_id.to_string(),
        event_type: input.event_type.to_string(),
        event_count: input.event_count,
        latest_payload_present: input.latest_payload_present,
        latest_decision: input.latest_decision,
        readback_ready: input.readback_ready,
        replay_consistent: input.replay_consistent,
        no_live_guardrail_ready: input.no_live_guardrail_ready,
        ready: input.readback_ready && input.replay_consistent && input.no_live_guardrail_ready,
    }
}

pub(super) fn governed_surface_family(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "spawn_agents_on_csv" | "report_agent_job_result" => "agent_jobs",
        "spawn_agent" | "spawn_agent_v2" => "subagent_spawn",
        "send_message" | "followup_task" => "subagent_handoff",
        "close_agent" | "wait_agent" => "subagent_lifecycle",
        _ => "governed_tool",
    }
}

pub(super) fn governed_surface_durable_fact_source_present(source_surface_id: &str) -> bool {
    matches!(
        source_surface_id,
        "spawn_agents_on_csv" | "report_agent_job_result" | "wait_agent"
    )
}

pub(super) fn governed_surface_next_action(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "spawn_agents_on_csv" | "report_agent_job_result" => {
            "project agent_jobs TaskResult events into canonical WorkGraph nodes"
        }
        "spawn_agent" | "spawn_agent_v2" => {
            "require TaskResult, verifier, reducer, and parent-close policy before promotion"
        }
        "send_message" | "followup_task" => {
            "attach durable handoff edges and input schemas to canonical WorkGraph"
        }
        "close_agent" | "wait_agent" => {
            "project lifecycle barriers and terminal waits into canonical WorkGraph events"
        }
        _ => "add canonical WorkGraph projection adapter",
    }
}

pub(super) fn non_governed_surface_entries() -> Vec<WorkGraphSurfaceAuditEntry> {
    vec![
        non_governed_surface(
            "update_plan_tool",
            "planning",
            "planning",
            false,
            false,
            "project checklist steps as durable PlanStep nodes with stable ids",
        ),
        non_governed_surface(
            "plan_mode_proposed_plan",
            "planning",
            "planning",
            false,
            false,
            "project proposed plan blocks into non-mutating graph proposals",
        ),
        non_governed_surface(
            "hepta_runtime_task_board",
            "hepta_runtime",
            "runtime_task_board",
            true,
            false,
            "adapt task_board tasks, dependencies, leases, and terminal events into WorkGraph",
        ),
        non_governed_surface(
            "hepta_runtime_worker_tasks",
            "hepta_runtime",
            "runtime_worker_tasks",
            false,
            true,
            "wrap worker task results, artifacts, patches, and command evidence in TaskResultEnvelope",
        ),
        non_governed_surface(
            "hepta_runtime_multi_agent_reducer",
            "hepta_runtime",
            "runtime_multi_agent",
            false,
            true,
            "map reducer consensus and agent runtime pool evidence into canonical TaskResult reducers",
        ),
        non_governed_surface(
            "hepta_runtime_scheduler_store",
            "hepta_runtime",
            "runtime_scheduler",
            true,
            false,
            "route scheduler admission decisions through the canonical WorkGraph admission controller",
        ),
    ]
}

pub(super) fn non_governed_surface(
    source_surface_id: &'static str,
    family: &'static str,
    owner_lane: &'static str,
    durable_fact_source_present: bool,
    result_contract_required: bool,
    next_action: &'static str,
) -> WorkGraphSurfaceAuditEntry {
    let missing_task_result_contract_parts = if result_contract_required {
        vec![
            "task_result_contract".to_string(),
            "verifier".to_string(),
            "reducer".to_string(),
        ]
    } else {
        Vec::new()
    };
    WorkGraphSurfaceAuditEntry {
        source_surface_id: source_surface_id.to_string(),
        family,
        owner_lane,
        present_in_current_head: true,
        observed_this_run: false,
        durable_fact_source_present,
        canonical_work_graph_write_enabled: false,
        shadow_only: true,
        result_contract_required,
        result_contract_present: false,
        verifier_present: false,
        reducer_present: false,
        role_manifest_decision: "not_governed_by_agent_card_registry".to_string(),
        promotion_readiness_decision: "not_observed_this_run".to_string(),
        promotion_ready: false,
        task_result_contract_plan_decision:
            "task_result_contract_plan_not_governed_shadow_no_live_cutover".to_string(),
        task_result_contract_plan_ready: !result_contract_required,
        task_result_contract_id: "not_governed_by_agent_card_registry".to_string(),
        terminal_delivery_surface: "canonical_work_graph_adapter_required".to_string(),
        missing_task_result_contract_parts,
        task_result_contract_next_actions: vec![next_action.to_string()],
        next_action,
    }
}

pub(super) fn build_optimization_blockers(
    canonical_write_enabled_count: usize,
    result_contract_gap_count: usize,
    verifier_reducer_gap_count: usize,
    entries: &[WorkGraphSurfaceAuditEntry],
) -> Vec<String> {
    let mut blockers = Vec::new();
    if canonical_write_enabled_count == 0 {
        blockers.push(
            "no audited source surface writes canonical WorkGraph nodes yet; all entries remain shadow/projection-only"
                .to_string(),
        );
    }
    if result_contract_gap_count > 0 {
        blockers.push(format!(
            "{result_contract_gap_count} audited source surface(s) still require TaskResult contract coverage"
        ));
    }
    if verifier_reducer_gap_count > 0 {
        blockers.push(format!(
            "{verifier_reducer_gap_count} audited source surface(s) still lack verifier/reducer coverage"
        ));
    }
    blockers.extend(
        entries
            .iter()
            .filter(|entry| entry.result_contract_required && !entry.result_contract_present)
            .map(|entry| format!("{}: {}", entry.source_surface_id, entry.next_action)),
    );
    blockers
}
