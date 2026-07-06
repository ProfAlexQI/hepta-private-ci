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
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview.rs
)"
ack_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-preview-report.sh
)"
ack_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-preview-gate.sh
)"
readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview.rs
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-receipt-preview-gate.sh
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

prior_report_script="$ROOT/scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-receipt-preview-report.sh"
required_prior_gates="$(
  "$prior_report_script" |
    jq -c '.required_prior_gates
      | map(select(. != "hepta_work_graph_durable_identity_preview_gate"))
      + [
          "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate",
          "hepta_work_graph_durable_identity_preview_gate"
        ]'
)"

jq -n \
  --argjson ack_rust_module_present "$ack_rust_module_present" \
  --argjson ack_report_script_present "$ack_report_script_present" \
  --argjson ack_gate_script_present "$ack_gate_script_present" \
  --argjson readback_rust_module_present "$readback_rust_module_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  --argjson durable_identity_rust_module_present "$durable_identity_rust_module_present" \
  --argjson durable_identity_report_script_present "$durable_identity_report_script_present" \
  --argjson durable_identity_gate_script_present "$durable_identity_gate_script_present" \
  --argjson durable_identity_report "$durable_identity_report" \
  --argjson required_prior_gates "$required_prior_gates" \
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
  def ack_ids: [
    "terminal_receipt_retention_policy_readback_acknowledgement",
    "terminal_receipt_expiry_guard_readback_acknowledgement",
    "terminal_receipt_supersession_guard_readback_acknowledgement",
    "terminal_receipt_gc_denial_readback_acknowledgement",
    "terminal_receipt_zero_effect_digest_readback_acknowledgement",
    "terminal_receipt_release_public_claim_denial_readback_acknowledgement"
  ];
  def ack($id; $source): {
    id: $id,
    source_readback_receipt_id: $source,
    required_fields: (durable_fields + ["acknowledgementId", "sourceReadbackReceiptId", "readbackReceiptHash", "retentionScope", "acknowledgementHash", "accepted", "recordingEnabled", "nextGate"]),
    acceptance_allowed: false,
    acknowledgement_recording_enabled: false,
    receipt_recording_enabled: false,
    authority_grant_enabled: false,
    public_claim_enabled: false,
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
    ack("terminal_receipt_retention_policy_readback_acknowledgement"; "terminal_receipt_retention_policy_readback_receipt"),
    ack("terminal_receipt_expiry_guard_readback_acknowledgement"; "terminal_receipt_expiry_guard_readback_receipt"),
    ack("terminal_receipt_supersession_guard_readback_acknowledgement"; "terminal_receipt_supersession_guard_readback_receipt"),
    ack("terminal_receipt_gc_denial_readback_acknowledgement"; "terminal_receipt_gc_denial_readback_receipt"),
    ack("terminal_receipt_zero_effect_digest_readback_acknowledgement"; "terminal_receipt_zero_effect_digest_readback_receipt"),
    ack("terminal_receipt_release_public_claim_denial_readback_acknowledgement"; "terminal_receipt_release_public_claim_denial_readback_receipt")
  ] as $acknowledgement_contracts
  | [
    reason("durable_identity_evidence_missing"; "terminal receipt retention readback acknowledgement does not include durable identity evidence"),
    reason("terminal_retention_readback_ack_is_not_acceptance"; "terminal retention readback acknowledgement only confirms local preview visibility"),
    reason("terminal_retention_readback_ack_cannot_record_acknowledgement"; "terminal retention readback acknowledgement cannot record acknowledgement state"),
    reason("terminal_retention_readback_ack_cannot_record_approval"; "terminal retention readback acknowledgement cannot record approval or acceptance"),
    reason("terminal_retention_readback_ack_cannot_grant_authority"; "terminal retention readback acknowledgement cannot grant WorkGraph authority"),
    reason("terminal_retention_readback_ack_cannot_enable_persistence"; "terminal retention readback acknowledgement cannot enable live persistence, WAL, or checkpoints"),
    reason("terminal_retention_readback_ack_cannot_start_rollout"; "terminal retention readback acknowledgement cannot start rollout or route traffic"),
    reason("terminal_retention_readback_ack_cannot_publish_or_send"; "terminal retention readback acknowledgement cannot publish release state, record public claims, or send externally")
  ] as $non_acceptance_reasons
  | [
    denial("deny_durable_identity_terminal_receipt_retention_readback_ack_recording"; "durable_identity_terminal_receipt_retention_readback_acknowledgement_evidence"; "terminal receipt retention readback acknowledgement recording is blocked without durable identity evidence"),
    denial("terminal_retention_readback_ack_recording_denied"; "terminal_retention_readback_acknowledgement_store"; "terminal receipt retention readback acknowledgement recording is disabled in preview"),
    denial("terminal_receipt_retention_state_recording_denied"; "terminal_receipt_retention_state_store"; "readback acknowledgement cannot persist terminal receipt retention state"),
    denial("terminal_retention_readback_receipt_recording_denied"; "terminal_receipt_retention_readback_receipt_store"; "readback acknowledgement cannot persist terminal receipt readback state"),
    denial("terminal_operator_acceptance_recording_denied"; "operator_acceptance_record"; "terminal retention readback acknowledgement is not operator acceptance"),
    denial("terminal_approval_ledger_recording_denied"; "approval_ledger"; "terminal retention readback acknowledgement cannot write approval ledger entries"),
    denial("terminal_authority_grant_recording_denied"; "authority_grant_record"; "terminal retention readback acknowledgement cannot grant authority"),
    denial("terminal_release_public_claim_delivery_recording_denied"; "release_public_claim_external_delivery_record"; "terminal retention readback acknowledgement cannot publish release state, record public claims, or create delivery records")
  ] as $recording_denials
  | [
    guard("terminal_retention_readback_receipt_expired"; "terminal retention readback receipt exceeded the local preview window"),
    guard("terminal_retention_readback_receipt_scope_superseded"; "terminal retention readback receipt scope was superseded by a newer blocker report"),
    guard("terminal_retention_readback_receipt_digest_mismatch"; "terminal retention readback receipt digest does not match local evidence"),
    guard("terminal_retention_gc_denial_receipt_replayed"; "terminal garbage-collection denial readback receipt replay was observed"),
    guard("terminal_retention_readback_ack_replay_detected"; "terminal retention readback acknowledgement idempotency key has already been observed")
  ] as $expiry_replay_guards
  | [
    view("operator_terminal_retention_readback_acknowledgement_view"; "operator"; durable_fields + ["acknowledgementId", "sourceReadbackReceiptId", "accepted", "nextGate"]),
    view("auditor_terminal_retention_readback_acknowledgement_view"; "auditor"; durable_fields + ["acknowledgementHash", "sourceReadbackReceiptHash", "scopeDigest", "zeroEffectHash"]),
    view("release_owner_terminal_retention_readback_acknowledgement_view"; "release_owner"; durable_fields + ["releaseDenied", "publicationDenied", "publicClaimDenied", "externalDeliveryDenied"]),
    view("runtime_terminal_retention_readback_ack_zero_effect_view"; "system"; durable_fields + ["acknowledgementRecorded", "retentionStatePersisted", "authorityGranted", "publicClaimRecorded", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("terminal_receipt_retention_readback_acknowledgements_require_durable_identity_evidence"; "terminal receipt retention readback acknowledgement contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("terminal_retention_readback_acknowledgements_are_hash_only"; "acknowledgements expose only local hash-only readback receipt references"),
    invariant("terminal_retention_readback_acknowledgements_are_non_accepting"; "terminal retention readback acknowledgement visibility cannot become acceptance"),
    invariant("terminal_retention_readback_acknowledgements_are_non_recording"; "acknowledgement preview cannot record receipt, approval, acceptance, authority, public claim, or retention state"),
    invariant("terminal_retention_readback_acknowledgement_views_are_local_only"; "operator, auditor, release-owner, and runtime views cannot be sent externally"),
    invariant("terminal_retention_readback_acknowledgement_requires_readback_receipt_gate"; "acknowledgement preview requires terminal receipt retention readback receipt evidence first"),
    invariant("terminal_retention_readback_acknowledgement_preview_has_no_side_effects"; "this gate cannot persist, grant authority, enable live execution, publish, record public claims, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate",
      schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_v1",
      preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_no_recording",
      acknowledgement_contract_count: ($acknowledgement_contracts | length),
      non_acceptance_reason_count: ($non_acceptance_reasons | length),
      recording_denial_count: ($recording_denials | length),
      expiry_replay_guard_count: ($expiry_replay_guards | length),
      local_view_count: ($local_views | length),
      invariant_count: ($invariants | length),
      required_prior_gates: $required_prior_gates,
      acknowledgement_contracts: $acknowledgement_contracts,
      non_acceptance_reasons: $non_acceptance_reasons,
      recording_denials: $recording_denials,
      expiry_replay_guards: $expiry_replay_guards,
      local_views: $local_views,
      durable_identity_evidence: {
        schema_version: $durable_identity_report.schema_version,
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: durable_fields,
        required_for_acknowledgement_ids: ack_ids,
        durable_field_count: (durable_fields | length),
        preview_binding_count: 5,
        invariant_count: ($invariants | length),
        currently_satisfied: false
      },
      invariants: $invariants,
      recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate",
      ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview: true,
      ready_for_operator_acceptance: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement: {
          rust_module_present: $ack_rust_module_present,
          report_script_present: $ack_report_script_present,
          gate_script_present: $ack_gate_script_present
        },
        persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt: {
          rust_module_present: $readback_rust_module_present,
          gate_script_present: $readback_gate_script_present
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
        terminal_decision_recorded: false,
        terminal_decision_receipt_recorded: false,
        terminal_receipt_retention_state_persisted: false,
        readback_receipt_persisted: false,
        readback_acknowledgement_recorded: false,
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
        public_claim_recorded: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
