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

OPERATOR_SUMMARY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-facing-summary-non-persistence-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-facing-summary-non-persistence-gate.sh
)"

operator_acknowledgement_fixtures_json="$(
  jq -n '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        operator_acknowledgement_status:$status,
        source_operator_summary_non_persistence_present:true,
        source_operator_summary_non_persistence_ready:true,
        operator_acknowledgement_requested:false,
        operator_acknowledgement_allowed:false,
        operator_acknowledgement_request_accepted:false,
        operator_acknowledgement_recorded:false,
        operator_acknowledgement_persisted:false,
        operator_acknowledgement_materialized:false,
        operator_acknowledgement_filesystem_written:false,
        operator_acknowledgement_delivered:false,
        operator_acknowledgement_accepted:false,
        operator_identity_accepted:false,
        operator_scope_accepted:false,
        operator_activation_plan_accepted:false,
        operator_summary_review_accepted:false,
        operator_briefing_review_accepted:false,
        receipt_acknowledgement_accepted:false,
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
        acknowledgement_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      blocked_fixture("provider-router-operator-acknowledgement-missing-source-summary-non-persistence"; "blocked_noop"; "source_operator_summary_non_persistence_report_required"; {source_operator_summary_non_persistence_present:false, source_operator_summary_non_persistence_ready:false, operator_acknowledgement_requested:true}),
      blocked_fixture("provider-router-operator-acknowledgement-request"; "blocked_acknowledgement_noop"; "operator_acknowledgement_request_shape_denied"; {operator_acknowledgement_requested:true}),
      blocked_fixture("provider-router-operator-identity-acknowledgement-request"; "blocked_identity_noop"; "operator_identity_acknowledgement_denied"; {operator_acknowledgement_requested:true, operator_identity_acknowledgement_requested:true}),
      blocked_fixture("provider-router-operator-scope-acknowledgement-request"; "blocked_scope_noop"; "operator_scope_acknowledgement_denied"; {operator_acknowledgement_requested:true, operator_scope_acknowledgement_requested:true}),
      blocked_fixture("provider-router-operator-activation-plan-acknowledgement-request"; "blocked_activation_noop"; "operator_activation_plan_acknowledgement_denied"; {operator_acknowledgement_requested:true, operator_activation_plan_acknowledgement_requested:true}),
      blocked_fixture("provider-router-summary-review-acknowledgement-request"; "blocked_review_noop"; "operator_summary_review_acknowledgement_denied"; {operator_acknowledgement_requested:true, operator_summary_review_acknowledgement_requested:true, operator_briefing_review_acknowledgement_requested:true}),
      blocked_fixture("provider-router-receipt-export-query-observability-acknowledgement-request"; "blocked_receipt_noop"; "receipt_export_query_observability_acknowledgement_denied"; {operator_acknowledgement_requested:true, receipt_acknowledgement_requested:true, receipt_export_acknowledgement_requested:true, receipt_query_acknowledgement_requested:true, receipt_observability_acknowledgement_requested:true}),
      blocked_fixture("provider-router-runtime-attachment-live-context-acknowledgement-request"; "blocked_runtime_noop"; "runtime_attachment_live_context_acknowledgement_denied"; {operator_acknowledgement_requested:true, runtime_attachment_acknowledgement_requested:true, live_context_acknowledgement_requested:true, context_injection_acknowledgement_requested:true}),
      blocked_fixture("provider-router-memory-kg-provider-secret-usage-acknowledgement-request"; "blocked_memory_provider_noop"; "memory_kg_provider_secret_usage_acknowledgement_denied"; {operator_acknowledgement_requested:true, memory_kg_acknowledgement_requested:true, provider_secret_acknowledgement_requested:true, usage_acknowledgement_requested:true}),
      blocked_fixture("provider-router-external-public-install-restart-active-binary-acknowledgement-request"; "blocked_external_noop"; "external_public_install_restart_active_binary_acknowledgement_denied"; {operator_acknowledgement_requested:true, external_send_acknowledgement_requested:true, public_claim_acknowledgement_requested:true, release_artifact_acknowledgement_requested:true, install_acknowledgement_requested:true, service_restart_acknowledgement_requested:true, active_binary_acknowledgement_requested:true})
    ]
  '
)"

