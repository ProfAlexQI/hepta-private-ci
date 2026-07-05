#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-preview-report" "$REPORT_SCRIPT")"
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
  and .gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate"
  and .schema_version == "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_v1"
  and .preview_mode == "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_no_recording"
  and .acknowledgement_contract_count == 6
  and (.acknowledgement_contracts | length) == .acknowledgement_contract_count
  and (.acknowledgement_contracts | map(.id) == [
    "terminal_receipt_retention_policy_readback_acknowledgement",
    "terminal_receipt_expiry_guard_readback_acknowledgement",
    "terminal_receipt_supersession_guard_readback_acknowledgement",
    "terminal_receipt_gc_denial_readback_acknowledgement",
    "terminal_receipt_zero_effect_digest_readback_acknowledgement",
    "terminal_receipt_release_public_claim_denial_readback_acknowledgement"
  ])
  and (.acknowledgement_contracts | all(
    .acceptance_allowed == false
    and .acknowledgement_recording_enabled == false
    and .receipt_recording_enabled == false
    and .authority_grant_enabled == false
    and .public_claim_enabled == false
    and .external_delivery_enabled == false
    and (.required_fields | length) >= 15
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .non_acceptance_reason_count == 8
  and (.non_acceptance_reasons | length) == .non_acceptance_reason_count
  and (.non_acceptance_reasons | map(.id) == [
    "durable_identity_evidence_missing",
    "terminal_retention_readback_ack_is_not_acceptance",
    "terminal_retention_readback_ack_cannot_record_acknowledgement",
    "terminal_retention_readback_ack_cannot_record_approval",
    "terminal_retention_readback_ack_cannot_grant_authority",
    "terminal_retention_readback_ack_cannot_enable_persistence",
    "terminal_retention_readback_ack_cannot_start_rollout",
    "terminal_retention_readback_ack_cannot_publish_or_send"
  ])
  and (.non_acceptance_reasons | all(.blocks_acceptance == true and (.applies_to_acknowledgement_ids | length) == 6))
  and .recording_denial_count == 8
  and (.recording_denials | length) == .recording_denial_count
  and (.recording_denials | all(.blocks_recording == true))
  and (.recording_denials | map(.id) == [
    "deny_durable_identity_terminal_receipt_retention_readback_ack_recording",
    "terminal_retention_readback_ack_recording_denied",
    "terminal_receipt_retention_state_recording_denied",
    "terminal_retention_readback_receipt_recording_denied",
    "terminal_operator_acceptance_recording_denied",
    "terminal_approval_ledger_recording_denied",
    "terminal_authority_grant_recording_denied",
    "terminal_release_public_claim_delivery_recording_denied"
  ])
  and .expiry_replay_guard_count == 5
  and (.expiry_replay_guards | length) == .expiry_replay_guard_count
  and (.expiry_replay_guards | all(.blocks_acknowledgement == true and (.applies_to_acknowledgement_ids | length) == 6))
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
  and .durable_identity_evidence.required_for_acknowledgement_ids == [
    "terminal_receipt_retention_policy_readback_acknowledgement",
    "terminal_receipt_expiry_guard_readback_acknowledgement",
    "terminal_receipt_supersession_guard_readback_acknowledgement",
    "terminal_receipt_gc_denial_readback_acknowledgement",
    "terminal_receipt_zero_effect_digest_readback_acknowledgement",
    "terminal_receipt_release_public_claim_denial_readback_acknowledgement"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "terminal_receipt_retention_readback_acknowledgements_require_durable_identity_evidence",
    "terminal_retention_readback_acknowledgements_are_hash_only",
    "terminal_retention_readback_acknowledgements_are_non_accepting",
    "terminal_retention_readback_acknowledgements_are_non_recording",
    "terminal_retention_readback_acknowledgement_views_are_local_only",
    "terminal_retention_readback_acknowledgement_requires_readback_receipt_gate",
    "terminal_retention_readback_acknowledgement_preview_has_no_side_effects"
  ])
  and (.invariants | all(.required == true))
  and (.required_prior_gates[-2] == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate")
  and (.required_prior_gates[-1] == "hepta_work_graph_durable_identity_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate"
  and .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement.rust_module_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement.report_script_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement.gate_script_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt.rust_module_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview --lib

echo "Hepta WorkGraph persistence acceptance effect application denial receipt retention expiry readback acknowledgement terminal decision non-promotion receipt retention expiry readback acknowledgement preview gate passed"
