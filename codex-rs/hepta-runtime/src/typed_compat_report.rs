use serde_json::Value;
use thiserror::Error;

pub const TYPED_COMPAT_REPORT_IDS: &[&str] = &[
    "hepta-systems-current-reality-matrix-compact-cache-boundary-readback",
    "hepta-systems-work-graph-adapter-projection-fixture",
    "hepta-systems-work-graph-append-only-event-intake-preview",
    "hepta-systems-work-graph-append-only-store-enablement-precondition-preview",
    "hepta-systems-work-graph-append-only-store-enablement-precondition-readback-preview",
    "hepta-systems-work-graph-append-only-store-runtime-durable-store-switch-application-preview",
    "hepta-systems-work-graph-append-only-store-runtime-durable-store-switch-preview",
    "hepta-systems-work-graph-append-only-store-runtime-enablement-application-preview",
    "hepta-systems-work-graph-append-only-store-runtime-enablement-preview",
    "hepta-systems-work-graph-append-only-store-runtime-idempotency-mutation-application-preview",
    "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-application-preview",
    "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-preview",
    "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-readback-preview",
    "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-application-preview",
    "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-preview",
    "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-readback-preview",
    "hepta-systems-work-graph-append-only-store-runtime-write-boundary-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-adapter-projection-gap-closure-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-no-cutover-guard-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-operator-review-packet-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-operator-review-packet-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-replay-readback-execution-closure-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-runtime-adapter-enforcement-closure-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-side-effect-lock-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-closeout-packet-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-readback-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-event-store-enablement-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-persistence-guard-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-replay-readback-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-shadow-write-application-preview",
    "hepta-systems-work-graph-append-only-work-graph-events-shadow-write-preview",
    "hepta-systems-work-graph-canonical-adapter-inventory-application-preview",
    "hepta-systems-work-graph-canonical-adapter-inventory-preview",
    "hepta-systems-work-graph-canonical-adapter-inventory-readback-preview",
    "hepta-systems-work-graph-current-state-inventory",
    "hepta-systems-work-graph-idempotency-readback-adapter-preview",
    "hepta-systems-work-graph-observability-timeline-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-acknowledgement-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-acknowledgement-replay-idempotency-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview",
    "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-preview",
    "hepta-systems-work-graph-projection-adapter-gap-closure-application-preview",
    "hepta-systems-work-graph-projection-adapter-gap-closure-preview",
    "hepta-systems-work-graph-projection-adapter-gap-closure-readback-preview",
    "hepta-systems-work-graph-role-manifest-enforcement-gap-closure-preview",
    "hepta-systems-work-graph-runtime-application-promotion-gap-closure-preview",
    "hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-preview",
    "hepta-systems-work-graph-state-store-persistence-preview",
    "hepta-systems-work-graph-store-idempotency-guard-gap-closure-preview",
    "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-application-preview",
    "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-preview",
    "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-readback-preview",
    "hepta-systems-work-graph-terminal-task-result-wrapper-preview",
    "hepta-systems-work-graph-unified-projection-audit-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-runtime-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-canonical-adapter-inventory-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-operator-review-side-effect-lock-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-application-promotion-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-durable-store-switch-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-rollback-readback-execution-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-store-guard-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-adapter-projection-gap-closure-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-denial-evidence-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-final-no-enablement-readiness-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-no-cutover-guard-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-readiness-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-side-effect-lock-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-closeout-packet-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-operator-review-packet-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-enablement-rerun-preview",
    "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-persistence-guard-rerun-preview",
    "hepta-systems-workflow-durable-store-adapter",
    "hepta-systems-workflow-durable-store-test-only-append-fixture",
    "hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence",
    "hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation",
    "hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-deterministic-replay-validator-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback",
    "hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-lease-idempotency-index-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback",
    "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-local-persistence-readback",
    "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-feature-gated-readback",
    "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-local-persistence-readback",
    "hepta-systems-workflow-temporal-lite-work-graph-projection-feature-gated-readback",
];

