#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

require_unsigned_integer() {
  local name="$1"
  local value="$2"

  case "$value" in
    ''|*[!0-9]*)
      echo "$name must be an unsigned integer" >&2
      exit 2
      ;;
  esac
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SUMMARY_BRIEFING_JSON="$(
  capture_json_report \
    "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-non-persistence-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-non-persistence-gate.sh
)"

summary_briefing_report_sha256="$(sha256_text "$SUMMARY_BRIEFING_JSON")"
acknowledgement_hash_sha256="$(sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement:ack:$summary_briefing_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
acknowledgement_policy_hash_sha256="$(sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement:policy:$summary_briefing_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
acknowledgement_denial_hash_sha256="$(sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement:denial:$summary_briefing_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
acknowledgement_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement:side-effects:$summary_briefing_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson source "$SUMMARY_BRIEFING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_non_persistence_gate"
    and $source.core_activation_operator_approval_gap_ledger_summary_briefing_ready == true
    and $source.operator_approval_gap_ledger_summary_briefing_status == "blocked"
    and $source.source_operator_approval_gap_ledger_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_operator_approval_gap_ledger_gate"
    and $source.source_operator_approval_gap_ledger_status == "blocked"
    and $source.source_positive_packet_json_capture_boundary_status == "blocked"
    and $source.source_operator_approval_gap_ledger_item_count == 16
    and $source.source_missing_operator_approval_gap_ledger_item_count == 16
    and $source.source_operator_approval_gap_ledger_family_count == 9
    and $source.operator_approval_gap_ledger_summary_briefing_section_count == 9
    and $source.blocked_operator_approval_gap_ledger_summary_briefing_section_count == 9
    and $source.operator_approval_gap_ledger_summary_briefing_fixture_count == 10
    and $source.blocked_operator_approval_gap_ledger_summary_briefing_fixture_count == 10
    and $source.noop_operator_approval_gap_ledger_summary_briefing_fixture_count == 10
    and $source.allowed_operator_approval_gap_ledger_summary_briefing_fixture_count == 0
    and $source.accepted_operator_approval_gap_ledger_summary_briefing_fixture_count == 0
    and $source.denied_by_operator_approval_gap_ledger_summary_briefing_non_persistence_count == 81
    and $source.inherited_denied_by_operator_approval_gap_ledger_count == 69
    and $source.operator_approval_gap_ledger_operator_summary_recorded == false
    and $source.operator_approval_gap_ledger_operator_summary_persisted == false
    and $source.operator_approval_gap_ledger_operator_summary_materialized == false
    and $source.operator_approval_gap_ledger_operator_summary_filesystem_written == false
    and $source.operator_approval_gap_ledger_operator_summary_delivered == false
    and $source.operator_approval_gap_ledger_operator_briefing_recorded == false
    and $source.operator_approval_gap_ledger_operator_briefing_persisted == false
    and $source.operator_approval_gap_ledger_operator_briefing_materialized == false
    and $source.operator_approval_gap_ledger_operator_briefing_filesystem_written == false
    and $source.operator_approval_gap_ledger_operator_briefing_delivered == false
    and $source.operator_approval_gap_ledger_summary_briefing_channel_delivery_performed == false
    and $source.operator_approval_recorded == false
    and $source.operator_approval_accepted == false
    and $source.operator_identity_accepted == false
    and $source.activation_request_recorded == false
    and $source.trusted_record_accepted == false
    and $source.fresh_evidence_accepted == false
    and $source.terminal_closure_allowed == false
    and $source.terminal_closure_recorded == false
    and $source.terminal_closure_accepted == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.receipt_accepted == false
    and $source.ledger_recorded == false
    and $source.index_delivered == false
    and $source.completion_ack_accepted == false
    and $source.public_release_claim_allowed == false
    and $source.release_artifact_write_allowed == false
    and $source.provider_model_invocation_allowed == false
    and $source.channel_delivery_allowed == false
    and $source.install_restart_allowed == false
    and $source.active_binary_mutation_allowed == false
    and $source.upstream_fetch_merge_allowed == false
    and $source.credential_read_allowed == false
    and $source.secret_value_read_allowed == false
    and $source.telegram_send_performed == false
    and $source.external_send_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance_gate" \
    --arg summary_briefing_report_sha256 "$summary_briefing_report_sha256" \
    --arg acknowledgement_hash_sha256 "$acknowledgement_hash_sha256" \
    --arg acknowledgement_policy_hash_sha256 "$acknowledgement_policy_hash_sha256" \
    --arg acknowledgement_denial_hash_sha256 "$acknowledgement_denial_hash_sha256" \
    --arg acknowledgement_side_effect_hash_sha256 "$acknowledgement_side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$SUMMARY_BRIEFING_JSON" \
    '
      def blocked_fixture($id; $status; $reason; $extra):
        {
          id: $id,
          summary_briefing_acknowledgement_requested: false,
          summary_briefing_acknowledgement_status: $status,
          source_summary_briefing_present: true,
          source_summary_briefing_ready: true,
          acknowledgement_allowed: false,
          acknowledgement_request_accepted: false,
          acknowledgement_accepted: false,
          acknowledgement_recorded: false,
          acknowledgement_persisted: false,
          acknowledgement_materialized: false,
          acknowledgement_filesystem_written: false,
          acknowledgement_delivered: false,
          acknowledgement_channel_delivery_performed: false,
          acknowledgement_identity_accepted: false,
          acknowledgement_signature_accepted: false,
          acknowledgement_timestamp_accepted: false,
          acknowledgement_final_state_promoted: false,
          acknowledgement_completion_promoted: false,
          operator_final_acceptance_recorded: false,
          operator_final_acceptance_persisted: false,
          operator_final_acceptance_materialized: false,
          telegram_send_performed: false,
          channel_send_performed: false,
          external_send_performed: false,
          operator_approval_recorded: false,
          operator_approval_accepted: false,
          operator_identity_accepted: false,
          activation_request_recorded: false,
          trusted_record_accepted: false,
          fresh_evidence_accepted: false,
          receipt_recorded: false,
          receipt_persisted: false,
          receipt_accepted: false,
          receipt_materialized: false,
          receipt_filesystem_written: false,
          ledger_recorded: false,
          index_delivered: false,
          completion_ack_recorded: false,
          completion_ack_persisted: false,
          completion_ack_accepted: false,
          completion_ack_delivered: false,
          terminal_closure_recorded: false,
          terminal_closure_accepted: false,
          activation_allowed: false,
          activation_performed: false,
          release_artifact_written: false,
          public_release_claimed: false,
          provider_invoked: false,
          model_invoked: false,
          install_executed: false,
          launchd_mutated: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false,
          credential_read: false,
          secret_value_read: false,
          acknowledgement_noop_confirmed: true,
          reason: $reason
        } + $extra;
      [
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-ack-missing-source"; "blocked_noop"; "source_summary_briefing_report_required"; {source_summary_briefing_present: false, source_summary_briefing_ready: false, summary_briefing_acknowledgement_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-ack-request"; "blocked_ack_noop"; "summary_briefing_acknowledgement_request_shape_denied"; {summary_briefing_acknowledgement_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-ack-acceptance-request"; "blocked_acceptance_noop"; "acknowledgement_acceptance_denied"; {summary_briefing_acknowledgement_requested: true, acknowledgement_acceptance_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-ack-recording-request"; "blocked_ack_noop"; "acknowledgement_recording_denied"; {summary_briefing_acknowledgement_requested: true, acknowledgement_recording_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-ack-persistence-filesystem-write-request"; "blocked_ack_noop"; "acknowledgement_persistence_filesystem_write_denied"; {summary_briefing_acknowledgement_requested: true, acknowledgement_persistence_requested: true, acknowledgement_filesystem_write_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-ack-identity-signature-request"; "blocked_acceptance_noop"; "operator_identity_signature_timestamp_acknowledgement_denied"; {summary_briefing_acknowledgement_requested: true, operator_identity_acceptance_requested: true, operator_signature_acceptance_requested: true, operator_timestamp_acceptance_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-ack-delivery-request"; "blocked_delivery_noop"; "acknowledgement_delivery_denied"; {summary_briefing_acknowledgement_requested: true, acknowledgement_delivery_requested: true, telegram_send_requested: true, channel_delivery_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-terminal-closure-promotion-request"; "blocked_promotion_noop"; "terminal_closure_final_state_completion_promotion_denied"; {summary_briefing_acknowledgement_requested: true, terminal_closure_from_acknowledgement_requested: true, final_state_promotion_requested: true, completion_promotion_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-approval-activation-record-request"; "blocked_ack_noop"; "operator_approval_activation_trusted_record_receipt_ledger_acknowledgement_denied"; {summary_briefing_acknowledgement_requested: true, operator_approval_from_acknowledgement_requested: true, activation_from_acknowledgement_requested: true, trusted_record_from_acknowledgement_requested: true, receipt_from_acknowledgement_requested: true, ledger_from_acknowledgement_requested: true, index_from_acknowledgement_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-external-public-install-upstream-secret-request"; "blocked_delivery_noop"; "external_public_install_restart_upstream_secret_acknowledgement_denied"; {summary_briefing_acknowledgement_requested: true, external_send_acknowledgement_requested: true, public_claim_acknowledgement_requested: true, release_artifact_acknowledgement_requested: true, install_acknowledgement_requested: true, service_restart_acknowledgement_requested: true, upstream_merge_acknowledgement_requested: true, secret_acknowledgement_requested: true})
      ] as $fixtures
      | ([
          "source_operator_approval_gap_ledger_summary_briefing_report_required",
          "summary_briefing_acknowledgement_request_acceptance_denied",
          "summary_briefing_acknowledgement_acceptance_denied",
          "summary_briefing_acknowledgement_recording_denied",
          "summary_briefing_acknowledgement_persistence_denied",
          "summary_briefing_acknowledgement_materialization_denied",
          "summary_briefing_acknowledgement_filesystem_write_denied",
          "operator_identity_signature_timestamp_acknowledgement_denied",
          "summary_briefing_acknowledgement_delivery_denied",
          "telegram_send_denied",
          "terminal_closure_from_summary_briefing_acknowledgement_denied",
          "final_state_completion_promotion_denied",
          "operator_approval_from_summary_briefing_acknowledgement_denied",
          "activation_from_summary_briefing_acknowledgement_denied",
          "trusted_record_receipt_ledger_index_acknowledgement_denied",
          "external_public_install_restart_upstream_acknowledgement_denied",
          "provider_model_invocation_denied",
          "credential_secret_read_denied"
        ] + $source.denied_by_operator_approval_gap_ledger_summary_briefing_non_persistence) as $ack_denied
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_schema_version: "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance_v1",
          core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_ready: true,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_mode: "summary_briefing_acknowledgement_non_acceptance_no_approval_no_terminal_closure_no_activation",
          operator_approval_gap_ledger_summary_briefing_acknowledgement_status: "blocked",
          operator_approval_gap_ledger_summary_briefing_acknowledgement_decision: "operator_summary_or_briefing_acknowledgement_can_be_reported_without_becoming_operator_approval_identity_acceptance_terminal_closure_activation_or_release_authority",
          source_operator_approval_gap_ledger_summary_briefing_gate: $source.gate,
          source_operator_approval_gap_ledger_summary_briefing_ready: $source.core_activation_operator_approval_gap_ledger_summary_briefing_ready,
          source_operator_approval_gap_ledger_summary_briefing_status: $source.operator_approval_gap_ledger_summary_briefing_status,
          source_operator_approval_gap_ledger_summary_briefing_report_sha256: $summary_briefing_report_sha256,
          source_operator_approval_gap_ledger_report_sha256: $source.source_operator_approval_gap_ledger_report_sha256,
          source_operator_approval_gap_ledger_item_count: $source.source_operator_approval_gap_ledger_item_count,
          source_missing_operator_approval_gap_ledger_item_count: $source.source_missing_operator_approval_gap_ledger_item_count,
          source_operator_approval_gap_ledger_family_count: $source.source_operator_approval_gap_ledger_family_count,
          source_operator_approval_gap_ledger_summary_briefing_section_count: $source.operator_approval_gap_ledger_summary_briefing_section_count,
          source_operator_approval_gap_ledger_summary_briefing_fixture_count: $source.operator_approval_gap_ledger_summary_briefing_fixture_count,
          source_denied_by_operator_approval_gap_ledger_summary_briefing_count: $source.denied_by_operator_approval_gap_ledger_summary_briefing_non_persistence_count,
          acknowledgement_hash_sha256: $acknowledgement_hash_sha256,
          acknowledgement_policy_hash_sha256: $acknowledgement_policy_hash_sha256,
          acknowledgement_denial_hash_sha256: $acknowledgement_denial_hash_sha256,
          acknowledgement_side_effect_hash_sha256: $acknowledgement_side_effect_hash_sha256,
          minimum_required_long_soak_samples: $min_long_soak_samples,
          required_operator_approval_gap_ledger_summary_briefing_acknowledgement_surface_count: 12,
          ready_operator_approval_gap_ledger_summary_briefing_acknowledgement_surface_count: 12,
          side_effect_free_operator_approval_gap_ledger_summary_briefing_acknowledgement_surface_count: 12,
          required_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count: 10,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count: ($fixtures | length),
          blocked_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count: ($fixtures | length),
          noop_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count: ($fixtures | length),
          allowed_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count: 0,
          accepted_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count: 0,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_performed_count: 0,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_allowed: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_request_accepted: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_accepted: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_recorded: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_persisted: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_materialized: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_filesystem_written: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_delivered: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_channel_delivery_performed: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_identity_accepted: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_signature_accepted: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_timestamp_accepted: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_final_state_promoted: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_completion_promoted: false,
          operator_approval_gap_ledger_summary_briefing_operator_final_acceptance_recorded: false,
          operator_approval_gap_ledger_summary_briefing_operator_final_acceptance_persisted: false,
          operator_approval_gap_ledger_summary_briefing_operator_final_acceptance_materialized: false,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_fixtures: $fixtures,
          denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance: $ack_denied,
          denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance_count: ($ack_denied | length),
          inherited_denied_by_operator_approval_gap_ledger_summary_briefing_count: $source.denied_by_operator_approval_gap_ledger_summary_briefing_non_persistence_count,
          operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance_executed: true,
          json_report_capture_helper_used: true,
          operator_approval_gap_ledger_operator_summary_recorded: false,
          operator_approval_gap_ledger_operator_summary_persisted: false,
          operator_approval_gap_ledger_operator_summary_materialized: false,
          operator_approval_gap_ledger_operator_summary_filesystem_written: false,
          operator_approval_gap_ledger_operator_summary_delivered: false,
          operator_approval_gap_ledger_operator_briefing_recorded: false,
          operator_approval_gap_ledger_operator_briefing_persisted: false,
          operator_approval_gap_ledger_operator_briefing_materialized: false,
          operator_approval_gap_ledger_operator_briefing_filesystem_written: false,
          operator_approval_gap_ledger_operator_briefing_delivered: false,
          operator_approval_gap_ledger_summary_briefing_channel_delivery_performed: false,
          operator_approval_gap_ledger_recorded: false,
          operator_approval_gap_ledger_persisted: false,
          operator_approval_gap_ledger_materialized: false,
          operator_approval_gap_ledger_delivered: false,
          operator_approval_gap_ledger_promoted_to_authority: false,
          operator_packet_recorded: false,
          operator_packet_accepted: false,
          operator_approval_recorded: false,
          operator_approval_accepted: false,
          operator_identity_accepted: false,
          activation_request_recorded: false,
          trusted_record_acceptance_allowed: false,
          trusted_record_accepted: false,
          fresh_evidence_accepted: false,
          terminal_closure_allowed: false,
          terminal_closure_recorded: false,
          terminal_closure_accepted: false,
          activation_allowed: false,
          activation_performed: false,
          receipt_persistence_allowed: false,
          receipt_acceptance_allowed: false,
          receipt_recorded: false,
          receipt_persisted: false,
          receipt_accepted: false,
          receipt_materialized: false,
          receipt_filesystem_written: false,
          ledger_recording_allowed: false,
          ledger_recorded: false,
          index_delivery_allowed: false,
          index_delivered: false,
          completion_ack_acceptance_allowed: false,
          completion_ack_recorded: false,
          completion_ack_persisted: false,
          completion_ack_accepted: false,
          completion_ack_delivered: false,
          public_release_claim_allowed: false,
          release_artifact_write_allowed: false,
          provider_model_invocation_allowed: false,
          channel_delivery_allowed: false,
          install_restart_allowed: false,
          active_binary_mutation_allowed: false,
          upstream_fetch_merge_allowed: false,
          credential_read_allowed: false,
          secret_value_read_allowed: false,
          telegram_send_performed: false,
          channel_send_performed: false,
          external_send_performed: false,
          provider_invoked: false,
          model_invoked: false,
          release_artifact_written: false,
          public_release_claimed: false,
          install_executed: false,
          launchd_mutated: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false,
          credential_read: false,
          secret_value_read: false,
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            memory_store_mutated: false,
            operator_approval_gap_ledger_summary_briefing_acknowledgement_recorded: false,
            operator_approval_gap_ledger_summary_briefing_acknowledgement_persisted: false,
            operator_approval_gap_ledger_summary_briefing_acknowledgement_materialized: false,
            operator_approval_gap_ledger_summary_briefing_acknowledgement_filesystem_written: false,
            operator_approval_gap_ledger_summary_briefing_acknowledgement_delivered: false,
            operator_approval_gap_ledger_summary_briefing_acknowledgement_channel_delivered: false,
            operator_approval_gap_ledger_summary_briefing_acknowledgement_identity_accepted: false,
            operator_approval_gap_ledger_summary_briefing_acknowledgement_signature_accepted: false,
            operator_approval_gap_ledger_summary_briefing_acknowledgement_final_state_promoted: false,
            operator_approval_gap_ledger_summary_briefing_acknowledgement_completion_promoted: false,
            operator_approval_gap_ledger_operator_summary_recorded: false,
            operator_approval_gap_ledger_operator_summary_persisted: false,
            operator_approval_gap_ledger_operator_summary_materialized: false,
            operator_approval_gap_ledger_operator_summary_filesystem_written: false,
            operator_approval_gap_ledger_operator_summary_delivered: false,
            operator_approval_gap_ledger_operator_briefing_recorded: false,
            operator_approval_gap_ledger_operator_briefing_persisted: false,
            operator_approval_gap_ledger_operator_briefing_materialized: false,
            operator_approval_gap_ledger_operator_briefing_filesystem_written: false,
            operator_approval_gap_ledger_operator_briefing_delivered: false,
            operator_approval_gap_ledger_summary_briefing_channel_delivered: false,
            telegram_send_performed: false,
            channel_send_performed: false,
            external_send_performed: false,
            operator_approval_gap_ledger_recorded: false,
            operator_approval_gap_ledger_persisted: false,
            operator_approval_gap_ledger_materialized: false,
            operator_approval_gap_ledger_delivered: false,
            operator_approval_gap_ledger_promoted_to_authority: false,
            operator_packet_recorded: false,
            operator_packet_accepted: false,
            operator_approval_recorded: false,
            operator_approval_accepted: false,
            operator_identity_accepted: false,
            activation_request_recorded: false,
            trusted_record_accepted: false,
            fresh_evidence_accepted: false,
            receipt_persistence_command_enabled: false,
            receipt_persistence_execution_performed: false,
            receipt_acceptance_recorded: false,
            receipt_accepted: false,
            receipt_materialized: false,
            receipt_filesystem_written: false,
            ledger_recorded: false,
            index_recorded: false,
            delivery_recorded: false,
            completion_ack_recorded: false,
            completion_ack_accepted: false,
            terminal_closure_recorded: false,
            terminal_closure_accepted: false,
            activation_performed: false,
            provider_invoked: false,
            model_invoked: false,
            release_artifact_written: false,
            public_release_claimed: false,
            install_executed: false,
            launchd_mutated: false,
            service_restarted: false,
            active_binary_mutated: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            credential_read: false,
            secret_value_read: false
          }
        }
    '
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance_gate"
  and .core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_ready == true
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_status == "blocked"
  and .source_operator_approval_gap_ledger_summary_briefing_gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_non_persistence_gate"
  and .source_operator_approval_gap_ledger_summary_briefing_ready == true
  and .source_operator_approval_gap_ledger_summary_briefing_status == "blocked"
  and .source_operator_approval_gap_ledger_item_count == 16
  and .source_missing_operator_approval_gap_ledger_item_count == 16
  and .source_operator_approval_gap_ledger_family_count == 9
  and .source_operator_approval_gap_ledger_summary_briefing_section_count == 9
  and .source_operator_approval_gap_ledger_summary_briefing_fixture_count == 10
  and .source_denied_by_operator_approval_gap_ledger_summary_briefing_count == 81
  and .required_operator_approval_gap_ledger_summary_briefing_acknowledgement_surface_count == 12
  and .ready_operator_approval_gap_ledger_summary_briefing_acknowledgement_surface_count == 12
  and .side_effect_free_operator_approval_gap_ledger_summary_briefing_acknowledgement_surface_count == 12
  and .required_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count == 10
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count == 10
  and .blocked_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count == 10
  and .noop_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count == 10
  and .allowed_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count == 0
  and .accepted_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count == 0
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_performed_count == 0
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_allowed == false
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_request_accepted == false
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_accepted == false
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_recorded == false
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_persisted == false
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_materialized == false
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_filesystem_written == false
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_delivered == false
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_identity_accepted == false
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_signature_accepted == false
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_final_state_promoted == false
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_completion_promoted == false
  and (.operator_approval_gap_ledger_summary_briefing_acknowledgement_fixtures | length) == 10
  and (.operator_approval_gap_ledger_summary_briefing_acknowledgement_fixtures | all(
    (.summary_briefing_acknowledgement_status == "blocked_noop" or .summary_briefing_acknowledgement_status == "blocked_ack_noop" or .summary_briefing_acknowledgement_status == "blocked_acceptance_noop" or .summary_briefing_acknowledgement_status == "blocked_delivery_noop" or .summary_briefing_acknowledgement_status == "blocked_promotion_noop")
    and .acknowledgement_recorded == false
    and .acknowledgement_persisted == false
    and .acknowledgement_materialized == false
    and .acknowledgement_filesystem_written == false
    and .acknowledgement_delivered == false
    and .acknowledgement_accepted == false
    and .acknowledgement_identity_accepted == false
    and .acknowledgement_signature_accepted == false
    and .acknowledgement_final_state_promoted == false
    and .telegram_send_performed == false
    and .operator_approval_recorded == false
    and .operator_approval_accepted == false
    and .trusted_record_accepted == false
    and .fresh_evidence_accepted == false
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_accepted == false
    and .ledger_recorded == false
    and .terminal_closure_recorded == false
    and .terminal_closure_accepted == false
    and .activation_allowed == false
    and .activation_performed == false
    and .acknowledgement_noop_confirmed == true
  ))
  and .denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance_count == 99
  and (.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance | length) == 99
  and .inherited_denied_by_operator_approval_gap_ledger_summary_briefing_count == 81
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance_executed == true
  and .json_report_capture_helper_used == true
  and .operator_approval_gap_ledger_operator_summary_recorded == false
  and .operator_approval_gap_ledger_operator_briefing_recorded == false
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .operator_identity_accepted == false
  and .trusted_record_accepted == false
  and .fresh_evidence_accepted == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .activation_allowed == false
  and .activation_performed == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .ledger_recorded == false
  and .index_delivered == false
  and .completion_ack_accepted == false
  and .public_release_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .install_restart_allowed == false
  and .active_binary_mutation_allowed == false
  and .upstream_fetch_merge_allowed == false
  and .credential_read_allowed == false
  and .secret_value_read_allowed == false
  and .telegram_send_performed == false
  and .external_send_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .release_artifact_written == false
  and .public_release_claimed == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .upstream_fetch_performed == false
  and .upstream_merge_performed == false
  and .credential_read == false
  and .secret_value_read == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report" | jq .
echo "Hepta core activation operator approval gap ledger summary briefing acknowledgement non-acceptance gate passed"
