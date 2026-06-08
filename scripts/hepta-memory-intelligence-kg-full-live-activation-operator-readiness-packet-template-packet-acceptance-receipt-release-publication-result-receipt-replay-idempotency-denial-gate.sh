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

RESULT_RECEIPT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence-gate.sh
)"

result_receipt_report_sha256="$(sha256_text "$RESULT_RECEIPT_JSON")"
result_receipt_replay_idempotency_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial:$result_receipt_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$RESULT_RECEIPT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_ready == true
    and $source.release_publication_result_receipt_surface_count == 14
    and $source.release_publication_result_receipt_attempt_count == 14
    and $source.release_publication_result_receipt_allowed_count == 0
    and $source.release_publication_result_receipt_accepted_count == 0
    and $source.release_publication_result_receipt_recorded_count == 0
    and $source.release_publication_result_receipt_persisted_count == 0
    and $source.release_publication_result_receipt_materialized_count == 0
    and $source.release_publication_result_receipt_filesystem_written_count == 0
    and $source.release_publication_result_receipt_ledger_written_count == 0
    and $source.release_publication_result_receipt_indexed_count == 0
    and $source.release_publication_result_receipt_enqueued_count == 0
    and $source.release_publication_result_receipt_delivered_count == 0
    and $source.release_publication_result_receipt_exported_count == 0
    and $source.release_publication_result_receipt_query_registered_count == 0
    and $source.release_publication_result_receipt_observability_recorded_count == 0
    and $source.release_publication_result_receipt_hash_bound_count == 0
    and $source.release_publication_result_receipt_signature_accepted_count == 0
    and $source.release_publication_result_receipt_timestamp_accepted_count == 0
    and $source.release_publication_result_receipt_status_accepted_count == 0
    and $source.publication_completion_ack_recorded_count == 0
    and $source.publication_completion_ack_persisted_count == 0
    and $source.publication_completion_ack_accepted_count == 0
    and $source.release_publication_result_receipt_acceptance_recorded_count == 0
    and $source.release_publication_result_receipt_operator_approval_derived_count == 0
    and $source.release_publication_result_receipt_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_live_execution_allowed_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_allowed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_persisted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_materialized == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_filesystem_written == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_ledger_written == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_indexed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_enqueued == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_exported == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_query_registered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_observability_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_hash_bound == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_signature_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_timestamp_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_status_accepted == false
    and $source.packet_acceptance_receipt_publication_completion_ack_recorded == false
    and $source.packet_acceptance_receipt_publication_completion_ack_persisted == false
    and $source.packet_acceptance_receipt_publication_completion_ack_accepted == false
    and $source.packet_acceptance_receipt_release_publication_allowed == false
    and $source.packet_acceptance_receipt_release_publication_accepted == false
    and $source.packet_acceptance_receipt_release_publication_recorded == false
    and $source.packet_acceptance_receipt_release_artifact_written == false
    and $source.packet_acceptance_receipt_public_artifact_written == false
    and $source.packet_acceptance_receipt_publication_queue_enqueued == false
    and $source.packet_acceptance_receipt_publication_manifest_written == false
    and $source.packet_acceptance_receipt_public_distribution_performed == false
    and $source.packet_acceptance_receipt_channel_delivery_performed == false
    and $source.packet_acceptance_receipt_external_publication_sent == false
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

