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
    "hepta-core-activation-terminal-closure-operator-packet-authority-replay-matrix-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-authority-replay-matrix-gate.sh
)"

matrix_report_sha256="$(sha256_text "$MATRIX_JSON")"
acceptance_skeleton_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-skeleton:skeleton:$matrix_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
acceptance_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-skeleton:policy:$matrix_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
acceptance_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-skeleton:side-effects:$matrix_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson matrix "$MATRIX_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $matrix.runtime == "hepta"
    and $matrix.status == "ready"
    and $matrix.gate == "hepta_core_activation_terminal_closure_operator_packet_authority_replay_matrix_gate"
    and $matrix.terminal_closure_operator_packet_authority_replay_matrix_ready == true
    and $matrix.operator_packet_authority_replay_matrix_mode == "stdout_only_report_only_matrix_no_approval_no_persistence_no_delivery_no_activation"
    and $matrix.operator_packet_authority_replay_matrix_status == "blocked"
    and $matrix.source_operator_packet_dry_run_validator_gate == "hepta_core_activation_terminal_closure_operator_packet_dry_run_validator_gate"
    and $matrix.source_operator_packet_dry_run_validator_status == "blocked"
    and $matrix.source_operator_packet_template_gate == "hepta_core_activation_terminal_closure_operator_packet_template_gate"
    and $matrix.source_operator_packet_template_status == "blocked"
    and $matrix.source_gap_evidence_index_gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
    and $matrix.source_gap_evidence_index_status == "blocked"
    and $matrix.source_terminal_closure_verdict == "blocked"
    and $matrix.source_validator_fixture_count == 8
    and $matrix.source_blocked_validator_fixture_count == 8
    and $matrix.source_allowed_validator_fixture_count == 0
    and $matrix.source_operator_packet_template_section_count == 12
    and $matrix.source_unique_operator_packet_field_count == 24
    and $matrix.required_authority_replay_fixture_count == 10
    and $matrix.authority_replay_fixture_count == 10
    and $matrix.blocked_authority_replay_fixture_count == 10
    and $matrix.allowed_authority_replay_fixture_count == 0
    and $matrix.replay_entry_point_count == 6
    and ($matrix.replay_entry_points | all(.replay_authority_allowed == false and .accepted_authority_required == true))
    and ($matrix.authority_replay_matrix_fixtures | length) == 10
    and ($matrix.authority_replay_matrix_fixtures | all(
      .validation_status == "blocked"
      and .replay_attempted == true
      and .dry_run_only == true
      and .report_only == true
      and .matrix_only == true
      and .operator_packet_recorded == false
      and .operator_packet_persisted == false
      and .operator_packet_accepted == false
      and .operator_packet_delivered == false
      and .operator_packet_authorizes_activation == false
      and .operator_packet_authorizes_terminal_closure == false
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
    and $matrix.operator_packet_persisted == false
    and $matrix.operator_packet_accepted == false
    and $matrix.operator_packet_delivered == false
    and $matrix.operator_packet_authorizes_activation == false
    and $matrix.operator_packet_authorizes_terminal_closure == false
    and $matrix.terminal_closure_recorded == false
    and $matrix.activation_allowed == false
    and $matrix.receipt_accepted == false
    and $matrix.ledger_recorded == false
    and $matrix.index_delivered == false
    and $matrix.completion_ack_accepted == false
    and ($matrix.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

precondition_families_json="$(
  jq -n '
    [
      {
        family_id: "record-shape",
        required: true,
        satisfied: false,
        reason: "trusted record shape is declared but no trusted record is recorded"
      },
      {
        family_id: "operator-identity-binding",
        required: true,
        satisfied: false,
        reason: "operator identity hash and binding method must be verified before acceptance"
      },
      {
        family_id: "activation-request-nonce-binding",
        required: true,
        satisfied: false,
        reason: "activation request generation and single-use nonce must bind every record"
      },
      {
        family_id: "hash-binding",
        required: true,
        satisfied: false,
        reason: "record payload, evidence, receipt, ledger, and delivery hashes must bind together"
      },
      {
        family_id: "freshness-window",
        required: true,
        satisfied: false,
        reason: "fresh live evidence must fall inside an accepted freshness window"
      },
      {
        family_id: "receipt-ledger-precondition",
        required: true,
        satisfied: false,
        reason: "receipt persistence, acceptance, and ledger recording must already be authorized"
      },
      {
        family_id: "delivery-completion-ack-precondition",
        required: true,
        satisfied: false,
        reason: "index delivery and completion acknowledgement cannot precede accepted records"
      }
    ]
  '
)"

record_specs=(
  "operator-authority-trusted-record|operator-authority,operator-identity|operator_approval_id,operator_approval_attestation_hash,operator_identity_hash,operator_identity_binding_method|explicit_operator_approval_record_missing,operator_identity_hash_missing|operator authority and identity must be current, hash-bound, and accepted"
  "activation-request-trusted-record|activation-request|activation_request_id,activation_request_generation,activation_request_nonce,single_surface_activation_scope|activation_request_record_missing|activation request must be current, monotonic, single-use, and scope-bound"
  "fresh-long-soak-trusted-record|fresh-long-soak-evidence|fresh_long_soak_evidence_id,long_soak_sample_set_hash,freshness_window_started_at,freshness_window_expires_at|fresh_24_sample_long_soak_evidence_record_missing|fresh live long-soak evidence must be inside the accepted window"
  "trusted-evidence-set-record|trusted-evidence-set|fresh_trusted_evidence_record_set_id,trusted_evidence_source_hash,trusted_evidence_set_hash|fresh_trusted_evidence_record_set_missing|trusted evidence set hash and source hash must bind the accepted records"
  "filesystem-approval-trusted-record|filesystem-persistence-approval|filesystem_persistence_approval_id,output_path_allowlist_id,output_path_binding_hash|filesystem_persistence_approval_record_missing|filesystem persistence requires explicit approval and path binding"
  "receipt-persistence-trusted-record|receipt-persistence-command,receipt-persistence-execution|receipt_persistence_command_id,receipt_persistence_command_enablement_id,receipt_persistence_execution_id,receipt_payload_hash|receipt_persistence_command_enablement_missing,receipt_persistence_execution_record_missing|receipt persistence command and execution must be authorized before acceptance"
  "receipt-ledger-binding-trusted-record|receipt-acceptance,ledger-record|receipt_acceptance_id,evidence_receipt_id,ledger_record_id,receipt_ledger_binding_hash|receipt_acceptance_record_missing,ledger_record_missing|receipt acceptance and ledger record must bind the same evidence chain"
  "delivery-completion-trusted-record|index-delivery,completion-ack|index_record_id,delivery_record_id,completion_ack_id,completion_ack_binding_hash|index_delivery_records_missing,completion_ack_record_missing|index delivery and completion acknowledgement must bind accepted receipt and ledger records"
)

record_specs_json="$(
  printf '%s\n' "${record_specs[@]}" \
    | jq -R -s '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            skeleton_record_id: .[0],
            source_packet_section_ids: (.[1] | split(",")),
            required_record_fields: (.[2] | split(",")),
            source_terminal_closure_requirements: (.[3] | split(",")),
            acceptance_instruction: .[4]
          })
      '
)"

