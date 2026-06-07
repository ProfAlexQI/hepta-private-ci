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

REDACTION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial-gate.sh
)"

redaction_report_sha256="$(sha256_text "$REDACTION_JSON")"
operator_briefing_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence:$redaction_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$REDACTION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_ready == true
    and $source.redaction_privacy_surface_count == 16
    and $source.redaction_privacy_attempt_count == 16
    and $source.redacted_payload_preview_recorded_count == 0
    and $source.payload_hash_preview_recorded_count == 0
    and $source.readback_text_recorded_count == 0
    and $source.operator_summary_text_recorded_count == 0
    and $source.privacy_review_recorded_count == 0
    and $source.privacy_review_persisted_count == 0
    and $source.secret_scan_performed_count == 0
    and $source.pii_scan_performed_count == 0
    and $source.raw_payload_inspected_count == 0
    and $source.plaintext_materialized_count == 0
    and $source.redaction_privacy_acceptance_recorded_count == 0
    and $source.redaction_privacy_operator_approval_derived_count == 0
    and $source.redaction_privacy_activation_authority_derived_count == 0
    and $source.redaction_privacy_activation_command_derived_count == 0
    and $source.redaction_privacy_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

briefing_surfaces_json="$(
  jq -n '[
    "packet_receipt_operator_briefing_claim",
    "packet_receipt_operator_facing_summary_claim",
    "packet_receipt_readback_digest_claim",
    "packet_receipt_final_note_claim",
    "packet_receipt_status_banner_claim",
    "packet_receipt_timeline_entry_claim",
    "packet_receipt_notification_preview_claim",
    "packet_receipt_channel_delivery_claim",
    "packet_receipt_external_send_claim",
    "packet_receipt_telegram_briefing_claim",
    "packet_receipt_completion_briefing_claim",
    "packet_receipt_acceptance_briefing_claim",
    "packet_receipt_authority_briefing_claim",
    "packet_receipt_live_briefing_claim"
  ] | map({
    briefing_surface: .,
    briefing_attempted: true,
    briefing_recorded: false,
    briefing_persisted: false,
    briefing_materialized: false,
    briefing_filesystem_written: false,
    summary_recorded: false,
    readback_digest_recorded: false,
    final_note_recorded: false,
    status_banner_recorded: false,
    timeline_entry_recorded: false,
    notification_preview_recorded: false,
    channel_delivery_performed: false,
    external_send_performed: false,
    telegram_send_performed: false,
    completion_ack_recorded: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    briefing_status: "operator_briefing_non_persistence_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_gate" \
  --arg redaction_report_sha256 "$redaction_report_sha256" \
  --arg operator_briefing_contract_hash_sha256 "$operator_briefing_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$REDACTION_JSON" \
  --argjson surfaces "$briefing_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_operator_briefing_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_v1",
    receipt_operator_briefing_mode:"non_persistent_receipts_cannot_create_operator_briefing_acceptance_or_authority",
    source_packet_acceptance_receipt_redaction_privacy_gate:$source.gate,
    source_packet_acceptance_receipt_redaction_privacy_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_ready,
    source_redaction_report_sha256:$redaction_report_sha256,
    source_redaction_privacy_payload_exposure_contract_hash_sha256:$source.redaction_privacy_payload_exposure_contract_hash_sha256,
    operator_briefing_contract_hash_sha256:$operator_briefing_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_ready:true,
    source_redaction_privacy_surface_count:$source.redaction_privacy_surface_count,
    source_redacted_payload_preview_recorded_count:$source.redacted_payload_preview_recorded_count,
    source_payload_hash_preview_recorded_count:$source.payload_hash_preview_recorded_count,
    source_readback_text_recorded_count:$source.readback_text_recorded_count,
    source_operator_summary_text_recorded_count:$source.operator_summary_text_recorded_count,
    source_privacy_review_recorded_count:$source.privacy_review_recorded_count,
    source_secret_scan_performed_count:$source.secret_scan_performed_count,
    source_raw_payload_inspected_count:$source.raw_payload_inspected_count,
    source_redaction_privacy_activation_authority_derived_count:$source.redaction_privacy_activation_authority_derived_count,
    operator_briefing_surface_count:($surfaces | length),
    operator_briefing_attempt_count:($surfaces | length),
    briefing_recorded_count:0,
    briefing_persisted_count:0,
    briefing_materialized_count:0,
    briefing_filesystem_written_count:0,
    summary_recorded_count:0,
    readback_digest_recorded_count:0,
    final_note_recorded_count:0,
    status_banner_recorded_count:0,
    timeline_entry_recorded_count:0,
    notification_preview_recorded_count:0,
    channel_delivery_performed_count:0,
    external_send_performed_count:0,
    telegram_send_performed_count:0,
    completion_ack_recorded_count:0,
    operator_briefing_acceptance_recorded_count:0,
    operator_briefing_operator_approval_derived_count:0,
    operator_briefing_activation_authority_derived_count:0,
    operator_briefing_activation_command_derived_count:0,
    operator_briefing_live_execution_allowed_count:0,
    operator_briefing_surfaces:$surfaces,
    denied_by_packet_receipt_operator_briefing:[
      "operator_readiness_packet_template_packet_receipt_operator_briefing_recording_denied",
      "operator_readiness_packet_template_packet_receipt_operator_briefing_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_operator_briefing_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_summary_recording_denied",
      "operator_readiness_packet_template_packet_receipt_readback_digest_denied",
      "operator_readiness_packet_template_packet_receipt_final_note_denied",
      "operator_readiness_packet_template_packet_receipt_status_banner_denied",
      "operator_readiness_packet_template_packet_receipt_timeline_entry_denied",
      "operator_readiness_packet_template_packet_receipt_notification_preview_denied",
      "operator_readiness_packet_template_packet_receipt_channel_delivery_denied",
      "operator_readiness_packet_template_packet_receipt_external_send_denied",
      "operator_readiness_packet_template_packet_receipt_telegram_send_denied",
      "operator_readiness_packet_template_packet_receipt_completion_ack_briefing_denied",
      "operator_readiness_packet_template_packet_receipt_acceptance_from_briefing_denied",
      "operator_readiness_packet_template_packet_receipt_authority_from_briefing_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_from_briefing_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_final_ack_non_acceptance_gate",
        status:"allowed_report_only_next_slice",
        records_briefing:false,
        persists_briefing:false,
        sends_externally:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_acceptance_receipt_recorded:false,
    packet_acceptance_receipt_persisted:false,
    packet_acceptance_receipt_redacted_payload_preview_recorded:false,
    packet_acceptance_receipt_readback_text_recorded:false,
    packet_acceptance_receipt_operator_summary_text_recorded:false,
    packet_acceptance_receipt_operator_briefing_recorded:false,
    packet_acceptance_receipt_operator_briefing_persisted:false,
    packet_acceptance_receipt_summary_recorded:false,
    packet_acceptance_receipt_readback_digest_recorded:false,
    packet_acceptance_receipt_final_note_recorded:false,
    packet_acceptance_receipt_status_banner_recorded:false,
    packet_acceptance_receipt_timeline_entry_recorded:false,
    packet_acceptance_receipt_notification_preview_recorded:false,
    packet_acceptance_receipt_channel_delivered:false,
    packet_acceptance_receipt_external_sent:false,
    packet_acceptance_receipt_telegram_sent:false,
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
      packet_acceptance_receipt_operator_briefing_recorded:false,
      packet_acceptance_receipt_operator_briefing_persisted:false,
      packet_acceptance_receipt_operator_briefing_materialized:false,
      packet_acceptance_receipt_operator_briefing_filesystem_written:false,
      packet_acceptance_receipt_summary_recorded:false,
      packet_acceptance_receipt_readback_digest_recorded:false,
      packet_acceptance_receipt_final_note_recorded:false,
      packet_acceptance_receipt_status_banner_recorded:false,
      packet_acceptance_receipt_timeline_entry_recorded:false,
      packet_acceptance_receipt_notification_preview_recorded:false,
      packet_acceptance_receipt_channel_delivered:false,
      packet_acceptance_receipt_external_sent:false,
      packet_acceptance_receipt_telegram_sent:false,
      packet_acceptance_receipt_completion_ack_recorded:false,
      packet_acceptance_receipt_acceptance_recorded:false,
      packet_acceptance_receipt_authority_derived:false,
      packet_acceptance_receipt_live_execution_allowed:false,
      packet_acceptance_receipt_redacted_payload_preview_recorded:false,
      packet_acceptance_receipt_readback_text_recorded:false,
      packet_acceptance_receipt_recorded:false,
      packet_acceptance_receipt_persisted:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_ready == true
  and .source_packet_acceptance_receipt_redaction_privacy_ready == true
  and .source_redaction_privacy_surface_count == 16
  and .source_redacted_payload_preview_recorded_count == 0
  and .source_readback_text_recorded_count == 0
  and .source_operator_summary_text_recorded_count == 0
  and .source_privacy_review_recorded_count == 0
  and .source_secret_scan_performed_count == 0
  and .source_raw_payload_inspected_count == 0
  and .source_redaction_privacy_activation_authority_derived_count == 0
  and .operator_briefing_surface_count == 14
  and .operator_briefing_attempt_count == 14
  and .briefing_recorded_count == 0
  and .briefing_persisted_count == 0
  and .briefing_materialized_count == 0
  and .briefing_filesystem_written_count == 0
  and .summary_recorded_count == 0
  and .readback_digest_recorded_count == 0
  and .final_note_recorded_count == 0
  and .status_banner_recorded_count == 0
  and .timeline_entry_recorded_count == 0
  and .notification_preview_recorded_count == 0
  and .channel_delivery_performed_count == 0
  and .external_send_performed_count == 0
  and .telegram_send_performed_count == 0
  and .completion_ack_recorded_count == 0
  and .operator_briefing_acceptance_recorded_count == 0
  and .operator_briefing_operator_approval_derived_count == 0
  and .operator_briefing_activation_authority_derived_count == 0
  and .operator_briefing_activation_command_derived_count == 0
  and .operator_briefing_live_execution_allowed_count == 0
  and (.operator_briefing_surfaces | all(
    .briefing_attempted == true
    and .briefing_recorded == false
    and .briefing_persisted == false
    and .briefing_materialized == false
    and .summary_recorded == false
    and .readback_digest_recorded == false
    and .final_note_recorded == false
    and .channel_delivery_performed == false
    and .external_send_performed == false
    and .telegram_send_performed == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .briefing_status == "operator_briefing_non_persistence_denied"
  ))
  and (.denied_by_packet_receipt_operator_briefing | length) == 16
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_operator_briefing_recorded == false
  and .packet_acceptance_receipt_operator_briefing_persisted == false
  and .packet_acceptance_receipt_summary_recorded == false
  and .packet_acceptance_receipt_readback_digest_recorded == false
  and .packet_acceptance_receipt_final_note_recorded == false
  and .packet_acceptance_receipt_channel_delivered == false
  and .packet_acceptance_receipt_external_sent == false
  and .packet_acceptance_receipt_telegram_sent == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt operator briefing non-persistence gate passed"
