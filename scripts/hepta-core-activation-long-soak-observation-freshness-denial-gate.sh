#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
OBSERVATION_SOAK_SAMPLES="${HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_SAMPLES:-$MIN_LONG_SOAK_SAMPLES}"
OBSERVATION_SOAK_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS:-1}"
LONG_SOAK_EVIDENCE_MAX_AGE_MINUTES="${HEPTA_LONG_SOAK_EVIDENCE_MAX_AGE_MINUTES:-120}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

require_unsigned_integer() {
  local name="$1"
  local value="$2"

  case "$value" in
    ''|*[!0-9]*)
      echo "$name must be an unsigned integer" >&2
      exit 2
      ;;
  esac
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
require_unsigned_integer "HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_SAMPLES" "$OBSERVATION_SOAK_SAMPLES"
require_unsigned_integer "HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS" "$OBSERVATION_SOAK_INTERVAL_SECONDS"
require_unsigned_integer "HEPTA_LONG_SOAK_EVIDENCE_MAX_AGE_MINUTES" "$LONG_SOAK_EVIDENCE_MAX_AGE_MINUTES"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

if [[ "$OBSERVATION_SOAK_SAMPLES" -lt "$MIN_LONG_SOAK_SAMPLES" ]]; then
  echo "freshness observation samples must be at least HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" >&2
  exit 1
fi

if [[ "$LONG_SOAK_EVIDENCE_MAX_AGE_MINUTES" -lt 1 ]]; then
  echo "long-soak evidence max age must be at least 1 minute" >&2
  exit 1
fi

NON_ACCEPTANCE_JSON="$(
  capture_json_report \
    "hepta-core-activation-long-soak-observation-non-acceptance-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_LONG_SOAK_OBSERVATION_SAMPLES="$OBSERVATION_SOAK_SAMPLES" \
      HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS="$OBSERVATION_SOAK_INTERVAL_SECONDS" \
      scripts/hepta-core-activation-long-soak-observation-non-acceptance-gate.sh
)"

FRESHNESS_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-freshness-policy" \
    scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh
)"

non_acceptance_report_sha256="$(sha256_text "$NON_ACCEPTANCE_JSON")"
freshness_policy_report_sha256="$(sha256_text "$FRESHNESS_JSON")"
freshness_denial_policy_hash_sha256="$(sha256_text "hepta-core-activation-long-soak-observation-freshness-denial:policy:$non_acceptance_report_sha256:$freshness_policy_report_sha256:$MIN_LONG_SOAK_SAMPLES:$OBSERVATION_SOAK_SAMPLES:$OBSERVATION_SOAK_INTERVAL_SECONDS:$LONG_SOAK_EVIDENCE_MAX_AGE_MINUTES")"
freshness_denial_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-long-soak-observation-freshness-denial:side-effects:$non_acceptance_report_sha256:$freshness_policy_report_sha256:$MIN_LONG_SOAK_SAMPLES:$OBSERVATION_SOAK_SAMPLES:$OBSERVATION_SOAK_INTERVAL_SECONDS:$LONG_SOAK_EVIDENCE_MAX_AGE_MINUTES")"

