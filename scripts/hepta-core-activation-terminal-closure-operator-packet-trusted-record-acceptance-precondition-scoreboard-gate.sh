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

NEGATIVE_MATRIX_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-negative-fixture-matrix-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-negative-fixture-matrix-gate.sh
)"

negative_matrix_report_sha256="$(sha256_text "$NEGATIVE_MATRIX_JSON")"
scoreboard_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-precondition-scoreboard:scoreboard:$negative_matrix_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
scoreboard_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-precondition-scoreboard:policy:$negative_matrix_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
scoreboard_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-precondition-scoreboard:side-effects:$negative_matrix_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson matrix "$NEGATIVE_MATRIX_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $matrix.runtime == "hepta"
    and $matrix.status == "ready"
    and $matrix.gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_gate"
    and $matrix.terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_ready == true
    and $matrix.trusted_record_acceptance_negative_fixture_matrix_mode == "stdout_only_report_only_negative_fixture_matrix_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
    and $matrix.trusted_record_acceptance_negative_fixture_matrix_status == "blocked"
    and $matrix.source_trusted_record_acceptance_skeleton_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_gate"
    and $matrix.source_trusted_record_acceptance_skeleton_status == "blocked"
    and $matrix.source_operator_packet_authority_replay_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_authority_replay_matrix_gate"
    and $matrix.source_operator_packet_dry_run_validator_gate == "hepta_core_activation_terminal_closure_operator_packet_dry_run_validator_gate"
    and $matrix.source_operator_packet_template_gate == "hepta_core_activation_terminal_closure_operator_packet_template_gate"
    and $matrix.source_gap_evidence_index_gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
    and $matrix.source_terminal_closure_verdict == "blocked"
    and $matrix.source_trusted_record_skeleton_count == 8
    and $matrix.source_blocked_trusted_record_skeleton_count == 8
    and $matrix.source_accepted_trusted_record_count == 0
    and $matrix.source_precondition_family_count == 7
    and $matrix.source_required_precondition_check_count == 56
    and $matrix.source_satisfied_precondition_check_count == 0
    and $matrix.source_required_acceptance_record_field_count == 30
    and $matrix.source_authority_replay_fixture_count == 10
    and $matrix.source_validator_fixture_count == 8
    and $matrix.source_operator_packet_template_section_count == 12
    and $matrix.source_unique_operator_packet_field_count == 24
    and $matrix.required_negative_fixture_count == 12
    and $matrix.negative_fixture_count == 12
    and $matrix.blocked_negative_fixture_count == 12
    and $matrix.allowed_negative_fixture_count == 0
    and $matrix.precondition_family_count == 7
    and ($matrix.precondition_families | length) == 7
    and ($matrix.trusted_record_acceptance_negative_fixtures | length) == 12
    and ($matrix.trusted_record_acceptance_negative_fixtures | all(
      .validation_status == "blocked"
      and .acceptance_attempted == true
      and .negative_fixture_only == true
      and .dry_run_only == true
      and .report_only == true
      and .trusted_record_recorded == false
      and .trusted_record_persisted == false
      and .trusted_record_accepted == false
      and .trusted_record_delivered == false
      and .trusted_record_fresh == false
      and .operator_packet_recorded == false
      and .operator_packet_accepted == false
      and .trusted_record_acceptance_allowed == false
      and .trusted_record_delivery_allowed == false
      and .terminal_closure_recorded == false
      and .receipt_accepted == false
      and .ledger_recorded == false
      and .index_delivered == false
      and .completion_ack_accepted == false
      and .activation_allowed == false
      and .public_release_claim_allowed == false
      and .release_artifact_write_allowed == false
    ))
    and $matrix.operator_packet_recorded == false
    and $matrix.operator_packet_accepted == false
    and $matrix.trusted_record_acceptance_allowed == false
    and $matrix.trusted_record_accepted == false
    and $matrix.terminal_closure_recorded == false
    and $matrix.activation_allowed == false
    and ($matrix.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

skeleton_record_ids_json="$(
  jq -n '
    [
      "operator-authority-trusted-record",
      "activation-request-trusted-record",
      "fresh-long-soak-trusted-record",
      "trusted-evidence-set-record",
      "filesystem-approval-trusted-record",
      "receipt-persistence-trusted-record",
      "receipt-ledger-binding-trusted-record",
      "delivery-completion-trusted-record"
    ]
  '
)"

