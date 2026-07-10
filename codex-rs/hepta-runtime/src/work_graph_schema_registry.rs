use serde::Serialize;

use crate::work_graph_canonical_schema_types::WORK_GRAPH_CANONICAL_SCHEMA_TYPES_REPORT_ONLY_GATE;
use crate::work_graph_canonical_schema_types::work_graph_canonical_schema_type_definitions;

pub const WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_GATE: &str =
    "hepta_work_graph_canonical_schema_fixture_report_generation_gate";
pub const WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_SCHEMA_VERSION: &str =
    "work_graph_canonical_schema_fixture_report_generation_v1";
pub const WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_canonical_schema_fixture_report_generation_readback_gate";
pub const WORK_GRAPH_CANONICAL_SCHEMA_TYPES_READBACK_GATE: &str =
    "hepta_work_graph_canonical_schema_types_report_only_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalSchemaFixtureReportGenerationReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_schema_types_gate: &'static str,
    pub source_schema_types_readback_gate: &'static str,
    pub schema_fixture_count: usize,
    pub report_fixture_definition_count: usize,
    pub gate_fixture_definition_count: usize,
    pub identity_assertion_count: usize,
    pub field_assertion_count: usize,
    pub join_key_assertion_count: usize,
    pub generator_phase_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub fixture_definitions: Vec<WorkGraphCanonicalSchemaFixtureDefinition>,
    pub generator_phases: Vec<WorkGraphCanonicalSchemaFixtureGeneratorPhase>,
    pub blockers: Vec<WorkGraphCanonicalSchemaFixtureGenerationBlocker>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_canonical_schema_fixture_report_generation_readback: bool,
    pub ready_for_append_only_work_graph_event_store: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphCanonicalSchemaFixtureGenerationSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalSchemaFixtureDefinition {
    pub fixture_id: String,
    pub rust_type_name: &'static str,
    pub collection_name: &'static str,
    pub identity_field: &'static str,
    pub report_fixture_id: String,
    pub gate_fixture_id: String,
    pub report_template_id: String,
    pub gate_assertion_template_id: String,
    pub field_assertion_count: usize,
    pub join_key_assertion_count: usize,
    pub required_prior_gate: &'static str,
    pub generated_from_schema_catalog: bool,
    pub visible_only: bool,
    pub filesystem_written: bool,
    pub persistence_enabled: bool,
    pub enforcement_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalSchemaFixtureGeneratorPhase {
    pub phase_id: &'static str,
    pub sequence: usize,
    pub summary: &'static str,
    pub runtime_mutation_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalSchemaFixtureGenerationBlocker {
    pub id: &'static str,
    pub severity: &'static str,
    pub surface: &'static str,
    pub blocks_live_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalSchemaFixtureGenerationSideEffects {
    pub filesystem_written: bool,
    pub fixture_definition_recorded: bool,
    pub fixture_generated: bool,
    pub report_fixture_written: bool,
    pub gate_fixture_written: bool,
    pub schema_registered: bool,
    pub schema_persisted: bool,
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

pub fn hepta_work_graph_canonical_schema_fixture_report_generation_report()
-> WorkGraphCanonicalSchemaFixtureReportGenerationReport {
    let fixture_definitions = work_graph_canonical_schema_fixture_definitions();
    let generator_phases = work_graph_canonical_schema_fixture_generator_phases();
    let blockers = work_graph_canonical_schema_fixture_generation_blockers();
    let required_prior_gates = vec![
        WORK_GRAPH_CANONICAL_SCHEMA_TYPES_REPORT_ONLY_GATE,
        WORK_GRAPH_CANONICAL_SCHEMA_TYPES_READBACK_GATE,
    ];

    WorkGraphCanonicalSchemaFixtureReportGenerationReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_GATE,
        schema_version: WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_SCHEMA_VERSION,
        preview_mode: "canonical_schema_fixture_report_generation_no_write",
        source_schema_types_gate: WORK_GRAPH_CANONICAL_SCHEMA_TYPES_REPORT_ONLY_GATE,
        source_schema_types_readback_gate: WORK_GRAPH_CANONICAL_SCHEMA_TYPES_READBACK_GATE,
        schema_fixture_count: fixture_definitions.len(),
        report_fixture_definition_count: fixture_definitions.len(),
        gate_fixture_definition_count: fixture_definitions.len(),
        identity_assertion_count: fixture_definitions.len(),
        field_assertion_count: fixture_definitions
            .iter()
            .map(|fixture| fixture.field_assertion_count)
            .sum(),
        join_key_assertion_count: fixture_definitions
            .iter()
            .map(|fixture| fixture.join_key_assertion_count)
            .sum(),
        generator_phase_count: generator_phases.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        fixture_definitions,
        generator_phases,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_CANONICAL_SCHEMA_FIXTURE_REPORT_GENERATION_RECOMMENDED_NEXT_GATE,
        ready_for_canonical_schema_fixture_report_generation_readback: true,
        ready_for_append_only_work_graph_event_store: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_task_result_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphCanonicalSchemaFixtureGenerationSideEffects::none(),
    }
}

pub fn work_graph_canonical_schema_fixture_definitions()
-> Vec<WorkGraphCanonicalSchemaFixtureDefinition> {
    work_graph_canonical_schema_type_definitions()
        .into_iter()
        .map(|schema_type| WorkGraphCanonicalSchemaFixtureDefinition {
            fixture_id: format!("schema_fixture.{}.v1", schema_type.collection_name),
            rust_type_name: schema_type.rust_type_name,
            collection_name: schema_type.collection_name,
            identity_field: schema_type.identity_field,
            report_fixture_id: format!("report_fixture.{}.v1", schema_type.collection_name),
            gate_fixture_id: format!("gate_fixture.{}.v1", schema_type.collection_name),
            report_template_id: format!("report_template.{}.v1", schema_type.collection_name),
            gate_assertion_template_id: format!(
                "gate_assertion_template.{}.v1",
                schema_type.collection_name
            ),
            field_assertion_count: schema_type.field_names.len(),
            join_key_assertion_count: schema_type.join_key_fields.len(),
            required_prior_gate: WORK_GRAPH_CANONICAL_SCHEMA_TYPES_READBACK_GATE,
            generated_from_schema_catalog: true,
            visible_only: true,
            filesystem_written: false,
            persistence_enabled: false,
            enforcement_enabled: false,
        })
        .collect()
}

pub fn work_graph_canonical_schema_fixture_generator_phases()
-> Vec<WorkGraphCanonicalSchemaFixtureGeneratorPhase> {
    vec![
        generator_phase(
            "schema_catalog_readback",
            1,
            "consume canonical schema type report and readback digests",
        ),
        generator_phase(
            "fixture_plan_derivation",
            2,
            "derive one report fixture and one gate fixture per canonical collection",
        ),
        generator_phase(
            "report_template_preview",
            3,
            "preview collection report templates without writing files",
        ),
        generator_phase(
            "gate_assertion_preview",
            4,
            "preview identity, field, and join-key gate assertions",
        ),
        generator_phase(
            "readback_handoff",
            5,
            "hand off visible-only fixture catalog to the next P3 precondition",
        ),
    ]
}

pub fn work_graph_canonical_schema_fixture_generation_blockers()
-> Vec<WorkGraphCanonicalSchemaFixtureGenerationBlocker> {
    vec![
        blocker(
            "canonical_schema_fixture_generation_report_only",
            "high",
            "fixture_generation",
            "keep generated fixture definitions visible-only until a writer path is reviewed",
        ),
        blocker(
            "report_fixture_templates_not_written",
            "high",
            "report_templates",
            "materialize report templates only after fixture readback and diff review",
        ),
        blocker(
            "gate_fixture_templates_not_written",
            "high",
            "gate_templates",
            "materialize gate templates only after assertion coverage is read back",
        ),
        blocker(
            "append_only_wal_still_disabled",
            "critical",
            "event_store",
            "keep WAL writes disabled until schema fixture generation readback closes",
        ),
        blocker(
            "scheduler_task_result_enforcement_still_disabled",
            "critical",
            "runtime_enforcement",
            "keep scheduler and TaskResult enforcement denied until fixture-backed dry-run passes",
        ),
        blocker(
            "role_manifest_admission_still_disabled",
            "critical",
            "role_manifest",
            "keep AgentCard and role-manifest admission denied until fixture-backed dry-run passes",
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
            "open live execution only after all P3-P6 blockers close",
        ),
    ]
}

impl WorkGraphCanonicalSchemaFixtureGenerationSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            fixture_definition_recorded: false,
            fixture_generated: false,
            report_fixture_written: false,
            gate_fixture_written: false,
            schema_registered: false,
            schema_persisted: false,
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

fn generator_phase(
    phase_id: &'static str,
    sequence: usize,
    summary: &'static str,
) -> WorkGraphCanonicalSchemaFixtureGeneratorPhase {
    WorkGraphCanonicalSchemaFixtureGeneratorPhase {
        phase_id,
        sequence,
        summary,
        runtime_mutation_allowed: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    surface: &'static str,
    recommended_fix: &'static str,
) -> WorkGraphCanonicalSchemaFixtureGenerationBlocker {
    WorkGraphCanonicalSchemaFixtureGenerationBlocker {
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
    fn fixture_definitions_follow_schema_catalog() {
        let report = hepta_work_graph_canonical_schema_fixture_report_generation_report();
        let fixture_ids = report
            .fixture_definitions
            .iter()
            .map(|fixture| fixture.fixture_id.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            fixture_ids,
            [
                "schema_fixture.work_nodes.v1",
                "schema_fixture.work_edges.v1",
                "schema_fixture.task_results.v1",
                "schema_fixture.leases.v1",
                "schema_fixture.budgets.v1",
                "schema_fixture.approvals.v1",
                "schema_fixture.artifacts.v1",
                "schema_fixture.evidence.v1",
                "schema_fixture.timeline_events.v1",
            ]
        );
        assert_eq!(report.schema_fixture_count, 9);
        assert_eq!(report.report_fixture_definition_count, 9);
        assert_eq!(report.gate_fixture_definition_count, 9);
    }

    #[test]
    fn fixture_generation_counts_schema_assertions() {
        let report = hepta_work_graph_canonical_schema_fixture_report_generation_report();

        assert_eq!(report.identity_assertion_count, 9);
        assert_eq!(report.field_assertion_count, 82);
        assert_eq!(report.join_key_assertion_count, 36);
        assert_eq!(report.generator_phase_count, 5);
        assert!(report.fixture_definitions.iter().all(|fixture| {
            fixture.generated_from_schema_catalog
                && fixture.visible_only
                && !fixture.filesystem_written
                && !fixture.persistence_enabled
                && !fixture.enforcement_enabled
        }));
    }

    #[test]
    fn fixture_generation_preserves_source_gates() {
        let report = hepta_work_graph_canonical_schema_fixture_report_generation_report();

        assert_eq!(
            report.required_prior_gates,
            [
                WORK_GRAPH_CANONICAL_SCHEMA_TYPES_REPORT_ONLY_GATE,
                WORK_GRAPH_CANONICAL_SCHEMA_TYPES_READBACK_GATE,
            ]
        );
        assert_eq!(
            report.source_schema_types_gate,
            WORK_GRAPH_CANONICAL_SCHEMA_TYPES_REPORT_ONLY_GATE
        );
        assert_eq!(
            report.source_schema_types_readback_gate,
            WORK_GRAPH_CANONICAL_SCHEMA_TYPES_READBACK_GATE
        );
    }

    #[test]
    fn fixture_generation_keeps_runtime_disabled() {
        let report = hepta_work_graph_canonical_schema_fixture_report_generation_report();

        assert_eq!(
            report.side_effects,
            WorkGraphCanonicalSchemaFixtureGenerationSideEffects::none()
        );
        assert!(report.ready_for_canonical_schema_fixture_report_generation_readback);
        assert!(!report.ready_for_append_only_work_graph_event_store);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_live_execution);
        assert!(
            report
                .blockers
                .iter()
                .all(|blocker| blocker.blocks_live_execution)
        );
    }
}
