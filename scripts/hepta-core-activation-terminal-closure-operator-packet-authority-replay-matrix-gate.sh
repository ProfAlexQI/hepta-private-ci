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

VALIDATOR_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-dry-run-validator-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-dry-run-validator-gate.sh
)"

validator_report_sha256="$(sha256_text "$VALIDATOR_JSON")"
authority_replay_matrix_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-authority-replay-matrix:matrix:$validator_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
authority_replay_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-authority-replay-matrix:policy:$validator_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
authority_replay_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-authority-replay-matrix:side-effects:$validator_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson validator "$VALIDATOR_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $validator.runtime == "hepta"
    and $validator.status == "ready"
    and $validator.gate == "hepta_core_activation_terminal_closure_operator_packet_dry_run_validator_gate"
    and $validator.terminal_closure_operator_packet_dry_run_validator_ready == true
    and $validator.operator_packet_dry_run_validator_mode == "stdout_only_report_only_validator_no_approval_no_persistence_no_delivery_no_activation"
    and $validator.operator_packet_dry_run_validator_status == "blocked"
    and $validator.source_operator_packet_template_gate == "hepta_core_activation_terminal_closure_operator_packet_template_gate"
    and $validator.source_operator_packet_template_status == "blocked"
    and $validator.source_gap_evidence_index_gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
    and $validator.source_gap_evidence_index_status == "blocked"
    and $validator.source_terminal_closure_verdict == "blocked"
    and $validator.source_required_gap_evidence_count == 12
    and $validator.source_indexed_gap_evidence_count == 12
    and $validator.source_operator_packet_template_section_count == 12
    and $validator.source_unique_operator_packet_field_count == 24
    and $validator.required_validator_fixture_count == 8
    and $validator.validator_fixture_count == 8
    and $validator.blocked_validator_fixture_count == 8
    and $validator.allowed_validator_fixture_count == 0
    and $validator.future_packet_authority_denied_count == 8
    and ($validator.operator_packet_required_fields | length) == 24
    and ($validator.operator_packet_template_sections | length) == 12
    and ($validator.validator_fixtures | length) == 8
    and ($validator.validator_fixtures | all(
      .validation_status == "blocked"
      and .dry_run_only == true
      and .report_only == true
      and .validator_only == true
      and .operator_input_required == true
      and .operator_packet_recorded == false
      and .operator_packet_persisted == false
      and .operator_packet_accepted == false
      and .operator_packet_delivered == false
      and .operator_packet_authorizes_activation == false
      and .operator_packet_authorizes_terminal_closure == false
      and .terminal_closure_allowed == false
      and .terminal_closure_recorded == false
      and .terminal_closure_accepted == false
      and .activation_allowed == false
      and .receipt_persisted == false
      and .receipt_accepted == false
      and .ledger_recorded == false
      and .index_delivered == false
      and .completion_ack_accepted == false
      and .public_release_claim_allowed == false
      and .release_artifact_write_allowed == false
    ))
    and ($validator.validator_fixtures | any(.fixture_id == "complete-shape-without-recording-authority" and .complete_shape_without_recording_authority == true))
    and ($validator.validator_fixtures | any(.fixture_id == "cross-request-evidence-or-approval-reuse" and .cross_request_reuse == true))
    and ($validator.validator_fixtures | any(.fixture_id == "stale-or-expired-long-soak-evidence" and .stale_or_expired_evidence == true))
    and ($validator.validator_fixtures | any(.fixture_id == "receipt-ledger-ack-without-acceptance"))
    and ($validator.validator_fixtures | any(.fixture_id == "public-claim-or-artifact-attempt" and .public_claim_or_artifact_requested == true))
    and $validator.operator_packet_recorded == false
    and $validator.operator_packet_persisted == false
    and $validator.operator_packet_accepted == false
    and $validator.operator_packet_delivered == false
    and $validator.operator_packet_authorizes_activation == false
    and $validator.operator_packet_authorizes_terminal_closure == false
    and $validator.operator_packet_authorizes_receipt_persistence == false
    and $validator.operator_packet_authorizes_ledger_recording == false
    and $validator.terminal_closure_recorded == false
    and $validator.activation_allowed == false
    and $validator.receipt_persistence_allowed == false
    and $validator.receipt_acceptance_allowed == false
    and $validator.ledger_recording_allowed == false
    and $validator.index_delivery_allowed == false
    and $validator.completion_ack_acceptance_allowed == false
    and $validator.public_release_claim_allowed == false
    and $validator.release_artifact_write_allowed == false
    and ($validator.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

fixture_specs=(
  "template-fixture-replayed-to-terminal-closure|terminal_closure|hepta_core_activation_evidence_receipt_terminal_closure_decision_gate|template-only-no-packet-record|0|24|packet_not_recorded|template output cannot be replayed as terminal closure authority|true|false|false|false|false|false"
  "complete-shape-replayed-to-terminal-closure|terminal_closure|hepta_core_activation_evidence_receipt_terminal_closure_decision_gate|complete-shape-without-recording-authority|24|0|complete_shape_not_recorded_authority|complete packet shape cannot replace recorded and accepted operator authority|true|false|false|false|false|false"
  "complete-shape-replayed-to-receipt-acceptance|receipt_acceptance|hepta_core_activation_evidence_receipt_acceptance_denial_gate|complete-shape-without-recording-authority|24|0|receipt_acceptance_authority_missing|complete packet shape cannot accept a receipt without real receipt persistence and authority|false|true|false|false|false|false"
  "complete-shape-replayed-to-ledger-record|ledger_record|hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate|complete-shape-without-recording-authority|24|0|ledger_record_authority_missing|complete packet shape cannot record ledger state without accepted evidence and receipt chain|false|false|true|false|false|false"
  "cross-request-packet-replay-to-terminal-closure|terminal_closure|hepta_core_activation_evidence_receipt_terminal_closure_decision_gate|cross-request-evidence-or-approval-reuse|24|0|cross_request_reuse_denied|cross-request approval or evidence cannot close the current activation request|true|false|false|false|false|false"
  "stale-evidence-packet-replay-to-receipt-ledger|receipt_ledger|hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate|stale-or-expired-long-soak-evidence|24|0|fresh_evidence_expired|stale or expired long-soak evidence cannot be replayed into receipt or ledger authority|false|true|true|false|false|false"
  "receipt-ledger-ack-replay-to-terminal-closure|terminal_closure|hepta_core_activation_evidence_receipt_terminal_closure_decision_gate|receipt-ledger-ack-without-acceptance|22|2|receipt_ledger_ack_without_acceptance_denied|receipt ledger and acknowledgement shape cannot bypass receipt acceptance|true|true|true|true|true|false"
  "public-claim-packet-replay-to-release-governance|public_release|hepta_upstream_codex_latest_release_governance_non_activation_gate|public-claim-or-artifact-attempt|24|0|public_claim_denied|public claim and release artifact attempts remain denied without accepted operator authority|false|false|false|false|false|true"
  "delivered-index-without-accepted-packet-replay|index_delivery|hepta_core_activation_evidence_receipt_acceptance_denial_gate|complete-shape-without-recording-authority|24|0|delivery_without_acceptance_denied|index delivery cannot stand in for accepted operator packet and receipt records|false|false|false|true|true|false"
  "superseded-packet-pair-replay-to-current-request|activation_request|hepta_core_activation_operator_approval_fresh_evidence_supersession_expiry_denial_gate|cross-request-evidence-or-approval-reuse|24|0|superseded_pair_replay_denied|superseded approval evidence pairs cannot be replayed as current request authority|true|false|false|false|false|false"
)

fixture_specs_json="$(
  printf '%s\n' "${fixture_specs[@]}" \
    | jq -R -s '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            fixture_id: .[0],
            replay_surface: .[1],
            replay_target_gate: .[2],
            source_validator_fixture_id: .[3],
            packet_shape_field_count: (.[4] | tonumber),
            missing_operator_packet_field_count: (.[5] | tonumber),
            authority_failure_id: .[6],
            authority_failure_reason: .[7],
            terminal_closure_replay_requested: (.[8] == "true"),
            receipt_acceptance_replay_requested: (.[9] == "true"),
            ledger_recording_replay_requested: (.[10] == "true"),
            index_delivery_replay_requested: (.[11] == "true"),
            completion_ack_replay_requested: (.[12] == "true"),
            public_claim_or_artifact_replay_requested: (.[13] == "true")
          })
      '
)"

