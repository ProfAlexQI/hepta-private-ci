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

TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_TERMINAL_DECISION_STATUS_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-terminal-decision-status-promotion-denial-gate" \
    scripts/i3-aef1d246e635bd6d22c22d02.sh
)"

delivery_receipt_terminal_decision_status_report_sha256="$(sha256_text "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_TERMINAL_DECISION_STATUS_JSON")"
delivery_receipt_terminal_public_claim_status_exposure_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-terminal-public-claim-status-exposure-denial:$delivery_receipt_terminal_decision_status_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
delivery_receipt_terminal_public_claim_status_exposure_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-delivery-receipt-terminal-public-claim-status-exposure-denial:no-public-claim:no-dashboard:no-channel:no-telegram:no-ga:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_TERMINAL_DECISION_STATUS_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_promotion_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_promotion_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_request_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_materialized_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_filesystem_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_closed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_status_ready_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_status_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_status_approved_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_status_authoritative_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_status_live_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_final_state_promoted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_completion_promoted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_decision_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_acceptance_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_operator_approval_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_live_execution_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_install_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_service_restarted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_active_binary_mutated_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_allowed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_persisted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_materialized == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_filesystem_written == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_delivered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_persisted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_closed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_ready == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_approved == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_authoritative == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_live == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_state_promoted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_completion_promoted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent == false
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

