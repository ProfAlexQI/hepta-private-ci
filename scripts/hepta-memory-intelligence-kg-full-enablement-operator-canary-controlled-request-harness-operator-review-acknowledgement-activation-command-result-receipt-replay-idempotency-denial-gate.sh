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

NO_PERSISTENCE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-no-persistence-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-no-persistence-gate.sh
)"

replay_idempotency_fixtures_json="$(
  jq -n '
    def replay_fixture($id; $status; $reason; $extra):
      {
        fixture_id: $id,
        replay_idempotency_status: $status,
        source_result_receipt_no_persistence_present: true,
        source_result_receipt_no_persistence_ready: true,
        replay_requested: true,
        canonical_blocked_noop_result_receipt_identity_required: true,
        activation_command_result_receipt_replay_allowed: false,
        activation_command_result_receipt_replay_recorded: false,
        activation_command_result_receipt_replay_persisted: false,
        activation_command_result_receipt_replay_materialized: false,
        activation_command_result_receipt_replay_filesystem_written: false,
        activation_command_result_receipt_replay_performed: false,
        activation_command_result_receipt_duplicate_accepted: false,
        activation_command_result_receipt_duplicate_recorded: false,
        activation_command_result_receipt_duplicate_persisted: false,
        activation_command_result_receipt_idempotency_key_accepted: false,
        activation_command_result_receipt_idempotency_key_recorded: false,
        activation_command_result_receipt_idempotency_state_recorded: false,
        activation_command_result_receipt_idempotency_state_persisted: false,
        activation_command_result_receipt_idempotency_state_materialized: false,
        activation_command_result_receipt_idempotency_filesystem_written: false,
        activation_command_result_receipt_replay_nonce_accepted: false,
        activation_command_result_receipt_replay_nonce_recorded: false,
        activation_command_result_receipt_cross_scope_reuse_accepted: false,
        activation_command_result_receipt_status_upgrade_accepted: false,
        activation_command_result_receipt_completed_status_accepted: false,
        activation_command_result_receipt_ack_replay_accepted: false,
        activation_command_result_receipt_ledger_replay_accepted: false,
        activation_command_result_receipt_index_replay_accepted: false,
        activation_command_result_receipt_delivery_replay_accepted: false,
        activation_command_result_receipt_export_replay_accepted: false,
        activation_command_result_receipt_query_replay_accepted: false,
        activation_command_result_receipt_observability_replay_accepted: false,
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
        activation_command_completion_ack_delivered: false,
        operator_approval_from_replay_accepted: false,
        operator_approval_from_receipt_accepted: false,
        activation_from_replay_allowed: false,
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
        receipt_noop_confirmed: true,
        denial_reason: $reason
      } + $extra;
    [
      replay_fixture("missing-source-result-receipt-no-persistence-report"; "blocked_noop"; "source_result_receipt_no_persistence_report_required"; {source_result_receipt_no_persistence_present: false, source_result_receipt_no_persistence_ready: false}),
      replay_fixture("acknowledgement-activation-command-result-receipt-duplicate-identity-replay-attempt"; "blocked_duplicate_noop"; "duplicate_result_receipt_identity_replay_denied"; {duplicate_result_receipt_identity_requested: true}),
      replay_fixture("acknowledgement-activation-command-result-receipt-replay-acceptance-attempt"; "blocked_replay_noop"; "result_receipt_replay_acceptance_denied"; {result_receipt_replay_acceptance_requested: true}),
      replay_fixture("acknowledgement-activation-command-result-receipt-idempotency-key-recording-attempt"; "blocked_idempotency_key_noop"; "idempotency_key_recording_denied"; {idempotency_key_acceptance_requested: true, idempotency_key_recording_requested: true}),
      replay_fixture("acknowledgement-activation-command-result-receipt-idempotency-state-persistence-attempt"; "blocked_idempotency_state_noop"; "idempotency_state_persistence_materialization_denied"; {idempotency_state_recording_requested: true, idempotency_state_persistence_requested: true, idempotency_state_materialization_requested: true, idempotency_filesystem_write_requested: true}),
      replay_fixture("acknowledgement-activation-command-result-receipt-cross-scope-reuse-attempt"; "blocked_cross_scope_noop"; "cross_scope_result_receipt_reuse_denied"; {cross_scope_reuse_requested: true}),
      replay_fixture("acknowledgement-activation-command-result-receipt-stale-nonce-out-of-order-replay-attempt"; "blocked_nonce_order_noop"; "stale_nonce_out_of_order_receipt_replay_denied"; {stale_nonce_replay_requested: true, out_of_order_replay_requested: true, replay_nonce_acceptance_requested: true}),
      replay_fixture("acknowledgement-activation-command-result-receipt-completion-ledger-delivery-replay-attempt"; "blocked_completion_ledger_delivery_noop"; "completion_ack_ledger_delivery_replay_denied"; {completion_ack_replay_requested: true, ledger_replay_requested: true, index_replay_requested: true, delivery_replay_requested: true}),
      replay_fixture("acknowledgement-activation-command-result-receipt-activation-provider-memory-kg-replay-attempt"; "blocked_activation_provider_memory_kg_noop"; "activation_provider_memory_kg_replay_denied"; {result_receipt_status_upgrade_requested: true, completed_status_acceptance_requested: true, operator_approval_from_replay_requested: true, activation_from_replay_requested: true, context_injection_replay_requested: true, provider_replay_requested: true, model_replay_requested: true, memory_store_replay_requested: true, external_kg_replay_requested: true, live_kg_replay_requested: true}),
      replay_fixture("acknowledgement-activation-command-result-receipt-external-public-install-upstream-secret-replay-attempt"; "blocked_external_noop"; "external_public_install_restart_upstream_secret_replay_denied"; {external_send_replay_requested: true, public_claim_replay_requested: true, install_replay_requested: true, launchd_restart_replay_requested: true, service_restart_replay_requested: true, active_binary_mutation_replay_requested: true, upstream_replay_requested: true, credential_replay_requested: true, secret_value_replay_requested: true})
    ]
  '
)"

