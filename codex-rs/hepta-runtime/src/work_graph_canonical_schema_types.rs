use serde::Deserialize;
use serde::Serialize;

pub const WORK_GRAPH_CANONICAL_SCHEMA_TYPES_REPORT_ONLY_GATE: &str =
    "hepta_work_graph_canonical_schema_types_report_only_gate";
pub const WORK_GRAPH_CANONICAL_SCHEMA_TYPES_SCHEMA_VERSION: &str =
    "work_graph_canonical_schema_types_report_only_v1";
pub const WORK_GRAPH_CANONICAL_SCHEMA_TYPES_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_canonical_schema_fixture_report_generation_gate";
pub const WORK_GRAPH_CANONICAL_STATUS_SUMMARY_READBACK_GATE: &str =
    "hepta_work_graph_canonical_workgraph_status_summary_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkNode {
    pub work_node_id: String,
    pub gate_id: String,
    pub lineage_id: String,
    pub source_report_hash: String,
    pub required_prior_gate_ids: Vec<String>,
    pub frontier: String,
    pub next_action: String,
    pub source_surface_id: String,
    pub node_kind: String,
    pub status: String,
    pub owner_agent_id: Option<String>,
    pub lease_id: Option<String>,
    pub budget_id: Option<String>,
    pub approval_id: Option<String>,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkEdge {
    pub work_edge_id: String,
    pub source_work_node_id: String,
    pub target_work_node_id: String,
    pub edge_kind: String,
    pub required: bool,
    pub source_report_hash: String,
    pub evidence_id: Option<String>,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_result_id: String,
    pub work_node_id: String,
    pub status: String,
    pub summary: String,
    pub artifact_ids: Vec<String>,
    pub evidence_ids: Vec<String>,
    pub risk_ids: Vec<String>,
    pub verifier_id: String,
    pub reducer_id: Option<String>,
    pub usage_id: Option<String>,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lease {
    pub lease_id: String,
    pub work_node_id: String,
    pub owner_agent_id: String,
    pub lane: String,
    pub acquired_at_unix_seconds: i64,
    pub expires_at_unix_seconds: i64,
    pub stale: bool,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Budget {
    pub budget_id: String,
    pub work_node_id: String,
    pub token_budget: u64,
    pub tool_call_budget: u64,
    pub wall_clock_budget_ms: u64,
    pub consumed_token_count: u64,
    pub exceeded: bool,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Approval {
    pub approval_id: String,
    pub work_node_id: String,
    pub approval_state: String,
    pub reviewer: String,
    pub risk_tier: String,
    pub recorded: bool,
    pub source_report_hash: String,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Artifact {
    pub artifact_id: String,
    pub work_node_id: String,
    pub artifact_kind: String,
    pub redaction_state: String,
    pub content_hash: String,
    pub path_hint: Option<String>,
    pub persisted: bool,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_id: String,
    pub work_node_id: String,
    pub evidence_kind: String,
    pub source_gate: String,
    pub report_hash: String,
    pub accepted: bool,
    pub persisted: bool,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub timeline_event_id: String,
    pub work_node_id: String,
    pub event_kind: String,
    pub occurred_at_unix_seconds: i64,
    pub actor: String,
    pub payload_hash: String,
    pub persisted: bool,
    pub trace_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalSchemaTypesReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_status_summary_readback_gate: &'static str,
    pub schema_type_count: usize,
    pub schema_collection_count: usize,
    pub schema_field_count: usize,
    pub identity_field_count: usize,
    pub join_key_field_count: usize,
    pub schema_types: Vec<WorkGraphCanonicalSchemaTypeDefinition>,
    pub blockers: Vec<WorkGraphCanonicalSchemaTypeBlocker>,
    pub blocker_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub required_prior_gate_count: usize,
    pub recommended_next_gate: &'static str,
    pub ready_for_canonical_schema_fixture_generation: bool,
    pub ready_for_append_only_work_graph_event_store: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphCanonicalSchemaTypesSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalSchemaTypeDefinition {
    pub rust_type_name: &'static str,
    pub collection_name: &'static str,
    pub identity_field: &'static str,
    pub field_names: Vec<&'static str>,
    pub join_key_fields: Vec<&'static str>,
    pub persistence_enabled: bool,
    pub enforcement_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalSchemaTypeBlocker {
    pub id: &'static str,
    pub severity: &'static str,
    pub surface: &'static str,
    pub blocks_live_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalSchemaTypesSideEffects {
    pub filesystem_written: bool,
    pub schema_registered: bool,
    pub schema_persisted: bool,
    pub fixture_generated: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub event_store_enabled: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub operator_approval_requested: bool,
    pub feature_flag_enabled: bool,
    pub canary_started: bool,
    pub cutover_performed: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_canonical_schema_types_report_only_report()
-> WorkGraphCanonicalSchemaTypesReport {
    let schema_types = work_graph_canonical_schema_type_definitions();
    let blockers = work_graph_canonical_schema_type_blockers();
    let schema_field_count = schema_types
        .iter()
        .map(|schema_type| schema_type.field_names.len())
        .sum();
    let join_key_field_count = schema_types
        .iter()
        .map(|schema_type| schema_type.join_key_fields.len())
        .sum();

    WorkGraphCanonicalSchemaTypesReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_CANONICAL_SCHEMA_TYPES_REPORT_ONLY_GATE,
        schema_version: WORK_GRAPH_CANONICAL_SCHEMA_TYPES_SCHEMA_VERSION,
        preview_mode: "rust_canonical_schema_types_report_only_no_persistence",
        source_status_summary_readback_gate: WORK_GRAPH_CANONICAL_STATUS_SUMMARY_READBACK_GATE,
        schema_type_count: schema_types.len(),
        schema_collection_count: schema_types.len(),
        schema_field_count,
        identity_field_count: schema_types.len(),
        join_key_field_count,
        schema_types,
        blocker_count: blockers.len(),
        blockers,
        required_prior_gates: vec![WORK_GRAPH_CANONICAL_STATUS_SUMMARY_READBACK_GATE],
        required_prior_gate_count: 1,
        recommended_next_gate: WORK_GRAPH_CANONICAL_SCHEMA_TYPES_RECOMMENDED_NEXT_GATE,
        ready_for_canonical_schema_fixture_generation: true,
        ready_for_append_only_work_graph_event_store: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_task_result_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphCanonicalSchemaTypesSideEffects::none(),
    }
}

pub fn work_graph_canonical_schema_type_definitions() -> Vec<WorkGraphCanonicalSchemaTypeDefinition>
{
    vec![
        schema_type(
            "WorkNode",
            "work_nodes",
            "work_node_id",
            vec![
                "work_node_id",
                "gate_id",
                "lineage_id",
                "source_report_hash",
                "required_prior_gate_ids",
                "frontier",
                "next_action",
                "source_surface_id",
                "node_kind",
                "status",
                "owner_agent_id",
                "lease_id",
                "budget_id",
                "approval_id",
                "trace_id",
            ],
            vec![
                "work_node_id",
                "gate_id",
                "lineage_id",
                "source_report_hash",
                "trace_id",
            ],
        ),
        schema_type(
            "WorkEdge",
            "work_edges",
            "work_edge_id",
            vec![
                "work_edge_id",
                "source_work_node_id",
                "target_work_node_id",
                "edge_kind",
                "required",
                "source_report_hash",
                "evidence_id",
                "trace_id",
            ],
            vec![
                "work_edge_id",
                "source_work_node_id",
                "target_work_node_id",
                "trace_id",
            ],
        ),
        schema_type(
            "TaskResult",
            "task_results",
            "task_result_id",
            vec![
                "task_result_id",
                "work_node_id",
                "status",
                "summary",
                "artifact_ids",
                "evidence_ids",
                "risk_ids",
                "verifier_id",
                "reducer_id",
                "usage_id",
                "trace_id",
            ],
            vec!["task_result_id", "work_node_id", "trace_id"],
        ),
        schema_type(
            "Lease",
            "leases",
            "lease_id",
            vec![
                "lease_id",
                "work_node_id",
                "owner_agent_id",
                "lane",
                "acquired_at_unix_seconds",
                "expires_at_unix_seconds",
                "stale",
                "trace_id",
            ],
            vec!["lease_id", "work_node_id", "owner_agent_id", "trace_id"],
        ),
        schema_type(
            "Budget",
            "budgets",
            "budget_id",
            vec![
                "budget_id",
                "work_node_id",
                "token_budget",
                "tool_call_budget",
                "wall_clock_budget_ms",
                "consumed_token_count",
                "exceeded",
                "trace_id",
            ],
            vec!["budget_id", "work_node_id", "trace_id"],
        ),
        schema_type(
            "Approval",
            "approvals",
            "approval_id",
            vec![
                "approval_id",
                "work_node_id",
                "approval_state",
                "reviewer",
                "risk_tier",
                "recorded",
                "source_report_hash",
                "trace_id",
            ],
            vec![
                "approval_id",
                "work_node_id",
                "source_report_hash",
                "trace_id",
            ],
        ),
        schema_type(
            "Artifact",
            "artifacts",
            "artifact_id",
            vec![
                "artifact_id",
                "work_node_id",
                "artifact_kind",
                "redaction_state",
                "content_hash",
                "path_hint",
                "persisted",
                "trace_id",
            ],
            vec!["artifact_id", "work_node_id", "content_hash", "trace_id"],
        ),
        schema_type(
            "Evidence",
            "evidence",
            "evidence_id",
            vec![
                "evidence_id",
                "work_node_id",
                "evidence_kind",
                "source_gate",
                "report_hash",
                "accepted",
                "persisted",
                "trace_id",
            ],
            vec![
                "evidence_id",
                "work_node_id",
                "source_gate",
                "report_hash",
                "trace_id",
            ],
        ),
        schema_type(
            "TimelineEvent",
            "timeline_events",
            "timeline_event_id",
            vec![
                "timeline_event_id",
                "work_node_id",
                "event_kind",
                "occurred_at_unix_seconds",
                "actor",
                "payload_hash",
                "persisted",
                "trace_id",
            ],
            vec![
                "timeline_event_id",
                "work_node_id",
                "payload_hash",
                "trace_id",
            ],
        ),
    ]
}

pub fn work_graph_canonical_schema_type_blockers() -> Vec<WorkGraphCanonicalSchemaTypeBlocker> {
    vec![
        blocker(
            "canonical_schema_types_report_only",
            "high",
            "schema_registry",
            "turn the Rust schema catalog into generated fixtures before registration",
        ),
        blocker(
            "canonical_schema_fixture_generation_missing",
            "high",
            "fixture_generation",
            "generate report/gate fixtures from the schema catalog before reducing script drift",
        ),
        blocker(
            "append_only_wal_disabled",
            "critical",
            "event_store",
            "keep WAL and checkpoint writes disabled until schema fixtures and replay diffs close",
        ),
        blocker(
            "scheduler_task_result_role_enforcement_disabled",
            "critical",
            "runtime_enforcement",
            "keep scheduler, TaskResult, and role-manifest enforcement denied until schema-backed dry-run gates close",
        ),
        blocker(
            "lease_budget_approval_not_authoritative",
            "high",
            "admission_inputs",
            "keep lease, budget, and approval rows as planned schema only until admission consumes evidence",
        ),
        blocker(
            "artifact_evidence_timeline_not_persisted",
            "high",
            "evidence",
            "keep artifact, evidence, and timeline rows redacted and unpersisted until event-store cutover",
        ),
        blocker(
            "feature_canary_cutover_blocked",
            "critical",
            "release",
            "require operator approval, feature flag, canary, and rollback proof before cutover",
        ),
        blocker(
            "live_execution_blocked",
            "critical",
            "live",
            "open live execution only after all P2-P6 blockers close",
        ),
    ]
}

impl WorkGraphCanonicalSchemaTypesSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            schema_registered: false,
            schema_persisted: false,
            fixture_generated: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            event_store_enabled: false,
            wal_written: false,
            checkpoint_written: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            operator_approval_requested: false,
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

fn schema_type(
    rust_type_name: &'static str,
    collection_name: &'static str,
    identity_field: &'static str,
    field_names: Vec<&'static str>,
    join_key_fields: Vec<&'static str>,
) -> WorkGraphCanonicalSchemaTypeDefinition {
    WorkGraphCanonicalSchemaTypeDefinition {
        rust_type_name,
        collection_name,
        identity_field,
        field_names,
        join_key_fields,
        persistence_enabled: false,
        enforcement_enabled: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    surface: &'static str,
    recommended_fix: &'static str,
) -> WorkGraphCanonicalSchemaTypeBlocker {
    WorkGraphCanonicalSchemaTypeBlocker {
        id,
        severity,
        surface,
        blocks_live_execution: true,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_schema_types_cover_required_records() {
        let report = hepta_work_graph_canonical_schema_types_report_only_report();
        let type_names = report
            .schema_types
            .iter()
            .map(|schema_type| schema_type.rust_type_name)
            .collect::<Vec<_>>();

        assert_eq!(
            type_names,
            [
                "WorkNode",
                "WorkEdge",
                "TaskResult",
                "Lease",
                "Budget",
                "Approval",
                "Artifact",
                "Evidence",
                "TimelineEvent",
            ]
        );
        assert_eq!(report.schema_type_count, 9);
        assert_eq!(report.schema_collection_count, 9);
        assert_eq!(report.schema_field_count, 82);
        assert_eq!(report.identity_field_count, 9);
    }

    #[test]
    fn canonical_schema_preserves_short_frontier_identity_fields() {
        let report = hepta_work_graph_canonical_schema_types_report_only_report();
        let work_node = report
            .schema_types
            .iter()
            .find(|schema_type| schema_type.rust_type_name == "WorkNode")
            .expect("WorkNode schema type");

        assert_eq!(work_node.identity_field, "work_node_id");
        assert_eq!(
            work_node.field_names,
            [
                "work_node_id",
                "gate_id",
                "lineage_id",
                "source_report_hash",
                "required_prior_gate_ids",
                "frontier",
                "next_action",
                "source_surface_id",
                "node_kind",
                "status",
                "owner_agent_id",
                "lease_id",
                "budget_id",
                "approval_id",
                "trace_id",
            ]
        );
        assert!(
            ["gate_id", "lineage_id", "source_report_hash", "trace_id"]
                .iter()
                .all(|field| work_node.join_key_fields.contains(field))
        );
    }

    #[test]
    fn canonical_schema_rows_are_serializable_and_owned() {
        let node = WorkNode {
            work_node_id: "wg.node.schema.v1".to_string(),
            gate_id: WORK_GRAPH_CANONICAL_SCHEMA_TYPES_REPORT_ONLY_GATE.to_string(),
            lineage_id: "wg.lineage.optimization.p2".to_string(),
            source_report_hash: "0".repeat(64),
            required_prior_gate_ids: vec![
                WORK_GRAPH_CANONICAL_STATUS_SUMMARY_READBACK_GATE.to_string(),
            ],
            frontier: "P2".to_string(),
            next_action: WORK_GRAPH_CANONICAL_SCHEMA_TYPES_RECOMMENDED_NEXT_GATE.to_string(),
            source_surface_id: "canonical_workgraph_status_summary".to_string(),
            node_kind: "schema_contract".to_string(),
            status: "report_only".to_string(),
            owner_agent_id: None,
            lease_id: None,
            budget_id: None,
            approval_id: None,
            trace_id: "trace-schema-types".to_string(),
        };

        let json = serde_json::to_value(&node).expect("serialize WorkNode");
        assert_eq!(json["work_node_id"], "wg.node.schema.v1");
        assert_eq!(
            json["required_prior_gate_ids"][0],
            WORK_GRAPH_CANONICAL_STATUS_SUMMARY_READBACK_GATE
        );
    }

    #[test]
    fn canonical_schema_report_keeps_runtime_disabled() {
        let report = hepta_work_graph_canonical_schema_types_report_only_report();

        assert_eq!(
            report.side_effects,
            WorkGraphCanonicalSchemaTypesSideEffects::none()
        );
        assert!(report.ready_for_canonical_schema_fixture_generation);
        assert!(!report.ready_for_append_only_work_graph_event_store);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_live_execution);
        assert!(report.schema_types.iter().all(
            |schema_type| !schema_type.persistence_enabled && !schema_type.enforcement_enabled
        ));
        assert!(
            report
                .blockers
                .iter()
                .all(|blocker| blocker.blocks_live_execution)
        );
    }
}