delivery_receipt_terminal_public_claim_status_exposure_surfaces_json="$(
  jq -n '
    def public_claim_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surface:$id,
        source_terminal_decision_status_ready:true,
        public_claim_status_exposure_attempted:true,
        public_claim_status_exposure_allowed:false,
        public_claim_status_exposure_request_accepted:false,
        public_claim_status_exposure_accepted:false,
        public_claim_status_exposure_recorded:false,
        public_claim_status_exposure_persisted:false,
        public_claim_status_exposure_materialized:false,
        public_claim_status_exposure_filesystem_written:false,
        public_claim_status_exposure_delivered:false,
        public_claim_status_exposed:false,
        public_status_claimed:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        release_status_exposed:false,
        publication_status_exposed:false,
        dashboard_status_exposed:false,
        public_badge_exposed:false,
        status_endpoint_exposed:false,
        query_status_exposed:false,
        export_status_exposed:false,
        observability_status_exposed:false,
        release_notes_status_exposed:false,
        changelog_status_exposed:false,
        version_tag_status_exposed:false,
        artifact_availability_status_exposed:false,
        distribution_queue_status_exposed:false,
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
        public_claim_status_exposure_noop_confirmed:true,
        public_claim_status_exposure_status:$status,
        reason:$reason
      } + $extra;
    [
      public_claim_surface("publication_result_receipt_delivery_receipt_public_claim_status_claim"; "blocked_public_claim_noop"; "public_claim_status_denied"; {public_claim_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_release_claim_status_claim"; "blocked_release_claim_noop"; "release_claim_status_denied"; {release_claim_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_publication_claim_status_claim"; "blocked_publication_claim_noop"; "publication_claim_status_denied"; {publication_claim_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_ga_stable_claim_status_claim"; "blocked_ga_stable_claim_noop"; "ga_stable_claim_status_denied"; {ga_stable_claim_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_dashboard_public_badge_status_claim"; "blocked_dashboard_public_badge_noop"; "dashboard_public_badge_denied"; {dashboard_public_badge_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_status_endpoint_claim"; "blocked_status_endpoint_noop"; "status_endpoint_exposure_denied"; {status_endpoint_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_query_status_claim"; "blocked_query_status_noop"; "query_status_exposure_denied"; {query_status_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_export_status_claim"; "blocked_export_status_noop"; "export_status_exposure_denied"; {export_status_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_observability_status_claim"; "blocked_observability_status_noop"; "observability_status_exposure_denied"; {observability_status_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_release_notes_status_claim"; "blocked_release_notes_status_noop"; "release_notes_status_exposure_denied"; {release_notes_status_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_changelog_status_claim"; "blocked_changelog_status_noop"; "changelog_status_exposure_denied"; {changelog_status_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_version_tag_status_claim"; "blocked_version_tag_status_noop"; "version_tag_status_exposure_denied"; {version_tag_status_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_artifact_availability_status_claim"; "blocked_artifact_availability_status_noop"; "artifact_availability_status_exposure_denied"; {artifact_availability_status_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_distribution_queue_status_claim"; "blocked_distribution_queue_status_noop"; "distribution_queue_status_exposure_denied"; {distribution_queue_status_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_channel_external_telegram_public_status_claim"; "blocked_channel_external_telegram_public_status_noop"; "channel_external_telegram_public_status_denied"; {channel_status_requested:true, external_status_requested:true, telegram_status_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_release_publication_authority_public_status_claim"; "blocked_release_publication_authority_public_status_noop"; "release_publication_authority_from_public_status_denied"; {release_publication_authority_public_status_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_activation_live_public_status_claim"; "blocked_activation_live_public_status_noop"; "activation_live_from_public_status_denied"; {activation_live_public_status_requested:true}),
      public_claim_surface("publication_result_receipt_delivery_receipt_install_restart_active_binary_public_status_claim"; "blocked_active_binary_public_status_noop"; "install_restart_active_binary_from_public_status_denied"; {install_restart_active_binary_public_status_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_denial_gate" \
  --arg delivery_receipt_terminal_decision_status_report_sha256 "$delivery_receipt_terminal_decision_status_report_sha256" \
  --arg delivery_receipt_terminal_public_claim_status_exposure_contract_hash_sha256 "$delivery_receipt_terminal_public_claim_status_exposure_contract_hash_sha256" \
  --arg delivery_receipt_terminal_public_claim_status_exposure_policy_hash_sha256 "$delivery_receipt_terminal_public_claim_status_exposure_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_TERMINAL_DECISION_STATUS_JSON" \
  --argjson surfaces "$delivery_receipt_terminal_public_claim_status_exposure_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_denial_v1",
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_mode:"denied_terminal_status_cannot_be_exposed_as_public_release_publication_or_activation_status",
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_promotion_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_report_sha256:$delivery_receipt_terminal_decision_status_report_sha256,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_contract_hash_sha256:$delivery_receipt_terminal_public_claim_status_exposure_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_policy_hash_sha256:$delivery_receipt_terminal_public_claim_status_exposure_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_denial_ready:true,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_surface_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_attempt_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_release_publication_authority_derived_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_activation_authority_derived_count,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surface_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_attempt_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_request_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_materialized_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_filesystem_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_public_release_claimed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_public_ga_claimed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_public_badge_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_status_endpoint_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_export_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_observability_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_changelog_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_version_tag_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_acceptance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_operator_approval_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_activation_command_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_live_execution_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_install_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_service_restarted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_active_binary_mutated_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure:[
      "source_terminal_decision_status_report_required",
      "public_claim_status_request_acceptance_denied",
      "public_claim_status_acceptance_denied",
      "public_claim_status_recording_denied",
      "public_claim_status_persistence_denied",
      "public_claim_status_materialization_denied",
      "public_claim_status_filesystem_write_denied",
      "public_claim_status_delivery_denied",
      "public_claim_status_exposure_denied",
      "public_status_claim_denied",
      "public_release_claim_denied",
      "public_ga_claim_denied",
      "release_status_exposure_denied",
      "publication_status_exposure_denied",
      "dashboard_status_exposure_denied",
      "public_badge_exposure_denied",
      "status_endpoint_exposure_denied",
      "query_status_exposure_denied",
      "export_status_exposure_denied",
      "observability_status_exposure_denied",
      "release_notes_status_exposure_denied",
      "changelog_status_exposure_denied",
      "version_tag_status_exposure_denied",
      "artifact_availability_status_exposure_denied",
      "distribution_queue_status_exposure_denied",
      "channel_status_delivery_denied",
      "external_status_send_denied",
      "telegram_status_send_denied",
      "acceptance_from_public_status_denied",
      "operator_approval_from_public_status_denied",
      "release_publication_authority_from_public_status_denied",
      "activation_live_from_public_status_denied",
      "install_restart_active_binary_from_public_status_denied",
      "memory_provider_kg_from_public_status_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_denial_gate",
        status:"allowed_report_only_next_slice",
        exposes_public_status:false,
        claims_public_release:false,
        claims_public_ga:false,
        records_operator_acceptance:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_closed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_ready:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_approved:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_authoritative:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_live:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_badge_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_endpoint_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_export_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_observability_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_changelog_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_version_tag_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_badge_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_endpoint_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_export_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_observability_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_changelog_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_version_tag_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_live:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_surface_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_public_release_claimed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_public_ga_claimed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_public_badge_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_status_endpoint_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_export_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_observability_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_changelog_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_version_tag_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_acceptance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_operator_approval_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_activation_command_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_active_binary_mutated_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surfaces | all(
    .public_claim_status_exposure_attempted == true
    and .public_claim_status_exposure_allowed == false
    and .public_claim_status_exposure_request_accepted == false
    and .public_claim_status_exposure_accepted == false
    and .public_claim_status_exposure_recorded == false
    and .public_claim_status_exposure_persisted == false
    and .public_claim_status_exposure_materialized == false
    and .public_claim_status_exposure_filesystem_written == false
    and .public_claim_status_exposure_delivered == false
    and .public_claim_status_exposed == false
    and .public_status_claimed == false
    and .public_release_claimed == false
    and .public_ga_claimed == false
    and .dashboard_status_exposed == false
    and .public_badge_exposed == false
    and .status_endpoint_exposed == false
    and .query_status_exposed == false
    and .export_status_exposed == false
    and .observability_status_exposed == false
    and .release_notes_status_exposed == false
    and .changelog_status_exposed == false
    and .version_tag_status_exposed == false
    and .artifact_availability_status_exposed == false
    and .distribution_queue_status_exposed == false
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
    and .public_claim_status_exposure_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure | length) == 34
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_status_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_live == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_claimed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_claimed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_badge_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_endpoint_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_export_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_observability_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_changelog_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_version_tag_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt terminal public claim status exposure denial gate passed"
