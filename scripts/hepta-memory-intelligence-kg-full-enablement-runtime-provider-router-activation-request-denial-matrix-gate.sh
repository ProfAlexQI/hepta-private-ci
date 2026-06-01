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

OPERATOR_ACK_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance-gate.sh
)"

activation_request_fixtures_json="$(
  jq -n '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        activation_request_status:$status,
        source_operator_acknowledgement_non_acceptance_present:true,
        source_operator_acknowledgement_non_acceptance_ready:true,
        activation_request_requested:false,
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
        operator_acknowledgement_accepted:false,
        operator_identity_accepted:false,
        operator_scope_accepted:false,
        operator_activation_plan_accepted:false,
        runtime_attachment_acknowledged:false,
        live_context_acknowledged:false,
        memory_kg_acknowledged:false,
        provider_secret_acknowledged:false,
        telegram_send_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        operator_summary_recorded:false,
        operator_summary_persisted:false,
        operator_summary_materialized:false,
        operator_summary_filesystem_written:false,
        operator_summary_delivered:false,
        operator_briefing_recorded:false,
        operator_briefing_persisted:false,
        operator_briefing_materialized:false,
        operator_briefing_filesystem_written:false,
        operator_briefing_delivered:false,
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
        rollback_executed:false,
        public_release_claimed:false,
        service_restart_performed:false,
        active_binary_mutated:false,
        activation_request_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      blocked_fixture("provider-router-activation-request-missing-source-operator-acknowledgement-non-acceptance"; "blocked_noop"; "source_operator_acknowledgement_non_acceptance_report_required"; {source_operator_acknowledgement_non_acceptance_present:false, source_operator_acknowledgement_non_acceptance_ready:false, activation_request_requested:true}),
      blocked_fixture("provider-router-activation-request"; "blocked_activation_noop"; "activation_request_shape_denied"; {activation_request_requested:true}),
      blocked_fixture("provider-router-activation-identity-scope-request"; "blocked_identity_scope_noop"; "activation_identity_scope_denied"; {activation_request_requested:true, activation_identity_requested:true, activation_scope_requested:true}),
      blocked_fixture("provider-router-activation-nonce-generation-request"; "blocked_nonce_generation_noop"; "activation_nonce_generation_denied"; {activation_request_requested:true, activation_nonce_requested:true, activation_generation_requested:true}),
      blocked_fixture("provider-router-runtime-attachment-activation-request"; "blocked_runtime_noop"; "runtime_attachment_activation_denied"; {activation_request_requested:true, runtime_attachment_requested:true, runtime_router_mutation_requested:true}),
      blocked_fixture("provider-router-live-context-activation-request"; "blocked_context_noop"; "live_context_context_injection_activation_denied"; {activation_request_requested:true, live_context_attachment_requested:true, context_injection_requested:true}),
      blocked_fixture("provider-router-adapter-provider-model-activation-request"; "blocked_provider_noop"; "adapter_provider_model_activation_denied"; {activation_request_requested:true, adapter_invocation_requested:true, provider_invocation_requested:true, model_invocation_requested:true}),
      blocked_fixture("provider-router-memory-kg-activation-request"; "blocked_memory_kg_noop"; "memory_kg_activation_denied"; {activation_request_requested:true, memory_store_write_requested:true, live_kg_write_requested:true}),
      blocked_fixture("provider-router-receipt-readback-router-handoff-activation-request"; "blocked_receipt_router_noop"; "receipt_readback_router_handoff_activation_denied"; {activation_request_requested:true, receipt_record_requested:true, receipt_persist_requested:true, receipt_accept_requested:true, readback_evidence_requested:true, router_handoff_requested:true}),
      blocked_fixture("provider-router-external-public-install-restart-active-binary-activation-request"; "blocked_external_noop"; "external_public_install_restart_active_binary_activation_denied"; {activation_request_requested:true, external_send_requested:true, public_claim_requested:true, release_artifact_requested:true, install_requested:true, service_restart_requested:true, active_binary_mutation_requested:true})
    ]
  '
)"

