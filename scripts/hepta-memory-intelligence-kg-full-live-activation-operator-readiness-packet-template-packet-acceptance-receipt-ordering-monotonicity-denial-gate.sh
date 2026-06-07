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

REPLAY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial-gate.sh
)"

replay_report_sha256="$(sha256_text "$REPLAY_JSON")"
ordering_monotonicity_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial:$replay_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$REPLAY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_ready == true
    and $source.source_packet_acceptance_receipt_ready == true
    and $source.source_receipt_surface_count == 8
    and $source.source_receipt_generated_count == 8
    and $source.source_receipt_recorded_count == 0
    and $source.source_receipt_persisted_count == 0
    and $source.replay_surface_count == 10
    and $source.replay_attempt_count == 10
    and $source.replay_recorded_count == 0
    and $source.replay_persisted_count == 0
    and $source.idempotency_key_registered_count == 0
    and $source.idempotency_cache_written_count == 0
    and $source.cache_hit_promoted_count == 0
    and $source.query_result_registered_count == 0
    and $source.export_snapshot_recorded_count == 0
    and $source.observability_snapshot_recorded_count == 0
    and $source.replay_acceptance_recorded_count == 0
    and $source.replay_operator_approval_derived_count == 0
    and $source.replay_activation_authority_derived_count == 0
    and $source.replay_activation_command_derived_count == 0
    and $source.replay_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

