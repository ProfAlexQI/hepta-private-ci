#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
MEMORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-memory-capability-absorption-inventory")"
PROVIDER_JSON="$(curl -fsS "$BASE_URL/api/hepta-provider-metadata-inventory")"

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .surface_count == 14
  and .absorbed_or_represented_count >= 13
  and .gap_report_ready_count == 14
  and .live_mutation_enabled_count == 0
  and .plugin_registry_mutation_enabled == false
  and .side_effects.plugin_registry_mutated == false
  and .side_effects.filesystem_written == false
  and .side_effects.gateway_mutation_performed == false
  and (.memory_capability_surfaces[]
      | select(.name == "plugin-migration")
      | .old_ops_file == "plugin_migration_ops.rs"
        and .migration_status == "represented_by_plugin_migration_plan_closure"
        and .safe_next_mode == "plugin_migration_plan_closed_without_registry_or_filesystem_write"
        and .gap_report_ready == true
        and .absorbed_or_represented == true
        and .live_mutation_enabled == false)
' <<<"$MEMORY_JSON" >/dev/null

jq -e '
  .runtime == "hepta"
  and (.status == "attention" or .status == "ready")
  and .metadata_inventory_ready == true
  and .provider_adapter_count == 15
  and .provider_live_invocation_enabled == false
  and .credentialed_smoke_performed == false
  and (.provider_adapters[]
      | select(.name == "provider-registration")
      | .migration_status == "partially_absorbed_as_registry_metadata"
        and .live_invocation_performed == false
        and .credential_read_performed == false)
  and .side_effects.provider_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.external_network_read == false
  and .side_effects.external_send_performed == false
  and .side_effects.model_invoked == false
  and .side_effects.gateway_mutation_performed == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.filesystem_written == false
' <<<"$PROVIDER_JSON" >/dev/null

report="$(jq -n \
  --argjson memory "$MEMORY_JSON" \
  --argjson provider "$PROVIDER_JSON" \
  '{
    product:"Hepta",
    runtime:"hepta",
    status:"ready",
    compatibility_mode:"hepta_plugin_migration_plan_closure",
    side_effect_free:true,
    endpoint:"/api/hepta-memory-capability-absorption-inventory",
    supporting_endpoint:"/api/hepta-provider-metadata-inventory",
    closed_surface:"plugin-migration",
    old_ops_file:"plugin_migration_ops.rs",
    migration_status:"represented_by_plugin_migration_plan_closure",
    absorbed_or_represented_count:$memory.absorbed_or_represented_count,
    live_mutation_enabled_count:$memory.live_mutation_enabled_count,
    plugin_registry_mutation_enabled:$memory.plugin_registry_mutation_enabled,
    provider_adapter_count:$provider.provider_adapter_count,
    provider_live_invocation_enabled:$provider.provider_live_invocation_enabled,
    credentialed_smoke_performed:$provider.credentialed_smoke_performed,
    next_slices:[],
    side_effects:{
      memory:$memory.side_effects,
      provider:$provider.side_effects
    }
  }')"

printf '%s\n' "$report"
echo "Hepta plugin migration plan closure gate passed"
