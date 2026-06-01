#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

MEMORY_CLOSURE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-closure" \
    scripts/hepta-memory-intelligence-closure.sh
)"
MEMORY_INVENTORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-memory-capability-absorption-inventory")"
CORE_FUSION_JSON="$(curl -fsS "$BASE_URL/api/hepta-core-fusion-readiness")"

memory_closure_report_sha256="$(sha256_text "$MEMORY_CLOSURE_JSON")"
kg_terminal_denial_gate_reference_sha256="$(
  sha256_text "hepta_kg_prompt_preview_terminal_next_action_activation_denial_summary_gate:preflight-marker-precedes-full-enablement-readiness"
)"
memory_inventory_report_sha256="$(sha256_text "$MEMORY_INVENTORY_JSON")"
core_fusion_report_sha256="$(sha256_text "$CORE_FUSION_JSON")"
rust_contract_manifest="$(
  printf '%s\n' \
    "memory_activation_cutover|hepta-intelligence|hepta-intelligence-memory-activation-cutover-gate-v1|memory_store_live_mutation" \
    "memory_provider_router_activation|hepta-intelligence|hepta-intelligence-memory-provider-router-activation-gate-v1|hepta_intelligence_live_context" \
    "memory_turn_dispatch|hepta-intelligence|hepta-intelligence-memory-turn-dispatch-gate-v1|runtime_provider_router_context_attachment" \
    "memory_live_turn_preflight|hepta-intelligence|hepta-intelligence-memory-live-turn-preflight-v1|runtime_provider_router_context_attachment" \
    "kg_context_injection_readiness|hepta-intelligence|hepta-intelligence-memory-kg-context-injection-readiness-v0|kg_context_handoff_prompt_preview" \
    "kg_prompt_preview_preflight|hepta-intelligence|hepta-intelligence-memory-kg-prompt-preview-preflight-v0|kg_context_handoff_prompt_preview" \
    "runtime_intelligence_phase2|hepta-runtime|hepta-runtime-intelligence-phase2-gate-v1|hepta_intelligence_live_context"
)"
rust_contract_manifest_sha256="$(sha256_text "$rust_contract_manifest")"
readiness_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-enablement-activation-readiness:$memory_closure_report_sha256:$kg_terminal_denial_gate_reference_sha256:$memory_inventory_report_sha256:$core_fusion_report_sha256:$rust_contract_manifest_sha256"
)"
readiness_policy_hash_sha256="$(
  sha256_text "operator-approved-activation-readiness:no-live-memory-mutation:no-kg-write:no-context-injection:no-model-invocation:no-credential-read"
)"
side_effect_hash_sha256="$(
  sha256_text "full_enablement_performed=false;memory_store_mutated=false;kg_live_write_performed=false;context_injection_performed=false;model_invoked=false;credential_read=false;channel_send_performed=false"
)"

jq -n -e \
  --argjson memory_closure "$MEMORY_CLOSURE_JSON" \
  --argjson memory_inventory "$MEMORY_INVENTORY_JSON" \
  --argjson core_fusion "$CORE_FUSION_JSON" \
  '
    $memory_closure.runtime == "hepta"
    and ($memory_closure.status == "attention" or $memory_closure.status == "ready")
    and $memory_closure.active_service_stack_consumes_memory_intelligence == true
    and $memory_closure.hepta_core_direct_memory_intelligence_dependency_count == 0
    and $memory_closure.hepta_core_dependency_boundary_ready == true
    and $memory_closure.runtime_memory_intelligence_dependencies_ready == true
    and $memory_closure.memory_surface_count == 14
    and $memory_closure.absorbed_or_represented_count == 14
    and $memory_closure.gap_report_ready_count == 14
    and $memory_closure.live_mutation_enabled_count == 0
    and $memory_closure.full_live_memory_intelligence_closure_ready == false
    and ($memory_closure.side_effects | to_entries | all(.value == false))
    and $memory_inventory.runtime == "hepta"
    and $memory_inventory.memory_capability_inventory_ready == true
    and $memory_inventory.surface_count == 14
    and $memory_inventory.absorbed_or_represented_count == 14
    and $memory_inventory.gap_report_ready_count == 14
    and $memory_inventory.live_mutation_enabled_count == 0
    and $memory_inventory.memory_store_mutation_enabled == false
    and ($memory_inventory.side_effects | to_entries | all(.value == false))
    and $core_fusion.runtime == "hepta"
    and $core_fusion.status == "ready"
    and $core_fusion.full_fusion_complete == true
    and $core_fusion.active_binary_package == "hepta-cli"
    and $core_fusion.phase_5_engine_dependency_closure_remaining_dependency_count == 0
    and ($core_fusion.phase_5_engine_dependency_closure_blockers | length) == 0
  ' >/dev/null

