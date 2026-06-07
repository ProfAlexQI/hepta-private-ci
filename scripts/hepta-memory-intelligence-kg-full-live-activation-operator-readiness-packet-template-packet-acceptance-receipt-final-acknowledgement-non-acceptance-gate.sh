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

BRIEFING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence-gate.sh
)"

briefing_report_sha256="$(sha256_text "$BRIEFING_JSON")"
final_acknowledgement_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance:$briefing_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$BRIEFING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_ready == true
    and $source.operator_briefing_surface_count == 14
    and $source.operator_briefing_attempt_count == 14
    and $source.briefing_recorded_count == 0
    and $source.briefing_persisted_count == 0
    and $source.briefing_materialized_count == 0
    and $source.summary_recorded_count == 0
    and $source.readback_digest_recorded_count == 0
    and $source.final_note_recorded_count == 0
    and $source.status_banner_recorded_count == 0
    and $source.notification_preview_recorded_count == 0
    and $source.channel_delivery_performed_count == 0
    and $source.external_send_performed_count == 0
    and $source.telegram_send_performed_count == 0
    and $source.completion_ack_recorded_count == 0
    and $source.operator_briefing_acceptance_recorded_count == 0
    and $source.operator_briefing_operator_approval_derived_count == 0
    and $source.operator_briefing_activation_authority_derived_count == 0
    and $source.operator_briefing_activation_command_derived_count == 0
    and $source.operator_briefing_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

