#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
SAMPLES="${HEPTA_SOAK_SAMPLES:-${HEPTA_CODEX_SOAK_SAMPLES:-12}}"
INTERVAL_SECONDS="${HEPTA_SOAK_INTERVAL_SECONDS:-${HEPTA_CODEX_SOAK_INTERVAL_SECONDS:-5}}"

case "$SAMPLES" in
  ''|*[!0-9]*) echo "HEPTA_SOAK_SAMPLES must be numeric; legacy HEPTA_CODEX_SOAK_SAMPLES is also accepted" >&2; exit 2 ;;
esac
if (( SAMPLES < 1 )); then
  echo "HEPTA_SOAK_SAMPLES must be at least 1" >&2
  exit 2
fi
case "$INTERVAL_SECONDS" in
  ''|*[!0-9]*) echo "HEPTA_SOAK_INTERVAL_SECONDS must be numeric; legacy HEPTA_CODEX_SOAK_INTERVAL_SECONDS is also accepted" >&2; exit 2 ;;
esac

ok=0
fail=0
last_owner=""
last_post=""
last_production=""
last_production_attention_budget_known=false
last_production_readiness_state_known=false
last_production_readiness_classification="unknown"

for sample in $(seq 1 "$SAMPLES"); do
  state="$(curl -fsS "$BASE_URL/api/watchdog-state" 2>/dev/null || echo '{}')"
  health="$(jq -r '.status // "fail"' <<<"$state" 2>/dev/null || echo fail)"
  route="$(jq -r '.route | .status + ":" + (.missing_route_count|tostring)' <<<"$state" 2>/dev/null || echo fail)"
  owner="$(jq -r '.owner | .active_owner + ":" + (.double_poller_risk|tostring) + ":" + (.hepta_poll_loop_armed|tostring)' <<<"$state" 2>/dev/null || echo fail)"
  poll="$(jq -r '.poll | .status + ":" + (.external_network_read_by_status|tostring) + ":" + (.external_send_by_status|tostring)' <<<"$state" 2>/dev/null || echo fail)"
  post="$(jq -r '.native_post.activation | (.activation_currently_enabled|tostring) + ":" + (.real_mutation_performed|tostring) + ":" + (.external_side_effects|tostring)' <<<"$state" 2>/dev/null || echo fail)"
  stores="$(jq -r '.native_post.stores | (.store_jsonl_valid|tostring) + ":" + (.store_capacity_ok|tostring)' <<<"$state" 2>/dev/null || echo fail)"
  operator="$(jq -r '.operator | .status + ":" + (.security_mode // "") + ":" + (.legacy_owner_coexistence_ready|tostring) + ":" + (.attention_reason // "")' <<<"$state" 2>/dev/null || echo fail)"
  production="$(jq -r '.production | .status + ":" + (.attention_budget_ok|tostring) + ":" + (.recent_bot_api_ok|tostring) + ":" + (.observation_ready|tostring) + ":" + (.observation_fresh|tostring) + ":" + (.poll_loop_armed|tostring) + ":" + (.cursor_ready|tostring) + ":" + (.delivery_ledger_ready|tostring) + ":" + ((.readiness_blockers // []) | join(","))' <<<"$state" 2>/dev/null || echo fail)"
  last_owner="$owner"
  last_post="$post"
  last_production="$production"

  legacy_sample_ready=false
  production_sample_ready=false
  production_attention_sample_ready=false
  production_attention_budget_known=false
  production_readiness_state_known=false
  production_readiness_classification="unknown"
  if [[ "$owner" == "legacy_openclaw:false:false" \
    && ("$poll" == "gated:false:false" || "$poll" == "disabled:false:false") \
    && "$post" == "false:false:false" \
    && "$operator" == "attention:legacy_owner_coexistence_ready:true:telegram_replacement_not_requested" ]]; then
    legacy_sample_ready=true
  fi
  if [[ "$legacy_sample_ready" == true \
    && "$production" == "gated:true:true:false:false:false:true:true:poll_loop_not_armed,production_guards_not_ready,observation_min_poll_iterations,observation_stale" ]]; then
    production_attention_budget_known=true
    production_readiness_state_known=true
    production_readiness_classification="warming_observation_budget"
  fi
  if [[ "$legacy_sample_ready" == true \
    && "$poll" == "disabled:false:false" \
    && "$production" == "disabled:true:true:false:false:false:false:true:telegram_plugin_not_requested,poll_loop_not_armed,cursor_not_ready,production_guards_not_ready,observation_min_poll_iterations,observation_stale" ]]; then
    production_attention_budget_known=true
    production_readiness_state_known=true
    production_readiness_classification="legacy_owner_plugin_disabled"
  fi
  if [[ "$owner" == "parallel_bots:false:true" \
    && "$poll" == "armed:false:false" \
    && "$post" == "true:false:false" \
    && "$operator" == "ready:active_replacement_ready:false:none" ]]; then
    production_sample_ready=true
  fi
  if [[ "$production_sample_ready" == true \
    && "$production" == ready:true:true:true:true:true:true:true:* ]]; then
    production_attention_budget_known=true
    production_readiness_state_known=true
    production_readiness_classification="ready"
  fi
  if [[ "$owner" == "parallel_bots:false:true" \
    && "$poll" == "armed:false:false" \
    && "$post" == "true:false:false" \
    && "$operator" == "attention:attention_required:false:security_gate_not_ready" \
    && "$production" == "attention:false:true:true:true:true:true:true:attention_budget_exceeded" ]]; then
    production_attention_sample_ready=true
    production_attention_budget_known=true
    production_readiness_state_known=true
    production_readiness_classification="attention_budget_exceeded"
  fi
  last_production_attention_budget_known="$production_attention_budget_known"
  last_production_readiness_state_known="$production_readiness_state_known"
  last_production_readiness_classification="$production_readiness_classification"

  if [[ "$health" == "ready" \
    && "$route" == "ready:0" \
    && "$stores" == "true:true" \
    && ("$legacy_sample_ready" == true || "$production_sample_ready" == true || "$production_attention_sample_ready" == true) ]]; then
    ok=$((ok + 1))
  else
    fail=$((fail + 1))
    printf 'sample=%s health=%s route=%s owner=%s poll=%s post=%s stores=%s operator=%s production=%s\n' \
      "$sample" "$health" "$route" "$owner" "$poll" "$post" "$stores" "$operator" "$production" >&2
  fi

  if [[ "$sample" != "$SAMPLES" ]]; then
    sleep "$INTERVAL_SECONDS"
  fi
done

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg owner "$last_owner" \
  --arg post "$last_post" \
  --arg production "$last_production" \
  --arg production_readiness_classification "$last_production_readiness_classification" \
  --argjson production_attention_budget_known "$last_production_attention_budget_known" \
  --argjson production_readiness_state_known "$last_production_readiness_state_known" \
  --argjson samples "$SAMPLES" \
  --argjson ok "$ok" \
  --argjson fail "$fail" \
  '{product:$product,runtime:$runtime,base_url:$base_url,status:(if $fail == 0 then "ready" else "failed" end),samples:$samples,ok:$ok,fail:$fail,active_owner:$owner,legacy_owner_preserved:($owner == "legacy_openclaw:false:false"),telegram_live_send_enabled:($owner == "parallel_bots:false:true"),native_post_real_activation_enabled:($post == "true:false:false"),telegram_production_readiness:$production,telegram_production_attention_budget_known:$production_attention_budget_known,telegram_production_readiness_state_known:$production_readiness_state_known,telegram_production_readiness_classification:$production_readiness_classification}'

if [[ "$fail" != "0" ]]; then
  exit 1
fi

echo "Hepta live soak passed"
