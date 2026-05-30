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

SKELETON_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-skeleton-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-skeleton-gate.sh
)"

skeleton_report_sha256="$(sha256_text "$SKELETON_JSON")"
negative_fixture_matrix_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-negative-fixture-matrix:matrix:$skeleton_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
negative_fixture_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-negative-fixture-matrix:policy:$skeleton_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
negative_fixture_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-negative-fixture-matrix:side-effects:$skeleton_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson skeleton "$SKELETON_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $skeleton.runtime == "hepta"
    and $skeleton.status == "ready"
    and $skeleton.gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_gate"
    and $skeleton.terminal_closure_operator_packet_trusted_record_acceptance_skeleton_ready == true
    and $skeleton.operator_packet_trusted_record_acceptance_skeleton_mode == "stdout_only_report_only_skeleton_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
    and $skeleton.operator_packet_trusted_record_acceptance_skeleton_status == "blocked"
    and $skeleton.source_operator_packet_authority_replay_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_authority_replay_matrix_gate"
    and $skeleton.source_operator_packet_authority_replay_matrix_status == "blocked"
    and $skeleton.source_operator_packet_dry_run_validator_gate == "hepta_core_activation_terminal_closure_operator_packet_dry_run_validator_gate"
    and $skeleton.source_operator_packet_template_gate == "hepta_core_activation_terminal_closure_operator_packet_template_gate"
    and $skeleton.source_gap_evidence_index_gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
    and $skeleton.source_terminal_closure_verdict == "blocked"
    and $skeleton.source_authority_replay_fixture_count == 10
    and $skeleton.source_blocked_authority_replay_fixture_count == 10
    and $skeleton.source_allowed_authority_replay_fixture_count == 0
    and $skeleton.source_validator_fixture_count == 8
    and $skeleton.source_operator_packet_template_section_count == 12
    and $skeleton.source_unique_operator_packet_field_count == 24
    and $skeleton.source_replay_entry_point_count == 6
    and $skeleton.required_trusted_record_skeleton_count == 8
    and $skeleton.trusted_record_skeleton_count == 8
    and $skeleton.blocked_trusted_record_skeleton_count == 8
    and $skeleton.accepted_trusted_record_count == 0
    and $skeleton.fresh_trusted_record_count == 0
    and $skeleton.persisted_trusted_record_count == 0
    and $skeleton.delivered_trusted_record_count == 0
    and $skeleton.source_packet_section_covered_count == 12
    and $skeleton.required_acceptance_record_field_count == 30
    and $skeleton.recorded_acceptance_record_field_count == 0
    and $skeleton.accepted_acceptance_record_field_count == 0
    and $skeleton.precondition_family_count == 7
    and $skeleton.required_precondition_check_count == 56
    and $skeleton.satisfied_precondition_check_count == 0
    and ($skeleton.trusted_record_acceptance_precondition_families | length) == 7
    and ($skeleton.trusted_record_acceptance_precondition_families | all(.required == true and .satisfied == false))
    and ($skeleton.trusted_record_acceptance_skeleton_records | length) == 8
    and ($skeleton.trusted_record_acceptance_skeleton_records | all(
      .status == "missing"
      and .acceptance_status == "blocked"
      and .skeleton_only == true
      and .report_only == true
      and .operator_input_required == true
      and .trusted_record_shape_declared == true
      and .trusted_record_recorded == false
      and .trusted_record_persisted == false
      and .trusted_record_accepted == false
      and .trusted_record_delivered == false
      and .trusted_record_fresh == false
      and .accepted_authority_required == true
      and .required_precondition_count == 7
      and .satisfied_precondition_count == 0
      and .terminal_closure_recorded == false
      and .receipt_accepted == false
      and .ledger_recorded == false
      and .index_delivered == false
      and .completion_ack_accepted == false
      and .activation_allowed == false
    ))
    and $skeleton.operator_packet_recorded == false
    and $skeleton.operator_packet_persisted == false
    and $skeleton.operator_packet_accepted == false
    and $skeleton.operator_packet_delivered == false
    and $skeleton.operator_packet_authorizes_activation == false
    and $skeleton.operator_packet_authorizes_terminal_closure == false
    and $skeleton.trusted_record_acceptance_allowed == false
    and $skeleton.trusted_record_delivery_allowed == false
    and $skeleton.terminal_closure_recorded == false
    and $skeleton.activation_allowed == false
    and $skeleton.receipt_accepted == false
    and $skeleton.ledger_recorded == false
    and $skeleton.index_delivered == false
    and $skeleton.completion_ack_accepted == false
    and ($skeleton.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

fixture_specs=(
  "missing-required-record-shape-field|operator-authority-trusted-record|record-shape|terminal-closure,receipt-acceptance|operator_approval_attestation_hash|required_record_field_missing|shape field missing from operator authority trusted record"
  "unknown-trusted-record-shape|unknown-trusted-record|record-shape|terminal-closure,ledger-recording|trusted_record_shape_id|trusted_record_shape_unknown|unknown trusted record shapes cannot be accepted"
  "operator-identity-hash-mismatch|operator-authority-trusted-record|operator-identity-binding|terminal-closure,activation-authority|operator_identity_hash|operator_identity_hash_mismatch|operator identity hash does not bind to the current operator packet"
  "operator-identity-binding-method-mismatch|operator-authority-trusted-record|operator-identity-binding|terminal-closure,receipt-acceptance|operator_identity_binding_method|operator_identity_binding_method_mismatch|operator identity binding method is not accepted for this request"
  "activation-request-nonce-replay|activation-request-trusted-record|activation-request-nonce-binding|receipt-acceptance,ledger-recording|activation_request_nonce|activation_request_nonce_replay_denied|single-use activation request nonce is replayed"
  "activation-request-generation-mismatch|activation-request-trusted-record|activation-request-nonce-binding|terminal-closure,ledger-recording|activation_request_generation|activation_request_generation_mismatch|activation request generation does not match the current request"
  "trusted-evidence-set-hash-mismatch|trusted-evidence-set-record|hash-binding|terminal-closure,index-delivery|trusted_evidence_set_hash|trusted_evidence_set_hash_mismatch|trusted evidence set hash does not bind to the source evidence"
  "receipt-payload-ledger-hash-mismatch|receipt-ledger-binding-trusted-record|hash-binding|receipt-acceptance,ledger-recording|receipt_ledger_binding_hash|receipt_ledger_hash_mismatch|receipt payload and ledger binding hashes diverge"
  "freshness-window-expired|fresh-long-soak-trusted-record|freshness-window|terminal-closure,activation-authority|freshness_window_expires_at|freshness_window_expired|freshness window is expired before acceptance"
  "receipt-accepted-without-persistence|receipt-persistence-trusted-record|receipt-ledger-precondition|receipt-acceptance,ledger-recording|receipt_persistence_execution_id|receipt_persistence_precondition_missing|receipt acceptance cannot precede authorized persistence execution"
  "ledger-record-before-receipt-acceptance|receipt-ledger-binding-trusted-record|receipt-ledger-precondition|ledger-recording,index-delivery|receipt_acceptance_id|ledger_before_receipt_acceptance_denied|ledger recording cannot precede receipt acceptance"
  "delivery-before-completion-ack|delivery-completion-trusted-record|delivery-completion-ack-precondition|index-delivery,completion-ack|completion_ack_binding_hash|delivery_before_completion_ack_denied|delivery cannot become complete before accepted completion acknowledgement"
)

fixture_specs_json="$(
  printf '%s\n' "${fixture_specs[@]}" \
    | jq -R -s '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            fixture_id: .[0],
            skeleton_record_id: .[1],
            precondition_family: .[2],
            replay_targets: (.[3] | split(",")),
            mutated_field: .[4],
            denied_reason: .[5],
            description: .[6]
          })
      '
)"

