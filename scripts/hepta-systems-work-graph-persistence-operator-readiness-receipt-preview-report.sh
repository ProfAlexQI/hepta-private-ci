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

readiness_receipt_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_operator_readiness_receipt_preview.rs
)"
readiness_receipt_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-operator-readiness-receipt-preview-report.sh
)"
readiness_receipt_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-operator-readiness-receipt-preview-gate.sh
)"
readiness_packet_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_operator_readiness_packet_preview.rs
)"
readiness_packet_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-operator-readiness-packet-preview-gate.sh
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
  --argjson readiness_receipt_rust_module_present "$readiness_receipt_rust_module_present" \
  --argjson readiness_receipt_report_script_present "$readiness_receipt_report_script_present" \
  --argjson readiness_receipt_gate_script_present "$readiness_receipt_gate_script_present" \
  --argjson readiness_packet_rust_module_present "$readiness_packet_rust_module_present" \
  --argjson readiness_packet_gate_script_present "$readiness_packet_gate_script_present" \
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
    "hepta_work_graph_durable_identity_preview_gate"
  ];
  def receipt_ids: [
    "store_persistence_readiness_receipt",
    "wal_checkpoint_readiness_receipt",
    "readback_receipt_readiness_receipt",
    "replay_execution_readiness_receipt",
    "external_publication_readiness_receipt",
    "full_rollout_abort_readiness_receipt"
  ];
  def receipt($id; $packet): {
    id: $id,
    source_packet_template_id: $packet,
    required_fields: (durable_fields + ["receiptId", "packetTemplateId", "packetDigestHash", "operatorScopeHash", "readinessSectionsHash", "signatureState", "expiryState", "releasePublicationDenied", "approvalRecordingDenied", "externalDeliveryDenied"]),
    redaction_state: "redacted_hash_only",
    persistence_enabled: false,
    approval_recording_enabled: false,
    external_delivery_enabled: false
  };
  def digest($id; $fields): {
    id: $id,
    compared_fields: $fields,
    blocks_acceptance: true
  };
  def signature_denial($id; $trigger): {
    id: $id,
    applies_to_receipt_ids: receipt_ids,
    trigger: $trigger,
    blocks_receipt_acceptance: true
  };
  def acceptance_denial($id; $reason): {
    id: $id,
    applies_to_receipt_ids: receipt_ids,
    reason: $reason,
    blocks_promotion: true
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
    receipt("store_persistence_readiness_receipt"; "store_persistence_readiness_packet"),
    receipt("wal_checkpoint_readiness_receipt"; "wal_checkpoint_readiness_packet"),
    receipt("readback_receipt_readiness_receipt"; "readback_receipt_readiness_packet"),
    receipt("replay_execution_readiness_receipt"; "replay_execution_readiness_packet"),
    receipt("external_publication_readiness_receipt"; "external_publication_readiness_packet"),
    receipt("full_rollout_abort_readiness_receipt"; "full_rollout_abort_readiness_packet")
  ] as $receipt_contracts
  | [
    digest("check_durable_identity_digest"; durable_fields),
    digest("check_packet_template_digest"; ["packetTemplateId", "packetDigestHash", "requiredSectionIds"]),
    digest("check_section_completion_digest"; ["readinessSectionsHash", "validationDenialIds", "completeSectionIds"]),
    digest("check_operator_scope_digest"; ["operatorScopeHash", "operatorIdHash", "signatureState"]),
    digest("check_expiry_revocation_digest"; ["expiresAtUnixMs", "revocationStatus", "supersessionId"]),
    digest("check_release_publication_denial_digest"; ["releaseDenialIds", "publicationDenied", "externalDeliveryDenied"]),
    digest("check_side_effect_denial_digest"; ["approvalRecordingDenied", "receiptPersistenceDenied", "sideEffectHash"])
  ] as $digest_checks
  | [
    signature_denial("durable_identity_evidence_missing"; "receipt does not include durable identity evidence"),
    signature_denial("missing_signature_hash"; "receipt does not include operator signature hash"),
    signature_denial("invalid_operator_scope_signature"; "operator signature does not match the required scope hash"),
    signature_denial("packet_expired"; "readiness packet expires before receipt readback"),
    signature_denial("packet_superseded"; "newer readiness packet supersedes this receipt"),
    signature_denial("operator_scope_revoked"; "operator authority scope was revoked"),
    signature_denial("rollback_owner_revoked"; "rollback or quarantine owner was revoked")
  ] as $signature_denials
  | [
    acceptance_denial("durable_identity_evidence_missing"; "preview receipt attempted acceptance without durable identity evidence"),
    acceptance_denial("approval_recording_attempted"; "preview receipt attempted to record approval"),
    acceptance_denial("release_publication_attempted"; "preview receipt attempted release or publication"),
    acceptance_denial("external_delivery_attempted"; "preview receipt attempted external delivery"),
    acceptance_denial("receipt_persistence_attempted"; "preview receipt attempted durable receipt persistence"),
    acceptance_denial("enforcement_rollout_attempted"; "preview receipt attempted enforcement rollout"),
    acceptance_denial("live_readback_attempted"; "preview receipt attempted live readback"),
    acceptance_denial("readiness_receipt_not_hash_only"; "readiness receipt contains payload material instead of hashes")
  ] as $acceptance_denials
  | [
    view("operator_readiness_receipt_summary_view"; "operator"; durable_fields + ["receiptId", "packetTemplateId", "signatureState", "acceptanceDenied"]),
    view("auditor_readiness_receipt_digest_view"; "auditor"; durable_fields + ["packetDigestHash", "readinessSectionsHash", "sideEffectHash", "redactionState"]),
    view("rollback_owner_revocation_view"; "rollback_owner"; durable_fields + ["rollbackOwnerId", "revocationStatus", "quarantineScope", "killSwitchId"]),
    view("release_publication_denial_view"; "release_owner"; durable_fields + ["releaseDenialIds", "publicationDenied", "externalDeliveryDenied", "nextGate"])
  ] as $readback_views
  | [
    invariant("operator_readiness_receipts_require_durable_identity_evidence"; "readiness receipt contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("readiness_receipts_are_hash_only"; "readiness receipts expose packet hashes, receipt hashes, denial ids, and redaction state only"),
    invariant("signature_denials_block_acceptance"; "missing, invalid, expired, superseded, or revoked signature state blocks acceptance"),
    invariant("receipt_readback_is_non_persistent"; "receipt readback views are local preview shapes and cannot persist receipt state"),
    invariant("approval_recording_is_denied"; "readiness receipt preview cannot record operator acceptance or approval"),
    invariant("release_publication_and_external_delivery_are_denied"; "readiness receipt preview cannot release, publish, or send externally"),
    invariant("operator_readiness_receipt_preview_has_no_side_effects"; "this gate cannot persist receipts, record approvals, execute readback, enable enforcement, route traffic, publish releases, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_operator_readiness_receipt_preview_gate",
      schema_version: "work_graph_persistence_operator_readiness_receipt_preview_v1",
      preview_mode: "read_only_persistence_operator_readiness_receipt_preview_no_receipt_write",
      receipt_contract_count: ($receipt_contracts | length),
      digest_check_count: ($digest_checks | length),
      signature_denial_count: ($signature_denials | length),
      acceptance_denial_count: ($acceptance_denials | length),
      readback_view_count: ($readback_views | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      receipt_contracts: $receipt_contracts,
      digest_checks: $digest_checks,
      signature_denials: $signature_denials,
      acceptance_denials: $acceptance_denials,
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
      recommended_next_gate: "hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_gate",
      ready_for_operator_readiness_receipt_acknowledgement_preview: true,
      ready_for_operator_acceptance: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_operator_readiness_receipt: {
          rust_module_present: $readiness_receipt_rust_module_present,
          report_script_present: $readiness_receipt_report_script_present,
          gate_script_present: $readiness_receipt_gate_script_present
        },
        persistence_operator_readiness_packet: {
          rust_module_present: $readiness_packet_rust_module_present,
          gate_script_present: $readiness_packet_gate_script_present
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
        readiness_receipt_persisted: false,
        operator_acceptance_recorded: false,
        approval_recorded: false,
        live_readback_executed: false,
        enforcement_enabled: false,
        rollout_started: false,
        traffic_routed: false,
        release_published: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
