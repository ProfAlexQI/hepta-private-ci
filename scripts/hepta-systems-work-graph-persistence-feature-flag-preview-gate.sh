#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-feature-flag-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-feature-flag-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_persistence_feature_flag_preview_gate"
  and .schema_version == "work_graph_persistence_feature_flag_preview_v1"
  and .preview_mode == "read_only_persistence_feature_flag_preview_no_flag_mutation"
  and .feature_flag_count == 6
  and (.feature_flags | length) == .feature_flag_count
  and (.feature_flags | map(.id) == [
    "work_graph_store_persistence_flag",
    "work_graph_wal_append_flag",
    "work_graph_checkpoint_write_flag",
    "work_graph_readback_receipt_persistence_flag",
    "work_graph_idempotency_index_write_flag",
    "work_graph_replay_execution_feature_flag"
  ])
  and (.feature_flags | all(
    .default_enabled == false
    and .operator_mutable_in_preview == false
    and .allows_live_writes_in_preview == false
    and (.required_enablement_ids | index("explicit_feature_flag"))
    and (.required_enablement_ids | index("durable_identity_evidence_packet"))
  ))
  and .enablement_packet_count == 12
  and (.enablement_packets | length) == .enablement_packet_count
  and (.enablement_packets | map(.id) == [
    "durable_identity_evidence_packet",
    "explicit_feature_flag",
    "prior_gate_digest",
    "shadow_readback_digest",
    "operator_approval_packet",
    "rollback_plan",
    "wal_schema_digest",
    "idempotency_guard_digest",
    "checkpoint_hash_plan",
    "disk_budget_packet",
    "drift_budget_packet",
    "redaction_packet"
  ])
  and (.enablement_packets | all(.currently_satisfied == false and (.required_fields | length) >= 3))
  and (.enablement_packets[] | select(.id == "durable_identity_evidence_packet") | .required_fields == [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ])
  and (.enablement_packets[] | select(.id == "durable_identity_evidence_packet") | .source_gate_ids == [
    "hepta_work_graph_durable_identity_preview_gate"
  ])
  and (.enablement_packets[] | select(.id == "prior_gate_digest") | .source_gate_ids | index("hepta_work_graph_shadow_adapter_readback_preview_gate"))
  and .rollout_stage_count == 5
  and (.rollout_stages | length) == .rollout_stage_count
  and (.rollout_stages | map(.id) == [
    "disabled",
    "local_dry_run",
    "shadow_write_fixture_only",
    "shadow_readback_compare",
    "canary_lane_dry_run"
  ])
  and (.rollout_stages | all(.traffic_ppm == 0 and .promotion_allowed == false))
  and .rollback_guard_count == 6
  and (.rollback_guards | length) == .rollback_guard_count
  and (.rollback_guards | map(.id) == [
    "operator_kill_switch",
    "wal_checksum_mismatch",
    "shadow_readback_drift",
    "idempotency_collision",
    "disk_budget_exceeded",
    "operator_approval_expired"
  ])
  and (.rollback_guards | all(.blocks_feature_flag_activation == true and .required_before_any_write == true))
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "feature_flags_require_durable_identity_evidence",
    "feature_flags_default_off",
    "prior_gate_digest_required_before_flag_enablement",
    "operator_packet_and_rollback_plan_required",
    "canary_stages_have_zero_live_traffic_in_preview",
    "rollback_guards_block_any_write_path",
    "persistence_feature_flag_preview_has_no_side_effects"
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
    "hepta_work_graph_activation_enforcement_blocker_preview_gate",
    "hepta_work_graph_shadow_adapter_readback_preview_gate",
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
  and .durable_identity_evidence.required_for_feature_flag_ids == [
    "work_graph_store_persistence_flag",
    "work_graph_wal_append_flag",
    "work_graph_checkpoint_write_flag",
    "work_graph_readback_receipt_persistence_flag",
    "work_graph_idempotency_index_write_flag",
    "work_graph_replay_execution_feature_flag"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .recommended_next_gate == "hepta_work_graph_persistence_canary_dry_run_preview_gate"
  and .ready_for_persistence_canary_dry_run_preview == true
  and .ready_for_feature_flag_activation == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_feature_flag.rust_module_present == true
  and .source_probes.persistence_feature_flag.report_script_present == true
  and .source_probes.persistence_feature_flag.gate_script_present == true
  and .source_probes.shadow_adapter_readback.gate_script_present == true
  and .source_probes.activation_blocker.gate_script_present == true
  and .source_probes.state_store_persistence.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_feature_flag_preview --lib

echo "Hepta WorkGraph persistence feature flag preview gate passed"
