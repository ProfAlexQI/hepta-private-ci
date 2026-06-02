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

ACTIVATION_REQUEST_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-gate.sh
)"

activation_command_fixtures_json="$(
  jq -n '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        activation_command_status:$status,
        source_activation_request_denial_matrix_present:true,
        source_activation_request_denial_matrix_ready:true,
        activation_command_requested:true,
        activation_command_shape_registered:false,
        activation_command_allowed:false,
        activation_command_accepted:false,
        activation_command_enabled:false,
        activation_command_invoked:false,
        activation_command_dispatched:false,
        activation_command_dispatch_performed:false,
        activation_command_noop_decision_recorded:false,
        activation_command_noop_decision_persisted:false,
        activation_command_noop_decision_accepted:false,
        activation_command_handoff_recorded:false,
        activation_command_handoff_persisted:false,
        activation_command_handoff_accepted:false,
        activation_command_handoff_materialized:false,
        activation_command_handoff_filesystem_written:false,
        activation_command_result_receipt_recorded:false,
        activation_command_result_receipt_persisted:false,
        activation_command_result_receipt_accepted:false,
        activation_command_result_receipt_exported:false,
        activation_command_result_receipt_query_registered:false,
        activation_command_result_receipt_observability_recorded:false,
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
        activation_command_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      blocked_fixture("provider-router-activation-command-missing-source-activation-request-denial-matrix"; "blocked_noop"; "source_activation_request_denial_matrix_report_required"; {source_activation_request_denial_matrix_present:false, source_activation_request_denial_matrix_ready:false}),
      blocked_fixture("provider-router-activation-command-handoff-request"; "blocked_command_noop"; "activation_command_handoff_shape_denied"; {}),
      blocked_fixture("provider-router-activation-command-registration-enable-request"; "blocked_register_enable_noop"; "activation_command_registration_enablement_denied"; {activation_command_registration_requested:true, activation_command_enable_requested:true}),
      blocked_fixture("provider-router-activation-command-direct-invocation-request"; "blocked_invocation_noop"; "activation_command_invocation_denied"; {activation_command_invocation_requested:true}),
      blocked_fixture("provider-router-activation-command-runtime-router-dispatch-request"; "blocked_dispatch_noop"; "runtime_router_dispatch_denied"; {runtime_router_dispatch_requested:true, runtime_router_mutation_requested:true}),
      blocked_fixture("provider-router-activation-command-live-context-injection-request"; "blocked_context_noop"; "live_context_context_injection_command_denied"; {live_context_attachment_requested:true, context_injection_requested:true}),
      blocked_fixture("provider-router-activation-command-adapter-provider-model-request"; "blocked_provider_noop"; "adapter_provider_model_command_denied"; {adapter_invocation_requested:true, provider_invocation_requested:true, model_invocation_requested:true}),
      blocked_fixture("provider-router-activation-command-memory-kg-request"; "blocked_memory_kg_noop"; "memory_kg_command_denied"; {memory_store_write_requested:true, live_kg_write_requested:true}),
      blocked_fixture("provider-router-activation-command-receipt-readback-router-handoff-request"; "blocked_receipt_router_noop"; "receipt_readback_router_handoff_command_denied"; {receipt_record_requested:true, receipt_persist_requested:true, receipt_export_requested:true, receipt_query_requested:true, receipt_observability_requested:true, readback_evidence_requested:true, router_handoff_requested:true}),
      blocked_fixture("provider-router-activation-command-external-public-install-restart-active-binary-request"; "blocked_external_noop"; "external_public_install_restart_active_binary_command_denied"; {external_send_requested:true, public_claim_requested:true, public_ga_claim_requested:true, release_artifact_write_requested:true, install_requested:true, launchd_restart_requested:true, service_restart_requested:true, active_binary_mutation_requested:true})
    ]
  '
)"

