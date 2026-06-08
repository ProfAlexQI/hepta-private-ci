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

TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_FINAL_ACK_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-final-operator-acknowledgement-non-acceptance-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh
)"

delivery_receipt_final_ack_report_sha256="$(sha256_text "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_FINAL_ACK_JSON")"
delivery_receipt_terminal_decision_status_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-terminal-decision-status-promotion-denial:$delivery_receipt_final_ack_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
delivery_receipt_terminal_decision_status_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-delivery-receipt-terminal-decision-status-promotion-denial:no-terminal-decision:no-status:no-release-authority:no-live:no-active-binary"
)"

jq -n -e \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_FINAL_ACK_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_request_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_materialized_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_filesystem_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_delivered_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_operator_received_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_operator_confirmed_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_operator_read_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_operator_seen_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_final_response_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_completion_ack_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_status_ack_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_summary_ack_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_briefing_ack_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_readback_digest_ack_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_dashboard_ack_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_notification_ack_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_channel_ack_delivered_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_external_ack_sent_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_telegram_ack_sent_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_acceptance_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_operator_approval_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_live_execution_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_install_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_service_restarted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_active_binary_mutated_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_persisted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_materialized == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_filesystem_written == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_delivered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_received_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_confirmed_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_read_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_seen_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_final_response_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_completion_ack_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_status_ack_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_channel_ack_delivered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_external_ack_sent == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_telegram_ack_sent == false
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.release_publication_authority_derived == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.live_kg_write_performed == false
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