jq -n -e \
  --argjson non_acceptance "$NON_ACCEPTANCE_JSON" \
  --argjson freshness "$FRESHNESS_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson observation_soak_samples "$OBSERVATION_SOAK_SAMPLES" \
  --argjson long_soak_evidence_max_age_minutes "$LONG_SOAK_EVIDENCE_MAX_AGE_MINUTES" \
  '
    $non_acceptance.runtime == "hepta"
    and $non_acceptance.status == "ready"
    and $non_acceptance.gate == "hepta_core_activation_long_soak_observation_non_acceptance_gate"
    and $non_acceptance.long_soak_observation_non_acceptance_ready == true
    and ($non_acceptance.release_long_soak_observed == true or $non_acceptance.observation_soak_known_operator_security_attention == true)
    and $non_acceptance.release_long_soak_sample_count == $observation_soak_samples
    and $non_acceptance.release_long_soak_sample_count >= $min_long_soak_samples
    and (($non_acceptance.release_long_soak_ok_count == $observation_soak_samples and $non_acceptance.release_long_soak_fail_count == 0)
      or ($non_acceptance.observation_soak_known_operator_security_attention == true and $non_acceptance.release_long_soak_ok_count == 0 and $non_acceptance.release_long_soak_fail_count == $observation_soak_samples))
    and $non_acceptance.observation_satisfies_long_soak_evidence == false
    and $non_acceptance.observation_satisfies_fresh_evidence == false
    and $non_acceptance.observation_satisfies_operator_approval == false
    and $non_acceptance.observation_satisfies_ledger_receipt == false
    and $non_acceptance.observation_satisfies_terminal_closure == false
    and $non_acceptance.long_soak_evidence_recorded == false
    and $non_acceptance.long_soak_evidence_persisted == false
    and $non_acceptance.long_soak_evidence_fresh == false
    and $non_acceptance.operator_approval_recorded == false
    and $non_acceptance.activation_request_recorded == false
    and $non_acceptance.receipt_persisted == false
    and $non_acceptance.receipt_accepted == false
    and $non_acceptance.ledger_recorded == false
    and $non_acceptance.terminal_closure_verdict == "blocked"
    and $non_acceptance.terminal_closure_recorded == false
    and $non_acceptance.terminal_closure_accepted == false
    and $non_acceptance.activation_allowed == false
    and $non_acceptance.live_mutation_execution_ready == false
    and $non_acceptance.public_release_claim_allowed == false
    and $non_acceptance.release_artifact_write_allowed == false
    and ($non_acceptance.side_effects | to_entries | all(
      .value == false
      or .key == "watchdog_http_read_performed"
      or .key == "soak_http_read_performed"
      or .key == "terminal_closure_source_read_performed"
    ))
    and $freshness.product == "Hepta"
    and $freshness.status == "ready"
    and $freshness.policy_id == "upstream-codex-activation-evidence-freshness-policy"
    and $freshness.policy_status.required_evidence_count == 8
    and $freshness.policy_status.policy_entry_count == 8
    and $freshness.policy_status.missing_evidence_count == 8
    and $freshness.policy_status.fresh_evidence_count == 0
    and $freshness.policy_status.freshness_policy_ready == true
    and $freshness.policy_status.activation_blocked_by_freshness_policy == true
    and $freshness.policy_status.activation_allowed_by_freshness_policy == false
    and $freshness.policy_status.active_wiring_allowed == false
    and ($freshness.evidence_freshness_entries | length) == 8
    and ($freshness.evidence_freshness_entries | any(.evidence_id == "long_soak_evidence_id" and .max_age_policy == "120 minutes" and .recorded == false and .fresh == false))
    and ($freshness.evidence_freshness_entries | all(.recorded == false and .fresh == false))
    and $freshness.denied_active_decisions.public_release_claim_allowed == false
    and $freshness.denied_active_decisions.release_artifact_write_allowed == false
    and ($freshness.side_effects | to_entries | all(.value == false))
    and $long_soak_evidence_max_age_minutes >= 1
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_long_soak_observation_freshness_denial_gate" \
  --arg non_acceptance_report_sha256 "$non_acceptance_report_sha256" \
  --arg freshness_policy_report_sha256 "$freshness_policy_report_sha256" \
  --arg freshness_denial_policy_hash_sha256 "$freshness_denial_policy_hash_sha256" \
  --arg freshness_denial_side_effect_hash_sha256 "$freshness_denial_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson observation_soak_samples "$OBSERVATION_SOAK_SAMPLES" \
  --argjson observation_soak_interval_seconds "$OBSERVATION_SOAK_INTERVAL_SECONDS" \
  --argjson long_soak_evidence_max_age_minutes "$LONG_SOAK_EVIDENCE_MAX_AGE_MINUTES" \
  --argjson non_acceptance "$NON_ACCEPTANCE_JSON" \
  --argjson freshness "$FRESHNESS_JSON" \
  '
    ([
      {
        id:"observation-without-trusted-record",
        fixture_kind:"observation_only_no_evidence_id",
        blocked:true,
        long_soak_observed:$non_acceptance.release_long_soak_observed,
        known_operator_security_attention:($non_acceptance.observation_soak_known_operator_security_attention // false),
        evidence_recorded:false,
        evidence_fresh:false,
        activation_allowed:false,
        reason:"24-sample observation or known attention-blocked observation has no trusted evidence id"
      },
      {
        id:"stale-long-soak-evidence-record",
        fixture_kind:"recorded_but_stale_long_soak",
        blocked:true,
        max_age_minutes:$long_soak_evidence_max_age_minutes,
        evidence_recorded:true,
        evidence_fresh:false,
        activation_allowed:false,
        reason:"recorded long-soak evidence must be rejected after the freshness window"
      },
      {
        id:"insufficient-sample-count",
        fixture_kind:"sample_count_below_release_long_soak_minimum",
        blocked:true,
        observed_sample_count:($min_long_soak_samples - 1),
        required_sample_count:$min_long_soak_samples,
        evidence_recorded:false,
        evidence_fresh:false,
        activation_allowed:false,
        reason:"short or partial soak cannot satisfy release long-soak evidence"
      },
      {
        id:"source-report-hash-mismatch",
        fixture_kind:"long_soak_report_hash_mismatch",
        blocked:true,
        source_report_sha256:$non_acceptance_report_sha256,
        evidence_recorded:true,
        evidence_fresh:false,
        activation_allowed:false,
        reason:"freshness requires the evidence record hash to match the observed source report"
      },
      {
        id:"scope-mismatch",
        fixture_kind:"activation_scope_mismatch",
        blocked:true,
        evidence_recorded:true,
        evidence_fresh:false,
        activation_allowed:false,
        reason:"fresh long-soak evidence must be bound to the same activation request and scope"
      },
      {
        id:"operator-approval-missing",
        fixture_kind:"fresh_soak_without_operator_approval",
        blocked:true,
        evidence_recorded:true,
        evidence_fresh:true,
        operator_approval_recorded:false,
        activation_allowed:false,
        reason:"fresh evidence alone cannot replace explicit operator approval"
      },
      {
        id:"ledger-receipt-missing",
        fixture_kind:"fresh_soak_without_ledger_receipt",
        blocked:true,
        evidence_recorded:true,
        evidence_fresh:true,
        ledger_recorded:false,
        receipt_accepted:false,
        activation_allowed:false,
        reason:"fresh evidence must still pass ledger and receipt closure"
      },
      {
        id:"public-or-artifact-attempt",
        fixture_kind:"freshness_claim_with_public_or_artifact_write",
        blocked:true,
        evidence_recorded:false,
        evidence_fresh:false,
        public_release_claim_allowed:false,
        release_artifact_write_allowed:false,
        activation_allowed:false,
        reason:"freshness checks never authorize public claims or artifact writes"
      }
    ]) as $fixtures
    | ([
      "long_soak_observation_not_a_trusted_evidence_record",
      "long_soak_observation_not_fresh_evidence",
      "trusted_long_soak_evidence_record_missing",
      "stale_long_soak_evidence_rejected",
      "insufficient_sample_count_rejected",
      "long_soak_report_hash_mismatch_rejected",
      "activation_scope_mismatch_rejected",
      "operator_approval_missing",
      "ledger_record_missing",
      "receipt_acceptance_missing",
      "terminal_closure_missing",
      "activation_denied",
      "live_mutation_denied",
      "active_binary_mutation_denied",
      "install_restart_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "credential_secret_read_denied",
      "upstream_fetch_denied",
      "upstream_merge_denied"
    ]) as $denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      long_soak_observation_freshness_denial_schema_version:"hepta_core_activation_long_soak_observation_freshness_denial_v1",
      long_soak_observation_freshness_denial_ready:true,
      freshness_denial_mode:"observation_only_and_stale_or_mismatched_evidence_rejected_no_activation_no_persistence",
      freshness_decision:"blocked_until_fresh_trusted_24_sample_long_soak_evidence_operator_approval_matching_scope_ledger_receipt_and_terminal_closure_exist",
      required_source_count:2,
      ready_source_count:2,
      activation_blocking_source_count:2,
      source_non_acceptance_gate:$non_acceptance.gate,
      source_freshness_policy_gate:$freshness.freshness_policy_gate,
      source_non_acceptance_report_sha256:$non_acceptance_report_sha256,
      source_freshness_policy_report_sha256:$freshness_policy_report_sha256,
      freshness_denial_policy_hash_sha256:$freshness_denial_policy_hash_sha256,
      freshness_denial_side_effect_hash_sha256:$freshness_denial_side_effect_hash_sha256,
      minimum_required_long_soak_samples:$min_long_soak_samples,
      observation_soak_samples:$observation_soak_samples,
      observation_soak_interval_seconds:$observation_soak_interval_seconds,
      long_soak_evidence_max_age_minutes:$long_soak_evidence_max_age_minutes,
      observation_soak_status:$non_acceptance.observation_soak_status,
      observation_soak_known_operator_security_attention:($non_acceptance.observation_soak_known_operator_security_attention // false),
      observation_soak_passed:($non_acceptance.observation_soak_passed // false),
      release_long_soak_observed:$non_acceptance.release_long_soak_observed,
      release_long_soak_sample_count:$non_acceptance.release_long_soak_sample_count,
      release_long_soak_ok_count:$non_acceptance.release_long_soak_ok_count,
      release_long_soak_fail_count:$non_acceptance.release_long_soak_fail_count,
      source_required_evidence_count:$freshness.policy_status.required_evidence_count,
      source_missing_evidence_count:$freshness.policy_status.missing_evidence_count,
      source_fresh_evidence_count:$freshness.policy_status.fresh_evidence_count,
      source_freshness_policy_ready:$freshness.policy_status.freshness_policy_ready,
      source_activation_blocked_by_freshness_policy:$freshness.policy_status.activation_blocked_by_freshness_policy,
      long_soak_observation_recorded_as_evidence:false,
      long_soak_observation_persisted_as_evidence:false,
      long_soak_observation_accepted_as_fresh_evidence:false,
      long_soak_evidence_recorded:false,
      long_soak_evidence_persisted:false,
      long_soak_evidence_fresh:false,
      stale_long_soak_evidence_accepted:false,
      mismatched_long_soak_evidence_accepted:false,
      scope_mismatched_long_soak_evidence_accepted:false,
      fresh_evidence_count:0,
      fresh_trusted_record_count:0,
      operator_approval_recorded:false,
      activation_request_recorded:false,
      ledger_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      terminal_closure_recorded:false,
      terminal_closure_accepted:false,
      activation_allowed:false,
      live_mutation_execution_ready:false,
      active_binary_mutation_allowed:false,
      install_execution_allowed:false,
      active_service_restart_allowed:false,
      public_release_claim_allowed:false,
      release_artifact_write_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      credential_secret_read_allowed:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      required_freshness_fixture_count:($fixtures | length),
      blocked_freshness_fixture_count:($fixtures | map(select(.blocked == true)) | length),
      allowed_freshness_fixture_count:($fixtures | map(select(.blocked == false)) | length),
      freshness_denial_fixtures:$fixtures,
      denied_by_long_soak_observation_freshness_denial:$denied,
      denied_by_long_soak_observation_freshness_denial_count:($denied | length),
      side_effects:{
        non_acceptance_source_read_performed:true,
        freshness_policy_source_read_performed:true,
        watchdog_http_read_performed:true,
        soak_http_read_performed:true,
        activation_performed:false,
        active_runtime_mutated:false,
        active_binary_mutated:false,
        active_service_restarted:false,
        launchd_restarted:false,
        release_artifact_written:false,
        public_artifact_written:false,
        public_release_published:false,
        public_claim_sent:false,
        workspace_write_performed:false,
        evidence_record_persisted:false,
        evidence_record_accepted:false,
        stale_evidence_accepted:false,
        mismatched_evidence_accepted:false,
        receipt_persisted:false,
        receipt_accepted:false,
        ledger_recorded:false,
        terminal_closure_recorded:false,
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        provider_model_invoked:false,
        channel_delivery_performed:false,
        credential_secret_read:false,
        upstream_fetch_performed:false,
        upstream_merge_performed:false
      }
    }
  ')"

printf '%s\n' "$report" | jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_long_soak_observation_freshness_denial_gate"
  and .long_soak_observation_freshness_denial_ready == true
  and .required_source_count == 2
  and .ready_source_count == 2
  and .activation_blocking_source_count == 2
  and .minimum_required_long_soak_samples >= 24
  and .observation_soak_samples >= .minimum_required_long_soak_samples
  and (.release_long_soak_observed == true or .observation_soak_known_operator_security_attention == true)
  and .release_long_soak_sample_count == .observation_soak_samples
  and ((.release_long_soak_ok_count == .observation_soak_samples and .release_long_soak_fail_count == 0)
    or (.observation_soak_known_operator_security_attention == true and .release_long_soak_ok_count == 0 and .release_long_soak_fail_count == .observation_soak_samples))
  and .source_required_evidence_count == 8
  and .source_missing_evidence_count == 8
  and .source_fresh_evidence_count == 0
  and .source_freshness_policy_ready == true
  and .source_activation_blocked_by_freshness_policy == true
  and .long_soak_observation_recorded_as_evidence == false
  and .long_soak_observation_persisted_as_evidence == false
  and .long_soak_observation_accepted_as_fresh_evidence == false
  and .long_soak_evidence_recorded == false
  and .long_soak_evidence_persisted == false
  and .long_soak_evidence_fresh == false
  and .stale_long_soak_evidence_accepted == false
  and .mismatched_long_soak_evidence_accepted == false
  and .scope_mismatched_long_soak_evidence_accepted == false
  and .fresh_evidence_count == 0
  and .fresh_trusted_record_count == 0
  and .operator_approval_recorded == false
  and .activation_request_recorded == false
  and .ledger_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .active_binary_mutation_allowed == false
  and .install_execution_allowed == false
  and .active_service_restart_allowed == false
  and .public_release_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .required_freshness_fixture_count == 8
  and .blocked_freshness_fixture_count == 8
  and .allowed_freshness_fixture_count == 0
  and (.freshness_denial_fixtures | length) == 8
  and (.freshness_denial_fixtures | all(.blocked == true and .activation_allowed == false))
  and .denied_by_long_soak_observation_freshness_denial_count == 22
  and (.denied_by_long_soak_observation_freshness_denial | length) == 22
  and (.side_effects | to_entries | all(
    .value == false
    or .key == "non_acceptance_source_read_performed"
    or .key == "freshness_policy_source_read_performed"
    or .key == "watchdog_http_read_performed"
    or .key == "soak_http_read_performed"
  ))
' >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta core activation long-soak observation freshness denial gate passed"
