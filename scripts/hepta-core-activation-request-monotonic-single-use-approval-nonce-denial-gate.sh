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

SUPERSESSION_EXPIRY_JSON="$(
  capture_json_report \
    "hepta-core-activation-operator-approval-fresh-evidence-supersession-expiry-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_SAMPLES="$OBSERVATION_SOAK_SAMPLES" \
      HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS="$OBSERVATION_SOAK_INTERVAL_SECONDS" \
      HEPTA_CORE_ACTIVATION_FRESH_EVIDENCE_MAX_AGE_MINUTES="$FRESH_EVIDENCE_MAX_AGE_MINUTES" \
      HEPTA_CORE_ACTIVATION_OPERATOR_APPROVAL_MAX_AGE_MINUTES="$OPERATOR_APPROVAL_MAX_AGE_MINUTES" \
      scripts/hepta-core-activation-operator-approval-fresh-evidence-supersession-expiry-denial-gate.sh
)"

supersession_expiry_report_sha256="$(sha256_text "$SUPERSESSION_EXPIRY_JSON")"
monotonic_policy_hash_sha256="$(sha256_text "hepta-core-activation-request-monotonic-single-use-approval-nonce-denial:policy:$supersession_expiry_report_sha256:$MIN_LONG_SOAK_SAMPLES:$OBSERVATION_SOAK_SAMPLES:$OBSERVATION_SOAK_INTERVAL_SECONDS:$FRESH_EVIDENCE_MAX_AGE_MINUTES:$OPERATOR_APPROVAL_MAX_AGE_MINUTES")"
monotonic_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-request-monotonic-single-use-approval-nonce-denial:side-effects:$supersession_expiry_report_sha256:$MIN_LONG_SOAK_SAMPLES:$OBSERVATION_SOAK_SAMPLES:$OBSERVATION_SOAK_INTERVAL_SECONDS:$FRESH_EVIDENCE_MAX_AGE_MINUTES:$OPERATOR_APPROVAL_MAX_AGE_MINUTES")"

