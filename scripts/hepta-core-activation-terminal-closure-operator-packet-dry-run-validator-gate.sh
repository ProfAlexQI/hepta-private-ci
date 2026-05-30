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

TEMPLATE_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-template-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-template-gate.sh
)"

template_report_sha256="$(sha256_text "$TEMPLATE_JSON")"
validator_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-dry-run-validator:fixtures:$template_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
validator_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-dry-run-validator:policy:$template_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
validator_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-dry-run-validator:side-effects:$template_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson template "$TEMPLATE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $template.runtime == "hepta"
    and $template.status == "ready"
    and $template.gate == "hepta_core_activation_terminal_closure_operator_packet_template_gate"
    and $template.terminal_closure_operator_packet_template_ready == true
    and $template.operator_packet_template_mode == "stdout_only_report_only_template_no_approval_no_persistence_no_delivery_no_activation"
    and $template.operator_packet_template_status == "blocked"
    and $template.source_gap_evidence_index_gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
    and $template.source_gap_evidence_index_status == "blocked"
    and $template.source_terminal_closure_verdict == "blocked"
    and $template.required_gap_evidence_count == 12
    and $template.indexed_gap_evidence_count == 12
    and $template.required_operator_packet_section_count == 12
    and $template.operator_packet_template_section_count == 12
    and $template.missing_operator_packet_section_count == 12
    and $template.ready_operator_packet_section_count == 0
    and $template.operator_input_required_section_count == 12
    and $template.report_only_section_count == 12
    and $template.activation_blocking_section_count == 12
    and $template.terminal_closure_blocking_section_count == 12
    and $template.required_operator_packet_field_count == 24
    and $template.unique_operator_packet_field_count == 24
    and $template.recorded_operator_packet_field_count == 0
    and $template.accepted_operator_packet_field_count == 0
    and $template.delivered_operator_packet_field_count == 0
    and ($template.operator_packet_template_sections | length) == 12
    and ($template.operator_packet_template_required_fields | length) == 24
    and ($template.operator_packet_template_sections | all(
      .status == "missing"
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
    and $template.operator_packet_recorded == false
    and $template.operator_packet_persisted == false
    and $template.operator_packet_accepted == false
    and $template.operator_packet_delivered == false
    and $template.operator_packet_authorizes_activation == false
    and $template.operator_packet_authorizes_terminal_closure == false
    and $template.operator_packet_authorizes_receipt_persistence == false
    and $template.operator_packet_authorizes_ledger_recording == false
    and $template.terminal_closure_allowed == false
    and $template.terminal_closure_recorded == false
    and $template.terminal_closure_accepted == false
    and $template.activation_allowed == false
    and $template.activation_performed == false
    and $template.public_release_claim_allowed == false
    and $template.release_artifact_write_allowed == false
    and $template.provider_model_invocation_allowed == false
    and $template.channel_delivery_allowed == false
    and $template.install_restart_allowed == false
    and $template.active_binary_mutation_allowed == false
    and $template.upstream_fetch_merge_allowed == false
    and $template.credential_read_allowed == false
    and $template.secret_value_read_allowed == false
    and ($template.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

fixture_specs=(
  "template-only-no-packet-record|0|24|false|false|false|false|template_rendering_is_not_operator_approval|template output exists but no operator packet record exists"
  "missing-required-operator-authority-fields|20|4|false|false|false|false|operator_authority_fields_missing|operator approval and identity hash authority fields are missing"
  "missing-activation-request-generation-or-nonce|21|3|false|false|false|false|activation_request_nonce_missing|activation request generation, nonce, or bounded scope is missing"
  "stale-or-expired-long-soak-evidence|24|0|true|false|false|false|fresh_evidence_expired|syntactically complete packet shape carries stale or expired long-soak evidence"
  "cross-request-evidence-or-approval-reuse|24|0|false|true|false|false|cross_request_reuse_denied|approval or evidence belongs to a different activation request"
  "receipt-ledger-ack-without-acceptance|22|2|false|false|false|false|receipt_ledger_ack_without_acceptance|downstream receipt, ledger, or acknowledgement shape cannot replace accepted records"
  "public-claim-or-artifact-attempt|24|0|false|false|true|false|public_claim_denied|public claim or release artifact request is denied without an accepted operator packet"
  "complete-shape-without-recording-authority|24|0|false|false|false|true|complete_shape_not_recorded_authority|all required fields are present but the packet is not recorded, accepted, persisted, or delivered"
)

validator_fixture_lines=()
for fixture_spec in "${fixture_specs[@]}"; do
  IFS='|' read -r fixture_id recorded_field_count missing_field_count stale_evidence cross_request public_claim complete_shape blocked_reason_id blocked_reason <<<"$fixture_spec"
  fixture_witness_hash_sha256="$(
    sha256_text "hepta-core-activation-terminal-closure-operator-packet-dry-run-validator:$fixture_id:$recorded_field_count:$missing_field_count:$blocked_reason_id:$template_report_sha256"
  )"
  validator_fixture_lines+=("$fixture_spec|$fixture_witness_hash_sha256")
done

validator_fixtures_json="$(
  printf '%s\n' "${validator_fixture_lines[@]}" \
    | jq -R -s '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            fixture_id: .[0],
            validation_status: "blocked",
            recorded_operator_packet_field_count: (.[1] | tonumber),
            missing_operator_packet_field_count: (.[2] | tonumber),
            stale_or_expired_evidence: (.[3] == "true"),
            cross_request_reuse: (.[4] == "true"),
            public_claim_or_artifact_requested: (.[5] == "true"),
            complete_shape_without_recording_authority: (.[6] == "true"),
            blocked_reason_id: .[7],
            blocked_reason: .[8],
            fixture_witness_hash_sha256: .[9],
            dry_run_only: true,
            report_only: true,
            validator_only: true,
            operator_input_required: true,
            operator_packet_recorded: false,
            operator_packet_persisted: false,
            operator_packet_accepted: false,
            operator_packet_delivered: false,
            operator_packet_authorizes_activation: false,
            operator_packet_authorizes_terminal_closure: false,
            terminal_closure_allowed: false,
            terminal_closure_recorded: false,
            terminal_closure_accepted: false,
            activation_allowed: false,
            activation_performed: false,
            receipt_persisted: false,
            receipt_accepted: false,
            ledger_recorded: false,
            index_delivered: false,
            completion_ack_accepted: false,
            public_release_claim_allowed: false,
            release_artifact_write_allowed: false
          })
      '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_dry_run_validator_gate" \
  --arg template_report_sha256 "$template_report_sha256" \
  --arg validator_hash_sha256 "$validator_hash_sha256" \
  --arg validator_policy_hash_sha256 "$validator_policy_hash_sha256" \
  --arg validator_side_effect_hash_sha256 "$validator_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson template "$TEMPLATE_JSON" \
  --argjson fixtures "$validator_fixtures_json" \
  '
    {
      product: $product,
      runtime: $runtime,
      status: "ready",
      base_url: $base_url,
      gate: $gate,
      terminal_closure_operator_packet_dry_run_validator_schema_version: "hepta_core_activation_terminal_closure_operator_packet_dry_run_validator_v1",
      terminal_closure_operator_packet_dry_run_validator_ready: true,
      operator_packet_dry_run_validator_mode: "stdout_only_report_only_validator_no_approval_no_persistence_no_delivery_no_activation",
      operator_packet_dry_run_validator_status: "blocked",
      operator_packet_dry_run_validator_decision: "all_future_packet_fixtures_blocked_until_a_real_operator_packet_is_explicitly_recorded_accepted_persisted_delivered_and_bound_to_current_request",
      source_operator_packet_template_gate: $template.gate,
      source_operator_packet_template_status: $template.operator_packet_template_status,
      source_operator_packet_template_report_sha256: $template_report_sha256,
      source_gap_evidence_index_gate: $template.source_gap_evidence_index_gate,
      source_gap_evidence_index_status: $template.source_gap_evidence_index_status,
      source_gap_evidence_index_report_sha256: $template.source_gap_evidence_index_report_sha256,
      source_terminal_closure_gate: $template.source_terminal_closure_gate,
      source_terminal_closure_verdict: $template.source_terminal_closure_verdict,
      source_terminal_closure_report_sha256: $template.source_terminal_closure_report_sha256,
      source_operator_packet_template_hash_sha256: $template.operator_packet_template_hash_sha256,
      validator_hash_sha256: $validator_hash_sha256,
      validator_policy_hash_sha256: $validator_policy_hash_sha256,
      validator_side_effect_hash_sha256: $validator_side_effect_hash_sha256,
      minimum_required_long_soak_samples: $min_long_soak_samples,
      source_required_gap_evidence_count: $template.required_gap_evidence_count,
      source_indexed_gap_evidence_count: $template.indexed_gap_evidence_count,
      source_operator_packet_template_section_count: $template.operator_packet_template_section_count,
      source_operator_input_required_section_count: $template.operator_input_required_section_count,
      source_required_operator_packet_field_count: $template.required_operator_packet_field_count,
      source_unique_operator_packet_field_count: $template.unique_operator_packet_field_count,
      required_validator_fixture_count: 8,
      validator_fixture_count: ($fixtures | length),
      blocked_validator_fixture_count: ($fixtures | map(select(.validation_status == "blocked")) | length),
      allowed_validator_fixture_count: 0,
      missing_required_field_fixture_count: ($fixtures | map(select(.missing_operator_packet_field_count > 0)) | length),
      stale_or_expired_fixture_count: ($fixtures | map(select(.stale_or_expired_evidence == true)) | length),
      cross_request_fixture_count: ($fixtures | map(select(.cross_request_reuse == true)) | length),
      public_claim_or_artifact_fixture_count: ($fixtures | map(select(.public_claim_or_artifact_requested == true)) | length),
      complete_shape_fixture_count: ($fixtures | map(select(.recorded_operator_packet_field_count == 24 and .missing_operator_packet_field_count == 0)) | length),
      complete_shape_without_recording_authority_fixture_count: ($fixtures | map(select(.complete_shape_without_recording_authority == true)) | length),
      future_packet_authority_denied_count: ($fixtures | map(select(.operator_packet_authorizes_activation == false and .operator_packet_authorizes_terminal_closure == false)) | length),
      operator_packet_required_fields: $template.operator_packet_template_required_fields,
      operator_packet_template_sections: $template.operator_packet_template_sections,
      validator_fixtures: $fixtures,
      denied_by_terminal_closure_operator_packet_dry_run_validator: [
        "operator_packet_template_not_operator_approval",
        "operator_packet_future_shape_recording_denied",
        "operator_packet_future_shape_persistence_denied",
        "operator_packet_future_shape_acceptance_denied",
        "operator_packet_future_shape_delivery_denied",
        "operator_packet_complete_shape_authority_denied",
        "cross_request_approval_reuse_denied",
        "stale_long_soak_evidence_reuse_denied",
        "receipt_ledger_ack_without_acceptance_denied",
        "public_claim_without_accepted_packet_denied",
        "release_artifact_write_without_accepted_packet_denied",
        "terminal_closure_recording_denied",
        "activation_execution_denied",
        "install_restart_denied",
        "provider_model_invocation_denied",
        "channel_delivery_denied",
        "upstream_fetch_merge_denied",
        "credential_secret_read_denied"
      ],
      denied_by_terminal_closure_operator_packet_dry_run_validator_count: 18,
      operator_packet_template_rendered: true,
      operator_packet_dry_run_validator_executed: true,
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
      ledger_recording_allowed: false,
      index_delivery_allowed: false,
      completion_ack_acceptance_allowed: false,
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
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_dry_run_validator_gate"
  and .terminal_closure_operator_packet_dry_run_validator_schema_version == "hepta_core_activation_terminal_closure_operator_packet_dry_run_validator_v1"
  and .terminal_closure_operator_packet_dry_run_validator_ready == true
  and .operator_packet_dry_run_validator_mode == "stdout_only_report_only_validator_no_approval_no_persistence_no_delivery_no_activation"
  and .operator_packet_dry_run_validator_status == "blocked"
  and .source_operator_packet_template_gate == "hepta_core_activation_terminal_closure_operator_packet_template_gate"
  and .source_operator_packet_template_status == "blocked"
  and .source_gap_evidence_index_gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
  and .source_gap_evidence_index_status == "blocked"
  and .source_terminal_closure_verdict == "blocked"
  and .source_required_gap_evidence_count == 12
  and .source_indexed_gap_evidence_count == 12
  and .source_operator_packet_template_section_count == 12
  and .source_operator_input_required_section_count == 12
  and .source_required_operator_packet_field_count == 24
  and .source_unique_operator_packet_field_count == 24
  and .required_validator_fixture_count == 8
  and .validator_fixture_count == 8
  and .blocked_validator_fixture_count == 8
  and .allowed_validator_fixture_count == 0
  and .missing_required_field_fixture_count == 4
  and .stale_or_expired_fixture_count == 1
  and .cross_request_fixture_count == 1
  and .public_claim_or_artifact_fixture_count == 1
  and .complete_shape_fixture_count == 4
  and .complete_shape_without_recording_authority_fixture_count == 1
  and .future_packet_authority_denied_count == 8
  and (.operator_packet_required_fields | length) == 24
  and (.operator_packet_template_sections | length) == 12
  and (.validator_fixtures | all(
    .validation_status == "blocked"
    and .dry_run_only == true
    and .report_only == true
    and .validator_only == true
    and .operator_input_required == true
    and (.fixture_witness_hash_sha256 | test("^[0-9a-f]{64}$"))
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
    and .activation_performed == false
    and .receipt_persisted == false
    and .receipt_accepted == false
    and .ledger_recorded == false
    and .index_delivered == false
    and .completion_ack_accepted == false
    and .public_release_claim_allowed == false
    and .release_artifact_write_allowed == false
  ))
  and (.validator_fixtures | any(.fixture_id == "template-only-no-packet-record" and .recorded_operator_packet_field_count == 0 and .missing_operator_packet_field_count == 24))
  and (.validator_fixtures | any(.fixture_id == "missing-required-operator-authority-fields" and .missing_operator_packet_field_count == 4))
  and (.validator_fixtures | any(.fixture_id == "missing-activation-request-generation-or-nonce" and .missing_operator_packet_field_count == 3))
  and (.validator_fixtures | any(.fixture_id == "stale-or-expired-long-soak-evidence" and .stale_or_expired_evidence == true))
  and (.validator_fixtures | any(.fixture_id == "cross-request-evidence-or-approval-reuse" and .cross_request_reuse == true))
  and (.validator_fixtures | any(.fixture_id == "receipt-ledger-ack-without-acceptance" and .missing_operator_packet_field_count == 2))
  and (.validator_fixtures | any(.fixture_id == "public-claim-or-artifact-attempt" and .public_claim_or_artifact_requested == true))
  and (.validator_fixtures | any(.fixture_id == "complete-shape-without-recording-authority" and .complete_shape_without_recording_authority == true))
  and .denied_by_terminal_closure_operator_packet_dry_run_validator_count == 18
  and (.denied_by_terminal_closure_operator_packet_dry_run_validator | length) == 18
  and .operator_packet_template_rendered == true
  and .operator_packet_dry_run_validator_executed == true
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
  and .ledger_recording_allowed == false
  and .index_delivery_allowed == false
  and .completion_ack_acceptance_allowed == false
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
