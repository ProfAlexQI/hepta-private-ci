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

ORDERING_MONOTONICITY_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh
)"

cancellation_supersession_fixtures_json="$(
  jq -n '
    def cancellation_supersession_fixture($id; $status; $reason; $extra):
      {
        fixture_id: $id,
        cancellation_supersession_status: $status,
        source_ordering_monotonicity_present: true,
        source_ordering_monotonicity_ready: true,
        cancellation_requested: true,
        supersession_requested: false,
        canonical_blocked_noop_result_receipt_identity_required: true,
        activation_command_result_receipt_cancellation_allowed: false,
        activation_command_result_receipt_cancellation_recorded: false,
        activation_command_result_receipt_cancellation_persisted: false,
        activation_command_result_receipt_cancellation_materialized: false,
        activation_command_result_receipt_cancellation_filesystem_written: false,
        activation_command_result_receipt_cancellation_request_accepted: false,
        activation_command_result_receipt_supersession_allowed: false,
        activation_command_result_receipt_supersession_recorded: false,
        activation_command_result_receipt_supersession_persisted: false,
        activation_command_result_receipt_supersession_materialized: false,
        activation_command_result_receipt_supersession_filesystem_written: false,
        activation_command_result_receipt_supersession_request_accepted: false,
        activation_command_result_receipt_replacement_receipt_accepted: false,
        activation_command_result_receipt_replacement_receipt_recorded: false,
        activation_command_result_receipt_replacement_receipt_persisted: false,
        activation_command_result_receipt_replacement_hash_accepted: false,
        activation_command_result_receipt_tombstone_recorded: false,
        activation_command_result_receipt_tombstone_persisted: false,
        activation_command_result_receipt_delete_marker_recorded: false,
        activation_command_result_receipt_ack_cancellation_accepted: false,
        activation_command_result_receipt_ledger_cancellation_accepted: false,
        activation_command_result_receipt_index_cancellation_accepted: false,
        activation_command_result_receipt_delivery_cancellation_accepted: false,
        activation_command_result_receipt_export_cancellation_accepted: false,
        activation_command_result_receipt_query_cancellation_accepted: false,
        activation_command_result_receipt_observability_cancellation_accepted: false,
        activation_command_result_receipt_ordering_allowed: false,
        activation_command_result_receipt_ordering_recorded: false,
        activation_command_result_receipt_ordering_persisted: false,
        activation_command_result_receipt_sequence_cursor_accepted: false,
        activation_command_result_receipt_sequence_cursor_recorded: false,
        activation_command_result_receipt_sequence_cursor_persisted: false,
        activation_command_result_receipt_monotonicity_state_recorded: false,
        activation_command_result_receipt_monotonicity_state_persisted: false,
        activation_command_result_receipt_latest_wins_overwrite_accepted: false,
        activation_command_result_receipt_same_sequence_hash_override_accepted: false,
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
        operator_approval_from_cancellation_accepted: false,
        operator_approval_from_supersession_accepted: false,
        activation_from_cancellation_allowed: false,
        activation_from_supersession_allowed: false,
        activation_from_ordering_allowed: false,
        activation_from_replay_allowed: false,
        activation_from_receipt_allowed: false,
        activation_command_allowed: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
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
        memory_store_mutated: false,
        external_kg_adapter_read_performed: false,
        live_kg_write_performed: false,
        credential_read: false,
        secret_file_read: false,
        auth_secret_read: false,
        secret_value_read: false,
        raw_payload_plaintext_recorded: false,
        raw_payload_plaintext_persisted: false,
        channel_send_performed: false,
        telegram_send_performed: false,
        external_send_performed: false,
        public_claim_performed: false,
        public_release_claimed: false,
        public_ga_claimed: false,
        release_artifact_written: false,
        install_performed: false,
        install_executed: false,
        launchd_mutated: false,
        service_restarted: false,
        service_restart_performed: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        rollback_executed: false,
        receipt_noop_confirmed: true,
        denial_reason: $reason
      } + $extra;
    [
      cancellation_supersession_fixture("missing-source-ordering-monotonicity-report"; "blocked_noop"; "source_result_receipt_ordering_monotonicity_report_required"; {source_ordering_monotonicity_present: false, source_ordering_monotonicity_ready: false}),
      cancellation_supersession_fixture("cancel-blocked-noop-result-receipt"; "blocked_cancellation_noop"; "cancellation_of_blocked_noop_result_receipt_denied"; {cancellation_request_shape: "cancel_blocked_noop_result_receipt"}),
      cancellation_supersession_fixture("supersede-blocked-noop-with-completed-result-receipt"; "blocked_supersession_noop"; "supersession_of_blocked_noop_with_completed_result_receipt_denied"; {supersession_requested: true, cancellation_requested: false, requested_replacement_status: "completed"}),
      cancellation_supersession_fixture("replacement-receipt-recording-persistence-attempt"; "blocked_supersession_noop"; "replacement_receipt_recording_persistence_denied"; {supersession_requested: true, cancellation_requested: false, replacement_receipt_requested: true, replacement_hash_requested: true, requested_hash_relation: "different_hash_for_same_receipt_identity"}),
      cancellation_supersession_fixture("tombstone-delete-marker-attempt"; "blocked_cancellation_noop"; "tombstone_delete_marker_denied"; {tombstone_requested: true, delete_marker_requested: true}),
      cancellation_supersession_fixture("completion-acknowledgement-cancellation-replacement-attempt"; "blocked_cancellation_supersession_noop"; "completion_acknowledgement_cancellation_replacement_denied"; {completion_ack_cancellation_requested: true, ack_cancellation_requested: true, supersession_requested: true, requested_ack_replacement_status: "accepted"}),
      cancellation_supersession_fixture("ledger-index-delivery-export-query-observability-bypass-attempt"; "blocked_ledger_index_delivery_noop"; "ledger_index_delivery_export_query_observability_cancellation_supersession_bypass_denied"; {ledger_cancellation_requested: true, index_cancellation_requested: true, delivery_cancellation_requested: true, export_cancellation_requested: true, query_cancellation_requested: true, observability_cancellation_requested: true}),
      cancellation_supersession_fixture("context-provider-model-memory-kg-supersession-attempt"; "blocked_context_provider_memory_kg_noop"; "context_provider_model_memory_kg_supersession_denied"; {supersession_requested: true, cancellation_requested: false, context_injection_supersession_requested: true, provider_supersession_requested: true, model_supersession_requested: true, memory_store_supersession_requested: true, external_kg_supersession_requested: true, live_kg_supersession_requested: true}),
      cancellation_supersession_fixture("rollback-secret-external-public-install-supersession-attempt"; "blocked_secret_external_install_noop"; "rollback_secret_external_public_install_supersession_denied"; {supersession_requested: true, cancellation_requested: false, rollback_supersession_requested: true, credential_secret_supersession_requested: true, external_send_supersession_requested: true, public_claim_supersession_requested: true, release_artifact_supersession_requested: true, install_supersession_requested: true, service_restart_supersession_requested: true, active_binary_mutation_supersession_requested: true, upstream_supersession_requested: true}),
      cancellation_supersession_fixture("latest-wins-sequence-cursor-cancellation-supersession-bypass-attempt"; "blocked_latest_wins_cursor_noop"; "latest_wins_sequence_cursor_cancellation_supersession_bypass_denied"; {latest_wins_cancellation_bypass_requested: true, latest_wins_supersession_bypass_requested: true, sequence_cursor_cancellation_bypass_requested: true, monotonicity_state_supersession_bypass_requested: true})
    ]
  '
)"

