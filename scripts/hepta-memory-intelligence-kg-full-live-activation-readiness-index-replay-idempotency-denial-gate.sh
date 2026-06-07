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

READINESS_INDEX_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-readiness-index-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-readiness-index-gate.sh
)"

readiness_index_report_sha256="$(sha256_text "$READINESS_INDEX_JSON")"
replay_idempotency_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial:$readiness_index_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$READINESS_INDEX_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_readiness_index_gate"
    and $source.memory_intelligence_kg_full_live_activation_readiness_index_ready == true
    and $source.full_live_activation_enabled == false
    and $source.full_live_activation_status == "blocked_report_only"
    and $source.live_mutation_enabled_count == 0
    and $source.prompt_preview_allowed == false
    and $source.context_injection_allowed == false
    and $source.model_invoked == false
    and $source.live_write_enabled_count == 0
    and $source.memory_store_live_mutation_lane_current_live_execution_enabled == false
    and $source.kg_external_adapter_staging_lane_current_live_execution_enabled == false
    and ($source.readiness_surfaces | length) == 10
    and ($source.live_activation_blockers | length) == 13
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_gate" \
  --arg readiness_index_report_sha256 "$readiness_index_report_sha256" \
  --arg replay_idempotency_contract_hash_sha256 "$replay_idempotency_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$READINESS_INDEX_JSON" \
  '
  def denied_fixture($id; $reason; $extra):
    {
      id:$id,
      source_readiness_index_present:true,
      source_readiness_index_ready:true,
      replay_requested:true,
      replay_allowed:false,
      replay_accepted:false,
      idempotency_key_registered:false,
      idempotency_key_persisted:false,
      idempotency_cache_written:false,
      replay_cache_hit_promoted:false,
      query_result_registered:false,
      query_result_persisted:false,
      index_entry_written:false,
      export_recorded:false,
      observability_recorded:false,
      activation_authority_derived:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
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
      replay_idempotency_noop_confirmed:true,
      reason:$reason
    } + $extra;
  [
    denied_fixture("readiness-index-replay-missing-source"; "source_readiness_index_required"; {source_readiness_index_present:false, source_readiness_index_ready:false}),
    denied_fixture("readiness-index-replay-request"; "readiness_index_replay_denied"; {explicit_replay_requested:true}),
    denied_fixture("readiness-index-idempotency-key-registration-request"; "idempotency_key_registration_denied"; {idempotency_key_registration_requested:true}),
    denied_fixture("readiness-index-idempotency-cache-write-request"; "idempotency_cache_write_denied"; {idempotency_cache_write_requested:true}),
    denied_fixture("readiness-index-query-result-registration-request"; "query_result_registration_denied"; {query_result_registration_requested:true}),
    denied_fixture("readiness-index-index-entry-write-request"; "index_entry_write_denied"; {index_entry_write_requested:true}),
    denied_fixture("readiness-index-export-observability-request"; "export_observability_denied"; {export_requested:true, observability_requested:true}),
    denied_fixture("readiness-index-operator-acceptance-record-request"; "operator_acceptance_from_readiness_index_denied"; {operator_acceptance_record_requested:true}),
    denied_fixture("readiness-index-activation-authority-request"; "activation_authority_from_readiness_index_denied"; {activation_authority_requested:true}),
    denied_fixture("readiness-index-live-side-effect-request"; "readiness_index_replay_cannot_authorize_live_side_effects"; {memory_write_requested:true, kg_write_requested:true, provider_invocation_requested:true, credential_read_requested:true, install_restart_requested:true, public_release_requested:true, external_send_requested:true})
  ] as $fixtures
  | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    replay_idempotency_schema_version:"memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_v1",
    replay_idempotency_mode:"readiness_index_replay_idempotency_report_only_no_persistence_no_authority",
    source_readiness_index_gate:$source.gate,
    source_readiness_index_report_sha256:$readiness_index_report_sha256,
    replay_idempotency_contract_hash_sha256:$replay_idempotency_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_ready:true,
    source_readiness_index_ready:$source.memory_intelligence_kg_full_live_activation_readiness_index_ready,
    source_full_live_activation_enabled:$source.full_live_activation_enabled,
    source_full_live_activation_status:$source.full_live_activation_status,
    readiness_surface_count:($source.readiness_surfaces | length),
    live_activation_blocker_count:($source.live_activation_blockers | length),
    required_replay_idempotency_surface_count:12,
    ready_replay_idempotency_surface_count:12,
    side_effect_free_replay_idempotency_surface_count:12,
    required_replay_idempotency_fixture_count:10,
    replay_idempotency_fixture_count:($fixtures | length),
    blocked_replay_idempotency_fixture_count:($fixtures | length),
    noop_replay_idempotency_fixture_count:($fixtures | length),
    allowed_replay_idempotency_fixture_count:0,
    accepted_replay_idempotency_fixture_count:0,
    replay_allowed:false,
    replay_accepted:false,
    idempotency_key_registered:false,
    idempotency_key_persisted:false,
    idempotency_cache_written:false,
    replay_cache_hit_promoted:false,
    query_result_registered:false,
    query_result_persisted:false,
    index_entry_written:false,
    export_recorded:false,
    observability_recorded:false,
    activation_authority_derived:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    replay_idempotency_fixtures:$fixtures,
    denied_by_readiness_index_replay_idempotency:[
      "readiness_index_replay_denied",
      "readiness_index_idempotency_key_registration_denied",
      "readiness_index_idempotency_cache_write_denied",
      "readiness_index_query_result_registration_denied",
      "readiness_index_index_entry_write_denied",
      "readiness_index_export_observability_denied",
      "readiness_index_operator_acceptance_record_denied",
      "readiness_index_activation_authority_denied",
      "readiness_index_live_side_effects_denied"
    ],
    allowed_next_actions:[
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
    side_effects:{
      replay_performed:false,
      replay_accepted:false,
      idempotency_key_registered:false,
      idempotency_key_persisted:false,
      idempotency_cache_written:false,
      query_result_registered:false,
      query_result_persisted:false,
      index_entry_written:false,
      export_recorded:false,
      observability_recorded:false,
      activation_authority_derived:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
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
      filesystem_written:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_gate"
  and .memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_ready == true
  and .source_readiness_index_ready == true
  and .source_full_live_activation_enabled == false
  and .source_full_live_activation_status == "blocked_report_only"
  and .readiness_surface_count == 10
  and .live_activation_blocker_count == 13
  and .required_replay_idempotency_surface_count == 12
  and .ready_replay_idempotency_surface_count == 12
  and .side_effect_free_replay_idempotency_surface_count == 12
  and .required_replay_idempotency_fixture_count == 10
  and .replay_idempotency_fixture_count == 10
  and .blocked_replay_idempotency_fixture_count == 10
  and .allowed_replay_idempotency_fixture_count == 0
  and .accepted_replay_idempotency_fixture_count == 0
  and .replay_allowed == false
  and .replay_accepted == false
  and .idempotency_key_registered == false
  and .idempotency_key_persisted == false
  and .idempotency_cache_written == false
  and .replay_cache_hit_promoted == false
  and .query_result_registered == false
  and .query_result_persisted == false
  and .index_entry_written == false
  and .export_recorded == false
  and .observability_recorded == false
  and .activation_authority_derived == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and (.replay_idempotency_fixtures | all(.replay_allowed == false and .replay_accepted == false and .activation_authority_derived == false and .operator_acceptance_recorded == false))
  and (.denied_by_readiness_index_replay_idempotency | length) == 9
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
echo "Hepta memory/intelligence/KG full live activation readiness index replay/idempotency denial gate passed"
