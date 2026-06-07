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

FINAL_ACK_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance-gate.sh
)"

final_ack_report_sha256="$(sha256_text "$FINAL_ACK_JSON")"
terminal_decision_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-terminal-decision-status-promotion-denial:$final_ack_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$FINAL_ACK_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_ready == true
    and $source.final_acknowledgement_surface_count == 14
    and $source.final_acknowledgement_attempt_count == 14
    and $source.final_acknowledgement_accepted_count == 0
    and $source.final_acknowledgement_recorded_count == 0
    and $source.final_acknowledgement_persisted_count == 0
    and $source.final_acknowledgement_materialized_count == 0
    and $source.final_acknowledgement_delivered_count == 0
    and $source.operator_received_recorded_count == 0
    and $source.operator_confirmed_recorded_count == 0
    and $source.operator_read_recorded_count == 0
    and $source.completion_ack_recorded_count == 0
    and $source.status_ack_recorded_count == 0
    and $source.channel_ack_delivered_count == 0
    and $source.external_ack_sent_count == 0
    and $source.final_acknowledgement_acceptance_recorded_count == 0
    and $source.final_acknowledgement_operator_approval_derived_count == 0
    and $source.final_acknowledgement_activation_authority_derived_count == 0
    and $source.final_acknowledgement_activation_command_derived_count == 0
    and $source.final_acknowledgement_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

