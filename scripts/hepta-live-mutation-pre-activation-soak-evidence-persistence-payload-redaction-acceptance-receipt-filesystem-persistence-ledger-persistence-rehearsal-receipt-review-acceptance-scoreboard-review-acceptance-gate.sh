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

SCOREBOARD_REVIEW_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-gate.sh
)"

scoreboard_review_report_sha256="$(
  printf '%s' "$SCOREBOARD_REVIEW_JSON" | shasum -a 256 | awk '{print $1}'
)"

scoreboard_review_acceptance_fixture_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance:fixtures:$scoreboard_review_report_sha256"
)"
scoreboard_review_acceptance_denial_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance:denials:$scoreboard_review_report_sha256"
)"
scoreboard_review_acceptance_policy_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance:policy:$scoreboard_review_report_sha256"
)"
scoreboard_review_acceptance_side_effect_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance:side-effects:$scoreboard_review_report_sha256"
)"

jq -n -e \
  --argjson review "$SCOREBOARD_REVIEW_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $review.runtime == "hepta"
    and $review.status == "ready"
    and $review.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_gate"
    and $review.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_ready == true
    and $review.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_ready == true
    and $review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_report_sha256 != ""
    and $review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_report_sha256 != ""
    and $review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256 != ""
    and $review.minimum_required_samples >= 24
    and $review.required_scoreboard_review_family_count == 6
    and $review.ready_scoreboard_review_family_count == 6
    and $review.activation_blocking_scoreboard_review_family_count == 6
    and $review.required_scoreboard_family_count == 10
    and $review.ready_scoreboard_family_count == 10
    and $review.activation_blocking_scoreboard_family_count == 10
    and $review.scoreboard_family_reviewed_count == 10
    and $review.scoreboard_entry_review_count == 5
    and $review.reviewed_scoreboard_entry_count == 5
    and $review.blocked_scoreboard_entry_count == 5
    and $review.accepted_scoreboard_entry_count == 0
    and $review.denied_scoreboard_reason_count == 20
    and $review.reviewed_denial_reason_count == 20
    and $review.accepted_denial_reason_count == 0
    and $review.recorded_rehearsal_receipt_review_acceptance_field_count == 0
    and $review.blocked_rehearsal_receipt_review_acceptance_fixture_count == 4
    and $review.allowed_rehearsal_receipt_review_acceptance_fixture_count == 0
    and $review.accepted_rehearsal_receipt_review_count == 0
    and $review.review_acceptance_policy_satisfied_count == 0
    and $review.review_acceptance_allowed_count == 0
    and $review.rehearsal_receipt_review_acceptance_performed_count == 0
    and $review.rehearsal_receipt_review_acceptance_recorded_count == 0
    and $review.rehearsal_receipt_review_acceptance_persisted_count == 0
    and $review.rehearsal_receipt_review_acceptance_materialized_count == 0
    and $review.rehearsal_receipt_review_acceptance_filesystem_written_count == 0
    and $review.scoreboard_review_performed_count == 0
    and $review.scoreboard_review_recorded_count == 0
    and $review.scoreboard_review_persisted_count == 0
    and $review.scoreboard_review_materialized_count == 0
    and $review.scoreboard_review_filesystem_written_count == 0
    and $review.scoreboard_persisted_count == 0
    and $review.scoreboard_materialized_count == 0
    and $review.scoreboard_filesystem_written_count == 0
    and $review.ledger_persistence_allowed == false
    and $review.ledger_persistence_execution_performed == false
    and $review.ledger_recorded == false
    and $review.ledger_persisted == false
    and $review.ledger_materialized == false
    and $review.ledger_filesystem_written == false
    and $review.receipt_persistence_execution_performed_count == 0
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
    and $review.operator_approval_recorded == false
    and $review.fresh_pre_activation_soak_evidence_recorded == false
    and $review.accepted_redaction_proof_recorded == false
    and $review.rollback_rehearsal_evidence_recorded == false
    and $review.public_artifact_decision_recorded == false
    and $review.public_claim_allowed == false
    and $review.release_artifact_write_allowed == false
    and $review.raw_payload_plaintext_recorded == false
    and $review.raw_payload_plaintext_persisted == false
    and $review.raw_payload_inspected == false
    and $review.live_secret_scan_performed == false
    and $review.receipt_persistence_enabled == false
    and $review.scoreboard_review_family_hash_sha256 != ""
    and $review.scoreboard_review_entry_hash_sha256 != ""
    and $review.scoreboard_review_denial_hash_sha256 != ""
    and $review.scoreboard_review_side_effect_hash_sha256 != ""
    and $review.activation_blocked_by_rehearsal_receipt_review_acceptance_scoreboard_review == true
    and $review.activation_allowed_by_rehearsal_receipt_review_acceptance_scoreboard_review == false
    and $review.activation_allowed == false
    and $review.live_mutation_execution_ready == false
    and ($review.scoreboard_review_families | length) == 6
    and ($review.scoreboard_review_entries | length) == 5
    and ($review.scoreboard_review_entries | all(.ready == true and .blocked == true))
    and ($review.denied_by_rehearsal_receipt_review_acceptance_scoreboard_review | length) == 20
    and ($review.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_gate" \
  --arg scoreboard_review_report_sha256 "$scoreboard_review_report_sha256" \
  --arg scoreboard_review_acceptance_fixture_hash_sha256 "$scoreboard_review_acceptance_fixture_hash_sha256" \
  --arg scoreboard_review_acceptance_denial_hash_sha256 "$scoreboard_review_acceptance_denial_hash_sha256" \
  --arg scoreboard_review_acceptance_policy_hash_sha256 "$scoreboard_review_acceptance_policy_hash_sha256" \
  --arg scoreboard_review_acceptance_side_effect_hash_sha256 "$scoreboard_review_acceptance_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson review "$SCOREBOARD_REVIEW_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_schema_version:"payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_v1",
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_gate:$review.gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_ready:$review.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_ready,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_report_sha256:$scoreboard_review_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256:$review.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256,
    source_pre_activation_soak_report_sha256:$review.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$review.source_persistence_denial_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_ready:true,
    required_scoreboard_review_acceptance_field_count:20,
    scoreboard_review_acceptance_field_count:20,
    recorded_scoreboard_review_acceptance_field_count:0,
    redacted_or_hashed_scoreboard_review_acceptance_field_count:18,
    required_scoreboard_review_acceptance_fixture_count:4,
    scoreboard_review_acceptance_fixture_count:4,
    blocked_scoreboard_review_acceptance_fixture_count:4,
    allowed_scoreboard_review_acceptance_fixture_count:0,
    scoreboard_review_acceptance_hash_count:4,
    scoreboard_review_acceptance_requested_count:4,
    scoreboard_review_acceptance_performed_count:0,
    scoreboard_review_acceptance_recorded_count:0,
    scoreboard_review_acceptance_persisted_count:0,
    scoreboard_review_acceptance_materialized_count:0,
    scoreboard_review_acceptance_filesystem_written_count:0,
    accepted_scoreboard_review_count:0,
    scoreboard_review_acceptance_policy_satisfied_count:0,
    scoreboard_review_acceptance_allowed_count:0,
    required_scoreboard_review_family_count:$review.required_scoreboard_review_family_count,
    ready_scoreboard_review_family_count:$review.ready_scoreboard_review_family_count,
    activation_blocking_scoreboard_review_family_count:$review.activation_blocking_scoreboard_review_family_count,
    scoreboard_family_reviewed_count:$review.scoreboard_family_reviewed_count,
    scoreboard_entry_review_count:$review.scoreboard_entry_review_count,
    reviewed_scoreboard_entry_count:$review.reviewed_scoreboard_entry_count,
    blocked_scoreboard_entry_count:$review.blocked_scoreboard_entry_count,
    accepted_scoreboard_entry_count:$review.accepted_scoreboard_entry_count,
    denied_scoreboard_reason_count:$review.denied_scoreboard_reason_count,
    reviewed_denial_reason_count:$review.reviewed_denial_reason_count,
    accepted_denial_reason_count:$review.accepted_denial_reason_count,
    scoreboard_review_performed_count:0,
    scoreboard_review_recorded_count:0,
    scoreboard_review_persisted_count:0,
    scoreboard_review_materialized_count:0,
    scoreboard_review_filesystem_written_count:0,
    scoreboard_persisted_count:0,
    scoreboard_materialized_count:0,
    scoreboard_filesystem_written_count:0,
    accepted_rehearsal_receipt_review_count:0,
    review_acceptance_policy_satisfied_count:0,
    review_acceptance_allowed_count:0,
    rehearsal_receipt_review_acceptance_performed_count:0,
    rehearsal_receipt_review_acceptance_recorded_count:0,
    rehearsal_receipt_review_acceptance_persisted_count:0,
    rehearsal_receipt_review_acceptance_materialized_count:0,
    rehearsal_receipt_review_acceptance_filesystem_written_count:0,
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
    scoreboard_review_acceptance_fixture_hash_sha256:$scoreboard_review_acceptance_fixture_hash_sha256,
    scoreboard_review_acceptance_denial_hash_sha256:$scoreboard_review_acceptance_denial_hash_sha256,
    scoreboard_review_acceptance_policy_hash_sha256:$scoreboard_review_acceptance_policy_hash_sha256,
    scoreboard_review_acceptance_side_effect_hash_sha256:$scoreboard_review_acceptance_side_effect_hash_sha256,
    activation_blocked_by_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance:true,
    activation_allowed_by_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    scoreboard_review_acceptance_denial_reason:"scoreboard review acceptance remains schema-only because no operator approval, fresh evidence, accepted redaction proof, accepted scoreboard review, rollback rehearsal evidence, output-path selection, persistence approval, public artifact decision, activation, or live mutation is recorded",
    required_scoreboard_review_acceptance_fields:[
      "scoreboard_review_acceptance_id",
      "acceptance_schema_version",
      "source_scoreboard_review_report_sha256",
      "source_scoreboard_report_sha256",
      "source_review_acceptance_report_sha256",
      "source_review_report_sha256",
      "acceptance_fixture_id",
      "deterministic_acceptance_sha256",
      "scoreboard_review_family_hash",
      "scoreboard_review_entry_hash",
      "scoreboard_review_denial_hash",
      "scoreboard_review_side_effect_hash",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "fresh_pre_activation_soak_evidence_id",
      "active_binary_sha256",
      "trusted_source_binding",
      "accepted_scoreboard_review_ids",
      "public_claim_and_artifact_decision"
    ],
    scoreboard_review_acceptance_fixtures:[
      {
        id:"family-readiness-scoreboard-review-acceptance",
        ready:true,
        blocked:true,
        deterministic_acceptance_sha256:$scoreboard_review_acceptance_fixture_hash_sha256,
        accepted_scoreboard_review_count:0,
        reason:"scoreboard review families are ready but not accepted"
      },
      {
        id:"entry-blocking-scoreboard-review-acceptance",
        ready:true,
        blocked:true,
        accepted_scoreboard_entry_count:0,
        reason:"all reviewed scoreboard entries remain blocked"
      },
      {
        id:"denial-set-scoreboard-review-acceptance",
        ready:true,
        blocked:true,
        accepted_denial_reason_count:0,
        reason:"no denial reason has been waived by an approval record"
      },
      {
        id:"public-artifact-scoreboard-review-acceptance",
        ready:true,
        blocked:true,
        public_claim_allowed:false,
        release_artifact_write_allowed:false,
        reason:"public claim and release artifact decisions are absent"
      }
    ],
    denied_by_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance:[
      "scoreboard_review_acceptance_recording_denied",
      "scoreboard_review_acceptance_materialization_denied",
      "scoreboard_review_acceptance_persistence_denied",
      "scoreboard_review_acceptance_execution_denied",
      "scoreboard_review_recording_denied",
      "scoreboard_review_persistence_denied",
      "scoreboard_persistence_denied",
      "scoreboard_filesystem_write_denied",
      "review_acceptance_recording_denied",
      "operator_approval_missing",
      "fresh_pre_activation_soak_evidence_missing_or_stale",
      "accepted_redaction_proof_missing",
      "accepted_scoreboard_review_missing",
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
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_scoreboard_review_acceptance_field_count == 20
  and .scoreboard_review_acceptance_field_count == 20
  and .recorded_scoreboard_review_acceptance_field_count == 0
  and .redacted_or_hashed_scoreboard_review_acceptance_field_count == 18
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
  and .scoreboard_review_performed_count == 0
  and .scoreboard_review_recorded_count == 0
  and .scoreboard_review_persisted_count == 0
  and .scoreboard_review_materialized_count == 0
  and .scoreboard_review_filesystem_written_count == 0
  and .scoreboard_persisted_count == 0
  and .scoreboard_materialized_count == 0
  and .scoreboard_filesystem_written_count == 0
  and .accepted_rehearsal_receipt_review_count == 0
  and .review_acceptance_policy_satisfied_count == 0
  and .review_acceptance_allowed_count == 0
  and .rehearsal_receipt_review_acceptance_performed_count == 0
  and .rehearsal_receipt_review_acceptance_recorded_count == 0
  and .rehearsal_receipt_review_acceptance_persisted_count == 0
  and .rehearsal_receipt_review_acceptance_materialized_count == 0
  and .rehearsal_receipt_review_acceptance_filesystem_written_count == 0
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
  and .scoreboard_review_acceptance_fixture_hash_sha256 != ""
  and .scoreboard_review_acceptance_denial_hash_sha256 != ""
  and .scoreboard_review_acceptance_policy_hash_sha256 != ""
  and .scoreboard_review_acceptance_side_effect_hash_sha256 != ""
  and .activation_blocked_by_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance == true
  and .activation_allowed_by_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.required_scoreboard_review_acceptance_fields | length) == 20
  and (.scoreboard_review_acceptance_fixtures | length) == 4
  and (.scoreboard_review_acceptance_fixtures | all(.ready == true and .blocked == true))
  and (.denied_by_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance | length) == 22
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance gate passed"
