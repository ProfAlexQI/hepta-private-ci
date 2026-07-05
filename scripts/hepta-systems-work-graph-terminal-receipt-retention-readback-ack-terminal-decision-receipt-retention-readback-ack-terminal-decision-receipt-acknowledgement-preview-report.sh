#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

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

rust_module="codex-rs/hepta-runtime/src/wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_ack_preview.rs"
report_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-acknowledgement-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-acknowledgement-preview-gate.sh"
prior_report_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-preview-gate.sh"
durable_identity_report_script="scripts/hepta-systems-work-graph-durable-identity-preview-report.sh"

required_prior_gates="$("$ROOT/$prior_report_script" | jq -c '(.required_prior_gates | map(select(. != "hepta_work_graph_durable_identity_preview_gate"))) + [.gate, "hepta_work_graph_durable_identity_preview_gate"]')"
durable_identity_report="$("$ROOT/$durable_identity_report_script")"
rust_module_present="$(bool_for path_exists "$rust_module")"
report_script_present="$(bool_for path_exists "$report_script")"
gate_script_present="$(bool_for path_exists "$gate_script")"
prior_report_script_present="$(bool_for path_exists "$prior_report_script")"
prior_gate_script_present="$(bool_for path_exists "$prior_gate_script")"
durable_identity_report_script_present="$(bool_for path_exists "$durable_identity_report_script")"