ordering_monotonicity_report_sha256="$(sha256_text "$ORDERING_MONOTONICITY_JSON")"
ordering_monotonicity_contract_hash_sha256="$(jq -r '.ordering_monotonicity_contract_hash_sha256' <<<"$ORDERING_MONOTONICITY_JSON")"
ordering_monotonicity_policy_hash_sha256="$(jq -r '.ordering_monotonicity_policy_hash_sha256' <<<"$ORDERING_MONOTONICITY_JSON")"
source_replay_idempotency_report_sha256="$(jq -r '.source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_report_sha256' <<<"$ORDERING_MONOTONICITY_JSON")"
cancellation_supersession_fixtures_sha256="$(sha256_text "$cancellation_supersession_fixtures_json")"
cancellation_supersession_contract_hash_sha256="$(
  sha256_text "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial:v1:source=$ordering_monotonicity_report_sha256:ordering=$ordering_monotonicity_contract_hash_sha256:replay=$source_replay_idempotency_report_sha256:fixtures=$cancellation_supersession_fixtures_sha256:cancel=0:supersede=0:replace=0:persist=0:authority=0:live=0"
)"
cancellation_supersession_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial:v1:no-cancel:no-supersede:no-replacement:no-tombstone:no-delete:no-authority:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_side_effects=false;fixtures=10;cancel=0;supersede=0;replacement=0;tombstone=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ORDERING_MONOTONICITY_JSON" \
  --argjson fixtures "$cancellation_supersession_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status == "blocked"
    and $source.ordering_monotonicity_fixture_count == 10
    and $source.blocked_ordering_monotonicity_fixture_count == 10
    and $source.noop_ordering_monotonicity_fixture_count == 10
    and $source.allowed_ordering_monotonicity_fixture_count == 0
    and $source.accepted_ordering_monotonicity_fixture_count == 0
    and $source.ordering_monotonicity_performed_count == 0
    and $source.sequence_cursor_accepted_count == 0
    and $source.sequence_cursor_recorded_count == 0
    and $source.monotonicity_state_recorded_count == 0
    and $source.monotonicity_state_persisted_count == 0
    and $source.activation_command_result_receipt_ordering_allowed == false
    and $source.activation_command_result_receipt_ordering_recorded == false
    and $source.activation_command_result_receipt_ordering_persisted == false
    and $source.activation_command_result_receipt_sequence_cursor_accepted == false
    and $source.activation_command_result_receipt_sequence_cursor_recorded == false
    and $source.activation_command_result_receipt_sequence_cursor_persisted == false
    and $source.activation_command_result_receipt_monotonicity_state_recorded == false
    and $source.activation_command_result_receipt_monotonicity_state_persisted == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.operator_approval_from_ordering_accepted == false
    and $source.activation_from_ordering_allowed == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_request_accepted == false
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
    and ($source.side_effects | to_entries | all(.value == false))
    and ($source.allowed_next_actions | any(.action == "stage_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial" and .status == "allowed_report_only_next_slice" and .accepts_cancellation == false and .accepts_supersession == false and .persists_replacement_receipt == false and .mutates_runtime == false and .invokes_model == false and .writes_memory_or_kg == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.cancellation_supersession_status | startswith("blocked_"))
      and .activation_command_result_receipt_cancellation_allowed == false
      and .activation_command_result_receipt_cancellation_recorded == false
      and .activation_command_result_receipt_cancellation_persisted == false
      and .activation_command_result_receipt_cancellation_request_accepted == false
      and .activation_command_result_receipt_supersession_allowed == false
      and .activation_command_result_receipt_supersession_recorded == false
      and .activation_command_result_receipt_supersession_persisted == false
      and .activation_command_result_receipt_supersession_request_accepted == false
      and .activation_command_result_receipt_replacement_receipt_accepted == false
      and .activation_command_result_receipt_replacement_receipt_recorded == false
      and .activation_command_result_receipt_replacement_receipt_persisted == false
      and .activation_command_result_receipt_replacement_hash_accepted == false
      and .activation_command_result_receipt_tombstone_recorded == false
      and .activation_command_result_receipt_delete_marker_recorded == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_command_completion_ack_recorded == false
      and .operator_approval_from_cancellation_accepted == false
      and .operator_approval_from_supersession_accepted == false
      and .activation_from_cancellation_allowed == false
      and .activation_from_supersession_allowed == false
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
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_gate" \
    --arg ordering_monotonicity_report_sha256 "$ordering_monotonicity_report_sha256" \
    --arg ordering_monotonicity_contract_hash_sha256 "$ordering_monotonicity_contract_hash_sha256" \
    --arg ordering_monotonicity_policy_hash_sha256 "$ordering_monotonicity_policy_hash_sha256" \
    --arg source_replay_idempotency_report_sha256 "$source_replay_idempotency_report_sha256" \
    --arg cancellation_supersession_fixtures_sha256 "$cancellation_supersession_fixtures_sha256" \
    --arg cancellation_supersession_contract_hash_sha256 "$cancellation_supersession_contract_hash_sha256" \
    --arg cancellation_supersession_policy_hash_sha256 "$cancellation_supersession_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ORDERING_MONOTONICITY_JSON" \
    --argjson fixtures "$cancellation_supersession_fixtures_json" \
    '
      ($source.denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity + [
        "source_result_receipt_ordering_monotonicity_report_required",
        "canonical_blocked_noop_result_receipt_identity_required",
        "cancellation_request_acceptance_denied",
        "cancellation_recording_denied",
        "cancellation_persistence_denied",
        "cancellation_materialization_denied",
        "cancellation_filesystem_write_denied",
        "supersession_request_acceptance_denied",
        "supersession_recording_denied",
        "supersession_persistence_denied",
        "supersession_materialization_denied",
        "supersession_filesystem_write_denied",
        "replacement_receipt_acceptance_denied",
        "replacement_receipt_recording_denied",
        "replacement_receipt_persistence_denied",
        "replacement_hash_acceptance_denied",
        "tombstone_recording_denied",
        "delete_marker_recording_denied",
        "completion_acknowledgement_cancellation_denied",
        "ledger_index_delivery_cancellation_denied",
        "export_query_observability_cancellation_denied",
        "context_provider_model_supersession_denied",
        "memory_kg_supersession_denied",
        "rollback_secret_supersession_denied",
        "external_public_release_supersession_denied",
        "install_restart_active_binary_supersession_denied",
        "upstream_supersession_denied",
        "latest_wins_cancellation_supersession_bypass_denied",
        "sequence_cursor_cancellation_supersession_bypass_denied",
        "operator_approval_from_cancellation_supersession_denied",
        "activation_from_cancellation_supersession_denied"
      ]) as $denials |
      {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_schema_version: "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_v1",
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready: true,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status: "blocked",
        cancellation_supersession_mode: "stdout_only_cancellation_supersession_denial_no_record_no_persist_no_replacement_no_authority_no_live",
        cancellation_supersession_decision: "blocked_noop_activation_command_result_receipt_cannot_be_cancelled_superseded_replaced_or_promoted_to_authority",
        minimum_required_samples: $min_long_soak_samples,
        source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_gate: $source.gate,
        source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_status: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status,
        source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_report_sha256: $ordering_monotonicity_report_sha256,
        source_ordering_monotonicity_contract_hash_sha256: $ordering_monotonicity_contract_hash_sha256,
        source_ordering_monotonicity_policy_hash_sha256: $ordering_monotonicity_policy_hash_sha256,
        source_replay_idempotency_report_sha256: $source_replay_idempotency_report_sha256,
        cancellation_supersession_fixtures_sha256: $cancellation_supersession_fixtures_sha256,
        cancellation_supersession_contract_hash_sha256: $cancellation_supersession_contract_hash_sha256,
        cancellation_supersession_policy_hash_sha256: $cancellation_supersession_policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        source_ordering_monotonicity_fixture_count: $source.ordering_monotonicity_fixture_count,
        source_blocked_ordering_monotonicity_fixture_count: $source.blocked_ordering_monotonicity_fixture_count,
        source_noop_ordering_monotonicity_fixture_count: $source.noop_ordering_monotonicity_fixture_count,
        source_accepted_ordering_monotonicity_fixture_count: $source.accepted_ordering_monotonicity_fixture_count,
        source_ordering_monotonicity_performed_count: $source.ordering_monotonicity_performed_count,
        source_sequence_cursor_accepted_count: $source.sequence_cursor_accepted_count,
        source_sequence_cursor_recorded_count: $source.sequence_cursor_recorded_count,
        source_monotonicity_state_recorded_count: $source.monotonicity_state_recorded_count,
        source_monotonicity_state_persisted_count: $source.monotonicity_state_persisted_count,
        cancellation_supersession_surface_count: 14,
        cancellation_supersession_surface_ready_count: 14,
        cancellation_supersession_side_effect_free_surface_count: 14,
        cancellation_supersession_fixtures: $fixtures,
        cancellation_supersession_fixture_count: ($fixtures | length),
        blocked_cancellation_supersession_fixture_count: ($fixtures | length),
        noop_cancellation_supersession_fixture_count: ($fixtures | length),
        allowed_cancellation_supersession_fixture_count: 0,
        accepted_cancellation_supersession_fixture_count: 0,
        cancellation_fixture_count: ($fixtures | map(select(.cancellation_requested == true)) | length),
        supersession_fixture_count: ($fixtures | map(select(.supersession_requested == true)) | length),
        cancellation_denied_count: ($fixtures | map(select(.cancellation_requested == true)) | length),
        supersession_denied_count: ($fixtures | map(select(.supersession_requested == true)) | length),
        cancellation_performed_count: 0,
        supersession_performed_count: 0,
        replacement_receipt_accepted_count: 0,
        replacement_receipt_recorded_count: 0,
        replacement_receipt_persisted_count: 0,
        tombstone_recorded_count: 0,
        delete_marker_recorded_count: 0,
        activation_command_result_receipt_cancellation_allowed: false,
        activation_command_result_receipt_cancellation_recorded: false,
        activation_command_result_receipt_cancellation_persisted: false,
        activation_command_result_receipt_cancellation_materialized: false,
        activation_command_result_receipt_cancellation_filesystem_written: false,
        activation_command_result_receipt_cancellation_request_accepted: false,
        activation_command_result_receipt_supersession_allowed: false,
        activation_command_result_receipt_supersession_recorded: false,
        activation_command_result_receipt_supersession_persisted: false,
        activation_command_result_receipt_supersession_materialized: false,
        activation_command_result_receipt_supersession_filesystem_written: false,
        activation_command_result_receipt_supersession_request_accepted: false,
        activation_command_result_receipt_replacement_receipt_accepted: false,
        activation_command_result_receipt_replacement_receipt_recorded: false,
        activation_command_result_receipt_replacement_receipt_persisted: false,
        activation_command_result_receipt_replacement_hash_accepted: false,
        activation_command_result_receipt_tombstone_recorded: false,
        activation_command_result_receipt_tombstone_persisted: false,
        activation_command_result_receipt_delete_marker_recorded: false,
        activation_command_result_receipt_ack_cancellation_accepted: false,
        activation_command_result_receipt_ledger_cancellation_accepted: false,
        activation_command_result_receipt_index_cancellation_accepted: false,
        activation_command_result_receipt_delivery_cancellation_accepted: false,
        activation_command_result_receipt_export_cancellation_accepted: false,
        activation_command_result_receipt_query_cancellation_accepted: false,
        activation_command_result_receipt_observability_cancellation_accepted: false,
        activation_command_result_receipt_ordering_allowed: false,
        activation_command_result_receipt_ordering_recorded: false,
        activation_command_result_receipt_ordering_persisted: false,
        activation_command_result_receipt_sequence_cursor_accepted: false,
        activation_command_result_receipt_sequence_cursor_recorded: false,
        activation_command_result_receipt_sequence_cursor_persisted: false,
        activation_command_result_receipt_monotonicity_state_recorded: false,
        activation_command_result_receipt_monotonicity_state_persisted: false,
        activation_command_result_receipt_latest_wins_overwrite_accepted: false,
        activation_command_result_receipt_same_sequence_hash_override_accepted: false,
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
        operator_approval_from_cancellation_accepted: false,
        operator_approval_from_supersession_accepted: false,
        operator_approval_from_ordering_accepted: false,
        operator_approval_from_replay_accepted: false,
        operator_approval_from_receipt_accepted: false,
        activation_from_cancellation_allowed: false,
        activation_from_supersession_allowed: false,
        activation_from_ordering_allowed: false,
        activation_from_replay_allowed: false,
        activation_from_receipt_allowed: false,
        activation_command_allowed: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
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
            action: "review_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial",
            status: "allowed_report_only",
            accepts_cancellation: false,
            accepts_supersession: false,
            persists_replacement_receipt: false,
            mutates_runtime: false,
            invokes_model: false,
            writes_memory_or_kg: false
          },
          {
            action: "stage_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
            status: "allowed_report_only_next_slice",
            accepts_cancellation: false,
            accepts_supersession: false,
            writes_audit_trail: false,
            persists_evidence: false,
            mutates_runtime: false,
            invokes_model: false,
            writes_memory_or_kg: false
          }
        ],
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession: $denials,
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_count: ($denials | length),
        side_effects: {
          workspace_written: false,
          filesystem_written: false,
          activation_command_result_receipt_cancellation_recorded: false,
          activation_command_result_receipt_cancellation_persisted: false,
          activation_command_result_receipt_cancellation_materialized: false,
          activation_command_result_receipt_cancellation_filesystem_written: false,
          activation_command_result_receipt_supersession_recorded: false,
          activation_command_result_receipt_supersession_persisted: false,
          activation_command_result_receipt_supersession_materialized: false,
          activation_command_result_receipt_supersession_filesystem_written: false,
          activation_command_result_receipt_replacement_receipt_recorded: false,
          activation_command_result_receipt_replacement_receipt_persisted: false,
          activation_command_result_receipt_replacement_hash_accepted: false,
          activation_command_result_receipt_tombstone_recorded: false,
          activation_command_result_receipt_tombstone_persisted: false,
          activation_command_result_receipt_delete_marker_recorded: false,
          activation_command_result_receipt_ack_cancellation_accepted: false,
          activation_command_result_receipt_ledger_cancellation_accepted: false,
          activation_command_result_receipt_index_cancellation_accepted: false,
          activation_command_result_receipt_delivery_cancellation_accepted: false,
          activation_command_result_receipt_export_cancellation_accepted: false,
          activation_command_result_receipt_query_cancellation_accepted: false,
          activation_command_result_receipt_observability_cancellation_accepted: false,
          activation_command_result_receipt_ordering_recorded: false,
          activation_command_result_receipt_ordering_persisted: false,
          activation_command_result_receipt_sequence_cursor_recorded: false,
          activation_command_result_receipt_sequence_cursor_persisted: false,
          activation_command_result_receipt_monotonicity_state_recorded: false,
          activation_command_result_receipt_monotonicity_state_persisted: false,
          activation_command_result_receipt_recorded: false,
          activation_command_result_receipt_persisted: false,
          activation_command_result_receipt_accepted: false,
          activation_command_result_receipt_materialized: false,
          activation_command_completion_ack_recorded: false,
          activation_command_completion_ack_accepted: false,
          operator_approval_from_cancellation_accepted: false,
          operator_approval_from_supersession_accepted: false,
          activation_from_cancellation_allowed: false,
          activation_from_supersession_allowed: false,
          activation_command_enabled: false,
          activation_command_invoked: false,
          activation_command_dispatched: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status == "blocked"
  and .source_ordering_monotonicity_fixture_count == 10
  and .source_accepted_ordering_monotonicity_fixture_count == 0
  and .source_ordering_monotonicity_performed_count == 0
  and .source_sequence_cursor_accepted_count == 0
  and .source_sequence_cursor_recorded_count == 0
  and .source_monotonicity_state_recorded_count == 0
  and .source_monotonicity_state_persisted_count == 0
  and .cancellation_supersession_fixture_count == 10
  and .blocked_cancellation_supersession_fixture_count == 10
  and .noop_cancellation_supersession_fixture_count == 10
  and .allowed_cancellation_supersession_fixture_count == 0
  and .accepted_cancellation_supersession_fixture_count == 0
  and .cancellation_performed_count == 0
  and .supersession_performed_count == 0
  and .replacement_receipt_accepted_count == 0
  and .replacement_receipt_recorded_count == 0
  and .replacement_receipt_persisted_count == 0
  and .tombstone_recorded_count == 0
  and .delete_marker_recorded_count == 0
  and .activation_command_result_receipt_cancellation_allowed == false
  and .activation_command_result_receipt_cancellation_recorded == false
  and .activation_command_result_receipt_cancellation_persisted == false
  and .activation_command_result_receipt_cancellation_request_accepted == false
  and .activation_command_result_receipt_supersession_allowed == false
  and .activation_command_result_receipt_supersession_recorded == false
  and .activation_command_result_receipt_supersession_persisted == false
  and .activation_command_result_receipt_supersession_request_accepted == false
  and .activation_command_result_receipt_replacement_receipt_accepted == false
  and .activation_command_result_receipt_replacement_receipt_recorded == false
  and .activation_command_result_receipt_replacement_receipt_persisted == false
  and .activation_command_result_receipt_replacement_hash_accepted == false
  and .activation_command_result_receipt_tombstone_recorded == false
  and .activation_command_result_receipt_delete_marker_recorded == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .operator_approval_from_cancellation_accepted == false
  and .operator_approval_from_supersession_accepted == false
  and .activation_from_cancellation_allowed == false
  and .activation_from_supersession_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
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
  and (.cancellation_supersession_fixtures | all(
    .activation_command_result_receipt_cancellation_allowed == false
    and .activation_command_result_receipt_cancellation_recorded == false
    and .activation_command_result_receipt_cancellation_persisted == false
    and .activation_command_result_receipt_supersession_allowed == false
    and .activation_command_result_receipt_supersession_recorded == false
    and .activation_command_result_receipt_supersession_persisted == false
    and .activation_command_result_receipt_replacement_receipt_accepted == false
    and .activation_command_result_receipt_replacement_receipt_recorded == false
    and .activation_command_result_receipt_replacement_receipt_persisted == false
    and .operator_approval_from_cancellation_accepted == false
    and .operator_approval_from_supersession_accepted == false
    and .activation_from_cancellation_allowed == false
    and .activation_from_supersession_allowed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .receipt_noop_confirmed == true
  ))
  and .denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_count >= 160
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt cancellation/supersession denial gate passed"