#[derive(Debug, Error)]
pub enum TypedCompatReportError {
    #[error("unknown typed compatibility report: {0}")]
    UnknownReport(String),
    #[error("typed compatibility report contract violation: {0}")]
    ContractViolation(String),
    #[error("typed compatibility report serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}

macro_rules! serialize_report {
    ($report:expr) => {
        serde_json::to_value($report).map_err(TypedCompatReportError::from)
    };
}

fn contract_object_mut<'a>(
    value: &'a mut Value,
    context: &str,
) -> Result<&'a mut serde_json::Map<String, Value>, TypedCompatReportError> {
    value.as_object_mut().ok_or_else(|| {
        TypedCompatReportError::ContractViolation(format!("{context} must be a JSON object"))
    })
}

fn take_contract_field(
    object: &mut serde_json::Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<Value, TypedCompatReportError> {
    object.remove(field).ok_or_else(|| {
        TypedCompatReportError::ContractViolation(format!(
            "{context} must expose required field {field}"
        ))
    })
}

fn workflow_durable_store_adapter_compat_report() -> Result<Value, TypedCompatReportError> {
    let append_plan = crate::hepta_workflow_durable_store_append_plan_report();
    let harness = crate::hepta_workflow_durable_store_adapter_harness_report();
    let adapter = crate::workflow_durable_store_adapter_report(&append_plan, &harness);
    let mut report = serde_json::to_value(adapter)?;
    let object = contract_object_mut(&mut report, "workflow durable store adapter report")?;

    let source_harness_surface =
        take_contract_field(object, "source_harness_surface", "typed adapter report")?;
    let source_harness_ready =
        take_contract_field(object, "source_harness_ready", "typed adapter report")?;
    object.insert(
        "source_adapter_harness_surface".to_string(),
        source_harness_surface,
    );
    object.insert(
        "source_adapter_harness_ready".to_string(),
        source_harness_ready,
    );
    object.insert(
        "source_append_only_event_intake_surface".to_string(),
        Value::String(append_plan.source_gate.to_string()),
    );
    object.insert(
        "source_append_only_event_intake_ready".to_string(),
        Value::Bool(append_plan.source_append_only_event_intake_ready),
    );
    object.insert(
        "source_append_only_event_contract_count".to_string(),
        Value::from(append_plan.event_contract_count),
    );
    object.insert("lib_export_present".to_string(), Value::Bool(true));
    object.insert(
        "local_gate".to_string(),
        Value::String(
            "scripts/lib/hepta-gate-pair-compat-v1/hepta-systems-workflow-durable-store-adapter.gate"
                .to_string(),
        ),
    );
    object.insert(
        "architecture_note".to_string(),
        Value::String(
            "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_DURABLE_STORE_ADAPTER_2026-06-27.md"
                .to_string(),
        ),
    );
    object.insert("side_effect_free".to_string(), Value::Bool(true));
    object.insert(
        "source_files".to_string(),
        serde_json::json!({
            "adapter": "codex-rs/hepta-runtime/src/workflow_durable_store_adapter.rs",
            "append_only_intake_report": "scripts/hepta-systems-work-graph-append-only-event-intake-preview-report.sh",
            "append_plan": "codex-rs/hepta-runtime/src/workflow_durable_store_append_plan.rs",
            "harness": "codex-rs/hepta-runtime/src/workflow_durable_store_adapter_harness.rs"
        }),
    );
    object.remove("recommended_next_gate");
    object.insert(
        "next_migration_step".to_string(),
        Value::String(crate::WORKFLOW_DURABLE_STORE_ADAPTER_RECOMMENDED_NEXT_GATE.to_string()),
    );
    object.insert(
        "next_actions".to_string(),
        serde_json::json!([
            crate::WORKFLOW_DURABLE_STORE_ADAPTER_RECOMMENDED_NEXT_GATE,
            "keep_event_log_sqlite_replay_rollback_and_live_execution_disabled_until_explicit_cutover"
        ]),
    );

    let entries = object
        .get_mut("entries")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| {
            TypedCompatReportError::ContractViolation(
                "typed adapter report must expose entries as a JSON array".to_string(),
            )
        })?;
    if entries.len() != append_plan.append_plans.len() || entries.len() != harness.receipts.len() {
        return Err(TypedCompatReportError::ContractViolation(format!(
            "typed adapter entry count {} does not match append-plan count {} and harness count {}",
            entries.len(),
            append_plan.append_plans.len(),
            harness.receipts.len()
        )));
    }
    for (index, ((entry, plan), receipt)) in entries
        .iter_mut()
        .zip(&append_plan.append_plans)
        .zip(&harness.receipts)
        .enumerate()
    {
        let entry_event_contract_id = entry
            .get("event_contract_id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                TypedCompatReportError::ContractViolation(format!(
                    "typed adapter entry {index} must expose string event_contract_id"
                ))
            })?;
        if entry_event_contract_id != plan.event_contract_id
            || plan.event_contract_id != receipt.event_contract_id
        {
            return Err(TypedCompatReportError::ContractViolation(format!(
                "typed adapter entry {index} event contract mismatch: report={entry_event_contract_id}, append_plan={}, harness={}",
                plan.event_contract_id, receipt.event_contract_id
            )));
        }
        let entry = contract_object_mut(entry, "typed adapter entry")?;
        entry.insert(
            "target_collection_ids".to_string(),
            serde_json::to_value(&plan.target_collection_ids)?,
        );
        entry.insert(
            "required_fields".to_string(),
            serde_json::to_value(&plan.required_fields)?,
        );
        entry.insert(
            "idempotency_key_fields".to_string(),
            serde_json::to_value(&plan.idempotency_key_fields)?,
        );
        entry.insert(
            "append_policy".to_string(),
            Value::String(plan.append_policy.to_string()),
        );
        entry.insert(
            "append_suppressed_by_feature_gate".to_string(),
            Value::Bool(receipt.append_suppressed_by_feature_gate),
        );
        entry.insert(
            "noop_receipt_projected".to_string(),
            Value::Bool(receipt.noop_receipt_projected),
        );
        entry.insert(
            "checkpoint_write_enabled".to_string(),
            Value::Bool(plan.checkpoint_write_enabled),
        );
    }

    let side_effects = object
        .get_mut("side_effects")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| {
            TypedCompatReportError::ContractViolation(
                "typed adapter report must expose side_effects as a JSON object".to_string(),
            )
        })?;
    side_effects.remove("live_execution_started");
    for key in [
        "report_written",
        "git_index_mutated",
        "workflow_event_log_mutated",
        "provider_invoked",
        "model_invoked",
        "gateway_or_auth_mutated",
        "native_post_mutation_performed",
        "channel_send_performed",
        "package_or_release_written",
        "public_ga_promoted",
    ] {
        side_effects.insert(key.to_string(), Value::Bool(false));
    }

    Ok(report)
}

