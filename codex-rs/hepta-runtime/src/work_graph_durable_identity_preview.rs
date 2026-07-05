use serde::Serialize;

pub const WORK_GRAPH_DURABLE_IDENTITY_PREVIEW_GATE: &str =
    "hepta_work_graph_durable_identity_preview_gate";
pub const WORK_GRAPH_DURABLE_IDENTITY_SCHEMA_VERSION: &str =
    "work_graph_durable_identity_preview_v1";
pub const WORK_GRAPH_DURABLE_IDENTITY_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_promotion_precondition_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableIdentityPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub durable_field_count: usize,
    pub preview_binding_count: usize,
    pub invariant_count: usize,
    pub required_prior_gates: Vec<&'static str>,
    pub durable_fields: Vec<WorkGraphDurableFieldPreview>,
    pub preview_bindings: Vec<WorkGraphDurablePreviewBinding>,
    pub invariants: Vec<WorkGraphDurableIdentityInvariantPreview>,
    pub recommended_next_gate: &'static str,
    pub ready_for_promotion_precondition_preview: bool,
    pub ready_for_durable_runtime: bool,
    pub ready_for_replay_execution: bool,
    pub ready_for_rollback_execution: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphDurableIdentityPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableFieldPreview {
    pub id: &'static str,
    pub phase: &'static str,
    pub source_fields: Vec<&'static str>,
    pub required_prior_gate: &'static str,
    pub persistence_policy: &'static str,
    pub mutates_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurablePreviewBinding {
    pub id: &'static str,
    pub source_gate: &'static str,
    pub source_contract_ids: Vec<&'static str>,
    pub binds_fields: Vec<&'static str>,
    pub required: bool,
    pub mutates_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableIdentityInvariantPreview {
    pub id: &'static str,
    pub required: bool,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphDurableIdentityPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub replay_executed: bool,
    pub rollback_performed: bool,
    pub receipt_persisted: bool,
    pub idempotency_index_mutated: bool,
    pub promotion_performed: bool,
    pub runtime_mutation_performed: bool,
    pub scheduler_cutover_performed: bool,
    pub approval_recorded: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_durable_identity_preview_report() -> WorkGraphDurableIdentityPreviewReport {
    let durable_fields = work_graph_durable_identity_fields();
    let preview_bindings = work_graph_durable_identity_preview_bindings();
    let invariants = work_graph_durable_identity_invariants();

    WorkGraphDurableIdentityPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_DURABLE_IDENTITY_PREVIEW_GATE,
        schema_version: WORK_GRAPH_DURABLE_IDENTITY_SCHEMA_VERSION,
        preview_mode: "read_only_durable_identity_contract_preview_no_state_writes",
        durable_field_count: durable_fields.len(),
        preview_binding_count: preview_bindings.len(),
        invariant_count: invariants.len(),
        required_prior_gates: work_graph_durable_identity_required_prior_gates(),
        durable_fields,
        preview_bindings,
        invariants,
        recommended_next_gate: WORK_GRAPH_DURABLE_IDENTITY_RECOMMENDED_NEXT_GATE,
        ready_for_promotion_precondition_preview: true,
        ready_for_durable_runtime: false,
        ready_for_replay_execution: false,
        ready_for_rollback_execution: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphDurableIdentityPreviewSideEffects::none(),
    }
}

pub fn work_graph_durable_identity_required_prior_gates() -> Vec<&'static str> {
    vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
        "hepta_work_graph_unified_state_store_preview_gate",
        "hepta_work_graph_adapter_projection_fixture_gate",
        "hepta_work_graph_state_store_persistence_preview_gate",
        "hepta_work_graph_replay_readback_preview_gate",
    ]
}

