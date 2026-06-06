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

ACTIVATION_REQUEST_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix-gate.sh
)"

activation_request_report_sha256="$(sha256_text "$ACTIVATION_REQUEST_JSON")"
activation_request_denial_matrix_hash_sha256="$(jq -r '.activation_request_denial_matrix_hash_sha256' <<<"$ACTIVATION_REQUEST_JSON")"
source_review_index_hash_sha256="$(jq -r '.source_operator_review_index_hash_sha256' <<<"$ACTIVATION_REQUEST_JSON")"
source_acknowledgement_index_hash_sha256="$(jq -r '.source_operator_review_acknowledgement_index_hash_sha256' <<<"$ACTIVATION_REQUEST_JSON")"

activation_command_fixtures_json="$(
  jq -n '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        fixture_id: $id,
        activation_command_status: $status,
        source_activation_request_denial_matrix_present: true,
        source_activation_request_denial_matrix_ready: true,
        activation_command_requested: true,
        activation_command_shape_registered: false,
        activation_command_allowed: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_command_dispatch_performed: false,
        activation_command_noop_decision_recorded: false,
        activation_command_noop_decision_persisted: false,
        activation_command_noop_decision_accepted: false,
        activation_command_handoff_recorded: false,
        activation_command_handoff_persisted: false,
        activation_command_handoff_accepted: false,
        activation_command_handoff_materialized: false,
        activation_command_handoff_filesystem_written: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_exported: false,
        activation_command_result_receipt_query_registered: false,
        activation_command_result_receipt_observability_recorded: false,
        activation_request_allowed: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_materialized: false,
        activation_request_filesystem_written: false,
        activation_request_delivered: false,
        activation_request_executed: false,
        activation_nonce_generated: false,
        activation_identity_accepted: false,
        activation_scope_accepted: false,
        activation_final_state_promoted: false,
        operator_review_acknowledgement_accepted: false,
        operator_review_acknowledgement_recorded: false,
        operator_review_acknowledgement_persisted: false,
        operator_approval_recorded: false,
        operator_identity_accepted: false,
        dispatch_allowed: false,
        dispatch_performed: false,
        execution_allowed: false,
        execution_performed: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        memory_store_write_performed: false,
        external_kg_adapter_read_performed: false,
        live_kg_write_performed: false,
        credential_read: false,
        secret_file_read: false,
        channel_send_performed: false,
        install_performed: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        activation_command_noop_confirmed: true,
        denial_reason: $reason
      } + $extra;
    [
      blocked_fixture("missing-source-activation-request-denial-matrix-report"; "blocked_noop"; "source_activation_request_denial_matrix_report_required"; {source_activation_request_denial_matrix_present: false, source_activation_request_denial_matrix_ready: false}),
      blocked_fixture("acknowledgement-activation-command-handoff-request"; "blocked_command_noop"; "activation_command_handoff_shape_denied"; {}),
      blocked_fixture("acknowledgement-activation-command-registration-enable-request"; "blocked_register_enable_noop"; "activation_command_registration_enablement_denied"; {activation_command_registration_requested: true, activation_command_enable_requested: true}),
      blocked_fixture("acknowledgement-activation-command-direct-invocation-request"; "blocked_invocation_noop"; "activation_command_invocation_denied"; {activation_command_invocation_requested: true}),
      blocked_fixture("acknowledgement-activation-command-dispatch-request"; "blocked_dispatch_noop"; "activation_command_dispatch_denied"; {activation_command_dispatch_requested: true}),
      blocked_fixture("acknowledgement-activation-command-execution-request"; "blocked_execution_noop"; "activation_command_execution_denied"; {activation_command_execution_requested: true}),
      blocked_fixture("acknowledgement-activation-command-context-provider-model-request"; "blocked_context_provider_model_noop"; "context_provider_model_command_denied"; {context_attachment_requested: true, provider_invocation_requested: true, model_invocation_requested: true}),
      blocked_fixture("acknowledgement-activation-command-memory-kg-request"; "blocked_memory_kg_noop"; "memory_kg_command_denied"; {memory_write_requested: true, kg_write_requested: true}),
      blocked_fixture("acknowledgement-activation-command-result-receipt-readback-request"; "blocked_receipt_readback_noop"; "command_result_receipt_readback_denied"; {command_result_receipt_record_requested: true, command_result_receipt_persist_requested: true, readback_requested: true}),
      blocked_fixture("acknowledgement-activation-command-external-public-install-secret-request"; "blocked_external_secret_noop"; "external_public_install_secret_command_denied"; {external_send_requested: true, public_claim_requested: true, install_requested: true, restart_requested: true, secret_access_requested: true})
    ]
  '
)"