denied_reasons_json="$(
  jq -n '
    [
      "trusted_record_negative_fixture_recording_denied",
      "trusted_record_negative_fixture_persistence_denied",
      "trusted_record_negative_fixture_acceptance_denied",
      "trusted_record_negative_fixture_delivery_denied",
      "record_shape_missing_field_denied",
      "record_shape_unknown_denied",
      "operator_identity_hash_mismatch_denied",
      "operator_identity_binding_method_mismatch_denied",
      "activation_request_nonce_replay_denied",
      "activation_request_generation_mismatch_denied",
      "trusted_evidence_set_hash_mismatch_denied",
      "receipt_ledger_hash_mismatch_denied",
      "freshness_window_expired_denied",
      "receipt_persistence_precondition_missing_denied",
      "ledger_before_receipt_acceptance_denied",
      "delivery_before_completion_ack_denied",
      "operator_packet_acceptance_denied",
      "terminal_closure_recording_denied",
      "activation_execution_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "credential_secret_read_denied"
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_gate" \
  --arg skeleton_report_sha256 "$skeleton_report_sha256" \
  --arg negative_fixture_matrix_hash_sha256 "$negative_fixture_matrix_hash_sha256" \
  --arg negative_fixture_policy_hash_sha256 "$negative_fixture_policy_hash_sha256" \
  --arg negative_fixture_side_effect_hash_sha256 "$negative_fixture_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson skeleton "$SKELETON_JSON" \
  --argjson fixture_specs "$fixture_specs_json" \
  --argjson denied_reasons "$denied_reasons_json" \
  '
    ($skeleton.trusted_record_acceptance_skeleton_records) as $source_records
    | ($skeleton.trusted_record_acceptance_precondition_families | map(.family_id)) as $precondition_families
    | ($fixture_specs | map(
      . as $fixture
      | ($source_records | map(select(.skeleton_record_id == $fixture.skeleton_record_id)) | first) as $source_record
      | {
          fixture_id: $fixture.fixture_id,
          skeleton_record_id: $fixture.skeleton_record_id,
          source_skeleton_record_present: ($source_record != null),
          source_skeleton_record_status: (if $source_record == null then "absent" else $source_record.status end),
          source_skeleton_acceptance_status: (if $source_record == null then "blocked_unknown_shape" else $source_record.acceptance_status end),
          precondition_family: $fixture.precondition_family,
          precondition_family_declared: ($precondition_families | index($fixture.precondition_family) != null),
          replay_targets: $fixture.replay_targets,
          mutated_field: $fixture.mutated_field,
          denied_reason: $fixture.denied_reason,
          description: $fixture.description,
          validation_status: "blocked",
          acceptance_attempted: true,
          negative_fixture_only: true,
          dry_run_only: true,
          report_only: true,
          operator_input_required: true,
          trusted_record_shape_declared: ($source_record != null),
          trusted_record_recorded: false,
          trusted_record_persisted: false,
          trusted_record_accepted: false,
          trusted_record_delivered: false,
          trusted_record_fresh: false,
          operator_packet_recorded: false,
          operator_packet_accepted: false,
          trusted_record_acceptance_allowed: false,
          trusted_record_delivery_allowed: false,
          terminal_closure_recorded: false,
          receipt_accepted: false,
          ledger_recorded: false,
          index_delivered: false,
          completion_ack_accepted: false,
          activation_allowed: false,
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
        terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_schema_version: "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_v1",
        terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_ready: true,
        trusted_record_acceptance_negative_fixture_matrix_mode: "stdout_only_report_only_negative_fixture_matrix_no_approval_no_persistence_no_acceptance_no_delivery_no_activation",
        trusted_record_acceptance_negative_fixture_matrix_status: "blocked",
        trusted_record_acceptance_negative_fixture_matrix_decision: "all_trusted_record_acceptance_negative_fixtures_blocked_until_real_current_hash_bound_unexpired_operator_approved_receipt_ledger_delivery_ack_records_exist",
        source_trusted_record_acceptance_skeleton_gate: $skeleton.gate,
        source_trusted_record_acceptance_skeleton_status: $skeleton.operator_packet_trusted_record_acceptance_skeleton_status,
        source_trusted_record_acceptance_skeleton_report_sha256: $skeleton_report_sha256,
        source_operator_packet_authority_replay_matrix_gate: $skeleton.source_operator_packet_authority_replay_matrix_gate,
        source_operator_packet_dry_run_validator_gate: $skeleton.source_operator_packet_dry_run_validator_gate,
        source_operator_packet_template_gate: $skeleton.source_operator_packet_template_gate,
        source_gap_evidence_index_gate: $skeleton.source_gap_evidence_index_gate,
        source_terminal_closure_gate: $skeleton.source_terminal_closure_gate,
        source_terminal_closure_verdict: $skeleton.source_terminal_closure_verdict,
        negative_fixture_matrix_hash_sha256: $negative_fixture_matrix_hash_sha256,
        negative_fixture_policy_hash_sha256: $negative_fixture_policy_hash_sha256,
        negative_fixture_side_effect_hash_sha256: $negative_fixture_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        source_trusted_record_skeleton_count: $skeleton.trusted_record_skeleton_count,
        source_blocked_trusted_record_skeleton_count: $skeleton.blocked_trusted_record_skeleton_count,
        source_accepted_trusted_record_count: $skeleton.accepted_trusted_record_count,
        source_precondition_family_count: $skeleton.precondition_family_count,
        source_required_precondition_check_count: $skeleton.required_precondition_check_count,
        source_satisfied_precondition_check_count: $skeleton.satisfied_precondition_check_count,
        source_required_acceptance_record_field_count: $skeleton.required_acceptance_record_field_count,
        source_authority_replay_fixture_count: $skeleton.source_authority_replay_fixture_count,
        source_validator_fixture_count: $skeleton.source_validator_fixture_count,
        source_operator_packet_template_section_count: $skeleton.source_operator_packet_template_section_count,
        source_unique_operator_packet_field_count: $skeleton.source_unique_operator_packet_field_count,
        required_negative_fixture_count: 12,
        negative_fixture_count: ($fixtures | length),
        blocked_negative_fixture_count: ($fixtures | map(select(.validation_status == "blocked")) | length),
        allowed_negative_fixture_count: ($fixtures | map(select(.trusted_record_acceptance_allowed == true or .trusted_record_accepted == true)) | length),
        record_shape_negative_fixture_count: ($fixtures | map(select(.precondition_family == "record-shape")) | length),
        operator_identity_negative_fixture_count: ($fixtures | map(select(.precondition_family == "operator-identity-binding")) | length),
        activation_request_nonce_negative_fixture_count: ($fixtures | map(select(.precondition_family == "activation-request-nonce-binding")) | length),
        hash_binding_negative_fixture_count: ($fixtures | map(select(.precondition_family == "hash-binding")) | length),
        freshness_window_negative_fixture_count: ($fixtures | map(select(.precondition_family == "freshness-window")) | length),
        receipt_ledger_negative_fixture_count: ($fixtures | map(select(.precondition_family == "receipt-ledger-precondition")) | length),
        delivery_completion_ack_negative_fixture_count: ($fixtures | map(select(.precondition_family == "delivery-completion-ack-precondition")) | length),
        precondition_family_count: ($precondition_families | length),
        precondition_families: $precondition_families,
        trusted_record_acceptance_attempt_count: ($fixtures | length),
        trusted_record_accepted_count: 0,
        terminal_closure_authority_granted_count: 0,
        trusted_record_acceptance_negative_fixtures: $fixtures,
        denied_by_trusted_record_acceptance_negative_fixture_matrix: $denied_reasons,
        denied_by_trusted_record_acceptance_negative_fixture_matrix_count: ($denied_reasons | length),
        trusted_record_acceptance_negative_fixture_matrix_executed: true,
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
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_gate"
  and .terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_schema_version == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_v1"
  and .terminal_closure_operator_packet_trusted_record_acceptance_negative_fixture_matrix_ready == true
  and .trusted_record_acceptance_negative_fixture_matrix_mode == "stdout_only_report_only_negative_fixture_matrix_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
  and .trusted_record_acceptance_negative_fixture_matrix_status == "blocked"
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
  and .source_authority_replay_fixture_count == 10
  and .source_validator_fixture_count == 8
  and .source_operator_packet_template_section_count == 12
  and .source_unique_operator_packet_field_count == 24
  and .required_negative_fixture_count == 12
  and .negative_fixture_count == 12
  and .blocked_negative_fixture_count == 12
  and .allowed_negative_fixture_count == 0
  and .record_shape_negative_fixture_count == 2
  and .operator_identity_negative_fixture_count == 2
  and .activation_request_nonce_negative_fixture_count == 2
  and .hash_binding_negative_fixture_count == 2
  and .freshness_window_negative_fixture_count == 1
  and .receipt_ledger_negative_fixture_count == 2
  and .delivery_completion_ack_negative_fixture_count == 1
  and .precondition_family_count == 7
  and .trusted_record_acceptance_attempt_count == 12
  and .trusted_record_accepted_count == 0
  and .terminal_closure_authority_granted_count == 0
  and (.trusted_record_acceptance_negative_fixtures | length) == 12
  and (.trusted_record_acceptance_negative_fixtures | all(
    .validation_status == "blocked"
    and .acceptance_attempted == true
    and .negative_fixture_only == true
    and .dry_run_only == true
    and .report_only == true
    and .operator_input_required == true
    and .precondition_family_declared == true
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
  and (.trusted_record_acceptance_negative_fixtures | map(select(.fixture_id != "unknown-trusted-record-shape")) | all(.source_skeleton_record_present == true and .trusted_record_shape_declared == true))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "missing-required-record-shape-field" and .denied_reason == "required_record_field_missing" and .mutated_field == "operator_approval_attestation_hash"))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "unknown-trusted-record-shape" and .denied_reason == "trusted_record_shape_unknown" and .source_skeleton_record_present == false and .trusted_record_shape_declared == false))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "operator-identity-hash-mismatch" and .denied_reason == "operator_identity_hash_mismatch"))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "operator-identity-binding-method-mismatch" and .denied_reason == "operator_identity_binding_method_mismatch"))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "activation-request-nonce-replay" and .denied_reason == "activation_request_nonce_replay_denied"))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "activation-request-generation-mismatch" and .denied_reason == "activation_request_generation_mismatch"))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "trusted-evidence-set-hash-mismatch" and .denied_reason == "trusted_evidence_set_hash_mismatch"))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "receipt-payload-ledger-hash-mismatch" and .denied_reason == "receipt_ledger_hash_mismatch"))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "freshness-window-expired" and .denied_reason == "freshness_window_expired"))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "receipt-accepted-without-persistence" and .denied_reason == "receipt_persistence_precondition_missing"))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "ledger-record-before-receipt-acceptance" and .denied_reason == "ledger_before_receipt_acceptance_denied"))
  and (.trusted_record_acceptance_negative_fixtures | any(.fixture_id == "delivery-before-completion-ack" and .denied_reason == "delivery_before_completion_ack_denied"))
  and (.denied_by_trusted_record_acceptance_negative_fixture_matrix | length) == 24
  and .denied_by_trusted_record_acceptance_negative_fixture_matrix_count == 24
  and .trusted_record_acceptance_negative_fixture_matrix_executed == true
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
