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

DISPATCH_ENVELOPE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-dispatch-envelope-lock-validator-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-controlled-request-dispatch-envelope-lock-validator-gate.sh
)"

dispatch_envelope_report_sha256="$(sha256_text "$DISPATCH_ENVELOPE_JSON")"
harness_scoreboard_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-controlled-request-harness-no-dispatch-readback-audit-scoreboard:v1:source-dispatch-envelope-locks:9-bindings:single-route:single-namespace:readback:audit:no-dispatch:no-payload:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_side_effects=false;bindings=9;scoreboard=9;fixtures=7;route=0;namespace=0;payload=0;dispatch=0;execute=0;readback=0;audit=0;context=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$DISPATCH_ENVELOPE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_gate"
    and $source.operator_canary_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_ready == true
    and $source.operator_canary_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_status == "blocked"
    and $source.dispatch_envelope_lock_binding_count == 9
    and $source.dispatch_envelope_lock_binding_declared_count == 9
    and $source.dispatch_envelope_lock_binding_required_count == 9
    and $source.dispatch_envelope_lock_binding_report_only_count == 9
    and $source.dispatch_envelope_lock_binding_operator_input_required_count == 9
    and $source.dispatch_envelope_lock_binding_operator_input_supplied_count == 0
    and $source.dispatch_envelope_lock_binding_recorded_count == 0
    and $source.dispatch_envelope_lock_binding_persisted_count == 0
    and $source.dispatch_envelope_lock_binding_delivered_count == 0
    and $source.dispatch_envelope_lock_binding_accepted_count == 0
    and $source.dispatch_envelope_lock_binding_authorizes_dispatch_count == 0
    and $source.dispatch_envelope_lock_binding_authorizes_context_attachment_count == 0
    and $source.dispatch_envelope_lock_binding_authorizes_provider_model_invocation_count == 0
    and $source.dispatch_envelope_lock_binding_authorizes_memory_write_count == 0
    and $source.dispatch_envelope_lock_binding_authorizes_external_kg_read_count == 0
    and $source.dispatch_envelope_lock_binding_authorizes_live_kg_write_count == 0
    and $source.dispatch_envelope_lock_binding_authorizes_live_execution_count == 0
    and $source.route_scope_lock_binding_declared == true
    and $source.namespace_scope_lock_binding_declared == true
    and $source.payload_readback_receipt_hash_lock_binding_declared == true
    and $source.audit_receipt_hash_lock_binding_declared == true
    and $source.dispatch_budget_one_lock_binding_declared == true
    and $source.secret_injection_absent_lock_binding_declared == true
    and $source.controlled_request_dispatch_budget_declared == 1
    and $source.controlled_request_dispatch_budget_accepted == false
    and $source.controlled_request_dispatch_budget_consumed == 0
    and $source.dispatch_envelope_recorded == false
    and $source.dispatch_envelope_persisted == false
    and $source.dispatch_envelope_materialized == false
    and $source.dispatch_envelope_delivered == false
    and $source.dispatch_envelope_accepted == false
    and $source.dispatch_envelope_authorizes_dispatch == false
    and $source.dispatch_envelope_authorizes_live_execution == false
    and $source.operator_record_accepted == false
    and $source.canary_harness_activation_ready == false
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and $source.controlled_request_dispatched == false
    and $source.controlled_request_executed == false
    and $source.context_injection_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.memory_store_write_performed == false
    and $source.external_kg_adapter_read_performed == false
    and $source.live_kg_write_performed == false
    and $source.credential_read == false
    and $source.secret_file_read == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

