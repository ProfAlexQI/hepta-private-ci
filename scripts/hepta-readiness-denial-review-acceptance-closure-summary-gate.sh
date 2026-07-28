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

CLOSURE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-readiness-denial-review-acceptance-closure-gate" \
    scripts/i3-e08c8c0b4e1b74cb4ad1d7b3.sh
)"

closure_report_sha256="$(printf '%s' "$CLOSURE_JSON" | shasum -a 256 | awk '{print $1}')"
summary_family_hash_sha256="$(sha256_text "readiness-denial-review-acceptance-closure-summary:families:$closure_report_sha256")"
summary_denial_hash_sha256="$(sha256_text "readiness-denial-review-acceptance-closure-summary:denial:$closure_report_sha256")"
summary_policy_hash_sha256="$(sha256_text "readiness-denial-review-acceptance-closure-summary:policy:$closure_report_sha256")"
summary_side_effect_hash_sha256="$(sha256_text "readiness-denial-review-acceptance-closure-summary:side-effects:$closure_report_sha256")"

jq -n -e \
  --argjson closure "$CLOSURE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $closure.runtime == "hepta"
    and $closure.status == "ready"
    and $closure.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_closure_gate"
    and $closure.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_closure_ready == true
    and $closure.closure_mode == "schema_only_closure_activation_blocked"
    and $closure.closure_decision == "readiness_denial_review_acceptance_closed_without_activation"
    and $closure.readiness_decision == "not_ready_for_live_mutation"
    and $closure.denial_review_decision == "readiness_denial_confirmed"
    and $closure.denial_review_acceptance_decision == "readiness_denial_review_not_accepted"
    and $closure.closure_family_count == 5
    and $closure.ready_closure_family_count == 5
    and $closure.activation_blocking_closure_family_count == 5
    and $closure.closed_blocked_denial_review_acceptance_fixture_count == 4
    and $closure.closed_allowed_denial_review_acceptance_fixture_count == 0
    and $closure.closed_denial_review_acceptance_denial_reason_count == 19
    and $closure.accepted_readiness_denial_review_count == 0
    and $closure.denial_review_acceptance_policy_satisfied_count == 0
    and $closure.denial_review_acceptance_allowed_count == 0
    and $closure.readiness_denial_review_acceptance_closed == true
    and $closure.readiness_denial_review_acceptance_closure_recorded == false
    and $closure.readiness_denial_review_acceptance_closure_persisted == false
    and $closure.readiness_denial_review_acceptance_closure_materialized == false
    and $closure.readiness_denial_review_acceptance_closure_filesystem_written == false
    and $closure.activation_allowed == false
    and $closure.live_mutation_execution_ready == false
    and ($closure.closure_families | length) == 5
    and ($closure.closure_families | all(.ready == true and .blocked == true))
    and ($closure.denied_by_readiness_denial_review_acceptance_closure | length) == 19
    and ($closure.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_readiness_denial_review_acceptance_closure_summary_gate" \
  --arg closure_report_sha256 "$closure_report_sha256" \
  --arg summary_family_hash_sha256 "$summary_family_hash_sha256" \
  --arg summary_denial_hash_sha256 "$summary_denial_hash_sha256" \
  --arg summary_policy_hash_sha256 "$summary_policy_hash_sha256" \
  --arg summary_side_effect_hash_sha256 "$summary_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson closure "$CLOSURE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    readiness_denial_review_acceptance_closure_summary_schema_version:"readiness_denial_review_acceptance_closure_summary_v1",
    source_closure_gate:$closure.gate,
    source_closure_ready:$closure.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_review_acceptance_readiness_denial_review_acceptance_closure_ready,
    source_closure_report_sha256:$closure_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    readiness_denial_review_acceptance_closure_summary_ready:true,
    final_chain_mode:"schema_only_final_summary_activation_blocked",
    final_chain_decision:"readiness_denial_review_acceptance_closure_summarized_without_activation",
    closure_mode:$closure.closure_mode,
    closure_decision:$closure.closure_decision,
    readiness_decision:$closure.readiness_decision,
    denial_review_decision:$closure.denial_review_decision,
    denial_review_acceptance_decision:$closure.denial_review_acceptance_decision,
    required_summary_family_count:6,
    summary_family_count:6,
    ready_summary_family_count:6,
    activation_blocking_summary_family_count:6,
    inherited_closure_family_count:$closure.closure_family_count,
    inherited_ready_closure_family_count:$closure.ready_closure_family_count,
    inherited_activation_blocking_closure_family_count:$closure.activation_blocking_closure_family_count,
    inherited_blocked_denial_review_acceptance_fixture_count:$closure.closed_blocked_denial_review_acceptance_fixture_count,
    inherited_allowed_denial_review_acceptance_fixture_count:$closure.closed_allowed_denial_review_acceptance_fixture_count,
    inherited_denial_reason_count:$closure.closed_denial_review_acceptance_denial_reason_count,
    accepted_readiness_denial_review_count:$closure.accepted_readiness_denial_review_count,
    denial_review_acceptance_policy_satisfied_count:$closure.denial_review_acceptance_policy_satisfied_count,
    denial_review_acceptance_allowed_count:$closure.denial_review_acceptance_allowed_count,
    readiness_denial_review_acceptance_closed:$closure.readiness_denial_review_acceptance_closed,
    readiness_denial_review_acceptance_closure_recorded:$closure.readiness_denial_review_acceptance_closure_recorded,
    readiness_denial_review_acceptance_closure_persisted:$closure.readiness_denial_review_acceptance_closure_persisted,
    readiness_denial_review_acceptance_closure_materialized:$closure.readiness_denial_review_acceptance_closure_materialized,
    readiness_denial_review_acceptance_closure_filesystem_written:$closure.readiness_denial_review_acceptance_closure_filesystem_written,
    readiness_denial_review_acceptance_closure_summary_recorded:false,
    readiness_denial_review_acceptance_closure_summary_persisted:false,
    readiness_denial_review_acceptance_closure_summary_materialized:false,
    readiness_denial_review_acceptance_closure_summary_filesystem_written:false,
    terminal_summary_closed:true,
    readiness_allowed:$closure.readiness_allowed,
    activation_allowed:$closure.activation_allowed,
    live_mutation_execution_ready:$closure.live_mutation_execution_ready,
    denied_readiness_condition_count:$closure.denied_readiness_condition_count,
    denied_readiness_reason_count:$closure.denied_readiness_reason_count,
    summary_family_hash_sha256:$summary_family_hash_sha256,
    summary_denial_hash_sha256:$summary_denial_hash_sha256,
    summary_policy_hash_sha256:$summary_policy_hash_sha256,
    summary_side_effect_hash_sha256:$summary_side_effect_hash_sha256,
    command_invocation_performed_count:0,
    command_execution_performed_count:0,
    materialization_execution_performed_count:0,
    receipt_persistence_execution_performed_count:0,
    ledger_persistence_execution_performed_count:0,
    filesystem_persistence_execution_performed_count:0,
    filesystem_write_performed:false,
    workspace_write_performed:false,
    public_claim_allowed:false,
    release_artifact_write_allowed:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_inspected:false,
    live_secret_scan_performed:false,
    summary_families:[
      {
        id:"source-closure-gate-summary",
        ready:true,
        blocked:true,
        reason:"source closure gate is ready and remains activation-blocking"
      },
      {
        id:"closure-family-summary",
        ready:true,
        blocked:true,
        closure_family_count:$closure.closure_family_count,
        reason:"all closure families remain activation-blocking"
      },
      {
        id:"fixture-summary",
        ready:true,
        blocked:true,
        blocked_fixture_count:$closure.closed_blocked_denial_review_acceptance_fixture_count,
        allowed_fixture_count:$closure.closed_allowed_denial_review_acceptance_fixture_count,
        reason:"all inherited denial review acceptance fixtures are blocked"
      },
      {
        id:"denial-set-summary",
        ready:true,
        blocked:true,
        denial_reason_count:$closure.closed_denial_review_acceptance_denial_reason_count,
        reason:"the inherited denial set is summarized with no waivers"
      },
      {
        id:"persistence-side-effect-summary",
        ready:true,
        blocked:true,
        summary_recorded:false,
        summary_persisted:false,
        summary_materialized:false,
        reason:"summary is report-only and not persisted or materialized"
      },
      {
        id:"activation-boundary-summary",
        ready:true,
        blocked:true,
        activation_allowed:false,
        live_mutation_execution_ready:false,
        reason:"activation and live mutation remain denied"
      }
    ],
    denied_by_readiness_denial_review_acceptance_closure_summary:(
      [
        "readiness_denial_review_acceptance_closure_summary_recording_denied",
        "readiness_denial_review_acceptance_closure_summary_materialization_denied",
        "readiness_denial_review_acceptance_closure_summary_persistence_denied",
        "readiness_denial_review_acceptance_closure_summary_filesystem_write_denied"
      ] + $closure.denied_by_readiness_denial_review_acceptance_closure
    ),
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
      readiness_denial_review_acceptance_closure_summary_performed:false,
      readiness_denial_review_acceptance_closure_summary_recorded:false,
      readiness_denial_review_acceptance_closure_summary_persisted:false,
      readiness_denial_review_acceptance_closure_summary_materialized:false,
      readiness_denial_review_acceptance_closure_summary_filesystem_written:false,
      readiness_denial_review_acceptance_closure_recorded:false,
      readiness_denial_review_acceptance_closure_persisted:false,
      readiness_denial_review_acceptance_closure_materialized:false,
      readiness_denial_review_acceptance_closure_filesystem_written:false,
      readiness_denial_review_acceptance_recorded:false,
      readiness_denial_review_acceptance_persisted:false,
      readiness_denial_review_acceptance_materialized:false,
      readiness_denial_review_acceptance_filesystem_written:false,
      readiness_denial_review_recorded:false,
      readiness_denial_review_persisted:false,
      readiness_denial_review_materialized:false,
      readiness_denial_review_filesystem_written:false,
      readiness_recorded:false,
      readiness_persisted:false,
      readiness_materialized:false,
      readiness_filesystem_written:false,
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
  and .readiness_denial_review_acceptance_closure_summary_ready == true
  and .final_chain_mode == "schema_only_final_summary_activation_blocked"
  and .final_chain_decision == "readiness_denial_review_acceptance_closure_summarized_without_activation"
  and .summary_family_count == 6
  and .ready_summary_family_count == 6
  and .activation_blocking_summary_family_count == 6
  and .inherited_closure_family_count == 5
  and .inherited_blocked_denial_review_acceptance_fixture_count == 4
  and .inherited_allowed_denial_review_acceptance_fixture_count == 0
  and .inherited_denial_reason_count == 19
  and .terminal_summary_closed == true
  and .readiness_denial_review_acceptance_closure_summary_recorded == false
  and .readiness_denial_review_acceptance_closure_summary_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.summary_families | length) == 6
  and (.summary_families | all(.ready == true and .blocked == true))
  and (.denied_by_readiness_denial_review_acceptance_closure_summary | length) == 23
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta readiness denial review acceptance closure summary gate passed"
