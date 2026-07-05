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

readiness_packet_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_operator_readiness_packet_preview.rs
)"
readiness_packet_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-operator-readiness-packet-preview-report.sh
)"
readiness_packet_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-operator-readiness-packet-preview-gate.sh
)"
rollout_blocker_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_enforcement_rollout_blocker_preview.rs
)"
rollout_blocker_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-enforcement-rollout-blocker-preview-gate.sh
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
  --argjson readiness_packet_rust_module_present "$readiness_packet_rust_module_present" \
  --argjson readiness_packet_report_script_present "$readiness_packet_report_script_present" \
  --argjson readiness_packet_gate_script_present "$readiness_packet_gate_script_present" \
  --argjson rollout_blocker_rust_module_present "$rollout_blocker_rust_module_present" \
  --argjson rollout_blocker_gate_script_present "$rollout_blocker_gate_script_present" \
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
    "hepta_work_graph_durable_identity_preview_gate"
  ];
  def templates: [
    "store_persistence_readiness_packet",
    "wal_checkpoint_readiness_packet",
    "readback_receipt_readiness_packet",
    "replay_execution_readiness_packet",
    "external_publication_readiness_packet",
    "full_rollout_abort_readiness_packet"
  ];
  def template($id; $stage; $sections): {
    id: $id,
    target_rollout_stage_id: $stage,
    required_section_ids: $sections,
    acceptance_allowed: false,
    external_delivery_enabled: false
  };
  def section($id; $fields): {
    id: $id,
    required_fields: $fields,
    redaction_state: "redacted_hash_only",
    currently_complete: false
  };
  def denial($id; $sections; $reason): {
    id: $id,
    applies_to_section_ids: $sections,
    reason: $reason,
    blocks_acceptance: true
  };
  def guard($id; $templates; $fields): {
    id: $id,
    applies_to_template_ids: $templates,
    required_evidence_fields: $fields,
    currently_satisfied: false
  };
  def revoke($id; $templates; $trigger): {
    id: $id,
    applies_to_template_ids: $templates,
    trigger: $trigger,
    blocks_acceptance: true
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  [
    template("store_persistence_readiness_packet"; "store_persistence_enforcement_rollout"; ["durable_identity_section", "operator_scope_section", "shadow_live_digest_section", "rollback_owner_section", "release_denial_section"]),
    template("wal_checkpoint_readiness_packet"; "wal_append_enforcement_rollout"; ["durable_identity_section", "operator_scope_section", "wal_checkpoint_schema_section", "traffic_ramp_blocker_section", "kill_switch_section"]),
    template("readback_receipt_readiness_packet"; "readback_receipt_enforcement_rollout"; ["durable_identity_section", "operator_scope_section", "receipt_redaction_section", "retention_expiry_section", "rollback_owner_section"]),
    template("replay_execution_readiness_packet"; "replay_execution_enforcement_rollout"; ["durable_identity_section", "operator_scope_section", "replay_drift_budget_section", "lane_lease_section", "kill_switch_section"]),
    template("external_publication_readiness_packet"; "external_publication_enforcement_rollout"; ["durable_identity_section", "operator_scope_section", "external_policy_section", "release_denial_section", "external_delivery_readback_section"]),
    template("full_rollout_abort_readiness_packet"; "all_persistence_enforcement_rollouts"; ["durable_identity_section", "abort_scope_section", "kill_switch_section", "rollback_owner_section", "retention_expiry_section"])
  ] as $packet_templates
  | [
    section("durable_identity_section"; durable_fields),
    section("operator_scope_section"; ["operatorScopeHash", "operatorIdHash", "expiresAtUnixMs"]),
    section("shadow_live_digest_section"; ["shadowDigestHash", "futureLiveProbeId", "mismatchClassifierIds"]),
    section("rollback_owner_section"; ["rollbackOwnerId", "quarantineScope", "killSwitchId"]),
    section("release_denial_section"; ["releaseDenialIds", "publicationDenied", "targetSurfaceId"]),
    section("wal_checkpoint_schema_section"; ["walSchemaDigest", "checkpointSchemaDigest", "diskBudgetHash"]),
    section("traffic_ramp_blocker_section"; ["trafficRampBlockerIds", "maxTrafficPpm", "rampDenied"]),
    section("receipt_redaction_section"; ["receiptSchemaHash", "redactionState", "payloadHashOnly"]),
    section("retention_expiry_section"; ["expiresAtUnixMs", "revocationReasonHash", "retentionPolicyHash"]),
    section("replay_drift_budget_section"; ["driftBudgetHash", "replayIdempotencyHash", "laneLeaseHash"]),
    section("lane_lease_section"; ["laneId", "agentId", "leaseExpiresAtUnixMs"]),
    section("kill_switch_section"; ["killSwitchIds", "armedInPreview", "rollbackOwnerIds"]),
    section("external_policy_section"; ["deliveryPolicyHash", "externalTargetScope", "externalDeliveryDisabled"]),
    section("external_delivery_readback_section"; ["deliveryReadbackGate", "readbackHash", "publicationDenied"]),
    section("abort_scope_section"; ["abortReasonHash", "affectedStageIds", "quarantineScopes"])
  ] as $packet_sections
  | [
    denial("deny_missing_durable_identity_evidence"; ["durable_identity_section"]; "durable identity evidence packet is missing or unsatisfied"),
    denial("deny_missing_operator_scope"; ["operator_scope_section"]; "operator scope or identity hash is missing"),
    denial("deny_missing_shadow_live_digest"; ["shadow_live_digest_section"]; "shadow/live digest evidence is missing"),
    denial("deny_missing_rollback_owner"; ["rollback_owner_section"]; "rollback owner or quarantine scope is missing"),
    denial("deny_release_denial_matrix_missing"; ["release_denial_section"]; "release/publication denial matrix is missing"),
    denial("deny_traffic_ramp_not_zero"; ["traffic_ramp_blocker_section"]; "traffic ramp is not locked to zero"),
    denial("deny_receipt_redaction_missing"; ["receipt_redaction_section"]; "readiness packet is not redacted/hash-only"),
    denial("deny_packet_expired_or_revoked"; ["retention_expiry_section"]; "readiness packet is expired, superseded, or revoked"),
    denial("deny_external_policy_missing"; ["external_policy_section", "external_delivery_readback_section"]; "external policy or delivery readback gate is missing")
  ] as $validation_denials
  | [
    guard("guard_durable_identity_evidence_declared"; templates; durable_fields),
    guard("guard_non_recording_preview_acceptance"; templates; ["previewMode", "approvalRecorded", "sideEffectHash"]),
    guard("guard_all_sections_complete"; templates; ["requiredSectionIds", "completeSectionIds", "validationDenialIds"]),
    guard("guard_release_publication_denied"; templates; ["releaseDenialIds", "publicationDenied", "externalDeliveryDisabled"]),
    guard("guard_rollback_owners_declared"; templates; ["rollbackOwnerIds", "quarantineScopes", "killSwitchIds"]),
    guard("guard_expiry_and_revocation_current"; templates; ["expiresAtUnixMs", "revocationStatus", "supersessionId"])
  ] as $acceptance_guards
  | [
    revoke("readiness_packet_expired"; templates; "expiresAtUnixMs is in the past"),
    revoke("readiness_packet_superseded"; templates; "newer packet digest supersedes this preview"),
    revoke("operator_scope_revoked"; templates; "operator authority scope is revoked"),
    revoke("rollback_owner_revoked"; templates; "rollback owner is unavailable or revoked")
  ] as $expiry_revocations
  | [
    invariant("operator_readiness_requires_durable_identity_evidence"; "operator readiness packets require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("readiness_packets_are_non_accepting"; "packet templates describe requirements but cannot record operator acceptance"),
    invariant("every_packet_requires_operator_scope"; "all readiness packet templates include an operator scope section"),
    invariant("release_and_publication_stay_denied"; "operator readiness cannot override release or publication denial in preview"),
    invariant("expiry_revocation_blocks_acceptance"; "expired, superseded, or revoked packets cannot become future acceptance receipts"),
    invariant("external_delivery_requires_separate_policy"; "external publication readiness has its own policy and readback sections"),
    invariant("operator_readiness_packet_preview_has_no_side_effects"; "this gate cannot persist packets, record approvals, enable enforcement, route traffic, publish releases, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_operator_readiness_packet_preview_gate",
      schema_version: "work_graph_persistence_operator_readiness_packet_preview_v1",
      preview_mode: "read_only_persistence_operator_readiness_packet_preview_no_acceptance",
      packet_template_count: ($packet_templates | length),
      packet_section_count: ($packet_sections | length),
      validation_denial_count: ($validation_denials | length),
      acceptance_guard_count: ($acceptance_guards | length),
      expiry_revocation_count: ($expiry_revocations | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      packet_templates: $packet_templates,
      packet_sections: $packet_sections,
      validation_denials: $validation_denials,
      acceptance_guards: $acceptance_guards,
      expiry_revocations: $expiry_revocations,
      durable_identity_evidence: {
        schema_version: $durable_identity_report.schema_version,
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: durable_fields,
        required_for_template_ids: templates,
        required_section_id: "durable_identity_section",
        durable_field_count: $durable_identity_report.durable_field_count,
        preview_binding_count: $durable_identity_report.preview_binding_count,
        invariant_count: $durable_identity_report.invariant_count,
        currently_satisfied: false
      },
      invariants: $invariants,
      recommended_next_gate: "hepta_work_graph_persistence_operator_readiness_receipt_preview_gate",
      ready_for_operator_readiness_receipt_preview: true,
      ready_for_operator_acceptance: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_operator_readiness_packet: {
          rust_module_present: $readiness_packet_rust_module_present,
          report_script_present: $readiness_packet_report_script_present,
          gate_script_present: $readiness_packet_gate_script_present
        },
        persistence_enforcement_rollout_blocker: {
          rust_module_present: $rollout_blocker_rust_module_present,
          gate_script_present: $rollout_blocker_gate_script_present
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
        readiness_packet_persisted: false,
        operator_acceptance_recorded: false,
        approval_recorded: false,
        enforcement_enabled: false,
        rollout_started: false,
        traffic_routed: false,
        live_readback_executed: false,
        release_published: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