pub fn typed_compat_report(id: &str) -> Result<Value, TypedCompatReportError> {
    match id {
        "hepta-systems-current-reality-matrix-compact-cache-boundary-readback" => serialize_report!(crate::hepta_current_reality_matrix_compact_cache_boundary_readback_report()),
        "hepta-systems-work-graph-adapter-projection-fixture" => serialize_report!(crate::hepta_work_graph_adapter_projection_fixture_report()),
        "hepta-systems-work-graph-append-only-event-intake-preview" => serialize_report!(crate::hepta_work_graph_append_only_event_intake_preview_report()),
        "hepta-systems-work-graph-append-only-store-enablement-precondition-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_enablement_precondition_preview_report()),
        "hepta-systems-work-graph-append-only-store-enablement-precondition-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_enablement_precondition_readback_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-durable-store-switch-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_durable_store_switch_application_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-durable-store-switch-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-enablement-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_enablement_application_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-enablement-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_enablement_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-idempotency-mutation-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_idempotency_mutation_application_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_rollback_readback_execution_application_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_rollback_readback_execution_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_rollback_readback_execution_readback_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_application_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_report()),
        "hepta-systems-work-graph-append-only-store-runtime-write-boundary-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_store_runtime_write_boundary_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-adapter-projection-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_adapter_projection_gap_closure_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_denial_evidence_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_denial_evidence_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-denial-evidence-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_denial_evidence_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-no-cutover-guard-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_no_cutover_guard_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-operator-review-packet-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_operator_review_packet_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-operator-review-packet-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_operator_review_packet_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_readiness_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_readiness_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-readiness-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_readiness_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-replay-readback-execution-closure-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_replay_readback_execution_closure_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-runtime-adapter-enforcement-closure-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-side-effect-lock-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_side_effect_lock_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-closeout-packet-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_closeout_packet_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-readback-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_readback_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-event-store-enablement-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_event_store_enablement_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-persistence-guard-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_persistence_guard_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-replay-readback-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_replay_readback_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-shadow-write-application-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_shadow_write_application_preview_report()),
        "hepta-systems-work-graph-append-only-work-graph-events-shadow-write-preview" => serialize_report!(crate::hepta_work_graph_append_only_work_graph_events_shadow_write_preview_report()),
        "hepta-systems-work-graph-canonical-adapter-inventory-application-preview" => serialize_report!(crate::hepta_work_graph_canonical_adapter_inventory_application_preview_report()),
        "hepta-systems-work-graph-canonical-adapter-inventory-preview" => serialize_report!(crate::hepta_work_graph_canonical_adapter_inventory_preview_report()),
        "hepta-systems-work-graph-canonical-adapter-inventory-readback-preview" => serialize_report!(crate::hepta_work_graph_canonical_adapter_inventory_readback_preview_report()),
        "hepta-systems-work-graph-current-state-inventory" => serialize_report!(crate::hepta_work_graph_current_state_inventory_report()),
        "hepta-systems-work-graph-idempotency-readback-adapter-preview" => serialize_report!(crate::hepta_work_graph_idempotency_readback_adapter_preview_report()),
        "hepta-systems-work-graph-observability-timeline-preview" => serialize_report!(crate::hepta_work_graph_observability_timeline_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-acknowledgement-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-acknowledgement-replay-idempotency-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_report()),
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-preview" => serialize_report!(crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_report()),
        "hepta-systems-work-graph-projection-adapter-gap-closure-application-preview" => serialize_report!(crate::hepta_work_graph_projection_adapter_gap_closure_application_preview_report()),
        "hepta-systems-work-graph-projection-adapter-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_projection_adapter_gap_closure_preview_report()),
        "hepta-systems-work-graph-projection-adapter-gap-closure-readback-preview" => serialize_report!(crate::hepta_work_graph_projection_adapter_gap_closure_readback_preview_report()),
        "hepta-systems-work-graph-role-manifest-enforcement-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_role_manifest_enforcement_gap_closure_preview_report()),
        "hepta-systems-work-graph-runtime-application-promotion-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_runtime_application_promotion_gap_closure_preview_report()),
        "hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_scheduler_admission_enforcement_gap_closure_preview_report()),
        "hepta-systems-work-graph-state-store-persistence-preview" => serialize_report!(crate::hepta_work_graph_state_store_persistence_preview_report()),
        "hepta-systems-work-graph-store-idempotency-guard-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_store_idempotency_guard_gap_closure_preview_report()),
        "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-application-preview" => serialize_report!(crate::hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_report()),
        "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-preview" => serialize_report!(crate::hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_report()),
        "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-readback-preview" => serialize_report!(crate::hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_report()),
        "hepta-systems-work-graph-terminal-task-result-wrapper-preview" => serialize_report!(crate::hepta_work_graph_terminal_task_result_wrapper_preview_report()),
        "hepta-systems-work-graph-unified-projection-audit-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_audit_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-runtime-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-canonical-adapter-inventory-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-operator-review-side-effect-lock-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_operator_review_side_effect_lock_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-application-promotion-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_runtime_application_promotion_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-durable-store-switch-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_runtime_durable_store_switch_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-rollback-readback-execution-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_runtime_rollback_readback_execution_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-store-guard-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-adapter-projection-gap-closure-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_adapter_projection_gap_closure_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-denial-evidence-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_denial_evidence_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-final-no-enablement-readiness-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_final_no_enablement_readiness_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-no-cutover-guard-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_no_cutover_guard_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-readiness-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_readiness_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-side-effect-lock-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_side_effect_lock_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-closeout-packet-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_closeout_packet_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-operator-review-packet-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_operator_review_packet_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-enablement-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_enablement_rerun_preview_report()),
        "hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-persistence-guard-rerun-preview" => serialize_report!(crate::hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_persistence_guard_rerun_preview_report()),
        "hepta-systems-workflow-durable-store-adapter" => {
            workflow_durable_store_adapter_compat_report()
        }
        "hepta-systems-workflow-durable-store-test-only-append-fixture" => serialize_report!(crate::hepta_workflow_durable_store_test_only_append_fixture_report()),
        "hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence" => serialize_report!(crate::hepta_workflow_temporal_lite_append_only_event_store_minimal_local_persistence_report()),
        "hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation" => serialize_report!(crate::hepta_workflow_temporal_lite_append_only_event_store_test_implementation_report()),
        "hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-deterministic-replay-validator-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_report()),
        "hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-lease-idempotency-index-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_report()),
        "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-local-persistence-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_report()),
        "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback_report()),
        "hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-local-persistence-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback_report()),
        "hepta-systems-workflow-temporal-lite-work-graph-projection-feature-gated-readback" => serialize_report!(crate::hepta_workflow_temporal_lite_work_graph_projection_feature_gated_readback_report()),
        _ => Err(TypedCompatReportError::UnknownReport(id.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn typed_compatibility_report_registry_is_unique() {
        assert!(TYPED_COMPAT_REPORT_IDS.len() >= 100);
        assert_eq!(
            TYPED_COMPAT_REPORT_IDS
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len(),
            TYPED_COMPAT_REPORT_IDS.len()
        );
    }

    #[test]
    fn representative_typed_compatibility_report_is_read_only() {
        let id = "hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-preview";
        let report = typed_compat_report(id).expect("typed report should render");
        let object = report
            .as_object()
            .expect("typed report should be an object");
        assert!(!object.is_empty());
        let side_effects = object
            .get("side_effects")
            .and_then(Value::as_object)
            .expect("typed report should declare side effects");
        assert!(
            side_effects
                .values()
                .all(|value| value == &Value::Bool(false))
        );
    }

    #[test]
    fn durable_store_compatibility_report_preserves_legacy_business_fields() {
        let report = typed_compat_report("hepta-systems-workflow-durable-store-adapter")
            .expect("durable store compatibility report should render");
        let object = report
            .as_object()
            .expect("durable store compatibility report should be an object");
        for field in [
            "source_append_only_event_intake_surface",
            "source_append_only_event_intake_ready",
            "source_append_only_event_contract_count",
            "source_append_plan_surface",
            "source_append_plan_ready",
            "source_adapter_harness_surface",
            "source_adapter_harness_ready",
            "adapter_contract_ready",
            "temporal_lite_adapter_ready",
            "next_actions",
            "next_migration_step",
            "local_gate",
            "architecture_note",
            "source_files",
            "side_effect_free",
        ] {
            assert!(object.contains_key(field), "missing legacy field {field}");
        }
        assert!(!object.contains_key("source_harness_surface"));
        assert!(!object.contains_key("source_harness_ready"));
        assert!(!object.contains_key("recommended_next_gate"));

        let entries = object
            .get("entries")
            .and_then(Value::as_array)
            .expect("durable store compatibility report should expose entries");
        assert_eq!(entries.len(), 9);
        for entry in entries {
            let entry = entry
                .as_object()
                .expect("durable store compatibility entry should be an object");
            for field in [
                "target_collection_ids",
                "required_fields",
                "idempotency_key_fields",
                "append_policy",
                "append_suppressed_by_feature_gate",
                "noop_receipt_projected",
                "checkpoint_write_enabled",
            ] {
                assert!(entry.contains_key(field), "missing entry field {field}");
            }
        }

        let side_effects = object
            .get("side_effects")
            .and_then(Value::as_object)
            .expect("durable store compatibility report should expose side effects");
        for field in [
            "report_written",
            "git_index_mutated",
            "workflow_event_log_mutated",
            "provider_invoked",
            "model_invoked",
            "gateway_or_auth_mutated",
            "native_post_mutation_performed",
            "channel_send_performed",
            "package_or_release_written",
            "public_ga_promoted",
        ] {
            assert_eq!(side_effects.get(field), Some(&Value::Bool(false)));
        }
        assert!(!side_effects.contains_key("live_execution_started"));
    }

    #[test]
    fn durable_store_contract_helpers_fail_closed_without_panicking() {
        let mut non_object = Value::Null;
        assert!(matches!(
            contract_object_mut(&mut non_object, "fixture"),
            Err(TypedCompatReportError::ContractViolation(message))
                if message == "fixture must be a JSON object"
        ));

        let mut object = serde_json::Map::new();
        assert!(matches!(
            take_contract_field(&mut object, "required", "fixture"),
            Err(TypedCompatReportError::ContractViolation(message))
                if message == "fixture must expose required field required"
        ));
    }

    #[test]
    fn unknown_typed_compatibility_report_fails_closed() {
        assert!(matches!(
            typed_compat_report("unknown"),
            Err(TypedCompatReportError::UnknownReport(id)) if id == "unknown"
        ));
    }
}
