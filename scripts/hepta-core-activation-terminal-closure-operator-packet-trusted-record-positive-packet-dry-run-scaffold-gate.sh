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

SCOREBOARD_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-precondition-scoreboard-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-precondition-scoreboard-gate.sh
)"

scoreboard_report_sha256="$(sha256_text "$SCOREBOARD_JSON")"
positive_packet_scaffold_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-dry-run-scaffold:scaffold:$scoreboard_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
positive_packet_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-dry-run-scaffold:policy:$scoreboard_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
positive_packet_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-dry-run-scaffold:side-effects:$scoreboard_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson scoreboard "$SCOREBOARD_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $scoreboard.runtime == "hepta"
    and $scoreboard.status == "ready"
    and $scoreboard.gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_gate"
    and $scoreboard.terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_ready == true
    and $scoreboard.trusted_record_acceptance_precondition_scoreboard_mode == "stdout_only_report_only_precondition_scoreboard_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
    and $scoreboard.trusted_record_acceptance_precondition_scoreboard_status == "blocked"
    and $scoreboard.source_trusted_record_acceptance_negative_fixture_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_gate"
    and $scoreboard.source_trusted_record_acceptance_negative_fixture_matrix_status == "blocked"
    and $scoreboard.source_trusted_record_acceptance_skeleton_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_gate"
    and $scoreboard.source_trusted_record_acceptance_skeleton_status == "blocked"
    and $scoreboard.source_terminal_closure_verdict == "blocked"
    and $scoreboard.source_trusted_record_skeleton_count == 8
    and $scoreboard.source_blocked_trusted_record_skeleton_count == 8
    and $scoreboard.source_accepted_trusted_record_count == 0
    and $scoreboard.source_precondition_family_count == 7
    and $scoreboard.source_required_precondition_check_count == 56
    and $scoreboard.source_satisfied_precondition_check_count == 0
    and $scoreboard.source_negative_fixture_count == 12
    and $scoreboard.source_blocked_negative_fixture_count == 12
    and $scoreboard.source_allowed_negative_fixture_count == 0
    and $scoreboard.required_scoreboard_item_count == 56
    and $scoreboard.scoreboard_item_count == 56
    and $scoreboard.satisfied_scoreboard_item_count == 0
    and $scoreboard.unsatisfied_scoreboard_item_count == 56
    and $scoreboard.precondition_family_count == 7
    and $scoreboard.required_precondition_check_count == 56
    and $scoreboard.satisfied_precondition_check_count == 0
    and $scoreboard.future_positive_family_count == 7
    and $scoreboard.future_positive_family_satisfied_count == 0
    and $scoreboard.future_positive_family_missing_count == 7
    and ($scoreboard.trusted_record_acceptance_precondition_scoreboard_items | length) == 56
    and ($scoreboard.trusted_record_acceptance_precondition_scoreboard_items | all(
      .required == true
      and .status == "missing"
      and .scoreboard_status == "blocked"
      and .satisfied == false
      and .report_only == true
      and .trusted_record_recorded == false
      and .trusted_record_persisted == false
      and .trusted_record_accepted == false
      and .trusted_record_delivered == false
      and .terminal_closure_recorded == false
      and .activation_allowed == false
    ))
    and ($scoreboard.trusted_record_acceptance_future_positive_families | length) == 7
    and ($scoreboard.trusted_record_acceptance_future_positive_families | all(.status == "missing" and .satisfied == false and .trusted_record_acceptance_allowed == false))
    and ($scoreboard.source_negative_fixtures | length) == 12
    and ($scoreboard.source_negative_fixtures | all(.validation_status == "blocked" and .trusted_record_acceptance_allowed == false and .trusted_record_accepted == false))
    and $scoreboard.operator_packet_recorded == false
    and $scoreboard.operator_packet_persisted == false
    and $scoreboard.operator_packet_accepted == false
    and $scoreboard.trusted_record_acceptance_allowed == false
    and $scoreboard.trusted_record_accepted == false
    and $scoreboard.terminal_closure_recorded == false
    and $scoreboard.activation_allowed == false
    and $scoreboard.receipt_accepted == false
    and $scoreboard.ledger_recorded == false
    and $scoreboard.index_delivered == false
    and $scoreboard.completion_ack_accepted == false
    and ($scoreboard.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

denied_reasons_json="$(
  jq -n '
    [
      "trusted_record_positive_packet_dry_run_scaffold_not_acceptance",
      "future_positive_packet_shape_cannot_authorize_by_shape",
      "explicit_operator_approval_record_missing",
      "current_operator_identity_attestation_missing",
      "activation_request_record_missing",
      "activation_request_nonce_binding_missing",
      "fresh_live_evidence_acceptance_missing",
      "trusted_evidence_set_hash_binding_missing",
      "filesystem_approval_record_missing",
      "receipt_persistence_enablement_missing",
      "receipt_persistence_execution_missing",
      "receipt_acceptance_record_missing",
      "ledger_record_missing",
      "index_delivery_record_missing",
      "completion_ack_record_missing",
      "terminal_closure_recording_denied",
      "trusted_record_recording_denied",
      "trusted_record_persistence_denied",
      "trusted_record_acceptance_denied",
      "trusted_record_delivery_denied",
      "activation_execution_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "install_restart_denied",
      "upstream_fetch_merge_denied",
      "credential_secret_read_denied"
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_gate" \
  --arg scoreboard_report_sha256 "$scoreboard_report_sha256" \
  --arg positive_packet_scaffold_hash_sha256 "$positive_packet_scaffold_hash_sha256" \
  --arg positive_packet_policy_hash_sha256 "$positive_packet_policy_hash_sha256" \
  --arg positive_packet_side_effect_hash_sha256 "$positive_packet_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson scoreboard "$SCOREBOARD_JSON" \
  --argjson denied_reasons "$denied_reasons_json" \
  '
    ($scoreboard.trusted_record_acceptance_precondition_scoreboard_items) as $scoreboard_items
    | ($scoreboard.trusted_record_acceptance_future_positive_families) as $future_families
    | ($scoreboard.source_negative_fixtures) as $negative_fixtures
    | ($scoreboard_items
        | map(. + {
            positive_packet_fixture_id: "future-complete-positive-trusted-record-packet-dry-run",
            positive_packet_scoreboard_alignment_status: "represented_but_unsatisfied",
            positive_packet_shape_field_declared: true,
            positive_packet_authority_material_present: false,
            positive_packet_receipt_ledger_material_present: false,
            trusted_record_accepted: false,
            trusted_record_acceptance_allowed: false,
            terminal_closure_recorded: false,
            activation_allowed: false
          })
      ) as $alignment_items
    | ($alignment_items
        | sort_by(.skeleton_record_id)
        | group_by(.skeleton_record_id)
        | map({
            skeleton_record_id: .[0].skeleton_record_id,
            positive_packet_record_id: (.[0].skeleton_record_id + "::future-positive-dry-run"),
            positive_packet_fixture_id: "future-complete-positive-trusted-record-packet-dry-run",
            future_positive_packet_record_shape_declared: true,
            future_positive_packet_record_shape_complete: true,
            scoreboard_item_count: length,
            represented_scoreboard_item_count: length,
            satisfied_scoreboard_item_count: 0,
            unsatisfied_scoreboard_item_count: length,
            precondition_families: map(.precondition_family),
            scoreboard_item_ids: map(.scoreboard_item_id),
            source_negative_fixture_ids: (map(.source_negative_fixture_ids[]) | unique),
            dry_run_only: true,
            report_only: true,
            status: "blocked",
            acceptance_status: "blocked",
            trusted_record_recorded: false,
            trusted_record_persisted: false,
            trusted_record_accepted: false,
            trusted_record_delivered: false,
            trusted_record_fresh: false,
            operator_packet_accepted: false,
            receipt_accepted: false,
            ledger_recorded: false,
            index_delivered: false,
            completion_ack_accepted: false,
            terminal_closure_recorded: false,
            activation_allowed: false
          })
      ) as $positive_packet_records
    | ($future_families
        | map({
            future_positive_family_id,
            family_id,
            required_future_evidence,
            acceptance_instruction,
            positive_packet_shape_declared: true,
            scoreboard_alignment_complete: true,
            current_authority_evidence_present: false,
            status: "blocked",
            satisfied: false,
            report_only: true,
            trusted_record_acceptance_allowed: false,
            trusted_record_accepted: false
          })
      ) as $positive_packet_families
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_schema_version: "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_v1",
        terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_ready: true,
        trusted_record_positive_packet_dry_run_scaffold_mode: "stdout_only_report_only_positive_packet_dry_run_scaffold_no_approval_no_persistence_no_acceptance_no_delivery_no_activation",
        trusted_record_positive_packet_dry_run_scaffold_status: "blocked",
        trusted_record_positive_packet_dry_run_scaffold_decision: "future_complete_positive_packet_shape_is_declared_but_remains_unaccepted_until_explicit_operator_approval_and_receipt_ledger_delivery_ack_persistence_exist",
        source_trusted_record_acceptance_precondition_scoreboard_gate: $scoreboard.gate,
        source_trusted_record_acceptance_precondition_scoreboard_status: $scoreboard.trusted_record_acceptance_precondition_scoreboard_status,
        source_trusted_record_acceptance_precondition_scoreboard_report_sha256: $scoreboard_report_sha256,
        source_trusted_record_acceptance_negative_fixture_matrix_gate: $scoreboard.source_trusted_record_acceptance_negative_fixture_matrix_gate,
        source_trusted_record_acceptance_skeleton_gate: $scoreboard.source_trusted_record_acceptance_skeleton_gate,
        source_operator_packet_authority_replay_matrix_gate: $scoreboard.source_operator_packet_authority_replay_matrix_gate,
        source_operator_packet_dry_run_validator_gate: $scoreboard.source_operator_packet_dry_run_validator_gate,
        source_operator_packet_template_gate: $scoreboard.source_operator_packet_template_gate,
        source_gap_evidence_index_gate: $scoreboard.source_gap_evidence_index_gate,
        source_terminal_closure_gate: $scoreboard.source_terminal_closure_gate,
        source_terminal_closure_verdict: $scoreboard.source_terminal_closure_verdict,
        positive_packet_scaffold_hash_sha256: $positive_packet_scaffold_hash_sha256,
        positive_packet_policy_hash_sha256: $positive_packet_policy_hash_sha256,
        positive_packet_side_effect_hash_sha256: $positive_packet_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        source_scoreboard_item_count: $scoreboard.scoreboard_item_count,
        source_required_scoreboard_item_count: $scoreboard.required_scoreboard_item_count,
        source_satisfied_scoreboard_item_count: $scoreboard.satisfied_scoreboard_item_count,
        source_unsatisfied_scoreboard_item_count: $scoreboard.unsatisfied_scoreboard_item_count,
        source_precondition_family_count: $scoreboard.precondition_family_count,
        source_future_positive_family_count: $scoreboard.future_positive_family_count,
        source_trusted_record_skeleton_count: $scoreboard.source_trusted_record_skeleton_count,
        source_negative_fixture_count: $scoreboard.source_negative_fixture_count,
        source_blocked_negative_fixture_count: $scoreboard.source_blocked_negative_fixture_count,
        source_allowed_negative_fixture_count: $scoreboard.source_allowed_negative_fixture_count,
        required_positive_packet_fixture_count: 1,
        positive_packet_fixture_count: 1,
        blocked_positive_packet_fixture_count: 1,
        accepted_positive_packet_fixture_count: 0,
        positive_packet_authority_granted_count: 0,
        positive_packet_terminal_closure_granted_count: 0,
        positive_packet_activation_granted_count: 0,
        future_positive_packet_fixture: {
          fixture_id: "future-complete-positive-trusted-record-packet-dry-run",
          fixture_kind: "future_positive_packet_shape_scaffold",
          packet_shape_complete: true,
          packet_scoreboard_alignment_complete: true,
          packet_scoreboard_item_count: ($alignment_items | length),
          packet_required_scoreboard_item_count: 56,
          packet_represented_scoreboard_item_count: ($alignment_items | length),
          packet_satisfied_scoreboard_item_count: 0,
          packet_unsatisfied_scoreboard_item_count: ($alignment_items | length),
          packet_future_positive_family_count: ($positive_packet_families | length),
          packet_trusted_record_count: ($positive_packet_records | length),
          packet_source_negative_fixture_count: ($negative_fixtures | length),
          packet_source_blocked_negative_fixture_count: ($negative_fixtures | map(select(.validation_status == "blocked")) | length),
          dry_run_only: true,
          report_only: true,
          status: "blocked",
          acceptance_status: "blocked",
          explicit_operator_approval_record_present: false,
          current_operator_identity_attestation_present: false,
          activation_request_record_present: false,
          fresh_live_evidence_acceptance_present: false,
          receipt_persistence_execution_present: false,
          receipt_acceptance_record_present: false,
          ledger_record_present: false,
          index_delivery_record_present: false,
          completion_ack_record_present: false,
          trusted_record_acceptance_allowed: false,
          trusted_record_accepted: false,
          terminal_closure_recorded: false,
          activation_allowed: false
        },
        future_positive_packet_records: $positive_packet_records,
        future_positive_packet_family_scaffold: $positive_packet_families,
        future_positive_packet_scoreboard_alignment_items: $alignment_items,
        source_negative_fixtures: $negative_fixtures,
        denied_by_trusted_record_positive_packet_dry_run_scaffold: $denied_reasons,
        denied_by_trusted_record_positive_packet_dry_run_scaffold_count: ($denied_reasons | length),
        trusted_record_positive_packet_dry_run_scaffold_executed: true,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        operator_packet_delivered: false,
        operator_packet_authorizes_activation: false,
        operator_packet_authorizes_terminal_closure: false,
        trusted_record_recording_allowed: false,
        trusted_record_persistence_allowed: false,
        trusted_record_acceptance_allowed: false,
        trusted_record_delivery_allowed: false,
        trusted_record_accepted: false,
        trusted_record_delivered: false,
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
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_gate"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_schema_version == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_v1"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_ready == true
  and .trusted_record_positive_packet_dry_run_scaffold_mode == "stdout_only_report_only_positive_packet_dry_run_scaffold_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
  and .trusted_record_positive_packet_dry_run_scaffold_status == "blocked"
  and .source_trusted_record_acceptance_precondition_scoreboard_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_gate"
  and .source_trusted_record_acceptance_precondition_scoreboard_status == "blocked"
  and .source_trusted_record_acceptance_negative_fixture_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_gate"
  and .source_trusted_record_acceptance_skeleton_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_gate"
  and .source_terminal_closure_verdict == "blocked"
  and .source_scoreboard_item_count == 56
  and .source_required_scoreboard_item_count == 56
  and .source_satisfied_scoreboard_item_count == 0
  and .source_unsatisfied_scoreboard_item_count == 56
  and .source_precondition_family_count == 7
  and .source_future_positive_family_count == 7
  and .source_trusted_record_skeleton_count == 8
  and .source_negative_fixture_count == 12
  and .source_blocked_negative_fixture_count == 12
  and .source_allowed_negative_fixture_count == 0
  and .required_positive_packet_fixture_count == 1
  and .positive_packet_fixture_count == 1
  and .blocked_positive_packet_fixture_count == 1
  and .accepted_positive_packet_fixture_count == 0
  and .positive_packet_authority_granted_count == 0
  and .positive_packet_terminal_closure_granted_count == 0
  and .positive_packet_activation_granted_count == 0
  and .future_positive_packet_fixture.fixture_id == "future-complete-positive-trusted-record-packet-dry-run"
  and .future_positive_packet_fixture.packet_shape_complete == true
  and .future_positive_packet_fixture.packet_scoreboard_alignment_complete == true
  and .future_positive_packet_fixture.packet_scoreboard_item_count == 56
  and .future_positive_packet_fixture.packet_required_scoreboard_item_count == 56
  and .future_positive_packet_fixture.packet_represented_scoreboard_item_count == 56
  and .future_positive_packet_fixture.packet_satisfied_scoreboard_item_count == 0
  and .future_positive_packet_fixture.packet_unsatisfied_scoreboard_item_count == 56
  and .future_positive_packet_fixture.packet_future_positive_family_count == 7
  and .future_positive_packet_fixture.packet_trusted_record_count == 8
  and .future_positive_packet_fixture.packet_source_negative_fixture_count == 12
  and .future_positive_packet_fixture.packet_source_blocked_negative_fixture_count == 12
  and .future_positive_packet_fixture.dry_run_only == true
  and .future_positive_packet_fixture.report_only == true
  and .future_positive_packet_fixture.status == "blocked"
  and .future_positive_packet_fixture.acceptance_status == "blocked"
  and .future_positive_packet_fixture.explicit_operator_approval_record_present == false
  and .future_positive_packet_fixture.current_operator_identity_attestation_present == false
  and .future_positive_packet_fixture.activation_request_record_present == false
  and .future_positive_packet_fixture.fresh_live_evidence_acceptance_present == false
  and .future_positive_packet_fixture.receipt_persistence_execution_present == false
  and .future_positive_packet_fixture.receipt_acceptance_record_present == false
  and .future_positive_packet_fixture.ledger_record_present == false
  and .future_positive_packet_fixture.index_delivery_record_present == false
  and .future_positive_packet_fixture.completion_ack_record_present == false
  and .future_positive_packet_fixture.trusted_record_acceptance_allowed == false
  and .future_positive_packet_fixture.trusted_record_accepted == false
  and .future_positive_packet_fixture.terminal_closure_recorded == false
  and .future_positive_packet_fixture.activation_allowed == false
  and (.future_positive_packet_records | length) == 8
  and (.future_positive_packet_records | all(
    .future_positive_packet_record_shape_declared == true
    and .future_positive_packet_record_shape_complete == true
    and .scoreboard_item_count == 7
    and .represented_scoreboard_item_count == 7
    and .satisfied_scoreboard_item_count == 0
    and .unsatisfied_scoreboard_item_count == 7
    and (.precondition_families | length) == 7
    and (.scoreboard_item_ids | length) == 7
    and .dry_run_only == true
    and .report_only == true
    and .status == "blocked"
    and .acceptance_status == "blocked"
    and .trusted_record_recorded == false
    and .trusted_record_persisted == false
    and .trusted_record_accepted == false
    and .trusted_record_delivered == false
    and .trusted_record_fresh == false
    and .operator_packet_accepted == false
    and .receipt_accepted == false
    and .ledger_recorded == false
    and .index_delivered == false
    and .completion_ack_accepted == false
    and .terminal_closure_recorded == false
    and .activation_allowed == false
  ))
  and (.future_positive_packet_family_scaffold | length) == 7
  and (.future_positive_packet_family_scaffold | all(
    .positive_packet_shape_declared == true
    and .scoreboard_alignment_complete == true
    and .current_authority_evidence_present == false
    and .status == "blocked"
    and .satisfied == false
    and .report_only == true
    and .trusted_record_acceptance_allowed == false
    and .trusted_record_accepted == false
  ))
  and (.future_positive_packet_scoreboard_alignment_items | length) == 56
  and (.future_positive_packet_scoreboard_alignment_items | all(
    .positive_packet_fixture_id == "future-complete-positive-trusted-record-packet-dry-run"
    and .positive_packet_scoreboard_alignment_status == "represented_but_unsatisfied"
    and .positive_packet_shape_field_declared == true
    and .positive_packet_authority_material_present == false
    and .positive_packet_receipt_ledger_material_present == false
    and .trusted_record_accepted == false
    and .trusted_record_acceptance_allowed == false
    and .terminal_closure_recorded == false
    and .activation_allowed == false
  ))
  and (.source_negative_fixtures | length) == 12
  and (.source_negative_fixtures | all(.validation_status == "blocked" and .trusted_record_acceptance_allowed == false and .trusted_record_accepted == false))
  and (.denied_by_trusted_record_positive_packet_dry_run_scaffold | length) == 28
  and .denied_by_trusted_record_positive_packet_dry_run_scaffold_count == 28
  and .trusted_record_positive_packet_dry_run_scaffold_executed == true
  and .operator_packet_recorded == false
  and .operator_packet_persisted == false
  and .operator_packet_accepted == false
  and .operator_packet_delivered == false
  and .operator_packet_authorizes_activation == false
  and .operator_packet_authorizes_terminal_closure == false
  and .trusted_record_recording_allowed == false
  and .trusted_record_persistence_allowed == false
  and .trusted_record_acceptance_allowed == false
  and .trusted_record_delivery_allowed == false
  and .trusted_record_accepted == false
  and .trusted_record_delivered == false
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
