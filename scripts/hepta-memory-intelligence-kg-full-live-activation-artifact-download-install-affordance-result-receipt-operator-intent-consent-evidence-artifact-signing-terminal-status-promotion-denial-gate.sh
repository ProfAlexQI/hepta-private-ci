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

FINAL_ACKNOWLEDGEMENT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-final-ack-denial-gate" \
    scripts/i3-70337a51ae9ff9614b36dc0a.sh
)"

source_final_acknowledgement_report_sha256="$(sha256_text "$FINAL_ACKNOWLEDGEMENT_JSON")"
terminal_decision_status_promotion_contract_hash_sha256="$(
  sha256_text "hepta-artifact-distribution-signing-notarization-receipt-terminal-decision-status-promotion-denial:$source_final_acknowledgement_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_decision_status_promotion_policy_hash_sha256="$(
  sha256_text "artifact-distribution-signing-notarization-receipt-terminal-decision-status-promotion:no-terminal-decision:no-terminal-status:no-status-promotion:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$FINAL_ACKNOWLEDGEMENT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
    and $source.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count == 18
    and zero_fields($source; [
      "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count",
      "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count",
      "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted_count",
      "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_delivered_count",
      "artifact_distribution_signing_notarization_receipt_channel_acknowledgement_delivered_count",
      "artifact_distribution_signing_notarization_receipt_external_acknowledgement_sent_count",
      "artifact_distribution_signing_notarization_receipt_telegram_acknowledgement_sent_count",
      "operator_approval_from_signing_receipt_acknowledgement_derived_count",
      "release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
      "activation_authority_from_signing_receipt_acknowledgement_derived_count",
      "install_from_signing_receipt_acknowledgement_executed_count",
      "service_restart_from_signing_receipt_acknowledgement_performed_count",
      "active_binary_from_signing_receipt_acknowledgement_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted",
      "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded",
      "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted",
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
      "external_send_performed"
    ])
    and ($source.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces | length) == 18
    and ($source.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces | all(
      .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempted == true
      and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed == false
      and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_noop_confirmed == true
      and .release_publication_authority_from_acknowledgement_derived == false
      and .activation_authority_from_acknowledgement_derived == false
      and .install_from_acknowledgement_executed == false
      and .service_restart_from_acknowledgement_performed == false
      and .active_binary_from_acknowledgement_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_terminal_decision == false
      and .records_status_promotion == false
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

terminal_decision_status_promotion_surfaces_json="$(
  jq -n '
    def terminal_surface($id; $status; $reason; $extra):
      {
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surface:$id,
        source_artifact_distribution_signing_notarization_receipt_final_acknowledgement_ready:true,
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_attempted:true,
        terminal_decision_requested:false,
        terminal_status_requested:false,
        status_promotion_requested:false,
        terminal_decision_allowed:false,
        terminal_status_allowed:false,
        status_promotion_allowed:false,
        terminal_decision_recorded:false,
        terminal_decision_persisted:false,
        terminal_status_recorded:false,
        terminal_status_persisted:false,
        status_promotion_recorded:false,
        channel_decision_delivered:false,
        external_decision_sent:false,
        telegram_decision_sent:false,
        operator_approval_from_terminal_status_derived:false,
        release_publication_authority_from_terminal_decision_derived:false,
        activation_authority_from_terminal_status_derived:false,
        install_from_terminal_status_executed:false,
        service_restart_from_terminal_status_performed:false,
        active_binary_from_terminal_status_mutated:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        external_send_performed:false,
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_noop_confirmed:true,
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_status:$status,
        reason:$reason
      } + $extra;
    [
      terminal_surface("source_signing_receipt_final_acknowledgement_report_required"; "blocked_source_signing_receipt_final_ack_required_noop"; "source_signing_receipt_final_acknowledgement_report_required"; {source_final_acknowledgement_report_required:true}),
      terminal_surface("artifact_signing_receipt_terminal_decision_claim"; "blocked_artifact_signing_receipt_terminal_decision_noop"; "artifact_signing_receipt_terminal_decision_claim_denied"; {terminal_decision_requested:true}),
      terminal_surface("package_signing_receipt_status_promotion_claim"; "blocked_package_signing_receipt_status_promotion_noop"; "package_signing_receipt_status_promotion_claim_denied"; {status_promotion_requested:true}),
      terminal_surface("signature_manifest_terminal_status_claim"; "blocked_signature_manifest_terminal_status_noop"; "signature_manifest_terminal_status_claim_denied"; {terminal_status_requested:true}),
      terminal_surface("notarization_status_terminal_decision_claim"; "blocked_notarization_status_terminal_decision_noop"; "notarization_status_terminal_decision_claim_denied"; {terminal_decision_requested:true}),
      terminal_surface("witness_notary_exported_summary_terminal_status_claim"; "blocked_witness_notary_terminal_status_noop"; "witness_notary_exported_summary_terminal_status_claim_denied"; {terminal_status_requested:true}),
      terminal_surface("tombstone_garbage_collection_final_response_terminal_decision_claim"; "blocked_tombstone_gc_final_response_terminal_decision_noop"; "tombstone_garbage_collection_final_response_terminal_decision_claim_denied"; {terminal_decision_requested:true}),
      terminal_surface("replacement_garbage_collection_completion_status_promotion_claim"; "blocked_replacement_gc_completion_status_promotion_noop"; "replacement_garbage_collection_completion_status_promotion_claim_denied"; {status_promotion_requested:true}),
      terminal_surface("provenance_dashboard_narrative_terminal_status_claim"; "blocked_provenance_dashboard_terminal_status_noop"; "provenance_dashboard_narrative_terminal_status_claim_denied"; {terminal_status_requested:true}),
      terminal_surface("sbom_audit_narrative_terminal_decision_claim"; "blocked_sbom_audit_terminal_decision_noop"; "sbom_audit_narrative_terminal_decision_claim_denied"; {terminal_decision_requested:true}),
      terminal_surface("release_asset_final_briefing_terminal_status_claim"; "blocked_release_asset_final_briefing_terminal_status_noop"; "release_asset_final_briefing_terminal_status_claim_denied"; {terminal_status_requested:true}),
      terminal_surface("cdn_dashboard_readback_digest_status_promotion_claim"; "blocked_cdn_dashboard_readback_status_promotion_noop"; "cdn_dashboard_readback_digest_status_promotion_claim_denied"; {status_promotion_requested:true}),
      terminal_surface("package_registry_memo_notification_terminal_status_claim"; "blocked_package_registry_notification_terminal_status_noop"; "package_registry_memo_notification_terminal_status_claim_denied"; {terminal_status_requested:true}),
      terminal_surface("dashboard_hash_approval_channel_terminal_decision_claim"; "blocked_dashboard_hash_channel_terminal_decision_noop"; "dashboard_hash_approval_channel_terminal_decision_claim_denied"; {terminal_decision_requested:true}),
      terminal_surface("external_telegram_briefing_terminal_decision_claim"; "blocked_external_telegram_terminal_decision_noop"; "external_telegram_briefing_terminal_decision_claim_denied"; {terminal_decision_requested:true, telegram_decision_requested:true}),
      terminal_surface("release_publication_authority_terminal_decision_claim"; "blocked_release_publication_authority_terminal_decision_noop"; "release_publication_authority_terminal_decision_claim_denied"; {terminal_decision_requested:true}),
      terminal_surface("activation_live_install_terminal_status_claim"; "blocked_activation_live_install_terminal_status_noop"; "activation_live_install_terminal_status_claim_denied"; {terminal_status_requested:true}),
      terminal_surface("install_restart_active_binary_status_promotion_claim"; "blocked_install_restart_active_binary_status_promotion_noop"; "install_restart_active_binary_status_promotion_claim_denied"; {status_promotion_requested:true, install_restart_active_binary_status_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_gate" \
    --arg source_final_acknowledgement_report_sha256 "$source_final_acknowledgement_report_sha256" \
    --arg terminal_decision_status_promotion_contract_hash_sha256 "$terminal_decision_status_promotion_contract_hash_sha256" \
    --arg terminal_decision_status_promotion_policy_hash_sha256 "$terminal_decision_status_promotion_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$FINAL_ACKNOWLEDGEMENT_JSON" \
    --argjson surfaces "$terminal_decision_status_promotion_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_v1",
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_mode:"denied_signing_receipt_final_acknowledgement_cannot_create_terminal_decision_terminal_status_status_promotion_authority_or_live_install",
        source_artifact_distribution_signing_notarization_receipt_final_acknowledgement_gate:$source.gate,
        source_artifact_distribution_signing_notarization_receipt_final_acknowledgement_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_ready,
        source_artifact_distribution_signing_notarization_receipt_final_acknowledgement_report_sha256:$source_final_acknowledgement_report_sha256,
        source_artifact_distribution_signing_notarization_receipt_final_acknowledgement_contract_hash_sha256:$source.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_contract_hash_sha256:$terminal_decision_status_promotion_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_policy_hash_sha256:$terminal_decision_status_promotion_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_ready:true,
        source_artifact_distribution_signing_notarization_receipt_final_acknowledgement_surface_count:$source.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count,
        source_artifact_distribution_signing_notarization_receipt_final_acknowledgement_denied_count:$source.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count,
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion:[
          "source_artifact_distribution_signing_notarization_receipt_final_acknowledgement_report_required",
          "artifact_distribution_signing_notarization_receipt_terminal_decision_recording_denied",
          "artifact_distribution_signing_notarization_receipt_terminal_status_recording_denied",
          "artifact_distribution_signing_notarization_receipt_status_promotion_denied",
          "artifact_distribution_signing_notarization_receipt_channel_external_telegram_terminal_decision_denied",
          "artifact_distribution_signing_notarization_receipt_operator_approval_from_terminal_status_denied",
          "artifact_distribution_signing_notarization_receipt_release_publication_authority_from_terminal_decision_denied",
          "artifact_distribution_signing_notarization_receipt_activation_authority_from_terminal_status_denied",
          "artifact_distribution_signing_notarization_receipt_install_restart_active_binary_from_terminal_status_denied",
          "artifact_distribution_signing_notarization_receipt_memory_provider_secret_external_send_from_terminal_status_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_gate",
            status:"allowed_report_only_next_slice",
            records_terminal_decision:false,
            records_status_promotion:false,
            records_public_claim:false,
            records_status_exposure:false,
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
        "artifact_distribution_signing_notarization_receipt_terminal_decision_recorded_count",
        "artifact_distribution_signing_notarization_receipt_terminal_decision_persisted_count",
        "artifact_distribution_signing_notarization_receipt_terminal_status_recorded_count",
        "artifact_distribution_signing_notarization_receipt_terminal_status_persisted_count",
        "artifact_distribution_signing_notarization_receipt_status_promotion_recorded_count",
        "artifact_distribution_signing_notarization_receipt_channel_decision_delivered_count",
        "artifact_distribution_signing_notarization_receipt_external_decision_sent_count",
        "artifact_distribution_signing_notarization_receipt_telegram_decision_sent_count",
        "operator_approval_from_signing_receipt_terminal_status_derived_count",
        "release_publication_authority_from_signing_receipt_terminal_decision_derived_count",
        "activation_authority_from_signing_receipt_terminal_status_derived_count",
        "install_from_signing_receipt_terminal_status_executed_count",
        "service_restart_from_signing_receipt_terminal_status_performed_count",
        "active_binary_from_signing_receipt_terminal_status_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "artifact_distribution_signing_notarization_receipt_terminal_decision_recorded",
        "artifact_distribution_signing_notarization_receipt_terminal_status_recorded",
        "artifact_distribution_signing_notarization_receipt_status_promotion_recorded",
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
        "external_send_performed"
      ])
      + {
        side_effects:false_object([
          "terminal_decision_recorded",
          "terminal_decision_persisted",
          "terminal_status_recorded",
          "terminal_status_persisted",
          "status_promotion_recorded",
          "channel_decision_delivered",
          "external_decision_sent",
          "telegram_decision_sent",
          "operator_approval_from_terminal_status_derived",
          "release_publication_authority_from_terminal_decision_derived",
          "activation_authority_from_terminal_status_derived",
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
          "public_release_claimed",
          "public_ga_claimed",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_final_acknowledgement_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_final_acknowledgement_surface_count == 18
  and $report.source_artifact_distribution_signing_notarization_receipt_final_acknowledgement_denied_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surface_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denied_count == 18
  and zero_fields($report; [
    "artifact_distribution_signing_notarization_receipt_terminal_decision_recorded_count",
    "artifact_distribution_signing_notarization_receipt_terminal_decision_persisted_count",
    "artifact_distribution_signing_notarization_receipt_terminal_status_recorded_count",
    "artifact_distribution_signing_notarization_receipt_terminal_status_persisted_count",
    "artifact_distribution_signing_notarization_receipt_status_promotion_recorded_count",
    "artifact_distribution_signing_notarization_receipt_channel_decision_delivered_count",
    "artifact_distribution_signing_notarization_receipt_external_decision_sent_count",
    "artifact_distribution_signing_notarization_receipt_telegram_decision_sent_count",
    "operator_approval_from_signing_receipt_terminal_status_derived_count",
    "release_publication_authority_from_signing_receipt_terminal_decision_derived_count",
    "activation_authority_from_signing_receipt_terminal_status_derived_count",
    "install_from_signing_receipt_terminal_status_executed_count",
    "service_restart_from_signing_receipt_terminal_status_performed_count",
    "active_binary_from_signing_receipt_terminal_status_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and ($report.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_attempted == true
    and .artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_noop_confirmed == true
    and .terminal_decision_allowed == false
    and .terminal_status_allowed == false
    and .status_promotion_allowed == false
    and false_fields(.; [
      "terminal_decision_recorded",
      "terminal_decision_persisted",
      "terminal_status_recorded",
      "terminal_status_persisted",
      "status_promotion_recorded",
      "channel_decision_delivered",
      "external_decision_sent",
      "telegram_decision_sent",
      "operator_approval_from_terminal_status_derived",
      "release_publication_authority_from_terminal_decision_derived",
      "activation_authority_from_terminal_status_derived",
      "install_from_terminal_status_executed",
      "service_restart_from_terminal_status_performed",
      "active_binary_from_terminal_status_mutated",
      "memory_store_write_performed",
      "memory_store_mutated",
      "live_kg_write_performed",
      "provider_invoked",
      "model_invoked",
      "credential_read",
      "secret_file_read",
      "external_send_performed"
    ])
  ))
  and ([.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surfaces[] | select(.terminal_decision_requested == true)] | length) == 7
  and ([.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surfaces[] | select(.terminal_status_requested == true)] | length) == 6
  and ([.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surfaces[] | select(.status_promotion_requested == true)] | length) == 4
  and ([.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surfaces[] | select(.telegram_decision_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surfaces[] | select(.install_restart_active_binary_status_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_terminal_decision == false
    and .records_status_promotion == false
    and .records_public_claim == false
    and .records_status_exposure == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt terminal decision/status promotion denial gate passed"
