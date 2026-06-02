#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
OBSERVATION_SOAK_SAMPLES="${HEPTA_LONG_SOAK_OBSERVATION_SAMPLES:-$MIN_LONG_SOAK_SAMPLES}"
OBSERVATION_SOAK_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS:-1}"

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
require_unsigned_integer "HEPTA_LONG_SOAK_OBSERVATION_SAMPLES" "$OBSERVATION_SOAK_SAMPLES"
require_unsigned_integer "HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS" "$OBSERVATION_SOAK_INTERVAL_SECONDS"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

if [[ "$OBSERVATION_SOAK_SAMPLES" -lt "$MIN_LONG_SOAK_SAMPLES" ]]; then
  echo "long-soak observation samples must be at least HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" >&2
  exit 1
fi

LONG_SOAK_OBSERVATION_JSON="$(
  capture_json_report \
    "hepta-terminal-watchdog-soak-regression-gate-long-observation" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_TERMINAL_SOAK_SAMPLES="$OBSERVATION_SOAK_SAMPLES" \
      HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS="$OBSERVATION_SOAK_INTERVAL_SECONDS" \
      scripts/hepta-terminal-watchdog-soak-regression-gate.sh
)"

TERMINAL_CLOSURE_JSON="$(
  capture_json_report \
    "hepta-core-activation-evidence-receipt-terminal-closure-decision-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-evidence-receipt-terminal-closure-decision-gate.sh
)"

long_soak_observation_report_sha256="$(sha256_text "$LONG_SOAK_OBSERVATION_JSON")"
terminal_closure_report_sha256="$(sha256_text "$TERMINAL_CLOSURE_JSON")"
non_acceptance_policy_hash_sha256="$(sha256_text "hepta-core-activation-long-soak-observation-non-acceptance:policy:$long_soak_observation_report_sha256:$terminal_closure_report_sha256:$MIN_LONG_SOAK_SAMPLES:$OBSERVATION_SOAK_SAMPLES:$OBSERVATION_SOAK_INTERVAL_SECONDS")"
non_acceptance_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-long-soak-observation-non-acceptance:side-effects:$long_soak_observation_report_sha256:$terminal_closure_report_sha256:$MIN_LONG_SOAK_SAMPLES:$OBSERVATION_SOAK_SAMPLES:$OBSERVATION_SOAK_INTERVAL_SECONDS")"

