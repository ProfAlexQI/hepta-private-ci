#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-activation-enforcement-blocker-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-activation-enforcement-blocker-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_activation_enforcement_blocker_preview_gate"
  and .schema_version == "work_graph_activation_enforcement_blocker_preview_v1"
  and .preview_mode == "read_only_activation_enforcement_blocker_preview_no_activation"
  and .activation_surface_count == 8
  and (.activation_surfaces | length) == .activation_surface_count
  and (.activation_surfaces | map(.id) == [
    "store_persistence_activation",
    "wal_replay_execution_activation",
    "promotion_execution_activation",
    "scheduler_cutover_activation",
    "adapter_projection_enforcement_activation",
    "approval_recording_activation",
    "external_delivery_activation",
    "operator_dashboard_publication_activation"
  ])
  and (.activation_surfaces | all(.blocked_by_default == true and (.required_blocker_ids | length) >= 4 and (.required_blocker_ids | index("durable_identity_evidence_missing"))))
  and .blocker_count == 15
  and (.blockers | length) == .blocker_count
  and (.blockers | map(.id) | index("durable_identity_evidence_missing"))
  and (.blockers | map(.id) | index("feature_flag_not_enabled"))
  and (.blockers | map(.id) | index("external_delivery_policy_missing"))
  and (.blockers | all(.blocks_activation == true and (.applies_to_surface_ids | length) >= 1))
  and (.blockers[] | select(.id == "durable_identity_evidence_missing") | .applies_to_surface_ids == [
    "store_persistence_activation",
    "wal_replay_execution_activation",
    "promotion_execution_activation",
    "scheduler_cutover_activation",
    "adapter_projection_enforcement_activation",
    "approval_recording_activation",
    "external_delivery_activation",
    "operator_dashboard_publication_activation"
  ])
  and .required_enablement_count == 7
  and (.required_enablements | length) == .required_enablement_count
  and (.required_enablements | map(.id) == [
    "durable_identity_evidence_packet",
    "explicit_feature_flag",
    "operator_activation_packet",
    "shadow_readback_match",
    "rollback_quarantine_plan",
    "external_scope_policy",
    "redaction_review_packet"
  ])
  and (.required_enablements | all(.currently_satisfied == false and (.required_evidence_fields | length) >= 3))
  and (.required_enablements[] | select(.id == "durable_identity_evidence_packet") | .required_evidence_fields == [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ])
  and .kill_switch_count == 4
  and (.kill_switches | length) == .kill_switch_count
  and (.kill_switches | map(.id) == [
    "kill_all_work_graph_activation",
    "kill_external_delivery_activation",
    "kill_scheduler_cutover_activation",
    "kill_adapter_enforcement_activation"
  ])
  and (.kill_switches | all(.armed_in_preview == true and (.target_surface_ids | length) >= 1))
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "activation_requires_durable_identity_evidence",
    "activation_is_blocked_by_default",
    "feature_flag_and_operator_packet_required",
    "shadow_readback_precedes_enforcement",
    "external_delivery_has_separate_policy",
    "kill_switches_are_defined_before_activation",
    "activation_blocker_preview_has_no_side_effects"
  ])
  and (.invariants | all(.required == true))
  and (.required_prior_gates == [
    "hepta_work_graph_contract_preview_gate",
    "hepta_work_graph_task_result_contract_preview_gate",
    "hepta_work_graph_scheduler_admission_controller_preview_gate",
    "hepta_work_graph_observability_timeline_preview_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate",
    "hepta_work_graph_unified_state_store_preview_gate",
    "hepta_work_graph_adapter_projection_fixture_gate",
    "hepta_work_graph_state_store_persistence_preview_gate",
    "hepta_work_graph_replay_readback_preview_gate",
    "hepta_work_graph_promotion_precondition_preview_gate",
    "hepta_work_graph_durable_identity_preview_gate"
  ])
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ]
  and .durable_identity_evidence.required_for_surface_ids == [
    "store_persistence_activation",
    "wal_replay_execution_activation",
    "promotion_execution_activation",
    "scheduler_cutover_activation",
    "adapter_projection_enforcement_activation",
    "approval_recording_activation",
    "external_delivery_activation",
    "operator_dashboard_publication_activation"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .recommended_next_gate == "hepta_work_graph_shadow_adapter_readback_preview_gate"
  and .ready_for_shadow_adapter_readback_preview == true
  and .ready_for_activation == false
  and .ready_for_live_execution == false
  and .source_probes.activation_blocker.rust_module_present == true
  and .source_probes.activation_blocker.report_script_present == true
  and .source_probes.activation_blocker.gate_script_present == true
  and .source_probes.promotion_precondition.rust_module_present == true
  and .source_probes.promotion_precondition.report_script_present == true
  and .source_probes.promotion_precondition.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_activation_enforcement_blocker_preview --lib

echo "Hepta WorkGraph activation/enforcement blocker preview gate passed"
