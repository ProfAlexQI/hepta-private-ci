#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$REPO_ROOT/codex-rs/target/release/hepta}}"
INSTALLED_BIN="${HEPTA_INSTALLED_BIN:-${HEPTA_CODEX_INSTALLED_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"
BACKUP_ROOT="${HEPTA_BACKUP_ROOT:-$HOME/.openclaw/workspace/backups}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

if [[ -z "${HEPTA_RELEASE_BIN:-}${HEPTA_CODEX_RELEASE_BIN:-}" && ! -f "$RELEASE_BIN" && -f "$INSTALLED_BIN" ]]; then
  RELEASE_BIN="$INSTALLED_BIN"
fi

release_sha=""
installed_sha=""
if [[ -f "$RELEASE_BIN" ]]; then
  release_sha="$(shasum -a 256 "$RELEASE_BIN" | awk '{print $1}')"
fi
if [[ -f "$INSTALLED_BIN" ]]; then
  installed_sha="$(shasum -a 256 "$INSTALLED_BIN" | awk '{print $1}')"
fi

rollback_backup_count="$(
  find "$BACKUP_ROOT" -maxdepth 2 -type f -name hepta.previous 2>/dev/null \
    | grep -c '/hepta-active-binary-' || true
)"

MEMORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-memory-capability-absorption-inventory")"
PACKET_JSON="$(curl -fsS "$BASE_URL/api/hepta-public-ga-operator-approval-packet")"
RELEASE_JSON="$(curl -fsS "$BASE_URL/api/hepta-release-hardening-status-gate")"
PROVIDER_PLAN_JSON="$(curl -fsS "$BASE_URL/api/hepta-provider-channel-dry-run-plan")"
RUNTIME_JSON="$(curl -fsS "$BASE_URL/api/hepta-runtime-session-dry-run-inventory")"
LOCAL_JSON="$(curl -fsS "$BASE_URL/api/hepta-local-tooling-content-inventory")"
CORE_JSON="$(curl -fsS "$BASE_URL/api/hepta-core-fusion-readiness")"
DEPENDENCY_JSON="$(curl -fsS "$BASE_URL/api/hepta-engine-dependency-closure")"

