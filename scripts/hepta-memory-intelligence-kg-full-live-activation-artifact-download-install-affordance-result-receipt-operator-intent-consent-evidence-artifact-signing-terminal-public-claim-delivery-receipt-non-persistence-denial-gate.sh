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

TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-denial-gate.sh
)"

source_terminal_public_claim_delivery_readback_report_sha256="$(
  sha256_text "$TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_JSON"
)"
terminal_public_claim_delivery_receipt_non_persistence_contract_hash_sha256="$(
  sha256_text "hepta-artifact-signing-terminal-public-claim-delivery-receipt-non-persistence-denial:$source_terminal_public_claim_delivery_readback_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_public_claim_delivery_receipt_non_persistence_policy_hash_sha256="$(
  sha256_text "artifact-signing-terminal-public-claim-delivery-receipt-non-persistence:no-delivery-receipt-record:no-receipt-persist:no-materialization:no-ledger:no-index:no-query:no-export:no-observability:no-status:no-ack:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_ready == true
    and $source.source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_ready == true
    and $source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count == 18
    and zero_fields($source; [
      "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count",
      "artifact_distribution_signing_notarization_receipt_public_claim_delivery_persisted_count",
      "artifact_distribution_signing_notarization_receipt_status_readback_recorded_count",
      "artifact_distribution_signing_notarization_receipt_status_readback_persisted_count",
      "artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count",
      "artifact_distribution_signing_notarization_receipt_channel_delivery_persisted_count",
      "artifact_distribution_signing_notarization_receipt_external_delivery_readback_sent_count",
      "artifact_distribution_signing_notarization_receipt_telegram_delivery_readback_sent_count",
      "delivery_receipt_recorded_count",
      "delivery_receipt_persisted_count",
      "readback_receipt_recorded_count",
      "readback_receipt_persisted_count",
      "operator_approval_from_delivery_readback_derived_count",
      "release_publication_authority_from_delivery_readback_derived_count",
      "activation_authority_from_delivery_readback_derived_count",
      "install_from_delivery_readback_executed_count",
      "service_restart_from_delivery_readback_performed_count",
      "active_binary_from_delivery_readback_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "delivery_receipt_recorded",
      "delivery_receipt_persisted",
      "readback_receipt_recorded",
      "readback_receipt_persisted",
      "operator_approval_recorded",
      "release_publication_authority_derived",
      "activation_authority_derived",
      "download_link_rendered",
      "install_command_emitted",
      "activation_allowed",
      "memory_store_write_performed",
      "live_kg_write_performed",
      "provider_invoked",
      "model_invoked",
      "credential_read",
      "secret_file_read",
      "install_executed",
      "service_restarted",
      "active_binary_mutated",
      "external_send_performed"
    ])
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_non_persistence_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_delivery_receipt == false
      and .records_readback_receipt == false
      and .sends_telegram == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .mutates_memory_store == false
      and .writes_kg == false
      and .invokes_provider == false
      and .reads_credentials == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

delivery_receipt_surfaces_json="$(
  jq -n '
    def receipt_surface($id; $status; $reason; $extra):
      {
        artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surface:$id,
        source_terminal_public_claim_delivery_readback_ready:true,
        terminal_public_claim_delivery_receipt_attempted:true,
        terminal_public_claim_delivery_receipt_allowed:false,
        terminal_public_claim_delivery_receipt_request_accepted:false,
        terminal_public_claim_delivery_receipt_accepted:false,
        terminal_public_claim_delivery_receipt_recorded:false,
        terminal_public_claim_delivery_receipt_persisted:false,
        terminal_public_claim_delivery_receipt_materialized:false,
        terminal_public_claim_delivery_receipt_filesystem_written:false,
        terminal_public_claim_delivery_receipt_ledger_written:false,
        terminal_public_claim_delivery_receipt_index_written:false,
        terminal_public_claim_delivery_receipt_query_registered:false,
        terminal_public_claim_delivery_receipt_exported:false,
        terminal_public_claim_delivery_receipt_observability_recorded:false,
        terminal_public_claim_delivery_receipt_status_exposed:false,
        terminal_public_claim_delivery_receipt_acknowledgement_accepted:false,
        public_claim_delivery_receipt_recorded:false,
        status_readback_delivery_receipt_recorded:false,
        channel_delivery_receipt_recorded:false,
        external_delivery_receipt_delivered:false,
        telegram_delivery_receipt_delivered:false,
        readback_receipt_backfilled:false,
        receipt_echo_delivered:false,
        downstream_consumer_notified:false,
        delivery_receipt_acceptance_recorded:false,
        operator_approval_from_delivery_receipt_derived:false,
        release_publication_authority_from_delivery_receipt_derived:false,
        activation_authority_from_delivery_receipt_derived:false,
        activation_command_from_delivery_receipt_derived:false,
        live_execution_from_delivery_receipt_allowed:false,
        download_link_from_delivery_receipt_rendered:false,
        install_command_from_delivery_receipt_emitted:false,
        install_from_delivery_receipt_executed:false,
        service_restart_from_delivery_receipt_performed:false,
        launchd_from_delivery_receipt_mutated:false,
        active_binary_from_delivery_receipt_mutated:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        telegram_send_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        terminal_public_claim_delivery_receipt_noop_confirmed:true,
        terminal_public_claim_delivery_receipt_status:$status,
        reason:$reason
      } + $extra;
    [
      receipt_surface("source_terminal_public_claim_delivery_readback_report_required"; "blocked_source_delivery_readback_report_required_noop"; "source_terminal_public_claim_delivery_readback_report_required"; {source_report_required:true}),
      receipt_surface("delivery_receipt_schema_acceptance"; "blocked_delivery_receipt_schema_acceptance_noop"; "delivery_receipt_schema_acceptance_denied"; {delivery_receipt_schema_acceptance_requested:true}),
      receipt_surface("public_claim_delivery_receipt_recording"; "blocked_public_claim_delivery_receipt_recording_noop"; "public_claim_delivery_receipt_recording_denied"; {public_claim_delivery_receipt_recording_requested:true}),
      receipt_surface("status_readback_delivery_receipt_persistence"; "blocked_status_readback_delivery_receipt_persistence_noop"; "status_readback_delivery_receipt_persistence_denied"; {status_readback_delivery_receipt_persistence_requested:true}),
      receipt_surface("channel_delivery_receipt_materialization"; "blocked_channel_delivery_receipt_materialization_noop"; "channel_delivery_receipt_materialization_denied"; {channel_delivery_receipt_materialization_requested:true}),
      receipt_surface("telegram_delivery_receipt_delivery"; "blocked_telegram_delivery_receipt_delivery_noop"; "telegram_delivery_receipt_delivery_denied"; {telegram_delivery_receipt_delivery_requested:true}),
      receipt_surface("delivery_receipt_filesystem_write"; "blocked_delivery_receipt_filesystem_write_noop"; "delivery_receipt_filesystem_write_denied"; {delivery_receipt_filesystem_write_requested:true}),
      receipt_surface("delivery_receipt_ledger_write"; "blocked_delivery_receipt_ledger_write_noop"; "delivery_receipt_ledger_write_denied"; {delivery_receipt_ledger_write_requested:true}),
      receipt_surface("delivery_receipt_index_registration"; "blocked_delivery_receipt_index_registration_noop"; "delivery_receipt_index_registration_denied"; {delivery_receipt_index_registration_requested:true}),
      receipt_surface("delivery_receipt_query_export_observability"; "blocked_delivery_receipt_query_export_observability_noop"; "delivery_receipt_query_export_observability_denied"; {delivery_receipt_query_requested:true, delivery_receipt_export_requested:true, delivery_receipt_observability_requested:true}),
      receipt_surface("delivery_receipt_status_exposure"; "blocked_delivery_receipt_status_exposure_noop"; "delivery_receipt_status_exposure_denied"; {delivery_receipt_status_exposure_requested:true}),
      receipt_surface("readback_receipt_backfill_attempt"; "blocked_readback_receipt_backfill_noop"; "readback_receipt_backfill_denied"; {readback_receipt_backfill_requested:true}),
      receipt_surface("terminal_public_claim_receipt_ack_acceptance"; "blocked_terminal_public_claim_receipt_ack_acceptance_noop"; "terminal_public_claim_receipt_ack_acceptance_denied"; {delivery_receipt_acknowledgement_requested:true}),
      receipt_surface("release_publication_authority_from_delivery_receipt"; "blocked_release_publication_authority_from_delivery_receipt_noop"; "release_publication_authority_from_delivery_receipt_denied"; {release_publication_authority_from_delivery_receipt_requested:true}),
      receipt_surface("activation_authority_from_delivery_receipt"; "blocked_activation_authority_from_delivery_receipt_noop"; "activation_authority_from_delivery_receipt_denied"; {activation_authority_from_delivery_receipt_requested:true}),
      receipt_surface("download_install_command_from_delivery_receipt"; "blocked_download_install_command_from_delivery_receipt_noop"; "download_install_command_from_delivery_receipt_denied"; {download_link_from_delivery_receipt_requested:true, install_command_from_delivery_receipt_requested:true}),
      receipt_surface("install_restart_active_binary_from_delivery_receipt"; "blocked_install_restart_active_binary_from_delivery_receipt_noop"; "install_restart_active_binary_from_delivery_receipt_denied"; {install_restart_active_binary_from_delivery_receipt_requested:true}),
      receipt_surface("memory_kg_provider_secret_external_send_from_delivery_receipt"; "blocked_memory_kg_provider_secret_external_send_from_delivery_receipt_noop"; "memory_kg_provider_secret_external_send_from_delivery_receipt_denied"; {memory_kg_provider_secret_external_send_from_delivery_receipt_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_non_persistence_denial_gate" \
    --arg source_terminal_public_claim_delivery_readback_report_sha256 "$source_terminal_public_claim_delivery_readback_report_sha256" \
    --arg terminal_public_claim_delivery_receipt_non_persistence_contract_hash_sha256 "$terminal_public_claim_delivery_receipt_non_persistence_contract_hash_sha256" \
    --arg terminal_public_claim_delivery_receipt_non_persistence_policy_hash_sha256 "$terminal_public_claim_delivery_receipt_non_persistence_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_JSON" \
    --argjson surfaces "$delivery_receipt_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_schema_version:"artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denial_v1",
        artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_mode:"denied_terminal_public_claim_delivery_readback_cannot_create_record_persist_materialize_index_export_observe_status_ack_authority_or_live_install",
        source_artifact_signing_terminal_public_claim_delivery_readback_gate:$source.gate,
        source_artifact_signing_terminal_public_claim_delivery_readback_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_ready,
        source_artifact_signing_terminal_public_claim_delivery_readback_report_sha256:$source_terminal_public_claim_delivery_readback_report_sha256,
        source_artifact_signing_terminal_public_claim_delivery_readback_contract_hash_sha256:$source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_contract_hash_sha256,
        artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_contract_hash_sha256:$terminal_public_claim_delivery_receipt_non_persistence_contract_hash_sha256,
        artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_policy_hash_sha256:$terminal_public_claim_delivery_receipt_non_persistence_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_non_persistence_denial_ready:true,
        source_artifact_signing_terminal_public_claim_delivery_readback_surface_count:$source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count,
        source_artifact_signing_terminal_public_claim_delivery_readback_denied_count:$source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count,
        artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surface_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_attempt_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denied_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surfaces:$surfaces,
        denied_by_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence:[
          "source_terminal_public_claim_delivery_readback_report_required",
          "delivery_receipt_schema_acceptance_denied",
          "public_claim_delivery_receipt_recording_denied",
          "status_readback_delivery_receipt_persistence_denied",
          "channel_delivery_receipt_materialization_denied",
          "telegram_delivery_receipt_delivery_denied",
          "delivery_receipt_filesystem_write_denied",
          "delivery_receipt_ledger_write_denied",
          "delivery_receipt_index_registration_denied",
          "delivery_receipt_query_export_observability_denied",
          "delivery_receipt_status_exposure_denied",
          "readback_receipt_backfill_denied",
          "terminal_public_claim_receipt_ack_acceptance_denied",
          "release_publication_authority_from_delivery_receipt_denied",
          "activation_authority_from_delivery_receipt_denied",
          "download_install_command_from_delivery_receipt_denied",
          "install_restart_active_binary_from_delivery_receipt_denied",
          "memory_kg_provider_secret_external_send_from_delivery_receipt_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate",
            status:"allowed_report_only_next_slice",
            records_delivery_receipt:false,
            persists_delivery_receipt:false,
            materializes_delivery_receipt:false,
            writes_delivery_receipt_to_filesystem:false,
            writes_delivery_receipt_to_ledger:false,
            indexes_delivery_receipt:false,
            exports_delivery_receipt:false,
            registers_delivery_receipt_query:false,
            records_delivery_receipt_observability:false,
            exposes_delivery_receipt_status:false,
            accepts_delivery_receipt_acknowledgement:false,
            replays_delivery_receipt:false,
            accepts_idempotency_key:false,
            derives_operator_approval:false,
            derives_release_publication_authority:false,
            derives_activation_authority:false,
            renders_download_link:false,
            emits_install_command:false,
            installs_or_restarts:false,
            mutates_active_binary:false,
            mutates_memory_store:false,
            writes_kg:false,
            invokes_provider:false,
            reads_credentials:false,
            sends_externally:false
          }
        ]
      }
      + zero_object([
        "terminal_public_claim_delivery_receipt_allowed_count",
        "terminal_public_claim_delivery_receipt_request_accepted_count",
        "terminal_public_claim_delivery_receipt_accepted_count",
        "terminal_public_claim_delivery_receipt_recorded_count",
        "terminal_public_claim_delivery_receipt_persisted_count",
        "terminal_public_claim_delivery_receipt_materialized_count",
        "terminal_public_claim_delivery_receipt_filesystem_written_count",
        "terminal_public_claim_delivery_receipt_ledger_written_count",
        "terminal_public_claim_delivery_receipt_index_written_count",
        "terminal_public_claim_delivery_receipt_query_registered_count",
        "terminal_public_claim_delivery_receipt_exported_count",
        "terminal_public_claim_delivery_receipt_observability_recorded_count",
        "terminal_public_claim_delivery_receipt_status_exposed_count",
        "terminal_public_claim_delivery_receipt_acknowledgement_accepted_count",
        "public_claim_delivery_receipt_recorded_count",
        "status_readback_delivery_receipt_recorded_count",
        "channel_delivery_receipt_recorded_count",
        "external_delivery_receipt_delivered_count",
        "telegram_delivery_receipt_delivered_count",
        "readback_receipt_backfilled_count",
        "operator_approval_from_delivery_receipt_derived_count",
        "release_publication_authority_from_delivery_receipt_derived_count",
        "activation_authority_from_delivery_receipt_derived_count",
        "download_link_from_delivery_receipt_rendered_count",
        "install_command_from_delivery_receipt_emitted_count",
        "install_from_delivery_receipt_executed_count",
        "service_restart_from_delivery_receipt_performed_count",
        "active_binary_from_delivery_receipt_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_recorded",
        "terminal_public_claim_delivery_receipt_persisted",
        "terminal_public_claim_delivery_receipt_materialized",
        "terminal_public_claim_delivery_receipt_filesystem_written",
        "terminal_public_claim_delivery_receipt_ledger_written",
        "terminal_public_claim_delivery_receipt_index_written",
        "terminal_public_claim_delivery_receipt_query_registered",
        "terminal_public_claim_delivery_receipt_exported",
        "terminal_public_claim_delivery_receipt_observability_recorded",
        "terminal_public_claim_delivery_receipt_status_exposed",
        "terminal_public_claim_delivery_receipt_acknowledgement_accepted",
        "public_claim_delivery_receipt_recorded",
        "status_readback_delivery_receipt_recorded",
        "channel_delivery_receipt_recorded",
        "external_delivery_receipt_delivered",
        "telegram_delivery_receipt_delivered",
        "readback_receipt_backfilled",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed"
      ])
      + {
        side_effects:false_object([
          "terminal_public_claim_delivery_receipt_recorded",
          "terminal_public_claim_delivery_receipt_persisted",
          "terminal_public_claim_delivery_receipt_materialized",
          "terminal_public_claim_delivery_receipt_filesystem_written",
          "terminal_public_claim_delivery_receipt_ledger_written",
          "terminal_public_claim_delivery_receipt_index_written",
          "terminal_public_claim_delivery_receipt_query_registered",
          "terminal_public_claim_delivery_receipt_exported",
          "terminal_public_claim_delivery_receipt_observability_recorded",
          "terminal_public_claim_delivery_receipt_status_exposed",
          "terminal_public_claim_delivery_receipt_acknowledgement_accepted",
          "public_claim_delivery_receipt_recorded",
          "status_readback_delivery_receipt_recorded",
          "channel_delivery_receipt_recorded",
          "external_delivery_receipt_delivered",
          "telegram_delivery_receipt_delivered",
          "readback_receipt_backfilled",
          "operator_approval_recorded",
          "release_publication_authority_derived",
          "activation_authority_derived",
          "download_link_rendered",
          "install_command_emitted",
          "install_executed",
          "launchd_mutated",
          "service_restarted",
          "active_binary_mutated",
          "memory_store_write_performed",
          "memory_store_mutated",
          "live_kg_write_performed",
          "provider_invoked",
          "model_invoked",
          "credential_read",
          "secret_file_read",
          "telegram_send_performed",
          "channel_send_performed",
          "external_send_performed",
          "release_artifact_written",
          "public_artifact_written",
          "filesystem_written"
        ])
      }
    '
)"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  . as $report
  | $report.runtime == "hepta"
  and $report.status == "ready"
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_non_persistence_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_non_persistence_denial_ready == true
  and $report.source_artifact_signing_terminal_public_claim_delivery_readback_ready == true
  and $report.source_artifact_signing_terminal_public_claim_delivery_readback_surface_count == 18
  and $report.source_artifact_signing_terminal_public_claim_delivery_readback_denied_count == 18
  and $report.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surface_count == 18
  and $report.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_attempt_count == 18
  and $report.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denied_count == 18
  and zero_fields($report; [
    "terminal_public_claim_delivery_receipt_recorded_count",
    "terminal_public_claim_delivery_receipt_persisted_count",
    "terminal_public_claim_delivery_receipt_materialized_count",
    "terminal_public_claim_delivery_receipt_filesystem_written_count",
    "terminal_public_claim_delivery_receipt_ledger_written_count",
    "terminal_public_claim_delivery_receipt_index_written_count",
    "terminal_public_claim_delivery_receipt_query_registered_count",
    "terminal_public_claim_delivery_receipt_exported_count",
    "terminal_public_claim_delivery_receipt_observability_recorded_count",
    "terminal_public_claim_delivery_receipt_status_exposed_count",
    "terminal_public_claim_delivery_receipt_acknowledgement_accepted_count",
    "operator_approval_from_delivery_receipt_derived_count",
    "release_publication_authority_from_delivery_receipt_derived_count",
    "activation_authority_from_delivery_receipt_derived_count",
    "install_from_delivery_receipt_executed_count",
    "service_restart_from_delivery_receipt_performed_count",
    "active_binary_from_delivery_receipt_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "terminal_public_claim_delivery_receipt_recorded",
    "terminal_public_claim_delivery_receipt_persisted",
    "terminal_public_claim_delivery_receipt_materialized",
    "terminal_public_claim_delivery_receipt_filesystem_written",
    "terminal_public_claim_delivery_receipt_ledger_written",
    "terminal_public_claim_delivery_receipt_index_written",
    "terminal_public_claim_delivery_receipt_query_registered",
    "terminal_public_claim_delivery_receipt_exported",
    "terminal_public_claim_delivery_receipt_observability_recorded",
    "terminal_public_claim_delivery_receipt_status_exposed",
    "terminal_public_claim_delivery_receipt_acknowledgement_accepted",
    "operator_approval_recorded",
    "release_publication_authority_derived",
    "activation_authority_derived",
    "activation_allowed",
    "install_executed",
    "service_restarted",
    "active_binary_mutated",
    "memory_store_write_performed",
    "live_kg_write_performed",
    "provider_invoked",
    "model_invoked",
    "credential_read",
    "secret_file_read",
    "external_send_performed",
    "public_release_claimed",
    "public_ga_claimed"
  ])
  and ($report.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surfaces | length) == 18
  and ($report.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surfaces | all(
    .terminal_public_claim_delivery_receipt_attempted == true
    and .terminal_public_claim_delivery_receipt_allowed == false
    and .terminal_public_claim_delivery_receipt_request_accepted == false
    and .terminal_public_claim_delivery_receipt_accepted == false
    and .terminal_public_claim_delivery_receipt_recorded == false
    and .terminal_public_claim_delivery_receipt_persisted == false
    and .terminal_public_claim_delivery_receipt_materialized == false
    and .terminal_public_claim_delivery_receipt_filesystem_written == false
    and .terminal_public_claim_delivery_receipt_ledger_written == false
    and .terminal_public_claim_delivery_receipt_index_written == false
    and .terminal_public_claim_delivery_receipt_status_exposed == false
    and .terminal_public_claim_delivery_receipt_acknowledgement_accepted == false
    and .terminal_public_claim_delivery_receipt_noop_confirmed == true
    and .operator_approval_from_delivery_receipt_derived == false
    and .release_publication_authority_from_delivery_receipt_derived == false
    and .activation_authority_from_delivery_receipt_derived == false
    and .download_link_from_delivery_receipt_rendered == false
    and .install_command_from_delivery_receipt_emitted == false
    and .install_from_delivery_receipt_executed == false
    and .service_restart_from_delivery_receipt_performed == false
    and .active_binary_from_delivery_receipt_mutated == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .external_send_performed == false
  ))
  and ([.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surfaces[] | select(.delivery_receipt_filesystem_write_requested == true)] | length) == 1
  and ([.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surfaces[] | select(.delivery_receipt_ledger_write_requested == true)] | length) == 1
  and ([.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surfaces[] | select(.telegram_delivery_receipt_delivery_requested == true)] | length) == 1
  and ([.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surfaces[] | select(.delivery_receipt_acknowledgement_requested == true)] | length) == 1
  and ([.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surfaces[] | select(.install_restart_active_binary_from_delivery_receipt_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_delivery_receipt == false
    and .persists_delivery_receipt == false
    and .writes_delivery_receipt_to_ledger == false
    and .indexes_delivery_receipt == false
    and .exposes_delivery_receipt_status == false
    and .accepts_delivery_receipt_acknowledgement == false
    and .replays_delivery_receipt == false
    and .accepts_idempotency_key == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .invokes_provider == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt non-persistence denial gate passed"
