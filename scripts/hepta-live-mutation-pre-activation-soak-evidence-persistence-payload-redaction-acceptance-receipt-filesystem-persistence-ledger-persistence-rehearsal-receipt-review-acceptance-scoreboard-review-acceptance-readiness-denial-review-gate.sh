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

READINESS_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-gate.sh
)"

readiness_report_sha256="$(
  printf '%s' "$READINESS_JSON" | shasum -a 256 | awk '{print $1}'
)"
denial_review_family_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review:families:$readiness_report_sha256"
)"
denial_review_reason_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review:reasons:$readiness_report_sha256"
)"
denial_review_policy_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review:policy:$readiness_report_sha256"
)"
denial_review_side_effect_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review:side-effects:$readiness_report_sha256"
)"

jq -n -e \
  --argjson readiness "$READINESS_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $readiness.runtime == "hepta"
    and $readiness.status == "ready"
    and $readiness.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_gate"
    and $readiness.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_ready == true
    and $readiness.readiness_mode == "schema_only_activation_blocked"
    and $readiness.readiness_decision == "not_ready_for_live_mutation"
    and $readiness.required_readiness_condition_count == 12
    and $readiness.readiness_condition_count == 12
    and $readiness.satisfied_readiness_condition_count == 0
    and $readiness.blocked_readiness_condition_count == 12
    and $readiness.recorded_readiness_field_count == 0
    and $readiness.readiness_recorded == false
    and $readiness.readiness_persisted == false
    and $readiness.readiness_materialized == false
    and $readiness.readiness_filesystem_written == false
    and $readiness.operator_approval_recorded == false
    and $readiness.single_surface_activation_scope_recorded == false
    and $readiness.fresh_pre_activation_soak_evidence_recorded == false
    and $readiness.active_binary_sha_recorded == false
    and $readiness.trusted_source_binding_recorded == false
    and $readiness.accepted_redaction_proof_recorded == false
    and $readiness.accepted_scoreboard_review_recorded == false
    and $readiness.rollback_rehearsal_evidence_recorded == false
    and $readiness.output_path_selection_recorded == false
    and $readiness.ledger_persistence_approval_recorded == false
    and $readiness.receipt_persistence_approval_recorded == false
    and $readiness.public_artifact_decision_recorded == false
    and $readiness.readiness_allowed == false
    and $readiness.activation_allowed == false
    and $readiness.live_mutation_execution_ready == false
    and ($readiness.readiness_conditions | length) == 12
    and ($readiness.readiness_conditions | all(.satisfied == false and .blocks_activation == true))
    and ($readiness.denied_by_readiness_gate | length) == 16
    and ($readiness.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_gate" \
  --arg readiness_report_sha256 "$readiness_report_sha256" \
  --arg denial_review_family_hash_sha256 "$denial_review_family_hash_sha256" \
  --arg denial_review_reason_hash_sha256 "$denial_review_reason_hash_sha256" \
  --arg denial_review_policy_hash_sha256 "$denial_review_policy_hash_sha256" \
  --arg denial_review_side_effect_hash_sha256 "$denial_review_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson readiness "$READINESS_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    readiness_denial_review_schema_version:"payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_v1",
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_gate:$readiness.gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_ready:$readiness.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_ready,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_report_sha256:$readiness_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_report_sha256:$readiness.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_report_sha256:$readiness.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_ready:true,
    readiness_mode:$readiness.readiness_mode,
    readiness_decision:$readiness.readiness_decision,
    denial_review_mode:"schema_only_review_activation_blocked",
    denial_review_decision:"readiness_denial_confirmed",
    required_denial_review_family_count:5,
    denial_review_family_count:5,
    ready_denial_review_family_count:5,
    activation_blocking_denial_review_family_count:5,
    reviewed_readiness_condition_count:$readiness.readiness_condition_count,
    blocked_readiness_condition_count:$readiness.blocked_readiness_condition_count,
    accepted_readiness_condition_count:0,
    reviewed_denial_reason_count:($readiness.denied_by_readiness_gate | length),
    accepted_denial_reason_count:0,
    readiness_denial_review_performed_count:0,
    readiness_denial_review_recorded_count:0,
    readiness_denial_review_persisted_count:0,
    readiness_denial_review_materialized_count:0,
    readiness_denial_review_filesystem_written_count:0,
    readiness_denial_review_allowed_count:0,
    recorded_readiness_field_count:$readiness.recorded_readiness_field_count,
    readiness_recorded:$readiness.readiness_recorded,
    readiness_persisted:$readiness.readiness_persisted,
    readiness_materialized:$readiness.readiness_materialized,
    readiness_filesystem_written:$readiness.readiness_filesystem_written,
    readiness_allowed:$readiness.readiness_allowed,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    required_readiness_condition_count:$readiness.required_readiness_condition_count,
    readiness_condition_count:$readiness.readiness_condition_count,
    satisfied_readiness_condition_count:$readiness.satisfied_readiness_condition_count,
    denied_readiness_condition_count:$readiness.blocked_readiness_condition_count,
    denied_readiness_reason_count:($readiness.denied_by_readiness_gate | length),
    denial_review_family_hash_sha256:$denial_review_family_hash_sha256,
    denial_review_reason_hash_sha256:$denial_review_reason_hash_sha256,
    denial_review_policy_hash_sha256:$denial_review_policy_hash_sha256,
    denial_review_side_effect_hash_sha256:$denial_review_side_effect_hash_sha256,
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
    readiness_denial_review_families:[
      {
        id:"readiness-condition-review",
        ready:true,
        blocked:true,
        reviewed_condition_count:$readiness.readiness_condition_count,
        accepted_condition_count:0,
        reason:"all readiness conditions are reviewed and remain unsatisfied"
      },
      {
        id:"readiness-denial-set-review",
        ready:true,
        blocked:true,
        reviewed_denial_reason_count:($readiness.denied_by_readiness_gate | length),
        accepted_denial_reason_count:0,
        reason:"the denial set is complete and no denial is waived"
      },
      {
        id:"readiness-recording-review",
        ready:true,
        blocked:true,
        readiness_recorded:false,
        readiness_persisted:false,
        readiness_materialized:false,
        reason:"readiness remains schema-only with no recording, persistence, or materialization"
      },
      {
        id:"scoreboard-acceptance-inheritance-review",
        ready:true,
        blocked:true,
        accepted_scoreboard_review_count:$readiness.accepted_scoreboard_review_count,
        scoreboard_review_acceptance_allowed_count:$readiness.scoreboard_review_acceptance_allowed_count,
        reason:"the inherited scoreboard review acceptance remains blocked and unaccepted"
      },
      {
        id:"live-mutation-boundary-review",
        ready:true,
        blocked:true,
        activation_allowed:false,
        live_mutation_execution_ready:false,
        reason:"activation and live mutation remain denied"
      }
    ],
    denied_by_readiness_denial_review:[
      "readiness_denial_review_recording_denied",
      "readiness_denial_review_materialization_denied",
      "readiness_denial_review_persistence_denied",
      "readiness_denial_review_filesystem_write_denied",
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
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .readiness_mode == "schema_only_activation_blocked"
  and .readiness_decision == "not_ready_for_live_mutation"
  and .denial_review_mode == "schema_only_review_activation_blocked"
  and .denial_review_decision == "readiness_denial_confirmed"
  and .required_denial_review_family_count == 5
  and .denial_review_family_count == 5
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
  and .recorded_readiness_field_count == 0
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
  and .denial_review_family_hash_sha256 != ""
  and .denial_review_reason_hash_sha256 != ""
  and .denial_review_policy_hash_sha256 != ""
  and .denial_review_side_effect_hash_sha256 != ""
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
  and (.readiness_denial_review_families | length) == 5
  and (.readiness_denial_review_families | all(.ready == true and .blocked == true))
  and (.denied_by_readiness_denial_review | length) == 18
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance readiness denial review gate passed"
