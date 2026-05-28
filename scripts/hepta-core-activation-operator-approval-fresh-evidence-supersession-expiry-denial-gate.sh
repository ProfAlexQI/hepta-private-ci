#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
OBSERVATION_SOAK_SAMPLES="${HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_SAMPLES:-$MIN_LONG_SOAK_SAMPLES}"
OBSERVATION_SOAK_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS:-1}"
FRESH_EVIDENCE_MAX_AGE_MINUTES="${HEPTA_CORE_ACTIVATION_FRESH_EVIDENCE_MAX_AGE_MINUTES:-120}"
OPERATOR_APPROVAL_MAX_AGE_MINUTES="${HEPTA_CORE_ACTIVATION_OPERATOR_APPROVAL_MAX_AGE_MINUTES:-120}"

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
require_unsigned_integer "HEPTA_CORE_ACTIVATION_FRESH_EVIDENCE_MAX_AGE_MINUTES" "$FRESH_EVIDENCE_MAX_AGE_MINUTES"
require_unsigned_integer "HEPTA_CORE_ACTIVATION_OPERATOR_APPROVAL_MAX_AGE_MINUTES" "$OPERATOR_APPROVAL_MAX_AGE_MINUTES"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

if [[ "$OBSERVATION_SOAK_SAMPLES" -lt "$MIN_LONG_SOAK_SAMPLES" ]]; then
  echo "freshness observation samples must be at least HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" >&2
  exit 1
fi

if [[ "$FRESH_EVIDENCE_MAX_AGE_MINUTES" -lt 1 ]]; then
  echo "fresh evidence max age must be at least 1 minute" >&2
  exit 1
fi

if [[ "$OPERATOR_APPROVAL_MAX_AGE_MINUTES" -lt 1 ]]; then
  echo "operator approval max age must be at least 1 minute" >&2
  exit 1
fi

FRESHNESS_DENIAL_JSON="$(
  capture_json_report \
    "hepta-core-activation-long-soak-observation-freshness-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_SAMPLES="$OBSERVATION_SOAK_SAMPLES" \
      HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS="$OBSERVATION_SOAK_INTERVAL_SECONDS" \
      HEPTA_LONG_SOAK_EVIDENCE_MAX_AGE_MINUTES="$FRESH_EVIDENCE_MAX_AGE_MINUTES" \
      scripts/hepta-core-activation-long-soak-observation-freshness-denial-gate.sh
)"

APPROVAL_PACKET_JSON="$(
  capture_json_report \
    "hepta-core-activation-long-soak-operator-approval-packet-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-long-soak-operator-approval-packet-gate.sh
)"

freshness_denial_report_sha256="$(sha256_text "$FRESHNESS_DENIAL_JSON")"
approval_packet_report_sha256="$(sha256_text "$APPROVAL_PACKET_JSON")"
supersession_expiry_policy_hash_sha256="$(sha256_text "hepta-core-activation-operator-approval-fresh-evidence-supersession-expiry-denial:policy:$freshness_denial_report_sha256:$approval_packet_report_sha256:$MIN_LONG_SOAK_SAMPLES:$OBSERVATION_SOAK_SAMPLES:$OBSERVATION_SOAK_INTERVAL_SECONDS:$FRESH_EVIDENCE_MAX_AGE_MINUTES:$OPERATOR_APPROVAL_MAX_AGE_MINUTES")"
supersession_expiry_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-operator-approval-fresh-evidence-supersession-expiry-denial:side-effects:$freshness_denial_report_sha256:$approval_packet_report_sha256:$MIN_LONG_SOAK_SAMPLES:$OBSERVATION_SOAK_SAMPLES:$OBSERVATION_SOAK_INTERVAL_SECONDS:$FRESH_EVIDENCE_MAX_AGE_MINUTES:$OPERATOR_APPROVAL_MAX_AGE_MINUTES")"

