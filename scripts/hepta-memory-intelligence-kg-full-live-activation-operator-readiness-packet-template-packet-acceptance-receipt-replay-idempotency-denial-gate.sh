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

RECEIPT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence-gate.sh
)"

receipt_report_sha256="$(sha256_text "$RECEIPT_JSON")"
replay_idempotency_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial:$receipt_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$RECEIPT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_ready == true
    and $source.source_packet_assembly_ready == true
    and $source.source_packet_assembly_attempt_count == 4
    and $source.source_packet_assembled_count == 0
    and $source.source_packet_accepted_count == 0
    and $source.source_packet_activation_authority_derived_count == 0
    and $source.receipt_surface_count == 8
    and $source.receipt_generated_count == 8
    and $source.receipt_recorded_count == 0
    and $source.receipt_persisted_count == 0
    and $source.receipt_materialized_count == 0
    and $source.receipt_indexed_count == 0
    and $source.receipt_queryable_count == 0
    and $source.receipt_exportable_count == 0
    and $source.receipt_observable_count == 0
    and $source.receipt_delivered_count == 0
    and $source.receipt_acceptance_recorded_count == 0
    and $source.receipt_operator_approval_derived_count == 0
    and $source.receipt_activation_authority_derived_count == 0
    and $source.receipt_activation_command_derived_count == 0
    and $source.receipt_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

