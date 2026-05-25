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

ACCEPTANCE_MATRIX_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-matrix-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-matrix-gate.sh
)"

acceptance_matrix_report_sha256="$(printf '%s' "$ACCEPTANCE_MATRIX_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson matrix "$ACCEPTANCE_MATRIX_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $matrix.runtime == "hepta"
    and $matrix.status == "ready"
    and $matrix.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_matrix_gate"
    and $matrix.payload_redaction_acceptance_matrix_ready == true
    and $matrix.source_payload_redaction_proof_ready == true
    and $matrix.payload_redaction_acceptance_matrix_recorded == false
    and $matrix.payload_redaction_proof_recorded == false
    and $matrix.payload_redaction_proof_accepted == false
    and $matrix.accepted_redaction_proof_count == 0
    and $matrix.reviewed_redaction_proof_count == 0
    and $matrix.blocked_redaction_acceptance_fixture_count == 6
    and $matrix.required_acceptance_check_count_per_proof == 8
    and $matrix.satisfied_acceptance_check_count == 0
    and $matrix.payload_review_persisted == false
    and $matrix.payload_redaction_proof_persisted == false
    and $matrix.payload_redaction_acceptance_matrix_persisted == false
    and $matrix.raw_payload_plaintext_recorded == false
    and $matrix.raw_payload_plaintext_persisted == false
    and $matrix.live_secret_scan_performed == false
    and $matrix.receipt_persistence_enabled == false
    and $matrix.receipt_persisted == false
    and $matrix.activation_allowed == false
    and $matrix.live_mutation_execution_ready == false
    and ($matrix.acceptance_matrix_fixtures | length) == 6
    and ($matrix.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_command_contract_gate" \
  --arg acceptance_matrix_report_sha256 "$acceptance_matrix_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson matrix "$ACCEPTANCE_MATRIX_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_payload_redaction_acceptance_matrix_gate:$matrix.gate,
    source_payload_redaction_acceptance_matrix_ready:$matrix.payload_redaction_acceptance_matrix_ready,
    source_receipt_payload_sha256:$matrix.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$matrix.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$matrix.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$matrix.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$matrix.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$matrix.source_no_secret_payload_review_report_sha256,
    source_payload_redaction_proof_report_sha256:$matrix.source_payload_redaction_proof_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$acceptance_matrix_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_command_contract_ready:true,
    payload_redaction_acceptance_receipt_command_recorded:false,
    payload_redaction_acceptance_receipt_command_enabled_by_default:false,
    payload_redaction_acceptance_receipt_command_invoked:false,
    payload_redaction_acceptance_receipt_command_execution_performed:false,
    payload_redaction_acceptance_receipt_recorded:false,
    payload_redaction_acceptance_receipt_persisted:false,
    payload_redaction_acceptance_matrix_recorded:false,
    payload_redaction_acceptance_matrix_persisted:false,
    payload_redaction_proof_recorded:false,
    payload_redaction_proof_accepted:false,
    accepted_redaction_proof_count:0,
    reviewed_redaction_proof_count:0,
    required_receipt_command_field_count:12,
    recorded_receipt_command_field_count:0,
    redacted_or_hashed_field_count:10,
    blocked_receipt_command_fixture_count:6,
    allowed_receipt_command_fixture_count:0,
    command_invocation_attempt_count:0,
    command_invocation_performed_count:0,
    receipt_persistence_execution_performed_count:0,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    live_secret_scan_performed:false,
    pre_activation_soak_evidence_persistence_allowed:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    command_denial_reason:"acceptance receipt command is schema-only; no accepted redaction proof, recorded approval, command invocation, receipt persistence, or live mutation exists",
    required_receipt_command_fields:[
      "payload_redaction_acceptance_receipt_command_id",
      "payload_redaction_acceptance_matrix_id",
      "payload_redaction_proof_id",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "source_payload_redaction_acceptance_matrix_report_sha256",
      "accepted_redaction_proof_ids",
      "redacted_payload_summary_sha256",
      "receipt_output_path_redacted",
      "rollback_plan_id",
      "public_claim_and_artifact_decision"
    ],
    denied_receipt_command_fixtures:[
      {
        id:"schema-only-no-command",
        recorded_command_field_count:0,
        command_accepted:false,
        command_invocation_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"receipt_command_not_recorded"
      },
      {
        id:"command-without-accepted-proof",
        recorded_command_field_count:5,
        accepted_redaction_proof_count:0,
        command_accepted:false,
        command_invocation_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"accepted_redaction_proof_required"
      },
      {
        id:"command-without-operator-scope",
        recorded_command_field_count:8,
        operator_identity_hash_recorded:false,
        single_surface_activation_scope_recorded:false,
        command_accepted:false,
        command_invocation_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"operator_identity_and_single_surface_scope_required"
      },
      {
        id:"command-with-plaintext-payload",
        recorded_command_field_count:10,
        raw_payload_plaintext_recorded:true,
        command_accepted:false,
        command_invocation_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"plaintext_payload_forbidden"
      },
      {
        id:"command-without-redacted-output-path",
        recorded_command_field_count:11,
        receipt_output_path_redacted:false,
        command_accepted:false,
        command_invocation_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"redacted_receipt_output_path_required"
      },
      {
        id:"public-artifact-command-attempt",
        recorded_command_field_count:12,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        command_accepted:false,
        command_invocation_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"public_claim_and_release_artifact_denied"
      }
    ],
    denied_by_acceptance_receipt_command_contract:[
      "receipt_command_not_recorded",
      "payload_redaction_acceptance_matrix_not_recorded",
      "payload_redaction_proof_not_accepted",
      "operator_identity_not_recorded",
      "single_surface_activation_scope_not_recorded",
      "redacted_payload_summary_hash_not_recorded",
      "receipt_output_path_not_redacted",
      "plaintext_payload_recording_denied",
      "public_claim_and_release_artifact_denied"
    ],
    required_before_receipt_command_invocation:[
      "payload_redaction_acceptance_receipt_command_id",
      "payload_redaction_acceptance_matrix_id",
      "payload_redaction_proof_id",
      "accepted_redaction_proof_ids",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "source_payload_redaction_acceptance_matrix_report_sha256",
      "redacted_payload_summary_sha256",
      "receipt_output_path_redacted",
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
      filesystem_written:false,
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
  and .payload_redaction_acceptance_receipt_command_contract_ready == true
  and .source_payload_redaction_acceptance_matrix_ready == true
  and .source_receipt_payload_sha256 != ""
  and .source_pre_activation_soak_report_sha256 != ""
  and .source_persistence_denial_report_sha256 != ""
  and .source_approval_packet_report_sha256 != ""
  and .source_operator_scope_report_sha256 != ""
  and .source_no_secret_payload_review_report_sha256 != ""
  and .source_payload_redaction_proof_report_sha256 != ""
  and .source_payload_redaction_acceptance_matrix_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .payload_redaction_acceptance_receipt_command_recorded == false
  and .payload_redaction_acceptance_receipt_command_enabled_by_default == false
  and .payload_redaction_acceptance_receipt_command_invoked == false
  and .payload_redaction_acceptance_receipt_command_execution_performed == false
  and .payload_redaction_acceptance_receipt_recorded == false
  and .payload_redaction_acceptance_receipt_persisted == false
  and .payload_redaction_acceptance_matrix_recorded == false
  and .payload_redaction_acceptance_matrix_persisted == false
  and .payload_redaction_proof_recorded == false
  and .payload_redaction_proof_accepted == false
  and .accepted_redaction_proof_count == 0
  and .reviewed_redaction_proof_count == 0
  and .required_receipt_command_field_count == 12
  and .recorded_receipt_command_field_count == 0
  and .blocked_receipt_command_fixture_count == 6
  and .allowed_receipt_command_fixture_count == 0
  and .command_invocation_attempt_count == 0
  and .command_invocation_performed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .live_secret_scan_performed == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.required_receipt_command_fields | length) == 12
  and (.denied_receipt_command_fixtures | length) == 6
  and (.denied_receipt_command_fixtures | all(.command_accepted == false and .command_invocation_performed == false and .receipt_persisted == false and .activation_allowed == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt command contract gate passed"
