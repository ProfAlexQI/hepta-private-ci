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

PRECONDITION_SCOREBOARD_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-positive-precondition-scoreboard-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-positive-precondition-scoreboard-gate.sh
)"

precondition_scoreboard_report_sha256="$(sha256_text "$PRECONDITION_SCOREBOARD_JSON")"
operator_record_template_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-trusted-operator-acceptance-record-template:v1:source-positive-precondition-scoreboard:$precondition_scoreboard_report_sha256:no-record:no-accept:no-dispatch:no-live"
)"
operator_record_template_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-trusted-operator-acceptance-record-template-policy:v1:12-sections:36-fields:stdout-only"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_trusted_operator_acceptance_record_template_side_effects=false;template_rendered=true;recorded=0;persisted=0;accepted=0;delivered=0;dispatch=0;context=0;provider=0;model=0;memory=0;kg=0;secret=0;restart=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$PRECONDITION_SCOREBOARD_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_positive_precondition_scoreboard_gate"
    and $source.operator_canary_trusted_operator_acceptance_record_positive_precondition_scoreboard_ready == true
    and $source.operator_canary_trusted_operator_acceptance_record_positive_precondition_scoreboard_status == "blocked"
    and $source.positive_precondition_count == 12
    and $source.declared_positive_precondition_count == 12
    and $source.satisfied_positive_precondition_count == 0
    and $source.missing_positive_precondition_count == 12
    and $source.accepted_positive_precondition_count == 0
    and $source.dispatch_authorizing_positive_precondition_count == 0
    and $source.live_authorizing_positive_precondition_count == 0
    and $source.positive_precondition_family_count == 9
    and ($source.positive_preconditions | all(
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
    ))
    and $source.operator_record_required == true
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

section_specs=(
  "operator-identity-current|operator-identity|operator_identity_id,operator_identity_hash,operator_identity_asserted_at|provide a current hash-only operator identity binding for the canary"
  "operator-signature-hash-matches-payload|operator-signature|operator_signature_hash,operator_signature_payload_hash,operator_signature_algorithm|provide a signature hash over the exact trusted operator record payload"
  "operator-signed-at-fresh|operator-timestamp|operator_signed_at,operator_signature_expires_at,operator_timestamp_freshness_window|provide a fresh signed-at timestamp and bounded expiry window"
  "route-scope-matches-canary|route-scope|canary_route_id,canary_route_scope_hash,canary_route_scope_generation|bind the record to the single approved canary route"
  "namespace-scope-matches-canary|namespace-scope|canary_namespace_id,canary_namespace_scope_hash,canary_namespace_generation|bind the record to the single approved canary namespace"
  "value-scoreboard-hash-matches|value-scoreboard-binding|value_scoreboard_report_hash,value_scoreboard_policy_hash,value_scoreboard_generation|bind the record to the accepted value scoreboard hash"
  "readback-receipt-hash-matches|readback-receipt-binding|payload_readback_receipt_hash,payload_readback_report_hash,payload_readback_generation|bind the record to the payload readback receipt hash"
  "audit-receipt-hash-matches|audit-receipt-binding|audit_receipt_hash,audit_report_hash,audit_generation|bind the record to the audit receipt hash"
  "idempotency-nonce-current-and-unused|idempotency-nonce|idempotency_nonce,idempotency_nonce_generation,idempotency_nonce_unused_proof_hash|provide a current unused nonce bound to this canary"
  "rollback-plan-and-kill-switch-present|rollback-kill-switch|rollback_plan_hash,rollback_rehearsal_hash,kill_switch_id,kill_switch_test_hash|provide rollback plan and kill switch evidence before any canary arm"
  "dispatch-budget-equals-one|dispatch-budget|controlled_request_budget,controlled_request_budget_hash|limit the canary to exactly one controlled request"
  "secret-injection-absent|secret-boundary|secret_injection_absence_attestation_hash,credential_read_absence_hash,redaction_policy_hash|prove the record carries no secret material or credential injection"
)

section_specs_json="$(
  printf '%s\n' "${section_specs[@]}" \
    | jq -R -s '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            precondition_id: .[0],
            template_section_id: .[1],
            required_record_fields: (.[2] | split(",")),
            operator_instruction: .[3]
          })
      '
)"