replay_surfaces_json="$(
  jq -n '[
    "packet_receipt_replay",
    "packet_receipt_idempotency_key_registration",
    "packet_receipt_idempotency_cache_write",
    "packet_receipt_cache_hit_promotion",
    "packet_receipt_query_result_replay",
    "packet_receipt_export_snapshot_replay",
    "packet_receipt_observability_snapshot_replay",
    "packet_receipt_operator_summary_replay",
    "packet_receipt_completion_ack_replay",
    "packet_receipt_authority_replay"
  ] | map({
    replay_surface: .,
    replay_attempted: true,
    replay_recorded: false,
    replay_persisted: false,
    idempotency_key_registered: false,
    idempotency_cache_written: false,
    cache_hit_promoted: false,
    query_result_registered: false,
    export_snapshot_recorded: false,
    observability_snapshot_recorded: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    replay_status: "replay_idempotency_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_gate" \
  --arg receipt_report_sha256 "$receipt_report_sha256" \
  --arg replay_idempotency_contract_hash_sha256 "$replay_idempotency_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RECEIPT_JSON" \
  --argjson replay_surfaces "$replay_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_replay_idempotency_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_v1",
    receipt_replay_idempotency_mode:"non_persistent_receipts_cannot_replay_cache_or_derive_authority",
    source_packet_acceptance_receipt_gate:$source.gate,
    source_packet_acceptance_receipt_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_ready,
    source_receipt_report_sha256:$receipt_report_sha256,
    replay_idempotency_contract_hash_sha256:$replay_idempotency_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_ready:true,
    source_receipt_surface_count:$source.receipt_surface_count,
    source_receipt_generated_count:$source.receipt_generated_count,
    source_receipt_recorded_count:$source.receipt_recorded_count,
    source_receipt_persisted_count:$source.receipt_persisted_count,
    source_receipt_acceptance_recorded_count:$source.receipt_acceptance_recorded_count,
    source_receipt_activation_authority_derived_count:$source.receipt_activation_authority_derived_count,
    replay_surface_count:($replay_surfaces | length),
    replay_attempt_count:($replay_surfaces | length),
    replay_recorded_count:0,
    replay_persisted_count:0,
    idempotency_key_registered_count:0,
    idempotency_cache_written_count:0,
    cache_hit_promoted_count:0,
    query_result_registered_count:0,
    export_snapshot_recorded_count:0,
    observability_snapshot_recorded_count:0,
    replay_acceptance_recorded_count:0,
    replay_operator_approval_derived_count:0,
    replay_activation_authority_derived_count:0,
    replay_activation_command_derived_count:0,
    replay_live_execution_allowed_count:0,
    replay_surfaces:$replay_surfaces,
    denied_by_packet_receipt_replay_idempotency:[
      "operator_readiness_packet_template_packet_receipt_replay_recording_denied",
      "operator_readiness_packet_template_packet_receipt_replay_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_idempotency_key_registration_denied",
      "operator_readiness_packet_template_packet_receipt_idempotency_cache_write_denied",
      "operator_readiness_packet_template_packet_receipt_cache_hit_promotion_denied",
      "operator_readiness_packet_template_packet_receipt_query_result_registration_denied",
      "operator_readiness_packet_template_packet_receipt_export_snapshot_denied",
      "operator_readiness_packet_template_packet_receipt_observability_snapshot_denied",
      "operator_readiness_packet_template_packet_receipt_acceptance_replay_denied",
      "operator_readiness_packet_template_packet_receipt_authority_replay_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_gate",
        status:"allowed_report_only_next_slice",
        persists_receipt:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_template_recorded:false,
    packet_template_persisted:false,
    packet_assembly_performed:false,
    packet_assembly_recorded:false,
    packet_assembly_persisted:false,
    packet_complete:false,
    packet_ready:false,
    packet_accepted:false,
    packet_acceptance_receipt_recorded:false,
    packet_acceptance_receipt_persisted:false,
    packet_acceptance_receipt_materialized:false,
    packet_acceptance_receipt_indexed:false,
    packet_acceptance_receipt_delivered:false,
    packet_acceptance_receipt_replayed:false,
    packet_acceptance_receipt_idempotency_key_registered:false,
    packet_acceptance_receipt_idempotency_cache_written:false,
    packet_acceptance_receipt_cache_hit_promoted:false,
    packet_acceptance_receipt_query_result_registered:false,
    packet_acceptance_receipt_export_snapshot_recorded:false,
    packet_acceptance_receipt_observability_snapshot_recorded:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    activation_authority_derived:false,
    activation_command_derived:false,
    activation_allowed:false,
    activation_performed:false,
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
      packet_acceptance_receipt_replayed:false,
      packet_acceptance_receipt_replay_recorded:false,
      packet_acceptance_receipt_replay_persisted:false,
      packet_acceptance_receipt_idempotency_key_registered:false,
      packet_acceptance_receipt_idempotency_cache_written:false,
      packet_acceptance_receipt_cache_hit_promoted:false,
      packet_acceptance_receipt_query_result_registered:false,
      packet_acceptance_receipt_export_snapshot_recorded:false,
      packet_acceptance_receipt_observability_snapshot_recorded:false,
      packet_acceptance_receipt_acceptance_recorded:false,
      packet_acceptance_receipt_authority_derived:false,
      packet_acceptance_receipt_live_execution_allowed:false,
      packet_acceptance_receipt_recorded:false,
      packet_acceptance_receipt_persisted:false,
      packet_acceptance_receipt_materialized:false,
      packet_acceptance_receipt_indexed:false,
      packet_acceptance_receipt_delivered:false,
      packet_template_recorded:false,
      packet_template_persisted:false,
      packet_assembly_performed:false,
      packet_assembly_recorded:false,
      packet_assembly_persisted:false,
      packet_ready_promoted:false,
      packet_acceptance_recorded:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      activation_authority_derived:false,
      activation_command_derived:false,
      activation_allowed:false,
      activation_performed:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_ready == true
  and .source_packet_acceptance_receipt_ready == true
  and .source_receipt_surface_count == 8
  and .source_receipt_generated_count == 8
  and .source_receipt_recorded_count == 0
  and .source_receipt_persisted_count == 0
  and .source_receipt_acceptance_recorded_count == 0
  and .source_receipt_activation_authority_derived_count == 0
  and .replay_surface_count == 10
  and .replay_attempt_count == 10
  and .replay_recorded_count == 0
  and .replay_persisted_count == 0
  and .idempotency_key_registered_count == 0
  and .idempotency_cache_written_count == 0
  and .cache_hit_promoted_count == 0
  and .query_result_registered_count == 0
  and .export_snapshot_recorded_count == 0
  and .observability_snapshot_recorded_count == 0
  and .replay_acceptance_recorded_count == 0
  and .replay_operator_approval_derived_count == 0
  and .replay_activation_authority_derived_count == 0
  and .replay_activation_command_derived_count == 0
  and .replay_live_execution_allowed_count == 0
  and (.replay_surfaces | all(
    .replay_attempted == true
    and .replay_recorded == false
    and .replay_persisted == false
    and .idempotency_key_registered == false
    and .idempotency_cache_written == false
    and .cache_hit_promoted == false
    and .query_result_registered == false
    and .export_snapshot_recorded == false
    and .observability_snapshot_recorded == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .replay_status == "replay_idempotency_denied"
  ))
  and (.denied_by_packet_receipt_replay_idempotency | length) == 11
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_template_recorded == false
  and .packet_template_persisted == false
  and .packet_assembly_performed == false
  and .packet_assembly_recorded == false
  and .packet_assembly_persisted == false
  and .packet_complete == false
  and .packet_ready == false
  and .packet_accepted == false
  and .packet_acceptance_receipt_recorded == false
  and .packet_acceptance_receipt_persisted == false
  and .packet_acceptance_receipt_replayed == false
  and .packet_acceptance_receipt_idempotency_key_registered == false
  and .packet_acceptance_receipt_idempotency_cache_written == false
  and .packet_acceptance_receipt_cache_hit_promoted == false
  and .packet_acceptance_receipt_query_result_registered == false
  and .packet_acceptance_receipt_export_snapshot_recorded == false
  and .packet_acceptance_receipt_observability_snapshot_recorded == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt replay/idempotency denial gate passed"
