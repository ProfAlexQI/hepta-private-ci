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

receipt_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_preview.rs
)"
receipt_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-preview-report.sh
)"
receipt_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-preview-gate.sh
)"
blocker_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_blocker_preview.rs
)"
blocker_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-blocker-preview-gate.sh
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
  --argjson receipt_rust_module_present "$receipt_rust_module_present" \
  --argjson receipt_report_script_present "$receipt_report_script_present" \
  --argjson receipt_gate_script_present "$receipt_gate_script_present" \
  --argjson blocker_rust_module_present "$blocker_rust_module_present" \
  --argjson blocker_gate_script_present "$blocker_gate_script_present" \
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
    "operator_acceptance_recording_denial_receipt",
    "approval_ledger_write_denial_receipt",
    "authority_grant_denial_receipt",
    "graph_state_persistence_denial_receipt",
    "wal_checkpoint_write_denial_receipt",
    "enforcement_rollout_denial_receipt",
    "release_publication_denial_receipt",
    "external_delivery_denial_receipt"
  ];
  def receipt($id; $surface): {
    id: $id,
    source_effect_surface_id: $surface,
    required_fields: (durable_fields + ["denialReceiptId", "effectSurfaceId", "effectBlockerIds", "applyGuardIds", "rollbackQuarantineIds", "effectApplied", "sideEffectHash", "zeroEffectProofHash"]),
    redaction_state: "hash_only_redacted",
    persistence_enabled: false,
    effect_applied: false,
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
    blocks_receipt_acceptance: true
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
    receipt("operator_acceptance_recording_denial_receipt"; "operator_acceptance_recording_effect"),
    receipt("approval_ledger_write_denial_receipt"; "approval_ledger_write_effect"),
    receipt("authority_grant_denial_receipt"; "authority_grant_effect"),
    receipt("graph_state_persistence_denial_receipt"; "graph_state_persistence_effect"),
    receipt("wal_checkpoint_write_denial_receipt"; "wal_checkpoint_write_effect"),
    receipt("enforcement_rollout_denial_receipt"; "enforcement_rollout_effect"),
    receipt("release_publication_denial_receipt"; "release_publication_effect"),
    receipt("external_delivery_denial_receipt"; "external_delivery_effect")
  ] as $denial_receipts
  | [
    check("check_durable_identity_digest"; durable_fields),
    check("check_effect_surface_digest"; ["effectSurfaceId", "requestedEffect", "requiredFieldIds"]),
    check("check_blocker_digest"; ["effectBlockerIds", "blockerReasonHash", "effectBlocked"]),
    check("check_apply_guard_digest"; ["applyGuardIds", "requiredDenialFieldsHash", "applyBlocked"]),
    check("check_rollback_quarantine_digest"; ["rollbackQuarantineIds", "rollbackOwnerRequired", "quarantineRequired"]),
    check("check_zero_side_effect_digest"; ["sideEffectHash", "zeroWriteProofHash", "zeroTrafficProofHash"]),
    check("check_prior_gate_digest"; ["priorGateId", "priorGateReportHash", "receiptSourceHash"])
  ] as $digest_checks
  | [
    mismatch("durable_identity_evidence_missing"; "denial receipt is missing durable identity evidence"),
    mismatch("missing_effect_surface_digest"; "denial receipt is missing effect surface digest"),
    mismatch("missing_effect_blocker_digest"; "denial receipt is missing blocker digest"),
    mismatch("missing_apply_guard_digest"; "denial receipt is missing apply guard digest"),
    mismatch("side_effect_digest_nonzero"; "denial receipt does not prove zero side effects"),
    mismatch("rollback_quarantine_digest_missing"; "denial receipt is missing rollback or quarantine digest"),
    mismatch("denial_receipt_persistence_attempted"; "denial receipt cannot be persisted in preview"),
    mismatch("release_publication_attempted"; "denial receipt cannot publish release status"),
    mismatch("external_delivery_attempted"; "denial receipt cannot be sent externally")
  ] as $mismatch_denials
  | [
    view("operator_effect_denial_receipt_view"; "operator"; durable_fields + ["denialReceiptId", "effectSurfaceId", "effectBlockerIds", "nextGate"]),
    view("auditor_effect_denial_receipt_digest_view"; "auditor"; durable_fields + ["denialReceiptHash", "applyGuardDigestHash", "rollbackQuarantineHash", "sideEffectHash"]),
    view("release_owner_effect_denial_receipt_view"; "release_owner"; durable_fields + ["releaseDenied", "publicationDenied", "externalDeliveryDenied", "denialReceiptId"]),
    view("runtime_effect_denial_receipt_zero_effect_view"; "system"; durable_fields + ["authorityGranted", "statePersisted", "trafficRouted", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("acceptance_effect_application_denial_receipts_require_durable_identity_evidence"; "effect application denial receipts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("effect_denial_receipts_are_hash_only"; "denial receipts contain hash-only redacted evidence"),
    invariant("effect_denial_receipts_are_non_persistent"; "denial receipts cannot write graph state or receipt state"),
    invariant("effect_denial_receipts_prove_zero_side_effects"; "denial receipts must prove zero writes, zero traffic, zero release, and zero external sends"),
    invariant("effect_denial_receipt_views_are_local_only"; "operator, auditor, release-owner, and runtime views cannot be sent externally"),
    invariant("effect_denial_receipt_requires_application_blocker_gate"; "denial receipt preview requires the effect application blocker gate"),
    invariant("effect_denial_receipt_preview_has_no_side_effects"; "this gate cannot persist receipts, record approval, grant authority, enable live execution, publish, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_gate",
      schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_preview_v1",
      preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_preview_no_receipt_write",
      denial_receipt_count: ($denial_receipts | length),
      digest_check_count: ($digest_checks | length),
      mismatch_denial_count: ($mismatch_denials | length),
      local_view_count: ($local_views | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      denial_receipts: $denial_receipts,
      digest_checks: $digest_checks,
      mismatch_denials: $mismatch_denials,
      local_views: $local_views,
      durable_identity_evidence: {
        schema_version: $durable_identity_report.schema_version,
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: durable_fields,
        required_for_receipt_ids: receipt_ids,
        durable_field_count: $durable_identity_report.durable_field_count,
        preview_binding_count: $durable_identity_report.preview_binding_count,
        invariant_count: $durable_identity_report.invariant_count,
        currently_satisfied: false
      },
      invariants: $invariants,
      recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_gate",
      ready_for_acceptance_effect_application_denial_receipt_acknowledgement_preview: true,
      ready_for_operator_acceptance: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_acceptance_effect_application_denial_receipt: {
          rust_module_present: $receipt_rust_module_present,
          report_script_present: $receipt_report_script_present,
          gate_script_present: $receipt_gate_script_present
        },
        persistence_acceptance_effect_application_blocker: {
          rust_module_present: $blocker_rust_module_present,
          gate_script_present: $blocker_gate_script_present
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
        denial_receipt_persisted: false,
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
