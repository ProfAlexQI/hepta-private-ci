#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"

tmp_dir="${TMPDIR:-/tmp}/hepta-memory-intelligence-closure.$$"
mkdir -p "$tmp_dir"
trap 'rm -rf "$tmp_dir"' EXIT

cli_tree="$tmp_dir/hepta-cli.tree"
runtime_tree="$tmp_dir/hepta-runtime.tree"
core_tree="$tmp_dir/hepta-core.tree"

cargo tree --offline --manifest-path "$MANIFEST" -p hepta-cli --edges normal --prefix none >"$cli_tree"
cargo tree --offline --manifest-path "$MANIFEST" -p hepta-runtime --edges normal --prefix none >"$runtime_tree"
cargo tree --offline --manifest-path "$MANIFEST" -p hepta-core --edges normal --prefix none >"$core_tree"

require_tree_package() {
  local tree_file="$1"
  local package_name="$2"
  if ! grep -Eq "^${package_name} v" "$tree_file"; then
    echo "expected ${package_name} in ${tree_file}" >&2
    exit 1
  fi
}

reject_core_package() {
  local package_name="$1"
  if grep -Eq "^${package_name} v" "$core_tree"; then
    echo "hepta-core must not directly depend on ${package_name}" >&2
    exit 1
  fi
}

for package_name in hepta-gateway hepta-runtime hepta-intelligence hepta-kernel hepta-memory hepta-plugins; do
  require_tree_package "$cli_tree" "$package_name"
done

for package_name in hepta-intelligence hepta-kernel hepta-memory hepta-plugins; do
  require_tree_package "$runtime_tree" "$package_name"
done

reject_core_package hepta-intelligence
reject_core_package hepta-memory
reject_core_package hepta-runtime
reject_core_package hepta-kernel

MEMORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-memory-capability-absorption-inventory")"

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .memory_capability_inventory_ready == true
  and .surface_count == 14
  and .absorbed_or_represented_count == 13
  and .gap_report_ready_count == 14
  and .live_mutation_enabled_count == 0
  and (.memory_capability_surfaces[]
      | select(.name == "memory-rem")
      | .migration_status == "represented_by_memory_rem_status_closure"
        and .absorbed_or_represented == true
        and .live_mutation_enabled == false)
  and (.memory_capability_surfaces[]
      | select(.name == "memory-tools")
      | .migration_status == "represented_by_memory_tools_catalog_closure"
        and .absorbed_or_represented == true
        and .live_mutation_enabled == false)
  and (.memory_capability_surfaces[]
      | select(.name == "native-residual-runtime")
      | .migration_status == "represented_by_native_residual_runtime_status_closure"
        and .absorbed_or_represented == true
        and .live_mutation_enabled == false)
  and (.memory_capability_surfaces[]
      | select(.name == "plugin-migration")
      | .migration_status == "represented_by_plugin_migration_plan_closure"
        and .absorbed_or_represented == true
        and .live_mutation_enabled == false)
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
  and .side_effects.filesystem_written == false
  and .side_effects.external_network_read == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.channel_send_performed == false
  and .side_effects.gateway_mutation_performed == false
  and ([.memory_capability_surfaces[]
      | select(.absorbed_or_represented == false)
      | .name] | sort) == [
        "skill-workshop"
      ]
' <<<"$MEMORY_JSON" >/dev/null

report="$(jq -n \
  --arg base_url "$BASE_URL" \
  --argjson memory "$MEMORY_JSON" \
  '{
    product:"Hepta",
    runtime:"hepta",
    status:"attention",
    compatibility_mode:"hepta_memory_intelligence_closure_gate",
    side_effect_free:true,
    active_service_stack_consumes_memory_intelligence:true,
    hepta_core_direct_memory_intelligence_dependency_count:0,
    hepta_core_dependency_boundary_ready:true,
    runtime_memory_intelligence_dependencies_ready:true,
    memory_capability_endpoint:"/api/hepta-memory-capability-absorption-inventory",
    memory_surface_count:$memory.surface_count,
    absorbed_or_represented_count:$memory.absorbed_or_represented_count,
    gap_report_ready_count:$memory.gap_report_ready_count,
    live_mutation_enabled_count:$memory.live_mutation_enabled_count,
    full_live_memory_intelligence_closure_ready:false,
    gap_only_surface_count:([
      $memory.memory_capability_surfaces[]
      | select(.absorbed_or_represented == false)
    ] | length),
    gap_only_surfaces:[
      $memory.memory_capability_surfaces[]
      | select(.absorbed_or_represented == false)
      | {
          name,
          old_ops_file,
          migration_status,
          safe_next_mode,
          live_mutation_enabled
        }
    ],
    blocked_live_mutations:[
      "memory_store_mutation",
      "capability_registry_mutation",
      "plugin_registry_mutation",
      "coding_agent_spawn",
      "search_provider_live_query",
      "skill_workshop_write"
    ],
    next_slices:[
      "add skill-workshop plan closure without skill write"
    ],
    side_effects:$memory.side_effects
  }')"

printf '%s\n' "$report"
echo "Hepta memory/intelligence closure gate passed"
