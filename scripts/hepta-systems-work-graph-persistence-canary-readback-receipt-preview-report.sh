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
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_canary_readback_receipt_preview.rs
)"
receipt_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-canary-readback-receipt-preview-report.sh
)"
receipt_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-canary-readback-receipt-preview-gate.sh
)"
canary_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-canary-dry-run-preview-gate.sh
)"
feature_flag_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-feature-flag-preview-gate.sh
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
  --argjson canary_gate_script_present "$canary_gate_script_present" \
  --argjson feature_flag_gate_script_present "$feature_flag_gate_script_present" \
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
  def receipt_contract_ids: [
    "store_persistence_canary_receipt",
    "wal_append_canary_receipt",
    "checkpoint_write_canary_receipt",
    "readback_receipt_canary_receipt",
    "idempotency_index_canary_receipt",
    "replay_execution_canary_receipt"
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
    "hepta_work_graph_durable_identity_preview_gate"
  ];
  def receipt($id; $scenario): {
    id: $id,
    source_dry_run_scenario_id: $scenario,
    required_fields: (durable_fields + ["receiptId", "scenarioId", "featureFlagId", "priorGateReportHash", "evidenceHash", "zeroTrafficProof", "zeroWriteProof", "rollbackGuardIds", "redactionState"]),
    redaction_state: "redacted_hash_only",
    persistence_enabled: false,
    external_delivery_enabled: false
  };
  def digest($id; $fields): {
    id: $id,
    compared_fields: $fields,
    blocks_promotion: true
  };
  def denial($id; $trigger; $receipts): {
    id: $id,
    trigger: $trigger,
    applies_to_receipt_ids: $receipts,
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
    receipt("store_persistence_canary_receipt"; "canary_store_persistence_dry_run"),
    receipt("wal_append_canary_receipt"; "canary_wal_append_dry_run"),
    receipt("checkpoint_write_canary_receipt"; "canary_checkpoint_write_dry_run"),
    receipt("readback_receipt_canary_receipt"; "canary_readback_receipt_dry_run"),
    receipt("idempotency_index_canary_receipt"; "canary_idempotency_index_dry_run"),
    receipt("replay_execution_canary_receipt"; "canary_replay_execution_dry_run")
  ] as $receipt_contracts
  | ($receipt_contracts | map(.id)) as $receipt_ids
  | [
    digest("check_prior_gate_digest_hash"; ["priorGateIds", "priorGateReportHash", "generatedAtUnixMs"]),
    digest("check_feature_flag_digest_hash"; ["featureFlagId", "defaultState", "operatorIdHash"]),
    digest("check_canary_evidence_hash"; ["scenarioId", "expectedEvidenceIds", "evidenceHash"]),
    digest("check_zero_write_and_traffic_hash"; ["trafficPpm", "writeMode", "sideEffectHash"]),
    digest("check_rollback_guard_hash"; ["rollbackGuardIds", "receiptHash", "redactionState"]),
    digest("check_durable_identity_digest_hash"; durable_fields)
  ] as $digest_checks
  | [
    denial("missing_prior_gate_digest"; "receipt does not include the full prior gate digest"; $receipt_ids),
    denial("feature_flag_not_default_off"; "receipt proposes a mutable or enabled feature flag"; $receipt_ids),
    denial("canary_evidence_hash_missing"; "receipt omits scenario evidence hash"; $receipt_ids),
    denial("zero_write_or_traffic_not_proven"; "receipt cannot prove zero traffic and zero writes"; $receipt_ids),
    denial("rollback_guard_receipt_missing"; "receipt omits rollback guard coverage"; $receipt_ids),
    denial("receipt_redaction_missing"; "receipt is not redacted/hash-only"; $receipt_ids),
    denial("durable_identity_evidence_missing"; "receipt omits workflow, run, step, checkpoint, replay, rollback, or receipt identity"; $receipt_ids)
  ] as $denial_reasons
  | [
    view("operator_canary_receipt_summary"; "operator"; ["workflow_id", "run_id", "step_id", "receipt_hash", "scenarioId", "featureFlagId", "zeroTraffic", "zeroWrites", "rollbackGuardIds"]),
    view("auditor_canary_digest_view"; "auditor"; ["workflow_id", "run_id", "checkpoint", "receipt_hash", "receiptId", "priorGateReportHash", "evidenceHash", "redactionState"]),
    view("rollback_receipt_preview_view"; "operator"; ["workflow_id", "rollback_anchor", "replay_key", "receipt_hash", "triggerGuardId", "receiptHash", "rollbackOwnerHash", "expiresAtUnixMs"]),
    view("promotion_blocker_packet_view"; "system"; ["workflow_id", "run_id", "step_id", "receipt_hash", "denialReasonIds", "digestCheckIds", "receiptContractIds", "nextGate"])
  ] as $operator_views
  | [
    invariant("canary_readback_receipts_require_durable_identity_evidence"; "receipt contracts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("canary_receipts_are_hash_only"; "receipt contracts expose hashes, refs, and redaction state instead of payloads"),
    invariant("canary_receipts_require_prior_gate_digest"; "receipt previews cannot be accepted without the complete prior gate digest"),
    invariant("canary_receipts_prove_zero_write_and_traffic"; "receipt previews must carry evidence for zero live traffic and zero persisted writes"),
    invariant("canary_receipt_denials_block_promotion"; "any missing digest, redaction, rollback, or zero-write proof blocks promotion"),
    invariant("operator_views_are_not_external_delivery"; "operator and auditor views are local preview shapes and cannot be sent externally"),
    invariant("persistence_canary_readback_receipt_preview_has_no_side_effects"; "this gate cannot execute readback, persist receipts, promote state, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
      schema_version: "work_graph_persistence_canary_readback_receipt_preview_v1",
      preview_mode: "read_only_persistence_canary_readback_receipt_preview_no_receipt_write",
      receipt_contract_count: ($receipt_contracts | length),
      digest_check_count: ($digest_checks | length),
      denial_reason_count: ($denial_reasons | length),
      operator_view_count: ($operator_views | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      receipt_contracts: $receipt_contracts,
      digest_checks: $digest_checks,
      denial_reasons: $denial_reasons,
      operator_views: $operator_views,
      durable_identity_evidence: {
        schema_version: $durable_identity_report.schema_version,
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: durable_fields,
        required_for_receipt_contract_ids: receipt_contract_ids,
        durable_field_count: $durable_identity_report.durable_field_count,
        preview_binding_count: $durable_identity_report.preview_binding_count,
        invariant_count: $durable_identity_report.invariant_count,
        currently_satisfied: false
      },
      invariants: $invariants,
      recommended_next_gate: "hepta_work_graph_persistence_promotion_blocker_preview_gate",
      ready_for_promotion_blocker_preview: true,
      ready_for_receipt_persistence: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_canary_readback_receipt: {
          rust_module_present: $receipt_rust_module_present,
          report_script_present: $receipt_report_script_present,
          gate_script_present: $receipt_gate_script_present
        },
        persistence_canary_dry_run: {
          gate_script_present: $canary_gate_script_present
        },
        persistence_feature_flag: {
          gate_script_present: $feature_flag_gate_script_present
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
        receipt_persisted: false,
        readback_performed: false,
        promotion_performed: false,
        feature_flag_mutated: false,
        canary_executed: false,
        live_traffic_routed: false,
        wal_written: false,
        checkpoint_written: false,
        approval_recorded: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
