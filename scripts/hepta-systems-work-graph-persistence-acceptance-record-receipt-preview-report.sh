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
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_record_receipt_preview.rs
)"
receipt_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-record-receipt-preview-report.sh
)"
receipt_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-record-receipt-preview-gate.sh
)"
intake_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_record_intake_preview.rs
)"
intake_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-record-intake-preview-gate.sh
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
  --argjson intake_rust_module_present "$intake_rust_module_present" \
  --argjson intake_gate_script_present "$intake_gate_script_present" \
  --argjson durable_identity_rust_module_present "$durable_identity_rust_module_present" \
  --argjson durable_identity_report_script_present "$durable_identity_report_script_present" \
  --argjson durable_identity_gate_script_present "$durable_identity_gate_script_present" \
  --argjson durable_identity_report "$durable_identity_report" \
  '
  def durable_fields: [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ];
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
    "hepta_work_graph_durable_identity_preview_gate"
  ];
  def receipt_ids: [
    "trusted_operator_acceptance_record_receipt",
    "approval_decision_record_receipt",
    "live_persistence_enablement_record_receipt",
    "rollback_quarantine_owner_attestation_receipt",
    "release_publication_owner_attestation_receipt",
    "external_delivery_consent_record_receipt"
  ];
  def receipt($id; $source): {
    id: $id,
    source_record_template_id: $source,
    required_fields: (durable_fields + ["receiptId", "sourceRecordTemplateId", "recordDigestHash", "intakeGuardIds", "validationDenialIds", "redactionDigestIds", "acceptanceDenied", "authorityDenied", "sideEffectHash"]),
    redaction_state: "hash_only_redacted",
    persistence_enabled: false,
    approval_recording_enabled: false,
    authority_grant_enabled: false,
    external_delivery_enabled: false
  };
  def check($id; $fields): {
    id: $id,
    compared_fields: $fields,
    blocks_acceptance: true
  };
  def denial($id; $reason): {
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
    receipt("trusted_operator_acceptance_record_receipt"; "trusted_operator_acceptance_record"),
    receipt("approval_decision_record_receipt"; "approval_decision_record"),
    receipt("live_persistence_enablement_record_receipt"; "live_persistence_enablement_record"),
    receipt("rollback_quarantine_owner_attestation_receipt"; "rollback_quarantine_owner_attestation"),
    receipt("release_publication_owner_attestation_receipt"; "release_publication_owner_attestation"),
    receipt("external_delivery_consent_record_receipt"; "external_delivery_consent_record")
  ] as $receipt_contracts
  | [
    check("check_durable_identity_digest"; durable_fields),
    check("check_acceptance_record_template_digest"; ["recordTemplateId", "recordDigestHash", "requiredFieldIds"]),
    check("check_validation_denial_digest"; ["validationDenialIds", "blockedEffectIds", "missingFieldIds"]),
    check("check_redaction_digest"; ["redactionDigestIds", "hashOnlyFieldIds", "redactionState"]),
    check("check_authority_scope_digest"; ["targetAuthoritySurfaceIds", "authorityDenied", "explicitAuthorityGrantDenied"]),
    check("check_side_effect_guard_digest"; ["intakeGuardIds", "sideEffectHash", "persistenceDenied"]),
    check("check_expiry_revocation_digest"; ["expiresAtUnixMs", "revocationStatus", "supersessionId"])
  ] as $digest_checks
  | [
    denial("durable_identity_evidence_missing"; "receipt is missing durable identity evidence"),
    denial("missing_record_digest_hash"; "receipt is missing the hash-only acceptance record digest"),
    denial("record_template_absent"; "source acceptance record template is absent from preview intake"),
    denial("validation_denial_present"; "acceptance record validation denials are still present"),
    denial("recording_attempted"; "receipt cannot record acceptance or approval decisions"),
    denial("authority_grant_attempted"; "receipt cannot grant WorkGraph persistence authority"),
    denial("live_execution_attempted"; "receipt cannot enable live persistence, WAL, checkpoint, enforcement, or rollout execution"),
    denial("release_publication_attempted"; "receipt cannot publish release status or artifacts"),
    denial("external_delivery_attempted"; "receipt cannot send acknowledgements or readiness externally")
  ] as $receipt_denials
  | [
    view("operator_acceptance_record_receipt_view"; "operator"; durable_fields + ["receiptId", "recordTemplateId", "receiptDenialIds", "acceptanceDenied"]),
    view("auditor_acceptance_record_receipt_digest_view"; "auditor"; durable_fields + ["recordDigestHash", "receiptDigestHash", "redactionDigestIds", "sideEffectHash"]),
    view("release_owner_acceptance_record_receipt_denial_view"; "release_owner"; durable_fields + ["releaseDenied", "publicationDenied", "externalDeliveryDenied", "receiptDenialIds"]),
    view("runtime_acceptance_record_receipt_zero_effect_view"; "system"; durable_fields + ["operatorAcceptanceRecorded", "authorityGranted", "livePersistenceEnabled", "externalSendPerformed"])
  ] as $readback_views
  | [
    invariant("acceptance_record_receipts_require_durable_identity_evidence"; "acceptance record receipts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("acceptance_record_receipts_are_hash_only"; "acceptance record receipts contain hash-only redacted evidence"),
    invariant("acceptance_record_receipts_are_non_persistent"; "receipt contracts cannot write acceptance, approval, authority, or graph state"),
    invariant("acceptance_record_receipts_block_live_effects"; "receipts block live persistence, WAL, checkpoints, enforcement, rollout, traffic, release, and delivery"),
    invariant("acceptance_record_receipt_readback_views_are_local_only"; "operator, auditor, release-owner, and runtime receipt views cannot be sent externally"),
    invariant("acceptance_record_receipt_requires_intake_gate"; "receipt preview requires acceptance record intake preview as its direct prior gate"),
    invariant("acceptance_record_receipt_preview_has_no_side_effects"; "this gate cannot persist receipts, record approval, grant authority, enable live execution, publish, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_acceptance_record_receipt_preview_gate",
      schema_version: "work_graph_persistence_acceptance_record_receipt_preview_v1",
      preview_mode: "read_only_persistence_acceptance_record_receipt_preview_no_receipt_write",
      receipt_contract_count: ($receipt_contracts | length),
      digest_check_count: ($digest_checks | length),
      receipt_denial_count: ($receipt_denials | length),
      readback_view_count: ($readback_views | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      receipt_contracts: $receipt_contracts,
      digest_checks: $digest_checks,
      receipt_denials: $receipt_denials,
      readback_views: $readback_views,
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
      recommended_next_gate: "hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_gate",
      ready_for_acceptance_record_receipt_acknowledgement_preview: true,
      ready_for_operator_acceptance: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_acceptance_record_receipt: {
          rust_module_present: $receipt_rust_module_present,
          report_script_present: $receipt_report_script_present,
          gate_script_present: $receipt_gate_script_present
        },
        persistence_acceptance_record_intake: {
          rust_module_present: $intake_rust_module_present,
          gate_script_present: $intake_gate_script_present
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
        acceptance_record_persisted: false,
        acceptance_record_receipt_persisted: false,
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
