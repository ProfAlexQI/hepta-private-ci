#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
RELEASE_JSON="$(curl -fsS "$BASE_URL/api/hepta-release-hardening-status-gate")"
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
  and .compatibility_mode == "native_release_hardening_status_gate_inventory"
  and .side_effect_free == true
  and .old_release_hardening_script_family_count == 12
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count == 64
  and .missing_route_count == 0
  and .status_gate_count == 12
  and .local_status_gate_ready_count == 12
  and .live_execution_enabled_count == 0
  and .external_production_gate_count == 3
  and .launchd_mutation_required_count == 3
  and .filesystem_artifact_write_required_count == 2
  and .operator_approval_required_count == 12
  and .release_hardening_status_gate_ready == true
  and .old_script_execution_compatibility_claimed == true
  and (.external_production_gate_enabled == false or .external_production_gate_enabled == true)
  and (.release_artifact_pack_enabled == false or .release_artifact_pack_enabled == true)
  and .launchd_service_mutation_enabled == false
  and .recurring_watchdog_install_enabled == false
  and .local_import_execution_enabled == false
  and .autonomous_subagent_spawn_enabled == false
  and .side_effects.process_spawned == false
  and .side_effects.filesystem_read == false
  and .side_effects.filesystem_written == false
  and .side_effects.release_artifact_written == false
  and .side_effects.launchd_mutated == false
  and .side_effects.watchdog_service_installed == false
  and .side_effects.external_network_read == false
  and .side_effects.external_send_performed == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.telegram_owner_handoff_performed == false
  and .side_effects.telegram_read_performed == false
  and .side_effects.telegram_send_performed == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.channel_read_performed == false
  and .side_effects.channel_send_performed == false
  and .side_effects.coding_agent_spawned == false
  and .side_effects.gateway_mutation_performed == false
' <<<"$RELEASE_JSON" >/dev/null

for payload in "$MEMORY_JSON" "$LOCAL_JSON" "$CHANNEL_JSON" "$RUNTIME_JSON" "$CLI_JSON" "$PROVIDER_JSON" "$MERGE_JSON"; do
  jq -e '
    .runtime == "hepta-codex"
    and .current_hepta_codex_script_total == 17
    and .native_gateway_source_command_count == 64
    and .missing_route_count == 0
  ' <<<"$payload" >/dev/null
done

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta-codex" \
  --arg base_url "$BASE_URL" \
  --argjson release "$RELEASE_JSON" \
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
    endpoint:"/api/hepta-release-hardening-status-gate",
    old_release_hardening_script_family_count:$release.old_release_hardening_script_family_count,
    status_gate_count:$release.status_gate_count,
    local_status_gate_ready_count:$release.local_status_gate_ready_count,
    live_execution_enabled_count:$release.live_execution_enabled_count,
    external_production_gate_count:$release.external_production_gate_count,
    launchd_mutation_required_count:$release.launchd_mutation_required_count,
    filesystem_artifact_write_required_count:$release.filesystem_artifact_write_required_count,
    operator_approval_required_count:$release.operator_approval_required_count,
    current_hepta_codex_script_total:$release.current_hepta_codex_script_total,
    native_gateway_source_command_count:$release.native_gateway_source_command_count,
    route_count:$release.route_count,
    missing_route_count:$release.missing_route_count,
    release_hardening_status_gate_ready:$release.release_hardening_status_gate_ready,
    external_production_gate_enabled:$release.external_production_gate_enabled,
    release_artifact_pack_enabled:$release.release_artifact_pack_enabled,
    launchd_service_mutation_enabled:$release.launchd_service_mutation_enabled,
    recurring_watchdog_install_enabled:$release.recurring_watchdog_install_enabled,
    local_import_execution_enabled:$release.local_import_execution_enabled,
    autonomous_subagent_spawn_enabled:$release.autonomous_subagent_spawn_enabled,
    reports_synchronized: (
      $release.current_hepta_codex_script_total == $memory.current_hepta_codex_script_total
      and $release.native_gateway_source_command_count == $memory.native_gateway_source_command_count
      and $release.current_hepta_codex_script_total == $local.current_hepta_codex_script_total
      and $release.native_gateway_source_command_count == $local.native_gateway_source_command_count
      and $release.current_hepta_codex_script_total == $channel.current_hepta_codex_script_total
      and $release.native_gateway_source_command_count == $channel.native_gateway_source_command_count
      and $release.current_hepta_codex_script_total == $runtime_inventory.current_hepta_codex_script_total
      and $release.native_gateway_source_command_count == $runtime_inventory.native_gateway_source_command_count
      and $release.current_hepta_codex_script_total == $cli.current_hepta_codex_script_total
      and $release.native_gateway_source_command_count == $cli.native_gateway_source_command_count
      and $release.current_hepta_codex_script_total == $provider.current_hepta_codex_script_total
      and $release.native_gateway_source_command_count == $provider.native_gateway_source_command_count
      and $release.current_hepta_codex_script_total == $merge.current_hepta_codex_script_total
      and $release.native_gateway_source_command_count == $merge.native_gateway_source_command_count
      and $release.missing_route_count == $memory.missing_route_count
      and $release.missing_route_count == $local.missing_route_count
      and $release.missing_route_count == $channel.missing_route_count
      and $release.missing_route_count == $runtime_inventory.missing_route_count
      and $release.missing_route_count == $cli.missing_route_count
      and $release.missing_route_count == $provider.missing_route_count
      and $release.missing_route_count == $merge.missing_route_count
    ),
    side_effects:$release.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "release/hardening, memory, local tooling/content, channel, runtime/session, provider, CLI, and merge-completion reports are out of sync" >&2
  exit 1
fi

echo "Hepta Codex release/hardening status gate passed"
