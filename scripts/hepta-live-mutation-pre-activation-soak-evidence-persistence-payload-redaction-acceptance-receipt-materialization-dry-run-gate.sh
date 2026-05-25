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

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

WRITE_ENABLE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-write-enable-fixture-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-write-enable-fixture-gate.sh
)"

write_enable_report_sha256="$(printf '%s' "$WRITE_ENABLE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson write "$WRITE_ENABLE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $write.runtime == "hepta"
    and $write.status == "ready"
    and $write.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_write_enable_fixture_gate"
    and $write.payload_redaction_acceptance_receipt_write_enable_fixture_ready == true
    and $write.source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256 != ""
    and $write.required_write_enable_fixture_count == 5
    and $write.write_enable_fixture_count == 5
    and $write.blocked_write_enable_fixture_count == 5
    and $write.allowed_write_enable_fixture_count == 0
    and $write.explicit_write_enable_requested_fixture_count == 5
    and $write.command_invocation_performed_count == 0
    and $write.command_execution_performed_count == 0
    and $write.receipt_persistence_execution_performed_count == 0
    and $write.filesystem_write_performed_count == 0
    and $write.workspace_write_performed_count == 0
    and $write.receipt_persisted_count == 0
    and $write.raw_payload_plaintext_recorded == false
    and $write.raw_payload_plaintext_persisted == false
    and $write.receipt_persistence_enabled == false
    and $write.activation_allowed == false
    and $write.live_mutation_execution_ready == false
    and ($write.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_materialization_dry_run_gate" \
  --arg write_enable_report_sha256 "$write_enable_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson write "$WRITE_ENABLE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_payload_redaction_acceptance_receipt_write_enable_fixture_gate:$write.gate,
    source_payload_redaction_acceptance_receipt_write_enable_fixture_ready:$write.payload_redaction_acceptance_receipt_write_enable_fixture_ready,
    source_receipt_payload_sha256:$write.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$write.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$write.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$write.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$write.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$write.source_no_secret_payload_review_report_sha256,
    source_payload_redaction_proof_report_sha256:$write.source_payload_redaction_proof_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$write.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_acceptance_receipt_command_contract_report_sha256:$write.source_payload_redaction_acceptance_receipt_command_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256:$write.source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256:$write.source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256:$write_enable_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_materialization_dry_run_ready:true,
    payload_redaction_acceptance_receipt_write_enable_fixture_ready:true,
    payload_redaction_acceptance_receipt_no_write_sink_contract_ready:true,
    payload_redaction_acceptance_receipt_invocation_dry_run_ready:true,
    payload_redaction_acceptance_receipt_command_contract_ready:true,
    payload_redaction_acceptance_receipt_command_recorded:false,
    payload_redaction_acceptance_receipt_command_enabled_by_default:false,
    payload_redaction_acceptance_receipt_command_invocation_requested_count:5,
    payload_redaction_acceptance_receipt_command_invocation_performed_count:0,
    payload_redaction_acceptance_receipt_command_execution_performed_count:0,
    payload_redaction_acceptance_receipt_recorded:false,
    payload_redaction_acceptance_receipt_materialized:false,
    payload_redaction_acceptance_receipt_persisted:false,
    payload_redaction_acceptance_matrix_recorded:false,
    payload_redaction_acceptance_matrix_persisted:false,
    payload_redaction_proof_recorded:false,
    payload_redaction_proof_accepted:false,
    accepted_redaction_proof_count:0,
    reviewed_redaction_proof_count:0,
    required_materialization_fixture_count:5,
    materialization_fixture_count:5,
    blocked_materialization_fixture_count:5,
    allowed_materialization_fixture_count:0,
    explicit_write_enable_requested_fixture_count:5,
    materialization_denied_without_operator_scope_count:1,
    materialization_denied_command_disabled_count:1,
    materialization_denied_persistence_disabled_count:1,
    materialization_denied_plaintext_payload_count:1,
    materialization_denied_public_artifact_count:1,
    deterministic_materialization_plan_count:3,
    deterministic_materialization_plan_persisted_count:0,
    payload_hash_planned_fixture_count:5,
    redacted_payload_summary_hash_bound_fixture_count:5,
    redacted_output_path_planned_fixture_count:4,
    redacted_receipt_shape_fixture_count:3,
    accepted_redaction_proof_bound_fixture_count:4,
    operator_scope_bound_fixture_count:4,
    plaintext_payload_attempt_count:1,
    public_claim_attempt_count:1,
    release_artifact_write_attempt_count:1,
    filesystem_persistence_allowed_count:0,
    command_invocation_attempt_count:5,
    command_invocation_performed_count:0,
    command_execution_performed_count:0,
    receipt_persistence_execution_performed_count:0,
    materialization_execution_performed_count:0,
    materialization_executed_count:0,
    filesystem_write_performed_count:0,
    workspace_write_performed_count:0,
    receipt_materialized_count:0,
    receipt_persisted_count:0,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    live_secret_scan_performed:false,
    pre_activation_soak_evidence_persistence_allowed:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    materialization_dry_run_denial_reason:"deterministic redacted receipt materialization plans are modeled, but every fixture remains blocked because command execution, materialization execution, receipt persistence, filesystem writes, public artifacts, plaintext payloads, and live mutation are still disabled",
    materialization_plan_required_before_execution:[
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "accepted_redaction_proof_ids",
      "redacted_payload_summary_sha256",
      "receipt_output_path_redacted",
      "receipt_materialization_plan_id",
      "receipt_persistence_approval",
      "fresh_pre_activation_soak_evidence",
      "rollback_plan_id",
      "public_claim_and_artifact_decision"
    ],
    materialization_fixtures:[
      {
        id:"materialization-without-operator-scope",
        write_enable_requested:true,
        materialization_requested:true,
        materialization_status:"blocked_dry_run",
        deterministic_materialization_plan_ready:false,
        planned_receipt_payload_hash:"redacted-summary-sha256:operator-scope-missing",
        recorded_command_field_count:12,
        accepted_redaction_proof_count:1,
        operator_identity_hash_recorded:false,
        single_surface_activation_scope_recorded:false,
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_enabled:false,
        receipt_persistence_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"operator_scope_missing"
      },
      {
        id:"operator-scoped-but-command-disabled",
        write_enable_requested:true,
        materialization_requested:true,
        materialization_status:"blocked_dry_run",
        deterministic_materialization_plan_ready:true,
        planned_receipt_payload_hash:"redacted-summary-sha256:command-disabled",
        recorded_command_field_count:12,
        accepted_redaction_proof_count:1,
        operator_identity_hash_recorded:true,
        single_surface_activation_scope_recorded:true,
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_enabled:false,
        receipt_persistence_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"receipt_command_disabled_by_default"
      },
      {
        id:"accepted-proof-but-persistence-disabled",
        write_enable_requested:true,
        materialization_requested:true,
        materialization_status:"blocked_dry_run",
        deterministic_materialization_plan_ready:true,
        planned_receipt_payload_hash:"redacted-summary-sha256:persistence-disabled",
        recorded_command_field_count:12,
        accepted_redaction_proof_count:1,
        operator_identity_hash_recorded:true,
        single_surface_activation_scope_recorded:true,
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_enabled:false,
        receipt_persistence_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"receipt_persistence_disabled"
      },
      {
        id:"plaintext-materialization-attempt",
        write_enable_requested:true,
        materialization_requested:true,
        materialization_status:"blocked_dry_run",
        deterministic_materialization_plan_ready:false,
        planned_receipt_payload_hash:"rejected:plaintext-payload-forbidden",
        recorded_command_field_count:12,
        accepted_redaction_proof_count:1,
        operator_identity_hash_recorded:true,
        single_surface_activation_scope_recorded:true,
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:false,
        raw_payload_plaintext_recorded:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_enabled:false,
        receipt_persistence_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"plaintext_payload_forbidden"
      },
      {
        id:"public-artifact-materialization-attempt",
        write_enable_requested:true,
        materialization_requested:true,
        materialization_status:"blocked_dry_run",
        deterministic_materialization_plan_ready:true,
        planned_receipt_payload_hash:"redacted-summary-sha256:public-artifact-denied",
        recorded_command_field_count:12,
        accepted_redaction_proof_count:1,
        operator_identity_hash_recorded:true,
        single_surface_activation_scope_recorded:true,
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_enabled:false,
        receipt_persistence_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"public_claim_and_release_artifact_denied"
      }
    ],
    denied_by_materialization_dry_run:[
      "operator_scope_missing",
      "receipt_command_disabled",
      "receipt_persistence_disabled",
      "plaintext_payload_materialization_denied",
      "public_claim_and_release_artifact_denied",
      "filesystem_write_denied",
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
      filesystem_written:false,
      workspace_write_performed:false,
      release_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      receipt_materialized:false,
      receipt_persisted:false,
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
  and .payload_redaction_acceptance_receipt_materialization_dry_run_ready == true
  and .payload_redaction_acceptance_receipt_write_enable_fixture_ready == true
  and .source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .payload_redaction_acceptance_receipt_command_recorded == false
  and .payload_redaction_acceptance_receipt_command_enabled_by_default == false
  and .payload_redaction_acceptance_receipt_command_invocation_requested_count == 5
  and .payload_redaction_acceptance_receipt_command_invocation_performed_count == 0
  and .payload_redaction_acceptance_receipt_command_execution_performed_count == 0
  and .payload_redaction_acceptance_receipt_recorded == false
  and .payload_redaction_acceptance_receipt_materialized == false
  and .payload_redaction_acceptance_receipt_persisted == false
  and .payload_redaction_acceptance_matrix_recorded == false
  and .payload_redaction_acceptance_matrix_persisted == false
  and .payload_redaction_proof_recorded == false
  and .payload_redaction_proof_accepted == false
  and .accepted_redaction_proof_count == 0
  and .required_materialization_fixture_count == 5
  and .materialization_fixture_count == 5
  and .blocked_materialization_fixture_count == 5
  and .allowed_materialization_fixture_count == 0
  and .explicit_write_enable_requested_fixture_count == 5
  and .materialization_denied_without_operator_scope_count == 1
  and .materialization_denied_command_disabled_count == 1
  and .materialization_denied_persistence_disabled_count == 1
  and .materialization_denied_plaintext_payload_count == 1
  and .materialization_denied_public_artifact_count == 1
  and .deterministic_materialization_plan_count == 3
  and .deterministic_materialization_plan_persisted_count == 0
  and .payload_hash_planned_fixture_count == 5
  and .redacted_payload_summary_hash_bound_fixture_count == 5
  and .redacted_output_path_planned_fixture_count == 4
  and .redacted_receipt_shape_fixture_count == 3
  and .accepted_redaction_proof_bound_fixture_count == 4
  and .operator_scope_bound_fixture_count == 4
  and .plaintext_payload_attempt_count == 1
  and .public_claim_attempt_count == 1
  and .release_artifact_write_attempt_count == 1
  and .filesystem_persistence_allowed_count == 0
  and .command_invocation_attempt_count == 5
  and .command_invocation_performed_count == 0
  and .command_execution_performed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .materialization_execution_performed_count == 0
  and .materialization_executed_count == 0
  and .filesystem_write_performed_count == 0
  and .workspace_write_performed_count == 0
  and .receipt_materialized_count == 0
  and .receipt_persisted_count == 0
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .live_secret_scan_performed == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.materialization_fixtures | length) == 5
  and (.materialization_fixtures | all(.write_enable_requested == true and .materialization_requested == true and .materialization_status == "blocked_dry_run" and .command_invocation_requested == true and .command_invocation_performed == false and .command_execution_performed == false and .receipt_persistence_execution_performed == false and .materialization_execution_performed == false and .filesystem_write_requested == true and .filesystem_write_performed == false and .receipt_materialized == false and .receipt_persisted == false and .activation_allowed == false))
  and ([.materialization_fixtures[] | select(.deterministic_materialization_plan_ready == true)] | length) == 3
  and ([.materialization_fixtures[] | select(.raw_payload_plaintext_recorded == true)] | length) == 1
  and ([.materialization_fixtures[] | select(.public_claim_requested == true and .release_artifact_write_requested == true)] | length) == 1
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt materialization dry-run gate passed"
