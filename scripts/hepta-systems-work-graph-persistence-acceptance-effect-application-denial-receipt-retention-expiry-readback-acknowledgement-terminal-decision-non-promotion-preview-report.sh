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

terminal_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview.rs
)"
terminal_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-preview-report.sh
)"
terminal_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-preview-gate.sh
)"
replay_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview.rs
)"
replay_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview-gate.sh
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
  --argjson terminal_rust_module_present "$terminal_rust_module_present" \
  --argjson terminal_report_script_present "$terminal_report_script_present" \
  --argjson terminal_gate_script_present "$terminal_gate_script_present" \
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
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate",
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
  def replay_ids: [
    "duplicate_retention_readback_receipt_replay",
    "duplicate_retention_readback_acknowledgement_replay",
    "stale_retention_readback_digest_replay",
    "superseded_retention_scope_acknowledgement_replay",
    "cross_scope_retention_readback_acknowledgement_replay",
    "out_of_order_retention_readback_acknowledgement_replay"
  ];
  def surface_ids: [
    "operator_terminal_decision_visibility",
    "release_owner_terminal_decision_visibility",
    "auditor_terminal_decision_visibility",
    "rollback_owner_terminal_decision_visibility",
    "runtime_terminal_state_summary_visibility",
    "external_delivery_terminal_decision_echo"
  ];
  def surface($id; $audience; $visibility): {
    id: $id,
    audience: $audience,
    source_replay_scenario_ids: replay_ids,
    decision_visibility: $visibility,
    required_fields: (durable_fields + ["terminalDecisionSurfaceId", "decisionVisibility", "sourceReplayScenarioIds", "nonPromotionProofHash"]),
    decision_recording_allowed: false,
    promotion_allowed: false,
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
    blocks_external_delivery: true
  };
  def authority_guard($id; $fields): {
    id: $id,
    required_fields: (durable_fields + $fields),
    authority_grant_allowed: false
  };
  def release_guard($id; $fields): {
    id: $id,
    required_fields: (durable_fields + $fields),
    release_publication_allowed: false,
    delivery_allowed: false
  };
  def view($id; $audience; $fields): {
    id: $id,
    audience: $audience,
    required_fields: (durable_fields + $fields),
    external_delivery_enabled: false
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  [
    surface("operator_terminal_decision_visibility"; "operator"; "local_operator_terminal_decision_read_only"),
    surface("release_owner_terminal_decision_visibility"; "release_owner"; "local_release_owner_terminal_decision_read_only"),
    surface("auditor_terminal_decision_visibility"; "auditor"; "local_auditor_terminal_decision_read_only"),
    surface("rollback_owner_terminal_decision_visibility"; "rollback_owner"; "local_rollback_owner_terminal_decision_read_only"),
    surface("runtime_terminal_state_summary_visibility"; "system"; "local_runtime_terminal_state_summary_read_only"),
    surface("external_delivery_terminal_decision_echo"; "external_delivery"; "external_delivery_echo_denied")
  ] as $terminal_decision_surfaces
  | [
    denial("durable_identity_evidence_missing"; "retention readback acknowledgement terminal decision cannot proceed without durable identity evidence"),
    denial("terminal_decision_visibility_cannot_promote_persistence"; "terminal decision visibility cannot promote persistence, WAL, or checkpoints"),
    denial("terminal_decision_visibility_cannot_grant_acceptance_authority"; "terminal decision visibility cannot grant acceptance or approval authority"),
    denial("release_owner_terminal_decision_cannot_publish_release"; "release-owner terminal decision visibility cannot publish release state"),
    denial("operator_terminal_decision_cannot_start_rollout"; "operator terminal decision visibility cannot start rollout or route traffic"),
    denial("auditor_terminal_decision_cannot_record_approval"; "auditor terminal decision visibility cannot record approval"),
    denial("rollback_owner_terminal_decision_cannot_mutate_quarantine"; "rollback-owner terminal decision visibility cannot mutate rollback or quarantine state"),
    denial("external_delivery_terminal_decision_echo_cannot_send"; "external delivery echo of terminal decision must stay denied"),
    denial("terminal_decision_summary_cannot_claim_live_completion"; "terminal decision summary cannot claim live persistence completion")
  ] as $non_promotion_denials
  | [
    authority_guard("accepted_record_authority_absent"; ["acceptedRecordId", "acceptedRecordHash", "authoritySignature"]),
    authority_guard("approval_recording_authority_absent"; ["approvalRecordId", "approvalScope", "approvalSignature"]),
    authority_guard("operator_enablement_authority_absent"; ["operatorEnablementPacketId", "enablementScope", "enablementSignature"]),
    authority_guard("release_owner_authority_absent"; ["releaseOwnerId", "releaseScope", "releaseSignature"]),
    authority_guard("live_persistence_authority_absent"; ["livePersistenceFlag", "walAuthorityHash", "checkpointAuthorityHash"]),
    authority_guard("external_delivery_authority_absent"; ["externalDeliveryPolicyId", "deliveryScope", "deliverySignature"])
  ] as $authority_guards
  | [
    release_guard("release_publication_gate_remains_denied"; ["releasePublicationRequest", "releaseOwnerSignature", "releaseHash"]),
    release_guard("public_claim_gate_remains_denied"; ["publicClaimRequest", "claimAudience", "claimHash"]),
    release_guard("traffic_ramp_gate_remains_zero"; ["trafficRampRequest", "trafficPercent", "killSwitchState"]),
    release_guard("external_delivery_gate_remains_denied"; ["externalDeliveryRequest", "destinationPolicy", "deliveryHash"]),
    release_guard("artifact_publication_gate_remains_denied"; ["artifactPublicationRequest", "artifactHash", "redactionHash"]),
    release_guard("terminal_completion_claim_remains_local"; ["terminalDecisionId", "completionClaimHash", "localViewHash"])
  ] as $release_delivery_guards
  | [
    view("operator_terminal_non_promotion_view"; "operator"; ["terminalDecisionSurfaceId", "promotionAllowed", "authorityGranted", "nextGate"]),
    view("release_owner_terminal_release_denial_view"; "release_owner"; ["releasePublished", "publicClaimRecorded", "trafficRouted", "externalDeliveryDenied"]),
    view("auditor_terminal_digest_denial_view"; "auditor"; ["priorReplayGateDigest", "terminalDecisionHash", "authorityGuardId", "denialId"]),
    view("runtime_terminal_zero_effect_view"; "system"; ["terminalDecisionRecorded", "livePersistenceEnabled", "releasePublished", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("retention_readback_ack_terminal_decision_requires_durable_identity_evidence"; "retention readback acknowledgement terminal decision requires workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("terminal_decision_visibility_is_not_promotion"; "operator or release-owner terminal decision visibility cannot promote persistence"),
    invariant("terminal_decision_visibility_is_not_authority"; "terminal decision visibility cannot grant acceptance, approval, or delivery authority"),
    invariant("terminal_decision_requires_replay_idempotency_gate"; "terminal decision visibility requires replay idempotency evidence first"),
    invariant("terminal_decision_keeps_release_and_rollout_denied"; "release publication, public claim, rollout, and traffic routing remain denied"),
    invariant("terminal_decision_views_are_local_only"; "operator, auditor, release-owner, rollback-owner, and runtime views cannot be sent externally"),
    invariant("terminal_decision_preview_has_no_side_effects"; "this gate cannot record terminal decisions, persist state, grant authority, publish, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate",
      schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_v1",
      preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_no_promotion",
      terminal_decision_surface_count: ($terminal_decision_surfaces | length),
      non_promotion_denial_count: ($non_promotion_denials | length),
      authority_guard_count: ($authority_guards | length),
      release_delivery_guard_count: ($release_delivery_guards | length),
      local_view_count: ($local_views | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      terminal_decision_surfaces: $terminal_decision_surfaces,
      non_promotion_denials: $non_promotion_denials,
      authority_guards: $authority_guards,
      release_delivery_guards: $release_delivery_guards,
      local_views: $local_views,
      durable_identity_evidence: {
        schema_version: $durable_identity_report.schema_version,
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: durable_fields,
        required_for_terminal_decision_surface_ids: surface_ids,
        durable_field_count: (durable_fields | length),
        preview_binding_count: 5,
        invariant_count: ($invariants | length),
        currently_satisfied: false
      },
      invariants: $invariants,
      recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate",
      ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview: true,
      ready_for_operator_acceptance: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion: {
          rust_module_present: $terminal_rust_module_present,
          report_script_present: $terminal_report_script_present,
          gate_script_present: $terminal_gate_script_present
        },
        persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency: {
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
        readback_receipt_persisted: false,
        readback_acknowledgement_recorded: false,
        replay_recorded: false,
        terminal_decision_recorded: false,
        terminal_decision_persisted: false,
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