jq -n -e \
  --argjson observation "$LONG_SOAK_OBSERVATION_JSON" \
  --argjson closure "$TERMINAL_CLOSURE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson observation_soak_samples "$OBSERVATION_SOAK_SAMPLES" \
  '
    $observation.runtime == "hepta"
    and $observation.status == "ready"
    and $observation.gate == "hepta_terminal_watchdog_soak_regression_gate"
    and $observation.terminal_soak_samples == $observation_soak_samples
    and $observation.terminal_soak_samples >= $min_long_soak_samples
    and $observation.minimum_long_soak_required_samples == $min_long_soak_samples
    and $observation.terminal_soak_is_release_long_soak == true
    and $observation.terminal_soak_regression_class == "release_long_soak_observation"
    and ($observation.release_long_soak_observed == true or $observation.soak_known_operator_security_attention == true)
    and $observation.release_long_soak_sample_count == $observation.soak_samples
    and (($observation.soak_ok == $observation.soak_samples and $observation.soak_fail == 0)
      or ($observation.soak_known_operator_security_attention == true and $observation.soak_ok == 0 and $observation.soak_fail == $observation.soak_samples))
    and $observation.release_long_soak_evidence_recorded == false
    and $observation.release_long_soak_evidence_persisted == false
    and $observation.release_long_soak_evidence_accepted == false
    and $observation.release_long_soak_authorizes_activation == false
    and $observation.terminal_soak_authorizes_live_mutation == false
    and $observation.terminal_soak_authorizes_public_claim == false
    and $observation.terminal_soak_authorizes_public_distribution == false
    and $observation.public_release_claim_allowed == false
    and $observation.release_artifact_write_allowed == false
    and $observation.provider_model_invocation_allowed == false
    and $observation.channel_delivery_allowed == false
    and ($observation.side_effects | to_entries | all(.value == false))
    and $closure.runtime == "hepta"
    and $closure.status == "ready"
    and $closure.gate == "hepta_core_activation_evidence_receipt_terminal_closure_decision_gate"
    and $closure.terminal_closure_decision_gate_ready == true
    and $closure.terminal_closure_verdict == "blocked"
    and $closure.operator_approval_recorded == false
    and $closure.activation_request_recorded == false
    and $closure.long_soak_evidence_recorded == false
    and $closure.long_soak_evidence_persisted == false
    and $closure.long_soak_evidence_fresh == false
    and $closure.evidence_receipt_persisted == false
    and $closure.receipt_persisted == false
    and $closure.receipt_accepted == false
    and $closure.ledger_recorded == false
    and $closure.index_recorded == false
    and $closure.delivery_recorded == false
    and $closure.completion_ack_recorded == false
    and $closure.terminal_closure_recorded == false
    and $closure.terminal_closure_accepted == false
    and $closure.activation_allowed == false
    and $closure.live_mutation_execution_ready == false
    and $closure.public_release_claim_allowed == false
    and $closure.release_artifact_write_allowed == false
    and $closure.provider_model_invocation_allowed == false
    and $closure.channel_delivery_allowed == false
    and ($closure.side_effects | to_entries | all(.value == false))
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_long_soak_observation_non_acceptance_gate" \
  --arg long_soak_observation_report_sha256 "$long_soak_observation_report_sha256" \
  --arg terminal_closure_report_sha256 "$terminal_closure_report_sha256" \
  --arg non_acceptance_policy_hash_sha256 "$non_acceptance_policy_hash_sha256" \
  --arg non_acceptance_side_effect_hash_sha256 "$non_acceptance_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson observation_soak_samples "$OBSERVATION_SOAK_SAMPLES" \
  --argjson observation_soak_interval_seconds "$OBSERVATION_SOAK_INTERVAL_SECONDS" \
  --argjson observation "$LONG_SOAK_OBSERVATION_JSON" \
  --argjson closure "$TERMINAL_CLOSURE_JSON" \
  '
    ([
      "release_long_soak_observation_not_activation_evidence",
      "release_long_soak_observation_recording_denied",
      "release_long_soak_observation_persistence_denied",
      "release_long_soak_observation_acceptance_denied",
      "fresh_trusted_long_soak_evidence_record_missing",
      "operator_approval_not_recorded",
      "activation_request_not_recorded",
      "ledger_record_not_recorded",
      "index_record_not_recorded",
      "delivery_record_not_recorded",
      "completion_ack_not_recorded",
      "receipt_not_persisted",
      "receipt_not_accepted",
      "terminal_closure_not_recorded",
      "terminal_closure_not_accepted",
      "activation_denied",
      "live_mutation_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "install_restart_denied",
      "upstream_fetch_merge_denied"
    ]) as $denied
    | ([
      {
        id:"explicit-release-long-soak-observation",
        ready:true,
        blocked:true,
        source_gate:$observation.gate,
        source_report_sha256:$long_soak_observation_report_sha256,
        observed:$observation.release_long_soak_observed,
        known_operator_security_attention:($observation.soak_known_operator_security_attention // false),
        samples:$observation.release_long_soak_sample_count,
        reason:"24-sample-class observation is present or blocked by known operator-security attention, and remains observation-only"
      },
      {
        id:"observation-non-evidence-boundary",
        ready:true,
        blocked:true,
        release_long_soak_evidence_recorded:false,
        release_long_soak_evidence_persisted:false,
        release_long_soak_evidence_accepted:false,
        reason:"observation does not create a fresh trusted evidence record"
      },
      {
        id:"terminal-closure-transitive-denial",
        ready:true,
        blocked:true,
        source_gate:$closure.gate,
        source_report_sha256:$terminal_closure_report_sha256,
        terminal_closure_verdict:$closure.terminal_closure_verdict,
        reason:"terminal closure remains blocked without approval, ledger, receipt, delivery, and ack records"
      },
      {
        id:"activation-publication-artifact-boundary",
        ready:true,
        blocked:true,
        activation_allowed:false,
        live_mutation_execution_ready:false,
        public_release_claim_allowed:false,
        release_artifact_write_allowed:false,
        reason:"observation cannot authorize activation, live mutation, public claim, or artifact write"
      }
    ]) as $families
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      long_soak_observation_non_acceptance_schema_version:"hepta_core_activation_long_soak_observation_non_acceptance_v1",
      long_soak_observation_non_acceptance_ready:true,
      non_acceptance_mode:"explicit_24_sample_observation_bound_to_terminal_closure_denial_no_persistence_no_activation",
      non_acceptance_decision:"release_long_soak_observed_but_not_recorded_persisted_accepted_or_authorized_as_activation_evidence",
      required_source_count:2,
      ready_source_count:2,
      activation_blocking_source_count:2,
      source_long_soak_observation_gate:$observation.gate,
      source_terminal_closure_gate:$closure.gate,
      source_long_soak_observation_report_sha256:$long_soak_observation_report_sha256,
      source_terminal_closure_report_sha256:$terminal_closure_report_sha256,
      non_acceptance_policy_hash_sha256:$non_acceptance_policy_hash_sha256,
      non_acceptance_side_effect_hash_sha256:$non_acceptance_side_effect_hash_sha256,
      minimum_required_long_soak_samples:$min_long_soak_samples,
      observation_soak_samples:$observation_soak_samples,
      observation_soak_interval_seconds:$observation_soak_interval_seconds,
      observation_class:$observation.terminal_soak_regression_class,
      observation_soak_status:$observation.soak_status,
      observation_soak_known_operator_security_attention:($observation.soak_known_operator_security_attention // false),
      observation_soak_passed:($observation.soak_passed // false),
      release_long_soak_observed:$observation.release_long_soak_observed,
      release_long_soak_sample_count:$observation.release_long_soak_sample_count,
      release_long_soak_ok_count:$observation.soak_ok,
      release_long_soak_fail_count:$observation.soak_fail,
      observation_satisfies_long_soak_evidence:false,
      observation_satisfies_fresh_evidence:false,
      observation_satisfies_operator_approval:false,
      observation_satisfies_ledger_receipt:false,
      observation_satisfies_terminal_closure:false,
      release_long_soak_evidence_recorded:false,
      release_long_soak_evidence_persisted:false,
      release_long_soak_evidence_accepted:false,
      long_soak_evidence_recorded:false,
      long_soak_evidence_persisted:false,
      long_soak_evidence_fresh:false,
      operator_approval_recorded:false,
      activation_request_recorded:false,
      evidence_receipt_persisted:false,
      receipt_persisted:false,
      receipt_accepted:false,
      ledger_recorded:false,
      index_recorded:false,
      delivery_recorded:false,
      completion_ack_recorded:false,
      terminal_closure_verdict:$closure.terminal_closure_verdict,
      terminal_closure_recorded:false,
      terminal_closure_accepted:false,
      activation_allowed:false,
      live_mutation_execution_ready:false,
      public_release_claim_allowed:false,
      release_artifact_write_allowed:false,
      public_distribution_publication_allowed:false,
      install_execution_allowed:false,
      active_service_restart_allowed:false,
      active_binary_mutation_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      credential_secret_read_allowed:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      readiness_families:$families,
      denied_by_long_soak_observation_non_acceptance:$denied,
      denied_by_long_soak_observation_non_acceptance_count:($denied | length),
      side_effects:{
        watchdog_http_read_performed:true,
        soak_http_read_performed:true,
        terminal_closure_source_read_performed:true,
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
        receipt_persisted:false,
        receipt_accepted:false,
        ledger_recorded:false,
        index_recorded:false,
        delivery_recorded:false,
        completion_ack_recorded:false,
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
  and .gate == "hepta_core_activation_long_soak_observation_non_acceptance_gate"
  and .long_soak_observation_non_acceptance_ready == true
  and .required_source_count == 2
  and .ready_source_count == 2
  and .activation_blocking_source_count == 2
  and .minimum_required_long_soak_samples >= 24
  and .observation_soak_samples >= .minimum_required_long_soak_samples
  and .observation_class == "release_long_soak_observation"
  and (.release_long_soak_observed == true or .observation_soak_known_operator_security_attention == true)
  and .release_long_soak_sample_count == .observation_soak_samples
  and ((.release_long_soak_ok_count == .observation_soak_samples and .release_long_soak_fail_count == 0)
    or (.observation_soak_known_operator_security_attention == true and .release_long_soak_ok_count == 0 and .release_long_soak_fail_count == .observation_soak_samples))
  and .observation_satisfies_long_soak_evidence == false
  and .observation_satisfies_fresh_evidence == false
  and .observation_satisfies_operator_approval == false
  and .observation_satisfies_ledger_receipt == false
  and .observation_satisfies_terminal_closure == false
  and .release_long_soak_evidence_recorded == false
  and .release_long_soak_evidence_persisted == false
  and .release_long_soak_evidence_accepted == false
  and .long_soak_evidence_recorded == false
  and .long_soak_evidence_persisted == false
  and .long_soak_evidence_fresh == false
  and .operator_approval_recorded == false
  and .activation_request_recorded == false
  and .evidence_receipt_persisted == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .ledger_recorded == false
  and .index_recorded == false
  and .delivery_recorded == false
  and .completion_ack_recorded == false
  and .terminal_closure_verdict == "blocked"
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .public_release_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and (.readiness_families | length) == 4
  and (.readiness_families | all(.ready == true and .blocked == true))
  and .denied_by_long_soak_observation_non_acceptance_count == 23
  and (.denied_by_long_soak_observation_non_acceptance | length) == 23
  and (.side_effects | to_entries | all(.value == false or .key == "watchdog_http_read_performed" or .key == "soak_http_read_performed" or .key == "terminal_closure_source_read_performed"))
' >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta core activation long-soak observation non-acceptance gate passed"