result_receipt_replay_surfaces_json="$(
  jq -n '[
    "publication_result_receipt_replay",
    "publication_result_receipt_duplicate_replay",
    "publication_result_receipt_retry_replay",
    "publication_result_receipt_idempotency_key_registration",
    "publication_result_receipt_idempotency_cache_write",
    "publication_result_receipt_cache_hit_promotion",
    "publication_result_receipt_hash_replay_binding",
    "publication_result_receipt_signature_timestamp_status_replay",
    "publication_result_receipt_query_result_replay",
    "publication_result_receipt_export_snapshot_replay",
    "publication_result_receipt_observability_snapshot_replay",
    "publication_completion_ack_replay",
    "publication_result_receipt_release_publication_authority_replay",
    "publication_result_receipt_activation_live_install_restart_active_binary_replay"
  ] | map({
    release_publication_result_receipt_replay_surface: .,
    source_release_publication_result_receipt_present: true,
    source_release_publication_result_receipt_no_persistence_ready: true,
    result_receipt_replay_attempted: true,
    result_receipt_replay_allowed: false,
    result_receipt_replay_accepted: false,
    result_receipt_replay_recorded: false,
    result_receipt_replay_persisted: false,
    result_receipt_replay_materialized: false,
    result_receipt_duplicate_accepted: false,
    result_receipt_retry_accepted: false,
    idempotency_key_registered: false,
    idempotency_cache_written: false,
    idempotency_cache_hit_promoted: false,
    replay_hash_bound: false,
    replay_signature_accepted: false,
    replay_timestamp_accepted: false,
    replay_status_accepted: false,
    query_result_replayed: false,
    export_snapshot_replayed: false,
    observability_snapshot_replayed: false,
    publication_completion_ack_replayed: false,
    publication_completion_ack_recorded: false,
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
    result_receipt_replay_noop_confirmed: true,
    release_publication_result_receipt_replay_status: "release_publication_result_receipt_replay_idempotency_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_gate" \
  --arg result_receipt_report_sha256 "$result_receipt_report_sha256" \
  --arg result_receipt_replay_idempotency_contract_hash_sha256 "$result_receipt_replay_idempotency_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RESULT_RECEIPT_JSON" \
  --argjson surfaces "$result_receipt_replay_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_replay_idempotency_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_v1",
    receipt_release_publication_result_receipt_replay_idempotency_mode:"denied_release_publication_result_receipt_cannot_replay_cache_or_derive_authority",
    source_packet_acceptance_receipt_release_publication_result_receipt_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_report_sha256:$result_receipt_report_sha256,
    source_release_publication_result_receipt_contract_hash_sha256:$source.release_publication_result_receipt_contract_hash_sha256,
    release_publication_result_receipt_replay_idempotency_contract_hash_sha256:$result_receipt_replay_idempotency_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_ready:true,
    source_release_publication_result_receipt_surface_count:$source.release_publication_result_receipt_surface_count,
    source_release_publication_result_receipt_attempt_count:$source.release_publication_result_receipt_attempt_count,
    source_release_publication_result_receipt_recorded_count:$source.release_publication_result_receipt_recorded_count,
    source_release_publication_result_receipt_persisted_count:$source.release_publication_result_receipt_persisted_count,
    source_release_publication_result_receipt_materialized_count:$source.release_publication_result_receipt_materialized_count,
    source_release_publication_result_receipt_delivered_count:$source.release_publication_result_receipt_delivered_count,
    source_release_publication_result_receipt_exported_count:$source.release_publication_result_receipt_exported_count,
    source_release_publication_result_receipt_query_registered_count:$source.release_publication_result_receipt_query_registered_count,
    source_release_publication_result_receipt_observability_recorded_count:$source.release_publication_result_receipt_observability_recorded_count,
    source_publication_completion_ack_recorded_count:$source.publication_completion_ack_recorded_count,
    source_release_publication_result_receipt_activation_authority_derived_count:$source.release_publication_result_receipt_activation_authority_derived_count,
    release_publication_result_receipt_replay_surface_count:($surfaces | length),
    release_publication_result_receipt_replay_attempt_count:($surfaces | length),
    release_publication_result_receipt_replay_allowed_count:0,
    release_publication_result_receipt_replay_accepted_count:0,
    release_publication_result_receipt_replay_recorded_count:0,
    release_publication_result_receipt_replay_persisted_count:0,
    release_publication_result_receipt_replay_materialized_count:0,
    release_publication_result_receipt_duplicate_accepted_count:0,
    release_publication_result_receipt_retry_accepted_count:0,
    release_publication_result_receipt_idempotency_key_registered_count:0,
    release_publication_result_receipt_idempotency_cache_written_count:0,
    release_publication_result_receipt_idempotency_cache_hit_promoted_count:0,
    release_publication_result_receipt_replay_hash_bound_count:0,
    release_publication_result_receipt_replay_signature_accepted_count:0,
    release_publication_result_receipt_replay_timestamp_accepted_count:0,
    release_publication_result_receipt_replay_status_accepted_count:0,
    release_publication_result_receipt_query_result_replayed_count:0,
    release_publication_result_receipt_export_snapshot_replayed_count:0,
    release_publication_result_receipt_observability_snapshot_replayed_count:0,
    publication_completion_ack_replayed_count:0,
    publication_completion_ack_recorded_count:0,
    release_publication_result_receipt_replay_acceptance_recorded_count:0,
    release_publication_result_receipt_replay_operator_approval_derived_count:0,
    release_publication_result_receipt_replay_release_publication_authority_derived_count:0,
    release_publication_result_receipt_replay_activation_authority_derived_count:0,
    release_publication_result_receipt_replay_activation_command_derived_count:0,
    release_publication_result_receipt_replay_live_execution_allowed_count:0,
    release_publication_result_receipt_replay_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency:[
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replay_recording_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replay_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replay_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_duplicate_replay_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retry_replay_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_idempotency_key_registration_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_idempotency_cache_write_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_cache_hit_promotion_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_hash_replay_binding_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_signature_timestamp_status_replay_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_export_observability_replay_denied",
      "operator_readiness_packet_template_packet_receipt_publication_completion_ack_replay_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_authority_replay_denied",
      "operator_readiness_packet_template_packet_receipt_activation_live_authority_replay_denied",
      "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_replay_denied",
      "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_replay_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_gate",
        status:"allowed_report_only_next_slice",
        records_release_publication_result_receipt_replay:false,
        registers_idempotency_key:false,
        writes_idempotency_cache:false,
        promotes_cache_hit:false,
        records_publication_completion_ack:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_replayed:false,
    packet_acceptance_receipt_release_publication_result_receipt_replay_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_replay_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_replay_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_replay_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_replay_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_duplicate_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_retry_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered:false,
    packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted:false,
    packet_acceptance_receipt_release_publication_result_receipt_replay_hash_bound:false,
    packet_acceptance_receipt_release_publication_result_receipt_replay_signature_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_replay_timestamp_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_replay_status_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_query_result_replayed:false,
    packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_replayed:false,
    packet_acceptance_receipt_release_publication_result_receipt_observability_snapshot_replayed:false,
    packet_acceptance_receipt_publication_completion_ack_replayed:false,
    packet_acceptance_receipt_publication_completion_ack_recorded:false,
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
    packet_acceptance_receipt_release_publication_allowed:false,
    packet_acceptance_receipt_release_publication_accepted:false,
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
    packet_acceptance_receipt_terminal_status_promoted_to_release_approval:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_replayed:false,
      packet_acceptance_receipt_release_publication_result_receipt_replay_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_replay_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_replay_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_duplicate_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_retry_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered:false,
      packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted:false,
      packet_acceptance_receipt_release_publication_result_receipt_replay_hash_bound:false,
      packet_acceptance_receipt_release_publication_result_receipt_replay_signature_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_replay_timestamp_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_replay_status_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_query_result_replayed:false,
      packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_replayed:false,
      packet_acceptance_receipt_release_publication_result_receipt_observability_snapshot_replayed:false,
      packet_acceptance_receipt_publication_completion_ack_replayed:false,
      packet_acceptance_receipt_publication_completion_ack_recorded:false,
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
      packet_acceptance_receipt_terminal_status_promoted_to_release_approval:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_ready == true
  and .source_release_publication_result_receipt_surface_count == 14
  and .source_release_publication_result_receipt_attempt_count == 14
  and .source_release_publication_result_receipt_recorded_count == 0
  and .source_release_publication_result_receipt_persisted_count == 0
  and .source_release_publication_result_receipt_materialized_count == 0
  and .source_release_publication_result_receipt_delivered_count == 0
  and .source_release_publication_result_receipt_exported_count == 0
  and .source_release_publication_result_receipt_query_registered_count == 0
  and .source_release_publication_result_receipt_observability_recorded_count == 0
  and .source_publication_completion_ack_recorded_count == 0
  and .source_release_publication_result_receipt_activation_authority_derived_count == 0
  and .release_publication_result_receipt_replay_surface_count == 14
  and .release_publication_result_receipt_replay_attempt_count == 14
  and .release_publication_result_receipt_replay_allowed_count == 0
  and .release_publication_result_receipt_replay_accepted_count == 0
  and .release_publication_result_receipt_replay_recorded_count == 0
  and .release_publication_result_receipt_replay_persisted_count == 0
  and .release_publication_result_receipt_replay_materialized_count == 0
  and .release_publication_result_receipt_duplicate_accepted_count == 0
  and .release_publication_result_receipt_retry_accepted_count == 0
  and .release_publication_result_receipt_idempotency_key_registered_count == 0
  and .release_publication_result_receipt_idempotency_cache_written_count == 0
  and .release_publication_result_receipt_idempotency_cache_hit_promoted_count == 0
  and .release_publication_result_receipt_replay_hash_bound_count == 0
  and .release_publication_result_receipt_replay_signature_accepted_count == 0
  and .release_publication_result_receipt_replay_timestamp_accepted_count == 0
  and .release_publication_result_receipt_replay_status_accepted_count == 0
  and .release_publication_result_receipt_query_result_replayed_count == 0
  and .release_publication_result_receipt_export_snapshot_replayed_count == 0
  and .release_publication_result_receipt_observability_snapshot_replayed_count == 0
  and .publication_completion_ack_replayed_count == 0
  and .publication_completion_ack_recorded_count == 0
  and .release_publication_result_receipt_replay_acceptance_recorded_count == 0
  and .release_publication_result_receipt_replay_operator_approval_derived_count == 0
  and .release_publication_result_receipt_replay_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_replay_activation_authority_derived_count == 0
  and .release_publication_result_receipt_replay_activation_command_derived_count == 0
  and .release_publication_result_receipt_replay_live_execution_allowed_count == 0
  and (.release_publication_result_receipt_replay_surfaces | all(
    .result_receipt_replay_attempted == true
    and .result_receipt_replay_allowed == false
    and .result_receipt_replay_accepted == false
    and .result_receipt_replay_recorded == false
    and .result_receipt_replay_persisted == false
    and .result_receipt_replay_materialized == false
    and .result_receipt_duplicate_accepted == false
    and .result_receipt_retry_accepted == false
    and .idempotency_key_registered == false
    and .idempotency_cache_written == false
    and .idempotency_cache_hit_promoted == false
    and .replay_hash_bound == false
    and .replay_signature_accepted == false
    and .replay_timestamp_accepted == false
    and .replay_status_accepted == false
    and .query_result_replayed == false
    and .export_snapshot_replayed == false
    and .observability_snapshot_replayed == false
    and .publication_completion_ack_replayed == false
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
    and .result_receipt_replay_noop_confirmed == true
    and .release_publication_result_receipt_replay_status == "release_publication_result_receipt_replay_idempotency_denied"
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency | length) == 16
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_replayed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_replay_allowed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_replay_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_replay_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_replay_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_duplicate_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_retry_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_replay_hash_bound == false
  and .packet_acceptance_receipt_release_publication_result_receipt_replay_signature_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_replay_timestamp_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_replay_status_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_query_result_replayed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_replayed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_observability_snapshot_replayed == false
  and .packet_acceptance_receipt_publication_completion_ack_replayed == false
  and .packet_acceptance_receipt_publication_completion_ack_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_exported == false
  and .packet_acceptance_receipt_release_publication_result_receipt_query_registered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_observability_recorded == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt replay/idempotency denial gate passed"
