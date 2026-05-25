#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
MEMORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-memory-capability-absorption-inventory")"
RUNTIME_JSON="$(curl -fsS "$BASE_URL/api/hepta-runtime-session-dry-run-inventory")"

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .surface_count == 14
  and .absorbed_or_represented_count == 12
  and .gap_report_ready_count == 14
  and .live_mutation_enabled_count == 0
  and .memory_store_mutation_enabled == false
  and .side_effects.memory_store_mutated == false
  and .side_effects.filesystem_written == false
  and .side_effects.gateway_mutation_performed == false
  and (.memory_capability_surfaces[]
      | select(.name == "native-residual-runtime")
      | .old_ops_file == "native_residual_runtime_ops.rs"
        and .migration_status == "represented_by_native_residual_runtime_status_closure"
        and .safe_next_mode == "residual_runtime_status_closed_without_process_or_gateway_mutation"
        and .gap_report_ready == true
        and .absorbed_or_represented == true
        and .live_mutation_enabled == false)
' <<<"$MEMORY_JSON" >/dev/null

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .dry_run_inventory_ready == true
  and .dry_run_surface_count == 12
  and .live_mutation_surface_count == 0
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
  and .side_effects.external_network_read == false
  and .side_effects.external_send_performed == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.filesystem_written == false
' <<<"$RUNTIME_JSON" >/dev/null

report="$(jq -n \
  --argjson memory "$MEMORY_JSON" \
  --argjson runtime "$RUNTIME_JSON" \
  '{
    product:"Hepta",
    runtime:"hepta",
    status:"ready",
    compatibility_mode:"hepta_native_residual_runtime_status_closure",
    side_effect_free:true,
    endpoint:"/api/hepta-memory-capability-absorption-inventory",
    supporting_endpoint:"/api/hepta-runtime-session-dry-run-inventory",
    closed_surface:"native-residual-runtime",
    old_ops_file:"native_residual_runtime_ops.rs",
    migration_status:"represented_by_native_residual_runtime_status_closure",
    absorbed_or_represented_count:$memory.absorbed_or_represented_count,
    live_mutation_enabled_count:$memory.live_mutation_enabled_count,
    runtime_dry_run_surface_count:$runtime.dry_run_surface_count,
    runtime_live_mutation_surface_count:$runtime.live_mutation_surface_count,
    next_slices:[
      "add plugin-migration plan closure without registry or filesystem write",
      "add skill-workshop plan closure without skill write"
    ],
    side_effects:{
      memory:$memory.side_effects,
      runtime:$runtime.side_effects
    }
  }')"

printf '%s\n' "$report"
echo "Hepta native residual runtime status closure gate passed"
