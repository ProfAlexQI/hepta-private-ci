#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
SOAK_INTERVAL_SECONDS="${HEPTA_LIVE_MUTATION_SOAK_INTERVAL_SECONDS:-5}"
RUN_SOAK="${HEPTA_LIVE_MUTATION_PRE_ACTIVATION_SOAK_RUN:-0}"

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

if [[ ! -x scripts/hepta-live-soak.sh ]]; then
  echo "scripts/hepta-live-soak.sh is missing or not executable" >&2
  exit 1
fi

bash -n scripts/hepta-live-soak.sh

RECEIPT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-approval-evidence-receipt-gate" \
    scripts/hepta-live-mutation-approval-evidence-receipt-gate.sh
)"

SOAK_JSON="null"
if [[ "$RUN_SOAK" == "1" ]]; then
  SOAK_JSON="$(
    HEPTA_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_SOAK_INTERVAL_SECONDS="$SOAK_INTERVAL_SECONDS" \
      capture_json_report "hepta-live-soak" \
      scripts/hepta-live-soak.sh
  )"
fi

jq -n -e \
  --argjson receipt "$RECEIPT_JSON" \
  --argjson soak "$SOAK_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --arg run_soak "$RUN_SOAK" \
  '
    $receipt.runtime == "hepta"
    and $receipt.status == "ready"
    and $receipt.gate == "hepta_live_mutation_approval_evidence_receipt_gate"
    and $receipt.approval_evidence_receipt_ready == true
    and $receipt.activation_allowed == false
    and $receipt.live_mutation_execution_ready == false
    and $receipt.receipt_persistence_enabled == false
    and $receipt.receipt_persisted == false
    and $receipt.operator_approval_recorded == false
    and $receipt.receipt_payload.minimum_long_soak_required_samples >= 24
    and $receipt.receipt_payload.long_soak_executed_by_this_gate == false
    and ($receipt.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
    and (
      ($run_soak != "1" and $soak == null)
      or (
        $run_soak == "1"
        and $soak.runtime == "hepta"
        and $soak.status == "ready"
        and $soak.samples >= $min_long_soak_samples
        and $soak.ok == $soak.samples
        and $soak.fail == 0
      )
    )
  ' >/dev/null

soak_evidence_sha256=""
if [[ "$SOAK_JSON" != "null" ]]; then
  soak_evidence_sha256="$(printf '%s' "$SOAK_JSON" | shasum -a 256 | awk '{print $1}')"
fi

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_gate" \
  --arg soak_command "HEPTA_SOAK_SAMPLES=$MIN_LONG_SOAK_SAMPLES HEPTA_SOAK_INTERVAL_SECONDS=$SOAK_INTERVAL_SECONDS scripts/hepta-live-soak.sh" \
  --arg soak_evidence_sha256 "$soak_evidence_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson receipt "$RECEIPT_JSON" \
  --argjson soak "$SOAK_JSON" \
  --arg run_soak "$RUN_SOAK" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_receipt_gate:$receipt.gate,
    source_receipt_ready:$receipt.approval_evidence_receipt_ready,
    source_receipt_payload_sha256:$receipt.receipt_payload_sha256,
    pre_activation_soak_evidence_gate_ready:true,
    minimum_required_samples:$min_long_soak_samples,
    required_soak_command:$soak_command,
    long_soak_executed_by_this_gate:($run_soak == "1"),
    long_soak_execution_default_enabled:false,
    long_soak_evidence_candidate_ready:(
      if $run_soak == "1" then
        ($soak.status == "ready" and $soak.ok == $soak.samples and $soak.fail == 0)
      else
        true
      end
    ),
    long_soak_evidence_persisted:false,
    long_soak_evidence_sha256:$soak_evidence_sha256,
    observed_long_soak:(
      if $run_soak == "1" then {
        status:$soak.status,
        samples:$soak.samples,
        ok:$soak.ok,
        fail:$soak.fail,
        active_owner:$soak.active_owner,
        legacy_owner_preserved:$soak.legacy_owner_preserved,
        telegram_live_send_enabled:$soak.telegram_live_send_enabled,
        native_post_real_activation_enabled:$soak.native_post_real_activation_enabled
      } else null end
    ),
    activation_allowed:false,
    live_mutation_execution_ready:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    operator_approval_recorded:false,
    required_before_activation:[
      "explicit_operator_approval_id",
      "single_surface_activation_scope",
      "fresh_trusted_evidence_record",
      "current_installed_binary_backup_after_approval",
      "reviewed_rollback_plan",
      "fresh_24_sample_pre_activation_soak_evidence",
      "persisted_no_secret_side_effect_receipt",
      "post_activation_watchdog",
      "post_activation_minimum_24_sample_soak"
    ],
    denied_by_soak_gate:[
      "operator_approval_not_recorded",
      "single_surface_activation_scope_missing",
      "fresh_trusted_evidence_not_persisted",
      "pre_activation_soak_evidence_not_persisted",
      "post_activation_soak_not_executed"
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
      external_send_performed:false,
      credential_read:false
    }
  }')"

jq -e '
  .status == "ready"
  and .pre_activation_soak_evidence_gate_ready == true
  and .source_receipt_ready == true
  and .minimum_required_samples >= 24
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .receipt_persisted == false
  and .long_soak_evidence_persisted == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence gate passed"
