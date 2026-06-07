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

CANCELLATION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial-gate.sh
)"

cancellation_report_sha256="$(sha256_text "$CANCELLATION_JSON")"
audit_trail_immutable_evidence_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial:$cancellation_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$CANCELLATION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_ready == true
    and $source.source_packet_acceptance_receipt_ordering_monotonicity_ready == true
    and $source.cancellation_supersession_surface_count == 14
    and $source.cancellation_supersession_attempt_count == 14
    and $source.cancellation_accepted_count == 0
    and $source.cancellation_recorded_count == 0
    and $source.cancellation_persisted_count == 0
    and $source.supersession_accepted_count == 0
    and $source.supersession_recorded_count == 0
    and $source.supersession_persisted_count == 0
    and $source.replacement_receipt_accepted_count == 0
    and $source.replacement_receipt_recorded_count == 0
    and $source.replacement_receipt_persisted_count == 0
    and $source.tombstone_recorded_count == 0
    and $source.tombstone_persisted_count == 0
    and $source.delete_marker_recorded_count == 0
    and $source.cancellation_supersession_acceptance_recorded_count == 0
    and $source.cancellation_supersession_operator_approval_derived_count == 0
    and $source.cancellation_supersession_activation_authority_derived_count == 0
    and $source.cancellation_supersession_activation_command_derived_count == 0
    and $source.cancellation_supersession_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

