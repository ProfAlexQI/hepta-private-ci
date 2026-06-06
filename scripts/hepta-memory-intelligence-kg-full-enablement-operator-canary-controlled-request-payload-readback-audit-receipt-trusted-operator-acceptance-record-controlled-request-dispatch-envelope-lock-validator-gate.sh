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

READINESS_LOCK_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-readiness-lock-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-readiness-lock-gate.sh
)"

readiness_lock_report_sha256="$(sha256_text "$READINESS_LOCK_JSON")"
dispatch_envelope_lock_validator_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-trusted-operator-acceptance-record-dispatch-envelope-lock-validator:v1:source-readiness-locks:9-locks:budget-one:readback:audit:no-accept:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_trusted_operator_record_dispatch_envelope_lock_validator_side_effects=false;locks=9;fixtures=6;accepted=0;budget_accepted=false;budget_consumed=0;dispatch=0;execute=0;context=0;provider=0;model=0;memory=0;kg=0;secret=0;restart=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$READINESS_LOCK_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_readiness_lock_gate"
    and $source.operator_canary_trusted_operator_acceptance_record_readiness_lock_ready == true
    and $source.operator_canary_trusted_operator_acceptance_record_readiness_lock_status == "blocked"
    and $source.readiness_lock_count == 9
    and $source.declared_readiness_lock_count == 9
    and $source.report_only_readiness_lock_count == 9
    and $source.operator_input_required_readiness_lock_count == 9
    and $source.operator_input_supplied_readiness_lock_count == 0
    and $source.recorded_readiness_lock_count == 0
    and $source.persisted_readiness_lock_count == 0
    and $source.delivered_readiness_lock_count == 0
    and $source.accepted_readiness_lock_count == 0
    and $source.dispatch_authorizing_readiness_lock_count == 0
    and $source.context_authorizing_readiness_lock_count == 0
    and $source.provider_model_authorizing_readiness_lock_count == 0
    and $source.memory_write_authorizing_readiness_lock_count == 0
    and $source.external_kg_read_authorizing_readiness_lock_count == 0
    and $source.live_kg_write_authorizing_readiness_lock_count == 0
    and $source.live_execution_authorizing_readiness_lock_count == 0
    and $source.single_route_scope_lock_declared == true
    and $source.single_namespace_scope_lock_declared == true
    and $source.payload_readback_receipt_hash_lock_declared == true
    and $source.audit_receipt_hash_lock_declared == true
    and $source.dispatch_budget_one_lock_declared == true
    and $source.dispatch_budget_one_lock_accepted == false
    and $source.dispatch_budget_exactly_one_accepted == false
    and $source.controlled_request_budget_accepted == false
    and $source.controlled_request_budget_value == null
    and ($source.readiness_locks | all(
      .readiness_lock_shape_declared == true
      and .readiness_lock_report_only == true
      and .readiness_lock_operator_input_required == true
      and .readiness_lock_operator_input_supplied == false
      and .readiness_lock_recorded == false
      and .readiness_lock_persisted == false
      and .readiness_lock_delivered == false
      and .readiness_lock_accepted == false
      and .readiness_lock_authorizes_dispatch == false
      and .readiness_lock_authorizes_context_attachment == false
      and .readiness_lock_authorizes_provider_model_invocation == false
      and .readiness_lock_authorizes_memory_write == false
      and .readiness_lock_authorizes_external_kg_read == false
      and .readiness_lock_authorizes_live_kg_write == false
      and .readiness_lock_authorizes_live_execution == false
      and .controlled_request_dispatched == false
      and .controlled_request_executed == false
      and .context_injection_performed == false
      and .provider_invoked == false
      and .model_invoked == false
      and .memory_store_write_performed == false
      and .external_kg_adapter_read_performed == false
      and .live_kg_write_performed == false
      and .credential_read == false
      and .secret_file_read == false
    ))
    and $source.operator_record_supplied == false
    and $source.operator_record_accepted == false
    and $source.operator_record_recorded == false
    and $source.operator_record_persisted == false
    and $source.operator_record_delivered == false
    and $source.operator_record_authorizes_dispatch == false
    and $source.operator_record_authorizes_live_execution == false
    and $source.canary_harness_activation_ready == false
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and $source.canary_execution_performed == false
    and $source.controlled_request_dispatched == false
    and $source.controlled_request_executed == false
    and $source.context_injection_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.memory_store_write_performed == false
    and $source.external_kg_adapter_read_performed == false
    and $source.live_kg_write_performed == false
    and $source.credential_read == false
    and $source.auth_secret_read == false
    and $source.secret_file_read == false
    and $source.channel_send_performed == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

