#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

path_exists() {
  local path="$1"
  [[ -e "$path" ]]
}

bool_for() {
  if "$@"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
}

retention_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview.rs
)"
retention_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-preview-report.sh
)"
retention_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-preview-gate.sh
)"
replay_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_replay_idempotency_preview.rs
)"
replay_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-replay-idempotency-preview-gate.sh
)"
durable_identity_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_durable_identity_preview.rs
)"
durable_identity_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-durable-identity-preview-report.sh
)"
durable_identity_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-durable-identity-preview-gate.sh
)"
durable_identity_report="$(
  capture_json_report \
    "hepta-work-graph-durable-identity-preview-report" \
    "$ROOT/scripts/hepta-systems-work-graph-durable-identity-preview-report.sh"
)"

jq -n \
  --argjson retention_rust_module_present "$retention_rust_module_present" \
  --argjson retention_report_script_present "$retention_report_script_present" \
  --argjson retention_gate_script_present "$retention_gate_script_present" \
  --argjson replay_rust_module_present "$replay_rust_module_present" \
  --argjson replay_gate_script_present "$replay_gate_script_present" \
  --argjson durable_identity_rust_module_present "$durable_identity_rust_module_present" \
  --argjson durable_identity_report_script_present "$durable_identity_report_script_present" \
  --argjson durable_identity_gate_script_present "$durable_identity_gate_script_present" \
  --argjson durable_identity_report "$durable_identity_report" \
  '
  def prior_gates: [
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
    "hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_gate",
    "hepta_work_graph_persistence_acceptance_authority_blocker_preview_gate",
    "hepta_work_graph_persistence_acceptance_record_intake_preview_gate",
    "hepta_work_graph_persistence_acceptance_record_receipt_preview_gate",
    "hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_replay_idempotency_preview_gate",
    "hepta_work_graph_durable_identity_preview_gate"
  ];
  def durable_fields: [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ];
  def policy_ids: [
    "effect_denial_receipt_local_view_retention_policy",
    "effect_denial_receipt_acknowledgement_retention_policy",
    "effect_denial_receipt_replay_index_retention_policy",
    "effect_denial_receipt_zero_effect_digest_retention_policy",
    "effect_denial_receipt_supersession_marker_retention_policy",
    "effect_denial_receipt_release_external_denial_retention_policy"
  ];
  def policy($id; $scope; $window): {
    id: $id,
    scope: $scope,
    retention_window: $window,
    required_fields: (durable_fields + ["retentionPolicyId", "scope", "retentionWindow", "hashOnlyEvidence"]),
    hash_only: true,
    persistence_enabled: false,
    garbage_collection_allowed: false
  };
  def expiry($id; $trigger): {
    id: $id,
    applies_to_policy_ids: policy_ids,
    trigger: $trigger,
    blocks_acceptance: true,
    blocks_persistence: true
  };
  def supersession($id; $target; $fields): {
    id: $id,
    supersedes: $target,
    required_fields: (durable_fields + $fields),
    blocks_mutation: true
  };
  def gc_denial($id; $target; $reason): {
    id: $id,
    target: $target,
    reason: $reason,
    garbage_collection_allowed: false,
    blocks_mutation: true
  };
  def view($id; $audience; $fields): {
    id: $id,
    audience: $audience,
    required_fields: $fields,
    external_delivery_enabled: false
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  [
    policy("effect_denial_receipt_local_view_retention_policy"; "operator_auditor_release_owner_local_views"; "bounded_to_preview_session_window"),
    policy("effect_denial_receipt_acknowledgement_retention_policy"; "non_recording_acknowledgement_visibility"; "bounded_to_acknowledgement_preview_window"),
    policy("effect_denial_receipt_replay_index_retention_policy"; "idempotency_and_monotonicity_keys"; "bounded_to_replay_preview_window"),
    policy("effect_denial_receipt_zero_effect_digest_retention_policy"; "zero_write_zero_traffic_zero_external_digest"; "bounded_to_digest_verification_window"),
    policy("effect_denial_receipt_supersession_marker_retention_policy"; "scope_epoch_and_supersession_markers"; "bounded_to_current_scope_epoch"),
    policy("effect_denial_receipt_release_external_denial_retention_policy"; "release_publication_external_delivery_denials"; "bounded_to_release_owner_preview_window")
  ] as $retention_policies
  | [
    expiry("retention_window_expired"; "retention window expired without authority packet"),
    expiry("receipt_scope_superseded"; "receipt scope was superseded by a newer effect blocker report"),
    expiry("prior_gate_digest_expired"; "prior gate digest is no longer current for this scope"),
    expiry("zero_effect_digest_stale"; "zero-effect digest no longer matches local readback"),
    expiry("operator_visibility_window_expired"; "operator local visibility window expired without acceptance authority"),
    expiry("release_external_delivery_scope_expired"; "release and external delivery denial scope expired without publication authority")
  ] as $expiry_guards
  | [
    supersession("newer_effect_blocker_report_supersedes_receipt"; "effect_application_blocker_report"; ["priorGateDigest", "newGateDigest", "scopeEpoch"]),
    supersession("newer_denial_receipt_supersedes_acknowledgement"; "effect_application_denial_receipt_acknowledgement"; ["denialReceiptHash", "acknowledgementHash", "supersessionHash"]),
    supersession("replay_epoch_supersedes_retention_scope"; "denial_receipt_replay_epoch"; ["replayEpoch", "readbackSequence", "scopeEpoch"]),
    supersession("rollback_quarantine_owner_scope_superseded"; "rollback_quarantine_owner_scope"; ["rollbackOwnerHash", "quarantineHash", "scopeEpoch"]),
    supersession("release_owner_scope_superseded"; "release_publication_external_delivery_scope"; ["releaseOwnerHash", "publicationDenied", "externalDeliveryDenied"])
  ] as $supersession_guards
  | [
    gc_denial("durable_identity_evidence_missing"; "durable_identity_retention_evidence"; "retention expiry preview cannot advance without durable identity evidence"),
    gc_denial("gc_cannot_delete_live_state"; "work_graph_state_store"; "retention expiry preview cannot mutate live graph state"),
    gc_denial("gc_cannot_delete_receipt_evidence"; "effect_denial_receipt_evidence"; "hash-only denial evidence remains local readback evidence, not a deletable live record"),
    gc_denial("gc_cannot_persist_tombstone"; "retention_tombstone_store"; "preview garbage collection cannot persist tombstones"),
    gc_denial("gc_cannot_unlock_authority"; "authority_grant_record"; "expiry or garbage collection cannot grant authority"),
    gc_denial("gc_cannot_publish_release"; "release_publication_record"; "expiry or garbage collection cannot publish release status"),
    gc_denial("gc_cannot_send_external_delivery"; "external_delivery_record"; "expiry or garbage collection cannot send external delivery")
  ] as $garbage_collection_denials
  | [
    view("operator_effect_denial_receipt_retention_expiry_view"; "operator"; durable_fields + ["retentionPolicyId", "expiryGuardId", "expired", "nextGate"]),
    view("auditor_effect_denial_receipt_retention_digest_view"; "auditor"; durable_fields + ["retentionPolicyHash", "expiryGuardHash", "supersessionHash", "zeroEffectHash"]),
    view("release_owner_effect_denial_receipt_gc_denial_view"; "release_owner"; durable_fields + ["releaseDenied", "publicationDenied", "externalDeliveryDenied", "garbageCollectionDenied"]),
    view("runtime_effect_denial_receipt_retention_zero_effect_view"; "system"; durable_fields + ["retentionStatePersisted", "garbageCollectionPerformed", "authorityGranted", "trafficRouted", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("effect_denial_receipt_retention_requires_durable_identity_evidence"; "effect denial receipt retention expiry requires workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("effect_denial_receipt_retention_is_bounded"; "retention policies are bounded to local preview windows"),
    invariant("effect_denial_receipt_expiry_blocks_acceptance"; "expired receipt visibility cannot become acceptance or approval recording"),
    invariant("effect_denial_receipt_supersession_blocks_mutation"; "superseded scope, digest, replay epoch, or owner scope cannot mutate state"),
    invariant("effect_denial_receipt_gc_is_denied"; "garbage collection is preview-denied and cannot persist tombstones"),
    invariant("effect_denial_receipt_retention_views_are_local_only"; "retention, expiry, supersession, and GC denial views cannot be sent externally"),
    invariant("effect_denial_receipt_retention_preview_has_no_side_effects"; "this gate cannot persist retention state, record expiry, grant authority, publish, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_gate",
      schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_v1",
      preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_no_retention_write",
      retention_policy_count: ($retention_policies | length),
      expiry_guard_count: ($expiry_guards | length),
      supersession_guard_count: ($supersession_guards | length),
      garbage_collection_denial_count: ($garbage_collection_denials | length),
      local_view_count: ($local_views | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      retention_policies: $retention_policies,
      expiry_guards: $expiry_guards,
      supersession_guards: $supersession_guards,
      garbage_collection_denials: $garbage_collection_denials,
      local_views: $local_views,
      durable_identity_evidence: {
        schema_version: $durable_identity_report.schema_version,
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: durable_fields,
        required_for_retention_policy_ids: policy_ids,
        durable_field_count: (durable_fields | length),
        preview_binding_count: 5,
        invariant_count: ($invariants | length),
        currently_satisfied: false
      },
      invariants: $invariants,
      recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_gate",
      ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview: true,
      ready_for_operator_acceptance: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_acceptance_effect_application_denial_receipt_retention_expiry: {
          rust_module_present: $retention_rust_module_present,
          report_script_present: $retention_report_script_present,
          gate_script_present: $retention_gate_script_present
        },
        persistence_acceptance_effect_application_denial_receipt_replay_idempotency: {
          rust_module_present: $replay_rust_module_present,
          gate_script_present: $replay_gate_script_present
        },
        durable_identity: {
          rust_module_present: $durable_identity_rust_module_present,
          report_script_present: $durable_identity_report_script_present,
          gate_script_present: $durable_identity_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        retention_state_persisted: false,
        expiry_recorded: false,
        garbage_collection_performed: false,
        tombstone_persisted: false,
        operator_acceptance_recorded: false,
        approval_recorded: false,
        authority_granted: false,
        live_persistence_enabled: false,
        wal_written: false,
        checkpoint_written: false,
        enforcement_enabled: false,
        rollout_started: false,
        traffic_routed: false,
        release_published: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