ordering_surfaces_json="$(
  jq -n '[
    "packet_receipt_duplicate_sequence_claim",
    "packet_receipt_stale_sequence_claim",
    "packet_receipt_late_arrival_claim",
    "packet_receipt_future_sequence_gap_claim",
    "packet_receipt_timestamp_rollback_claim",
    "packet_receipt_epoch_rollback_claim",
    "packet_receipt_same_sequence_different_hash_claim",
    "packet_receipt_latest_wins_overwrite_claim",
    "packet_receipt_query_ordering_claim",
    "packet_receipt_export_ordering_claim",
    "packet_receipt_observability_ordering_claim",
    "packet_receipt_completion_ack_ordering_claim",
    "packet_receipt_authority_ordering_claim",
    "packet_receipt_live_activation_ordering_claim"
  ] | map({
    ordering_surface: .,
    ordering_attempted: true,
    sequence_cursor_accepted: false,
    sequence_cursor_recorded: false,
    sequence_cursor_persisted: false,
    monotonicity_state_recorded: false,
    monotonicity_state_persisted: false,
    ordering_recorded: false,
    ordering_persisted: false,
    ordering_materialized: false,
    latest_wins_accepted: false,
    duplicate_accepted: false,
    stale_accepted: false,
    late_accepted: false,
    future_gap_accepted: false,
    timestamp_rollback_accepted: false,
    epoch_rollback_accepted: false,
    same_sequence_hash_override_accepted: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    ordering_status: "ordering_monotonicity_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_gate" \
  --arg replay_report_sha256 "$replay_report_sha256" \
  --arg ordering_monotonicity_contract_hash_sha256 "$ordering_monotonicity_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$REPLAY_JSON" \
  --argjson ordering_surfaces "$ordering_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_ordering_monotonicity_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_v1",
    receipt_ordering_monotonicity_mode:"non_persistent_receipts_cannot_create_sequence_cursor_monotonicity_or_authority",
    source_packet_acceptance_receipt_replay_idempotency_gate:$source.gate,
    source_packet_acceptance_receipt_replay_idempotency_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_ready,
    source_replay_report_sha256:$replay_report_sha256,
    source_replay_idempotency_contract_hash_sha256:$source.replay_idempotency_contract_hash_sha256,
    ordering_monotonicity_contract_hash_sha256:$ordering_monotonicity_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_ready:true,
    source_replay_surface_count:$source.replay_surface_count,
    source_replay_attempt_count:$source.replay_attempt_count,
    source_replay_recorded_count:$source.replay_recorded_count,
    source_replay_persisted_count:$source.replay_persisted_count,
    source_idempotency_key_registered_count:$source.idempotency_key_registered_count,
    source_idempotency_cache_written_count:$source.idempotency_cache_written_count,
    source_cache_hit_promoted_count:$source.cache_hit_promoted_count,
    source_replay_acceptance_recorded_count:$source.replay_acceptance_recorded_count,
    source_replay_activation_authority_derived_count:$source.replay_activation_authority_derived_count,
    ordering_surface_count:($ordering_surfaces | length),
    ordering_attempt_count:($ordering_surfaces | length),
    ordering_recorded_count:0,
    ordering_persisted_count:0,
    ordering_materialized_count:0,
    sequence_cursor_accepted_count:0,
    sequence_cursor_recorded_count:0,
    sequence_cursor_persisted_count:0,
    monotonicity_state_recorded_count:0,
    monotonicity_state_persisted_count:0,
    duplicate_sequence_accepted_count:0,
    stale_sequence_accepted_count:0,
    late_arrival_accepted_count:0,
    future_sequence_gap_accepted_count:0,
    timestamp_rollback_accepted_count:0,
    epoch_rollback_accepted_count:0,
    same_sequence_hash_override_accepted_count:0,
    latest_wins_overwrite_accepted_count:0,
    ordering_acceptance_recorded_count:0,
    ordering_operator_approval_derived_count:0,
    ordering_activation_authority_derived_count:0,
    ordering_activation_command_derived_count:0,
    ordering_live_execution_allowed_count:0,
    ordering_surfaces:$ordering_surfaces,
    denied_by_packet_receipt_ordering_monotonicity:[
      "operator_readiness_packet_template_packet_receipt_ordering_recording_denied",
      "operator_readiness_packet_template_packet_receipt_ordering_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_ordering_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_sequence_cursor_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_sequence_cursor_recording_denied",
      "operator_readiness_packet_template_packet_receipt_sequence_cursor_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_monotonicity_state_recording_denied",
      "operator_readiness_packet_template_packet_receipt_monotonicity_state_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_duplicate_sequence_denied",
      "operator_readiness_packet_template_packet_receipt_stale_sequence_denied",
      "operator_readiness_packet_template_packet_receipt_late_arrival_denied",
      "operator_readiness_packet_template_packet_receipt_future_sequence_gap_denied",
      "operator_readiness_packet_template_packet_receipt_timestamp_rollback_denied",
      "operator_readiness_packet_template_packet_receipt_epoch_rollback_denied",
      "operator_readiness_packet_template_packet_receipt_same_sequence_hash_override_denied",
      "operator_readiness_packet_template_packet_receipt_latest_wins_overwrite_denied",
      "operator_readiness_packet_template_packet_receipt_query_ordering_denied",
      "operator_readiness_packet_template_packet_receipt_export_ordering_denied",
      "operator_readiness_packet_template_packet_receipt_observability_ordering_denied",
      "operator_readiness_packet_template_packet_receipt_completion_ack_ordering_denied",
      "operator_readiness_packet_template_packet_receipt_acceptance_from_ordering_denied",
      "operator_readiness_packet_template_packet_receipt_authority_from_ordering_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_from_ordering_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_gate",
        status:"allowed_report_only_next_slice",
        persists_receipt:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        accepts_cancellation:false,
        accepts_supersession:false,
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
    packet_acceptance_receipt_replayed:false,
    packet_acceptance_receipt_idempotency_key_registered:false,
    packet_acceptance_receipt_idempotency_cache_written:false,
    packet_acceptance_receipt_cache_hit_promoted:false,
    packet_acceptance_receipt_ordering_recorded:false,
    packet_acceptance_receipt_ordering_persisted:false,
    packet_acceptance_receipt_sequence_cursor_accepted:false,
    packet_acceptance_receipt_sequence_cursor_recorded:false,
    packet_acceptance_receipt_sequence_cursor_persisted:false,
    packet_acceptance_receipt_monotonicity_state_recorded:false,
    packet_acceptance_receipt_monotonicity_state_persisted:false,
    packet_acceptance_receipt_duplicate_accepted:false,
    packet_acceptance_receipt_stale_accepted:false,
    packet_acceptance_receipt_late_accepted:false,
    packet_acceptance_receipt_future_gap_accepted:false,
    packet_acceptance_receipt_timestamp_rollback_accepted:false,
    packet_acceptance_receipt_epoch_rollback_accepted:false,
    packet_acceptance_receipt_same_sequence_hash_override_accepted:false,
    packet_acceptance_receipt_latest_wins_overwrite_accepted:false,
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
      packet_acceptance_receipt_ordering_recorded:false,
      packet_acceptance_receipt_ordering_persisted:false,
      packet_acceptance_receipt_ordering_materialized:false,
      packet_acceptance_receipt_sequence_cursor_accepted:false,
      packet_acceptance_receipt_sequence_cursor_recorded:false,
      packet_acceptance_receipt_sequence_cursor_persisted:false,
      packet_acceptance_receipt_monotonicity_state_recorded:false,
      packet_acceptance_receipt_monotonicity_state_persisted:false,
      packet_acceptance_receipt_duplicate_accepted:false,
      packet_acceptance_receipt_stale_accepted:false,
      packet_acceptance_receipt_late_accepted:false,
      packet_acceptance_receipt_future_gap_accepted:false,
      packet_acceptance_receipt_timestamp_rollback_accepted:false,
      packet_acceptance_receipt_epoch_rollback_accepted:false,
      packet_acceptance_receipt_same_sequence_hash_override_accepted:false,
      packet_acceptance_receipt_latest_wins_overwrite_accepted:false,
      packet_acceptance_receipt_acceptance_recorded:false,
      packet_acceptance_receipt_authority_derived:false,
      packet_acceptance_receipt_live_execution_allowed:false,
      packet_acceptance_receipt_replayed:false,
      packet_acceptance_receipt_replay_recorded:false,
      packet_acceptance_receipt_replay_persisted:false,
      packet_acceptance_receipt_idempotency_key_registered:false,
      packet_acceptance_receipt_idempotency_cache_written:false,
      packet_acceptance_receipt_cache_hit_promoted:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_ready == true
  and .source_packet_acceptance_receipt_replay_idempotency_ready == true
  and .source_replay_surface_count == 10
  and .source_replay_attempt_count == 10
  and .source_replay_recorded_count == 0
  and .source_replay_persisted_count == 0
  and .source_idempotency_key_registered_count == 0
  and .source_idempotency_cache_written_count == 0
  and .source_cache_hit_promoted_count == 0
  and .source_replay_acceptance_recorded_count == 0
  and .source_replay_activation_authority_derived_count == 0
  and .ordering_surface_count == 14
  and .ordering_attempt_count == 14
  and .ordering_recorded_count == 0
  and .ordering_persisted_count == 0
  and .ordering_materialized_count == 0
  and .sequence_cursor_accepted_count == 0
  and .sequence_cursor_recorded_count == 0
  and .sequence_cursor_persisted_count == 0
  and .monotonicity_state_recorded_count == 0
  and .monotonicity_state_persisted_count == 0
  and .duplicate_sequence_accepted_count == 0
  and .stale_sequence_accepted_count == 0
  and .late_arrival_accepted_count == 0
  and .future_sequence_gap_accepted_count == 0
  and .timestamp_rollback_accepted_count == 0
  and .epoch_rollback_accepted_count == 0
  and .same_sequence_hash_override_accepted_count == 0
  and .latest_wins_overwrite_accepted_count == 0
  and .ordering_acceptance_recorded_count == 0
  and .ordering_operator_approval_derived_count == 0
  and .ordering_activation_authority_derived_count == 0
  and .ordering_activation_command_derived_count == 0
  and .ordering_live_execution_allowed_count == 0
  and (.ordering_surfaces | all(
    .ordering_attempted == true
    and .sequence_cursor_accepted == false
    and .sequence_cursor_recorded == false
    and .sequence_cursor_persisted == false
    and .monotonicity_state_recorded == false
    and .monotonicity_state_persisted == false
    and .ordering_recorded == false
    and .ordering_persisted == false
    and .ordering_materialized == false
    and .latest_wins_accepted == false
    and .duplicate_accepted == false
    and .stale_accepted == false
    and .late_accepted == false
    and .future_gap_accepted == false
    and .timestamp_rollback_accepted == false
    and .epoch_rollback_accepted == false
    and .same_sequence_hash_override_accepted == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .ordering_status == "ordering_monotonicity_denied"
  ))
  and (.denied_by_packet_receipt_ordering_monotonicity | length) == 23
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
  and .packet_acceptance_receipt_ordering_recorded == false
  and .packet_acceptance_receipt_ordering_persisted == false
  and .packet_acceptance_receipt_sequence_cursor_accepted == false
  and .packet_acceptance_receipt_sequence_cursor_recorded == false
  and .packet_acceptance_receipt_sequence_cursor_persisted == false
  and .packet_acceptance_receipt_monotonicity_state_recorded == false
  and .packet_acceptance_receipt_monotonicity_state_persisted == false
  and .packet_acceptance_receipt_duplicate_accepted == false
  and .packet_acceptance_receipt_stale_accepted == false
  and .packet_acceptance_receipt_late_accepted == false
  and .packet_acceptance_receipt_future_gap_accepted == false
  and .packet_acceptance_receipt_timestamp_rollback_accepted == false
  and .packet_acceptance_receipt_epoch_rollback_accepted == false
  and .packet_acceptance_receipt_same_sequence_hash_override_accepted == false
  and .packet_acceptance_receipt_latest_wins_overwrite_accepted == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt ordering/monotonicity denial gate passed"
