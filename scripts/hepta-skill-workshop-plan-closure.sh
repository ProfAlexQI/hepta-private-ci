#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
MEMORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-memory-capability-absorption-inventory")"
LOCAL_TOOLING_JSON="$(curl -fsS "$BASE_URL/api/hepta-local-tooling-content-inventory")"

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .surface_count == 14
  and .absorbed_or_represented_count == 14
  and .gap_report_ready_count == 14
  and .live_mutation_enabled_count == 0
  and .skill_workshop_write_enabled == false
  and .side_effects.skill_workshop_written == false
  and .side_effects.filesystem_written == false
  and .side_effects.gateway_mutation_performed == false
  and ([.memory_capability_surfaces[]
      | select(.absorbed_or_represented == false)
      | .name] | sort) == []
  and (.memory_capability_surfaces[]
      | select(.name == "skill-workshop")
      | .old_ops_file == "skill_workshop_ops.rs"
        and .migration_status == "represented_by_skill_workshop_plan_closure"
        and .safe_next_mode == "skill_workshop_plan_closed_without_skill_write"
        and .gap_report_ready == true
        and .absorbed_or_represented == true
        and .live_mutation_enabled == false)
' <<<"$MEMORY_JSON" >/dev/null

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .local_tooling_inventory_ready == true
  and .filesystem_write_enabled == false
  and .tool_invocation_enabled == false
  and .side_effects.filesystem_read == false
  and .side_effects.filesystem_written == false
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
    compatibility_mode:"hepta_skill_workshop_plan_closure",
    side_effect_free:true,
    endpoint:"/api/hepta-memory-capability-absorption-inventory",
    supporting_endpoint:"/api/hepta-local-tooling-content-inventory",
    closed_surface:"skill-workshop",
    old_ops_file:"skill_workshop_ops.rs",
    migration_status:"represented_by_skill_workshop_plan_closure",
    absorbed_or_represented_count:$memory.absorbed_or_represented_count,
    live_mutation_enabled_count:$memory.live_mutation_enabled_count,
    gap_only_surface_count:([
      $memory.memory_capability_surfaces[]
      | select(.absorbed_or_represented == false)
    ] | length),
    skill_workshop_write_enabled:$memory.skill_workshop_write_enabled,
    next_slices:[
      "keep live memory/capability mutations disabled until explicit operator approval"
    ],
    side_effects:{
      memory:$memory.side_effects,
      local_tooling:$local_tooling.side_effects
    }
  }')"

printf '%s\n' "$report"
echo "Hepta skill workshop plan closure gate passed"
