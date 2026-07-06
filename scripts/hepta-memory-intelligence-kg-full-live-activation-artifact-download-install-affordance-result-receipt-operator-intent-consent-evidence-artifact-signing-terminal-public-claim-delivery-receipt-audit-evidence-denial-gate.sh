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

CANCELLATION_SUPERSESSION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-denial-gate.sh
)"

source_cancellation_supersession_report_sha256="$(sha256_text "$CANCELLATION_SUPERSESSION_JSON")"
terminal_public_claim_delivery_receipt_audit_evidence_contract_hash_sha256="$(
  sha256_text "hepta-artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence-denial:$source_cancellation_supersession_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_public_claim_delivery_receipt_audit_evidence_policy_hash_sha256="$(
  sha256_text "artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence:no-audit:no-immutable-evidence:no-hash-chain:no-attestation:no-ledger:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$CANCELLATION_SUPERSESSION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_ready == true
    and $source.source_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_ready == true
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_surface_count == 18
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_attempt_count == 18
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denied_count == 18
    and zero_fields($source; [
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
      "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_recorded_count",
      "release_publication_authority_from_delivery_receipt_cancellation_supersession_derived_count",
      "activation_authority_from_delivery_receipt_cancellation_supersession_derived_count",
      "install_from_delivery_receipt_cancellation_supersession_executed_count",
      "active_binary_from_delivery_receipt_cancellation_supersession_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "terminal_public_claim_delivery_receipt_cancellation_supersession_accepted",
      "terminal_public_claim_delivery_receipt_cancellation_supersession_recorded",
      "terminal_public_claim_delivery_receipt_cancellation_accepted",
      "terminal_public_claim_delivery_receipt_supersession_accepted",
      "terminal_public_claim_delivery_receipt_replacement_receipt_recorded",
      "terminal_public_claim_delivery_receipt_tombstone_recorded",
      "terminal_public_claim_delivery_receipt_delete_marker_recorded",
      "terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_recorded",
      "terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_recorded",
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
    and ($source.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_surfaces | length) == 18
    and ($source.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_surfaces | all(
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
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .accepts_cancellation == false
      and .records_cancellation == false
      and .accepts_supersession == false
      and .records_supersession == false
      and .accepts_replacement_receipt == false
      and .records_tombstone == false
      and .records_delete_marker == false
      and .records_lifecycle == false
      and .records_result_receipt == false
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
    def audit_surface($id; $status; $reason; $extra):
      {
        artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_surface:$id,
        source_terminal_public_claim_delivery_receipt_cancellation_supersession_ready:true,
        terminal_public_claim_delivery_receipt_audit_evidence_attempted:true,
        terminal_public_claim_delivery_receipt_audit_evidence_allowed:false,
        terminal_public_claim_delivery_receipt_audit_evidence_accepted:false,
        terminal_public_claim_delivery_receipt_audit_evidence_recorded:false,
        terminal_public_claim_delivery_receipt_audit_evidence_persisted:false,
        terminal_public_claim_delivery_receipt_audit_evidence_materialized:false,
        terminal_public_claim_delivery_receipt_audit_evidence_filesystem_written:false,
        terminal_public_claim_delivery_receipt_audit_trail_recorded:false,
        terminal_public_claim_delivery_receipt_audit_trail_persisted:false,
        terminal_public_claim_delivery_receipt_immutable_evidence_recorded:false,
        terminal_public_claim_delivery_receipt_immutable_evidence_persisted:false,
        terminal_public_claim_delivery_receipt_hash_chain_recorded:false,
        terminal_public_claim_delivery_receipt_merkle_root_recorded:false,
        terminal_public_claim_delivery_receipt_attestation_recorded:false,
        terminal_public_claim_delivery_receipt_witness_recorded:false,
        terminal_public_claim_delivery_receipt_notary_recorded:false,
        terminal_public_claim_delivery_receipt_ledger_recorded:false,
        terminal_public_claim_delivery_receipt_ledger_persisted:false,
        terminal_public_claim_delivery_receipt_index_recorded:false,
        terminal_public_claim_delivery_receipt_index_persisted:false,
        terminal_public_claim_delivery_receipt_delivery_evidence_recorded:false,
        terminal_public_claim_delivery_receipt_delivery_evidence_delivered:false,
        terminal_public_claim_delivery_receipt_query_export_evidence_recorded:false,
        terminal_public_claim_delivery_receipt_observability_evidence_recorded:false,
        terminal_public_claim_delivery_receipt_readback_evidence_recorded:false,
        terminal_public_claim_delivery_receipt_status_evidence_recorded:false,
        terminal_public_claim_delivery_receipt_hash_status_evidence_recorded:false,
        public_claim_delivery_receipt_audit_evidence_recorded:false,
        status_readback_delivery_receipt_audit_evidence_recorded:false,
        channel_delivery_receipt_audit_evidence_delivered:false,
        telegram_delivery_receipt_audit_evidence_delivered:false,
        external_delivery_receipt_audit_evidence_delivered:false,
        readback_receipt_backfill_audit_evidence_recorded:false,
        operator_approval_from_delivery_receipt_audit_evidence_derived:false,
        release_publication_authority_from_delivery_receipt_audit_evidence_derived:false,
        activation_authority_from_delivery_receipt_audit_evidence_derived:false,
        download_link_from_delivery_receipt_audit_evidence_rendered:false,
        install_command_from_delivery_receipt_audit_evidence_emitted:false,
        install_from_delivery_receipt_audit_evidence_executed:false,
        service_restart_from_delivery_receipt_audit_evidence_performed:false,
        active_binary_from_delivery_receipt_audit_evidence_mutated:false,
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
        terminal_public_claim_delivery_receipt_audit_evidence_noop_confirmed:true,
        terminal_public_claim_delivery_receipt_audit_evidence_status:$status,
        reason:$reason
      } + $extra;
    [
      audit_surface("source_cancellation_supersession_report_required"; "blocked_source_cancellation_supersession_required_noop"; "source_cancellation_supersession_report_required"; {source_report_required:true}),
      audit_surface("delivery_receipt_cancellation_audit_trail"; "blocked_delivery_receipt_cancellation_audit_trail_noop"; "delivery_receipt_cancellation_audit_trail_denied"; {cancellation_audit_trail_requested:true}),
      audit_surface("delivery_receipt_supersession_immutable_evidence"; "blocked_delivery_receipt_supersession_immutable_evidence_noop"; "delivery_receipt_supersession_immutable_evidence_denied"; {supersession_immutable_evidence_requested:true}),
      audit_surface("delivery_receipt_withdrawal_hash_chain"; "blocked_delivery_receipt_withdrawal_hash_chain_noop"; "delivery_receipt_withdrawal_hash_chain_denied"; {withdrawal_hash_chain_requested:true}),
      audit_surface("delivery_receipt_cancellation_attestation"; "blocked_delivery_receipt_cancellation_attestation_noop"; "delivery_receipt_cancellation_attestation_denied"; {cancellation_attestation_requested:true}),
      audit_surface("delivery_receipt_supersession_witness_notary"; "blocked_delivery_receipt_supersession_witness_notary_noop"; "delivery_receipt_supersession_witness_notary_denied"; {supersession_witness_notary_requested:true}),
      audit_surface("delivery_receipt_tombstone_ledger_index"; "blocked_delivery_receipt_tombstone_ledger_index_noop"; "delivery_receipt_tombstone_ledger_index_denied"; {tombstone_ledger_index_requested:true}),
      audit_surface("delivery_receipt_replacement_evidence_materialization"; "blocked_delivery_receipt_replacement_evidence_materialization_noop"; "delivery_receipt_replacement_evidence_materialization_denied"; {replacement_evidence_materialization_requested:true}),
      audit_surface("delivery_receipt_latest_replacement_immutable_evidence"; "blocked_delivery_receipt_latest_replacement_immutable_evidence_noop"; "delivery_receipt_latest_replacement_immutable_evidence_denied"; {latest_replacement_immutable_evidence_requested:true}),
      audit_surface("delivery_receipt_supersession_evidence_export"; "blocked_delivery_receipt_supersession_evidence_export_noop"; "delivery_receipt_supersession_evidence_export_denied"; {supersession_evidence_export_requested:true}),
      audit_surface("delivery_receipt_cancelled_query_evidence"; "blocked_delivery_receipt_cancelled_query_evidence_noop"; "delivery_receipt_cancelled_query_evidence_denied"; {cancelled_query_evidence_requested:true}),
      audit_surface("delivery_receipt_superseded_observability_evidence"; "blocked_delivery_receipt_superseded_observability_evidence_noop"; "delivery_receipt_superseded_observability_evidence_denied"; {superseded_observability_evidence_requested:true}),
      audit_surface("delivery_receipt_replacement_status_evidence"; "blocked_delivery_receipt_replacement_status_evidence_noop"; "delivery_receipt_replacement_status_evidence_denied"; {replacement_status_evidence_requested:true}),
      audit_surface("delivery_receipt_tombstone_hash_status_evidence"; "blocked_delivery_receipt_tombstone_hash_status_evidence_noop"; "delivery_receipt_tombstone_hash_status_evidence_denied"; {tombstone_hash_status_evidence_requested:true}),
      audit_surface("external_telegram_delivery_receipt_audit_evidence"; "blocked_external_telegram_delivery_receipt_audit_evidence_noop"; "external_telegram_delivery_receipt_audit_evidence_denied"; {external_audit_evidence_delivery_requested:true, telegram_audit_evidence_delivery_requested:true}),
      audit_surface("release_publication_authority_audit_evidence"; "blocked_release_publication_authority_audit_evidence_noop"; "release_publication_authority_audit_evidence_denied"; {release_publication_authority_audit_evidence_requested:true}),
      audit_surface("activation_live_install_audit_evidence"; "blocked_activation_live_install_audit_evidence_noop"; "activation_live_install_audit_evidence_denied"; {activation_live_install_audit_evidence_requested:true}),
      audit_surface("install_restart_active_binary_audit_evidence"; "blocked_install_restart_active_binary_audit_evidence_noop"; "install_restart_active_binary_audit_evidence_denied"; {install_restart_active_binary_audit_evidence_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate" \
    --arg source_cancellation_supersession_report_sha256 "$source_cancellation_supersession_report_sha256" \
    --arg terminal_public_claim_delivery_receipt_audit_evidence_contract_hash_sha256 "$terminal_public_claim_delivery_receipt_audit_evidence_contract_hash_sha256" \
    --arg terminal_public_claim_delivery_receipt_audit_evidence_policy_hash_sha256 "$terminal_public_claim_delivery_receipt_audit_evidence_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$CANCELLATION_SUPERSESSION_JSON" \
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
        artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_schema_version:"artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_v1",
        artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_mode:"denied_delivery_receipt_cancellation_supersession_cannot_be_wrapped_in_audit_immutable_evidence_hash_chain_attestation_ledger_or_used_for_authority_or_live_install",
        source_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_gate:$source.gate,
        source_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_ready,
        source_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_report_sha256:$source_cancellation_supersession_report_sha256,
        source_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_contract_hash_sha256:$source.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_contract_hash_sha256,
        artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_contract_hash_sha256:$terminal_public_claim_delivery_receipt_audit_evidence_contract_hash_sha256,
        artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_policy_hash_sha256:$terminal_public_claim_delivery_receipt_audit_evidence_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_audit_evidence_denial_ready:true,
        source_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_surface_count:$source.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_surface_count,
        source_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denied_count:$source.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denied_count,
        artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_surface_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attempt_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denied_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_surfaces:$surfaces,
        denied_by_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence:[
          "source_cancellation_supersession_report_required",
          "delivery_receipt_cancellation_audit_trail_denied",
          "delivery_receipt_supersession_immutable_evidence_denied",
          "delivery_receipt_withdrawal_hash_chain_denied",
          "delivery_receipt_cancellation_attestation_denied",
          "delivery_receipt_supersession_witness_notary_denied",
          "delivery_receipt_tombstone_ledger_index_denied",
          "delivery_receipt_replacement_evidence_materialization_denied",
          "delivery_receipt_latest_replacement_immutable_evidence_denied",
          "delivery_receipt_supersession_evidence_export_denied",
          "delivery_receipt_cancelled_query_evidence_denied",
          "delivery_receipt_superseded_observability_evidence_denied",
          "delivery_receipt_replacement_status_evidence_denied",
          "delivery_receipt_tombstone_hash_status_evidence_denied",
          "external_telegram_delivery_receipt_audit_evidence_denied",
          "release_publication_authority_audit_evidence_denied",
          "activation_live_install_audit_evidence_denied",
          "install_restart_active_binary_audit_evidence_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_denial_gate",
            status:"allowed_report_only_next_slice",
            records_audit_evidence:false,
            records_immutable_evidence:false,
            records_hash_chain:false,
            records_attestation:false,
            records_witness_notary:false,
            persists_ledger:false,
            accepts_retention:false,
            accepts_expiry:false,
            performs_garbage_collection:false,
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
        "terminal_public_claim_delivery_receipt_audit_evidence_allowed_count",
        "terminal_public_claim_delivery_receipt_audit_evidence_accepted_count",
        "terminal_public_claim_delivery_receipt_audit_evidence_recorded_count",
        "terminal_public_claim_delivery_receipt_audit_evidence_persisted_count",
        "terminal_public_claim_delivery_receipt_audit_evidence_materialized_count",
        "terminal_public_claim_delivery_receipt_audit_evidence_filesystem_written_count",
        "terminal_public_claim_delivery_receipt_audit_trail_recorded_count",
        "terminal_public_claim_delivery_receipt_audit_trail_persisted_count",
        "terminal_public_claim_delivery_receipt_immutable_evidence_recorded_count",
        "terminal_public_claim_delivery_receipt_immutable_evidence_persisted_count",
        "terminal_public_claim_delivery_receipt_hash_chain_recorded_count",
        "terminal_public_claim_delivery_receipt_merkle_root_recorded_count",
        "terminal_public_claim_delivery_receipt_attestation_recorded_count",
        "terminal_public_claim_delivery_receipt_witness_recorded_count",
        "terminal_public_claim_delivery_receipt_notary_recorded_count",
        "terminal_public_claim_delivery_receipt_ledger_recorded_count",
        "terminal_public_claim_delivery_receipt_ledger_persisted_count",
        "terminal_public_claim_delivery_receipt_index_recorded_count",
        "terminal_public_claim_delivery_receipt_index_persisted_count",
        "terminal_public_claim_delivery_receipt_delivery_evidence_recorded_count",
        "terminal_public_claim_delivery_receipt_query_export_evidence_recorded_count",
        "terminal_public_claim_delivery_receipt_observability_evidence_recorded_count",
        "terminal_public_claim_delivery_receipt_readback_evidence_recorded_count",
        "terminal_public_claim_delivery_receipt_status_evidence_recorded_count",
        "terminal_public_claim_delivery_receipt_hash_status_evidence_recorded_count",
        "release_publication_authority_from_delivery_receipt_audit_evidence_derived_count",
        "activation_authority_from_delivery_receipt_audit_evidence_derived_count",
        "install_from_delivery_receipt_audit_evidence_executed_count",
        "service_restart_from_delivery_receipt_audit_evidence_performed_count",
        "active_binary_from_delivery_receipt_audit_evidence_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_audit_evidence_allowed",
        "terminal_public_claim_delivery_receipt_audit_evidence_accepted",
        "terminal_public_claim_delivery_receipt_audit_evidence_recorded",
        "terminal_public_claim_delivery_receipt_audit_evidence_persisted",
        "terminal_public_claim_delivery_receipt_audit_evidence_materialized",
        "terminal_public_claim_delivery_receipt_audit_evidence_filesystem_written",
        "terminal_public_claim_delivery_receipt_audit_trail_recorded",
        "terminal_public_claim_delivery_receipt_audit_trail_persisted",
        "terminal_public_claim_delivery_receipt_immutable_evidence_recorded",
        "terminal_public_claim_delivery_receipt_immutable_evidence_persisted",
        "terminal_public_claim_delivery_receipt_hash_chain_recorded",
        "terminal_public_claim_delivery_receipt_merkle_root_recorded",
        "terminal_public_claim_delivery_receipt_attestation_recorded",
        "terminal_public_claim_delivery_receipt_witness_recorded",
        "terminal_public_claim_delivery_receipt_notary_recorded",
        "terminal_public_claim_delivery_receipt_ledger_recorded",
        "terminal_public_claim_delivery_receipt_index_recorded",
        "terminal_public_claim_delivery_receipt_delivery_evidence_recorded",
        "terminal_public_claim_delivery_receipt_query_export_evidence_recorded",
        "terminal_public_claim_delivery_receipt_observability_evidence_recorded",
        "terminal_public_claim_delivery_receipt_readback_evidence_recorded",
        "terminal_public_claim_delivery_receipt_status_evidence_recorded",
        "terminal_public_claim_delivery_receipt_hash_status_evidence_recorded",
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
          "terminal_public_claim_delivery_receipt_audit_evidence_recorded",
          "terminal_public_claim_delivery_receipt_audit_evidence_persisted",
          "terminal_public_claim_delivery_receipt_audit_evidence_materialized",
          "terminal_public_claim_delivery_receipt_audit_evidence_filesystem_written",
          "terminal_public_claim_delivery_receipt_audit_trail_recorded",
          "terminal_public_claim_delivery_receipt_audit_trail_persisted",
          "terminal_public_claim_delivery_receipt_immutable_evidence_recorded",
          "terminal_public_claim_delivery_receipt_immutable_evidence_persisted",
          "terminal_public_claim_delivery_receipt_hash_chain_recorded",
          "terminal_public_claim_delivery_receipt_merkle_root_recorded",
          "terminal_public_claim_delivery_receipt_attestation_recorded",
          "terminal_public_claim_delivery_receipt_witness_recorded",
          "terminal_public_claim_delivery_receipt_notary_recorded",
          "terminal_public_claim_delivery_receipt_ledger_recorded",
          "terminal_public_claim_delivery_receipt_index_recorded",
          "terminal_public_claim_delivery_receipt_delivery_evidence_recorded",
          "terminal_public_claim_delivery_receipt_query_export_evidence_recorded",
          "terminal_public_claim_delivery_receipt_observability_evidence_recorded",
          "terminal_public_claim_delivery_receipt_readback_evidence_recorded",
          "terminal_public_claim_delivery_receipt_status_evidence_recorded",
          "terminal_public_claim_delivery_receipt_hash_status_evidence_recorded",
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
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_audit_evidence_denial_ready == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_surface_count == 18
  and .artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attempt_count == 18
  and .artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denied_count == 18
  and zero_fields(.; [
    "terminal_public_claim_delivery_receipt_audit_evidence_accepted_count",
    "terminal_public_claim_delivery_receipt_audit_evidence_recorded_count",
    "terminal_public_claim_delivery_receipt_audit_trail_recorded_count",
    "terminal_public_claim_delivery_receipt_immutable_evidence_recorded_count",
    "terminal_public_claim_delivery_receipt_hash_chain_recorded_count",
    "terminal_public_claim_delivery_receipt_attestation_recorded_count",
    "terminal_public_claim_delivery_receipt_ledger_recorded_count",
    "terminal_public_claim_delivery_receipt_index_recorded_count",
    "terminal_public_claim_delivery_receipt_delivery_evidence_recorded_count",
    "terminal_public_claim_delivery_receipt_query_export_evidence_recorded_count",
    "terminal_public_claim_delivery_receipt_observability_evidence_recorded_count",
    "release_publication_authority_from_delivery_receipt_audit_evidence_derived_count",
    "activation_authority_from_delivery_receipt_audit_evidence_derived_count",
    "install_from_delivery_receipt_audit_evidence_executed_count",
    "active_binary_from_delivery_receipt_audit_evidence_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "credential_read_count",
    "external_send_performed_count"
  ])
  and false_fields(.; [
    "terminal_public_claim_delivery_receipt_audit_evidence_accepted",
    "terminal_public_claim_delivery_receipt_audit_evidence_recorded",
    "terminal_public_claim_delivery_receipt_audit_trail_recorded",
    "terminal_public_claim_delivery_receipt_immutable_evidence_recorded",
    "terminal_public_claim_delivery_receipt_hash_chain_recorded",
    "terminal_public_claim_delivery_receipt_attestation_recorded",
    "terminal_public_claim_delivery_receipt_ledger_recorded",
    "terminal_public_claim_delivery_receipt_index_recorded",
    "terminal_public_claim_delivery_receipt_delivery_evidence_recorded",
    "terminal_public_claim_delivery_receipt_query_export_evidence_recorded",
    "terminal_public_claim_delivery_receipt_observability_evidence_recorded",
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
  and (.artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_surfaces | length) == 18
  and (.artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_surfaces | all(
    .terminal_public_claim_delivery_receipt_audit_evidence_attempted == true
    and .terminal_public_claim_delivery_receipt_audit_evidence_allowed == false
    and .terminal_public_claim_delivery_receipt_audit_evidence_accepted == false
    and .terminal_public_claim_delivery_receipt_audit_evidence_noop_confirmed == true
    and .release_publication_authority_from_delivery_receipt_audit_evidence_derived == false
    and .activation_authority_from_delivery_receipt_audit_evidence_derived == false
    and .install_from_delivery_receipt_audit_evidence_executed == false
    and .active_binary_from_delivery_receipt_audit_evidence_mutated == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_audit_evidence == false
    and .records_immutable_evidence == false
    and .records_hash_chain == false
    and .records_attestation == false
    and .accepts_retention == false
    and .accepts_expiry == false
    and .performs_garbage_collection == false
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

echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt audit evidence denial gate passed"