operator_ack_report_sha256="$(sha256_text "$OPERATOR_ACK_JSON")"
activation_request_fixtures_sha256="$(sha256_text "$activation_request_fixtures_json")"
activation_request_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-activation-request-denial-matrix:$operator_ack_report_sha256:$activation_request_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
activation_request_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-activation-request-denial-matrix:report-only:no-activation-acceptance:no-activation-record:no-activation-execute:no-runtime-mutation:no-context-attachment:no-adapter:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "activation=false;activation_record=false;activation_persist=false;runtime_mutation=false;context=false;adapter=false;model=false;memory=false;kg=false;secret=false;service_restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$OPERATOR_ACK_JSON" \
  --argjson fixtures "$activation_request_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_gate"
    and $source.operator_acknowledgement_non_acceptance_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_v1"
    and $source.runtime_provider_router_operator_acknowledgement_non_acceptance_ready == true
    and $source.runtime_provider_router_operator_acknowledgement_non_acceptance_status == "blocked"
    and $source.operator_acknowledgement_fixture_count == 10
    and $source.blocked_operator_acknowledgement_fixture_count == 10
    and $source.noop_operator_acknowledgement_fixture_count == 10
    and $source.allowed_operator_acknowledgement_fixture_count == 0
    and $source.accepted_operator_acknowledgement_fixture_count == 0
    and $source.operator_acknowledgement_denied_count == 10
    and $source.operator_acknowledgement_performed_count == 0
    and $source.operator_acknowledgement_accepted == false
    and $source.operator_acknowledgement_recorded == false
    and $source.operator_acknowledgement_persisted == false
    and $source.operator_identity_accepted == false
    and $source.operator_scope_accepted == false
    and $source.operator_activation_plan_accepted == false
    and $source.receipt_acknowledgement_accepted == false
    and $source.runtime_attachment_acknowledged == false
    and $source.live_context_acknowledged == false
    and $source.memory_kg_acknowledged == false
    and $source.provider_secret_acknowledged == false
    and $source.operator_summary_persisted == false
    and $source.operator_briefing_persisted == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
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
    and $source.runtime_router_mutated == false
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
    and $source.rollback_executed == false
    and $source.service_restart_performed == false
    and $source.active_binary_mutated == false
    and ($source.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_request_denial_matrix" and .status == "allowed_report_only_next_slice" and .accepts_activation_request == false and .accepts_acknowledgement == false and .persists_summary == false and .exports_receipt == false and .invokes_adapter == false and .invokes_model == false))
    and ($source.denied_by_operator_acknowledgement_non_acceptance | length) == 27
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.activation_request_status == "blocked_noop" or .activation_request_status == "blocked_activation_noop" or .activation_request_status == "blocked_identity_scope_noop" or .activation_request_status == "blocked_nonce_generation_noop" or .activation_request_status == "blocked_runtime_noop" or .activation_request_status == "blocked_context_noop" or .activation_request_status == "blocked_provider_noop" or .activation_request_status == "blocked_memory_kg_noop" or .activation_request_status == "blocked_receipt_router_noop" or .activation_request_status == "blocked_external_noop")
      and .activation_request_allowed == false
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
      and .public_release_claimed == false
      and .service_restart_performed == false
      and .active_binary_mutated == false
      and .activation_request_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_gate" \
  --arg operator_ack_report_sha256 "$operator_ack_report_sha256" \
  --arg activation_request_fixtures_sha256 "$activation_request_fixtures_sha256" \
  --arg activation_request_contract_hash_sha256 "$activation_request_contract_hash_sha256" \
  --arg activation_request_policy_hash_sha256 "$activation_request_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$OPERATOR_ACK_JSON" \
  --argjson fixtures "$activation_request_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_request_denial_matrix_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_v1",
    activation_request_denial_matrix_mode:"runtime_provider_router_activation_request_denial_matrix_no_accept_no_execute_no_activation",
    source_operator_acknowledgement_non_acceptance_gate:$source.gate,
    source_operator_acknowledgement_non_acceptance_ready:$source.runtime_provider_router_operator_acknowledgement_non_acceptance_ready,
    source_operator_acknowledgement_non_acceptance_status:$source.runtime_provider_router_operator_acknowledgement_non_acceptance_status,
    source_operator_acknowledgement_non_acceptance_report_sha256:$operator_ack_report_sha256,
    source_operator_summary_non_persistence_gate:$source.source_operator_summary_non_persistence_gate,
    source_receipt_observability_denial_gate:$source.source_receipt_observability_denial_gate,
    source_runtime_model_provider_router:$source.source_runtime_model_provider_router,
    activation_request_fixtures_sha256:$activation_request_fixtures_sha256,
    activation_request_contract_hash_sha256:$activation_request_contract_hash_sha256,
    activation_request_policy_hash_sha256:$activation_request_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_activation_request_denial_matrix_ready:true,
    runtime_provider_router_activation_request_denial_matrix_status:"blocked",
    operator_acknowledgement_non_acceptance_ready:$source.runtime_provider_router_operator_acknowledgement_non_acceptance_ready,
    operator_acknowledgement_non_acceptance_status:$source.runtime_provider_router_operator_acknowledgement_non_acceptance_status,
    operator_acknowledgement_fixture_count:$source.operator_acknowledgement_fixture_count,
    blocked_operator_acknowledgement_fixture_count:$source.blocked_operator_acknowledgement_fixture_count,
    noop_operator_acknowledgement_fixture_count:$source.noop_operator_acknowledgement_fixture_count,
    allowed_operator_acknowledgement_fixture_count:$source.allowed_operator_acknowledgement_fixture_count,
    accepted_operator_acknowledgement_fixture_count:$source.accepted_operator_acknowledgement_fixture_count,
    operator_acknowledgement_denied_count:$source.operator_acknowledgement_denied_count,
    operator_acknowledgement_performed_count:$source.operator_acknowledgement_performed_count,
    operator_summary_denied_count:$source.operator_summary_denied_count,
    operator_briefing_denied_count:$source.operator_briefing_denied_count,
    operator_summary_performed_count:$source.operator_summary_performed_count,
    operator_briefing_performed_count:$source.operator_briefing_performed_count,
    receipt_export_denied_count:$source.receipt_export_denied_count,
    receipt_query_denied_count:$source.receipt_query_denied_count,
    receipt_observability_denied_count:$source.receipt_observability_denied_count,
    receipt_export_performed_count:$source.receipt_export_performed_count,
    receipt_query_performed_count:$source.receipt_query_performed_count,
    receipt_observability_performed_count:$source.receipt_observability_performed_count,
    activation_request_surface_count:12,
    activation_request_surface_ready_count:12,
    activation_request_side_effect_free_surface_count:12,
    activation_request_fixture_count:($fixtures | length),
    blocked_activation_request_fixture_count:($fixtures | length),
    noop_activation_request_fixture_count:($fixtures | length),
    allowed_activation_request_fixture_count:0,
    accepted_activation_request_fixture_count:0,
    activation_request_denied_count:10,
    activation_request_performed_count:0,
    activation_execution_performed_count:0,
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
    operator_acknowledgement_accepted:false,
    operator_identity_accepted:false,
    operator_scope_accepted:false,
    operator_activation_plan_accepted:false,
    runtime_attachment_acknowledged:false,
    live_context_acknowledged:false,
    memory_kg_acknowledged:false,
    provider_secret_acknowledged:false,
    operator_summary_recorded:false,
    operator_summary_persisted:false,
    operator_summary_materialized:false,
    operator_summary_filesystem_written:false,
    operator_summary_delivered:false,
    operator_briefing_recorded:false,
    operator_briefing_persisted:false,
    operator_briefing_materialized:false,
    operator_briefing_filesystem_written:false,
    operator_briefing_delivered:false,
    telegram_send_performed:false,
    channel_send_performed:false,
    external_send_performed:false,
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
    rollback_executed:false,
    public_release_claimed:false,
    service_restart_performed:false,
    active_binary_mutated:false,
    activation_request_surfaces:[
      "source_operator_acknowledgement_non_acceptance_report_required",
      "activation_request_shape_denied",
      "activation_request_recording_denied",
      "activation_request_persistence_denied",
      "activation_identity_scope_denied",
      "activation_nonce_generation_denied",
      "runtime_attachment_activation_denied",
      "live_context_context_injection_activation_denied",
      "adapter_provider_model_activation_denied",
      "memory_kg_activation_denied",
      "receipt_readback_router_handoff_activation_denied",
      "external_public_install_restart_active_binary_activation_denied"
    ],
    activation_request_fixtures:$fixtures,
    denied_by_activation_request_denial_matrix:[
      "source_operator_acknowledgement_non_acceptance_report_required",
      "activation_request_acceptance_denied",
      "activation_request_recording_denied",
      "activation_request_persistence_denied",
      "activation_request_materialization_denied",
      "activation_request_filesystem_write_denied",
      "activation_request_delivery_denied",
      "activation_request_execution_denied",
      "activation_request_activation_denied",
      "operator_acknowledgement_acceptance_denied",
      "operator_identity_acceptance_denied",
      "operator_scope_acceptance_denied",
      "activation_nonce_acceptance_denied",
      "activation_generation_acceptance_denied",
      "runtime_attachment_denied",
      "live_context_attachment_denied",
      "context_injection_denied",
      "adapter_invocation_denied",
      "provider_model_invocation_denied",
      "memory_store_write_denied",
      "live_kg_write_denied",
      "receipt_record_persist_accept_denied",
      "receipt_export_query_observability_denied",
      "router_handoff_readback_persistence_denied",
      "usage_recording_denied",
      "secret_material_read_denied",
      "external_public_install_restart_active_binary_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_activation_request_denial_matrix",
        status:"allowed_report_only",
        accepts_activation_request:false,
        executes_activation:false,
        mutates_runtime:false,
        invokes_adapter:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_activation_command_noop_handoff",
        status:"allowed_report_only_next_slice",
        accepts_activation_request:false,
        records_activation:false,
        executes_activation:false,
        attaches_live_context:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        mutates_runtime:false,
        attaches_live_context:false,
        invokes_model:false,
        writes_kg:false
      }
    ],
    source_operator_acknowledgement_non_acceptance_report_required:true,
    activation_request_acceptance_forbidden:true,
    activation_request_recording_forbidden:true,
    activation_request_persistence_forbidden:true,
    activation_request_execution_forbidden:true,
    activation_runtime_mutation_forbidden:true,
    live_context_attachment_forbidden:true,
    context_injection_forbidden:true,
    adapter_invocation_forbidden:true,
    provider_model_invocation_forbidden:true,
    memory_kg_write_forbidden:true,
    auth_secret_read_forbidden:true,
    usage_recording_forbidden:true,
    side_effects:{
      activation_request_recorded:false,
      activation_request_persisted:false,
      activation_request_materialized:false,
      activation_request_filesystem_written:false,
      activation_request_delivered:false,
      activation_request_executed:false,
      activation_activated:false,
      activation_nonce_accepted:false,
      activation_generation_accepted:false,
      operator_acknowledgement_accepted:false,
      operator_identity_accepted:false,
      operator_scope_accepted:false,
      operator_activation_plan_accepted:false,
      runtime_attachment_acknowledged:false,
      live_context_acknowledged:false,
      memory_kg_acknowledged:false,
      provider_secret_acknowledged:false,
      operator_summary_recorded:false,
      operator_summary_persisted:false,
      operator_summary_materialized:false,
      operator_summary_filesystem_written:false,
      operator_summary_delivered:false,
      operator_briefing_recorded:false,
      operator_briefing_persisted:false,
      operator_briefing_materialized:false,
      operator_briefing_filesystem_written:false,
      operator_briefing_delivered:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
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
      rollback_executed:false,
      filesystem_written:false,
      public_release_claimed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_gate"
  and .activation_request_denial_matrix_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_v1"
  and .runtime_provider_router_activation_request_denial_matrix_ready == true
  and .runtime_provider_router_activation_request_denial_matrix_status == "blocked"
  and .operator_acknowledgement_non_acceptance_ready == true
  and .operator_acknowledgement_non_acceptance_status == "blocked"
  and .operator_acknowledgement_fixture_count == 10
  and .blocked_operator_acknowledgement_fixture_count == 10
  and .noop_operator_acknowledgement_fixture_count == 10
  and .allowed_operator_acknowledgement_fixture_count == 0
  and .accepted_operator_acknowledgement_fixture_count == 0
  and .operator_acknowledgement_denied_count == 10
  and .operator_acknowledgement_performed_count == 0
  and .activation_request_surface_count == 12
  and .activation_request_surface_ready_count == 12
  and .activation_request_side_effect_free_surface_count == 12
  and .activation_request_fixture_count == 10
  and .blocked_activation_request_fixture_count == 10
  and .noop_activation_request_fixture_count == 10
  and .allowed_activation_request_fixture_count == 0
  and .accepted_activation_request_fixture_count == 0
  and .activation_request_denied_count == 10
  and .activation_request_performed_count == 0
  and .activation_execution_performed_count == 0
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
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.activation_request_surfaces | length) == 12
  and (.activation_request_fixtures | length) == 10
  and (.activation_request_fixtures | all((.activation_request_status == "blocked_noop" or .activation_request_status == "blocked_activation_noop" or .activation_request_status == "blocked_identity_scope_noop" or .activation_request_status == "blocked_nonce_generation_noop" or .activation_request_status == "blocked_runtime_noop" or .activation_request_status == "blocked_context_noop" or .activation_request_status == "blocked_provider_noop" or .activation_request_status == "blocked_memory_kg_noop" or .activation_request_status == "blocked_receipt_router_noop" or .activation_request_status == "blocked_external_noop") and .activation_request_recorded == false and .activation_request_persisted == false and .activation_request_executed == false and .activation_activated == false and .runtime_router_mutated == false and .runtime_attachment_performed == false and .live_context_attached == false and .context_injection_performed == false and .adapter_invoked == false and .provider_invoked == false and .model_invoked == false and .auth_secret_read == false and .credential_read == false and .secret_file_read == false and .usage_recorded == false and .memory_store_write_performed == false and .memory_store_mutated == false and .live_kg_write_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .readback_evidence_recorded == false and .readback_evidence_persisted == false and .router_handoff_recorded == false and .router_handoff_persisted == false and .activation_request_noop_confirmed == true))
  and (.denied_by_activation_request_denial_matrix | length) == 27
  and (.allowed_next_actions | any(.action == "review_runtime_provider_router_activation_request_denial_matrix" and .status == "allowed_report_only" and .accepts_activation_request == false and .executes_activation == false and .mutates_runtime == false and .invokes_adapter == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_noop_handoff" and .status == "allowed_report_only_next_slice" and .accepts_activation_request == false and .records_activation == false and .executes_activation == false and .attaches_live_context == false and .invokes_model == false))
  and .source_operator_acknowledgement_non_acceptance_report_required == true
  and .activation_request_acceptance_forbidden == true
  and .activation_request_recording_forbidden == true
  and .activation_request_persistence_forbidden == true
  and .activation_request_execution_forbidden == true
  and .activation_runtime_mutation_forbidden == true
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
echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation request denial matrix gate passed"
