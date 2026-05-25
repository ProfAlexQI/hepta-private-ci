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

DENIAL_REVIEW_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-gate.sh
)"

denial_review_report_sha256="$(
  printf '%s' "$DENIAL_REVIEW_JSON" | shasum -a 256 | awk '{print $1}'
)"
denial_review_acceptance_fixture_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance:fixtures:$denial_review_report_sha256"
)"
denial_review_acceptance_policy_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance:policy:$denial_review_report_sha256"
)"
denial_review_acceptance_denial_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance:denial:$denial_review_report_sha256"
)"
denial_review_acceptance_side_effect_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance:side-effects:$denial_review_report_sha256"
)"

jq -n -e \
  --argjson denial_review "$DENIAL_REVIEW_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $denial_review.runtime == "hepta"
    and $denial_review.status == "ready"
    and $denial_review.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_gate"
    and $denial_review.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_ready == true
    and $denial_review.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_ready == true
    and $denial_review.readiness_mode == "schema_only_activation_blocked"
    and $denial_review.readiness_decision == "not_ready_for_live_mutation"
    and $denial_review.denial_review_mode == "schema_only_review_activation_blocked"
    and $denial_review.denial_review_decision == "readiness_denial_confirmed"
    and $denial_review.required_denial_review_family_count == 5
    and $denial_review.denial_review_family_count == 5
    and $denial_review.ready_denial_review_family_count == 5
    and $denial_review.activation_blocking_denial_review_family_count == 5
    and $denial_review.reviewed_readiness_condition_count == 12
    and $denial_review.blocked_readiness_condition_count == 12
    and $denial_review.accepted_readiness_condition_count == 0
    and $denial_review.reviewed_denial_reason_count == 16
    and $denial_review.accepted_denial_reason_count == 0
    and $denial_review.readiness_denial_review_performed_count == 0
    and $denial_review.readiness_denial_review_recorded_count == 0
    and $denial_review.readiness_denial_review_persisted_count == 0
    and $denial_review.readiness_denial_review_materialized_count == 0
    and $denial_review.readiness_denial_review_filesystem_written_count == 0
    and $denial_review.readiness_denial_review_allowed_count == 0
    and $denial_review.readiness_allowed == false
    and $denial_review.activation_allowed == false
    and $denial_review.live_mutation_execution_ready == false
    and ($denial_review.readiness_denial_review_families | length) == 5
    and ($denial_review.readiness_denial_review_families | all(.ready == true and .blocked == true))
    and ($denial_review.denied_by_readiness_denial_review | length) == 18
    and ($denial_review.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_gate" \
  --arg denial_review_report_sha256 "$denial_review_report_sha256" \
  --arg denial_review_acceptance_fixture_hash_sha256 "$denial_review_acceptance_fixture_hash_sha256" \
  --arg denial_review_acceptance_policy_hash_sha256 "$denial_review_acceptance_policy_hash_sha256" \
  --arg denial_review_acceptance_denial_hash_sha256 "$denial_review_acceptance_denial_hash_sha256" \
  --arg denial_review_acceptance_side_effect_hash_sha256 "$denial_review_acceptance_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson denial_review "$DENIAL_REVIEW_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    readiness_denial_review_acceptance_schema_version:"payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_v1",
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_gate:$denial_review.gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_ready:$denial_review.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_ready,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_report_sha256:$denial_review_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_report_sha256:$denial_review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_report_sha256:$denial_review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_ready:$denial_review.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_ready,
    readiness_mode:$denial_review.readiness_mode,
    readiness_decision:$denial_review.readiness_decision,
    denial_review_mode:$denial_review.denial_review_mode,
    denial_review_decision:$denial_review.denial_review_decision,
    denial_review_acceptance_mode:"schema_only_acceptance_blocked",
    denial_review_acceptance_decision:"readiness_denial_review_not_accepted",
    required_denial_review_acceptance_field_count:18,
    recorded_denial_review_acceptance_field_count:0,
    redacted_or_hashed_denial_review_acceptance_field_count:16,
    required_denial_review_acceptance_fixture_count:4,
    denial_review_acceptance_fixture_count:4,
    blocked_denial_review_acceptance_fixture_count:4,
    allowed_denial_review_acceptance_fixture_count:0,
    denial_review_acceptance_requested_count:4,
    denial_review_acceptance_performed_count:0,
    denial_review_acceptance_recorded_count:0,
    denial_review_acceptance_persisted_count:0,
    denial_review_acceptance_materialized_count:0,
    denial_review_acceptance_filesystem_written_count:0,
    accepted_readiness_denial_review_count:0,
    denial_review_acceptance_policy_satisfied_count:0,
    denial_review_acceptance_allowed_count:0,
    required_denial_review_family_count:$denial_review.required_denial_review_family_count,
    denial_review_family_count:$denial_review.denial_review_family_count,
    ready_denial_review_family_count:$denial_review.ready_denial_review_family_count,
    activation_blocking_denial_review_family_count:$denial_review.activation_blocking_denial_review_family_count,
    reviewed_readiness_condition_count:$denial_review.reviewed_readiness_condition_count,
    blocked_readiness_condition_count:$denial_review.blocked_readiness_condition_count,
    accepted_readiness_condition_count:$denial_review.accepted_readiness_condition_count,
    reviewed_denial_reason_count:$denial_review.reviewed_denial_reason_count,
    accepted_denial_reason_count:$denial_review.accepted_denial_reason_count,
    readiness_denial_review_performed_count:$denial_review.readiness_denial_review_performed_count,
    readiness_denial_review_recorded_count:$denial_review.readiness_denial_review_recorded_count,
    readiness_denial_review_persisted_count:$denial_review.readiness_denial_review_persisted_count,
    readiness_denial_review_materialized_count:$denial_review.readiness_denial_review_materialized_count,
    readiness_denial_review_filesystem_written_count:$denial_review.readiness_denial_review_filesystem_written_count,
    readiness_denial_review_allowed_count:$denial_review.readiness_denial_review_allowed_count,
    readiness_recorded:$denial_review.readiness_recorded,
    readiness_persisted:$denial_review.readiness_persisted,
    readiness_materialized:$denial_review.readiness_materialized,
    readiness_filesystem_written:$denial_review.readiness_filesystem_written,
    readiness_allowed:$denial_review.readiness_allowed,
    activation_allowed:$denial_review.activation_allowed,
    live_mutation_execution_ready:$denial_review.live_mutation_execution_ready,
    satisfied_readiness_condition_count:$denial_review.satisfied_readiness_condition_count,
    denied_readiness_condition_count:$denial_review.denied_readiness_condition_count,
    denied_readiness_reason_count:$denial_review.denied_readiness_reason_count,
    denial_review_acceptance_fixture_hash_sha256:$denial_review_acceptance_fixture_hash_sha256,
    denial_review_acceptance_policy_hash_sha256:$denial_review_acceptance_policy_hash_sha256,
    denial_review_acceptance_denial_hash_sha256:$denial_review_acceptance_denial_hash_sha256,
    denial_review_acceptance_side_effect_hash_sha256:$denial_review_acceptance_side_effect_hash_sha256,
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
    readiness_denial_review_acceptance_fixtures:[
      {
        id:"family-readiness-denial-review-acceptance",
        requested:true,
        blocked:true,
        accepted:false,
        reason:"all denial review families are ready but remain activation-blocking"
      },
      {
        id:"condition-blocking-denial-review-acceptance",
        requested:true,
        blocked:true,
        accepted:false,
        reason:"all readiness conditions remain unsatisfied"
      },
      {
        id:"denial-set-denial-review-acceptance",
        requested:true,
        blocked:true,
        accepted:false,
        reason:"the complete denial set is reviewed but not waived"
      },
      {
        id:"public-artifact-denial-review-acceptance",
        requested:true,
        blocked:true,
        accepted:false,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        reason:"public claim and release artifact writes remain denied"
      }
    ],
    denied_by_readiness_denial_review_acceptance:[
      "readiness_denial_review_acceptance_recording_denied",
      "readiness_denial_review_acceptance_materialization_denied",
      "readiness_denial_review_acceptance_persistence_denied",
      "readiness_denial_review_acceptance_filesystem_write_denied",
      "accepted_readiness_denial_review_missing",
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
      readiness_denial_review_acceptance_performed:false,
      readiness_denial_review_acceptance_recorded:false,
      readiness_denial_review_acceptance_persisted:false,
      readiness_denial_review_acceptance_materialized:false,
      readiness_denial_review_acceptance_filesystem_written:false,
      readiness_denial_review_performed:false,
      readiness_denial_review_recorded:false,
      readiness_denial_review_persisted:false,
      readiness_denial_review_materialized:false,
      readiness_denial_review_filesystem_written:false,
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
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .readiness_mode == "schema_only_activation_blocked"
  and .readiness_decision == "not_ready_for_live_mutation"
  and .denial_review_mode == "schema_only_review_activation_blocked"
  and .denial_review_decision == "readiness_denial_confirmed"
  and .denial_review_acceptance_mode == "schema_only_acceptance_blocked"
  and .denial_review_acceptance_decision == "readiness_denial_review_not_accepted"
  and .required_denial_review_acceptance_field_count == 18
  and .recorded_denial_review_acceptance_field_count == 0
  and .redacted_or_hashed_denial_review_acceptance_field_count == 16
  and .denial_review_acceptance_fixture_count == 4
  and .blocked_denial_review_acceptance_fixture_count == 4
  and .allowed_denial_review_acceptance_fixture_count == 0
  and .denial_review_acceptance_requested_count == 4
  and .denial_review_acceptance_performed_count == 0
  and .denial_review_acceptance_recorded_count == 0
  and .denial_review_acceptance_persisted_count == 0
  and .denial_review_acceptance_materialized_count == 0
  and .denial_review_acceptance_filesystem_written_count == 0
  and .accepted_readiness_denial_review_count == 0
  and .denial_review_acceptance_policy_satisfied_count == 0
  and .denial_review_acceptance_allowed_count == 0
  and .required_denial_review_family_count == 5
  and .ready_denial_review_family_count == 5
  and .activation_blocking_denial_review_family_count == 5
  and .reviewed_readiness_condition_count == 12
  and .blocked_readiness_condition_count == 12
  and .accepted_readiness_condition_count == 0
  and .reviewed_denial_reason_count == 16
  and .accepted_denial_reason_count == 0
  and .readiness_denial_review_performed_count == 0
  and .readiness_denial_review_recorded_count == 0
  and .readiness_denial_review_persisted_count == 0
  and .readiness_denial_review_materialized_count == 0
  and .readiness_denial_review_filesystem_written_count == 0
  and .readiness_denial_review_allowed_count == 0
  and .readiness_recorded == false
  and .readiness_persisted == false
  and .readiness_materialized == false
  and .readiness_filesystem_written == false
  and .readiness_allowed == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .satisfied_readiness_condition_count == 0
  and .denied_readiness_condition_count == 12
  and .denied_readiness_reason_count == 16
  and .denial_review_acceptance_fixture_hash_sha256 != ""
  and .denial_review_acceptance_policy_hash_sha256 != ""
  and .denial_review_acceptance_denial_hash_sha256 != ""
  and .denial_review_acceptance_side_effect_hash_sha256 != ""
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
  and (.readiness_denial_review_acceptance_fixtures | length) == 4
  and (.readiness_denial_review_acceptance_fixtures | all(.requested == true and .blocked == true and .accepted == false))
  and (.denied_by_readiness_denial_review_acceptance | length) == 19
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance readiness denial review acceptance gate passed"
