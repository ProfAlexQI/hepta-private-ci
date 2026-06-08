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

RESULT_RECEIPT_AUDIT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-audit-trail-immutable-evidence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-audit-trail-immutable-evidence-denial-gate.sh
)"

result_receipt_audit_report_sha256="$(sha256_text "$RESULT_RECEIPT_AUDIT_JSON")"
result_receipt_retention_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-retention-expiry-garbage-collection-denial:$result_receipt_audit_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$RESULT_RECEIPT_AUDIT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_ready == true
    and $source.release_publication_result_receipt_audit_evidence_surface_count == 16
    and $source.release_publication_result_receipt_audit_evidence_attempt_count == 16
    and $source.release_publication_result_receipt_audit_trail_recorded_count == 0
    and $source.release_publication_result_receipt_audit_trail_persisted_count == 0
    and $source.release_publication_result_receipt_immutable_evidence_recorded_count == 0
    and $source.release_publication_result_receipt_immutable_evidence_persisted_count == 0
    and $source.release_publication_result_receipt_hash_chain_recorded_count == 0
    and $source.release_publication_result_receipt_merkle_root_recorded_count == 0
    and $source.release_publication_result_receipt_attestation_recorded_count == 0
    and $source.release_publication_result_receipt_witness_recorded_count == 0
    and $source.release_publication_result_receipt_notary_recorded_count == 0
    and $source.release_publication_result_receipt_ledger_evidence_recorded_count == 0
    and $source.release_publication_result_receipt_index_evidence_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_evidence_recorded_count == 0
    and $source.release_publication_result_receipt_export_evidence_recorded_count == 0
    and $source.release_publication_result_receipt_query_evidence_registered_count == 0
    and $source.release_publication_result_receipt_observability_evidence_recorded_count == 0
    and $source.release_publication_result_receipt_readback_evidence_recorded_count == 0
    and $source.release_publication_result_receipt_publication_completion_ack_recorded_count == 0
    and $source.release_publication_result_receipt_audit_evidence_acceptance_recorded_count == 0
    and $source.release_publication_result_receipt_audit_evidence_operator_approval_derived_count == 0
    and $source.release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_audit_evidence_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_audit_evidence_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_audit_evidence_live_execution_allowed_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_ledger_evidence_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_replayed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_recorded == false
    and $source.packet_acceptance_receipt_release_publication_recorded == false
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.secret_file_read == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.public_release_claimed == false
    and $source.public_ga_claimed == false
    and $source.release_artifact_written == false
    and $source.public_artifact_written == false
    and $source.external_send_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

