#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
MEMORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-memory-capability-absorption-inventory")"
LOCAL_JSON="$(curl -fsS "$BASE_URL/api/hepta-local-tooling-content-inventory")"
CHANNEL_JSON="$(curl -fsS "$BASE_URL/api/hepta-channel-adapter-status-inventory")"
RUNTIME_JSON="$(curl -fsS "$BASE_URL/api/hepta-runtime-session-dry-run-inventory")"
CLI_JSON="$(curl -fsS "$BASE_URL/api/hepta-cli-command-inventory")"
PROVIDER_JSON="$(curl -fsS "$BASE_URL/api/hepta-provider-metadata-inventory")"
MERGE_JSON="$(curl -fsS "$BASE_URL/api/hepta-merge-completion")"

jq -e '
  .runtime == "hepta-codex"
  and .status == "attention"
  and .compatibility_mode == "native_memory_capability_absorption_gap_inventory"
  and .side_effect_free == true
  and .old_memory_capability_ops_file_count == 14
  and .current_hepta_codex_script_total == 16
  and .native_gateway_source_command_count == 63
  and .missing_route_count == 0
  and .surface_count == 14
  and .absorbed_or_represented_count == 9
  and .gap_report_ready_count == 14
  and .live_mutation_enabled_count == 0
  and .memory_capability_inventory_ready == true
  and .old_cli_invocation_compatibility_claimed == false
  and .memory_store_mutation_enabled == false
  and .capability_registry_mutation_enabled == false
  and .plugin_registry_mutation_enabled == false
  and .coding_agent_spawn_enabled == false
  and .search_provider_live_query_enabled == false
  and .skill_workshop_write_enabled == false
  and .side_effects.memory_store_mutated == false
  and .side_effects.capability_registry_mutated == false
  and .side_effects.plugin_registry_mutated == false
  and .side_effects.coding_agent_spawned == false
  and .side_effects.skill_workshop_written == false
  and .side_effects.filesystem_read == false
  and .side_effects.filesystem_written == false
  and .side_effects.external_network_read == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.channel_read_performed == false
  and .side_effects.channel_send_performed == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.gateway_mutation_performed == false
  and .side_effects.external_send_performed == false
' <<<"$MEMORY_JSON" >/dev/null

for payload in "$LOCAL_JSON" "$CHANNEL_JSON" "$RUNTIME_JSON" "$CLI_JSON" "$PROVIDER_JSON" "$MERGE_JSON"; do
  jq -e '
    .runtime == "hepta-codex"
    and .current_hepta_codex_script_total == 16
    and .native_gateway_source_command_count == 63
    and .missing_route_count == 0
  ' <<<"$payload" >/dev/null
done

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta-codex" \
  --arg base_url "$BASE_URL" \
  --argjson memory "$MEMORY_JSON" \
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
    endpoint:"/api/hepta-memory-capability-absorption-inventory",
    old_memory_capability_ops_file_count:$memory.old_memory_capability_ops_file_count,
    surface_count:$memory.surface_count,
    absorbed_or_represented_count:$memory.absorbed_or_represented_count,
    gap_report_ready_count:$memory.gap_report_ready_count,
    live_mutation_enabled_count:$memory.live_mutation_enabled_count,
    current_hepta_codex_script_total:$memory.current_hepta_codex_script_total,
    native_gateway_source_command_count:$memory.native_gateway_source_command_count,
    route_count:$memory.route_count,
    missing_route_count:$memory.missing_route_count,
    memory_store_mutation_enabled:$memory.memory_store_mutation_enabled,
    capability_registry_mutation_enabled:$memory.capability_registry_mutation_enabled,
    plugin_registry_mutation_enabled:$memory.plugin_registry_mutation_enabled,
    coding_agent_spawn_enabled:$memory.coding_agent_spawn_enabled,
    search_provider_live_query_enabled:$memory.search_provider_live_query_enabled,
    skill_workshop_write_enabled:$memory.skill_workshop_write_enabled,
    reports_synchronized: (
      $memory.current_hepta_codex_script_total == $local.current_hepta_codex_script_total
      and $memory.native_gateway_source_command_count == $local.native_gateway_source_command_count
      and $memory.current_hepta_codex_script_total == $channel.current_hepta_codex_script_total
      and $memory.native_gateway_source_command_count == $channel.native_gateway_source_command_count
      and $memory.current_hepta_codex_script_total == $runtime_inventory.current_hepta_codex_script_total
      and $memory.native_gateway_source_command_count == $runtime_inventory.native_gateway_source_command_count
      and $memory.current_hepta_codex_script_total == $cli.current_hepta_codex_script_total
      and $memory.native_gateway_source_command_count == $cli.native_gateway_source_command_count
      and $memory.current_hepta_codex_script_total == $provider.current_hepta_codex_script_total
      and $memory.native_gateway_source_command_count == $provider.native_gateway_source_command_count
      and $memory.current_hepta_codex_script_total == $merge.current_hepta_codex_script_total
      and $memory.native_gateway_source_command_count == $merge.native_gateway_source_command_count
      and $memory.missing_route_count == $local.missing_route_count
      and $memory.missing_route_count == $channel.missing_route_count
      and $memory.missing_route_count == $runtime_inventory.missing_route_count
      and $memory.missing_route_count == $cli.missing_route_count
      and $memory.missing_route_count == $provider.missing_route_count
      and $memory.missing_route_count == $merge.missing_route_count
    ),
    side_effects:$memory.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "memory/capability, local tooling/content, channel, runtime/session, provider, CLI, and merge-completion reports are out of sync" >&2
  exit 1
fi

echo "Hepta Codex memory/capability inventory passed"
