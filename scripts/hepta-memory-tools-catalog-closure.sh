#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
MEMORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-memory-capability-absorption-inventory")"
LOCAL_TOOLING_JSON="$(curl -fsS "$BASE_URL/api/hepta-local-tooling-content-inventory")"

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .surface_count == 14
  and .absorbed_or_represented_count >= 11
  and .gap_report_ready_count == 14
  and .live_mutation_enabled_count == 0
  and .memory_store_mutation_enabled == false
  and .side_effects.memory_store_mutated == false
  and .side_effects.filesystem_written == false
  and .side_effects.gateway_mutation_performed == false
  and (.memory_capability_surfaces[]
      | select(.name == "memory-tools")
      | .old_ops_file == "memory_tools_ops.rs"
        and .migration_status == "represented_by_memory_tools_catalog_closure"
        and .safe_next_mode == "memory_tools_catalog_closed_without_tool_invocation"
        and .gap_report_ready == true
        and .absorbed_or_represented == true
        and .live_mutation_enabled == false)
' <<<"$MEMORY_JSON" >/dev/null

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .local_tooling_inventory_ready == true
  and .tool_invocation_enabled_count == 0
  and .tool_invocation_enabled == false
  and .side_effects.process_spawned == false
  and .side_effects.filesystem_read == false
  and .side_effects.filesystem_written == false
  and .side_effects.external_network_read == false
  and .side_effects.tool_invoked == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.gateway_mutation_performed == false
  and .side_effects.external_send_performed == false
' <<<"$LOCAL_TOOLING_JSON" >/dev/null

report="$(jq -n \
  --argjson memory "$MEMORY_JSON" \
  --argjson local_tooling "$LOCAL_TOOLING_JSON" \
  '{
    product:"Hepta",
    runtime:"hepta",
    status:"ready",
    compatibility_mode:"hepta_memory_tools_catalog_closure",
    side_effect_free:true,
    endpoint:"/api/hepta-memory-capability-absorption-inventory",
    supporting_endpoint:"/api/hepta-local-tooling-content-inventory",
    closed_surface:"memory-tools",
    old_ops_file:"memory_tools_ops.rs",
    migration_status:"represented_by_memory_tools_catalog_closure",
    absorbed_or_represented_count:$memory.absorbed_or_represented_count,
    live_mutation_enabled_count:$memory.live_mutation_enabled_count,
    tool_invocation_enabled_count:$local_tooling.tool_invocation_enabled_count,
    tool_invocation_enabled:$local_tooling.tool_invocation_enabled,
    next_slices:[
      "add plugin-migration plan closure without registry or filesystem write",
      "add skill-workshop plan closure without skill write"
    ],
    side_effects:{
      memory:$memory.side_effects,
      local_tooling:$local_tooling.side_effects
    }
  }')"

printf '%s\n' "$report"
echo "Hepta memory-tools catalog closure gate passed"
