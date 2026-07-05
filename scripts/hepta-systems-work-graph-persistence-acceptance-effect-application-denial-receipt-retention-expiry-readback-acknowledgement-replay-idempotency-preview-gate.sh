#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview-report" "$REPORT_SCRIPT")"
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
  and .gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate"
  and .schema_version == "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_v1"
  and .preview_mode == "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_no_replay_write"
  and .replay_scenario_count == 6
  and (.replay_scenarios | length) == .replay_scenario_count
  and (.replay_scenarios | map(.id) == [
    "duplicate_retention_readback_receipt_replay",
    "duplicate_retention_readback_acknowledgement_replay",
    "stale_retention_readback_digest_replay",
    "superseded_retention_scope_acknowledgement_replay",
    "cross_scope_retention_readback_acknowledgement_replay",
    "out_of_order_retention_readback_acknowledgement_replay"
  ])
  and (.replay_scenarios | all(
    .acknowledgement_recording_allowed == false
    and .mutation_allowed == false
    and (.source_acknowledgement_ids | length) == 6
    and (.required_fields | length) >= 11
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .idempotency_guard_count == 7
  and (.idempotency_guards | length) == .idempotency_guard_count
  and (.idempotency_guards | map(.id) == [
    "retention_readback_receipt_idempotency_key_required",
    "retention_readback_acknowledgement_idempotency_key_required",
    "retention_readback_prior_gate_digest_binding_required",
    "retention_scope_epoch_binding_required",
    "retention_zero_side_effect_digest_binding_required",
    "retention_readback_acknowledgement_sequence_required",
    "replay_does_not_unlock_retention_or_acceptance"
  ])
  and (.idempotency_guards | all(
    .blocks_replay_mutation == true
    and (.required_fields | length) >= 10
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .replay_denial_count == 8
  and (.replay_denials | length) == .replay_denial_count
  and (.replay_denials | map(.id) == [
    "durable_identity_evidence_missing",
    "duplicate_readback_cannot_record_acknowledgement",
    "duplicate_acknowledgement_cannot_record_acceptance",
    "stale_readback_digest_cannot_grant_authority",
    "cross_scope_replay_cannot_enable_live_persistence",
    "out_of_order_replay_cannot_start_rollout",
    "superseded_replay_cannot_publish_release",
    "replayed_retention_acknowledgement_cannot_send_external_delivery"
  ])
  and (.replay_denials | all(
    .blocks_acknowledgement_recording == true
    and .blocks_acceptance == true
    and .blocks_mutation == true
    and (.applies_to_replay_scenario_ids | length) == 6
  ))
  and .monotonicity_check_count == 5
  and (.monotonicity_checks | length) == .monotonicity_check_count
  and (.monotonicity_checks | map(.id) == [
    "retention_readback_receipt_sequence_check",
    "retention_readback_acknowledgement_sequence_check",
    "retention_prior_gate_digest_monotonicity_check",
    "retention_scope_epoch_monotonicity_check",
    "retention_zero_effect_digest_stability_check"
  ])
  and (.monotonicity_checks | all(
    .blocks_out_of_order_replay == true
    and (.compared_fields | length) >= 10
    and (.compared_fields | index("workflow_id") != null)
    and (.compared_fields | index("receipt_hash") != null)
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
  and .durable_identity_evidence.required_for_replay_scenario_ids == [
    "duplicate_retention_readback_receipt_replay",
    "duplicate_retention_readback_acknowledgement_replay",
    "stale_retention_readback_digest_replay",
    "superseded_retention_scope_acknowledgement_replay",
    "cross_scope_retention_readback_acknowledgement_replay",
    "out_of_order_retention_readback_acknowledgement_replay"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "retention_readback_ack_replay_requires_durable_identity_evidence",
    "retention_readback_ack_replay_is_idempotent",
    "retention_readback_ack_replay_keeps_zero_side_effects",
    "retention_readback_ack_replay_requires_acknowledgement_gate",
    "retention_readback_ack_replay_is_scope_bound",
    "retention_readback_ack_replay_views_are_local_only",
    "retention_readback_ack_replay_preview_has_no_side_effects"
  ])
  and (.invariants | all(.required == true))
  and (.required_prior_gates[-2] == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_gate")
  and (.required_prior_gates[-1] == "hepta_work_graph_durable_identity_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate"
  and .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency.rust_module_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency.report_script_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency.gate_script_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement.rust_module_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview --lib

echo "Hepta WorkGraph persistence acceptance effect application denial receipt retention expiry readback acknowledgement replay idempotency preview gate passed"
