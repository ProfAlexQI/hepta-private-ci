#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
PLAN_JSON="$(curl -fsS "$BASE_URL/api/hepta-provider-channel-dry-run-plan")"
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
  and .compatibility_mode == "native_provider_channel_runtime_dry_run_plan"
  and .side_effect_free == true
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count == 64
  and .missing_route_count == 0
  and .plan_family_count == 5
  and .covered_old_ops_file_count == 43
  and .covered_provider_ops_file_count == 15
  and .covered_search_ops_file_count == 3
  and .covered_channel_ops_file_count == 13
  and .covered_runtime_ops_file_count == 12
  and .dry_run_plan_ready_count == 5
  and .isolated_fixture_contract_count == 5
  and .live_invocation_enabled_count == 0
  and .credential_read_required_count == 0
  and .operator_approval_required_count == 5
  and .provider_prompt_execution_enabled == false
  and .search_network_query_enabled == false
  and .channel_delivery_enabled == false
  and .runtime_store_mutation_enabled == false
  and .isolated_fixture_materialized == false
  and .dry_run_plan_ready == true
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.external_network_read == false
  and .side_effects.search_query_performed == false
  and .side_effects.channel_read_performed == false
  and .side_effects.channel_send_performed == false
  and .side_effects.telegram_owner_handoff_performed == false
  and .side_effects.telegram_read_performed == false
  and .side_effects.telegram_send_performed == false
  and .side_effects.process_spawned == false
  and .side_effects.filesystem_read == false
  and .side_effects.filesystem_written == false
  and .side_effects.task_registry_mutated == false
  and .side_effects.session_store_mutated == false
  and .side_effects.gateway_event_enqueued == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.gateway_mutation_performed == false
  and .side_effects.external_send_performed == false
' <<<"$PLAN_JSON" >/dev/null

for payload in "$RELEASE_JSON" "$MEMORY_JSON" "$LOCAL_JSON" "$CHANNEL_JSON" "$RUNTIME_JSON" "$CLI_JSON" "$PROVIDER_JSON" "$MERGE_JSON"; do
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
  --argjson plan "$PLAN_JSON" \
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
    endpoint:"/api/hepta-provider-channel-dry-run-plan",
    plan_family_count:$plan.plan_family_count,
    covered_old_ops_file_count:$plan.covered_old_ops_file_count,
    covered_provider_ops_file_count:$plan.covered_provider_ops_file_count,
    covered_search_ops_file_count:$plan.covered_search_ops_file_count,
    covered_channel_ops_file_count:$plan.covered_channel_ops_file_count,
    covered_runtime_ops_file_count:$plan.covered_runtime_ops_file_count,
    dry_run_plan_ready_count:$plan.dry_run_plan_ready_count,
    isolated_fixture_contract_count:$plan.isolated_fixture_contract_count,
    live_invocation_enabled_count:$plan.live_invocation_enabled_count,
    credential_read_required_count:$plan.credential_read_required_count,
    operator_approval_required_count:$plan.operator_approval_required_count,
    current_hepta_codex_script_total:$plan.current_hepta_codex_script_total,
    native_gateway_source_command_count:$plan.native_gateway_source_command_count,
    route_count:$plan.route_count,
    missing_route_count:$plan.missing_route_count,
    provider_prompt_execution_enabled:$plan.provider_prompt_execution_enabled,
    search_network_query_enabled:$plan.search_network_query_enabled,
    channel_delivery_enabled:$plan.channel_delivery_enabled,
    runtime_store_mutation_enabled:$plan.runtime_store_mutation_enabled,
    isolated_fixture_materialized:$plan.isolated_fixture_materialized,
    reports_synchronized: (
      $plan.current_hepta_codex_script_total == $release.current_hepta_codex_script_total
      and $plan.native_gateway_source_command_count == $release.native_gateway_source_command_count
      and $plan.current_hepta_codex_script_total == $memory.current_hepta_codex_script_total
      and $plan.native_gateway_source_command_count == $memory.native_gateway_source_command_count
      and $plan.current_hepta_codex_script_total == $local.current_hepta_codex_script_total
      and $plan.native_gateway_source_command_count == $local.native_gateway_source_command_count
      and $plan.current_hepta_codex_script_total == $channel.current_hepta_codex_script_total
      and $plan.native_gateway_source_command_count == $channel.native_gateway_source_command_count
      and $plan.current_hepta_codex_script_total == $runtime_inventory.current_hepta_codex_script_total
      and $plan.native_gateway_source_command_count == $runtime_inventory.native_gateway_source_command_count
      and $plan.current_hepta_codex_script_total == $cli.current_hepta_codex_script_total
      and $plan.native_gateway_source_command_count == $cli.native_gateway_source_command_count
      and $plan.current_hepta_codex_script_total == $provider.current_hepta_codex_script_total
      and $plan.native_gateway_source_command_count == $provider.native_gateway_source_command_count
      and $plan.current_hepta_codex_script_total == $merge.current_hepta_codex_script_total
      and $plan.native_gateway_source_command_count == $merge.native_gateway_source_command_count
      and $plan.missing_route_count == $release.missing_route_count
      and $plan.missing_route_count == $memory.missing_route_count
      and $plan.missing_route_count == $local.missing_route_count
      and $plan.missing_route_count == $channel.missing_route_count
      and $plan.missing_route_count == $runtime_inventory.missing_route_count
      and $plan.missing_route_count == $cli.missing_route_count
      and $plan.missing_route_count == $provider.missing_route_count
      and $plan.missing_route_count == $merge.missing_route_count
    ),
    side_effects:$plan.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "provider/channel dry-run plan and existing inventory reports are out of sync" >&2
  exit 1
fi

echo "Hepta Codex provider/channel dry-run plan passed"
