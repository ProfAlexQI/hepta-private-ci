#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"

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
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-negative-fixture-matrix-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-negative-fixture-matrix-gate.sh
)"

negative_matrix_report_sha256="$(sha256_text "$NEGATIVE_MATRIX_JSON")"
positive_precondition_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-trusted-operator-acceptance-record-positive-precondition-scoreboard:v1:source-negative-fixture-matrix:12-preconditions:no-operator-record:no-accept:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_trusted_operator_acceptance_record_positive_precondition_scoreboard_side_effects=false;satisfied=0;accepted=0;recorded=0;persisted=0;dispatch=0;context=0;provider=0;model=0;memory=0;kg=0;secret=0;restart=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$NEGATIVE_MATRIX_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_negative_fixture_matrix_gate"
    and $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_ready == true
    and $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_status == "blocked"
    and $source.source_trusted_operator_acceptance_record_intake_validator_ready == true
    and $source.source_trusted_operator_acceptance_record_intake_validator_status == "blocked"
    and $source.required_negative_fixture_count == 12
    and $source.negative_fixture_count == 12
    and $source.supplied_negative_fixture_count == 12
    and $source.blocked_negative_fixture_count == 12
    and $source.accepted_negative_fixture_count == 0
    and $source.recorded_negative_fixture_count == 0
    and $source.persisted_negative_fixture_count == 0
    and $source.delivered_negative_fixture_count == 0
    and $source.operator_identity_negative_fixture_count == 1
    and $source.operator_signature_negative_fixture_count == 1
    and $source.operator_timestamp_negative_fixture_count == 1
    and $source.scope_negative_fixture_count == 2
    and $source.hash_binding_negative_fixture_count == 3
    and $source.idempotency_negative_fixture_count == 1
    and $source.rollback_kill_switch_negative_fixture_count == 1
    and $source.dispatch_budget_negative_fixture_count == 1
    and $source.secret_injection_negative_fixture_count == 1
    and $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_dispatch_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_context_attachment_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_provider_model_invocation_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_memory_write_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_external_kg_read_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_live_kg_write_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_live_execution_count == 0
    and ($source.operator_canary_trusted_operator_acceptance_record_negative_fixtures | all(
      .negative_fixture_acceptance_attempted == true
      and .negative_fixture_blocked == true
      and .trusted_operator_acceptance_record_accepted == false
      and .trusted_operator_acceptance_record_recorded == false
      and .trusted_operator_acceptance_record_persisted == false
      and .trusted_operator_acceptance_record_delivered == false
      and .authorizes_canary_dispatch == false
      and .authorizes_context_attachment == false
      and .authorizes_provider_model_invocation == false
      and .authorizes_memory_write == false
      and .authorizes_external_kg_read == false
      and .authorizes_live_kg_write == false
      and .authorizes_live_execution == false
      and .controlled_request_dispatched == false
      and .controlled_request_executed == false
      and .context_injection_performed == false
      and .provider_invoked == false
      and .model_invoked == false
      and .memory_store_write_performed == false
      and .external_kg_adapter_read_performed == false
      and .live_kg_write_performed == false
      and .credential_read == false
      and .auth_secret_read == false
      and .secret_file_read == false
      and .install_performed == false
      and .service_restarted == false
      and .active_binary_mutated == false
    ))
    and $source.operator_record_accepted == false
    and $source.operator_record_recorded == false
    and $source.operator_record_persisted == false
    and $source.operator_record_authorizes_dispatch == false
    and $source.operator_record_authorizes_context_attachment == false
    and $source.operator_record_authorizes_provider_model_invocation == false
    and $source.operator_record_authorizes_memory_write == false
    and $source.operator_record_authorizes_external_kg_read == false
    and $source.operator_record_authorizes_live_kg_write == false
    and $source.operator_record_authorizes_live_execution == false
    and $source.canary_harness_activation_ready == false
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and $source.canary_execution_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_positive_precondition_scoreboard_gate" \
    --arg negative_matrix_report_sha256 "$negative_matrix_report_sha256" \
    --arg positive_precondition_policy_hash_sha256 "$positive_precondition_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$NEGATIVE_MATRIX_JSON" \
    '
      [
        {
          precondition_id: "operator-identity-current",
          precondition_family: "operator_identity",
          required_evidence: "current operator identity must be present and bound to the canary record",
          source_negative_fixture_family: "operator_identity"
        },
        {
          precondition_id: "operator-signature-hash-matches-payload",
          precondition_family: "operator_signature",
          required_evidence: "operator signature hash must match the exact trusted record payload",
          source_negative_fixture_family: "operator_signature"
        },
        {
          precondition_id: "operator-signed-at-fresh",
          precondition_family: "operator_timestamp",
          required_evidence: "signed-at timestamp must be fresh for the current canary scope",
          source_negative_fixture_family: "operator_timestamp"
        },
        {
          precondition_id: "route-scope-matches-canary",
          precondition_family: "scope",
          required_evidence: "trusted record route scope must match the single canary route",
          source_negative_fixture_family: "scope"
        },
        {
          precondition_id: "namespace-scope-matches-canary",
          precondition_family: "scope",
          required_evidence: "trusted record namespace scope must match the single canary namespace",
          source_negative_fixture_family: "scope"
        },
        {
          precondition_id: "value-scoreboard-hash-matches",
          precondition_family: "hash_binding",
          required_evidence: "value scoreboard hash must bind to the source acceptance packet value scoreboard",
          source_negative_fixture_family: "hash_binding"
        },
        {
          precondition_id: "readback-receipt-hash-matches",
          precondition_family: "hash_binding",
          required_evidence: "readback receipt hash must bind to the payload readback preview",
          source_negative_fixture_family: "hash_binding"
        },
        {
          precondition_id: "audit-receipt-hash-matches",
          precondition_family: "hash_binding",
          required_evidence: "audit receipt hash must bind to the audit trail preview",
          source_negative_fixture_family: "hash_binding"
        },
        {
          precondition_id: "idempotency-nonce-current-and-unused",
          precondition_family: "idempotency",
          required_evidence: "idempotency nonce must be current, unused, and bound to the operator record",
          source_negative_fixture_family: "idempotency"
        },
        {
          precondition_id: "rollback-plan-and-kill-switch-present",
          precondition_family: "rollback_kill_switch",
          required_evidence: "rollback plan and kill switch must be present before any canary arm",
          source_negative_fixture_family: "rollback_kill_switch"
        },
        {
          precondition_id: "dispatch-budget-equals-one",
          precondition_family: "dispatch_budget",
          required_evidence: "dispatch budget must be exactly one controlled request for the canary",
          source_negative_fixture_family: "dispatch_budget"
        },
        {
          precondition_id: "secret-injection-absent",
          precondition_family: "secret_boundary",
          required_evidence: "operator record must not carry secret material or credential injection",
          source_negative_fixture_family: "secret_injection"
        }
      ]
      | map(
          . + {
            source_negative_matrix_hash_bound: true,
            source_negative_matrix_sha256: $negative_matrix_report_sha256,
            positive_precondition_declared: true,
            positive_precondition_satisfied: false,
            positive_precondition_missing: true,
            positive_precondition_accepted: false,
            trusted_operator_record_field_accepted: false,
            authorizes_canary_dispatch: false,
            authorizes_context_attachment: false,
            authorizes_provider_model_invocation: false,
            authorizes_memory_write: false,
            authorizes_external_kg_read: false,
            authorizes_live_kg_write: false,
            authorizes_live_execution: false,
            status: "blocked_until_real_operator_record_evidence"
          }
        ) as $positive_preconditions
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_trusted_operator_acceptance_record_positive_precondition_scoreboard_ready: true,
          operator_canary_trusted_operator_acceptance_record_positive_precondition_scoreboard_status: "blocked",
          scoreboard_mode: "stdout_only_report_only_positive_precondition_scoreboard_no_operator_record_no_acceptance_no_dispatch_no_live",
          min_long_soak_samples: $min_long_soak_samples,
          negative_matrix_report_sha256: $negative_matrix_report_sha256,
          positive_precondition_policy_hash_sha256: $positive_precondition_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          source_negative_fixture_matrix_ready: $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_ready,
          source_negative_fixture_matrix_status: $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_status,
          source_negative_fixture_count: $source.negative_fixture_count,
          source_blocked_negative_fixture_count: $source.blocked_negative_fixture_count,
          source_accepted_negative_fixture_count: $source.accepted_negative_fixture_count,
          source_recorded_negative_fixture_count: $source.recorded_negative_fixture_count,
          source_persisted_negative_fixture_count: $source.persisted_negative_fixture_count,
          source_delivered_negative_fixture_count: $source.delivered_negative_fixture_count,
          source_authorizes_dispatch_count: $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_dispatch_count,
          source_authorizes_context_attachment_count: $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_context_attachment_count,
          source_authorizes_provider_model_invocation_count: $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_provider_model_invocation_count,
          source_authorizes_memory_write_count: $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_memory_write_count,
          source_authorizes_external_kg_read_count: $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_external_kg_read_count,
          source_authorizes_live_kg_write_count: $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_live_kg_write_count,
          source_authorizes_live_execution_count: $source.operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_live_execution_count,
          positive_precondition_count: ($positive_preconditions | length),
          declared_positive_precondition_count: ($positive_preconditions | map(select(.positive_precondition_declared == true)) | length),
          satisfied_positive_precondition_count: ($positive_preconditions | map(select(.positive_precondition_satisfied == true)) | length),
          missing_positive_precondition_count: ($positive_preconditions | map(select(.positive_precondition_missing == true)) | length),
          accepted_positive_precondition_count: ($positive_preconditions | map(select(.positive_precondition_accepted == true)) | length),
          dispatch_authorizing_positive_precondition_count: ($positive_preconditions | map(select(.authorizes_canary_dispatch == true)) | length),
          live_authorizing_positive_precondition_count: ($positive_preconditions | map(select(.authorizes_live_execution == true)) | length),
          positive_precondition_family_count: ($positive_preconditions | map(.precondition_family) | unique | length),
          positive_preconditions: $positive_preconditions,
          operator_record_required: true,
          operator_record_supplied: false,
          operator_record_accepted: false,
          operator_record_recorded: false,
          operator_record_persisted: false,
          operator_record_delivered: false,
          operator_record_authorizes_dispatch: false,
          operator_record_authorizes_context_attachment: false,
          operator_record_authorizes_provider_model_invocation: false,
          operator_record_authorizes_memory_write: false,
          operator_record_authorizes_external_kg_read: false,
          operator_record_authorizes_live_kg_write: false,
          operator_record_authorizes_live_execution: false,
          canary_harness_activation_ready: false,
          canary_harness_armed: false,
          canary_harness_executable: false,
          canary_live_enabled: false,
          canary_execution_performed: false,
          controlled_request_dispatched: false,
          controlled_request_executed: false,
          context_injection_performed: false,
          provider_invoked: false,
          model_invoked: false,
          memory_store_write_performed: false,
          memory_store_mutated: false,
          external_kg_adapter_read_performed: false,
          live_kg_write_performed: false,
          credential_read: false,
          auth_secret_read: false,
          secret_file_read: false,
          channel_send_performed: false,
          service_restarted: false,
          active_binary_mutated: false,
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            operator_record_recorded: false,
            operator_record_persisted: false,
            operator_record_delivered: false,
            operator_record_accepted: false,
            canary_harness_armed: false,
            controlled_request_dispatched: false,
            controlled_request_executed: false,
            context_injection_performed: false,
            provider_invoked: false,
            model_invoked: false,
            memory_store_write_performed: false,
            memory_store_mutated: false,
            external_kg_adapter_read_performed: false,
            live_kg_write_performed: false,
            credential_read: false,
            auth_secret_read: false,
            secret_file_read: false,
            channel_send_performed: false,
            service_restarted: false,
            active_binary_mutated: false,
            install_performed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false
          }
        }
    '
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_positive_precondition_scoreboard_gate"
  and .operator_canary_trusted_operator_acceptance_record_positive_precondition_scoreboard_ready == true
  and .operator_canary_trusted_operator_acceptance_record_positive_precondition_scoreboard_status == "blocked"
  and .source_negative_fixture_matrix_ready == true
  and .source_negative_fixture_matrix_status == "blocked"
  and .source_negative_fixture_count == 12
  and .source_blocked_negative_fixture_count == 12
  and .source_accepted_negative_fixture_count == 0
  and .source_recorded_negative_fixture_count == 0
  and .source_persisted_negative_fixture_count == 0
  and .source_delivered_negative_fixture_count == 0
  and .source_authorizes_dispatch_count == 0
  and .source_authorizes_context_attachment_count == 0
  and .source_authorizes_provider_model_invocation_count == 0
  and .source_authorizes_memory_write_count == 0
  and .source_authorizes_external_kg_read_count == 0
  and .source_authorizes_live_kg_write_count == 0
  and .source_authorizes_live_execution_count == 0
  and .positive_precondition_count == 12
  and .declared_positive_precondition_count == 12
  and .satisfied_positive_precondition_count == 0
  and .missing_positive_precondition_count == 12
  and .accepted_positive_precondition_count == 0
  and .dispatch_authorizing_positive_precondition_count == 0
  and .live_authorizing_positive_precondition_count == 0
  and .positive_precondition_family_count == 9
  and (.positive_preconditions | all(
    .source_negative_matrix_hash_bound == true
    and .positive_precondition_declared == true
    and .positive_precondition_satisfied == false
    and .positive_precondition_missing == true
    and .positive_precondition_accepted == false
    and .trusted_operator_record_field_accepted == false
    and .authorizes_canary_dispatch == false
    and .authorizes_context_attachment == false
    and .authorizes_provider_model_invocation == false
    and .authorizes_memory_write == false
    and .authorizes_external_kg_read == false
    and .authorizes_live_kg_write == false
    and .authorizes_live_execution == false
    and .status == "blocked_until_real_operator_record_evidence"
  ))
  and .operator_record_required == true
  and .operator_record_supplied == false
  and .operator_record_accepted == false
  and .operator_record_recorded == false
  and .operator_record_persisted == false
  and .operator_record_delivered == false
  and .operator_record_authorizes_dispatch == false
  and .operator_record_authorizes_context_attachment == false
  and .operator_record_authorizes_provider_model_invocation == false
  and .operator_record_authorizes_memory_write == false
  and .operator_record_authorizes_external_kg_read == false
  and .operator_record_authorizes_live_kg_write == false
  and .operator_record_authorizes_live_execution == false
  and .canary_harness_activation_ready == false
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .canary_execution_performed == false
  and .controlled_request_dispatched == false
  and .controlled_request_executed == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .external_kg_adapter_read_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .auth_secret_read == false
  and .secret_file_read == false
  and .channel_send_performed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG trusted operator acceptance record positive precondition scoreboard gate passed"
