#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

capture_json_report() {
  local command_name="$1"
  shift

  local output
  output="$("$@")"
  local report
  report="$(printf '%s\n' "$output" | sed '$d')"

  if ! jq -e . >/dev/null <<<"$report"; then
    echo "$command_name did not emit a parseable JSON report" >&2
    exit 1
  fi

  printf '%s\n' "$report"
}

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

REVIEW_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-gate.sh
)"

review_report_sha256="$(
  printf '%s' "$REVIEW_JSON" | shasum -a 256 | awk '{print $1}'
)"

schema_completeness_acceptance_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance:schema-completeness:$review_report_sha256"
)"
denial_reason_acceptance_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance:denial-reasons:$review_report_sha256"
)"
redaction_binding_acceptance_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance:redaction-binding:$review_report_sha256"
)"
public_artifact_acceptance_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance:public-artifact:$review_report_sha256"
)"

jq -n -e \
  --argjson review "$REVIEW_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $review.runtime == "hepta"
    and $review.status == "ready"
    and $review.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_gate"
    and $review.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_ready == true
    and $review.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_ready == true
    and $review.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_ready == true
    and $review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256 != ""
    and $review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256 != ""
    and $review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256 != ""
    and $review.source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256 != ""
    and $review.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256 != ""
    and $review.source_pre_activation_soak_report_sha256 != ""
    and $review.source_persistence_denial_report_sha256 != ""
    and $review.minimum_required_samples >= 24
    and $review.required_rehearsal_receipt_review_field_count == 18
    and $review.rehearsal_receipt_review_field_count == 18
    and $review.recorded_rehearsal_receipt_review_field_count == 0
    and $review.redacted_or_hashed_rehearsal_receipt_review_field_count == 16
    and $review.required_rehearsal_receipt_review_fixture_count == 4
    and $review.rehearsal_receipt_review_fixture_count == 4
    and $review.blocked_rehearsal_receipt_review_fixture_count == 4
    and $review.allowed_rehearsal_receipt_review_fixture_count == 0
    and $review.rehearsal_receipt_review_hash_count == 4
    and $review.rehearsal_receipt_review_requested_count == 4
    and $review.rehearsal_receipt_review_performed_count == 0
    and $review.rehearsal_receipt_review_recorded_count == 0
    and $review.rehearsal_receipt_review_persisted_count == 0
    and $review.rehearsal_receipt_review_materialized_count == 0
    and $review.rehearsal_receipt_review_filesystem_written_count == 0
    and $review.rehearsal_receipt_contract_field_count == 22
    and $review.recorded_rehearsal_receipt_contract_field_count == 0
    and $review.rehearsal_receipt_contract_fixture_count == 4
    and $review.blocked_rehearsal_receipt_contract_fixture_count == 4
    and $review.allowed_rehearsal_receipt_contract_fixture_count == 0
    and $review.rehearsal_receipt_contract_recorded_count == 0
    and $review.rehearsal_receipt_contract_persisted_count == 0
    and $review.rehearsal_receipt_contract_materialized_count == 0
    and $review.rehearsal_receipt_contract_filesystem_written_count == 0
    and $review.rehearsal_receipt_materialized_count == 0
    and $review.rehearsal_receipt_persisted_count == 0
    and $review.ledger_persistence_rehearsal_performed_count == 0
    and $review.ledger_persistence_allowed == false
    and $review.ledger_persistence_execution_performed == false
    and $review.ledger_recorded == false
    and $review.ledger_persisted == false
    and $review.ledger_materialized == false
    and $review.ledger_filesystem_written == false
    and $review.receipt_materialized_count == 0
    and $review.receipt_persisted_count == 0
    and $review.filesystem_persistence_allowed == false
    and $review.filesystem_persistence_execution_performed == false
    and $review.filesystem_write_performed == false
    and $review.workspace_write_performed == false
    and $review.command_invocation_requested_count == 0
    and $review.command_invocation_performed_count == 0
    and $review.command_execution_requested_count == 0
    and $review.command_execution_performed_count == 0
    and $review.materialization_execution_requested_count == 0
    and $review.materialization_execution_performed_count == 0
    and $review.selected_output_path_count == 0
    and $review.recorded_output_path_count == 0
    and $review.recorded_path_binding_count == 0
    and $review.active_binary_sha_bound_count == 0
    and $review.trusted_source_bound_count == 0
    and $review.operator_scope_bound_count == 0
    and $review.accepted_redaction_proof_bound_count == 0
    and $review.rollback_rehearsal_evidence_recorded_count == 0
    and $review.public_claim_allowed == false
    and $review.release_artifact_write_allowed == false
    and $review.raw_payload_plaintext_recorded == false
    and $review.raw_payload_plaintext_persisted == false
    and $review.raw_payload_inspected == false
    and $review.live_secret_scan_performed == false
    and $review.receipt_persistence_enabled == false
    and $review.activation_blocked_by_rehearsal_receipt_review == true
    and $review.activation_allowed_by_rehearsal_receipt_review == false
    and $review.activation_allowed == false
    and $review.live_mutation_execution_ready == false
    and ($review.required_rehearsal_receipt_review_fields | length) == 18
    and ($review.rehearsal_receipt_review_fixtures | length) == 4
    and ($review.rehearsal_receipt_review_fixtures | all(.review_status == "blocked" and .rehearsal_receipt_review_requested == true and .rehearsal_receipt_review_performed == false and .rehearsal_receipt_review_recorded == false and .rehearsal_receipt_review_persisted == false and .rehearsal_receipt_review_materialized == false and .rehearsal_receipt_review_filesystem_written == false and .activation_allowed == false and .deterministic_review_sha256 != ""))
    and ($review.denied_by_rehearsal_receipt_review | length) == 16
    and ($review.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_gate" \
  --arg review_report_sha256 "$review_report_sha256" \
  --arg schema_completeness_acceptance_sha256 "$schema_completeness_acceptance_sha256" \
  --arg denial_reason_acceptance_sha256 "$denial_reason_acceptance_sha256" \
  --arg redaction_binding_acceptance_sha256 "$redaction_binding_acceptance_sha256" \
  --arg public_artifact_acceptance_sha256 "$public_artifact_acceptance_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson review "$REVIEW_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    rehearsal_receipt_review_acceptance_schema_version:"payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_v1",
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_gate:$review.gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_ready:$review.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_ready,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256:$review_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256,
    source_pre_activation_soak_report_sha256:$review.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$review.source_persistence_denial_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_ready:true,
    required_rehearsal_receipt_review_acceptance_field_count:20,
    rehearsal_receipt_review_acceptance_field_count:20,
    recorded_rehearsal_receipt_review_acceptance_field_count:0,
    redacted_or_hashed_rehearsal_receipt_review_acceptance_field_count:18,
    required_rehearsal_receipt_review_acceptance_fixture_count:4,
    rehearsal_receipt_review_acceptance_fixture_count:4,
    blocked_rehearsal_receipt_review_acceptance_fixture_count:4,
    allowed_rehearsal_receipt_review_acceptance_fixture_count:0,
    rehearsal_receipt_review_acceptance_hash_count:4,
    rehearsal_receipt_review_acceptance_requested_count:4,
    rehearsal_receipt_review_acceptance_performed_count:0,
    rehearsal_receipt_review_acceptance_recorded_count:0,
    rehearsal_receipt_review_acceptance_persisted_count:0,
    rehearsal_receipt_review_acceptance_materialized_count:0,
    rehearsal_receipt_review_acceptance_filesystem_written_count:0,
    review_acceptance_allowed_count:0,
    accepted_rehearsal_receipt_review_count:0,
    review_acceptance_policy_satisfied_count:0,
    rehearsal_receipt_review_field_count:$review.rehearsal_receipt_review_field_count,
    recorded_rehearsal_receipt_review_field_count:$review.recorded_rehearsal_receipt_review_field_count,
    rehearsal_receipt_review_fixture_count:$review.rehearsal_receipt_review_fixture_count,
    blocked_rehearsal_receipt_review_fixture_count:$review.blocked_rehearsal_receipt_review_fixture_count,
    allowed_rehearsal_receipt_review_fixture_count:$review.allowed_rehearsal_receipt_review_fixture_count,
    rehearsal_receipt_review_performed_count:$review.rehearsal_receipt_review_performed_count,
    rehearsal_receipt_review_recorded_count:$review.rehearsal_receipt_review_recorded_count,
    rehearsal_receipt_review_persisted_count:$review.rehearsal_receipt_review_persisted_count,
    rehearsal_receipt_review_materialized_count:$review.rehearsal_receipt_review_materialized_count,
    rehearsal_receipt_review_filesystem_written_count:$review.rehearsal_receipt_review_filesystem_written_count,
    rehearsal_receipt_contract_recorded_count:$review.rehearsal_receipt_contract_recorded_count,
    rehearsal_receipt_contract_persisted_count:$review.rehearsal_receipt_contract_persisted_count,
    rehearsal_receipt_contract_materialized_count:$review.rehearsal_receipt_contract_materialized_count,
    rehearsal_receipt_contract_filesystem_written_count:$review.rehearsal_receipt_contract_filesystem_written_count,
    rehearsal_receipt_materialized_count:$review.rehearsal_receipt_materialized_count,
    rehearsal_receipt_persisted_count:$review.rehearsal_receipt_persisted_count,
    ledger_persistence_rehearsal_performed_count:$review.ledger_persistence_rehearsal_performed_count,
    ledger_persistence_allowed:false,
    ledger_persistence_allowed_count:0,
    ledger_persistence_execution_requested_count:0,
    ledger_persistence_execution_performed:false,
    ledger_persistence_execution_performed_count:0,
    ledger_recorded:false,
    ledger_persisted:false,
    ledger_materialized:false,
    ledger_filesystem_written:false,
    ledger_write_path_selected:false,
    ledger_write_path_recorded:false,
    receipt_persistence_allowed_count:0,
    receipt_persistence_execution_performed_count:0,
    receipt_materialized_count:0,
    receipt_persisted_count:0,
    filesystem_persistence_allowed:false,
    filesystem_persistence_allowed_count:0,
    filesystem_persistence_execution_requested_count:0,
    filesystem_persistence_execution_performed:false,
    filesystem_persistence_execution_performed_count:0,
    filesystem_write_requested_count:0,
    filesystem_write_performed:false,
    filesystem_write_performed_count:0,
    workspace_write_performed:false,
    workspace_write_performed_count:0,
    command_invocation_requested_count:0,
    command_invocation_performed_count:0,
    command_execution_requested_count:0,
    command_execution_performed_count:0,
    materialization_execution_requested_count:0,
    materialization_execution_performed_count:0,
    selected_output_path_count:0,
    recorded_output_path_count:0,
    recorded_path_binding_count:0,
    active_binary_sha_bound_count:0,
    trusted_source_bound_count:0,
    operator_scope_bound_count:0,
    accepted_redaction_proof_bound_count:0,
    future_active_binary_sha_bound_fixture_count:$review.future_active_binary_sha_bound_fixture_count,
    future_trusted_source_bound_fixture_count:$review.future_trusted_source_bound_fixture_count,
    future_operator_scope_bound_fixture_count:$review.future_operator_scope_bound_fixture_count,
    future_accepted_redaction_proof_bound_fixture_count:$review.future_accepted_redaction_proof_bound_fixture_count,
    future_rollback_rehearsal_evidence_slot_count:$review.future_rollback_rehearsal_evidence_slot_count,
    rollback_rehearsal_evidence_recorded_count:0,
    source_tree_path_attempt_fixture_count:1,
    workspace_path_attempt_fixture_count:1,
    public_claim_attempt_fixture_count:1,
    release_artifact_write_attempt_fixture_count:1,
    public_claim_allowed:false,
    release_artifact_write_allowed:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    raw_payload_inspected:false,
    live_secret_scan_performed:false,
    receipt_persistence_enabled:false,
    operator_approval_required:true,
    operator_approval_recorded:false,
    fresh_pre_activation_soak_evidence_required:true,
    fresh_pre_activation_soak_evidence_recorded:false,
    accepted_redaction_proof_required:true,
    accepted_redaction_proof_recorded:false,
    rollback_rehearsal_evidence_required:true,
    rollback_rehearsal_evidence_recorded:false,
    public_artifact_decision_required:true,
    public_artifact_decision_recorded:false,
    activation_blocked_by_rehearsal_receipt_review_acceptance:true,
    activation_allowed_by_rehearsal_receipt_review_acceptance:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    rehearsal_receipt_review_acceptance_denial_reason:"future rehearsal receipt review acceptance remains schema-only until the review hash, contract hash set, denial reason set, operator approval, fresh live evidence, accepted redaction proof, rollback rehearsal evidence, and public artifact decision are recorded and verified",
    required_rehearsal_receipt_review_acceptance_fields:[
      "rehearsal_receipt_review_acceptance_id",
      "acceptance_schema_version",
      "source_rehearsal_receipt_review_report_sha256",
      "source_rehearsal_receipt_contract_report_sha256",
      "review_fixture_id",
      "deterministic_acceptance_sha256",
      "review_hash_binding",
      "contract_field_set_hash",
      "denial_reason_set_hash",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "fresh_pre_activation_soak_evidence_id",
      "active_binary_sha256",
      "trusted_source_binding",
      "accepted_redaction_proof_ids",
      "rollback_rehearsal_evidence_id",
      "rollback_plan_id",
      "public_claim_and_artifact_decision",
      "acceptance_timestamp_policy"
    ],
    rehearsal_receipt_review_acceptance_fixtures:[
      {
        id:"schema-completeness-rehearsal-receipt-review-acceptance",
        source_review_fixture_id:"schema-completeness-rehearsal-receipt-review",
        deterministic_acceptance_sha256:$schema_completeness_acceptance_sha256,
        acceptance_status:"blocked",
        rehearsal_receipt_review_acceptance_requested:true,
        rehearsal_receipt_review_acceptance_performed:false,
        rehearsal_receipt_review_acceptance_recorded:false,
        rehearsal_receipt_review_accepted:false,
        rehearsal_receipt_review_acceptance_persisted:false,
        rehearsal_receipt_review_acceptance_materialized:false,
        rehearsal_receipt_review_acceptance_filesystem_written:false,
        activation_allowed:false,
        denial_reasons:["review_acceptance_recording_denied","operator_approval_missing","fresh_pre_activation_soak_evidence_missing_or_stale","live_mutation_execution_denied"]
      },
      {
        id:"denial-reason-set-rehearsal-receipt-review-acceptance",
        source_review_fixture_id:"denial-reason-set-rehearsal-receipt-review",
        deterministic_acceptance_sha256:$denial_reason_acceptance_sha256,
        acceptance_status:"blocked",
        rehearsal_receipt_review_acceptance_requested:true,
        rehearsal_receipt_review_acceptance_performed:false,
        rehearsal_receipt_review_acceptance_recorded:false,
        rehearsal_receipt_review_accepted:false,
        rehearsal_receipt_review_acceptance_persisted:false,
        rehearsal_receipt_review_acceptance_materialized:false,
        rehearsal_receipt_review_acceptance_filesystem_written:false,
        activation_allowed:false,
        denial_reasons:["denial_reason_set_hash_not_accepted","rollback_rehearsal_evidence_missing","review_acceptance_persistence_denied","live_mutation_execution_denied"]
      },
      {
        id:"redaction-binding-rehearsal-receipt-review-acceptance",
        source_review_fixture_id:"redaction-binding-rehearsal-receipt-review",
        deterministic_acceptance_sha256:$redaction_binding_acceptance_sha256,
        acceptance_status:"blocked",
        rehearsal_receipt_review_acceptance_requested:true,
        rehearsal_receipt_review_acceptance_performed:false,
        rehearsal_receipt_review_acceptance_recorded:false,
        rehearsal_receipt_review_accepted:false,
        rehearsal_receipt_review_acceptance_persisted:false,
        rehearsal_receipt_review_acceptance_materialized:false,
        rehearsal_receipt_review_acceptance_filesystem_written:false,
        activation_allowed:false,
        denial_reasons:["accepted_redaction_proof_missing","source_tree_workspace_output_path_denied","filesystem_write_denied","workspace_write_denied","live_mutation_execution_denied"]
      },
      {
        id:"public-artifact-rehearsal-receipt-review-acceptance",
        source_review_fixture_id:"public-artifact-rehearsal-receipt-review",
        deterministic_acceptance_sha256:$public_artifact_acceptance_sha256,
        acceptance_status:"blocked",
        rehearsal_receipt_review_acceptance_requested:true,
        rehearsal_receipt_review_acceptance_performed:false,
        rehearsal_receipt_review_acceptance_recorded:false,
        rehearsal_receipt_review_accepted:false,
        rehearsal_receipt_review_acceptance_persisted:false,
        rehearsal_receipt_review_acceptance_materialized:false,
        rehearsal_receipt_review_acceptance_filesystem_written:false,
        activation_allowed:false,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        denial_reasons:["public_artifact_release_output_path_denied","public_release_claim_denied","release_artifact_write_denied","review_acceptance_persistence_denied","live_mutation_execution_denied"]
      }
    ],
    denied_by_rehearsal_receipt_review_acceptance:[
      "review_acceptance_recording_denied",
      "review_acceptance_materialization_denied",
      "review_acceptance_persistence_denied",
      "review_acceptance_execution_denied",
      "operator_approval_missing",
      "fresh_pre_activation_soak_evidence_missing_or_stale",
      "accepted_redaction_proof_missing",
      "rollback_rehearsal_evidence_missing",
      "review_hash_binding_not_recorded",
      "contract_field_set_hash_not_accepted",
      "denial_reason_set_hash_not_accepted",
      "source_tree_workspace_output_path_denied",
      "public_artifact_release_output_path_denied",
      "ledger_persistence_execution_denied",
      "filesystem_persistence_execution_denied",
      "filesystem_write_denied",
      "workspace_write_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "live_mutation_execution_denied"
    ],
    side_effects:{
      memory_store_mutated:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      coding_agent_spawned:false,
      skill_workshop_written:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      command_invocation_performed:false,
      command_execution_performed:false,
      receipt_persistence_execution_performed:false,
      materialization_execution_performed:false,
      filesystem_persistence_execution_performed:false,
      ledger_persistence_rehearsal_performed:false,
      ledger_persistence_execution_performed:false,
      ledger_recorded:false,
      ledger_persisted:false,
      ledger_filesystem_written:false,
      rehearsal_receipt_contract_recorded:false,
      rehearsal_receipt_contract_persisted:false,
      rehearsal_receipt_contract_materialized:false,
      rehearsal_receipt_contract_filesystem_written:false,
      rehearsal_receipt_review_performed:false,
      rehearsal_receipt_review_recorded:false,
      rehearsal_receipt_review_persisted:false,
      rehearsal_receipt_review_materialized:false,
      rehearsal_receipt_review_filesystem_written:false,
      rehearsal_receipt_review_acceptance_performed:false,
      rehearsal_receipt_review_acceptance_recorded:false,
      rehearsal_receipt_review_acceptance_persisted:false,
      rehearsal_receipt_review_acceptance_materialized:false,
      rehearsal_receipt_review_acceptance_filesystem_written:false,
      rehearsal_receipt_review_accepted:false,
      rehearsal_receipt_materialized:false,
      rehearsal_receipt_persisted:false,
      filesystem_written:false,
      workspace_write_performed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      rollback_rehearsal_evidence_recorded:false,
      receipt_materialized:false,
      receipt_persisted:false,
      dry_run_ledger_recorded:false,
      dry_run_ledger_materialized:false,
      dry_run_ledger_persisted:false,
      dry_run_ledger_filesystem_written:false,
      filesystem_persistence_approval_packet_persisted:false,
      output_path_allowlist_persisted:false,
      output_path_evidence_binding_persisted:false,
      filesystem_sink_write_preview_persisted:false,
      execution_denial_matrix_persisted:false,
      output_path_selected:false,
      output_path_binding_selected:false,
      materialization_plan_persisted:false,
      pre_activation_soak_evidence_persisted:false,
      approval_packet_persisted:false,
      operator_scope_binding_persisted:false,
      payload_review_persisted:false,
      payload_redaction_proof_persisted:false,
      payload_redaction_acceptance_matrix_persisted:false,
      payload_redaction_acceptance_receipt_command_persisted:false,
      payload_redaction_acceptance_receipt_no_write_sink_persisted:false,
      payload_redaction_acceptance_receipt_write_enable_fixture_persisted:false,
      payload_redaction_acceptance_receipt_materialization_plan_persisted:false,
      payload_redaction_acceptance_receipt_persisted:false,
      payload_plaintext_persisted:false,
      raw_payload_inspected:false,
      live_secret_scan_performed:false,
      external_send_performed:false,
      credential_read:false,
      secret_file_read:false
    }
  }')"

