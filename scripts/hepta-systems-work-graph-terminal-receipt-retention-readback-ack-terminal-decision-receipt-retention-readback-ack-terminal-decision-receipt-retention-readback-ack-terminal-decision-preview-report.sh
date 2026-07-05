#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

path_exists() { [[ -e "$1" ]]; }
bool_for() {
  if "$@"; then printf 'true\n'; else printf 'false\n'; fi
}

rust_module="codex-rs/hepta-runtime/src/wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_preview.rs"
report_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-preview-gate.sh"
prior_report_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview-gate.sh"

required_prior_gates="$("$ROOT/$prior_report_script" | jq -c '.required_prior_gates + [.gate]')"
rust_module_present="$(bool_for path_exists "$rust_module")"
report_script_present="$(bool_for path_exists "$report_script")"
gate_script_present="$(bool_for path_exists "$gate_script")"
prior_report_script_present="$(bool_for path_exists "$prior_report_script")"
prior_gate_script_present="$(bool_for path_exists "$prior_gate_script")"

jq -n \
  --argjson required_prior_gates "$required_prior_gates" \
  --argjson rust_module_present "$rust_module_present" \
  --argjson report_script_present "$report_script_present" \
  --argjson gate_script_present "$gate_script_present" \
  --argjson prior_report_script_present "$prior_report_script_present" \
  --argjson prior_gate_script_present "$prior_gate_script_present" \
  '
  def replay_ids: [
    "duplicate_terminal_decision_receipt_retention_readback_receipt_replay",
    "duplicate_terminal_decision_receipt_retention_readback_acknowledgement_replay",
    "stale_terminal_decision_receipt_retention_readback_digest_replay",
    "superseded_terminal_decision_receipt_retention_readback_scope_replay",
    "cross_scope_terminal_decision_receipt_retention_readback_acknowledgement_replay",
    "out_of_order_terminal_decision_receipt_retention_readback_acknowledgement_replay"
  ];
  def surface_ids: [
    "operator_terminal_decision_receipt_retention_readback_ack_decision_visibility",
    "release_owner_terminal_decision_receipt_retention_readback_ack_decision_visibility",
    "auditor_terminal_decision_receipt_retention_readback_ack_decision_visibility",
    "rollback_owner_terminal_decision_receipt_retention_readback_ack_decision_visibility",
    "runtime_terminal_decision_receipt_retention_readback_ack_summary_visibility",
    "external_delivery_terminal_decision_receipt_retention_readback_ack_echo"
  ];
  def surface($id; $audience; $visibility): {
    id: $id,
    audience: $audience,
    source_replay_scenario_ids: replay_ids,
    decision_visibility: $visibility,
    decision_recording_allowed: false,
    promotion_allowed: false,
    authority_grant_allowed: false,
    public_claim_enabled: false,
    external_delivery_enabled: false
  };
  def denial($id; $reason): {
    id: $id,
    applies_to_surface_ids: surface_ids,
    reason: $reason,
    blocks_persistence_promotion: true,
    blocks_authority_grant: true,
    blocks_rollout: true,
    blocks_release_publication: true,
    blocks_public_claim: true,
    blocks_external_delivery: true
  };
  def authority_guard($id; $fields): {
    id: $id,
    required_fields: $fields,
    authority_grant_allowed: false
  };
  def release_guard($id; $fields): {
    id: $id,
    required_fields: $fields,
    release_publication_allowed: false,
    public_claim_allowed: false,
    delivery_allowed: false
  };
  def view($id; $audience; $fields): {
    id: $id,
    audience: $audience,
    required_fields: $fields,
    external_delivery_enabled: false
  };
  def invariant($id; $reason): { id: $id, required: true, reason: $reason };
  def side_effects_false: {
    filesystem_written: false,
    graph_state_persisted: false,
    terminal_decision_recorded: false,
    terminal_decision_receipt_recorded: false,
    terminal_decision_receipt_retention_state_persisted: false,
    readback_receipt_persisted: false,
    readback_acknowledgement_recorded: false,
    readback_acknowledgement_replay_recorded: false,
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
  };
  [
    surface("operator_terminal_decision_receipt_retention_readback_ack_decision_visibility"; "operator"; "local_operator_terminal_decision_receipt_retention_readback_ack_decision_read_only"),
    surface("release_owner_terminal_decision_receipt_retention_readback_ack_decision_visibility"; "release_owner"; "local_release_owner_terminal_decision_receipt_retention_readback_ack_decision_read_only"),
    surface("auditor_terminal_decision_receipt_retention_readback_ack_decision_visibility"; "auditor"; "local_auditor_terminal_decision_receipt_retention_readback_ack_decision_read_only"),
    surface("rollback_owner_terminal_decision_receipt_retention_readback_ack_decision_visibility"; "rollback_owner"; "local_rollback_owner_terminal_decision_receipt_retention_readback_ack_decision_read_only"),
    surface("runtime_terminal_decision_receipt_retention_readback_ack_summary_visibility"; "system"; "local_runtime_terminal_decision_receipt_retention_readback_ack_summary_read_only"),
    surface("external_delivery_terminal_decision_receipt_retention_readback_ack_echo"; "external_delivery"; "external_delivery_echo_denied")
  ] as $terminal_decision_surfaces
  | [
    denial("terminal_decision_receipt_retention_readback_ack_decision_cannot_promote_persistence"; "terminal decision visibility after terminal decision receipt retention readback acknowledgement replay cannot promote persistence, WAL, or checkpoints"),
    denial("terminal_decision_receipt_retention_readback_ack_decision_cannot_grant_authority"; "terminal decision visibility after terminal decision receipt retention readback acknowledgement replay cannot grant acceptance or approval authority"),
    denial("release_owner_terminal_decision_receipt_retention_readback_ack_cannot_publish"; "release-owner terminal decision visibility cannot publish release state or public claims"),
    denial("operator_terminal_decision_receipt_retention_readback_ack_cannot_start_rollout"; "operator terminal decision visibility cannot start rollout or route traffic"),
    denial("auditor_terminal_decision_receipt_retention_readback_ack_cannot_record_approval"; "auditor terminal decision visibility cannot record approval"),
    denial("rollback_owner_terminal_decision_receipt_retention_readback_ack_cannot_mutate_quarantine"; "rollback-owner terminal decision visibility cannot mutate rollback or quarantine state"),
    denial("external_delivery_terminal_decision_receipt_retention_readback_ack_cannot_send"; "external delivery echo of terminal decision receipt retention readback acknowledgement decision must stay denied"),
    denial("terminal_decision_receipt_retention_readback_ack_summary_cannot_claim_live_completion"; "terminal decision summary cannot claim live persistence completion")
  ] as $non_promotion_denials
  | [
    authority_guard("accepted_record_authority_absent_after_terminal_decision_receipt_retention_readback_ack"; ["acceptedRecordId", "acceptedRecordHash", "authoritySignature"]),
    authority_guard("approval_recording_authority_absent_after_terminal_decision_receipt_retention_readback_ack"; ["approvalRecordId", "approvalScope", "approvalSignature"]),
    authority_guard("operator_enablement_authority_absent_after_terminal_decision_receipt_retention_readback_ack"; ["operatorEnablementPacketId", "enablementScope", "enablementSignature"]),
    authority_guard("release_owner_authority_absent_after_terminal_decision_receipt_retention_readback_ack"; ["releaseOwnerId", "releaseScope", "releaseSignature"]),
    authority_guard("live_persistence_authority_absent_after_terminal_decision_receipt_retention_readback_ack"; ["livePersistenceFlag", "walAuthorityHash", "checkpointAuthorityHash"]),
    authority_guard("external_delivery_authority_absent_after_terminal_decision_receipt_retention_readback_ack"; ["externalDeliveryPolicyId", "deliveryScope", "deliverySignature"])
  ] as $authority_guards
  | [
    release_guard("release_publication_gate_denied_after_terminal_decision_receipt_retention_readback_ack"; ["releasePublicationRequest", "releaseOwnerSignature", "releaseHash"]),
    release_guard("public_claim_gate_denied_after_terminal_decision_receipt_retention_readback_ack"; ["publicClaimRequest", "claimAudience", "claimHash"]),
    release_guard("traffic_ramp_gate_zero_after_terminal_decision_receipt_retention_readback_ack"; ["trafficRampRequest", "trafficPercent", "killSwitchState"]),
    release_guard("external_delivery_gate_denied_after_terminal_decision_receipt_retention_readback_ack"; ["externalDeliveryRequest", "destinationPolicy", "deliveryHash"]),
    release_guard("artifact_publication_gate_denied_after_terminal_decision_receipt_retention_readback_ack"; ["artifactPublicationRequest", "artifactHash", "redactionHash"]),
    release_guard("terminal_decision_receipt_retention_readback_ack_completion_claim_local_only"; ["terminalDecisionId", "completionClaimHash", "localViewHash"])
  ] as $release_delivery_guards
  | [
    view("operator_terminal_decision_receipt_retention_readback_ack_non_promotion_view"; "operator"; ["terminalDecisionSurfaceId", "promotionAllowed", "authorityGranted", "nextGate"]),
    view("release_owner_terminal_decision_receipt_retention_readback_ack_release_denial_view"; "release_owner"; ["releasePublished", "publicClaimRecorded", "trafficRouted", "externalDeliveryDenied"]),
    view("auditor_terminal_decision_receipt_retention_readback_ack_digest_denial_view"; "auditor"; ["priorReplayGateDigest", "terminalDecisionHash", "authorityGuardId", "denialId"]),
    view("runtime_terminal_decision_receipt_retention_readback_ack_zero_effect_view"; "system"; ["terminalDecisionRecorded", "livePersistenceEnabled", "releasePublished", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("terminal_decision_receipt_retention_readback_ack_visibility_is_not_promotion"; "terminal decision visibility cannot promote persistence"),
    invariant("terminal_decision_receipt_retention_readback_ack_visibility_is_not_authority"; "terminal decision visibility cannot grant acceptance, approval, or delivery authority"),
    invariant("terminal_decision_receipt_retention_readback_ack_requires_replay_idempotency_gate"; "terminal decision visibility requires terminal decision receipt retention readback acknowledgement replay idempotency evidence first"),
    invariant("terminal_decision_receipt_retention_readback_ack_keeps_release_and_rollout_denied"; "release publication, public claim, rollout, and traffic routing remain denied"),
    invariant("terminal_decision_receipt_retention_readback_ack_views_are_local_only"; "operator, auditor, release-owner, rollback-owner, and runtime views cannot be sent externally"),
    invariant("terminal_decision_receipt_retention_readback_ack_preview_has_no_side_effects"; "this gate cannot record terminal decisions, persist state, grant authority, publish, or send externally")
  ] as $invariants
  | {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate",
    schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_v1",
    preview_mode: "read_only_terminal_decision_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_preview_no_promotion",
    terminal_decision_surface_count: ($terminal_decision_surfaces | length),
    non_promotion_denial_count: ($non_promotion_denials | length),
    authority_guard_count: ($authority_guards | length),
    release_delivery_guard_count: ($release_delivery_guards | length),
    local_view_count: ($local_views | length),
    invariant_count: ($invariants | length),
    required_prior_gates: $required_prior_gates,
    terminal_decision_surfaces: $terminal_decision_surfaces,
    non_promotion_denials: $non_promotion_denials,
    authority_guards: $authority_guards,
    release_delivery_guards: $release_delivery_guards,
    local_views: $local_views,
    invariants: $invariants,
    recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate",
    ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview: true,
    ready_for_operator_acceptance: false,
    ready_for_live_persistence: false,
    side_effects: side_effects_false,
    source_probes: {
      terminal_decision_receipt_retention_readback_ack_terminal_decision: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      terminal_decision_receipt_retention_readback_ack_replay: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      }
    }
  }'
