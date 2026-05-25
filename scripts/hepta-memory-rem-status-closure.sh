#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
MEMORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-memory-capability-absorption-inventory")"

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .surface_count == 14
  and .absorbed_or_represented_count >= 10
  and .gap_report_ready_count == 14
  and .live_mutation_enabled_count == 0
  and .memory_store_mutation_enabled == false
  and .side_effects.memory_store_mutated == false
  and .side_effects.filesystem_written == false
  and .side_effects.gateway_mutation_performed == false
  and (.memory_capability_surfaces[]
      | select(.name == "memory-rem")
      | .old_ops_file == "memory_rem_ops.rs"
        and .migration_status == "represented_by_memory_rem_status_closure"
        and .safe_next_mode == "memory_rem_status_closed_without_memory_store_mutation"
        and .gap_report_ready == true
        and .absorbed_or_represented == true
        and .live_mutation_enabled == false)
' <<<"$MEMORY_JSON" >/dev/null

report="$(jq -n \
  --argjson memory "$MEMORY_JSON" \
  '{
    product:"Hepta",
    runtime:"hepta",
    status:"ready",
    compatibility_mode:"hepta_memory_rem_status_closure",
    side_effect_free:true,
    endpoint:"/api/hepta-memory-capability-absorption-inventory",
    closed_surface:"memory-rem",
    old_ops_file:"memory_rem_ops.rs",
    migration_status:"represented_by_memory_rem_status_closure",
    absorbed_or_represented_count:$memory.absorbed_or_represented_count,
    live_mutation_enabled_count:$memory.live_mutation_enabled_count,
    memory_store_mutation_enabled:$memory.memory_store_mutation_enabled,
    next_slices:[
      "add native-residual-runtime status closure without process or gateway mutation",
      "add plugin-migration plan closure without registry or filesystem write",
      "add skill-workshop plan closure without skill write"
    ],
    side_effects:$memory.side_effects
  }')"

printf '%s\n' "$report"
echo "Hepta memory-rem status closure gate passed"
