use serde::Serialize;

pub const WORK_GRAPH_CURRENT_STATE_INVENTORY_GATE: &str =
    "hepta_work_graph_current_state_inventory_gate";
pub const WORK_GRAPH_CURRENT_STATE_INVENTORY_SCHEMA_VERSION: &str =
    "work_graph_current_state_inventory_v1";
pub const WORK_GRAPH_CURRENT_STATE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_contract_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCurrentStateInventoryReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub system_status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub inventory_mode: &'static str,
    pub source_surface_count: usize,
    pub active_p0_gap_count: usize,
    pub source_surfaces: Vec<WorkGraphSourceSurfaceInventory>,
    pub active_p0_gaps: Vec<WorkGraphP0GapInventory>,
    pub recommended_next_gate: &'static str,
    pub ready_for_work_graph_contract_preview: bool,
    pub ready_for_scheduler_cutover: bool,
    pub side_effects: WorkGraphInventorySideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSourceSurfaceInventory {
    pub id: &'static str,
    pub kind: &'static str,
    pub present_in_current_head: bool,
    pub durable_fact_source_present: bool,
    pub partial_graph_edge_store_present: bool,
    pub work_graph_adapter_present: bool,
    pub task_result_adapter_present: bool,
    pub trace_adapter_present: bool,
    pub blocker_ids: Vec<&'static str>,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphP0GapInventory {
    pub id: &'static str,
    pub severity: &'static str,
    pub source_surface_ids: Vec<&'static str>,
    pub reason: &'static str,
    pub recommended_next_action: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphInventorySideEffects {
    pub filesystem_written: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub agent_spawn_performed: bool,
    pub gateway_mutation_performed: bool,
    pub credential_read: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_current_state_inventory_report() -> WorkGraphCurrentStateInventoryReport {
    let source_surfaces = work_graph_current_state_source_surfaces();
    let active_p0_gaps = work_graph_current_state_p0_gaps();
    WorkGraphCurrentStateInventoryReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        system_status: "attention",
        gate: WORK_GRAPH_CURRENT_STATE_INVENTORY_GATE,
        schema_version: WORK_GRAPH_CURRENT_STATE_INVENTORY_SCHEMA_VERSION,
        inventory_mode: "read_only_current_head_contract_inventory",
        source_surface_count: source_surfaces.len(),
        active_p0_gap_count: active_p0_gaps.len(),
        source_surfaces,
        active_p0_gaps,
        recommended_next_gate: WORK_GRAPH_CURRENT_STATE_RECOMMENDED_NEXT_GATE,
        ready_for_work_graph_contract_preview: true,
        ready_for_scheduler_cutover: false,
        side_effects: WorkGraphInventorySideEffects::none(),
    }
}

pub fn work_graph_current_state_source_surfaces() -> Vec<WorkGraphSourceSurfaceInventory> {
    vec![
        WorkGraphSourceSurfaceInventory {
            id: "update_plan_tool",
            kind: "planning_notification",
            present_in_current_head: true,
            durable_fact_source_present: false,
            partial_graph_edge_store_present: false,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec![
                "unified_work_graph_state_store_not_enforced",
                "plan_step_identity_projection_missing",
            ],
            note: "Checklist steps are emitted as turn events, not durable plan nodes.",
        },
        WorkGraphSourceSurfaceInventory {
            id: "plan_mode_proposed_plan",
            kind: "human_review_plan",
            present_in_current_head: true,
            durable_fact_source_present: false,
            partial_graph_edge_store_present: false,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec!["plan_mode_projection_adapter_missing"],
            note: "Plan Mode has strong non-mutating rules but no executable graph contract.",
        },
        WorkGraphSourceSurfaceInventory {
            id: "app_server_turn_plan_notification",
            kind: "app_server_notification",
            present_in_current_head: true,
            durable_fact_source_present: false,
            partial_graph_edge_store_present: false,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec!["turn_plan_notification_work_graph_projection_missing"],
            note: "The app-server mirrors plan steps as notifications without graph semantics.",
        },
        WorkGraphSourceSurfaceInventory {
            id: "multi_agent_v2_thread_spawn",
            kind: "agent_orchestration",
            present_in_current_head: true,
            durable_fact_source_present: false,
            partial_graph_edge_store_present: true,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec![
                "agent_task_lifecycle_fact_source_missing",
                "role_manifest_contract_not_enforced",
            ],
            note: "Thread spawn edges are partially durable, but agent tasks are not WorkGraph nodes.",
        },
        WorkGraphSourceSurfaceInventory {
            id: "multi_agent_v2_mailbox_wait",
            kind: "agent_orchestration",
            present_in_current_head: true,
            durable_fact_source_present: false,
            partial_graph_edge_store_present: false,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec!["target_status_wait_contract_missing"],
            note: "wait_agent waits for mailbox progress, not named task terminal states.",
        },
        WorkGraphSourceSurfaceInventory {
            id: "codex_agent_graph_store",
            kind: "partial_graph_store",
            present_in_current_head: true,
            durable_fact_source_present: true,
            partial_graph_edge_store_present: true,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec!["graph_store_only_tracks_thread_spawn_edges"],
            note: "The store tracks parent/child thread edges, not plan nodes, artifacts, gates, or results.",
        },
        WorkGraphSourceSurfaceInventory {
            id: "agent_jobs_batch_workers",
            kind: "batch_worker_orchestration",
            present_in_current_head: true,
            durable_fact_source_present: true,
            partial_graph_edge_store_present: false,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec!["task_result_contract_not_enforced_for_agent_jobs"],
            note: "Agent jobs persist item results, but output_schema is not the unified TaskResult contract.",
        },
        WorkGraphSourceSurfaceInventory {
            id: "hepta_runtime_task_board",
            kind: "runtime_task_queue",
            present_in_current_head: true,
            durable_fact_source_present: true,
            partial_graph_edge_store_present: false,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec!["task_board_work_graph_adapter_missing"],
            note: "Task board already has dependencies, leases, workers, and terminal delivery guards.",
        },
        WorkGraphSourceSurfaceInventory {
            id: "hepta_runtime_worker_tasks",
            kind: "runtime_worker_task_model",
            present_in_current_head: true,
            durable_fact_source_present: false,
            partial_graph_edge_store_present: false,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec!["worker_task_task_result_adapter_missing"],
            note: "Worker tasks model artifacts, commands, safety, and patches but are not unified results.",
        },
        WorkGraphSourceSurfaceInventory {
            id: "hepta_runtime_scheduler_store",
            kind: "runtime_scheduler_store",
            present_in_current_head: true,
            durable_fact_source_present: true,
            partial_graph_edge_store_present: false,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec!["scheduler_admission_controller_not_enforced"],
            note: "Scheduler records jobs, runs, wakes, readback, and idempotency but not graph admission.",
        },
        WorkGraphSourceSurfaceInventory {
            id: "hepta_runtime_agent_harness",
            kind: "runtime_agent_harness",
            present_in_current_head: true,
            durable_fact_source_present: true,
            partial_graph_edge_store_present: false,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec!["agent_harness_work_graph_projection_missing"],
            note: "Harness ledger has lineage and readback but no shared WorkGraph projection.",
        },
        WorkGraphSourceSurfaceInventory {
            id: "hepta_runtime_multi_agent_reducer",
            kind: "runtime_multi_agent_demo",
            present_in_current_head: true,
            durable_fact_source_present: false,
            partial_graph_edge_store_present: false,
            work_graph_adapter_present: false,
            task_result_adapter_present: false,
            trace_adapter_present: false,
            blocker_ids: vec!["semantic_task_result_reducer_missing"],
            note: "Reducer modes exist, but reducer evidence is not a production TaskResult pipeline.",
        },
    ]
}

pub fn work_graph_current_state_p0_gaps() -> Vec<WorkGraphP0GapInventory> {
    vec![
        WorkGraphP0GapInventory {
            id: "unified_work_graph_state_store_not_enforced",
            severity: "p0",
            source_surface_ids: vec![
                "update_plan_tool",
                "multi_agent_v2_thread_spawn",
                "hepta_runtime_task_board",
                "hepta_runtime_scheduler_store",
            ],
            reason: "Planning, agent orchestration, runtime tasks, and scheduler runs cannot be queried through one enforced durable fact graph.",
            recommended_next_action: "Use the unified state store preview to add adapter projection fixtures before enabling persistence.",
        },
        WorkGraphP0GapInventory {
            id: "task_result_contract_not_enforced",
            severity: "p0",
            source_surface_ids: vec![
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_multi_agent_reducer",
            ],
            reason: "Subagent completions, batch item results, worker artifacts, and reducer decisions do not share one result schema.",
            recommended_next_action: "Use the TaskResult preview contract to add adapter validator coverage before enabling enforcement.",
        },
        WorkGraphP0GapInventory {
            id: "scheduler_admission_controller_not_enforced",
            severity: "p0",
            source_surface_ids: vec![
                "hepta_runtime_scheduler_store",
                "hepta_runtime_task_board",
                "multi_agent_v2_thread_spawn",
            ],
            reason: "No single enforced admission check gates dependencies, leases, lane ownership, approval, idempotency, and budget before work starts.",
            recommended_next_action: "Use the dry-run admission preview to add fixture coverage before enabling scheduler enforcement.",
        },
        WorkGraphP0GapInventory {
            id: "work_graph_observability_timeline_not_enforced",
            severity: "p0",
            source_surface_ids: vec![
                "app_server_turn_plan_notification",
                "multi_agent_v2_thread_spawn",
                "agent_jobs_batch_workers",
                "hepta_runtime_agent_harness",
            ],
            reason: "There is no enforced unified trace from plan step to agent, mailbox, tool, artifact, gate, and result.",
            recommended_next_action: "Use the timeline preview to add local trace fixtures before dashboards or external exports.",
        },
        WorkGraphP0GapInventory {
            id: "role_manifest_contract_not_enforced",
            severity: "p0",
            source_surface_ids: vec!["multi_agent_v2_thread_spawn"],
            reason: "Agent roles are not enforced through declared capabilities, tool permissions, output schemas, verifiers, budgets, and lanes.",
            recommended_next_action: "Use the role manifest preview to add role adapter fixtures before enabling permission enforcement.",
        },
    ]
}

impl WorkGraphInventorySideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            agent_spawn_performed: false,
            gateway_mutation_performed: false,
            credential_read: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_state_inventory_lists_planning_multi_agent_and_runtime_surfaces() {
        let report = hepta_work_graph_current_state_inventory_report();

        let surface_ids = report
            .source_surfaces
            .iter()
            .map(|surface| surface.id)
            .collect::<Vec<_>>();

        assert_eq!(
            surface_ids,
            [
                "update_plan_tool",
                "plan_mode_proposed_plan",
                "app_server_turn_plan_notification",
                "multi_agent_v2_thread_spawn",
                "multi_agent_v2_mailbox_wait",
                "codex_agent_graph_store",
                "agent_jobs_batch_workers",
                "hepta_runtime_task_board",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_agent_harness",
                "hepta_runtime_multi_agent_reducer",
            ]
        );
        assert_eq!(report.source_surface_count, 12);
    }

    #[test]
    fn current_state_inventory_keeps_cutover_blocked_but_contract_preview_ready() {
        let report = hepta_work_graph_current_state_inventory_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.system_status, "attention");
        assert!(report.ready_for_work_graph_contract_preview);
        assert!(!report.ready_for_scheduler_cutover);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_CURRENT_STATE_RECOMMENDED_NEXT_GATE
        );
    }

    #[test]
    fn current_state_inventory_names_the_active_p0_gaps() {
        let report = hepta_work_graph_current_state_inventory_report();
        let gap_ids = report
            .active_p0_gaps
            .iter()
            .map(|gap| gap.id)
            .collect::<Vec<_>>();

        assert_eq!(
            gap_ids,
            [
                "unified_work_graph_state_store_not_enforced",
                "task_result_contract_not_enforced",
                "scheduler_admission_controller_not_enforced",
                "work_graph_observability_timeline_not_enforced",
                "role_manifest_contract_not_enforced",
            ]
        );
        assert_eq!(report.active_p0_gap_count, 5);
    }

    #[test]
    fn current_state_inventory_is_read_only() {
        let side_effects = hepta_work_graph_current_state_inventory_report().side_effects;

        assert_eq!(side_effects, WorkGraphInventorySideEffects::none());
    }
}
