#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"

GA_JSON="$(curl -fsS "$BASE_URL/api/hepta-public-ga-readiness")"
MERGE_JSON="$(curl -fsS "$BASE_URL/api/hepta-merge-completion")"
PLAN_JSON="$(curl -fsS "$BASE_URL/api/hepta-provider-channel-dry-run-plan")"
RELEASE_JSON="$(curl -fsS "$BASE_URL/api/hepta-release-hardening-status-gate")"
CLI_JSON="$(curl -fsS "$BASE_URL/api/hepta-cli-command-inventory")"
NATIVE_PACKAGING_JSON="$(curl -fsS "$BASE_URL/api/hepta-native-packaging-gate")"
LEGACY_CLOSURE_JSON="$(curl -fsS "$BASE_URL/api/hepta-legacy-compatibility-closure")"
OWNER_JSON="$(curl -fsS "$BASE_URL/api/telegram-owner-handoff")"
POST_JSON="$(curl -fsS "$BASE_URL/api/native-post-activation-plan")"

jq -e '
  .runtime == "hepta"
  and (.status == "blocked" or .status == "ready")
  and .compatibility_mode == "native_public_ga_readiness_gate"
  and .side_effect_free == true
  and .current_hepta_codex_script_total == 17
  and .native_gateway_source_command_count >= 69
  and .missing_route_count == 0
  and .local_reports_synchronized == true
  and .local_gate_matrix_ready == true
  and (.public_ga_ready == false or .public_ga_ready == true)
  and .public_ga_claimed == false
  and .external_public_release_performed == false
  and .native_post_dry_run_evidence_ready == true
  and (.native_post_real_activation_ready == false or .native_post_real_activation_ready == true)
  and (.credentialed_provider_smoke_ready == false or .credentialed_provider_smoke_ready == true)
  and (.channel_live_delivery_ready == false or .channel_live_delivery_ready == true)
  and .old_cli_command_breadth_fully_migrated == true
  and .old_release_hardening_script_execution_compatibility_claimed == true
  and .hepta_native_release_packaging_ready == true
  and .side_effects.public_release_published == false
  and .side_effects.release_artifact_written == false
  and .side_effects.launchd_mutated == false
  and .side_effects.credential_read == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.channel_read_performed == false
  and .side_effects.channel_send_performed == false
  and .side_effects.telegram_owner_handoff_performed == false
  and .side_effects.telegram_read_performed == false
  and .side_effects.telegram_send_performed == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.external_network_read == false
  and .side_effects.external_send_performed == false
' <<<"$GA_JSON" >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --argjson ga "$GA_JSON" \
  --argjson merge "$MERGE_JSON" \
  --argjson plan "$PLAN_JSON" \
  --argjson release "$RELEASE_JSON" \
  --argjson cli "$CLI_JSON" \
  --argjson native_packaging "$NATIVE_PACKAGING_JSON" \
  --argjson legacy_closure "$LEGACY_CLOSURE_JSON" \
  --argjson owner "$OWNER_JSON" \
  --argjson post "$POST_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    endpoint:"/api/hepta-public-ga-readiness",
    public_ga_ready:$ga.public_ga_ready,
    public_ga_claimed:$ga.public_ga_claimed,
    blocker_count:$ga.blocker_count,
    blockers:$ga.blockers,
    local_gate_matrix_ready:$ga.local_gate_matrix_ready,
    local_reports_synchronized:$ga.local_reports_synchronized,
    current_hepta_codex_script_total:$ga.current_hepta_codex_script_total,
    native_gateway_source_command_count:$ga.native_gateway_source_command_count,
    route_count:$ga.route_count,
    missing_route_count:$ga.missing_route_count,
    reports_synchronized: (
      $ga.current_hepta_codex_script_total == $merge.current_hepta_codex_script_total
      and $ga.native_gateway_source_command_count == $merge.native_gateway_source_command_count
      and $ga.current_hepta_codex_script_total == $plan.current_hepta_codex_script_total
      and $ga.native_gateway_source_command_count == $plan.native_gateway_source_command_count
      and $ga.current_hepta_codex_script_total == $release.current_hepta_codex_script_total
      and $ga.native_gateway_source_command_count == $release.native_gateway_source_command_count
      and $ga.current_hepta_codex_script_total == $cli.current_hepta_codex_script_total
      and $ga.native_gateway_source_command_count == $cli.native_gateway_source_command_count
      and $ga.current_hepta_codex_script_total == $native_packaging.current_hepta_codex_script_total
      and $ga.native_gateway_source_command_count == $native_packaging.native_gateway_source_command_count
      and $ga.current_hepta_codex_script_total == $legacy_closure.current_hepta_codex_script_total
      and $ga.native_gateway_source_command_count == $legacy_closure.native_gateway_source_command_count
      and $ga.missing_route_count == $merge.missing_route_count
      and $ga.missing_route_count == $plan.missing_route_count
      and $ga.missing_route_count == $release.missing_route_count
      and $ga.missing_route_count == $cli.missing_route_count
      and $ga.missing_route_count == $native_packaging.missing_route_count
      and $ga.missing_route_count == $legacy_closure.missing_route_count
    ),
    expected_external_blockers:{
      active_owner:$owner.active_owner,
      hepta_takeover_ready:$owner.hepta_takeover_ready,
      native_post_activation_enabled:$post.activation_currently_enabled,
      native_post_single_handler_scope_ready:$post.single_handler_scope_ready,
      old_cli_command_breadth_fully_migrated:$cli.old_cli_command_breadth_fully_migrated,
      release_script_execution_compatibility:$release.old_script_execution_compatibility_claimed,
      native_packaging_local_gate_ready:$native_packaging.local_packaging_gate_ready,
      native_packaging_public_distribution_artifact_written:$native_packaging.public_distribution_artifact_written,
      legacy_closure_ready:$legacy_closure.local_route_script_coverage_ready
    },
    side_effects:$ga.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "public GA readiness report and supporting reports are out of sync" >&2
  exit 1
fi

echo "Hepta public GA readiness gate passed"
