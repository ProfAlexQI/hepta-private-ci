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

ACCEPTANCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-gate.sh
)"

acceptance_report_sha256="$(
  printf '%s' "$ACCEPTANCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
readiness_condition_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness:conditions:$acceptance_report_sha256"
)"
readiness_denial_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness:denials:$acceptance_report_sha256"
)"
readiness_policy_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness:policy:$acceptance_report_sha256"
)"
readiness_side_effect_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness:side-effects:$acceptance_report_sha256"
)"

jq -n -e \
  --argjson acceptance "$ACCEPTANCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $acceptance.runtime == "hepta"
    and $acceptance.status == "ready"
    and $acceptance.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_gate"
    and $acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_ready == true
    and $acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_ready == true
    and $acceptance.required_scoreboard_review_acceptance_field_count == 20
    and $acceptance.scoreboard_review_acceptance_field_count == 20
    and $acceptance.recorded_scoreboard_review_acceptance_field_count == 0
    and $acceptance.scoreboard_review_acceptance_fixture_count == 4
    and $acceptance.blocked_scoreboard_review_acceptance_fixture_count == 4
    and $acceptance.allowed_scoreboard_review_acceptance_fixture_count == 0
    and $acceptance.scoreboard_review_acceptance_performed_count == 0
    and $acceptance.scoreboard_review_acceptance_recorded_count == 0
    and $acceptance.scoreboard_review_acceptance_persisted_count == 0
    and $acceptance.scoreboard_review_acceptance_materialized_count == 0
    and $acceptance.scoreboard_review_acceptance_filesystem_written_count == 0
    and $acceptance.accepted_scoreboard_review_count == 0
    and $acceptance.scoreboard_review_acceptance_policy_satisfied_count == 0
    and $acceptance.scoreboard_review_acceptance_allowed_count == 0
    and $acceptance.operator_approval_recorded == false
    and $acceptance.fresh_pre_activation_soak_evidence_recorded == false
    and $acceptance.accepted_redaction_proof_recorded == false
    and $acceptance.rollback_rehearsal_evidence_recorded == false
    and $acceptance.public_artifact_decision_recorded == false
    and $acceptance.filesystem_write_performed == false
    and $acceptance.workspace_write_performed == false
    and $acceptance.public_claim_allowed == false
    and $acceptance.release_artifact_write_allowed == false
    and $acceptance.receipt_persistence_enabled == false
    and $acceptance.activation_allowed == false
    and $acceptance.live_mutation_execution_ready == false
    and ($acceptance.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_gate" \
  --arg acceptance_report_sha256 "$acceptance_report_sha256" \
  --arg readiness_condition_hash_sha256 "$readiness_condition_hash_sha256" \
  --arg readiness_denial_hash_sha256 "$readiness_denial_hash_sha256" \
  --arg readiness_policy_hash_sha256 "$readiness_policy_hash_sha256" \
  --arg readiness_side_effect_hash_sha256 "$readiness_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson acceptance "$ACCEPTANCE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    readiness_schema_version:"payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_v1",
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_gate:$acceptance.gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_ready:$acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_ready,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_report_sha256:$acceptance_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256,
    source_pre_activation_soak_report_sha256:$acceptance.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$acceptance.source_persistence_denial_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_ready:true,
    readiness_mode:"schema_only_activation_blocked",
    readiness_decision:"not_ready_for_live_mutation",
    required_readiness_condition_count:12,
    readiness_condition_count:12,
    satisfied_readiness_condition_count:0,
    blocked_readiness_condition_count:12,
    recorded_readiness_field_count:0,
    readiness_recorded:false,
    readiness_persisted:false,
    readiness_materialized:false,
    readiness_filesystem_written:false,
    readiness_condition_hash_sha256:$readiness_condition_hash_sha256,
    readiness_denial_hash_sha256:$readiness_denial_hash_sha256,
    readiness_policy_hash_sha256:$readiness_policy_hash_sha256,
    readiness_side_effect_hash_sha256:$readiness_side_effect_hash_sha256,
    required_scoreboard_review_acceptance_field_count:$acceptance.required_scoreboard_review_acceptance_field_count,
    recorded_scoreboard_review_acceptance_field_count:$acceptance.recorded_scoreboard_review_acceptance_field_count,
    scoreboard_review_acceptance_fixture_count:$acceptance.scoreboard_review_acceptance_fixture_count,
    blocked_scoreboard_review_acceptance_fixture_count:$acceptance.blocked_scoreboard_review_acceptance_fixture_count,
    allowed_scoreboard_review_acceptance_fixture_count:$acceptance.allowed_scoreboard_review_acceptance_fixture_count,
    scoreboard_review_acceptance_performed_count:$acceptance.scoreboard_review_acceptance_performed_count,
    scoreboard_review_acceptance_recorded_count:$acceptance.scoreboard_review_acceptance_recorded_count,
    scoreboard_review_acceptance_persisted_count:$acceptance.scoreboard_review_acceptance_persisted_count,
    scoreboard_review_acceptance_materialized_count:$acceptance.scoreboard_review_acceptance_materialized_count,
    scoreboard_review_acceptance_filesystem_written_count:$acceptance.scoreboard_review_acceptance_filesystem_written_count,
    accepted_scoreboard_review_count:$acceptance.accepted_scoreboard_review_count,
    scoreboard_review_acceptance_policy_satisfied_count:$acceptance.scoreboard_review_acceptance_policy_satisfied_count,
    scoreboard_review_acceptance_allowed_count:$acceptance.scoreboard_review_acceptance_allowed_count,
    operator_approval_recorded:false,
    single_surface_activation_scope_recorded:false,
    fresh_pre_activation_soak_evidence_recorded:false,
    active_binary_sha_recorded:false,
    trusted_source_binding_recorded:false,
    accepted_redaction_proof_recorded:false,
    accepted_scoreboard_review_recorded:false,
    rollback_rehearsal_evidence_recorded:false,
    output_path_selection_recorded:false,
    ledger_persistence_approval_recorded:false,
    receipt_persistence_approval_recorded:false,
    public_artifact_decision_recorded:false,
    readiness_allowed:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    command_invocation_performed_count:0,
    command_execution_performed_count:0,
    materialization_execution_performed_count:0,
    receipt_persistence_execution_performed_count:0,
    ledger_persistence_execution_performed_count:0,
    filesystem_persistence_execution_performed_count:0,
    filesystem_write_performed:false,
    workspace_write_performed:false,
    receipt_persisted_count:0,
    ledger_persisted:false,
    scoreboard_persisted_count:0,
    public_claim_allowed:false,
    release_artifact_write_allowed:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    raw_payload_inspected:false,
    live_secret_scan_performed:false,
    required_readiness_conditions:[
      "operator_approval_recorded",
      "single_surface_activation_scope_recorded",
      "fresh_pre_activation_soak_evidence_recorded",
      "active_binary_sha_recorded",
      "trusted_source_binding_recorded",
      "accepted_redaction_proof_recorded",
      "accepted_scoreboard_review_recorded",
      "rollback_rehearsal_evidence_recorded",
      "output_path_selection_recorded",
      "ledger_persistence_approval_recorded",
      "receipt_persistence_approval_recorded",
      "public_claim_and_artifact_decision_recorded"
    ],
    readiness_conditions:[
      {id:"operator-approval", satisfied:false, blocks_activation:true},
      {id:"single-surface-scope", satisfied:false, blocks_activation:true},
      {id:"fresh-pre-activation-soak", satisfied:false, blocks_activation:true},
      {id:"active-binary-sha", satisfied:false, blocks_activation:true},
      {id:"trusted-source-binding", satisfied:false, blocks_activation:true},
      {id:"accepted-redaction-proof", satisfied:false, blocks_activation:true},
      {id:"accepted-scoreboard-review", satisfied:false, blocks_activation:true},
      {id:"rollback-rehearsal-evidence", satisfied:false, blocks_activation:true},
      {id:"output-path-selection", satisfied:false, blocks_activation:true},
      {id:"ledger-persistence-approval", satisfied:false, blocks_activation:true},
      {id:"receipt-persistence-approval", satisfied:false, blocks_activation:true},
      {id:"public-claim-and-artifact-decision", satisfied:false, blocks_activation:true}
    ],
    denied_by_readiness_gate:[
      "operator_approval_missing",
      "single_surface_activation_scope_missing",
      "fresh_pre_activation_soak_evidence_missing",
      "active_binary_sha_missing",
      "trusted_source_binding_missing",
      "accepted_redaction_proof_missing",
      "accepted_scoreboard_review_missing",
      "rollback_rehearsal_evidence_missing",
      "output_path_selection_missing",
      "ledger_persistence_approval_missing",
      "receipt_persistence_approval_missing",
      "public_claim_and_artifact_decision_missing",
      "readiness_recording_denied",
      "readiness_persistence_denied",
      "activation_denied",
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
      materialization_execution_performed:false,
      receipt_persistence_execution_performed:false,
      ledger_persistence_execution_performed:false,
      filesystem_persistence_execution_performed:false,
      readiness_recorded:false,
      readiness_persisted:false,
      readiness_materialized:false,
      readiness_filesystem_written:false,
      scoreboard_review_acceptance_performed:false,
      scoreboard_review_acceptance_recorded:false,
      scoreboard_review_acceptance_persisted:false,
      scoreboard_review_acceptance_materialized:false,
      scoreboard_review_acceptance_filesystem_written:false,
      scoreboard_review_performed:false,
      scoreboard_review_recorded:false,
      scoreboard_review_persisted:false,
      scoreboard_review_materialized:false,
      scoreboard_review_filesystem_written:false,
      scoreboard_recorded:false,
      scoreboard_persisted:false,
      scoreboard_materialized:false,
      scoreboard_filesystem_written:false,
      rehearsal_receipt_review_acceptance_recorded:false,
      rehearsal_receipt_review_acceptance_persisted:false,
      rehearsal_receipt_review_acceptance_materialized:false,
      rehearsal_receipt_review_acceptance_filesystem_written:false,
      ledger_recorded:false,
      ledger_persisted:false,
      ledger_materialized:false,
      ledger_filesystem_written:false,
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
      output_path_selected:false,
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
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_readiness_condition_count == 12
  and .readiness_condition_count == 12
  and .satisfied_readiness_condition_count == 0
  and .blocked_readiness_condition_count == 12
  and .recorded_readiness_field_count == 0
  and .readiness_recorded == false
  and .readiness_persisted == false
  and .readiness_materialized == false
  and .readiness_filesystem_written == false
  and .required_scoreboard_review_acceptance_field_count == 20
  and .recorded_scoreboard_review_acceptance_field_count == 0
  and .scoreboard_review_acceptance_fixture_count == 4
  and .blocked_scoreboard_review_acceptance_fixture_count == 4
  and .allowed_scoreboard_review_acceptance_fixture_count == 0
  and .scoreboard_review_acceptance_performed_count == 0
  and .scoreboard_review_acceptance_recorded_count == 0
  and .scoreboard_review_acceptance_persisted_count == 0
  and .scoreboard_review_acceptance_materialized_count == 0
  and .scoreboard_review_acceptance_filesystem_written_count == 0
  and .accepted_scoreboard_review_count == 0
  and .scoreboard_review_acceptance_policy_satisfied_count == 0
  and .scoreboard_review_acceptance_allowed_count == 0
  and .operator_approval_recorded == false
  and .single_surface_activation_scope_recorded == false
  and .fresh_pre_activation_soak_evidence_recorded == false
  and .active_binary_sha_recorded == false
  and .trusted_source_binding_recorded == false
  and .accepted_redaction_proof_recorded == false
  and .accepted_scoreboard_review_recorded == false
  and .rollback_rehearsal_evidence_recorded == false
  and .output_path_selection_recorded == false
  and .ledger_persistence_approval_recorded == false
  and .receipt_persistence_approval_recorded == false
  and .public_artifact_decision_recorded == false
  and .readiness_allowed == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .command_invocation_performed_count == 0
  and .command_execution_performed_count == 0
  and .materialization_execution_performed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .ledger_persistence_execution_performed_count == 0
  and .filesystem_persistence_execution_performed_count == 0
  and .filesystem_write_performed == false
  and .workspace_write_performed == false
  and .receipt_persisted_count == 0
  and .ledger_persisted == false
  and .scoreboard_persisted_count == 0
  and .public_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .raw_payload_inspected == false
  and .live_secret_scan_performed == false
  and .readiness_condition_hash_sha256 != ""
  and .readiness_denial_hash_sha256 != ""
  and .readiness_policy_hash_sha256 != ""
  and .readiness_side_effect_hash_sha256 != ""
  and (.required_readiness_conditions | length) == 12
  and (.readiness_conditions | length) == 12
  and (.readiness_conditions | all(.satisfied == false and .blocks_activation == true))
  and (.denied_by_readiness_gate | length) == 16
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance readiness gate passed"