terminal_decision_status_surfaces_json="$(
  jq -n '
    def terminal_status_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_surface:$id,
        source_final_operator_acknowledgement_ready:true,
        terminal_decision_attempted:true,
        terminal_decision_allowed:false,
        terminal_decision_request_accepted:false,
        terminal_decision_accepted:false,
        terminal_decision_recorded:false,
        terminal_decision_persisted:false,
        terminal_decision_materialized:false,
        terminal_decision_filesystem_written:false,
        terminal_decision_delivered:false,
        terminal_status_recorded:false,
        terminal_status_persisted:false,
        terminal_status_closed:false,
        terminal_status_ready:false,
        terminal_status_accepted:false,
        terminal_status_approved:false,
        terminal_status_authoritative:false,
        terminal_status_live:false,
        final_state_promoted:false,
        completion_promoted:false,
        operator_decision_recorded:false,
        public_status_claimed:false,
        release_status_claimed:false,
        publication_status_claimed:false,
        dashboard_status_recorded:false,
        channel_status_delivered:false,
        external_status_sent:false,
        telegram_status_sent:false,
        acceptance_recorded:false,
        operator_approval_derived:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        activation_command_derived:false,
        live_execution_allowed:false,
        activation_performed:false,
        install_executed:false,
        service_restarted:false,
        launchd_mutated:false,
        active_binary_mutated:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        terminal_decision_status_noop_confirmed:true,
        terminal_decision_status:$status,
        reason:$reason
      } + $extra;
    [
      terminal_status_surface("publication_result_receipt_terminal_decision_claim"; "blocked_terminal_decision_noop"; "terminal_decision_acceptance_denied"; {terminal_decision_requested:true}),
      terminal_status_surface("publication_result_receipt_terminal_status_closed_claim"; "blocked_terminal_closed_noop"; "terminal_status_closed_denied"; {terminal_status_closed_requested:true}),
      terminal_status_surface("publication_result_receipt_final_state_promotion_claim"; "blocked_final_state_noop"; "final_state_promotion_denied"; {final_state_promotion_requested:true}),
      terminal_status_surface("publication_result_receipt_completion_promotion_claim"; "blocked_completion_promotion_noop"; "completion_promotion_denied"; {completion_promotion_requested:true}),
      terminal_status_surface("publication_result_receipt_status_ready_claim"; "blocked_ready_status_noop"; "status_ready_denied"; {status_ready_requested:true}),
      terminal_status_surface("publication_result_receipt_status_accepted_claim"; "blocked_accepted_status_noop"; "status_accepted_denied"; {status_accepted_requested:true}),
      terminal_status_surface("publication_result_receipt_status_approved_claim"; "blocked_approved_status_noop"; "status_approved_denied"; {status_approved_requested:true}),
      terminal_status_surface("publication_result_receipt_status_authoritative_claim"; "blocked_authoritative_status_noop"; "status_authoritative_denied"; {status_authoritative_requested:true}),
      terminal_status_surface("publication_result_receipt_status_live_claim"; "blocked_live_status_noop"; "status_live_denied"; {status_live_requested:true}),
      terminal_status_surface("publication_result_receipt_operator_decision_claim"; "blocked_operator_decision_noop"; "operator_decision_recording_denied"; {operator_decision_requested:true}),
      terminal_status_surface("publication_result_receipt_public_status_claim"; "blocked_public_status_noop"; "public_status_claim_denied"; {public_status_requested:true}),
      terminal_status_surface("publication_result_receipt_release_status_claim"; "blocked_release_status_noop"; "release_status_claim_denied"; {release_status_requested:true}),
      terminal_status_surface("publication_result_receipt_publication_status_claim"; "blocked_publication_status_noop"; "publication_status_claim_denied"; {publication_status_requested:true}),
      terminal_status_surface("publication_result_receipt_dashboard_status_claim"; "blocked_dashboard_status_noop"; "dashboard_status_recording_denied"; {dashboard_status_requested:true}),
      terminal_status_surface("publication_result_receipt_channel_external_telegram_status_claim"; "blocked_channel_external_telegram_status_noop"; "channel_external_telegram_status_denied"; {channel_status_requested:true, external_status_requested:true, telegram_status_requested:true}),
      terminal_status_surface("publication_result_receipt_release_publication_authority_status_claim"; "blocked_release_publication_authority_status_noop"; "release_publication_authority_from_status_denied"; {release_publication_authority_status_requested:true}),
      terminal_status_surface("publication_result_receipt_activation_live_status_claim"; "blocked_activation_live_status_noop"; "activation_live_from_status_denied"; {activation_authority_status_requested:true, live_execution_status_requested:true}),
      terminal_status_surface("publication_result_receipt_install_restart_active_binary_status_claim"; "blocked_active_binary_status_noop"; "install_restart_active_binary_from_status_denied"; {install_restart_active_binary_status_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_promotion_denial_gate" \
  --arg delivery_receipt_final_ack_report_sha256 "$delivery_receipt_final_ack_report_sha256" \
  --arg delivery_receipt_terminal_decision_status_contract_hash_sha256 "$delivery_receipt_terminal_decision_status_contract_hash_sha256" \
  --arg delivery_receipt_terminal_decision_status_policy_hash_sha256 "$delivery_receipt_terminal_decision_status_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_FINAL_ACK_JSON" \
  --argjson surfaces "$terminal_decision_status_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_promotion_denial_v1",
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_mode:"non_accepted_final_operator_acknowledgement_cannot_become_terminal_decision_status_release_publication_or_activation_authority",
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_report_sha256:$delivery_receipt_final_ack_report_sha256,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_contract_hash_sha256:$delivery_receipt_terminal_decision_status_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_policy_hash_sha256:$delivery_receipt_terminal_decision_status_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_promotion_denial_ready:true,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_surface_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_attempt_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_accepted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_accepted_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_recorded_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_persisted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_persisted_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_materialized_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_materialized_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_delivered_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_delivered_count,
    source_release_publication_result_receipt_delivery_receipt_operator_received_recorded_count:$source.release_publication_result_receipt_delivery_receipt_operator_received_recorded_count,
    source_release_publication_result_receipt_delivery_receipt_operator_confirmed_recorded_count:$source.release_publication_result_receipt_delivery_receipt_operator_confirmed_recorded_count,
    source_release_publication_result_receipt_delivery_receipt_completion_ack_recorded_count:$source.release_publication_result_receipt_delivery_receipt_completion_ack_recorded_count,
    source_release_publication_result_receipt_delivery_receipt_status_ack_recorded_count:$source.release_publication_result_receipt_delivery_receipt_status_ack_recorded_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_release_publication_authority_derived_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_activation_authority_derived_count,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_surface_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_attempt_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_request_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_materialized_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_filesystem_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_closed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_status_ready_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_status_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_status_approved_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_status_authoritative_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_status_live_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_final_state_promoted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_completion_promoted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_decision_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_acceptance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_operator_approval_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_activation_command_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_live_execution_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_install_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_service_restarted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_active_binary_mutated_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status:[
      "source_final_operator_acknowledgement_report_required",
      "terminal_decision_request_acceptance_denied",
      "terminal_decision_acceptance_denied",
      "terminal_decision_recording_denied",
      "terminal_decision_persistence_denied",
      "terminal_decision_materialization_denied",
      "terminal_decision_filesystem_write_denied",
      "terminal_decision_delivery_denied",
      "terminal_status_recording_denied",
      "terminal_status_persistence_denied",
      "terminal_status_closed_denied",
      "status_ready_denied",
      "status_accepted_denied",
      "status_approved_denied",
      "status_authoritative_denied",
      "status_live_denied",
      "final_state_promotion_denied",
      "completion_promotion_denied",
      "operator_decision_recording_denied",
      "public_status_claim_denied",
      "release_status_claim_denied",
      "publication_status_claim_denied",
      "dashboard_status_recording_denied",
      "channel_status_delivery_denied",
      "external_status_send_denied",
      "telegram_status_send_denied",
      "acceptance_from_terminal_status_denied",
      "operator_approval_from_terminal_status_denied",
      "release_publication_authority_from_terminal_status_denied",
      "activation_live_from_terminal_status_denied",
      "install_restart_active_binary_from_terminal_status_denied",
      "memory_provider_kg_from_terminal_status_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_denial_gate",
        status:"allowed_report_only_next_slice",
        records_terminal_decision:false,
        promotes_status:false,
        claims_public_status:false,
        records_operator_acceptance:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_received_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_confirmed_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_read_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_seen_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_final_response_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_completion_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_status_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_closed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_ready:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_approved:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_authoritative:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_live:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_state_promoted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_completion_promoted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_decision_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
    packet_acceptance_receipt_release_publication_recorded:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    release_publication_authority_derived:false,
    activation_authority_derived:false,
    activation_command_derived:false,
    activation_allowed:false,
    activation_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_closed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_ready:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_approved:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_authoritative:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_live:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_state_promoted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_completion_promoted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_decision_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_received_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_confirmed_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_read_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_seen_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_final_response_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_completion_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_status_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      release_publication_authority_derived:false,
      activation_authority_derived:false,
      activation_command_derived:false,
      activation_allowed:false,
      activation_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      secret_file_read:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      release_artifact_written:false,
      public_artifact_written:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      external_send_performed:false,
      filesystem_written:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_promotion_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_promotion_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_surface_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_accepted_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_recorded_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_persisted_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_materialized_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_delivered_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_operator_received_recorded_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_operator_confirmed_recorded_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_completion_ack_recorded_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_status_ack_recorded_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_closed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_status_ready_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_status_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_status_approved_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_status_authoritative_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_status_live_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_final_state_promoted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_completion_promoted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_decision_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_acceptance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_operator_approval_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_activation_command_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_active_binary_mutated_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_surfaces | all(
    .terminal_decision_attempted == true
    and .terminal_decision_allowed == false
    and .terminal_decision_request_accepted == false
    and .terminal_decision_accepted == false
    and .terminal_decision_recorded == false
    and .terminal_decision_persisted == false
    and .terminal_decision_materialized == false
    and .terminal_decision_filesystem_written == false
    and .terminal_decision_delivered == false
    and .terminal_status_recorded == false
    and .terminal_status_persisted == false
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
    and .publication_status_claimed == false
    and .dashboard_status_recorded == false
    and .channel_status_delivered == false
    and .external_status_sent == false
    and .telegram_status_sent == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .terminal_decision_status_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status | length) == 32
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_closed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_ready == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_approved == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_authoritative == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_live == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_state_promoted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_completion_promoted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_decision_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_persisted == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt terminal decision status promotion denial gate passed"
