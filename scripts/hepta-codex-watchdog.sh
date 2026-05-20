#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
RELEASE_BIN="${HEPTA_CODEX_RELEASE_BIN:-$REPO_ROOT/codex-rs/target/release/hepta}"
INSTALLED_BIN="${HEPTA_CODEX_INSTALLED_BIN:-$HOME/.local/opt/hepta-codex/bin/hepta-codex}"

release_sha=""
installed_sha=""
if [[ -f "$RELEASE_BIN" ]]; then
  release_sha="$(shasum -a 256 "$RELEASE_BIN" | awk '{print $1}')"
fi
if [[ -f "$INSTALLED_BIN" ]]; then
  installed_sha="$(shasum -a 256 "$INSTALLED_BIN" | awk '{print $1}')"
fi

health_json="$(curl -fsS "$BASE_URL/health")"
route_json="$(curl -fsS "$BASE_URL/api/control-ui-route-parity")"
operator_json="$(curl -fsS "$BASE_URL/api/operator-security")"
owner_json="$(curl -fsS "$BASE_URL/api/telegram-owner-handoff")"
poll_json="$(curl -fsS "$BASE_URL/api/telegram-poll-loop")"
post_json="$(curl -fsS "$BASE_URL/api/native-post-activation-plan")"
stores_json="$(curl -fsS "$BASE_URL/api/native-post-execution-stores")"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta-codex" \
  --arg base_url "$BASE_URL" \
  --arg release_sha "$release_sha" \
  --arg installed_sha "$installed_sha" \
  --argjson health "$health_json" \
  --argjson route "$route_json" \
  --argjson operator "$operator_json" \
  --argjson owner "$owner_json" \
  --argjson poll "$poll_json" \
  --argjson post "$post_json" \
  --argjson stores "$stores_json" \
  '{
    product:$product,
    runtime:$runtime,
    base_url:$base_url,
    status: (
      if $health.status == "ready"
        and $route.status == "ready"
        and $route.missing_route_count == 0
        and $operator.status == "attention"
        and $operator.legacy_owner_coexistence_ready == true
        and $operator.attention_reason == "telegram_replacement_not_requested"
        and $owner.active_owner == "legacy_openclaw"
        and $owner.double_poller_risk == false
        and $owner.hepta_poll_loop_armed == false
        and $poll.status == "gated"
        and $poll.external_network_read_by_status == false
        and $poll.external_send_by_status == false
        and $post.status == "ready"
        and $post.activation_currently_enabled == false
        and $post.real_mutation_performed == false
        and $post.external_side_effects == false
        and $stores.status == "ready"
        and $stores.store_jsonl_valid == true
        and $stores.store_capacity_ok == true
      then "ok" else "failed" end
    ),
    release_sha256:$release_sha,
    installed_sha256:$installed_sha,
    binary_sha_match: ($release_sha != "" and $release_sha == $installed_sha),
    health:$health.status,
    route_count:$route.route_count,
    missing_route_count:$route.missing_route_count,
    operator_security_status:$operator.status,
    security_mode:$operator.security_mode,
    active_owner:$owner.active_owner,
    double_poller_risk:$owner.double_poller_risk,
    telegram_poll_loop_status:$poll.status,
    native_post_activation_enabled:$post.activation_currently_enabled,
    native_post_store_lines:$stores.total_line_count,
    side_effects:{
      telegram_read_by_status:$poll.external_network_read_by_status,
      telegram_send_by_status:$poll.external_send_by_status,
      native_post_real_mutation:$post.real_mutation_performed,
      native_post_external_side_effects:$post.external_side_effects
    }
  }')"

printf '%s\n' "$report"

if [[ "$(printf '%s' "$report" | jq -r '.status')" != "ok" ]]; then
  exit 1
fi

echo "Hepta Codex watchdog passed"
