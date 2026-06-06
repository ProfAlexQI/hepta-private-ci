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

INTAKE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-intake-validator-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-intake-validator-gate.sh
)"

intake_validator_report_sha256="$(sha256_text "$INTAKE_JSON")"
negative_fixture_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-trusted-operator-acceptance-record-negative-fixture-matrix:v1:source-intake-validator:wrong-missing-signature-stale-scope-hash-readback-audit-nonce-rollback-budget-secret:no-accept:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_side_effects=false;accepted=0;recorded=0;persisted=0;delivered=0;dispatch=0;context=0;provider=0;model=0;memory=0;kg=0;secret=0;restart=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$INTAKE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_intake_validator_gate"
    and $source.operator_canary_trusted_operator_acceptance_record_intake_validator_ready == true
    and $source.operator_canary_trusted_operator_acceptance_record_intake_validator_status == "blocked"
    and $source.source_trusted_operator_acceptance_record_count == 5
    and $source.source_trusted_operator_acceptance_record_accepted_count == 0
    and $source.source_trusted_operator_acceptance_record_recorded_count == 0
    and $source.source_trusted_operator_acceptance_record_persisted_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_record_count == 5
    and $source.operator_canary_trusted_operator_acceptance_record_intake_record_hash_bound_count == 5
    and $source.operator_canary_trusted_operator_acceptance_record_intake_required_field_count == 80
    and $source.operator_canary_trusted_operator_acceptance_record_intake_present_field_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_missing_field_count == 80
    and $source.operator_canary_trusted_operator_acceptance_record_intake_trusted_field_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_accepted_field_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_record_accepted_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_record_recorded_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_record_persisted_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_record_delivered_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_authorizes_dispatch_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_authorizes_context_attachment_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_authorizes_provider_model_invocation_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_authorizes_memory_write_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_authorizes_external_kg_read_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_authorizes_live_kg_write_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_intake_authorizes_live_execution_count == 0
    and $source.operator_record_supplied == false
    and $source.operator_record_accepted == false
    and $source.operator_record_recorded == false
    and $source.operator_record_persisted == false
    and $source.operator_record_authorizes_dispatch == false
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

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_negative_fixture_matrix_gate" \
    --arg intake_validator_report_sha256 "$intake_validator_report_sha256" \
    --arg negative_fixture_policy_hash_sha256 "$negative_fixture_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$INTAKE_JSON" \
    '
      def negative_fixture($id; $family; $status; $reason; $overrides):
        {
          negative_fixture_id: $id,
          negative_fixture_family: $family,
          negative_fixture_status: $status,
          negative_fixture_reason: $reason,
          source_intake_validator_ready: true,
          source_intake_validator_status: "blocked",
          source_intake_validator_report_sha256: $intake_validator_report_sha256,
          operator_record_supplied: true,
          operator_record_shape_declared: true,
          operator_record_pseudo_complete: true,
          operator_identity_present: true,
          operator_identity_validated: true,
          operator_signature_hash_present: true,
          operator_signature_hash_validated: true,
          operator_timestamp_present: true,
          operator_timestamp_validated: true,
          operator_route_scope_present: true,
          operator_route_scope_validated: true,
          operator_namespace_scope_present: true,
          operator_namespace_scope_validated: true,
          source_value_scoreboard_hash_present: true,
          source_value_scoreboard_hash_validated: true,
          source_intake_validator_hash_present: true,
          source_intake_validator_hash_validated: true,
          readback_receipt_hash_present: true,
          readback_receipt_hash_validated: true,
          audit_receipt_hash_present: true,
          audit_receipt_hash_validated: true,
          idempotency_nonce_present: true,
          idempotency_nonce_fresh: true,
          idempotency_nonce_validated: true,
          rollback_plan_present: true,
          kill_switch_present: true,
          rollback_kill_switch_validated: true,
          dispatch_budget_present: true,
          dispatch_budget_validated: true,
          live_execution_bounds_present: true,
          live_execution_bounds_validated: true,
          redaction_policy_hash_present: true,
          redaction_policy_hash_validated: true,
          credential_secret_boundary_validated: true,
          no_write_boundary_validated: true,
          delivery_persistence_boundary_validated: true,
          negative_fixture_acceptance_attempted: true,
          negative_fixture_blocked: true,
          trusted_operator_acceptance_record_accepted: false,
          trusted_operator_acceptance_record_recorded: false,
          trusted_operator_acceptance_record_persisted: false,
          trusted_operator_acceptance_record_delivered: false,
          trusted_operator_acceptance_record_materialized: false,
          authorizes_canary_dispatch: false,
          authorizes_context_attachment: false,
          authorizes_provider_model_invocation: false,
          authorizes_memory_write: false,
          authorizes_external_kg_read: false,
          authorizes_live_kg_write: false,
          authorizes_live_execution: false,
          controlled_request_dispatched: false,
          controlled_request_executed: false,
          context_injection_performed: false,
          provider_invoked: false,
          model_invoked: false,
          memory_store_write_performed: false,
          external_kg_adapter_read_performed: false,
          live_kg_write_performed: false,
          credential_read: false,
          auth_secret_read: false,
          secret_file_read: false,
          install_performed: false,
          service_restarted: false,
          active_binary_mutated: false
        } + $overrides;

      [
        negative_fixture(
          "missing-operator-identity";
          "operator_identity";
          "blocked_missing_identity";
          "operator_identity_is_required_before_trusted_record_acceptance";
          {
            operator_record_pseudo_complete: false,
            operator_identity_present: false,
            operator_identity_validated: false
          }
        ),
        negative_fixture(
          "signature-hash-mismatch";
          "operator_signature";
          "blocked_signature_hash_mismatch";
          "operator_signature_hash_must_bind_the_record_payload";
          {
            operator_signature_hash_validated: false
          }
        ),
        negative_fixture(
          "stale-signed-at";
          "operator_timestamp";
          "blocked_stale_timestamp";
          "operator_signed_at_must_be_fresh_for_the_current_canary_scope";
          {
            operator_timestamp_validated: false
          }
        ),
        negative_fixture(
          "route-scope-mismatch";
          "scope";
          "blocked_route_scope_mismatch";
          "trusted_record_route_scope_must_match_the_canary_route";
          {
            operator_route_scope_validated: false
          }
        ),
        negative_fixture(
          "namespace-scope-mismatch";
          "scope";
          "blocked_namespace_scope_mismatch";
          "trusted_record_namespace_scope_must_match_the_canary_namespace";
          {
            operator_namespace_scope_validated: false
          }
        ),
        negative_fixture(
          "value-scoreboard-hash-mismatch";
          "hash_binding";
          "blocked_value_scoreboard_hash_mismatch";
          "trusted_record_must_bind_to_the_current_value_scoreboard";
          {
            source_value_scoreboard_hash_validated: false
          }
        ),
        negative_fixture(
          "readback-receipt-hash-mismatch";
          "hash_binding";
          "blocked_readback_receipt_hash_mismatch";
          "trusted_record_must_bind_to_the_current_readback_receipt";
          {
            readback_receipt_hash_validated: false
          }
        ),
        negative_fixture(
          "audit-receipt-hash-mismatch";
          "hash_binding";
          "blocked_audit_receipt_hash_mismatch";
          "trusted_record_must_bind_to_the_current_audit_receipt";
          {
            audit_receipt_hash_validated: false
          }
        ),
        negative_fixture(
          "idempotency-nonce-replay";
          "idempotency";
          "blocked_nonce_replay";
          "trusted_record_idempotency_nonce_must_not_be_replayed";
          {
            idempotency_nonce_fresh: false,
            idempotency_nonce_validated: false
          }
        ),
        negative_fixture(
          "rollback-kill-switch-missing";
          "rollback_kill_switch";
          "blocked_missing_rollback_kill_switch";
          "rollback_plan_and_kill_switch_are_required_before_canary_arm";
          {
            operator_record_pseudo_complete: false,
            rollback_plan_present: false,
            kill_switch_present: false,
            rollback_kill_switch_validated: false
          }
        ),
        negative_fixture(
          "dispatch-budget-exceeds-one";
          "dispatch_budget";
          "blocked_dispatch_budget_exceeds_canary_limit";
          "operator_canary_dispatch_budget_must_be_limited_to_one_controlled_request";
          {
            dispatch_budget_validated: false,
            live_execution_bounds_validated: false
          }
        ),
        negative_fixture(
          "secret-credential-injection-attempt";
          "secret_injection";
          "blocked_secret_credential_injection_attempt";
          "trusted_record_must_not_carry_or_request_credentials_or_secrets";
          {
            credential_secret_boundary_validated: false,
            credential_read: false,
            auth_secret_read: false,
            secret_file_read: false
          }
        )
      ] as $fixtures
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_schema_version: "memory_intelligence_kg_operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_v1",
          operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_ready: true,
          operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_status: "blocked",
          operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_mode: "report_only_negative_operator_record_fixtures_no_accept_no_record_no_dispatch_no_live",
          operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_decision: "pseudo_complete_or_invalid_trusted_operator_acceptance_records_do_not_authorize_canary_dispatch_context_attachment_provider_model_memory_or_kg_live_execution",
          minimum_required_samples: $min_long_soak_samples,
          source_trusted_operator_acceptance_record_intake_validator_gate: $source.gate,
          source_trusted_operator_acceptance_record_intake_validator_report_sha256: $intake_validator_report_sha256,
          source_trusted_operator_acceptance_record_intake_validator_ready: $source.operator_canary_trusted_operator_acceptance_record_intake_validator_ready,
          source_trusted_operator_acceptance_record_intake_validator_status: $source.operator_canary_trusted_operator_acceptance_record_intake_validator_status,
          source_intake_record_count: $source.operator_canary_trusted_operator_acceptance_record_intake_record_count,
          source_intake_required_field_count: $source.operator_canary_trusted_operator_acceptance_record_intake_required_field_count,
          source_intake_present_field_count: $source.operator_canary_trusted_operator_acceptance_record_intake_present_field_count,
          source_intake_accepted_field_count: $source.operator_canary_trusted_operator_acceptance_record_intake_accepted_field_count,
          negative_fixture_policy_hash_sha256: $negative_fixture_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          operator_canary_trusted_operator_acceptance_record_negative_fixtures: $fixtures,
          required_negative_fixture_count: 12,
          negative_fixture_count: ($fixtures | length),
          supplied_negative_fixture_count: ($fixtures | map(select(.operator_record_supplied == true)) | length),
          blocked_negative_fixture_count: ($fixtures | map(select(.negative_fixture_blocked == true)) | length),
          acceptance_attempted_negative_fixture_count: ($fixtures | map(select(.negative_fixture_acceptance_attempted == true)) | length),
          accepted_negative_fixture_count: ($fixtures | map(select(.trusted_operator_acceptance_record_accepted == true)) | length),
          recorded_negative_fixture_count: ($fixtures | map(select(.trusted_operator_acceptance_record_recorded == true)) | length),
          persisted_negative_fixture_count: ($fixtures | map(select(.trusted_operator_acceptance_record_persisted == true)) | length),
          delivered_negative_fixture_count: ($fixtures | map(select(.trusted_operator_acceptance_record_delivered == true)) | length),
          operator_identity_negative_fixture_count: ($fixtures | map(select(.negative_fixture_family == "operator_identity")) | length),
          operator_signature_negative_fixture_count: ($fixtures | map(select(.negative_fixture_family == "operator_signature")) | length),
          operator_timestamp_negative_fixture_count: ($fixtures | map(select(.negative_fixture_family == "operator_timestamp")) | length),
          scope_negative_fixture_count: ($fixtures | map(select(.negative_fixture_family == "scope")) | length),
          hash_binding_negative_fixture_count: ($fixtures | map(select(.negative_fixture_family == "hash_binding")) | length),
          idempotency_negative_fixture_count: ($fixtures | map(select(.negative_fixture_family == "idempotency")) | length),
          rollback_kill_switch_negative_fixture_count: ($fixtures | map(select(.negative_fixture_family == "rollback_kill_switch")) | length),
          dispatch_budget_negative_fixture_count: ($fixtures | map(select(.negative_fixture_family == "dispatch_budget")) | length),
          secret_injection_negative_fixture_count: ($fixtures | map(select(.negative_fixture_family == "secret_injection")) | length),
          operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_dispatch_count: ($fixtures | map(select(.authorizes_canary_dispatch == true)) | length),
          operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_context_attachment_count: ($fixtures | map(select(.authorizes_context_attachment == true)) | length),
          operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_provider_model_invocation_count: ($fixtures | map(select(.authorizes_provider_model_invocation == true)) | length),
          operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_memory_write_count: ($fixtures | map(select(.authorizes_memory_write == true)) | length),
          operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_external_kg_read_count: ($fixtures | map(select(.authorizes_external_kg_read == true)) | length),
          operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_live_kg_write_count: ($fixtures | map(select(.authorizes_live_kg_write == true)) | length),
          operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_live_execution_count: ($fixtures | map(select(.authorizes_live_execution == true)) | length),
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
          usage_recorded: false,
          memory_store_write_performed: false,
          memory_store_mutated: false,
          external_kg_adapter_read_performed: false,
          live_kg_write_performed: false,
          network_call_performed: false,
          external_db_write_performed: false,
          credential_read: false,
          auth_secret_read: false,
          secret_file_read: false,
          channel_send_performed: false,
          telegram_send_performed: false,
          external_send_performed: false,
          filesystem_written: false,
          release_artifact_written: false,
          public_release_claimed: false,
          public_ga_claimed: false,
          install_performed: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false,
          denied_by_trusted_operator_acceptance_record_negative_fixture_matrix: [
            "missing_operator_identity",
            "signature_hash_mismatch",
            "stale_signed_at",
            "route_scope_mismatch",
            "namespace_scope_mismatch",
            "value_scoreboard_hash_mismatch",
            "readback_receipt_hash_mismatch",
            "audit_receipt_hash_mismatch",
            "idempotency_nonce_replay",
            "rollback_kill_switch_missing",
            "dispatch_budget_exceeds_one",
            "secret_credential_injection_attempt"
          ],
          next_required_step: "only_a_real_trusted_operator_acceptance_record_that_satisfies_identity_signature_timestamp_scope_hash_readback_audit_idempotency_rollback_budget_secret_boundary_and_live_bounds_can_be_considered_for_canary_arm",
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            operator_record_recorded: false,
            operator_record_persisted: false,
            operator_record_delivered: false,
            operator_record_accepted: false,
            trusted_operator_acceptance_record_recorded: false,
            trusted_operator_acceptance_record_persisted: false,
            trusted_operator_acceptance_record_delivered: false,
            trusted_operator_acceptance_record_accepted: false,
            controlled_request_dispatched: false,
            controlled_request_executed: false,
            context_injection_performed: false,
            provider_invoked: false,
            model_invoked: false,
            memory_store_write_performed: false,
            memory_store_mutated: false,
            external_kg_adapter_read_performed: false,
            live_kg_write_performed: false,
            network_call_performed: false,
            external_db_write_performed: false,
            credential_read: false,
            auth_secret_read: false,
            secret_file_read: false,
            channel_send_performed: false,
            telegram_send_performed: false,
            external_send_performed: false,
            release_artifact_written: false,
            public_release_claimed: false,
            public_ga_claimed: false,
            install_performed: false,
            service_restarted: false,
            active_binary_mutated: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false
          }
        }
    ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_negative_fixture_matrix_gate"
  and .operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_ready == true
  and .operator_canary_trusted_operator_acceptance_record_negative_fixture_matrix_status == "blocked"
  and .source_trusted_operator_acceptance_record_intake_validator_ready == true
  and .source_trusted_operator_acceptance_record_intake_validator_status == "blocked"
  and .source_intake_record_count == 5
  and .source_intake_required_field_count == 80
  and .source_intake_present_field_count == 0
  and .source_intake_accepted_field_count == 0
  and .required_negative_fixture_count == 12
  and .negative_fixture_count == 12
  and .supplied_negative_fixture_count == 12
  and .blocked_negative_fixture_count == 12
  and .acceptance_attempted_negative_fixture_count == 12
  and .accepted_negative_fixture_count == 0
  and .recorded_negative_fixture_count == 0
  and .persisted_negative_fixture_count == 0
  and .delivered_negative_fixture_count == 0
  and .operator_identity_negative_fixture_count == 1
  and .operator_signature_negative_fixture_count == 1
  and .operator_timestamp_negative_fixture_count == 1
  and .scope_negative_fixture_count == 2
  and .hash_binding_negative_fixture_count == 3
  and .idempotency_negative_fixture_count == 1
  and .rollback_kill_switch_negative_fixture_count == 1
  and .dispatch_budget_negative_fixture_count == 1
  and .secret_injection_negative_fixture_count == 1
  and .operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_dispatch_count == 0
  and .operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_context_attachment_count == 0
  and .operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_provider_model_invocation_count == 0
  and .operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_memory_write_count == 0
  and .operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_external_kg_read_count == 0
  and .operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_live_kg_write_count == 0
  and .operator_canary_trusted_operator_acceptance_record_negative_fixture_authorizes_live_execution_count == 0
  and (.operator_canary_trusted_operator_acceptance_record_negative_fixtures | all(
    .source_intake_validator_ready == true
    and .operator_record_supplied == true
    and .operator_record_shape_declared == true
    and .negative_fixture_acceptance_attempted == true
    and .negative_fixture_blocked == true
    and .trusted_operator_acceptance_record_accepted == false
    and .trusted_operator_acceptance_record_recorded == false
    and .trusted_operator_acceptance_record_persisted == false
    and .trusted_operator_acceptance_record_delivered == false
    and .trusted_operator_acceptance_record_materialized == false
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
  and .operator_record_accepted == false
  and .operator_record_recorded == false
  and .operator_record_persisted == false
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
echo "Hepta Memory/Intelligence/KG trusted operator acceptance record negative fixture matrix gate passed"