acceptance_sequence_json="$(
  jq -n '
    [
      {
        step_id: "record-shapes-declared",
        status: "skeleton_only_not_executed",
        instruction: "declare the eight trusted-record skeleton shapes without accepting them"
      },
      {
        step_id: "bind-current-operator-identity",
        status: "skeleton_only_not_executed",
        instruction: "bind operator approval and identity hash to the current activation request"
      },
      {
        step_id: "bind-current-request-nonce",
        status: "skeleton_only_not_executed",
        instruction: "bind all records to current activation request generation and single-use nonce"
      },
      {
        step_id: "bind-evidence-and-artifact-hashes",
        status: "skeleton_only_not_executed",
        instruction: "bind evidence set, long-soak sample set, receipt payload, ledger, and delivery hashes"
      },
      {
        step_id: "prove-freshness-window",
        status: "skeleton_only_not_executed",
        instruction: "prove fresh live evidence falls inside the accepted freshness window"
      },
      {
        step_id: "satisfy-receipt-ledger-preconditions",
        status: "skeleton_only_not_executed",
        instruction: "require receipt persistence enablement, execution, acceptance, and ledger recording first"
      },
      {
        step_id: "defer-delivery-and-completion-ack",
        status: "skeleton_only_not_executed",
        instruction: "defer index delivery and completion acknowledgement until accepted records exist"
      }
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_gate" \
  --arg matrix_report_sha256 "$matrix_report_sha256" \
  --arg acceptance_skeleton_hash_sha256 "$acceptance_skeleton_hash_sha256" \
  --arg acceptance_policy_hash_sha256 "$acceptance_policy_hash_sha256" \
  --arg acceptance_side_effect_hash_sha256 "$acceptance_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson matrix "$MATRIX_JSON" \
  --argjson precondition_families "$precondition_families_json" \
  --argjson record_specs "$record_specs_json" \
  --argjson acceptance_sequence "$acceptance_sequence_json" \
  '
    ($record_specs | map(
      . as $record
      | {
          skeleton_record_id: $record.skeleton_record_id,
          source_packet_section_ids: $record.source_packet_section_ids,
          source_terminal_closure_requirements: $record.source_terminal_closure_requirements,
          required_record_fields: $record.required_record_fields,
          acceptance_instruction: $record.acceptance_instruction,
          status: "missing",
          acceptance_status: "blocked",
          skeleton_only: true,
          report_only: true,
          operator_input_required: true,
          trusted_record_shape_declared: true,
          trusted_record_recorded: false,
          trusted_record_persisted: false,
          trusted_record_accepted: false,
          trusted_record_delivered: false,
          trusted_record_fresh: false,
          accepted_authority_required: true,
          precondition_families_required: ($precondition_families | map(.family_id)),
          required_precondition_count: ($precondition_families | length),
          satisfied_precondition_count: 0,
          record_shape_satisfied: false,
          operator_identity_binding_satisfied: false,
          activation_request_nonce_binding_satisfied: false,
          hash_binding_satisfied: false,
          freshness_window_satisfied: false,
          receipt_ledger_preconditions_satisfied: false,
          delivery_completion_ack_preconditions_satisfied: false,
          operator_packet_authorizes_activation: false,
          operator_packet_authorizes_terminal_closure: false,
          terminal_closure_recorded: false,
          receipt_accepted: false,
          ledger_recorded: false,
          index_delivered: false,
          completion_ack_accepted: false,
          activation_allowed: false
        }
    )) as $records
    | ($records | map(.required_record_fields[]) | unique) as $required_fields
    | ($records | map(.source_packet_section_ids[]) | unique) as $source_sections
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_closure_operator_packet_trusted_record_acceptance_skeleton_schema_version: "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_v1",
        terminal_closure_operator_packet_trusted_record_acceptance_skeleton_ready: true,
        operator_packet_trusted_record_acceptance_skeleton_mode: "stdout_only_report_only_skeleton_no_approval_no_persistence_no_acceptance_no_delivery_no_activation",
        operator_packet_trusted_record_acceptance_skeleton_status: "blocked",
        operator_packet_trusted_record_acceptance_skeleton_decision: "trusted_record_acceptance_shape_is_declared_but_no_operator_packet_or_trusted_record_is_recorded_accepted_persisted_or_authoritative",
        source_operator_packet_authority_replay_matrix_gate: $matrix.gate,
        source_operator_packet_authority_replay_matrix_status: $matrix.operator_packet_authority_replay_matrix_status,
        source_operator_packet_authority_replay_matrix_report_sha256: $matrix_report_sha256,
        source_operator_packet_dry_run_validator_gate: $matrix.source_operator_packet_dry_run_validator_gate,
        source_operator_packet_template_gate: $matrix.source_operator_packet_template_gate,
        source_gap_evidence_index_gate: $matrix.source_gap_evidence_index_gate,
        source_terminal_closure_gate: $matrix.source_terminal_closure_gate,
        source_terminal_closure_verdict: $matrix.source_terminal_closure_verdict,
        acceptance_skeleton_hash_sha256: $acceptance_skeleton_hash_sha256,
        acceptance_policy_hash_sha256: $acceptance_policy_hash_sha256,
        acceptance_side_effect_hash_sha256: $acceptance_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        source_authority_replay_fixture_count: $matrix.authority_replay_fixture_count,
        source_blocked_authority_replay_fixture_count: $matrix.blocked_authority_replay_fixture_count,
        source_allowed_authority_replay_fixture_count: $matrix.allowed_authority_replay_fixture_count,
        source_validator_fixture_count: $matrix.source_validator_fixture_count,
        source_operator_packet_template_section_count: $matrix.source_operator_packet_template_section_count,
        source_unique_operator_packet_field_count: $matrix.source_unique_operator_packet_field_count,
        source_replay_entry_point_count: $matrix.replay_entry_point_count,
        required_trusted_record_skeleton_count: 8,
        trusted_record_skeleton_count: ($records | length),
        blocked_trusted_record_skeleton_count: ($records | map(select(.acceptance_status == "blocked")) | length),
        accepted_trusted_record_count: 0,
        fresh_trusted_record_count: 0,
        persisted_trusted_record_count: 0,
        delivered_trusted_record_count: 0,
        source_packet_section_covered_count: ($source_sections | length),
        required_acceptance_record_field_count: ($required_fields | length),
        recorded_acceptance_record_field_count: 0,
        accepted_acceptance_record_field_count: 0,
        precondition_family_count: ($precondition_families | length),
        required_precondition_check_count: (($records | length) * ($precondition_families | length)),
        satisfied_precondition_check_count: 0,
        record_shape_satisfied_count: 0,
        operator_identity_binding_satisfied_count: 0,
        activation_request_nonce_binding_satisfied_count: 0,
        hash_binding_satisfied_count: 0,
        freshness_window_satisfied_count: 0,
        receipt_ledger_preconditions_satisfied_count: 0,
        delivery_completion_ack_preconditions_satisfied_count: 0,
        trusted_record_acceptance_precondition_families: $precondition_families,
        trusted_record_acceptance_sequence: $acceptance_sequence,
        required_acceptance_record_fields: $required_fields,
        covered_source_packet_sections: $source_sections,
        trusted_record_acceptance_skeleton_records: $records,
        denied_by_terminal_closure_operator_packet_trusted_record_acceptance_skeleton: [
          "trusted_record_skeleton_not_operator_approval",
          "trusted_record_skeleton_recording_denied",
          "trusted_record_skeleton_persistence_denied",
          "trusted_record_skeleton_acceptance_denied",
          "trusted_record_skeleton_delivery_denied",
          "operator_identity_binding_missing",
          "activation_request_nonce_binding_missing",
          "hash_binding_not_satisfied",
          "freshness_window_not_satisfied",
          "receipt_persistence_precondition_missing",
          "receipt_acceptance_precondition_missing",
          "ledger_record_precondition_missing",
          "index_delivery_precondition_missing",
          "completion_ack_precondition_missing",
          "terminal_closure_recording_denied",
          "activation_execution_denied",
          "public_release_claim_denied",
          "release_artifact_write_denied",
          "install_restart_denied",
          "provider_model_invocation_denied",
          "channel_delivery_denied",
          "upstream_fetch_merge_denied",
          "credential_secret_read_denied"
        ],
        denied_by_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_count: 23,
        trusted_record_acceptance_skeleton_executed: true,
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
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_gate"
  and .terminal_closure_operator_packet_trusted_record_acceptance_skeleton_schema_version == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_v1"
  and .terminal_closure_operator_packet_trusted_record_acceptance_skeleton_ready == true
  and .operator_packet_trusted_record_acceptance_skeleton_mode == "stdout_only_report_only_skeleton_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
  and .operator_packet_trusted_record_acceptance_skeleton_status == "blocked"
  and .source_operator_packet_authority_replay_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_authority_replay_matrix_gate"
  and .source_operator_packet_authority_replay_matrix_status == "blocked"
  and .source_operator_packet_dry_run_validator_gate == "hepta_core_activation_terminal_closure_operator_packet_dry_run_validator_gate"
  and .source_operator_packet_template_gate == "hepta_core_activation_terminal_closure_operator_packet_template_gate"
  and .source_gap_evidence_index_gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
  and .source_terminal_closure_verdict == "blocked"
  and .source_authority_replay_fixture_count == 10
  and .source_blocked_authority_replay_fixture_count == 10
  and .source_allowed_authority_replay_fixture_count == 0
  and .source_validator_fixture_count == 8
  and .source_operator_packet_template_section_count == 12
  and .source_unique_operator_packet_field_count == 24
  and .source_replay_entry_point_count == 6
  and .required_trusted_record_skeleton_count == 8
  and .trusted_record_skeleton_count == 8
  and .blocked_trusted_record_skeleton_count == 8
  and .accepted_trusted_record_count == 0
  and .fresh_trusted_record_count == 0
  and .persisted_trusted_record_count == 0
  and .delivered_trusted_record_count == 0
  and .source_packet_section_covered_count == 12
  and .required_acceptance_record_field_count == 30
  and .recorded_acceptance_record_field_count == 0
  and .accepted_acceptance_record_field_count == 0
  and .precondition_family_count == 7
  and .required_precondition_check_count == 56
  and .satisfied_precondition_check_count == 0
  and .record_shape_satisfied_count == 0
  and .operator_identity_binding_satisfied_count == 0
  and .activation_request_nonce_binding_satisfied_count == 0
  and .hash_binding_satisfied_count == 0
  and .freshness_window_satisfied_count == 0
  and .receipt_ledger_preconditions_satisfied_count == 0
  and .delivery_completion_ack_preconditions_satisfied_count == 0
  and (.trusted_record_acceptance_precondition_families | length) == 7
  and (.trusted_record_acceptance_precondition_families | all(.required == true and .satisfied == false))
  and (.trusted_record_acceptance_sequence | length) == 7
  and (.trusted_record_acceptance_sequence | all(.status == "skeleton_only_not_executed"))
  and (.trusted_record_acceptance_skeleton_records | length) == 8
  and (.trusted_record_acceptance_skeleton_records | all(
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
    and .record_shape_satisfied == false
    and .operator_identity_binding_satisfied == false
    and .activation_request_nonce_binding_satisfied == false
    and .hash_binding_satisfied == false
    and .freshness_window_satisfied == false
    and .receipt_ledger_preconditions_satisfied == false
    and .delivery_completion_ack_preconditions_satisfied == false
    and .operator_packet_authorizes_activation == false
    and .operator_packet_authorizes_terminal_closure == false
    and .terminal_closure_recorded == false
    and .receipt_accepted == false
    and .ledger_recorded == false
    and .index_delivered == false
    and .completion_ack_accepted == false
    and .activation_allowed == false
  ))
  and (.trusted_record_acceptance_skeleton_records | any(.skeleton_record_id == "operator-authority-trusted-record" and (.source_packet_section_ids | index("operator-authority") != null) and (.source_packet_section_ids | index("operator-identity") != null)))
  and (.trusted_record_acceptance_skeleton_records | any(.skeleton_record_id == "activation-request-trusted-record" and (.required_record_fields | index("activation_request_nonce") != null)))
  and (.trusted_record_acceptance_skeleton_records | any(.skeleton_record_id == "fresh-long-soak-trusted-record" and (.required_record_fields | index("freshness_window_expires_at") != null)))
  and (.trusted_record_acceptance_skeleton_records | any(.skeleton_record_id == "trusted-evidence-set-record" and (.required_record_fields | index("trusted_evidence_set_hash") != null)))
  and (.trusted_record_acceptance_skeleton_records | any(.skeleton_record_id == "filesystem-approval-trusted-record" and (.required_record_fields | index("output_path_binding_hash") != null)))
  and (.trusted_record_acceptance_skeleton_records | any(.skeleton_record_id == "receipt-persistence-trusted-record" and (.source_packet_section_ids | index("receipt-persistence-command") != null) and (.source_packet_section_ids | index("receipt-persistence-execution") != null)))
  and (.trusted_record_acceptance_skeleton_records | any(.skeleton_record_id == "receipt-ledger-binding-trusted-record" and (.required_record_fields | index("receipt_ledger_binding_hash") != null)))
  and (.trusted_record_acceptance_skeleton_records | any(.skeleton_record_id == "delivery-completion-trusted-record" and (.source_packet_section_ids | index("index-delivery") != null) and (.source_packet_section_ids | index("completion-ack") != null)))
  and .denied_by_terminal_closure_operator_packet_trusted_record_acceptance_skeleton_count == 23
  and (.denied_by_terminal_closure_operator_packet_trusted_record_acceptance_skeleton | length) == 23
  and .trusted_record_acceptance_skeleton_executed == true
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