no_persistence_report_sha256="$(sha256_text "$NO_PERSISTENCE_JSON")"
result_receipt_no_persistence_hash_sha256="$(jq -r '.result_receipt_no_persistence_hash_sha256' <<<"$NO_PERSISTENCE_JSON")"
source_activation_command_noop_handoff_hash_sha256="$(jq -r '.source_activation_command_noop_handoff_hash_sha256' <<<"$NO_PERSISTENCE_JSON")"
replay_idempotency_fixtures_sha256="$(sha256_text "$replay_idempotency_fixtures_json")"
replay_idempotency_contract_hash_sha256="$(
  sha256_text "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial:v1:source=$no_persistence_report_sha256:receipt=$result_receipt_no_persistence_hash_sha256:fixtures=$replay_idempotency_fixtures_sha256:replay=0:duplicate=0:idempotency=0:persist=0:authority=0:live=0"
)"
replay_idempotency_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial:v1:no-replay:no-duplicate:no-idempotency-key:no-idempotency-state:no-cross-scope:no-authority:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_side_effects=false;fixtures=10;replay=0;duplicate=0;idempotency=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$NO_PERSISTENCE_JSON" \
  --argjson fixtures "$replay_idempotency_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_status == "blocked"
    and $source.source_activation_command_fixture_count == 10
    and $source.source_accepted_activation_command_fixture_count == 0
    and $source.activation_command_result_receipt_surface_count == 14
    and $source.activation_command_result_receipt_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_fixture_count == 10
    and $source.noop_activation_command_result_receipt_fixture_count == 10
    and $source.accepted_activation_command_result_receipt_fixture_count == 0
    and $source.activation_command_result_receipt_performed_count == 0
    and $source.activation_command_result_receipt_shape_registered == false
    and $source.activation_command_result_receipt_allowed == false
    and $source.activation_command_result_receipt_schema_accepted == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_materialized == false
    and $source.activation_command_result_receipt_filesystem_written == false
    and $source.activation_command_result_receipt_exported == false
    and $source.activation_command_result_receipt_query_registered == false
    and $source.activation_command_result_receipt_observability_recorded == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.activation_command_completion_ack_accepted == false
    and $source.operator_approval_from_receipt_accepted == false
    and $source.activation_from_receipt_allowed == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_request_accepted == false
    and $source.activation_request_recorded == false
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
    and ($source.allowed_next_actions | any(.action == "stage_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial" and .status == "allowed_report_only_next_slice" and .accepts_duplicate_receipt == false and .records_idempotency == false and .persists_replay_state == false and .writes_memory_or_kg == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.replay_idempotency_status | startswith("blocked_"))
      and .activation_command_result_receipt_replay_allowed == false
      and .activation_command_result_receipt_replay_recorded == false
      and .activation_command_result_receipt_replay_persisted == false
      and .activation_command_result_receipt_replay_performed == false
      and .activation_command_result_receipt_duplicate_accepted == false
      and .activation_command_result_receipt_idempotency_key_accepted == false
      and .activation_command_result_receipt_idempotency_key_recorded == false
      and .activation_command_result_receipt_idempotency_state_recorded == false
      and .activation_command_result_receipt_idempotency_state_persisted == false
      and .activation_command_result_receipt_replay_nonce_accepted == false
      and .activation_command_result_receipt_cross_scope_reuse_accepted == false
      and .activation_command_result_receipt_status_upgrade_accepted == false
      and .activation_command_result_receipt_completed_status_accepted == false
      and .activation_command_result_receipt_ack_replay_accepted == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_command_completion_ack_recorded == false
      and .operator_approval_from_replay_accepted == false
      and .activation_from_replay_allowed == false
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
      and .receipt_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_gate" \
    --arg no_persistence_report_sha256 "$no_persistence_report_sha256" \
    --arg result_receipt_no_persistence_hash_sha256 "$result_receipt_no_persistence_hash_sha256" \
    --arg source_activation_command_noop_handoff_hash_sha256 "$source_activation_command_noop_handoff_hash_sha256" \
    --arg replay_idempotency_fixtures_sha256 "$replay_idempotency_fixtures_sha256" \
    --arg replay_idempotency_contract_hash_sha256 "$replay_idempotency_contract_hash_sha256" \
    --arg replay_idempotency_policy_hash_sha256 "$replay_idempotency_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$NO_PERSISTENCE_JSON" \
    --argjson fixtures "$replay_idempotency_fixtures_json" \
    '
      ($source.denied_by_operator_review_acknowledgement_activation_command_result_receipt_no_persistence + [
        "source_result_receipt_no_persistence_report_required",
        "canonical_blocked_noop_result_receipt_identity_required",
        "duplicate_result_receipt_identity_replay_denied",
        "result_receipt_replay_acceptance_denied",
        "idempotency_key_acceptance_denied",
        "idempotency_key_recording_denied",
        "idempotency_state_recording_denied",
        "idempotency_state_persistence_denied",
        "idempotency_state_materialization_denied",
        "idempotency_filesystem_write_denied",
        "cross_scope_result_receipt_reuse_denied",
        "stale_nonce_replay_denied",
        "out_of_order_receipt_replay_denied",
        "completion_ack_replay_denied",
        "ledger_index_delivery_replay_denied",
        "export_query_observability_replay_denied",
        "status_upgrade_replay_denied",
        "completed_status_replay_denied",
        "operator_approval_from_replay_denied",
        "activation_from_replay_denied",
        "context_injection_replay_denied",
        "provider_model_replay_denied",
        "memory_store_replay_denied",
        "external_kg_replay_denied",
        "live_kg_replay_denied",
        "credential_secret_replay_denied",
        "external_public_install_restart_replay_denied",
        "active_binary_mutation_replay_denied",
        "upstream_replay_denied"
      ]) as $denials |
      {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_schema_version: "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_v1",
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready: true,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status: "blocked",
        replay_idempotency_mode: "stdout_only_duplicate_replay_and_idempotency_denial_no_record_no_persist_no_authority_no_live",
        replay_idempotency_decision: "blocked_noop_activation_command_result_receipt_cannot_be_replayed_duplicated_or_converted_into_idempotency_authority",
        minimum_required_samples: $min_long_soak_samples,
        source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_gate: $source.gate,
        source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_status: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_status,
        source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_report_sha256: $no_persistence_report_sha256,
        source_result_receipt_no_persistence_hash_sha256: $result_receipt_no_persistence_hash_sha256,
        source_activation_command_noop_handoff_hash_sha256: $source_activation_command_noop_handoff_hash_sha256,
        replay_idempotency_fixtures_sha256: $replay_idempotency_fixtures_sha256,
        replay_idempotency_contract_hash_sha256: $replay_idempotency_contract_hash_sha256,
        replay_idempotency_policy_hash_sha256: $replay_idempotency_policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        source_activation_command_result_receipt_surface_count: $source.activation_command_result_receipt_surface_count,
        source_activation_command_result_receipt_fixture_count: $source.activation_command_result_receipt_fixture_count,
        source_blocked_activation_command_result_receipt_fixture_count: $source.blocked_activation_command_result_receipt_fixture_count,
        source_noop_activation_command_result_receipt_fixture_count: $source.noop_activation_command_result_receipt_fixture_count,
        source_accepted_activation_command_result_receipt_fixture_count: $source.accepted_activation_command_result_receipt_fixture_count,
        replay_idempotency_surface_count: 14,
        replay_idempotency_surface_ready_count: 14,
        replay_idempotency_side_effect_free_surface_count: 14,
        replay_idempotency_fixtures: $fixtures,
        replay_idempotency_fixture_count: ($fixtures | length),
        blocked_replay_idempotency_fixture_count: ($fixtures | length),
        noop_replay_idempotency_fixture_count: ($fixtures | length),
        allowed_replay_idempotency_fixture_count: 0,
        accepted_replay_idempotency_fixture_count: 0,
        duplicate_result_receipt_replay_fixture_count: 1,
        result_receipt_replay_acceptance_fixture_count: 1,
        idempotency_key_recording_fixture_count: 1,
        idempotency_state_persistence_fixture_count: 1,
        cross_scope_result_receipt_reuse_fixture_count: 1,
        stale_nonce_out_of_order_replay_fixture_count: 1,
        completion_ledger_delivery_replay_fixture_count: 1,
        activation_provider_memory_kg_replay_fixture_count: 1,
        external_public_install_upstream_secret_replay_fixture_count: 1,
        replay_idempotency_denied_count: 10,
        replay_idempotency_performed_count: 0,
        duplicate_result_receipt_accepted_count: 0,
        idempotency_state_recorded_count: 0,
        idempotency_state_persisted_count: 0,
        activation_command_result_receipt_replay_allowed: false,
        activation_command_result_receipt_replay_recorded: false,
        activation_command_result_receipt_replay_persisted: false,
        activation_command_result_receipt_replay_materialized: false,
        activation_command_result_receipt_replay_filesystem_written: false,
        activation_command_result_receipt_replay_performed: false,
        activation_command_result_receipt_duplicate_accepted: false,
        activation_command_result_receipt_duplicate_recorded: false,
        activation_command_result_receipt_duplicate_persisted: false,
        activation_command_result_receipt_idempotency_key_accepted: false,
        activation_command_result_receipt_idempotency_key_recorded: false,
        activation_command_result_receipt_idempotency_state_recorded: false,
        activation_command_result_receipt_idempotency_state_persisted: false,
        activation_command_result_receipt_idempotency_state_materialized: false,
        activation_command_result_receipt_idempotency_filesystem_written: false,
        activation_command_result_receipt_replay_nonce_accepted: false,
        activation_command_result_receipt_replay_nonce_recorded: false,
        activation_command_result_receipt_cross_scope_reuse_accepted: false,
        activation_command_result_receipt_status_upgrade_accepted: false,
        activation_command_result_receipt_completed_status_accepted: false,
        activation_command_result_receipt_ack_replay_accepted: false,
        activation_command_result_receipt_ledger_replay_accepted: false,
        activation_command_result_receipt_index_replay_accepted: false,
        activation_command_result_receipt_delivery_replay_accepted: false,
        activation_command_result_receipt_export_replay_accepted: false,
        activation_command_result_receipt_query_replay_accepted: false,
        activation_command_result_receipt_observability_replay_accepted: false,
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
        activation_command_completion_ack_delivered: false,
        operator_approval_from_replay_accepted: false,
        operator_approval_from_receipt_accepted: false,
        activation_from_replay_allowed: false,
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
        replay_idempotency_surfaces: [
          "source result receipt no-persistence report required",
          "canonical blocked/no-op result receipt identity required",
          "duplicate receipt rejection required",
          "replay request rejection required",
          "idempotency key/state recording denied",
          "idempotency persistence/materialization denied",
          "cross-scope receipt reuse denied",
          "nonce/order replay denied",
          "completion ack ledger delivery replay denied",
          "status upgrade and activation from replay denied",
          "context/provider/model replay denied",
          "memory/KG replay denied",
          "secret/channel replay denied",
          "external/public/install/restart/upstream replay denied"
        ],
        allowed_next_actions: [
          {
            action: "review_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial",
            status: "allowed_report_only",
            accepts_duplicate_receipt: false,
            records_idempotency: false,
            persists_replay_state: false,
            mutates_runtime: false,
            invokes_model: false,
            writes_memory_or_kg: false
          },
          {
            action: "stage_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial",
            status: "allowed_report_only_next_slice",
            accepts_out_of_order_receipt: false,
            records_sequence_cursor: false,
            persists_ordering_state: false,
            mutates_runtime: false,
            invokes_model: false,
            writes_memory_or_kg: false
          }
        ],
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency: $denials,
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_count: ($denials | length),
        side_effects: {
          workspace_written: false,
          filesystem_written: false,
          activation_command_result_receipt_replay_recorded: false,
          activation_command_result_receipt_replay_persisted: false,
          activation_command_result_receipt_replay_performed: false,
          activation_command_result_receipt_duplicate_accepted: false,
          activation_command_result_receipt_duplicate_recorded: false,
          activation_command_result_receipt_duplicate_persisted: false,
          activation_command_result_receipt_idempotency_key_recorded: false,
          activation_command_result_receipt_idempotency_state_recorded: false,
          activation_command_result_receipt_idempotency_state_persisted: false,
          activation_command_result_receipt_idempotency_state_materialized: false,
          activation_command_result_receipt_idempotency_filesystem_written: false,
          activation_command_result_receipt_replay_nonce_recorded: false,
          activation_command_result_receipt_cross_scope_reuse_accepted: false,
          activation_command_result_receipt_status_upgrade_accepted: false,
          activation_command_result_receipt_completed_status_accepted: false,
          activation_command_result_receipt_ack_replay_accepted: false,
          activation_command_result_receipt_ledger_replay_accepted: false,
          activation_command_result_receipt_index_replay_accepted: false,
          activation_command_result_receipt_delivery_replay_accepted: false,
          activation_command_result_receipt_export_replay_accepted: false,
          activation_command_result_receipt_query_replay_accepted: false,
          activation_command_result_receipt_observability_replay_accepted: false,
          activation_command_result_receipt_recorded: false,
          activation_command_result_receipt_persisted: false,
          activation_command_result_receipt_accepted: false,
          activation_command_result_receipt_materialized: false,
          activation_command_result_receipt_filesystem_written: false,
          activation_command_completion_ack_recorded: false,
          activation_command_completion_ack_persisted: false,
          activation_command_completion_ack_accepted: false,
          operator_approval_from_replay_accepted: false,
          activation_from_replay_allowed: false,
          activation_from_receipt_allowed: false,
          activation_command_enabled: false,
          activation_command_invoked: false,
          activation_command_dispatched: false,
          activation_command_handoff_recorded: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status == "blocked"
  and .source_activation_command_result_receipt_fixture_count == 10
  and .source_accepted_activation_command_result_receipt_fixture_count == 0
  and .replay_idempotency_fixture_count == 10
  and .blocked_replay_idempotency_fixture_count == 10
  and .noop_replay_idempotency_fixture_count == 10
  and .allowed_replay_idempotency_fixture_count == 0
  and .accepted_replay_idempotency_fixture_count == 0
  and .replay_idempotency_performed_count == 0
  and .duplicate_result_receipt_accepted_count == 0
  and .idempotency_state_recorded_count == 0
  and .activation_command_result_receipt_replay_allowed == false
  and .activation_command_result_receipt_replay_recorded == false
  and .activation_command_result_receipt_replay_persisted == false
  and .activation_command_result_receipt_replay_performed == false
  and .activation_command_result_receipt_duplicate_accepted == false
  and .activation_command_result_receipt_idempotency_key_accepted == false
  and .activation_command_result_receipt_idempotency_state_recorded == false
  and .activation_command_result_receipt_idempotency_state_persisted == false
  and .activation_command_result_receipt_replay_nonce_accepted == false
  and .activation_command_result_receipt_cross_scope_reuse_accepted == false
  and .activation_command_result_receipt_status_upgrade_accepted == false
  and .activation_command_result_receipt_completed_status_accepted == false
  and .activation_command_result_receipt_ack_replay_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .operator_approval_from_replay_accepted == false
  and .activation_from_replay_allowed == false
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
  and (.replay_idempotency_fixtures | all(
    .activation_command_result_receipt_replay_allowed == false
    and .activation_command_result_receipt_duplicate_accepted == false
    and .activation_command_result_receipt_idempotency_state_recorded == false
    and .activation_command_result_receipt_idempotency_state_persisted == false
    and .activation_command_result_receipt_cross_scope_reuse_accepted == false
    and .activation_from_replay_allowed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .receipt_noop_confirmed == true
  ))
  and .denied_by_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_count >= 110
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt replay/idempotency denial gate passed"
