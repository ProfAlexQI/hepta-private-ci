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

TEMPLATE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-template-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-template-gate.sh
)"

template_report_sha256="$(sha256_text "$TEMPLATE_JSON")"
readiness_lock_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-trusted-operator-acceptance-record-readiness-lock:v1:source-template:9-locks:no-record:no-accept:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_trusted_operator_acceptance_record_readiness_lock_side_effects=false;locks=9;accepted=0;recorded=0;persisted=0;dispatch=0;context=0;provider=0;model=0;memory=0;kg=0;secret=0;restart=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TEMPLATE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_template_gate"
    and $source.operator_canary_trusted_operator_acceptance_record_template_ready == true
    and $source.operator_canary_trusted_operator_acceptance_record_template_status == "blocked"
    and $source.template_mode == "stdout_only_report_only_template_no_operator_record_no_acceptance_no_dispatch_no_live"
    and $source.source_positive_precondition_count == 12
    and $source.source_missing_positive_precondition_count == 12
    and $source.source_accepted_positive_precondition_count == 0
    and $source.required_template_section_count == 12
    and $source.operator_record_template_section_count == 12
    and $source.rendered_template_section_count == 12
    and $source.missing_operator_input_section_count == 12
    and $source.satisfied_template_section_count == 0
    and $source.accepted_template_section_count == 0
    and $source.required_operator_record_field_count == 36
    and $source.unique_operator_record_field_count == 36
    and $source.supplied_operator_record_field_count == 0
    and $source.trusted_operator_record_field_count == 0
    and $source.accepted_operator_record_field_count == 0
    and ($source.operator_record_template_sections | any(.precondition_id == "route-scope-matches-canary" and .template_section_id == "route-scope"))
    and ($source.operator_record_template_sections | any(.precondition_id == "namespace-scope-matches-canary" and .template_section_id == "namespace-scope"))
    and ($source.operator_record_template_sections | any(.precondition_id == "value-scoreboard-hash-matches" and .template_section_id == "value-scoreboard-binding"))
    and ($source.operator_record_template_sections | any(.precondition_id == "readback-receipt-hash-matches" and .template_section_id == "readback-receipt-binding"))
    and ($source.operator_record_template_sections | any(.precondition_id == "audit-receipt-hash-matches" and .template_section_id == "audit-receipt-binding"))
    and ($source.operator_record_template_sections | any(.precondition_id == "idempotency-nonce-current-and-unused" and .template_section_id == "idempotency-nonce"))
    and ($source.operator_record_template_sections | any(.precondition_id == "rollback-plan-and-kill-switch-present" and .template_section_id == "rollback-kill-switch"))
    and ($source.operator_record_template_sections | any(.precondition_id == "dispatch-budget-equals-one" and .template_section_id == "dispatch-budget"))
    and ($source.operator_record_template_sections | any(.precondition_id == "secret-injection-absent" and .template_section_id == "secret-boundary"))
    and ($source.operator_record_template_sections | all(
      .template_section_status == "missing_operator_input"
      and .operator_input_required == true
      and .template_only == true
      and .report_only == true
      and .record_field_supplied_count == 0
      and .record_field_trusted_count == 0
      and .record_field_accepted_count == 0
      and .section_satisfied == false
      and .section_accepted == false
      and .record_section_recorded == false
      and .record_section_persisted == false
      and .record_section_delivered == false
      and .authorizes_canary_dispatch == false
      and .authorizes_context_attachment == false
      and .authorizes_provider_model_invocation == false
      and .authorizes_memory_write == false
      and .authorizes_external_kg_read == false
      and .authorizes_live_kg_write == false
      and .authorizes_live_execution == false
    ))
    and $source.operator_record_template_rendered == true
    and $source.operator_record_supplied == false
    and $source.operator_record_accepted == false
    and $source.operator_record_recorded == false
    and $source.operator_record_persisted == false
    and $source.operator_record_delivered == false
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