negative_fixtures_json="$(
  jq -n '
    [
      {
        fixture_id: "route-scope-unaccepted",
        fixture_kind: "canary_route_scope_missing_or_unaccepted",
        missing_scoreboard_entry: "single-route-scope-lock",
        requested_dispatch_budget: 1,
        denial_reason: "canary route must be accepted before harness arm or dispatch preview"
      },
      {
        fixture_id: "namespace-scope-unaccepted",
        fixture_kind: "canary_namespace_scope_missing_or_unaccepted",
        missing_scoreboard_entry: "single-namespace-scope-lock",
        requested_dispatch_budget: 1,
        denial_reason: "canary namespace must be accepted before harness arm or dispatch preview"
      },
      {
        fixture_id: "payload-readback-hash-unaccepted",
        fixture_kind: "readback_receipt_hash_missing_or_unaccepted",
        missing_scoreboard_entry: "payload-readback-receipt-hash-lock",
        requested_dispatch_budget: 1,
        denial_reason: "payload readback receipt hash must be accepted before any request leaves preview"
      },
      {
        fixture_id: "audit-receipt-hash-unaccepted",
        fixture_kind: "audit_receipt_hash_missing_or_unaccepted",
        missing_scoreboard_entry: "audit-receipt-hash-lock",
        requested_dispatch_budget: 1,
        denial_reason: "audit receipt hash must be accepted before any request leaves preview"
      },
      {
        fixture_id: "budget-not-exactly-one",
        fixture_kind: "dispatch_budget_violation",
        missing_scoreboard_entry: "dispatch-budget-one-lock",
        requested_dispatch_budget: 2,
        denial_reason: "controlled request canary budget must remain exactly one"
      },
      {
        fixture_id: "dispatch-attempt-before-acceptance",
        fixture_kind: "premature_dispatch_attempt",
        missing_scoreboard_entry: null,
        requested_dispatch_budget: 1,
        denial_reason: "no-dispatch harness blocks dispatch until every scoreboard entry is accepted"
      },
      {
        fixture_id: "secret-injection-attempt",
        fixture_kind: "secret_boundary_violation",
        missing_scoreboard_entry: "secret-injection-absent-lock",
        requested_dispatch_budget: 1,
        denial_reason: "controlled request harness cannot carry secrets or credential injection"
      }
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_gate" \
    --arg source_dispatch_envelope_report_sha256 "$dispatch_envelope_report_sha256" \
    --arg harness_scoreboard_policy_hash_sha256 "$harness_scoreboard_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$DISPATCH_ENVELOPE_JSON" \
    --argjson negative_fixtures "$negative_fixtures_json" \
    '
      ($source.dispatch_envelope_lock_bindings | map({
        harness_scoreboard_entry_id: ("canary-harness-scoreboard-" + .lock_id),
        lock_id,
        lock_family,
        source_precondition_id,
        source_dispatch_envelope_lock_binding_id: .dispatch_envelope_lock_binding_id,
        source_dispatch_envelope_report_sha256: $source_dispatch_envelope_report_sha256,
        harness_scoreboard_entry_declared: true,
        harness_scoreboard_entry_required: true,
        harness_scoreboard_entry_report_only: true,
        harness_scoreboard_entry_operator_input_required: .dispatch_envelope_binding_operator_input_required,
        harness_scoreboard_entry_operator_input_supplied: false,
        harness_scoreboard_entry_recorded: false,
        harness_scoreboard_entry_persisted: false,
        harness_scoreboard_entry_delivered: false,
        harness_scoreboard_entry_accepted: false,
        harness_scoreboard_entry_authorizes_harness_arm: false,
        harness_scoreboard_entry_authorizes_payload_materialization: false,
        harness_scoreboard_entry_authorizes_dispatch: false,
        harness_scoreboard_entry_authorizes_readback_receipt_persistence: false,
        harness_scoreboard_entry_authorizes_audit_receipt_persistence: false,
        harness_scoreboard_entry_authorizes_context_attachment: false,
        harness_scoreboard_entry_authorizes_provider_model_invocation: false,
        harness_scoreboard_entry_authorizes_memory_write: false,
        harness_scoreboard_entry_authorizes_external_kg_read: false,
        harness_scoreboard_entry_authorizes_live_kg_write: false,
        harness_scoreboard_entry_authorizes_live_execution: false,
        canary_harness_armed: false,
        payload_materialized: false,
        controlled_request_dispatched: false,
        controlled_request_executed: false,
        readback_receipt_persisted: false,
        audit_receipt_persisted: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        memory_store_write_performed: false,
        external_kg_adapter_read_performed: false,
        live_kg_write_performed: false,
        credential_read: false,
        secret_file_read: false,
        status: "blocked_until_trusted_operator_scoreboard_acceptance"
      })) as $scoreboard_entries
      | ($negative_fixtures | map(. + {
          fixture_status: "blocked",
          harness_scoreboard_accepted: false,
          canary_harness_arm_allowed: false,
          canary_harness_armed: false,
          payload_materialization_allowed: false,
          payload_materialized: false,
          controlled_request_dispatch_allowed: false,
          controlled_request_dispatched: false,
          controlled_request_executed: false,
          readback_receipt_persisted: false,
          audit_receipt_persisted: false,
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
          operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_schema_version: "memory_intelligence_kg_operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_v1",
          operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_ready: true,
          operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_status: "blocked",
          harness_scoreboard_mode: "stdout_only_report_only_single_route_single_namespace_no_dispatch_no_payload_no_live",
          harness_scoreboard_decision: "single_route_single_namespace_canary_harness_shape_is_declared_but_no_dispatch_payload_readback_audit_or_live_work_can_occur_until_the_scoreboard_is_accepted",
          min_long_soak_samples: $min_long_soak_samples,
          source_dispatch_envelope_lock_validator_gate: $source.gate,
          source_dispatch_envelope_lock_validator_status: $source.operator_canary_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_status,
          source_dispatch_envelope_report_sha256: $source_dispatch_envelope_report_sha256,
          source_dispatch_envelope_lock_binding_count: $source.dispatch_envelope_lock_binding_count,
          source_dispatch_envelope_accepted: $source.dispatch_envelope_accepted,
          source_dispatch_envelope_authorizes_dispatch: $source.dispatch_envelope_authorizes_dispatch,
          source_controlled_request_dispatch_budget_declared: $source.controlled_request_dispatch_budget_declared,
          source_controlled_request_dispatch_budget_accepted: $source.controlled_request_dispatch_budget_accepted,
          harness_scoreboard_policy_hash_sha256: $harness_scoreboard_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          harness_scoreboard_entries: $scoreboard_entries,
          harness_scoreboard_entry_count: ($scoreboard_entries | length),
          harness_scoreboard_entry_declared_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_declared == true)) | length),
          harness_scoreboard_entry_required_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_required == true)) | length),
          harness_scoreboard_entry_report_only_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_report_only == true)) | length),
          harness_scoreboard_entry_operator_input_required_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_operator_input_required == true)) | length),
          harness_scoreboard_entry_operator_input_supplied_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_operator_input_supplied == true)) | length),
          harness_scoreboard_entry_recorded_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_recorded == true)) | length),
          harness_scoreboard_entry_persisted_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_persisted == true)) | length),
          harness_scoreboard_entry_delivered_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_delivered == true)) | length),
          harness_scoreboard_entry_accepted_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_accepted == true)) | length),
          harness_scoreboard_entry_authorizes_harness_arm_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_authorizes_harness_arm == true)) | length),
          harness_scoreboard_entry_authorizes_payload_materialization_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_authorizes_payload_materialization == true)) | length),
          harness_scoreboard_entry_authorizes_dispatch_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_authorizes_dispatch == true)) | length),
          harness_scoreboard_entry_authorizes_readback_receipt_persistence_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_authorizes_readback_receipt_persistence == true)) | length),
          harness_scoreboard_entry_authorizes_audit_receipt_persistence_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_authorizes_audit_receipt_persistence == true)) | length),
          harness_scoreboard_entry_authorizes_context_attachment_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_authorizes_context_attachment == true)) | length),
          harness_scoreboard_entry_authorizes_provider_model_invocation_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_authorizes_provider_model_invocation == true)) | length),
          harness_scoreboard_entry_authorizes_memory_write_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_authorizes_memory_write == true)) | length),
          harness_scoreboard_entry_authorizes_external_kg_read_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_authorizes_external_kg_read == true)) | length),
          harness_scoreboard_entry_authorizes_live_kg_write_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_authorizes_live_kg_write == true)) | length),
          harness_scoreboard_entry_authorizes_live_execution_count: ($scoreboard_entries | map(select(.harness_scoreboard_entry_authorizes_live_execution == true)) | length),
          single_route_scope_scoreboard_entry_declared: ($scoreboard_entries | any(.lock_id == "single-route-scope-lock" and .harness_scoreboard_entry_declared == true)),
          single_namespace_scope_scoreboard_entry_declared: ($scoreboard_entries | any(.lock_id == "single-namespace-scope-lock" and .harness_scoreboard_entry_declared == true)),
          value_scoreboard_hash_entry_declared: ($scoreboard_entries | any(.lock_id == "value-scoreboard-hash-lock" and .harness_scoreboard_entry_declared == true)),
          payload_readback_receipt_hash_entry_declared: ($scoreboard_entries | any(.lock_id == "payload-readback-receipt-hash-lock" and .harness_scoreboard_entry_declared == true)),
          audit_receipt_hash_entry_declared: ($scoreboard_entries | any(.lock_id == "audit-receipt-hash-lock" and .harness_scoreboard_entry_declared == true)),
          idempotency_nonce_entry_declared: ($scoreboard_entries | any(.lock_id == "idempotency-nonce-current-unused-lock" and .harness_scoreboard_entry_declared == true)),
          rollback_kill_switch_entry_declared: ($scoreboard_entries | any(.lock_id == "rollback-kill-switch-lock" and .harness_scoreboard_entry_declared == true)),
          dispatch_budget_one_entry_declared: ($scoreboard_entries | any(.lock_id == "dispatch-budget-one-lock" and .harness_scoreboard_entry_declared == true)),
          secret_injection_absent_entry_declared: ($scoreboard_entries | any(.lock_id == "secret-injection-absent-lock" and .harness_scoreboard_entry_declared == true)),
          canary_route_scope_declared: true,
          canary_route_scope_accepted: false,
          canary_namespace_scope_declared: true,
          canary_namespace_scope_accepted: false,
          controlled_request_dispatch_budget_declared: 1,
          controlled_request_dispatch_budget_accepted: false,
          controlled_request_dispatch_budget_consumed: 0,
          controlled_request_dispatch_budget_remaining: 0,
          no_dispatch_guard_declared: true,
          no_dispatch_guard_active: true,
          dispatch_envelope_shape_bound: true,
          dispatch_envelope_accepted: false,
          harness_scoreboard_accepted: false,
          canary_harness_shape_declared: true,
          canary_harness_arm_allowed: false,
          canary_harness_armed: false,
          canary_harness_executable: false,
          canary_live_enabled: false,
          payload_shape_hash_planned: true,
          payload_materialization_allowed: false,
          payload_materialized: false,
          raw_payload_inspected: false,
          readback_receipt_preview_declared: true,
          readback_receipt_persistence_allowed: false,
          readback_receipt_persisted: false,
          audit_receipt_preview_declared: true,
          audit_receipt_persistence_allowed: false,
          audit_receipt_persisted: false,
          dispatch_envelope_negative_fixtures: $fixtures,
          harness_scoreboard_negative_fixture_count: ($fixtures | length),
          harness_scoreboard_blocked_negative_fixture_count: ($fixtures | map(select(.fixture_status == "blocked")) | length),
          harness_scoreboard_allowed_negative_fixture_count: ($fixtures | map(select(.fixture_status == "allowed")) | length),
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
          denied_by_harness_no_dispatch_scoreboard: [
            "harness_scoreboard_not_operator_approval",
            "canary_route_scope_acceptance_missing",
            "canary_namespace_scope_acceptance_missing",
            "value_scoreboard_hash_acceptance_missing",
            "payload_readback_hash_acceptance_missing",
            "audit_receipt_hash_acceptance_missing",
            "idempotency_nonce_acceptance_missing",
            "rollback_kill_switch_acceptance_missing",
            "dispatch_budget_one_acceptance_missing",
            "secret_injection_absence_acceptance_missing",
            "canary_harness_arm_denied",
            "payload_materialization_denied",
            "controlled_request_dispatch_denied",
            "controlled_request_execution_denied",
            "readback_receipt_persistence_denied",
            "audit_receipt_persistence_denied",
            "context_attachment_denied",
            "provider_model_invocation_denied",
            "memory_write_denied",
            "external_kg_read_denied",
            "live_kg_write_denied",
            "secret_credential_read_denied"
          ],
          denied_by_harness_no_dispatch_scoreboard_count: 22,
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            harness_scoreboard_recorded: false,
            harness_scoreboard_persisted: false,
            harness_scoreboard_delivered: false,
            harness_scoreboard_accepted: false,
            canary_harness_armed: false,
            canary_live_enabled: false,
            payload_materialized: false,
            raw_payload_inspected: false,
            controlled_request_dispatched: false,
            controlled_request_executed: false,
            readback_receipt_persisted: false,
            audit_receipt_persisted: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_gate"
  and .operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_ready == true
  and .operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_status == "blocked"
  and .harness_scoreboard_mode == "stdout_only_report_only_single_route_single_namespace_no_dispatch_no_payload_no_live"
  and .source_dispatch_envelope_lock_validator_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_controlled_request_dispatch_envelope_lock_validator_gate"
  and .source_dispatch_envelope_lock_validator_status == "blocked"
  and .source_dispatch_envelope_lock_binding_count == 9
  and .source_dispatch_envelope_accepted == false
  and .source_dispatch_envelope_authorizes_dispatch == false
  and .source_controlled_request_dispatch_budget_declared == 1
  and .source_controlled_request_dispatch_budget_accepted == false
  and .harness_scoreboard_entry_count == 9
  and .harness_scoreboard_entry_declared_count == 9
  and .harness_scoreboard_entry_required_count == 9
  and .harness_scoreboard_entry_report_only_count == 9
  and .harness_scoreboard_entry_operator_input_required_count == 9
  and .harness_scoreboard_entry_operator_input_supplied_count == 0
  and .harness_scoreboard_entry_recorded_count == 0
  and .harness_scoreboard_entry_persisted_count == 0
  and .harness_scoreboard_entry_delivered_count == 0
  and .harness_scoreboard_entry_accepted_count == 0
  and .harness_scoreboard_entry_authorizes_harness_arm_count == 0
  and .harness_scoreboard_entry_authorizes_payload_materialization_count == 0
  and .harness_scoreboard_entry_authorizes_dispatch_count == 0
  and .harness_scoreboard_entry_authorizes_readback_receipt_persistence_count == 0
  and .harness_scoreboard_entry_authorizes_audit_receipt_persistence_count == 0
  and .harness_scoreboard_entry_authorizes_context_attachment_count == 0
  and .harness_scoreboard_entry_authorizes_provider_model_invocation_count == 0
  and .harness_scoreboard_entry_authorizes_memory_write_count == 0
  and .harness_scoreboard_entry_authorizes_external_kg_read_count == 0
  and .harness_scoreboard_entry_authorizes_live_kg_write_count == 0
  and .harness_scoreboard_entry_authorizes_live_execution_count == 0
  and .single_route_scope_scoreboard_entry_declared == true
  and .single_namespace_scope_scoreboard_entry_declared == true
  and .value_scoreboard_hash_entry_declared == true
  and .payload_readback_receipt_hash_entry_declared == true
  and .audit_receipt_hash_entry_declared == true
  and .idempotency_nonce_entry_declared == true
  and .rollback_kill_switch_entry_declared == true
  and .dispatch_budget_one_entry_declared == true
  and .secret_injection_absent_entry_declared == true
  and .canary_route_scope_declared == true
  and .canary_route_scope_accepted == false
  and .canary_namespace_scope_declared == true
  and .canary_namespace_scope_accepted == false
  and .controlled_request_dispatch_budget_declared == 1
  and .controlled_request_dispatch_budget_accepted == false
  and .controlled_request_dispatch_budget_consumed == 0
  and .controlled_request_dispatch_budget_remaining == 0
  and .no_dispatch_guard_declared == true
  and .no_dispatch_guard_active == true
  and .dispatch_envelope_shape_bound == true
  and .dispatch_envelope_accepted == false
  and .harness_scoreboard_accepted == false
  and .canary_harness_shape_declared == true
  and .canary_harness_arm_allowed == false
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .payload_shape_hash_planned == true
  and .payload_materialization_allowed == false
  and .payload_materialized == false
  and .raw_payload_inspected == false
  and .readback_receipt_preview_declared == true
  and .readback_receipt_persistence_allowed == false
  and .readback_receipt_persisted == false
  and .audit_receipt_preview_declared == true
  and .audit_receipt_persistence_allowed == false
  and .audit_receipt_persisted == false
  and .harness_scoreboard_negative_fixture_count == 7
  and .harness_scoreboard_blocked_negative_fixture_count == 7
  and .harness_scoreboard_allowed_negative_fixture_count == 0
  and (.dispatch_envelope_negative_fixtures | all(
    .fixture_status == "blocked"
    and .harness_scoreboard_accepted == false
    and .canary_harness_arm_allowed == false
    and .canary_harness_armed == false
    and .payload_materialization_allowed == false
    and .payload_materialized == false
    and .controlled_request_dispatch_allowed == false
    and .controlled_request_dispatched == false
    and .controlled_request_executed == false
    and .readback_receipt_persisted == false
    and .audit_receipt_persisted == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
  ))
  and (.harness_scoreboard_entries | all(
    .harness_scoreboard_entry_declared == true
    and .harness_scoreboard_entry_required == true
    and .harness_scoreboard_entry_report_only == true
    and .harness_scoreboard_entry_operator_input_required == true
    and .harness_scoreboard_entry_operator_input_supplied == false
    and .harness_scoreboard_entry_recorded == false
    and .harness_scoreboard_entry_persisted == false
    and .harness_scoreboard_entry_delivered == false
    and .harness_scoreboard_entry_accepted == false
    and .harness_scoreboard_entry_authorizes_harness_arm == false
    and .harness_scoreboard_entry_authorizes_payload_materialization == false
    and .harness_scoreboard_entry_authorizes_dispatch == false
    and .harness_scoreboard_entry_authorizes_live_execution == false
    and .canary_harness_armed == false
    and .payload_materialized == false
    and .controlled_request_dispatched == false
    and .controlled_request_executed == false
    and .readback_receipt_persisted == false
    and .audit_receipt_persisted == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
  ))
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
  and .denied_by_harness_no_dispatch_scoreboard_count == 22
  and (.denied_by_harness_no_dispatch_scoreboard | length) == 22
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness no-dispatch readback/audit scoreboard gate passed"
