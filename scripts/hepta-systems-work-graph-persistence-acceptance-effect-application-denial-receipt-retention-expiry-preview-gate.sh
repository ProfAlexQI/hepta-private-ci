#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-preview-report" "$REPORT_SCRIPT")"
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
  and .gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_gate"
  and .schema_version == "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_v1"
  and .preview_mode == "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_no_retention_write"
  and .retention_policy_count == 6
  and (.retention_policies | length) == .retention_policy_count
  and (.retention_policies | map(.id) == [
    "effect_denial_receipt_local_view_retention_policy",
    "effect_denial_receipt_acknowledgement_retention_policy",
    "effect_denial_receipt_replay_index_retention_policy",
    "effect_denial_receipt_zero_effect_digest_retention_policy",
    "effect_denial_receipt_supersession_marker_retention_policy",
    "effect_denial_receipt_release_external_denial_retention_policy"
  ])
  and (.retention_policies | all(
    .hash_only == true
    and .persistence_enabled == false
    and .garbage_collection_allowed == false
    and (.required_fields | index("workflow_id"))
    and (.required_fields | index("receipt_hash"))
    and (.required_fields | length) >= 11
  ))
  and .expiry_guard_count == 6
  and (.expiry_guards | length) == .expiry_guard_count
  and (.expiry_guards | map(.id) == [
    "retention_window_expired",
    "receipt_scope_superseded",
    "prior_gate_digest_expired",
    "zero_effect_digest_stale",
    "operator_visibility_window_expired",
    "release_external_delivery_scope_expired"
  ])
  and (.expiry_guards | all(.blocks_acceptance == true and .blocks_persistence == true and (.applies_to_policy_ids | length) == 6))
  and .supersession_guard_count == 5
  and (.supersession_guards | length) == .supersession_guard_count
  and (.supersession_guards | map(.id) == [
    "newer_effect_blocker_report_supersedes_receipt",
    "newer_denial_receipt_supersedes_acknowledgement",
    "replay_epoch_supersedes_retention_scope",
    "rollback_quarantine_owner_scope_superseded",
    "release_owner_scope_superseded"
  ])
  and (.supersession_guards | all(
    .blocks_mutation == true
    and (.required_fields | index("workflow_id"))
    and (.required_fields | index("receipt_hash"))
    and (.required_fields | length) >= 10
  ))
  and .garbage_collection_denial_count == 7
  and (.garbage_collection_denials | length) == .garbage_collection_denial_count
  and (.garbage_collection_denials | map(.id) == [
    "durable_identity_evidence_missing",
    "gc_cannot_delete_live_state",
    "gc_cannot_delete_receipt_evidence",
    "gc_cannot_persist_tombstone",
    "gc_cannot_unlock_authority",
    "gc_cannot_publish_release",
    "gc_cannot_send_external_delivery"
  ])
  and (.garbage_collection_denials | all(.garbage_collection_allowed == false and .blocks_mutation == true))
  and .local_view_count == 4
  and (.local_views | length) == .local_view_count
  and (.local_views | all(
    .external_delivery_enabled == false
    and (.required_fields | index("workflow_id"))
    and (.required_fields | index("receipt_hash"))
    and (.required_fields | length) >= 11
  ))
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "effect_denial_receipt_retention_requires_durable_identity_evidence",
    "effect_denial_receipt_retention_is_bounded",
    "effect_denial_receipt_expiry_blocks_acceptance",
    "effect_denial_receipt_supersession_blocks_mutation",
    "effect_denial_receipt_gc_is_denied",
    "effect_denial_receipt_retention_views_are_local_only",
    "effect_denial_receipt_retention_preview_has_no_side_effects"
  ])
  and (.invariants | all(.required == true))
  and (.required_prior_gates[-2] == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_replay_idempotency_preview_gate")
  and (.required_prior_gates[-1] == "hepta_work_graph_durable_identity_preview_gate")
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == durable_fields
  and .durable_identity_evidence.required_for_retention_policy_ids == [
    "effect_denial_receipt_local_view_retention_policy",
    "effect_denial_receipt_acknowledgement_retention_policy",
    "effect_denial_receipt_replay_index_retention_policy",
    "effect_denial_receipt_zero_effect_digest_retention_policy",
    "effect_denial_receipt_supersession_marker_retention_policy",
    "effect_denial_receipt_release_external_denial_retention_policy"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .recommended_next_gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_gate"
  and .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry.rust_module_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry.report_script_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry.gate_script_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_replay_idempotency.rust_module_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_replay_idempotency.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview --lib

echo "Hepta WorkGraph persistence acceptance effect application denial receipt retention expiry preview gate passed"
