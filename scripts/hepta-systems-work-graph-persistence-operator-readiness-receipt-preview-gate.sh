#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-operator-readiness-receipt-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-operator-readiness-receipt-preview-report" "$REPORT_SCRIPT")"
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
  and .gate == "hepta_work_graph_persistence_operator_readiness_receipt_preview_gate"
  and .schema_version == "work_graph_persistence_operator_readiness_receipt_preview_v1"
  and .preview_mode == "read_only_persistence_operator_readiness_receipt_preview_no_receipt_write"
  and .receipt_contract_count == 6
  and (.receipt_contracts | length) == .receipt_contract_count
  and (.receipt_contracts | map(.id) == [
    "store_persistence_readiness_receipt",
    "wal_checkpoint_readiness_receipt",
    "readback_receipt_readiness_receipt",
    "replay_execution_readiness_receipt",
    "external_publication_readiness_receipt",
    "full_rollout_abort_readiness_receipt"
  ])
  and (.receipt_contracts | all(
    .redaction_state == "redacted_hash_only"
    and .persistence_enabled == false
    and .approval_recording_enabled == false
    and .external_delivery_enabled == false
    and (.required_fields | length) >= 17
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .digest_check_count == 7
  and (.digest_checks | length) == .digest_check_count
  and (.digest_checks[] | select(.id == "check_durable_identity_digest") | .compared_fields == durable_fields)
  and (.digest_checks | all(.blocks_acceptance == true and (.compared_fields | length) >= 3))
  and .signature_denial_count == 7
  and (.signature_denials | length) == .signature_denial_count
  and (.signature_denials | map(.id) == [
    "durable_identity_evidence_missing",
    "missing_signature_hash",
    "invalid_operator_scope_signature",
    "packet_expired",
    "packet_superseded",
    "operator_scope_revoked",
    "rollback_owner_revoked"
  ])
  and (.signature_denials | all(.blocks_receipt_acceptance == true and (.applies_to_receipt_ids | length) == 6))
  and .acceptance_denial_count == 8
  and (.acceptance_denials | length) == .acceptance_denial_count
  and (.acceptance_denials | map(.id) == [
    "durable_identity_evidence_missing",
    "approval_recording_attempted",
    "release_publication_attempted",
    "external_delivery_attempted",
    "receipt_persistence_attempted",
    "enforcement_rollout_attempted",
    "live_readback_attempted",
    "readiness_receipt_not_hash_only"
  ])
  and (.acceptance_denials | all(.blocks_promotion == true and (.applies_to_receipt_ids | length) == 6))
  and .readback_view_count == 4
  and (.readback_views | length) == .readback_view_count
  and (.readback_views | map(.id) == [
    "operator_readiness_receipt_summary_view",
    "auditor_readiness_receipt_digest_view",
    "rollback_owner_revocation_view",
    "release_publication_denial_view"
  ])
  and (.readback_views | all(
    .external_delivery_enabled == false
    and (.required_fields | length) >= 11
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == durable_fields
  and .durable_identity_evidence.required_for_receipt_ids == [
    "store_persistence_readiness_receipt",
    "wal_checkpoint_readiness_receipt",
    "readback_receipt_readiness_receipt",
    "replay_execution_readiness_receipt",
    "external_publication_readiness_receipt",
    "full_rollout_abort_readiness_receipt"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count >= 5
  and .durable_identity_evidence.invariant_count >= 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "operator_readiness_receipts_require_durable_identity_evidence",
    "readiness_receipts_are_hash_only",
    "signature_denials_block_acceptance",
    "receipt_readback_is_non_persistent",
    "approval_recording_is_denied",
    "release_publication_and_external_delivery_are_denied",
    "operator_readiness_receipt_preview_has_no_side_effects"
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
    "hepta_work_graph_durable_identity_preview_gate"
  ])
  and .recommended_next_gate == "hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_gate"
  and .ready_for_operator_readiness_receipt_acknowledgement_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_operator_readiness_receipt.rust_module_present == true
  and .source_probes.persistence_operator_readiness_receipt.report_script_present == true
  and .source_probes.persistence_operator_readiness_receipt.gate_script_present == true
  and .source_probes.persistence_operator_readiness_packet.rust_module_present == true
  and .source_probes.persistence_operator_readiness_packet.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_operator_readiness_receipt_preview --lib

echo "Hepta WorkGraph persistence operator readiness receipt preview gate passed"
