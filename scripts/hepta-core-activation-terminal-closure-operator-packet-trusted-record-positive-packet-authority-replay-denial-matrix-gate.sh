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

POSITIVE_SCAFFOLD_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-dry-run-scaffold-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-dry-run-scaffold-gate.sh
)"

positive_scaffold_report_sha256="$(sha256_text "$POSITIVE_SCAFFOLD_JSON")"
authority_replay_matrix_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-matrix:matrix:$positive_scaffold_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
authority_replay_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-matrix:policy:$positive_scaffold_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
authority_replay_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-matrix:side-effects:$positive_scaffold_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson scaffold "$POSITIVE_SCAFFOLD_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $scaffold.runtime == "hepta"
    and $scaffold.status == "ready"
    and $scaffold.gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_gate"
    and $scaffold.terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_ready == true
    and $scaffold.trusted_record_positive_packet_dry_run_scaffold_mode == "stdout_only_report_only_positive_packet_dry_run_scaffold_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
    and $scaffold.trusted_record_positive_packet_dry_run_scaffold_status == "blocked"
    and $scaffold.source_trusted_record_acceptance_precondition_scoreboard_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_gate"
    and $scaffold.source_trusted_record_acceptance_precondition_scoreboard_status == "blocked"
    and $scaffold.source_trusted_record_acceptance_negative_fixture_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_gate"
    and $scaffold.source_trusted_record_acceptance_skeleton_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_gate"
    and $scaffold.source_terminal_closure_verdict == "blocked"
    and $scaffold.source_scoreboard_item_count == 56
    and $scaffold.source_required_scoreboard_item_count == 56
    and $scaffold.source_satisfied_scoreboard_item_count == 0
    and $scaffold.source_unsatisfied_scoreboard_item_count == 56
    and $scaffold.source_precondition_family_count == 7
    and $scaffold.source_future_positive_family_count == 7
    and $scaffold.source_trusted_record_skeleton_count == 8
    and $scaffold.source_negative_fixture_count == 12
    and $scaffold.source_blocked_negative_fixture_count == 12
    and $scaffold.source_allowed_negative_fixture_count == 0
    and $scaffold.required_positive_packet_fixture_count == 1
    and $scaffold.positive_packet_fixture_count == 1
    and $scaffold.blocked_positive_packet_fixture_count == 1
    and $scaffold.accepted_positive_packet_fixture_count == 0
    and $scaffold.positive_packet_authority_granted_count == 0
    and $scaffold.positive_packet_terminal_closure_granted_count == 0
    and $scaffold.positive_packet_activation_granted_count == 0
    and $scaffold.future_positive_packet_fixture.fixture_id == "future-complete-positive-trusted-record-packet-dry-run"
    and $scaffold.future_positive_packet_fixture.packet_shape_complete == true
    and $scaffold.future_positive_packet_fixture.packet_scoreboard_alignment_complete == true
    and $scaffold.future_positive_packet_fixture.packet_scoreboard_item_count == 56
    and $scaffold.future_positive_packet_fixture.packet_satisfied_scoreboard_item_count == 0
    and $scaffold.future_positive_packet_fixture.packet_trusted_record_count == 8
    and $scaffold.future_positive_packet_fixture.packet_future_positive_family_count == 7
    and $scaffold.future_positive_packet_fixture.packet_source_negative_fixture_count == 12
    and $scaffold.future_positive_packet_fixture.packet_source_blocked_negative_fixture_count == 12
    and $scaffold.future_positive_packet_fixture.status == "blocked"
    and $scaffold.future_positive_packet_fixture.acceptance_status == "blocked"
    and $scaffold.future_positive_packet_fixture.explicit_operator_approval_record_present == false
    and $scaffold.future_positive_packet_fixture.current_operator_identity_attestation_present == false
    and $scaffold.future_positive_packet_fixture.activation_request_record_present == false
    and $scaffold.future_positive_packet_fixture.fresh_live_evidence_acceptance_present == false
    and $scaffold.future_positive_packet_fixture.receipt_persistence_execution_present == false
    and $scaffold.future_positive_packet_fixture.receipt_acceptance_record_present == false
    and $scaffold.future_positive_packet_fixture.ledger_record_present == false
    and $scaffold.future_positive_packet_fixture.index_delivery_record_present == false
    and $scaffold.future_positive_packet_fixture.completion_ack_record_present == false
    and $scaffold.future_positive_packet_fixture.trusted_record_acceptance_allowed == false
    and $scaffold.future_positive_packet_fixture.trusted_record_accepted == false
    and $scaffold.future_positive_packet_fixture.terminal_closure_recorded == false
    and $scaffold.future_positive_packet_fixture.activation_allowed == false
    and ($scaffold.future_positive_packet_records | length) == 8
    and ($scaffold.future_positive_packet_records | all(
      .future_positive_packet_record_shape_declared == true
      and .future_positive_packet_record_shape_complete == true
      and .scoreboard_item_count == 7
      and .represented_scoreboard_item_count == 7
      and .satisfied_scoreboard_item_count == 0
      and .unsatisfied_scoreboard_item_count == 7
      and .status == "blocked"
      and .acceptance_status == "blocked"
      and .trusted_record_recorded == false
      and .trusted_record_persisted == false
      and .trusted_record_accepted == false
      and .trusted_record_delivered == false
      and .terminal_closure_recorded == false
      and .activation_allowed == false
    ))
    and ($scaffold.future_positive_packet_family_scaffold | length) == 7
    and ($scaffold.future_positive_packet_family_scaffold | all(.status == "blocked" and .current_authority_evidence_present == false and .trusted_record_acceptance_allowed == false))
    and ($scaffold.future_positive_packet_scoreboard_alignment_items | length) == 56
    and ($scaffold.future_positive_packet_scoreboard_alignment_items | all(
      .positive_packet_scoreboard_alignment_status == "represented_but_unsatisfied"
      and .positive_packet_shape_field_declared == true
      and .positive_packet_authority_material_present == false
      and .positive_packet_receipt_ledger_material_present == false
      and .trusted_record_accepted == false
      and .trusted_record_acceptance_allowed == false
      and .terminal_closure_recorded == false
      and .activation_allowed == false
    ))
    and ($scaffold.source_negative_fixtures | length) == 12
    and ($scaffold.source_negative_fixtures | all(.validation_status == "blocked" and .trusted_record_acceptance_allowed == false and .trusted_record_accepted == false))
    and $scaffold.operator_packet_recorded == false
    and $scaffold.operator_packet_persisted == false
    and $scaffold.operator_packet_accepted == false
    and $scaffold.trusted_record_acceptance_allowed == false
    and $scaffold.trusted_record_accepted == false
    and $scaffold.terminal_closure_recorded == false
    and $scaffold.activation_allowed == false
    and $scaffold.receipt_accepted == false
    and $scaffold.ledger_recorded == false
    and $scaffold.index_delivered == false
    and $scaffold.completion_ack_accepted == false
    and ($scaffold.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

fixture_specs=(
  "future-positive-packet-replayed-to-trusted-record-acceptance|trusted_record_acceptance|hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_gate|shape_complete_not_accepted|shape-complete positive packet cannot accept trusted records by shape alone|true|false|false|false|false|false|false|false"
  "future-positive-packet-replayed-to-terminal-closure|terminal_closure|hepta_core_activation_evidence_receipt_terminal_closure_decision_gate|terminal_closure_authority_missing|shape-complete positive packet cannot record terminal closure without accepted authority and evidence|false|true|false|false|false|false|false|false"
  "future-positive-packet-replayed-to-receipt-acceptance|receipt_acceptance|hepta_core_activation_evidence_receipt_acceptance_denial_gate|receipt_acceptance_authority_missing|shape-complete positive packet cannot accept receipt state without persistence execution and approval|false|false|true|false|false|false|false|false"
  "future-positive-packet-replayed-to-ledger-recording|ledger_recording|hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate|ledger_record_authority_missing|shape-complete positive packet cannot record ledger state without accepted receipt and evidence chain|false|false|false|true|false|false|false|false"
  "future-positive-packet-replayed-to-index-delivery|index_delivery|hepta_core_activation_evidence_receipt_acceptance_denial_gate|index_delivery_authority_missing|shape-complete positive packet cannot deliver index state without accepted ledger and completion binding|false|false|false|false|true|false|false|false"
  "future-positive-packet-replayed-to-completion-ack|completion_ack|hepta_core_activation_evidence_receipt_acceptance_denial_gate|completion_ack_authority_missing|shape-complete positive packet cannot accept completion acknowledgement without delivered accepted records|false|false|false|false|false|true|false|false"
  "future-positive-packet-replayed-to-activation|activation_execution|hepta_core_activation_readiness_summary_gate|activation_authority_missing|shape-complete positive packet cannot activate without terminal closure and accepted authority chain|false|false|false|false|false|false|true|false"
  "future-positive-packet-replayed-to-public-release|public_release|hepta_upstream_codex_latest_release_governance_non_activation_gate|public_claim_authority_missing|shape-complete positive packet cannot write release artifacts or public claims|false|false|false|false|false|false|false|true"
  "scoreboard-represented-but-unsatisfied-packet-to-terminal-closure|scoreboard_alignment|hepta_core_activation_evidence_receipt_terminal_closure_decision_gate|scoreboard_items_unsatisfied|represented 56-item scoreboard still has zero satisfied checks and cannot close terminal authority|true|true|false|false|false|false|false|false"
  "source-negative-fixture-replay-through-positive-packet|negative_fixture_replay|hepta_core_activation_evidence_receipt_acceptance_denial_gate|source_negative_fixtures_remain_blocked|blocked negative fixture evidence remains blocked even when wrapped by the positive packet scaffold|true|false|true|false|false|false|false|false"
  "delivery-chain-shape-without-completion-ack-replay|delivery_ack_chain|hepta_core_activation_evidence_receipt_terminal_closure_decision_gate|delivery_ack_chain_without_acceptance|delivery and acknowledgement shape cannot bypass accepted receipt ledger and terminal closure records|false|true|false|false|true|true|false|false"
  "receipt-ledger-delivery-chain-without-accepted-records-replay|receipt_ledger_delivery_chain|hepta_core_activation_evidence_receipt_terminal_closure_decision_gate|receipt_ledger_delivery_without_acceptance|receipt ledger delivery and ack chain cannot activate without accepted trusted records|false|false|true|true|true|true|true|false"
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
            authority_failure_id: .[3],
            authority_failure_reason: .[4],
            trusted_record_acceptance_replay_requested: (.[5] == "true"),
            terminal_closure_replay_requested: (.[6] == "true"),
            receipt_acceptance_replay_requested: (.[7] == "true"),
            ledger_recording_replay_requested: (.[8] == "true"),
            index_delivery_replay_requested: (.[9] == "true"),
            completion_ack_replay_requested: (.[10] == "true"),
            activation_replay_requested: (.[11] == "true"),
            public_claim_or_artifact_replay_requested: (.[12] == "true")
          })
      '
)"

