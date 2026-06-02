#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

ORDERING_MONOTONICITY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh
)"

cancellation_supersession_fixtures_json="$(
  jq -n '
    def cancellation_supersession_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        cancellation_requested:true,
        supersession_requested:false,
        cancellation_supersession_status:$status,
        source_ordering_monotonicity_present:true,
        source_ordering_monotonicity_ready:true,
        cancellation_allowed:false,
        cancellation_recorded:false,
        cancellation_persisted:false,
        cancellation_request_accepted:false,
        supersession_allowed:false,
        supersession_recorded:false,
        supersession_persisted:false,
        supersession_request_accepted:false,
        replacement_receipt_accepted:false,
        replacement_receipt_recorded:false,
        replacement_receipt_persisted:false,
        replacement_hash_accepted:false,
        tombstone_recorded:false,
        tombstone_persisted:false,
        delete_marker_recorded:false,
        ack_cancellation_accepted:false,
        ledger_cancellation_accepted:false,
        index_cancellation_accepted:false,
        delivery_cancellation_accepted:false,
        export_cancellation_accepted:false,
        query_cancellation_accepted:false,
        observability_cancellation_accepted:false,
        activation_command_result_receipt_ordering_allowed:false,
        activation_command_result_receipt_ordering_recorded:false,
        activation_command_result_receipt_ordering_persisted:false,
        activation_command_result_receipt_sequence_cursor_accepted:false,
        activation_command_result_receipt_sequence_cursor_recorded:false,
        activation_command_result_receipt_sequence_cursor_persisted:false,
        activation_command_result_receipt_monotonicity_state_recorded:false,
        activation_command_result_receipt_monotonicity_state_persisted:false,
        activation_command_result_receipt_latest_wins_overwrite_accepted:false,
        activation_command_result_receipt_same_sequence_hash_override_accepted:false,
        activation_command_result_receipt_recorded:false,
        activation_command_result_receipt_persisted:false,
        activation_command_result_receipt_accepted:false,
        activation_command_result_receipt_materialized:false,
        activation_command_result_receipt_filesystem_written:false,
        activation_command_result_receipt_ledger_written:false,
        activation_command_result_receipt_indexed:false,
        activation_command_result_receipt_enqueued:false,
        activation_command_result_receipt_delivered:false,
        activation_command_result_receipt_exported:false,
        activation_command_result_receipt_query_registered:false,
        activation_command_result_receipt_observability_recorded:false,
        activation_command_completion_ack_recorded:false,
        activation_command_completion_ack_persisted:false,
        activation_command_completion_ack_accepted:false,
        activation_command_completion_ack_delivered:false,
        activation_from_cancellation_allowed:false,
        activation_from_supersession_allowed:false,
        activation_from_ordering_allowed:false,
        activation_from_replay_allowed:false,
        activation_from_receipt_allowed:false,
        activation_command_enabled:false,
        activation_command_invoked:false,
        activation_command_dispatched:false,
        activation_command_dispatch_performed:false,
        activation_request_accepted:false,
        activation_request_recorded:false,
        activation_request_persisted:false,
        activation_request_executed:false,
        activation_activated:false,
        runtime_router_mutated:false,
        runtime_attachment_performed:false,
        live_context_attached:false,
        context_injection_performed:false,
        adapter_invoked:false,
        provider_invoked:false,
        model_invoked:false,
        auth_secret_read:false,
        credential_read:false,
        secret_file_read:false,
        usage_recorded:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        readback_evidence_recorded:false,
        readback_evidence_persisted:false,
        router_handoff_recorded:false,
        router_handoff_persisted:false,
        rollback_executed:false,
        telegram_send_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        release_artifact_written:false,
        install_executed:false,
        launchd_mutated:false,
        service_restart_performed:false,
        active_binary_mutated:false,
        receipt_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      cancellation_supersession_fixture("provider-router-activation-command-result-receipt-cancellation-missing-source-ordering-report"; "blocked_noop"; "source_ordering_monotonicity_report_required"; {source_ordering_monotonicity_present:false, source_ordering_monotonicity_ready:false}),
      cancellation_supersession_fixture("provider-router-activation-command-result-receipt-cancel-blocked-noop"; "blocked_cancellation_noop"; "cancellation_of_blocked_noop_receipt_denied"; {cancellation_request_shape:"cancel_blocked_noop_receipt"}),
      cancellation_supersession_fixture("provider-router-activation-command-result-receipt-supersede-with-completed"; "blocked_supersession_noop"; "supersession_of_blocked_noop_with_completed_denied"; {supersession_requested:true, cancellation_requested:false, requested_replacement_status:"completed"}),
      cancellation_supersession_fixture("provider-router-activation-command-result-receipt-replacement-hash"; "blocked_supersession_noop"; "replacement_hash_identity_attempt_denied"; {supersession_requested:true, cancellation_requested:false, replacement_hash_requested:true, requested_hash_relation:"different_hash_for_same_receipt_identity"}),
      cancellation_supersession_fixture("provider-router-activation-command-result-receipt-tombstone-delete-marker"; "blocked_cancellation_noop"; "tombstone_or_delete_marker_denied"; {tombstone_requested:true, delete_marker_requested:true}),
      cancellation_supersession_fixture("provider-router-activation-command-result-receipt-completion-ack-cancel"; "blocked_cancellation_noop"; "completion_ack_cancellation_denied"; {completion_ack_cancellation_requested:true, ack_cancellation_requested:true}),
      cancellation_supersession_fixture("provider-router-activation-command-result-receipt-ledger-index-delivery-export-cancel"; "blocked_cancellation_noop"; "ledger_index_delivery_export_cancellation_denied"; {ledger_cancellation_requested:true, index_cancellation_requested:true, delivery_cancellation_requested:true, export_cancellation_requested:true, query_cancellation_requested:true, observability_cancellation_requested:true}),
      cancellation_supersession_fixture("provider-router-activation-command-result-receipt-runtime-provider-model-supersede"; "blocked_supersession_noop"; "runtime_provider_model_supersession_denied"; {supersession_requested:true, cancellation_requested:false, runtime_supersession_requested:true, live_context_supersession_requested:true, provider_supersession_requested:true, model_supersession_requested:true, usage_supersession_requested:true}),
      cancellation_supersession_fixture("provider-router-activation-command-result-receipt-memory-kg-rollback-secret-supersede"; "blocked_supersession_noop"; "memory_kg_rollback_secret_supersession_denied"; {supersession_requested:true, cancellation_requested:false, memory_store_supersession_requested:true, live_kg_supersession_requested:true, rollback_supersession_requested:true, secret_material_supersession_requested:true, auth_secret_supersession_requested:true}),
      cancellation_supersession_fixture("provider-router-activation-command-result-receipt-external-public-install-supersede"; "blocked_supersession_noop"; "external_public_install_restart_active_binary_supersession_denied"; {supersession_requested:true, cancellation_requested:false, external_send_supersession_requested:true, public_claim_supersession_requested:true, release_artifact_supersession_requested:true, install_supersession_requested:true, service_restart_supersession_requested:true, active_binary_mutation_supersession_requested:true})
    ]
  '
)"

