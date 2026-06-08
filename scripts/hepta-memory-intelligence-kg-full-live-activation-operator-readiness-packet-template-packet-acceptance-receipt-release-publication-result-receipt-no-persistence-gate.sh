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

RELEASE_PUBLICATION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-denial-gate.sh
)"

release_publication_report_sha256="$(sha256_text "$RELEASE_PUBLICATION_JSON")"
release_publication_result_receipt_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence:$release_publication_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$RELEASE_PUBLICATION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_ready == true
    and $source.source_packet_acceptance_receipt_terminal_decision_status_ready == true
    and $source.release_publication_surface_count == 14
    and $source.release_publication_attempt_count == 14
    and $source.release_publication_allowed_count == 0
    and $source.release_publication_accepted_count == 0
    and $source.release_publication_recorded_count == 0
    and $source.release_publication_persisted_count == 0
    and $source.release_publication_materialized_count == 0
    and $source.release_artifact_written_count == 0
    and $source.public_artifact_written_count == 0
    and $source.artifact_signature_accepted_count == 0
    and $source.artifact_notarization_accepted_count == 0
    and $source.publication_queue_enqueued_count == 0
    and $source.publication_manifest_written_count == 0
    and $source.public_distribution_performed_count == 0
    and $source.channel_delivery_performed_count == 0
    and $source.external_publication_sent_count == 0
    and $source.public_version_tag_created_count == 0
    and $source.release_notes_materialized_count == 0
    and $source.changelog_materialized_count == 0
    and $source.public_release_claimed_count == 0
    and $source.public_ga_claimed_count == 0
    and $source.terminal_status_release_approval_promoted_count == 0
    and $source.release_publication_acceptance_recorded_count == 0
    and $source.release_publication_operator_approval_derived_count == 0
    and $source.release_publication_activation_authority_derived_count == 0
    and $source.release_publication_activation_command_derived_count == 0
    and $source.release_publication_live_execution_allowed_count == 0
    and $source.packet_acceptance_receipt_release_publication_allowed == false
    and $source.packet_acceptance_receipt_release_publication_accepted == false
    and $source.packet_acceptance_receipt_release_publication_recorded == false
    and $source.packet_acceptance_receipt_release_publication_persisted == false
    and $source.packet_acceptance_receipt_release_publication_materialized == false
    and $source.packet_acceptance_receipt_release_artifact_written == false
    and $source.packet_acceptance_receipt_public_artifact_written == false
    and $source.packet_acceptance_receipt_artifact_signature_accepted == false
    and $source.packet_acceptance_receipt_artifact_notarization_accepted == false
    and $source.packet_acceptance_receipt_publication_queue_enqueued == false
    and $source.packet_acceptance_receipt_publication_manifest_written == false
    and $source.packet_acceptance_receipt_public_distribution_performed == false
    and $source.packet_acceptance_receipt_public_version_tag_created == false
    and $source.packet_acceptance_receipt_release_notes_materialized == false
    and $source.packet_acceptance_receipt_changelog_materialized == false
    and $source.packet_acceptance_receipt_public_release_claimed == false
    and $source.packet_acceptance_receipt_public_ga_claimed == false
    and $source.packet_acceptance_receipt_terminal_status_promoted_to_release_approval == false
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

