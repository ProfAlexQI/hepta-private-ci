#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-operator-readiness-packet-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-operator-readiness-packet-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  def durable_fields: [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ];
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_persistence_operator_readiness_packet_preview_gate"
  and .schema_version == "work_graph_persistence_operator_readiness_packet_preview_v1"
  and .preview_mode == "read_only_persistence_operator_readiness_packet_preview_no_acceptance"
  and .packet_template_count == 6
  and (.packet_templates | length) == .packet_template_count
  and (.packet_templates | map(.id) == [
    "store_persistence_readiness_packet",
    "wal_checkpoint_readiness_packet",
    "readback_receipt_readiness_packet",
    "replay_execution_readiness_packet",
    "external_publication_readiness_packet",
    "full_rollout_abort_readiness_packet"
  ])
  and (.packet_templates | all(
    .acceptance_allowed == false
    and .external_delivery_enabled == false
    and (.required_section_ids | length) >= 5
    and (.required_section_ids | index("durable_identity_section") != null)
  ))
  and .packet_section_count == 15
  and (.packet_sections | length) == .packet_section_count
  and (.packet_sections[] | select(.id == "durable_identity_section") | .required_fields == durable_fields)
  and (.packet_sections | all(
    .redaction_state == "redacted_hash_only"
    and .currently_complete == false
    and (.required_fields | length) >= 3
  ))
  and .validation_denial_count == 9
  and (.validation_denials | length) == .validation_denial_count
  and (.validation_denials | map(.id) == [
    "deny_missing_durable_identity_evidence",
    "deny_missing_operator_scope",
    "deny_missing_shadow_live_digest",
    "deny_missing_rollback_owner",
    "deny_release_denial_matrix_missing",
    "deny_traffic_ramp_not_zero",
    "deny_receipt_redaction_missing",
    "deny_packet_expired_or_revoked",
    "deny_external_policy_missing"
  ])
  and (.validation_denials | all(.blocks_acceptance == true and (.applies_to_section_ids | length) >= 1))
  and .acceptance_guard_count == 6
  and (.acceptance_guards | length) == .acceptance_guard_count
  and (.acceptance_guards | map(.id) == [
    "guard_durable_identity_evidence_declared",
    "guard_non_recording_preview_acceptance",
    "guard_all_sections_complete",
    "guard_release_publication_denied",
    "guard_rollback_owners_declared",
    "guard_expiry_and_revocation_current"
  ])
  and (.acceptance_guards | all(
    .currently_satisfied == false
    and (.applies_to_template_ids | length) == 6
    and (.required_evidence_fields | length) >= 3
  ))
  and (.acceptance_guards[] | select(.id == "guard_durable_identity_evidence_declared") | .required_evidence_fields == durable_fields)
  and .expiry_revocation_count == 4
  and (.expiry_revocations | length) == .expiry_revocation_count
  and (.expiry_revocations | map(.id) == [
    "readiness_packet_expired",
    "readiness_packet_superseded",
    "operator_scope_revoked",
    "rollback_owner_revoked"
  ])
  and (.expiry_revocations | all(.blocks_acceptance == true and (.applies_to_template_ids | length) == 6))
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == durable_fields
  and .durable_identity_evidence.required_for_template_ids == [
    "store_persistence_readiness_packet",
    "wal_checkpoint_readiness_packet",
    "readback_receipt_readiness_packet",
    "replay_execution_readiness_packet",
    "external_publication_readiness_packet",
    "full_rollout_abort_readiness_packet"
  ]
  and .durable_identity_evidence.required_section_id == "durable_identity_section"
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count >= 5
  and .durable_identity_evidence.invariant_count >= 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "operator_readiness_requires_durable_identity_evidence",
    "readiness_packets_are_non_accepting",
    "every_packet_requires_operator_scope",
    "release_and_publication_stay_denied",
    "expiry_revocation_blocks_acceptance",
    "external_delivery_requires_separate_policy",
    "operator_readiness_packet_preview_has_no_side_effects"
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
    "hepta_work_graph_persistence_promotion_blocker_preview_gate",
    "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate",
    "hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate",
    "hepta_work_graph_durable_identity_preview_gate"
  ])
  and .recommended_next_gate == "hepta_work_graph_persistence_operator_readiness_receipt_preview_gate"
  and .ready_for_operator_readiness_receipt_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_operator_readiness_packet.rust_module_present == true
  and .source_probes.persistence_operator_readiness_packet.report_script_present == true
  and .source_probes.persistence_operator_readiness_packet.gate_script_present == true
  and .source_probes.persistence_enforcement_rollout_blocker.rust_module_present == true
  and .source_probes.persistence_enforcement_rollout_blocker.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_operator_readiness_packet_preview --lib

echo "Hepta WorkGraph persistence operator readiness packet preview gate passed"
