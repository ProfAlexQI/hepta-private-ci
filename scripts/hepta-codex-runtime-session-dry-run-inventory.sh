#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
RUNTIME_JSON="$(curl -fsS "$BASE_URL/api/hepta-runtime-session-dry-run-inventory")"
CLI_JSON="$(curl -fsS "$BASE_URL/api/hepta-cli-command-inventory")"
PROVIDER_JSON="$(curl -fsS "$BASE_URL/api/hepta-provider-metadata-inventory")"
MERGE_JSON="$(curl -fsS "$BASE_URL/api/hepta-merge-completion")"

jq -e '
  .runtime == "hepta-codex"
  and .status == "attention"
  and .compatibility_mode == "native_runtime_session_dry_run_inventory"
  and .side_effect_free == true
  and .old_runtime_ops_file_count == 12
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count == 67
  and .missing_route_count == 0
  and .dry_run_surface_count == 12
  and .covered_old_ops_file_count == 12
  and .planner_ready_count == 12
  and .live_mutation_surface_count == 0
  and .dry_run_inventory_ready == true
  and .old_cli_invocation_compatibility_claimed == false
  and .task_registry_mutation_enabled == false
  and .session_store_mutation_enabled == false
  and .gateway_event_enqueue_enabled == false
  and .external_telemetry_push_enabled == false
  and .side_effects.task_registry_mutated == false
  and .side_effects.session_store_mutated == false
  and .side_effects.gateway_event_enqueued == false
  and .side_effects.hook_enqueued == false
  and .side_effects.process_spawned == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.external_network_read == false
  and .side_effects.external_send_performed == false
  and .side_effects.telegram_read_performed == false
  and .side_effects.message_sent == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.filesystem_written == false
' <<<"$RUNTIME_JSON" >/dev/null

jq -e '
  .runtime == "hepta-codex"
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count == 67
  and .missing_route_count == 0
' <<<"$CLI_JSON" >/dev/null

jq -e '
  .runtime == "hepta-codex"
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count == 67
  and .missing_route_count == 0
  and (.provider_live_invocation_enabled == .credentialed_smoke_performed)
' <<<"$PROVIDER_JSON" >/dev/null

jq -e '
  .runtime == "hepta-codex"
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count == 67
  and .route_matrix_ready == true
  and .missing_route_count == 0
  and (.telegram_live_send_enabled == false or .telegram_live_send_enabled == true)
  and (.native_post_real_activation_enabled == false or .native_post_real_activation_enabled == true)
' <<<"$MERGE_JSON" >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta-codex" \
  --arg base_url "$BASE_URL" \
  --argjson runtime_inventory "$RUNTIME_JSON" \
  --argjson cli "$CLI_JSON" \
  --argjson provider "$PROVIDER_JSON" \
  --argjson merge "$MERGE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    endpoint:"/api/hepta-runtime-session-dry-run-inventory",
    old_runtime_ops_file_count:$runtime_inventory.old_runtime_ops_file_count,
    dry_run_surface_count:$runtime_inventory.dry_run_surface_count,
    planner_ready_count:$runtime_inventory.planner_ready_count,
    live_mutation_surface_count:$runtime_inventory.live_mutation_surface_count,
    current_hepta_codex_script_total:$runtime_inventory.current_hepta_codex_script_total,
    native_gateway_source_command_count:$runtime_inventory.native_gateway_source_command_count,
    route_count:$runtime_inventory.route_count,
    missing_route_count:$runtime_inventory.missing_route_count,
    task_registry_mutation_enabled:$runtime_inventory.task_registry_mutation_enabled,
    session_store_mutation_enabled:$runtime_inventory.session_store_mutation_enabled,
    gateway_event_enqueue_enabled:$runtime_inventory.gateway_event_enqueue_enabled,
    external_telemetry_push_enabled:$runtime_inventory.external_telemetry_push_enabled,
    reports_synchronized: (
      $runtime_inventory.current_hepta_codex_script_total == $cli.current_hepta_codex_script_total
      and $runtime_inventory.native_gateway_source_command_count == $cli.native_gateway_source_command_count
      and $runtime_inventory.current_hepta_codex_script_total == $provider.current_hepta_codex_script_total
      and $runtime_inventory.native_gateway_source_command_count == $provider.native_gateway_source_command_count
      and $runtime_inventory.current_hepta_codex_script_total == $merge.current_hepta_codex_script_total
      and $runtime_inventory.native_gateway_source_command_count == $merge.native_gateway_source_command_count
      and $runtime_inventory.missing_route_count == $cli.missing_route_count
      and $runtime_inventory.missing_route_count == $provider.missing_route_count
      and $runtime_inventory.missing_route_count == $merge.missing_route_count
    ),
    side_effects:$runtime_inventory.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "runtime/session, provider, CLI, and merge-completion reports are out of sync" >&2
  exit 1
fi

echo "Hepta Codex runtime/session dry-run inventory passed"