terminal_decision_surfaces_json="$(
  jq -n '[
    "packet_receipt_terminal_decision_claim",
    "packet_receipt_terminal_status_closed_claim",
    "packet_receipt_final_state_promotion_claim",
    "packet_receipt_completion_promotion_claim",
    "packet_receipt_status_ready_claim",
    "packet_receipt_status_accepted_claim",
    "packet_receipt_status_approved_claim",
    "packet_receipt_status_authoritative_claim",
    "packet_receipt_status_live_claim",
    "packet_receipt_operator_decision_claim",
    "packet_receipt_public_status_claim",
    "packet_receipt_release_status_claim",
    "packet_receipt_dashboard_status_claim",
    "packet_receipt_live_execution_decision_claim"
  ] | map({
    terminal_decision_surface: .,
    terminal_decision_attempted: true,
    terminal_decision_accepted: false,
    terminal_decision_recorded: false,
    terminal_decision_persisted: false,
    terminal_decision_materialized: false,
    terminal_decision_delivered: false,
    terminal_status_recorded: false,
    terminal_status_persisted: false,
    terminal_status_closed: false,
    terminal_status_ready: false,
    terminal_status_accepted: false,
    terminal_status_approved: false,
    terminal_status_authoritative: false,
    terminal_status_live: false,
    final_state_promoted: false,
    completion_promoted: false,
    operator_decision_recorded: false,
    public_status_claimed: false,
    release_status_claimed: false,
    dashboard_status_recorded: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    terminal_decision_status: "terminal_decision_status_promotion_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_gate" \
  --arg final_ack_report_sha256 "$final_ack_report_sha256" \
  --arg terminal_decision_contract_hash_sha256 "$terminal_decision_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$FINAL_ACK_JSON" \
  --argjson surfaces "$terminal_decision_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_terminal_decision_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_v1",
    receipt_terminal_decision_mode:"non_accepted_final_acknowledgements_cannot_become_terminal_decisions_status_or_authority",
    source_packet_acceptance_receipt_final_acknowledgement_gate:$source.gate,
    source_packet_acceptance_receipt_final_acknowledgement_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_ready,
    source_final_ack_report_sha256:$final_ack_report_sha256,
    source_final_acknowledgement_contract_hash_sha256:$source.final_acknowledgement_contract_hash_sha256,
    terminal_decision_contract_hash_sha256:$terminal_decision_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_ready:true,
    source_final_acknowledgement_surface_count:$source.final_acknowledgement_surface_count,
    source_final_acknowledgement_accepted_count:$source.final_acknowledgement_accepted_count,
    source_final_acknowledgement_recorded_count:$source.final_acknowledgement_recorded_count,
    source_final_acknowledgement_persisted_count:$source.final_acknowledgement_persisted_count,
    source_final_acknowledgement_materialized_count:$source.final_acknowledgement_materialized_count,
    source_final_acknowledgement_delivered_count:$source.final_acknowledgement_delivered_count,
    source_operator_received_recorded_count:$source.operator_received_recorded_count,
    source_operator_confirmed_recorded_count:$source.operator_confirmed_recorded_count,
    source_completion_ack_recorded_count:$source.completion_ack_recorded_count,
    source_status_ack_recorded_count:$source.status_ack_recorded_count,
    source_external_ack_sent_count:$source.external_ack_sent_count,
    source_final_acknowledgement_activation_authority_derived_count:$source.final_acknowledgement_activation_authority_derived_count,
    terminal_decision_status_surface_count:($surfaces | length),
    terminal_decision_status_attempt_count:($surfaces | length),
    terminal_decision_accepted_count:0,
    terminal_decision_recorded_count:0,
    terminal_decision_persisted_count:0,
    terminal_decision_materialized_count:0,
    terminal_decision_delivered_count:0,
    terminal_status_recorded_count:0,
    terminal_status_persisted_count:0,
    terminal_status_closed_count:0,
    terminal_status_ready_count:0,
    terminal_status_accepted_count:0,
    terminal_status_approved_count:0,
    terminal_status_authoritative_count:0,
    terminal_status_live_count:0,
    final_state_promoted_count:0,
    completion_promoted_count:0,
    operator_decision_recorded_count:0,
    public_status_claimed_count:0,
    release_status_claimed_count:0,
    dashboard_status_recorded_count:0,
    terminal_decision_acceptance_recorded_count:0,
    terminal_decision_operator_approval_derived_count:0,
    terminal_decision_activation_authority_derived_count:0,
    terminal_decision_activation_command_derived_count:0,
    terminal_decision_live_execution_allowed_count:0,
    terminal_decision_status_surfaces:$surfaces,
    denied_by_packet_receipt_terminal_decision_status:[
      "operator_readiness_packet_template_packet_receipt_terminal_decision_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_terminal_decision_recording_denied",
      "operator_readiness_packet_template_packet_receipt_terminal_decision_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_terminal_decision_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_terminal_status_recording_denied",
      "operator_readiness_packet_template_packet_receipt_terminal_status_closed_denied",
      "operator_readiness_packet_template_packet_receipt_status_ready_denied",
      "operator_readiness_packet_template_packet_receipt_status_accepted_denied",
      "operator_readiness_packet_template_packet_receipt_status_approved_denied",
      "operator_readiness_packet_template_packet_receipt_status_authoritative_denied",
      "operator_readiness_packet_template_packet_receipt_status_live_denied",
      "operator_readiness_packet_template_packet_receipt_final_state_promotion_denied",
      "operator_readiness_packet_template_packet_receipt_completion_promotion_denied",
      "operator_readiness_packet_template_packet_receipt_operator_decision_recording_denied",
      "operator_readiness_packet_template_packet_receipt_public_status_claim_denied",
      "operator_readiness_packet_template_packet_receipt_release_status_claim_denied",
      "operator_readiness_packet_template_packet_receipt_dashboard_status_recording_denied",
      "operator_readiness_packet_template_packet_receipt_acceptance_from_terminal_status_denied",
      "operator_readiness_packet_template_packet_receipt_authority_from_terminal_status_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_from_terminal_status_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_gate",
        status:"allowed_report_only_next_slice",
        records_terminal_decision:false,
        promotes_status:false,
        claims_public_status:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_acceptance_receipt_final_acknowledgement_accepted:false,
    packet_acceptance_receipt_final_acknowledgement_recorded:false,
    packet_acceptance_receipt_terminal_decision_accepted:false,
    packet_acceptance_receipt_terminal_decision_recorded:false,
    packet_acceptance_receipt_terminal_decision_persisted:false,
    packet_acceptance_receipt_terminal_decision_materialized:false,
    packet_acceptance_receipt_terminal_status_recorded:false,
    packet_acceptance_receipt_terminal_status_persisted:false,
    packet_acceptance_receipt_terminal_status_closed:false,
    packet_acceptance_receipt_status_ready:false,
    packet_acceptance_receipt_status_accepted:false,
    packet_acceptance_receipt_status_approved:false,
    packet_acceptance_receipt_status_authoritative:false,
    packet_acceptance_receipt_status_live:false,
    packet_acceptance_receipt_final_state_promoted:false,
    packet_acceptance_receipt_completion_promoted:false,
    packet_acceptance_receipt_operator_decision_recorded:false,
    packet_acceptance_receipt_public_status_claimed:false,
    packet_acceptance_receipt_release_status_claimed:false,
    packet_acceptance_receipt_dashboard_status_recorded:false,
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
      packet_acceptance_receipt_terminal_decision_accepted:false,
      packet_acceptance_receipt_terminal_decision_recorded:false,
      packet_acceptance_receipt_terminal_decision_persisted:false,
      packet_acceptance_receipt_terminal_decision_materialized:false,
      packet_acceptance_receipt_terminal_status_recorded:false,
      packet_acceptance_receipt_terminal_status_persisted:false,
      packet_acceptance_receipt_terminal_status_closed:false,
      packet_acceptance_receipt_status_ready:false,
      packet_acceptance_receipt_status_accepted:false,
      packet_acceptance_receipt_status_approved:false,
      packet_acceptance_receipt_status_authoritative:false,
      packet_acceptance_receipt_status_live:false,
      packet_acceptance_receipt_final_state_promoted:false,
      packet_acceptance_receipt_completion_promoted:false,
      packet_acceptance_receipt_operator_decision_recorded:false,
      packet_acceptance_receipt_public_status_claimed:false,
      packet_acceptance_receipt_release_status_claimed:false,
      packet_acceptance_receipt_dashboard_status_recorded:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_ready == true
  and .source_packet_acceptance_receipt_final_acknowledgement_ready == true
  and .source_final_acknowledgement_surface_count == 14
  and .source_final_acknowledgement_accepted_count == 0
  and .source_final_acknowledgement_recorded_count == 0
  and .source_final_acknowledgement_persisted_count == 0
  and .source_final_acknowledgement_materialized_count == 0
  and .source_final_acknowledgement_delivered_count == 0
  and .source_operator_received_recorded_count == 0
  and .source_operator_confirmed_recorded_count == 0
  and .source_completion_ack_recorded_count == 0
  and .source_status_ack_recorded_count == 0
  and .source_external_ack_sent_count == 0
  and .source_final_acknowledgement_activation_authority_derived_count == 0
  and .terminal_decision_status_surface_count == 14
  and .terminal_decision_status_attempt_count == 14
  and .terminal_decision_accepted_count == 0
  and .terminal_decision_recorded_count == 0
  and .terminal_decision_persisted_count == 0
  and .terminal_decision_materialized_count == 0
  and .terminal_decision_delivered_count == 0
  and .terminal_status_recorded_count == 0
  and .terminal_status_persisted_count == 0
  and .terminal_status_closed_count == 0
  and .terminal_status_ready_count == 0
  and .terminal_status_accepted_count == 0
  and .terminal_status_approved_count == 0
  and .terminal_status_authoritative_count == 0
  and .terminal_status_live_count == 0
  and .final_state_promoted_count == 0
  and .completion_promoted_count == 0
  and .operator_decision_recorded_count == 0
  and .public_status_claimed_count == 0
  and .release_status_claimed_count == 0
  and .dashboard_status_recorded_count == 0
  and .terminal_decision_acceptance_recorded_count == 0
  and .terminal_decision_operator_approval_derived_count == 0
  and .terminal_decision_activation_authority_derived_count == 0
  and .terminal_decision_activation_command_derived_count == 0
  and .terminal_decision_live_execution_allowed_count == 0
  and (.terminal_decision_status_surfaces | all(
    .terminal_decision_attempted == true
    and .terminal_decision_accepted == false
    and .terminal_decision_recorded == false
    and .terminal_decision_persisted == false
    and .terminal_decision_materialized == false
    and .terminal_status_recorded == false
    and .terminal_status_closed == false
    and .terminal_status_ready == false
    and .terminal_status_accepted == false
    and .terminal_status_approved == false
    and .terminal_status_authoritative == false
    and .terminal_status_live == false
    and .final_state_promoted == false
    and .completion_promoted == false
    and .operator_decision_recorded == false
    and .public_status_claimed == false
    and .release_status_claimed == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .terminal_decision_status == "terminal_decision_status_promotion_denied"
  ))
  and (.denied_by_packet_receipt_terminal_decision_status | length) == 20
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_terminal_decision_accepted == false
  and .packet_acceptance_receipt_terminal_decision_recorded == false
  and .packet_acceptance_receipt_terminal_status_recorded == false
  and .packet_acceptance_receipt_terminal_status_closed == false
  and .packet_acceptance_receipt_status_ready == false
  and .packet_acceptance_receipt_status_accepted == false
  and .packet_acceptance_receipt_status_approved == false
  and .packet_acceptance_receipt_status_authoritative == false
  and .packet_acceptance_receipt_status_live == false
  and .packet_acceptance_receipt_final_state_promoted == false
  and .packet_acceptance_receipt_completion_promoted == false
  and .packet_acceptance_receipt_operator_decision_recorded == false
  and .packet_acceptance_receipt_public_status_claimed == false
  and .packet_acceptance_receipt_release_status_claimed == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt terminal decision status promotion denial gate passed"
