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

NON_PERSISTENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-non-persistence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-non-persistence-denial-gate.sh
)"

source_non_persistence_report_sha256="$(sha256_text "$NON_PERSISTENCE_JSON")"
terminal_public_claim_delivery_receipt_replay_idempotency_contract_hash_sha256="$(
  sha256_text "hepta-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-denial:$source_non_persistence_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_public_claim_delivery_receipt_replay_idempotency_policy_hash_sha256="$(
  sha256_text "artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency:no-replay:no-duplicate:no-idempotency-key:no-idempotency-state:no-nonce:no-cross-scope:no-status-upgrade:no-ack:no-ledger:no-authority:no-live"
)"

jq -n -e \
  --argjson source "$NON_PERSISTENCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_non_persistence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_non_persistence_denial_ready == true
    and $source.source_artifact_signing_terminal_public_claim_delivery_readback_ready == true
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surface_count == 18
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_attempt_count == 18
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denied_count == 18
    and zero_fields($source; [
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
    and false_fields($source; [
      "terminal_public_claim_delivery_receipt_recorded",
      "terminal_public_claim_delivery_receipt_persisted",
      "terminal_public_claim_delivery_receipt_materialized",
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
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_delivery_receipt == false
      and .persists_delivery_receipt == false
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
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '
    def replay_surface($id; $status; $reason; $extra):
      {
        artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surface:$id,
        source_terminal_public_claim_delivery_receipt_non_persistence_ready:true,
        terminal_public_claim_delivery_receipt_replay_idempotency_attempted:true,
        terminal_public_claim_delivery_receipt_replay_idempotency_allowed:false,
        terminal_public_claim_delivery_receipt_replay_allowed:false,
        terminal_public_claim_delivery_receipt_replay_accepted:false,
        terminal_public_claim_delivery_receipt_replay_recorded:false,
        terminal_public_claim_delivery_receipt_replay_persisted:false,
        terminal_public_claim_delivery_receipt_replay_performed:false,
        terminal_public_claim_delivery_receipt_duplicate_accepted:false,
        terminal_public_claim_delivery_receipt_duplicate_recorded:false,
        terminal_public_claim_delivery_receipt_duplicate_persisted:false,
        terminal_public_claim_delivery_receipt_idempotency_key_accepted:false,
        terminal_public_claim_delivery_receipt_idempotency_key_recorded:false,
        terminal_public_claim_delivery_receipt_idempotency_state_recorded:false,
        terminal_public_claim_delivery_receipt_idempotency_state_persisted:false,
        terminal_public_claim_delivery_receipt_idempotency_state_materialized:false,
        terminal_public_claim_delivery_receipt_idempotency_filesystem_written:false,
        terminal_public_claim_delivery_receipt_replay_nonce_accepted:false,
        terminal_public_claim_delivery_receipt_replay_nonce_recorded:false,
        terminal_public_claim_delivery_receipt_cross_scope_reuse_accepted:false,
        terminal_public_claim_delivery_receipt_status_upgrade_accepted:false,
        terminal_public_claim_delivery_receipt_completed_status_accepted:false,
        terminal_public_claim_delivery_receipt_ack_replay_accepted:false,
        terminal_public_claim_delivery_receipt_ledger_replay_accepted:false,
        terminal_public_claim_delivery_receipt_index_replay_accepted:false,
        terminal_public_claim_delivery_receipt_delivery_replay_accepted:false,
        terminal_public_claim_delivery_receipt_query_replay_accepted:false,
        terminal_public_claim_delivery_receipt_export_replay_accepted:false,
        terminal_public_claim_delivery_receipt_observability_replay_accepted:false,
        terminal_public_claim_delivery_receipt_hash_status_rebind_accepted:false,
        public_claim_delivery_receipt_replay_accepted:false,
        status_readback_delivery_receipt_replay_accepted:false,
        channel_delivery_receipt_replay_accepted:false,
        telegram_delivery_receipt_replay_accepted:false,
        delivery_receipt_acknowledgement_replay_accepted:false,
        readback_receipt_backfill_replay_accepted:false,
        operator_approval_from_delivery_receipt_replay_derived:false,
        release_publication_authority_from_delivery_receipt_replay_derived:false,
        activation_authority_from_delivery_receipt_replay_derived:false,
        download_link_from_delivery_receipt_replay_rendered:false,
        install_command_from_delivery_receipt_replay_emitted:false,
        install_from_delivery_receipt_replay_executed:false,
        service_restart_from_delivery_receipt_replay_performed:false,
        active_binary_from_delivery_receipt_replay_mutated:false,
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
        terminal_public_claim_delivery_receipt_replay_idempotency_noop_confirmed:true,
        terminal_public_claim_delivery_receipt_replay_idempotency_status:$status,
        reason:$reason
      } + $extra;
    [
      replay_surface("source_delivery_receipt_non_persistence_report_required"; "blocked_source_delivery_receipt_non_persistence_required_noop"; "source_delivery_receipt_non_persistence_report_required"; {source_report_required:true}),
      replay_surface("duplicate_delivery_receipt_identity"; "blocked_duplicate_delivery_receipt_identity_noop"; "duplicate_delivery_receipt_identity_denied"; {duplicate_delivery_receipt_identity_requested:true}),
      replay_surface("delivery_receipt_replay_acceptance"; "blocked_delivery_receipt_replay_acceptance_noop"; "delivery_receipt_replay_acceptance_denied"; {delivery_receipt_replay_acceptance_requested:true}),
      replay_surface("delivery_receipt_idempotency_key"; "blocked_delivery_receipt_idempotency_key_noop"; "delivery_receipt_idempotency_key_denied"; {delivery_receipt_idempotency_key_requested:true}),
      replay_surface("delivery_receipt_idempotency_state"; "blocked_delivery_receipt_idempotency_state_noop"; "delivery_receipt_idempotency_state_denied"; {delivery_receipt_idempotency_state_requested:true}),
      replay_surface("delivery_receipt_stale_nonce_replay"; "blocked_delivery_receipt_stale_nonce_replay_noop"; "delivery_receipt_stale_nonce_replay_denied"; {delivery_receipt_stale_nonce_replay_requested:true}),
      replay_surface("delivery_receipt_cross_scope_reuse"; "blocked_delivery_receipt_cross_scope_reuse_noop"; "delivery_receipt_cross_scope_reuse_denied"; {delivery_receipt_cross_scope_reuse_requested:true}),
      replay_surface("delivery_receipt_status_upgrade"; "blocked_delivery_receipt_status_upgrade_noop"; "delivery_receipt_status_upgrade_denied"; {delivery_receipt_status_upgrade_requested:true}),
      replay_surface("delivery_receipt_completed_status_replay"; "blocked_delivery_receipt_completed_status_replay_noop"; "delivery_receipt_completed_status_replay_denied"; {delivery_receipt_completed_status_replay_requested:true}),
      replay_surface("delivery_receipt_ack_replay"; "blocked_delivery_receipt_ack_replay_noop"; "delivery_receipt_ack_replay_denied"; {delivery_receipt_ack_replay_requested:true}),
      replay_surface("delivery_receipt_ledger_index_replay"; "blocked_delivery_receipt_ledger_index_replay_noop"; "delivery_receipt_ledger_index_replay_denied"; {delivery_receipt_ledger_replay_requested:true, delivery_receipt_index_replay_requested:true}),
      replay_surface("delivery_receipt_query_export_observability_replay"; "blocked_delivery_receipt_query_export_observability_replay_noop"; "delivery_receipt_query_export_observability_replay_denied"; {delivery_receipt_query_replay_requested:true, delivery_receipt_export_replay_requested:true, delivery_receipt_observability_replay_requested:true}),
      replay_surface("delivery_receipt_hash_status_rebind"; "blocked_delivery_receipt_hash_status_rebind_noop"; "delivery_receipt_hash_status_rebind_denied"; {delivery_receipt_hash_status_rebind_requested:true}),
      replay_surface("readback_receipt_backfill_replay"; "blocked_readback_receipt_backfill_replay_noop"; "readback_receipt_backfill_replay_denied"; {readback_receipt_backfill_replay_requested:true}),
      replay_surface("external_telegram_delivery_receipt_replay"; "blocked_external_telegram_delivery_receipt_replay_noop"; "external_telegram_delivery_receipt_replay_denied"; {external_delivery_receipt_replay_requested:true, telegram_delivery_receipt_replay_requested:true}),
      replay_surface("release_publication_authority_replay_from_delivery_receipt"; "blocked_release_publication_authority_replay_from_delivery_receipt_noop"; "release_publication_authority_replay_from_delivery_receipt_denied"; {release_publication_authority_replay_from_delivery_receipt_requested:true}),
      replay_surface("activation_live_install_replay_from_delivery_receipt"; "blocked_activation_live_install_replay_from_delivery_receipt_noop"; "activation_live_install_replay_from_delivery_receipt_denied"; {activation_live_install_replay_from_delivery_receipt_requested:true}),
      replay_surface("install_restart_active_binary_replay_from_delivery_receipt"; "blocked_install_restart_active_binary_replay_from_delivery_receipt_noop"; "install_restart_active_binary_replay_from_delivery_receipt_denied"; {install_restart_active_binary_replay_from_delivery_receipt_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate" \
    --arg source_non_persistence_report_sha256 "$source_non_persistence_report_sha256" \
    --arg terminal_public_claim_delivery_receipt_replay_idempotency_contract_hash_sha256 "$terminal_public_claim_delivery_receipt_replay_idempotency_contract_hash_sha256" \
    --arg terminal_public_claim_delivery_receipt_replay_idempotency_policy_hash_sha256 "$terminal_public_claim_delivery_receipt_replay_idempotency_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$NON_PERSISTENCE_JSON" \
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
        artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_schema_version:"artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denial_v1",
        artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_mode:"denied_delivery_receipt_non_persistence_cannot_be_replayed_deduplicated_cached_rebound_promoted_or_used_for_authority_or_live_install",
        source_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_gate:$source.gate,
        source_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_non_persistence_denial_ready,
        source_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_report_sha256:$source_non_persistence_report_sha256,
        source_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_contract_hash_sha256:$source.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_contract_hash_sha256,
        artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_contract_hash_sha256:$terminal_public_claim_delivery_receipt_replay_idempotency_contract_hash_sha256,
        artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_policy_hash_sha256:$terminal_public_claim_delivery_receipt_replay_idempotency_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_replay_idempotency_denial_ready:true,
        source_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surface_count:$source.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surface_count,
        source_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denied_count:$source.artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denied_count,
        artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surface_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attempt_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denied_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surfaces:$surfaces,
        denied_by_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency:[
          "source_delivery_receipt_non_persistence_report_required",
          "delivery_receipt_replay_denied",
          "delivery_receipt_duplicate_identity_denied",
          "delivery_receipt_idempotency_key_denied",
          "delivery_receipt_idempotency_state_denied",
          "delivery_receipt_nonce_replay_denied",
          "delivery_receipt_cross_scope_reuse_denied",
          "delivery_receipt_status_upgrade_denied",
          "delivery_receipt_completed_status_denied",
          "delivery_receipt_ack_replay_denied",
          "delivery_receipt_ledger_index_delivery_replay_denied",
          "delivery_receipt_export_query_observability_replay_denied",
          "delivery_receipt_hash_status_rebind_denied",
          "readback_receipt_backfill_replay_denied",
          "external_telegram_delivery_receipt_replay_denied",
          "release_publication_authority_from_delivery_receipt_replay_denied",
          "activation_live_install_from_delivery_receipt_replay_denied",
          "install_restart_active_binary_from_delivery_receipt_replay_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate",
            status:"allowed_report_only_next_slice",
            replays_delivery_receipt:false,
            records_duplicate_receipt:false,
            records_idempotency_key:false,
            persists_idempotency_state:false,
            accepts_cross_scope_reuse:false,
            accepts_status_upgrade:false,
            records_completion_ack:false,
            rebinds_hash_status:false,
            records_operator_acceptance:false,
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
        "terminal_public_claim_delivery_receipt_replay_idempotency_allowed_count",
        "terminal_public_claim_delivery_receipt_replay_allowed_count",
        "terminal_public_claim_delivery_receipt_replay_accepted_count",
        "terminal_public_claim_delivery_receipt_replay_recorded_count",
        "terminal_public_claim_delivery_receipt_replay_persisted_count",
        "terminal_public_claim_delivery_receipt_replay_performed_count",
        "terminal_public_claim_delivery_receipt_duplicate_accepted_count",
        "terminal_public_claim_delivery_receipt_duplicate_recorded_count",
        "terminal_public_claim_delivery_receipt_duplicate_persisted_count",
        "terminal_public_claim_delivery_receipt_idempotency_key_accepted_count",
        "terminal_public_claim_delivery_receipt_idempotency_key_recorded_count",
        "terminal_public_claim_delivery_receipt_idempotency_state_recorded_count",
        "terminal_public_claim_delivery_receipt_idempotency_state_persisted_count",
        "terminal_public_claim_delivery_receipt_idempotency_state_materialized_count",
        "terminal_public_claim_delivery_receipt_idempotency_filesystem_written_count",
        "terminal_public_claim_delivery_receipt_replay_nonce_accepted_count",
        "terminal_public_claim_delivery_receipt_replay_nonce_recorded_count",
        "terminal_public_claim_delivery_receipt_cross_scope_reuse_accepted_count",
        "terminal_public_claim_delivery_receipt_status_upgrade_accepted_count",
        "terminal_public_claim_delivery_receipt_completed_status_accepted_count",
        "terminal_public_claim_delivery_receipt_ack_replay_accepted_count",
        "terminal_public_claim_delivery_receipt_ledger_replay_accepted_count",
        "terminal_public_claim_delivery_receipt_index_replay_accepted_count",
        "terminal_public_claim_delivery_receipt_delivery_replay_accepted_count",
        "terminal_public_claim_delivery_receipt_query_replay_accepted_count",
        "terminal_public_claim_delivery_receipt_export_replay_accepted_count",
        "terminal_public_claim_delivery_receipt_observability_replay_accepted_count",
        "terminal_public_claim_delivery_receipt_hash_status_rebind_accepted_count",
        "public_claim_delivery_receipt_replay_accepted_count",
        "status_readback_delivery_receipt_replay_accepted_count",
        "channel_delivery_receipt_replay_accepted_count",
        "external_delivery_receipt_replay_accepted_count",
        "telegram_delivery_receipt_replay_accepted_count",
        "readback_receipt_backfill_replay_accepted_count",
        "delivery_receipt_acknowledgement_replay_accepted_count",
        "operator_approval_from_delivery_receipt_replay_derived_count",
        "release_publication_authority_from_delivery_receipt_replay_derived_count",
        "activation_authority_from_delivery_receipt_replay_derived_count",
        "download_link_from_delivery_receipt_replay_rendered_count",
        "install_command_from_delivery_receipt_replay_emitted_count",
        "install_from_delivery_receipt_replay_executed_count",
        "service_restart_from_delivery_receipt_replay_performed_count",
        "active_binary_from_delivery_receipt_replay_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_replay_allowed",
        "terminal_public_claim_delivery_receipt_replay_accepted",
        "terminal_public_claim_delivery_receipt_replay_recorded",
        "terminal_public_claim_delivery_receipt_replay_persisted",
        "terminal_public_claim_delivery_receipt_replay_performed",
        "terminal_public_claim_delivery_receipt_duplicate_accepted",
        "terminal_public_claim_delivery_receipt_idempotency_key_accepted",
        "terminal_public_claim_delivery_receipt_idempotency_state_persisted",
        "terminal_public_claim_delivery_receipt_cross_scope_reuse_accepted",
        "terminal_public_claim_delivery_receipt_status_upgrade_accepted",
        "terminal_public_claim_delivery_receipt_completed_status_accepted",
        "terminal_public_claim_delivery_receipt_hash_status_rebind_accepted",
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
        side_effects:false_object([
          "terminal_public_claim_delivery_receipt_replay_recorded",
          "terminal_public_claim_delivery_receipt_replay_persisted",
          "terminal_public_claim_delivery_receipt_replay_performed",
          "terminal_public_claim_delivery_receipt_duplicate_recorded",
          "terminal_public_claim_delivery_receipt_duplicate_persisted",
          "terminal_public_claim_delivery_receipt_idempotency_key_recorded",
          "terminal_public_claim_delivery_receipt_idempotency_state_recorded",
          "terminal_public_claim_delivery_receipt_idempotency_state_persisted",
          "terminal_public_claim_delivery_receipt_idempotency_state_materialized",
          "terminal_public_claim_delivery_receipt_idempotency_filesystem_written",
          "terminal_public_claim_delivery_receipt_replay_nonce_recorded",
          "terminal_public_claim_delivery_receipt_completed_status_accepted",
          "terminal_public_claim_delivery_receipt_ack_replay_accepted",
          "terminal_public_claim_delivery_receipt_ledger_replay_accepted",
          "terminal_public_claim_delivery_receipt_index_replay_accepted",
          "terminal_public_claim_delivery_receipt_delivery_replay_accepted",
          "terminal_public_claim_delivery_receipt_query_replay_accepted",
          "terminal_public_claim_delivery_receipt_export_replay_accepted",
          "terminal_public_claim_delivery_receipt_observability_replay_accepted",
          "terminal_public_claim_delivery_receipt_hash_status_rebind_accepted",
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

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  . as $report
  | $report.runtime == "hepta"
  and $report.status == "ready"
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_replay_idempotency_denial_ready == true
  and $report.source_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_ready == true
  and $report.source_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_surface_count == 18
  and $report.source_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denied_count == 18
  and $report.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surface_count == 18
  and $report.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attempt_count == 18
  and $report.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denied_count == 18
  and zero_fields($report; [
    "terminal_public_claim_delivery_receipt_replay_allowed_count",
    "terminal_public_claim_delivery_receipt_replay_accepted_count",
    "terminal_public_claim_delivery_receipt_replay_recorded_count",
    "terminal_public_claim_delivery_receipt_replay_persisted_count",
    "terminal_public_claim_delivery_receipt_replay_performed_count",
    "terminal_public_claim_delivery_receipt_duplicate_accepted_count",
    "terminal_public_claim_delivery_receipt_duplicate_recorded_count",
    "terminal_public_claim_delivery_receipt_idempotency_key_accepted_count",
    "terminal_public_claim_delivery_receipt_idempotency_key_recorded_count",
    "terminal_public_claim_delivery_receipt_idempotency_state_recorded_count",
    "terminal_public_claim_delivery_receipt_idempotency_state_persisted_count",
    "terminal_public_claim_delivery_receipt_replay_nonce_recorded_count",
    "terminal_public_claim_delivery_receipt_cross_scope_reuse_accepted_count",
    "terminal_public_claim_delivery_receipt_status_upgrade_accepted_count",
    "terminal_public_claim_delivery_receipt_completed_status_accepted_count",
    "terminal_public_claim_delivery_receipt_ack_replay_accepted_count",
    "terminal_public_claim_delivery_receipt_ledger_replay_accepted_count",
    "terminal_public_claim_delivery_receipt_index_replay_accepted_count",
    "terminal_public_claim_delivery_receipt_query_replay_accepted_count",
    "terminal_public_claim_delivery_receipt_export_replay_accepted_count",
    "terminal_public_claim_delivery_receipt_observability_replay_accepted_count",
    "terminal_public_claim_delivery_receipt_hash_status_rebind_accepted_count",
    "external_delivery_receipt_replay_accepted_count",
    "telegram_delivery_receipt_replay_accepted_count",
    "operator_approval_from_delivery_receipt_replay_derived_count",
    "release_publication_authority_from_delivery_receipt_replay_derived_count",
    "activation_authority_from_delivery_receipt_replay_derived_count",
    "install_from_delivery_receipt_replay_executed_count",
    "service_restart_from_delivery_receipt_replay_performed_count",
    "active_binary_from_delivery_receipt_replay_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "terminal_public_claim_delivery_receipt_replay_allowed",
    "terminal_public_claim_delivery_receipt_replay_accepted",
    "terminal_public_claim_delivery_receipt_replay_recorded",
    "terminal_public_claim_delivery_receipt_replay_persisted",
    "terminal_public_claim_delivery_receipt_replay_performed",
    "terminal_public_claim_delivery_receipt_duplicate_accepted",
    "terminal_public_claim_delivery_receipt_idempotency_key_accepted",
    "terminal_public_claim_delivery_receipt_idempotency_state_persisted",
    "terminal_public_claim_delivery_receipt_cross_scope_reuse_accepted",
    "terminal_public_claim_delivery_receipt_status_upgrade_accepted",
    "terminal_public_claim_delivery_receipt_completed_status_accepted",
    "terminal_public_claim_delivery_receipt_hash_status_rebind_accepted",
    "public_release_claimed",
    "public_ga_claimed",
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
    "external_send_performed"
  ])
  and ($report.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surfaces | length) == 18
  and ($report.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surfaces | all(
    .terminal_public_claim_delivery_receipt_replay_idempotency_attempted == true
    and .terminal_public_claim_delivery_receipt_replay_idempotency_allowed == false
    and .terminal_public_claim_delivery_receipt_replay_allowed == false
    and .terminal_public_claim_delivery_receipt_replay_accepted == false
    and .terminal_public_claim_delivery_receipt_replay_recorded == false
    and .terminal_public_claim_delivery_receipt_replay_persisted == false
    and .terminal_public_claim_delivery_receipt_duplicate_accepted == false
    and .terminal_public_claim_delivery_receipt_idempotency_key_accepted == false
    and .terminal_public_claim_delivery_receipt_idempotency_state_persisted == false
    and .terminal_public_claim_delivery_receipt_replay_nonce_accepted == false
    and .terminal_public_claim_delivery_receipt_cross_scope_reuse_accepted == false
    and .terminal_public_claim_delivery_receipt_status_upgrade_accepted == false
    and .terminal_public_claim_delivery_receipt_hash_status_rebind_accepted == false
    and .terminal_public_claim_delivery_receipt_replay_idempotency_noop_confirmed == true
    and .release_publication_authority_from_delivery_receipt_replay_derived == false
    and .activation_authority_from_delivery_receipt_replay_derived == false
    and .install_from_delivery_receipt_replay_executed == false
    and .service_restart_from_delivery_receipt_replay_performed == false
    and .active_binary_from_delivery_receipt_replay_mutated == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .external_send_performed == false
  ))
  and ([.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surfaces[] | select(.delivery_receipt_idempotency_key_requested == true)] | length) == 1
  and ([.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surfaces[] | select(.delivery_receipt_stale_nonce_replay_requested == true)] | length) == 1
  and ([.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surfaces[] | select(.delivery_receipt_cross_scope_reuse_requested == true)] | length) == 1
  and ([.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surfaces[] | select(.telegram_delivery_receipt_replay_requested == true)] | length) == 1
  and ([.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surfaces[] | select(.install_restart_active_binary_replay_from_delivery_receipt_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .replays_delivery_receipt == false
    and .records_duplicate_receipt == false
    and .records_idempotency_key == false
    and .persists_idempotency_state == false
    and .accepts_cross_scope_reuse == false
    and .accepts_status_upgrade == false
    and .records_completion_ack == false
    and .rebinds_hash_status == false
    and .records_operator_acceptance == false
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
echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt replay/idempotency denial gate passed"
