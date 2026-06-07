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

AUDIT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial-gate.sh
)"

audit_report_sha256="$(sha256_text "$AUDIT_JSON")"
retention_expiry_gc_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-retention-expiry-garbage-collection-denial:$audit_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$AUDIT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.audit_evidence_surface_count == 16
    and $source.audit_evidence_attempt_count == 16
    and $source.audit_trail_recorded_count == 0
    and $source.audit_trail_persisted_count == 0
    and $source.immutable_evidence_recorded_count == 0
    and $source.immutable_evidence_persisted_count == 0
    and $source.hash_chain_recorded_count == 0
    and $source.merkle_root_recorded_count == 0
    and $source.attestation_recorded_count == 0
    and $source.ledger_evidence_recorded_count == 0
    and $source.readback_evidence_recorded_count == 0
    and $source.audit_evidence_acceptance_recorded_count == 0
    and $source.audit_evidence_operator_approval_derived_count == 0
    and $source.audit_evidence_activation_authority_derived_count == 0
    and $source.audit_evidence_activation_command_derived_count == 0
    and $source.audit_evidence_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

retention_surfaces_json="$(
  jq -n '[
    "packet_receipt_retention_policy_claim",
    "packet_receipt_retention_index_claim",
    "packet_receipt_ttl_update_claim",
    "packet_receipt_ttl_extension_claim",
    "packet_receipt_expiry_scheduler_claim",
    "packet_receipt_expiry_timer_claim",
    "packet_receipt_gc_scan_claim",
    "packet_receipt_gc_candidate_claim",
    "packet_receipt_delete_claim",
    "packet_receipt_tombstone_sweep_claim",
    "packet_receipt_archive_claim",
    "packet_receipt_compaction_claim",
    "packet_receipt_ledger_retention_claim",
    "packet_receipt_index_retention_claim",
    "packet_receipt_delivery_retention_claim",
    "packet_receipt_authority_retention_claim",
    "packet_receipt_live_retention_claim"
  ] | map({
    retention_surface: .,
    retention_expiry_or_gc_attempted: true,
    retention_policy_accepted: false,
    retention_policy_recorded: false,
    retention_policy_persisted: false,
    retention_index_recorded: false,
    ttl_update_accepted: false,
    ttl_update_recorded: false,
    ttl_extension_accepted: false,
    ttl_extension_recorded: false,
    expiry_accepted: false,
    expiry_recorded: false,
    expiry_persisted: false,
    expiry_scheduler_registered: false,
    expiry_timer_started: false,
    garbage_collection_accepted: false,
    garbage_collection_scan_performed: false,
    garbage_collection_candidate_recorded: false,
    garbage_collection_decision_recorded: false,
    delete_accepted: false,
    delete_performed: false,
    tombstone_recorded: false,
    sweep_performed: false,
    archive_written: false,
    compaction_performed: false,
    compaction_artifact_written: false,
    ledger_retention_recorded: false,
    ledger_retention_persisted: false,
    index_retention_recorded: false,
    index_retention_persisted: false,
    delivery_retention_recorded: false,
    delivery_retention_persisted: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    retention_gc_status: "retention_expiry_garbage_collection_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_gate" \
  --arg audit_report_sha256 "$audit_report_sha256" \
  --arg retention_expiry_gc_contract_hash_sha256 "$retention_expiry_gc_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$AUDIT_JSON" \
  --argjson retention_surfaces "$retention_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_retention_expiry_gc_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_v1",
    receipt_retention_expiry_gc_mode:"non_persistent_receipts_cannot_create_retention_expiry_gc_state_or_authority",
    source_packet_acceptance_receipt_audit_evidence_gate:$source.gate,
    source_packet_acceptance_receipt_audit_evidence_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_ready,
    source_audit_report_sha256:$audit_report_sha256,
    source_audit_trail_immutable_evidence_contract_hash_sha256:$source.audit_trail_immutable_evidence_contract_hash_sha256,
    retention_expiry_garbage_collection_contract_hash_sha256:$retention_expiry_gc_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_ready:true,
    source_audit_evidence_surface_count:$source.audit_evidence_surface_count,
    source_audit_evidence_attempt_count:$source.audit_evidence_attempt_count,
    source_audit_trail_recorded_count:$source.audit_trail_recorded_count,
    source_immutable_evidence_recorded_count:$source.immutable_evidence_recorded_count,
    source_hash_chain_recorded_count:$source.hash_chain_recorded_count,
    source_ledger_evidence_recorded_count:$source.ledger_evidence_recorded_count,
    source_audit_evidence_acceptance_recorded_count:$source.audit_evidence_acceptance_recorded_count,
    source_audit_evidence_activation_authority_derived_count:$source.audit_evidence_activation_authority_derived_count,
    retention_expiry_gc_surface_count:($retention_surfaces | length),
    retention_expiry_gc_attempt_count:($retention_surfaces | length),
    retention_policy_accepted_count:0,
    retention_policy_recorded_count:0,
    retention_policy_persisted_count:0,
    retention_index_recorded_count:0,
    ttl_update_accepted_count:0,
    ttl_update_recorded_count:0,
    ttl_extension_accepted_count:0,
    ttl_extension_recorded_count:0,
    expiry_accepted_count:0,
    expiry_recorded_count:0,
    expiry_persisted_count:0,
    expiry_scheduler_registered_count:0,
    expiry_timer_started_count:0,
    garbage_collection_accepted_count:0,
    garbage_collection_scan_performed_count:0,
    garbage_collection_candidate_recorded_count:0,
    garbage_collection_decision_recorded_count:0,
    delete_accepted_count:0,
    delete_performed_count:0,
    tombstone_recorded_count:0,
    sweep_performed_count:0,
    archive_written_count:0,
    compaction_performed_count:0,
    compaction_artifact_written_count:0,
    ledger_retention_recorded_count:0,
    ledger_retention_persisted_count:0,
    index_retention_recorded_count:0,
    index_retention_persisted_count:0,
    delivery_retention_recorded_count:0,
    delivery_retention_persisted_count:0,
    retention_gc_acceptance_recorded_count:0,
    retention_gc_operator_approval_derived_count:0,
    retention_gc_activation_authority_derived_count:0,
    retention_gc_activation_command_derived_count:0,
    retention_gc_live_execution_allowed_count:0,
    retention_surfaces:$retention_surfaces,
    denied_by_packet_receipt_retention_expiry_garbage_collection:[
      "operator_readiness_packet_template_packet_receipt_retention_policy_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_retention_policy_recording_denied",
      "operator_readiness_packet_template_packet_receipt_retention_policy_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_retention_index_recording_denied",
      "operator_readiness_packet_template_packet_receipt_ttl_update_denied",
      "operator_readiness_packet_template_packet_receipt_ttl_extension_denied",
      "operator_readiness_packet_template_packet_receipt_expiry_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_expiry_recording_denied",
      "operator_readiness_packet_template_packet_receipt_expiry_scheduler_denied",
      "operator_readiness_packet_template_packet_receipt_expiry_timer_denied",
      "operator_readiness_packet_template_packet_receipt_garbage_collection_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_garbage_collection_scan_denied",
      "operator_readiness_packet_template_packet_receipt_garbage_collection_candidate_denied",
      "operator_readiness_packet_template_packet_receipt_delete_denied",
      "operator_readiness_packet_template_packet_receipt_tombstone_sweep_denied",
      "operator_readiness_packet_template_packet_receipt_archive_denied",
      "operator_readiness_packet_template_packet_receipt_compaction_denied",
      "operator_readiness_packet_template_packet_receipt_ledger_index_delivery_retention_denied",
      "operator_readiness_packet_template_packet_receipt_acceptance_from_retention_gc_denied",
      "operator_readiness_packet_template_packet_receipt_authority_from_retention_gc_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_from_retention_gc_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_gate",
        status:"allowed_report_only_next_slice",
        persists_receipt:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        exports_receipt:false,
        registers_query:false,
        records_observability:false,
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
    packet_acceptance_receipt_audit_trail_recorded:false,
    packet_acceptance_receipt_immutable_evidence_recorded:false,
    packet_acceptance_receipt_retention_policy_recorded:false,
    packet_acceptance_receipt_retention_policy_persisted:false,
    packet_acceptance_receipt_retention_index_recorded:false,
    packet_acceptance_receipt_ttl_update_recorded:false,
    packet_acceptance_receipt_ttl_extension_recorded:false,
    packet_acceptance_receipt_expiry_recorded:false,
    packet_acceptance_receipt_expiry_scheduler_registered:false,
    packet_acceptance_receipt_expiry_timer_started:false,
    packet_acceptance_receipt_garbage_collection_scan_performed:false,
    packet_acceptance_receipt_garbage_collection_candidate_recorded:false,
    packet_acceptance_receipt_delete_performed:false,
    packet_acceptance_receipt_tombstone_recorded:false,
    packet_acceptance_receipt_archive_written:false,
    packet_acceptance_receipt_compaction_performed:false,
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
      packet_acceptance_receipt_retention_policy_recorded:false,
      packet_acceptance_receipt_retention_policy_persisted:false,
      packet_acceptance_receipt_retention_index_recorded:false,
      packet_acceptance_receipt_ttl_update_recorded:false,
      packet_acceptance_receipt_ttl_extension_recorded:false,
      packet_acceptance_receipt_expiry_recorded:false,
      packet_acceptance_receipt_expiry_persisted:false,
      packet_acceptance_receipt_expiry_scheduler_registered:false,
      packet_acceptance_receipt_expiry_timer_started:false,
      packet_acceptance_receipt_garbage_collection_scan_performed:false,
      packet_acceptance_receipt_garbage_collection_candidate_recorded:false,
      packet_acceptance_receipt_garbage_collection_decision_recorded:false,
      packet_acceptance_receipt_delete_performed:false,
      packet_acceptance_receipt_tombstone_recorded:false,
      packet_acceptance_receipt_sweep_performed:false,
      packet_acceptance_receipt_archive_written:false,
      packet_acceptance_receipt_compaction_performed:false,
      packet_acceptance_receipt_compaction_artifact_written:false,
      packet_acceptance_receipt_ledger_retention_recorded:false,
      packet_acceptance_receipt_index_retention_recorded:false,
      packet_acceptance_receipt_delivery_retention_recorded:false,
      packet_acceptance_receipt_acceptance_recorded:false,
      packet_acceptance_receipt_authority_derived:false,
      packet_acceptance_receipt_live_execution_allowed:false,
      packet_acceptance_receipt_audit_trail_recorded:false,
      packet_acceptance_receipt_immutable_evidence_recorded:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .source_packet_acceptance_receipt_audit_evidence_ready == true
  and .source_audit_evidence_surface_count == 16
  and .source_audit_trail_recorded_count == 0
  and .source_immutable_evidence_recorded_count == 0
  and .source_audit_evidence_acceptance_recorded_count == 0
  and .source_audit_evidence_activation_authority_derived_count == 0
  and .retention_expiry_gc_surface_count == 17
  and .retention_expiry_gc_attempt_count == 17
  and .retention_policy_accepted_count == 0
  and .retention_policy_recorded_count == 0
  and .retention_policy_persisted_count == 0
  and .retention_index_recorded_count == 0
  and .ttl_update_accepted_count == 0
  and .ttl_update_recorded_count == 0
  and .ttl_extension_accepted_count == 0
  and .ttl_extension_recorded_count == 0
  and .expiry_accepted_count == 0
  and .expiry_recorded_count == 0
  and .expiry_persisted_count == 0
  and .expiry_scheduler_registered_count == 0
  and .expiry_timer_started_count == 0
  and .garbage_collection_accepted_count == 0
  and .garbage_collection_scan_performed_count == 0
  and .garbage_collection_candidate_recorded_count == 0
  and .garbage_collection_decision_recorded_count == 0
  and .delete_accepted_count == 0
  and .delete_performed_count == 0
  and .tombstone_recorded_count == 0
  and .sweep_performed_count == 0
  and .archive_written_count == 0
  and .compaction_performed_count == 0
  and .retention_gc_acceptance_recorded_count == 0
  and .retention_gc_operator_approval_derived_count == 0
  and .retention_gc_activation_authority_derived_count == 0
  and .retention_gc_activation_command_derived_count == 0
  and .retention_gc_live_execution_allowed_count == 0
  and (.retention_surfaces | all(
    .retention_expiry_or_gc_attempted == true
    and .retention_policy_accepted == false
    and .retention_policy_recorded == false
    and .retention_policy_persisted == false
    and .retention_index_recorded == false
    and .ttl_update_accepted == false
    and .expiry_accepted == false
    and .expiry_scheduler_registered == false
    and .expiry_timer_started == false
    and .garbage_collection_accepted == false
    and .garbage_collection_scan_performed == false
    and .delete_performed == false
    and .archive_written == false
    and .compaction_performed == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .retention_gc_status == "retention_expiry_garbage_collection_denied"
  ))
  and (.denied_by_packet_receipt_retention_expiry_garbage_collection | length) == 21
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_retention_policy_recorded == false
  and .packet_acceptance_receipt_expiry_scheduler_registered == false
  and .packet_acceptance_receipt_garbage_collection_scan_performed == false
  and .packet_acceptance_receipt_delete_performed == false
  and .packet_acceptance_receipt_archive_written == false
  and .packet_acceptance_receipt_compaction_performed == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt retention/expiry/garbage-collection denial gate passed"
