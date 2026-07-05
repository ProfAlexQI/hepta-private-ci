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

readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview.rs
)"
readback_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-receipt-preview-report.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-receipt-preview-gate.sh
)"
retention_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview.rs
)"
retention_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-preview-gate.sh
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
  --argjson readback_rust_module_present "$readback_rust_module_present" \
  --argjson readback_report_script_present "$readback_report_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  --argjson retention_rust_module_present "$retention_rust_module_present" \
  --argjson retention_gate_script_present "$retention_gate_script_present" \
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
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_gate",
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
  def receipt_ids: [
    "retention_policy_readback_receipt",
    "expiry_guard_readback_receipt",
    "supersession_guard_readback_receipt",
    "garbage_collection_denial_readback_receipt",
    "zero_effect_digest_readback_receipt",
    "release_external_denial_readback_receipt"
  ];
  def receipt($id; $source): {
    id: $id,
    source_retention_surface: $source,
    required_fields: (durable_fields + ["readbackReceiptId", "sourceRetentionSurface", "retentionPolicyHash", "expiryGuardHash", "supersessionHash", "garbageCollectionDenialHash", "zeroEffectHash", "nextGate"]),
    redaction_state: "hash_only_redacted",
    persistence_enabled: false,
    external_delivery_enabled: false
  };
  def check($id; $fields): {
    id: $id,
    compared_fields: $fields,
    blocks_receipt_acceptance: true
  };
  def mismatch($id; $reason): {
    id: $id,
    applies_to_receipt_ids: receipt_ids,
    reason: $reason,
    blocks_acceptance: true,
    blocks_persistence: true
  };
  def guard($id; $fields): {
    id: $id,
    required_fields: $fields,
    blocks_recording: true
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
    receipt("retention_policy_readback_receipt"; "effect_denial_receipt_retention_policies"),
    receipt("expiry_guard_readback_receipt"; "retention_expiry_guards"),
    receipt("supersession_guard_readback_receipt"; "retention_supersession_guards"),
    receipt("garbage_collection_denial_readback_receipt"; "retention_garbage_collection_denials"),
    receipt("zero_effect_digest_readback_receipt"; "retention_zero_effect_digests"),
    receipt("release_external_denial_readback_receipt"; "release_publication_external_delivery_denials")
  ] as $readback_receipts
  | [
    check("check_durable_identity_digest"; durable_fields),
    check("check_retention_policy_digest"; durable_fields + ["retentionPolicyIds", "retentionWindowHash", "hashOnly"]),
    check("check_expiry_guard_digest"; durable_fields + ["expiryGuardIds", "expired", "blocksPersistence"]),
    check("check_supersession_digest"; durable_fields + ["supersessionGuardIds", "scopeEpochHash", "blocksMutation"]),
    check("check_garbage_collection_denial_digest"; durable_fields + ["garbageCollectionDenialIds", "gcAllowed", "tombstonePersisted"]),
    check("check_zero_effect_digest"; durable_fields + ["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"]),
    check("check_prior_gate_digest"; durable_fields + ["priorGateId", "priorGateDigest", "readbackReceiptHash"])
  ] as $digest_checks
  | [
    mismatch("durable_identity_evidence_missing"; "retention readback receipt is missing durable identity evidence"),
    mismatch("missing_retention_policy_digest"; "retention readback receipt is missing policy digest"),
    mismatch("expired_receipt_replayed"; "expired retention receipt was replayed"),
    mismatch("superseded_scope_replayed"; "superseded retention scope was replayed"),
    mismatch("garbage_collection_tombstone_persistence_attempted"; "retention readback attempted to persist a GC tombstone"),
    mismatch("zero_effect_digest_nonzero"; "retention readback does not prove zero side effects"),
    mismatch("release_publication_attempted"; "retention readback cannot publish release status"),
    mismatch("external_delivery_attempted"; "retention readback cannot send external delivery")
  ] as $mismatch_denials
  | [
    guard("hash_only_retention_receipt_required"; durable_fields + ["retentionPolicyHash", "expiryGuardHash", "supersessionHash"]),
    guard("non_persistent_readback_required"; durable_fields + ["persistenceEnabled", "receiptPersisted", "tombstonePersisted"]),
    guard("local_view_only_required"; durable_fields + ["operatorViewHash", "auditorViewHash", "releaseOwnerViewHash"]),
    guard("bounded_retention_window_required"; durable_fields + ["retentionWindow", "expiryState", "scopeEpoch"]),
    guard("next_gate_acknowledgement_required"; durable_fields + ["recommendedNextGate", "acknowledgementAllowed", "acceptanceAllowed"])
  ] as $receipt_guards
  | [
    view("operator_retention_readback_receipt_view"; "operator"; durable_fields + ["readbackReceiptId", "retentionPolicyId", "expired", "nextGate"]),
    view("auditor_retention_readback_digest_view"; "auditor"; durable_fields + ["readbackReceiptHash", "retentionPolicyHash", "gcDenialHash", "zeroEffectHash"]),
    view("release_owner_retention_readback_denial_view"; "release_owner"; durable_fields + ["releaseDenied", "publicationDenied", "externalDeliveryDenied", "readbackReceiptId"]),
    view("runtime_retention_readback_zero_effect_view"; "system"; durable_fields + ["retentionStatePersisted", "readbackReceiptPersisted", "authorityGranted", "trafficRouted", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("retention_readback_receipts_require_durable_identity_evidence"; "retention expiry readback receipts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("retention_readback_receipts_are_hash_only"; "retention expiry readback receipts contain hash-only redacted evidence"),
    invariant("retention_readback_receipts_are_non_persistent"; "retention expiry readback cannot write receipt, retention, expiry, or tombstone state"),
    invariant("retention_readback_receipts_block_acceptance"; "retention expiry readback cannot become acceptance or approval recording"),
    invariant("retention_readback_receipts_block_gc_mutation"; "retention expiry readback cannot perform garbage collection or persist tombstones"),
    invariant("retention_readback_receipt_views_are_local_only"; "operator, auditor, release-owner, and runtime readback views cannot be sent externally"),
    invariant("retention_readback_receipt_preview_has_no_side_effects"; "this gate cannot persist, grant authority, enable live execution, publish, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_gate",
      schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_v1",
      preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_no_receipt_write",
      readback_receipt_count: ($readback_receipts | length),
      digest_check_count: ($digest_checks | length),
      mismatch_denial_count: ($mismatch_denials | length),
      receipt_guard_count: ($receipt_guards | length),
      local_view_count: ($local_views | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      readback_receipts: $readback_receipts,
      digest_checks: $digest_checks,
      mismatch_denials: $mismatch_denials,
      receipt_guards: $receipt_guards,
      local_views: $local_views,
      durable_identity_evidence: {
        schema_version: $durable_identity_report.schema_version,
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: durable_fields,
        required_for_readback_receipt_ids: receipt_ids,
        durable_field_count: (durable_fields | length),
        preview_binding_count: 5,
        invariant_count: ($invariants | length),
        currently_satisfied: false
      },
      invariants: $invariants,
      recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_gate",
      ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview: true,
      ready_for_operator_acceptance: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt: {
          rust_module_present: $readback_rust_module_present,
          report_script_present: $readback_report_script_present,
          gate_script_present: $readback_gate_script_present
        },
        persistence_acceptance_effect_application_denial_receipt_retention_expiry: {
          rust_module_present: $retention_rust_module_present,
          gate_script_present: $retention_gate_script_present
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
        readback_receipt_persisted: false,
        receipt_acknowledgement_recorded: false,
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