release_publication_result_receipt_surfaces_json="$(
  jq -n '[
    "source_release_publication_report_required",
    "publication_result_receipt_recording_denied",
    "publication_result_receipt_persistence_denied",
    "publication_result_receipt_materialization_denied",
    "publication_result_receipt_filesystem_write_denied",
    "publication_result_receipt_ledger_index_denied",
    "publication_result_receipt_enqueue_delivery_denied",
    "publication_result_receipt_export_query_denied",
    "publication_result_receipt_observability_denied",
    "publication_result_receipt_hash_binding_denied",
    "publication_result_receipt_signature_timestamp_status_denied",
    "publication_completion_ack_denied",
    "publication_result_receipt_release_publication_authority_denied",
    "publication_result_receipt_activation_live_install_restart_active_binary_denied"
  ] | map({
    release_publication_result_receipt_surface: .,
    source_release_publication_report_present: true,
    source_release_publication_denial_ready: true,
    publication_result_receipt_attempted: true,
    publication_result_receipt_allowed: false,
    publication_result_receipt_accepted: false,
    publication_result_receipt_recorded: false,
    publication_result_receipt_persisted: false,
    publication_result_receipt_materialized: false,
    publication_result_receipt_filesystem_written: false,
    publication_result_receipt_ledger_written: false,
    publication_result_receipt_indexed: false,
    publication_result_receipt_enqueued: false,
    publication_result_receipt_delivered: false,
    publication_result_receipt_exported: false,
    publication_result_receipt_query_registered: false,
    publication_result_receipt_observability_recorded: false,
    publication_result_receipt_hash_bound: false,
    publication_result_receipt_signature_accepted: false,
    publication_result_receipt_timestamp_accepted: false,
    publication_result_receipt_status_accepted: false,
    publication_completion_ack_recorded: false,
    publication_completion_ack_persisted: false,
    publication_completion_ack_accepted: false,
    release_publication_recorded: false,
    release_publication_persisted: false,
    release_publication_materialized: false,
    release_artifact_written: false,
    public_artifact_written: false,
    artifact_signature_accepted: false,
    artifact_notarization_accepted: false,
    publication_queue_enqueued: false,
    publication_manifest_written: false,
    public_distribution_performed: false,
    channel_delivery_performed: false,
    external_send_performed: false,
    public_version_tag_created: false,
    release_notes_materialized: false,
    changelog_materialized: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    terminal_status_promoted_to_release_approval: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
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
    publication_result_receipt_noop_confirmed: true,
    release_publication_result_receipt_status: "release_publication_result_receipt_no_persistence_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_gate" \
  --arg release_publication_report_sha256 "$release_publication_report_sha256" \
  --arg release_publication_result_receipt_contract_hash_sha256 "$release_publication_result_receipt_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RELEASE_PUBLICATION_JSON" \
  --argjson surfaces "$release_publication_result_receipt_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_v1",
    receipt_release_publication_result_receipt_mode:"denied_release_publication_attempt_cannot_persist_result_receipt_or_derive_authority",
    source_packet_acceptance_receipt_release_publication_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_ready,
    source_packet_acceptance_receipt_release_publication_report_sha256:$release_publication_report_sha256,
    source_release_publication_contract_hash_sha256:$source.release_publication_contract_hash_sha256,
    release_publication_result_receipt_contract_hash_sha256:$release_publication_result_receipt_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_ready:true,
    source_release_publication_surface_count:$source.release_publication_surface_count,
    source_release_publication_attempt_count:$source.release_publication_attempt_count,
    source_release_publication_allowed_count:$source.release_publication_allowed_count,
    source_release_publication_accepted_count:$source.release_publication_accepted_count,
    source_release_publication_recorded_count:$source.release_publication_recorded_count,
    source_release_publication_persisted_count:$source.release_publication_persisted_count,
    source_release_artifact_written_count:$source.release_artifact_written_count,
    source_public_artifact_written_count:$source.public_artifact_written_count,
    source_public_distribution_performed_count:$source.public_distribution_performed_count,
    source_public_release_claimed_count:$source.public_release_claimed_count,
    source_public_ga_claimed_count:$source.public_ga_claimed_count,
    source_release_publication_activation_authority_derived_count:$source.release_publication_activation_authority_derived_count,
    release_publication_result_receipt_surface_count:($surfaces | length),
    release_publication_result_receipt_attempt_count:($surfaces | length),
    release_publication_result_receipt_allowed_count:0,
    release_publication_result_receipt_accepted_count:0,
    release_publication_result_receipt_recorded_count:0,
    release_publication_result_receipt_persisted_count:0,
    release_publication_result_receipt_materialized_count:0,
    release_publication_result_receipt_filesystem_written_count:0,
    release_publication_result_receipt_ledger_written_count:0,
    release_publication_result_receipt_indexed_count:0,
    release_publication_result_receipt_enqueued_count:0,
    release_publication_result_receipt_delivered_count:0,
    release_publication_result_receipt_exported_count:0,
    release_publication_result_receipt_query_registered_count:0,
    release_publication_result_receipt_observability_recorded_count:0,
    release_publication_result_receipt_hash_bound_count:0,
    release_publication_result_receipt_signature_accepted_count:0,
    release_publication_result_receipt_timestamp_accepted_count:0,
    release_publication_result_receipt_status_accepted_count:0,
    publication_completion_ack_recorded_count:0,
    publication_completion_ack_persisted_count:0,
    publication_completion_ack_accepted_count:0,
    release_publication_result_receipt_acceptance_recorded_count:0,
    release_publication_result_receipt_operator_approval_derived_count:0,
    release_publication_result_receipt_activation_authority_derived_count:0,
    release_publication_result_receipt_activation_command_derived_count:0,
    release_publication_result_receipt_live_execution_allowed_count:0,
    release_publication_result_receipt_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_no_persistence:[
      "operator_readiness_packet_template_packet_receipt_source_release_publication_report_required",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_recording_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_filesystem_write_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_ledger_write_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_index_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_enqueue_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_delivery_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_export_query_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_observability_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_hash_binding_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_signature_timestamp_status_denied",
      "operator_readiness_packet_template_packet_receipt_publication_completion_ack_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_authority_denied",
      "operator_readiness_packet_template_packet_receipt_activation_live_authority_denied",
      "operator_readiness_packet_template_packet_receipt_memory_provider_install_restart_active_binary_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_gate",
        status:"allowed_report_only_next_slice",
        records_release_publication_result_receipt:false,
        persists_release_publication_result_receipt:false,
        writes_release_artifact:false,
        writes_public_artifact:false,
        enqueues_publication:false,
        records_publication_completion_ack:false,
        claims_public_release:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_allowed:false,
    packet_acceptance_receipt_release_publication_accepted:false,
    packet_acceptance_receipt_release_publication_recorded:false,
    packet_acceptance_receipt_release_publication_persisted:false,
    packet_acceptance_receipt_release_publication_materialized:false,
    packet_acceptance_receipt_release_artifact_written:false,
    packet_acceptance_receipt_public_artifact_written:false,
    packet_acceptance_receipt_artifact_signature_accepted:false,
    packet_acceptance_receipt_artifact_notarization_accepted:false,
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
    packet_acceptance_receipt_terminal_status_promoted_to_release_approval:false,
    packet_acceptance_receipt_release_publication_result_receipt_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_ledger_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_indexed:false,
    packet_acceptance_receipt_release_publication_result_receipt_enqueued:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_exported:false,
    packet_acceptance_receipt_release_publication_result_receipt_query_registered:false,
    packet_acceptance_receipt_release_publication_result_receipt_observability_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_hash_bound:false,
    packet_acceptance_receipt_release_publication_result_receipt_signature_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_timestamp_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_status_accepted:false,
    packet_acceptance_receipt_publication_completion_ack_recorded:false,
    packet_acceptance_receipt_publication_completion_ack_persisted:false,
    packet_acceptance_receipt_publication_completion_ack_accepted:false,
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
      packet_acceptance_receipt_release_publication_allowed:false,
      packet_acceptance_receipt_release_publication_accepted:false,
      packet_acceptance_receipt_release_publication_recorded:false,
      packet_acceptance_receipt_release_publication_persisted:false,
      packet_acceptance_receipt_release_publication_materialized:false,
      packet_acceptance_receipt_release_artifact_written:false,
      packet_acceptance_receipt_public_artifact_written:false,
      packet_acceptance_receipt_artifact_signature_accepted:false,
      packet_acceptance_receipt_artifact_notarization_accepted:false,
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
      packet_acceptance_receipt_terminal_status_promoted_to_release_approval:false,
      packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_ledger_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_indexed:false,
      packet_acceptance_receipt_release_publication_result_receipt_enqueued:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_exported:false,
      packet_acceptance_receipt_release_publication_result_receipt_query_registered:false,
      packet_acceptance_receipt_release_publication_result_receipt_observability_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_hash_bound:false,
      packet_acceptance_receipt_release_publication_result_receipt_signature_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_timestamp_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_status_accepted:false,
      packet_acceptance_receipt_publication_completion_ack_recorded:false,
      packet_acceptance_receipt_publication_completion_ack_persisted:false,
      packet_acceptance_receipt_publication_completion_ack_accepted:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_ready == true
  and .source_packet_acceptance_receipt_release_publication_ready == true
  and .source_release_publication_surface_count == 14
  and .source_release_publication_attempt_count == 14
  and .source_release_publication_allowed_count == 0
  and .source_release_publication_accepted_count == 0
  and .source_release_publication_recorded_count == 0
  and .source_release_publication_persisted_count == 0
  and .source_release_artifact_written_count == 0
  and .source_public_artifact_written_count == 0
  and .source_public_distribution_performed_count == 0
  and .source_public_release_claimed_count == 0
  and .source_public_ga_claimed_count == 0
  and .source_release_publication_activation_authority_derived_count == 0
  and .release_publication_result_receipt_surface_count == 14
  and .release_publication_result_receipt_attempt_count == 14
  and .release_publication_result_receipt_allowed_count == 0
  and .release_publication_result_receipt_accepted_count == 0
  and .release_publication_result_receipt_recorded_count == 0
  and .release_publication_result_receipt_persisted_count == 0
  and .release_publication_result_receipt_materialized_count == 0
  and .release_publication_result_receipt_filesystem_written_count == 0
  and .release_publication_result_receipt_ledger_written_count == 0
  and .release_publication_result_receipt_indexed_count == 0
  and .release_publication_result_receipt_enqueued_count == 0
  and .release_publication_result_receipt_delivered_count == 0
  and .release_publication_result_receipt_exported_count == 0
  and .release_publication_result_receipt_query_registered_count == 0
  and .release_publication_result_receipt_observability_recorded_count == 0
  and .release_publication_result_receipt_hash_bound_count == 0
  and .release_publication_result_receipt_signature_accepted_count == 0
  and .release_publication_result_receipt_timestamp_accepted_count == 0
  and .release_publication_result_receipt_status_accepted_count == 0
  and .publication_completion_ack_recorded_count == 0
  and .publication_completion_ack_persisted_count == 0
  and .publication_completion_ack_accepted_count == 0
  and .release_publication_result_receipt_acceptance_recorded_count == 0
  and .release_publication_result_receipt_operator_approval_derived_count == 0
  and .release_publication_result_receipt_activation_authority_derived_count == 0
  and .release_publication_result_receipt_activation_command_derived_count == 0
  and .release_publication_result_receipt_live_execution_allowed_count == 0
  and (.release_publication_result_receipt_surfaces | all(
    .publication_result_receipt_attempted == true
    and .publication_result_receipt_allowed == false
    and .publication_result_receipt_accepted == false
    and .publication_result_receipt_recorded == false
    and .publication_result_receipt_persisted == false
    and .publication_result_receipt_filesystem_written == false
    and .publication_result_receipt_ledger_written == false
    and .publication_result_receipt_indexed == false
    and .publication_result_receipt_enqueued == false
    and .publication_result_receipt_delivered == false
    and .publication_result_receipt_exported == false
    and .publication_result_receipt_query_registered == false
    and .publication_result_receipt_observability_recorded == false
    and .publication_result_receipt_hash_bound == false
    and .publication_result_receipt_signature_accepted == false
    and .publication_result_receipt_timestamp_accepted == false
    and .publication_result_receipt_status_accepted == false
    and .publication_completion_ack_recorded == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .public_distribution_performed == false
    and .channel_delivery_performed == false
    and .external_send_performed == false
    and .public_release_claimed == false
    and .public_ga_claimed == false
    and .terminal_status_promoted_to_release_approval == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
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
    and .publication_result_receipt_noop_confirmed == true
    and .release_publication_result_receipt_status == "release_publication_result_receipt_no_persistence_denied"
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_no_persistence | length) == 17
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_allowed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_ledger_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_indexed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_enqueued == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_exported == false
  and .packet_acceptance_receipt_release_publication_result_receipt_query_registered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_observability_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_hash_bound == false
  and .packet_acceptance_receipt_release_publication_result_receipt_signature_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_timestamp_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_status_accepted == false
  and .packet_acceptance_receipt_publication_completion_ack_recorded == false
  and .packet_acceptance_receipt_publication_completion_ack_persisted == false
  and .packet_acceptance_receipt_publication_completion_ack_accepted == false
  and .packet_acceptance_receipt_release_publication_allowed == false
  and .packet_acceptance_receipt_release_publication_accepted == false
  and .packet_acceptance_receipt_release_publication_recorded == false
  and .packet_acceptance_receipt_release_artifact_written == false
  and .packet_acceptance_receipt_public_artifact_written == false
  and .packet_acceptance_receipt_publication_queue_enqueued == false
  and .packet_acceptance_receipt_publication_manifest_written == false
  and .packet_acceptance_receipt_public_distribution_performed == false
  and .packet_acceptance_receipt_channel_delivery_performed == false
  and .packet_acceptance_receipt_external_publication_sent == false
  and .packet_acceptance_receipt_public_version_tag_created == false
  and .packet_acceptance_receipt_release_notes_materialized == false
  and .packet_acceptance_receipt_changelog_materialized == false
  and .packet_acceptance_receipt_public_release_claimed == false
  and .packet_acceptance_receipt_public_ga_claimed == false
  and .packet_acceptance_receipt_terminal_status_promoted_to_release_approval == false
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

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt no-persistence gate passed"