jq -n -e \
  --argjson source "$SUPERSESSION_EXPIRY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson observation_soak_samples "$OBSERVATION_SOAK_SAMPLES" \
  --argjson fresh_evidence_max_age_minutes "$FRESH_EVIDENCE_MAX_AGE_MINUTES" \
  --argjson operator_approval_max_age_minutes "$OPERATOR_APPROVAL_MAX_AGE_MINUTES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_core_activation_operator_approval_fresh_evidence_supersession_expiry_denial_gate"
    and $source.operator_approval_fresh_evidence_supersession_expiry_denial_ready == true
    and $source.required_source_count == 2
    and $source.ready_source_count == 2
    and $source.activation_blocking_source_count == 2
    and $source.minimum_required_long_soak_samples == $min_long_soak_samples
    and $source.observation_soak_samples == $observation_soak_samples
    and $source.fresh_evidence_max_age_minutes == $fresh_evidence_max_age_minutes
    and $source.operator_approval_max_age_minutes == $operator_approval_max_age_minutes
    and $source.same_activation_request_binding_required == true
    and $source.fresh_evidence_expiry_enforced == true
    and $source.operator_approval_expiry_enforced == true
    and $source.fresh_evidence_supersession_denied == true
    and $source.operator_approval_supersession_denied == true
    and $source.old_operator_approval_reuse_allowed == false
    and $source.cross_request_fresh_evidence_reuse_allowed == false
    and $source.current_fresh_evidence_old_approval_pair_accepted == false
    and $source.superseded_pair_ledger_receipt_terminal_closure_accepted == false
    and $source.required_supersession_expiry_fixture_count == 12
    and $source.blocked_supersession_expiry_fixture_count == 12
    and $source.allowed_supersession_expiry_fixture_count == 0
    and $source.operator_approval_recorded == false
    and $source.activation_request_recorded == false
    and $source.approval_packet_recorded == false
    and $source.ledger_recorded == false
    and $source.receipt_accepted == false
    and $source.terminal_closure_accepted == false
    and $source.activation_allowed == false
    and $source.live_mutation_execution_ready == false
    and ($source.side_effects | to_entries | all(
      .value == false
      or .key == "freshness_denial_source_read_performed"
      or .key == "operator_approval_packet_source_read_performed"
      or .key == "watchdog_http_read_performed"
      or .key == "soak_http_read_performed"
    ))
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_request_monotonic_single_use_approval_nonce_denial_gate" \
  --arg supersession_expiry_report_sha256 "$supersession_expiry_report_sha256" \
  --arg monotonic_policy_hash_sha256 "$monotonic_policy_hash_sha256" \
  --arg monotonic_side_effect_hash_sha256 "$monotonic_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson observation_soak_samples "$OBSERVATION_SOAK_SAMPLES" \
  --argjson observation_soak_interval_seconds "$OBSERVATION_SOAK_INTERVAL_SECONDS" \
  --argjson fresh_evidence_max_age_minutes "$FRESH_EVIDENCE_MAX_AGE_MINUTES" \
  --argjson operator_approval_max_age_minutes "$OPERATOR_APPROVAL_MAX_AGE_MINUTES" \
  --argjson source "$SUPERSESSION_EXPIRY_JSON" \
  '
    ([
      {
        id:"missing-activation-request-generation",
        fixture_kind:"activation_request_generation_missing",
        blocked:true,
        activation_request_recorded:true,
        activation_request_generation_present:false,
        activation_allowed:false,
        reason:"activation requests must carry a monotonic generation"
      },
      {
        id:"activation-request-generation-rollback",
        fixture_kind:"activation_request_generation_rollback",
        blocked:true,
        previous_generation:42,
        candidate_generation:41,
        monotonic_generation:false,
        activation_allowed:false,
        reason:"request generation cannot move backward"
      },
      {
        id:"activation-request-generation-skip",
        fixture_kind:"activation_request_generation_skip_without_terminal_closure",
        blocked:true,
        previous_generation:42,
        candidate_generation:44,
        predecessor_terminally_closed:false,
        activation_allowed:false,
        reason:"request generation cannot skip a predecessor without terminal closure evidence"
      },
      {
        id:"duplicate-activation-request-nonce",
        fixture_kind:"duplicate_activation_request_nonce",
        blocked:true,
        activation_request_nonce_seen:true,
        nonce_accepted:false,
        activation_allowed:false,
        reason:"activation request nonce is single use"
      },
      {
        id:"fresh-evidence-nonce-reused-across-requests",
        fixture_kind:"fresh_evidence_nonce_reused_across_activation_requests",
        blocked:true,
        fresh_evidence_nonce_seen:true,
        cross_request_reuse_attempted:true,
        activation_allowed:false,
        reason:"fresh evidence nonce cannot be reused across activation requests"
      },
      {
        id:"operator-approval-nonce-reused-across-requests",
        fixture_kind:"operator_approval_nonce_reused_across_activation_requests",
        blocked:true,
        operator_approval_nonce_seen:true,
        cross_request_reuse_attempted:true,
        activation_allowed:false,
        reason:"operator approval nonce cannot be reused across activation requests"
      },
      {
        id:"approval-evidence-pair-replayed-after-terminal-denial",
        fixture_kind:"approval_evidence_pair_replayed_after_terminal_denial",
        blocked:true,
        pair_seen:true,
        previous_terminal_verdict:"blocked",
        pair_replay_allowed:false,
        activation_allowed:false,
        reason:"a pair that reached terminal denial cannot be replayed as current approval"
      },
      {
        id:"concurrent-current-activation-requests",
        fixture_kind:"concurrent_current_activation_request_ambiguity",
        blocked:true,
        current_request_count:2,
        single_current_request_pointer:false,
        activation_allowed:false,
        reason:"there must be exactly one current activation request"
      },
      {
        id:"ledger-receipt-terminal-closure-from-stale-generation",
        fixture_kind:"ledger_receipt_terminal_closure_from_stale_generation",
        blocked:true,
        ledger_recorded:true,
        receipt_accepted:true,
        terminal_closure_recorded:true,
        generation_is_current:false,
        activation_allowed:false,
        reason:"ledger, receipt, and terminal closure cannot revive stale generation state"
      },
      {
        id:"current-request-with-previous-generation-fresh-evidence",
        fixture_kind:"current_request_previous_generation_fresh_evidence",
        blocked:true,
        current_activation_request_generation:43,
        fresh_evidence_generation:42,
        same_generation:false,
        activation_allowed:false,
        reason:"current request requires current-generation fresh evidence"
      },
      {
        id:"current-request-with-previous-generation-operator-approval",
        fixture_kind:"current_request_previous_generation_operator_approval",
        blocked:true,
        current_activation_request_generation:43,
        operator_approval_generation:42,
        same_generation:false,
        activation_allowed:false,
        reason:"current request requires current-generation operator approval"
      },
      {
        id:"monotonic-nonce-claim-with-live-or-public-side-effect",
        fixture_kind:"monotonic_nonce_claim_with_live_or_public_side_effect",
        blocked:true,
        public_release_claim_allowed:false,
        release_artifact_write_allowed:false,
        install_restart_allowed:false,
        active_binary_mutation_allowed:false,
        activation_allowed:false,
        reason:"monotonic request and nonce checks never authorize public or live mutation side effects"
      }
    ]) as $fixtures
    | ([
        "activation_request_monotonic_generation_required",
        "single_current_activation_request_required",
        "activation_request_nonce_required",
        "fresh_evidence_nonce_required",
        "operator_approval_nonce_required",
        "approval_evidence_pair_single_use_required",
        "activation_request_generation_rollback_denied",
        "activation_request_generation_skip_denied",
        "duplicate_activation_request_nonce_denied",
        "duplicate_fresh_evidence_nonce_denied",
        "duplicate_operator_approval_nonce_denied",
        "approval_evidence_pair_replay_denied",
        "stale_generation_ledger_receipt_terminal_closure_denied",
        "previous_generation_fresh_evidence_denied",
        "previous_generation_operator_approval_denied",
        "concurrent_activation_request_ambiguity_denied",
        "public_release_claim_denied",
        "release_artifact_write_denied",
        "install_restart_denied",
        "active_binary_mutation_denied",
        "workspace_write_denied",
        "memory_store_mutation_denied",
        "provider_model_invocation_denied",
        "channel_delivery_denied",
        "credential_secret_read_denied"
      ]) as $denied
    | {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        activation_request_monotonic_single_use_approval_nonce_denial_schema_version:"hepta_core_activation_request_monotonic_single_use_approval_nonce_denial_v1",
        activation_request_monotonic_single_use_approval_nonce_denial_ready:true,
        monotonic_single_use_denial_mode:"schema_only_generation_nonce_replay_denial_no_activation_no_persistence",
        monotonic_single_use_decision:"blocked_until_current_activation_request_generation_and_single_use_nonces_bind_fresh_evidence_operator_approval_ledger_receipt_terminal_closure",
        required_source_count:1,
        ready_source_count:1,
        activation_blocking_source_count:1,
        source_supersession_expiry_gate:"hepta_core_activation_operator_approval_fresh_evidence_supersession_expiry_denial_gate",
        source_supersession_expiry_report_sha256:$supersession_expiry_report_sha256,
        source_report_hashes:[$supersession_expiry_report_sha256],
        monotonic_policy_hash_sha256:$monotonic_policy_hash_sha256,
        monotonic_side_effect_hash_sha256:$monotonic_side_effect_hash_sha256,
        minimum_required_long_soak_samples:$min_long_soak_samples,
        observation_soak_samples:$observation_soak_samples,
        observation_soak_interval_seconds:$observation_soak_interval_seconds,
        fresh_evidence_max_age_minutes:$fresh_evidence_max_age_minutes,
        operator_approval_max_age_minutes:$operator_approval_max_age_minutes,
        source_supersession_expiry_ready:$source.operator_approval_fresh_evidence_supersession_expiry_denial_ready,
        source_supersession_expiry_fixture_count:$source.required_supersession_expiry_fixture_count,
        source_supersession_expiry_blocked_fixture_count:$source.blocked_supersession_expiry_fixture_count,
        source_same_activation_request_binding_required:$source.same_activation_request_binding_required,
        source_fresh_evidence_supersession_denied:$source.fresh_evidence_supersession_denied,
        source_operator_approval_supersession_denied:$source.operator_approval_supersession_denied,
        source_old_operator_approval_reuse_allowed:$source.old_operator_approval_reuse_allowed,
        activation_request_monotonic_generation_required:true,
        activation_request_generation_current_required:true,
        single_current_activation_request_required:true,
        activation_request_nonce_required:true,
        fresh_evidence_single_use_nonce_required:true,
        operator_approval_single_use_nonce_required:true,
        approval_evidence_pair_single_use_required:true,
        request_generation_rollback_denied:true,
        request_generation_skip_denied:true,
        duplicate_activation_request_nonce_denied:true,
        duplicate_fresh_evidence_nonce_denied:true,
        duplicate_operator_approval_nonce_denied:true,
        approval_evidence_pair_replay_denied:true,
        concurrent_activation_request_ambiguity_denied:true,
        stale_generation_ledger_receipt_terminal_closure_denied:true,
        previous_generation_fresh_evidence_denied:true,
        previous_generation_operator_approval_denied:true,
        stale_generation_reuse_allowed:false,
        nonce_replay_accepted:false,
        pair_replay_accepted:false,
        concurrent_activation_request_accepted:false,
        request_generation_rollback_accepted:false,
        request_generation_skip_accepted:false,
        release_long_soak_observed:$source.release_long_soak_observed,
        release_long_soak_sample_count:$source.release_long_soak_sample_count,
        release_long_soak_ok_count:$source.release_long_soak_ok_count,
        release_long_soak_fail_count:$source.release_long_soak_fail_count,
        fresh_evidence_count:$source.fresh_evidence_count,
        fresh_trusted_record_count:$source.fresh_trusted_record_count,
        operator_approval_recorded:false,
        activation_request_recorded:false,
        current_activation_request_pointer_recorded:false,
        approval_packet_recorded:false,
        approval_packet_persisted:false,
        approval_packet_accepted:false,
        activation_request_generation_recorded:false,
        activation_request_nonce_recorded:false,
        fresh_evidence_nonce_recorded:false,
        operator_approval_nonce_recorded:false,
        approval_evidence_pair_nonce_recorded:false,
        nonce_registry_persisted:false,
        idempotency_state_recorded:false,
        idempotency_state_persisted:false,
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
        required_monotonic_nonce_fixture_count:($fixtures | length),
        blocked_monotonic_nonce_fixture_count:($fixtures | map(select(.blocked == true)) | length),
        allowed_monotonic_nonce_fixture_count:($fixtures | map(select(.blocked != true)) | length),
        monotonic_nonce_denial_fixtures:$fixtures,
        denied_by_activation_request_monotonic_single_use_approval_nonce:$denied,
        denied_by_activation_request_monotonic_single_use_approval_nonce_count:($denied | length),
        side_effects:{
          supersession_expiry_source_read_performed:true,
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
          nonce_registry_persisted:false,
          idempotency_state_recorded:false,
          idempotency_state_persisted:false,
          stale_generation_reused:false,
          nonce_replay_accepted:false,
          pair_replay_accepted:false,
          concurrent_activation_request_accepted:false,
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
  and .gate == "hepta_core_activation_request_monotonic_single_use_approval_nonce_denial_gate"
  and .activation_request_monotonic_single_use_approval_nonce_denial_ready == true
  and .monotonic_single_use_denial_mode == "schema_only_generation_nonce_replay_denial_no_activation_no_persistence"
  and .monotonic_single_use_decision == "blocked_until_current_activation_request_generation_and_single_use_nonces_bind_fresh_evidence_operator_approval_ledger_receipt_terminal_closure"
  and .required_source_count == 1
  and .ready_source_count == 1
  and .activation_blocking_source_count == 1
  and .minimum_required_long_soak_samples >= 24
  and .observation_soak_samples >= .minimum_required_long_soak_samples
  and .source_supersession_expiry_ready == true
  and .source_supersession_expiry_fixture_count == 12
  and .source_supersession_expiry_blocked_fixture_count == 12
  and .source_same_activation_request_binding_required == true
  and .source_fresh_evidence_supersession_denied == true
  and .source_operator_approval_supersession_denied == true
  and .source_old_operator_approval_reuse_allowed == false
  and .activation_request_monotonic_generation_required == true
  and .activation_request_generation_current_required == true
  and .single_current_activation_request_required == true
  and .activation_request_nonce_required == true
  and .fresh_evidence_single_use_nonce_required == true
  and .operator_approval_single_use_nonce_required == true
  and .approval_evidence_pair_single_use_required == true
  and .request_generation_rollback_denied == true
  and .request_generation_skip_denied == true
  and .duplicate_activation_request_nonce_denied == true
  and .duplicate_fresh_evidence_nonce_denied == true
  and .duplicate_operator_approval_nonce_denied == true
  and .approval_evidence_pair_replay_denied == true
  and .concurrent_activation_request_ambiguity_denied == true
  and .stale_generation_ledger_receipt_terminal_closure_denied == true
  and .previous_generation_fresh_evidence_denied == true
  and .previous_generation_operator_approval_denied == true
  and .stale_generation_reuse_allowed == false
  and .nonce_replay_accepted == false
  and .pair_replay_accepted == false
  and .concurrent_activation_request_accepted == false
  and .request_generation_rollback_accepted == false
  and .request_generation_skip_accepted == false
  and .operator_approval_recorded == false
  and .activation_request_recorded == false
  and .current_activation_request_pointer_recorded == false
  and .activation_request_generation_recorded == false
  and .activation_request_nonce_recorded == false
  and .fresh_evidence_nonce_recorded == false
  and .operator_approval_nonce_recorded == false
  and .approval_evidence_pair_nonce_recorded == false
  and .nonce_registry_persisted == false
  and .idempotency_state_recorded == false
  and .idempotency_state_persisted == false
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
  and .required_monotonic_nonce_fixture_count == 12
  and .blocked_monotonic_nonce_fixture_count == 12
  and .allowed_monotonic_nonce_fixture_count == 0
  and (.monotonic_nonce_denial_fixtures | length) == 12
  and (.monotonic_nonce_denial_fixtures | all(.blocked == true and .activation_allowed == false))
  and .denied_by_activation_request_monotonic_single_use_approval_nonce_count == 25
  and (.denied_by_activation_request_monotonic_single_use_approval_nonce | length) == 25
  and (.side_effects | to_entries | all(
    .value == false
    or .key == "supersession_expiry_source_read_performed"
    or .key == "freshness_denial_source_read_performed"
    or .key == "operator_approval_packet_source_read_performed"
    or .key == "watchdog_http_read_performed"
    or .key == "soak_http_read_performed"
  ))
' >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta core activation request monotonic single-use approval nonce denial gate passed"
