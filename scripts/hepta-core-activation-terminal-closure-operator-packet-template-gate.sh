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

GAP_INDEX_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-gap-evidence-index-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-gap-evidence-index-gate.sh
)"

gap_index_report_sha256="$(sha256_text "$GAP_INDEX_JSON")"
operator_packet_template_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-template:template:$gap_index_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
operator_packet_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-template:policy:$gap_index_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
operator_packet_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-template:side-effects:$gap_index_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson gap "$GAP_INDEX_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $gap.runtime == "hepta"
    and $gap.status == "ready"
    and $gap.gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
    and $gap.terminal_closure_gap_evidence_index_ready == true
    and $gap.terminal_closure_gap_evidence_index_status == "blocked"
    and $gap.source_terminal_closure_verdict == "blocked"
    and $gap.source_required_terminal_closure_missing_requirement_count == 12
    and $gap.source_remaining_terminal_closure_missing_requirement_count == 12
    and $gap.required_gap_evidence_count == 12
    and $gap.indexed_gap_evidence_count == 12
    and $gap.missing_gap_evidence_count == 12
    and $gap.ready_gap_evidence_count == 0
    and $gap.report_only_gap_evidence_count == 12
    and $gap.activation_blocking_gap_evidence_count == 12
    and $gap.terminal_closure_blocking_gap_evidence_count == 12
    and $gap.source_gate_count >= 6
    and $gap.doc_anchor_count >= 6
    and $gap.witness_hash_count == 12
    and ($gap.gap_evidence_index | length) == 12
    and ($gap.gap_evidence_index | all(
      .status == "missing"
      and .source_gate != ""
      and .source_gate_path != ""
      and .source_report_sha256 != ""
      and .doc_anchor != ""
      and .source_field_value == false
      and .activation_blocking == true
      and .terminal_closure_blocking == true
      and .report_only == true
      and .mutates_runtime == false
      and .persists_evidence == false
      and .records_approval == false
      and .records_receipt == false
      and .records_ledger == false
      and .delivers_index == false
      and .accepts_completion_ack == false
    ))
    and $gap.terminal_closure_recorded == false
    and $gap.terminal_closure_accepted == false
    and $gap.activation_allowed == false
    and $gap.operator_approval_recording_allowed == false
    and $gap.activation_request_recording_allowed == false
    and $gap.fresh_evidence_acceptance_allowed == false
    and $gap.filesystem_persistence_approval_allowed == false
    and $gap.receipt_persistence_execution_allowed == false
    and $gap.receipt_acceptance_allowed == false
    and $gap.ledger_recording_allowed == false
    and $gap.index_delivery_allowed == false
    and $gap.completion_ack_acceptance_allowed == false
    and $gap.public_release_claim_allowed == false
    and $gap.release_artifact_write_allowed == false
    and ($gap.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

section_specs=(
  "explicit_operator_approval_record_missing|operator-authority|operator_approval_id,operator_approval_attestation_hash|operator must provide an explicit approval record bound to the current activation request"
  "operator_identity_hash_missing|operator-identity|operator_identity_hash,operator_identity_binding_method|operator identity must be represented as a hash-only accountability binding"
  "activation_request_record_missing|activation-request|activation_request_id,activation_request_generation,activation_request_nonce,single_surface_activation_scope|activation request must be current monotonic single-use scope"
  "fresh_24_sample_long_soak_evidence_record_missing|fresh-long-soak-evidence|fresh_long_soak_evidence_id,long_soak_sample_set_hash|fresh 24-sample long-soak evidence must be recorded as trusted evidence"
  "fresh_trusted_evidence_record_set_missing|trusted-evidence-set|fresh_trusted_evidence_record_set_id,trusted_evidence_source_hash|trusted evidence records must be accepted as a set"
  "filesystem_persistence_approval_record_missing|filesystem-persistence-approval|filesystem_persistence_approval_id,output_path_allowlist_id|filesystem persistence needs explicit approval and allowlist binding"
  "receipt_persistence_command_enablement_missing|receipt-persistence-command|receipt_persistence_command_id,receipt_persistence_command_enablement_id|receipt persistence command must be explicitly enabled"
  "receipt_persistence_execution_record_missing|receipt-persistence-execution|receipt_persistence_execution_id,receipt_payload_hash|receipt persistence execution must exist before acceptance"
  "receipt_acceptance_record_missing|receipt-acceptance|receipt_acceptance_id,evidence_receipt_id|receipt acceptance must be recorded separately from persistence"
  "ledger_record_missing|ledger-record|ledger_record_id|ledger record must bind the accepted receipt and evidence chain"
  "index_delivery_records_missing|index-delivery|index_record_id,delivery_record_id|operator-facing index and delivery records must exist"
  "completion_ack_record_missing|completion-ack|completion_ack_id|completion acknowledgement must be recorded and accepted"
)

section_specs_json="$(
  printf '%s\n' "${section_specs[@]}" \
    | jq -R -s '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            requirement: .[0],
            packet_section_id: .[1],
            required_packet_fields: (.[2] | split(",")),
            operator_instruction: .[3]
          })
      '
)"

