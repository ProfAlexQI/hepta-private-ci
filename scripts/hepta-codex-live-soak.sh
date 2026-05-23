#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
SAMPLES="${HEPTA_CODEX_SOAK_SAMPLES:-12}"
INTERVAL_SECONDS="${HEPTA_CODEX_SOAK_INTERVAL_SECONDS:-5}"

case "$SAMPLES" in
  ''|*[!0-9]*) echo "HEPTA_CODEX_SOAK_SAMPLES must be numeric" >&2; exit 2 ;;
esac
case "$INTERVAL_SECONDS" in
  ''|*[!0-9]*) echo "HEPTA_CODEX_SOAK_INTERVAL_SECONDS must be numeric" >&2; exit 2 ;;
esac

ok=0
fail=0
last_owner=""
last_post=""

for sample in $(seq 1 "$SAMPLES"); do
  health="$(curl -fsS "$BASE_URL/health" | jq -r '.status' 2>/dev/null || echo fail)"
  route="$(curl -fsS "$BASE_URL/api/control-ui-route-parity" | jq -r '.status + ":" + (.missing_route_count|tostring)' 2>/dev/null || echo fail)"
  owner="$(curl -fsS "$BASE_URL/api/telegram-owner-handoff" | jq -r '.active_owner + ":" + (.double_poller_risk|tostring) + ":" + (.hepta_poll_loop_armed|tostring)' 2>/dev/null || echo fail)"
  poll="$(curl -fsS "$BASE_URL/api/telegram-poll-loop" | jq -r '.status + ":" + (.external_network_read_by_status|tostring) + ":" + (.external_send_by_status|tostring)' 2>/dev/null || echo fail)"
  post="$(curl -fsS "$BASE_URL/api/native-post-activation-plan" | jq -r '(.activation_currently_enabled|tostring) + ":" + (.real_mutation_performed|tostring) + ":" + (.external_side_effects|tostring)' 2>/dev/null || echo fail)"
  stores="$(curl -fsS "$BASE_URL/api/native-post-execution-stores" | jq -r '(.store_jsonl_valid|tostring) + ":" + (.store_capacity_ok|tostring)' 2>/dev/null || echo fail)"
  operator="$(curl -fsS "$BASE_URL/api/operator-security" | jq -r '.status + ":" + (.security_mode // "") + ":" + (.legacy_owner_coexistence_ready|tostring) + ":" + (.attention_reason // "")' 2>/dev/null || echo fail)"
  last_owner="$owner"
  last_post="$post"

  legacy_sample_ready=false
  production_sample_ready=false
  if [[ "$owner" == "legacy_openclaw:false:false" \
    && "$poll" == "gated:false:false" \
    && "$post" == "false:false:false" \
    && "$operator" == "attention:legacy_owner_coexistence_ready:true:telegram_replacement_not_requested" ]]; then
    legacy_sample_ready=true
  fi
  if [[ "$owner" == "parallel_bots:false:true" \
    && "$poll" == "armed:false:false" \
    && "$post" == "true:false:false" \
    && "$operator" == "ready:active_replacement_ready:false:none" ]]; then
    production_sample_ready=true
  fi

  if [[ "$health" == "ready" \
    && "$route" == "ready:0" \
    && "$stores" == "true:true" \
    && ("$legacy_sample_ready" == true || "$production_sample_ready" == true) ]]; then
    ok=$((ok + 1))
  else
    fail=$((fail + 1))
    printf 'sample=%s health=%s route=%s owner=%s poll=%s post=%s stores=%s operator=%s\n' \
      "$sample" "$health" "$route" "$owner" "$poll" "$post" "$stores" "$operator" >&2
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
  --argjson samples "$SAMPLES" \
  --argjson ok "$ok" \
  --argjson fail "$fail" \
  '{product:$product,runtime:$runtime,base_url:$base_url,status:(if $fail == 0 then "ready" else "failed" end),samples:$samples,ok:$ok,fail:$fail,active_owner:$owner,legacy_owner_preserved:($owner == "legacy_openclaw:false:false"),telegram_live_send_enabled:($owner == "parallel_bots:false:true"),native_post_real_activation_enabled:($post == "true:false:false")}'

if [[ "$fail" != "0" ]]; then
  exit 1
fi

echo "Hepta Codex live soak passed"
