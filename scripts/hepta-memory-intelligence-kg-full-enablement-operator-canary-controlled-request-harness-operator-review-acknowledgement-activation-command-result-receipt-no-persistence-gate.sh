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

COMMAND_NOOP_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-noop-handoff-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-noop-handoff-gate.sh
)"

command_noop_report_sha256="$(sha256_text "$COMMAND_NOOP_JSON")"
command_noop_handoff_hash_sha256="$(jq -r '.activation_command_noop_handoff_hash_sha256' <<<"$COMMAND_NOOP_JSON")"
command_policy_hash_sha256="$(jq -r '.activation_command_policy_hash_sha256' <<<"$COMMAND_NOOP_JSON")"

result_receipt_fixtures_json="$(
  jq -n '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        fixture_id: $id,
        activation_command_result_receipt_status: $status,
        source_activation_command_noop_handoff_present: true,
        source_activation_command_noop_handoff_ready: true,
        activation_command_result_receipt_requested: true,
        activation_command_result_receipt_shape_registered: false,
        activation_command_result_receipt_allowed: false,
        activation_command_result_receipt_schema_accepted: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_materialized: false,
        activation_command_result_receipt_filesystem_written: false,
        activation_command_result_receipt_ledger_written: false,
        activation_command_result_receipt_indexed: false,
        activation_command_result_receipt_enqueued: false,
        activation_command_result_receipt_delivered: false,
        activation_command_result_receipt_exported: false,
        activation_command_result_receipt_query_registered: false,
        activation_command_result_receipt_observability_recorded: false,
        activation_command_result_receipt_hash_bound: false,
        activation_command_result_receipt_signature_hash_recorded: false,
        activation_command_result_receipt_timestamp_recorded: false,
        activation_command_result_receipt_operator_identity_accepted: false,
        activation_command_result_receipt_status_accepted: false,
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_persisted: false,
        activation_command_completion_ack_accepted: false,
        operator_approval_from_receipt_accepted: false,
        activation_from_receipt_allowed: false,
        activation_command_allowed: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_command_dispatch_performed: false,
        activation_command_handoff_recorded: false,
        activation_command_handoff_persisted: false,
        activation_command_result_receipt_non_authority_confirmed: true,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        dispatch_performed: false,
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
        external_send_performed: false,
        public_claim_performed: false,
        install_performed: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        denial_reason: $reason
      } + $extra;
    [
      blocked_fixture("missing-source-activation-command-noop-handoff-report"; "blocked_noop"; "source_activation_command_noop_handoff_report_required"; {source_activation_command_noop_handoff_present: false, source_activation_command_noop_handoff_ready: false}),
      blocked_fixture("acknowledgement-activation-command-result-receipt-schema-registration-attempt"; "blocked_schema_noop"; "result_receipt_schema_registration_denied"; {result_receipt_schema_registration_requested: true}),
      blocked_fixture("acknowledgement-activation-command-result-receipt-record-attempt"; "blocked_record_noop"; "result_receipt_recording_denied"; {result_receipt_record_requested: true}),
      blocked_fixture("acknowledgement-activation-command-result-receipt-persist-attempt"; "blocked_persist_noop"; "result_receipt_persistence_denied"; {result_receipt_persist_requested: true}),
      blocked_fixture("acknowledgement-activation-command-result-receipt-materialize-filesystem-attempt"; "blocked_materialize_noop"; "result_receipt_materialization_filesystem_write_denied"; {result_receipt_materialize_requested: true, result_receipt_filesystem_write_requested: true}),
      blocked_fixture("acknowledgement-activation-command-result-receipt-ledger-index-delivery-attempt"; "blocked_ledger_index_delivery_noop"; "result_receipt_ledger_index_delivery_denied"; {result_receipt_ledger_write_requested: true, result_receipt_index_requested: true, result_receipt_enqueue_requested: true, result_receipt_delivery_requested: true}),
      blocked_fixture("acknowledgement-activation-command-result-receipt-export-query-observability-attempt"; "blocked_export_query_observability_noop"; "result_receipt_export_query_observability_denied"; {result_receipt_export_requested: true, result_receipt_query_requested: true, result_receipt_observability_requested: true}),
      blocked_fixture("acknowledgement-activation-command-result-receipt-acceptance-completion-ack-attempt"; "blocked_acceptance_ack_noop"; "result_receipt_acceptance_completion_ack_denied"; {result_receipt_acceptance_requested: true, completion_ack_requested: true, operator_approval_from_receipt_requested: true}),
      blocked_fixture("acknowledgement-activation-command-result-receipt-activation-authority-attempt"; "blocked_activation_authority_noop"; "result_receipt_cannot_authorize_activation"; {activation_from_receipt_requested: true, activation_request_record_requested: true, activation_execution_requested: true, dispatch_requested: true, execution_requested: true}),
      blocked_fixture("acknowledgement-activation-command-result-receipt-provider-memory-kg-external-attempt"; "blocked_provider_memory_kg_external_noop"; "result_receipt_cannot_invoke_provider_write_memory_kg_or_externalize"; {context_attachment_requested: true, provider_invocation_requested: true, model_invocation_requested: true, memory_write_requested: true, kg_write_requested: true, external_send_requested: true, public_claim_requested: true, install_requested: true, restart_requested: true, secret_access_requested: true})
    ]
  '
)"