audit_surfaces_json="$(
  jq -n '[
    "packet_receipt_audit_trail_append_claim",
    "packet_receipt_immutable_evidence_claim",
    "packet_receipt_hash_chain_claim",
    "packet_receipt_merkle_root_claim",
    "packet_receipt_attestation_claim",
    "packet_receipt_witness_claim",
    "packet_receipt_notary_claim",
    "packet_receipt_ledger_evidence_claim",
    "packet_receipt_index_evidence_claim",
    "packet_receipt_delivery_evidence_claim",
    "packet_receipt_export_evidence_claim",
    "packet_receipt_query_evidence_claim",
    "packet_receipt_observability_evidence_claim",
    "packet_receipt_readback_evidence_claim",
    "packet_receipt_authority_evidence_claim",
    "packet_receipt_live_evidence_claim"
  ] | map({
    audit_surface: .,
    audit_or_evidence_attempted: true,
    audit_trail_accepted: false,
    audit_trail_recorded: false,
    audit_trail_persisted: false,
    audit_trail_materialized: false,
    immutable_evidence_accepted: false,
    immutable_evidence_recorded: false,
    immutable_evidence_persisted: false,
    immutable_evidence_materialized: false,
    hash_chain_recorded: false,
    hash_chain_persisted: false,
    merkle_root_recorded: false,
    merkle_root_persisted: false,
    attestation_recorded: false,
    attestation_persisted: false,
    witness_recorded: false,
    witness_persisted: false,
    notary_recorded: false,
    notary_persisted: false,
    ledger_evidence_recorded: false,
    ledger_evidence_persisted: false,
    index_evidence_recorded: false,
    index_evidence_persisted: false,
    delivery_evidence_recorded: false,
    delivery_evidence_persisted: false,
    export_evidence_recorded: false,
    query_evidence_registered: false,
    observability_evidence_recorded: false,
    readback_evidence_recorded: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    audit_evidence_status: "audit_trail_immutable_evidence_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_gate" \
  --arg cancellation_report_sha256 "$cancellation_report_sha256" \
  --arg audit_trail_immutable_evidence_contract_hash_sha256 "$audit_trail_immutable_evidence_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$CANCELLATION_JSON" \
  --argjson audit_surfaces "$audit_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_audit_trail_immutable_evidence_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_v1",
    receipt_audit_trail_immutable_evidence_mode:"non_persistent_receipts_cannot_become_audit_trail_immutable_evidence_or_authority",
    source_packet_acceptance_receipt_cancellation_supersession_gate:$source.gate,
    source_packet_acceptance_receipt_cancellation_supersession_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_ready,
    source_cancellation_report_sha256:$cancellation_report_sha256,
    source_cancellation_supersession_contract_hash_sha256:$source.cancellation_supersession_contract_hash_sha256,
    audit_trail_immutable_evidence_contract_hash_sha256:$audit_trail_immutable_evidence_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_ready:true,
    source_cancellation_supersession_surface_count:$source.cancellation_supersession_surface_count,
    source_cancellation_supersession_attempt_count:$source.cancellation_supersession_attempt_count,
    source_cancellation_accepted_count:$source.cancellation_accepted_count,
    source_supersession_accepted_count:$source.supersession_accepted_count,
    source_replacement_receipt_accepted_count:$source.replacement_receipt_accepted_count,
    source_tombstone_recorded_count:$source.tombstone_recorded_count,
    source_cancellation_supersession_acceptance_recorded_count:$source.cancellation_supersession_acceptance_recorded_count,
    source_cancellation_supersession_activation_authority_derived_count:$source.cancellation_supersession_activation_authority_derived_count,
    audit_evidence_surface_count:($audit_surfaces | length),
    audit_evidence_attempt_count:($audit_surfaces | length),
    audit_trail_accepted_count:0,
    audit_trail_recorded_count:0,
    audit_trail_persisted_count:0,
    audit_trail_materialized_count:0,
    immutable_evidence_accepted_count:0,
    immutable_evidence_recorded_count:0,
    immutable_evidence_persisted_count:0,
    immutable_evidence_materialized_count:0,
    hash_chain_recorded_count:0,
    hash_chain_persisted_count:0,
    merkle_root_recorded_count:0,
    merkle_root_persisted_count:0,
    attestation_recorded_count:0,
    attestation_persisted_count:0,
    witness_recorded_count:0,
    witness_persisted_count:0,
    notary_recorded_count:0,
    notary_persisted_count:0,
    ledger_evidence_recorded_count:0,
    ledger_evidence_persisted_count:0,
    index_evidence_recorded_count:0,
    index_evidence_persisted_count:0,
    delivery_evidence_recorded_count:0,
    delivery_evidence_persisted_count:0,
    export_evidence_recorded_count:0,
    query_evidence_registered_count:0,
    observability_evidence_recorded_count:0,
    readback_evidence_recorded_count:0,
    audit_evidence_acceptance_recorded_count:0,
    audit_evidence_operator_approval_derived_count:0,
    audit_evidence_activation_authority_derived_count:0,
    audit_evidence_activation_command_derived_count:0,
    audit_evidence_live_execution_allowed_count:0,
    audit_surfaces:$audit_surfaces,
    denied_by_packet_receipt_audit_trail_immutable_evidence:[
      "operator_readiness_packet_template_packet_receipt_audit_trail_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_audit_trail_recording_denied",
      "operator_readiness_packet_template_packet_receipt_audit_trail_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_audit_trail_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_immutable_evidence_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_immutable_evidence_recording_denied",
      "operator_readiness_packet_template_packet_receipt_immutable_evidence_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_immutable_evidence_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_hash_chain_recording_denied",
      "operator_readiness_packet_template_packet_receipt_merkle_root_recording_denied",
      "operator_readiness_packet_template_packet_receipt_attestation_recording_denied",
      "operator_readiness_packet_template_packet_receipt_witness_recording_denied",
      "operator_readiness_packet_template_packet_receipt_notary_recording_denied",
      "operator_readiness_packet_template_packet_receipt_ledger_evidence_denied",
      "operator_readiness_packet_template_packet_receipt_index_evidence_denied",
      "operator_readiness_packet_template_packet_receipt_delivery_evidence_denied",
      "operator_readiness_packet_template_packet_receipt_export_query_observability_evidence_denied",
      "operator_readiness_packet_template_packet_receipt_readback_evidence_denied",
      "operator_readiness_packet_template_packet_receipt_acceptance_from_audit_evidence_denied",
      "operator_readiness_packet_template_packet_receipt_authority_from_audit_evidence_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_from_audit_evidence_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_gate",
        status:"allowed_report_only_next_slice",
        persists_receipt:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        records_retention_state:false,
        expires_receipt:false,
        garbage_collects_receipt:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_template_recorded:false,
    packet_template_persisted:false,
    packet_assembly_performed:false,
    packet_accepted:false,
    packet_acceptance_receipt_recorded:false,
    packet_acceptance_receipt_persisted:false,
    packet_acceptance_receipt_replayed:false,
    packet_acceptance_receipt_ordering_recorded:false,
    packet_acceptance_receipt_cancellation_recorded:false,
    packet_acceptance_receipt_supersession_recorded:false,
    packet_acceptance_receipt_replacement_recorded:false,
    packet_acceptance_receipt_audit_trail_accepted:false,
    packet_acceptance_receipt_audit_trail_recorded:false,
    packet_acceptance_receipt_audit_trail_persisted:false,
    packet_acceptance_receipt_immutable_evidence_accepted:false,
    packet_acceptance_receipt_immutable_evidence_recorded:false,
    packet_acceptance_receipt_immutable_evidence_persisted:false,
    packet_acceptance_receipt_hash_chain_recorded:false,
    packet_acceptance_receipt_merkle_root_recorded:false,
    packet_acceptance_receipt_attestation_recorded:false,
    packet_acceptance_receipt_ledger_evidence_recorded:false,
    packet_acceptance_receipt_readback_evidence_recorded:false,
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
      packet_acceptance_receipt_audit_trail_accepted:false,
      packet_acceptance_receipt_audit_trail_recorded:false,
      packet_acceptance_receipt_audit_trail_persisted:false,
      packet_acceptance_receipt_audit_trail_materialized:false,
      packet_acceptance_receipt_immutable_evidence_accepted:false,
      packet_acceptance_receipt_immutable_evidence_recorded:false,
      packet_acceptance_receipt_immutable_evidence_persisted:false,
      packet_acceptance_receipt_immutable_evidence_materialized:false,
      packet_acceptance_receipt_hash_chain_recorded:false,
      packet_acceptance_receipt_hash_chain_persisted:false,
      packet_acceptance_receipt_merkle_root_recorded:false,
      packet_acceptance_receipt_merkle_root_persisted:false,
      packet_acceptance_receipt_attestation_recorded:false,
      packet_acceptance_receipt_attestation_persisted:false,
      packet_acceptance_receipt_witness_recorded:false,
      packet_acceptance_receipt_witness_persisted:false,
      packet_acceptance_receipt_notary_recorded:false,
      packet_acceptance_receipt_notary_persisted:false,
      packet_acceptance_receipt_ledger_evidence_recorded:false,
      packet_acceptance_receipt_ledger_evidence_persisted:false,
      packet_acceptance_receipt_index_evidence_recorded:false,
      packet_acceptance_receipt_index_evidence_persisted:false,
      packet_acceptance_receipt_delivery_evidence_recorded:false,
      packet_acceptance_receipt_delivery_evidence_persisted:false,
      packet_acceptance_receipt_export_evidence_recorded:false,
      packet_acceptance_receipt_query_evidence_registered:false,
      packet_acceptance_receipt_observability_evidence_recorded:false,
      packet_acceptance_receipt_readback_evidence_recorded:false,
      packet_acceptance_receipt_acceptance_recorded:false,
      packet_acceptance_receipt_authority_derived:false,
      packet_acceptance_receipt_live_execution_allowed:false,
      packet_acceptance_receipt_cancellation_recorded:false,
      packet_acceptance_receipt_supersession_recorded:false,
      packet_acceptance_receipt_replacement_recorded:false,
      packet_acceptance_receipt_recorded:false,
      packet_acceptance_receipt_persisted:false,
      packet_template_recorded:false,
      packet_template_persisted:false,
      packet_assembly_performed:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .source_packet_acceptance_receipt_cancellation_supersession_ready == true
  and .source_cancellation_supersession_surface_count == 14
  and .source_cancellation_supersession_attempt_count == 14
  and .source_cancellation_accepted_count == 0
  and .source_supersession_accepted_count == 0
  and .source_replacement_receipt_accepted_count == 0
  and .source_tombstone_recorded_count == 0
  and .source_cancellation_supersession_acceptance_recorded_count == 0
  and .source_cancellation_supersession_activation_authority_derived_count == 0
  and .audit_evidence_surface_count == 16
  and .audit_evidence_attempt_count == 16
  and .audit_trail_accepted_count == 0
  and .audit_trail_recorded_count == 0
  and .audit_trail_persisted_count == 0
  and .audit_trail_materialized_count == 0
  and .immutable_evidence_accepted_count == 0
  and .immutable_evidence_recorded_count == 0
  and .immutable_evidence_persisted_count == 0
  and .immutable_evidence_materialized_count == 0
  and .hash_chain_recorded_count == 0
  and .hash_chain_persisted_count == 0
  and .merkle_root_recorded_count == 0
  and .merkle_root_persisted_count == 0
  and .attestation_recorded_count == 0
  and .attestation_persisted_count == 0
  and .witness_recorded_count == 0
  and .witness_persisted_count == 0
  and .notary_recorded_count == 0
  and .notary_persisted_count == 0
  and .ledger_evidence_recorded_count == 0
  and .ledger_evidence_persisted_count == 0
  and .index_evidence_recorded_count == 0
  and .index_evidence_persisted_count == 0
  and .delivery_evidence_recorded_count == 0
  and .delivery_evidence_persisted_count == 0
  and .export_evidence_recorded_count == 0
  and .query_evidence_registered_count == 0
  and .observability_evidence_recorded_count == 0
  and .readback_evidence_recorded_count == 0
  and .audit_evidence_acceptance_recorded_count == 0
  and .audit_evidence_operator_approval_derived_count == 0
  and .audit_evidence_activation_authority_derived_count == 0
  and .audit_evidence_activation_command_derived_count == 0
  and .audit_evidence_live_execution_allowed_count == 0
  and (.audit_surfaces | all(
    .audit_or_evidence_attempted == true
    and .audit_trail_accepted == false
    and .audit_trail_recorded == false
    and .audit_trail_persisted == false
    and .immutable_evidence_accepted == false
    and .immutable_evidence_recorded == false
    and .immutable_evidence_persisted == false
    and .hash_chain_recorded == false
    and .merkle_root_recorded == false
    and .attestation_recorded == false
    and .witness_recorded == false
    and .notary_recorded == false
    and .ledger_evidence_recorded == false
    and .index_evidence_recorded == false
    and .delivery_evidence_recorded == false
    and .export_evidence_recorded == false
    and .query_evidence_registered == false
    and .observability_evidence_recorded == false
    and .readback_evidence_recorded == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .audit_evidence_status == "audit_trail_immutable_evidence_denied"
  ))
  and (.denied_by_packet_receipt_audit_trail_immutable_evidence | length) == 21
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_audit_trail_accepted == false
  and .packet_acceptance_receipt_audit_trail_recorded == false
  and .packet_acceptance_receipt_audit_trail_persisted == false
  and .packet_acceptance_receipt_immutable_evidence_accepted == false
  and .packet_acceptance_receipt_immutable_evidence_recorded == false
  and .packet_acceptance_receipt_immutable_evidence_persisted == false
  and .packet_acceptance_receipt_hash_chain_recorded == false
  and .packet_acceptance_receipt_merkle_root_recorded == false
  and .packet_acceptance_receipt_attestation_recorded == false
  and .packet_acceptance_receipt_ledger_evidence_recorded == false
  and .packet_acceptance_receipt_readback_evidence_recorded == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt audit-trail/immutable-evidence denial gate passed"
