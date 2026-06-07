#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

MEMORY_CLOSURE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-memory-intelligence-closure" \
    scripts/hepta-memory-intelligence-closure.sh
)"

KG_PREFLIGHT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-kg-prompt-preview-preflight-gate" \
    scripts/hepta-kg-prompt-preview-preflight-gate.sh
)"

MEMORY_STAGING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-memory-live-mutation-staging-fixture-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-memory-live-mutation-staging-fixture-gate.sh
)"

KG_ADAPTER_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-kg-external-adapter-staging-receipt-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-kg-external-adapter-staging-receipt-gate.sh
)"

PUBLICATION_RECEIPT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence-gate.sh
)"

memory_closure_report_sha256="$(sha256_text "$MEMORY_CLOSURE_JSON")"
kg_preflight_report_sha256="$(sha256_text "$KG_PREFLIGHT_JSON")"
memory_staging_report_sha256="$(sha256_text "$MEMORY_STAGING_JSON")"
kg_adapter_report_sha256="$(sha256_text "$KG_ADAPTER_JSON")"
publication_receipt_report_sha256="$(sha256_text "$PUBLICATION_RECEIPT_JSON")"
readiness_index_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-readiness-index:$memory_closure_report_sha256:$kg_preflight_report_sha256:$memory_staging_report_sha256:$kg_adapter_report_sha256:$publication_receipt_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson memory "$MEMORY_CLOSURE_JSON" \
  --argjson kg "$KG_PREFLIGHT_JSON" \
  --argjson memory_staging "$MEMORY_STAGING_JSON" \
  --argjson adapter "$KG_ADAPTER_JSON" \
  --argjson publication "$PUBLICATION_RECEIPT_JSON" \
  '
    $memory.runtime == "hepta"
    and $memory.status == "attention"
    and $memory.active_service_stack_consumes_memory_intelligence == true
    and $memory.runtime_memory_intelligence_dependencies_ready == true
    and $memory.memory_surface_count == 14
    and $memory.absorbed_or_represented_count == 14
    and $memory.live_mutation_enabled_count == 0
    and $memory.full_live_memory_intelligence_closure_ready == false
    and ($memory.blocked_live_mutations | sort) == ([
      "capability_registry_mutation",
      "coding_agent_spawn",
      "memory_store_mutation",
      "plugin_registry_mutation",
      "search_provider_live_query",
      "skill_workshop_write"
    ] | sort)
    and ($memory.side_effects | to_entries | all(.value == false))
    and $kg.runtime == "hepta"
    and $kg.status == "ready"
    and $kg.gate == "hepta_kg_prompt_preview_preflight_gate"
    and $kg.preflight_report_status == "blocked"
    and $kg.prompt_preview_allowed == false
    and $kg.context_injection_allowed == false
    and $kg.model_invoked == false
    and $kg.live_write_enabled_count == 0
    and ($kg.side_effects | to_entries | all(.value == false))
    and $memory_staging.runtime == "hepta"
    and $memory_staging.status == "ready"
    and $memory_staging.full_enablement_memory_live_mutation_staging_fixture_ready == true
    and $memory_staging.memory_store_live_mutation_lane_ready == true
    and $memory_staging.memory_store_live_mutation_lane_current_live_execution_enabled == false
    and $memory_staging.memory_store_write_performed == false
    and $memory_staging.memory_store_mutated == false
    and $memory_staging.hepta_intelligence_context_attached == false
    and $memory_staging.context_injection_performed == false
    and $memory_staging.provider_invoked == false
    and $memory_staging.model_invoked == false
    and $memory_staging.live_kg_write_performed == false
    and $memory_staging.credential_read == false
    and ($memory_staging.side_effects | to_entries | all(.value == false))
    and $adapter.runtime == "hepta"
    and $adapter.status == "ready"
    and $adapter.kg_external_adapter_staging_lane_ready == true
    and $adapter.kg_external_adapter_staging_lane_current_live_execution_enabled == false
    and $adapter.external_adapter_client_constructed == false
    and $adapter.network_call_performed == false
    and $adapter.external_db_write_performed == false
    and $adapter.live_kg_write_performed == false
    and $adapter.credential_read == false
    and $adapter.secret_file_read == false
    and ($adapter.side_effects | to_entries | all(.value == false))
    and $publication.runtime == "hepta"
    and $publication.status == "ready"
    and $publication.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_ready == true
    and $publication.publication_result_receipt_persisted == false
    and $publication.public_release_published == false
    and $publication.public_ga_claimed == false
    and $publication.release_artifact_written == false
    and $publication.public_artifact_written == false
    and $publication.external_send_performed == false
    and $publication.provider_invoked == false
    and $publication.model_invoked == false
    and $publication.install_executed == false
    and $publication.service_restarted == false
    and $publication.active_binary_mutated == false
    and ($publication.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_readiness_index_gate" \
  --arg memory_closure_report_sha256 "$memory_closure_report_sha256" \
  --arg kg_preflight_report_sha256 "$kg_preflight_report_sha256" \
  --arg memory_staging_report_sha256 "$memory_staging_report_sha256" \
  --arg kg_adapter_report_sha256 "$kg_adapter_report_sha256" \
  --arg publication_receipt_report_sha256 "$publication_receipt_report_sha256" \
  --arg readiness_index_contract_hash_sha256 "$readiness_index_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson memory "$MEMORY_CLOSURE_JSON" \
  --argjson kg "$KG_PREFLIGHT_JSON" \
  --argjson memory_staging "$MEMORY_STAGING_JSON" \
  --argjson adapter "$KG_ADAPTER_JSON" \
  --argjson publication "$PUBLICATION_RECEIPT_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    readiness_index_schema_version:"memory_intelligence_kg_full_live_activation_readiness_index_v1",
    readiness_index_mode:"report_only_no_activation_no_secret_no_provider_no_write",
    memory_intelligence_kg_full_live_activation_readiness_index_ready:true,
    full_live_activation_enabled:false,
    full_live_activation_status:"blocked_report_only",
    minimum_required_samples:$min_long_soak_samples,
    readiness_index_contract_hash_sha256:$readiness_index_contract_hash_sha256,
    source_reports:{
      memory_intelligence_closure:{gate:"hepta_memory_intelligence_closure_gate", sha256:$memory_closure_report_sha256},
      kg_prompt_preview_preflight:{gate:$kg.gate, sha256:$kg_preflight_report_sha256},
      memory_live_mutation_staging:{gate:$memory_staging.gate, sha256:$memory_staging_report_sha256},
      kg_external_adapter_staging:{gate:$adapter.gate, sha256:$kg_adapter_report_sha256},
      operator_canary_publication_receipt:{gate:$publication.gate, sha256:$publication_receipt_report_sha256}
    },
    active_service_stack_consumes_memory_intelligence:$memory.active_service_stack_consumes_memory_intelligence,
    runtime_memory_intelligence_dependencies_ready:$memory.runtime_memory_intelligence_dependencies_ready,
    memory_capability_inventory_ready:true,
    memory_surface_count:$memory.memory_surface_count,
    absorbed_or_represented_count:$memory.absorbed_or_represented_count,
    gap_report_ready_count:$memory.gap_report_ready_count,
    live_mutation_enabled_count:$memory.live_mutation_enabled_count,
    full_live_memory_intelligence_closure_ready:$memory.full_live_memory_intelligence_closure_ready,
    kg_prompt_preview_preflight_ready:true,
    kg_prompt_preview_status:$kg.preflight_report_status,
    prompt_preview_allowed:false,
    context_injection_allowed:false,
    model_invoked:false,
    live_write_enabled_count:0,
    memory_store_live_mutation_lane_ready:true,
    memory_store_live_mutation_lane_current_live_execution_enabled:false,
    kg_external_adapter_staging_lane_ready:true,
    kg_external_adapter_staging_lane_current_live_execution_enabled:false,
    operator_canary_publication_result_receipt_no_persistence_ready:true,
    readiness_surfaces:[
      {surface:"core_runtime_dependency_attachment", ready:true, blocked:false, mode:"code_dependency_and_report_ready"},
      {surface:"memory_capability_absorption", ready:true, blocked:false, mode:"absorbed_or_represented_report_only"},
      {surface:"memory_live_mutation_execution", ready:true, blocked:true, reason:"memory_store_live_execution_disabled"},
      {surface:"kg_prompt_preview_context_injection", ready:true, blocked:true, reason:"prompt_preview_and_context_injection_disabled"},
      {surface:"kg_external_adapter_staging", ready:true, blocked:true, reason:"credential_read_network_and_external_write_disabled"},
      {surface:"operator_canary_activation_chain", ready:true, blocked:true, reason:"operator_canary_chain_remains_noop_report_only"},
      {surface:"publication_release_artifact_boundary", ready:true, blocked:true, reason:"release_artifact_publication_and_receipt_persistence_denied"},
      {surface:"provider_model_invocation_boundary", ready:true, blocked:true, reason:"provider_and_model_invocation_disabled"},
      {surface:"credential_secret_boundary", ready:true, blocked:true, reason:"credential_and_secret_read_disabled"},
      {surface:"install_restart_active_binary_boundary", ready:true, blocked:true, reason:"install_restart_active_binary_mutation_denied"}
    ],
    live_activation_blockers:[
      "memory_store_mutation_disabled",
      "context_injection_disabled",
      "prompt_preview_disabled",
      "kg_external_adapter_live_execution_disabled",
      "live_kg_write_disabled",
      "credential_secret_read_disabled",
      "provider_model_invocation_disabled",
      "operator_approval_packet_missing",
      "redaction_review_missing",
      "rollback_kill_switch_not_accepted_for_live",
      "post_write_validation_not_persisted",
      "idempotency_replay_ordering_not_live",
      "install_restart_active_binary_denied"
    ],
    allowed_next_actions:[
      {
        action:"stage_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial",
        status:"allowed_report_only_next_slice",
        mutates_memory_store:false,
        writes_kg:false,
        invokes_provider:false,
        persists_receipt:false
      },
      {
        action:"prepare_operator_activation_readiness_packet_template",
        status:"allowed_report_only_next_slice",
        records_operator_acceptance:false,
        activates_live:false,
        publishes_artifact:false
      }
    ],
    memory_store_write_performed:false,
    memory_store_mutated:false,
    hepta_intelligence_context_attached:false,
    prompt_preview_rendered:false,
    context_injection_performed:false,
    provider_invoked:false,
    external_kg_adapter_read_performed:false,
    external_adapter_client_constructed:false,
    network_call_performed:false,
    external_db_write_performed:false,
    live_kg_write_performed:false,
    credential_read:false,
    secret_file_read:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    public_release_claimed:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    public_artifact_written:false,
    external_send_performed:false,
    side_effects:{
      memory_store_write_performed:false,
      memory_store_mutated:false,
      hepta_intelligence_context_attached:false,
      prompt_preview_rendered:false,
      context_injection_performed:false,
      provider_invoked:false,
      model_invoked:false,
      external_kg_adapter_read_performed:false,
      external_adapter_client_constructed:false,
      network_call_performed:false,
      external_db_write_performed:false,
      live_kg_write_performed:false,
      credential_read:false,
      secret_file_read:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      external_send_performed:false,
      gateway_mutation_performed:false,
      filesystem_written:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_readiness_index_gate"
  and .memory_intelligence_kg_full_live_activation_readiness_index_ready == true
  and .full_live_activation_enabled == false
  and .full_live_activation_status == "blocked_report_only"
  and .active_service_stack_consumes_memory_intelligence == true
  and .runtime_memory_intelligence_dependencies_ready == true
  and .memory_capability_inventory_ready == true
  and .memory_surface_count == 14
  and .absorbed_or_represented_count == 14
  and .live_mutation_enabled_count == 0
  and .full_live_memory_intelligence_closure_ready == false
  and .kg_prompt_preview_preflight_ready == true
  and .kg_prompt_preview_status == "blocked"
  and .prompt_preview_allowed == false
  and .context_injection_allowed == false
  and .model_invoked == false
  and .live_write_enabled_count == 0
  and .memory_store_live_mutation_lane_ready == true
  and .memory_store_live_mutation_lane_current_live_execution_enabled == false
  and .kg_external_adapter_staging_lane_ready == true
  and .kg_external_adapter_staging_lane_current_live_execution_enabled == false
  and .operator_canary_publication_result_receipt_no_persistence_ready == true
  and (.readiness_surfaces | length) == 10
  and (.readiness_surfaces | any(.surface == "core_runtime_dependency_attachment" and .ready == true and .blocked == false))
  and (.readiness_surfaces | any(.surface == "memory_live_mutation_execution" and .ready == true and .blocked == true))
  and (.readiness_surfaces | any(.surface == "kg_external_adapter_staging" and .ready == true and .blocked == true))
  and (.readiness_surfaces | any(.surface == "provider_model_invocation_boundary" and .ready == true and .blocked == true))
  and (.readiness_surfaces | any(.surface == "credential_secret_boundary" and .ready == true and .blocked == true))
  and (.live_activation_blockers | length) == 13
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .hepta_intelligence_context_attached == false
  and .prompt_preview_rendered == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_kg_adapter_read_performed == false
  and .external_adapter_client_constructed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .secret_file_read == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full live activation readiness index gate passed"
