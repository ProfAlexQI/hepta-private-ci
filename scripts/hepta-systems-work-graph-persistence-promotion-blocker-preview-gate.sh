#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-promotion-blocker-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-promotion-blocker-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_persistence_promotion_blocker_preview_gate"
  and .schema_version == "work_graph_persistence_promotion_blocker_preview_v1"
  and .preview_mode == "read_only_persistence_promotion_blocker_preview_no_promotion"
  and .promotion_blocker_count == 8
  and (.promotion_blockers | length) == .promotion_blocker_count
  and (.promotion_blockers | map(.id) == [
    "missing_canary_receipt_digest",
    "durable_identity_evidence_missing",
    "operator_acknowledgement_missing",
    "release_denial_matrix_missing",
    "rollback_quarantine_not_armed",
    "zero_write_or_traffic_receipt_absent",
    "canary_scope_exceeds_backend_lane",
    "external_delivery_policy_missing"
  ])
  and (.promotion_blockers | all(
    .blocks_persistence_promotion == true
    and .blocks_live_execution == true
    and (.applies_to_target_ids | length) >= 1
    and (.required_evidence_fields | length) >= 3
    and (.required_evidence_fields | index("workflow_id"))
    and (.required_evidence_fields | index("run_id"))
    and (.required_evidence_fields | index("step_id"))
    and (.required_evidence_fields | index("checkpoint"))
    and (.required_evidence_fields | index("replay_key"))
    and (.required_evidence_fields | index("rollback_anchor"))
    and (.required_evidence_fields | index("receipt_hash"))
  ))
  and .release_denial_count == 6
  and (.release_denials | length) == .release_denial_count
  and (.release_denials | map(.id) == [
    "deny_store_persistence_release",
    "deny_wal_append_release",
    "deny_checkpoint_release",
    "deny_readback_receipt_release",
    "deny_replay_execution_release",
    "deny_external_publication_release"
  ])
  and (.release_denials | all(
    .blocks_release == true
    and (.applies_to_blocker_ids | length) >= 3
    and (.applies_to_blocker_ids | index("durable_identity_evidence_missing"))
  ))
  and (.release_denials | map(.target_channel) == [
    "durable_work_graph_store",
    "work_graph_wal",
    "work_graph_checkpoint",
    "readback_receipt_store",
    "work_graph_replay_executor",
    "external_delivery"
  ])
  and .operator_acknowledgement_count == 5
  and (.operator_acknowledgements | length) == .operator_acknowledgement_count
  and (.operator_acknowledgements | map(.id) == [
    "operator_promotion_blocker_ack",
    "auditor_digest_ack",
    "release_owner_non_acceptance_ack",
    "rollback_owner_quarantine_ack",
    "external_delivery_scope_ack"
  ])
  and (.operator_acknowledgements | all(
    .currently_satisfied == false
    and .external_delivery_enabled == false
    and .approval_recorded == false
    and (.required_fields | length) >= 4
    and (.required_fields | index("workflow_id"))
    and (.required_fields | index("receipt_hash"))
  ))
  and .rollback_quarantine_count == 6
  and (.rollback_quarantines | length) == .rollback_quarantine_count
  and (.rollback_quarantines | map(.id) == [
    "quarantine_store_persistence_on_missing_receipt",
    "quarantine_wal_append_on_zero_write_failure",
    "quarantine_replay_execution_on_lane_scope_failure",
    "quarantine_release_publication_on_policy_gap",
    "quarantine_promotion_on_operator_gap",
    "quarantine_promotion_on_durable_identity_gap"
  ])
  and (.rollback_quarantines | all(.armed_in_preview == true))
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
  and .durable_identity_evidence.required_for_promotion_target_ids == [
    "store_persistence_promotion",
    "wal_append_promotion",
    "checkpoint_write_promotion",
    "readback_receipt_persistence_promotion",
    "idempotency_index_promotion",
    "replay_execution_promotion",
    "external_release_publication_promotion"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "promotion_blockers_require_durable_identity_evidence",
    "promotion_blocked_after_canary_until_acknowledged",
    "release_denials_are_target_specific",
    "operator_acknowledgement_is_non_recording",
    "rollback_quarantine_precedes_promotion_execution",
    "external_release_has_independent_denial",
    "persistence_promotion_blocker_preview_has_no_side_effects"
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
    "hepta_work_graph_persistence_feature_flag_preview_gate",
    "hepta_work_graph_persistence_canary_dry_run_preview_gate",
    "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
    "hepta_work_graph_durable_identity_preview_gate"
  ])
  and .recommended_next_gate == "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate"
  and .ready_for_shadow_live_readback_comparison_preview == true
  and .ready_for_persistence_promotion == false
  and .ready_for_release_publication == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_promotion_blocker.rust_module_present == true
  and .source_probes.persistence_promotion_blocker.report_script_present == true
  and .source_probes.persistence_promotion_blocker.gate_script_present == true
  and .source_probes.persistence_canary_readback_receipt.rust_module_present == true
  and .source_probes.persistence_canary_readback_receipt.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_promotion_blocker_preview --lib

echo "Hepta WorkGraph persistence promotion blocker preview gate passed"
