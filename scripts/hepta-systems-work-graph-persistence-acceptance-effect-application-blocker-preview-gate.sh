#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-blocker-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-acceptance-effect-application-blocker-preview-report" "$REPORT_SCRIPT")"
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
  and .gate == "hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_gate"
  and .schema_version == "work_graph_persistence_acceptance_effect_application_blocker_preview_v1"
  and .preview_mode == "read_only_persistence_acceptance_effect_application_blocker_preview_no_apply"
  and .effect_surface_count == 8
  and (.effect_surfaces | length) == .effect_surface_count
  and (.effect_surfaces | map(.id) == [
    "operator_acceptance_recording_effect",
    "approval_ledger_write_effect",
    "authority_grant_effect",
    "graph_state_persistence_effect",
    "wal_checkpoint_write_effect",
    "enforcement_rollout_effect",
    "release_publication_effect",
    "external_delivery_effect"
  ])
  and (.effect_surfaces | all(
    .effect_applied == false
    and .persistence_enabled == false
    and .external_delivery_enabled == false
    and (.required_fields | length) >= 11
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .effect_blocker_count == 10
  and (.effect_blockers | length) == .effect_blocker_count
  and (.effect_blockers | map(.id) == [
    "durable_identity_evidence_missing",
    "accepted_looking_record_is_not_apply_authority",
    "receipt_acknowledgement_is_not_apply_authority",
    "approval_recording_precondition_absent",
    "authority_grant_precondition_absent",
    "persistence_feature_flag_still_disabled",
    "zero_write_or_traffic_receipt_required",
    "rollback_quarantine_not_armed_for_apply",
    "release_publication_policy_not_accepted",
    "external_delivery_consent_absent"
  ])
  and (.effect_blockers | all(.blocks_effect_application == true and (.applies_to_effect_surface_ids | length) >= 1))
  and .apply_guard_count == 7
  and (.apply_guards | length) == .apply_guard_count
  and (.apply_guards | map(.id) == [
    "record_to_approval_recording_guard",
    "receipt_ack_to_authority_guard",
    "authority_to_persistence_guard",
    "persistence_to_wal_checkpoint_guard",
    "persistence_to_rollout_guard",
    "rollout_to_release_guard",
    "release_to_external_delivery_guard"
  ])
  and (.apply_guards | all(
    .blocks_apply == true
    and (.required_denial_fields | length) >= 10
    and (.required_denial_fields | index("workflow_id") != null)
    and (.required_denial_fields | index("receipt_hash") != null)
  ))
  and .rollback_quarantine_count == 5
  and (.rollback_quarantines | length) == .rollback_quarantine_count
  and (.rollback_quarantines | map(.id) == [
    "graph_state_persistence_quarantine",
    "wal_checkpoint_write_quarantine",
    "enforcement_rollout_quarantine",
    "release_publication_quarantine",
    "external_delivery_quarantine"
  ])
  and (.rollback_quarantines | all(
    .rollback_owner_required == true
    and .quarantine_required == true
    and .armed_in_preview == false
  ))
  and .local_view_count == 4
  and (.local_views | length) == .local_view_count
  and (.local_views | map(.id) == [
    "operator_effect_application_blocker_view",
    "auditor_effect_application_denial_view",
    "release_owner_effect_application_blocker_view",
    "runtime_effect_application_zero_effect_view"
  ])
  and (.local_views | all(
    .external_delivery_enabled == false
    and (.required_fields | length) >= 11
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == durable_fields
  and .durable_identity_evidence.required_for_effect_surface_ids == [
    "operator_acceptance_recording_effect",
    "approval_ledger_write_effect",
    "authority_grant_effect",
    "graph_state_persistence_effect",
    "wal_checkpoint_write_effect",
    "enforcement_rollout_effect",
    "release_publication_effect",
    "external_delivery_effect"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count >= 5
  and .durable_identity_evidence.invariant_count >= 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "acceptance_effect_application_requires_durable_identity_evidence",
    "accepted_looking_records_cannot_apply_effects",
    "approval_and_authority_effects_are_blocked",
    "persistence_and_rollout_effects_are_blocked",
    "release_and_external_delivery_effects_are_blocked",
    "rollback_quarantine_required_but_not_armed",
    "acceptance_effect_application_blocker_preview_has_no_side_effects"
  ])
  and (.invariants | all(.required == true))
  and (.required_prior_gates[-1] == "hepta_work_graph_durable_identity_preview_gate")
  and (.required_prior_gates | index("hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_gate") != null)
  and .recommended_next_gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_gate"
  and .ready_for_acceptance_effect_application_denial_receipt_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_acceptance_effect_application_blocker.rust_module_present == true
  and .source_probes.persistence_acceptance_effect_application_blocker.report_script_present == true
  and .source_probes.persistence_acceptance_effect_application_blocker.gate_script_present == true
  and .source_probes.persistence_acceptance_record_receipt_acknowledgement.rust_module_present == true
  and .source_probes.persistence_acceptance_record_receipt_acknowledgement.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_acceptance_effect_application_blocker_preview --lib

echo "Hepta WorkGraph persistence acceptance effect application blocker preview gate passed"