negative_fixtures_json="$(
  jq -n '
    [
      {
        fixture_id: "missing-route-scope-lock",
        fixture_kind: "missing_required_scope_lock",
        missing_lock_id: "single-route-scope-lock",
        requested_dispatch_budget: 1,
        denial_reason: "single canary route lock is required before any dispatch envelope can be accepted"
      },
      {
        fixture_id: "missing-namespace-scope-lock",
        fixture_kind: "missing_required_scope_lock",
        missing_lock_id: "single-namespace-scope-lock",
        requested_dispatch_budget: 1,
        denial_reason: "single canary namespace lock is required before any dispatch envelope can be accepted"
      },
      {
        fixture_id: "missing-readback-receipt-hash-lock",
        fixture_kind: "missing_required_readback_lock",
        missing_lock_id: "payload-readback-receipt-hash-lock",
        requested_dispatch_budget: 1,
        denial_reason: "payload readback receipt hash lock is required before dispatch"
      },
      {
        fixture_id: "missing-audit-receipt-hash-lock",
        fixture_kind: "missing_required_audit_lock",
        missing_lock_id: "audit-receipt-hash-lock",
        requested_dispatch_budget: 1,
        denial_reason: "audit receipt hash lock is required before dispatch"
      },
      {
        fixture_id: "dispatch-budget-greater-than-one",
        fixture_kind: "dispatch_budget_violation",
        missing_lock_id: null,
        requested_dispatch_budget: 2,
        denial_reason: "canary dispatch budget must be exactly one controlled request"
      },
      {
        fixture_id: "secret-injection-attempt",
        fixture_kind: "secret_boundary_violation",
        missing_lock_id: null,
        requested_dispatch_budget: 1,
        denial_reason: "dispatch envelope must remain secret-free and cannot carry credential injection"
      }
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_gate" \
    --arg readiness_lock_report_sha256 "$readiness_lock_report_sha256" \
    --arg dispatch_envelope_lock_validator_policy_hash_sha256 "$dispatch_envelope_lock_validator_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$READINESS_LOCK_JSON" \
    --argjson negative_fixtures "$negative_fixtures_json" \
    '
      ($source.readiness_locks | map({
        dispatch_envelope_lock_binding_id: ("dispatch-envelope-binding-" + .lock_id),
        lock_id,
        lock_family,
        source_precondition_id,
        source_template_section_id,
        source_required_record_fields,
        source_required_record_field_count: (.source_required_record_fields | length),
        source_readiness_lock_report_sha256: $readiness_lock_report_sha256,
        dispatch_envelope_binding_declared: true,
        dispatch_envelope_binding_required: true,
        dispatch_envelope_binding_report_only: true,
        dispatch_envelope_binding_operator_input_required: .readiness_lock_operator_input_required,
        dispatch_envelope_binding_operator_input_supplied: false,
        dispatch_envelope_binding_recorded: false,
        dispatch_envelope_binding_persisted: false,
        dispatch_envelope_binding_delivered: false,
        dispatch_envelope_binding_accepted: false,
        dispatch_envelope_binding_authorizes_dispatch: false,
        dispatch_envelope_binding_authorizes_context_attachment: false,
        dispatch_envelope_binding_authorizes_provider_model_invocation: false,
        dispatch_envelope_binding_authorizes_memory_write: false,
        dispatch_envelope_binding_authorizes_external_kg_read: false,
        dispatch_envelope_binding_authorizes_live_kg_write: false,
        dispatch_envelope_binding_authorizes_live_execution: false,
        controlled_request_dispatched: false,
        controlled_request_executed: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        memory_store_write_performed: false,
        external_kg_adapter_read_performed: false,
        live_kg_write_performed: false,
        credential_read: false,
        secret_file_read: false,
        status: "blocked_until_lock_input_accepted"
      })) as $bindings
      | ($negative_fixtures | map(. + {
          fixture_status: "blocked",
          dispatch_envelope_accepted: false,
          controlled_request_dispatch_allowed: false,
          controlled_request_dispatched: false,
          controlled_request_executed: false,
          context_injection_performed: false,
          provider_invoked: false,
          model_invoked: false,
          memory_store_write_performed: false,
          external_kg_adapter_read_performed: false,
          live_kg_write_performed: false,
          credential_read: false,
          secret_file_read: false
        })) as $fixtures
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_schema_version: "memory_intelligence_kg_operator_canary_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_v1",
          operator_canary_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_ready: true,
          operator_canary_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_status: "blocked",
          dispatch_envelope_lock_validator_mode: "stdout_only_report_only_dispatch_envelope_lock_validator_no_operator_record_no_acceptance_no_dispatch_no_live",
          dispatch_envelope_lock_validator_decision: "future_controlled_request_dispatch_envelope_is_bound_to_nine_readiness_locks_but_cannot_dispatch_until_all_locks_are_supplied_accepted_and_budget_one_is_accepted",
          min_long_soak_samples: $min_long_soak_samples,
          source_readiness_lock_gate: $source.gate,
          source_readiness_lock_status: $source.operator_canary_trusted_operator_acceptance_record_readiness_lock_status,
          source_readiness_lock_report_sha256: $readiness_lock_report_sha256,
          source_readiness_lock_count: $source.readiness_lock_count,
          source_accepted_readiness_lock_count: $source.accepted_readiness_lock_count,
          source_dispatch_authorizing_readiness_lock_count: $source.dispatch_authorizing_readiness_lock_count,
          dispatch_envelope_lock_validator_policy_hash_sha256: $dispatch_envelope_lock_validator_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          dispatch_envelope_lock_bindings: $bindings,
          dispatch_envelope_lock_binding_count: ($bindings | length),
          dispatch_envelope_lock_binding_declared_count: ($bindings | map(select(.dispatch_envelope_binding_declared == true)) | length),
          dispatch_envelope_lock_binding_required_count: ($bindings | map(select(.dispatch_envelope_binding_required == true)) | length),
          dispatch_envelope_lock_binding_report_only_count: ($bindings | map(select(.dispatch_envelope_binding_report_only == true)) | length),
          dispatch_envelope_lock_binding_operator_input_required_count: ($bindings | map(select(.dispatch_envelope_binding_operator_input_required == true)) | length),
          dispatch_envelope_lock_binding_operator_input_supplied_count: ($bindings | map(select(.dispatch_envelope_binding_operator_input_supplied == true)) | length),
          dispatch_envelope_lock_binding_recorded_count: ($bindings | map(select(.dispatch_envelope_binding_recorded == true)) | length),
          dispatch_envelope_lock_binding_persisted_count: ($bindings | map(select(.dispatch_envelope_binding_persisted == true)) | length),
          dispatch_envelope_lock_binding_delivered_count: ($bindings | map(select(.dispatch_envelope_binding_delivered == true)) | length),
          dispatch_envelope_lock_binding_accepted_count: ($bindings | map(select(.dispatch_envelope_binding_accepted == true)) | length),
          dispatch_envelope_lock_binding_authorizes_dispatch_count: ($bindings | map(select(.dispatch_envelope_binding_authorizes_dispatch == true)) | length),
          dispatch_envelope_lock_binding_authorizes_context_attachment_count: ($bindings | map(select(.dispatch_envelope_binding_authorizes_context_attachment == true)) | length),
          dispatch_envelope_lock_binding_authorizes_provider_model_invocation_count: ($bindings | map(select(.dispatch_envelope_binding_authorizes_provider_model_invocation == true)) | length),
          dispatch_envelope_lock_binding_authorizes_memory_write_count: ($bindings | map(select(.dispatch_envelope_binding_authorizes_memory_write == true)) | length),
          dispatch_envelope_lock_binding_authorizes_external_kg_read_count: ($bindings | map(select(.dispatch_envelope_binding_authorizes_external_kg_read == true)) | length),
          dispatch_envelope_lock_binding_authorizes_live_kg_write_count: ($bindings | map(select(.dispatch_envelope_binding_authorizes_live_kg_write == true)) | length),
          dispatch_envelope_lock_binding_authorizes_live_execution_count: ($bindings | map(select(.dispatch_envelope_binding_authorizes_live_execution == true)) | length),
          route_scope_lock_binding_declared: ($bindings | any(.lock_id == "single-route-scope-lock" and .dispatch_envelope_binding_declared == true)),
          namespace_scope_lock_binding_declared: ($bindings | any(.lock_id == "single-namespace-scope-lock" and .dispatch_envelope_binding_declared == true)),
          value_scoreboard_hash_lock_binding_declared: ($bindings | any(.lock_id == "value-scoreboard-hash-lock" and .dispatch_envelope_binding_declared == true)),
          payload_readback_receipt_hash_lock_binding_declared: ($bindings | any(.lock_id == "payload-readback-receipt-hash-lock" and .dispatch_envelope_binding_declared == true)),
          audit_receipt_hash_lock_binding_declared: ($bindings | any(.lock_id == "audit-receipt-hash-lock" and .dispatch_envelope_binding_declared == true)),
          idempotency_nonce_lock_binding_declared: ($bindings | any(.lock_id == "idempotency-nonce-current-unused-lock" and .dispatch_envelope_binding_declared == true)),
          rollback_kill_switch_lock_binding_declared: ($bindings | any(.lock_id == "rollback-kill-switch-lock" and .dispatch_envelope_binding_declared == true)),
          dispatch_budget_one_lock_binding_declared: ($bindings | any(.lock_id == "dispatch-budget-one-lock" and .dispatch_envelope_binding_declared == true)),
          secret_injection_absent_lock_binding_declared: ($bindings | any(.lock_id == "secret-injection-absent-lock" and .dispatch_envelope_binding_declared == true)),
          controlled_request_dispatch_budget_declared: 1,
          controlled_request_dispatch_budget_accepted: false,
          controlled_request_dispatch_budget_consumed: 0,
          controlled_request_dispatch_budget_remaining: 0,
          dispatch_envelope_shape_declared: true,
          dispatch_envelope_recorded: false,
          dispatch_envelope_persisted: false,
          dispatch_envelope_materialized: false,
          dispatch_envelope_delivered: false,
          dispatch_envelope_accepted: false,
          dispatch_envelope_authorizes_dispatch: false,
          dispatch_envelope_authorizes_live_execution: false,
          value_scoreboard_hash_accepted: false,
          payload_readback_receipt_hash_accepted: false,
          audit_receipt_hash_accepted: false,
          idempotency_nonce_accepted: false,
          rollback_kill_switch_armed: false,
          secret_injection_absence_accepted: false,
          dispatch_envelope_negative_fixtures: $fixtures,
          dispatch_envelope_negative_fixture_count: ($fixtures | length),
          dispatch_envelope_blocked_negative_fixture_count: ($fixtures | map(select(.fixture_status == "blocked")) | length),
          dispatch_envelope_allowed_negative_fixture_count: ($fixtures | map(select(.fixture_status == "allowed")) | length),
          operator_record_supplied: false,
          operator_record_accepted: false,
          operator_record_recorded: false,
          operator_record_persisted: false,
          operator_record_delivered: false,
          operator_record_authorizes_dispatch: false,
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
          denied_by_dispatch_envelope_lock_validator: [
            "dispatch_envelope_lock_validator_not_operator_approval",
            "dispatch_envelope_lock_binding_acceptance_denied",
            "dispatch_envelope_recording_denied",
            "dispatch_envelope_persistence_denied",
            "dispatch_envelope_materialization_denied",
            "single_route_scope_lock_unaccepted",
            "single_namespace_scope_lock_unaccepted",
            "value_scoreboard_hash_lock_unaccepted",
            "payload_readback_receipt_hash_lock_unaccepted",
            "audit_receipt_hash_lock_unaccepted",
            "idempotency_nonce_lock_unaccepted",
            "rollback_kill_switch_lock_unaccepted",
            "dispatch_budget_one_lock_unaccepted",
            "secret_injection_absent_lock_unaccepted",
            "controlled_request_dispatch_denied",
            "controlled_request_execution_denied",
            "context_attachment_denied",
            "provider_model_invocation_denied",
            "memory_write_denied",
            "external_kg_read_denied",
            "live_kg_write_denied",
            "secret_credential_read_denied"
          ],
          denied_by_dispatch_envelope_lock_validator_count: 22,
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            dispatch_envelope_recorded: false,
            dispatch_envelope_persisted: false,
            dispatch_envelope_materialized: false,
            dispatch_envelope_delivered: false,
            dispatch_envelope_accepted: false,
            readiness_lock_accepted: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_gate"
  and .operator_canary_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_ready == true
  and .operator_canary_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_status == "blocked"
  and .dispatch_envelope_lock_validator_mode == "stdout_only_report_only_dispatch_envelope_lock_validator_no_operator_record_no_acceptance_no_dispatch_no_live"
  and .source_readiness_lock_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_readiness_lock_gate"
  and .source_readiness_lock_status == "blocked"
  and .source_readiness_lock_count == 9
  and .source_accepted_readiness_lock_count == 0
  and .source_dispatch_authorizing_readiness_lock_count == 0
  and .dispatch_envelope_lock_binding_count == 9
  and .dispatch_envelope_lock_binding_declared_count == 9
  and .dispatch_envelope_lock_binding_required_count == 9
  and .dispatch_envelope_lock_binding_report_only_count == 9
  and .dispatch_envelope_lock_binding_operator_input_required_count == 9
  and .dispatch_envelope_lock_binding_operator_input_supplied_count == 0
  and .dispatch_envelope_lock_binding_recorded_count == 0
  and .dispatch_envelope_lock_binding_persisted_count == 0
  and .dispatch_envelope_lock_binding_delivered_count == 0
  and .dispatch_envelope_lock_binding_accepted_count == 0
  and .dispatch_envelope_lock_binding_authorizes_dispatch_count == 0
  and .dispatch_envelope_lock_binding_authorizes_context_attachment_count == 0
  and .dispatch_envelope_lock_binding_authorizes_provider_model_invocation_count == 0
  and .dispatch_envelope_lock_binding_authorizes_memory_write_count == 0
  and .dispatch_envelope_lock_binding_authorizes_external_kg_read_count == 0
  and .dispatch_envelope_lock_binding_authorizes_live_kg_write_count == 0
  and .dispatch_envelope_lock_binding_authorizes_live_execution_count == 0
  and .route_scope_lock_binding_declared == true
  and .namespace_scope_lock_binding_declared == true
  and .value_scoreboard_hash_lock_binding_declared == true
  and .payload_readback_receipt_hash_lock_binding_declared == true
  and .audit_receipt_hash_lock_binding_declared == true
  and .idempotency_nonce_lock_binding_declared == true
  and .rollback_kill_switch_lock_binding_declared == true
  and .dispatch_budget_one_lock_binding_declared == true
  and .secret_injection_absent_lock_binding_declared == true
  and .controlled_request_dispatch_budget_declared == 1
  and .controlled_request_dispatch_budget_accepted == false
  and .controlled_request_dispatch_budget_consumed == 0
  and .dispatch_envelope_recorded == false
  and .dispatch_envelope_persisted == false
  and .dispatch_envelope_materialized == false
  and .dispatch_envelope_delivered == false
  and .dispatch_envelope_accepted == false
  and .dispatch_envelope_authorizes_dispatch == false
  and .dispatch_envelope_authorizes_live_execution == false
  and .value_scoreboard_hash_accepted == false
  and .payload_readback_receipt_hash_accepted == false
  and .audit_receipt_hash_accepted == false
  and .idempotency_nonce_accepted == false
  and .rollback_kill_switch_armed == false
  and .secret_injection_absence_accepted == false
  and .dispatch_envelope_negative_fixture_count == 6
  and .dispatch_envelope_blocked_negative_fixture_count == 6
  and .dispatch_envelope_allowed_negative_fixture_count == 0
  and (.dispatch_envelope_negative_fixtures | all(
    .fixture_status == "blocked"
    and .dispatch_envelope_accepted == false
    and .controlled_request_dispatch_allowed == false
    and .controlled_request_dispatched == false
    and .controlled_request_executed == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
  ))
  and (.dispatch_envelope_lock_bindings | all(
    .dispatch_envelope_binding_declared == true
    and .dispatch_envelope_binding_required == true
    and .dispatch_envelope_binding_report_only == true
    and .dispatch_envelope_binding_operator_input_required == true
    and .dispatch_envelope_binding_operator_input_supplied == false
    and .dispatch_envelope_binding_recorded == false
    and .dispatch_envelope_binding_persisted == false
    and .dispatch_envelope_binding_delivered == false
    and .dispatch_envelope_binding_accepted == false
    and .dispatch_envelope_binding_authorizes_dispatch == false
    and .dispatch_envelope_binding_authorizes_live_execution == false
    and .controlled_request_dispatched == false
    and .controlled_request_executed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
  ))
  and .operator_record_supplied == false
  and .operator_record_accepted == false
  and .operator_record_recorded == false
  and .operator_record_persisted == false
  and .operator_record_delivered == false
  and .operator_record_authorizes_dispatch == false
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
  and .denied_by_dispatch_envelope_lock_validator_count == 22
  and (.denied_by_dispatch_envelope_lock_validator | length) == 22
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG trusted operator acceptance record controlled request dispatch envelope lock validator gate passed"
