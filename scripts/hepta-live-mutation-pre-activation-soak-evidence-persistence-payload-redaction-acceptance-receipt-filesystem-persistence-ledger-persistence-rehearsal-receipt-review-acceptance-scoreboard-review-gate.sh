#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SCOREBOARD_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-gate.sh
)"

scoreboard_report_sha256="$(
  printf '%s' "$SCOREBOARD_JSON" | shasum -a 256 | awk '{print $1}'
)"

scoreboard_review_family_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review:families:$scoreboard_report_sha256"
)"
scoreboard_review_entry_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review:entries:$scoreboard_report_sha256"
)"
scoreboard_review_denial_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review:denials:$scoreboard_report_sha256"
)"
scoreboard_review_side_effect_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review:side-effects:$scoreboard_report_sha256"
)"

jq -n -e \
  --argjson scoreboard "$SCOREBOARD_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $scoreboard.runtime == "hepta"
    and $scoreboard.status == "ready"
    and $scoreboard.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_gate"
    and $scoreboard.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_ready == true
    and $scoreboard.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_ready == true
    and $scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_report_sha256 != ""
    and $scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256 != ""
    and $scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256 != ""
    and $scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256 != ""
    and $scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256 != ""
    and $scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256 != ""
    and $scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256 != ""
    and $scoreboard.source_pre_activation_soak_report_sha256 != ""
    and $scoreboard.source_persistence_denial_report_sha256 != ""
    and $scoreboard.minimum_required_samples >= 24
    and $scoreboard.required_scoreboard_family_count == 10
    and $scoreboard.ready_scoreboard_family_count == 10
    and $scoreboard.activation_blocking_scoreboard_family_count == 10
    and $scoreboard.required_rehearsal_receipt_review_acceptance_field_count == 20
    and $scoreboard.rehearsal_receipt_review_acceptance_field_count == 20
    and $scoreboard.recorded_rehearsal_receipt_review_acceptance_field_count == 0
    and $scoreboard.rehearsal_receipt_review_acceptance_fixture_count == 4
    and $scoreboard.blocked_rehearsal_receipt_review_acceptance_fixture_count == 4
    and $scoreboard.allowed_rehearsal_receipt_review_acceptance_fixture_count == 0
    and $scoreboard.accepted_rehearsal_receipt_review_count == 0
    and $scoreboard.review_acceptance_policy_satisfied_count == 0
    and $scoreboard.review_acceptance_allowed_count == 0
    and $scoreboard.rehearsal_receipt_review_acceptance_performed_count == 0
    and $scoreboard.rehearsal_receipt_review_acceptance_recorded_count == 0
    and $scoreboard.rehearsal_receipt_review_acceptance_persisted_count == 0
    and $scoreboard.rehearsal_receipt_review_acceptance_materialized_count == 0
    and $scoreboard.rehearsal_receipt_review_acceptance_filesystem_written_count == 0
    and $scoreboard.ledger_persistence_allowed == false
    and $scoreboard.ledger_persistence_execution_performed == false
    and $scoreboard.ledger_recorded == false
    and $scoreboard.ledger_persisted == false
    and $scoreboard.ledger_materialized == false
    and $scoreboard.ledger_filesystem_written == false
    and $scoreboard.receipt_persistence_execution_performed_count == 0
    and $scoreboard.receipt_materialized_count == 0
    and $scoreboard.receipt_persisted_count == 0
    and $scoreboard.filesystem_persistence_allowed == false
    and $scoreboard.filesystem_persistence_execution_performed == false
    and $scoreboard.filesystem_write_performed == false
    and $scoreboard.workspace_write_performed == false
    and $scoreboard.command_invocation_requested_count == 0
    and $scoreboard.command_invocation_performed_count == 0
    and $scoreboard.command_execution_requested_count == 0
    and $scoreboard.command_execution_performed_count == 0
    and $scoreboard.materialization_execution_requested_count == 0
    and $scoreboard.materialization_execution_performed_count == 0
    and $scoreboard.selected_output_path_count == 0
    and $scoreboard.recorded_output_path_count == 0
    and $scoreboard.recorded_path_binding_count == 0
    and $scoreboard.operator_approval_recorded == false
    and $scoreboard.fresh_pre_activation_soak_evidence_recorded == false
    and $scoreboard.accepted_redaction_proof_recorded == false
    and $scoreboard.rollback_rehearsal_evidence_recorded == false
    and $scoreboard.public_artifact_decision_recorded == false
    and $scoreboard.public_claim_allowed == false
    and $scoreboard.release_artifact_write_allowed == false
    and $scoreboard.raw_payload_plaintext_recorded == false
    and $scoreboard.raw_payload_plaintext_persisted == false
    and $scoreboard.raw_payload_inspected == false
    and $scoreboard.live_secret_scan_performed == false
    and $scoreboard.receipt_persistence_enabled == false
    and $scoreboard.scoreboard_family_hash_sha256 != ""
    and $scoreboard.scoreboard_denial_hash_sha256 != ""
    and $scoreboard.scoreboard_side_effect_hash_sha256 != ""
    and $scoreboard.activation_blocked_by_rehearsal_receipt_review_acceptance_scoreboard == true
    and $scoreboard.activation_allowed_by_rehearsal_receipt_review_acceptance_scoreboard == false
    and $scoreboard.activation_allowed == false
    and $scoreboard.live_mutation_execution_ready == false
    and ($scoreboard.scoreboard_families | length) == 10
    and ($scoreboard.scoreboard_entries | length) == 5
    and ($scoreboard.scoreboard_entries | all(.ready == true and .blocked == true))
    and ($scoreboard.denied_by_rehearsal_receipt_review_acceptance_scoreboard | length) == 20
    and ($scoreboard.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_gate" \
  --arg scoreboard_report_sha256 "$scoreboard_report_sha256" \
  --arg scoreboard_review_family_hash_sha256 "$scoreboard_review_family_hash_sha256" \
  --arg scoreboard_review_entry_hash_sha256 "$scoreboard_review_entry_hash_sha256" \
  --arg scoreboard_review_denial_hash_sha256 "$scoreboard_review_denial_hash_sha256" \
  --arg scoreboard_review_side_effect_hash_sha256 "$scoreboard_review_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson scoreboard "$SCOREBOARD_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    rehearsal_receipt_review_acceptance_scoreboard_review_schema_version:"payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_v1",
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_gate:$scoreboard.gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_ready:$scoreboard.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_ready,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_report_sha256:$scoreboard_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_report_sha256:$scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256:$scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256:$scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256:$scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256:$scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256:$scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256:$scoreboard.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256,
    source_pre_activation_soak_report_sha256:$scoreboard.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$scoreboard.source_persistence_denial_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_ready:true,
    required_scoreboard_review_family_count:6,
    ready_scoreboard_review_family_count:6,
    activation_blocking_scoreboard_review_family_count:6,
    required_scoreboard_family_count:$scoreboard.required_scoreboard_family_count,
    ready_scoreboard_family_count:$scoreboard.ready_scoreboard_family_count,
    activation_blocking_scoreboard_family_count:$scoreboard.activation_blocking_scoreboard_family_count,
    scoreboard_family_reviewed_count:10,
    scoreboard_entry_review_count:5,
    reviewed_scoreboard_entry_count:5,
    blocked_scoreboard_entry_count:5,
    accepted_scoreboard_entry_count:0,
    denied_scoreboard_reason_count:20,
    reviewed_denial_reason_count:20,
    accepted_denial_reason_count:0,
    required_rehearsal_receipt_review_acceptance_field_count:$scoreboard.required_rehearsal_receipt_review_acceptance_field_count,
    recorded_rehearsal_receipt_review_acceptance_field_count:$scoreboard.recorded_rehearsal_receipt_review_acceptance_field_count,
    rehearsal_receipt_review_acceptance_fixture_count:$scoreboard.rehearsal_receipt_review_acceptance_fixture_count,
    blocked_rehearsal_receipt_review_acceptance_fixture_count:$scoreboard.blocked_rehearsal_receipt_review_acceptance_fixture_count,
    allowed_rehearsal_receipt_review_acceptance_fixture_count:$scoreboard.allowed_rehearsal_receipt_review_acceptance_fixture_count,
    accepted_rehearsal_receipt_review_count:0,
    review_acceptance_policy_satisfied_count:0,
    review_acceptance_allowed_count:0,
    rehearsal_receipt_review_acceptance_performed_count:0,
    rehearsal_receipt_review_acceptance_recorded_count:0,
    rehearsal_receipt_review_acceptance_persisted_count:0,
    rehearsal_receipt_review_acceptance_materialized_count:0,
    rehearsal_receipt_review_acceptance_filesystem_written_count:0,
    scoreboard_review_performed_count:0,
    scoreboard_review_recorded_count:0,
    scoreboard_review_persisted_count:0,
    scoreboard_review_materialized_count:0,
    scoreboard_review_filesystem_written_count:0,
    scoreboard_persisted_count:0,
    scoreboard_materialized_count:0,
    scoreboard_filesystem_written_count:0,
    ledger_persistence_allowed:false,
    ledger_persistence_execution_performed:false,
    ledger_recorded:false,
    ledger_persisted:false,
    ledger_materialized:false,
    ledger_filesystem_written:false,
    receipt_persistence_allowed_count:0,
    receipt_persistence_execution_performed_count:0,
    receipt_materialized_count:0,
    receipt_persisted_count:0,
    filesystem_persistence_allowed:false,
    filesystem_persistence_execution_performed:false,
    filesystem_write_performed:false,
    workspace_write_performed:false,
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
    operator_approval_recorded:false,
    fresh_pre_activation_soak_evidence_recorded:false,
    accepted_redaction_proof_recorded:false,
    rollback_rehearsal_evidence_recorded:false,
    public_artifact_decision_recorded:false,
    public_claim_allowed:false,
    release_artifact_write_allowed:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    raw_payload_inspected:false,
    live_secret_scan_performed:false,
    receipt_persistence_enabled:false,
    scoreboard_review_family_hash_sha256:$scoreboard_review_family_hash_sha256,
    scoreboard_review_entry_hash_sha256:$scoreboard_review_entry_hash_sha256,
    scoreboard_review_denial_hash_sha256:$scoreboard_review_denial_hash_sha256,
    scoreboard_review_side_effect_hash_sha256:$scoreboard_review_side_effect_hash_sha256,
    activation_blocked_by_rehearsal_receipt_review_acceptance_scoreboard_review:true,
    activation_allowed_by_rehearsal_receipt_review_acceptance_scoreboard_review:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    scoreboard_review_denial_reason:"scoreboard families and entries are review-ready as report-only contracts, but no scoreboard review, acceptance, operator approval, fresh evidence, output-path selection, persistence approval, public artifact decision, activation, or live mutation is recorded",
    scoreboard_review_families:[
      "source-scoreboard-gate",
      "scoreboard-family-review",
      "scoreboard-entry-review",
      "scoreboard-denial-review",
      "scoreboard-side-effect-review",
      "scoreboard-activation-boundary-review"
    ],
    scoreboard_review_entries:[
      {
        id:"scoreboard-family-readiness-review",
        ready:true,
        blocked:true,
        reviewed_family_count:10,
        activation_blocking_family_count:10,
        reason:"all source scoreboard families are ready and remain activation-blocking"
      },
      {
        id:"scoreboard-entry-blocking-review",
        ready:true,
        blocked:true,
        reviewed_entry_count:5,
        blocked_entry_count:5,
        accepted_entry_count:0,
        reason:"all source scoreboard entries are blocked and none are accepted"
      },
      {
        id:"scoreboard-denial-set-review",
        ready:true,
        blocked:true,
        reviewed_denial_reason_count:20,
        accepted_denial_reason_count:0,
        reason:"the denial set is complete and no denial is waived"
      },
      {
        id:"scoreboard-side-effect-review",
        ready:true,
        blocked:true,
        scoreboard_review_performed:false,
        scoreboard_review_recorded:false,
        scoreboard_review_persisted:false,
        reason:"review remains schema-only with no persistence or materialization"
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
    denied_by_rehearsal_receipt_review_acceptance_scoreboard_review:[
      "scoreboard_review_recording_denied",
      "scoreboard_review_materialization_denied",
      "scoreboard_review_persistence_denied",
      "scoreboard_review_execution_denied",
      "scoreboard_persistence_denied",
      "scoreboard_filesystem_write_denied",
      "review_acceptance_recording_denied",
      "review_acceptance_persistence_denied",
      "operator_approval_missing",
      "fresh_pre_activation_soak_evidence_missing_or_stale",
      "accepted_redaction_proof_missing",
      "rollback_rehearsal_evidence_missing",
      "output_path_selection_missing",
      "ledger_persistence_execution_denied",
      "receipt_persistence_execution_denied",
      "filesystem_persistence_execution_denied",
      "filesystem_write_denied",
      "workspace_write_denied",
      "public_artifact_decision_missing",
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
      ledger_persistence_execution_performed:false,
      scoreboard_review_performed:false,
      scoreboard_review_recorded:false,
      scoreboard_review_persisted:false,
      scoreboard_review_materialized:false,
      scoreboard_review_filesystem_written:false,
      scoreboard_recorded:false,
      scoreboard_persisted:false,
      scoreboard_materialized:false,
      scoreboard_filesystem_written:false,
      rehearsal_receipt_review_acceptance_performed:false,
      rehearsal_receipt_review_acceptance_recorded:false,
      rehearsal_receipt_review_acceptance_persisted:false,
      rehearsal_receipt_review_acceptance_materialized:false,
      rehearsal_receipt_review_acceptance_filesystem_written:false,
      rehearsal_receipt_review_performed:false,
      rehearsal_receipt_review_recorded:false,
      rehearsal_receipt_review_persisted:false,
      rehearsal_receipt_review_materialized:false,
      rehearsal_receipt_review_filesystem_written:false,
      rehearsal_receipt_materialized:false,
      rehearsal_receipt_persisted:false,
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
      output_path_binding_selected:false,
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
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_scoreboard_review_family_count == 6
  and .ready_scoreboard_review_family_count == 6
  and .activation_blocking_scoreboard_review_family_count == 6
  and .required_scoreboard_family_count == 10
  and .ready_scoreboard_family_count == 10
  and .activation_blocking_scoreboard_family_count == 10
  and .scoreboard_family_reviewed_count == 10
  and .scoreboard_entry_review_count == 5
  and .reviewed_scoreboard_entry_count == 5
  and .blocked_scoreboard_entry_count == 5
  and .accepted_scoreboard_entry_count == 0
  and .denied_scoreboard_reason_count == 20
  and .reviewed_denial_reason_count == 20
  and .accepted_denial_reason_count == 0
  and .recorded_rehearsal_receipt_review_acceptance_field_count == 0
  and .blocked_rehearsal_receipt_review_acceptance_fixture_count == 4
  and .allowed_rehearsal_receipt_review_acceptance_fixture_count == 0
  and .accepted_rehearsal_receipt_review_count == 0
  and .review_acceptance_policy_satisfied_count == 0
  and .review_acceptance_allowed_count == 0
  and .rehearsal_receipt_review_acceptance_performed_count == 0
  and .rehearsal_receipt_review_acceptance_recorded_count == 0
  and .rehearsal_receipt_review_acceptance_persisted_count == 0
  and .rehearsal_receipt_review_acceptance_materialized_count == 0
  and .rehearsal_receipt_review_acceptance_filesystem_written_count == 0
  and .scoreboard_review_performed_count == 0
  and .scoreboard_review_recorded_count == 0
  and .scoreboard_review_persisted_count == 0
  and .scoreboard_review_materialized_count == 0
  and .scoreboard_review_filesystem_written_count == 0
  and .scoreboard_persisted_count == 0
  and .scoreboard_materialized_count == 0
  and .scoreboard_filesystem_written_count == 0
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
  and .operator_approval_recorded == false
  and .fresh_pre_activation_soak_evidence_recorded == false
  and .accepted_redaction_proof_recorded == false
  and .rollback_rehearsal_evidence_recorded == false
  and .public_artifact_decision_recorded == false
  and .public_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .raw_payload_inspected == false
  and .live_secret_scan_performed == false
  and .receipt_persistence_enabled == false
  and .scoreboard_review_family_hash_sha256 != ""
  and .scoreboard_review_entry_hash_sha256 != ""
  and .scoreboard_review_denial_hash_sha256 != ""
  and .scoreboard_review_side_effect_hash_sha256 != ""
  and .activation_blocked_by_rehearsal_receipt_review_acceptance_scoreboard_review == true
  and .activation_allowed_by_rehearsal_receipt_review_acceptance_scoreboard_review == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.scoreboard_review_families | length) == 6
  and (.scoreboard_review_entries | length) == 5
  and (.scoreboard_review_entries | all(.ready == true and .blocked == true))
  and (.denied_by_rehearsal_receipt_review_acceptance_scoreboard_review | length) == 20
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review gate passed"
