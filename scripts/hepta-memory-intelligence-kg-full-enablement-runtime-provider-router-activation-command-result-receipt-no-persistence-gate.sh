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

NOOP_HANDOFF_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-gate.sh
)"

result_receipt_fixtures_json="$(
  jq -n '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        activation_command_result_receipt_status:$status,
        source_activation_command_noop_handoff_present:true,
        source_activation_command_noop_handoff_ready:true,
        activation_command_result_receipt_requested:true,
        activation_command_result_receipt_shape_registered:false,
        activation_command_result_receipt_allowed:false,
        activation_command_result_receipt_schema_accepted:false,
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
        activation_command_result_receipt_hash_bound:false,
        activation_command_result_receipt_signature_hash_recorded:false,
        activation_command_result_receipt_timestamp_recorded:false,
        activation_command_result_receipt_operator_identity_accepted:false,
        activation_command_result_receipt_status_accepted:false,
        activation_command_result_receipt_blocked_noop_status_accepted:false,
        activation_command_completion_ack_recorded:false,
        activation_command_completion_ack_persisted:false,
        activation_command_completion_ack_accepted:false,
        activation_command_completion_ack_materialized:false,
        activation_command_completion_ack_delivered:false,
        operator_approval_from_receipt_accepted:false,
        activation_from_receipt_allowed:false,
        activation_command_shape_registered:false,
        activation_command_accepted:false,
        activation_command_enabled:false,
        activation_command_invoked:false,
        activation_command_dispatched:false,
        activation_command_dispatch_performed:false,
        activation_command_noop_decision_recorded:false,
        activation_command_noop_decision_persisted:false,
        activation_command_handoff_recorded:false,
        activation_command_handoff_persisted:false,
        activation_command_handoff_accepted:false,
        activation_command_handoff_materialized:false,
        activation_command_handoff_filesystem_written:false,
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
        receipt_noop_confirmed:true,
        receipt_exported:false,
        receipt_query_registered:false,
        receipt_observability_recorded:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
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
        reason:$reason
      } + $extra;
    [
      blocked_fixture("provider-router-activation-command-result-receipt-missing-source-noop-handoff"; "blocked_noop"; "source_activation_command_noop_handoff_report_required"; {source_activation_command_noop_handoff_present:false, source_activation_command_noop_handoff_ready:false}),
      blocked_fixture("provider-router-activation-command-result-receipt-schema-registration-attempt"; "blocked_schema_noop"; "result_receipt_schema_registration_denied"; {result_receipt_schema_registration_requested:true}),
      blocked_fixture("provider-router-activation-command-result-receipt-record-attempt"; "blocked_record_noop"; "result_receipt_recording_denied"; {result_receipt_record_requested:true}),
      blocked_fixture("provider-router-activation-command-result-receipt-persist-attempt"; "blocked_persist_noop"; "result_receipt_persistence_denied"; {result_receipt_persist_requested:true}),
      blocked_fixture("provider-router-activation-command-result-receipt-materialize-filesystem-attempt"; "blocked_materialize_noop"; "result_receipt_materialization_filesystem_write_denied"; {result_receipt_materialize_requested:true, result_receipt_filesystem_write_requested:true}),
      blocked_fixture("provider-router-activation-command-result-receipt-ledger-index-queue-delivery-attempt"; "blocked_ledger_index_delivery_noop"; "result_receipt_ledger_index_queue_delivery_denied"; {result_receipt_ledger_write_requested:true, result_receipt_index_requested:true, result_receipt_enqueue_requested:true, result_receipt_delivery_requested:true}),
      blocked_fixture("provider-router-activation-command-result-receipt-export-query-observability-attempt"; "blocked_export_query_observability_noop"; "result_receipt_export_query_observability_denied"; {result_receipt_export_requested:true, result_receipt_query_requested:true, result_receipt_observability_requested:true}),
      blocked_fixture("provider-router-activation-command-result-receipt-acceptance-completion-ack-attempt"; "blocked_acceptance_ack_noop"; "result_receipt_acceptance_completion_ack_denied"; {result_receipt_acceptance_requested:true, completion_ack_requested:true, operator_approval_from_receipt_requested:true}),
      blocked_fixture("provider-router-activation-command-result-receipt-runtime-context-provider-memory-kg-attempt"; "blocked_runtime_provider_memory_kg_noop"; "result_receipt_cannot_activate_runtime_provider_memory_or_kg"; {result_receipt_status_requested:"completed", activation_from_receipt_requested:true, runtime_router_mutation_requested:true, live_context_attachment_requested:true, context_injection_requested:true, provider_invocation_requested:true, model_invocation_requested:true, usage_record_requested:true, memory_store_write_requested:true, live_kg_write_requested:true}),
      blocked_fixture("provider-router-activation-command-result-receipt-external-public-install-restart-active-binary-attempt"; "blocked_external_noop"; "result_receipt_cannot_send_publish_install_restart_or_mutate_active_binary"; {external_send_requested:true, public_claim_requested:true, public_ga_claim_requested:true, release_artifact_write_requested:true, install_requested:true, launchd_restart_requested:true, service_restart_requested:true, active_binary_mutation_requested:true})
    ]
  '
)"