lock_specs_json="$(
  jq -n '
    [
      {
        lock_id: "single-route-scope-lock",
        lock_family: "scope",
        precondition_id: "route-scope-matches-canary",
        required_lock_assertion: "exactly one canary route must be bound before dispatch"
      },
      {
        lock_id: "single-namespace-scope-lock",
        lock_family: "scope",
        precondition_id: "namespace-scope-matches-canary",
        required_lock_assertion: "exactly one canary namespace must be bound before dispatch"
      },
      {
        lock_id: "value-scoreboard-hash-lock",
        lock_family: "hash_binding",
        precondition_id: "value-scoreboard-hash-matches",
        required_lock_assertion: "value scoreboard hash must match the accepted packet value scoreboard"
      },
      {
        lock_id: "payload-readback-receipt-hash-lock",
        lock_family: "readback",
        precondition_id: "readback-receipt-hash-matches",
        required_lock_assertion: "payload readback receipt hash must match the previewed request payload"
      },
      {
        lock_id: "audit-receipt-hash-lock",
        lock_family: "audit",
        precondition_id: "audit-receipt-hash-matches",
        required_lock_assertion: "audit receipt hash must match the audit trail preview"
      },
      {
        lock_id: "idempotency-nonce-current-unused-lock",
        lock_family: "idempotency",
        precondition_id: "idempotency-nonce-current-and-unused",
        required_lock_assertion: "idempotency nonce must be current and unused"
      },
      {
        lock_id: "rollback-kill-switch-lock",
        lock_family: "rollback",
        precondition_id: "rollback-plan-and-kill-switch-present",
        required_lock_assertion: "rollback plan and kill switch must be present before canary arm"
      },
      {
        lock_id: "dispatch-budget-one-lock",
        lock_family: "budget",
        precondition_id: "dispatch-budget-equals-one",
        required_lock_assertion: "dispatch budget must be exactly one controlled request"
      },
      {
        lock_id: "secret-injection-absent-lock",
        lock_family: "secret_boundary",
        precondition_id: "secret-injection-absent",
        required_lock_assertion: "operator record must carry no secret material or credential injection"
      }
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_readiness_lock_gate" \
    --arg template_report_sha256 "$template_report_sha256" \
    --arg readiness_lock_policy_hash_sha256 "$readiness_lock_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$TEMPLATE_JSON" \
    --argjson lock_specs "$lock_specs_json" \
    '
      ($lock_specs | map(
        . as $spec
        | ($source.operator_record_template_sections[] | select(.precondition_id == $spec.precondition_id)) as $section
        | {
            lock_id: $spec.lock_id,
            lock_family: $spec.lock_family,
            source_precondition_id: $spec.precondition_id,
            source_template_section_id: $section.template_section_id,
            source_template_section_status: $section.template_section_status,
            source_negative_fixture_family: $section.source_negative_fixture_family,
            source_negative_matrix_hash_bound: $section.source_negative_matrix_hash_bound,
            source_negative_matrix_sha256: $section.source_negative_matrix_sha256,
            source_required_record_fields: $section.required_record_fields,
            required_lock_assertion: $spec.required_lock_assertion,
            source_template_report_sha256: $template_report_sha256,
            readiness_lock_shape_declared: true,
            readiness_lock_report_only: true,
            readiness_lock_operator_input_required: true,
            readiness_lock_operator_input_supplied: false,
            readiness_lock_recorded: false,
            readiness_lock_persisted: false,
            readiness_lock_delivered: false,
            readiness_lock_accepted: false,
            readiness_lock_authorizes_dispatch: false,
            readiness_lock_authorizes_context_attachment: false,
            readiness_lock_authorizes_provider_model_invocation: false,
            readiness_lock_authorizes_memory_write: false,
            readiness_lock_authorizes_external_kg_read: false,
            readiness_lock_authorizes_live_kg_write: false,
            readiness_lock_authorizes_live_execution: false,
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
            status: "blocked_until_real_trusted_operator_record_lock_input"
          }
      )) as $locks
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_trusted_operator_acceptance_record_readiness_lock_schema_version: "memory_intelligence_kg_operator_canary_trusted_operator_acceptance_record_readiness_lock_v1",
          operator_canary_trusted_operator_acceptance_record_readiness_lock_ready: true,
          operator_canary_trusted_operator_acceptance_record_readiness_lock_status: "blocked",
          readiness_lock_mode: "stdout_only_report_only_lock_summary_no_operator_record_no_acceptance_no_dispatch_no_live",
          readiness_lock_decision: "single_route_single_namespace_readback_audit_nonce_rollback_budget_and_secret_boundary_locks_are_declared_but_no_operator_record_supplies_or_accepts_them",
          min_long_soak_samples: $min_long_soak_samples,
          source_operator_record_template_gate: $source.gate,
          source_operator_record_template_status: $source.operator_canary_trusted_operator_acceptance_record_template_status,
          source_operator_record_template_report_sha256: $template_report_sha256,
          source_required_template_section_count: $source.required_template_section_count,
          source_required_operator_record_field_count: $source.required_operator_record_field_count,
          source_supplied_operator_record_field_count: $source.supplied_operator_record_field_count,
          source_accepted_operator_record_field_count: $source.accepted_operator_record_field_count,
          readiness_lock_policy_hash_sha256: $readiness_lock_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          readiness_locks: $locks,
          readiness_lock_count: ($locks | length),
          declared_readiness_lock_count: ($locks | map(select(.readiness_lock_shape_declared == true)) | length),
          report_only_readiness_lock_count: ($locks | map(select(.readiness_lock_report_only == true)) | length),
          operator_input_required_readiness_lock_count: ($locks | map(select(.readiness_lock_operator_input_required == true)) | length),
          operator_input_supplied_readiness_lock_count: ($locks | map(select(.readiness_lock_operator_input_supplied == true)) | length),
          recorded_readiness_lock_count: ($locks | map(select(.readiness_lock_recorded == true)) | length),
          persisted_readiness_lock_count: ($locks | map(select(.readiness_lock_persisted == true)) | length),
          delivered_readiness_lock_count: ($locks | map(select(.readiness_lock_delivered == true)) | length),
          accepted_readiness_lock_count: ($locks | map(select(.readiness_lock_accepted == true)) | length),
          dispatch_authorizing_readiness_lock_count: ($locks | map(select(.readiness_lock_authorizes_dispatch == true)) | length),
          context_authorizing_readiness_lock_count: ($locks | map(select(.readiness_lock_authorizes_context_attachment == true)) | length),
          provider_model_authorizing_readiness_lock_count: ($locks | map(select(.readiness_lock_authorizes_provider_model_invocation == true)) | length),
          memory_write_authorizing_readiness_lock_count: ($locks | map(select(.readiness_lock_authorizes_memory_write == true)) | length),
          external_kg_read_authorizing_readiness_lock_count: ($locks | map(select(.readiness_lock_authorizes_external_kg_read == true)) | length),
          live_kg_write_authorizing_readiness_lock_count: ($locks | map(select(.readiness_lock_authorizes_live_kg_write == true)) | length),
          live_execution_authorizing_readiness_lock_count: ($locks | map(select(.readiness_lock_authorizes_live_execution == true)) | length),
          single_route_scope_lock_declared: ($locks | any(.lock_id == "single-route-scope-lock" and .readiness_lock_shape_declared == true)),
          single_namespace_scope_lock_declared: ($locks | any(.lock_id == "single-namespace-scope-lock" and .readiness_lock_shape_declared == true)),
          payload_readback_receipt_hash_lock_declared: ($locks | any(.lock_id == "payload-readback-receipt-hash-lock" and .readiness_lock_shape_declared == true)),
          audit_receipt_hash_lock_declared: ($locks | any(.lock_id == "audit-receipt-hash-lock" and .readiness_lock_shape_declared == true)),
          dispatch_budget_one_lock_declared: ($locks | any(.lock_id == "dispatch-budget-one-lock" and .readiness_lock_shape_declared == true)),
          dispatch_budget_one_lock_accepted: false,
          dispatch_budget_exactly_one_accepted: false,
          controlled_request_budget_accepted: false,
          controlled_request_budget_value: null,
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
          denied_by_readiness_lock: [
            "readiness_lock_not_operator_approval",
            "readiness_lock_recording_denied",
            "readiness_lock_persistence_denied",
            "readiness_lock_acceptance_denied",
            "single_route_scope_lock_unaccepted",
            "single_namespace_scope_lock_unaccepted",
            "payload_readback_receipt_hash_lock_unaccepted",
            "audit_receipt_hash_lock_unaccepted",
            "dispatch_budget_one_lock_unaccepted",
            "controlled_request_dispatch_denied",
            "context_attachment_denied",
            "provider_model_invocation_denied",
            "memory_write_denied",
            "external_kg_read_denied",
            "live_kg_write_denied",
            "live_execution_denied",
            "secret_credential_read_denied"
          ],
          denied_by_readiness_lock_count: 17,
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            readiness_lock_recorded: false,
            readiness_lock_persisted: false,
            readiness_lock_delivered: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_readiness_lock_gate"
  and .operator_canary_trusted_operator_acceptance_record_readiness_lock_schema_version == "memory_intelligence_kg_operator_canary_trusted_operator_acceptance_record_readiness_lock_v1"
  and .operator_canary_trusted_operator_acceptance_record_readiness_lock_ready == true
  and .operator_canary_trusted_operator_acceptance_record_readiness_lock_status == "blocked"
  and .readiness_lock_mode == "stdout_only_report_only_lock_summary_no_operator_record_no_acceptance_no_dispatch_no_live"
  and .source_operator_record_template_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_template_gate"
  and .source_operator_record_template_status == "blocked"
  and .source_required_template_section_count == 12
  and .source_required_operator_record_field_count == 36
  and .source_supplied_operator_record_field_count == 0
  and .source_accepted_operator_record_field_count == 0
  and .readiness_lock_count == 9
  and .declared_readiness_lock_count == 9
  and .report_only_readiness_lock_count == 9
  and .operator_input_required_readiness_lock_count == 9
  and .operator_input_supplied_readiness_lock_count == 0
  and .recorded_readiness_lock_count == 0
  and .persisted_readiness_lock_count == 0
  and .delivered_readiness_lock_count == 0
  and .accepted_readiness_lock_count == 0
  and .dispatch_authorizing_readiness_lock_count == 0
  and .context_authorizing_readiness_lock_count == 0
  and .provider_model_authorizing_readiness_lock_count == 0
  and .memory_write_authorizing_readiness_lock_count == 0
  and .external_kg_read_authorizing_readiness_lock_count == 0
  and .live_kg_write_authorizing_readiness_lock_count == 0
  and .live_execution_authorizing_readiness_lock_count == 0
  and .single_route_scope_lock_declared == true
  and .single_namespace_scope_lock_declared == true
  and .payload_readback_receipt_hash_lock_declared == true
  and .audit_receipt_hash_lock_declared == true
  and .dispatch_budget_one_lock_declared == true
  and .dispatch_budget_one_lock_accepted == false
  and .dispatch_budget_exactly_one_accepted == false
  and .controlled_request_budget_accepted == false
  and .controlled_request_budget_value == null
  and (.readiness_locks | all(
    .source_template_section_status == "missing_operator_input"
    and .source_negative_matrix_hash_bound == true
    and (.source_negative_matrix_sha256 | test("^[0-9a-f]{64}$"))
    and (.source_required_record_fields | length) > 0
    and .required_lock_assertion != ""
    and .readiness_lock_shape_declared == true
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
    and .status == "blocked_until_real_trusted_operator_record_lock_input"
  ))
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
  and .denied_by_readiness_lock_count == 17
  and (.denied_by_readiness_lock | length) == 17
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG trusted operator acceptance record readiness lock gate passed"