entry_point_specs_json="$(
  jq -n '
    [
      {
        entry_point_id: "terminal-closure",
        target_gate: "hepta_core_activation_evidence_receipt_terminal_closure_decision_gate",
        accepted_authority_required: true,
        replay_authority_allowed: false,
        terminal_closure_recorded: false
      },
      {
        entry_point_id: "receipt-acceptance",
        target_gate: "hepta_core_activation_evidence_receipt_acceptance_denial_gate",
        accepted_authority_required: true,
        replay_authority_allowed: false,
        receipt_accepted: false
      },
      {
        entry_point_id: "ledger-record",
        target_gate: "hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate",
        accepted_authority_required: true,
        replay_authority_allowed: false,
        ledger_recorded: false
      },
      {
        entry_point_id: "index-delivery",
        target_gate: "hepta_core_activation_evidence_receipt_acceptance_denial_gate",
        accepted_authority_required: true,
        replay_authority_allowed: false,
        index_delivered: false
      },
      {
        entry_point_id: "completion-ack",
        target_gate: "hepta_core_activation_evidence_receipt_acceptance_denial_gate",
        accepted_authority_required: true,
        replay_authority_allowed: false,
        completion_ack_accepted: false
      },
      {
        entry_point_id: "public-release-claim",
        target_gate: "hepta_upstream_codex_latest_release_governance_non_activation_gate",
        accepted_authority_required: true,
        replay_authority_allowed: false,
        public_release_claim_allowed: false,
        release_artifact_write_allowed: false
      }
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_authority_replay_matrix_gate" \
  --arg validator_report_sha256 "$validator_report_sha256" \
  --arg authority_replay_matrix_hash_sha256 "$authority_replay_matrix_hash_sha256" \
  --arg authority_replay_policy_hash_sha256 "$authority_replay_policy_hash_sha256" \
  --arg authority_replay_side_effect_hash_sha256 "$authority_replay_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson validator "$VALIDATOR_JSON" \
  --argjson fixture_specs "$fixture_specs_json" \
  --argjson entry_point_specs "$entry_point_specs_json" \
  '
    ($fixture_specs | map(
      . as $spec
      | ($validator.validator_fixtures[] | select(.fixture_id == $spec.source_validator_fixture_id)) as $source_fixture
      | {
          fixture_id: $spec.fixture_id,
          replay_surface: $spec.replay_surface,
          replay_target_gate: $spec.replay_target_gate,
          source_validator_fixture_id: $spec.source_validator_fixture_id,
          source_validator_fixture_status: $source_fixture.validation_status,
          source_fixture_witness_hash_sha256: $source_fixture.fixture_witness_hash_sha256,
          packet_shape_field_count: $spec.packet_shape_field_count,
          missing_operator_packet_field_count: $spec.missing_operator_packet_field_count,
          authority_failure_id: $spec.authority_failure_id,
          authority_failure_reason: $spec.authority_failure_reason,
          validation_status: "blocked",
          replay_attempted: true,
          dry_run_only: true,
          report_only: true,
          matrix_only: true,
          terminal_closure_replay_requested: $spec.terminal_closure_replay_requested,
          receipt_acceptance_replay_requested: $spec.receipt_acceptance_replay_requested,
          ledger_recording_replay_requested: $spec.ledger_recording_replay_requested,
          index_delivery_replay_requested: $spec.index_delivery_replay_requested,
          completion_ack_replay_requested: $spec.completion_ack_replay_requested,
          public_claim_or_artifact_replay_requested: $spec.public_claim_or_artifact_replay_requested,
          operator_packet_recorded: false,
          operator_packet_persisted: false,
          operator_packet_accepted: false,
          operator_packet_delivered: false,
          operator_packet_authorizes_activation: false,
          operator_packet_authorizes_terminal_closure: false,
          terminal_closure_allowed: false,
          terminal_closure_recorded: false,
          terminal_closure_accepted: false,
          receipt_persistence_allowed: false,
          receipt_accepted: false,
          ledger_recorded: false,
          index_delivered: false,
          completion_ack_accepted: false,
          activation_allowed: false,
          activation_performed: false,
          public_release_claim_allowed: false,
          release_artifact_write_allowed: false
        }
    )) as $fixtures
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_closure_operator_packet_authority_replay_matrix_schema_version: "hepta_core_activation_terminal_closure_operator_packet_authority_replay_matrix_v1",
        terminal_closure_operator_packet_authority_replay_matrix_ready: true,
        operator_packet_authority_replay_matrix_mode: "stdout_only_report_only_matrix_no_approval_no_persistence_no_delivery_no_activation",
        operator_packet_authority_replay_matrix_status: "blocked",
        operator_packet_authority_replay_matrix_decision: "all_packet_authority_replay_fixtures_blocked_until_real_operator_packet_authority_receipt_ledger_delivery_and_ack_records_exist",
        source_operator_packet_dry_run_validator_gate: $validator.gate,
        source_operator_packet_dry_run_validator_status: $validator.operator_packet_dry_run_validator_status,
        source_operator_packet_dry_run_validator_report_sha256: $validator_report_sha256,
        source_operator_packet_template_gate: $validator.source_operator_packet_template_gate,
        source_operator_packet_template_status: $validator.source_operator_packet_template_status,
        source_operator_packet_template_report_sha256: $validator.source_operator_packet_template_report_sha256,
        source_gap_evidence_index_gate: $validator.source_gap_evidence_index_gate,
        source_gap_evidence_index_status: $validator.source_gap_evidence_index_status,
        source_terminal_closure_gate: $validator.source_terminal_closure_gate,
        source_terminal_closure_verdict: $validator.source_terminal_closure_verdict,
        authority_replay_matrix_hash_sha256: $authority_replay_matrix_hash_sha256,
        authority_replay_policy_hash_sha256: $authority_replay_policy_hash_sha256,
        authority_replay_side_effect_hash_sha256: $authority_replay_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        source_validator_fixture_count: $validator.validator_fixture_count,
        source_blocked_validator_fixture_count: $validator.blocked_validator_fixture_count,
        source_allowed_validator_fixture_count: $validator.allowed_validator_fixture_count,
        source_operator_packet_template_section_count: $validator.source_operator_packet_template_section_count,
        source_unique_operator_packet_field_count: $validator.source_unique_operator_packet_field_count,
        required_authority_replay_fixture_count: 10,
        authority_replay_fixture_count: ($fixtures | length),
        blocked_authority_replay_fixture_count: ($fixtures | map(select(.validation_status == "blocked")) | length),
        allowed_authority_replay_fixture_count: 0,
        terminal_closure_replay_fixture_count: ($fixtures | map(select(.terminal_closure_replay_requested == true)) | length),
        receipt_acceptance_replay_fixture_count: ($fixtures | map(select(.receipt_acceptance_replay_requested == true)) | length),
        ledger_recording_replay_fixture_count: ($fixtures | map(select(.ledger_recording_replay_requested == true)) | length),
        index_delivery_replay_fixture_count: ($fixtures | map(select(.index_delivery_replay_requested == true)) | length),
        completion_ack_replay_fixture_count: ($fixtures | map(select(.completion_ack_replay_requested == true)) | length),
        public_claim_or_artifact_replay_fixture_count: ($fixtures | map(select(.public_claim_or_artifact_replay_requested == true)) | length),
        complete_shape_replay_fixture_count: ($fixtures | map(select(.packet_shape_field_count == 24 and .missing_operator_packet_field_count == 0)) | length),
        cross_request_replay_fixture_count: ($fixtures | map(select(.authority_failure_id | test("cross_request|superseded"))) | length),
        replay_entry_point_count: ($entry_point_specs | length),
        replay_entry_points: $entry_point_specs,
        authority_replay_matrix_fixtures: $fixtures,
        denied_by_terminal_closure_operator_packet_authority_replay_matrix: [
          "operator_packet_replay_recording_denied",
          "operator_packet_replay_persistence_denied",
          "operator_packet_replay_acceptance_denied",
          "operator_packet_replay_delivery_denied",
          "complete_shape_replay_authority_denied",
          "template_fixture_terminal_closure_replay_denied",
          "cross_request_replay_denied",
          "superseded_pair_replay_denied",
          "stale_evidence_replay_denied",
          "receipt_acceptance_replay_denied",
          "ledger_record_replay_denied",
          "index_delivery_replay_denied",
          "completion_ack_replay_denied",
          "terminal_closure_replay_denied",
          "activation_execution_denied",
          "public_release_claim_denied",
          "release_artifact_write_denied",
          "install_restart_denied",
          "provider_model_invocation_denied",
          "channel_delivery_denied",
          "upstream_fetch_merge_denied",
          "credential_secret_read_denied"
        ],
        denied_by_terminal_closure_operator_packet_authority_replay_matrix_count: 22,
        operator_packet_authority_replay_matrix_executed: true,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        operator_packet_delivered: false,
        operator_packet_authorizes_activation: false,
        operator_packet_authorizes_terminal_closure: false,
        operator_packet_authorizes_receipt_persistence: false,
        operator_packet_authorizes_ledger_recording: false,
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
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_authority_replay_matrix_gate"
  and .terminal_closure_operator_packet_authority_replay_matrix_schema_version == "hepta_core_activation_terminal_closure_operator_packet_authority_replay_matrix_v1"
  and .terminal_closure_operator_packet_authority_replay_matrix_ready == true
  and .operator_packet_authority_replay_matrix_mode == "stdout_only_report_only_matrix_no_approval_no_persistence_no_delivery_no_activation"
  and .operator_packet_authority_replay_matrix_status == "blocked"
  and .source_operator_packet_dry_run_validator_gate == "hepta_core_activation_terminal_closure_operator_packet_dry_run_validator_gate"
  and .source_operator_packet_dry_run_validator_status == "blocked"
  and .source_operator_packet_template_gate == "hepta_core_activation_terminal_closure_operator_packet_template_gate"
  and .source_gap_evidence_index_gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
  and .source_terminal_closure_verdict == "blocked"
  and .source_validator_fixture_count == 8
  and .source_blocked_validator_fixture_count == 8
  and .source_allowed_validator_fixture_count == 0
  and .source_operator_packet_template_section_count == 12
  and .source_unique_operator_packet_field_count == 24
  and .required_authority_replay_fixture_count == 10
  and .authority_replay_fixture_count == 10
  and .blocked_authority_replay_fixture_count == 10
  and .allowed_authority_replay_fixture_count == 0
  and .terminal_closure_replay_fixture_count == 5
  and .receipt_acceptance_replay_fixture_count == 3
  and .ledger_recording_replay_fixture_count == 3
  and .index_delivery_replay_fixture_count == 2
  and .completion_ack_replay_fixture_count == 2
  and .public_claim_or_artifact_replay_fixture_count == 1
  and .complete_shape_replay_fixture_count == 8
  and .cross_request_replay_fixture_count == 2
  and .replay_entry_point_count == 6
  and (.replay_entry_points | all(.replay_authority_allowed == false and .accepted_authority_required == true))
  and (.authority_replay_matrix_fixtures | length) == 10
  and (.authority_replay_matrix_fixtures | all(
    .validation_status == "blocked"
    and .replay_attempted == true
    and .dry_run_only == true
    and .report_only == true
    and .matrix_only == true
    and .source_validator_fixture_status == "blocked"
    and (.source_fixture_witness_hash_sha256 | test("^[0-9a-f]{64}$"))
    and .operator_packet_recorded == false
    and .operator_packet_persisted == false
    and .operator_packet_accepted == false
    and .operator_packet_delivered == false
    and .operator_packet_authorizes_activation == false
    and .operator_packet_authorizes_terminal_closure == false
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
  and (.authority_replay_matrix_fixtures | any(.fixture_id == "template-fixture-replayed-to-terminal-closure" and .packet_shape_field_count == 0 and .missing_operator_packet_field_count == 24))
  and (.authority_replay_matrix_fixtures | any(.fixture_id == "complete-shape-replayed-to-terminal-closure" and .terminal_closure_replay_requested == true))
  and (.authority_replay_matrix_fixtures | any(.fixture_id == "complete-shape-replayed-to-receipt-acceptance" and .receipt_acceptance_replay_requested == true))
  and (.authority_replay_matrix_fixtures | any(.fixture_id == "complete-shape-replayed-to-ledger-record" and .ledger_recording_replay_requested == true))
  and (.authority_replay_matrix_fixtures | any(.fixture_id == "cross-request-packet-replay-to-terminal-closure" and .authority_failure_id == "cross_request_reuse_denied"))
  and (.authority_replay_matrix_fixtures | any(.fixture_id == "stale-evidence-packet-replay-to-receipt-ledger" and .authority_failure_id == "fresh_evidence_expired"))
  and (.authority_replay_matrix_fixtures | any(.fixture_id == "receipt-ledger-ack-replay-to-terminal-closure" and .completion_ack_replay_requested == true))
  and (.authority_replay_matrix_fixtures | any(.fixture_id == "public-claim-packet-replay-to-release-governance" and .public_claim_or_artifact_replay_requested == true))
  and (.authority_replay_matrix_fixtures | any(.fixture_id == "delivered-index-without-accepted-packet-replay" and .index_delivery_replay_requested == true))
  and (.authority_replay_matrix_fixtures | any(.fixture_id == "superseded-packet-pair-replay-to-current-request" and .authority_failure_id == "superseded_pair_replay_denied"))
  and .denied_by_terminal_closure_operator_packet_authority_replay_matrix_count == 22
  and (.denied_by_terminal_closure_operator_packet_authority_replay_matrix | length) == 22
  and .operator_packet_authority_replay_matrix_executed == true
  and .operator_packet_recorded == false
  and .operator_packet_persisted == false
  and .operator_packet_accepted == false
  and .operator_packet_delivered == false
  and .operator_packet_authorizes_activation == false
  and .operator_packet_authorizes_terminal_closure == false
  and .operator_packet_authorizes_receipt_persistence == false
  and .operator_packet_authorizes_ledger_recording == false
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
