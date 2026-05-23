#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"

CLOSURE_JSON="$(curl -fsS "$BASE_URL/api/hepta-legacy-compatibility-closure")"
CLI_JSON="$(curl -fsS "$BASE_URL/api/hepta-cli-command-inventory")"
RELEASE_JSON="$(curl -fsS "$BASE_URL/api/hepta-release-hardening-status-gate")"
GA_JSON="$(curl -fsS "$BASE_URL/api/hepta-public-ga-readiness")"

jq -e '
  .runtime == "hepta-codex"
  and .status == "ready"
  and .compatibility_mode == "native_legacy_cli_script_compatibility_closure"
  and .side_effect_free == true
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count == 66
  and .missing_route_count == 0
  and .old_hepta_ops_file_count == 65
  and .old_hepta_rough_command_reference_count == 574
  and .old_hepta_script_total == 20
  and .ops_file_family_covered_count == 65
  and .release_hardening_script_family_count == 12
  and .release_hardening_status_gate_ready_count == 12
  and .local_route_script_coverage_ready == true
  and .old_cli_command_breadth_fully_migrated == true
  and .old_release_hardening_script_execution_compatibility_claimed == true
  and .dangerous_live_execution_reenabled == false
  and .credentialed_live_smoke_deferred == true
  and .external_release_deferred == true
  and .side_effects.process_spawned == false
  and .side_effects.filesystem_read == false
  and .side_effects.filesystem_written == false
  and .side_effects.release_artifact_written == false
  and .side_effects.credential_read == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.external_network_read == false
  and .side_effects.channel_read_performed == false
  and .side_effects.channel_send_performed == false
  and .side_effects.telegram_owner_handoff_performed == false
  and .side_effects.telegram_read_performed == false
  and .side_effects.telegram_send_performed == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.gateway_mutation_performed == false
  and .side_effects.external_send_performed == false
' <<<"$CLOSURE_JSON" >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta-codex" \
  --arg base_url "$BASE_URL" \
  --argjson closure "$CLOSURE_JSON" \
  --argjson cli "$CLI_JSON" \
  --argjson release "$RELEASE_JSON" \
  --argjson ga "$GA_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    endpoint:"/api/hepta-legacy-compatibility-closure",
    current_hepta_codex_script_total:$closure.current_hepta_codex_script_total,
    native_gateway_source_command_count:$closure.native_gateway_source_command_count,
    route_count:$closure.route_count,
    missing_route_count:$closure.missing_route_count,
    old_cli_command_breadth_fully_migrated:$closure.old_cli_command_breadth_fully_migrated,
    old_release_hardening_script_execution_compatibility_claimed:$closure.old_release_hardening_script_execution_compatibility_claimed,
    public_ga_ready:$ga.public_ga_ready,
    public_ga_blocker_count:$ga.blocker_count,
    reports_synchronized: (
      $closure.current_hepta_codex_script_total == $cli.current_hepta_codex_script_total
      and $closure.native_gateway_source_command_count == $cli.native_gateway_source_command_count
      and $closure.current_hepta_codex_script_total == $release.current_hepta_codex_script_total
      and $closure.native_gateway_source_command_count == $release.native_gateway_source_command_count
      and $closure.current_hepta_codex_script_total == $ga.current_hepta_codex_script_total
      and $closure.native_gateway_source_command_count == $ga.native_gateway_source_command_count
      and $closure.missing_route_count == $cli.missing_route_count
      and $closure.missing_route_count == $release.missing_route_count
      and $closure.missing_route_count == $ga.missing_route_count
    ),
    side_effects:$closure.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "legacy compatibility, CLI, release, and public GA reports are out of sync" >&2
  exit 1
fi

echo "Hepta Codex legacy compatibility closure passed"
