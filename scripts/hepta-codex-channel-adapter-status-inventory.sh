#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
CHANNEL_JSON="$(curl -fsS "$BASE_URL/api/hepta-channel-adapter-status-inventory")"
RUNTIME_JSON="$(curl -fsS "$BASE_URL/api/hepta-runtime-session-dry-run-inventory")"
CLI_JSON="$(curl -fsS "$BASE_URL/api/hepta-cli-command-inventory")"
PROVIDER_JSON="$(curl -fsS "$BASE_URL/api/hepta-provider-metadata-inventory")"
MERGE_JSON="$(curl -fsS "$BASE_URL/api/hepta-merge-completion")"

jq -e '
  .runtime == "hepta-codex"
  and .status == "attention"
  and .compatibility_mode == "native_channel_adapter_disabled_status_inventory"
  and .side_effect_free == true
  and .old_channel_ops_file_count == 13
  and .current_hepta_codex_script_total == 12
  and .native_gateway_source_command_count == 59
  and .missing_route_count == 0
  and .adapter_count == 13
  and .disabled_status_ready_count == 13
  and .live_adapter_enabled_count == 0
  and .channel_status_inventory_ready == true
  and .old_cli_invocation_compatibility_claimed == false
  and .live_channel_read_enabled == false
  and .live_channel_send_enabled == false
  and .owner_handoff_performed == false
  and .side_effects.channel_read_performed == false
  and .side_effects.channel_send_performed == false
  and .side_effects.credential_read == false
  and .side_effects.external_network_read == false
  and .side_effects.external_send_performed == false
  and .side_effects.gateway_mutation_performed == false
  and .side_effects.telegram_owner_handoff_performed == false
  and .side_effects.telegram_read_performed == false
  and .side_effects.telegram_send_performed == false
  and .side_effects.voice_call_performed == false
  and .side_effects.tts_audio_played == false
  and .side_effects.webhook_delivered == false
  and .side_effects.file_transfer_performed == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.filesystem_written == false
' <<<"$CHANNEL_JSON" >/dev/null

for payload in "$RUNTIME_JSON" "$CLI_JSON" "$PROVIDER_JSON" "$MERGE_JSON"; do
  jq -e '
    .runtime == "hepta-codex"
    and .current_hepta_codex_script_total == 12
    and .native_gateway_source_command_count == 59
    and .missing_route_count == 0
  ' <<<"$payload" >/dev/null
done

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta-codex" \
  --arg base_url "$BASE_URL" \
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
    endpoint:"/api/hepta-channel-adapter-status-inventory",
    old_channel_ops_file_count:$channel.old_channel_ops_file_count,
    adapter_count:$channel.adapter_count,
    disabled_status_ready_count:$channel.disabled_status_ready_count,
    live_adapter_enabled_count:$channel.live_adapter_enabled_count,
    current_hepta_codex_script_total:$channel.current_hepta_codex_script_total,
    native_gateway_source_command_count:$channel.native_gateway_source_command_count,
    route_count:$channel.route_count,
    missing_route_count:$channel.missing_route_count,
    live_channel_read_enabled:$channel.live_channel_read_enabled,
    live_channel_send_enabled:$channel.live_channel_send_enabled,
    owner_handoff_performed:$channel.owner_handoff_performed,
    reports_synchronized: (
      $channel.current_hepta_codex_script_total == $runtime_inventory.current_hepta_codex_script_total
      and $channel.native_gateway_source_command_count == $runtime_inventory.native_gateway_source_command_count
      and $channel.current_hepta_codex_script_total == $cli.current_hepta_codex_script_total
      and $channel.native_gateway_source_command_count == $cli.native_gateway_source_command_count
      and $channel.current_hepta_codex_script_total == $provider.current_hepta_codex_script_total
      and $channel.native_gateway_source_command_count == $provider.native_gateway_source_command_count
      and $channel.current_hepta_codex_script_total == $merge.current_hepta_codex_script_total
      and $channel.native_gateway_source_command_count == $merge.native_gateway_source_command_count
      and $channel.missing_route_count == $runtime_inventory.missing_route_count
      and $channel.missing_route_count == $cli.missing_route_count
      and $channel.missing_route_count == $provider.missing_route_count
      and $channel.missing_route_count == $merge.missing_route_count
    ),
    side_effects:$channel.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "channel, runtime/session, provider, CLI, and merge-completion reports are out of sync" >&2
  exit 1
fi

echo "Hepta Codex channel adapter status inventory passed"