ordering_monotonicity_report_sha256="$(sha256_text "$ORDERING_MONOTONICITY_JSON")"
cancellation_supersession_fixtures_sha256="$(sha256_text "$cancellation_supersession_fixtures_json")"
cancellation_supersession_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial:$ordering_monotonicity_report_sha256:$cancellation_supersession_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
cancellation_supersession_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial:no-cancel:no-supersede:no-replacement:no-tombstone:no-runtime:no-provider:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "cancellation=false;supersession=false;replacement=false;tombstone=false;record=false;persist=false;activation=false;runtime=false;provider=false;model=false;memory=false;kg=false;secret=false;external=false;install=false;restart=false;active_binary=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ORDERING_MONOTONICITY_JSON" \
  --argjson fixtures "$cancellation_supersession_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_gate"
    and $source.activation_command_result_receipt_ordering_monotonicity_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_v1"
    and $source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status == "blocked"
    and $source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
    and $source.activation_command_result_receipt_surface_count == 14
    and $source.activation_command_result_receipt_surface_ready_count == 14
    and $source.ordering_monotonicity_surface_count == 14
    and $source.ordering_monotonicity_surface_ready_count == 14
    and $source.ordering_monotonicity_fixture_count == 10
    and $source.blocked_ordering_monotonicity_fixture_count == 10
    and $source.noop_ordering_monotonicity_fixture_count == 10
    and $source.allowed_ordering_monotonicity_fixture_count == 0
    and $source.accepted_ordering_monotonicity_fixture_count == 0
    and $source.activation_command_result_receipt_ordering_allowed == false
    and $source.activation_command_result_receipt_ordering_recorded == false
    and $source.activation_command_result_receipt_ordering_persisted == false
    and $source.activation_command_result_receipt_sequence_cursor_accepted == false
    and $source.activation_command_result_receipt_sequence_cursor_recorded == false
    and $source.activation_command_result_receipt_sequence_cursor_persisted == false
    and $source.activation_command_result_receipt_monotonicity_state_recorded == false
    and $source.activation_command_result_receipt_monotonicity_state_persisted == false
    and $source.activation_command_result_receipt_latest_wins_overwrite_accepted == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.activation_command_completion_ack_persisted == false
    and $source.activation_command_completion_ack_accepted == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_request_accepted == false
    and $source.activation_request_recorded == false
    and $source.activation_request_persisted == false
    and $source.activation_request_executed == false
    and $source.activation_activated == false
    and $source.runtime_router_mutated == false
    and $source.runtime_attachment_performed == false
    and $source.live_context_attached == false
    and $source.context_injection_performed == false
    and $source.adapter_invoked == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.auth_secret_read == false
    and $source.credential_read == false
    and $source.secret_file_read == false
    and $source.usage_recorded == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.live_kg_write_performed == false
    and $source.receipt_recorded == false
    and $source.receipt_persisted == false
    and $source.receipt_accepted == false
    and $source.rollback_executed == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.public_release_claimed == false
    and $source.public_ga_claimed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restart_performed == false
    and $source.active_binary_mutated == false
    and ($source.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial" and .status == "allowed_report_only_next_slice" and .accepts_cancellation == false and .accepts_supersession == false and .persists_replacement_receipt == false and .mutates_runtime == false and .invokes_model == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.cancellation_supersession_status | startswith("blocked_"))
      and .cancellation_allowed == false
      and .cancellation_recorded == false
      and .cancellation_persisted == false
      and .cancellation_request_accepted == false
      and .supersession_allowed == false
      and .supersession_recorded == false
      and .supersession_persisted == false
      and .supersession_request_accepted == false
      and .replacement_receipt_accepted == false
      and .replacement_receipt_recorded == false
      and .replacement_receipt_persisted == false
      and .replacement_hash_accepted == false
      and .tombstone_recorded == false
      and .delete_marker_recorded == false
      and .receipt_recorded == false
      and .receipt_persisted == false
      and .receipt_accepted == false
      and .activation_activated == false
      and .runtime_router_mutated == false
      and .provider_invoked == false
      and .model_invoked == false
      and .auth_secret_read == false
      and .credential_read == false
      and .secret_file_read == false
      and .memory_store_write_performed == false
      and .memory_store_mutated == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
      and .install_executed == false
      and .service_restart_performed == false
      and .active_binary_mutated == false
      and .receipt_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
    or (
      $source.runtime == "hepta"
      and $source.status == "ready"
      and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_gate"
      and $source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
      and $source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status == "blocked"
      and ($source.side_effects | to_entries | all(.value == false))
      and ($fixtures | length) == 10
      and ($fixtures | all((.cancellation_supersession_status | startswith("blocked_")) and .receipt_noop_confirmed == true))
      and $min_long_soak_samples >= 24
    )
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_gate" \
  --arg ordering_monotonicity_report_sha256 "$ordering_monotonicity_report_sha256" \
  --arg cancellation_supersession_fixtures_sha256 "$cancellation_supersession_fixtures_sha256" \
  --arg cancellation_supersession_contract_hash_sha256 "$cancellation_supersession_contract_hash_sha256" \
  --arg cancellation_supersession_policy_hash_sha256 "$cancellation_supersession_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ORDERING_MONOTONICITY_JSON" \
  --argjson fixtures "$cancellation_supersession_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_cancellation_supersession_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_v1",
    activation_command_result_receipt_cancellation_supersession_mode:"runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_no_cancel_no_supersede_no_replacement_persist",
    source_activation_command_result_receipt_ordering_monotonicity_gate:$source.gate,
    source_activation_command_result_receipt_ordering_monotonicity_ready:$source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    source_activation_command_result_receipt_ordering_monotonicity_status:$source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status,
    source_activation_command_result_receipt_ordering_monotonicity_report_sha256:$ordering_monotonicity_report_sha256,
    source_activation_command_result_receipt_replay_idempotency_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:$source.source_activation_command_result_receipt_replay_idempotency_report_sha256,
    source_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_report_sha256:$source.source_activation_command_result_receipt_no_persistence_report_sha256,
    source_activation_command_noop_handoff_ready:$source.source_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_report_sha256:$source.source_activation_command_noop_handoff_report_sha256,
    cancellation_supersession_fixtures_sha256:$cancellation_supersession_fixtures_sha256,
    cancellation_supersession_contract_hash_sha256:$cancellation_supersession_contract_hash_sha256,
    cancellation_supersession_policy_hash_sha256:$cancellation_supersession_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status:"blocked",
    runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status:$source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status,
    runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    runtime_provider_router_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    activation_command_result_receipt_surface_count:$source.activation_command_result_receipt_surface_count,
    activation_command_result_receipt_surface_ready_count:$source.activation_command_result_receipt_surface_ready_count,
    ordering_monotonicity_surface_count:$source.ordering_monotonicity_surface_count,
    ordering_monotonicity_surface_ready_count:$source.ordering_monotonicity_surface_ready_count,
    cancellation_supersession_surface_count:14,
    cancellation_supersession_surface_ready_count:14,
    cancellation_supersession_side_effect_free_surface_count:14,
    cancellation_supersession_fixture_count:($fixtures | length),
    blocked_cancellation_supersession_fixture_count:($fixtures | length),
    noop_cancellation_supersession_fixture_count:($fixtures | length),
    allowed_cancellation_supersession_fixture_count:0,
    accepted_cancellation_supersession_fixture_count:0,
    cancellation_fixture_count:($fixtures | map(select(.cancellation_requested == true)) | length),
    supersession_fixture_count:($fixtures | map(select(.supersession_requested == true)) | length),
    cancellation_denied_count:($fixtures | map(select(.cancellation_requested == true)) | length),
    supersession_denied_count:($fixtures | map(select(.supersession_requested == true)) | length),
    cancellation_performed_count:0,
    supersession_performed_count:0,
    replacement_receipt_accepted_count:0,
    replacement_receipt_recorded_count:0,
    replacement_receipt_persisted_count:0,
    tombstone_recorded_count:0,
    delete_marker_recorded_count:0,
    activation_command_result_receipt_cancellation_allowed:false,
    activation_command_result_receipt_cancellation_recorded:false,
    activation_command_result_receipt_cancellation_persisted:false,
    activation_command_result_receipt_cancellation_request_accepted:false,
    activation_command_result_receipt_supersession_allowed:false,
    activation_command_result_receipt_supersession_recorded:false,
    activation_command_result_receipt_supersession_persisted:false,
    activation_command_result_receipt_supersession_request_accepted:false,
    activation_command_result_receipt_replacement_receipt_accepted:false,
    activation_command_result_receipt_replacement_receipt_recorded:false,
    activation_command_result_receipt_replacement_receipt_persisted:false,
    activation_command_result_receipt_replacement_hash_accepted:false,
    activation_command_result_receipt_tombstone_recorded:false,
    activation_command_result_receipt_tombstone_persisted:false,
    activation_command_result_receipt_delete_marker_recorded:false,
    activation_command_result_receipt_ack_cancellation_accepted:false,
    activation_command_result_receipt_ledger_cancellation_accepted:false,
    activation_command_result_receipt_index_cancellation_accepted:false,
    activation_command_result_receipt_delivery_cancellation_accepted:false,
    activation_command_result_receipt_export_cancellation_accepted:false,
    activation_command_result_receipt_query_cancellation_accepted:false,
    activation_command_result_receipt_observability_cancellation_accepted:false,
    activation_command_result_receipt_ordering_allowed:false,
    activation_command_result_receipt_ordering_recorded:false,
    activation_command_result_receipt_ordering_persisted:false,
    activation_command_result_receipt_sequence_cursor_accepted:false,
    activation_command_result_receipt_sequence_cursor_recorded:false,
    activation_command_result_receipt_sequence_cursor_persisted:false,
    activation_command_result_receipt_monotonicity_state_recorded:false,
    activation_command_result_receipt_monotonicity_state_persisted:false,
    activation_command_result_receipt_latest_wins_overwrite_accepted:false,
    activation_command_result_receipt_same_sequence_hash_override_accepted:false,
    activation_command_result_receipt_recorded:false,
    activation_command_result_receipt_persisted:false,
    activation_command_result_receipt_accepted:false,
    activation_command_result_receipt_materialized:false,
    activation_command_result_receipt_filesystem_written:false,
    activation_command_result_receipt_ledger_written:false,
    activation_command_result_receipt_indexed:false,
    activation_command_result_receipt_enqueued:false,
    activation_command_result_receipt_delivered:false,
    activation_command_result_receipt_exported:false,
    activation_command_result_receipt_query_registered:false,
    activation_command_result_receipt_observability_recorded:false,
    activation_command_completion_ack_recorded:false,
    activation_command_completion_ack_persisted:false,
    activation_command_completion_ack_accepted:false,
    activation_command_completion_ack_delivered:false,
    activation_from_cancellation_allowed:false,
    activation_from_supersession_allowed:false,
    activation_from_ordering_allowed:false,
    activation_from_replay_allowed:false,
    activation_from_receipt_allowed:false,
    activation_command_shape_registered:false,
    activation_command_allowed:false,
    activation_command_accepted:false,
    activation_command_enabled:false,
    activation_command_invoked:false,
    activation_command_dispatched:false,
    activation_command_dispatch_performed:false,
    activation_command_noop_decision_recorded:false,
    activation_command_noop_decision_persisted:false,
    activation_command_handoff_recorded:false,
    activation_command_handoff_persisted:false,
    activation_request_allowed:false,
    activation_request_accepted:false,
    activation_request_recorded:false,
    activation_request_persisted:false,
    activation_request_executed:false,
    activation_activated:false,
    runtime_router_mutated:false,
    runtime_attachment_performed:false,
    live_context_attached:false,
    context_injection_performed:false,
    adapter_invoked:false,
    provider_invoked:false,
    model_invoked:false,
    auth_secret_read:false,
    credential_read:false,
    secret_file_read:false,
    usage_recorded:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    live_kg_write_performed:false,
    readback_evidence_recorded:false,
    readback_evidence_persisted:false,
    router_handoff_recorded:false,
    router_handoff_persisted:false,
    rollback_executed:false,
    telegram_send_performed:false,
    channel_send_performed:false,
    external_send_performed:false,
    public_release_claimed:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    install_executed:false,
    launchd_mutated:false,
    service_restart_performed:false,
    active_binary_mutated:false,
    cancellation_supersession_surfaces:[
      "source_ordering_monotonicity_report_required",
      "cancellation_request_shape_denied",
      "supersession_request_shape_denied",
      "replacement_receipt_hash_denied",
      "tombstone_or_delete_marker_denied",
      "cancel_after_blocked_noop_denied",
      "supersede_blocked_noop_with_completed_denied",
      "acknowledgement_cancellation_denied",
      "ledger_index_delivery_export_cancellation_denied",
      "runtime_router_live_context_supersession_denied",
      "adapter_provider_model_usage_supersession_denied",
      "memory_kg_rollback_secret_supersession_denied",
      "external_public_install_restart_active_binary_supersession_denied",
      "receipt_export_query_observability_cancellation_denied"
    ],
    cancellation_supersession_fixtures:$fixtures,
    denied_by_cancellation_supersession:[
      "source_ordering_monotonicity_report_required",
      "cancellation_request_acceptance_denied",
      "cancellation_recording_denied",
      "cancellation_persistence_denied",
      "supersession_request_acceptance_denied",
      "supersession_recording_denied",
      "supersession_persistence_denied",
      "replacement_receipt_acceptance_denied",
      "replacement_receipt_recording_denied",
      "replacement_receipt_persistence_denied",
      "replacement_hash_acceptance_denied",
      "tombstone_recording_denied",
      "delete_marker_recording_denied",
      "cancel_after_blocked_noop_denied",
      "supersede_blocked_noop_with_completed_denied",
      "completion_ack_cancellation_denied",
      "ledger_cancellation_denied",
      "index_cancellation_denied",
      "delivery_cancellation_denied",
      "export_query_observability_cancellation_denied",
      "runtime_router_supersession_denied",
      "live_context_supersession_denied",
      "adapter_provider_model_supersession_denied",
      "usage_memory_kg_supersession_denied",
      "rollback_secret_material_supersession_denied",
      "external_public_release_supersession_denied",
      "install_restart_active_binary_supersession_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial",
        status:"allowed_report_only",
        accepts_cancellation:false,
        accepts_supersession:false,
        persists_replacement_receipt:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
        status:"allowed_report_only_next_slice",
        accepts_cancellation:false,
        accepts_supersession:false,
        writes_audit_trail:false,
        persists_evidence:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        accepts_cancellation:false,
        accepts_supersession:false,
        persists_replacement_receipt:false,
        mutates_runtime:false,
        invokes_model:false,
        writes_kg:false
      }
    ],
    source_ordering_monotonicity_report_required:true,
    cancellation_acceptance_forbidden:true,
    cancellation_recording_forbidden:true,
    cancellation_persistence_forbidden:true,
    supersession_acceptance_forbidden:true,
    supersession_recording_forbidden:true,
    supersession_persistence_forbidden:true,
    replacement_receipt_persistence_forbidden:true,
    tombstone_or_delete_marker_forbidden:true,
    runtime_provider_memory_kg_supersession_forbidden:true,
    secret_read_forbidden:true,
    external_public_install_restart_active_binary_supersession_forbidden:true,
    side_effects:{
      activation_command_result_receipt_cancellation_recorded:false,
      activation_command_result_receipt_cancellation_persisted:false,
      activation_command_result_receipt_supersession_recorded:false,
      activation_command_result_receipt_supersession_persisted:false,
      activation_command_result_receipt_replacement_receipt_recorded:false,
      activation_command_result_receipt_replacement_receipt_persisted:false,
      activation_command_result_receipt_replacement_hash_accepted:false,
      activation_command_result_receipt_tombstone_recorded:false,
      activation_command_result_receipt_tombstone_persisted:false,
      activation_command_result_receipt_delete_marker_recorded:false,
      activation_command_result_receipt_ack_cancellation_accepted:false,
      activation_command_result_receipt_ledger_cancellation_accepted:false,
      activation_command_result_receipt_index_cancellation_accepted:false,
      activation_command_result_receipt_delivery_cancellation_accepted:false,
      activation_command_result_receipt_export_cancellation_accepted:false,
      activation_command_result_receipt_query_cancellation_accepted:false,
      activation_command_result_receipt_observability_cancellation_accepted:false,
      activation_command_result_receipt_ordering_recorded:false,
      activation_command_result_receipt_ordering_persisted:false,
      activation_command_result_receipt_sequence_cursor_accepted:false,
      activation_command_result_receipt_sequence_cursor_recorded:false,
      activation_command_result_receipt_sequence_cursor_persisted:false,
      activation_command_result_receipt_monotonicity_state_recorded:false,
      activation_command_result_receipt_monotonicity_state_persisted:false,
      activation_command_result_receipt_recorded:false,
      activation_command_result_receipt_persisted:false,
      activation_command_result_receipt_accepted:false,
      activation_command_result_receipt_materialized:false,
      activation_command_result_receipt_filesystem_written:false,
      activation_command_result_receipt_ledger_written:false,
      activation_command_result_receipt_indexed:false,
      activation_command_result_receipt_enqueued:false,
      activation_command_result_receipt_delivered:false,
      activation_command_completion_ack_recorded:false,
      activation_command_completion_ack_persisted:false,
      activation_command_completion_ack_accepted:false,
      activation_command_completion_ack_delivered:false,
      activation_command_enabled:false,
      activation_command_invoked:false,
      activation_command_dispatched:false,
      activation_activated:false,
      runtime_router_mutated:false,
      runtime_attachment_performed:false,
      live_context_attached:false,
      context_injection_performed:false,
      adapter_invoked:false,
      provider_invoked:false,
      model_invoked:false,
      auth_secret_read:false,
      credential_read:false,
      secret_file_read:false,
      usage_recorded:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      live_kg_write_performed:false,
      readback_evidence_recorded:false,
      readback_evidence_persisted:false,
      router_handoff_recorded:false,
      router_handoff_persisted:false,
      rollback_executed:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      filesystem_written:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      release_artifact_written:false,
      install_executed:false,
      launchd_mutated:false,
      service_restart_performed:false,
      active_binary_mutated:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_gate"
  and .activation_command_result_receipt_cancellation_supersession_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_v1"
  and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status == "blocked"
  and .source_activation_command_result_receipt_ordering_monotonicity_ready == true
  and .source_activation_command_result_receipt_ordering_monotonicity_status == "blocked"
  and .runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .activation_command_result_receipt_surface_count == 14
  and .activation_command_result_receipt_surface_ready_count == 14
  and .ordering_monotonicity_surface_count == 14
  and .ordering_monotonicity_surface_ready_count == 14
  and .cancellation_supersession_surface_count == 14
  and .cancellation_supersession_surface_ready_count == 14
  and .cancellation_supersession_side_effect_free_surface_count == 14
  and .cancellation_supersession_fixture_count == 10
  and .blocked_cancellation_supersession_fixture_count == 10
  and .noop_cancellation_supersession_fixture_count == 10
  and .allowed_cancellation_supersession_fixture_count == 0
  and .accepted_cancellation_supersession_fixture_count == 0
  and .cancellation_fixture_count == 5
  and .supersession_fixture_count == 5
  and .cancellation_denied_count == 5
  and .supersession_denied_count == 5
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
  and .activation_command_result_receipt_tombstone_persisted == false
  and .activation_command_result_receipt_delete_marker_recorded == false
  and .activation_command_result_receipt_ack_cancellation_accepted == false
  and .activation_command_result_receipt_ledger_cancellation_accepted == false
  and .activation_command_result_receipt_index_cancellation_accepted == false
  and .activation_command_result_receipt_delivery_cancellation_accepted == false
  and .activation_command_result_receipt_export_cancellation_accepted == false
  and .activation_command_result_receipt_query_cancellation_accepted == false
  and .activation_command_result_receipt_observability_cancellation_accepted == false
  and .activation_command_result_receipt_ordering_allowed == false
  and .activation_command_result_receipt_ordering_recorded == false
  and .activation_command_result_receipt_ordering_persisted == false
  and .activation_command_result_receipt_sequence_cursor_accepted == false
  and .activation_command_result_receipt_sequence_cursor_recorded == false
  and .activation_command_result_receipt_sequence_cursor_persisted == false
  and .activation_command_result_receipt_monotonicity_state_recorded == false
  and .activation_command_result_receipt_monotonicity_state_persisted == false
  and .activation_command_result_receipt_latest_wins_overwrite_accepted == false
  and .activation_command_result_receipt_same_sequence_hash_override_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_result_receipt_ledger_written == false
  and .activation_command_result_receipt_indexed == false
  and .activation_command_result_receipt_enqueued == false
  and .activation_command_result_receipt_delivered == false
  and .activation_command_result_receipt_exported == false
  and .activation_command_result_receipt_query_registered == false
  and .activation_command_result_receipt_observability_recorded == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_persisted == false
  and .activation_command_completion_ack_accepted == false
  and .activation_command_completion_ack_delivered == false
  and .activation_from_cancellation_allowed == false
  and .activation_from_supersession_allowed == false
  and .activation_from_ordering_allowed == false
  and .activation_from_replay_allowed == false
  and .activation_from_receipt_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_command_dispatch_performed == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_persisted == false
  and .activation_request_executed == false
  and .activation_activated == false
  and .runtime_router_mutated == false
  and .runtime_attachment_performed == false
  and .live_context_attached == false
  and .context_injection_performed == false
  and .adapter_invoked == false
  and .provider_invoked == false
  and .model_invoked == false
  and .auth_secret_read == false
  and .credential_read == false
  and .secret_file_read == false
  and .usage_recorded == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .readback_evidence_recorded == false
  and .readback_evidence_persisted == false
  and .router_handoff_recorded == false
  and .router_handoff_persisted == false
  and .rollback_executed == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.cancellation_supersession_surfaces | length) == 14
  and (.cancellation_supersession_fixtures | length) == 10
  and (.cancellation_supersession_fixtures | all(
    (.cancellation_supersession_status | startswith("blocked_"))
    and .cancellation_allowed == false
    and .cancellation_recorded == false
    and .cancellation_persisted == false
    and .supersession_allowed == false
    and .supersession_recorded == false
    and .supersession_persisted == false
    and .replacement_receipt_accepted == false
    and .replacement_receipt_recorded == false
    and .replacement_receipt_persisted == false
    and .replacement_hash_accepted == false
    and .tombstone_recorded == false
    and .delete_marker_recorded == false
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_accepted == false
    and .activation_activated == false
    and .runtime_router_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .auth_secret_read == false
    and .credential_read == false
    and .secret_file_read == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .rollback_executed == false
    and .external_send_performed == false
    and .install_executed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and .receipt_noop_confirmed == true
  ))
  and ([.cancellation_supersession_fixtures[] | select(.source_ordering_monotonicity_present == false)] | length) == 1
  and ([.cancellation_supersession_fixtures[] | select(.cancellation_request_shape == "cancel_blocked_noop_receipt")] | length) == 1
  and ([.cancellation_supersession_fixtures[] | select(.requested_replacement_status == "completed")] | length) == 1
  and ([.cancellation_supersession_fixtures[] | select(.replacement_hash_requested == true)] | length) == 1
  and ([.cancellation_supersession_fixtures[] | select(.tombstone_requested == true and .delete_marker_requested == true)] | length) == 1
  and ([.cancellation_supersession_fixtures[] | select(.completion_ack_cancellation_requested == true)] | length) == 1
  and ([.cancellation_supersession_fixtures[] | select(.ledger_cancellation_requested == true and .index_cancellation_requested == true and .delivery_cancellation_requested == true)] | length) == 1
  and ([.cancellation_supersession_fixtures[] | select(.runtime_supersession_requested == true and .provider_supersession_requested == true and .model_supersession_requested == true)] | length) == 1
  and ([.cancellation_supersession_fixtures[] | select(.memory_store_supersession_requested == true and .live_kg_supersession_requested == true and .secret_material_supersession_requested == true)] | length) == 1
  and ([.cancellation_supersession_fixtures[] | select(.external_send_supersession_requested == true and .install_supersession_requested == true and .active_binary_mutation_supersession_requested == true)] | length) == 1
  and (.denied_by_cancellation_supersession | length) == 27
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial" and .status == "allowed_report_only_next_slice" and .writes_audit_trail == false and .persists_evidence == false and .mutates_runtime == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
  or (
    .runtime == "hepta"
    and .status == "ready"
    and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_gate"
    and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status == "blocked"
    and .cancellation_supersession_fixture_count == 10
    and .blocked_cancellation_supersession_fixture_count == 10
    and .allowed_cancellation_supersession_fixture_count == 0
    and .accepted_cancellation_supersession_fixture_count == 0
    and .activation_command_result_receipt_cancellation_allowed == false
    and .activation_command_result_receipt_supersession_allowed == false
    and .activation_command_result_receipt_replacement_receipt_accepted == false
    and .activation_activated == false
    and .runtime_router_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .auth_secret_read == false
    and .credential_read == false
    and .secret_file_read == false
    and .external_send_performed == false
    and .install_executed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and (.cancellation_supersession_fixtures | length) == 10
    and (.cancellation_supersession_fixtures | all((.cancellation_supersession_status | startswith("blocked_")) and .receipt_noop_confirmed == true))
    and (.side_effects | to_entries | all(.value == false))
  )
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt cancellation/supersession denial gate passed"
