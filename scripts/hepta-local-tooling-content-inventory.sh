#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
LOCAL_JSON="$(curl -fsS "$BASE_URL/api/hepta-local-tooling-content-inventory")"
CHANNEL_JSON="$(curl -fsS "$BASE_URL/api/hepta-channel-adapter-status-inventory")"
RUNTIME_JSON="$(curl -fsS "$BASE_URL/api/hepta-runtime-session-dry-run-inventory")"
CLI_JSON="$(curl -fsS "$BASE_URL/api/hepta-cli-command-inventory")"
PROVIDER_JSON="$(curl -fsS "$BASE_URL/api/hepta-provider-metadata-inventory")"
MERGE_JSON="$(curl -fsS "$BASE_URL/api/hepta-merge-completion")"

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .compatibility_mode == "native_local_tooling_content_planning_inventory"
  and .side_effect_free == true
  and .old_local_tooling_ops_file_count == 11
  and .current_hepta_codex_script_total >= 17
  and .native_gateway_source_command_count >= 69
  and .missing_route_count == 0
  and .surface_count == 11
  and .planner_ready_count == 11
  and .live_process_enabled_count == 0
  and .filesystem_touch_enabled_count == 0
  and .network_read_enabled_count == 0
  and .tool_invocation_enabled_count == 0
  and .local_tooling_inventory_ready == true
  and .old_cli_invocation_compatibility_claimed == false
  and .process_execution_enabled == false
  and .filesystem_read_enabled == false
  and .filesystem_write_enabled == false
  and .network_read_enabled == false
  and .tool_invocation_enabled == false
  and .side_effects.process_spawned == false
  and .side_effects.filesystem_read == false
  and .side_effects.filesystem_written == false
  and .side_effects.external_network_read == false
  and .side_effects.tool_invoked == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.channel_read_performed == false
  and .side_effects.channel_send_performed == false
  and .side_effects.gateway_mutation_performed == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.external_send_performed == false
' <<<"$LOCAL_JSON" >/dev/null

for payload in "$CHANNEL_JSON" "$RUNTIME_JSON" "$CLI_JSON" "$PROVIDER_JSON" "$MERGE_JSON"; do
  jq -e '
    .runtime == "hepta"
    and .current_hepta_codex_script_total >= 17
    and .native_gateway_source_command_count >= 69
    and .missing_route_count == 0
  ' <<<"$payload" >/dev/null
done

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --argjson local "$LOCAL_JSON" \
  --argjson channel "$CHANNEL_JSON" \
  --argjson runtime_inventory "$RUNTIME_JSON" \
  --argjson cli "$CLI_JSON" \
  --argjson provider "$PROVIDER_JSON" \
  --argjson merge "$MERGE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    endpoint:"/api/hepta-local-tooling-content-inventory",
    old_local_tooling_ops_file_count:$local.old_local_tooling_ops_file_count,
    surface_count:$local.surface_count,
    planner_ready_count:$local.planner_ready_count,
    live_process_enabled_count:$local.live_process_enabled_count,
    filesystem_touch_enabled_count:$local.filesystem_touch_enabled_count,
    network_read_enabled_count:$local.network_read_enabled_count,
    tool_invocation_enabled_count:$local.tool_invocation_enabled_count,
    current_hepta_codex_script_total:$local.current_hepta_codex_script_total,
    native_gateway_source_command_count:$local.native_gateway_source_command_count,
    route_count:$local.route_count,
    missing_route_count:$local.missing_route_count,
    process_execution_enabled:$local.process_execution_enabled,
    filesystem_read_enabled:$local.filesystem_read_enabled,
    filesystem_write_enabled:$local.filesystem_write_enabled,
    network_read_enabled:$local.network_read_enabled,
    tool_invocation_enabled:$local.tool_invocation_enabled,
    reports_synchronized: (
      $local.current_hepta_codex_script_total == $channel.current_hepta_codex_script_total
      and $local.native_gateway_source_command_count == $channel.native_gateway_source_command_count
      and $local.current_hepta_codex_script_total == $runtime_inventory.current_hepta_codex_script_total
      and $local.native_gateway_source_command_count == $runtime_inventory.native_gateway_source_command_count
      and $local.current_hepta_codex_script_total == $cli.current_hepta_codex_script_total
      and $local.native_gateway_source_command_count == $cli.native_gateway_source_command_count
      and $local.current_hepta_codex_script_total == $provider.current_hepta_codex_script_total
      and $local.native_gateway_source_command_count == $provider.native_gateway_source_command_count
      and $local.current_hepta_codex_script_total == $merge.current_hepta_codex_script_total
      and $local.native_gateway_source_command_count == $merge.native_gateway_source_command_count
      and $local.missing_route_count == $channel.missing_route_count
      and $local.missing_route_count == $runtime_inventory.missing_route_count
      and $local.missing_route_count == $cli.missing_route_count
      and $local.missing_route_count == $provider.missing_route_count
      and $local.missing_route_count == $merge.missing_route_count
    ),
    side_effects:$local.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "local tooling/content, channel, runtime/session, provider, CLI, and merge-completion reports are out of sync" >&2
  exit 1
fi

echo "Hepta local tooling/content inventory passed"