packet_generation_steps_json="$(
  jq -n '
    [
      {
        step_id: "inspect-gap-evidence-index",
        status: "template_only_not_executed",
        instruction: "review the 12 indexed terminal closure gaps and their witness hashes"
      },
      {
        step_id: "collect-current-operator-authority",
        status: "template_only_not_executed",
        instruction: "collect explicit operator approval, identity hash, activation request generation, and nonces"
      },
      {
        step_id: "bind-fresh-trusted-evidence",
        status: "template_only_not_executed",
        instruction: "bind fresh 24-sample long-soak evidence and trusted evidence records to the current request"
      },
      {
        step_id: "authorize-receipt-ledger-index-chain",
        status: "template_only_not_executed",
        instruction: "bind filesystem approval, receipt persistence, receipt acceptance, ledger, index, delivery, and acknowledgement records"
      },
      {
        step_id: "rerun-terminal-closure-after-real-records",
        status: "template_only_not_executed",
        instruction: "rerun terminal closure only after real records exist outside this template gate"
      }
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_template_gate" \
  --arg gap_index_report_sha256 "$gap_index_report_sha256" \
  --arg operator_packet_template_hash_sha256 "$operator_packet_template_hash_sha256" \
  --arg operator_packet_policy_hash_sha256 "$operator_packet_policy_hash_sha256" \
  --arg operator_packet_side_effect_hash_sha256 "$operator_packet_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson gap "$GAP_INDEX_JSON" \
  --argjson section_specs "$section_specs_json" \
  --argjson generation_steps "$packet_generation_steps_json" \
  '
    ($section_specs | map(
      . as $spec
      | ($gap.gap_evidence_index[] | select(.requirement == $spec.requirement)) as $gap_row
      | {
          requirement: $spec.requirement,
          packet_section_id: $spec.packet_section_id,
          status: "missing",
          source_gate: $gap_row.source_gate,
          source_gate_path: $gap_row.source_gate_path,
          source_report_sha256: $gap_row.source_report_sha256,
          source_field: $gap_row.source_field,
          source_field_value: $gap_row.source_field_value,
          doc_anchor: $gap_row.doc_anchor,
          denied_reason_id: $gap_row.denied_reason_id,
          denied_reason: $gap_row.denied_reason,
          gap_witness_hash_sha256: $gap_row.witness_hash_sha256,
          required_packet_fields: $spec.required_packet_fields,
          required_packet_field_count: ($spec.required_packet_fields | length),
          operator_instruction: $spec.operator_instruction,
          operator_input_required: true,
          recorded: false,
          persisted: false,
          accepted: false,
          delivered: false,
          template_only: true,
          report_only: true,
          activation_blocking: true,
          terminal_closure_blocking: true,
          mutates_runtime: false,
          persists_evidence: false,
          records_approval: false,
          records_receipt: false,
          records_ledger: false,
          delivers_index: false,
          accepts_completion_ack: false
        }
    )) as $sections
    | ($sections | map(.required_packet_fields[]) | unique) as $required_fields
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_closure_operator_packet_template_schema_version: "hepta_core_activation_terminal_closure_operator_packet_template_v1",
        terminal_closure_operator_packet_template_ready: true,
        operator_packet_template_mode: "stdout_only_report_only_template_no_approval_no_persistence_no_delivery_no_activation",
        operator_packet_template_status: "blocked",
        operator_packet_template_decision: "template_generated_from_gap_index_but_not_recorded_accepted_persisted_delivered_or_authorizing_activation",
        source_gap_evidence_index_gate: $gap.gate,
        source_gap_evidence_index_status: $gap.terminal_closure_gap_evidence_index_status,
        source_gap_evidence_index_report_sha256: $gap_index_report_sha256,
        source_terminal_closure_gate: $gap.source_terminal_closure_gate,
        source_terminal_closure_verdict: $gap.source_terminal_closure_verdict,
        source_terminal_closure_report_sha256: $gap.source_terminal_closure_report_sha256,
        operator_packet_template_hash_sha256: $operator_packet_template_hash_sha256,
        operator_packet_policy_hash_sha256: $operator_packet_policy_hash_sha256,
        operator_packet_side_effect_hash_sha256: $operator_packet_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        required_gap_evidence_count: $gap.required_gap_evidence_count,
        indexed_gap_evidence_count: $gap.indexed_gap_evidence_count,
        required_operator_packet_section_count: 12,
        operator_packet_template_section_count: ($sections | length),
        missing_operator_packet_section_count: ($sections | map(select(.status == "missing")) | length),
        ready_operator_packet_section_count: 0,
        operator_input_required_section_count: ($sections | map(select(.operator_input_required == true)) | length),
        report_only_section_count: ($sections | map(select(.report_only == true)) | length),
        activation_blocking_section_count: ($sections | map(select(.activation_blocking == true)) | length),
        terminal_closure_blocking_section_count: ($sections | map(select(.terminal_closure_blocking == true)) | length),
        required_operator_packet_field_count: ($sections | map(.required_packet_field_count) | add),
        unique_operator_packet_field_count: ($required_fields | length),
        recorded_operator_packet_field_count: 0,
        accepted_operator_packet_field_count: 0,
        delivered_operator_packet_field_count: 0,
        operator_packet_template_required_fields: $required_fields,
        operator_packet_template_sections: $sections,
        operator_packet_template_generation_steps: $generation_steps,
        operator_packet_template_generation_step_count: ($generation_steps | length),
        terminal_closure_missing_requirements: $gap.terminal_closure_missing_requirements,
        denied_by_terminal_closure_operator_packet_template: [
          "operator_packet_template_recording_denied",
          "operator_packet_template_persistence_denied",
          "operator_packet_template_acceptance_denied",
          "operator_packet_template_delivery_denied",
          "operator_packet_template_not_operator_approval",
          "operator_approval_recording_denied",
          "activation_request_recording_denied",
          "fresh_evidence_acceptance_denied",
          "filesystem_persistence_approval_denied",
          "receipt_persistence_execution_denied",
          "receipt_acceptance_denied",
          "ledger_recording_denied",
          "index_delivery_denied",
          "completion_ack_denied",
          "terminal_closure_recording_denied",
          "activation_execution_denied",
          "release_artifact_write_denied",
          "public_release_claim_denied"
        ],
        denied_by_terminal_closure_operator_packet_template_count: 18,
        operator_packet_template_rendered: true,
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
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_template_gate"
  and .terminal_closure_operator_packet_template_schema_version == "hepta_core_activation_terminal_closure_operator_packet_template_v1"
  and .terminal_closure_operator_packet_template_ready == true
  and .operator_packet_template_mode == "stdout_only_report_only_template_no_approval_no_persistence_no_delivery_no_activation"
  and .operator_packet_template_status == "blocked"
  and .source_gap_evidence_index_gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
  and .source_gap_evidence_index_status == "blocked"
  and .source_terminal_closure_verdict == "blocked"
  and .required_gap_evidence_count == 12
  and .indexed_gap_evidence_count == 12
  and .required_operator_packet_section_count == 12
  and .operator_packet_template_section_count == 12
  and .missing_operator_packet_section_count == 12
  and .ready_operator_packet_section_count == 0
  and .operator_input_required_section_count == 12
  and .report_only_section_count == 12
  and .activation_blocking_section_count == 12
  and .terminal_closure_blocking_section_count == 12
  and .required_operator_packet_field_count == 24
  and .unique_operator_packet_field_count == 24
  and .recorded_operator_packet_field_count == 0
  and .accepted_operator_packet_field_count == 0
  and .delivered_operator_packet_field_count == 0
  and ((.terminal_closure_missing_requirements | sort) == (.operator_packet_template_sections | map(.requirement) | sort))
  and (.operator_packet_template_sections | all(
    .status == "missing"
    and .source_gate != ""
    and .source_gate_path != ""
    and .source_report_sha256 != ""
    and .source_field != ""
    and .source_field_value == false
    and .doc_anchor != ""
    and .denied_reason_id != ""
    and .denied_reason != ""
    and (.gap_witness_hash_sha256 | test("^[0-9a-f]{64}$"))
    and (.required_packet_fields | length) == .required_packet_field_count
    and .operator_instruction != ""
    and .operator_input_required == true
    and .recorded == false
    and .persisted == false
    and .accepted == false
    and .delivered == false
    and .template_only == true
    and .report_only == true
    and .activation_blocking == true
    and .terminal_closure_blocking == true
    and .mutates_runtime == false
    and .persists_evidence == false
    and .records_approval == false
    and .records_receipt == false
    and .records_ledger == false
    and .delivers_index == false
    and .accepts_completion_ack == false
  ))
  and (.operator_packet_template_sections | any(.requirement == "explicit_operator_approval_record_missing" and .packet_section_id == "operator-authority"))
  and (.operator_packet_template_sections | any(.requirement == "activation_request_record_missing" and .packet_section_id == "activation-request"))
  and (.operator_packet_template_sections | any(.requirement == "fresh_24_sample_long_soak_evidence_record_missing" and .packet_section_id == "fresh-long-soak-evidence"))
  and (.operator_packet_template_sections | any(.requirement == "filesystem_persistence_approval_record_missing" and .packet_section_id == "filesystem-persistence-approval"))
  and (.operator_packet_template_sections | any(.requirement == "ledger_record_missing" and .packet_section_id == "ledger-record"))
  and (.operator_packet_template_sections | any(.requirement == "completion_ack_record_missing" and .packet_section_id == "completion-ack"))
  and .operator_packet_template_generation_step_count == 5
  and (.operator_packet_template_generation_steps | all(.status == "template_only_not_executed"))
  and .denied_by_terminal_closure_operator_packet_template_count == 18
  and (.denied_by_terminal_closure_operator_packet_template | length) == 18
  and .operator_packet_template_rendered == true
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