future_positive_families_json="$(
  jq -n '
    [
      {
        family_id: "record-shape",
        future_positive_family_id: "current-complete-trusted-record-shape",
        required_future_evidence: "all 30 required acceptance fields present on known trusted-record shapes",
        acceptance_instruction: "accept only current known shapes with every required field present and hash-bound"
      },
      {
        family_id: "operator-identity-binding",
        future_positive_family_id: "current-operator-identity-attestation",
        required_future_evidence: "operator identity hash and binding method match the current operator packet",
        acceptance_instruction: "bind the trusted record to explicit operator identity and approval attestation"
      },
      {
        family_id: "activation-request-nonce-binding",
        future_positive_family_id: "current-single-use-activation-request",
        required_future_evidence: "activation request id, generation, nonce, and scope are current and single-use",
        acceptance_instruction: "bind every record to the current non-superseded activation request generation"
      },
      {
        family_id: "hash-binding",
        future_positive_family_id: "end-to-end-evidence-receipt-ledger-hash-binding",
        required_future_evidence: "evidence set, long-soak samples, receipt payload, ledger, and delivery hashes agree",
        acceptance_instruction: "accept only if every downstream artifact hash binds to the same evidence chain"
      },
      {
        family_id: "freshness-window",
        future_positive_family_id: "unexpired-fresh-live-evidence-window",
        required_future_evidence: "fresh live evidence is observed and accepted before its freshness window expires",
        acceptance_instruction: "reject stale observations and accept only current live evidence inside the window"
      },
      {
        family_id: "receipt-ledger-precondition",
        future_positive_family_id: "ordered-receipt-persistence-acceptance-ledger-chain",
        required_future_evidence: "receipt persistence enablement, execution, acceptance, and ledger record exist in order",
        acceptance_instruction: "ledger recording cannot precede accepted receipt persistence and acceptance"
      },
      {
        family_id: "delivery-completion-ack-precondition",
        future_positive_family_id: "ordered-index-delivery-completion-ack-chain",
        required_future_evidence: "index delivery and completion acknowledgement bind to accepted receipt and ledger records",
        acceptance_instruction: "delivery cannot become complete before accepted completion acknowledgement"
      }
    ]
  '
)"