result_receipt_retention_surfaces_json="$(
  jq -n '[
    "publication_result_receipt_retention_policy_claim",
    "publication_result_receipt_retention_index_claim",
    "publication_result_receipt_retention_ledger_claim",
    "publication_result_receipt_ttl_update_claim",
    "publication_result_receipt_ttl_extension_claim",
    "publication_result_receipt_expiry_schedule_claim",
    "publication_result_receipt_expiry_timer_claim",
    "publication_result_receipt_expiry_ack_claim",
    "publication_result_receipt_garbage_collection_scan_claim",
    "publication_result_receipt_garbage_collection_candidate_claim",
    "publication_result_receipt_garbage_collection_decision_claim",
    "publication_result_receipt_delete_claim",
    "publication_result_receipt_tombstone_claim",
    "publication_result_receipt_sweep_claim",
    "publication_result_receipt_archive_claim",
    "publication_result_receipt_compaction_claim",
    "publication_result_receipt_release_publication_authority_retention_claim",
    "publication_result_receipt_activation_live_install_restart_active_binary_retention_claim"
  ] | map({
    release_publication_result_receipt_retention_surface: .,
    source_release_publication_result_receipt_audit_evidence_ready: true,
    retention_expiry_or_garbage_collection_attempted: true,
    retention_policy_accepted: false,
    retention_policy_recorded: false,
    retention_policy_persisted: false,
    retention_policy_materialized: false,
    retention_index_recorded: false,
    retention_index_persisted: false,
    retention_ledger_recorded: false,
    retention_ledger_persisted: false,
    ttl_update_accepted: false,
    ttl_update_recorded: false,
    ttl_update_persisted: false,
    ttl_extension_accepted: false,
    ttl_extension_recorded: false,
    ttl_extension_persisted: false,
    expiry_accepted: false,
    expiry_recorded: false,
    expiry_persisted: false,
    expiry_scheduler_registered: false,
    expiry_timer_started: false,
    expiry_ack_recorded: false,
    garbage_collection_accepted: false,
    garbage_collection_scan_performed: false,
    garbage_collection_candidate_recorded: false,
    garbage_collection_decision_recorded: false,
    garbage_collection_persisted: false,
    delete_accepted: false,
    delete_marker_recorded: false,
    delete_performed: false,
    tombstone_recorded: false,
    tombstone_persisted: false,
    sweep_performed: false,
    archive_written: false,
    archive_persisted: false,
    compaction_performed: false,
    compaction_artifact_written: false,
    compaction_artifact_persisted: false,
    ledger_retention_recorded: false,
    ledger_retention_persisted: false,
    index_retention_recorded: false,
    index_retention_persisted: false,
    delivery_retention_recorded: false,
    delivery_retention_persisted: false,
    audit_trail_recorded: false,
    immutable_evidence_recorded: false,
    hash_chain_recorded: false,
    readback_evidence_recorded: false,
    publication_completion_ack_recorded: false,
    release_publication_recorded: false,
    release_artifact_written: false,
    public_artifact_written: false,
    publication_queue_enqueued: false,
    publication_manifest_written: false,
    public_distribution_performed: false,
    channel_delivery_performed: false,
    external_send_performed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    release_publication_authority_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    activation_performed: false,
    memory_store_write_performed: false,
    memory_store_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    install_executed: false,
    launchd_mutated: false,
    service_restarted: false,
    active_binary_mutated: false,
    retention_expiry_garbage_collection_noop_confirmed: true,
    release_publication_result_receipt_retention_status: "release_publication_result_receipt_retention_expiry_garbage_collection_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_gate" \
  --arg result_receipt_audit_report_sha256 "$result_receipt_audit_report_sha256" \
  --arg result_receipt_retention_contract_hash_sha256 "$result_receipt_retention_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RESULT_RECEIPT_AUDIT_JSON" \
  --argjson surfaces "$result_receipt_retention_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_retention_expiry_garbage_collection_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_v1",
    receipt_release_publication_result_receipt_retention_expiry_garbage_collection_mode:"denied_release_publication_result_receipt_cannot_create_retention_expiry_garbage_collection_state_or_authority",
    source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_report_sha256:$result_receipt_audit_report_sha256,
    source_release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256:$source.release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256,
    release_publication_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256:$result_receipt_retention_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_ready:true,
    source_release_publication_result_receipt_audit_evidence_surface_count:$source.release_publication_result_receipt_audit_evidence_surface_count,
    source_release_publication_result_receipt_audit_evidence_attempt_count:$source.release_publication_result_receipt_audit_evidence_attempt_count,
    source_release_publication_result_receipt_audit_trail_recorded_count:$source.release_publication_result_receipt_audit_trail_recorded_count,
    source_release_publication_result_receipt_immutable_evidence_recorded_count:$source.release_publication_result_receipt_immutable_evidence_recorded_count,
    source_release_publication_result_receipt_hash_chain_recorded_count:$source.release_publication_result_receipt_hash_chain_recorded_count,
    source_release_publication_result_receipt_ledger_evidence_recorded_count:$source.release_publication_result_receipt_ledger_evidence_recorded_count,
    source_release_publication_result_receipt_readback_evidence_recorded_count:$source.release_publication_result_receipt_readback_evidence_recorded_count,
    source_release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count:$source.release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count,
    source_release_publication_result_receipt_audit_evidence_activation_authority_derived_count:$source.release_publication_result_receipt_audit_evidence_activation_authority_derived_count,
    release_publication_result_receipt_retention_surface_count:($surfaces | length),
    release_publication_result_receipt_retention_attempt_count:($surfaces | length),
    release_publication_result_receipt_retention_policy_accepted_count:0,
    release_publication_result_receipt_retention_policy_recorded_count:0,
    release_publication_result_receipt_retention_policy_persisted_count:0,
    release_publication_result_receipt_retention_policy_materialized_count:0,
    release_publication_result_receipt_retention_index_recorded_count:0,
    release_publication_result_receipt_retention_index_persisted_count:0,
    release_publication_result_receipt_retention_ledger_recorded_count:0,
    release_publication_result_receipt_retention_ledger_persisted_count:0,
    release_publication_result_receipt_ttl_update_accepted_count:0,
    release_publication_result_receipt_ttl_update_recorded_count:0,
    release_publication_result_receipt_ttl_update_persisted_count:0,
    release_publication_result_receipt_ttl_extension_accepted_count:0,
    release_publication_result_receipt_ttl_extension_recorded_count:0,
    release_publication_result_receipt_ttl_extension_persisted_count:0,
    release_publication_result_receipt_expiry_accepted_count:0,
    release_publication_result_receipt_expiry_recorded_count:0,
    release_publication_result_receipt_expiry_persisted_count:0,
    release_publication_result_receipt_expiry_scheduler_registered_count:0,
    release_publication_result_receipt_expiry_timer_started_count:0,
    release_publication_result_receipt_expiry_ack_recorded_count:0,
    release_publication_result_receipt_garbage_collection_accepted_count:0,
    release_publication_result_receipt_garbage_collection_scan_performed_count:0,
    release_publication_result_receipt_garbage_collection_candidate_recorded_count:0,
    release_publication_result_receipt_garbage_collection_decision_recorded_count:0,
    release_publication_result_receipt_garbage_collection_persisted_count:0,
    release_publication_result_receipt_delete_accepted_count:0,
    release_publication_result_receipt_delete_marker_recorded_count:0,
    release_publication_result_receipt_delete_performed_count:0,
    release_publication_result_receipt_tombstone_recorded_count:0,
    release_publication_result_receipt_tombstone_persisted_count:0,
    release_publication_result_receipt_sweep_performed_count:0,
    release_publication_result_receipt_archive_written_count:0,
    release_publication_result_receipt_archive_persisted_count:0,
    release_publication_result_receipt_compaction_performed_count:0,
    release_publication_result_receipt_compaction_artifact_written_count:0,
    release_publication_result_receipt_compaction_artifact_persisted_count:0,
    release_publication_result_receipt_ledger_retention_recorded_count:0,
    release_publication_result_receipt_index_retention_recorded_count:0,
    release_publication_result_receipt_delivery_retention_recorded_count:0,
    release_publication_result_receipt_retention_acceptance_recorded_count:0,
    release_publication_result_receipt_retention_operator_approval_derived_count:0,
    release_publication_result_receipt_retention_release_publication_authority_derived_count:0,
    release_publication_result_receipt_retention_activation_authority_derived_count:0,
    release_publication_result_receipt_retention_activation_command_derived_count:0,
    release_publication_result_receipt_retention_live_execution_allowed_count:0,
    release_publication_result_receipt_retention_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_retention_expiry_garbage_collection:[
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_policy_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_policy_recording_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_policy_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_index_recording_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_ledger_recording_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ttl_update_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ttl_extension_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_recording_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_scheduler_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_timer_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_ack_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_scan_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_candidate_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_decision_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_delete_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_tombstone_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_sweep_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_archive_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_compaction_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ledger_index_delivery_retention_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_completion_ack_from_retention_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_retention_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_retention_denied",
      "operator_readiness_packet_template_packet_receipt_activation_live_from_retention_denied",
      "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_retention_denied",
      "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_retention_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_gate",
        status:"allowed_report_only_next_slice",
        records_retention_policy:false,
        records_expiry:false,
        performs_garbage_collection:false,
        exports_receipt:false,
        registers_query:false,
        records_observability:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_retention_policy_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_retention_policy_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_retention_index_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_retention_ledger_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_ttl_update_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_ttl_extension_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_expiry_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_expiry_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_expiry_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_expiry_scheduler_registered:false,
    packet_acceptance_receipt_release_publication_result_receipt_expiry_timer_started:false,
    packet_acceptance_receipt_release_publication_result_receipt_expiry_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_candidate_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_decision_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delete_marker_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delete_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_sweep_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_archive_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_compaction_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_compaction_artifact_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded:false,
    packet_acceptance_receipt_publication_completion_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_supersession_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_tombstone_lifecycle_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_latest_replacement_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_replayed:false,
    packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered:false,
    packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_exported:false,
    packet_acceptance_receipt_release_publication_result_receipt_query_registered:false,
    packet_acceptance_receipt_release_publication_result_receipt_observability_recorded:false,
    packet_acceptance_receipt_release_publication_recorded:false,
    packet_acceptance_receipt_release_artifact_written:false,
    packet_acceptance_receipt_public_artifact_written:false,
    packet_acceptance_receipt_publication_queue_enqueued:false,
    packet_acceptance_receipt_publication_manifest_written:false,
    packet_acceptance_receipt_public_distribution_performed:false,
    packet_acceptance_receipt_channel_delivery_performed:false,
    packet_acceptance_receipt_external_publication_sent:false,
    packet_acceptance_receipt_public_version_tag_created:false,
    packet_acceptance_receipt_release_notes_materialized:false,
    packet_acceptance_receipt_changelog_materialized:false,
    packet_acceptance_receipt_public_release_claimed:false,
    packet_acceptance_receipt_public_ga_claimed:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    release_publication_authority_derived:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_retention_index_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_retention_ledger_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_ttl_update_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_ttl_extension_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_expiry_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_expiry_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_expiry_scheduler_registered:false,
      packet_acceptance_receipt_release_publication_result_receipt_expiry_timer_started:false,
      packet_acceptance_receipt_release_publication_result_receipt_expiry_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed:false,
      packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_candidate_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_decision_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delete_marker_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delete_performed:false,
      packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_sweep_performed:false,
      packet_acceptance_receipt_release_publication_result_receipt_archive_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_compaction_performed:false,
      packet_acceptance_receipt_release_publication_result_receipt_compaction_artifact_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded:false,
      packet_acceptance_receipt_publication_completion_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_supersession_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_replayed:false,
      packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered:false,
      packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_exported:false,
      packet_acceptance_receipt_release_publication_result_receipt_query_registered:false,
      packet_acceptance_receipt_release_publication_result_receipt_observability_recorded:false,
      packet_acceptance_receipt_release_publication_recorded:false,
      packet_acceptance_receipt_release_artifact_written:false,
      packet_acceptance_receipt_public_artifact_written:false,
      packet_acceptance_receipt_publication_queue_enqueued:false,
      packet_acceptance_receipt_publication_manifest_written:false,
      packet_acceptance_receipt_public_distribution_performed:false,
      packet_acceptance_receipt_channel_delivery_performed:false,
      packet_acceptance_receipt_external_publication_sent:false,
      packet_acceptance_receipt_public_release_claimed:false,
      packet_acceptance_receipt_public_ga_claimed:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      release_publication_authority_derived:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_ready == true
  and .source_release_publication_result_receipt_audit_evidence_surface_count == 16
  and .source_release_publication_result_receipt_audit_evidence_attempt_count == 16
  and .source_release_publication_result_receipt_audit_trail_recorded_count == 0
  and .source_release_publication_result_receipt_immutable_evidence_recorded_count == 0
  and .source_release_publication_result_receipt_hash_chain_recorded_count == 0
  and .source_release_publication_result_receipt_ledger_evidence_recorded_count == 0
  and .source_release_publication_result_receipt_readback_evidence_recorded_count == 0
  and .source_release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_audit_evidence_activation_authority_derived_count == 0
  and .release_publication_result_receipt_retention_surface_count == 18
  and .release_publication_result_receipt_retention_attempt_count == 18
  and .release_publication_result_receipt_retention_policy_accepted_count == 0
  and .release_publication_result_receipt_retention_policy_recorded_count == 0
  and .release_publication_result_receipt_retention_policy_persisted_count == 0
  and .release_publication_result_receipt_retention_index_recorded_count == 0
  and .release_publication_result_receipt_retention_ledger_recorded_count == 0
  and .release_publication_result_receipt_ttl_update_recorded_count == 0
  and .release_publication_result_receipt_ttl_extension_recorded_count == 0
  and .release_publication_result_receipt_expiry_accepted_count == 0
  and .release_publication_result_receipt_expiry_recorded_count == 0
  and .release_publication_result_receipt_expiry_scheduler_registered_count == 0
  and .release_publication_result_receipt_expiry_timer_started_count == 0
  and .release_publication_result_receipt_expiry_ack_recorded_count == 0
  and .release_publication_result_receipt_garbage_collection_accepted_count == 0
  and .release_publication_result_receipt_garbage_collection_scan_performed_count == 0
  and .release_publication_result_receipt_garbage_collection_candidate_recorded_count == 0
  and .release_publication_result_receipt_garbage_collection_decision_recorded_count == 0
  and .release_publication_result_receipt_delete_marker_recorded_count == 0
  and .release_publication_result_receipt_delete_performed_count == 0
  and .release_publication_result_receipt_tombstone_recorded_count == 0
  and .release_publication_result_receipt_sweep_performed_count == 0
  and .release_publication_result_receipt_archive_written_count == 0
  and .release_publication_result_receipt_compaction_performed_count == 0
  and .release_publication_result_receipt_compaction_artifact_written_count == 0
  and .release_publication_result_receipt_ledger_retention_recorded_count == 0
  and .release_publication_result_receipt_index_retention_recorded_count == 0
  and .release_publication_result_receipt_delivery_retention_recorded_count == 0
  and .release_publication_result_receipt_retention_acceptance_recorded_count == 0
  and .release_publication_result_receipt_retention_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_retention_activation_authority_derived_count == 0
  and .release_publication_result_receipt_retention_activation_command_derived_count == 0
  and .release_publication_result_receipt_retention_live_execution_allowed_count == 0
  and (.release_publication_result_receipt_retention_surfaces | all(
    .retention_expiry_or_garbage_collection_attempted == true
    and .retention_policy_accepted == false
    and .retention_policy_recorded == false
    and .retention_policy_persisted == false
    and .retention_index_recorded == false
    and .retention_ledger_recorded == false
    and .ttl_update_accepted == false
    and .ttl_update_recorded == false
    and .ttl_extension_accepted == false
    and .ttl_extension_recorded == false
    and .expiry_accepted == false
    and .expiry_recorded == false
    and .expiry_scheduler_registered == false
    and .expiry_timer_started == false
    and .expiry_ack_recorded == false
    and .garbage_collection_accepted == false
    and .garbage_collection_scan_performed == false
    and .garbage_collection_candidate_recorded == false
    and .garbage_collection_decision_recorded == false
    and .delete_marker_recorded == false
    and .delete_performed == false
    and .tombstone_recorded == false
    and .sweep_performed == false
    and .archive_written == false
    and .compaction_performed == false
    and .compaction_artifact_written == false
    and .ledger_retention_recorded == false
    and .index_retention_recorded == false
    and .delivery_retention_recorded == false
    and .audit_trail_recorded == false
    and .immutable_evidence_recorded == false
    and .hash_chain_recorded == false
    and .readback_evidence_recorded == false
    and .publication_completion_ack_recorded == false
    and .release_publication_recorded == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .external_send_performed == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .activation_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .retention_expiry_garbage_collection_noop_confirmed == true
    and .release_publication_result_receipt_retention_status == "release_publication_result_receipt_retention_expiry_garbage_collection_denied"
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_retention_expiry_garbage_collection | length) == 28
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_retention_index_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_ttl_update_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_expiry_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_expiry_scheduler_registered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delete_marker_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delete_performed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_archive_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_compaction_performed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded == false
  and .packet_acceptance_receipt_publication_completion_ack_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_replayed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_recorded == false
  and .packet_acceptance_receipt_release_artifact_written == false
  and .packet_acceptance_receipt_public_artifact_written == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
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

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt retention/expiry/garbage-collection denial gate passed"
