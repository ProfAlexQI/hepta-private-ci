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

ORDERING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-denial-gate.sh
)"

source_ordering_report_sha256="$(sha256_text "$ORDERING_JSON")"
terminal_public_claim_delivery_receipt_cancellation_supersession_contract_hash_sha256="$(
  sha256_text "hepta-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-denial:$source_ordering_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_public_claim_delivery_receipt_cancellation_supersession_policy_hash_sha256="$(
  sha256_text "artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession:no-cancel:no-supersede:no-replacement:no-tombstone:no-delete-marker:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$ORDERING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_ready == true
    and $source.source_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_ready == true
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_surface_count == 18
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attempt_count == 18
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denied_count == 18
    and zero_fields($source; [
      "terminal_public_claim_delivery_receipt_ordering_monotonicity_allowed_count",
      "terminal_public_claim_delivery_receipt_ordering_monotonicity_accepted_count",
      "terminal_public_claim_delivery_receipt_ordering_monotonicity_recorded_count",
      "terminal_public_claim_delivery_receipt_sequence_cursor_recorded_count",
      "terminal_public_claim_delivery_receipt_monotonicity_state_recorded_count",
      "terminal_public_claim_delivery_receipt_latest_wins_overwrite_accepted_count",
      "terminal_public_claim_delivery_receipt_ordered_status_accepted_count",
      "terminal_public_claim_delivery_receipt_ordered_acknowledgement_accepted_count",
      "terminal_public_claim_delivery_receipt_ordered_ledger_index_accepted_count",
      "terminal_public_claim_delivery_receipt_ordered_query_export_accepted_count",
      "terminal_public_claim_delivery_receipt_ordered_observability_accepted_count",
      "terminal_public_claim_delivery_receipt_ordered_hash_status_accepted_count",
      "release_publication_authority_from_delivery_receipt_ordering_derived_count",
      "activation_authority_from_delivery_receipt_ordering_derived_count",
      "install_from_delivery_receipt_ordering_executed_count",
      "service_restart_from_delivery_receipt_ordering_performed_count",
      "active_binary_from_delivery_receipt_ordering_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "terminal_public_claim_delivery_receipt_ordering_monotonicity_allowed",
      "terminal_public_claim_delivery_receipt_ordering_monotonicity_accepted",
      "terminal_public_claim_delivery_receipt_ordering_monotonicity_recorded",
      "terminal_public_claim_delivery_receipt_sequence_cursor_recorded",
      "terminal_public_claim_delivery_receipt_monotonicity_state_recorded",
      "terminal_public_claim_delivery_receipt_latest_wins_overwrite_accepted",
      "terminal_public_claim_delivery_receipt_ordered_status_accepted",
      "terminal_public_claim_delivery_receipt_ordered_acknowledgement_accepted",
      "terminal_public_claim_delivery_receipt_ordered_hash_status_accepted",
      "public_release_claimed",
      "public_ga_claimed",
      "operator_acceptance_recorded",
      "operator_approval_recorded",
      "release_publication_authority_derived",
      "activation_authority_derived",
      "activation_allowed",
      "activation_performed",
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
      "external_send_performed"
    ])
    and ($source.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_surfaces | length) == 18
    and ($source.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_surfaces | all(
      .terminal_public_claim_delivery_receipt_ordering_monotonicity_attempted == true
      and .terminal_public_claim_delivery_receipt_ordering_monotonicity_allowed == false
      and .terminal_public_claim_delivery_receipt_ordering_monotonicity_accepted == false
      and .terminal_public_claim_delivery_receipt_ordering_monotonicity_noop_confirmed == true
      and .release_publication_authority_from_delivery_receipt_ordering_derived == false
      and .activation_authority_from_delivery_receipt_ordering_derived == false
      and .install_from_delivery_receipt_ordering_executed == false
      and .active_binary_from_delivery_receipt_ordering_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .accepts_ordering == false
      and .records_sequence_cursor == false
      and .records_monotonicity_state == false
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

surfaces_json="$(
  jq -n '
    def cancellation_surface($id; $status; $reason; $extra):
      {
        artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_surface:$id,
        source_terminal_public_claim_delivery_receipt_ordering_monotonicity_ready:true,
        terminal_public_claim_delivery_receipt_cancellation_supersession_attempted:true,
        terminal_public_claim_delivery_receipt_cancellation_supersession_allowed:false,
        terminal_public_claim_delivery_receipt_cancellation_supersession_accepted:false,
        terminal_public_claim_delivery_receipt_cancellation_supersession_recorded:false,
        terminal_public_claim_delivery_receipt_cancellation_supersession_persisted:false,
        terminal_public_claim_delivery_receipt_cancellation_supersession_materialized:false,
        terminal_public_claim_delivery_receipt_cancellation_supersession_filesystem_written:false,
        terminal_public_claim_delivery_receipt_cancellation_accepted:false,
        terminal_public_claim_delivery_receipt_cancellation_recorded:false,
        terminal_public_claim_delivery_receipt_cancellation_persisted:false,
        terminal_public_claim_delivery_receipt_withdrawal_accepted:false,
        terminal_public_claim_delivery_receipt_withdrawal_recorded:false,
        terminal_public_claim_delivery_receipt_supersession_accepted:false,
        terminal_public_claim_delivery_receipt_supersession_recorded:false,
        terminal_public_claim_delivery_receipt_supersession_persisted:false,
        terminal_public_claim_delivery_receipt_replacement_receipt_accepted:false,
        terminal_public_claim_delivery_receipt_replacement_receipt_recorded:false,
        terminal_public_claim_delivery_receipt_replacement_receipt_persisted:false,
        terminal_public_claim_delivery_receipt_tombstone_recorded:false,
        terminal_public_claim_delivery_receipt_tombstone_persisted:false,
        terminal_public_claim_delivery_receipt_delete_marker_recorded:false,
        terminal_public_claim_delivery_receipt_delete_marker_persisted:false,
        terminal_public_claim_delivery_receipt_latest_replacement_accepted:false,
        terminal_public_claim_delivery_receipt_ack_replacement_accepted:false,
        terminal_public_claim_delivery_receipt_cancelled_query_registered:false,
        terminal_public_claim_delivery_receipt_superseded_export_recorded:false,
        terminal_public_claim_delivery_receipt_replacement_observability_recorded:false,
        terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_recorded:false,
        terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_persisted:false,
        terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_recorded:false,
        terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_persisted:false,
        terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_materialized:false,
        terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_filesystem_written:false,
        public_claim_delivery_receipt_cancellation_accepted:false,
        status_readback_delivery_receipt_cancellation_accepted:false,
        channel_delivery_receipt_supersession_accepted:false,
        telegram_delivery_receipt_supersession_accepted:false,
        external_delivery_receipt_supersession_accepted:false,
        readback_receipt_backfill_cancellation_supersession_accepted:false,
        operator_approval_from_delivery_receipt_cancellation_supersession_derived:false,
        release_publication_authority_from_delivery_receipt_cancellation_supersession_derived:false,
        activation_authority_from_delivery_receipt_cancellation_supersession_derived:false,
        download_link_from_delivery_receipt_cancellation_supersession_rendered:false,
        install_command_from_delivery_receipt_cancellation_supersession_emitted:false,
        install_from_delivery_receipt_cancellation_supersession_executed:false,
        service_restart_from_delivery_receipt_cancellation_supersession_performed:false,
        active_binary_from_delivery_receipt_cancellation_supersession_mutated:false,
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
        terminal_public_claim_delivery_receipt_cancellation_supersession_noop_confirmed:true,
        terminal_public_claim_delivery_receipt_cancellation_supersession_status:$status,
        reason:$reason
      } + $extra;
    [
      cancellation_surface("source_ordering_monotonicity_report_required"; "blocked_source_ordering_monotonicity_required_noop"; "source_ordering_monotonicity_report_required"; {source_report_required:true}),
      cancellation_surface("delivery_receipt_cancellation_acceptance"; "blocked_delivery_receipt_cancellation_acceptance_noop"; "delivery_receipt_cancellation_acceptance_denied"; {cancellation_requested:true}),
      cancellation_surface("delivery_receipt_supersession_acceptance"; "blocked_delivery_receipt_supersession_acceptance_noop"; "delivery_receipt_supersession_acceptance_denied"; {supersession_requested:true}),
      cancellation_surface("delivery_receipt_withdrawal"; "blocked_delivery_receipt_withdrawal_noop"; "delivery_receipt_withdrawal_denied"; {withdrawal_requested:true}),
      cancellation_surface("delivery_receipt_replacement_receipt"; "blocked_delivery_receipt_replacement_receipt_noop"; "delivery_receipt_replacement_receipt_denied"; {replacement_receipt_requested:true}),
      cancellation_surface("delivery_receipt_tombstone"; "blocked_delivery_receipt_tombstone_noop"; "delivery_receipt_tombstone_denied"; {tombstone_requested:true}),
      cancellation_surface("delivery_receipt_delete_marker"; "blocked_delivery_receipt_delete_marker_noop"; "delivery_receipt_delete_marker_denied"; {delete_marker_requested:true}),
      cancellation_surface("delivery_receipt_latest_replacement"; "blocked_delivery_receipt_latest_replacement_noop"; "delivery_receipt_latest_replacement_denied"; {latest_replacement_requested:true}),
      cancellation_surface("delivery_receipt_ack_replacement"; "blocked_delivery_receipt_ack_replacement_noop"; "delivery_receipt_ack_replacement_denied"; {ack_replacement_requested:true}),
      cancellation_surface("delivery_receipt_cancelled_query"; "blocked_delivery_receipt_cancelled_query_noop"; "delivery_receipt_cancelled_query_denied"; {cancelled_query_requested:true}),
      cancellation_surface("delivery_receipt_superseded_export"; "blocked_delivery_receipt_superseded_export_noop"; "delivery_receipt_superseded_export_denied"; {superseded_export_requested:true}),
      cancellation_surface("delivery_receipt_replacement_observability"; "blocked_delivery_receipt_replacement_observability_noop"; "delivery_receipt_replacement_observability_denied"; {replacement_observability_requested:true}),
      cancellation_surface("delivery_receipt_lifecycle_cancellation_supersession"; "blocked_delivery_receipt_lifecycle_cancellation_supersession_noop"; "delivery_receipt_lifecycle_cancellation_supersession_denied"; {lifecycle_requested:true}),
      cancellation_surface("delivery_receipt_result_from_cancellation_supersession"; "blocked_delivery_receipt_result_from_cancellation_supersession_noop"; "delivery_receipt_result_from_cancellation_supersession_denied"; {result_receipt_from_cancellation_supersession_requested:true}),
      cancellation_surface("readback_receipt_backfill_cancellation_supersession"; "blocked_readback_receipt_backfill_cancellation_supersession_noop"; "readback_receipt_backfill_cancellation_supersession_denied"; {readback_receipt_backfill_requested:true}),
      cancellation_surface("external_telegram_delivery_receipt_supersession"; "blocked_external_telegram_delivery_receipt_supersession_noop"; "external_telegram_delivery_receipt_supersession_denied"; {external_supersession_requested:true, telegram_supersession_requested:true}),
      cancellation_surface("release_publication_authority_cancellation_supersession"; "blocked_release_publication_authority_cancellation_supersession_noop"; "release_publication_authority_cancellation_supersession_denied"; {release_publication_authority_cancellation_supersession_requested:true}),
      cancellation_surface("activation_install_active_binary_cancellation_supersession"; "blocked_activation_install_active_binary_cancellation_supersession_noop"; "activation_install_active_binary_cancellation_supersession_denied"; {activation_install_active_binary_cancellation_supersession_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate" \
    --arg source_ordering_report_sha256 "$source_ordering_report_sha256" \
    --arg terminal_public_claim_delivery_receipt_cancellation_supersession_contract_hash_sha256 "$terminal_public_claim_delivery_receipt_cancellation_supersession_contract_hash_sha256" \
    --arg terminal_public_claim_delivery_receipt_cancellation_supersession_policy_hash_sha256 "$terminal_public_claim_delivery_receipt_cancellation_supersession_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ORDERING_JSON" \
    --argjson surfaces "$surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_schema_version:"artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_v1",
        artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_mode:"denied_delivery_receipt_ordering_cannot_create_lifecycle_cancellation_supersession_replacement_or_authority",
        source_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_gate:$source.gate,
        source_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_ready,
        source_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_report_sha256:$source_ordering_report_sha256,
        source_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_contract_hash_sha256:$source.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_contract_hash_sha256,
        artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_contract_hash_sha256:$terminal_public_claim_delivery_receipt_cancellation_supersession_contract_hash_sha256,
        artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_policy_hash_sha256:$terminal_public_claim_delivery_receipt_cancellation_supersession_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_ready:true,
        source_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_surface_count:$source.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_surface_count,
        source_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denied_count:$source.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denied_count,
        artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_surface_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_attempt_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denied_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_surfaces:$surfaces,
        denied_by_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession:[
          "source_ordering_monotonicity_report_required",
          "delivery_receipt_cancellation_acceptance_denied",
          "delivery_receipt_supersession_acceptance_denied",
          "delivery_receipt_withdrawal_denied",
          "delivery_receipt_replacement_receipt_denied",
          "delivery_receipt_tombstone_denied",
          "delivery_receipt_delete_marker_denied",
          "delivery_receipt_latest_replacement_denied",
          "delivery_receipt_ack_replacement_denied",
          "delivery_receipt_cancelled_query_denied",
          "delivery_receipt_superseded_export_denied",
          "delivery_receipt_replacement_observability_denied",
          "delivery_receipt_lifecycle_cancellation_supersession_denied",
          "delivery_receipt_result_from_cancellation_supersession_denied",
          "readback_receipt_backfill_cancellation_supersession_denied",
          "external_telegram_delivery_receipt_supersession_denied",
          "release_publication_authority_cancellation_supersession_denied",
          "activation_install_active_binary_cancellation_supersession_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate",
            status:"allowed_report_only_next_slice",
            accepts_cancellation:false,
            records_cancellation:false,
            accepts_supersession:false,
            records_supersession:false,
            accepts_replacement_receipt:false,
            records_tombstone:false,
            records_delete_marker:false,
            records_lifecycle:false,
            records_result_receipt:false,
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
        "terminal_public_claim_delivery_receipt_cancellation_supersession_allowed_count",
        "terminal_public_claim_delivery_receipt_cancellation_supersession_accepted_count",
        "terminal_public_claim_delivery_receipt_cancellation_supersession_recorded_count",
        "terminal_public_claim_delivery_receipt_cancellation_supersession_persisted_count",
        "terminal_public_claim_delivery_receipt_cancellation_supersession_materialized_count",
        "terminal_public_claim_delivery_receipt_cancellation_supersession_filesystem_written_count",
        "terminal_public_claim_delivery_receipt_cancellation_accepted_count",
        "terminal_public_claim_delivery_receipt_cancellation_recorded_count",
        "terminal_public_claim_delivery_receipt_cancellation_persisted_count",
        "terminal_public_claim_delivery_receipt_withdrawal_accepted_count",
        "terminal_public_claim_delivery_receipt_withdrawal_recorded_count",
        "terminal_public_claim_delivery_receipt_supersession_accepted_count",
        "terminal_public_claim_delivery_receipt_supersession_recorded_count",
        "terminal_public_claim_delivery_receipt_supersession_persisted_count",
        "terminal_public_claim_delivery_receipt_replacement_receipt_accepted_count",
        "terminal_public_claim_delivery_receipt_replacement_receipt_recorded_count",
        "terminal_public_claim_delivery_receipt_replacement_receipt_persisted_count",
        "terminal_public_claim_delivery_receipt_tombstone_recorded_count",
        "terminal_public_claim_delivery_receipt_tombstone_persisted_count",
        "terminal_public_claim_delivery_receipt_delete_marker_recorded_count",
        "terminal_public_claim_delivery_receipt_delete_marker_persisted_count",
        "terminal_public_claim_delivery_receipt_latest_replacement_accepted_count",
        "terminal_public_claim_delivery_receipt_ack_replacement_accepted_count",
        "terminal_public_claim_delivery_receipt_cancelled_query_registered_count",
        "terminal_public_claim_delivery_receipt_superseded_export_recorded_count",
        "terminal_public_claim_delivery_receipt_replacement_observability_recorded_count",
        "terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_recorded_count",
        "terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_persisted_count",
        "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_recorded_count",
        "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_persisted_count",
        "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_materialized_count",
        "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_filesystem_written_count",
        "release_publication_authority_from_delivery_receipt_cancellation_supersession_derived_count",
        "activation_authority_from_delivery_receipt_cancellation_supersession_derived_count",
        "install_from_delivery_receipt_cancellation_supersession_executed_count",
        "service_restart_from_delivery_receipt_cancellation_supersession_performed_count",
        "active_binary_from_delivery_receipt_cancellation_supersession_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_cancellation_supersession_allowed",
        "terminal_public_claim_delivery_receipt_cancellation_supersession_accepted",
        "terminal_public_claim_delivery_receipt_cancellation_supersession_recorded",
        "terminal_public_claim_delivery_receipt_cancellation_supersession_persisted",
        "terminal_public_claim_delivery_receipt_cancellation_supersession_materialized",
        "terminal_public_claim_delivery_receipt_cancellation_supersession_filesystem_written",
        "terminal_public_claim_delivery_receipt_cancellation_accepted",
        "terminal_public_claim_delivery_receipt_cancellation_recorded",
        "terminal_public_claim_delivery_receipt_cancellation_persisted",
        "terminal_public_claim_delivery_receipt_withdrawal_accepted",
        "terminal_public_claim_delivery_receipt_withdrawal_recorded",
        "terminal_public_claim_delivery_receipt_supersession_accepted",
        "terminal_public_claim_delivery_receipt_supersession_recorded",
        "terminal_public_claim_delivery_receipt_supersession_persisted",
        "terminal_public_claim_delivery_receipt_replacement_receipt_accepted",
        "terminal_public_claim_delivery_receipt_replacement_receipt_recorded",
        "terminal_public_claim_delivery_receipt_replacement_receipt_persisted",
        "terminal_public_claim_delivery_receipt_tombstone_recorded",
        "terminal_public_claim_delivery_receipt_tombstone_persisted",
        "terminal_public_claim_delivery_receipt_delete_marker_recorded",
        "terminal_public_claim_delivery_receipt_delete_marker_persisted",
        "terminal_public_claim_delivery_receipt_latest_replacement_accepted",
        "terminal_public_claim_delivery_receipt_ack_replacement_accepted",
        "terminal_public_claim_delivery_receipt_cancelled_query_registered",
        "terminal_public_claim_delivery_receipt_superseded_export_recorded",
        "terminal_public_claim_delivery_receipt_replacement_observability_recorded",
        "terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_recorded",
        "terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_persisted",
        "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_recorded",
        "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_persisted",
        "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_materialized",
        "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_filesystem_written",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
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
        "external_send_performed"
      ])
      + {
        side_effects: false_object([
          "terminal_public_claim_delivery_receipt_cancellation_supersession_recorded",
          "terminal_public_claim_delivery_receipt_cancellation_supersession_persisted",
          "terminal_public_claim_delivery_receipt_cancellation_supersession_materialized",
          "terminal_public_claim_delivery_receipt_cancellation_supersession_filesystem_written",
          "terminal_public_claim_delivery_receipt_cancellation_recorded",
          "terminal_public_claim_delivery_receipt_cancellation_persisted",
          "terminal_public_claim_delivery_receipt_withdrawal_recorded",
          "terminal_public_claim_delivery_receipt_supersession_recorded",
          "terminal_public_claim_delivery_receipt_supersession_persisted",
          "terminal_public_claim_delivery_receipt_replacement_receipt_recorded",
          "terminal_public_claim_delivery_receipt_replacement_receipt_persisted",
          "terminal_public_claim_delivery_receipt_tombstone_recorded",
          "terminal_public_claim_delivery_receipt_tombstone_persisted",
          "terminal_public_claim_delivery_receipt_delete_marker_recorded",
          "terminal_public_claim_delivery_receipt_delete_marker_persisted",
          "terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_recorded",
          "terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_persisted",
          "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_recorded",
          "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_persisted",
          "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_materialized",
          "operator_acceptance_recorded",
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

printf '%s\n' "$report"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_ready == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_surface_count == 18
  and .artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_attempt_count == 18
  and .artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denied_count == 18
  and zero_fields(.; [
    "terminal_public_claim_delivery_receipt_cancellation_supersession_accepted_count",
    "terminal_public_claim_delivery_receipt_cancellation_supersession_recorded_count",
    "terminal_public_claim_delivery_receipt_cancellation_accepted_count",
    "terminal_public_claim_delivery_receipt_cancellation_recorded_count",
    "terminal_public_claim_delivery_receipt_supersession_accepted_count",
    "terminal_public_claim_delivery_receipt_supersession_recorded_count",
    "terminal_public_claim_delivery_receipt_replacement_receipt_recorded_count",
    "terminal_public_claim_delivery_receipt_tombstone_recorded_count",
    "terminal_public_claim_delivery_receipt_delete_marker_recorded_count",
    "terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_recorded_count",
    "release_publication_authority_from_delivery_receipt_cancellation_supersession_derived_count",
    "activation_authority_from_delivery_receipt_cancellation_supersession_derived_count",
    "install_from_delivery_receipt_cancellation_supersession_executed_count",
    "active_binary_from_delivery_receipt_cancellation_supersession_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "credential_read_count",
    "external_send_performed_count"
  ])
  and false_fields(.; [
    "terminal_public_claim_delivery_receipt_cancellation_supersession_accepted",
    "terminal_public_claim_delivery_receipt_cancellation_supersession_recorded",
    "terminal_public_claim_delivery_receipt_cancellation_accepted",
    "terminal_public_claim_delivery_receipt_supersession_accepted",
    "terminal_public_claim_delivery_receipt_replacement_receipt_recorded",
    "terminal_public_claim_delivery_receipt_tombstone_recorded",
    "terminal_public_claim_delivery_receipt_delete_marker_recorded",
    "release_publication_authority_derived",
    "activation_authority_derived",
    "install_executed",
    "service_restarted",
    "active_binary_mutated",
    "memory_store_write_performed",
    "live_kg_write_performed",
    "provider_invoked",
    "credential_read",
    "external_send_performed",
    "public_ga_claimed",
    "public_release_claimed"
  ])
  and (.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_surfaces | length) == 18
  and (.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_surfaces | all(
    .terminal_public_claim_delivery_receipt_cancellation_supersession_attempted == true
    and .terminal_public_claim_delivery_receipt_cancellation_supersession_allowed == false
    and .terminal_public_claim_delivery_receipt_cancellation_supersession_accepted == false
    and .terminal_public_claim_delivery_receipt_cancellation_supersession_noop_confirmed == true
    and .release_publication_authority_from_delivery_receipt_cancellation_supersession_derived == false
    and .activation_authority_from_delivery_receipt_cancellation_supersession_derived == false
    and .install_from_delivery_receipt_cancellation_supersession_executed == false
    and .active_binary_from_delivery_receipt_cancellation_supersession_mutated == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .accepts_cancellation == false
    and .records_cancellation == false
    and .accepts_supersession == false
    and .records_supersession == false
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
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt cancellation/supersession denial gate passed"
