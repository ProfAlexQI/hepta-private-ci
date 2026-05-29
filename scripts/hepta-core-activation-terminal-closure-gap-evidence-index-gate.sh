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

TERMINAL_CLOSURE_JSON="$(
  capture_json_report \
    "hepta-core-activation-evidence-receipt-terminal-closure-decision-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-evidence-receipt-terminal-closure-decision-gate.sh
)"

terminal_closure_report_sha256="$(sha256_text "$TERMINAL_CLOSURE_JSON")"
gap_index_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-gap-evidence-index:index:$terminal_closure_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
gap_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-gap-evidence-index:policy:$terminal_closure_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
gap_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-gap-evidence-index:side-effects:$terminal_closure_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson closure "$TERMINAL_CLOSURE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $closure.runtime == "hepta"
    and $closure.status == "ready"
    and $closure.gate == "hepta_core_activation_evidence_receipt_terminal_closure_decision_gate"
    and $closure.terminal_closure_decision_gate_ready == true
    and $closure.terminal_closure_verdict == "blocked"
    and $closure.required_terminal_closure_missing_requirement_count == 12
    and $closure.remaining_terminal_closure_missing_requirement_count == 12
    and ($closure.terminal_closure_missing_requirements | length) == 12
    and $closure.required_terminal_closure_fixture_count == 6
    and $closure.blocked_terminal_closure_fixture_count == 6
    and $closure.allowed_terminal_closure_fixture_count == 0
    and $closure.operator_approval_recorded == false
    and $closure.operator_identity_hash_recorded == false
    and $closure.activation_request_recorded == false
    and $closure.long_soak_evidence_recorded == false
    and $closure.long_soak_evidence_fresh == false
    and $closure.fresh_trusted_evidence_records_accepted == false
    and $closure.filesystem_persistence_approval_recorded == false
    and $closure.receipt_persistence_command_enabled_by_default == false
    and $closure.receipt_persistence_command_invoked == false
    and $closure.receipt_persistence_execution_performed == false
    and $closure.receipt_acceptance_recorded == false
    and $closure.ledger_recorded == false
    and $closure.index_recorded == false
    and $closure.delivery_recorded == false
    and $closure.completion_ack_recorded == false
    and $closure.completion_ack_accepted == false
    and $closure.terminal_closure_allowed == false
    and $closure.terminal_closure_recorded == false
    and $closure.terminal_closure_accepted == false
    and $closure.activation_allowed == false
    and $closure.live_mutation_execution_allowed == false
    and $closure.public_release_claim_allowed == false
    and $closure.release_artifact_write_allowed == false
    and ($closure.denied_by_terminal_closure_decision_gate | length) == 14
    and ($closure.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

gap_specs=(
  "explicit_operator_approval_record_missing|hepta_core_activation_long_soak_operator_approval_packet_gate|scripts/hepta-core-activation-long-soak-operator-approval-packet-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_LONG_SOAK_OPERATOR_APPROVAL_PACKET_GATE.md#required-missing-records|operator_approval_recorded|operator_approval_not_recorded|explicit operator approval is absent, so terminal closure cannot record operator authority"
  "operator_identity_hash_missing|hepta_core_activation_long_soak_operator_approval_packet_gate|scripts/hepta-core-activation-long-soak-operator-approval-packet-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_LONG_SOAK_OPERATOR_APPROVAL_PACKET_GATE.md#required-missing-records|operator_identity_hash_recorded|operator_identity_hash_not_recorded|operator identity hash is absent, so approval cannot be bound to an accountable operator"
  "activation_request_record_missing|hepta_core_activation_request_monotonic_single_use_approval_nonce_denial_gate|scripts/hepta-core-activation-request-monotonic-single-use-approval-nonce-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_REQUEST_MONOTONIC_SINGLE_USE_APPROVAL_NONCE_DENIAL_GATE.md#blocked-transition-surface|activation_request_recorded|activation_request_not_recorded|activation request record is absent, so terminal closure has no current generation to close"
  "fresh_24_sample_long_soak_evidence_record_missing|hepta_core_activation_long_soak_observation_freshness_denial_gate|scripts/hepta-core-activation-long-soak-observation-freshness-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_LONG_SOAK_OBSERVATION_FRESHNESS_DENIAL_GATE.md#freshness-boundary|long_soak_evidence_recorded|fresh_24_sample_long_soak_evidence_not_recorded|fresh trusted 24-sample long-soak evidence is not recorded or accepted"
  "fresh_trusted_evidence_record_set_missing|hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate|scripts/hepta-core-activation-fresh-long-soak-evidence-ledger-receipt-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_FRESH_LONG_SOAK_EVIDENCE_LEDGER_RECEIPT_GATE.md#required-missing-records|fresh_trusted_evidence_records_accepted|trusted_evidence_records_not_accepted|trusted evidence record set is absent, so the long-soak observation cannot satisfy activation evidence"
  "filesystem_persistence_approval_record_missing|hepta_core_activation_evidence_receipt_filesystem_persistence_denial_gate|scripts/hepta-core-activation-evidence-receipt-filesystem-persistence-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_PERSISTENCE_DENIAL_GATE.md#persistence-boundary|filesystem_persistence_approval_recorded|filesystem_persistence_approval_not_recorded|filesystem persistence approval is absent, so receipt materialization remains denied"
  "receipt_persistence_command_enablement_missing|hepta_core_activation_evidence_receipt_acceptance_denial_gate|scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_ACCEPTANCE_DENIAL_GATE.md#receipt-acceptance-boundary|receipt_persistence_command_enabled_by_default|receipt_persistence_command_enablement_not_recorded|receipt persistence command enablement is absent and disabled by default"
  "receipt_persistence_execution_record_missing|hepta_core_activation_evidence_receipt_acceptance_denial_gate|scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_ACCEPTANCE_DENIAL_GATE.md#receipt-acceptance-boundary|receipt_persistence_execution_performed|receipt_persistence_execution_not_recorded|receipt persistence execution did not run and no execution record exists"
  "receipt_acceptance_record_missing|hepta_core_activation_evidence_receipt_acceptance_denial_gate|scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_ACCEPTANCE_DENIAL_GATE.md#receipt-acceptance-boundary|receipt_acceptance_recorded|receipt_acceptance_not_recorded|receipt acceptance is absent, so no persisted evidence receipt can close activation"
  "ledger_record_missing|hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate|scripts/hepta-core-activation-fresh-long-soak-evidence-ledger-receipt-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_FRESH_LONG_SOAK_EVIDENCE_LEDGER_RECEIPT_GATE.md#required-missing-records|ledger_recorded|ledger_record_not_recorded|ledger record is absent, so terminal closure has no durable acceptance chain"
  "index_delivery_records_missing|hepta_core_activation_evidence_receipt_acceptance_denial_gate|scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_ACCEPTANCE_DENIAL_GATE.md#receipt-acceptance-boundary|index_recorded|index_delivery_records_not_recorded|index and delivery records are absent, so no operator-facing completion delivery exists"
  "completion_ack_record_missing|hepta_core_activation_evidence_receipt_acceptance_denial_gate|scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_ACCEPTANCE_DENIAL_GATE.md#receipt-acceptance-boundary|completion_ack_recorded|completion_ack_not_recorded|completion acknowledgement is absent, so terminal closure cannot be accepted"
)

gap_record_lines=()
for gap_spec in "${gap_specs[@]}"; do
  IFS='|' read -r requirement source_gate source_gate_path doc_anchor source_field denied_reason_id denied_reason <<<"$gap_spec"
  witness_hash_sha256="$(
    sha256_text "hepta-core-activation-terminal-closure-gap-evidence-index:$requirement:$source_gate:$source_field:$denied_reason_id:$terminal_closure_report_sha256"
  )"
  gap_record_lines+=("$gap_spec|$witness_hash_sha256")
