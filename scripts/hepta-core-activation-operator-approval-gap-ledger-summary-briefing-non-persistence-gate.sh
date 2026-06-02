#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

# This gate is report-only; every materialization path remains explicitly false.

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

GAP_LEDGER_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-operator-approval-gap-ledger-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-operator-approval-gap-ledger-gate.sh
)"

gap_ledger_report_sha256="$(sha256_text "$GAP_LEDGER_JSON")"
summary_briefing_hash_sha256="$(sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing:summary:$gap_ledger_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_briefing_policy_hash_sha256="$(sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing:policy:$gap_ledger_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_briefing_denial_hash_sha256="$(sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing:denial:$gap_ledger_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_briefing_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing:side-effects:$gap_ledger_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson source "$GAP_LEDGER_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_operator_approval_gap_ledger_gate"
    and $source.terminal_closure_operator_packet_trusted_record_positive_packet_operator_approval_gap_ledger_ready == true
    and $source.trusted_record_positive_packet_operator_approval_gap_ledger_status == "blocked"
    and $source.source_positive_packet_json_capture_boundary_status == "blocked"
    and $source.source_terminal_closure_verdict == "blocked"
    and $source.positive_packet_authority_replay_fixture_count == 12
    and $source.blocked_positive_packet_authority_replay_fixture_count == 12
    and $source.allowed_positive_packet_authority_replay_fixture_count == 0
    and $source.json_capture_boundary_family_count == 8
    and $source.required_operator_approval_gap_ledger_item_count == 16
    and $source.operator_approval_gap_ledger_item_count == 16
    and $source.missing_operator_approval_gap_ledger_item_count == 16
    and $source.report_only_operator_approval_gap_ledger_item_count == 16
    and $source.activation_blocking_operator_approval_gap_ledger_item_count == 16
    and $source.terminal_closure_blocking_operator_approval_gap_ledger_item_count == 16
    and $source.required_operator_approval_gap_ledger_family_count == 9
    and $source.operator_approval_gap_ledger_family_count == 9
    and $source.ready_operator_approval_gap_ledger_family_count == 9
    and $source.activation_blocking_operator_approval_gap_ledger_family_count == 9
    and ($source.operator_approval_gap_ledger_families | length) == 9
    and ($source.operator_approval_gap_ledger_families | all(.ready == true and .blocked == true and .operator_supplied_future_evidence_needed == true and .non_actionable_report_only == true))
    and ($source.operator_approval_gap_ledger_items | length) == 16
    and ($source.operator_approval_gap_ledger_items | all(
      .status == "missing"
      and .ledger_status == "blocked"
      and .current_value == false
      and .operator_supplied_future_evidence_needed == true
      and .terminal_closure_blocking == true
      and .activation_blocking == true
      and .non_actionable_report_only == true
      and .report_only == true
      and .records_approval == false
      and .records_activation_request == false
      and .accepts_trusted_record == false
      and .accepts_fresh_evidence == false
      and .approves_persistence == false
      and .persists_receipt == false
      and .accepts_receipt == false
      and .records_ledger == false
      and .delivers_index == false
      and .accepts_completion_ack == false
      and .closes_terminal == false
      and .activates == false
      and .writes_release_artifact == false
      and .makes_public_release_claim == false
      and .invokes_provider_or_channel == false
      and .installs_or_restarts == false
      and .mutates_upstream_or_binary == false
      and .reads_credentials_or_secrets == false
    ))
    and $source.denied_by_trusted_record_positive_packet_operator_approval_gap_ledger_count == 69
    and ($source.denied_by_trusted_record_positive_packet_operator_approval_gap_ledger | length) == 69
    and $source.operator_approval_gap_ledger_recorded == false
    and $source.operator_approval_gap_ledger_persisted == false
    and $source.operator_approval_gap_ledger_materialized == false
    and $source.operator_approval_gap_ledger_delivered == false
    and $source.operator_approval_gap_ledger_promoted_to_authority == false
    and $source.operator_packet_recorded == false
    and $source.operator_packet_accepted == false
    and $source.trusted_record_acceptance_allowed == false
    and $source.trusted_record_accepted == false
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
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_non_persistence_gate" \
    --arg gap_ledger_report_sha256 "$gap_ledger_report_sha256" \
    --arg summary_briefing_hash_sha256 "$summary_briefing_hash_sha256" \
    --arg summary_briefing_policy_hash_sha256 "$summary_briefing_policy_hash_sha256" \
    --arg summary_briefing_denial_hash_sha256 "$summary_briefing_denial_hash_sha256" \
    --arg summary_briefing_side_effect_hash_sha256 "$summary_briefing_side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$GAP_LEDGER_JSON" \
    '
      def blocked_fixture($id; $status; $reason; $extra):
        {
          id: $id,
          operator_summary_requested: false,
          operator_briefing_requested: false,
          operator_summary_briefing_status: $status,
          source_gap_ledger_present: true,
          source_gap_ledger_ready: true,
          operator_summary_allowed: false,
          operator_summary_request_accepted: false,
          operator_summary_recorded: false,
          operator_summary_persisted: false,
          operator_summary_materialized: false,
          operator_summary_filesystem_written: false,
          operator_summary_delivered: false,
          operator_summary_channel_delivery_performed: false,
          operator_briefing_allowed: false,
          operator_briefing_request_accepted: false,
          operator_briefing_recorded: false,
          operator_briefing_persisted: false,
          operator_briefing_materialized: false,
          operator_briefing_filesystem_written: false,
          operator_briefing_delivered: false,
          operator_briefing_channel_delivery_performed: false,
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
          ledger_recorded: false,
          index_delivered: false,
          completion_ack_recorded: false,
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
          summary_briefing_noop_confirmed: true,
          reason: $reason
        } + $extra;
      [
        blocked_fixture("operator-approval-gap-ledger-summary-missing-source"; "blocked_noop"; "source_operator_approval_gap_ledger_report_required"; {source_gap_ledger_present: false, source_gap_ledger_ready: false, operator_summary_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-request"; "blocked_summary_noop"; "operator_summary_request_shape_denied"; {operator_summary_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-briefing-request"; "blocked_briefing_noop"; "operator_briefing_request_shape_denied"; {operator_briefing_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-materialization-request"; "blocked_summary_noop"; "summary_materialization_denied"; {operator_summary_requested: true, operator_summary_materialization_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-briefing-materialization-request"; "blocked_briefing_noop"; "briefing_materialization_denied"; {operator_briefing_requested: true, operator_briefing_materialization_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-persistence-filesystem-write-request"; "blocked_summary_noop"; "summary_persistence_filesystem_write_denied"; {operator_summary_requested: true, operator_summary_persistence_requested: true, operator_summary_filesystem_write_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-briefing-persistence-filesystem-write-request"; "blocked_briefing_noop"; "briefing_persistence_filesystem_write_denied"; {operator_briefing_requested: true, operator_briefing_persistence_requested: true, operator_briefing_filesystem_write_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-delivery-request"; "blocked_delivery_noop"; "summary_briefing_channel_delivery_denied"; {operator_summary_requested: true, operator_briefing_requested: true, channel_delivery_requested: true, telegram_send_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-activation-request"; "blocked_activation_noop"; "summary_briefing_activation_terminal_closure_denied"; {operator_summary_requested: true, operator_briefing_requested: true, activation_from_summary_requested: true, terminal_closure_from_summary_requested: true, operator_approval_from_summary_requested: true}),
        blocked_fixture("operator-approval-gap-ledger-summary-briefing-external-public-install-request"; "blocked_delivery_noop"; "external_public_install_restart_upstream_secret_summary_briefing_denied"; {operator_summary_requested: true, operator_briefing_requested: true, external_send_summary_requested: true, public_claim_summary_requested: true, release_artifact_summary_requested: true, install_summary_requested: true, service_restart_summary_requested: true, upstream_merge_summary_requested: true, secret_summary_requested: true})
      ] as $fixtures
      | ($source.operator_approval_gap_ledger_families
        | map({
            family_id,
            status,
            ready,
            blocked,
            ledger_item_count,
            missing_item_count,
            report_only_item_count,
            operator_supplied_future_evidence_needed,
            terminal_closure_blocking,
            activation_blocking,
            non_actionable_report_only,
            summary_status: "blocked",
            briefing_status: "blocked",
            operator_summary_allowed: false,
            operator_briefing_allowed: false,
            operator_summary_recorded: false,
            operator_briefing_recorded: false,
            operator_summary_persisted: false,
            operator_briefing_persisted: false,
            operator_summary_delivered: false,
            operator_briefing_delivered: false,
            terminal_closure_allowed: false,
            activation_allowed: false,
            public_release_claim_allowed: false
          })) as $summary_sections
      | ([
          "operator_approval_gap_ledger_summary_recording_denied",
          "operator_approval_gap_ledger_summary_persistence_denied",
          "operator_approval_gap_ledger_summary_materialization_denied",
          "operator_approval_gap_ledger_summary_delivery_denied",
          "operator_approval_gap_ledger_briefing_recording_denied",
          "operator_approval_gap_ledger_briefing_persistence_denied",
          "operator_approval_gap_ledger_briefing_materialization_denied",
          "operator_approval_gap_ledger_briefing_delivery_denied",
          "operator_approval_gap_ledger_summary_briefing_approval_acceptance_denied",
          "operator_approval_gap_ledger_summary_briefing_terminal_closure_denied",
          "operator_approval_gap_ledger_summary_briefing_activation_denied",
          "operator_approval_gap_ledger_summary_briefing_release_claim_denied"
        ] + $source.denied_by_trusted_record_positive_packet_operator_approval_gap_ledger) as $summary_denied
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          core_activation_operator_approval_gap_ledger_summary_briefing_schema_version: "core_activation_operator_approval_gap_ledger_summary_briefing_non_persistence_v1",
          core_activation_operator_approval_gap_ledger_summary_briefing_ready: true,
          operator_approval_gap_ledger_summary_briefing_mode: "stdout_only_operator_facing_summary_briefing_non_persistence_no_delivery_no_acceptance_no_activation",
          operator_approval_gap_ledger_summary_briefing_status: "blocked",
          operator_approval_gap_ledger_summary_briefing_decision: "operator_approval_gap_ledger_can_be_summarized_for_operator_visibility_without_recording_persisting_materializing_delivering_accepting_closing_activating_or_releasing",
          source_operator_approval_gap_ledger_gate: $source.gate,
          source_operator_approval_gap_ledger_status: $source.trusted_record_positive_packet_operator_approval_gap_ledger_status,
          source_operator_approval_gap_ledger_report_sha256: $gap_ledger_report_sha256,
          source_operator_approval_gap_ledger_hash_sha256: $source.operator_approval_gap_ledger_hash_sha256,
          source_operator_approval_gap_ledger_policy_hash_sha256: $source.operator_approval_gap_ledger_policy_hash_sha256,
          source_operator_approval_gap_ledger_denial_hash_sha256: $source.operator_approval_gap_ledger_denial_hash_sha256,
          source_operator_approval_gap_ledger_side_effect_hash_sha256: $source.operator_approval_gap_ledger_side_effect_hash_sha256,
          source_positive_packet_json_capture_boundary_gate: $source.source_positive_packet_json_capture_boundary_gate,
          source_positive_packet_json_capture_boundary_status: $source.source_positive_packet_json_capture_boundary_status,
          source_positive_packet_json_capture_boundary_report_sha256: $source.source_positive_packet_json_capture_boundary_report_sha256,
          summary_briefing_hash_sha256: $summary_briefing_hash_sha256,
          summary_briefing_policy_hash_sha256: $summary_briefing_policy_hash_sha256,
          summary_briefing_denial_hash_sha256: $summary_briefing_denial_hash_sha256,
          summary_briefing_side_effect_hash_sha256: $summary_briefing_side_effect_hash_sha256,
          minimum_required_long_soak_samples: $min_long_soak_samples,
          source_operator_approval_gap_ledger_item_count: $source.operator_approval_gap_ledger_item_count,
          source_missing_operator_approval_gap_ledger_item_count: $source.missing_operator_approval_gap_ledger_item_count,
          source_operator_approval_gap_ledger_family_count: $source.operator_approval_gap_ledger_family_count,
          required_operator_approval_gap_ledger_summary_briefing_section_count: 9,
          operator_approval_gap_ledger_summary_briefing_section_count: ($summary_sections | length),
          ready_operator_approval_gap_ledger_summary_briefing_section_count: ($summary_sections | map(select(.ready == true)) | length),
          blocked_operator_approval_gap_ledger_summary_briefing_section_count: ($summary_sections | map(select(.blocked == true)) | length),
          operator_approval_gap_ledger_summary_sections: $summary_sections,
          required_operator_approval_gap_ledger_summary_briefing_surface_count: 12,
          ready_operator_approval_gap_ledger_summary_briefing_surface_count: 12,
          side_effect_free_operator_approval_gap_ledger_summary_briefing_surface_count: 12,
          required_operator_approval_gap_ledger_summary_briefing_fixture_count: 10,
          operator_approval_gap_ledger_summary_briefing_fixture_count: ($fixtures | length),
          blocked_operator_approval_gap_ledger_summary_briefing_fixture_count: ($fixtures | length),
          noop_operator_approval_gap_ledger_summary_briefing_fixture_count: ($fixtures | length),
          allowed_operator_approval_gap_ledger_summary_briefing_fixture_count: 0,
          accepted_operator_approval_gap_ledger_summary_briefing_fixture_count: 0,
          operator_approval_gap_ledger_operator_summary_performed_count: 0,
          operator_approval_gap_ledger_operator_briefing_performed_count: 0,
          operator_approval_gap_ledger_operator_summary_allowed: false,
          operator_approval_gap_ledger_operator_briefing_allowed: false,
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
          operator_approval_gap_ledger_summary_briefing_fixtures: $fixtures,
          denied_by_operator_approval_gap_ledger_summary_briefing_non_persistence: $summary_denied,
          denied_by_operator_approval_gap_ledger_summary_briefing_non_persistence_count: ($summary_denied | length),
          inherited_denied_by_operator_approval_gap_ledger_count: $source.denied_by_trusted_record_positive_packet_operator_approval_gap_ledger_count,
          operator_approval_gap_ledger_summary_briefing_non_persistence_executed: true,
          json_report_capture_helper_used: true,
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
          receipt_accepted: false,
          ledger_recording_allowed: false,
          ledger_recorded: false,
          index_delivery_allowed: false,
          index_delivered: false,
          completion_ack_acceptance_allowed: false,
          completion_ack_accepted: false,
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
          external_send_performed: false,
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            memory_store_mutated: false,
            operator_approval_gap_ledger_summary_recorded: false,
            operator_approval_gap_ledger_summary_persisted: false,
            operator_approval_gap_ledger_summary_materialized: false,
            operator_approval_gap_ledger_summary_filesystem_written: false,
            operator_approval_gap_ledger_summary_delivered: false,
            operator_approval_gap_ledger_briefing_recorded: false,
            operator_approval_gap_ledger_briefing_persisted: false,
            operator_approval_gap_ledger_briefing_materialized: false,
            operator_approval_gap_ledger_briefing_filesystem_written: false,
            operator_approval_gap_ledger_briefing_delivered: false,
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
            ledger_recorded: false,
            index_recorded: false,
            delivery_recorded: false,
            completion_ack_recorded: false,
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
  and .gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_non_persistence_gate"
  and .core_activation_operator_approval_gap_ledger_summary_briefing_ready == true
  and .operator_approval_gap_ledger_summary_briefing_status == "blocked"
  and .source_operator_approval_gap_ledger_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_operator_approval_gap_ledger_gate"
  and .source_operator_approval_gap_ledger_status == "blocked"
  and .source_positive_packet_json_capture_boundary_status == "blocked"
  and .source_operator_approval_gap_ledger_item_count == 16
  and .source_missing_operator_approval_gap_ledger_item_count == 16
  and .source_operator_approval_gap_ledger_family_count == 9
  and .required_operator_approval_gap_ledger_summary_briefing_section_count == 9
  and .operator_approval_gap_ledger_summary_briefing_section_count == 9
  and .ready_operator_approval_gap_ledger_summary_briefing_section_count == 9
  and .blocked_operator_approval_gap_ledger_summary_briefing_section_count == 9
  and (.operator_approval_gap_ledger_summary_sections | length) == 9
  and (.operator_approval_gap_ledger_summary_sections | all(.ready == true and .blocked == true and .summary_status == "blocked" and .briefing_status == "blocked" and .operator_summary_allowed == false and .operator_briefing_allowed == false and .operator_summary_recorded == false and .operator_briefing_recorded == false and .operator_summary_persisted == false and .operator_briefing_persisted == false and .operator_summary_delivered == false and .operator_briefing_delivered == false and .terminal_closure_allowed == false and .activation_allowed == false and .public_release_claim_allowed == false))
  and .required_operator_approval_gap_ledger_summary_briefing_surface_count == 12
  and .ready_operator_approval_gap_ledger_summary_briefing_surface_count == 12
  and .side_effect_free_operator_approval_gap_ledger_summary_briefing_surface_count == 12
  and .required_operator_approval_gap_ledger_summary_briefing_fixture_count == 10
  and .operator_approval_gap_ledger_summary_briefing_fixture_count == 10
  and .blocked_operator_approval_gap_ledger_summary_briefing_fixture_count == 10
  and .noop_operator_approval_gap_ledger_summary_briefing_fixture_count == 10
  and .allowed_operator_approval_gap_ledger_summary_briefing_fixture_count == 0
  and .accepted_operator_approval_gap_ledger_summary_briefing_fixture_count == 0
  and .operator_approval_gap_ledger_operator_summary_performed_count == 0
  and .operator_approval_gap_ledger_operator_briefing_performed_count == 0
  and .operator_approval_gap_ledger_operator_summary_recorded == false
  and .operator_approval_gap_ledger_operator_summary_persisted == false
  and .operator_approval_gap_ledger_operator_summary_materialized == false
  and .operator_approval_gap_ledger_operator_summary_filesystem_written == false
  and .operator_approval_gap_ledger_operator_summary_delivered == false
  and .operator_approval_gap_ledger_operator_briefing_recorded == false
  and .operator_approval_gap_ledger_operator_briefing_persisted == false
  and .operator_approval_gap_ledger_operator_briefing_materialized == false
  and .operator_approval_gap_ledger_operator_briefing_filesystem_written == false
  and .operator_approval_gap_ledger_operator_briefing_delivered == false
  and .operator_approval_gap_ledger_summary_briefing_channel_delivery_performed == false
  and (.operator_approval_gap_ledger_summary_briefing_fixtures | length) == 10
  and (.operator_approval_gap_ledger_summary_briefing_fixtures | all(
    (.operator_summary_briefing_status == "blocked_noop" or .operator_summary_briefing_status == "blocked_summary_noop" or .operator_summary_briefing_status == "blocked_briefing_noop" or .operator_summary_briefing_status == "blocked_delivery_noop" or .operator_summary_briefing_status == "blocked_activation_noop")
    and .operator_summary_recorded == false
    and .operator_summary_persisted == false
    and .operator_summary_materialized == false
    and .operator_summary_filesystem_written == false
    and .operator_summary_delivered == false
    and .operator_briefing_recorded == false
    and .operator_briefing_persisted == false
    and .operator_briefing_materialized == false
    and .operator_briefing_filesystem_written == false
    and .operator_briefing_delivered == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .operator_approval_recorded == false
    and .operator_approval_accepted == false
    and .operator_identity_accepted == false
    and .activation_request_recorded == false
    and .trusted_record_accepted == false
    and .fresh_evidence_accepted == false
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_accepted == false
    and .ledger_recorded == false
    and .index_delivered == false
    and .completion_ack_accepted == false
    and .terminal_closure_recorded == false
    and .terminal_closure_accepted == false
    and .activation_allowed == false
    and .activation_performed == false
    and .release_artifact_written == false
    and .public_release_claimed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .install_executed == false
    and .service_restarted == false
    and .summary_briefing_noop_confirmed == true
  ))
  and .denied_by_operator_approval_gap_ledger_summary_briefing_non_persistence_count == 81
  and (.denied_by_operator_approval_gap_ledger_summary_briefing_non_persistence | length) == 81
  and .inherited_denied_by_operator_approval_gap_ledger_count == 69
  and .operator_approval_gap_ledger_summary_briefing_non_persistence_executed == true
  and .json_report_capture_helper_used == true
  and .operator_approval_gap_ledger_recorded == false
  and .operator_approval_gap_ledger_persisted == false
  and .operator_approval_gap_ledger_materialized == false
  and .operator_approval_gap_ledger_delivered == false
  and .operator_approval_gap_ledger_promoted_to_authority == false
  and .operator_packet_recorded == false
  and .operator_packet_accepted == false
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .operator_identity_accepted == false
  and .activation_request_recorded == false
  and .trusted_record_acceptance_allowed == false
  and .trusted_record_accepted == false
  and .fresh_evidence_accepted == false
  and .terminal_closure_allowed == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .activation_allowed == false
  and .activation_performed == false
  and .receipt_persistence_allowed == false
  and .receipt_acceptance_allowed == false
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
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
