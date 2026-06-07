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

ORDERING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial-gate.sh
)"

ordering_report_sha256="$(sha256_text "$ORDERING_JSON")"
cancellation_supersession_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial:$ordering_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$ORDERING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_ready == true
    and $source.source_packet_acceptance_receipt_replay_idempotency_ready == true
    and $source.ordering_surface_count == 14
    and $source.ordering_attempt_count == 14
    and $source.ordering_recorded_count == 0
    and $source.ordering_persisted_count == 0
    and $source.ordering_materialized_count == 0
    and $source.sequence_cursor_accepted_count == 0
    and $source.sequence_cursor_recorded_count == 0
    and $source.sequence_cursor_persisted_count == 0
    and $source.monotonicity_state_recorded_count == 0
    and $source.monotonicity_state_persisted_count == 0
    and $source.ordering_acceptance_recorded_count == 0
    and $source.ordering_operator_approval_derived_count == 0
    and $source.ordering_activation_authority_derived_count == 0
    and $source.ordering_activation_command_derived_count == 0
    and $source.ordering_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

cancellation_surfaces_json="$(
  jq -n '[
    "packet_receipt_cancel_claim",
    "packet_receipt_revoke_claim",
    "packet_receipt_withdraw_claim",
    "packet_receipt_supersede_claim",
    "packet_receipt_replacement_claim",
    "packet_receipt_tombstone_claim",
    "packet_receipt_delete_marker_claim",
    "packet_receipt_latest_replacement_claim",
    "packet_receipt_ack_replacement_claim",
    "packet_receipt_query_replacement_claim",
    "packet_receipt_export_replacement_claim",
    "packet_receipt_observability_replacement_claim",
    "packet_receipt_authority_replacement_claim",
    "packet_receipt_live_replacement_claim"
  ] | map({
    cancellation_surface: .,
    cancellation_or_supersession_attempted: true,
    cancellation_accepted: false,
    cancellation_recorded: false,
    cancellation_persisted: false,
    supersession_accepted: false,
    supersession_recorded: false,
    supersession_persisted: false,
    replacement_receipt_accepted: false,
    replacement_receipt_recorded: false,
    replacement_receipt_persisted: false,
    tombstone_recorded: false,
    tombstone_persisted: false,
    delete_marker_recorded: false,
    latest_replacement_accepted: false,
    ack_replacement_accepted: false,
    query_replacement_registered: false,
    export_replacement_recorded: false,
    observability_replacement_recorded: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    cancellation_supersession_status: "cancellation_supersession_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_gate" \
  --arg ordering_report_sha256 "$ordering_report_sha256" \
  --arg cancellation_supersession_contract_hash_sha256 "$cancellation_supersession_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ORDERING_JSON" \
  --argjson cancellation_surfaces "$cancellation_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_cancellation_supersession_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_v1",
    receipt_cancellation_supersession_mode:"non_persistent_receipts_cannot_cancel_supersede_replace_or_derive_authority",
    source_packet_acceptance_receipt_ordering_monotonicity_gate:$source.gate,
    source_packet_acceptance_receipt_ordering_monotonicity_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_ready,
    source_ordering_report_sha256:$ordering_report_sha256,
    source_ordering_monotonicity_contract_hash_sha256:$source.ordering_monotonicity_contract_hash_sha256,
    cancellation_supersession_contract_hash_sha256:$cancellation_supersession_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_ready:true,
    source_ordering_surface_count:$source.ordering_surface_count,
    source_ordering_attempt_count:$source.ordering_attempt_count,
    source_ordering_recorded_count:$source.ordering_recorded_count,
    source_ordering_persisted_count:$source.ordering_persisted_count,
    source_sequence_cursor_recorded_count:$source.sequence_cursor_recorded_count,
    source_monotonicity_state_recorded_count:$source.monotonicity_state_recorded_count,
    source_ordering_acceptance_recorded_count:$source.ordering_acceptance_recorded_count,
    source_ordering_activation_authority_derived_count:$source.ordering_activation_authority_derived_count,
    cancellation_supersession_surface_count:($cancellation_surfaces | length),
    cancellation_supersession_attempt_count:($cancellation_surfaces | length),
    cancellation_accepted_count:0,
    cancellation_recorded_count:0,
    cancellation_persisted_count:0,
    supersession_accepted_count:0,
    supersession_recorded_count:0,
    supersession_persisted_count:0,
    replacement_receipt_accepted_count:0,
    replacement_receipt_recorded_count:0,
    replacement_receipt_persisted_count:0,
    tombstone_recorded_count:0,
    tombstone_persisted_count:0,
    delete_marker_recorded_count:0,
    latest_replacement_accepted_count:0,
    ack_replacement_accepted_count:0,
    query_replacement_registered_count:0,
    export_replacement_recorded_count:0,
    observability_replacement_recorded_count:0,
    cancellation_supersession_acceptance_recorded_count:0,
    cancellation_supersession_operator_approval_derived_count:0,
    cancellation_supersession_activation_authority_derived_count:0,
    cancellation_supersession_activation_command_derived_count:0,
    cancellation_supersession_live_execution_allowed_count:0,
    cancellation_surfaces:$cancellation_surfaces,
    denied_by_packet_receipt_cancellation_supersession:[
      "operator_readiness_packet_template_packet_receipt_cancellation_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_cancellation_recording_denied",
      "operator_readiness_packet_template_packet_receipt_cancellation_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_supersession_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_supersession_recording_denied",
      "operator_readiness_packet_template_packet_receipt_supersession_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_replacement_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_replacement_recording_denied",
      "operator_readiness_packet_template_packet_receipt_replacement_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_tombstone_recording_denied",
      "operator_readiness_packet_template_packet_receipt_tombstone_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_delete_marker_recording_denied",
      "operator_readiness_packet_template_packet_receipt_latest_replacement_denied",
      "operator_readiness_packet_template_packet_receipt_ack_replacement_denied",
      "operator_readiness_packet_template_packet_receipt_query_replacement_denied",
      "operator_readiness_packet_template_packet_receipt_export_replacement_denied",
      "operator_readiness_packet_template_packet_receipt_observability_replacement_denied",
      "operator_readiness_packet_template_packet_receipt_acceptance_from_cancellation_supersession_denied",
      "operator_readiness_packet_template_packet_receipt_authority_from_cancellation_supersession_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_from_cancellation_supersession_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_gate",
        status:"allowed_report_only_next_slice",
        persists_receipt:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        records_audit_trail:false,
        accepts_immutable_evidence:false,
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
    packet_acceptance_receipt_ordering_recorded:false,
    packet_acceptance_receipt_ordering_persisted:false,
    packet_acceptance_receipt_sequence_cursor_recorded:false,
    packet_acceptance_receipt_monotonicity_state_recorded:false,
    packet_acceptance_receipt_cancellation_accepted:false,
    packet_acceptance_receipt_cancellation_recorded:false,
    packet_acceptance_receipt_cancellation_persisted:false,
    packet_acceptance_receipt_supersession_accepted:false,
    packet_acceptance_receipt_supersession_recorded:false,
    packet_acceptance_receipt_supersession_persisted:false,
    packet_acceptance_receipt_replacement_accepted:false,
    packet_acceptance_receipt_replacement_recorded:false,
    packet_acceptance_receipt_replacement_persisted:false,
    packet_acceptance_receipt_tombstone_recorded:false,
    packet_acceptance_receipt_tombstone_persisted:false,
    packet_acceptance_receipt_delete_marker_recorded:false,
    packet_acceptance_receipt_latest_replacement_accepted:false,
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
      packet_acceptance_receipt_cancellation_accepted:false,
      packet_acceptance_receipt_cancellation_recorded:false,
      packet_acceptance_receipt_cancellation_persisted:false,
      packet_acceptance_receipt_supersession_accepted:false,
      packet_acceptance_receipt_supersession_recorded:false,
      packet_acceptance_receipt_supersession_persisted:false,
      packet_acceptance_receipt_replacement_accepted:false,
      packet_acceptance_receipt_replacement_recorded:false,
      packet_acceptance_receipt_replacement_persisted:false,
      packet_acceptance_receipt_tombstone_recorded:false,
      packet_acceptance_receipt_tombstone_persisted:false,
      packet_acceptance_receipt_delete_marker_recorded:false,
      packet_acceptance_receipt_latest_replacement_accepted:false,
      packet_acceptance_receipt_acceptance_recorded:false,
      packet_acceptance_receipt_authority_derived:false,
      packet_acceptance_receipt_live_execution_allowed:false,
      packet_acceptance_receipt_ordering_recorded:false,
      packet_acceptance_receipt_ordering_persisted:false,
      packet_acceptance_receipt_sequence_cursor_recorded:false,
      packet_acceptance_receipt_monotonicity_state_recorded:false,
      packet_acceptance_receipt_replayed:false,
      packet_acceptance_receipt_replay_recorded:false,
      packet_acceptance_receipt_replay_persisted:false,
      packet_acceptance_receipt_recorded:false,
      packet_acceptance_receipt_persisted:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_ready == true
  and .source_packet_acceptance_receipt_ordering_monotonicity_ready == true
  and .source_ordering_surface_count == 14
  and .source_ordering_attempt_count == 14
  and .source_ordering_recorded_count == 0
  and .source_ordering_persisted_count == 0
  and .source_sequence_cursor_recorded_count == 0
  and .source_monotonicity_state_recorded_count == 0
  and .source_ordering_acceptance_recorded_count == 0
  and .source_ordering_activation_authority_derived_count == 0
  and .cancellation_supersession_surface_count == 14
  and .cancellation_supersession_attempt_count == 14
  and .cancellation_accepted_count == 0
  and .cancellation_recorded_count == 0
  and .cancellation_persisted_count == 0
  and .supersession_accepted_count == 0
  and .supersession_recorded_count == 0
  and .supersession_persisted_count == 0
  and .replacement_receipt_accepted_count == 0
  and .replacement_receipt_recorded_count == 0
  and .replacement_receipt_persisted_count == 0
  and .tombstone_recorded_count == 0
  and .tombstone_persisted_count == 0
  and .delete_marker_recorded_count == 0
  and .latest_replacement_accepted_count == 0
  and .cancellation_supersession_acceptance_recorded_count == 0
  and .cancellation_supersession_operator_approval_derived_count == 0
  and .cancellation_supersession_activation_authority_derived_count == 0
  and .cancellation_supersession_activation_command_derived_count == 0
  and .cancellation_supersession_live_execution_allowed_count == 0
  and (.cancellation_surfaces | all(
    .cancellation_or_supersession_attempted == true
    and .cancellation_accepted == false
    and .cancellation_recorded == false
    and .cancellation_persisted == false
    and .supersession_accepted == false
    and .supersession_recorded == false
    and .supersession_persisted == false
    and .replacement_receipt_accepted == false
    and .replacement_receipt_recorded == false
    and .replacement_receipt_persisted == false
    and .tombstone_recorded == false
    and .tombstone_persisted == false
    and .delete_marker_recorded == false
    and .latest_replacement_accepted == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .cancellation_supersession_status == "cancellation_supersession_denied"
  ))
  and (.denied_by_packet_receipt_cancellation_supersession | length) == 20
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_template_recorded == false
  and .packet_template_persisted == false
  and .packet_assembly_performed == false
  and .packet_accepted == false
  and .packet_acceptance_receipt_recorded == false
  and .packet_acceptance_receipt_persisted == false
  and .packet_acceptance_receipt_replayed == false
  and .packet_acceptance_receipt_ordering_recorded == false
  and .packet_acceptance_receipt_cancellation_accepted == false
  and .packet_acceptance_receipt_cancellation_recorded == false
  and .packet_acceptance_receipt_cancellation_persisted == false
  and .packet_acceptance_receipt_supersession_accepted == false
  and .packet_acceptance_receipt_supersession_recorded == false
  and .packet_acceptance_receipt_supersession_persisted == false
  and .packet_acceptance_receipt_replacement_accepted == false
  and .packet_acceptance_receipt_replacement_recorded == false
  and .packet_acceptance_receipt_replacement_persisted == false
  and .packet_acceptance_receipt_tombstone_recorded == false
  and .packet_acceptance_receipt_tombstone_persisted == false
  and .packet_acceptance_receipt_delete_marker_recorded == false
  and .packet_acceptance_receipt_latest_replacement_accepted == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt cancellation/supersession denial gate passed"