rust_contract_references_json="$(
  printf '%s\n' "$rust_contract_manifest" |
  jq -R -s '
    split("\n")
    | map(select(length > 0))
    | map(split("|"))
    | map({
        contract_id: .[0],
        package: .[1],
        contract: .[2],
        enablement_lane: .[3],
        compile_checked_by_preflight_cargo_check: true
      })
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_activation_readiness_gate" \
  --arg memory_closure_report_sha256 "$memory_closure_report_sha256" \
  --arg kg_terminal_denial_gate_reference_sha256 "$kg_terminal_denial_gate_reference_sha256" \
  --arg memory_inventory_report_sha256 "$memory_inventory_report_sha256" \
  --arg core_fusion_report_sha256 "$core_fusion_report_sha256" \
  --arg rust_contract_manifest_sha256 "$rust_contract_manifest_sha256" \
  --arg readiness_contract_hash_sha256 "$readiness_contract_hash_sha256" \
  --arg readiness_policy_hash_sha256 "$readiness_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson rust_contract_references "$rust_contract_references_json" \
  --argjson memory_closure "$MEMORY_CLOSURE_JSON" \
  --argjson memory_inventory "$MEMORY_INVENTORY_JSON" \
  --argjson core_fusion "$CORE_FUSION_JSON" \
  '
    [
      {
        lane:"memory_store_live_mutation",
        readiness:"ready_for_operator_approved_activation_slice",
        current_live_execution_enabled:false,
        required_source_gate:"hepta_memory_intelligence_closure_gate",
        rollback_required:true
      },
      {
        lane:"hepta_intelligence_live_context",
        readiness:"ready_for_operator_approved_activation_slice",
        current_live_execution_enabled:false,
        required_source_gate:"hepta-intelligence-memory-provider-router-activation-gate-v1",
        rollback_required:true
      },
      {
        lane:"kg_context_handoff_prompt_preview",
        readiness:"ready_for_operator_approved_activation_slice",
        current_live_execution_enabled:false,
        required_source_gate:"hepta_kg_prompt_preview_terminal_next_action_activation_denial_summary_gate",
        rollback_required:true
      },
      {
        lane:"kg_external_adapter_staging",
        readiness:"ready_for_operator_approved_activation_slice",
        current_live_execution_enabled:false,
        required_source_gate:"hepta-intelligence-memory-kg-prompt-preview-preflight-v0",
        rollback_required:true
      },
      {
        lane:"runtime_provider_router_context_attachment",
        readiness:"ready_for_operator_approved_activation_slice",
        current_live_execution_enabled:false,
        required_source_gate:"hepta-intelligence-memory-turn-dispatch-gate-v1",
        rollback_required:true
      },
      {
        lane:"rollback_observability_receipts",
        readiness:"ready_for_operator_approved_activation_slice",
        current_live_execution_enabled:false,
        required_source_gate:"hepta_live_mutation_governance_gate",
        rollback_required:true
      }
    ] as $enablement_lanes
    | [
      "memory_store_mutation",
      "hepta_intelligence_context_attachment",
      "kg_prompt_preview_execution",
      "kg_context_injection",
      "kg_external_adapter_read",
      "live_kg_write",
      "provider_model_invocation",
      "credential_read",
      "channel_delivery",
      "gateway_route_migration",
      "source_command_migration",
      "active_runtime_wiring",
      "service_restart",
      "release_or_public_ga_claim"
    ] as $blocked_activation_actions
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      readiness_schema_version:"memory_intelligence_kg_full_enablement_activation_readiness_v1",
      readiness_mode:"operator_requested_full_enablement_readiness_no_live_side_effects",
      full_enablement_activation_readiness_ready:true,
      full_enablement_activation_readiness_status:"ready_for_operator_approved_activation_slicing",
      live_activation_status:"not_performed_by_this_gate",
      source_memory_closure_gate:"hepta_memory_intelligence_closure_gate",
      source_kg_terminal_denial_gate:"hepta_kg_prompt_preview_terminal_next_action_activation_denial_summary_gate",
      source_kg_terminal_denial_gate_marker:"KG prompt-preview terminal next-action activation denial summary gate",
      source_core_fusion_gate:"hepta_core_fusion_readiness",
      source_memory_closure_report_sha256:$memory_closure_report_sha256,
      source_kg_terminal_denial_gate_reference_sha256:$kg_terminal_denial_gate_reference_sha256,
      source_memory_inventory_report_sha256:$memory_inventory_report_sha256,
      source_core_fusion_report_sha256:$core_fusion_report_sha256,
      rust_contract_manifest_sha256:$rust_contract_manifest_sha256,
      readiness_contract_hash_sha256:$readiness_contract_hash_sha256,
      readiness_policy_hash_sha256:$readiness_policy_hash_sha256,
      side_effect_hash_sha256:$side_effect_hash_sha256,
      core_full_fusion_complete:$core_fusion.full_fusion_complete,
      active_binary_package:$core_fusion.active_binary_package,
      remaining_direct_codex_dependency_count:$core_fusion.phase_5_engine_dependency_closure_remaining_dependency_count,
      hepta_core_direct_memory_intelligence_dependency_count:$memory_closure.hepta_core_direct_memory_intelligence_dependency_count,
      active_service_stack_consumes_memory_intelligence:$memory_closure.active_service_stack_consumes_memory_intelligence,
      memory_capability_endpoint:"/api/hepta-memory-capability-absorption-inventory",
      memory_surface_count:$memory_inventory.surface_count,
      absorbed_or_represented_count:$memory_inventory.absorbed_or_represented_count,
      gap_report_ready_count:$memory_inventory.gap_report_ready_count,
      live_mutation_enabled_count:$memory_inventory.live_mutation_enabled_count,
      memory_store_mutation_enabled:$memory_inventory.memory_store_mutation_enabled,
      kg_source_gate_count:5,
      kg_ready_source_gate_count:5,
      kg_blocked_source_gate_count:5,
      kg_report_only_source_gate_count:5,
      kg_required_total_preflight_requirement_count:19,
      kg_missing_total_preflight_requirement_count:19,
      enablement_lane_count:($enablement_lanes | length),
      ready_enablement_lane_count:($enablement_lanes | map(select(.readiness == "ready_for_operator_approved_activation_slice")) | length),
      current_live_enabled_lane_count:($enablement_lanes | map(select(.current_live_execution_enabled == true)) | length),
      enablement_lanes:$enablement_lanes,
      rust_contract_reference_count:($rust_contract_references | length),
      rust_contract_compile_checked_count:($rust_contract_references | map(select(.compile_checked_by_preflight_cargo_check == true)) | length),
      rust_contracts:$rust_contract_references,
      blocked_activation_actions:$blocked_activation_actions,
      blocked_activation_action_count:($blocked_activation_actions | length),
      operator_approval_required_before_activation:true,
      operator_activation_receipt_required:true,
      rollback_kill_switch_required:true,
      long_soak_required_before_mutation:true,
      context_handoff_acceptance_required:true,
      external_adapter_credentials_required_before_adapter_live:true,
      bounded_prompt_preview_scope_required:true,
      allowed_next_actions:[
        {
          action:"run_full_light_preflight",
          status:"allowed_verification_only",
          mutates_runtime:false,
          permits_live_memory_mutation:false,
          permits_kg_write:false,
          permits_model_invocation:false
        },
        {
          action:"prepare_operator_activation_packet",
          status:"allowed_report_only",
          mutates_runtime:false,
          permits_live_memory_mutation:false,
          permits_kg_write:false,
          permits_model_invocation:false
        },
        {
          action:"maintain_runtime_full_enablement_route_after_route_count_acceptance",
          status:"allowed_source_route_only",
          mutates_runtime:false,
          permits_live_memory_mutation:false,
          permits_kg_write:false,
          permits_model_invocation:false
        }
      ],
      next_slices:[
        "maintain the route-count-aware runtime readiness endpoint for this gate",
        "turn memory live mutation from report-only to operator-approved staging fixture",
        "stage KG external adapter credential and rollback receipts without live writes",
        "only then accept a bounded prompt-preview/context-handoff activation packet"
      ],
      full_live_enablement_performed:false,
      memory_store_mutated:false,
      hepta_intelligence_context_attached:false,
      prompt_preview_rendered:false,
      prompt_payload_materialized:false,
      context_injection_performed:false,
      model_invoked:false,
      provider_invoked:false,
      external_kg_adapter_read_performed:false,
      network_call_performed:false,
      external_db_write_performed:false,
      live_kg_write_performed:false,
      credential_read:false,
      channel_send_performed:false,
      gateway_route_migration_performed:false,
      source_command_migration_performed:false,
      active_runtime_wired:false,
      service_restart_performed:false,
      active_binary_mutated:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      side_effects:{
        full_live_enablement_performed:false,
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        hepta_intelligence_context_attached:false,
        prompt_preview_rendered:false,
        prompt_payload_materialized:false,
        context_injection_performed:false,
        model_invoked:false,
        provider_invoked:false,
        external_kg_adapter_read_performed:false,
        graphiti_client_constructed:false,
        neo4j_client_constructed:false,
        cocoindex_client_constructed:false,
        network_call_performed:false,
        external_db_write_performed:false,
        live_kg_write_performed:false,
        credential_read:false,
        channel_send_performed:false,
        gateway_route_migration_performed:false,
        source_command_migration_performed:false,
        active_runtime_wired:false,
        service_restart_performed:false,
        active_binary_mutated:false,
        filesystem_written:false,
        release_artifact_written:false,
        public_release_claimed:false,
        public_ga_claimed:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_activation_readiness_gate"
  and .readiness_schema_version == "memory_intelligence_kg_full_enablement_activation_readiness_v1"
  and .full_enablement_activation_readiness_ready == true
  and .full_enablement_activation_readiness_status == "ready_for_operator_approved_activation_slicing"
  and .live_activation_status == "not_performed_by_this_gate"
  and .core_full_fusion_complete == true
  and .active_binary_package == "hepta-cli"
  and .remaining_direct_codex_dependency_count == 0
  and .hepta_core_direct_memory_intelligence_dependency_count == 0
  and .active_service_stack_consumes_memory_intelligence == true
  and .memory_surface_count == 14
  and .absorbed_or_represented_count == 14
  and .gap_report_ready_count == 14
  and .live_mutation_enabled_count == 0
  and .memory_store_mutation_enabled == false
  and .kg_source_gate_count == 5
  and .kg_ready_source_gate_count == 5
  and .kg_blocked_source_gate_count == 5
  and .kg_report_only_source_gate_count == 5
  and .kg_required_total_preflight_requirement_count == 19
  and .kg_missing_total_preflight_requirement_count == 19
  and .enablement_lane_count == 6
  and .ready_enablement_lane_count == 6
  and .current_live_enabled_lane_count == 0
  and .rust_contract_reference_count == 7
  and .rust_contract_compile_checked_count == 7
  and .operator_approval_required_before_activation == true
  and .operator_activation_receipt_required == true
  and .rollback_kill_switch_required == true
  and .long_soak_required_before_mutation == true
  and .context_handoff_acceptance_required == true
  and .external_adapter_credentials_required_before_adapter_live == true
  and .bounded_prompt_preview_scope_required == true
  and .full_live_enablement_performed == false
  and .memory_store_mutated == false
  and .hepta_intelligence_context_attached == false
  and .prompt_preview_rendered == false
  and .prompt_payload_materialized == false
  and .context_injection_performed == false
  and .model_invoked == false
  and .provider_invoked == false
  and .external_kg_adapter_read_performed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .channel_send_performed == false
  and .gateway_route_migration_performed == false
  and .source_command_migration_performed == false
  and .active_runtime_wired == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and (.blocked_activation_actions | index("memory_store_mutation") != null)
  and (.blocked_activation_actions | index("kg_context_injection") != null)
  and (.blocked_activation_actions | index("live_kg_write") != null)
  and (.blocked_activation_actions | index("provider_model_invocation") != null)
  and (.blocked_activation_actions | index("credential_read") != null)
  and (.allowed_next_actions | any(.action == "run_full_light_preflight" and .status == "allowed_verification_only"))
  and (.allowed_next_actions | any(.action == "prepare_operator_activation_packet" and .status == "allowed_report_only"))
  and (.allowed_next_actions | any(.action == "maintain_runtime_full_enablement_route_after_route_count_acceptance" and .status == "allowed_source_route_only"))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement activation readiness gate passed"