noop_handoff_report_sha256="$(sha256_text "$NOOP_HANDOFF_JSON")"
result_receipt_fixtures_sha256="$(sha256_text "$result_receipt_fixtures_json")"
result_receipt_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence:$noop_handoff_report_sha256:$result_receipt_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
result_receipt_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-activation-command-result-receipt-no-persistence:report-only:no-receipt-record:no-receipt-persist:no-export:no-query:no-observability:no-activation:no-runtime:no-provider:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "result-receipt=false;record=false;persist=false;export=false;query=false;observability=false;activation=false;runtime=false;context=false;adapter=false;model=false;memory=false;kg=false;secret=false;service_restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$NOOP_HANDOFF_JSON" \
  --argjson fixtures "$result_receipt_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_gate"
    and $source.activation_command_noop_handoff_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_v1"
    and $source.runtime_provider_router_activation_command_noop_handoff_ready == true
    and $source.runtime_provider_router_activation_command_noop_handoff_status == "blocked"
    and $source.runtime_provider_router_activation_request_denial_matrix_ready == true
    and $source.activation_command_surface_count == 13
    and $source.activation_command_surface_ready_count == 13
    and $source.activation_command_fixture_count == 10
    and $source.blocked_activation_command_fixture_count == 10
    and $source.noop_activation_command_fixture_count == 10
    and $source.allowed_activation_command_fixture_count == 0
    and $source.accepted_activation_command_fixture_count == 0
    and $source.activation_command_denied_count == 10
    and $source.activation_command_performed_count == 0
    and $source.activation_command_dispatch_performed_count == 0
    and $source.activation_command_shape_registered == false
    and $source.activation_command_allowed == false
    and $source.activation_command_accepted == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_command_noop_decision_recorded == false
    and $source.activation_command_noop_decision_persisted == false
    and $source.activation_command_handoff_recorded == false
    and $source.activation_command_handoff_persisted == false
    and $source.activation_command_handoff_accepted == false
    and $source.activation_command_handoff_materialized == false
    and $source.activation_command_handoff_filesystem_written == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_exported == false
    and $source.activation_command_result_receipt_query_registered == false
    and $source.activation_command_result_receipt_observability_recorded == false
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
    and $source.readback_evidence_recorded == false
    and $source.readback_evidence_persisted == false
    and $source.router_handoff_recorded == false
    and $source.router_handoff_persisted == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.rollback_executed == false
    and $source.service_restart_performed == false
    and $source.active_binary_mutated == false
    and ($source.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_no_persistence" and .status == "allowed_report_only_next_slice" and .records_command_result == false and .persists_command_result == false and .exports_receipt == false and .registers_observability == false and .mutates_runtime == false and .invokes_model == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.activation_command_result_receipt_status == "blocked_noop" or .activation_command_result_receipt_status == "blocked_schema_noop" or .activation_command_result_receipt_status == "blocked_record_noop" or .activation_command_result_receipt_status == "blocked_persist_noop" or .activation_command_result_receipt_status == "blocked_materialize_noop" or .activation_command_result_receipt_status == "blocked_ledger_index_delivery_noop" or .activation_command_result_receipt_status == "blocked_export_query_observability_noop" or .activation_command_result_receipt_status == "blocked_acceptance_ack_noop" or .activation_command_result_receipt_status == "blocked_runtime_provider_memory_kg_noop" or .activation_command_result_receipt_status == "blocked_external_noop")
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
      and .activation_command_handoff_recorded == false
      and .activation_command_handoff_persisted == false
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
      and .receipt_recorded == false
      and .receipt_persisted == false
      and .receipt_accepted == false
      and .readback_evidence_recorded == false
      and .readback_evidence_persisted == false
      and .router_handoff_recorded == false
      and .router_handoff_persisted == false
      and .telegram_send_performed == false
      and .channel_send_performed == false
      and .external_send_performed == false
      and .rollback_executed == false
      and .public_release_claimed == false
      and .service_restart_performed == false
      and .active_binary_mutated == false
      and .receipt_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_gate" \
  --arg noop_handoff_report_sha256 "$noop_handoff_report_sha256" \
  --arg result_receipt_fixtures_sha256 "$result_receipt_fixtures_sha256" \
  --arg result_receipt_contract_hash_sha256 "$result_receipt_contract_hash_sha256" \
  --arg result_receipt_policy_hash_sha256 "$result_receipt_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$NOOP_HANDOFF_JSON" \
  --argjson fixtures "$result_receipt_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_no_persistence_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_v1",
    activation_command_result_receipt_no_persistence_mode:"runtime_provider_router_activation_command_result_receipt_no_persistence_no_record_no_persist_no_export_no_query",
    source_activation_command_noop_handoff_gate:$source.gate,
    source_activation_command_noop_handoff_ready:$source.runtime_provider_router_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_status:$source.runtime_provider_router_activation_command_noop_handoff_status,
    source_activation_command_noop_handoff_report_sha256:$noop_handoff_report_sha256,
    source_activation_request_denial_matrix_gate:$source.source_activation_request_denial_matrix_gate,
    source_activation_request_denial_matrix_ready:$source.source_activation_request_denial_matrix_ready,
    source_runtime_model_provider_router:$source.source_runtime_model_provider_router,
    result_receipt_fixtures_sha256:$result_receipt_fixtures_sha256,
    result_receipt_contract_hash_sha256:$result_receipt_contract_hash_sha256,
    result_receipt_policy_hash_sha256:$result_receipt_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_activation_command_result_receipt_no_persistence_ready:true,
    runtime_provider_router_activation_command_result_receipt_no_persistence_status:"blocked",
    runtime_provider_router_activation_command_noop_handoff_ready:$source.runtime_provider_router_activation_command_noop_handoff_ready,
    runtime_provider_router_activation_command_noop_handoff_status:$source.runtime_provider_router_activation_command_noop_handoff_status,
    activation_command_surface_count:$source.activation_command_surface_count,
    activation_command_surface_ready_count:$source.activation_command_surface_ready_count,
    activation_command_fixture_count:$source.activation_command_fixture_count,
    blocked_activation_command_fixture_count:$source.blocked_activation_command_fixture_count,
    noop_activation_command_fixture_count:$source.noop_activation_command_fixture_count,
    allowed_activation_command_fixture_count:$source.allowed_activation_command_fixture_count,
    accepted_activation_command_fixture_count:$source.accepted_activation_command_fixture_count,
    activation_command_denied_count:$source.activation_command_denied_count,
    activation_command_performed_count:$source.activation_command_performed_count,
    activation_command_dispatch_performed_count:$source.activation_command_dispatch_performed_count,
    activation_command_result_receipt_surface_count:14,
    activation_command_result_receipt_surface_ready_count:14,
    activation_command_result_receipt_side_effect_free_surface_count:14,
    activation_command_result_receipt_fixture_count:($fixtures | length),
    blocked_activation_command_result_receipt_fixture_count:($fixtures | length),
    noop_activation_command_result_receipt_fixture_count:($fixtures | length),
    allowed_activation_command_result_receipt_fixture_count:0,
    accepted_activation_command_result_receipt_fixture_count:0,
    activation_command_result_receipt_denied_count:10,
    activation_command_result_receipt_performed_count:0,
    activation_command_result_receipt_shape_registered:false,
    activation_command_result_receipt_allowed:false,
    activation_command_result_receipt_schema_accepted:false,
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
    activation_command_result_receipt_hash_bound:false,
    activation_command_result_receipt_signature_hash_recorded:false,
    activation_command_result_receipt_timestamp_recorded:false,
    activation_command_result_receipt_operator_identity_accepted:false,
    activation_command_result_receipt_status_accepted:false,
    activation_command_result_receipt_blocked_noop_status_accepted:false,
    activation_command_completion_ack_recorded:false,
    activation_command_completion_ack_persisted:false,
    activation_command_completion_ack_accepted:false,
    activation_command_completion_ack_materialized:false,
    activation_command_completion_ack_delivered:false,
    operator_approval_from_receipt_accepted:false,
    activation_from_receipt_allowed:false,
    activation_command_shape_registered:false,
    activation_command_allowed:false,
    activation_command_accepted:false,
    activation_command_enabled:false,
    activation_command_invoked:false,
    activation_command_dispatched:false,
    activation_command_noop_decision_recorded:false,
    activation_command_noop_decision_persisted:false,
    activation_command_noop_decision_accepted:false,
    activation_command_handoff_recorded:false,
    activation_command_handoff_persisted:false,
    activation_command_handoff_accepted:false,
    activation_command_handoff_materialized:false,
    activation_command_handoff_filesystem_written:false,
    activation_request_allowed:false,
    activation_request_accepted:false,
    activation_request_recorded:false,
    activation_request_persisted:false,
    activation_request_materialized:false,
    activation_request_filesystem_written:false,
    activation_request_delivered:false,
    activation_request_executed:false,
    activation_activated:false,
    activation_nonce_accepted:false,
    activation_generation_accepted:false,
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
    receipt_export_allowed:false,
    receipt_exported:false,
    receipt_query_allowed:false,
    receipt_query_registered:false,
    receipt_observability_allowed:false,
    receipt_observability_recorded:false,
    receipt_recorded:false,
    receipt_persisted:false,
    receipt_accepted:false,
    receipt_materialized:false,
    receipt_filesystem_written:false,
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
    activation_command_result_receipt_surfaces:[
      "source_activation_command_noop_handoff_report_required",
      "disabled_activation_command_noop_identity_required",
      "result_receipt_schema_registration_denied",
      "result_receipt_hash_signature_timestamp_binding_denied",
      "result_receipt_blocked_noop_status_acceptance_denied",
      "result_receipt_record_persist_materialize_denied",
      "result_receipt_filesystem_ledger_index_queue_delivery_denied",
      "result_receipt_export_query_observability_denied",
      "activation_command_completion_ack_denied",
      "operator_approval_and_activation_from_receipt_denied",
      "runtime_router_live_context_context_injection_denied",
      "adapter_provider_model_invocation_denied",
      "usage_memory_kg_write_denied",
      "external_public_install_restart_active_binary_denied"
    ],
    activation_command_result_receipt_fixtures:$fixtures,
    denied_by_activation_command_result_receipt_no_persistence:[
      "source_activation_command_noop_handoff_required",
      "activation_command_disabled_required",
      "activation_command_invocation_denied",
      "activation_command_dispatch_denied",
      "result_receipt_schema_registration_denied",
      "result_receipt_schema_acceptance_denied",
      "result_receipt_recording_denied",
      "result_receipt_persistence_denied",
      "result_receipt_acceptance_denied",
      "result_receipt_materialization_denied",
      "result_receipt_filesystem_write_denied",
      "result_receipt_ledger_write_denied",
      "result_receipt_indexing_denied",
      "result_receipt_queue_enqueue_denied",
      "result_receipt_delivery_denied",
      "result_receipt_export_denied",
      "result_receipt_query_registration_denied",
      "result_receipt_observability_recording_denied",
      "completion_ack_recording_denied",
      "completion_ack_persistence_denied",
      "completion_ack_acceptance_denied",
      "operator_approval_from_receipt_denied",
      "activation_from_receipt_denied",
      "runtime_router_mutation_denied",
      "live_context_attachment_denied",
      "context_injection_denied",
      "adapter_invocation_denied",
      "provider_model_invocation_denied",
      "usage_recording_denied",
      "memory_store_write_denied",
      "live_kg_write_denied",
      "secret_material_read_denied",
      "external_send_denied",
      "public_release_claim_denied",
      "install_restart_active_binary_mutation_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_activation_command_result_receipt_no_persistence",
        status:"allowed_report_only",
        records_command_result:false,
        persists_command_result:false,
        exports_receipt:false,
        registers_query:false,
        registers_observability:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial",
        status:"allowed_report_only_next_slice",
        accepts_duplicate_receipt:false,
        records_idempotency:false,
        persists_replay_state:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        records_command_result:false,
        persists_command_result:false,
        mutates_runtime:false,
        invokes_model:false,
        writes_kg:false
      }
    ],
    source_activation_command_noop_handoff_report_required:true,
    result_receipt_schema_registration_forbidden:true,
    result_receipt_recording_forbidden:true,
    result_receipt_persistence_forbidden:true,
    result_receipt_export_query_observability_forbidden:true,
    result_receipt_activation_forbidden:true,
    result_receipt_runtime_mutation_forbidden:true,
    result_receipt_context_attachment_forbidden:true,
    result_receipt_adapter_provider_model_invocation_forbidden:true,
    result_receipt_memory_kg_write_forbidden:true,
    result_receipt_secret_read_forbidden:true,
    result_receipt_external_public_install_restart_active_binary_forbidden:true,
    side_effects:{
      activation_command_result_receipt_shape_registered:false,
      activation_command_result_receipt_schema_accepted:false,
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
      operator_approval_from_receipt_accepted:false,
      activation_from_receipt_allowed:false,
      activation_command_shape_registered:false,
      activation_command_accepted:false,
      activation_command_enabled:false,
      activation_command_invoked:false,
      activation_command_dispatched:false,
      activation_command_dispatch_performed:false,
      activation_command_noop_decision_recorded:false,
      activation_command_noop_decision_persisted:false,
      activation_command_handoff_recorded:false,
      activation_command_handoff_persisted:false,
      activation_command_handoff_materialized:false,
      activation_request_recorded:false,
      activation_request_persisted:false,
      activation_request_materialized:false,
      activation_request_filesystem_written:false,
      activation_request_delivered:false,
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
      receipt_exported:false,
      receipt_query_registered:false,
      receipt_observability_recorded:false,
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      receipt_materialized:false,
      receipt_filesystem_written:false,
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
      service_restarted:false,
      active_binary_mutated:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_gate"
  and .activation_command_result_receipt_no_persistence_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_v1"
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_status == "blocked"
  and .runtime_provider_router_activation_command_noop_handoff_ready == true
  and .runtime_provider_router_activation_command_noop_handoff_status == "blocked"
  and .minimum_required_samples >= 24
  and .activation_command_surface_count == 13
  and .activation_command_surface_ready_count == 13
  and .activation_command_fixture_count == 10
  and .blocked_activation_command_fixture_count == 10
  and .noop_activation_command_fixture_count == 10
  and .allowed_activation_command_fixture_count == 0
  and .accepted_activation_command_fixture_count == 0
  and .activation_command_denied_count == 10
  and .activation_command_performed_count == 0
  and .activation_command_dispatch_performed_count == 0
  and .activation_command_result_receipt_surface_count == 14
  and .activation_command_result_receipt_surface_ready_count == 14
  and .activation_command_result_receipt_side_effect_free_surface_count == 14
  and .activation_command_result_receipt_fixture_count == 10
  and .blocked_activation_command_result_receipt_fixture_count == 10
  and .noop_activation_command_result_receipt_fixture_count == 10
  and .allowed_activation_command_result_receipt_fixture_count == 0
  and .accepted_activation_command_result_receipt_fixture_count == 0
  and .activation_command_result_receipt_denied_count == 10
  and .activation_command_result_receipt_performed_count == 0
  and .activation_command_result_receipt_shape_registered == false
  and .activation_command_result_receipt_schema_accepted == false
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
  and .activation_command_result_receipt_hash_bound == false
  and .activation_command_result_receipt_signature_hash_recorded == false
  and .activation_command_result_receipt_timestamp_recorded == false
  and .activation_command_result_receipt_operator_identity_accepted == false
  and .activation_command_result_receipt_status_accepted == false
  and .activation_command_result_receipt_blocked_noop_status_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_persisted == false
  and .activation_command_completion_ack_accepted == false
  and .activation_command_completion_ack_materialized == false
  and .activation_command_completion_ack_delivered == false
  and .operator_approval_from_receipt_accepted == false
  and .activation_from_receipt_allowed == false
  and .activation_command_shape_registered == false
  and .activation_command_allowed == false
  and .activation_command_accepted == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_command_handoff_recorded == false
  and .activation_command_handoff_persisted == false
  and .activation_command_handoff_accepted == false
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
  and .receipt_exported == false
  and .receipt_query_registered == false
  and .receipt_observability_recorded == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .readback_evidence_recorded == false
  and .readback_evidence_persisted == false
  and .router_handoff_recorded == false
  and .router_handoff_persisted == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .rollback_executed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_surfaces | length) == 14
  and (.activation_command_result_receipt_fixtures | length) == 10
  and (.activation_command_result_receipt_fixtures | all(
    (.activation_command_result_receipt_status == "blocked_noop" or .activation_command_result_receipt_status == "blocked_schema_noop" or .activation_command_result_receipt_status == "blocked_record_noop" or .activation_command_result_receipt_status == "blocked_persist_noop" or .activation_command_result_receipt_status == "blocked_materialize_noop" or .activation_command_result_receipt_status == "blocked_ledger_index_delivery_noop" or .activation_command_result_receipt_status == "blocked_export_query_observability_noop" or .activation_command_result_receipt_status == "blocked_acceptance_ack_noop" or .activation_command_result_receipt_status == "blocked_runtime_provider_memory_kg_noop" or .activation_command_result_receipt_status == "blocked_external_noop")
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
    and .activation_command_handoff_recorded == false
    and .activation_command_handoff_persisted == false
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
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_accepted == false
    and .readback_evidence_recorded == false
    and .readback_evidence_persisted == false
    and .router_handoff_recorded == false
    and .router_handoff_persisted == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .rollback_executed == false
    and .receipt_noop_confirmed == true
  ))
  and ([.activation_command_result_receipt_fixtures[] | select(.source_activation_command_noop_handoff_present == false and .source_activation_command_noop_handoff_ready == false)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.result_receipt_schema_registration_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.result_receipt_record_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.result_receipt_persist_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.result_receipt_materialize_requested == true and .result_receipt_filesystem_write_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.result_receipt_ledger_write_requested == true and .result_receipt_index_requested == true and .result_receipt_enqueue_requested == true and .result_receipt_delivery_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.result_receipt_export_requested == true and .result_receipt_query_requested == true and .result_receipt_observability_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.completion_ack_requested == true and .operator_approval_from_receipt_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.activation_from_receipt_requested == true and .runtime_router_mutation_requested == true and .live_kg_write_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.external_send_requested == true and .install_requested == true and .active_binary_mutation_requested == true)] | length) == 1
  and (.denied_by_activation_command_result_receipt_no_persistence | length) == 35
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial" and .status == "allowed_report_only_next_slice" and .accepts_duplicate_receipt == false and .records_idempotency == false and .persists_replay_state == false and .mutates_runtime == false and .invokes_model == false))
  and .source_activation_command_noop_handoff_report_required == true
  and .result_receipt_schema_registration_forbidden == true
  and .result_receipt_recording_forbidden == true
  and .result_receipt_persistence_forbidden == true
  and .result_receipt_export_query_observability_forbidden == true
  and .result_receipt_activation_forbidden == true
  and .result_receipt_runtime_mutation_forbidden == true
  and .result_receipt_context_attachment_forbidden == true
  and .result_receipt_adapter_provider_model_invocation_forbidden == true
  and .result_receipt_memory_kg_write_forbidden == true
  and .result_receipt_secret_read_forbidden == true
  and .result_receipt_external_public_install_restart_active_binary_forbidden == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt no-persistence gate passed"