jq -n -e \
  --arg release_sha "$release_sha" \
  --arg installed_sha "$installed_sha" \
  --argjson rollback_backup_count "$rollback_backup_count" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson memory "$MEMORY_JSON" \
  --argjson packet "$PACKET_JSON" \
  --argjson release "$RELEASE_JSON" \
  --argjson provider_plan "$PROVIDER_PLAN_JSON" \
  --argjson runtime_inventory "$RUNTIME_JSON" \
  --argjson local "$LOCAL_JSON" \
  --argjson core "$CORE_JSON" \
  --argjson dependency "$DEPENDENCY_JSON" \
  '
    $release_sha != ""
    and $installed_sha != ""
    and $release_sha == $installed_sha
    and $rollback_backup_count >= 1
    and $min_long_soak_samples >= 24
    and $memory.runtime == "hepta"
    and ($memory.status == "attention" or $memory.status == "ready")
    and $memory.surface_count == 14
    and $memory.absorbed_or_represented_count == 14
    and $memory.gap_report_ready_count == 14
    and $memory.live_mutation_enabled_count == 0
    and $memory.memory_capability_inventory_ready == true
    and $memory.memory_store_mutation_enabled == false
    and $memory.capability_registry_mutation_enabled == false
    and $memory.plugin_registry_mutation_enabled == false
    and $memory.coding_agent_spawn_enabled == false
    and $memory.search_provider_live_query_enabled == false
    and $memory.skill_workshop_write_enabled == false
    and ($memory.side_effects | to_entries | all(.value == false))
    and $packet.runtime == "hepta"
    and $packet.status == "ready"
    and $packet.approval_packet_ready == true
    and $packet.safe_default_mode == "plan_only_no_live_mutation"
    and $packet.irreversible_actions_blocked_by_default == true
    and $packet.required_operator_approval_count == 8
    and ($packet.side_effects | to_entries | all(.value == false))
    and $release.runtime == "hepta"
    and ($release.status == "attention" or $release.status == "ready")
    and $release.release_hardening_status_gate_ready == true
    and $release.status_gate_count == 12
    and $release.local_status_gate_ready_count == 12
    and $release.live_execution_enabled_count == 0
    and $release.operator_approval_required_count == 12
    and $release.autonomous_subagent_spawn_enabled == false
    and ($release.side_effects | to_entries | all(.value == false))
    and $provider_plan.runtime == "hepta"
    and ($provider_plan.status == "attention" or $provider_plan.status == "ready")
    and $provider_plan.dry_run_plan_ready == true
    and $provider_plan.live_invocation_enabled_count == 0
    and $provider_plan.credential_read_required_count == 0
    and $provider_plan.operator_approval_required_count == 5
    and $provider_plan.provider_prompt_execution_enabled == false
    and $provider_plan.search_network_query_enabled == false
    and $provider_plan.channel_delivery_enabled == false
    and $provider_plan.runtime_store_mutation_enabled == false
    and $provider_plan.isolated_fixture_materialized == false
    and ($provider_plan.side_effects | to_entries | all(.value == false))
    and $runtime_inventory.runtime == "hepta"
    and $runtime_inventory.live_mutation_surface_count == 0
    and $runtime_inventory.task_registry_mutation_enabled == false
    and $runtime_inventory.session_store_mutation_enabled == false
    and $runtime_inventory.gateway_event_enqueue_enabled == false
    and $runtime_inventory.external_telemetry_push_enabled == false
    and ($runtime_inventory.side_effects | to_entries | all(.value == false))
    and $local.runtime == "hepta"
    and $local.live_process_enabled_count == 0
    and $local.filesystem_touch_enabled_count == 0
    and $local.network_read_enabled_count == 0
    and $local.tool_invocation_enabled_count == 0
    and $local.process_execution_enabled == false
    and $local.filesystem_read_enabled == false
    and $local.filesystem_write_enabled == false
    and $local.network_read_enabled == false
    and $local.tool_invocation_enabled == false
    and ($local.side_effects | to_entries | all(.value == false))
    and $core.runtime == "hepta"
    and $core.status == "ready"
    and $core.full_fusion_complete == true
    and $core.active_binary_package == "hepta-cli"
    and $core.phase_5_engine_dependency_closure_remaining_dependency_count == 0
    and ($core.phase_5_engine_dependency_closure_blockers | length) == 0
    and $dependency.runtime == "hepta"
    and $dependency.status == "ready"
    and $dependency.full_fusion_complete == true
    and $dependency.remaining_direct_dependency_count == 0
    and ($dependency.blockers | length) == 0
    and $memory.current_hepta_codex_script_total == $packet.current_hepta_codex_script_total
    and $memory.native_gateway_source_command_count == $packet.native_gateway_source_command_count
    and $memory.current_hepta_codex_script_total == $release.current_hepta_codex_script_total
    and $memory.native_gateway_source_command_count == $release.native_gateway_source_command_count
    and $memory.current_hepta_codex_script_total == $provider_plan.current_hepta_codex_script_total
    and $memory.native_gateway_source_command_count == $provider_plan.native_gateway_source_command_count
    and $memory.current_hepta_codex_script_total == $runtime_inventory.current_hepta_codex_script_total
    and $memory.native_gateway_source_command_count == $runtime_inventory.native_gateway_source_command_count
    and $memory.current_hepta_codex_script_total == $local.current_hepta_codex_script_total
    and $memory.native_gateway_source_command_count == $local.native_gateway_source_command_count
    and $memory.missing_route_count == 0
    and $packet.missing_route_count == 0
    and $release.missing_route_count == 0
    and $provider_plan.missing_route_count == 0
    and $runtime_inventory.missing_route_count == 0
    and $local.missing_route_count == 0
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg release_sha "$release_sha" \
  --arg installed_sha "$installed_sha" \
  --arg installed_bin "$INSTALLED_BIN" \
  --arg release_bin "$RELEASE_BIN" \
  --argjson rollback_backup_count "$rollback_backup_count" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson memory "$MEMORY_JSON" \
  --argjson packet "$PACKET_JSON" \
  --argjson release "$RELEASE_JSON" \
  --argjson provider_plan "$PROVIDER_PLAN_JSON" \
  --argjson runtime_inventory "$RUNTIME_JSON" \
  --argjson local "$LOCAL_JSON" \
  --argjson core "$CORE_JSON" \
  --argjson dependency "$DEPENDENCY_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_live_mutation_governance_gate",
    governance_mode:"operator_approval_required_before_live_mutation",
    live_mutation_execution_ready:false,
    approval_packet_ready:$packet.approval_packet_ready,
    safe_default_mode:$packet.safe_default_mode,
    irreversible_actions_blocked_by_default:$packet.irreversible_actions_blocked_by_default,
    memory_capability_surface_count:$memory.surface_count,
    absorbed_or_represented_count:$memory.absorbed_or_represented_count,
    live_mutation_enabled_count:$memory.live_mutation_enabled_count,
    mutation_approval_required_count:(
      $packet.required_operator_approval_count
      + $release.operator_approval_required_count
      + $provider_plan.operator_approval_required_count
    ),
    release_installed_sha_match:($release_sha != "" and $release_sha == $installed_sha),
    release_bin:$release_bin,
    installed_bin:$installed_bin,
    installed_sha:$installed_sha,
    rollback_anchor_present:($rollback_backup_count >= 1),
    rollback_backup_count:$rollback_backup_count,
    minimum_long_soak_required_samples:$min_long_soak_samples,
    long_soak_required_before_mutation:true,
    long_soak_executed_by_this_gate:false,
    required_long_soak_command:("HEPTA_SOAK_SAMPLES=" + ($min_long_soak_samples|tostring) + " HEPTA_SOAK_INTERVAL_SECONDS=5 scripts/hepta-live-soak.sh"),
    reports_synchronized: (
      $memory.current_hepta_codex_script_total == $packet.current_hepta_codex_script_total
      and $memory.native_gateway_source_command_count == $packet.native_gateway_source_command_count
      and $memory.current_hepta_codex_script_total == $release.current_hepta_codex_script_total
      and $memory.native_gateway_source_command_count == $release.native_gateway_source_command_count
      and $memory.current_hepta_codex_script_total == $provider_plan.current_hepta_codex_script_total
      and $memory.native_gateway_source_command_count == $provider_plan.native_gateway_source_command_count
      and $memory.current_hepta_codex_script_total == $runtime_inventory.current_hepta_codex_script_total
      and $memory.native_gateway_source_command_count == $runtime_inventory.native_gateway_source_command_count
      and $memory.current_hepta_codex_script_total == $local.current_hepta_codex_script_total
      and $memory.native_gateway_source_command_count == $local.native_gateway_source_command_count
      and $memory.missing_route_count == 0
      and $packet.missing_route_count == 0
      and $release.missing_route_count == 0
      and $provider_plan.missing_route_count == 0
      and $runtime_inventory.missing_route_count == 0
      and $local.missing_route_count == 0
    ),
    core_full_fusion_complete:$core.full_fusion_complete,
    remaining_direct_dependency_count:$dependency.remaining_direct_dependency_count,
    blocked_live_mutations:[
      "memory_store_mutation",
      "capability_registry_mutation",
      "plugin_registry_mutation",
      "coding_agent_spawn",
      "search_provider_live_query",
      "skill_workshop_write",
      "provider_model_invocation",
      "channel_delivery",
      "runtime_store_mutation",
      "gateway_event_enqueue"
    ],
    required_before_any_live_mutation:[
      "scoped_operator_approval_id",
      "current_installed_binary_backup",
      "rollback_drill_or_revert_command",
      "minimum_24_sample_live_soak_after_plan",
      "single_surface_activation_scope",
      "post_activation_watchdog_and_soak",
      "side_effect_receipt_with_no_secret_values"
    ],
    side_effects:{
      memory_store_mutated:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      coding_agent_spawned:false,
      skill_workshop_written:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      filesystem_written:false,
      release_artifact_written:false,
      launchd_mutated:false,
      external_send_performed:false
    }
  }')"

printf '%s\n' "$report"
echo "Hepta live mutation governance gate passed"
