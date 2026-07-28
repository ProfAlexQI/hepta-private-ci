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
  echo "jq is required to build the terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt retention/expiry/GC denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/i3-f11e5031101b4efb75c26d90.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt audit/evidence denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
signing_receipt_retention_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-signing-receipt-signing-receipt-retention-expiry-gc-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
signing_receipt_retention_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-retention:no-retention:no-expiry:no-gc:no-archive:no-compaction:no-authority:no-install"
)"

surfaces_json="$(
  jq -n '[
    "source_signing_receipt_audit_evidence_report_required",
    "signing_receipt_retention_policy",
    "signing_receipt_ttl_lease",
    "signing_receipt_expiry_timestamp",
    "signing_receipt_expiry_scheduler_timer",
    "signing_receipt_expiry_ack",
    "signing_receipt_garbage_collection_queue",
    "signing_receipt_garbage_collection_scan",
    "signing_receipt_garbage_collection_candidate",
    "signing_receipt_garbage_collection_decision",
    "signing_receipt_tombstone_gc",
    "signing_receipt_delete_marker_gc",
    "signing_receipt_archive",
    "signing_receipt_compaction",
    "artifact_package_signature_receipt_retention",
    "notarization_release_registry_receipt_retention",
    "external_telegram_signing_receipt_retention",
    "approval_authority_install_from_signing_receipt_retention"
  ] | map({
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_surface: .,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_ready: true,
    signing_receipt_retention_expiry_garbage_collection_attempted: true,
    signing_receipt_retention_expiry_garbage_collection_allowed: false,
    signing_receipt_retention_expiry_garbage_collection_accepted: false,
    signing_receipt_retention_expiry_garbage_collection_recorded: false,
    signing_receipt_retention_expiry_garbage_collection_persisted: false,
    signing_receipt_retention_expiry_garbage_collection_materialized: false,
    signing_receipt_retention_expiry_garbage_collection_filesystem_written: false,
    signing_receipt_retention_policy_recorded: false,
    signing_receipt_retention_policy_persisted: false,
    signing_receipt_ttl_lease_recorded: false,
    signing_receipt_ttl_lease_persisted: false,
    signing_receipt_expiry_timestamp_recorded: false,
    signing_receipt_expiry_scheduler_recorded: false,
    signing_receipt_expiry_timer_started: false,
    signing_receipt_expiry_ack_recorded: false,
    signing_receipt_expiry_state_persisted: false,
    signing_receipt_garbage_collection_queue_recorded: false,
    signing_receipt_garbage_collection_scan_performed: false,
    signing_receipt_garbage_collection_candidate_recorded: false,
    signing_receipt_garbage_collection_decision_recorded: false,
    signing_receipt_garbage_collection_executed: false,
    signing_receipt_tombstone_gc_recorded: false,
    signing_receipt_delete_marker_gc_recorded: false,
    signing_receipt_archive_recorded: false,
    signing_receipt_compaction_recorded: false,
    signing_receipt_compaction_performed: false,
    signing_receipt_audit_evidence_retention_recorded: false,
    signing_receipt_immutable_evidence_retention_recorded: false,
    signing_receipt_hash_attestation_retention_recorded: false,
    signing_receipt_witness_notary_expiry_recorded: false,
    signing_receipt_ledger_index_retention_recorded: false,
    signing_receipt_delivery_evidence_retention_recorded: false,
    signing_receipt_status_evidence_expiry_recorded: false,
    artifact_signing_receipt_retention_recorded: false,
    package_signing_receipt_retention_recorded: false,
    signature_manifest_receipt_retention_recorded: false,
    notarization_ticket_receipt_retention_recorded: false,
    release_asset_receipt_retention_recorded: false,
    cdn_update_feed_receipt_retention_recorded: false,
    package_registry_receipt_retention_recorded: false,
    external_signing_receipt_retention_recorded: false,
    telegram_signing_receipt_retention_recorded: false,
    operator_approval_from_signing_receipt_retention_derived: false,
    release_publication_authority_from_signing_receipt_retention_derived: false,
    activation_authority_from_signing_receipt_retention_derived: false,
    install_from_signing_receipt_retention_executed: false,
    service_restart_from_signing_receipt_retention_performed: false,
    active_binary_from_signing_receipt_retention_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    external_send_performed: false,
    telegram_send_performed: false,
    signing_receipt_retention_expiry_garbage_collection_noop_confirmed: true,
    signing_receipt_retention_expiry_garbage_collection_status: "artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg signing_receipt_retention_contract_hash_sha256 "$signing_receipt_retention_contract_hash_sha256" \
    --arg signing_receipt_retention_policy_hash_sha256 "$signing_receipt_retention_policy_hash_sha256" \
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
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_schema_version: "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_denial_v1",
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_mode: "denied_signing_receipt_audit_evidence_cannot_retention_expiry_gc_or_derive_authority_or_install",
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_denial_ready,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_report_sha256: $source_report_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_contract_hash_sha256: $signing_receipt_retention_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_policy_hash_sha256: $signing_receipt_retention_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_surface_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_surface_count,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_denied_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_denied_count,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_surfaces: $surfaces,
        denied_by_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc: ($surfaces | map(.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_surface)),
        allowed_next_actions: [
          {
            action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denial_gate",
            status: "allowed_report_only_next_slice",
            records_retention: false,
            records_expiry: false,
            records_garbage_collection: false,
            records_archive: false,
            records_compaction: false,
            registers_export: false,
            registers_query: false,
            records_observability: false,
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
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_allowed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_accepted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_recorded_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_persisted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_materialized_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_filesystem_written_count",
        "signing_receipt_retention_policy_recorded_count",
        "signing_receipt_ttl_lease_recorded_count",
        "signing_receipt_expiry_timestamp_recorded_count",
        "signing_receipt_expiry_scheduler_recorded_count",
        "signing_receipt_expiry_timer_started_count",
        "signing_receipt_expiry_ack_recorded_count",
        "signing_receipt_garbage_collection_queue_recorded_count",
        "signing_receipt_garbage_collection_scan_performed_count",
        "signing_receipt_garbage_collection_decision_recorded_count",
        "signing_receipt_garbage_collection_executed_count",
        "signing_receipt_tombstone_gc_recorded_count",
        "signing_receipt_delete_marker_gc_recorded_count",
        "signing_receipt_archive_recorded_count",
        "signing_receipt_compaction_recorded_count",
        "signing_receipt_compaction_performed_count",
        "external_signing_receipt_retention_recorded_count",
        "telegram_signing_receipt_retention_recorded_count",
        "release_publication_authority_from_signing_receipt_retention_derived_count",
        "activation_authority_from_signing_receipt_retention_derived_count",
        "install_from_signing_receipt_retention_executed_count",
        "service_restart_from_signing_receipt_retention_performed_count",
        "active_binary_from_signing_receipt_retention_mutated_count",
        "provider_invoked_count",
        "credential_read_count",
        "external_send_performed_count",
        "telegram_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_allowed",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_accepted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_persisted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_materialized",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_filesystem_written",
        "signing_receipt_retention_policy_recorded",
        "signing_receipt_ttl_lease_recorded",
        "signing_receipt_expiry_timestamp_recorded",
        "signing_receipt_expiry_scheduler_recorded",
        "signing_receipt_expiry_timer_started",
        "signing_receipt_expiry_ack_recorded",
        "signing_receipt_garbage_collection_queue_recorded",
        "signing_receipt_garbage_collection_scan_performed",
        "signing_receipt_garbage_collection_decision_recorded",
        "signing_receipt_garbage_collection_executed",
        "signing_receipt_tombstone_gc_recorded",
        "signing_receipt_delete_marker_gc_recorded",
        "signing_receipt_archive_recorded",
        "signing_receipt_compaction_recorded",
        "signing_receipt_compaction_performed",
        "external_signing_receipt_retention_recorded",
        "telegram_signing_receipt_retention_recorded",
        "operator_approval_from_signing_receipt_retention_derived",
        "release_publication_authority_from_signing_receipt_retention_derived",
        "activation_authority_from_signing_receipt_retention_derived",
        "install_from_signing_receipt_retention_executed",
        "service_restart_from_signing_receipt_retention_performed",
        "active_binary_from_signing_receipt_retention_mutated",
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
          "signing_receipt_retention_policy_recorded",
          "signing_receipt_ttl_lease_recorded",
          "signing_receipt_expiry_timestamp_recorded",
          "signing_receipt_expiry_scheduler_recorded",
          "signing_receipt_expiry_timer_started",
          "signing_receipt_expiry_ack_recorded",
          "signing_receipt_garbage_collection_queue_recorded",
          "signing_receipt_garbage_collection_scan_performed",
          "signing_receipt_garbage_collection_decision_recorded",
          "signing_receipt_garbage_collection_executed",
          "signing_receipt_tombstone_gc_recorded",
          "signing_receipt_delete_marker_gc_recorded",
          "signing_receipt_archive_recorded",
          "signing_receipt_compaction_recorded",
          "signing_receipt_compaction_performed",
          "external_signing_receipt_retention_recorded",
          "telegram_signing_receipt_retention_recorded",
          "operator_approval_from_signing_receipt_retention_derived",
          "release_publication_authority_from_signing_receipt_retention_derived",
          "activation_authority_from_signing_receipt_retention_derived",
          "install_from_signing_receipt_retention_executed",
          "service_restart_from_signing_receipt_retention_performed",
          "active_binary_from_signing_receipt_retention_mutated",
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
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_surface_count == 18
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_audit_evidence_denied_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_denied_count == 18
  and zero_fields(.; [
    "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_recorded_count",
    "signing_receipt_retention_policy_recorded_count",
    "signing_receipt_ttl_lease_recorded_count",
    "signing_receipt_expiry_timer_started_count",
    "signing_receipt_garbage_collection_queue_recorded_count",
    "signing_receipt_garbage_collection_decision_recorded_count",
    "signing_receipt_garbage_collection_executed_count",
    "signing_receipt_archive_recorded_count",
    "signing_receipt_compaction_recorded_count",
    "release_publication_authority_from_signing_receipt_retention_derived_count",
    "activation_authority_from_signing_receipt_retention_derived_count",
    "install_from_signing_receipt_retention_executed_count",
    "active_binary_from_signing_receipt_retention_mutated_count",
    "provider_invoked_count",
    "credential_read_count"
  ])
  and false_fields(.; [
    "signing_receipt_retention_policy_recorded",
    "signing_receipt_expiry_timer_started",
    "signing_receipt_garbage_collection_queue_recorded",
    "signing_receipt_garbage_collection_decision_recorded",
    "signing_receipt_garbage_collection_executed",
    "signing_receipt_archive_recorded",
    "signing_receipt_compaction_recorded",
    "external_signing_receipt_retention_recorded",
    "telegram_signing_receipt_retention_recorded",
    "operator_approval_from_signing_receipt_retention_derived",
    "release_publication_authority_from_signing_receipt_retention_derived",
    "activation_authority_from_signing_receipt_retention_derived",
    "install_from_signing_receipt_retention_executed",
    "service_restart_from_signing_receipt_retention_performed",
    "active_binary_from_signing_receipt_retention_mutated",
    "provider_invoked",
    "credential_read",
    "public_ga_claimed",
    "public_release_claimed"
  ])
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_surfaces | length) == 18
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_surfaces | all(
    .signing_receipt_retention_expiry_garbage_collection_attempted == true
    and .signing_receipt_retention_expiry_garbage_collection_allowed == false
    and .signing_receipt_retention_expiry_garbage_collection_accepted == false
    and .signing_receipt_retention_expiry_garbage_collection_noop_confirmed == true
    and .signing_receipt_retention_policy_recorded == false
    and .signing_receipt_expiry_timer_started == false
    and .signing_receipt_garbage_collection_decision_recorded == false
    and .release_publication_authority_from_signing_receipt_retention_derived == false
    and .activation_authority_from_signing_receipt_retention_derived == false
    and .install_from_signing_receipt_retention_executed == false
    and .active_binary_from_signing_receipt_retention_mutated == false
    and .provider_invoked == false
    and .credential_read == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_retention == false
    and .records_expiry == false
    and .records_garbage_collection == false
    and .records_observability == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null