done

gap_evidence_index_json="$(
  printf '%s\n' "${gap_record_lines[@]}" \
    | jq -R -s \
      --arg terminal_closure_report_sha256 "$terminal_closure_report_sha256" \
      '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            requirement: .[0],
            status: "missing",
            source_gate: .[1],
            source_gate_path: .[2],
            source_report_sha256: $terminal_closure_report_sha256,
            doc_anchor: .[3],
            source_field: .[4],
            source_field_value: false,
            denied_reason_id: .[5],
            denied_reason: .[6],
            witness_hash_sha256: .[7],
            activation_blocking: true,
            terminal_closure_blocking: true,
            report_only: true,
            mutates_runtime: false,
            persists_evidence: false,
            records_approval: false,
            records_receipt: false,
            records_ledger: false,
            delivers_index: false,
            accepts_completion_ack: false
          })
      '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_gap_evidence_index_gate" \
  --arg terminal_closure_report_sha256 "$terminal_closure_report_sha256" \
  --arg gap_index_hash_sha256 "$gap_index_hash_sha256" \
  --arg gap_policy_hash_sha256 "$gap_policy_hash_sha256" \
  --arg gap_side_effect_hash_sha256 "$gap_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson closure "$TERMINAL_CLOSURE_JSON" \
  --argjson gap_evidence_index "$gap_evidence_index_json" \
  '
    {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_closure_gap_evidence_index_schema_version:"hepta_core_activation_terminal_closure_gap_evidence_index_v1",
      terminal_closure_gap_evidence_index_mode:"stdout_only_report_only_index_no_terminal_closure_no_persistence_no_delivery_no_activation",
      terminal_closure_gap_evidence_index_ready:true,
      terminal_closure_gap_evidence_index_status:"blocked",
      terminal_closure_gap_evidence_index_decision:"blocked_until_all_terminal_closure_gap_records_are_operator_approved_recorded_persisted_delivered_and_acknowledged",
      source_terminal_closure_gate:$closure.gate,
      source_terminal_closure_schema_version:$closure.terminal_closure_schema_version,
      source_terminal_closure_decision:$closure.terminal_closure_decision,
      source_terminal_closure_verdict:$closure.terminal_closure_verdict,
      source_terminal_closure_report_sha256:$terminal_closure_report_sha256,
      source_receipt_acceptance_denial_gate:$closure.source_receipt_acceptance_denial_gate,
      source_receipt_acceptance_denial_report_sha256:$closure.source_receipt_acceptance_denial_report_sha256,
      gap_index_hash_sha256:$gap_index_hash_sha256,
      gap_policy_hash_sha256:$gap_policy_hash_sha256,
      gap_side_effect_hash_sha256:$gap_side_effect_hash_sha256,
      minimum_required_long_soak_samples:$min_long_soak_samples,
      source_required_terminal_closure_missing_requirement_count:$closure.required_terminal_closure_missing_requirement_count,
      source_remaining_terminal_closure_missing_requirement_count:$closure.remaining_terminal_closure_missing_requirement_count,
      source_terminal_closure_fixture_count:$closure.terminal_closure_fixture_count,
      source_blocked_terminal_closure_fixture_count:$closure.blocked_terminal_closure_fixture_count,
      source_allowed_terminal_closure_fixture_count:$closure.allowed_terminal_closure_fixture_count,
      required_gap_evidence_count:($closure.terminal_closure_missing_requirements | length),
      indexed_gap_evidence_count:($gap_evidence_index | length),
      missing_gap_evidence_count:($gap_evidence_index | map(select(.status == "missing")) | length),
      ready_gap_evidence_count:0,
      report_only_gap_evidence_count:($gap_evidence_index | map(select(.report_only == true)) | length),
      activation_blocking_gap_evidence_count:($gap_evidence_index | map(select(.activation_blocking == true)) | length),
      terminal_closure_blocking_gap_evidence_count:($gap_evidence_index | map(select(.terminal_closure_blocking == true)) | length),
      source_gate_count:($gap_evidence_index | map(.source_gate) | unique | length),
      doc_anchor_count:($gap_evidence_index | map(.doc_anchor) | unique | length),
      witness_hash_count:($gap_evidence_index | map(.witness_hash_sha256) | unique | length),
      terminal_closure_missing_requirements:$closure.terminal_closure_missing_requirements,
      gap_evidence_index:$gap_evidence_index,
      denied_by_terminal_closure_gap_evidence_index:[
        "operator_approval_recording_denied",
        "activation_request_recording_denied",
        "fresh_evidence_acceptance_denied",
        "filesystem_persistence_approval_denied",
        "receipt_persistence_command_enablement_denied",
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
      denied_by_terminal_closure_gap_evidence_index_count:14,
      terminal_closure_allowed:false,
      terminal_closure_recorded:false,
      terminal_closure_persisted:false,
      terminal_closure_accepted:false,
      activation_allowed:false,
      activation_performed:false,
      operator_approval_recording_allowed:false,
      activation_request_recording_allowed:false,
      fresh_evidence_acceptance_allowed:false,
      filesystem_persistence_approval_allowed:false,
      receipt_persistence_command_enablement_allowed:false,
      receipt_persistence_execution_allowed:false,
      receipt_acceptance_allowed:false,
      ledger_recording_allowed:false,
      index_delivery_allowed:false,
      completion_ack_acceptance_allowed:false,
      public_release_claim_allowed:false,
      release_artifact_write_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      install_restart_allowed:false,
      active_binary_mutation_allowed:false,
      upstream_fetch_merge_allowed:false,
      credential_read_allowed:false,
      secret_value_read_allowed:false,
      side_effects:{
        workspace_written:false,
        filesystem_written:false,
        memory_store_mutated:false,
        approval_recorded:false,
        activation_request_recorded:false,
        fresh_evidence_accepted:false,
        receipt_persistence_command_enabled:false,
        receipt_persistence_execution_performed:false,
        receipt_acceptance_recorded:false,
        ledger_recorded:false,
        index_recorded:false,
        delivery_recorded:false,
        completion_ack_recorded:false,
        terminal_closure_recorded:false,
        terminal_closure_persisted:false,
        terminal_closure_accepted:false,
        activation_performed:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        telegram_send_performed:false,
        external_send_performed:false,
        release_artifact_written:false,
        public_release_claimed:false,
        install_executed:false,
        launchd_mutated:false,
        service_restarted:false,
        active_binary_mutated:false,
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        credential_read:false,
        secret_value_read:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_terminal_closure_gap_evidence_index_gate"
  and .terminal_closure_gap_evidence_index_schema_version == "hepta_core_activation_terminal_closure_gap_evidence_index_v1"
  and .terminal_closure_gap_evidence_index_mode == "stdout_only_report_only_index_no_terminal_closure_no_persistence_no_delivery_no_activation"
  and .terminal_closure_gap_evidence_index_ready == true
  and .terminal_closure_gap_evidence_index_status == "blocked"
  and .source_terminal_closure_gate == "hepta_core_activation_evidence_receipt_terminal_closure_decision_gate"
  and .source_terminal_closure_verdict == "blocked"
  and .source_required_terminal_closure_missing_requirement_count == 12
  and .source_remaining_terminal_closure_missing_requirement_count == 12
  and .source_terminal_closure_fixture_count == 6
  and .source_blocked_terminal_closure_fixture_count == 6
  and .source_allowed_terminal_closure_fixture_count == 0
  and .required_gap_evidence_count == 12
  and .indexed_gap_evidence_count == 12
  and .missing_gap_evidence_count == 12
  and .ready_gap_evidence_count == 0
  and .report_only_gap_evidence_count == 12
  and .activation_blocking_gap_evidence_count == 12
  and .terminal_closure_blocking_gap_evidence_count == 12
  and .source_gate_count >= 6
  and .doc_anchor_count >= 6
  and .witness_hash_count == 12
  and ((.terminal_closure_missing_requirements | sort) == (.gap_evidence_index | map(.requirement) | sort))
  and (.gap_evidence_index | all(
    .status == "missing"
    and .source_gate != ""
    and .source_gate_path != ""
    and .source_report_sha256 != ""
    and .doc_anchor != ""
    and .source_field != ""
    and .source_field_value == false
    and .denied_reason_id != ""
    and .denied_reason != ""
    and (.witness_hash_sha256 | test("^[0-9a-f]{64}$"))
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
  and (.gap_evidence_index | any(.requirement == "explicit_operator_approval_record_missing" and .source_gate == "hepta_core_activation_long_soak_operator_approval_packet_gate"))
  and (.gap_evidence_index | any(.requirement == "fresh_24_sample_long_soak_evidence_record_missing" and .source_gate == "hepta_core_activation_long_soak_observation_freshness_denial_gate"))
  and (.gap_evidence_index | any(.requirement == "filesystem_persistence_approval_record_missing" and .source_gate == "hepta_core_activation_evidence_receipt_filesystem_persistence_denial_gate"))
  and (.gap_evidence_index | any(.requirement == "receipt_acceptance_record_missing" and .source_gate == "hepta_core_activation_evidence_receipt_acceptance_denial_gate"))
  and (.gap_evidence_index | any(.requirement == "ledger_record_missing" and .source_gate == "hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate"))
  and (.gap_evidence_index | any(.requirement == "completion_ack_record_missing" and .source_gate == "hepta_core_activation_evidence_receipt_acceptance_denial_gate"))
  and .denied_by_terminal_closure_gap_evidence_index_count == 14
  and (.denied_by_terminal_closure_gap_evidence_index | length) == 14
  and .terminal_closure_allowed == false
  and .terminal_closure_recorded == false
  and .terminal_closure_persisted == false
  and .terminal_closure_accepted == false
  and .activation_allowed == false
  and .activation_performed == false
  and .operator_approval_recording_allowed == false
  and .activation_request_recording_allowed == false
  and .fresh_evidence_acceptance_allowed == false
  and .filesystem_persistence_approval_allowed == false
  and .receipt_persistence_command_enablement_allowed == false
  and .receipt_persistence_execution_allowed == false
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