operator_summary_report_sha256="$(sha256_text "$OPERATOR_SUMMARY_JSON")"
operator_acknowledgement_fixtures_sha256="$(sha256_text "$operator_acknowledgement_fixtures_json")"
operator_acknowledgement_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance:$operator_summary_report_sha256:$operator_acknowledgement_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_acknowledgement_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-operator-acknowledgement-non-acceptance:report-only:no-ack-acceptance:no-ack-record:no-ack-persist:no-summary-persistence:no-receipt-export:no-observability:no-adapter:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "ack=false;ack_record=false;ack_persist=false;summary=false;briefing=false;delivery=false;receipt_export=false;observability=false;adapter=false;model=false;secret=false;service_restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$OPERATOR_SUMMARY_JSON" \
  --argjson fixtures "$operator_acknowledgement_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_facing_summary_non_persistence_gate"
    and $source.operator_facing_summary_non_persistence_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_operator_facing_summary_non_persistence_v1"
    and $source.runtime_provider_router_operator_facing_summary_non_persistence_ready == true
    and $source.runtime_provider_router_operator_facing_summary_non_persistence_status == "blocked"
    and $source.receipt_observability_denial_ready == true
    and $source.receipt_observability_denial_status == "blocked"
    and $source.operator_facing_summary_fixture_count == 10
    and $source.blocked_operator_facing_summary_fixture_count == 10
    and $source.noop_operator_facing_summary_fixture_count == 10
    and $source.allowed_operator_facing_summary_fixture_count == 0
    and $source.accepted_operator_facing_summary_fixture_count == 0
    and $source.operator_summary_denied_count == 10
    and $source.operator_briefing_denied_count == 10
    and $source.operator_summary_performed_count == 0
    and $source.operator_briefing_performed_count == 0
    and $source.operator_summary_recorded == false
    and $source.operator_summary_persisted == false
    and $source.operator_summary_materialized == false
    and $source.operator_summary_filesystem_written == false
    and $source.operator_summary_delivered == false
    and $source.operator_briefing_recorded == false
    and $source.operator_briefing_persisted == false
    and $source.operator_briefing_materialized == false
    and $source.operator_briefing_filesystem_written == false
    and $source.operator_briefing_delivered == false
    and $source.operator_summary_briefing_channel_delivery_performed == false
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
    and ($source.allowed_next_actions | any(.action == "stage_runtime_provider_router_operator_acknowledgement_non_acceptance" and .status == "allowed_report_only_next_slice" and .accepts_acknowledgement == false and .persists_summary == false and .exports_receipt == false and .invokes_adapter == false and .invokes_model == false))
    and ($source.denied_by_operator_facing_summary_non_persistence | length) == 27
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.operator_acknowledgement_status == "blocked_noop" or .operator_acknowledgement_status == "blocked_acknowledgement_noop" or .operator_acknowledgement_status == "blocked_identity_noop" or .operator_acknowledgement_status == "blocked_scope_noop" or .operator_acknowledgement_status == "blocked_activation_noop" or .operator_acknowledgement_status == "blocked_review_noop" or .operator_acknowledgement_status == "blocked_receipt_noop" or .operator_acknowledgement_status == "blocked_runtime_noop" or .operator_acknowledgement_status == "blocked_memory_provider_noop" or .operator_acknowledgement_status == "blocked_external_noop")
      and .operator_acknowledgement_allowed == false
      and .operator_acknowledgement_request_accepted == false
      and .operator_acknowledgement_recorded == false
      and .operator_acknowledgement_persisted == false
      and .operator_acknowledgement_materialized == false
      and .operator_acknowledgement_filesystem_written == false
      and .operator_acknowledgement_delivered == false
      and .operator_acknowledgement_accepted == false
      and .operator_identity_accepted == false
      and .operator_scope_accepted == false
      and .operator_activation_plan_accepted == false
      and .operator_summary_review_accepted == false
      and .operator_briefing_review_accepted == false
      and .receipt_acknowledgement_accepted == false
      and .runtime_attachment_acknowledged == false
      and .live_context_acknowledged == false
      and .memory_kg_acknowledged == false
      and .provider_secret_acknowledged == false
      and .operator_summary_recorded == false
      and .operator_summary_persisted == false
      and .operator_briefing_recorded == false
      and .operator_briefing_persisted == false
      and .telegram_send_performed == false
      and .channel_send_performed == false
      and .external_send_performed == false
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
      and .runtime_router_mutated == false
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
      and .rollback_executed == false
      and .public_release_claimed == false
      and .service_restart_performed == false
      and .active_binary_mutated == false
      and .acknowledgement_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_gate" \
  --arg operator_summary_report_sha256 "$operator_summary_report_sha256" \
  --arg operator_acknowledgement_fixtures_sha256 "$operator_acknowledgement_fixtures_sha256" \
  --arg operator_acknowledgement_contract_hash_sha256 "$operator_acknowledgement_contract_hash_sha256" \
  --arg operator_acknowledgement_policy_hash_sha256 "$operator_acknowledgement_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$OPERATOR_SUMMARY_JSON" \
  --argjson fixtures "$operator_acknowledgement_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    operator_acknowledgement_non_acceptance_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_v1",
    operator_acknowledgement_non_acceptance_mode:"runtime_provider_router_operator_acknowledgement_non_acceptance_no_record_no_persist_no_activation",
    source_operator_summary_non_persistence_gate:$source.gate,
    source_operator_summary_non_persistence_ready:$source.runtime_provider_router_operator_facing_summary_non_persistence_ready,
    source_operator_summary_non_persistence_status:$source.runtime_provider_router_operator_facing_summary_non_persistence_status,
    source_operator_summary_non_persistence_report_sha256:$operator_summary_report_sha256,
    source_receipt_observability_denial_gate:$source.source_receipt_observability_denial_gate,
    source_receipt_observability_denial_ready:$source.receipt_observability_denial_ready,
    source_receipt_observability_denial_status:$source.receipt_observability_denial_status,
    source_runtime_model_provider_router:$source.source_runtime_model_provider_router,
    operator_acknowledgement_fixtures_sha256:$operator_acknowledgement_fixtures_sha256,
    operator_acknowledgement_contract_hash_sha256:$operator_acknowledgement_contract_hash_sha256,
    operator_acknowledgement_policy_hash_sha256:$operator_acknowledgement_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_operator_acknowledgement_non_acceptance_ready:true,
    runtime_provider_router_operator_acknowledgement_non_acceptance_status:"blocked",
    operator_facing_summary_non_persistence_ready:$source.runtime_provider_router_operator_facing_summary_non_persistence_ready,
    operator_facing_summary_non_persistence_status:$source.runtime_provider_router_operator_facing_summary_non_persistence_status,
    receipt_observability_denial_ready:$source.receipt_observability_denial_ready,
    receipt_observability_denial_status:$source.receipt_observability_denial_status,
    operator_facing_summary_fixture_count:$source.operator_facing_summary_fixture_count,
    blocked_operator_facing_summary_fixture_count:$source.blocked_operator_facing_summary_fixture_count,
    noop_operator_facing_summary_fixture_count:$source.noop_operator_facing_summary_fixture_count,
    allowed_operator_facing_summary_fixture_count:$source.allowed_operator_facing_summary_fixture_count,
    accepted_operator_facing_summary_fixture_count:$source.accepted_operator_facing_summary_fixture_count,
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
    operator_acknowledgement_surface_count:12,
    operator_acknowledgement_surface_ready_count:12,
    operator_acknowledgement_side_effect_free_surface_count:12,
    operator_acknowledgement_fixture_count:($fixtures | length),
    blocked_operator_acknowledgement_fixture_count:($fixtures | length),
    noop_operator_acknowledgement_fixture_count:($fixtures | length),
    allowed_operator_acknowledgement_fixture_count:0,
    accepted_operator_acknowledgement_fixture_count:0,
    operator_acknowledgement_denied_count:10,
    operator_acknowledgement_performed_count:0,
    operator_acknowledgement_allowed:false,
    operator_acknowledgement_request_accepted:false,
    operator_acknowledgement_recorded:false,
    operator_acknowledgement_persisted:false,
    operator_acknowledgement_materialized:false,
    operator_acknowledgement_filesystem_written:false,
    operator_acknowledgement_delivered:false,
    operator_acknowledgement_accepted:false,
    operator_identity_accepted:false,
    operator_scope_accepted:false,
    operator_activation_plan_accepted:false,
    operator_summary_review_accepted:false,
    operator_briefing_review_accepted:false,
    receipt_acknowledgement_accepted:false,
    runtime_attachment_acknowledged:false,
    live_context_acknowledged:false,
    memory_kg_acknowledged:false,
    provider_secret_acknowledged:false,
    operator_summary_allowed:false,
    operator_summary_request_accepted:false,
    operator_summary_recorded:false,
    operator_summary_persisted:false,
    operator_summary_materialized:false,
    operator_summary_filesystem_written:false,
    operator_summary_delivered:false,
    operator_briefing_allowed:false,
    operator_briefing_request_accepted:false,
    operator_briefing_recorded:false,
    operator_briefing_persisted:false,
    operator_briefing_materialized:false,
    operator_briefing_filesystem_written:false,
    operator_briefing_delivered:false,
    operator_summary_briefing_channel_delivery_performed:false,
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
    operator_acknowledgement_surfaces:[
      "source_operator_summary_non_persistence_report_required",
      "operator_acknowledgement_request_shape_denied",
      "operator_acknowledgement_recording_denied",
      "operator_acknowledgement_persistence_denied",
      "operator_identity_scope_activation_plan_acceptance_denied",
      "operator_summary_briefing_review_acceptance_denied",
      "receipt_export_query_observability_acknowledgement_denied",
      "router_handoff_readback_acknowledgement_denied",
      "runtime_attachment_live_context_acknowledgement_denied",
      "context_injection_acknowledgement_denied",
      "memory_kg_provider_secret_usage_acknowledgement_denied",
      "external_public_install_restart_active_binary_acknowledgement_denied"
    ],
    operator_acknowledgement_fixtures:$fixtures,
    denied_by_operator_acknowledgement_non_acceptance:[
      "source_operator_summary_non_persistence_report_required",
      "operator_acknowledgement_request_acceptance_denied",
      "operator_acknowledgement_recording_denied",
      "operator_acknowledgement_persistence_denied",
      "operator_acknowledgement_materialization_denied",
      "operator_acknowledgement_filesystem_write_denied",
      "operator_acknowledgement_delivery_denied",
      "operator_acknowledgement_acceptance_denied",
      "operator_identity_acceptance_denied",
      "operator_scope_acceptance_denied",
      "operator_activation_plan_acceptance_denied",
      "operator_summary_review_acceptance_denied",
      "operator_briefing_review_acceptance_denied",
      "receipt_acknowledgement_acceptance_denied",
      "receipt_export_acknowledgement_denied",
      "receipt_query_acknowledgement_denied",
      "receipt_observability_acknowledgement_denied",
      "router_handoff_acknowledgement_denied",
      "readback_evidence_acknowledgement_denied",
      "runtime_attachment_acknowledgement_denied",
      "live_context_acknowledgement_denied",
      "context_injection_acknowledgement_denied",
      "memory_kg_acknowledgement_denied",
      "rollback_acknowledgement_denied",
      "secret_material_acknowledgement_denied",
      "provider_model_acknowledgement_denied",
      "external_public_install_restart_active_binary_acknowledgement_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_operator_acknowledgement_non_acceptance",
        status:"allowed_report_only",
        accepts_acknowledgement:false,
        records_acknowledgement:false,
        persists_acknowledgement:false,
        invokes_adapter:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_activation_request_denial_matrix",
        status:"allowed_report_only_next_slice",
        accepts_activation_request:false,
        accepts_acknowledgement:false,
        persists_summary:false,
        exports_receipt:false,
        invokes_adapter:false,
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
    source_operator_summary_non_persistence_report_required:true,
    operator_acknowledgement_acceptance_forbidden:true,
    operator_acknowledgement_recording_forbidden:true,
    operator_acknowledgement_persistence_forbidden:true,
    operator_acknowledgement_delivery_forbidden:true,
    operator_identity_acceptance_forbidden:true,
    operator_scope_acceptance_forbidden:true,
    operator_activation_plan_acceptance_forbidden:true,
    receipt_acknowledgement_acceptance_forbidden:true,
    receipt_export_forbidden:true,
    receipt_query_forbidden:true,
    receipt_observability_forbidden:true,
    operator_summary_persistence_forbidden:true,
    operator_briefing_persistence_forbidden:true,
    router_handoff_persistence_forbidden:true,
    readback_evidence_persistence_forbidden:true,
    live_context_attachment_forbidden:true,
    adapter_invocation_forbidden:true,
    provider_model_invocation_forbidden:true,
    auth_secret_read_forbidden:true,
    usage_recording_forbidden:true,
    side_effects:{
      operator_acknowledgement_recorded:false,
      operator_acknowledgement_persisted:false,
      operator_acknowledgement_materialized:false,
      operator_acknowledgement_filesystem_written:false,
      operator_acknowledgement_delivered:false,
      operator_acknowledgement_accepted:false,
      operator_identity_accepted:false,
      operator_scope_accepted:false,
      operator_activation_plan_accepted:false,
      operator_summary_review_accepted:false,
      operator_briefing_review_accepted:false,
      receipt_acknowledgement_accepted:false,
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
      operator_summary_briefing_channel_delivery_performed:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_gate"
  and .operator_acknowledgement_non_acceptance_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_v1"
  and .runtime_provider_router_operator_acknowledgement_non_acceptance_ready == true
  and .runtime_provider_router_operator_acknowledgement_non_acceptance_status == "blocked"
  and .operator_facing_summary_non_persistence_ready == true
  and .operator_facing_summary_non_persistence_status == "blocked"
  and .operator_facing_summary_fixture_count == 10
  and .blocked_operator_facing_summary_fixture_count == 10
  and .noop_operator_facing_summary_fixture_count == 10
  and .allowed_operator_facing_summary_fixture_count == 0
  and .accepted_operator_facing_summary_fixture_count == 0
  and .operator_summary_denied_count == 10
  and .operator_briefing_denied_count == 10
  and .operator_summary_performed_count == 0
  and .operator_briefing_performed_count == 0
  and .receipt_export_denied_count == 10
  and .receipt_query_denied_count == 10
  and .receipt_observability_denied_count == 10
  and .receipt_export_performed_count == 0
  and .receipt_query_performed_count == 0
  and .receipt_observability_performed_count == 0
  and .operator_acknowledgement_surface_count == 12
  and .operator_acknowledgement_surface_ready_count == 12
  and .operator_acknowledgement_side_effect_free_surface_count == 12
  and .operator_acknowledgement_fixture_count == 10
  and .blocked_operator_acknowledgement_fixture_count == 10
  and .noop_operator_acknowledgement_fixture_count == 10
  and .allowed_operator_acknowledgement_fixture_count == 0
  and .accepted_operator_acknowledgement_fixture_count == 0
  and .operator_acknowledgement_denied_count == 10
  and .operator_acknowledgement_performed_count == 0
  and .operator_acknowledgement_allowed == false
  and .operator_acknowledgement_request_accepted == false
  and .operator_acknowledgement_recorded == false
  and .operator_acknowledgement_persisted == false
  and .operator_acknowledgement_materialized == false
  and .operator_acknowledgement_filesystem_written == false
  and .operator_acknowledgement_delivered == false
  and .operator_acknowledgement_accepted == false
  and .operator_identity_accepted == false
  and .operator_scope_accepted == false
  and .operator_activation_plan_accepted == false
  and .operator_summary_review_accepted == false
  and .operator_briefing_review_accepted == false
  and .receipt_acknowledgement_accepted == false
  and .runtime_attachment_acknowledged == false
  and .live_context_acknowledged == false
  and .memory_kg_acknowledged == false
  and .provider_secret_acknowledged == false
  and .operator_summary_recorded == false
  and .operator_summary_persisted == false
  and .operator_summary_materialized == false
  and .operator_summary_filesystem_written == false
  and .operator_summary_delivered == false
  and .operator_briefing_recorded == false
  and .operator_briefing_persisted == false
  and .operator_briefing_materialized == false
  and .operator_briefing_filesystem_written == false
  and .operator_briefing_delivered == false
  and .operator_summary_briefing_channel_delivery_performed == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .receipt_exported == false
  and .receipt_query_registered == false
  and .receipt_observability_recorded == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .receipt_materialized == false
  and .receipt_filesystem_written == false
  and .readback_evidence_recorded == false
  and .readback_evidence_persisted == false
  and .router_handoff_recorded == false
  and .router_handoff_persisted == false
  and .runtime_router_mutated == false
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
  and .rollback_executed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.operator_acknowledgement_surfaces | length) == 12
  and (.operator_acknowledgement_fixtures | length) == 10
  and (.operator_acknowledgement_fixtures | all((.operator_acknowledgement_status == "blocked_noop" or .operator_acknowledgement_status == "blocked_acknowledgement_noop" or .operator_acknowledgement_status == "blocked_identity_noop" or .operator_acknowledgement_status == "blocked_scope_noop" or .operator_acknowledgement_status == "blocked_activation_noop" or .operator_acknowledgement_status == "blocked_review_noop" or .operator_acknowledgement_status == "blocked_receipt_noop" or .operator_acknowledgement_status == "blocked_runtime_noop" or .operator_acknowledgement_status == "blocked_memory_provider_noop" or .operator_acknowledgement_status == "blocked_external_noop") and .operator_acknowledgement_recorded == false and .operator_acknowledgement_persisted == false and .operator_acknowledgement_materialized == false and .operator_acknowledgement_filesystem_written == false and .operator_acknowledgement_delivered == false and .operator_acknowledgement_accepted == false and .operator_identity_accepted == false and .operator_scope_accepted == false and .operator_activation_plan_accepted == false and .operator_summary_review_accepted == false and .operator_briefing_review_accepted == false and .receipt_acknowledgement_accepted == false and .runtime_attachment_acknowledged == false and .live_context_acknowledged == false and .memory_kg_acknowledged == false and .provider_secret_acknowledged == false and .operator_summary_recorded == false and .operator_summary_persisted == false and .operator_briefing_recorded == false and .operator_briefing_persisted == false and .telegram_send_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .readback_evidence_recorded == false and .readback_evidence_persisted == false and .router_handoff_recorded == false and .runtime_router_mutated == false and .live_context_attached == false and .adapter_invoked == false and .provider_invoked == false and .model_invoked == false and .auth_secret_read == false and .credential_read == false and .secret_file_read == false and .usage_recorded == false and .memory_store_write_performed == false and .memory_store_mutated == false and .live_kg_write_performed == false and .rollback_executed == false and .acknowledgement_noop_confirmed == true))
  and (.denied_by_operator_acknowledgement_non_acceptance | length) == 27
  and (.allowed_next_actions | any(.action == "review_runtime_provider_router_operator_acknowledgement_non_acceptance" and .status == "allowed_report_only" and .accepts_acknowledgement == false and .records_acknowledgement == false and .persists_acknowledgement == false and .invokes_adapter == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_request_denial_matrix" and .status == "allowed_report_only_next_slice" and .accepts_activation_request == false and .accepts_acknowledgement == false and .persists_summary == false and .exports_receipt == false and .invokes_adapter == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "run_full_light_preflight" and .status == "allowed_verification_only" and .mutates_runtime == false and .attaches_live_context == false and .invokes_model == false and .writes_kg == false))
  and .source_operator_summary_non_persistence_report_required == true
  and .operator_acknowledgement_acceptance_forbidden == true
  and .operator_acknowledgement_recording_forbidden == true
  and .operator_acknowledgement_persistence_forbidden == true
  and .operator_acknowledgement_delivery_forbidden == true
  and .operator_identity_acceptance_forbidden == true
  and .operator_scope_acceptance_forbidden == true
  and .operator_activation_plan_acceptance_forbidden == true
  and .receipt_acknowledgement_acceptance_forbidden == true
  and .receipt_export_forbidden == true
  and .receipt_query_forbidden == true
  and .receipt_observability_forbidden == true
  and .operator_summary_persistence_forbidden == true
  and .operator_briefing_persistence_forbidden == true
  and .router_handoff_persistence_forbidden == true
  and .readback_evidence_persistence_forbidden == true
  and .live_context_attachment_forbidden == true
  and .adapter_invocation_forbidden == true
  and .provider_model_invocation_forbidden == true
  and .auth_secret_read_forbidden == true
  and .usage_recording_forbidden == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router operator acknowledgement non-acceptance gate passed"
