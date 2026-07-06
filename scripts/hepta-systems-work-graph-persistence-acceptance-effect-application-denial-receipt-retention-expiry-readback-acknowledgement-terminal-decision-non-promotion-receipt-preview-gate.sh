#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-preview-report" "$REPORT_SCRIPT")"
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
  and .gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate"
  and .schema_version == "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_v1"
  and .preview_mode == "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_hash_only_no_recording"
  and .receipt_count == 6
  and (.receipts | length) == .receipt_count
  and (.receipts | map(.id) == [
    "operator_terminal_non_promotion_decision_receipt",
    "release_owner_terminal_non_promotion_decision_receipt",
    "authority_denial_terminal_non_promotion_receipt",
    "rollout_denial_terminal_non_promotion_receipt",
    "release_publication_denial_terminal_non_promotion_receipt",
    "external_delivery_denial_terminal_non_promotion_receipt"
  ])
  and (.receipts | all(
    .persisted == false
    and .acceptance_allowed == false
    and (.source_terminal_decision_surface_ids | length) == 6
    and (.required_fields | length) >= 12
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .digest_check_count == 6
  and (.digest_checks | length) == .digest_check_count
  and (.digest_checks | all(
    .blocks_receipt_recording == true
    and (.compared_fields | length) >= 10
    and (.compared_fields | index("workflow_id") != null)
    and (.compared_fields | index("receipt_hash") != null)
  ))
  and .mismatch_denial_count == 8
  and (.mismatch_denials | length) == .mismatch_denial_count
  and (.mismatch_denials | map(.id) == [
    "durable_identity_evidence_missing",
    "missing_terminal_decision_surface_cannot_record_receipt",
    "mismatched_terminal_decision_hash_cannot_accept",
    "stale_replay_idempotency_digest_cannot_grant_authority",
    "authority_guard_absence_cannot_start_rollout",
    "release_delivery_guard_absence_cannot_publish",
    "external_delivery_receipt_echo_cannot_send",
    "receipt_readback_is_not_live_completion"
  ])
  and (.mismatch_denials | all(
    .blocks_receipt_recording == true
    and .blocks_acceptance == true
    and .blocks_authority == true
    and .blocks_release_publication == true
    and .blocks_external_delivery == true
    and (.applies_to_receipt_ids | length) == 6
  ))
  and .receipt_guard_count == 5
  and (.receipt_guards | length) == .receipt_guard_count
  and (.receipt_guards | all(
    .receipt_recording_allowed == false
    and .promotion_allowed == false
    and (.required_fields | length) >= 10
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .local_view_count == 4
  and (.local_views | length) == .local_view_count
  and (.local_views | all(
    .external_delivery_enabled == false
    and (.required_fields | length) >= 11
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == durable_fields
  and .durable_identity_evidence.required_for_receipt_ids == [
    "operator_terminal_non_promotion_decision_receipt",
    "release_owner_terminal_non_promotion_decision_receipt",
    "authority_denial_terminal_non_promotion_receipt",
    "rollout_denial_terminal_non_promotion_receipt",
    "release_publication_denial_terminal_non_promotion_receipt",
    "external_delivery_denial_terminal_non_promotion_receipt"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "terminal_non_promotion_receipts_require_durable_identity_evidence",
    "terminal_non_promotion_receipts_are_hash_only",
    "terminal_non_promotion_receipts_are_not_recorded",
    "terminal_non_promotion_receipts_are_not_acceptance",
    "terminal_non_promotion_receipts_keep_release_denied",
    "terminal_non_promotion_receipt_views_are_local_only",
    "terminal_non_promotion_receipt_preview_has_no_side_effects"
  ])
  and (.invariants | all(.required == true))
  and (.required_prior_gates[-2] == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate")
  and (.required_prior_gates[-1] == "hepta_work_graph_durable_identity_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate"
  and .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt.rust_module_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt.report_script_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt.gate_script_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion.rust_module_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview --lib

echo "Hepta WorkGraph persistence acceptance effect application denial receipt retention expiry readback acknowledgement terminal decision non-promotion receipt preview gate passed"
