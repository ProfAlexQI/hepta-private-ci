#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
PROVIDER_JSON="$(curl -fsS "$BASE_URL/api/hepta-provider-metadata-inventory")"
CLI_JSON="$(curl -fsS "$BASE_URL/api/hepta-cli-command-inventory")"
MERGE_JSON="$(curl -fsS "$BASE_URL/api/hepta-merge-completion")"

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .compatibility_mode == "native_provider_metadata_inventory"
  and .side_effect_free == true
  and .old_provider_ops_file_count == 15
  and .adjacent_search_ops_file_count == 3
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count >= 69
  and .missing_route_count == 0
  and .provider_adapter_count == 15
  and .adjacent_search_adapter_count == 3
  and .metadata_inventory_ready == true
  and (.provider_live_invocation_enabled == .credentialed_smoke_performed)
  and .side_effects.provider_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.external_network_read == false
  and .side_effects.external_send_performed == false
  and .side_effects.model_invoked == false
  and .side_effects.telegram_read_performed == false
  and .side_effects.message_sent == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.filesystem_written == false
' <<<"$PROVIDER_JSON" >/dev/null

jq -e '
  .runtime == "hepta"
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count >= 69
  and .missing_route_count == 0
' <<<"$CLI_JSON" >/dev/null

jq -e '
  .runtime == "hepta"
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count >= 69
  and .missing_route_count == 0
  and (.telegram_live_send_enabled == false or .telegram_live_send_enabled == true)
  and (.native_post_real_activation_enabled == false or .native_post_real_activation_enabled == true)
' <<<"$MERGE_JSON" >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --argjson provider "$PROVIDER_JSON" \
  --argjson cli "$CLI_JSON" \
  --argjson merge "$MERGE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    endpoint:"/api/hepta-provider-metadata-inventory",
    old_provider_ops_file_count:$provider.old_provider_ops_file_count,
    adjacent_search_ops_file_count:$provider.adjacent_search_ops_file_count,
    provider_adapter_count:$provider.provider_adapter_count,
    adjacent_search_adapter_count:$provider.adjacent_search_adapter_count,
    current_hepta_codex_script_total:$provider.current_hepta_codex_script_total,
    native_gateway_source_command_count:$provider.native_gateway_source_command_count,
    route_count:$provider.route_count,
    missing_route_count:$provider.missing_route_count,
    provider_live_invocation_enabled:$provider.provider_live_invocation_enabled,
    credentialed_smoke_performed:$provider.credentialed_smoke_performed,
    reports_synchronized: (
      $provider.current_hepta_codex_script_total == $cli.current_hepta_codex_script_total
      and $provider.native_gateway_source_command_count == $cli.native_gateway_source_command_count
      and $provider.current_hepta_codex_script_total == $merge.current_hepta_codex_script_total
      and $provider.native_gateway_source_command_count == $merge.native_gateway_source_command_count
      and $provider.missing_route_count == $cli.missing_route_count
      and $provider.missing_route_count == $merge.missing_route_count
    ),
    side_effects:$provider.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "provider, CLI, and merge-completion reports are out of sync" >&2
  exit 1
fi

echo "Hepta provider metadata inventory passed"
