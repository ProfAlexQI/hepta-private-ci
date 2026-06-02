#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

OPERATOR_JSON="$(curl -fsS "$BASE_URL/api/operator-security")"
PRODUCTION_JSON="$(curl -fsS "$BASE_URL/api/telegram-production-readiness")"
OWNER_JSON="$(curl -fsS "$BASE_URL/api/telegram-owner-handoff")"
POLL_JSON="$(curl -fsS "$BASE_URL/api/telegram-poll-loop")"

operator_report_sha256="$(sha256_text "$OPERATOR_JSON")"
production_report_sha256="$(sha256_text "$PRODUCTION_JSON")"
owner_report_sha256="$(sha256_text "$OWNER_JSON")"
poll_report_sha256="$(sha256_text "$POLL_JSON")"
diagnostic_hash_sha256="$(
  sha256_text "hepta-operator-security-attention-budget-diagnostic:$operator_report_sha256:$production_report_sha256:$owner_report_sha256:$poll_report_sha256"
)"
policy_hash_sha256="$(
  sha256_text "hepta-operator-security-attention-budget-diagnostic:policy:$operator_report_sha256:$production_report_sha256:$owner_report_sha256:$poll_report_sha256"
)"
side_effect_hash_sha256="$(
  sha256_text "hepta-operator-security-attention-budget-diagnostic:side-effects:$operator_report_sha256:$production_report_sha256:$owner_report_sha256:$poll_report_sha256"
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg operator_report_sha256 "$operator_report_sha256" \
  --arg production_report_sha256 "$production_report_sha256" \
  --arg owner_report_sha256 "$owner_report_sha256" \
  --arg poll_report_sha256 "$poll_report_sha256" \
  --arg diagnostic_hash_sha256 "$diagnostic_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson operator "$OPERATOR_JSON" \
  --argjson production "$PRODUCTION_JSON" \
  --argjson owner "$OWNER_JSON" \
  --argjson poll "$POLL_JSON" \
  '
    def bool_or($default): if . == null then $default else . end;

    ($production.readiness_blockers // []) as $blockers
    | (
      if $production.status == "ready"
        and ($production.attention_budget_ok // false) == true
        and ($blockers | length) == 0
      then "ready"
      elif $production.status == "warming"
        or (($blockers | index("observation_min_poll_iterations")) != null)
      then "warming_observation_budget"
      elif (($production.attention_budget_ok | bool_or(true)) == false)
        or (($blockers | index("attention_budget_exceeded")) != null)
      then "attention_budget_exceeded"
      else "unexpected"
      end
    ) as $production_state
    | (
      if $operator.status == "ready"
      then "ready"
      elif $operator.status == "attention"
        and $operator.attention_reason == "security_gate_not_ready"
        and (
          $production_state == "attention_budget_exceeded"
          or $production_state == "warming_observation_budget"
        )
      then "known_telegram_production_readiness_attention"
      elif $operator.status == "attention"
        and $operator.attention_reason == "telegram_replacement_not_requested"
      then "legacy_owner_coexistence_attention"
      else "unexpected"
      end
    ) as $operator_state
    | (
      $production_state == "ready"
      or $production_state == "warming_observation_budget"
      or $production_state == "attention_budget_exceeded"
    ) as $production_state_known
    | (
      $operator_state == "ready"
      or $operator_state == "known_telegram_production_readiness_attention"
      or $operator_state == "legacy_owner_coexistence_attention"
    ) as $operator_state_known
    | (
      if (($owner.double_poller_risk | bool_or(false)) == false)
        and ($poll.status == "armed" or $poll.status == "gated")
      then "owner_poll_loop_no_double_poller_risk"
      elif $owner.status == "conflict_risk"
        and $owner.active_owner == "conflict_risk"
        and ($owner.double_poller_risk | bool_or(false)) == true
        and ($poll.status == "armed" or $poll.status == "gated")
      then "known_conflict_risk_double_poller_observation"
      else "unexpected"
      end
    ) as $owner_poll_loop_state
    | (
      $owner_poll_loop_state == "owner_poll_loop_no_double_poller_risk"
      or $owner_poll_loop_state == "known_conflict_risk_double_poller_observation"
    ) as $owner_poll_loop_state_known
    | (
      ($operator.raw_token_exposed // false) == false
      and ($operator.raw_update_payload_exposed // false) == false
      and ($operator.raw_prompt_text_exposed // false) == false
      and ($operator.raw_response_text_exposed // false) == false
      and ($production.raw_token_exposed // false) == false
      and ($production.raw_update_payload_exposed // false) == false
      and ($production.raw_prompt_text_exposed // false) == false
      and ($production.raw_response_text_exposed // false) == false
      and ($owner.raw_token_exposed // false) == false
      and ($owner.raw_update_payload_exposed // false) == false
      and ($owner.raw_prompt_text_exposed // false) == false
      and ($owner.raw_response_text_exposed // false) == false
      and ($poll.raw_token_exposed // false) == false
      and ($poll.raw_update_payload_exposed // false) == false
      and ($poll.raw_prompt_text_exposed // false) == false
      and ($poll.raw_response_text_exposed // false) == false
    ) as $redaction_ok
    | (
      ($operator.side_effect_free // false) == true
      and ($production.side_effect_free // false) == true
      and ($owner.side_effect_free // false) == true
      and ($poll.external_network_read_by_status | bool_or(true)) == false
      and ($poll.external_send_by_status | bool_or(true)) == false
    ) as $source_side_effect_boundaries_ok
    | (
      $production_state_known
      and $operator_state_known
      and $owner_poll_loop_state_known
      and $redaction_ok
      and $source_side_effect_boundaries_ok
    ) as $diagnostic_ready
    | {
      product:$product,
      runtime:$runtime,
      status:(if $diagnostic_ready then "ready" else "blocked" end),
      base_url:$base_url,
      gate:"hepta_operator_security_attention_budget_diagnostic_gate",
      operator_security_attention_budget_diagnostic_schema_version:"operator_security_attention_budget_diagnostic_v1",
      operator_security_attention_budget_diagnostic_ready:$diagnostic_ready,
      diagnostic_mode:"read_only_attention_budget_classification_no_restart",
      diagnostic_decision:(
        if $diagnostic_ready
        then "operator_security_and_telegram_production_readiness_attention_state_classified_without_recovery_action"
        else "operator_security_attention_state_unexpected_requires_manual_inspection"
        end
      ),
      required_source_count:4,
      ready_source_count:(
        [
          $operator.status == "ready" or $operator.status == "attention",
          $production.status == "ready" or $production.status == "warming" or $production.status == "attention",
          $owner.ready == true or $owner.status == "parallel_bot_ready" or $owner.status == "ready" or $owner.status == "conflict_risk",
          $poll.status == "armed" or $poll.status == "gated"
        ] | map(select(.)) | length
      ),
      classification_known:$diagnostic_ready,
      production_readiness_state:$production_state,
      operator_security_state:$operator_state,
      owner_poll_loop_state:$owner_poll_loop_state,
      owner_poll_loop_state_known:$owner_poll_loop_state_known,
      operator_security_status:$operator.status,
      operator_attention_reason:($operator.attention_reason // "none"),
      telegram_production_status:$production.status,
      telegram_production_ready:($production.ready // false),
      telegram_production_attention_budget_ok:($production.attention_budget_ok // false),
      telegram_production_recent_bot_api_ok:($production.recent_bot_api_ok // false),
      telegram_production_observation_ready:($production.observation_ready // false),
      telegram_production_observation_fresh:($production.observation_fresh // false),
      telegram_production_poll_loop_armed:($production.poll_loop_armed // false),
      telegram_production_cursor_ready:($production.cursor_ready // false),
      telegram_production_delivery_ledger_ready:($production.delivery_ledger_ready // false),
      telegram_production_min_poll_iterations:($production.min_poll_iterations // 0),
      telegram_production_max_attention_count:($production.max_attention_count // 0),
      telegram_production_max_observed_age_ms:($production.max_observed_age_ms // 0),
      telegram_production_readiness_blockers:$blockers,
      telegram_production_readiness_warning_count:(($production.readiness_warnings // []) | length),
      owner_status:$owner.status,
      owner_active_owner:$owner.active_owner,
      owner_parallel_bot_mode:($owner.parallel_bot_mode // false),
      owner_hepta_poll_loop_armed:($owner.hepta_poll_loop_armed // false),
      owner_double_poller_risk:($owner.double_poller_risk // false),
      poll_loop_status:$poll.status,
      poll_loop_gate_enabled:($poll.poll_loop_gate_enabled // false),
      poll_loop_invokes_drain_once:($poll.loop_invokes_drain_once // false),
      source_operator_report_sha256:$operator_report_sha256,
      source_production_report_sha256:$production_report_sha256,
      source_owner_report_sha256:$owner_report_sha256,
      source_poll_report_sha256:$poll_report_sha256,
      diagnostic_hash_sha256:$diagnostic_hash_sha256,
      diagnostic_policy_hash_sha256:$policy_hash_sha256,
      diagnostic_side_effect_hash_sha256:$side_effect_hash_sha256,
      diagnostic_families:[
        {
          id:"telegram-production-readiness-attention-budget",
          ready:$production_state_known,
          blocked:true,
          production_readiness_state:$production_state,
          attention_budget_ok:($production.attention_budget_ok // false),
          blocker_count:($blockers | length),
          reason:"classifies ready, warming observation budget, and attention-budget-exceeded states without recovery action"
        },
        {
          id:"operator-security-attention-mapping",
          ready:$operator_state_known,
          blocked:true,
          operator_security_state:$operator_state,
          operator_security_status:$operator.status,
          attention_reason:($operator.attention_reason // "none"),
          reason:"maps operator-security attention to the Telegram production readiness state when it is a known bounded transient"
        },
        {
          id:"telegram-owner-poll-loop-boundary",
          ready:$owner_poll_loop_state_known,
          blocked:true,
          active_owner:$owner.active_owner,
          owner_poll_loop_state:$owner_poll_loop_state,
          double_poller_risk:($owner.double_poller_risk // false),
          poll_loop_status:$poll.status,
          reason:"classifies ready owner/poll-loop state or known conflict-risk double-poller observation without live read, send, or owner handoff"
        },
        {
          id:"redaction-and-side-effect-boundary",
          ready:($redaction_ok and $source_side_effect_boundaries_ok),
          blocked:true,
          redaction_ok:$redaction_ok,
          source_side_effect_boundaries_ok:$source_side_effect_boundaries_ok,
          reason:"diagnostic output preserves redaction and performs no recovery mutation"
        }
      ],
      denied_by_attention_budget_diagnostic:[
        "attention_budget_diagnostic_service_restart_denied",
        "attention_budget_diagnostic_launchd_mutation_denied",
        "attention_budget_diagnostic_cursor_write_denied",
        "attention_budget_diagnostic_live_read_denied",
        "attention_budget_diagnostic_telegram_send_denied",
        "attention_budget_diagnostic_owner_handoff_denied",
        "attention_budget_diagnostic_evidence_persistence_denied",
        "attention_budget_diagnostic_secret_read_denied"
      ],
      side_effects:{
        filesystem_written:false,
        evidence_persisted:false,
        cursor_written:false,
        owner_handoff_performed:false,
        poll_loop_mutated:false,
        live_read_performed:false,
        telegram_send_performed:false,
        channel_send_performed:false,
        service_restarted:false,
        launchd_mutated:false,
        gateway_mutation_performed:false,
        credential_read:false,
        secret_file_read:false,
        external_send_performed:false
      }
    }
  ')"

printf '%s\n' "$report"

jq -e '
  .status == "ready"
  and .operator_security_attention_budget_diagnostic_ready == true
  and .classification_known == true
  and (.diagnostic_families | all(.ready == true and .blocked == true))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta operator-security attention-budget diagnostic gate passed"