template_generation_steps_json="$(
  jq -n '
    [
      {
        step_id: "inspect-positive-precondition-scoreboard",
        status: "template_only_not_executed",
        instruction: "review all 12 missing positive preconditions and their source negative-fixture family bindings"
      },
      {
        step_id: "collect-real-operator-record-fields",
        status: "template_only_not_executed",
        instruction: "collect the 36 template fields outside this gate; this gate does not supply or trust them"
      },
      {
        step_id: "bind-record-to-canary-scope-and-hashes",
        status: "template_only_not_executed",
        instruction: "bind route, namespace, value scoreboard, readback receipt, audit receipt, nonce, rollback, and secret-boundary evidence"
      },
      {
        step_id: "review-operator-record-before-intake",
        status: "template_only_not_executed",
        instruction: "review the real record before any future intake can consider acceptance"
      },
      {
        step_id: "rerun-intake-with-real-record",
        status: "template_only_not_executed",
        instruction: "rerun the intake path only after a real record exists outside this template gate"
      }
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_template_gate" \
    --arg precondition_scoreboard_report_sha256 "$precondition_scoreboard_report_sha256" \
    --arg operator_record_template_hash_sha256 "$operator_record_template_hash_sha256" \
    --arg operator_record_template_policy_hash_sha256 "$operator_record_template_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$PRECONDITION_SCOREBOARD_JSON" \
    --argjson section_specs "$section_specs_json" \
    --argjson generation_steps "$template_generation_steps_json" \
    '
      ($section_specs | map(
        . as $spec
        | ($source.positive_preconditions[] | select(.precondition_id == $spec.precondition_id)) as $precondition
        | {
            precondition_id: $spec.precondition_id,
            precondition_family: $precondition.precondition_family,
            template_section_id: $spec.template_section_id,
            template_section_status: "missing_operator_input",
            source_precondition_status: $precondition.status,
            source_negative_fixture_family: $precondition.source_negative_fixture_family,
            source_negative_matrix_hash_bound: $precondition.source_negative_matrix_hash_bound,
            source_negative_matrix_sha256: $precondition.source_negative_matrix_sha256,
            source_required_evidence: $precondition.required_evidence,
            required_record_fields: $spec.required_record_fields,
            required_record_field_count: ($spec.required_record_fields | length),
            operator_instruction: $spec.operator_instruction,
            operator_input_required: true,
            template_only: true,
            report_only: true,
            record_field_supplied_count: 0,
            record_field_trusted_count: 0,
            record_field_accepted_count: 0,
            section_satisfied: false,
            section_accepted: false,
            record_section_recorded: false,
            record_section_persisted: false,
            record_section_delivered: false,
            authorizes_canary_dispatch: false,
            authorizes_context_attachment: false,
            authorizes_provider_model_invocation: false,
            authorizes_memory_write: false,
            authorizes_external_kg_read: false,
            authorizes_live_kg_write: false,
            authorizes_live_execution: false
          }
      )) as $sections
      | ($sections | map(.required_record_fields[]) | unique) as $required_fields
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_trusted_operator_acceptance_record_template_schema_version: "memory_intelligence_kg_operator_canary_trusted_operator_acceptance_record_template_v1",
          operator_canary_trusted_operator_acceptance_record_template_ready: true,
          operator_canary_trusted_operator_acceptance_record_template_status: "blocked",
          template_mode: "stdout_only_report_only_template_no_operator_record_no_acceptance_no_dispatch_no_live",
          template_decision: "positive_preconditions_rendered_as_a_deterministic_operator_record_template_but_no_record_is_supplied_trusted_accepted_or_authorizing",
          min_long_soak_samples: $min_long_soak_samples,
          source_positive_precondition_scoreboard_gate: $source.gate,
          source_positive_precondition_scoreboard_status: $source.operator_canary_trusted_operator_acceptance_record_positive_precondition_scoreboard_status,
          source_positive_precondition_scoreboard_report_sha256: $precondition_scoreboard_report_sha256,
          source_positive_precondition_count: $source.positive_precondition_count,
          source_missing_positive_precondition_count: $source.missing_positive_precondition_count,
          source_accepted_positive_precondition_count: $source.accepted_positive_precondition_count,
          operator_record_template_hash_sha256: $operator_record_template_hash_sha256,
          operator_record_template_policy_hash_sha256: $operator_record_template_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          required_template_section_count: 12,
          operator_record_template_section_count: ($sections | length),
          rendered_template_section_count: ($sections | length),
          missing_operator_input_section_count: ($sections | map(select(.template_section_status == "missing_operator_input")) | length),
          satisfied_template_section_count: ($sections | map(select(.section_satisfied == true)) | length),
          accepted_template_section_count: ($sections | map(select(.section_accepted == true)) | length),
          operator_input_required_section_count: ($sections | map(select(.operator_input_required == true)) | length),
          report_only_section_count: ($sections | map(select(.report_only == true)) | length),
          required_operator_record_field_count: ($sections | map(.required_record_field_count) | add),
          unique_operator_record_field_count: ($required_fields | length),
          supplied_operator_record_field_count: 0,
          trusted_operator_record_field_count: 0,
          accepted_operator_record_field_count: 0,
          positive_precondition_family_count: ($sections | map(.precondition_family) | unique | length),
          operator_record_template_required_fields: $required_fields,
          operator_record_template_sections: $sections,
          operator_record_template_generation_steps: $generation_steps,
          operator_record_template_generation_step_count: ($generation_steps | length),
          next_required_step: "fill_template_with_real_trusted_operator_acceptance_record_outside_this_gate_then_rerun_intake_before_canary_arm_or_live_execution",
          denied_by_operator_record_template: [
            "operator_record_template_not_operator_approval",
            "operator_record_template_recording_denied",
            "operator_record_template_persistence_denied",
            "operator_record_template_acceptance_denied",
            "operator_record_template_delivery_denied",
            "operator_record_field_supply_denied",
            "operator_record_field_trust_denied",
            "operator_record_field_acceptance_denied",
            "operator_record_authority_denied",
            "canary_harness_arm_denied",
            "controlled_request_dispatch_denied",
            "context_attachment_denied",
            "provider_model_invocation_denied",
            "memory_write_denied",
            "external_kg_read_denied",
            "live_kg_write_denied",
            "live_execution_denied",
            "secret_credential_read_denied"
          ],
          denied_by_operator_record_template_count: 18,
          operator_record_template_rendered: true,
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
            operator_record_template_recorded: false,
            operator_record_template_persisted: false,
            operator_record_template_delivered: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_template_gate"
  and .operator_canary_trusted_operator_acceptance_record_template_schema_version == "memory_intelligence_kg_operator_canary_trusted_operator_acceptance_record_template_v1"
  and .operator_canary_trusted_operator_acceptance_record_template_ready == true
  and .operator_canary_trusted_operator_acceptance_record_template_status == "blocked"
  and .template_mode == "stdout_only_report_only_template_no_operator_record_no_acceptance_no_dispatch_no_live"
  and .source_positive_precondition_scoreboard_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_positive_precondition_scoreboard_gate"
  and .source_positive_precondition_scoreboard_status == "blocked"
  and .source_positive_precondition_count == 12
  and .source_missing_positive_precondition_count == 12
  and .source_accepted_positive_precondition_count == 0
  and .required_template_section_count == 12
  and .operator_record_template_section_count == 12
  and .rendered_template_section_count == 12
  and .missing_operator_input_section_count == 12
  and .satisfied_template_section_count == 0
  and .accepted_template_section_count == 0
  and .operator_input_required_section_count == 12
  and .report_only_section_count == 12
  and .required_operator_record_field_count == 36
  and .unique_operator_record_field_count == 36
  and .supplied_operator_record_field_count == 0
  and .trusted_operator_record_field_count == 0
  and .accepted_operator_record_field_count == 0
  and .positive_precondition_family_count == 9
  and (.operator_record_template_required_fields | length) == 36
  and (.operator_record_template_sections | all(
    .template_section_status == "missing_operator_input"
    and .source_precondition_status == "blocked_until_real_operator_record_evidence"
    and .source_negative_fixture_family != ""
    and .source_negative_matrix_hash_bound == true
    and (.source_negative_matrix_sha256 | test("^[0-9a-f]{64}$"))
    and .source_required_evidence != ""
    and (.required_record_fields | length) == .required_record_field_count
    and .operator_instruction != ""
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
  and (.operator_record_template_sections | any(.precondition_id == "operator-identity-current" and .template_section_id == "operator-identity"))
  and (.operator_record_template_sections | any(.precondition_id == "operator-signature-hash-matches-payload" and .template_section_id == "operator-signature"))
  and (.operator_record_template_sections | any(.precondition_id == "operator-signed-at-fresh" and .template_section_id == "operator-timestamp"))
  and (.operator_record_template_sections | any(.precondition_id == "dispatch-budget-equals-one" and .template_section_id == "dispatch-budget"))
  and (.operator_record_template_sections | any(.precondition_id == "secret-injection-absent" and .template_section_id == "secret-boundary"))
  and .operator_record_template_generation_step_count == 5
  and (.operator_record_template_generation_steps | all(.status == "template_only_not_executed"))
  and .denied_by_operator_record_template_count == 18
  and (.denied_by_operator_record_template | length) == 18
  and .operator_record_template_rendered == true
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
echo "Hepta Memory/Intelligence/KG trusted operator acceptance record template gate passed"
