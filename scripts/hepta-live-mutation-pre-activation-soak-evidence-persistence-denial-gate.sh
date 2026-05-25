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

SOAK_EVIDENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    HEPTA_LIVE_MUTATION_PRE_ACTIVATION_SOAK_RUN=0 \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-gate.sh
)"

soak_evidence_report_sha256="$(printf '%s' "$SOAK_EVIDENCE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson soak_evidence "$SOAK_EVIDENCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $soak_evidence.runtime == "hepta"
    and $soak_evidence.status == "ready"
    and $soak_evidence.gate == "hepta_live_mutation_pre_activation_soak_evidence_gate"
    and $soak_evidence.pre_activation_soak_evidence_gate_ready == true
    and $soak_evidence.source_receipt_ready == true
    and $soak_evidence.source_receipt_payload_sha256 != ""
    and $soak_evidence.minimum_required_samples >= 24
    and $soak_evidence.long_soak_execution_default_enabled == false
    and $soak_evidence.long_soak_evidence_candidate_ready == true
    and $soak_evidence.long_soak_evidence_persisted == false
    and $soak_evidence.activation_allowed == false
    and $soak_evidence.live_mutation_execution_ready == false
    and $soak_evidence.receipt_persistence_enabled == false
    and $soak_evidence.receipt_persisted == false
    and $soak_evidence.operator_approval_recorded == false
    and ($soak_evidence.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_denial_gate" \
  --arg soak_evidence_report_sha256 "$soak_evidence_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson soak_evidence "$SOAK_EVIDENCE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_pre_activation_soak_evidence_gate:$soak_evidence.gate,
    source_pre_activation_soak_evidence_gate_ready:$soak_evidence.pre_activation_soak_evidence_gate_ready,
    source_receipt_payload_sha256:$soak_evidence.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$soak_evidence_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    persistence_denial_gate_ready:true,
    pre_activation_soak_evidence_persistence_allowed:false,
    fresh_soak_evidence_recorded:false,
    fresh_soak_evidence_bound:false,
    long_soak_evidence_persisted:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    operator_approval_recorded:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    required_persistence_field_count:10,
    recorded_persistence_field_count:0,
    required_persistence_fields:[
      "operator_approval_id",
      "single_surface_activation_scope",
      "source_receipt_payload_sha256",
      "pre_activation_soak_report_sha256",
      "fresh_soak_evidence_record_id",
      "fresh_soak_evidence_captured_at_unix",
      "fresh_soak_evidence_sample_count",
      "fresh_soak_evidence_report_sha256",
      "installed_binary_sha256_after_approval",
      "rollback_plan_id"
    ],
    denied_persistence_fixtures:[
      {
        id:"missing-operator-approval",
        operator_approval_recorded:false,
        fresh_soak_evidence_recorded:true,
        source_receipt_hash_bound:true,
        persistence_allowed:false,
        reason:"operator_approval_id_missing"
      },
      {
        id:"missing-fresh-soak-evidence-record",
        operator_approval_recorded:true,
        fresh_soak_evidence_recorded:false,
        source_receipt_hash_bound:true,
        persistence_allowed:false,
        reason:"fresh_24_sample_soak_evidence_not_recorded"
      },
      {
        id:"source-receipt-hash-not-bound",
        operator_approval_recorded:true,
        fresh_soak_evidence_recorded:true,
        source_receipt_hash_bound:false,
        persistence_allowed:false,
        reason:"source_receipt_payload_sha256_missing_or_mismatched"
      },
      {
        id:"filesystem-persistence-request-before-approval",
        operator_approval_recorded:false,
        fresh_soak_evidence_recorded:false,
        source_receipt_hash_bound:false,
        persistence_allowed:false,
        reason:"receipt_persistence_denied_before_full_approval_packet"
      }
    ],
    denied_by_persistence_gate:[
      "operator_approval_not_recorded",
      "single_surface_activation_scope_missing",
      "fresh_soak_evidence_not_recorded",
      "source_receipt_payload_hash_not_bound_to_persistence_record",
      "installed_binary_sha_after_approval_not_recorded",
      "rollback_plan_id_not_recorded"
    ],
    required_before_persistence:[
      "explicit_operator_approval_id",
      "single_surface_activation_scope",
      "source_receipt_payload_hash_binding",
      "fresh_24_sample_pre_activation_soak_evidence_record",
      "current_installed_binary_backup_after_approval",
      "reviewed_rollback_plan_id",
      "no_secret_receipt_payload_review"
    ],
    required_before_activation:[
      "persisted_pre_activation_soak_evidence_receipt",
      "post_activation_watchdog",
      "post_activation_minimum_24_sample_soak",
      "side_effect_receipt_with_no_secret_values"
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
      filesystem_written:false,
      release_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      receipt_persisted:false,
      pre_activation_soak_evidence_persisted:false,
      external_send_performed:false,
      credential_read:false
    }
  }')"

jq -e '
  .status == "ready"
  and .persistence_denial_gate_ready == true
  and .source_pre_activation_soak_evidence_gate_ready == true
  and .source_receipt_payload_sha256 != ""
  and .minimum_required_samples >= 24
  and .pre_activation_soak_evidence_persistence_allowed == false
  and .fresh_soak_evidence_recorded == false
  and .fresh_soak_evidence_bound == false
  and .long_soak_evidence_persisted == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .operator_approval_recorded == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .required_persistence_field_count == 10
  and .recorded_persistence_field_count == 0
  and (.denied_persistence_fixtures | length) == 4
  and (.denied_persistence_fixtures | all(.persistence_allowed == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence denial gate passed"