pub fn work_graph_durable_identity_fields() -> Vec<WorkGraphDurableFieldPreview> {
    vec![
        durable_field(
            "workflow_id",
            "identity",
            vec!["traceId", "sourceThreadId", "sourceSurfaceId"],
            "hepta_work_graph_contract_preview_gate",
            "required_on_every_wal_record_before_store_persistence",
        ),
        durable_field(
            "run_id",
            "identity",
            vec!["schedulerRunId", "workerTaskId", "jobId", "attempt"],
            "hepta_work_graph_scheduler_admission_controller_preview_gate",
            "required_before_scheduler_or_worker_projection",
        ),
        durable_field(
            "step_id",
            "identity",
            vec!["stepIndex", "taskId", "nodeId", "edgeId"],
            "hepta_work_graph_task_result_contract_preview_gate",
            "required_before_task_result_or_edge_projection",
        ),
        durable_field(
            "checkpoint",
            "checkpoint",
            vec!["walHeadHash", "checkpointHash", "collectionMerkleRoot"],
            "hepta_work_graph_state_store_persistence_preview_gate",
            "derived_from_wal_and_disabled_until_replay_readback_passes",
        ),
        durable_field(
            "replay_key",
            "replay",
            vec![
                "validatedWalHeadHash",
                "sourceRecordKeys",
                "dedupedRecordSet",
            ],
            "hepta_work_graph_replay_readback_preview_gate",
            "deterministic_key_required_before_any_replay_execution",
        ),
        durable_field(
            "rollback_anchor",
            "rollback",
            vec!["checkpointHash", "detectorIds", "operatorApprovalRef"],
            "hepta_work_graph_replay_readback_preview_gate",
            "required_before_recovery_canary_or_runtime_rollback",
        ),
        durable_field(
            "receipt_hash",
            "receipt",
            vec![
                "taskResultHash",
                "approvalHash",
                "timelineHash",
                "redactedReadbackEvidenceRefs",
            ],
            "hepta_work_graph_replay_readback_preview_gate",
            "required_before_promotion_or_operator_audit_visibility",
        ),
    ]
}

pub fn work_graph_durable_identity_preview_bindings() -> Vec<WorkGraphDurablePreviewBinding> {
    vec![
        preview_binding(
            "state_store_wal_to_durable_identity",
            "hepta_work_graph_state_store_persistence_preview_gate",
            vec![
                "preview_append_node_record",
                "preview_append_task_result_record",
                "preview_append_timeline_event_record",
            ],
            vec!["workflow_id", "run_id", "step_id", "receipt_hash"],
        ),
        preview_binding(
            "checkpoint_contract_to_checkpoint",
            "hepta_work_graph_state_store_persistence_preview_gate",
            vec!["preview_full_graph_checkpoint", "preview_trace_checkpoint"],
            vec!["checkpoint", "receipt_hash"],
        ),
        preview_binding(
            "replay_hash_chain_to_replay_key",
            "hepta_work_graph_replay_readback_preview_gate",
            vec![
                "preview_validate_wal_hash_chain",
                "preview_apply_idempotency_window",
            ],
            vec!["checkpoint", "replay_key"],
        ),
        preview_binding(
            "recovery_preview_to_rollback_anchor",
            "hepta_work_graph_replay_readback_preview_gate",
            vec![
                "preview_quarantine_checkpoint",
                "preview_rebuild_projection_indexes",
                "preview_require_operator_replay_approval",
            ],
            vec!["rollback_anchor", "receipt_hash"],
        ),
        preview_binding(
            "readback_evidence_to_receipt_hash",
            "hepta_work_graph_replay_readback_preview_gate",
            vec![
                "assert_task_results_readback_matches_wal",
                "assert_approvals_readback_matches_wal",
                "assert_timeline_readback_matches_wal",
            ],
            vec!["workflow_id", "step_id", "receipt_hash"],
        ),
    ]
}

pub fn work_graph_durable_identity_invariants() -> Vec<WorkGraphDurableIdentityInvariantPreview> {
    vec![
        invariant(
            "durable_identity_required_before_persistence",
            "workflow, run, and step identity must be stable before WAL or checkpoint writes",
        ),
        invariant(
            "checkpoint_derived_from_wal",
            "checkpoint is an evidence pointer derived from WAL hashes, not an authority source",
        ),
        invariant(
            "replay_key_is_deterministic",
            "the same WAL head and source record keys must produce the same replay key",
        ),
        invariant(
            "rollback_anchor_precedes_recovery",
            "future recovery or rollback cannot run without a named checkpoint anchor",
        ),
        invariant(
            "receipt_hash_precedes_promotion",
            "promotion and operator audit require redacted receipt hash evidence first",
        ),
        invariant(
            "readback_evidence_is_redacted",
            "receipt hashes and evidence refs must not expose raw prompts or credentials",
        ),
        invariant(
            "durable_identity_preview_has_no_side_effects",
            "this preview cannot persist state, replay WAL, roll back, promote, or send externally",
        ),
    ]
}

impl WorkGraphDurableIdentityPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            replay_executed: false,
            rollback_performed: false,
            receipt_persisted: false,
            idempotency_index_mutated: false,
            promotion_performed: false,
            runtime_mutation_performed: false,
            scheduler_cutover_performed: false,
            approval_recorded: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn durable_field(
    id: &'static str,
    phase: &'static str,
    source_fields: Vec<&'static str>,
    required_prior_gate: &'static str,
    persistence_policy: &'static str,
) -> WorkGraphDurableFieldPreview {
    WorkGraphDurableFieldPreview {
        id,
        phase,
        source_fields,
        required_prior_gate,
        persistence_policy,
        mutates_state: false,
    }
}

fn preview_binding(
    id: &'static str,
    source_gate: &'static str,
    source_contract_ids: Vec<&'static str>,
    binds_fields: Vec<&'static str>,
) -> WorkGraphDurablePreviewBinding {
    WorkGraphDurablePreviewBinding {
        id,
        source_gate,
        source_contract_ids,
        binds_fields,
        required: true,
        mutates_state: false,
    }
}

fn invariant(id: &'static str, reason: &'static str) -> WorkGraphDurableIdentityInvariantPreview {
    WorkGraphDurableIdentityInvariantPreview {
        id,
        required: true,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durable_identity_preview_declares_the_minimal_field_set() {
        let report = hepta_work_graph_durable_identity_preview_report();
        let field_ids = report
            .durable_fields
            .iter()
            .map(|field| field.id)
            .collect::<Vec<_>>();

        assert_eq!(
            field_ids,
            [
                "workflow_id",
                "run_id",
                "step_id",
                "checkpoint",
                "replay_key",
                "rollback_anchor",
                "receipt_hash",
            ]
        );
        assert_eq!(report.durable_field_count, 7);
        assert!(
            report
                .durable_fields
                .iter()
                .all(|field| !field.mutates_state)
        );
    }

    #[test]
    fn durable_identity_preview_binds_state_store_and_replay_reports() {
        let report = hepta_work_graph_durable_identity_preview_report();
        let binding_ids = report
            .preview_bindings
            .iter()
            .map(|binding| binding.id)
            .collect::<Vec<_>>();

        assert_eq!(
            binding_ids,
            [
                "state_store_wal_to_durable_identity",
                "checkpoint_contract_to_checkpoint",
                "replay_hash_chain_to_replay_key",
                "recovery_preview_to_rollback_anchor",
                "readback_evidence_to_receipt_hash",
            ]
        );
        assert_eq!(report.preview_binding_count, 5);
        assert!(report.preview_bindings.iter().all(|binding| {
            binding.required
                && !binding.mutates_state
                && (binding.source_gate == "hepta_work_graph_state_store_persistence_preview_gate"
                    || binding.source_gate == "hepta_work_graph_replay_readback_preview_gate")
        }));
    }

    #[test]
    fn durable_identity_preview_keeps_execution_disabled() {
        let report = hepta_work_graph_durable_identity_preview_report();

        assert!(report.ready_for_promotion_precondition_preview);
        assert!(!report.ready_for_durable_runtime);
        assert!(!report.ready_for_replay_execution);
        assert!(!report.ready_for_rollback_execution);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphDurableIdentityPreviewSideEffects::none()
        );
    }

    #[test]
    fn durable_identity_preview_requires_existing_work_graph_gates() {
        let report = hepta_work_graph_durable_identity_preview_report();

        assert_eq!(
            report.required_prior_gates,
            [
                "hepta_work_graph_contract_preview_gate",
                "hepta_work_graph_task_result_contract_preview_gate",
                "hepta_work_graph_scheduler_admission_controller_preview_gate",
                "hepta_work_graph_observability_timeline_preview_gate",
                "hepta_work_graph_role_manifest_contract_preview_gate",
                "hepta_work_graph_unified_state_store_preview_gate",
                "hepta_work_graph_adapter_projection_fixture_gate",
                "hepta_work_graph_state_store_persistence_preview_gate",
                "hepta_work_graph_replay_readback_preview_gate",
            ]
        );
        assert_eq!(report.invariant_count, 7);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_DURABLE_IDENTITY_RECOMMENDED_NEXT_GATE
        );
    }
}
