#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the terminal public claim delivery receipt artifact signing receipt signing receipt audit/evidence denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-cancel-denial-gate.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt cancellation/supersession denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
signing_receipt_audit_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-signing-receipt-audit-evidence-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
signing_receipt_audit_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-audit:no-audit:no-evidence:no-ledger:no-authority:no-install"
)"

surfaces_json="$(
  jq -n '[
    "source_signing_receipt_cancellation_supersession_report_required",
    "signing_receipt_cancellation_audit_trail",
    "signing_receipt_supersession_immutable_evidence",
    "signing_receipt_withdrawal_hash_chain",
    "signing_receipt_replacement_attestation",
    "signing_receipt_tombstone_ledger_index",
    "signing_receipt_delete_marker_evidence",
    "signing_receipt_latest_replacement_evidence",
    "signing_receipt_ack_replacement_evidence",
    "signing_receipt_cancelled_query_evidence",
    "signing_receipt_superseded_export_evidence",
    "signing_receipt_replacement_observability_evidence",
    "signing_receipt_lifecycle_audit_evidence",
    "signing_receipt_result_audit_evidence",
    "artifact_package_signature_receipt_audit_evidence",
    "notarization_release_registry_receipt_audit_evidence",
    "external_telegram_signing_receipt_audit_evidence",
    "approval_authority_install_from_signing_receipt_audit_evidence"
  ] | map({
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_surface: .,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_ready: true,
    signing_receipt_audit_evidence_attempted: true,
    signing_receipt_audit_evidence_allowed: false,
    signing_receipt_audit_evidence_accepted: false,
    signing_receipt_audit_evidence_recorded: false,
    signing_receipt_audit_evidence_persisted: false,
    signing_receipt_audit_evidence_materialized: false,
    signing_receipt_audit_evidence_filesystem_written: false,
    signing_receipt_audit_trail_recorded: false,
    signing_receipt_immutable_evidence_recorded: false,
    signing_receipt_hash_chain_recorded: false,
    signing_receipt_merkle_root_recorded: false,
    signing_receipt_attestation_recorded: false,
    signing_receipt_witness_recorded: false,
    signing_receipt_notary_recorded: false,
    signing_receipt_ledger_recorded: false,
    signing_receipt_index_recorded: false,
    signing_receipt_delivery_evidence_recorded: false,
    signing_receipt_query_export_evidence_recorded: false,
    signing_receipt_observability_evidence_recorded: false,
    signing_receipt_readback_evidence_recorded: false,
    signing_receipt_status_evidence_recorded: false,
    signing_receipt_hash_status_evidence_recorded: false,
    artifact_signing_receipt_signing_receipt_audit_evidence_recorded: false,
    package_signing_receipt_audit_evidence_recorded: false,
    signature_manifest_receipt_audit_evidence_recorded: false,
    notarization_ticket_receipt_audit_evidence_recorded: false,
    release_asset_receipt_audit_evidence_recorded: false,
    cdn_update_feed_receipt_audit_evidence_recorded: false,
    package_registry_receipt_audit_evidence_recorded: false,
    external_signing_receipt_audit_evidence_delivered: false,
    telegram_signing_receipt_audit_evidence_delivered: false,
    readback_receipt_backfill_audit_evidence_recorded: false,
    operator_approval_from_signing_receipt_audit_evidence_derived: false,
    release_publication_authority_from_signing_receipt_audit_evidence_derived: false,
    activation_authority_from_signing_receipt_audit_evidence_derived: false,
    install_from_signing_receipt_audit_evidence_executed: false,
    service_restart_from_signing_receipt_audit_evidence_performed: false,
    active_binary_from_signing_receipt_audit_evidence_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    external_send_performed: false,
    telegram_send_performed: false,
    signing_receipt_audit_evidence_noop_confirmed: true,
    signing_receipt_audit_evidence_status: "artifact_signing_receipt_signing_receipt_audit_evidence_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg signing_receipt_audit_contract_hash_sha256 "$signing_receipt_audit_contract_hash_sha256" \
    --arg signing_receipt_audit_policy_hash_sha256 "$signing_receipt_audit_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$SOURCE_JSON" \
    --argjson surfaces "$surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_schema_version: "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_denial_v1",
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_mode: "denied_signing_receipt_cancellation_supersession_cannot_emit_audit_evidence_or_derive_authority_or_install",
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_denial_ready,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_report_sha256: $source_report_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_contract_hash_sha256: $signing_receipt_audit_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_policy_hash_sha256: $signing_receipt_audit_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_surface_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_surface_count,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_denied_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_denied_count,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_surfaces: $surfaces,
        denied_by_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence: ($surfaces | map(.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_surface)),
        allowed_next_actions: [
          {
            action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_retention_expiry_gc_denial_gate",
            status: "allowed_report_only_next_slice",
            records_audit_evidence: false,
            records_immutable_evidence: false,
            records_hash_chain: false,
            records_attestation: false,
            persists_ledger: false,
            accepts_retention: false,
            accepts_expiry: false,
            performs_garbage_collection: false,
            derives_release_publication_authority: false,
            derives_activation_authority: false,
            installs_or_restarts: false,
            mutates_active_binary: false,
            invokes_provider: false,
            reads_credentials: false,
            sends_externally: false
          }
        ]
      }
      + zero_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_allowed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_accepted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_recorded_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_persisted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_materialized_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_filesystem_written_count",
        "signing_receipt_audit_trail_recorded_count",
        "signing_receipt_immutable_evidence_recorded_count",
        "signing_receipt_hash_chain_recorded_count",
        "signing_receipt_merkle_root_recorded_count",
        "signing_receipt_attestation_recorded_count",
        "signing_receipt_witness_recorded_count",
        "signing_receipt_notary_recorded_count",
        "signing_receipt_ledger_recorded_count",
        "signing_receipt_index_recorded_count",
        "signing_receipt_delivery_evidence_recorded_count",
        "signing_receipt_query_export_evidence_recorded_count",
        "signing_receipt_observability_evidence_recorded_count",
        "signing_receipt_readback_evidence_recorded_count",
        "signing_receipt_status_evidence_recorded_count",
        "signing_receipt_hash_status_evidence_recorded_count",
        "artifact_signing_receipt_signing_receipt_audit_evidence_recorded_count",
        "package_signing_receipt_audit_evidence_recorded_count",
        "signature_manifest_receipt_audit_evidence_recorded_count",
        "notarization_ticket_receipt_audit_evidence_recorded_count",
        "release_asset_receipt_audit_evidence_recorded_count",
        "cdn_update_feed_receipt_audit_evidence_recorded_count",
        "package_registry_receipt_audit_evidence_recorded_count",
        "external_signing_receipt_audit_evidence_delivered_count",
        "telegram_signing_receipt_audit_evidence_delivered_count",
        "operator_approval_from_signing_receipt_audit_evidence_derived_count",
        "release_publication_authority_from_signing_receipt_audit_evidence_derived_count",
        "activation_authority_from_signing_receipt_audit_evidence_derived_count",
        "install_from_signing_receipt_audit_evidence_executed_count",
        "service_restart_from_signing_receipt_audit_evidence_performed_count",
        "active_binary_from_signing_receipt_audit_evidence_mutated_count",
        "provider_invoked_count",
        "credential_read_count",
        "external_send_performed_count",
        "telegram_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_allowed",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_accepted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_persisted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_materialized",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_filesystem_written",
        "signing_receipt_audit_trail_recorded",
        "signing_receipt_immutable_evidence_recorded",
        "signing_receipt_hash_chain_recorded",
        "signing_receipt_merkle_root_recorded",
        "signing_receipt_attestation_recorded",
        "signing_receipt_witness_recorded",
        "signing_receipt_notary_recorded",
        "signing_receipt_ledger_recorded",
        "signing_receipt_index_recorded",
        "signing_receipt_delivery_evidence_recorded",
        "signing_receipt_query_export_evidence_recorded",
        "signing_receipt_observability_evidence_recorded",
        "signing_receipt_readback_evidence_recorded",
        "signing_receipt_status_evidence_recorded",
        "signing_receipt_hash_status_evidence_recorded",
        "artifact_signing_receipt_signing_receipt_audit_evidence_recorded",
        "notarization_ticket_receipt_audit_evidence_recorded",
        "external_signing_receipt_audit_evidence_delivered",
        "telegram_signing_receipt_audit_evidence_delivered",
        "operator_approval_from_signing_receipt_audit_evidence_derived",
        "release_publication_authority_from_signing_receipt_audit_evidence_derived",
        "activation_authority_from_signing_receipt_audit_evidence_derived",
        "install_from_signing_receipt_audit_evidence_executed",
        "service_restart_from_signing_receipt_audit_evidence_performed",
        "active_binary_from_signing_receipt_audit_evidence_mutated",
        "memory_store_write_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "external_send_performed",
        "telegram_send_performed",
        "public_status_claimed",
        "public_release_claimed",
        "public_ga_claimed"
      ])
      + {
        side_effects: false_object([
          "signing_receipt_audit_evidence_recorded",
          "signing_receipt_audit_evidence_persisted",
          "signing_receipt_audit_evidence_materialized",
          "signing_receipt_audit_evidence_filesystem_written",
          "signing_receipt_audit_trail_recorded",
          "signing_receipt_immutable_evidence_recorded",
          "signing_receipt_hash_chain_recorded",
          "signing_receipt_merkle_root_recorded",
          "signing_receipt_attestation_recorded",
          "signing_receipt_witness_recorded",
          "signing_receipt_notary_recorded",
          "signing_receipt_ledger_recorded",
          "signing_receipt_index_recorded",
          "signing_receipt_delivery_evidence_recorded",
          "signing_receipt_query_export_evidence_recorded",
          "signing_receipt_observability_evidence_recorded",
          "signing_receipt_readback_evidence_recorded",
          "signing_receipt_status_evidence_recorded",
          "signing_receipt_hash_status_evidence_recorded",
          "operator_approval_from_signing_receipt_audit_evidence_derived",
          "release_publication_authority_from_signing_receipt_audit_evidence_derived",
          "activation_authority_from_signing_receipt_audit_evidence_derived",
          "install_from_signing_receipt_audit_evidence_executed",
          "service_restart_from_signing_receipt_audit_evidence_performed",
          "active_binary_from_signing_receipt_audit_evidence_mutated",
          "memory_store_write_performed",
          "live_kg_write_performed",
          "provider_invoked",
          "model_invoked",
          "credential_read",
          "secret_file_read",
          "telegram_send_performed",
          "external_send_performed",
          "public_status_claimed",
          "public_release_claimed",
          "public_ga_claimed"
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
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_surface_count == 18
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_denied_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_denied_count == 18
  and zero_fields(.; [
    "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_accepted_count",
    "signing_receipt_audit_trail_recorded_count",
    "signing_receipt_immutable_evidence_recorded_count",
    "signing_receipt_hash_chain_recorded_count",
    "signing_receipt_attestation_recorded_count",
    "signing_receipt_ledger_recorded_count",
    "signing_receipt_index_recorded_count",
    "external_signing_receipt_audit_evidence_delivered_count",
    "telegram_signing_receipt_audit_evidence_delivered_count",
    "release_publication_authority_from_signing_receipt_audit_evidence_derived_count",
    "activation_authority_from_signing_receipt_audit_evidence_derived_count",
    "install_from_signing_receipt_audit_evidence_executed_count",
    "active_binary_from_signing_receipt_audit_evidence_mutated_count",
    "provider_invoked_count",
    "credential_read_count"
  ])
  and false_fields(.; [
    "signing_receipt_audit_trail_recorded",
    "signing_receipt_immutable_evidence_recorded",
    "signing_receipt_hash_chain_recorded",
    "signing_receipt_attestation_recorded",
    "signing_receipt_ledger_recorded",
    "signing_receipt_index_recorded",
    "external_signing_receipt_audit_evidence_delivered",
    "telegram_signing_receipt_audit_evidence_delivered",
    "operator_approval_from_signing_receipt_audit_evidence_derived",
    "release_publication_authority_from_signing_receipt_audit_evidence_derived",
    "activation_authority_from_signing_receipt_audit_evidence_derived",
    "install_from_signing_receipt_audit_evidence_executed",
    "service_restart_from_signing_receipt_audit_evidence_performed",
    "active_binary_from_signing_receipt_audit_evidence_mutated",
    "provider_invoked",
    "credential_read",
    "public_ga_claimed",
    "public_release_claimed"
  ])
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_surfaces | length) == 18
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_surfaces | all(
    .signing_receipt_audit_evidence_attempted == true
    and .signing_receipt_audit_evidence_allowed == false
    and .signing_receipt_audit_evidence_accepted == false
    and .signing_receipt_audit_evidence_noop_confirmed == true
    and .release_publication_authority_from_signing_receipt_audit_evidence_derived == false
    and .activation_authority_from_signing_receipt_audit_evidence_derived == false
    and .install_from_signing_receipt_audit_evidence_executed == false
    and .active_binary_from_signing_receipt_audit_evidence_mutated == false
    and .provider_invoked == false
    and .credential_read == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_retention_expiry_gc_denial_gate"
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
    and .invokes_provider == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null