jq -n \
  --argjson required_prior_gates "$required_prior_gates" \
  --argjson durable_identity_report "$durable_identity_report" \
  --argjson rust_module_present "$rust_module_present" \
  --argjson report_script_present "$report_script_present" \
  --argjson gate_script_present "$gate_script_present" \
  --argjson prior_report_script_present "$prior_report_script_present" \
  --argjson prior_gate_script_present "$prior_gate_script_present" \
  --argjson durable_identity_report_script_present "$durable_identity_report_script_present" \
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
  def with_durable_fields($fields): durable_fields + $fields;
  def receipt_ids: [
    "operator_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt",
    "release_owner_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt",
    "authority_denial_terminal_decision_receipt_retention_readback_ack_receipt",
    "rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt",
    "release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt",
    "external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt"
  ];
  def ack_ids: [
    "operator_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement",
    "release_owner_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement",
    "authority_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
    "rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
    "release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
    "external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement"
  ];
  def contract($id; $visibility): {
    id: $id,
    source_receipt_ids: receipt_ids,
    acknowledgement_visibility: $visibility,
    required_fields: with_durable_fields([
      "acknowledgementId",
      "sourceReceiptIds",
      "acknowledgementVisibility",
      "acknowledgementHash",
      "recordingAllowed",
      "nextGate"
    ]),
    acknowledgement_recording_allowed: false,
    acceptance_allowed: false,
    authority_grant_allowed: false,
    public_claim_enabled: false,
    external_delivery_enabled: false
  };
  def reason($id; $text): {
    id: $id,
    applies_to_acknowledgement_ids: ack_ids,
    reason: $text,
    blocks_acceptance: true,
    blocks_authority: true
  };
  def denial($id; $target; $text): {
    id: $id,
    applies_to_acknowledgement_ids: ack_ids,
    target_record: $target,
    reason: $text,
    blocks_acknowledgement_recording: true,
    blocks_acceptance: true,
    blocks_authority: true,
    blocks_release_publication: true,
    blocks_public_claim: true,
    blocks_external_delivery: true
  };
  def guard($id; $fields): {
    id: $id,
    applies_to_acknowledgement_ids: ack_ids,
    required_fields: $fields,
    blocks_acknowledgement_recording: true
  };
  def view($id; $audience; $fields): {
    id: $id,
    audience: $audience,
    required_fields: $fields,
    external_delivery_enabled: false
  };
  def invariant($id; $text): {
    id: $id,
    required: true,
    reason: $text
  };
  def durable_identity_evidence: {
    schema_version: $durable_identity_report.schema_version,
    required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
    required_field_ids: durable_fields,
    required_for_acknowledgement_ids: ack_ids,
    durable_field_count: $durable_identity_report.durable_field_count,
    preview_binding_count: $durable_identity_report.preview_binding_count,
    invariant_count: $durable_identity_report.invariant_count,
    currently_satisfied: false
  };
  [
    contract("operator_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement"; "local_operator_receipt_acknowledgement_visibility"),
    contract("release_owner_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement"; "local_release_owner_receipt_acknowledgement_visibility"),
    contract("authority_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement"; "local_authority_denial_receipt_acknowledgement_visibility"),
    contract("rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement"; "local_rollout_denial_receipt_acknowledgement_visibility"),
    contract("release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement"; "local_release_publication_denial_receipt_acknowledgement_visibility"),
    contract("external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement"; "local_external_delivery_denial_receipt_acknowledgement_visibility")
  ] as $acknowledgement_contracts
  | [
    reason("durable_identity_evidence_missing"; "terminal decision receipt acknowledgement cannot proceed without durable identity evidence"),
    reason("terminal_decision_receipt_acknowledgement_is_not_acceptance"; "terminal decision receipt acknowledgement is local visibility, not acceptance"),
    reason("terminal_decision_receipt_acknowledgement_cannot_record_receipt"; "terminal decision receipt acknowledgement cannot record the underlying receipt"),
    reason("terminal_decision_receipt_acknowledgement_cannot_record_approval"; "terminal decision receipt acknowledgement cannot record approval"),
    reason("terminal_decision_receipt_acknowledgement_cannot_grant_authority"; "terminal decision receipt acknowledgement cannot grant authority"),
    reason("terminal_decision_receipt_acknowledgement_cannot_enable_live_persistence"; "terminal decision receipt acknowledgement cannot enable live persistence, WAL, or checkpoints"),
    reason("terminal_decision_receipt_acknowledgement_cannot_start_rollout"; "terminal decision receipt acknowledgement cannot start rollout or traffic routing"),
    reason("terminal_decision_receipt_acknowledgement_cannot_publish_or_send"; "terminal decision receipt acknowledgement cannot publish release state, record public claims, or send externally")
  ] as $non_acceptance_reasons
  | [
    denial("deny_durable_identity_terminal_receipt_retention_readback_ack_terminal_decision_receipt_ack_recording"; "terminal_decision_receipt_acknowledgement_store"; "terminal decision receipt acknowledgement recording is blocked without durable identity evidence"),
    denial("terminal_decision_receipt_acknowledgement_recording_denied"; "acknowledgement_record"; "acknowledgement recording is disabled"),
    denial("terminal_decision_receipt_recording_after_acknowledgement_denied"; "terminal_decision_receipt_record"; "receipt recording remains disabled after acknowledgement visibility"),
    denial("operator_acceptance_recording_after_terminal_decision_receipt_ack_denied"; "operator_acceptance_record"; "operator acceptance cannot be recorded from acknowledgement visibility"),
    denial("approval_ledger_recording_after_terminal_decision_receipt_ack_denied"; "approval_ledger_record"; "approval ledger recording is denied"),
    denial("authority_grant_recording_after_terminal_decision_receipt_ack_denied"; "authority_grant_record"; "authority grant recording is denied"),
    denial("release_public_claim_recording_after_terminal_decision_receipt_ack_denied"; "release_public_claim_record"; "release publication and public claim recording are denied"),
    denial("external_delivery_recording_after_terminal_decision_receipt_ack_denied"; "external_delivery_record"; "external delivery recording and send are denied")
  ] as $recording_denials
  | [
    guard("terminal_decision_receipt_expired_before_acknowledgement"; with_durable_fields(["receiptId", "expiresAt", "observedAt"])),
    guard("terminal_decision_receipt_scope_superseded_before_acknowledgement"; with_durable_fields(["receiptScope", "scopeEpoch", "supersessionHash"])),
    guard("terminal_decision_receipt_digest_mismatch_before_acknowledgement"; with_durable_fields(["receiptHash", "acknowledgementHash", "priorGateDigest"])),
    guard("terminal_decision_receipt_acknowledgement_replay_detected"; with_durable_fields(["acknowledgementId", "idempotencyKey", "replayHash"])),
    guard("terminal_decision_receipt_cross_scope_acknowledgement_detected"; with_durable_fields(["receiptScope", "acknowledgementScope", "scopeBindingHash"]))
  ] as $expiry_replay_guards
  | [
    view("operator_terminal_decision_receipt_acknowledgement_view"; "operator"; with_durable_fields(["acknowledgementId", "receiptHash", "acceptanceAllowed", "nextGate"])),
    view("release_owner_terminal_decision_receipt_acknowledgement_view"; "release_owner"; with_durable_fields(["releasePublished", "publicClaimRecorded", "authorityGranted", "externalDeliveryDenied"])),
    view("auditor_terminal_decision_receipt_acknowledgement_digest_view"; "auditor"; with_durable_fields(["receiptHash", "acknowledgementHash", "priorGateDigest", "recordingDenialId"])),
    view("runtime_terminal_decision_receipt_acknowledgement_zero_effect_view"; "system"; with_durable_fields(["acknowledgementRecorded", "authorityGranted", "trafficRouted", "externalSendPerformed"]))
  ] as $local_views
  | [
    invariant("terminal_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgements_require_durable_identity_evidence"; "terminal decision receipt acknowledgements require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("terminal_decision_receipt_acknowledgement_is_hash_only"; "acknowledgement visibility exposes local hashes only"),
    invariant("terminal_decision_receipt_acknowledgement_is_not_acceptance"; "acknowledgement does not become acceptance, approval, or authority"),
    invariant("terminal_decision_receipt_acknowledgement_is_not_recorded"; "acknowledgement and receipt recording remain disabled"),
    invariant("terminal_decision_receipt_acknowledgement_views_are_local_only"; "operator, release-owner, auditor, and runtime views stay local only"),
    invariant("terminal_decision_receipt_acknowledgement_requires_receipt_gate"; "acknowledgement preview requires the terminal decision receipt gate"),
    invariant("terminal_decision_receipt_acknowledgement_preview_has_no_side_effects"; "this gate cannot persist, write WAL/checkpoints, start rollout, publish, record public claims, or send externally")
  ] as $invariants
  | {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate",
    schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_v1",
    preview_mode: "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_no_recording",
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
    durable_identity_evidence: durable_identity_evidence,
    invariants: $invariants,
    recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate",
    ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview: true,
    ready_for_operator_acceptance: false,
    ready_for_live_persistence: false,
    side_effects: {
      filesystem_written: false,
      graph_state_persisted: false,
      terminal_decision_recorded: false,
      terminal_decision_receipt_recorded: false,
      terminal_decision_receipt_persisted: false,
      terminal_decision_receipt_acknowledgement_recorded: false,
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
    },
    source_probes: {
      terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      },
      durable_identity: {
        report_script_present: $durable_identity_report_script_present
      }
    }
  }
'
