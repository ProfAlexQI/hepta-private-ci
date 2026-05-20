#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
INVENTORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-cli-command-inventory")"
MERGE_JSON="$(curl -fsS "$BASE_URL/api/hepta-merge-completion")"

jq -e '
  .runtime == "hepta-codex"
  and .status == "attention"
  and .compatibility_mode == "native_cli_command_breadth_inventory"
  and .side_effect_free == true
  and .old_hepta_ops_file_count == 65
  and .old_hepta_rough_command_reference_count == 574
  and .old_hepta_script_total == 20
  and .current_hepta_codex_script_total == 14
  and .native_gateway_source_command_count == 61
  and .missing_route_count == 0
  and .ops_family_count == 5
  and .ops_file_family_covered_count == 65
  and .old_cli_command_breadth_fully_migrated == false
  and .safe_read_only_inventory_ready == true
  and .side_effects.provider_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.external_network_read == false
  and .side_effects.external_send_performed == false
  and .side_effects.telegram_read_performed == false
  and .side_effects.message_sent == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.filesystem_written == false
' <<<"$INVENTORY_JSON" >/dev/null

jq -e '
  .runtime == "hepta-codex"
  and .current_hepta_codex_script_total == 14
  and .native_gateway_source_command_count == 61
  and .route_matrix_ready == true
  and .missing_route_count == 0
  and .telegram_live_send_enabled == false
  and .native_post_real_activation_enabled == false
' <<<"$MERGE_JSON" >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta-codex" \
  --arg base_url "$BASE_URL" \
  --argjson inventory "$INVENTORY_JSON" \
  --argjson merge "$MERGE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    endpoint:"/api/hepta-cli-command-inventory",
    old_hepta_ops_file_count:$inventory.old_hepta_ops_file_count,
    old_hepta_rough_command_reference_count:$inventory.old_hepta_rough_command_reference_count,
    old_hepta_script_total:$inventory.old_hepta_script_total,
    current_hepta_codex_script_total:$inventory.current_hepta_codex_script_total,
    native_gateway_source_command_count:$inventory.native_gateway_source_command_count,
    route_count:$inventory.route_count,
    missing_route_count:$inventory.missing_route_count,
    ops_family_count:$inventory.ops_family_count,
    ops_file_family_covered_count:$inventory.ops_file_family_covered_count,
    old_cli_command_breadth_fully_migrated:$inventory.old_cli_command_breadth_fully_migrated,
    safe_read_only_inventory_ready:$inventory.safe_read_only_inventory_ready,
    merge_completion_synchronized: (
      $merge.current_hepta_codex_script_total == $inventory.current_hepta_codex_script_total
      and $merge.native_gateway_source_command_count == $inventory.native_gateway_source_command_count
      and $merge.missing_route_count == $inventory.missing_route_count
    ),
    side_effects:$inventory.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.merge_completion_synchronized' <<<"$report")" != "true" ]]; then
  echo "merge-completion and CLI inventory reports are out of sync" >&2
  exit 1
fi

echo "Hepta Codex CLI command inventory passed"
