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

ack_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview.rs
)"
ack_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-acknowledgement-preview-report.sh
)"
ack_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-acknowledgement-preview-gate.sh
)"
receipt_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_preview.rs
)"
receipt_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-preview-gate.sh
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
  --argjson ack_rust_module_present "$ack_rust_module_present" \
  --argjson ack_report_script_present "$ack_report_script_present" \
  --argjson ack_gate_script_present "$ack_gate_script_present" \
  --argjson receipt_rust_module_present "$receipt_rust_module_present" \
  --argjson receipt_gate_script_present "$receipt_gate_script_present" \
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
  def ack_ids: [
    "operator_acceptance_recording_denial_receipt_acknowledgement",
    "approval_ledger_write_denial_receipt_acknowledgement",
    "authority_grant_denial_receipt_acknowledgement",
    "graph_state_persistence_denial_receipt_acknowledgement",
    "wal_checkpoint_write_denial_receipt_acknowledgement",
    "enforcement_rollout_denial_receipt_acknowledgement",
    "release_publication_denial_receipt_acknowledgement",
    "external_delivery_denial_receipt_acknowledgement"
  ];
  def ack($id; $source): {
    id: $id,
    source_denial_receipt_id: $source,
    required_fields: (durable_fields + ["acknowledgementId", "sourceDenialReceiptId", "denialReceiptHash", "receiptScope", "acknowledgementHash", "accepted", "recordingEnabled", "nextGate"]),
    acceptance_allowed: false,
    acknowledgement_recording_enabled: false,
    authority_grant_enabled: false,
    external_delivery_enabled: false
  };
  def reason($id; $text): {
    id: $id,
    applies_to_acknowledgement_ids: ack_ids,
    reason: $text,
    blocks_acceptance: true
  };
  def denial($id; $target; $text): {
    id: $id,
    target_record: $target,
    reason: $text,
    blocks_recording: true
  };
  def guard($id; $trigger): {
    id: $id,
    applies_to_acknowledgement_ids: ack_ids,
    trigger: $trigger,
    blocks_acknowledgement: true
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
    ack("operator_acceptance_recording_denial_receipt_acknowledgement"; "operator_acceptance_recording_denial_receipt"),
    ack("approval_ledger_write_denial_receipt_acknowledgement"; "approval_ledger_write_denial_receipt"),
    ack("authority_grant_denial_receipt_acknowledgement"; "authority_grant_denial_receipt"),
    ack("graph_state_persistence_denial_receipt_acknowledgement"; "graph_state_persistence_denial_receipt"),
    ack("wal_checkpoint_write_denial_receipt_acknowledgement"; "wal_checkpoint_write_denial_receipt"),
    ack("enforcement_rollout_denial_receipt_acknowledgement"; "enforcement_rollout_denial_receipt"),
    ack("release_publication_denial_receipt_acknowledgement"; "release_publication_denial_receipt"),
    ack("external_delivery_denial_receipt_acknowledgement"; "external_delivery_denial_receipt")
  ] as $acknowledgement_contracts
  | [
    reason("durable_identity_evidence_missing"; "denial receipt acknowledgement does not include durable identity evidence"),
    reason("acknowledgement_is_not_effect_acceptance"; "denial receipt acknowledgement only confirms local preview visibility"),
    reason("acknowledgement_cannot_record_approval"; "denial receipt acknowledgement cannot record approval or acceptance"),
    reason("acknowledgement_cannot_grant_authority"; "denial receipt acknowledgement cannot grant WorkGraph authority"),
    reason("acknowledgement_cannot_enable_persistence_or_wal"; "denial receipt acknowledgement cannot enable persistence, WAL, or checkpoints"),
    reason("acknowledgement_cannot_start_rollout"; "denial receipt acknowledgement cannot start enforcement rollout or route traffic"),
    reason("acknowledgement_cannot_publish_release"; "denial receipt acknowledgement cannot publish release status or artifacts"),
    reason("acknowledgement_cannot_send_external_delivery"; "denial receipt acknowledgement cannot send external delivery")
  ] as $non_acceptance_reasons
  | [
    denial("deny_durable_identity_ack_recording"; "durable_identity_acknowledgement_evidence"; "acknowledgement recording is blocked without durable identity evidence"),
    denial("denial_receipt_acknowledgement_recording_denied"; "effect_denial_receipt_acknowledgement_store"; "acknowledgement recording is disabled in preview"),
    denial("operator_acceptance_recording_denied"; "operator_acceptance_record"; "denial receipt acknowledgement is not operator acceptance"),
    denial("approval_ledger_recording_denied"; "approval_ledger"; "denial receipt acknowledgement cannot write approval ledger entries"),
    denial("authority_grant_recording_denied"; "authority_grant_record"; "denial receipt acknowledgement cannot grant authority"),
    denial("graph_state_persistence_denied"; "work_graph_state_store"; "denial receipt acknowledgement cannot persist graph state"),
    denial("release_publication_recording_denied"; "release_publication_record"; "denial receipt acknowledgement cannot publish release state"),
    denial("external_delivery_recording_denied"; "external_delivery_record"; "denial receipt acknowledgement cannot create external delivery records")
  ] as $recording_denials
  | [
    guard("denial_receipt_expired"; "denial receipt age exceeds local preview window"),
    guard("denial_receipt_scope_superseded"; "denial receipt scope was superseded by a newer blocker report"),
    guard("denial_receipt_digest_mismatch"; "denial receipt digest does not match the local readback digest"),
    guard("source_blocker_gate_superseded"; "effect application blocker gate digest changed after receipt creation"),
    guard("acknowledgement_replay_detected"; "acknowledgement idempotency key has already been observed")
  ] as $expiry_guards
  | [
    view("operator_effect_denial_receipt_acknowledgement_view"; "operator"; durable_fields + ["acknowledgementId", "sourceDenialReceiptId", "accepted", "nextGate"]),
    view("auditor_effect_denial_receipt_acknowledgement_view"; "auditor"; durable_fields + ["acknowledgementHash", "sourceDenialReceiptHash", "scopeDigest", "zeroEffectHash"]),
    view("release_owner_effect_denial_receipt_acknowledgement_view"; "release_owner"; durable_fields + ["releaseDenied", "publicationDenied", "externalDeliveryDenied", "acknowledgementId"]),
    view("runtime_effect_denial_receipt_acknowledgement_zero_effect_view"; "system"; durable_fields + ["acknowledgementRecorded", "authorityGranted", "statePersisted", "trafficRouted", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("effect_denial_receipt_acknowledgements_require_durable_identity_evidence"; "effect denial receipt acknowledgement contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("effect_denial_receipt_acknowledgements_are_hash_only"; "acknowledgements expose only local hash-only receipt references"),
    invariant("effect_denial_receipt_acknowledgements_are_non_accepting"; "acknowledgement visibility cannot become effect acceptance"),
    invariant("effect_denial_receipt_acknowledgements_are_non_recording"; "acknowledgement preview cannot record approval, acceptance, authority, or receipt state"),
    invariant("effect_denial_receipt_acknowledgement_views_are_local_only"; "operator, auditor, release-owner, and runtime views cannot be sent externally"),
    invariant("effect_denial_receipt_acknowledgement_requires_denial_receipt_gate"; "acknowledgement preview requires hash-only denial receipt evidence first"),
    invariant("effect_denial_receipt_acknowledgement_preview_has_no_side_effects"; "this gate cannot persist, grant authority, enable live execution, publish, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_gate",
      schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_v1",
      preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_no_recording",
      acknowledgement_contract_count: ($acknowledgement_contracts | length),
      non_acceptance_reason_count: ($non_acceptance_reasons | length),
      recording_denial_count: ($recording_denials | length),
      expiry_guard_count: ($expiry_guards | length),
      local_view_count: ($local_views | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      acknowledgement_contracts: $acknowledgement_contracts,
      non_acceptance_reasons: $non_acceptance_reasons,
      recording_denials: $recording_denials,
      expiry_guards: $expiry_guards,
      local_views: $local_views,
      durable_identity_evidence: {
        schema_version: $durable_identity_report.schema_version,
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: durable_fields,
        required_for_acknowledgement_ids: ack_ids,
        durable_field_count: $durable_identity_report.durable_field_count,
        preview_binding_count: $durable_identity_report.preview_binding_count,
        invariant_count: $durable_identity_report.invariant_count,
        currently_satisfied: false
      },
      invariants: $invariants,
      recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_replay_idempotency_preview_gate",
      ready_for_acceptance_effect_application_denial_receipt_replay_idempotency_preview: true,
      ready_for_operator_acceptance: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_acceptance_effect_application_denial_receipt_acknowledgement: {
          rust_module_present: $ack_rust_module_present,
          report_script_present: $ack_report_script_present,
          gate_script_present: $ack_gate_script_present
        },
        persistence_acceptance_effect_application_denial_receipt: {
          rust_module_present: $receipt_rust_module_present,
          gate_script_present: $receipt_gate_script_present
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
        denial_receipt_acknowledgement_recorded: false,
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
