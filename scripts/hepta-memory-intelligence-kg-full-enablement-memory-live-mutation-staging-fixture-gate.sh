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

FULL_ENABLEMENT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-activation-readiness-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-activation-readiness-gate.sh
)"

WRITE_ENABLE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-gate.sh
)"

full_enablement_report_sha256="$(printf '%s' "$FULL_ENABLEMENT_JSON" | shasum -a 256 | awk '{print $1}')"
write_enable_fixture_report_sha256="$(printf '%s' "$WRITE_ENABLE_JSON" | shasum -a 256 | awk '{print $1}')"
staging_fixture_contract_hash_sha256="$(
  printf '%s' "hepta-full-enablement-memory-live-mutation-staging-fixture:$full_enablement_report_sha256:$write_enable_fixture_report_sha256:$MIN_LONG_SOAK_SAMPLES" \
    | shasum -a 256 | awk '{print $1}'
)"
side_effect_hash_sha256="$(
  printf '%s' "memory_store_mutated=false;write_enable_fixture_persisted=false;write_enable_fixture_materialized=false;operator_approval_recorded=false;live_mutation_execution_ready=false" \
    | shasum -a 256 | awk '{print $1}'
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson full "$FULL_ENABLEMENT_JSON" \
  --argjson fixture "$WRITE_ENABLE_JSON" \
  '
    $full.runtime == "hepta"
    and $full.status == "ready"
    and $full.gate == "hepta_memory_intelligence_kg_full_enablement_activation_readiness_gate"
    and $full.full_enablement_activation_readiness_ready == true
    and $full.full_enablement_activation_readiness_status == "ready_for_operator_approved_activation_slicing"
    and $full.live_activation_status == "not_performed_by_this_gate"
    and $full.memory_surface_count == 14
    and $full.absorbed_or_represented_count == 14
    and $full.live_mutation_enabled_count == 0
    and $full.memory_store_mutation_enabled == false
    and $full.enablement_lane_count == 6
    and $full.ready_enablement_lane_count == 6
    and $full.current_live_enabled_lane_count == 0
    and ($full.enablement_lanes | any(.lane == "memory_store_live_mutation" and .readiness == "ready_for_operator_approved_activation_slice" and .current_live_execution_enabled == false))
    and $full.operator_approval_required_before_activation == true
    and $full.long_soak_required_before_mutation == true
    and ($full.side_effects | to_entries | all(.value == false))
    and $fixture.runtime == "hepta"
    and $fixture.status == "ready"
    and $fixture.gate == "hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_gate"
    and $fixture.write_enable_fixture_mode == "memory_write_execution_write_enable_fixture_non_activation"
    and $fixture.memory_write_execution_write_enable_fixture_ready == true
    and $fixture.source_memory_write_execution_no_write_sink_contract_ready == true
    and $fixture.required_pre_execution_validation_check_count == 17
    and $fixture.accepted_pre_execution_validation_check_count == 0
    and $fixture.required_write_enable_surface_count == 10
    and $fixture.ready_write_enable_surface_count == 10
    and $fixture.side_effect_free_write_enable_surface_count == 10
    and $fixture.required_write_enable_fixture_count == 7
    and $fixture.write_enable_fixture_count == 7
    and $fixture.blocked_write_enable_fixture_count == 7
    and $fixture.allowed_write_enable_fixture_count == 0
    and $fixture.memory_write_execution_allowed_count == 0
    and $fixture.memory_write_execution_performed_count == 0
    and $fixture.memory_store_write_requested_fixture_count == 7
    and $fixture.memory_store_write_allowed_count == 0
    and $fixture.memory_store_write_performed_count == 0
    and $fixture.memory_store_mutation_allowed == false
    and $fixture.memory_store_mutated == false
    and $fixture.write_enable_fixture_recorded == false
    and $fixture.write_enable_fixture_persisted == false
    and $fixture.write_enable_fixture_materialized == false
    and $fixture.write_enable_fixture_filesystem_written == false
    and $fixture.operator_approval_recorded == false
    and $fixture.accepted_redaction_proof_count == 0
    and $fixture.memory_write_execution_ready == false
    and $fixture.live_mutation_execution_ready == false
    and $fixture.external_send_enabled == false
    and $fixture.public_claim_or_release_artifact_write_enabled == false
    and ($fixture.write_enable_fixtures | length) == 7
    and ($fixture.write_enable_fixtures | all(.write_enable_status == "blocked" and .execution_allowed == false and .execution_performed == false and .memory_store_write_allowed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .activation_allowed == false))
    and ($fixture.denied_by_write_enable_fixture | length) == 13
    and ($fixture.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_memory_live_mutation_staging_fixture_gate" \
  --arg full_enablement_report_sha256 "$full_enablement_report_sha256" \
  --arg write_enable_fixture_report_sha256 "$write_enable_fixture_report_sha256" \
  --arg staging_fixture_contract_hash_sha256 "$staging_fixture_contract_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson full "$FULL_ENABLEMENT_JSON" \
  --argjson fixture "$WRITE_ENABLE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    staging_fixture_schema_version:"memory_intelligence_kg_full_enablement_memory_live_mutation_staging_fixture_v1",
    staging_fixture_mode:"full_enablement_memory_store_live_mutation_staging_no_activation",
    source_full_enablement_activation_readiness_gate:$full.gate,
    source_memory_write_execution_write_enable_fixture_gate:$fixture.gate,
    source_full_enablement_report_sha256:$full_enablement_report_sha256,
    source_memory_write_execution_write_enable_fixture_report_sha256:$write_enable_fixture_report_sha256,
    staging_fixture_contract_hash_sha256:$staging_fixture_contract_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    full_enablement_activation_readiness_ready:$full.full_enablement_activation_readiness_ready,
    full_enablement_activation_readiness_status:$full.full_enablement_activation_readiness_status,
    source_live_activation_status:$full.live_activation_status,
    memory_store_live_mutation_lane_ready:($full.enablement_lanes | any(.lane == "memory_store_live_mutation" and .readiness == "ready_for_operator_approved_activation_slice")),
    memory_store_live_mutation_lane_current_live_execution_enabled:false,
    enablement_lane_count:$full.enablement_lane_count,
    ready_enablement_lane_count:$full.ready_enablement_lane_count,
    current_live_enabled_lane_count:$full.current_live_enabled_lane_count,
    memory_surface_count:$full.memory_surface_count,
    absorbed_or_represented_count:$full.absorbed_or_represented_count,
    live_mutation_enabled_count:$full.live_mutation_enabled_count,
    memory_store_mutation_enabled:$full.memory_store_mutation_enabled,
    operator_approval_required_before_activation:true,
    operator_activation_receipt_required:true,
    rollback_kill_switch_required:true,
    long_soak_required_before_mutation:true,
    full_enablement_memory_live_mutation_staging_fixture_ready:true,
    operator_approval_bound_staging_fixture_shape_ready:true,
    operator_approved_staging_fixture_recorded:false,
    operator_approved_staging_fixture_persisted:false,
    operator_approved_staging_fixture_accepted:false,
    staging_fixture_materialized:false,
    staging_fixture_filesystem_written:false,
    staging_fixture_live_activation_allowed:false,
    staging_fixture_live_activation_performed:false,
    required_pre_execution_validation_check_count:$fixture.required_pre_execution_validation_check_count,
    accepted_pre_execution_validation_check_count:$fixture.accepted_pre_execution_validation_check_count,
    required_write_enable_surface_count:$fixture.required_write_enable_surface_count,
    ready_write_enable_surface_count:$fixture.ready_write_enable_surface_count,
    side_effect_free_write_enable_surface_count:$fixture.side_effect_free_write_enable_surface_count,
    required_write_enable_fixture_count:$fixture.required_write_enable_fixture_count,
    write_enable_fixture_count:$fixture.write_enable_fixture_count,
    blocked_write_enable_fixture_count:$fixture.blocked_write_enable_fixture_count,
    allowed_write_enable_fixture_count:$fixture.allowed_write_enable_fixture_count,
    explicit_write_enable_requested_fixture_count:$fixture.explicit_write_enable_requested_fixture_count,
    memory_write_execution_denied_count:$fixture.memory_write_execution_denied_count,
    memory_write_execution_allowed_count:$fixture.memory_write_execution_allowed_count,
    memory_write_execution_performed_count:$fixture.memory_write_execution_performed_count,
    memory_store_write_requested_fixture_count:$fixture.memory_store_write_requested_fixture_count,
    memory_store_write_allowed_count:$fixture.memory_store_write_allowed_count,
    memory_store_write_performed_count:$fixture.memory_store_write_performed_count,
    memory_store_mutation_allowed:false,
    memory_store_mutated:false,
    memory_write_execution_ready:false,
    live_mutation_execution_ready:false,
    write_enable_surfaces:$fixture.write_enable_surfaces,
    write_enable_fixtures:$fixture.write_enable_fixtures,
    denied_by_staging_fixture:($fixture.denied_by_write_enable_fixture + [
      "full_enablement_memory_live_mutation_staging_fixture_not_recorded",
      "operator_approval_bound_staging_fixture_not_accepted",
      "staging_fixture_materialization_denied",
      "full_live_enablement_execution_denied"
    ]),
    allowed_next_actions:[
      {
        action:"review_memory_live_mutation_staging_fixture_shape",
        status:"allowed_report_only",
        mutates_memory_store:false,
        persists_fixture:false,
        activates_live_mutation:false
      },
      {
        action:"stage_kg_external_adapter_credentials_and_rollback_receipts",
        status:"allowed_report_only_next_slice",
        reads_credentials:false,
        invokes_external_adapter:false,
        writes_kg:false
      }
    ],
    full_live_enablement_performed:false,
    memory_store_write_path_enabled:false,
    memory_store_write_performed:false,
    hepta_intelligence_context_attached:false,
    prompt_preview_rendered:false,
    context_injection_performed:false,
    provider_invoked:false,
    model_invoked:false,
    external_kg_adapter_read_performed:false,
    live_kg_write_performed:false,
    credential_read:false,
    external_send_performed:false,
    channel_send_performed:false,
    public_release_claimed:false,
    public_ga_claimed:false,
    service_restart_performed:false,
    active_binary_mutated:false,
    side_effects:{
      full_live_enablement_performed:false,
      memory_store_mutated:false,
      memory_store_write_performed:false,
      memory_write_request_recorded:false,
      memory_write_request_persisted:false,
      memory_write_approval_packet_recorded:false,
      memory_write_approval_packet_persisted:false,
      memory_write_execution_preflight_recorded:false,
      memory_write_execution_preflight_persisted:false,
      memory_write_execution_write_enable_fixture_recorded:false,
      memory_write_execution_write_enable_fixture_persisted:false,
      memory_write_execution_write_enable_fixture_materialized:false,
      memory_write_execution_write_enable_fixture_filesystem_written:false,
      full_enablement_staging_fixture_recorded:false,
      full_enablement_staging_fixture_persisted:false,
      full_enablement_staging_fixture_materialized:false,
      full_enablement_staging_fixture_filesystem_written:false,
      explicit_write_enablement_recorded:false,
      explicit_write_enablement_persisted:false,
      operator_approval_recorded:false,
      pre_execution_validation_recorded:false,
      pre_execution_validation_persisted:false,
      payload_plaintext_persisted:false,
      raw_payload_inspected:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      hepta_intelligence_context_attached:false,
      prompt_preview_rendered:false,
      context_injection_performed:false,
      provider_invoked:false,
      model_invoked:false,
      provider_prompt_replayed:false,
      external_kg_adapter_read_performed:false,
      live_kg_write_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      filesystem_written:false,
      release_artifact_written:false,
      public_artifact_written:false,
      public_release_published:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      approval_record_persisted:false,
      receipt_persisted:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      rollback_executed:false,
      credential_read:false,
      secret_file_read:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_memory_live_mutation_staging_fixture_gate"
  and .staging_fixture_schema_version == "memory_intelligence_kg_full_enablement_memory_live_mutation_staging_fixture_v1"
  and .full_enablement_memory_live_mutation_staging_fixture_ready == true
  and .operator_approval_bound_staging_fixture_shape_ready == true
  and .memory_store_live_mutation_lane_ready == true
  and .memory_store_live_mutation_lane_current_live_execution_enabled == false
  and .enablement_lane_count == 6
  and .ready_enablement_lane_count == 6
  and .current_live_enabled_lane_count == 0
  and .live_mutation_enabled_count == 0
  and .memory_store_mutation_enabled == false
  and .operator_approval_required_before_activation == true
  and .operator_activation_receipt_required == true
  and .rollback_kill_switch_required == true
  and .long_soak_required_before_mutation == true
  and .operator_approved_staging_fixture_recorded == false
  and .operator_approved_staging_fixture_persisted == false
  and .operator_approved_staging_fixture_accepted == false
  and .staging_fixture_materialized == false
  and .staging_fixture_filesystem_written == false
  and .staging_fixture_live_activation_allowed == false
  and .staging_fixture_live_activation_performed == false
  and .required_pre_execution_validation_check_count == 17
  and .accepted_pre_execution_validation_check_count == 0
  and .required_write_enable_surface_count == 10
  and .ready_write_enable_surface_count == 10
  and .side_effect_free_write_enable_surface_count == 10
  and .required_write_enable_fixture_count == 7
  and .write_enable_fixture_count == 7
  and .blocked_write_enable_fixture_count == 7
  and .allowed_write_enable_fixture_count == 0
  and .memory_write_execution_denied_count == 7
  and .memory_write_execution_allowed_count == 0
  and .memory_write_execution_performed_count == 0
  and .memory_store_write_requested_fixture_count == 7
  and .memory_store_write_allowed_count == 0
  and .memory_store_write_performed_count == 0
  and .memory_store_mutation_allowed == false
  and .memory_store_mutated == false
  and .memory_write_execution_ready == false
  and .live_mutation_execution_ready == false
  and (.write_enable_fixtures | length) == 7
  and (.write_enable_fixtures | all(.write_enable_status == "blocked" and .execution_allowed == false and .execution_performed == false and .memory_store_write_allowed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .activation_allowed == false))
  and (.denied_by_staging_fixture | length) == 17
  and (.allowed_next_actions | any(.action == "review_memory_live_mutation_staging_fixture_shape" and .status == "allowed_report_only"))
  and (.allowed_next_actions | any(.action == "stage_kg_external_adapter_credentials_and_rollback_receipts" and .status == "allowed_report_only_next_slice"))
  and .full_live_enablement_performed == false
  and .memory_store_write_path_enabled == false
  and .memory_store_write_performed == false
  and .hepta_intelligence_context_attached == false
  and .prompt_preview_rendered == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_kg_adapter_read_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .external_send_performed == false
  and .channel_send_performed == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement memory live mutation staging fixture gate passed"
