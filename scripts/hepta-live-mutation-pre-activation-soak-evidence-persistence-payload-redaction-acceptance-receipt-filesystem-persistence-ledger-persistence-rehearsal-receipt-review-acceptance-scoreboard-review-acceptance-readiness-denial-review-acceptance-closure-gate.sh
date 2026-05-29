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

ACCEPTANCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance-gate.sh
)"

acceptance_report_sha256="$(
  printf '%s' "$ACCEPTANCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
closure_family_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance-closure:families:$acceptance_report_sha256"
)"
closure_denial_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance-closure:denial:$acceptance_report_sha256"
)"
closure_policy_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance-closure:policy:$acceptance_report_sha256"
)"
closure_side_effect_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance-closure:side-effects:$acceptance_report_sha256"
)"

jq -n -e \
  --argjson acceptance "$ACCEPTANCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $acceptance.runtime == "hepta"
    and $acceptance.status == "ready"
    and $acceptance.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_gate"
    and $acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_ready == true
    and $acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_ready == true
    and $acceptance.readiness_mode == "schema_only_activation_blocked"
    and $acceptance.readiness_decision == "not_ready_for_live_mutation"
    and $acceptance.denial_review_mode == "schema_only_review_activation_blocked"
    and $acceptance.denial_review_decision == "readiness_denial_confirmed"
    and $acceptance.denial_review_acceptance_mode == "schema_only_acceptance_blocked"
    and $acceptance.denial_review_acceptance_decision == "readiness_denial_review_not_accepted"
    and $acceptance.required_denial_review_acceptance_field_count == 18
    and $acceptance.recorded_denial_review_acceptance_field_count == 0
    and $acceptance.redacted_or_hashed_denial_review_acceptance_field_count == 16
    and $acceptance.denial_review_acceptance_fixture_count == 4
    and $acceptance.blocked_denial_review_acceptance_fixture_count == 4
    and $acceptance.allowed_denial_review_acceptance_fixture_count == 0
    and $acceptance.denial_review_acceptance_requested_count == 4
    and $acceptance.denial_review_acceptance_performed_count == 0
    and $acceptance.denial_review_acceptance_recorded_count == 0
    and $acceptance.denial_review_acceptance_persisted_count == 0
    and $acceptance.denial_review_acceptance_materialized_count == 0
    and $acceptance.denial_review_acceptance_filesystem_written_count == 0
    and $acceptance.accepted_readiness_denial_review_count == 0
    and $acceptance.denial_review_acceptance_policy_satisfied_count == 0
    and $acceptance.denial_review_acceptance_allowed_count == 0
    and $acceptance.denial_review_family_count == 5
    and $acceptance.ready_denial_review_family_count == 5
    and $acceptance.activation_blocking_denial_review_family_count == 5
    and $acceptance.reviewed_readiness_condition_count == 12
    and $acceptance.blocked_readiness_condition_count == 12
    and $acceptance.accepted_readiness_condition_count == 0
    and $acceptance.reviewed_denial_reason_count == 16
    and $acceptance.accepted_denial_reason_count == 0
    and $acceptance.readiness_allowed == false
    and $acceptance.activation_allowed == false
    and $acceptance.live_mutation_execution_ready == false
    and ($acceptance.readiness_denial_review_acceptance_fixtures | length) == 4
    and ($acceptance.readiness_denial_review_acceptance_fixtures | all(.requested == true and .blocked == true and .accepted == false))
    and ($acceptance.denied_by_readiness_denial_review_acceptance | length) == 19
    and ($acceptance.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_closure_gate" \
  --arg acceptance_report_sha256 "$acceptance_report_sha256" \
  --arg closure_family_hash_sha256 "$closure_family_hash_sha256" \
  --arg closure_denial_hash_sha256 "$closure_denial_hash_sha256" \
  --arg closure_policy_hash_sha256 "$closure_policy_hash_sha256" \
  --arg closure_side_effect_hash_sha256 "$closure_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson acceptance "$ACCEPTANCE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    readiness_denial_review_acceptance_closure_schema_version:"payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_closure_v1",
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_gate:$acceptance.gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_ready:$acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_ready,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_report_sha256:$acceptance_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_closure_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_ready:$acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_ready,
    closure_mode:"schema_only_closure_activation_blocked",
    closure_decision:"readiness_denial_review_acceptance_closed_without_activation",
    readiness_mode:$acceptance.readiness_mode,
    readiness_decision:$acceptance.readiness_decision,
    denial_review_mode:$acceptance.denial_review_mode,
    denial_review_decision:$acceptance.denial_review_decision,
    denial_review_acceptance_mode:$acceptance.denial_review_acceptance_mode,
    denial_review_acceptance_decision:$acceptance.denial_review_acceptance_decision,
    required_closure_family_count:5,
    closure_family_count:5,
    ready_closure_family_count:5,
    activation_blocking_closure_family_count:5,
    closed_denial_review_acceptance_fixture_count:$acceptance.denial_review_acceptance_fixture_count,
    closed_blocked_denial_review_acceptance_fixture_count:$acceptance.blocked_denial_review_acceptance_fixture_count,
    closed_allowed_denial_review_acceptance_fixture_count:$acceptance.allowed_denial_review_acceptance_fixture_count,
    closed_denial_review_acceptance_denial_reason_count:($acceptance.denied_by_readiness_denial_review_acceptance | length),
    accepted_readiness_denial_review_count:$acceptance.accepted_readiness_denial_review_count,
    denial_review_acceptance_policy_satisfied_count:$acceptance.denial_review_acceptance_policy_satisfied_count,
    denial_review_acceptance_allowed_count:$acceptance.denial_review_acceptance_allowed_count,
    required_denial_review_acceptance_field_count:$acceptance.required_denial_review_acceptance_field_count,
    recorded_denial_review_acceptance_field_count:$acceptance.recorded_denial_review_acceptance_field_count,
    redacted_or_hashed_denial_review_acceptance_field_count:$acceptance.redacted_or_hashed_denial_review_acceptance_field_count,
    readiness_denial_review_acceptance_recorded:false,
    readiness_denial_review_acceptance_persisted:false,
    readiness_denial_review_acceptance_materialized:false,
    readiness_denial_review_acceptance_filesystem_written:false,
    readiness_denial_review_acceptance_closed:true,
    readiness_denial_review_acceptance_closure_recorded:false,
    readiness_denial_review_acceptance_closure_persisted:false,
    readiness_denial_review_acceptance_closure_materialized:false,
    readiness_denial_review_acceptance_closure_filesystem_written:false,
    readiness_allowed:$acceptance.readiness_allowed,
    activation_allowed:$acceptance.activation_allowed,
    live_mutation_execution_ready:$acceptance.live_mutation_execution_ready,
    denied_readiness_condition_count:$acceptance.denied_readiness_condition_count,
    denied_readiness_reason_count:$acceptance.denied_readiness_reason_count,
    closure_family_hash_sha256:$closure_family_hash_sha256,
    closure_denial_hash_sha256:$closure_denial_hash_sha256,
    closure_policy_hash_sha256:$closure_policy_hash_sha256,
    closure_side_effect_hash_sha256:$closure_side_effect_hash_sha256,
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
    closure_families:[
      {
        id:"source-acceptance-gate-closure",
        ready:true,
        blocked:true,
        source_ready:$acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_ready,
        reason:"source denial review acceptance gate is ready and remains activation-blocking"
      },
      {
        id:"fixture-closure",
        ready:true,
        blocked:true,
        fixture_count:$acceptance.denial_review_acceptance_fixture_count,
        blocked_fixture_count:$acceptance.blocked_denial_review_acceptance_fixture_count,
        reason:"all denial review acceptance fixtures remain blocked"
      },
      {
        id:"denial-set-closure",
        ready:true,
        blocked:true,
        denial_reason_count:($acceptance.denied_by_readiness_denial_review_acceptance | length),
        accepted_denial_reason_count:0,
        reason:"no denial reason is waived"
      },
      {
        id:"persistence-closure",
        ready:true,
        blocked:true,
        closure_recorded:false,
        closure_persisted:false,
        closure_materialized:false,
        reason:"closure is report-only and is not persisted or materialized"
      },
      {
        id:"activation-boundary-closure",
        ready:true,
        blocked:true,
        activation_allowed:false,
        live_mutation_execution_ready:false,
        reason:"activation and live mutation remain denied"
      }
    ],
    denied_by_readiness_denial_review_acceptance_closure:[
      "readiness_denial_review_acceptance_closure_recording_denied",
      "readiness_denial_review_acceptance_closure_materialization_denied",
      "readiness_denial_review_acceptance_closure_persistence_denied",
      "readiness_denial_review_acceptance_closure_filesystem_write_denied",
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
      readiness_denial_review_acceptance_closure_performed:false,
      readiness_denial_review_acceptance_closure_recorded:false,
      readiness_denial_review_acceptance_closure_persisted:false,
      readiness_denial_review_acceptance_closure_materialized:false,
      readiness_denial_review_acceptance_closure_filesystem_written:false,
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
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_closure_ready == true
  and .closure_mode == "schema_only_closure_activation_blocked"
  and .closure_decision == "readiness_denial_review_acceptance_closed_without_activation"
  and .closure_family_count == 5
  and .ready_closure_family_count == 5
  and .activation_blocking_closure_family_count == 5
  and .closed_blocked_denial_review_acceptance_fixture_count == 4
  and .closed_allowed_denial_review_acceptance_fixture_count == 0
  and .closed_denial_review_acceptance_denial_reason_count == 19
  and .readiness_denial_review_acceptance_closed == true
  and .readiness_denial_review_acceptance_closure_recorded == false
  and .readiness_denial_review_acceptance_closure_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.closure_families | length) == 5
  and (.closure_families | all(.ready == true and .blocked == true))
  and (.denied_by_readiness_denial_review_acceptance_closure | length) == 19
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance readiness denial review acceptance closure gate passed"
