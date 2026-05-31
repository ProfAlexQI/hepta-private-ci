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

MATRIX_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-matrix-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-matrix-gate.sh
)"

matrix_report_sha256="$(sha256_text "$MATRIX_JSON")"
summary_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary:summary:$matrix_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary:policy:$matrix_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_denial_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary:denial:$matrix_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary:side-effects:$matrix_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson matrix "$MATRIX_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $matrix.runtime == "hepta"
    and $matrix.status == "ready"
    and $matrix.gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_gate"
    and $matrix.terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_ready == true
    and $matrix.trusted_record_positive_packet_authority_replay_denial_matrix_mode == "stdout_only_report_only_positive_packet_authority_replay_denial_matrix_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
    and $matrix.trusted_record_positive_packet_authority_replay_denial_matrix_status == "blocked"
    and $matrix.source_trusted_record_positive_packet_dry_run_scaffold_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_gate"
    and $matrix.source_trusted_record_positive_packet_dry_run_scaffold_status == "blocked"
    and $matrix.source_trusted_record_acceptance_precondition_scoreboard_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_gate"
    and $matrix.source_trusted_record_acceptance_negative_fixture_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_gate"
    and $matrix.source_terminal_closure_verdict == "blocked"
    and $matrix.source_positive_packet_fixture_count == 1
    and $matrix.source_blocked_positive_packet_fixture_count == 1
    and $matrix.source_accepted_positive_packet_fixture_count == 0
    and $matrix.source_positive_packet_scoreboard_item_count == 56
    and $matrix.source_positive_packet_satisfied_scoreboard_item_count == 0
    and $matrix.source_positive_packet_trusted_record_count == 8
    and $matrix.source_positive_packet_future_positive_family_count == 7
    and $matrix.source_scoreboard_item_count == 56
    and $matrix.source_required_scoreboard_item_count == 56
    and $matrix.source_satisfied_scoreboard_item_count == 0
    and $matrix.source_unsatisfied_scoreboard_item_count == 56
    and $matrix.source_negative_fixture_count == 12
    and $matrix.source_blocked_negative_fixture_count == 12
    and $matrix.source_allowed_negative_fixture_count == 0
    and $matrix.required_positive_packet_authority_replay_fixture_count == 12
    and $matrix.positive_packet_authority_replay_fixture_count == 12
    and $matrix.blocked_positive_packet_authority_replay_fixture_count == 12
    and $matrix.allowed_positive_packet_authority_replay_fixture_count == 0
    and $matrix.replay_entry_point_count == 8
    and ($matrix.replay_entry_points | length) == 8
    and ($matrix.replay_entry_points | all(.accepted_authority_required == true and .replay_authority_allowed == false))
    and ($matrix.positive_packet_authority_replay_denial_matrix_fixtures | length) == 12
    and ($matrix.positive_packet_authority_replay_denial_matrix_fixtures | all(
      .validation_status == "blocked"
      and .replay_attempted == true
      and .dry_run_only == true
      and .report_only == true
      and .matrix_only == true
      and .source_positive_packet_fixture_status == "blocked"
      and .source_positive_packet_acceptance_status == "blocked"
      and .packet_shape_complete == true
      and .packet_scoreboard_alignment_complete == true
      and .packet_trusted_record_count == 8
      and .packet_scoreboard_item_count == 56
      and .packet_satisfied_scoreboard_item_count == 0
      and .packet_source_negative_fixture_count == 12
      and .explicit_operator_approval_record_present == false
      and .current_operator_identity_attestation_present == false
      and .activation_request_record_present == false
      and .fresh_live_evidence_acceptance_present == false
      and .receipt_persistence_execution_present == false
      and .receipt_acceptance_record_present == false
      and .ledger_record_present == false
      and .index_delivery_record_present == false
      and .completion_ack_record_present == false
      and .operator_packet_recorded == false
      and .operator_packet_persisted == false
      and .operator_packet_accepted == false
      and .operator_packet_delivered == false
      and .operator_packet_authorizes_activation == false
      and .operator_packet_authorizes_terminal_closure == false
      and .trusted_record_accepted == false
      and .trusted_record_acceptance_allowed == false
      and .terminal_closure_allowed == false
      and .terminal_closure_recorded == false
      and .terminal_closure_accepted == false
      and .receipt_persistence_allowed == false
      and .receipt_accepted == false
      and .ledger_recorded == false
      and .index_delivered == false
      and .completion_ack_accepted == false
      and .activation_allowed == false
      and .activation_performed == false
      and .public_release_claim_allowed == false
      and .release_artifact_write_allowed == false
    ))
    and $matrix.denied_by_trusted_record_positive_packet_authority_replay_denial_matrix_count == 29
    and ($matrix.denied_by_trusted_record_positive_packet_authority_replay_denial_matrix | length) == 29
    and $matrix.operator_packet_recorded == false
    and $matrix.operator_packet_persisted == false
    and $matrix.operator_packet_accepted == false
    and $matrix.operator_packet_delivered == false
    and $matrix.trusted_record_acceptance_allowed == false
    and $matrix.trusted_record_accepted == false
    and $matrix.terminal_closure_allowed == false
    and $matrix.terminal_closure_recorded == false
    and $matrix.terminal_closure_accepted == false
    and $matrix.activation_allowed == false
    and $matrix.activation_performed == false
    and $matrix.receipt_persistence_allowed == false
    and $matrix.receipt_acceptance_allowed == false
    and $matrix.receipt_accepted == false
    and $matrix.ledger_recording_allowed == false
    and $matrix.ledger_recorded == false
    and $matrix.index_delivery_allowed == false
    and $matrix.index_delivered == false
    and $matrix.completion_ack_acceptance_allowed == false
    and $matrix.completion_ack_accepted == false
    and $matrix.public_release_claim_allowed == false
    and $matrix.release_artifact_write_allowed == false
    and $matrix.provider_model_invocation_allowed == false
    and $matrix.channel_delivery_allowed == false
    and $matrix.install_restart_allowed == false
    and $matrix.active_binary_mutation_allowed == false
    and $matrix.upstream_fetch_merge_allowed == false
    and $matrix.credential_read_allowed == false
    and $matrix.secret_value_read_allowed == false
    and ($matrix.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_gate" \
  --arg matrix_report_sha256 "$matrix_report_sha256" \
  --arg summary_hash_sha256 "$summary_hash_sha256" \
  --arg summary_policy_hash_sha256 "$summary_policy_hash_sha256" \
  --arg summary_denial_hash_sha256 "$summary_denial_hash_sha256" \
  --arg summary_side_effect_hash_sha256 "$summary_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson matrix "$MATRIX_JSON" \
  '
    ($matrix.positive_packet_authority_replay_denial_matrix_fixtures) as $fixtures
    | ($matrix.replay_entry_points) as $entry_points
    | ($fixtures
        | group_by(.replay_surface)
        | map({
            replay_surface: .[0].replay_surface,
            fixture_count: length,
            blocked_fixture_count: (map(select(.validation_status == "blocked")) | length),
            allowed_fixture_count: 0,
            trusted_record_acceptance_replay_count: (map(select(.trusted_record_acceptance_replay_requested == true)) | length),
            terminal_closure_replay_count: (map(select(.terminal_closure_replay_requested == true)) | length),
            receipt_acceptance_replay_count: (map(select(.receipt_acceptance_replay_requested == true)) | length),
            ledger_recording_replay_count: (map(select(.ledger_recording_replay_requested == true)) | length),
            index_delivery_replay_count: (map(select(.index_delivery_replay_requested == true)) | length),
            completion_ack_replay_count: (map(select(.completion_ack_replay_requested == true)) | length),
            activation_replay_count: (map(select(.activation_replay_requested == true)) | length),
            public_claim_or_artifact_replay_count: (map(select(.public_claim_or_artifact_replay_requested == true)) | length),
            summary_status: "blocked",
            replay_authority_allowed: false
          })) as $surface_summary
    | ($entry_points
        | map({
            entry_point_id,
            target_gate,
            accepted_authority_required,
            replay_authority_allowed,
            summary_status: "blocked"
          })) as $entry_point_summary
    | ([
        "positive_packet_authority_replay_summary_recording_denied",
        "positive_packet_authority_replay_summary_persistence_denied",
        "positive_packet_authority_replay_summary_materialization_denied",
        "positive_packet_authority_replay_summary_delivery_denied",
        "positive_packet_authority_replay_summary_terminal_closure_denied",
        "positive_packet_authority_replay_summary_activation_denied",
        "positive_packet_authority_replay_summary_release_claim_denied",
        "positive_packet_authority_replay_summary_secret_read_denied"
      ] + $matrix.denied_by_trusted_record_positive_packet_authority_replay_denial_matrix) as $summary_denied
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_schema_version: "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_v1",
        terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_ready: true,
        trusted_record_positive_packet_authority_replay_denial_summary_mode: "stdout_only_report_only_positive_packet_authority_replay_denial_summary_no_approval_no_persistence_no_acceptance_no_delivery_no_activation",
        trusted_record_positive_packet_authority_replay_denial_summary_status: "blocked",
        trusted_record_positive_packet_authority_replay_denial_summary_decision: "positive_packet_authority_replay_denials_summarized_without_recording_accepting_persisting_delivering_closing_activating_or_releasing",
        source_positive_packet_authority_replay_denial_matrix_gate: $matrix.gate,
        source_positive_packet_authority_replay_denial_matrix_status: $matrix.trusted_record_positive_packet_authority_replay_denial_matrix_status,
        source_positive_packet_authority_replay_denial_matrix_report_sha256: $matrix_report_sha256,
        source_trusted_record_positive_packet_dry_run_scaffold_gate: $matrix.source_trusted_record_positive_packet_dry_run_scaffold_gate,
        source_trusted_record_acceptance_precondition_scoreboard_gate: $matrix.source_trusted_record_acceptance_precondition_scoreboard_gate,
        source_trusted_record_acceptance_negative_fixture_matrix_gate: $matrix.source_trusted_record_acceptance_negative_fixture_matrix_gate,
        source_trusted_record_acceptance_skeleton_gate: $matrix.source_trusted_record_acceptance_skeleton_gate,
        source_terminal_closure_gate: $matrix.source_terminal_closure_gate,
        source_terminal_closure_verdict: $matrix.source_terminal_closure_verdict,
        positive_packet_authority_replay_summary_hash_sha256: $summary_hash_sha256,
        positive_packet_authority_replay_summary_policy_hash_sha256: $summary_policy_hash_sha256,
        positive_packet_authority_replay_summary_denial_hash_sha256: $summary_denial_hash_sha256,
        positive_packet_authority_replay_summary_side_effect_hash_sha256: $summary_side_effect_hash_sha256,
        source_positive_packet_authority_replay_matrix_hash_sha256: $matrix.positive_packet_authority_replay_matrix_hash_sha256,
        source_positive_packet_authority_replay_policy_hash_sha256: $matrix.positive_packet_authority_replay_policy_hash_sha256,
        source_positive_packet_authority_replay_side_effect_hash_sha256: $matrix.positive_packet_authority_replay_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        source_positive_packet_fixture_count: $matrix.source_positive_packet_fixture_count,
        source_blocked_positive_packet_fixture_count: $matrix.source_blocked_positive_packet_fixture_count,
        source_accepted_positive_packet_fixture_count: $matrix.source_accepted_positive_packet_fixture_count,
        source_positive_packet_scoreboard_item_count: $matrix.source_positive_packet_scoreboard_item_count,
        source_positive_packet_satisfied_scoreboard_item_count: $matrix.source_positive_packet_satisfied_scoreboard_item_count,
        source_positive_packet_trusted_record_count: $matrix.source_positive_packet_trusted_record_count,
        source_positive_packet_future_positive_family_count: $matrix.source_positive_packet_future_positive_family_count,
        source_scoreboard_item_count: $matrix.source_scoreboard_item_count,
        source_required_scoreboard_item_count: $matrix.source_required_scoreboard_item_count,
        source_satisfied_scoreboard_item_count: $matrix.source_satisfied_scoreboard_item_count,
        source_unsatisfied_scoreboard_item_count: $matrix.source_unsatisfied_scoreboard_item_count,
        source_negative_fixture_count: $matrix.source_negative_fixture_count,
        source_blocked_negative_fixture_count: $matrix.source_blocked_negative_fixture_count,
        source_allowed_negative_fixture_count: $matrix.source_allowed_negative_fixture_count,
        required_positive_packet_authority_replay_fixture_count: $matrix.required_positive_packet_authority_replay_fixture_count,
        positive_packet_authority_replay_fixture_count: $matrix.positive_packet_authority_replay_fixture_count,
        blocked_positive_packet_authority_replay_fixture_count: $matrix.blocked_positive_packet_authority_replay_fixture_count,
        allowed_positive_packet_authority_replay_fixture_count: $matrix.allowed_positive_packet_authority_replay_fixture_count,
        replay_entry_point_count: $matrix.replay_entry_point_count,
        replay_entry_point_summary_count: ($entry_point_summary | length),
        replay_entry_point_summary: $entry_point_summary,
        replay_surface_summary_count: ($surface_summary | length),
        replay_surface_summary: $surface_summary,
        source_denied_by_positive_packet_authority_replay_denial_matrix_count: $matrix.denied_by_trusted_record_positive_packet_authority_replay_denial_matrix_count,
        required_summary_family_count: 8,
        summary_family_count: 8,
        ready_summary_family_count: 8,
        activation_blocking_summary_family_count: 8,
        summary_families: [
          {
            id: "source-matrix-summary",
            ready: true,
            blocked: true,
            source_fixture_count: $matrix.positive_packet_authority_replay_fixture_count,
            blocked_fixture_count: $matrix.blocked_positive_packet_authority_replay_fixture_count,
            allowed_fixture_count: $matrix.allowed_positive_packet_authority_replay_fixture_count,
            reason: "source authority replay matrix is ready and all replay fixtures remain blocked"
          },
          {
            id: "positive-packet-shape-summary",
            ready: true,
            blocked: true,
            source_positive_packet_fixture_count: $matrix.source_positive_packet_fixture_count,
            source_positive_packet_trusted_record_count: $matrix.source_positive_packet_trusted_record_count,
            reason: "the positive packet shape is complete but no trusted record is accepted"
          },
          {
            id: "scoreboard-unsatisfied-summary",
            ready: true,
            blocked: true,
            source_scoreboard_item_count: $matrix.source_scoreboard_item_count,
            source_satisfied_scoreboard_item_count: $matrix.source_satisfied_scoreboard_item_count,
            source_unsatisfied_scoreboard_item_count: $matrix.source_unsatisfied_scoreboard_item_count,
            reason: "all 56 scoreboard items remain represented but unsatisfied"
          },
          {
            id: "entry-point-replay-summary",
            ready: true,
            blocked: true,
            replay_entry_point_count: $matrix.replay_entry_point_count,
            reason: "all downstream entry points require accepted authority and deny replay"
          },
          {
            id: "receipt-ledger-delivery-ack-summary",
            ready: true,
            blocked: true,
            receipt_accepted: false,
            ledger_recorded: false,
            index_delivered: false,
            completion_ack_accepted: false,
            reason: "receipt, ledger, delivery, and completion ack records are absent"
          },
          {
            id: "public-release-boundary-summary",
            ready: true,
            blocked: true,
            public_release_claim_allowed: false,
            release_artifact_write_allowed: false,
            reason: "public release claim and release artifact writes remain denied"
          },
          {
            id: "summary-persistence-side-effect-boundary",
            ready: true,
            blocked: true,
            summary_recorded: false,
            summary_persisted: false,
            summary_materialized: false,
            summary_delivered: false,
            reason: "summary is report-only and not recorded, persisted, materialized, or delivered"
          },
          {
            id: "terminal-activation-boundary-summary",
            ready: true,
            blocked: true,
            terminal_closure_recorded: false,
            activation_allowed: false,
            activation_performed: false,
            reason: "terminal closure and activation remain blocked"
          }
        ],
        denied_by_trusted_record_positive_packet_authority_replay_denial_summary: $summary_denied,
        denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count: ($summary_denied | length),
        trusted_record_positive_packet_authority_replay_denial_summary_executed: true,
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
        summary_recorded: false,
        summary_persisted: false,
        summary_materialized: false,
        summary_delivered: false,
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
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_gate"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_schema_version == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_v1"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_ready == true
  and .trusted_record_positive_packet_authority_replay_denial_summary_mode == "stdout_only_report_only_positive_packet_authority_replay_denial_summary_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
  and .trusted_record_positive_packet_authority_replay_denial_summary_status == "blocked"
  and .source_positive_packet_authority_replay_denial_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_gate"
  and .source_positive_packet_authority_replay_denial_matrix_status == "blocked"
  and .source_trusted_record_positive_packet_dry_run_scaffold_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_gate"
  and .source_trusted_record_acceptance_precondition_scoreboard_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_gate"
  and .source_terminal_closure_verdict == "blocked"
  and .source_positive_packet_fixture_count == 1
  and .source_blocked_positive_packet_fixture_count == 1
  and .source_accepted_positive_packet_fixture_count == 0
  and .source_positive_packet_scoreboard_item_count == 56
  and .source_positive_packet_satisfied_scoreboard_item_count == 0
  and .source_positive_packet_trusted_record_count == 8
  and .source_positive_packet_future_positive_family_count == 7
  and .source_scoreboard_item_count == 56
  and .source_required_scoreboard_item_count == 56
  and .source_satisfied_scoreboard_item_count == 0
  and .source_unsatisfied_scoreboard_item_count == 56
  and .source_negative_fixture_count == 12
  and .source_blocked_negative_fixture_count == 12
  and .source_allowed_negative_fixture_count == 0
  and .required_positive_packet_authority_replay_fixture_count == 12
  and .positive_packet_authority_replay_fixture_count == 12
  and .blocked_positive_packet_authority_replay_fixture_count == 12
  and .allowed_positive_packet_authority_replay_fixture_count == 0
  and .replay_entry_point_count == 8
  and .replay_entry_point_summary_count == 8
  and (.replay_entry_point_summary | all(.accepted_authority_required == true and .replay_authority_allowed == false and .summary_status == "blocked"))
  and .replay_surface_summary_count == 12
  and ((.replay_surface_summary | map(.fixture_count) | add) == 12)
  and (.replay_surface_summary | all(.blocked_fixture_count == .fixture_count and .allowed_fixture_count == 0 and .summary_status == "blocked" and .replay_authority_allowed == false))
  and .source_denied_by_positive_packet_authority_replay_denial_matrix_count == 29
  and .required_summary_family_count == 8
  and .summary_family_count == 8
  and .ready_summary_family_count == 8
  and .activation_blocking_summary_family_count == 8
  and (.summary_families | length) == 8
  and (.summary_families | all(.ready == true and .blocked == true))
  and (.summary_families | any(.id == "source-matrix-summary" and .source_fixture_count == 12 and .blocked_fixture_count == 12 and .allowed_fixture_count == 0))
  and (.summary_families | any(.id == "positive-packet-shape-summary" and .source_positive_packet_trusted_record_count == 8))
  and (.summary_families | any(.id == "scoreboard-unsatisfied-summary" and .source_satisfied_scoreboard_item_count == 0 and .source_unsatisfied_scoreboard_item_count == 56))
  and (.summary_families | any(.id == "entry-point-replay-summary" and .replay_entry_point_count == 8))
  and (.summary_families | any(.id == "receipt-ledger-delivery-ack-summary" and .receipt_accepted == false and .ledger_recorded == false and .index_delivered == false and .completion_ack_accepted == false))
  and (.summary_families | any(.id == "public-release-boundary-summary" and .public_release_claim_allowed == false and .release_artifact_write_allowed == false))
  and (.summary_families | any(.id == "summary-persistence-side-effect-boundary" and .summary_recorded == false and .summary_persisted == false and .summary_materialized == false and .summary_delivered == false))
  and (.summary_families | any(.id == "terminal-activation-boundary-summary" and .terminal_closure_recorded == false and .activation_allowed == false and .activation_performed == false))
  and .denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count == 37
  and (.denied_by_trusted_record_positive_packet_authority_replay_denial_summary | length) == 37
  and .trusted_record_positive_packet_authority_replay_denial_summary_executed == true
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
  and .summary_recorded == false
  and .summary_persisted == false
  and .summary_materialized == false
  and .summary_delivered == false
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