jq -n -e \
  --argjson freshness "$FRESHNESS_DENIAL_JSON" \
  --argjson approval "$APPROVAL_PACKET_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson observation_soak_samples "$OBSERVATION_SOAK_SAMPLES" \
  --argjson fresh_evidence_max_age_minutes "$FRESH_EVIDENCE_MAX_AGE_MINUTES" \
  --argjson operator_approval_max_age_minutes "$OPERATOR_APPROVAL_MAX_AGE_MINUTES" \
  '
    $freshness.runtime == "hepta"
    and $freshness.status == "ready"
    and $freshness.gate == "hepta_core_activation_long_soak_observation_freshness_denial_gate"
    and $freshness.long_soak_observation_freshness_denial_ready == true
    and $freshness.required_source_count == 2
    and $freshness.ready_source_count == 2
    and $freshness.activation_blocking_source_count == 2
    and $freshness.minimum_required_long_soak_samples >= 24
    and $freshness.observation_soak_samples == $observation_soak_samples
    and $freshness.release_long_soak_observed == true
    and $freshness.release_long_soak_sample_count == $observation_soak_samples
    and $freshness.release_long_soak_ok_count == $observation_soak_samples
    and $freshness.release_long_soak_fail_count == 0
    and $freshness.long_soak_evidence_max_age_minutes == $fresh_evidence_max_age_minutes
    and $freshness.fresh_evidence_count == 0
    and $freshness.fresh_trusted_record_count == 0
    and $freshness.operator_approval_recorded == false
    and $freshness.activation_request_recorded == false
    and $freshness.ledger_recorded == false
    and $freshness.receipt_accepted == false
    and $freshness.terminal_closure_accepted == false
    and $freshness.activation_allowed == false
    and $freshness.live_mutation_execution_ready == false
    and $freshness.required_freshness_fixture_count == 8
    and $freshness.blocked_freshness_fixture_count == 8
    and $freshness.allowed_freshness_fixture_count == 0
    and ($freshness.freshness_denial_fixtures | length) == 8
    and ($freshness.side_effects | to_entries | all(
      .value == false
      or .key == "non_acceptance_source_read_performed"
      or .key == "freshness_policy_source_read_performed"
      or .key == "watchdog_http_read_performed"
      or .key == "soak_http_read_performed"
    ))
    and $approval.runtime == "hepta"
    and $approval.status == "ready"
    and $approval.gate == "hepta_core_activation_long_soak_operator_approval_packet_gate"
    and $approval.long_soak_operator_approval_packet_ready == true
    and $approval.packet_mode == "schema_only_no_activation_no_persistence"
    and $approval.required_source_count == 5
    and $approval.ready_source_count == 5
    and $approval.activation_blocking_source_count == 5
    and $approval.minimum_required_long_soak_samples == $min_long_soak_samples
    and $approval.required_evidence_count == 8
    and $approval.missing_evidence_count == 8
    and $approval.fresh_evidence_count == 0
    and $approval.required_approval_packet_field_count == 16
    and $approval.recorded_approval_packet_field_count == 0
    and $approval.required_operator_approval_count == 8
    and $approval.long_soak_evidence_recorded == false
    and $approval.long_soak_evidence_fresh == false
    and $approval.operator_approval_recorded == false
    and $approval.operator_identity_hash_recorded == false
    and $approval.activation_request_recorded == false
    and $approval.approval_packet_recorded == false
    and $approval.approval_packet_persisted == false
    and $approval.approval_packet_accepted == false
    and $approval.operator_approved_activation_ready == false
    and $approval.activation_allowed == false
    and $approval.live_mutation_execution_ready == false
    and $approval.public_release_claim_allowed == false
    and $approval.release_artifact_write_allowed == false
    and ($approval.side_effects | to_entries | all(.value == false))
    and $operator_approval_max_age_minutes >= 1
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_operator_approval_fresh_evidence_supersession_expiry_denial_gate" \
  --arg freshness_denial_report_sha256 "$freshness_denial_report_sha256" \
  --arg approval_packet_report_sha256 "$approval_packet_report_sha256" \
  --arg supersession_expiry_policy_hash_sha256 "$supersession_expiry_policy_hash_sha256" \
  --arg supersession_expiry_side_effect_hash_sha256 "$supersession_expiry_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson observation_soak_samples "$OBSERVATION_SOAK_SAMPLES" \
  --argjson observation_soak_interval_seconds "$OBSERVATION_SOAK_INTERVAL_SECONDS" \
  --argjson fresh_evidence_max_age_minutes "$FRESH_EVIDENCE_MAX_AGE_MINUTES" \
  --argjson operator_approval_max_age_minutes "$OPERATOR_APPROVAL_MAX_AGE_MINUTES" \
  --argjson freshness "$FRESHNESS_DENIAL_JSON" \
  --argjson approval "$APPROVAL_PACKET_JSON" \
  '
    ([
      {
        id:"fresh-evidence-without-activation-request-binding",
        fixture_kind:"fresh_evidence_missing_activation_request_id",
        blocked:true,
        fresh_evidence_recorded:true,
        operator_approval_recorded:true,
        same_activation_request_binding:false,
        activation_allowed:false,
        reason:"fresh evidence must bind an activation_request_id"
      },
      {
        id:"fresh-evidence-activation-request-mismatch",
        fixture_kind:"fresh_evidence_bound_to_different_activation_request",
        blocked:true,
        fresh_evidence_activation_request_id:"activation-request-A",
        operator_approval_activation_request_id:"activation-request-B",
        same_activation_request_binding:false,
        activation_allowed:false,
        reason:"fresh evidence and operator approval must bind the same activation request"
      },
      {
        id:"expired-fresh-evidence",
        fixture_kind:"fresh_evidence_expired",
        blocked:true,
        max_age_minutes:$fresh_evidence_max_age_minutes,
        fresh_evidence_unexpired:false,
        activation_allowed:false,
        reason:"fresh evidence expires at the policy boundary"
      },
      {
        id:"superseded-fresh-evidence",
        fixture_kind:"fresh_evidence_superseded",
        blocked:true,
        fresh_evidence_superseded:true,
        activation_allowed:false,
        reason:"superseded fresh evidence cannot authorize activation"
      },
      {
        id:"operator-approval-without-evidence-binding",
        fixture_kind:"operator_approval_missing_fresh_evidence_binding",
        blocked:true,
        operator_approval_recorded:true,
        fresh_evidence_recorded:true,
        same_activation_request_binding:false,
        activation_allowed:false,
        reason:"operator approval must name the current fresh evidence record"
      },
      {
        id:"operator-approval-activation-request-mismatch",
        fixture_kind:"operator_approval_bound_to_different_activation_request",
        blocked:true,
        fresh_evidence_activation_request_id:"activation-request-current",
        operator_approval_activation_request_id:"activation-request-old",
        same_activation_request_binding:false,
        activation_allowed:false,
        reason:"operator approval cannot be borrowed across activation requests"
      },
      {
        id:"expired-operator-approval",
        fixture_kind:"operator_approval_expired",
        blocked:true,
        max_age_minutes:$operator_approval_max_age_minutes,
        operator_approval_unexpired:false,
        activation_allowed:false,
        reason:"operator approval expires at the policy boundary"
      },
      {
        id:"superseded-operator-approval",
        fixture_kind:"operator_approval_superseded",
        blocked:true,
        operator_approval_superseded:true,
        activation_allowed:false,
        reason:"superseded operator approval cannot authorize activation"
      },
      {
        id:"old-operator-approval-reused-after-new-request",
        fixture_kind:"old_operator_approval_reused_after_new_activation_request",
        blocked:true,
        old_operator_approval_reuse_attempted:true,
        activation_allowed:false,
        reason:"a new activation request invalidates reusable approval assumptions"
      },
      {
        id:"current-fresh-evidence-paired-with-old-approval",
        fixture_kind:"current_fresh_evidence_old_approval_pair",
        blocked:true,
        fresh_evidence_current:true,
        operator_approval_current:false,
        activation_allowed:false,
        reason:"current fresh evidence must be paired with current approval"
      },
      {
        id:"ledger-receipt-terminal-closure-from-superseded-pair",
        fixture_kind:"ledger_receipt_terminal_closure_from_superseded_pair",
        blocked:true,
        ledger_recorded:true,
        receipt_accepted:true,
        terminal_closure_recorded:true,
        pair_superseded:true,
        activation_allowed:false,
        reason:"downstream ledger and terminal records cannot revive a superseded pair"
      },
      {
        id:"public-artifact-install-or-live-mutation-attempt",
        fixture_kind:"supersession_expiry_claim_with_live_or_public_side_effect",
        blocked:true,
        public_release_claim_allowed:false,
        release_artifact_write_allowed:false,
        install_restart_allowed:false,
        active_binary_mutation_allowed:false,
        activation_allowed:false,
        reason:"supersession and expiry checks never authorize public or live mutation side effects"
      }
    ]) as $fixtures
    | ([
      "same_activation_request_binding_required",
      "fresh_evidence_activation_request_id_required",
      "operator_approval_activation_request_id_required",
      "fresh_evidence_unexpired_required",
      "operator_approval_unexpired_required",
      "fresh_evidence_not_superseded_required",
      "operator_approval_not_superseded_required",
      "old_operator_approval_reuse_denied",
      "cross_request_fresh_evidence_reuse_denied",
      "current_fresh_evidence_old_approval_pair_denied",
      "superseded_pair_ledger_receipt_terminal_closure_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "install_restart_denied",
      "active_binary_mutation_denied",
      "workspace_write_denied",
      "memory_store_mutation_denied",
      "credential_secret_read_denied"
    ]) as $denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      operator_approval_fresh_evidence_supersession_expiry_denial_schema_version:"hepta_core_activation_operator_approval_fresh_evidence_supersession_expiry_denial_v1",
      operator_approval_fresh_evidence_supersession_expiry_denial_ready:true,
      supersession_expiry_denial_mode:"schema_only_future_positive_fixture_rejected_no_activation_no_persistence",
      supersession_expiry_decision:"blocked_until_operator_approval_and_fresh_evidence_bind_same_current_activation_request_and_are_unexpired_unsuperseded",
      required_source_count:2,
      ready_source_count:2,
      activation_blocking_source_count:2,
      source_freshness_denial_gate:$freshness.gate,
      source_operator_approval_packet_gate:$approval.gate,
      source_freshness_denial_report_sha256:$freshness_denial_report_sha256,
      source_operator_approval_packet_report_sha256:$approval_packet_report_sha256,
      source_report_hashes:[
        $freshness_denial_report_sha256,
        $approval_packet_report_sha256
      ],
      supersession_expiry_policy_hash_sha256:$supersession_expiry_policy_hash_sha256,
      supersession_expiry_side_effect_hash_sha256:$supersession_expiry_side_effect_hash_sha256,
      minimum_required_long_soak_samples:$min_long_soak_samples,
      observation_soak_samples:$observation_soak_samples,
      observation_soak_interval_seconds:$observation_soak_interval_seconds,
      fresh_evidence_max_age_minutes:$fresh_evidence_max_age_minutes,
      operator_approval_max_age_minutes:$operator_approval_max_age_minutes,
      same_activation_request_binding_required:true,
      fresh_evidence_activation_request_binding_required:true,
      operator_approval_activation_request_binding_required:true,
      fresh_evidence_expiry_enforced:true,
      operator_approval_expiry_enforced:true,
      fresh_evidence_supersession_denied:true,
      operator_approval_supersession_denied:true,
      old_operator_approval_reuse_allowed:false,
      cross_request_fresh_evidence_reuse_allowed:false,
      expired_fresh_evidence_accepted:false,
      superseded_fresh_evidence_accepted:false,
      expired_operator_approval_accepted:false,
      superseded_operator_approval_accepted:false,
      current_fresh_evidence_old_approval_pair_accepted:false,
      superseded_pair_ledger_receipt_terminal_closure_accepted:false,
      release_long_soak_observed:$freshness.release_long_soak_observed,
      release_long_soak_sample_count:$freshness.release_long_soak_sample_count,
      release_long_soak_ok_count:$freshness.release_long_soak_ok_count,
      release_long_soak_fail_count:$freshness.release_long_soak_fail_count,
      source_freshness_denial_ready:$freshness.long_soak_observation_freshness_denial_ready,
      source_operator_approval_packet_ready:$approval.long_soak_operator_approval_packet_ready,
      source_operator_approval_packet_field_count:$approval.recorded_approval_packet_field_count,
      required_operator_approval_count:$approval.required_operator_approval_count,
      fresh_evidence_count:0,
      fresh_trusted_record_count:0,
      operator_approval_recorded:false,
      activation_request_recorded:false,
      long_soak_evidence_recorded:false,
      long_soak_evidence_fresh:false,
      approval_packet_recorded:false,
      approval_packet_persisted:false,
      approval_packet_accepted:false,
      ledger_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      terminal_closure_recorded:false,
      terminal_closure_accepted:false,
      activation_allowed:false,
      live_mutation_execution_ready:false,
      active_binary_mutation_allowed:false,
      install_restart_allowed:false,
      public_release_claim_allowed:false,
      release_artifact_write_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      credential_secret_read_allowed:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      required_supersession_expiry_fixture_count:($fixtures | length),
      blocked_supersession_expiry_fixture_count:($fixtures | map(select(.blocked == true)) | length),
      allowed_supersession_expiry_fixture_count:($fixtures | map(select(.blocked == false)) | length),
      supersession_expiry_denial_fixtures:$fixtures,
      denied_by_operator_approval_fresh_evidence_supersession_expiry:$denied,
      denied_by_operator_approval_fresh_evidence_supersession_expiry_count:($denied | length),
      side_effects:{
        freshness_denial_source_read_performed:true,
        operator_approval_packet_source_read_performed:true,
        watchdog_http_read_performed:true,
        soak_http_read_performed:true,
        activation_performed:false,
        active_runtime_mutated:false,
        active_binary_mutated:false,
        active_service_restarted:false,
        launchd_restarted:false,
        install_executed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        public_release_published:false,
        public_claim_sent:false,
        workspace_write_performed:false,
        evidence_record_persisted:false,
        evidence_record_accepted:false,
        stale_evidence_accepted:false,
        expired_evidence_accepted:false,
        superseded_evidence_accepted:false,
        expired_operator_approval_accepted:false,
        superseded_operator_approval_accepted:false,
        old_operator_approval_reused:false,
        approval_packet_persisted:false,
        approval_packet_accepted:false,
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
  and .gate == "hepta_core_activation_operator_approval_fresh_evidence_supersession_expiry_denial_gate"
  and .operator_approval_fresh_evidence_supersession_expiry_denial_ready == true
  and .supersession_expiry_denial_mode == "schema_only_future_positive_fixture_rejected_no_activation_no_persistence"
  and .supersession_expiry_decision == "blocked_until_operator_approval_and_fresh_evidence_bind_same_current_activation_request_and_are_unexpired_unsuperseded"
  and .required_source_count == 2
  and .ready_source_count == 2
  and .activation_blocking_source_count == 2
  and .minimum_required_long_soak_samples >= 24
  and .observation_soak_samples >= .minimum_required_long_soak_samples
  and .fresh_evidence_max_age_minutes >= 1
  and .operator_approval_max_age_minutes >= 1
  and .same_activation_request_binding_required == true
  and .fresh_evidence_activation_request_binding_required == true
  and .operator_approval_activation_request_binding_required == true
  and .fresh_evidence_expiry_enforced == true
  and .operator_approval_expiry_enforced == true
  and .fresh_evidence_supersession_denied == true
  and .operator_approval_supersession_denied == true
  and .old_operator_approval_reuse_allowed == false
  and .cross_request_fresh_evidence_reuse_allowed == false
  and .expired_fresh_evidence_accepted == false
  and .superseded_fresh_evidence_accepted == false
  and .expired_operator_approval_accepted == false
  and .superseded_operator_approval_accepted == false
  and .current_fresh_evidence_old_approval_pair_accepted == false
  and .superseded_pair_ledger_receipt_terminal_closure_accepted == false
  and .release_long_soak_observed == true
  and .release_long_soak_sample_count == .observation_soak_samples
  and .release_long_soak_ok_count == .observation_soak_samples
  and .release_long_soak_fail_count == 0
  and .source_freshness_denial_ready == true
  and .source_operator_approval_packet_ready == true
  and .source_operator_approval_packet_field_count == 0
  and .required_operator_approval_count == 8
  and .fresh_evidence_count == 0
  and .fresh_trusted_record_count == 0
  and .operator_approval_recorded == false
  and .activation_request_recorded == false
  and .long_soak_evidence_recorded == false
  and .long_soak_evidence_fresh == false
  and .approval_packet_recorded == false
  and .approval_packet_persisted == false
  and .approval_packet_accepted == false
  and .ledger_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .active_binary_mutation_allowed == false
  and .install_restart_allowed == false
  and .public_release_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .required_supersession_expiry_fixture_count == 12
  and .blocked_supersession_expiry_fixture_count == 12
  and .allowed_supersession_expiry_fixture_count == 0
  and (.supersession_expiry_denial_fixtures | length) == 12
  and (.supersession_expiry_denial_fixtures | all(.blocked == true and .activation_allowed == false))
  and .denied_by_operator_approval_fresh_evidence_supersession_expiry_count == 20
  and (.denied_by_operator_approval_fresh_evidence_supersession_expiry | length) == 20
  and (.side_effects | to_entries | all(
    .value == false
    or .key == "freshness_denial_source_read_performed"
    or .key == "operator_approval_packet_source_read_performed"
    or .key == "watchdog_http_read_performed"
    or .key == "soak_http_read_performed"
  ))
' >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta core activation operator approval fresh evidence supersession-expiry denial gate passed"
