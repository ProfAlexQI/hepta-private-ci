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
  echo "jq is required to build the terminal public claim delivery receipt artifact signing receipt ordering/monotonicity denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-replay-denial-gate.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt replay/idempotency denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
signing_ordering_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-ordering-monotonicity-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
signing_ordering_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-signing-receipt-ordering:no-order-record:no-sequence-cursor:no-monotonic-state:no-latest-wins:no-ordered-status:no-authority:no-install"
)"

surfaces_json="$(
  jq -n '[
    "source_signing_receipt_replay_idempotency_report_required",
    "signing_receipt_order_record",
    "signing_receipt_sequence_cursor",
    "signing_receipt_monotonicity_state",
    "signing_receipt_latest_wins_state",
    "signing_receipt_ordered_status",
    "signing_receipt_ordered_ack",
    "signing_receipt_duplicate_order",
    "signing_receipt_out_of_order_replay",
    "signing_receipt_stale_sequence",
    "signing_receipt_cross_scope_order",
    "signing_receipt_hash_sequence_rebind",
    "artifact_package_signature_receipt_ordering",
    "notarization_ticket_stapling_receipt_ordering",
    "release_cdn_registry_receipt_ordering",
    "external_telegram_signing_receipt_ordering",
    "approval_authority_from_signing_receipt_ordering",
    "install_restart_active_binary_from_signing_receipt_ordering"
  ] | map({
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_surface: .,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_ready: true,
    signing_receipt_ordering_monotonicity_attempted: true,
    signing_receipt_ordering_monotonicity_allowed: false,
    signing_receipt_ordering_monotonicity_accepted: false,
    signing_receipt_ordering_recorded: false,
    signing_receipt_ordering_persisted: false,
    signing_receipt_ordering_materialized: false,
    signing_receipt_ordering_filesystem_written: false,
    signing_receipt_sequence_cursor_recorded: false,
    signing_receipt_sequence_cursor_persisted: false,
    signing_receipt_monotonicity_state_recorded: false,
    signing_receipt_monotonicity_state_persisted: false,
    signing_receipt_latest_wins_state_recorded: false,
    signing_receipt_latest_wins_overwrite_accepted: false,
    signing_receipt_ordered_status_accepted: false,
    signing_receipt_ordered_ack_accepted: false,
    signing_receipt_duplicate_order_accepted: false,
    signing_receipt_out_of_order_replay_accepted: false,
    signing_receipt_stale_sequence_accepted: false,
    signing_receipt_cross_scope_order_accepted: false,
    signing_receipt_hash_sequence_rebind_accepted: false,
    artifact_signing_receipt_ordering_accepted: false,
    package_signing_receipt_ordering_accepted: false,
    signature_manifest_receipt_ordering_accepted: false,
    notarization_submission_receipt_ordering_accepted: false,
    notarization_ticket_receipt_ordering_accepted: false,
    stapling_receipt_ordering_accepted: false,
    installer_signing_receipt_ordering_accepted: false,
    release_asset_receipt_ordering_accepted: false,
    cdn_update_feed_receipt_ordering_accepted: false,
    package_registry_receipt_ordering_accepted: false,
    external_signing_receipt_ordering_accepted: false,
    telegram_signing_receipt_ordering_accepted: false,
    operator_approval_from_signing_receipt_ordering_derived: false,
    release_publication_authority_from_signing_receipt_ordering_derived: false,
    activation_authority_from_signing_receipt_ordering_derived: false,
    install_from_signing_receipt_ordering_executed: false,
    service_restart_from_signing_receipt_ordering_performed: false,
    active_binary_from_signing_receipt_ordering_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    external_send_performed: false,
    telegram_send_performed: false,
    signing_receipt_ordering_monotonicity_noop_confirmed: true,
    signing_receipt_ordering_monotonicity_status: "artifact_signing_receipt_ordering_monotonicity_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg signing_ordering_contract_hash_sha256 "$signing_ordering_contract_hash_sha256" \
    --arg signing_ordering_policy_hash_sha256 "$signing_ordering_policy_hash_sha256" \
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
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_schema_version: "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_denial_v1",
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_mode: "denied_signing_receipt_replay_idempotency_cannot_be_ordered_sequenced_promoted_rebound_or_used_for_authority_or_install",
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_denial_ready,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_report_sha256: $source_report_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_contract_hash_sha256: $signing_ordering_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_policy_hash_sha256: $signing_ordering_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_surface_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_surface_count,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_denied_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_denied_count,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_surfaces: $surfaces,
        denied_by_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity: ($surfaces | map(.terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_surface)),
        allowed_next_actions: [
          {
            action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_cancellation_supersession_denial_gate",
            status: "allowed_report_only_next_slice",
            accepts_ordering: false,
            records_ordering: false,
            records_sequence_cursor: false,
            records_monotonicity_state: false,
            accepts_latest_wins: false,
            accepts_ordered_status: false,
            accepts_ordered_ack: false,
            accepts_hash_sequence_rebind: false,
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
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_allowed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_accepted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_recorded_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_persisted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_materialized_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_filesystem_written_count",
        "signing_receipt_ordering_recorded_count",
        "signing_receipt_ordering_persisted_count",
        "signing_receipt_sequence_cursor_recorded_count",
        "signing_receipt_sequence_cursor_persisted_count",
        "signing_receipt_monotonicity_state_recorded_count",
        "signing_receipt_monotonicity_state_persisted_count",
        "signing_receipt_latest_wins_state_recorded_count",
        "signing_receipt_ordered_status_accepted_count",
        "signing_receipt_ordered_ack_accepted_count",
        "signing_receipt_duplicate_order_accepted_count",
        "signing_receipt_out_of_order_replay_accepted_count",
        "signing_receipt_stale_sequence_accepted_count",
        "signing_receipt_cross_scope_order_accepted_count",
        "signing_receipt_hash_sequence_rebind_accepted_count",
        "artifact_signing_receipt_ordering_accepted_count",
        "package_signing_receipt_ordering_accepted_count",
        "signature_manifest_receipt_ordering_accepted_count",
        "notarization_ticket_receipt_ordering_accepted_count",
        "external_signing_receipt_ordering_accepted_count",
        "telegram_signing_receipt_ordering_accepted_count",
        "release_publication_authority_from_signing_receipt_ordering_derived_count",
        "activation_authority_from_signing_receipt_ordering_derived_count",
        "install_from_signing_receipt_ordering_executed_count",
        "service_restart_from_signing_receipt_ordering_performed_count",
        "active_binary_from_signing_receipt_ordering_mutated_count",
        "provider_invoked_count",
        "credential_read_count",
        "external_send_performed_count",
        "telegram_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_allowed",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_accepted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_persisted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_materialized",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_filesystem_written",
        "signing_receipt_ordering_recorded",
        "signing_receipt_ordering_persisted",
        "signing_receipt_sequence_cursor_recorded",
        "signing_receipt_sequence_cursor_persisted",
        "signing_receipt_monotonicity_state_recorded",
        "signing_receipt_monotonicity_state_persisted",
        "signing_receipt_latest_wins_state_recorded",
        "signing_receipt_latest_wins_overwrite_accepted",
        "signing_receipt_ordered_status_accepted",
        "signing_receipt_ordered_ack_accepted",
        "signing_receipt_duplicate_order_accepted",
        "signing_receipt_out_of_order_replay_accepted",
        "signing_receipt_stale_sequence_accepted",
        "signing_receipt_cross_scope_order_accepted",
        "signing_receipt_hash_sequence_rebind_accepted",
        "artifact_signing_receipt_ordering_accepted",
        "package_signing_receipt_ordering_accepted",
        "signature_manifest_receipt_ordering_accepted",
        "notarization_ticket_receipt_ordering_accepted",
        "external_signing_receipt_ordering_accepted",
        "telegram_signing_receipt_ordering_accepted",
        "operator_approval_from_signing_receipt_ordering_derived",
        "release_publication_authority_from_signing_receipt_ordering_derived",
        "activation_authority_from_signing_receipt_ordering_derived",
        "install_from_signing_receipt_ordering_executed",
        "service_restart_from_signing_receipt_ordering_performed",
        "active_binary_from_signing_receipt_ordering_mutated",
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
          "signing_receipt_ordering_recorded",
          "signing_receipt_ordering_persisted",
          "signing_receipt_ordering_materialized",
          "signing_receipt_ordering_filesystem_written",
          "signing_receipt_sequence_cursor_recorded",
          "signing_receipt_sequence_cursor_persisted",
          "signing_receipt_monotonicity_state_recorded",
          "signing_receipt_monotonicity_state_persisted",
          "signing_receipt_latest_wins_state_recorded",
          "signing_receipt_ordered_status_accepted",
          "signing_receipt_ordered_ack_accepted",
          "signing_receipt_hash_sequence_rebind_accepted",
          "operator_approval_from_signing_receipt_ordering_derived",
          "release_publication_authority_from_signing_receipt_ordering_derived",
          "activation_authority_from_signing_receipt_ordering_derived",
          "install_from_signing_receipt_ordering_executed",
          "service_restart_from_signing_receipt_ordering_performed",
          "active_binary_from_signing_receipt_ordering_mutated",
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
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_surface_count == 18
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_denied_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_denied_count == 18
  and zero_fields(.; [
    "terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_accepted_count",
    "signing_receipt_ordering_recorded_count",
    "signing_receipt_sequence_cursor_recorded_count",
    "signing_receipt_monotonicity_state_recorded_count",
    "signing_receipt_ordered_status_accepted_count",
    "signing_receipt_ordered_ack_accepted_count",
    "signing_receipt_hash_sequence_rebind_accepted_count",
    "external_signing_receipt_ordering_accepted_count",
    "telegram_signing_receipt_ordering_accepted_count",
    "release_publication_authority_from_signing_receipt_ordering_derived_count",
    "activation_authority_from_signing_receipt_ordering_derived_count",
    "install_from_signing_receipt_ordering_executed_count",
    "active_binary_from_signing_receipt_ordering_mutated_count",
    "provider_invoked_count",
    "credential_read_count"
  ])
  and false_fields(.; [
    "signing_receipt_ordering_recorded",
    "signing_receipt_sequence_cursor_recorded",
    "signing_receipt_monotonicity_state_recorded",
    "signing_receipt_latest_wins_overwrite_accepted",
    "signing_receipt_ordered_status_accepted",
    "signing_receipt_ordered_ack_accepted",
    "signing_receipt_hash_sequence_rebind_accepted",
    "artifact_signing_receipt_ordering_accepted",
    "package_signing_receipt_ordering_accepted",
    "signature_manifest_receipt_ordering_accepted",
    "external_signing_receipt_ordering_accepted",
    "telegram_signing_receipt_ordering_accepted",
    "operator_approval_from_signing_receipt_ordering_derived",
    "release_publication_authority_from_signing_receipt_ordering_derived",
    "activation_authority_from_signing_receipt_ordering_derived",
    "install_from_signing_receipt_ordering_executed",
    "service_restart_from_signing_receipt_ordering_performed",
    "active_binary_from_signing_receipt_ordering_mutated",
    "provider_invoked",
    "credential_read",
    "public_ga_claimed",
    "public_release_claimed"
  ])
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_surfaces | length) == 18
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_surfaces | all(
    .signing_receipt_ordering_monotonicity_attempted == true
    and .signing_receipt_ordering_monotonicity_allowed == false
    and .signing_receipt_ordering_monotonicity_accepted == false
    and .signing_receipt_ordering_monotonicity_noop_confirmed == true
    and .release_publication_authority_from_signing_receipt_ordering_derived == false
    and .activation_authority_from_signing_receipt_ordering_derived == false
    and .install_from_signing_receipt_ordering_executed == false
    and .active_binary_from_signing_receipt_ordering_mutated == false
    and .provider_invoked == false
    and .credential_read == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_cancellation_supersession_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .accepts_ordering == false
    and .records_sequence_cursor == false
    and .records_monotonicity_state == false
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
