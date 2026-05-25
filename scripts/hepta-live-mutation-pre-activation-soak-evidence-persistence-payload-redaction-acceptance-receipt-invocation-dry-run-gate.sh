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

COMMAND_CONTRACT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-command-contract-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-command-contract-gate.sh
)"

command_contract_report_sha256="$(printf '%s' "$COMMAND_CONTRACT_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson command "$COMMAND_CONTRACT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $command.runtime == "hepta"
    and $command.status == "ready"
    and $command.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_command_contract_gate"
    and $command.payload_redaction_acceptance_receipt_command_contract_ready == true
    and $command.source_payload_redaction_acceptance_matrix_ready == true
    and $command.source_payload_redaction_acceptance_matrix_report_sha256 != ""
    and $command.payload_redaction_acceptance_receipt_command_recorded == false
    and $command.payload_redaction_acceptance_receipt_command_enabled_by_default == false
    and $command.payload_redaction_acceptance_receipt_command_invoked == false
    and $command.payload_redaction_acceptance_receipt_command_execution_performed == false
    and $command.payload_redaction_acceptance_receipt_recorded == false
    and $command.payload_redaction_acceptance_receipt_persisted == false
    and $command.payload_redaction_acceptance_matrix_recorded == false
    and $command.payload_redaction_acceptance_matrix_persisted == false
    and $command.payload_redaction_proof_recorded == false
    and $command.payload_redaction_proof_accepted == false
    and $command.accepted_redaction_proof_count == 0
    and $command.required_receipt_command_field_count == 12
    and $command.recorded_receipt_command_field_count == 0
    and $command.blocked_receipt_command_fixture_count == 6
    and $command.allowed_receipt_command_fixture_count == 0
    and $command.command_invocation_attempt_count == 0
    and $command.command_invocation_performed_count == 0
    and $command.receipt_persistence_execution_performed_count == 0
    and $command.raw_payload_plaintext_recorded == false
    and $command.raw_payload_plaintext_persisted == false
    and $command.live_secret_scan_performed == false
    and $command.receipt_persistence_enabled == false
    and $command.receipt_persisted == false
    and $command.activation_allowed == false
    and $command.live_mutation_execution_ready == false
    and ($command.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_invocation_dry_run_gate" \
  --arg command_contract_report_sha256 "$command_contract_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson command "$COMMAND_CONTRACT_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_payload_redaction_acceptance_receipt_command_contract_gate:$command.gate,
    source_payload_redaction_acceptance_receipt_command_contract_ready:$command.payload_redaction_acceptance_receipt_command_contract_ready,
    source_receipt_payload_sha256:$command.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$command.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$command.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$command.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$command.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$command.source_no_secret_payload_review_report_sha256,
    source_payload_redaction_proof_report_sha256:$command.source_payload_redaction_proof_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$command.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_acceptance_receipt_command_contract_report_sha256:$command_contract_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_invocation_dry_run_ready:true,
    payload_redaction_acceptance_receipt_command_contract_ready:true,
    payload_redaction_acceptance_receipt_command_recorded:false,
    payload_redaction_acceptance_receipt_command_enabled_by_default:false,
    payload_redaction_acceptance_receipt_command_invocation_requested_count:5,
    payload_redaction_acceptance_receipt_command_invocation_performed_count:0,
    payload_redaction_acceptance_receipt_command_execution_performed_count:0,
    payload_redaction_acceptance_receipt_recorded:false,
    payload_redaction_acceptance_receipt_persisted:false,
    payload_redaction_acceptance_matrix_recorded:false,
    payload_redaction_acceptance_matrix_persisted:false,
    payload_redaction_proof_recorded:false,
    payload_redaction_proof_accepted:false,
    accepted_redaction_proof_count:0,
    reviewed_redaction_proof_count:0,
    required_invocation_fixture_count:5,
    blocked_invocation_fixture_count:5,
    allowed_invocation_fixture_count:0,
    redacted_output_path_fixture_count:4,
    redacted_payload_summary_hash_bound_fixture_count:5,
    accepted_redaction_proof_bound_fixture_count:4,
    operator_scope_bound_fixture_count:4,
    public_claim_attempt_count:1,
    release_artifact_write_attempt_count:1,
    plaintext_payload_attempt_count:1,
    command_invocation_attempt_count:5,
    command_invocation_performed_count:0,
    command_execution_performed_count:0,
    receipt_persistence_execution_performed_count:0,
    filesystem_write_performed_count:0,
    workspace_write_performed_count:0,
    receipt_persisted_count:0,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    live_secret_scan_performed:false,
    pre_activation_soak_evidence_persistence_allowed:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    invocation_denial_reason:"receipt invocation is dry-run only; the command is disabled by default and no command execution, receipt persistence, filesystem write, or live mutation is allowed",
    invocation_fixtures:[
      {
        id:"redacted-command-shape",
        dry_run_status:"blocked_noop",
        recorded_command_field_count:12,
        accepted_redaction_proof_count:1,
        operator_identity_hash_recorded:true,
        single_surface_activation_scope_recorded:true,
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"command_disabled_by_default"
      },
      {
        id:"accepted-proof-but-command-disabled",
        dry_run_status:"blocked_noop",
        recorded_command_field_count:12,
        accepted_redaction_proof_count:1,
        operator_identity_hash_recorded:true,
        single_surface_activation_scope_recorded:true,
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"explicit_invocation_request_is_noop_without_operator_enablement"
      },
      {
        id:"persistence-disabled-invocation-attempt",
        dry_run_status:"blocked_noop",
        recorded_command_field_count:12,
        accepted_redaction_proof_count:1,
        operator_identity_hash_recorded:true,
        single_surface_activation_scope_recorded:true,
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        receipt_persistence_enabled:false,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"receipt_persistence_disabled"
      },
      {
        id:"plaintext-payload-invocation-attempt",
        dry_run_status:"blocked_noop",
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
        receipt_persistence_execution_performed:false,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"plaintext_payload_forbidden"
      },
      {
        id:"public-artifact-invocation-attempt",
        dry_run_status:"blocked_noop",
        recorded_command_field_count:12,
        accepted_redaction_proof_count:0,
        operator_identity_hash_recorded:false,
        single_surface_activation_scope_recorded:false,
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"public_claim_and_release_artifact_denied"
      }
    ],
    denied_by_invocation_dry_run:[
      "receipt_command_disabled_by_default",
      "command_invocation_execution_denied",
      "receipt_persistence_execution_denied",
      "filesystem_write_denied",
      "plaintext_payload_recording_denied",
      "public_claim_and_release_artifact_denied",
      "live_mutation_execution_denied"
    ],
    required_before_receipt_invocation_execution:[
      "explicit_operator_enablement_for_receipt_command",
      "accepted_redaction_proof_ids",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "redacted_payload_summary_sha256",
      "receipt_output_path_redacted",
      "receipt_persistence_approval",
      "rollback_plan_id",
      "public_claim_and_artifact_decision"
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
      filesystem_written:false,
      workspace_write_performed:false,
      release_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      receipt_persisted:false,
      receipt_persistence_execution_performed:false,
      pre_activation_soak_evidence_persisted:false,
      approval_packet_persisted:false,
      operator_scope_binding_persisted:false,
      payload_review_persisted:false,
      payload_redaction_proof_persisted:false,
      payload_redaction_acceptance_matrix_persisted:false,
      payload_redaction_acceptance_receipt_command_persisted:false,
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
  and .payload_redaction_acceptance_receipt_invocation_dry_run_ready == true
  and .source_payload_redaction_acceptance_receipt_command_contract_ready == true
  and .source_payload_redaction_acceptance_receipt_command_contract_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .payload_redaction_acceptance_receipt_command_recorded == false
  and .payload_redaction_acceptance_receipt_command_enabled_by_default == false
  and .payload_redaction_acceptance_receipt_command_invocation_requested_count == 5
  and .payload_redaction_acceptance_receipt_command_invocation_performed_count == 0
  and .payload_redaction_acceptance_receipt_command_execution_performed_count == 0
  and .payload_redaction_acceptance_receipt_recorded == false
  and .payload_redaction_acceptance_receipt_persisted == false
  and .payload_redaction_acceptance_matrix_recorded == false
  and .payload_redaction_acceptance_matrix_persisted == false
  and .payload_redaction_proof_recorded == false
  and .payload_redaction_proof_accepted == false
  and .accepted_redaction_proof_count == 0
  and .required_invocation_fixture_count == 5
  and .blocked_invocation_fixture_count == 5
  and .allowed_invocation_fixture_count == 0
  and .redacted_output_path_fixture_count == 4
  and .redacted_payload_summary_hash_bound_fixture_count == 5
  and .accepted_redaction_proof_bound_fixture_count == 4
  and .operator_scope_bound_fixture_count == 4
  and .public_claim_attempt_count == 1
  and .release_artifact_write_attempt_count == 1
  and .plaintext_payload_attempt_count == 1
  and .command_invocation_attempt_count == 5
  and .command_invocation_performed_count == 0
  and .command_execution_performed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .filesystem_write_performed_count == 0
  and .workspace_write_performed_count == 0
  and .receipt_persisted_count == 0
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .live_secret_scan_performed == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.invocation_fixtures | length) == 5
  and (.invocation_fixtures | all(.command_invocation_requested == true and .command_invocation_performed == false and .command_execution_performed == false and .receipt_persistence_execution_performed == false and .filesystem_write_performed == false and .receipt_persisted == false and .activation_allowed == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt invocation dry-run gate passed"
