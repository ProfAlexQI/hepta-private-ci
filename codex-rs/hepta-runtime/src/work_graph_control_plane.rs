use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_RECOMMENDED_NEXT_GATE;
use crate::WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_READBACK_GATE;
use crate::WORK_GRAPH_CANONICAL_STATUS_SUMMARY_READBACK_GATE;
use crate::WorkGraphAppendOnlyEventStoreFeatureGatedWalPreconditionReport;
use crate::WorkGraphCanonicalSchemaTypesReport;
use crate::WorkNode;
use crate::hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_report;
use crate::hepta_work_graph_canonical_schema_types_report_only_report;
use crate::work_graph_schema_registry::WorkGraphCanonicalSchemaFixtureReportGenerationReport;
use crate::work_graph_schema_registry::hepta_work_graph_canonical_schema_fixture_report_generation_report;

pub const WORK_GRAPH_CONTROL_PLANE_STATUS_FRONTIER_GATE: &str =
    "hepta_work_graph_control_plane_status_frontier_gate";
pub const WORK_GRAPH_CONTROL_PLANE_STATUS_FRONTIER_SCHEMA_VERSION: &str =
    "work_graph_control_plane_status_frontier_v1";
pub const WORK_GRAPH_CONTROL_PLANE_STATUS_FRONTIER_RECOMMENDED_NEXT_GATE: &str =
    WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_RECOMMENDED_NEXT_GATE;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphControlPlaneReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_report_count: usize,
    pub source_reports: Vec<WorkGraphControlPlaneSourceReport>,
    pub status_node_count: usize,
    pub status_nodes: Vec<WorkNode>,
    pub frontier_candidate_count: usize,
    pub frontier_candidates: Vec<WorkGraphFrontierCandidate>,
    pub blocker_count: usize,
    pub blockers: Vec<WorkGraphControlPlaneBlocker>,
    pub required_prior_gates: Vec<String>,
    pub required_prior_gate_count: usize,
    pub recommended_next_gate: &'static str,
    pub ready_for_read_only_workgraph_status_frontier: bool,
    pub ready_for_append_only_event_store_feature_gated_wal_precondition_readback: bool,
    pub ready_for_append_only_work_graph_event_store: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_feature_flag_enablement: bool,
    pub ready_for_canary: bool,
    pub ready_for_cutover: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphControlPlaneSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphControlPlaneSourceReport {
    pub source_id: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub status: &'static str,
    pub source_report_hash: String,
    pub ready: bool,
    pub no_write_confirmed: bool,
    pub recommended_next_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphFrontierCandidate {
    pub id: &'static str,
    pub priority: usize,
    pub optimization_stage: &'static str,
    pub work_node_id: &'static str,
    pub summary: &'static str,
    pub recommended_gate: &'static str,
    pub next_action: &'static str,
    pub blocked_by: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphControlPlaneBlocker {
    pub id: String,
    pub severity: &'static str,
    pub surface: &'static str,
    pub summary: String,
    pub blocks_live_execution: bool,
    pub recommended_fix: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphControlPlaneInspection {
    pub query: String,
    pub found: bool,
    pub target_kind: Option<&'static str>,
    pub status_node: Option<WorkNode>,
    pub frontier_candidate: Option<WorkGraphFrontierCandidate>,
    pub source_report: Option<WorkGraphControlPlaneSourceReport>,
    pub blocker: Option<WorkGraphControlPlaneBlocker>,
    pub matching_blockers: Vec<WorkGraphControlPlaneBlocker>,
    pub next_action: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphControlPlaneSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub event_store_enabled: bool,
    pub projection_index_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub replay_executed: bool,
    pub replay_diff_persisted: bool,
    pub idempotency_index_mutated: bool,
    pub scheduler_admission_enforced: bool,
    pub scheduler_live_blocking_enabled: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub role_manifest_persisted: bool,
    pub operator_approval_requested: bool,
    pub operator_approval_recorded: bool,
    pub feature_flag_enabled: bool,
    pub canary_started: bool,
    pub cutover_performed: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_control_plane_status_frontier_report() -> WorkGraphControlPlaneReport {
    let schema_report = hepta_work_graph_canonical_schema_types_report_only_report();
    let fixture_report = hepta_work_graph_canonical_schema_fixture_report_generation_report();
    let wal_report =
        hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_report();

    let source_reports =
        work_graph_control_plane_source_reports(&schema_report, &fixture_report, &wal_report);
    let status_nodes = work_graph_control_plane_status_nodes_from(
        &source_reports,
        &schema_report,
        &fixture_report,
        &wal_report,
    );
    let frontier_candidates = work_graph_control_plane_frontier_candidates();
    let blockers =
        work_graph_control_plane_blockers_from(&schema_report, &fixture_report, &wal_report);
    let required_prior_gates = work_graph_control_plane_required_prior_gates(
        &source_reports,
        &schema_report,
        &fixture_report,
        &wal_report,
    );

    WorkGraphControlPlaneReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_CONTROL_PLANE_STATUS_FRONTIER_GATE,
        schema_version: WORK_GRAPH_CONTROL_PLANE_STATUS_FRONTIER_SCHEMA_VERSION,
        preview_mode: "read_only_workgraph_status_frontier_inspect_no_writes",
        source_report_count: source_reports.len(),
        source_reports,
        status_node_count: status_nodes.len(),
        status_nodes,
        frontier_candidate_count: frontier_candidates.len(),
        frontier_candidates,
        blocker_count: blockers.len(),
        blockers,
        required_prior_gate_count: required_prior_gates.len(),
        required_prior_gates,
        recommended_next_gate: WORK_GRAPH_CONTROL_PLANE_STATUS_FRONTIER_RECOMMENDED_NEXT_GATE,
        ready_for_read_only_workgraph_status_frontier: true,
        ready_for_append_only_event_store_feature_gated_wal_precondition_readback: true,
        ready_for_append_only_work_graph_event_store: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_task_result_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_feature_flag_enablement: false,
        ready_for_canary: false,
        ready_for_cutover: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphControlPlaneSideEffects::none(),
    }
}

pub fn work_graph_control_plane_inspect(query: &str) -> WorkGraphControlPlaneInspection {
    let report = hepta_work_graph_control_plane_status_frontier_report();
    let status_node = report
        .status_nodes
        .iter()
        .find(|node| {
            node.work_node_id == query
                || node.gate_id == query
                || node.lineage_id == query
                || node.source_surface_id == query
        })
        .cloned();
    let frontier_candidate = report
        .frontier_candidates
        .iter()
        .find(|candidate| {
            candidate.id == query
                || candidate.work_node_id == query
                || candidate.recommended_gate == query
        })
        .cloned();
    let source_report = report
        .source_reports
        .iter()
        .find(|source| source.source_id == query || source.gate == query)
        .cloned();
    let blocker = report
        .blockers
        .iter()
        .find(|blocker| blocker.id == query)
        .cloned();

    let matching_blockers = status_node
        .as_ref()
        .map(|node| {
            report
                .blockers
                .iter()
                .filter(|blocker| node.next_action.contains(blocker.surface))
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    let target_kind = if status_node.is_some() {
        Some("status_node")
    } else if frontier_candidate.is_some() {
        Some("frontier_candidate")
    } else if source_report.is_some() {
        Some("source_report")
    } else if blocker.is_some() {
        Some("blocker")
    } else {
        None
    };
    let next_action = status_node
        .as_ref()
        .map(|node| node.next_action.clone())
        .or_else(|| frontier_candidate.map(|candidate| candidate.next_action.to_string()))
        .or_else(|| {
            blocker
                .as_ref()
                .map(|blocker| blocker.recommended_fix.clone())
        });

    WorkGraphControlPlaneInspection {
        query: query.to_string(),
        found: target_kind.is_some(),
        target_kind,
        status_node,
        frontier_candidate: report.frontier_candidates.into_iter().find(|candidate| {
            candidate.id == query
                || candidate.work_node_id == query
                || candidate.recommended_gate == query
        }),
        source_report,
        blocker,
        matching_blockers,
        next_action,
    }
}

pub fn work_graph_control_plane_status_nodes() -> Vec<WorkNode> {
    hepta_work_graph_control_plane_status_frontier_report().status_nodes
}

pub fn work_graph_control_plane_frontier_candidates() -> Vec<WorkGraphFrontierCandidate> {
    vec![
        frontier_candidate(
            "p1.workgraph_status_frontier_cli",
            1,
            "P1",
            "wg.control_plane.status_frontier_cli.v1",
            "use a read-only WorkGraph status/frontier/inspect CLI as the short-ID query entry",
            WORK_GRAPH_CONTROL_PLANE_STATUS_FRONTIER_GATE,
            "verify CLI output and use it before extending any long report chain",
            vec![],
        ),
        frontier_candidate(
            "p2.schema_backed_fixture_generation",
            2,
            "P2",
            "wg.schema.fixture_generation.v1",
            "derive report and gate fixtures from canonical Rust schema",
            WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_READBACK_GATE,
            "promote visible-only fixture definitions to reviewed generator output",
            vec!["status_summary_shell_materialization"],
        ),
        frontier_candidate(
            "p3.wal_precondition_readback",
            3,
            "P3",
            "wg.event_store.wal_precondition.v1",
            "read back feature-gated WAL preconditions without enabling persistence",
            WORK_GRAPH_APPEND_ONLY_EVENT_STORE_FEATURE_GATED_WAL_PRECONDITION_RECOMMENDED_NEXT_GATE,
            "verify deterministic event id, idempotency, checkpoint, replay, dead-letter, and rollback contracts",
            vec!["wal_checkpoint_write_blocked"],
        ),
        frontier_candidate(
            "p4.agent_jobs_task_board_dry_run_enforcement",
            4,
            "P4",
            "wg.enforcement.agent_jobs_task_board.dry_run.v1",
            "attach TaskResultEnvelope and evidence gates to agent_jobs plus task_board dry-run",
            "hepta_work_graph_agent_jobs_task_board_dry_run_enforcement_gate",
            "deny missing TaskResult, dependency, lease, budget, approval, idempotency, and trace evidence in dry-run",
            vec!["agent_jobs_task_board_enforcement_blocked"],
        ),
        frontier_candidate(
            "p5.agent_card_role_manifest_admission",
            5,
            "P5",
            "wg.admission.agent_card_role_manifest.v1",
            "gate spawn and handoff with AgentCard and role-manifest admission",
            "hepta_work_graph_agent_card_role_manifest_admission_gate",
            "check capability, allowed tools, lane, depth, thread, budget, side-effect class, result contract, verifier, and reducer",
            vec!["role_manifest_admission_blocked"],
        ),
        frontier_candidate(
            "p6.live_canary_cutover",
            6,
            "P6",
            "wg.release.live_canary_cutover.v1",
            "open live/canary/cutover only after observability and rollback proof close",
            "hepta_work_graph_live_canary_cutover_precondition_gate",
            "require operator approval, feature flag, canary, rollback proof, and observability dashboard",
            vec!["feature_canary_cutover_blocked", "live_execution_blocked"],
        ),
    ]
}

impl WorkGraphControlPlaneSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            event_store_enabled: false,
            projection_index_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            replay_executed: false,
            replay_diff_persisted: false,
            idempotency_index_mutated: false,
            scheduler_admission_enforced: false,
            scheduler_live_blocking_enabled: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            role_manifest_persisted: false,
            operator_approval_requested: false,
            operator_approval_recorded: false,
            feature_flag_enabled: false,
            canary_started: false,
            cutover_performed: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn work_graph_control_plane_source_reports(
    schema_report: &WorkGraphCanonicalSchemaTypesReport,
    fixture_report: &WorkGraphCanonicalSchemaFixtureReportGenerationReport,
    wal_report: &WorkGraphAppendOnlyEventStoreFeatureGatedWalPreconditionReport,
) -> Vec<WorkGraphControlPlaneSourceReport> {
    vec![
        WorkGraphControlPlaneSourceReport {
            source_id: "canonical_status_summary_readback_materialized",
            gate: WORK_GRAPH_CANONICAL_STATUS_SUMMARY_READBACK_GATE,
            schema_version: "work_graph_canonical_workgraph_status_summary_readback_v1",
            status: "ready",
            source_report_hash: digest_text(
                "canonical_status_summary_readback:source_reports=8:status_nodes=7:frontier=6:blockers=11:required_prior=327",
            ),
            ready: true,
            no_write_confirmed: true,
            recommended_next_gate: schema_report.gate,
        },
        WorkGraphControlPlaneSourceReport {
            source_id: "canonical_schema_types_report",
            gate: schema_report.gate,
            schema_version: schema_report.schema_version,
            status: schema_report.status,
            source_report_hash: digest_report(schema_report),
            ready: schema_report.ready_for_canonical_schema_fixture_generation,
            no_write_confirmed: !schema_report.side_effects.filesystem_written
                && !schema_report.side_effects.schema_persisted
                && !schema_report.side_effects.event_store_enabled
                && !schema_report.side_effects.runtime_mutation_performed,
            recommended_next_gate: schema_report.recommended_next_gate,
        },
        WorkGraphControlPlaneSourceReport {
            source_id: "canonical_schema_fixture_generation_report",
            gate: fixture_report.gate,
            schema_version: fixture_report.schema_version,
            status: fixture_report.status,
            source_report_hash: digest_report(fixture_report),
            ready: fixture_report.ready_for_canonical_schema_fixture_report_generation_readback,
            no_write_confirmed: !fixture_report.side_effects.filesystem_written
                && !fixture_report.side_effects.fixture_generated
                && !fixture_report.side_effects.report_fixture_written
                && !fixture_report.side_effects.runtime_mutation_performed,
            recommended_next_gate: fixture_report.recommended_next_gate,
        },
        WorkGraphControlPlaneSourceReport {
            source_id: "append_only_event_store_feature_gated_wal_precondition_report",
            gate: wal_report.gate,
            schema_version: wal_report.schema_version,
            status: wal_report.status,
            source_report_hash: digest_report(wal_report),
            ready: wal_report
                .ready_for_append_only_event_store_feature_gated_wal_precondition_readback,
            no_write_confirmed: !wal_report.side_effects.filesystem_written
                && !wal_report.side_effects.work_graph_event_persisted
                && !wal_report.side_effects.wal_written
                && !wal_report.side_effects.checkpoint_written
                && !wal_report.side_effects.runtime_mutation_performed,
            recommended_next_gate: wal_report.recommended_next_gate,
        },
    ]
}

fn work_graph_control_plane_status_nodes_from(
    source_reports: &[WorkGraphControlPlaneSourceReport],
    schema_report: &WorkGraphCanonicalSchemaTypesReport,
    fixture_report: &WorkGraphCanonicalSchemaFixtureReportGenerationReport,
    wal_report: &WorkGraphAppendOnlyEventStoreFeatureGatedWalPreconditionReport,
) -> Vec<WorkNode> {
    let source_hash = |source_id: &str| {
        source_reports
            .iter()
            .find(|source| source.source_id == source_id)
            .map(|source| source.source_report_hash.clone())
            .unwrap_or_else(|| digest_text(source_id))
    };

    vec![
        work_node(
            "wg.control_plane.status_frontier_cli.v1",
            WORK_GRAPH_CONTROL_PLANE_STATUS_FRONTIER_GATE,
            "hepta.workgraph.control_plane.v1",
            digest_text(WORK_GRAPH_CONTROL_PLANE_STATUS_FRONTIER_SCHEMA_VERSION),
            vec![WORK_GRAPH_CANONICAL_STATUS_SUMMARY_READBACK_GATE],
            "P1",
            "keep the CLI read-only and use it as the short-ID query entry",
            "hepta_workgraph_control_plane",
            "control_plane",
            "ready_read_only",
        ),
        work_node(
            "wg.status.canonical_summary_readback.v1",
            WORK_GRAPH_CANONICAL_STATUS_SUMMARY_READBACK_GATE,
            "hepta.workgraph.status_summary.v1",
            source_hash("canonical_status_summary_readback_materialized"),
            vec![],
            "P1",
            "replace shell-only status summary with schema-backed Rust projection",
            "hepta_workgraph_status_summary",
            "status_summary",
            "materialized_report_only",
        ),
        work_node(
            "wg.schema.canonical_types.v1",
            schema_report.gate,
            "hepta.workgraph.schema.v1",
            source_hash("canonical_schema_types_report"),
            schema_report.required_prior_gates.clone(),
            "P2",
            "keep WorkNode, WorkEdge, TaskResult, Lease, Budget, Approval, Artifact, Evidence, and TimelineEvent as canonical schema",
            "hepta_workgraph_canonical_schema",
            "canonical_schema",
            "ready_report_only",
        ),
        work_node(
            "wg.schema.fixture_generation.v1",
            fixture_report.gate,
            "hepta.workgraph.schema_fixture.v1",
            source_hash("canonical_schema_fixture_generation_report"),
            fixture_report.required_prior_gates.clone(),
            "P2",
            "derive report and gate fixtures from schema without writing generated files",
            "hepta_workgraph_schema_fixture_generation",
            "schema_fixture_generation",
            "ready_no_write",
        ),
        work_node(
            "wg.event_store.wal_precondition.v1",
            wal_report.gate,
            "hepta.workgraph.append_only_event_store.v1",
            source_hash("append_only_event_store_feature_gated_wal_precondition_report"),
            wal_report.required_prior_gates.clone(),
            "P3",
            "read back WAL preconditions before any event-store, WAL, checkpoint, replay, or idempotency mutation",
            "hepta_workgraph_event_store",
            "append_only_event_store",
            "ready_no_write",
        ),
        work_node(
            "wg.enforcement.agent_jobs_task_board.dry_run.v1",
            "hepta_work_graph_agent_jobs_task_board_dry_run_enforcement_gate",
            "hepta.workgraph.agent_jobs_task_board.v1",
            digest_text("agent_jobs_task_board_dry_run_enforcement:blocked"),
            vec![wal_report.gate],
            "P4",
            "attach dry-run denial reasons for missing TaskResultEnvelope, lease, dependency, budget, approval, idempotency, and trace evidence",
            "hepta_workgraph_agent_jobs_task_board",
            "dry_run_enforcement",
            "blocked_waiting_for_p3_readback",
        ),
        work_node(
            "wg.admission.agent_card_role_manifest.v1",
            "hepta_work_graph_agent_card_role_manifest_admission_gate",
            "hepta.workgraph.role_manifest_admission.v1",
            digest_text("agent_card_role_manifest_admission:blocked"),
            vec!["hepta_work_graph_agent_jobs_task_board_dry_run_enforcement_gate"],
            "P5",
            "gate spawn and handoff with AgentCard capability, allowed_tools, lane, depth, budget, side-effect class, result contract, verifier, and reducer",
            "hepta_workgraph_role_manifest_admission",
            "role_manifest_admission",
            "blocked_waiting_for_p4_dry_run",
        ),
        work_node(
            "wg.release.live_canary_cutover.v1",
            "hepta_work_graph_live_canary_cutover_precondition_gate",
            "hepta.workgraph.live_canary_cutover.v1",
            digest_text("live_canary_cutover:blocked"),
            vec!["hepta_work_graph_agent_card_role_manifest_admission_gate"],
            "P6",
            "require operator approval, feature flag, canary, rollback proof, and observability dashboard before live execution",
            "hepta_workgraph_live_canary_cutover",
            "live_canary_cutover",
            "blocked_until_all_control_plane_guards_close",
        ),
    ]
}

fn work_graph_control_plane_blockers_from(
    schema_report: &WorkGraphCanonicalSchemaTypesReport,
    fixture_report: &WorkGraphCanonicalSchemaFixtureReportGenerationReport,
    wal_report: &WorkGraphAppendOnlyEventStoreFeatureGatedWalPreconditionReport,
) -> Vec<WorkGraphControlPlaneBlocker> {
    let mut blockers = vec![
        blocker(
            "status_summary_shell_materialization",
            "medium",
            "status_summary",
            "canonical status summary is still materialized through shell/JQ instead of Rust schema projection",
            "move status summary node and frontier generation behind the Rust control-plane projection",
        ),
        blocker(
            "long_gate_chain_depth_no_longer_progress",
            "high",
            "planning",
            "long required-prior chains and final-audit-index branches are now a complexity risk",
            "use workgraph status/frontier/inspect short IDs as the operator entrypoint",
        ),
        blocker(
            "agent_jobs_task_board_enforcement_blocked",
            "critical",
            "agent_jobs_task_board",
            "agent_jobs and task_board enforcement remains dry-run and terminal-denied",
            "connect TaskResultEnvelope and evidence checks to dry-run denial reasons first",
        ),
        blocker(
            "role_manifest_admission_blocked",
            "critical",
            "role_manifest",
            "AgentCard and role-manifest admission is not yet the spawn/handoff gate",
            "route spawn and handoff through one admission evaluator in shadow mode",
        ),
        blocker(
            "feature_canary_cutover_blocked",
            "critical",
            "release",
            "feature flag, canary, cutover, and operator approval remain closed",
            "open only after replay diff, rollback anchor, and observability proof pass",
        ),
        blocker(
            "live_execution_blocked",
            "critical",
            "live",
            "live execution remains denied from the control plane",
            "keep all live paths closed until P3-P6 readiness is verified",
        ),
    ];

    blockers.extend(schema_report.blockers.iter().map(|blocker| {
        blocker_from_parts(
            format!("schema.{}", blocker.id),
            blocker.severity,
            blocker.surface,
            blocker.recommended_fix.to_string(),
        )
    }));
    blockers.extend(fixture_report.blockers.iter().map(|blocker| {
        blocker_from_parts(
            format!("fixture.{}", blocker.id),
            blocker.severity,
            blocker.surface,
            blocker.recommended_fix.to_string(),
        )
    }));
    blockers.extend(wal_report.blockers.iter().map(|blocker| {
        blocker_from_parts(
            format!("wal.{}", blocker.id),
            blocker.severity,
            blocker.surface,
            blocker.recommended_fix.to_string(),
        )
    }));

    blockers
}

fn work_graph_control_plane_required_prior_gates(
    source_reports: &[WorkGraphControlPlaneSourceReport],
    schema_report: &WorkGraphCanonicalSchemaTypesReport,
    fixture_report: &WorkGraphCanonicalSchemaFixtureReportGenerationReport,
    wal_report: &WorkGraphAppendOnlyEventStoreFeatureGatedWalPreconditionReport,
) -> Vec<String> {
    let mut gates: Vec<String> = source_reports
        .iter()
        .map(|source| source.gate.to_string())
        .collect();
    gates.extend(
        schema_report
            .required_prior_gates
            .iter()
            .map(|gate| gate.to_string()),
    );
    gates.extend(
        fixture_report
            .required_prior_gates
            .iter()
            .map(|gate| gate.to_string()),
    );
    gates.extend(
        wal_report
            .required_prior_gates
            .iter()
            .map(|gate| gate.to_string()),
    );
    gates.sort();
    gates.dedup();
    gates
}

fn work_node(
    work_node_id: &str,
    gate_id: &str,
    lineage_id: &str,
    source_report_hash: String,
    required_prior_gate_ids: Vec<&str>,
    frontier: &str,
    next_action: &str,
    source_surface_id: &str,
    node_kind: &str,
    status: &str,
) -> WorkNode {
    WorkNode {
        work_node_id: work_node_id.to_string(),
        gate_id: gate_id.to_string(),
        lineage_id: lineage_id.to_string(),
        source_report_hash,
        required_prior_gate_ids: required_prior_gate_ids
            .into_iter()
            .map(str::to_string)
            .collect(),
        frontier: frontier.to_string(),
        next_action: next_action.to_string(),
        source_surface_id: source_surface_id.to_string(),
        node_kind: node_kind.to_string(),
        status: status.to_string(),
        owner_agent_id: None,
        lease_id: None,
        budget_id: None,
        approval_id: None,
        trace_id: "trace.report_only.workgraph_control_plane.v1".to_string(),
    }
}

fn frontier_candidate(
    id: &'static str,
    priority: usize,
    optimization_stage: &'static str,
    work_node_id: &'static str,
    summary: &'static str,
    recommended_gate: &'static str,
    next_action: &'static str,
    blocked_by: Vec<&'static str>,
) -> WorkGraphFrontierCandidate {
    WorkGraphFrontierCandidate {
        id,
        priority,
        optimization_stage,
        work_node_id,
        summary,
        recommended_gate,
        next_action,
        blocked_by,
    }
}

fn blocker(
    id: &str,
    severity: &'static str,
    surface: &'static str,
    summary: &str,
    recommended_fix: &str,
) -> WorkGraphControlPlaneBlocker {
    WorkGraphControlPlaneBlocker {
        id: id.to_string(),
        severity,
        surface,
        summary: summary.to_string(),
        blocks_live_execution: true,
        recommended_fix: recommended_fix.to_string(),
    }
}

fn blocker_from_parts(
    id: String,
    severity: &'static str,
    surface: &'static str,
    recommended_fix: String,
) -> WorkGraphControlPlaneBlocker {
    WorkGraphControlPlaneBlocker {
        summary: recommended_fix.clone(),
        id,
        severity,
        surface,
        blocks_live_execution: true,
        recommended_fix,
    }
}

fn digest_report<T: Serialize>(value: &T) -> String {
    let payload = serde_json::to_vec(value)
        .unwrap_or_else(|_| b"work_graph_control_plane_digest_error".to_vec());
    digest_bytes(&payload)
}

fn digest_text(value: &str) -> String {
    digest_bytes(value.as_bytes())
}

fn digest_bytes(value: &[u8]) -> String {
    let digest = Sha256::digest(value);
    format!("sha256:{digest:x}")
}

#[cfg(test)]
#[path = "work_graph_control_plane_tests.rs"]
mod tests;
