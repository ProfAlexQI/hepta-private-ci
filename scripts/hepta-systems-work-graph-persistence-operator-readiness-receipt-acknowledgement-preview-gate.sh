#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-operator-readiness-receipt-acknowledgement-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-operator-readiness-receipt-acknowledgement-preview-report" "$REPORT_SCRIPT")"
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
  and .gate == "hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_gate"
  and .schema_version == "work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_v1"
  and .preview_mode == "read_only_persistence_operator_readiness_receipt_acknowledgement_preview_no_recording"
  and .acknowledgement_contract_count == 6
  and (.acknowledgement_contracts | length) == .acknowledgement_contract_count
  and (.acknowledgement_contracts | map(.id) == [
    "store_persistence_readiness_receipt_acknowledgement",
    "wal_checkpoint_readiness_receipt_acknowledgement",
    "readback_receipt_readiness_receipt_acknowledgement",
    "replay_execution_readiness_receipt_acknowledgement",
    "external_publication_readiness_receipt_acknowledgement",
    "full_rollout_abort_readiness_receipt_acknowledgement"
  ])
  and (.acknowledgement_contracts | all(
    .acceptance_allowed == false
    and .acknowledgement_recording_enabled == false
    and .external_delivery_enabled == false
    and (.required_fields | length) >= 16
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .non_acceptance_reason_count == 7
  and (.non_acceptance_reasons | length) == .non_acceptance_reason_count
  and (.non_acceptance_reasons | map(.id) == [
    "durable_identity_evidence_missing",
    "acknowledgement_is_not_operator_acceptance",
    "acknowledgement_cannot_grant_authority",
    "acknowledgement_cannot_record_approval",
    "acknowledgement_cannot_enable_live_execution",
    "acknowledgement_cannot_release_or_publish",
    "acknowledgement_cannot_send_externally"
  ])
  and (.non_acceptance_reasons | all(.blocks_acceptance == true and (.applies_to_acknowledgement_ids | length) == 6))
  and .recording_denial_count == 7
  and (.recording_denials | length) == .recording_denial_count
  and (.recording_denials | map(.id) == [
    "deny_durable_identity_ack_recording",
    "deny_operator_received_recording",
    "deny_operator_confirmed_recording",
    "deny_readback_ack_recording",
    "deny_status_ack_recording",
    "deny_channel_ack_delivery",
    "deny_external_ack_send"
  ])
  and (.recording_denials | all(.blocks_recording == true))
  and .expiry_guard_count == 4
  and (.expiry_guards | length) == .expiry_guard_count
  and (.expiry_guards | map(.id) == [
    "acknowledgement_expired",
    "acknowledgement_superseded",
    "acknowledgement_scope_revoked",
    "acknowledgement_receipt_digest_mismatch"
  ])
  and (.expiry_guards | all(.blocks_acknowledgement == true and (.applies_to_acknowledgement_ids | length) == 6))
  and .operator_view_count == 4
  and (.operator_views | length) == .operator_view_count
  and (.operator_views | map(.id) == [
    "operator_acknowledgement_non_acceptance_view",
    "auditor_acknowledgement_digest_view",
    "release_owner_acknowledgement_denial_view",
    "authority_blocker_preview_view"
  ])
  and (.operator_views | all(
    .external_delivery_enabled == false
    and (.required_fields | length) >= 11
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == durable_fields
  and .durable_identity_evidence.required_for_acknowledgement_ids == [
    "store_persistence_readiness_receipt_acknowledgement",
    "wal_checkpoint_readiness_receipt_acknowledgement",
    "readback_receipt_readiness_receipt_acknowledgement",
    "replay_execution_readiness_receipt_acknowledgement",
    "external_publication_readiness_receipt_acknowledgement",
    "full_rollout_abort_readiness_receipt_acknowledgement"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count >= 5
  and .durable_identity_evidence.invariant_count >= 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "operator_readiness_receipt_acknowledgements_require_durable_identity_evidence",
    "acknowledgement_is_non_acceptance",
    "acknowledgement_recording_is_blocked",
    "authority_cannot_derive_from_acknowledgement",
    "expiry_and_digest_guards_block_acknowledgement",
    "acknowledgement_views_are_local_only",
    "operator_readiness_receipt_acknowledgement_preview_has_no_side_effects"
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
    "hepta_work_graph_persistence_operator_readiness_packet_preview_gate",
    "hepta_work_graph_persistence_operator_readiness_receipt_preview_gate",
    "hepta_work_graph_durable_identity_preview_gate"
  ])
  and .recommended_next_gate == "hepta_work_graph_persistence_acceptance_authority_blocker_preview_gate"
  and .ready_for_acceptance_authority_blocker_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_operator_readiness_receipt_acknowledgement.rust_module_present == true
  and .source_probes.persistence_operator_readiness_receipt_acknowledgement.report_script_present == true
  and .source_probes.persistence_operator_readiness_receipt_acknowledgement.gate_script_present == true
  and .source_probes.persistence_operator_readiness_receipt.rust_module_present == true
  and .source_probes.persistence_operator_readiness_receipt.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_operator_readiness_receipt_acknowledgement_preview --lib

echo "Hepta WorkGraph persistence operator readiness receipt acknowledgement preview gate passed"