activation_command_fixtures_sha256="$(sha256_text "$activation_command_fixtures_json")"
activation_command_noop_handoff_hash_sha256="$(
  sha256_text "hepta-canary-operator-review-acknowledgement-activation-command-noop-handoff:v1:request=$activation_request_denial_matrix_hash_sha256:review=$source_review_index_hash_sha256:ack=$source_acknowledgement_index_hash_sha256:command=0:dispatch=0:execute=0:receipt=0:live=0"
)"
activation_command_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-noop-handoff:v1:no-command-register:no-command-enable:no-command-invoke:no-command-dispatch:no-handoff-persist:no-result-receipt:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_review_acknowledgement_activation_command_side_effects=false;command_requested=10;command_accepted=0;command_dispatched=0;command_executed=0;receipt=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ACTIVATION_REQUEST_JSON" \
  --argjson fixtures "$activation_command_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_status == "blocked"
    and $source.activation_request_denial_fixture_count == 9
    and $source.blocked_activation_request_fixture_count == 9
    and $source.accepted_activation_request_fixture_count == 0
    and $source.activation_request_performed_count == 0
    and $source.activation_request_accepted == false
    and $source.activation_request_recorded == false
    and $source.activation_request_persisted == false
    and $source.activation_request_executed == false
    and $source.dispatch_performed_count == 0
    and $source.execution_performed_count == 0
    and $source.context_injection_performed_count == 0
    and $source.provider_invoked_count == 0
    and $source.model_invoked_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.external_kg_adapter_read_performed_count == 0
    and $source.live_kg_write_performed_count == 0
    and $source.credential_read_count == 0
    and $source.secret_file_read_count == 0
    and $source.channel_send_performed_count == 0
    and $source.install_performed_count == 0
    and $source.service_restarted_count == 0
    and $source.active_binary_mutated_count == 0
    and $source.upstream_fetch_performed_count == 0
    and $source.upstream_merge_performed_count == 0
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and $source.denied_by_operator_review_acknowledgement_activation_request_denial_matrix_count == 26
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      .activation_command_requested == true
      and .activation_command_allowed == false
      and .activation_command_accepted == false
      and .activation_command_enabled == false
      and .activation_command_invoked == false
      and .activation_command_dispatched == false
      and .activation_command_dispatch_performed == false
      and .activation_command_noop_decision_recorded == false
      and .activation_command_handoff_recorded == false
      and .activation_command_result_receipt_recorded == false
      and .activation_request_accepted == false
      and .activation_request_executed == false
      and .dispatch_performed == false
      and .execution_performed == false
      and .context_injection_performed == false
      and .provider_invoked == false
      and .model_invoked == false
      and .memory_store_write_performed == false
      and .external_kg_adapter_read_performed == false
      and .live_kg_write_performed == false
      and .credential_read == false
      and .secret_file_read == false
      and .channel_send_performed == false
      and .install_performed == false
      and .service_restarted == false
      and .active_binary_mutated == false
      and .upstream_fetch_performed == false
      and .upstream_merge_performed == false
      and .activation_command_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_gate" \
    --arg activation_request_report_sha256 "$activation_request_report_sha256" \
    --arg activation_request_denial_matrix_hash_sha256 "$activation_request_denial_matrix_hash_sha256" \
    --arg activation_command_fixtures_sha256 "$activation_command_fixtures_sha256" \
    --arg activation_command_noop_handoff_hash_sha256 "$activation_command_noop_handoff_hash_sha256" \
    --arg activation_command_policy_hash_sha256 "$activation_command_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ACTIVATION_REQUEST_JSON" \
    --argjson fixtures "$activation_command_fixtures_json" \
    '
      ($source.denied_by_operator_review_acknowledgement_activation_request_denial_matrix + [
        "source_activation_request_denial_matrix_report_required",
        "activation_command_shape_registration_denied",
        "activation_command_acceptance_denied",
        "activation_command_enablement_denied",
        "activation_command_invocation_denied",
        "activation_command_dispatch_denied",
        "activation_command_dispatch_execution_denied",
        "activation_command_noop_decision_recording_denied",
        "activation_command_noop_decision_persistence_denied",
        "activation_command_handoff_recording_denied",
        "activation_command_handoff_persistence_denied",
        "activation_command_result_receipt_recording_denied",
        "activation_command_result_receipt_persistence_denied",
        "activation_command_result_receipt_acceptance_denied",
        "activation_command_result_receipt_export_query_observability_denied",
        "activation_request_acceptance_denied",
        "activation_request_execution_denied",
        "operator_review_acknowledgement_not_authority",
        "operator_approval_not_recorded",
        "dispatch_from_command_denied",
        "execution_from_command_denied",
        "context_injection_from_command_denied",
        "provider_model_invocation_denied",
        "memory_write_denied",
        "external_kg_read_denied",
        "live_kg_write_denied",
        "credential_secret_read_denied",
        "channel_delivery_denied",
        "install_restart_denied",
        "active_binary_mutation_denied",
        "upstream_fetch_merge_denied"
      ]) as $denials |
      {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_schema_version: "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_noop_handoff_v1",
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_ready: true,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_status: "blocked",
        activation_command_noop_handoff_mode: "stdout_only_activation_command_shapes_no_register_no_enable_no_invoke_no_dispatch_no_result_receipt_no_live",
        activation_command_noop_handoff_decision: "operator_review_acknowledgement_activation_request_denial_cannot_create_or_authorize_activation_commands",
        minimum_required_samples: $min_long_soak_samples,
        source_operator_review_acknowledgement_activation_request_denial_matrix_gate: $source.gate,
        source_operator_review_acknowledgement_activation_request_denial_matrix_status: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_status,
        source_operator_review_acknowledgement_activation_request_denial_matrix_report_sha256: $activation_request_report_sha256,
        source_activation_request_denial_matrix_hash_sha256: $activation_request_denial_matrix_hash_sha256,
        activation_command_fixtures_sha256: $activation_command_fixtures_sha256,
        activation_command_noop_handoff_hash_sha256: $activation_command_noop_handoff_hash_sha256,
        activation_command_policy_hash_sha256: $activation_command_policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        source_activation_request_denial_fixture_count: $source.activation_request_denial_fixture_count,
        source_blocked_activation_request_fixture_count: $source.blocked_activation_request_fixture_count,
        source_noop_activation_request_fixture_count: $source.noop_activation_request_fixture_count,
        source_accepted_activation_request_fixture_count: $source.accepted_activation_request_fixture_count,
        source_activation_request_performed_count: $source.activation_request_performed_count,
        activation_command_surface_count: 13,
        activation_command_surface_ready_count: 13,
        activation_command_side_effect_free_surface_count: 13,
        activation_command_fixtures: $fixtures,
        activation_command_fixture_count: ($fixtures | length),
        activation_command_requested_fixture_count: ($fixtures | map(select(.activation_command_requested == true)) | length),
        blocked_activation_command_fixture_count: ($fixtures | length),
        noop_activation_command_fixture_count: ($fixtures | length),
        allowed_activation_command_fixture_count: 0,
        accepted_activation_command_fixture_count: 0,
        activation_command_performed_count: 0,
        activation_command_dispatch_performed_count: 0,
        activation_command_shape_registered: false,
        activation_command_allowed: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_command_noop_decision_recorded: false,
        activation_command_noop_decision_persisted: false,
        activation_command_handoff_recorded: false,
        activation_command_handoff_persisted: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        operator_approval_recorded: false,
        dispatch_performed_count: 0,
        execution_performed_count: 0,
        context_injection_performed_count: 0,
        provider_invoked_count: 0,
        model_invoked_count: 0,
        memory_store_write_performed_count: 0,
        external_kg_adapter_read_performed_count: 0,
        live_kg_write_performed_count: 0,
        credential_read_count: 0,
        secret_file_read_count: 0,
        channel_send_performed_count: 0,
        install_performed_count: 0,
        service_restarted_count: 0,
        active_binary_mutated_count: 0,
        upstream_fetch_performed_count: 0,
        upstream_merge_performed_count: 0,
        canary_harness_armed: false,
        canary_harness_executable: false,
        canary_live_enabled: false,
        denied_by_operator_review_acknowledgement_activation_command_noop_handoff: $denials,
        denied_by_operator_review_acknowledgement_activation_command_noop_handoff_count: ($denials | length),
        side_effects: {
          workspace_written: false,
          filesystem_written: false,
          activation_command_registered: false,
          activation_command_enabled: false,
          activation_command_invoked: false,
          activation_command_dispatched: false,
          activation_command_handoff_recorded: false,
          activation_command_handoff_persisted: false,
          activation_command_result_receipt_recorded: false,
          activation_command_result_receipt_persisted: false,
          activation_request_recorded: false,
          activation_request_persisted: false,
          activation_request_executed: false,
          operator_approval_recorded: false,
          dispatch_performed: false,
          execution_performed: false,
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
          telegram_send_performed: false,
          install_performed: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false
        }
      }
    '
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_status == "blocked"
  and .source_activation_request_denial_fixture_count == 9
  and .source_accepted_activation_request_fixture_count == 0
  and .activation_command_fixture_count == 10
  and .blocked_activation_command_fixture_count == 10
  and .noop_activation_command_fixture_count == 10
  and .accepted_activation_command_fixture_count == 0
  and .activation_command_performed_count == 0
  and .activation_command_dispatch_performed_count == 0
  and .activation_command_shape_registered == false
  and .activation_command_allowed == false
  and .activation_command_accepted == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_command_handoff_recorded == false
  and .activation_command_handoff_persisted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .dispatch_performed_count == 0
  and .execution_performed_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .external_kg_adapter_read_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .install_performed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and .upstream_fetch_performed_count == 0
  and .upstream_merge_performed_count == 0
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and (.activation_command_fixtures | all(
    .activation_command_allowed == false
    and .activation_command_accepted == false
    and .activation_command_enabled == false
    and .activation_command_invoked == false
    and .activation_command_dispatched == false
    and .activation_command_dispatch_performed == false
    and .activation_command_handoff_recorded == false
    and .activation_command_result_receipt_recorded == false
    and .activation_request_accepted == false
    and .activation_request_executed == false
    and .dispatch_performed == false
    and .execution_performed == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .channel_send_performed == false
    and .install_performed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .upstream_fetch_performed == false
    and .upstream_merge_performed == false
  ))
  and .denied_by_operator_review_acknowledgement_activation_command_noop_handoff_count == 57
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command no-op handoff gate passed"
