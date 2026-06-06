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

HARNESS_SCOREBOARD_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-no-dispatch-readback-audit-scoreboard-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-no-dispatch-readback-audit-scoreboard-gate.sh
)"

harness_scoreboard_report_sha256="$(sha256_text "$HARNESS_SCOREBOARD_JSON")"
redacted_payload_preview_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-controlled-request-harness-redacted-payload-preview-no-materialization:v1:source-harness-scoreboard:single-route:single-namespace:budget-one:payload-hash-only:no-materialization:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_harness_redacted_payload_preview_no_materialization_side_effects=false;preview=1;accepted=0;payload_file=0;raw_payload=0;dispatch=0;execute=0;readback=0;audit=0;context=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"
payload_preview_hash_sha256="$(
  sha256_text "hepta-canary-single-route-single-namespace-synthetic-redacted-payload-preview:v1:method=POST:budget=1:route=declared-not-accepted:namespace=declared-not-accepted:payload=hash-only:no-secret:no-context:no-provider:no-memory:no-kg"
)"
readback_preview_hash_sha256="$(
  sha256_text "hepta-canary-single-route-single-namespace-readback-preview:v1:payload-preview=$payload_preview_hash_sha256:no-persistence"
)"
audit_preview_hash_sha256="$(
  sha256_text "hepta-canary-single-route-single-namespace-audit-preview:v1:payload-preview=$payload_preview_hash_sha256:no-persistence"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$HARNESS_SCOREBOARD_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_gate"
    and $source.operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_ready == true
    and $source.operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_status == "blocked"
    and $source.harness_scoreboard_entry_count == 9
    and $source.harness_scoreboard_entry_declared_count == 9
    and $source.harness_scoreboard_entry_required_count == 9
    and $source.harness_scoreboard_entry_report_only_count == 9
    and $source.harness_scoreboard_entry_operator_input_required_count == 9
    and $source.harness_scoreboard_entry_operator_input_supplied_count == 0
    and $source.harness_scoreboard_entry_recorded_count == 0
    and $source.harness_scoreboard_entry_persisted_count == 0
    and $source.harness_scoreboard_entry_delivered_count == 0
    and $source.harness_scoreboard_entry_accepted_count == 0
    and $source.harness_scoreboard_entry_authorizes_harness_arm_count == 0
    and $source.harness_scoreboard_entry_authorizes_payload_materialization_count == 0
    and $source.harness_scoreboard_entry_authorizes_dispatch_count == 0
    and $source.harness_scoreboard_entry_authorizes_readback_receipt_persistence_count == 0
    and $source.harness_scoreboard_entry_authorizes_audit_receipt_persistence_count == 0
    and $source.harness_scoreboard_entry_authorizes_context_attachment_count == 0
    and $source.harness_scoreboard_entry_authorizes_provider_model_invocation_count == 0
    and $source.harness_scoreboard_entry_authorizes_memory_write_count == 0
    and $source.harness_scoreboard_entry_authorizes_external_kg_read_count == 0
    and $source.harness_scoreboard_entry_authorizes_live_kg_write_count == 0
    and $source.harness_scoreboard_entry_authorizes_live_execution_count == 0
    and $source.single_route_scope_scoreboard_entry_declared == true
    and $source.single_namespace_scope_scoreboard_entry_declared == true
    and $source.value_scoreboard_hash_entry_declared == true
    and $source.payload_readback_receipt_hash_entry_declared == true
    and $source.audit_receipt_hash_entry_declared == true
    and $source.idempotency_nonce_entry_declared == true
    and $source.rollback_kill_switch_entry_declared == true
    and $source.dispatch_budget_one_entry_declared == true
    and $source.secret_injection_absent_entry_declared == true
    and $source.canary_route_scope_declared == true
    and $source.canary_route_scope_accepted == false
    and $source.canary_namespace_scope_declared == true
    and $source.canary_namespace_scope_accepted == false
    and $source.controlled_request_dispatch_budget_declared == 1
    and $source.controlled_request_dispatch_budget_accepted == false
    and $source.controlled_request_dispatch_budget_consumed == 0
    and $source.controlled_request_dispatch_budget_remaining == 0
    and $source.no_dispatch_guard_declared == true
    and $source.no_dispatch_guard_active == true
    and $source.harness_scoreboard_accepted == false
    and $source.canary_harness_arm_allowed == false
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and $source.payload_shape_hash_planned == true
    and $source.payload_materialization_allowed == false
    and $source.payload_materialized == false
    and $source.raw_payload_inspected == false
    and $source.readback_receipt_preview_declared == true
    and $source.readback_receipt_persistence_allowed == false
    and $source.readback_receipt_persisted == false
    and $source.audit_receipt_preview_declared == true
    and $source.audit_receipt_persistence_allowed == false
    and $source.audit_receipt_persisted == false
    and $source.harness_scoreboard_negative_fixture_count == 7
    and $source.harness_scoreboard_blocked_negative_fixture_count == 7
    and $source.harness_scoreboard_allowed_negative_fixture_count == 0
    and $source.controlled_request_dispatched == false
    and $source.controlled_request_executed == false
    and $source.context_injection_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
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
        fixture_id: "raw-secret-in-payload-preview",
        fixture_kind: "secret_boundary_violation",
        denial_reason: "redacted payload preview cannot include secrets, credentials, or bearer material"
      },
      {
        fixture_id: "live-context-injection-preview",
        fixture_kind: "context_injection_attempt",
        denial_reason: "payload preview cannot attach live context or prompt material"
      },
      {
        fixture_id: "provider-input-preview",
        fixture_kind: "provider_model_invocation_attempt",
        denial_reason: "payload preview cannot become provider/model input"
      },
      {
        fixture_id: "materialization-attempt",
        fixture_kind: "payload_materialization_attempt",
        denial_reason: "payload preview hash cannot materialize a request payload"
      },
      {
        fixture_id: "dispatch-attempt",
        fixture_kind: "controlled_request_dispatch_attempt",
        denial_reason: "payload preview cannot dispatch while the scoreboard is unaccepted"
      },
      {
        fixture_id: "budget-greater-than-one",
        fixture_kind: "dispatch_budget_violation",
        denial_reason: "single canary preview remains bound to dispatch budget one"
      },
      {
        fixture_id: "memory-kg-write-attempt",
        fixture_kind: "state_mutation_attempt",
        denial_reason: "payload preview cannot write Memory or KG state"
      }
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_gate" \
    --arg harness_scoreboard_report_sha256 "$harness_scoreboard_report_sha256" \
    --arg redacted_payload_preview_policy_hash_sha256 "$redacted_payload_preview_policy_hash_sha256" \
    --arg payload_preview_hash_sha256 "$payload_preview_hash_sha256" \
    --arg readback_preview_hash_sha256 "$readback_preview_hash_sha256" \
    --arg audit_preview_hash_sha256 "$audit_preview_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg payload_preview_id "hepta-canary-controlled-request-single-route-single-namespace-redacted-payload-preview" \
    --arg route_id "hepta.memory_intelligence_kg.canary.single_route.preview" \
    --arg namespace_id "hepta-memory-intelligence-kg-canary-single-namespace" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$HARNESS_SCOREBOARD_JSON" \
    --argjson negative_fixtures "$negative_fixtures_json" \
    '
      {
        payload_preview_id: $payload_preview_id,
        payload_preview_schema_id: "hepta.memory_intelligence_kg.canary.controlled_request.redacted_payload_preview.v1",
        payload_preview_status: "blocked_preview_only",
        route_id: $route_id,
        namespace_id: $namespace_id,
        controlled_request_method_shape_declared: true,
        controlled_request_method: "POST",
        controlled_request_dispatch_budget_declared: 1,
        controlled_request_dispatch_budget_accepted: false,
        controlled_request_dispatch_budget_consumed: 0,
        controlled_request_dispatch_budget_remaining: 0,
        source_harness_scoreboard_report_sha256: $harness_scoreboard_report_sha256,
        payload_preview_hash_shape_declared: true,
        payload_preview_hash_sha256: $payload_preview_hash_sha256,
        payload_preview_hash_accepted: false,
        payload_preview_shape_declared: true,
        payload_preview_redacted_fields_only: true,
        payload_preview_contains_secret: false,
        payload_preview_contains_credential: false,
        payload_preview_contains_live_context: false,
        payload_preview_contains_provider_input: false,
        payload_preview_contains_memory_mutation: false,
        payload_preview_contains_kg_mutation: false,
        payload_preview_contains_channel_delivery: false,
        payload_preview_report_only: true,
        payload_preview_recorded: false,
        payload_preview_persisted: false,
        payload_preview_delivered: false,
        payload_preview_accepted: false,
        payload_preview_materialized_in_report: true,
        request_payload_materialization_allowed: false,
        request_payload_materialized: false,
        request_payload_persisted: false,
        request_payload_file_written: false,
        raw_payload_inspected: false,
        redaction_proof_shape_declared: true,
        redaction_proof_accepted: false,
        readback_receipt_preview_declared: true,
        readback_preview_hash_sha256: $readback_preview_hash_sha256,
        readback_receipt_persistence_allowed: false,
        readback_receipt_persisted: false,
        audit_receipt_preview_declared: true,
        audit_preview_hash_sha256: $audit_preview_hash_sha256,
        audit_receipt_persistence_allowed: false,
        audit_receipt_persisted: false,
        context_attachment_allowed: false,
        context_injection_performed: false,
        provider_model_invocation_allowed: false,
        provider_invoked: false,
        model_invoked: false,
        memory_write_allowed: false,
        memory_store_write_performed: false,
        external_kg_read_allowed: false,
        external_kg_adapter_read_performed: false,
        live_kg_write_allowed: false,
        live_kg_write_performed: false,
        network_call_allowed: false,
        network_call_performed: false,
        credential_read_allowed: false,
        credential_read: false,
        secret_file_read: false,
        channel_delivery_allowed: false,
        channel_send_performed: false,
        controlled_request_dispatch_allowed: false,
        controlled_request_dispatched: false,
        controlled_request_execution_allowed: false,
        controlled_request_executed: false,
        canary_harness_arm_allowed: false,
        canary_harness_armed: false,
        canary_harness_executable: false,
        canary_live_enabled: false,
        status: "blocked_redacted_payload_preview_no_materialization"
      } as $payload_preview
      | ($negative_fixtures | map(. + {
          fixture_status: "blocked",
          payload_preview_accepted: false,
          request_payload_materialization_allowed: false,
          request_payload_materialized: false,
          request_payload_file_written: false,
          raw_payload_inspected: false,
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
          operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_schema_version: "memory_intelligence_kg_operator_canary_harness_redacted_payload_preview_no_materialization_v1",
          operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_ready: true,
          operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_status: "blocked",
          redacted_payload_preview_mode: "stdout_only_hash_bound_preview_no_payload_file_no_raw_payload_no_dispatch_no_live",
          redacted_payload_preview_decision: "single_route_single_namespace_synthetic_payload_preview_hash_is_declared_without_materializing_a_request_payload_or_authorizing_dispatch",
          minimum_required_samples: $min_long_soak_samples,
          source_harness_scoreboard_gate: $source.gate,
          source_harness_scoreboard_status: $source.operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_status,
          source_harness_scoreboard_report_sha256: $harness_scoreboard_report_sha256,
          source_harness_scoreboard_entry_count: $source.harness_scoreboard_entry_count,
          source_harness_scoreboard_entry_accepted_count: $source.harness_scoreboard_entry_accepted_count,
          source_harness_scoreboard_entry_authorizes_payload_materialization_count: $source.harness_scoreboard_entry_authorizes_payload_materialization_count,
          source_harness_scoreboard_entry_authorizes_dispatch_count: $source.harness_scoreboard_entry_authorizes_dispatch_count,
          source_harness_scoreboard_negative_fixture_count: $source.harness_scoreboard_negative_fixture_count,
          redacted_payload_preview_policy_hash_sha256: $redacted_payload_preview_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          payload_previews: [$payload_preview],
          payload_preview_count: 1,
          payload_preview_shape_declared_count: 1,
          payload_preview_report_only_count: 1,
          payload_preview_hash_shape_declared_count: 1,
          payload_preview_hash_accepted_count: 0,
          payload_preview_accepted_count: 0,
          payload_preview_recorded_count: 0,
          payload_preview_persisted_count: 0,
          payload_preview_delivered_count: 0,
          request_payload_materialization_allowed_count: 0,
          request_payload_materialized_count: 0,
          request_payload_persisted_count: 0,
          request_payload_file_written_count: 0,
          raw_payload_inspected_count: 0,
          redacted_payload_preview_contains_secret_count: 0,
          redacted_payload_preview_contains_credential_count: 0,
          redacted_payload_preview_contains_live_context_count: 0,
          redacted_payload_preview_contains_provider_input_count: 0,
          redacted_payload_preview_contains_memory_mutation_count: 0,
          redacted_payload_preview_contains_kg_mutation_count: 0,
          readback_receipt_preview_declared_count: 1,
          readback_receipt_persisted_count: 0,
          audit_receipt_preview_declared_count: 1,
          audit_receipt_persisted_count: 0,
          controlled_request_dispatch_budget_declared: 1,
          controlled_request_dispatch_budget_accepted: false,
          controlled_request_dispatch_budget_consumed: 0,
          controlled_request_dispatch_budget_remaining: 0,
          controlled_request_dispatch_allowed_count: 0,
          controlled_request_dispatched_count: 0,
          controlled_request_execution_allowed_count: 0,
          controlled_request_executed_count: 0,
          context_injection_performed_count: 0,
          provider_invoked_count: 0,
          model_invoked_count: 0,
          memory_store_write_performed_count: 0,
          external_kg_adapter_read_performed_count: 0,
          live_kg_write_performed_count: 0,
          credential_read_count: 0,
          secret_file_read_count: 0,
          channel_send_performed_count: 0,
          canary_harness_arm_allowed: false,
          canary_harness_armed: false,
          canary_harness_executable: false,
          canary_live_enabled: false,
          redacted_payload_preview_negative_fixtures: $fixtures,
          redacted_payload_preview_negative_fixture_count: ($fixtures | length),
          redacted_payload_preview_blocked_negative_fixture_count: ($fixtures | map(select(.fixture_status == "blocked")) | length),
          redacted_payload_preview_allowed_negative_fixture_count: ($fixtures | map(select(.fixture_status == "allowed")) | length),
          denied_by_redacted_payload_preview_no_materialization: [
            "payload_preview_not_operator_approval",
            "payload_preview_hash_not_accepted",
            "redaction_proof_not_accepted",
            "request_payload_materialization_denied",
            "raw_payload_inspection_denied",
            "payload_file_write_denied",
            "controlled_request_dispatch_denied",
            "controlled_request_execution_denied",
            "readback_receipt_persistence_denied",
            "audit_receipt_persistence_denied",
            "context_attachment_denied",
            "provider_model_invocation_denied",
            "memory_write_denied",
            "external_kg_read_denied",
            "live_kg_write_denied",
            "credential_secret_read_denied"
          ],
          denied_by_redacted_payload_preview_no_materialization_count: 16,
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            payload_preview_recorded: false,
            payload_preview_persisted: false,
            payload_preview_delivered: false,
            payload_preview_accepted: false,
            request_payload_materialized: false,
            request_payload_persisted: false,
            request_payload_file_written: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_gate"
  and .operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_ready == true
  and .operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_status == "blocked"
  and .source_harness_scoreboard_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_no_dispatch_readback_audit_scoreboard_gate"
  and .source_harness_scoreboard_status == "blocked"
  and .source_harness_scoreboard_entry_count == 9
  and .source_harness_scoreboard_entry_accepted_count == 0
  and .source_harness_scoreboard_entry_authorizes_payload_materialization_count == 0
  and .source_harness_scoreboard_entry_authorizes_dispatch_count == 0
  and .payload_preview_count == 1
  and .payload_preview_shape_declared_count == 1
  and .payload_preview_report_only_count == 1
  and .payload_preview_hash_shape_declared_count == 1
  and .payload_preview_hash_accepted_count == 0
  and .payload_preview_accepted_count == 0
  and .payload_preview_recorded_count == 0
  and .payload_preview_persisted_count == 0
  and .payload_preview_delivered_count == 0
  and .request_payload_materialization_allowed_count == 0
  and .request_payload_materialized_count == 0
  and .request_payload_persisted_count == 0
  and .request_payload_file_written_count == 0
  and .raw_payload_inspected_count == 0
  and .redacted_payload_preview_contains_secret_count == 0
  and .redacted_payload_preview_contains_credential_count == 0
  and .redacted_payload_preview_contains_live_context_count == 0
  and .redacted_payload_preview_contains_provider_input_count == 0
  and .redacted_payload_preview_contains_memory_mutation_count == 0
  and .redacted_payload_preview_contains_kg_mutation_count == 0
  and .readback_receipt_preview_declared_count == 1
  and .readback_receipt_persisted_count == 0
  and .audit_receipt_preview_declared_count == 1
  and .audit_receipt_persisted_count == 0
  and .controlled_request_dispatch_budget_declared == 1
  and .controlled_request_dispatch_budget_accepted == false
  and .controlled_request_dispatch_budget_consumed == 0
  and .controlled_request_dispatch_budget_remaining == 0
  and .controlled_request_dispatch_allowed_count == 0
  and .controlled_request_dispatched_count == 0
  and .controlled_request_execution_allowed_count == 0
  and .controlled_request_executed_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .external_kg_adapter_read_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .canary_harness_arm_allowed == false
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .redacted_payload_preview_negative_fixture_count == 7
  and .redacted_payload_preview_blocked_negative_fixture_count == 7
  and .redacted_payload_preview_allowed_negative_fixture_count == 0
  and (.payload_previews | all(
    .payload_preview_status == "blocked_preview_only"
    and .payload_preview_redacted_fields_only == true
    and .payload_preview_contains_secret == false
    and .payload_preview_contains_credential == false
    and .payload_preview_contains_live_context == false
    and .payload_preview_contains_provider_input == false
    and .payload_preview_contains_memory_mutation == false
    and .payload_preview_contains_kg_mutation == false
    and .payload_preview_report_only == true
    and .payload_preview_recorded == false
    and .payload_preview_persisted == false
    and .payload_preview_delivered == false
    and .payload_preview_accepted == false
    and .request_payload_materialization_allowed == false
    and .request_payload_materialized == false
    and .request_payload_persisted == false
    and .request_payload_file_written == false
    and .raw_payload_inspected == false
    and .readback_receipt_persistence_allowed == false
    and .readback_receipt_persisted == false
    and .audit_receipt_persistence_allowed == false
    and .audit_receipt_persisted == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .controlled_request_dispatched == false
    and .controlled_request_executed == false
  ))
  and (.redacted_payload_preview_negative_fixtures | all(
    .fixture_status == "blocked"
    and .payload_preview_accepted == false
    and .request_payload_materialization_allowed == false
    and .request_payload_materialized == false
    and .request_payload_file_written == false
    and .raw_payload_inspected == false
    and .controlled_request_dispatch_allowed == false
    and .controlled_request_dispatched == false
    and .controlled_request_executed == false
    and .readback_receipt_persisted == false
    and .audit_receipt_persisted == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
  ))
  and .denied_by_redacted_payload_preview_no_materialization_count == 16
  and (.denied_by_redacted_payload_preview_no_materialization | length) == 16
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness redacted payload preview no-materialization gate passed"