denied_reasons_json="$(
  jq -n '
    [
      "trusted_record_acceptance_precondition_scoreboard_not_acceptance",
      "trusted_record_acceptance_precondition_scoreboard_items_missing_denied",
      "trusted_record_acceptance_future_positive_evidence_missing",
      "record_shape_precondition_unsatisfied",
      "operator_identity_binding_precondition_unsatisfied",
      "activation_request_nonce_binding_precondition_unsatisfied",
      "hash_binding_precondition_unsatisfied",
      "freshness_window_precondition_unsatisfied",
      "receipt_ledger_precondition_unsatisfied",
      "delivery_completion_ack_precondition_unsatisfied",
      "negative_fixture_matrix_blocked_inputs_remain_blocked",
      "operator_packet_acceptance_denied",
      "trusted_record_recording_denied",
      "trusted_record_persistence_denied",
      "trusted_record_acceptance_denied",
      "trusted_record_delivery_denied",
      "terminal_closure_recording_denied",
      "receipt_acceptance_denied",
      "ledger_recording_denied",
      "index_delivery_denied",
      "completion_ack_acceptance_denied",
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
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_gate" \
  --arg negative_matrix_report_sha256 "$negative_matrix_report_sha256" \
  --arg scoreboard_hash_sha256 "$scoreboard_hash_sha256" \
  --arg scoreboard_policy_hash_sha256 "$scoreboard_policy_hash_sha256" \
  --arg scoreboard_side_effect_hash_sha256 "$scoreboard_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson matrix "$NEGATIVE_MATRIX_JSON" \
  --argjson skeleton_record_ids "$skeleton_record_ids_json" \
  --argjson future_positive_families "$future_positive_families_json" \
  --argjson denied_reasons "$denied_reasons_json" \
  '
    ($matrix.trusted_record_acceptance_negative_fixtures) as $negative_fixtures
    | ($future_positive_families | map(.family_id)) as $family_ids
    | ([
        $skeleton_record_ids[] as $record_id
        | $future_positive_families[] as $family
        | {
            scoreboard_item_id: ($record_id + "::" + $family.family_id),
            skeleton_record_id: $record_id,
            precondition_family: $family.family_id,
            future_positive_family_id: $family.future_positive_family_id,
            required_future_evidence: $family.required_future_evidence,
            acceptance_instruction: $family.acceptance_instruction,
            source_negative_fixture_ids: ($negative_fixtures | map(select(.precondition_family == $family.family_id) | .fixture_id)),
            source_negative_fixture_count: ($negative_fixtures | map(select(.precondition_family == $family.family_id)) | length),
            required: true,
            status: "missing",
            scoreboard_status: "blocked",
            satisfied: false,
            report_only: true,
            operator_input_required: true,
            future_positive_evidence_required: true,
            trusted_record_shape_declared: true,
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
          }
      ]) as $scoreboard_items
    | ($future_positive_families | map(
        . as $family
        | ($scoreboard_items | map(select(.precondition_family == $family.family_id))) as $family_items
        | ($negative_fixtures | map(select(.precondition_family == $family.family_id))) as $family_negative_fixtures
        | {
            family_id: $family.family_id,
            future_positive_family_id: $family.future_positive_family_id,
            required_future_evidence: $family.required_future_evidence,
            acceptance_instruction: $family.acceptance_instruction,
            required_check_count: ($family_items | length),
            satisfied_check_count: 0,
            unsatisfied_check_count: ($family_items | length),
            source_negative_fixture_count: ($family_negative_fixtures | length),
            source_negative_fixture_ids: ($family_negative_fixtures | map(.fixture_id)),
            status: "blocked",
            satisfied: false,
            report_only: true,
            future_positive_evidence_required: true,
            trusted_record_acceptance_allowed: false,
            trusted_record_accepted: false
          }
      )) as $family_scoreboard
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_schema_version: "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_v1",
        terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_ready: true,
        trusted_record_acceptance_precondition_scoreboard_mode: "stdout_only_report_only_precondition_scoreboard_no_approval_no_persistence_no_acceptance_no_delivery_no_activation",
        trusted_record_acceptance_precondition_scoreboard_status: "blocked",
        trusted_record_acceptance_precondition_scoreboard_decision: "all_56_trusted_record_acceptance_preconditions_remain_unsatisfied_until_current_hash_bound_unexpired_operator_approved_receipt_ledger_delivery_ack_records_exist",
        source_trusted_record_acceptance_negative_fixture_matrix_gate: $matrix.gate,
        source_trusted_record_acceptance_negative_fixture_matrix_status: $matrix.trusted_record_acceptance_negative_fixture_matrix_status,
        source_trusted_record_acceptance_negative_fixture_matrix_report_sha256: $negative_matrix_report_sha256,
        source_trusted_record_acceptance_skeleton_gate: $matrix.source_trusted_record_acceptance_skeleton_gate,
        source_trusted_record_acceptance_skeleton_status: $matrix.source_trusted_record_acceptance_skeleton_status,
        source_operator_packet_authority_replay_matrix_gate: $matrix.source_operator_packet_authority_replay_matrix_gate,
        source_operator_packet_dry_run_validator_gate: $matrix.source_operator_packet_dry_run_validator_gate,
        source_operator_packet_template_gate: $matrix.source_operator_packet_template_gate,
        source_gap_evidence_index_gate: $matrix.source_gap_evidence_index_gate,
        source_terminal_closure_gate: $matrix.source_terminal_closure_gate,
        source_terminal_closure_verdict: $matrix.source_terminal_closure_verdict,
        scoreboard_hash_sha256: $scoreboard_hash_sha256,
        scoreboard_policy_hash_sha256: $scoreboard_policy_hash_sha256,
        scoreboard_side_effect_hash_sha256: $scoreboard_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        source_trusted_record_skeleton_count: $matrix.source_trusted_record_skeleton_count,
        source_blocked_trusted_record_skeleton_count: $matrix.source_blocked_trusted_record_skeleton_count,
        source_accepted_trusted_record_count: $matrix.source_accepted_trusted_record_count,
        source_precondition_family_count: $matrix.source_precondition_family_count,
        source_required_precondition_check_count: $matrix.source_required_precondition_check_count,
        source_satisfied_precondition_check_count: $matrix.source_satisfied_precondition_check_count,
        source_required_acceptance_record_field_count: $matrix.source_required_acceptance_record_field_count,
        source_negative_fixture_count: $matrix.negative_fixture_count,
        source_blocked_negative_fixture_count: $matrix.blocked_negative_fixture_count,
        source_allowed_negative_fixture_count: $matrix.allowed_negative_fixture_count,
        source_authority_replay_fixture_count: $matrix.source_authority_replay_fixture_count,
        source_validator_fixture_count: $matrix.source_validator_fixture_count,
        source_operator_packet_template_section_count: $matrix.source_operator_packet_template_section_count,
        source_unique_operator_packet_field_count: $matrix.source_unique_operator_packet_field_count,
        required_scoreboard_item_count: 56,
        scoreboard_item_count: ($scoreboard_items | length),
        satisfied_scoreboard_item_count: ($scoreboard_items | map(select(.satisfied == true)) | length),
        unsatisfied_scoreboard_item_count: ($scoreboard_items | map(select(.satisfied == false)) | length),
        precondition_family_count: ($family_scoreboard | length),
        precondition_family_ready_count: 0,
        precondition_family_blocked_count: ($family_scoreboard | map(select(.status == "blocked")) | length),
        required_precondition_check_count: ($scoreboard_items | length),
        satisfied_precondition_check_count: 0,
        unsatisfied_precondition_check_count: ($scoreboard_items | length),
        future_positive_family_count: ($future_positive_families | length),
        future_positive_family_satisfied_count: 0,
        future_positive_family_missing_count: ($future_positive_families | length),
        record_shape_scoreboard_item_count: ($scoreboard_items | map(select(.precondition_family == "record-shape")) | length),
        operator_identity_scoreboard_item_count: ($scoreboard_items | map(select(.precondition_family == "operator-identity-binding")) | length),
        activation_request_nonce_scoreboard_item_count: ($scoreboard_items | map(select(.precondition_family == "activation-request-nonce-binding")) | length),
        hash_binding_scoreboard_item_count: ($scoreboard_items | map(select(.precondition_family == "hash-binding")) | length),
        freshness_window_scoreboard_item_count: ($scoreboard_items | map(select(.precondition_family == "freshness-window")) | length),
        receipt_ledger_scoreboard_item_count: ($scoreboard_items | map(select(.precondition_family == "receipt-ledger-precondition")) | length),
        delivery_completion_ack_scoreboard_item_count: ($scoreboard_items | map(select(.precondition_family == "delivery-completion-ack-precondition")) | length),
        trusted_record_acceptance_precondition_family_scoreboard: $family_scoreboard,
        trusted_record_acceptance_precondition_scoreboard_items: $scoreboard_items,
        trusted_record_acceptance_future_positive_families: ($future_positive_families | map(. + {
          status: "missing",
          satisfied: false,
          report_only: true,
          trusted_record_acceptance_allowed: false
        })),
        source_negative_fixtures: $negative_fixtures,
        denied_by_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard: $denied_reasons,
        denied_by_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_count: ($denied_reasons | length),
        trusted_record_acceptance_precondition_scoreboard_executed: true,
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
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_gate"
  and .terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_schema_version == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_v1"
  and .terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_ready == true
  and .trusted_record_acceptance_precondition_scoreboard_mode == "stdout_only_report_only_precondition_scoreboard_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
  and .trusted_record_acceptance_precondition_scoreboard_status == "blocked"
  and .source_trusted_record_acceptance_negative_fixture_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_gate"
  and .source_trusted_record_acceptance_negative_fixture_matrix_status == "blocked"
  and .source_trusted_record_acceptance_skeleton_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_gate"
  and .source_trusted_record_acceptance_skeleton_status == "blocked"
  and .source_operator_packet_authority_replay_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_authority_replay_matrix_gate"
  and .source_operator_packet_dry_run_validator_gate == "hepta_core_activation_terminal_closure_operator_packet_dry_run_validator_gate"
  and .source_operator_packet_template_gate == "hepta_core_activation_terminal_closure_operator_packet_template_gate"
  and .source_gap_evidence_index_gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
  and .source_terminal_closure_verdict == "blocked"
  and .source_trusted_record_skeleton_count == 8
  and .source_blocked_trusted_record_skeleton_count == 8
  and .source_accepted_trusted_record_count == 0
  and .source_precondition_family_count == 7
  and .source_required_precondition_check_count == 56
  and .source_satisfied_precondition_check_count == 0
  and .source_required_acceptance_record_field_count == 30
  and .source_negative_fixture_count == 12
  and .source_blocked_negative_fixture_count == 12
  and .source_allowed_negative_fixture_count == 0
  and .source_authority_replay_fixture_count == 10
  and .source_validator_fixture_count == 8
  and .source_operator_packet_template_section_count == 12
  and .source_unique_operator_packet_field_count == 24
  and .required_scoreboard_item_count == 56
  and .scoreboard_item_count == 56
  and .satisfied_scoreboard_item_count == 0
  and .unsatisfied_scoreboard_item_count == 56
  and .precondition_family_count == 7
  and .precondition_family_ready_count == 0
  and .precondition_family_blocked_count == 7
  and .required_precondition_check_count == 56
  and .satisfied_precondition_check_count == 0
  and .unsatisfied_precondition_check_count == 56
  and .future_positive_family_count == 7
  and .future_positive_family_satisfied_count == 0
  and .future_positive_family_missing_count == 7
  and .record_shape_scoreboard_item_count == 8
  and .operator_identity_scoreboard_item_count == 8
  and .activation_request_nonce_scoreboard_item_count == 8
  and .hash_binding_scoreboard_item_count == 8
  and .freshness_window_scoreboard_item_count == 8
  and .receipt_ledger_scoreboard_item_count == 8
  and .delivery_completion_ack_scoreboard_item_count == 8
  and (.trusted_record_acceptance_precondition_family_scoreboard | length) == 7
  and (.trusted_record_acceptance_precondition_family_scoreboard | all(
    .required_check_count == 8
    and .satisfied_check_count == 0
    and .unsatisfied_check_count == 8
    and .source_negative_fixture_count >= 1
    and .status == "blocked"
    and .satisfied == false
    and .report_only == true
    and .future_positive_evidence_required == true
    and .trusted_record_acceptance_allowed == false
    and .trusted_record_accepted == false
  ))
  and (.trusted_record_acceptance_precondition_family_scoreboard | any(.family_id == "record-shape" and .source_negative_fixture_count == 2))
  and (.trusted_record_acceptance_precondition_family_scoreboard | any(.family_id == "operator-identity-binding" and .source_negative_fixture_count == 2))
  and (.trusted_record_acceptance_precondition_family_scoreboard | any(.family_id == "activation-request-nonce-binding" and .source_negative_fixture_count == 2))
  and (.trusted_record_acceptance_precondition_family_scoreboard | any(.family_id == "hash-binding" and .source_negative_fixture_count == 2))
  and (.trusted_record_acceptance_precondition_family_scoreboard | any(.family_id == "freshness-window" and .source_negative_fixture_count == 1))
  and (.trusted_record_acceptance_precondition_family_scoreboard | any(.family_id == "receipt-ledger-precondition" and .source_negative_fixture_count == 2))
  and (.trusted_record_acceptance_precondition_family_scoreboard | any(.family_id == "delivery-completion-ack-precondition" and .source_negative_fixture_count == 1))
  and (.trusted_record_acceptance_precondition_scoreboard_items | length) == 56
  and (.trusted_record_acceptance_precondition_scoreboard_items | all(
    .required == true
    and .status == "missing"
    and .scoreboard_status == "blocked"
    and .satisfied == false
    and .report_only == true
    and .operator_input_required == true
    and .future_positive_evidence_required == true
    and .trusted_record_shape_declared == true
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
  and (.trusted_record_acceptance_precondition_scoreboard_items | any(.skeleton_record_id == "operator-authority-trusted-record" and .precondition_family == "record-shape" and (.source_negative_fixture_ids | index("missing-required-record-shape-field") != null)))
  and (.trusted_record_acceptance_precondition_scoreboard_items | any(.skeleton_record_id == "activation-request-trusted-record" and .precondition_family == "activation-request-nonce-binding" and (.source_negative_fixture_ids | index("activation-request-nonce-replay") != null)))
  and (.trusted_record_acceptance_precondition_scoreboard_items | any(.skeleton_record_id == "receipt-ledger-binding-trusted-record" and .precondition_family == "receipt-ledger-precondition" and (.source_negative_fixture_ids | index("ledger-record-before-receipt-acceptance") != null)))
  and (.trusted_record_acceptance_precondition_scoreboard_items | any(.skeleton_record_id == "delivery-completion-trusted-record" and .precondition_family == "delivery-completion-ack-precondition" and (.source_negative_fixture_ids | index("delivery-before-completion-ack") != null)))
  and (.trusted_record_acceptance_future_positive_families | length) == 7
  and (.trusted_record_acceptance_future_positive_families | all(
    .status == "missing"
    and .satisfied == false
    and .report_only == true
    and .trusted_record_acceptance_allowed == false
  ))
  and (.source_negative_fixtures | length) == 12
  and (.source_negative_fixtures | all(.validation_status == "blocked" and .trusted_record_acceptance_allowed == false and .trusted_record_accepted == false))
  and (.denied_by_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard | length) == 29
  and .denied_by_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_count == 29
  and .trusted_record_acceptance_precondition_scoreboard_executed == true
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