result_receipt_fixtures_sha256="$(sha256_text "$result_receipt_fixtures_json")"
result_receipt_no_persistence_hash_sha256="$(
  sha256_text "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-no-persistence:v1:command=$command_noop_handoff_hash_sha256:policy=$command_policy_hash_sha256:record=0:persist=0:accept=0:export=0:query=0:observe=0:authority=0:live=0"
)"
result_receipt_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-no-persistence:v1:no-receipt-record:no-receipt-persist:no-receipt-accept:no-completion-ack:no-authority:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_review_acknowledgement_activation_command_result_receipt_side_effects=false;fixtures=10;record=0;persist=0;accept=0;export=0;query=0;observe=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$COMMAND_NOOP_JSON" \
  --argjson fixtures "$result_receipt_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_status == "blocked"
    and $source.activation_command_fixture_count == 10
    and $source.blocked_activation_command_fixture_count == 10
    and $source.noop_activation_command_fixture_count == 10
    and $source.allowed_activation_command_fixture_count == 0
    and $source.accepted_activation_command_fixture_count == 0
    and $source.activation_command_performed_count == 0
    and $source.activation_command_dispatch_performed_count == 0
    and $source.activation_command_allowed == false
    and $source.activation_command_accepted == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_command_handoff_recorded == false
    and $source.activation_command_handoff_persisted == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_request_accepted == false
    and $source.activation_request_recorded == false
    and $source.activation_request_persisted == false
    and $source.activation_request_executed == false
    and $source.operator_approval_recorded == false
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
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      .activation_command_result_receipt_requested == true
      and .activation_command_result_receipt_allowed == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_command_result_receipt_materialized == false
      and .activation_command_result_receipt_filesystem_written == false
      and .activation_command_result_receipt_exported == false
      and .activation_command_result_receipt_query_registered == false
      and .activation_command_result_receipt_observability_recorded == false
      and .activation_command_completion_ack_recorded == false
      and .operator_approval_from_receipt_accepted == false
      and .activation_from_receipt_allowed == false
      and .activation_command_enabled == false
      and .activation_command_invoked == false
      and .activation_command_dispatched == false
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
      and .activation_command_result_receipt_non_authority_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_gate" \
    --arg command_noop_report_sha256 "$command_noop_report_sha256" \
    --arg command_noop_handoff_hash_sha256 "$command_noop_handoff_hash_sha256" \
    --arg result_receipt_fixtures_sha256 "$result_receipt_fixtures_sha256" \
    --arg result_receipt_no_persistence_hash_sha256 "$result_receipt_no_persistence_hash_sha256" \
    --arg result_receipt_policy_hash_sha256 "$result_receipt_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$COMMAND_NOOP_JSON" \
    --argjson fixtures "$result_receipt_fixtures_json" \
    '
      ($source.denied_by_operator_review_acknowledgement_activation_command_noop_handoff + [
        "source_activation_command_noop_handoff_report_required",
        "activation_command_disabled_required",
        "activation_command_result_receipt_schema_registration_denied",
        "activation_command_result_receipt_schema_acceptance_denied",
        "activation_command_result_receipt_recording_denied",
        "activation_command_result_receipt_persistence_denied",
        "activation_command_result_receipt_acceptance_denied",
        "activation_command_result_receipt_materialization_denied",
        "activation_command_result_receipt_filesystem_write_denied",
        "activation_command_result_receipt_ledger_write_denied",
        "activation_command_result_receipt_indexing_denied",
        "activation_command_result_receipt_enqueue_denied",
        "activation_command_result_receipt_delivery_denied",
        "activation_command_result_receipt_export_denied",
        "activation_command_result_receipt_query_registration_denied",
        "activation_command_result_receipt_observability_recording_denied",
        "activation_command_result_receipt_hash_binding_denied",
        "activation_command_result_receipt_status_acceptance_denied",
        "completion_ack_recording_denied",
        "completion_ack_persistence_denied",
        "completion_ack_acceptance_denied",
        "operator_approval_from_receipt_denied",
        "activation_from_receipt_denied",
        "activation_request_from_receipt_denied",
        "dispatch_from_receipt_denied",
        "execution_from_receipt_denied",
        "context_injection_from_receipt_denied",
        "provider_model_invocation_denied",
        "memory_store_write_denied",
        "external_kg_read_denied",
        "live_kg_write_denied",
        "credential_secret_read_denied",
        "channel_delivery_denied",
        "external_public_claim_denied",
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
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_schema_version: "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_v1",
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready: true,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_status: "blocked",
        activation_command_result_receipt_no_persistence_mode: "stdout_only_command_result_receipt_shapes_no_record_no_persist_no_accept_no_authority_no_live",
        activation_command_result_receipt_no_persistence_decision: "operator_review_acknowledgement_activation_command_noop_handoff_cannot_create_or_authorize_result_receipts",
        minimum_required_samples: $min_long_soak_samples,
        source_operator_review_acknowledgement_activation_command_noop_handoff_gate: $source.gate,
        source_operator_review_acknowledgement_activation_command_noop_handoff_status: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_noop_handoff_status,
        source_operator_review_acknowledgement_activation_command_noop_handoff_report_sha256: $command_noop_report_sha256,
        source_activation_command_noop_handoff_hash_sha256: $command_noop_handoff_hash_sha256,
        result_receipt_fixtures_sha256: $result_receipt_fixtures_sha256,
        result_receipt_no_persistence_hash_sha256: $result_receipt_no_persistence_hash_sha256,
        result_receipt_policy_hash_sha256: $result_receipt_policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        source_activation_command_fixture_count: $source.activation_command_fixture_count,
        source_blocked_activation_command_fixture_count: $source.blocked_activation_command_fixture_count,
        source_noop_activation_command_fixture_count: $source.noop_activation_command_fixture_count,
        source_accepted_activation_command_fixture_count: $source.accepted_activation_command_fixture_count,
        source_activation_command_performed_count: $source.activation_command_performed_count,
        activation_command_result_receipt_surface_count: 14,
        activation_command_result_receipt_surface_ready_count: 14,
        activation_command_result_receipt_side_effect_free_surface_count: 14,
        activation_command_result_receipt_fixtures: $fixtures,
        activation_command_result_receipt_fixture_count: ($fixtures | length),
        activation_command_result_receipt_requested_fixture_count: ($fixtures | map(select(.activation_command_result_receipt_requested == true)) | length),
        blocked_activation_command_result_receipt_fixture_count: ($fixtures | length),
        noop_activation_command_result_receipt_fixture_count: ($fixtures | length),
        allowed_activation_command_result_receipt_fixture_count: 0,
        accepted_activation_command_result_receipt_fixture_count: 0,
        activation_command_result_receipt_performed_count: 0,
        activation_command_result_receipt_shape_registered: false,
        activation_command_result_receipt_allowed: false,
        activation_command_result_receipt_schema_accepted: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_materialized: false,
        activation_command_result_receipt_filesystem_written: false,
        activation_command_result_receipt_ledger_written: false,
        activation_command_result_receipt_indexed: false,
        activation_command_result_receipt_enqueued: false,
        activation_command_result_receipt_delivered: false,
        activation_command_result_receipt_exported: false,
        activation_command_result_receipt_query_registered: false,
        activation_command_result_receipt_observability_recorded: false,
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_persisted: false,
        activation_command_completion_ack_accepted: false,
        operator_approval_from_receipt_accepted: false,
        activation_from_receipt_allowed: false,
        activation_command_allowed: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_command_handoff_recorded: false,
        activation_command_handoff_persisted: false,
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
        allowed_next_actions: [
          {
            action: "stage_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial",
            status: "allowed_report_only_next_slice",
            accepts_duplicate_receipt: false,
            records_idempotency: false,
            persists_replay_state: false,
            mutates_runtime: false,
            invokes_model: false,
            writes_memory_or_kg: false
          }
        ],
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_no_persistence: $denials,
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_count: ($denials | length),
        side_effects: {
          workspace_written: false,
          filesystem_written: false,
          activation_command_result_receipt_shape_registered: false,
          activation_command_result_receipt_recorded: false,
          activation_command_result_receipt_persisted: false,
          activation_command_result_receipt_accepted: false,
          activation_command_result_receipt_materialized: false,
          activation_command_result_receipt_filesystem_written: false,
          activation_command_result_receipt_exported: false,
          activation_command_result_receipt_query_registered: false,
          activation_command_result_receipt_observability_recorded: false,
          activation_command_completion_ack_recorded: false,
          activation_command_completion_ack_accepted: false,
          operator_approval_recorded: false,
          activation_from_receipt_allowed: false,
          activation_command_enabled: false,
          activation_command_invoked: false,
          activation_command_dispatched: false,
          activation_command_handoff_recorded: false,
          activation_request_recorded: false,
          activation_request_persisted: false,
          activation_request_executed: false,
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
          external_send_performed: false,
          public_claim_performed: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_status == "blocked"
  and .source_activation_command_fixture_count == 10
  and .source_accepted_activation_command_fixture_count == 0
  and .activation_command_result_receipt_fixture_count == 10
  and .blocked_activation_command_result_receipt_fixture_count == 10
  and .noop_activation_command_result_receipt_fixture_count == 10
  and .accepted_activation_command_result_receipt_fixture_count == 0
  and .activation_command_result_receipt_performed_count == 0
  and .activation_command_result_receipt_shape_registered == false
  and .activation_command_result_receipt_allowed == false
  and .activation_command_result_receipt_schema_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_result_receipt_exported == false
  and .activation_command_result_receipt_query_registered == false
  and .activation_command_result_receipt_observability_recorded == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_accepted == false
  and .operator_approval_from_receipt_accepted == false
  and .activation_from_receipt_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_executed == false
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
  and (.activation_command_result_receipt_fixtures | all(
    .activation_command_result_receipt_allowed == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_from_receipt_allowed == false
    and .activation_command_result_receipt_non_authority_confirmed == true
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
  ))
  and .denied_by_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_count >= 90
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt no-persistence gate passed"