entry_point_specs_json="$(
  jq -n '
    [
      {
        entry_point_id: "trusted-record-acceptance",
        target_gate: "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_gate",
        accepted_authority_required: true,
        replay_authority_allowed: false,
        trusted_record_accepted: false
      },
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
        entry_point_id: "ledger-recording",
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
        entry_point_id: "activation-execution",
        target_gate: "hepta_core_activation_readiness_summary_gate",
        accepted_authority_required: true,
        replay_authority_allowed: false,
        activation_allowed: false
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

denied_reasons_json="$(
  jq -n '
    [
      "trusted_record_positive_packet_replay_recording_denied",
      "trusted_record_positive_packet_replay_persistence_denied",
      "trusted_record_positive_packet_replay_acceptance_denied",
      "trusted_record_positive_packet_replay_delivery_denied",
      "positive_packet_shape_replay_authority_denied",
      "positive_packet_scoreboard_unsatisfied_denied",
      "positive_packet_operator_approval_missing",
      "positive_packet_activation_request_missing",
      "positive_packet_fresh_evidence_missing",
      "positive_packet_receipt_persistence_missing",
      "positive_packet_receipt_acceptance_missing",
      "positive_packet_ledger_record_missing",
      "positive_packet_index_delivery_missing",
      "positive_packet_completion_ack_missing",
      "source_negative_fixture_replay_denied",
      "trusted_record_acceptance_replay_denied",
      "terminal_closure_replay_denied",
      "receipt_acceptance_replay_denied",
      "ledger_recording_replay_denied",
      "index_delivery_replay_denied",
      "completion_ack_replay_denied",
      "activation_execution_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "install_restart_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "upstream_fetch_merge_denied",
      "credential_secret_read_denied"
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_gate" \
  --arg positive_scaffold_report_sha256 "$positive_scaffold_report_sha256" \
  --arg authority_replay_matrix_hash_sha256 "$authority_replay_matrix_hash_sha256" \
  --arg authority_replay_policy_hash_sha256 "$authority_replay_policy_hash_sha256" \
  --arg authority_replay_side_effect_hash_sha256 "$authority_replay_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson scaffold "$POSITIVE_SCAFFOLD_JSON" \
  --argjson fixture_specs "$fixture_specs_json" \
  --argjson entry_point_specs "$entry_point_specs_json" \
  --argjson denied_reasons "$denied_reasons_json" \
  '
    ($scaffold.future_positive_packet_fixture) as $source_fixture
    | ($fixture_specs | map(
        . as $spec
        | {
            fixture_id: $spec.fixture_id,
            replay_surface: $spec.replay_surface,
            replay_target_gate: $spec.replay_target_gate,
            source_positive_packet_fixture_id: $source_fixture.fixture_id,
            source_positive_packet_fixture_status: $source_fixture.status,
            source_positive_packet_acceptance_status: $source_fixture.acceptance_status,
            packet_shape_complete: $source_fixture.packet_shape_complete,
            packet_scoreboard_alignment_complete: $source_fixture.packet_scoreboard_alignment_complete,
            packet_trusted_record_count: $source_fixture.packet_trusted_record_count,
            packet_scoreboard_item_count: $source_fixture.packet_scoreboard_item_count,
            packet_satisfied_scoreboard_item_count: $source_fixture.packet_satisfied_scoreboard_item_count,
            packet_source_negative_fixture_count: $source_fixture.packet_source_negative_fixture_count,
            authority_failure_id: $spec.authority_failure_id,
            authority_failure_reason: $spec.authority_failure_reason,
            validation_status: "blocked",
            replay_attempted: true,
            dry_run_only: true,
            report_only: true,
            matrix_only: true,
            trusted_record_acceptance_replay_requested: $spec.trusted_record_acceptance_replay_requested,
            terminal_closure_replay_requested: $spec.terminal_closure_replay_requested,
            receipt_acceptance_replay_requested: $spec.receipt_acceptance_replay_requested,
            ledger_recording_replay_requested: $spec.ledger_recording_replay_requested,
            index_delivery_replay_requested: $spec.index_delivery_replay_requested,
            completion_ack_replay_requested: $spec.completion_ack_replay_requested,
            activation_replay_requested: $spec.activation_replay_requested,
            public_claim_or_artifact_replay_requested: $spec.public_claim_or_artifact_replay_requested,
            explicit_operator_approval_record_present: false,
            current_operator_identity_attestation_present: false,
            activation_request_record_present: false,
            fresh_live_evidence_acceptance_present: false,
            receipt_persistence_execution_present: false,
            receipt_acceptance_record_present: false,
            ledger_record_present: false,
            index_delivery_record_present: false,
            completion_ack_record_present: false,
            operator_packet_recorded: false,
            operator_packet_persisted: false,
            operator_packet_accepted: false,
            operator_packet_delivered: false,
            operator_packet_authorizes_activation: false,
            operator_packet_authorizes_terminal_closure: false,
            trusted_record_accepted: false,
            trusted_record_acceptance_allowed: false,
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
        terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_schema_version: "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_v1",
        terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_ready: true,
        trusted_record_positive_packet_authority_replay_denial_matrix_mode: "stdout_only_report_only_positive_packet_authority_replay_denial_matrix_no_approval_no_persistence_no_acceptance_no_delivery_no_activation",
        trusted_record_positive_packet_authority_replay_denial_matrix_status: "blocked",
        trusted_record_positive_packet_authority_replay_denial_matrix_decision: "all_future_positive_packet_authority_replay_fixtures_blocked_until_real_operator_approval_trusted_record_acceptance_receipt_ledger_delivery_ack_and_terminal_closure_records_exist",
        source_trusted_record_positive_packet_dry_run_scaffold_gate: $scaffold.gate,
        source_trusted_record_positive_packet_dry_run_scaffold_status: $scaffold.trusted_record_positive_packet_dry_run_scaffold_status,
        source_trusted_record_positive_packet_dry_run_scaffold_report_sha256: $positive_scaffold_report_sha256,
        source_trusted_record_acceptance_precondition_scoreboard_gate: $scaffold.source_trusted_record_acceptance_precondition_scoreboard_gate,
        source_trusted_record_acceptance_negative_fixture_matrix_gate: $scaffold.source_trusted_record_acceptance_negative_fixture_matrix_gate,
        source_trusted_record_acceptance_skeleton_gate: $scaffold.source_trusted_record_acceptance_skeleton_gate,
        source_operator_packet_authority_replay_matrix_gate: $scaffold.source_operator_packet_authority_replay_matrix_gate,
        source_operator_packet_dry_run_validator_gate: $scaffold.source_operator_packet_dry_run_validator_gate,
        source_operator_packet_template_gate: $scaffold.source_operator_packet_template_gate,
        source_gap_evidence_index_gate: $scaffold.source_gap_evidence_index_gate,
        source_terminal_closure_gate: $scaffold.source_terminal_closure_gate,
        source_terminal_closure_verdict: $scaffold.source_terminal_closure_verdict,
        positive_packet_authority_replay_matrix_hash_sha256: $authority_replay_matrix_hash_sha256,
        positive_packet_authority_replay_policy_hash_sha256: $authority_replay_policy_hash_sha256,
        positive_packet_authority_replay_side_effect_hash_sha256: $authority_replay_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        source_positive_packet_fixture_count: $scaffold.positive_packet_fixture_count,
        source_blocked_positive_packet_fixture_count: $scaffold.blocked_positive_packet_fixture_count,
        source_accepted_positive_packet_fixture_count: $scaffold.accepted_positive_packet_fixture_count,
        source_positive_packet_scoreboard_item_count: $source_fixture.packet_scoreboard_item_count,
        source_positive_packet_satisfied_scoreboard_item_count: $source_fixture.packet_satisfied_scoreboard_item_count,
        source_positive_packet_trusted_record_count: $source_fixture.packet_trusted_record_count,
        source_positive_packet_future_positive_family_count: $source_fixture.packet_future_positive_family_count,
        source_scoreboard_item_count: $scaffold.source_scoreboard_item_count,
        source_required_scoreboard_item_count: $scaffold.source_required_scoreboard_item_count,
        source_satisfied_scoreboard_item_count: $scaffold.source_satisfied_scoreboard_item_count,
        source_unsatisfied_scoreboard_item_count: $scaffold.source_unsatisfied_scoreboard_item_count,
        source_negative_fixture_count: $scaffold.source_negative_fixture_count,
        source_blocked_negative_fixture_count: $scaffold.source_blocked_negative_fixture_count,
        source_allowed_negative_fixture_count: $scaffold.source_allowed_negative_fixture_count,
        required_positive_packet_authority_replay_fixture_count: 12,
        positive_packet_authority_replay_fixture_count: ($fixtures | length),
        blocked_positive_packet_authority_replay_fixture_count: ($fixtures | map(select(.validation_status == "blocked")) | length),
        allowed_positive_packet_authority_replay_fixture_count: 0,
        trusted_record_acceptance_replay_fixture_count: ($fixtures | map(select(.trusted_record_acceptance_replay_requested == true)) | length),
        terminal_closure_replay_fixture_count: ($fixtures | map(select(.terminal_closure_replay_requested == true)) | length),
        receipt_acceptance_replay_fixture_count: ($fixtures | map(select(.receipt_acceptance_replay_requested == true)) | length),
        ledger_recording_replay_fixture_count: ($fixtures | map(select(.ledger_recording_replay_requested == true)) | length),
        index_delivery_replay_fixture_count: ($fixtures | map(select(.index_delivery_replay_requested == true)) | length),
        completion_ack_replay_fixture_count: ($fixtures | map(select(.completion_ack_replay_requested == true)) | length),
        activation_replay_fixture_count: ($fixtures | map(select(.activation_replay_requested == true)) | length),
        public_claim_or_artifact_replay_fixture_count: ($fixtures | map(select(.public_claim_or_artifact_replay_requested == true)) | length),
        replay_entry_point_count: ($entry_point_specs | length),
        replay_entry_points: $entry_point_specs,
        positive_packet_authority_replay_denial_matrix_fixtures: $fixtures,
        source_future_positive_packet_fixture: $source_fixture,
        denied_by_trusted_record_positive_packet_authority_replay_denial_matrix: $denied_reasons,
        denied_by_trusted_record_positive_packet_authority_replay_denial_matrix_count: ($denied_reasons | length),
        trusted_record_positive_packet_authority_replay_denial_matrix_executed: true,
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
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_gate"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_schema_version == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_v1"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_ready == true
  and .trusted_record_positive_packet_authority_replay_denial_matrix_mode == "stdout_only_report_only_positive_packet_authority_replay_denial_matrix_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
  and .trusted_record_positive_packet_authority_replay_denial_matrix_status == "blocked"
  and .source_trusted_record_positive_packet_dry_run_scaffold_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_dry_run_scaffold_gate"
  and .source_trusted_record_positive_packet_dry_run_scaffold_status == "blocked"
  and .source_trusted_record_acceptance_precondition_scoreboard_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_precondition_scoreboard_gate"
  and .source_trusted_record_acceptance_negative_fixture_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_gate"
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
  and .trusted_record_acceptance_replay_fixture_count == 3
  and .terminal_closure_replay_fixture_count == 3
  and .receipt_acceptance_replay_fixture_count == 3
  and .ledger_recording_replay_fixture_count == 2
  and .index_delivery_replay_fixture_count == 3
  and .completion_ack_replay_fixture_count == 3
  and .activation_replay_fixture_count == 2
  and .public_claim_or_artifact_replay_fixture_count == 1
  and .replay_entry_point_count == 8
  and (.replay_entry_points | all(.replay_authority_allowed == false and .accepted_authority_required == true))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | length) == 12
  and (.positive_packet_authority_replay_denial_matrix_fixtures | all(
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
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "future-positive-packet-replayed-to-trusted-record-acceptance" and .trusted_record_acceptance_replay_requested == true))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "future-positive-packet-replayed-to-terminal-closure" and .terminal_closure_replay_requested == true))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "future-positive-packet-replayed-to-receipt-acceptance" and .receipt_acceptance_replay_requested == true))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "future-positive-packet-replayed-to-ledger-recording" and .ledger_recording_replay_requested == true))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "future-positive-packet-replayed-to-index-delivery" and .index_delivery_replay_requested == true))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "future-positive-packet-replayed-to-completion-ack" and .completion_ack_replay_requested == true))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "future-positive-packet-replayed-to-activation" and .activation_replay_requested == true))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "future-positive-packet-replayed-to-public-release" and .public_claim_or_artifact_replay_requested == true))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "scoreboard-represented-but-unsatisfied-packet-to-terminal-closure" and .authority_failure_id == "scoreboard_items_unsatisfied"))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "source-negative-fixture-replay-through-positive-packet" and .authority_failure_id == "source_negative_fixtures_remain_blocked"))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "delivery-chain-shape-without-completion-ack-replay" and .completion_ack_replay_requested == true))
  and (.positive_packet_authority_replay_denial_matrix_fixtures | any(.fixture_id == "receipt-ledger-delivery-chain-without-accepted-records-replay" and .activation_replay_requested == true))
  and .denied_by_trusted_record_positive_packet_authority_replay_denial_matrix_count == 29
  and (.denied_by_trusted_record_positive_packet_authority_replay_denial_matrix | length) == 29
  and .trusted_record_positive_packet_authority_replay_denial_matrix_executed == true
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