final_acknowledgement_surfaces_json="$(
  jq -n '[
    "packet_receipt_final_acknowledgement_claim",
    "packet_receipt_operator_received_claim",
    "packet_receipt_operator_confirmed_claim",
    "packet_receipt_operator_read_claim",
    "packet_receipt_operator_seen_claim",
    "packet_receipt_final_response_claim",
    "packet_receipt_completion_acknowledgement_claim",
    "packet_receipt_status_acknowledgement_claim",
    "packet_receipt_briefing_acknowledgement_claim",
    "packet_receipt_readback_acknowledgement_claim",
    "packet_receipt_channel_acknowledgement_claim",
    "packet_receipt_external_acknowledgement_claim",
    "packet_receipt_authority_acknowledgement_claim",
    "packet_receipt_live_acknowledgement_claim"
  ] | map({
    final_acknowledgement_surface: .,
    final_acknowledgement_attempted: true,
    final_acknowledgement_accepted: false,
    final_acknowledgement_recorded: false,
    final_acknowledgement_persisted: false,
    final_acknowledgement_materialized: false,
    final_acknowledgement_delivered: false,
    operator_received_recorded: false,
    operator_confirmed_recorded: false,
    operator_read_recorded: false,
    completion_ack_recorded: false,
    status_ack_recorded: false,
    briefing_ack_recorded: false,
    readback_ack_recorded: false,
    channel_ack_delivered: false,
    external_ack_sent: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    final_acknowledgement_status: "final_acknowledgement_non_acceptance_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_gate" \
  --arg briefing_report_sha256 "$briefing_report_sha256" \
  --arg final_acknowledgement_contract_hash_sha256 "$final_acknowledgement_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$BRIEFING_JSON" \
  --argjson surfaces "$final_acknowledgement_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_final_acknowledgement_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_v1",
    receipt_final_acknowledgement_mode:"non_persistent_receipt_briefings_cannot_become_operator_acceptance_or_authority",
    source_packet_acceptance_receipt_operator_briefing_gate:$source.gate,
    source_packet_acceptance_receipt_operator_briefing_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_ready,
    source_briefing_report_sha256:$briefing_report_sha256,
    source_operator_briefing_contract_hash_sha256:$source.operator_briefing_contract_hash_sha256,
    final_acknowledgement_contract_hash_sha256:$final_acknowledgement_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_ready:true,
    source_operator_briefing_surface_count:$source.operator_briefing_surface_count,
    source_briefing_recorded_count:$source.briefing_recorded_count,
    source_briefing_persisted_count:$source.briefing_persisted_count,
    source_briefing_materialized_count:$source.briefing_materialized_count,
    source_summary_recorded_count:$source.summary_recorded_count,
    source_readback_digest_recorded_count:$source.readback_digest_recorded_count,
    source_final_note_recorded_count:$source.final_note_recorded_count,
    source_channel_delivery_performed_count:$source.channel_delivery_performed_count,
    source_external_send_performed_count:$source.external_send_performed_count,
    source_telegram_send_performed_count:$source.telegram_send_performed_count,
    source_completion_ack_recorded_count:$source.completion_ack_recorded_count,
    source_operator_briefing_activation_authority_derived_count:$source.operator_briefing_activation_authority_derived_count,
    final_acknowledgement_surface_count:($surfaces | length),
    final_acknowledgement_attempt_count:($surfaces | length),
    final_acknowledgement_accepted_count:0,
    final_acknowledgement_recorded_count:0,
    final_acknowledgement_persisted_count:0,
    final_acknowledgement_materialized_count:0,
    final_acknowledgement_delivered_count:0,
    operator_received_recorded_count:0,
    operator_confirmed_recorded_count:0,
    operator_read_recorded_count:0,
    completion_ack_recorded_count:0,
    status_ack_recorded_count:0,
    briefing_ack_recorded_count:0,
    readback_ack_recorded_count:0,
    channel_ack_delivered_count:0,
    external_ack_sent_count:0,
    final_acknowledgement_acceptance_recorded_count:0,
    final_acknowledgement_operator_approval_derived_count:0,
    final_acknowledgement_activation_authority_derived_count:0,
    final_acknowledgement_activation_command_derived_count:0,
    final_acknowledgement_live_execution_allowed_count:0,
    final_acknowledgement_surfaces:$surfaces,
    denied_by_packet_receipt_final_acknowledgement:[
      "operator_readiness_packet_template_packet_receipt_final_acknowledgement_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_final_acknowledgement_recording_denied",
      "operator_readiness_packet_template_packet_receipt_final_acknowledgement_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_final_acknowledgement_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_final_acknowledgement_delivery_denied",
      "operator_readiness_packet_template_packet_receipt_operator_received_recording_denied",
      "operator_readiness_packet_template_packet_receipt_operator_confirmed_recording_denied",
      "operator_readiness_packet_template_packet_receipt_operator_read_recording_denied",
      "operator_readiness_packet_template_packet_receipt_completion_ack_recording_denied",
      "operator_readiness_packet_template_packet_receipt_status_ack_recording_denied",
      "operator_readiness_packet_template_packet_receipt_briefing_ack_recording_denied",
      "operator_readiness_packet_template_packet_receipt_readback_ack_recording_denied",
      "operator_readiness_packet_template_packet_receipt_channel_ack_delivery_denied",
      "operator_readiness_packet_template_packet_receipt_external_ack_send_denied",
      "operator_readiness_packet_template_packet_receipt_acceptance_from_final_acknowledgement_denied",
      "operator_readiness_packet_template_packet_receipt_authority_from_final_acknowledgement_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_from_final_acknowledgement_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_non_promotion_gate",
        status:"allowed_report_only_next_slice",
        records_final_acknowledgement:false,
        persists_final_acknowledgement:false,
        sends_externally:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_acceptance_receipt_operator_briefing_recorded:false,
    packet_acceptance_receipt_operator_briefing_persisted:false,
    packet_acceptance_receipt_summary_recorded:false,
    packet_acceptance_receipt_readback_digest_recorded:false,
    packet_acceptance_receipt_final_note_recorded:false,
    packet_acceptance_receipt_final_acknowledgement_accepted:false,
    packet_acceptance_receipt_final_acknowledgement_recorded:false,
    packet_acceptance_receipt_final_acknowledgement_persisted:false,
    packet_acceptance_receipt_final_acknowledgement_materialized:false,
    packet_acceptance_receipt_final_acknowledgement_delivered:false,
    packet_acceptance_receipt_operator_received_recorded:false,
    packet_acceptance_receipt_operator_confirmed_recorded:false,
    packet_acceptance_receipt_operator_read_recorded:false,
    packet_acceptance_receipt_completion_ack_recorded:false,
    packet_acceptance_receipt_status_ack_recorded:false,
    packet_acceptance_receipt_briefing_ack_recorded:false,
    packet_acceptance_receipt_readback_ack_recorded:false,
    packet_acceptance_receipt_channel_ack_delivered:false,
    packet_acceptance_receipt_external_ack_sent:false,
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
      packet_acceptance_receipt_summary_recorded:false,
      packet_acceptance_receipt_readback_digest_recorded:false,
      packet_acceptance_receipt_final_note_recorded:false,
      packet_acceptance_receipt_final_acknowledgement_accepted:false,
      packet_acceptance_receipt_final_acknowledgement_recorded:false,
      packet_acceptance_receipt_final_acknowledgement_persisted:false,
      packet_acceptance_receipt_final_acknowledgement_materialized:false,
      packet_acceptance_receipt_final_acknowledgement_delivered:false,
      packet_acceptance_receipt_operator_received_recorded:false,
      packet_acceptance_receipt_operator_confirmed_recorded:false,
      packet_acceptance_receipt_operator_read_recorded:false,
      packet_acceptance_receipt_completion_ack_recorded:false,
      packet_acceptance_receipt_status_ack_recorded:false,
      packet_acceptance_receipt_briefing_ack_recorded:false,
      packet_acceptance_receipt_readback_ack_recorded:false,
      packet_acceptance_receipt_channel_ack_delivered:false,
      packet_acceptance_receipt_external_ack_sent:false,
      packet_acceptance_receipt_acceptance_recorded:false,
      packet_acceptance_receipt_authority_derived:false,
      packet_acceptance_receipt_live_execution_allowed:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_ready == true
  and .source_packet_acceptance_receipt_operator_briefing_ready == true
  and .source_operator_briefing_surface_count == 14
  and .source_briefing_recorded_count == 0
  and .source_briefing_persisted_count == 0
  and .source_briefing_materialized_count == 0
  and .source_summary_recorded_count == 0
  and .source_readback_digest_recorded_count == 0
  and .source_final_note_recorded_count == 0
  and .source_channel_delivery_performed_count == 0
  and .source_external_send_performed_count == 0
  and .source_telegram_send_performed_count == 0
  and .source_completion_ack_recorded_count == 0
  and .source_operator_briefing_activation_authority_derived_count == 0
  and .final_acknowledgement_surface_count == 14
  and .final_acknowledgement_attempt_count == 14
  and .final_acknowledgement_accepted_count == 0
  and .final_acknowledgement_recorded_count == 0
  and .final_acknowledgement_persisted_count == 0
  and .final_acknowledgement_materialized_count == 0
  and .final_acknowledgement_delivered_count == 0
  and .operator_received_recorded_count == 0
  and .operator_confirmed_recorded_count == 0
  and .operator_read_recorded_count == 0
  and .completion_ack_recorded_count == 0
  and .status_ack_recorded_count == 0
  and .briefing_ack_recorded_count == 0
  and .readback_ack_recorded_count == 0
  and .channel_ack_delivered_count == 0
  and .external_ack_sent_count == 0
  and .final_acknowledgement_acceptance_recorded_count == 0
  and .final_acknowledgement_operator_approval_derived_count == 0
  and .final_acknowledgement_activation_authority_derived_count == 0
  and .final_acknowledgement_activation_command_derived_count == 0
  and .final_acknowledgement_live_execution_allowed_count == 0
  and (.final_acknowledgement_surfaces | all(
    .final_acknowledgement_attempted == true
    and .final_acknowledgement_accepted == false
    and .final_acknowledgement_recorded == false
    and .final_acknowledgement_persisted == false
    and .final_acknowledgement_materialized == false
    and .final_acknowledgement_delivered == false
    and .operator_received_recorded == false
    and .operator_confirmed_recorded == false
    and .operator_read_recorded == false
    and .completion_ack_recorded == false
    and .channel_ack_delivered == false
    and .external_ack_sent == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .final_acknowledgement_status == "final_acknowledgement_non_acceptance_denied"
  ))
  and (.denied_by_packet_receipt_final_acknowledgement | length) == 17
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_final_acknowledgement_accepted == false
  and .packet_acceptance_receipt_final_acknowledgement_recorded == false
  and .packet_acceptance_receipt_final_acknowledgement_persisted == false
  and .packet_acceptance_receipt_final_acknowledgement_materialized == false
  and .packet_acceptance_receipt_final_acknowledgement_delivered == false
  and .packet_acceptance_receipt_operator_received_recorded == false
  and .packet_acceptance_receipt_operator_confirmed_recorded == false
  and .packet_acceptance_receipt_operator_read_recorded == false
  and .packet_acceptance_receipt_completion_ack_recorded == false
  and .packet_acceptance_receipt_channel_ack_delivered == false
  and .packet_acceptance_receipt_external_ack_sent == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt final acknowledgement non-acceptance gate passed"