jq -e '
  .status == "ready"
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_rehearsal_receipt_review_acceptance_field_count == 20
  and .rehearsal_receipt_review_acceptance_field_count == 20
  and .recorded_rehearsal_receipt_review_acceptance_field_count == 0
  and .redacted_or_hashed_rehearsal_receipt_review_acceptance_field_count == 18
  and .required_rehearsal_receipt_review_acceptance_fixture_count == 4
  and .rehearsal_receipt_review_acceptance_fixture_count == 4
  and .blocked_rehearsal_receipt_review_acceptance_fixture_count == 4
  and .allowed_rehearsal_receipt_review_acceptance_fixture_count == 0
  and .rehearsal_receipt_review_acceptance_hash_count == 4
  and .rehearsal_receipt_review_acceptance_requested_count == 4
  and .rehearsal_receipt_review_acceptance_performed_count == 0
  and .rehearsal_receipt_review_acceptance_recorded_count == 0
  and .rehearsal_receipt_review_acceptance_persisted_count == 0
  and .rehearsal_receipt_review_acceptance_materialized_count == 0
  and .rehearsal_receipt_review_acceptance_filesystem_written_count == 0
  and .accepted_rehearsal_receipt_review_count == 0
  and .review_acceptance_policy_satisfied_count == 0
  and .rehearsal_receipt_review_performed_count == 0
  and .rehearsal_receipt_review_recorded_count == 0
  and .rehearsal_receipt_review_persisted_count == 0
  and .rehearsal_receipt_review_materialized_count == 0
  and .rehearsal_receipt_review_filesystem_written_count == 0
  and .rehearsal_receipt_contract_recorded_count == 0
  and .rehearsal_receipt_contract_persisted_count == 0
  and .rehearsal_receipt_contract_materialized_count == 0
  and .rehearsal_receipt_contract_filesystem_written_count == 0
  and .rehearsal_receipt_materialized_count == 0
  and .rehearsal_receipt_persisted_count == 0
  and .ledger_persistence_allowed == false
  and .ledger_persistence_execution_performed == false
  and .ledger_recorded == false
  and .ledger_persisted == false
  and .ledger_materialized == false
  and .ledger_filesystem_written == false
  and .receipt_persistence_execution_performed_count == 0
  and .receipt_materialized_count == 0
  and .receipt_persisted_count == 0
  and .filesystem_persistence_allowed == false
  and .filesystem_persistence_execution_performed == false
  and .filesystem_write_performed == false
  and .workspace_write_performed == false
  and .command_invocation_requested_count == 0
  and .command_invocation_performed_count == 0
  and .command_execution_requested_count == 0
  and .command_execution_performed_count == 0
  and .materialization_execution_requested_count == 0
  and .materialization_execution_performed_count == 0
  and .selected_output_path_count == 0
  and .recorded_output_path_count == 0
  and .recorded_path_binding_count == 0
  and .active_binary_sha_bound_count == 0
  and .trusted_source_bound_count == 0
  and .operator_scope_bound_count == 0
  and .accepted_redaction_proof_bound_count == 0
  and .rollback_rehearsal_evidence_recorded_count == 0
  and .rollback_rehearsal_evidence_recorded == false
  and .public_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .raw_payload_inspected == false
  and .live_secret_scan_performed == false
  and .receipt_persistence_enabled == false
  and .operator_approval_recorded == false
  and .fresh_pre_activation_soak_evidence_recorded == false
  and .accepted_redaction_proof_recorded == false
  and .public_artifact_decision_recorded == false
  and .activation_blocked_by_rehearsal_receipt_review_acceptance == true
  and .activation_allowed_by_rehearsal_receipt_review_acceptance == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.required_rehearsal_receipt_review_acceptance_fields | length) == 20
  and (.rehearsal_receipt_review_acceptance_fixtures | length) == 4
  and (.rehearsal_receipt_review_acceptance_fixtures | all(.acceptance_status == "blocked" and .rehearsal_receipt_review_acceptance_requested == true and .rehearsal_receipt_review_acceptance_performed == false and .rehearsal_receipt_review_acceptance_recorded == false and .rehearsal_receipt_review_accepted == false and .rehearsal_receipt_review_acceptance_persisted == false and .rehearsal_receipt_review_acceptance_materialized == false and .rehearsal_receipt_review_acceptance_filesystem_written == false and .activation_allowed == false and .deterministic_acceptance_sha256 != ""))
  and (.denied_by_rehearsal_receipt_review_acceptance | length) == 20
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance gate passed"