activation_request_report_sha256="$(sha256_text "$ACTIVATION_REQUEST_JSON")"
activation_command_fixtures_sha256="$(sha256_text "$activation_command_fixtures_json")"
activation_command_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-activation-command-noop-handoff:$activation_request_report_sha256:$activation_command_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
activation_command_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-activation-command-noop-handoff:report-only:no-command-register:no-command-enable:no-command-invoke:no-dispatch:no-handoff-persist:no-provider:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "activation-command=false;register=false;enable=false;invoke=false;dispatch=false;handoff=false;runtime=false;context=false;adapter=false;model=false;memory=false;kg=false;secret=false;service_restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ACTIVATION_REQUEST_JSON" \
  --argjson fixtures "$activation_command_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_gate"
    and $source.activation_request_denial_matrix_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_v1"
    and $source.runtime_provider_router_activation_request_denial_matrix_ready == true
    and $source.runtime_provider_router_activation_request_denial_matrix_status == "blocked"
    and $source.activation_request_fixture_count == 10
    and $source.blocked_activation_request_fixture_count == 10
    and $source.noop_activation_request_fixture_count == 10
    and $source.allowed_activation_request_fixture_count == 0
    and $source.accepted_activation_request_fixture_count == 0
    and $source.activation_request_denied_count == 10
    and $source.activation_request_performed_count == 0
    and $source.activation_execution_performed_count == 0
    and $source.activation_request_allowed == false
    and $source.activation_request_accepted == false
    and $source.activation_request_recorded == false
    and $source.activation_request_persisted == false
    and $source.activation_request_materialized == false
    and $source.activation_request_filesystem_written == false
    and $source.activation_request_delivered == false
    and $source.activation_request_executed == false
    and $source.activation_activated == false
    and $source.activation_nonce_accepted == false
    and $source.activation_generation_accepted == false
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
    and $source.receipt_exported == false
    and $source.receipt_query_registered == false
    and $source.receipt_observability_recorded == false
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
    and $source.public_release_claimed == false
    and $source.service_restart_performed == false
    and $source.active_binary_mutated == false
    and ($source.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_noop_handoff" and .status == "allowed_report_only_next_slice" and .accepts_activation_request == false and .records_activation == false and .executes_activation == false and .attaches_live_context == false and .invokes_model == false))
    and ($source.denied_by_activation_request_denial_matrix | length) == 27
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.activation_command_status == "blocked_noop" or .activation_command_status == "blocked_command_noop" or .activation_command_status == "blocked_register_enable_noop" or .activation_command_status == "blocked_invocation_noop" or .activation_command_status == "blocked_dispatch_noop" or .activation_command_status == "blocked_context_noop" or .activation_command_status == "blocked_provider_noop" or .activation_command_status == "blocked_memory_kg_noop" or .activation_command_status == "blocked_receipt_router_noop" or .activation_command_status == "blocked_external_noop")
      and .activation_command_allowed == false
      and .activation_command_accepted == false
      and .activation_command_enabled == false
      and .activation_command_invoked == false
      and .activation_command_dispatched == false
      and .activation_command_dispatch_performed == false
      and .activation_command_noop_decision_recorded == false
      and .activation_command_noop_decision_persisted == false
      and .activation_command_handoff_recorded == false
      and .activation_command_handoff_persisted == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
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
      and .activation_command_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_gate" \
  --arg activation_request_report_sha256 "$activation_request_report_sha256" \
  --arg activation_command_fixtures_sha256 "$activation_command_fixtures_sha256" \
  --arg activation_command_contract_hash_sha256 "$activation_command_contract_hash_sha256" \
  --arg activation_command_policy_hash_sha256 "$activation_command_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ACTIVATION_REQUEST_JSON" \
  --argjson fixtures "$activation_command_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_noop_handoff_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_v1",
    activation_command_noop_handoff_mode:"runtime_provider_router_activation_command_noop_handoff_no_register_no_enable_no_invoke_no_dispatch",
    source_activation_request_denial_matrix_gate:$source.gate,
    source_activation_request_denial_matrix_ready:$source.runtime_provider_router_activation_request_denial_matrix_ready,
    source_activation_request_denial_matrix_status:$source.runtime_provider_router_activation_request_denial_matrix_status,
    source_activation_request_denial_matrix_report_sha256:$activation_request_report_sha256,
    source_operator_acknowledgement_non_acceptance_gate:$source.source_operator_acknowledgement_non_acceptance_gate,
    source_operator_summary_non_persistence_gate:$source.source_operator_summary_non_persistence_gate,
    source_receipt_observability_denial_gate:$source.source_receipt_observability_denial_gate,
    source_runtime_model_provider_router:$source.source_runtime_model_provider_router,
    activation_command_fixtures_sha256:$activation_command_fixtures_sha256,
    activation_command_contract_hash_sha256:$activation_command_contract_hash_sha256,
    activation_command_policy_hash_sha256:$activation_command_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_activation_command_noop_handoff_ready:true,
    runtime_provider_router_activation_command_noop_handoff_status:"blocked",
    runtime_provider_router_activation_request_denial_matrix_ready:$source.runtime_provider_router_activation_request_denial_matrix_ready,
    runtime_provider_router_activation_request_denial_matrix_status:$source.runtime_provider_router_activation_request_denial_matrix_status,
    activation_request_fixture_count:$source.activation_request_fixture_count,
    blocked_activation_request_fixture_count:$source.blocked_activation_request_fixture_count,
    noop_activation_request_fixture_count:$source.noop_activation_request_fixture_count,
    allowed_activation_request_fixture_count:$source.allowed_activation_request_fixture_count,
    accepted_activation_request_fixture_count:$source.accepted_activation_request_fixture_count,
    activation_request_denied_count:$source.activation_request_denied_count,
    activation_request_performed_count:$source.activation_request_performed_count,
    activation_execution_performed_count:$source.activation_execution_performed_count,
    activation_command_surface_count:13,
    activation_command_surface_ready_count:13,
    activation_command_side_effect_free_surface_count:13,
    activation_command_fixture_count:($fixtures | length),
    blocked_activation_command_fixture_count:($fixtures | length),
    noop_activation_command_fixture_count:($fixtures | length),
    allowed_activation_command_fixture_count:0,
    accepted_activation_command_fixture_count:0,
    activation_command_denied_count:10,
    activation_command_performed_count:0,
    activation_command_dispatch_performed_count:0,
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
    activation_command_result_receipt_recorded:false,
    activation_command_result_receipt_persisted:false,
    activation_command_result_receipt_accepted:false,
    activation_command_result_receipt_exported:false,
    activation_command_result_receipt_query_registered:false,
    activation_command_result_receipt_observability_recorded:false,
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
    activation_command_surfaces:[
      "source_activation_request_denial_matrix_report_required",
      "activation_command_handoff_shape_denied",
      "activation_command_registration_denied",
      "activation_command_enablement_denied",
      "activation_command_invocation_denied",
      "activation_command_dispatch_denied",
      "activation_command_handoff_record_persist_denied",
      "live_context_context_injection_command_denied",
      "adapter_provider_model_command_denied",
      "memory_kg_command_denied",
      "receipt_readback_router_handoff_command_denied",
      "command_result_receipt_export_query_observability_denied",
      "external_public_install_restart_active_binary_command_denied"
    ],
    activation_command_fixtures:$fixtures,
    denied_by_activation_command_noop_handoff:[
      "source_activation_request_denial_matrix_report_required",
      "activation_command_shape_registration_denied",
      "activation_command_acceptance_denied",
      "activation_command_enablement_denied",
      "activation_command_invocation_denied",
      "activation_command_dispatch_denied",
      "activation_command_noop_decision_recording_denied",
      "activation_command_noop_decision_persistence_denied",
      "activation_command_handoff_recording_denied",
      "activation_command_handoff_persistence_denied",
      "activation_command_handoff_acceptance_denied",
      "activation_command_handoff_materialization_denied",
      "activation_command_handoff_filesystem_write_denied",
      "activation_command_result_receipt_recording_denied",
      "activation_command_result_receipt_persistence_denied",
      "activation_request_acceptance_denied",
      "activation_execution_denied",
      "runtime_router_mutation_denied",
      "runtime_attachment_denied",
      "live_context_attachment_denied",
      "context_injection_denied",
      "adapter_invocation_denied",
      "provider_model_invocation_denied",
      "memory_store_write_denied",
      "live_kg_write_denied",
      "receipt_export_query_observability_denied",
      "router_handoff_readback_persistence_denied",
      "usage_recording_denied",
      "secret_material_read_denied",
      "external_public_install_restart_active_binary_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_activation_command_noop_handoff",
        status:"allowed_report_only",
        registers_command:false,
        enables_command:false,
        invokes_command:false,
        dispatches_command:false,
        persists_handoff:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_activation_command_result_receipt_no_persistence",
        status:"allowed_report_only_next_slice",
        records_command_result:false,
        persists_command_result:false,
        exports_receipt:false,
        registers_observability:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        mutates_runtime:false,
        dispatches_command:false,
        attaches_live_context:false,
        invokes_model:false,
        writes_kg:false
      }
    ],
    source_activation_request_denial_matrix_report_required:true,
    activation_command_registration_forbidden:true,
    activation_command_enablement_forbidden:true,
    activation_command_invocation_forbidden:true,
    activation_command_dispatch_forbidden:true,
    activation_command_handoff_persistence_forbidden:true,
    activation_command_result_receipt_persistence_forbidden:true,
    activation_request_acceptance_forbidden:true,
    activation_request_execution_forbidden:true,
    runtime_router_mutation_forbidden:true,
    live_context_attachment_forbidden:true,
    context_injection_forbidden:true,
    adapter_invocation_forbidden:true,
    provider_model_invocation_forbidden:true,
    memory_kg_write_forbidden:true,
    auth_secret_read_forbidden:true,
    usage_recording_forbidden:true,
    side_effects:{
      activation_command_shape_registered:false,
      activation_command_accepted:false,
      activation_command_enabled:false,
      activation_command_invoked:false,
      activation_command_dispatched:false,
      activation_command_dispatch_performed:false,
      activation_command_noop_decision_recorded:false,
      activation_command_noop_decision_persisted:false,
      activation_command_noop_decision_accepted:false,
      activation_command_handoff_recorded:false,
      activation_command_handoff_persisted:false,
      activation_command_handoff_accepted:false,
      activation_command_handoff_materialized:false,
      activation_command_handoff_filesystem_written:false,
      activation_command_result_receipt_recorded:false,
      activation_command_result_receipt_persisted:false,
      activation_command_result_receipt_accepted:false,
      activation_command_result_receipt_exported:false,
      activation_command_result_receipt_query_registered:false,
      activation_command_result_receipt_observability_recorded:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_gate"
  and .activation_command_noop_handoff_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_v1"
  and .runtime_provider_router_activation_command_noop_handoff_ready == true
  and .runtime_provider_router_activation_command_noop_handoff_status == "blocked"
  and .runtime_provider_router_activation_request_denial_matrix_ready == true
  and .runtime_provider_router_activation_request_denial_matrix_status == "blocked"
  and .activation_request_fixture_count == 10
  and .blocked_activation_request_fixture_count == 10
  and .noop_activation_request_fixture_count == 10
  and .allowed_activation_request_fixture_count == 0
  and .accepted_activation_request_fixture_count == 0
  and .activation_request_denied_count == 10
  and .activation_request_performed_count == 0
  and .activation_execution_performed_count == 0
  and .activation_command_surface_count == 13
  and .activation_command_surface_ready_count == 13
  and .activation_command_side_effect_free_surface_count == 13
  and .activation_command_fixture_count == 10
  and .blocked_activation_command_fixture_count == 10
  and .noop_activation_command_fixture_count == 10
  and .allowed_activation_command_fixture_count == 0
  and .accepted_activation_command_fixture_count == 0
  and .activation_command_denied_count == 10
  and .activation_command_performed_count == 0
  and .activation_command_dispatch_performed_count == 0
  and .activation_command_shape_registered == false
  and .activation_command_allowed == false
  and .activation_command_accepted == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_command_noop_decision_recorded == false
  and .activation_command_noop_decision_persisted == false
  and .activation_command_noop_decision_accepted == false
  and .activation_command_handoff_recorded == false
  and .activation_command_handoff_persisted == false
  and .activation_command_handoff_accepted == false
  and .activation_command_handoff_materialized == false
  and .activation_command_handoff_filesystem_written == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_exported == false
  and .activation_command_result_receipt_query_registered == false
  and .activation_command_result_receipt_observability_recorded == false
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
  and (.activation_command_surfaces | length) == 13
  and (.activation_command_fixtures | length) == 10
  and (.activation_command_fixtures | all((.activation_command_status == "blocked_noop" or .activation_command_status == "blocked_command_noop" or .activation_command_status == "blocked_register_enable_noop" or .activation_command_status == "blocked_invocation_noop" or .activation_command_status == "blocked_dispatch_noop" or .activation_command_status == "blocked_context_noop" or .activation_command_status == "blocked_provider_noop" or .activation_command_status == "blocked_memory_kg_noop" or .activation_command_status == "blocked_receipt_router_noop" or .activation_command_status == "blocked_external_noop") and .activation_command_allowed == false and .activation_command_enabled == false and .activation_command_invoked == false and .activation_command_dispatched == false and .activation_command_handoff_recorded == false and .activation_command_handoff_persisted == false and .activation_command_result_receipt_recorded == false and .activation_command_result_receipt_persisted == false and .activation_request_accepted == false and .activation_request_recorded == false and .activation_request_persisted == false and .activation_request_executed == false and .activation_activated == false and .runtime_router_mutated == false and .live_context_attached == false and .context_injection_performed == false and .adapter_invoked == false and .provider_invoked == false and .model_invoked == false and .auth_secret_read == false and .credential_read == false and .secret_file_read == false and .usage_recorded == false and .memory_store_write_performed == false and .memory_store_mutated == false and .live_kg_write_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .readback_evidence_recorded == false and .readback_evidence_persisted == false and .router_handoff_recorded == false and .router_handoff_persisted == false and .activation_command_noop_confirmed == true))
  and ([.activation_command_fixtures[] | select(.source_activation_request_denial_matrix_present == false and .source_activation_request_denial_matrix_ready == false)] | length) == 1
  and ([.activation_command_fixtures[] | select(.activation_command_registration_requested == true and .activation_command_enable_requested == true)] | length) == 1
  and ([.activation_command_fixtures[] | select(.activation_command_invocation_requested == true)] | length) == 1
  and ([.activation_command_fixtures[] | select(.runtime_router_dispatch_requested == true and .runtime_router_mutation_requested == true)] | length) == 1
  and ([.activation_command_fixtures[] | select(.external_send_requested == true and .release_artifact_write_requested == true and .active_binary_mutation_requested == true)] | length) == 1
  and (.denied_by_activation_command_noop_handoff | length) == 30
  and (.allowed_next_actions | any(.action == "review_runtime_provider_router_activation_command_noop_handoff" and .status == "allowed_report_only" and .registers_command == false and .enables_command == false and .invokes_command == false and .dispatches_command == false and .persists_handoff == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_no_persistence" and .status == "allowed_report_only_next_slice" and .records_command_result == false and .persists_command_result == false and .exports_receipt == false and .registers_observability == false and .mutates_runtime == false and .invokes_model == false))
  and .source_activation_request_denial_matrix_report_required == true
  and .activation_command_registration_forbidden == true
  and .activation_command_enablement_forbidden == true
  and .activation_command_invocation_forbidden == true
  and .activation_command_dispatch_forbidden == true
  and .activation_command_handoff_persistence_forbidden == true
  and .activation_command_result_receipt_persistence_forbidden == true
  and .activation_request_acceptance_forbidden == true
  and .activation_request_execution_forbidden == true
  and .runtime_router_mutation_forbidden == true
  and .live_context_attachment_forbidden == true
  and .context_injection_forbidden == true
  and .adapter_invocation_forbidden == true
  and .provider_model_invocation_forbidden == true
  and .memory_kg_write_forbidden == true
  and .auth_secret_read_forbidden == true
  and .usage_recording_forbidden == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command no-op handoff gate passed"
