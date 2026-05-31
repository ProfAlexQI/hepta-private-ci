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

SUMMARY_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-gate.sh
)"

summary_report_sha256="$(sha256_text "$SUMMARY_JSON")"
summary_index_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index:index:$summary_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_index_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index:policy:$summary_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_index_denial_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index:denial:$summary_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_index_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index:side-effects:$summary_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson summary "$SUMMARY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $summary.runtime == "hepta"
    and $summary.status == "ready"
    and $summary.gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_gate"
    and $summary.terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_ready == true
    and $summary.trusted_record_positive_packet_authority_replay_denial_summary_mode == "stdout_only_report_only_positive_packet_authority_replay_denial_summary_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
    and $summary.trusted_record_positive_packet_authority_replay_denial_summary_status == "blocked"
    and $summary.source_positive_packet_authority_replay_denial_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_gate"
    and $summary.source_positive_packet_authority_replay_denial_matrix_status == "blocked"
    and $summary.source_trusted_record_positive_packet_dry_run_scaffold_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_gate"
    and $summary.source_trusted_record_acceptance_precondition_scoreboard_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_gate"
    and $summary.source_terminal_closure_verdict == "blocked"
    and $summary.required_positive_packet_authority_replay_fixture_count == 12
    and $summary.positive_packet_authority_replay_fixture_count == 12
    and $summary.blocked_positive_packet_authority_replay_fixture_count == 12
    and $summary.allowed_positive_packet_authority_replay_fixture_count == 0
    and $summary.replay_entry_point_count == 8
    and $summary.replay_entry_point_summary_count == 8
    and ($summary.replay_entry_point_summary | length) == 8
    and ($summary.replay_entry_point_summary | all(.accepted_authority_required == true and .replay_authority_allowed == false and .summary_status == "blocked"))
    and $summary.replay_surface_summary_count == 12
    and ($summary.replay_surface_summary | length) == 12
    and (($summary.replay_surface_summary | map(.fixture_count) | add) == 12)
    and ($summary.replay_surface_summary | all(.blocked_fixture_count == .fixture_count and .allowed_fixture_count == 0 and .summary_status == "blocked" and .replay_authority_allowed == false))
    and $summary.required_summary_family_count == 8
    and $summary.summary_family_count == 8
    and $summary.ready_summary_family_count == 8
    and $summary.activation_blocking_summary_family_count == 8
    and ($summary.summary_families | length) == 8
    and ($summary.summary_families | all(.ready == true and .blocked == true))
    and $summary.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count == 37
    and ($summary.denied_by_trusted_record_positive_packet_authority_replay_denial_summary | length) == 37
    and $summary.trusted_record_positive_packet_authority_replay_denial_summary_executed == true
    and $summary.summary_recorded == false
    and $summary.summary_persisted == false
    and $summary.summary_materialized == false
    and $summary.summary_delivered == false
    and $summary.trusted_record_acceptance_allowed == false
    and $summary.trusted_record_accepted == false
    and $summary.terminal_closure_allowed == false
    and $summary.terminal_closure_recorded == false
    and $summary.activation_allowed == false
    and $summary.activation_performed == false
    and $summary.receipt_persistence_allowed == false
    and $summary.receipt_accepted == false
    and $summary.ledger_recorded == false
    and $summary.index_delivered == false
    and $summary.completion_ack_accepted == false
    and $summary.public_release_claim_allowed == false
    and $summary.release_artifact_write_allowed == false
    and ($summary.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_gate" \
  --arg summary_report_sha256 "$summary_report_sha256" \
  --arg summary_index_hash_sha256 "$summary_index_hash_sha256" \
  --arg summary_index_policy_hash_sha256 "$summary_index_policy_hash_sha256" \
  --arg summary_index_denial_hash_sha256 "$summary_index_denial_hash_sha256" \
  --arg summary_index_side_effect_hash_sha256 "$summary_index_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson summary "$SUMMARY_JSON" \
  '
    ($summary.replay_entry_point_summary
      | map(. + {
          index_status: "blocked",
          summary_indexed: true,
          summary_index_authority_allowed: false,
          summary_index_recorded: false,
          summary_index_persisted: false,
          summary_index_delivered: false,
          terminal_closure_allowed: false,
          activation_allowed: false
        })) as $entry_point_index
    | ($summary.replay_surface_summary
      | map(. + {
          index_status: "blocked",
          summary_indexed: true,
          summary_index_authority_allowed: false,
          summary_index_recorded: false,
          summary_index_persisted: false,
          summary_index_delivered: false,
          terminal_closure_allowed: false,
          activation_allowed: false
        })) as $surface_index
    | ($summary.summary_families
      | map(. + {
          index_status: "blocked",
          summary_indexed: true,
          summary_index_authority_allowed: false,
          summary_index_recorded: false,
          summary_index_persisted: false,
          summary_index_delivered: false,
          terminal_closure_allowed: false,
          activation_allowed: false
        })) as $family_index
    | ([
        "positive_packet_authority_replay_denial_summary_index_recording_denied",
        "positive_packet_authority_replay_denial_summary_index_persistence_denied",
        "positive_packet_authority_replay_denial_summary_index_materialization_denied",
        "positive_packet_authority_replay_denial_summary_index_delivery_denied",
        "positive_packet_authority_replay_denial_summary_index_authority_promotion_denied",
        "positive_packet_authority_replay_denial_summary_index_terminal_closure_denied",
        "positive_packet_authority_replay_denial_summary_index_activation_denied",
        "positive_packet_authority_replay_denial_summary_index_public_release_claim_denied"
      ] + $summary.denied_by_trusted_record_positive_packet_authority_replay_denial_summary) as $index_denied
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_schema_version: "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_v1",
        terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_ready: true,
        trusted_record_positive_packet_authority_replay_denial_summary_index_mode: "stdout_only_report_only_positive_packet_authority_replay_denial_summary_index_no_approval_no_persistence_no_acceptance_no_delivery_no_activation",
        trusted_record_positive_packet_authority_replay_denial_summary_index_status: "blocked",
        trusted_record_positive_packet_authority_replay_denial_summary_index_decision: "positive_packet_authority_replay_denial_summary_indexed_without_recording_persisting_materializing_delivering_promoting_closing_activating_or_releasing",
        source_positive_packet_authority_replay_denial_summary_gate: $summary.gate,
        source_positive_packet_authority_replay_denial_summary_status: $summary.trusted_record_positive_packet_authority_replay_denial_summary_status,
        source_positive_packet_authority_replay_denial_summary_report_sha256: $summary_report_sha256,
        source_positive_packet_authority_replay_denial_matrix_gate: $summary.source_positive_packet_authority_replay_denial_matrix_gate,
        source_positive_packet_authority_replay_denial_matrix_status: $summary.source_positive_packet_authority_replay_denial_matrix_status,
        source_positive_packet_authority_replay_denial_matrix_report_sha256: $summary.source_positive_packet_authority_replay_denial_matrix_report_sha256,
        source_trusted_record_positive_packet_dry_run_scaffold_gate: $summary.source_trusted_record_positive_packet_dry_run_scaffold_gate,
        source_trusted_record_acceptance_precondition_scoreboard_gate: $summary.source_trusted_record_acceptance_precondition_scoreboard_gate,
        source_trusted_record_acceptance_negative_fixture_matrix_gate: $summary.source_trusted_record_acceptance_negative_fixture_matrix_gate,
        source_terminal_closure_gate: $summary.source_terminal_closure_gate,
        source_terminal_closure_verdict: $summary.source_terminal_closure_verdict,
        positive_packet_authority_replay_denial_summary_index_hash_sha256: $summary_index_hash_sha256,
        positive_packet_authority_replay_denial_summary_index_policy_hash_sha256: $summary_index_policy_hash_sha256,
        positive_packet_authority_replay_denial_summary_index_denial_hash_sha256: $summary_index_denial_hash_sha256,
        positive_packet_authority_replay_denial_summary_index_side_effect_hash_sha256: $summary_index_side_effect_hash_sha256,
        source_positive_packet_authority_replay_summary_hash_sha256: $summary.positive_packet_authority_replay_summary_hash_sha256,
        source_positive_packet_authority_replay_summary_policy_hash_sha256: $summary.positive_packet_authority_replay_summary_policy_hash_sha256,
        source_positive_packet_authority_replay_summary_denial_hash_sha256: $summary.positive_packet_authority_replay_summary_denial_hash_sha256,
        source_positive_packet_authority_replay_summary_side_effect_hash_sha256: $summary.positive_packet_authority_replay_summary_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        source_positive_packet_fixture_count: $summary.source_positive_packet_fixture_count,
        source_blocked_positive_packet_fixture_count: $summary.source_blocked_positive_packet_fixture_count,
        source_accepted_positive_packet_fixture_count: $summary.source_accepted_positive_packet_fixture_count,
        source_positive_packet_scoreboard_item_count: $summary.source_positive_packet_scoreboard_item_count,
        source_positive_packet_satisfied_scoreboard_item_count: $summary.source_positive_packet_satisfied_scoreboard_item_count,
        source_positive_packet_trusted_record_count: $summary.source_positive_packet_trusted_record_count,
        source_positive_packet_future_positive_family_count: $summary.source_positive_packet_future_positive_family_count,
        required_positive_packet_authority_replay_fixture_count: $summary.required_positive_packet_authority_replay_fixture_count,
        positive_packet_authority_replay_fixture_count: $summary.positive_packet_authority_replay_fixture_count,
        blocked_positive_packet_authority_replay_fixture_count: $summary.blocked_positive_packet_authority_replay_fixture_count,
        allowed_positive_packet_authority_replay_fixture_count: $summary.allowed_positive_packet_authority_replay_fixture_count,
        source_replay_entry_point_count: $summary.replay_entry_point_count,
        required_indexed_replay_entry_point_summary_count: 8,
        indexed_replay_entry_point_summary_count: ($entry_point_index | length),
        blocked_indexed_replay_entry_point_summary_count: ($entry_point_index | map(select(.index_status == "blocked")) | length),
        indexed_replay_entry_point_summary: $entry_point_index,
        source_replay_surface_summary_count: $summary.replay_surface_summary_count,
        required_indexed_replay_surface_summary_count: 12,
        indexed_replay_surface_summary_count: ($surface_index | length),
        blocked_indexed_replay_surface_summary_count: ($surface_index | map(select(.index_status == "blocked")) | length),
        indexed_replay_surface_summary: $surface_index,
        source_summary_family_count: $summary.summary_family_count,
        required_summary_index_family_count: 8,
        summary_index_family_count: 8,
        ready_summary_index_family_count: 8,
        activation_blocking_summary_index_family_count: 8,
        indexed_source_summary_families: $family_index,
        summary_index_families: [
          {
            id: "source-summary-index",
            ready: true,
            blocked: true,
            source_summary_status: $summary.trusted_record_positive_packet_authority_replay_denial_summary_status,
            source_summary_report_sha256: $summary_report_sha256,
            reason: "the source summary is ready and blocked, so it can only be indexed as non-authority evidence"
          },
          {
            id: "entry-point-summary-index",
            ready: true,
            blocked: true,
            indexed_replay_entry_point_summary_count: ($entry_point_index | length),
            replay_authority_allowed: false,
            reason: "all replay entry point summaries still require accepted authority"
          },
          {
            id: "replay-surface-summary-index",
            ready: true,
            blocked: true,
            indexed_replay_surface_summary_count: ($surface_index | length),
            replay_authority_allowed: false,
            reason: "all replay surface summaries remain blocked"
          },
          {
            id: "summary-family-index",
            ready: true,
            blocked: true,
            indexed_source_summary_family_count: ($family_index | length),
            activation_blocking_summary_family_count: $summary.activation_blocking_summary_family_count,
            reason: "all source summary families are preserved as blocked index entries"
          },
          {
            id: "inherited-denial-reason-index",
            ready: true,
            blocked: true,
            inherited_denial_reason_count: $summary.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count,
            reason: "all inherited summary denial reasons stay attached to the index"
          },
          {
            id: "summary-index-persistence-boundary",
            ready: true,
            blocked: true,
            summary_index_recorded: false,
            summary_index_persisted: false,
            summary_index_materialized: false,
            summary_index_delivered: false,
            reason: "the summary index is report-only and is not recorded, persisted, materialized, or delivered"
          },
          {
            id: "receipt-ledger-delivery-ack-index-boundary",
            ready: true,
            blocked: true,
            receipt_accepted: false,
            ledger_recorded: false,
            index_delivered: false,
            completion_ack_accepted: false,
            reason: "receipt, ledger, delivery, and acknowledgement authority remain absent"
          },
          {
            id: "terminal-release-activation-index-boundary",
            ready: true,
            blocked: true,
            terminal_closure_recorded: false,
            activation_allowed: false,
            public_release_claim_allowed: false,
            release_artifact_write_allowed: false,
            reason: "the index cannot promote summary evidence to closure, activation, public release, or artifacts"
          }
        ],
        inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count: $summary.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count,
        inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary: $summary.denied_by_trusted_record_positive_packet_authority_replay_denial_summary,
        denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index: $index_denied,
        denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count: ($index_denied | length),
        trusted_record_positive_packet_authority_replay_denial_summary_index_executed: true,
        summary_index_authority_allowed: false,
        summary_index_recorded: false,
        summary_index_persisted: false,
        summary_index_materialized: false,
        summary_index_delivered: false,
        summary_index_promoted_to_authority: false,
        summary_recorded: false,
        summary_persisted: false,
        summary_materialized: false,
        summary_delivered: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        operator_packet_delivered: false,
        operator_packet_authorizes_activation: false,
        operator_packet_authorizes_terminal_closure: false,
        operator_packet_authorizes_receipt_persistence: false,
        operator_packet_authorizes_ledger_recording: false,
        trusted_record_acceptance_allowed: false,
        trusted_record_accepted: false,
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
        side_effects: {
          workspace_written: false,
          filesystem_written: false,
          memory_store_mutated: false,
          summary_index_recorded: false,
          summary_index_persisted: false,
          summary_index_materialized: false,
          summary_index_delivered: false,
          summary_index_promoted_to_authority: false,
          summary_recorded: false,
          summary_persisted: false,
          summary_materialized: false,
          summary_delivered: false,
          operator_packet_recorded: false,
          operator_packet_persisted: false,
          operator_packet_accepted: false,
          operator_packet_delivered: false,
          trusted_record_recorded: false,
          trusted_record_persisted: false,
          trusted_record_accepted: false,
          trusted_record_delivered: false,
          approval_recorded: false,
          activation_request_recorded: false,
          fresh_evidence_accepted: false,
          receipt_persistence_command_enabled: false,
          receipt_persistence_execution_performed: false,
          receipt_acceptance_recorded: false,
          ledger_recorded: false,
          index_recorded: false,
          delivery_recorded: false,
          completion_ack_recorded: false,
          terminal_closure_recorded: false,
          terminal_closure_persisted: false,
          terminal_closure_accepted: false,
          activation_performed: false,
          provider_invoked: false,
          model_invoked: false,
          channel_send_performed: false,
          telegram_send_performed: false,
          external_send_performed: false,
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
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_gate"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_schema_version == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_v1"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_ready == true
  and .trusted_record_positive_packet_authority_replay_denial_summary_index_mode == "stdout_only_report_only_positive_packet_authority_replay_denial_summary_index_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
  and .trusted_record_positive_packet_authority_replay_denial_summary_index_status == "blocked"
  and .source_positive_packet_authority_replay_denial_summary_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_gate"
  and .source_positive_packet_authority_replay_denial_summary_status == "blocked"
  and .source_positive_packet_authority_replay_denial_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_gate"
  and .source_positive_packet_authority_replay_denial_matrix_status == "blocked"
  and .source_terminal_closure_verdict == "blocked"
  and .required_positive_packet_authority_replay_fixture_count == 12
  and .positive_packet_authority_replay_fixture_count == 12
  and .blocked_positive_packet_authority_replay_fixture_count == 12
  and .allowed_positive_packet_authority_replay_fixture_count == 0
  and .source_replay_entry_point_count == 8
  and .required_indexed_replay_entry_point_summary_count == 8
  and .indexed_replay_entry_point_summary_count == 8
  and .blocked_indexed_replay_entry_point_summary_count == 8
  and (.indexed_replay_entry_point_summary | length) == 8
  and (.indexed_replay_entry_point_summary | all(
    .accepted_authority_required == true
    and .replay_authority_allowed == false
    and .summary_status == "blocked"
    and .index_status == "blocked"
    and .summary_index_authority_allowed == false
    and .summary_index_recorded == false
    and .summary_index_persisted == false
    and .summary_index_delivered == false
    and .terminal_closure_allowed == false
    and .activation_allowed == false
  ))
  and .source_replay_surface_summary_count == 12
  and .required_indexed_replay_surface_summary_count == 12
  and .indexed_replay_surface_summary_count == 12
  and .blocked_indexed_replay_surface_summary_count == 12
  and (.indexed_replay_surface_summary | length) == 12
  and ((.indexed_replay_surface_summary | map(.fixture_count) | add) == 12)
  and (.indexed_replay_surface_summary | all(
    .blocked_fixture_count == .fixture_count
    and .allowed_fixture_count == 0
    and .summary_status == "blocked"
    and .replay_authority_allowed == false
    and .index_status == "blocked"
    and .summary_index_authority_allowed == false
    and .summary_index_recorded == false
    and .summary_index_persisted == false
    and .summary_index_delivered == false
    and .terminal_closure_allowed == false
    and .activation_allowed == false
  ))
  and .source_summary_family_count == 8
  and .required_summary_index_family_count == 8
  and .summary_index_family_count == 8
  and .ready_summary_index_family_count == 8
  and .activation_blocking_summary_index_family_count == 8
  and (.indexed_source_summary_families | length) == 8
  and (.indexed_source_summary_families | all(
    .ready == true
    and .blocked == true
    and .index_status == "blocked"
    and .summary_index_authority_allowed == false
    and .summary_index_recorded == false
    and .summary_index_persisted == false
    and .summary_index_delivered == false
    and .terminal_closure_allowed == false
    and .activation_allowed == false
  ))
  and (.summary_index_families | length) == 8
  and (.summary_index_families | all(.ready == true and .blocked == true))
  and (.summary_index_families | any(.id == "source-summary-index" and .source_summary_status == "blocked"))
  and (.summary_index_families | any(.id == "entry-point-summary-index" and .indexed_replay_entry_point_summary_count == 8 and .replay_authority_allowed == false))
  and (.summary_index_families | any(.id == "replay-surface-summary-index" and .indexed_replay_surface_summary_count == 12 and .replay_authority_allowed == false))
  and (.summary_index_families | any(.id == "summary-family-index" and .indexed_source_summary_family_count == 8 and .activation_blocking_summary_family_count == 8))
  and (.summary_index_families | any(.id == "inherited-denial-reason-index" and .inherited_denial_reason_count == 37))
  and (.summary_index_families | any(.id == "summary-index-persistence-boundary" and .summary_index_recorded == false and .summary_index_persisted == false and .summary_index_materialized == false and .summary_index_delivered == false))
  and (.summary_index_families | any(.id == "receipt-ledger-delivery-ack-index-boundary" and .receipt_accepted == false and .ledger_recorded == false and .index_delivered == false and .completion_ack_accepted == false))
  and (.summary_index_families | any(.id == "terminal-release-activation-index-boundary" and .terminal_closure_recorded == false and .activation_allowed == false and .public_release_claim_allowed == false and .release_artifact_write_allowed == false))
  and .inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count == 37
  and (.inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary | length) == 37
  and .denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count == 45
  and (.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index | length) == 45
  and .trusted_record_positive_packet_authority_replay_denial_summary_index_executed == true
  and .summary_index_authority_allowed == false
  and .summary_index_recorded == false
  and .summary_index_persisted == false
  and .summary_index_materialized == false
  and .summary_index_delivered == false
  and .summary_index_promoted_to_authority == false
  and .summary_recorded == false
  and .summary_persisted == false
  and .summary_materialized == false
  and .summary_delivered == false
  and .operator_packet_recorded == false
  and .operator_packet_persisted == false
  and .operator_packet_accepted == false
  and .operator_packet_delivered == false
  and .operator_packet_authorizes_activation == false
  and .operator_packet_authorizes_terminal_closure == false
  and .operator_packet_authorizes_receipt_persistence == false
  and .operator_packet_authorizes_ledger_recording == false
  and .trusted_record_acceptance_allowed == false
  and .trusted_record_accepted == false
  and .terminal_closure_allowed == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .activation_allowed == false
  and .activation_performed == false
  and .receipt_persistence_allowed == false
  and .receipt_acceptance_allowed == false
  and .receipt_accepted == false
  and .ledger_recording_allowed == false
  and .ledger_recorded == false
  and .index_delivery_allowed == false
  and .index_delivered == false
  and .completion_ack_acceptance_allowed == false
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
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
