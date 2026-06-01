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

RECEIPT_OBSERVABILITY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-receipt-observability-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-receipt-observability-denial-gate.sh
)"

operator_summary_fixtures_json="$(
  jq -n '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        operator_summary_requested:false,
        operator_briefing_requested:false,
        operator_facing_summary_status:$status,
        source_receipt_observability_denial_present:true,
        source_receipt_observability_denial_ready:true,
        operator_summary_allowed:false,
        operator_summary_request_accepted:false,
        operator_summary_recorded:false,
        operator_summary_persisted:false,
        operator_summary_materialized:false,
        operator_summary_filesystem_written:false,
        operator_summary_delivered:false,
        operator_summary_channel_delivery_performed:false,
        operator_briefing_allowed:false,
        operator_briefing_request_accepted:false,
        operator_briefing_recorded:false,
        operator_briefing_persisted:false,
        operator_briefing_materialized:false,
        operator_briefing_filesystem_written:false,
        operator_briefing_delivered:false,
        operator_briefing_channel_delivery_performed:false,
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
        public_release_claimed:false,
        service_restart_performed:false,
        active_binary_mutated:false,
        summary_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      blocked_fixture("provider-router-operator-summary-missing-source-receipt-observability-denial"; "blocked_noop"; "source_receipt_observability_denial_report_required"; {source_receipt_observability_denial_present:false, source_receipt_observability_denial_ready:false, operator_summary_requested:true}),
      blocked_fixture("provider-router-operator-summary-request"; "blocked_summary_noop"; "operator_summary_request_shape_denied"; {operator_summary_requested:true}),
      blocked_fixture("provider-router-operator-briefing-request"; "blocked_briefing_noop"; "operator_briefing_request_shape_denied"; {operator_briefing_requested:true}),
      blocked_fixture("provider-router-operator-summary-materialization-request"; "blocked_summary_noop"; "operator_summary_materialization_denied"; {operator_summary_requested:true, operator_summary_materialization_requested:true}),
      blocked_fixture("provider-router-operator-briefing-materialization-request"; "blocked_briefing_noop"; "operator_briefing_materialization_denied"; {operator_briefing_requested:true, operator_briefing_materialization_requested:true}),
      blocked_fixture("provider-router-operator-summary-persistence-filesystem-request"; "blocked_summary_noop"; "operator_summary_persistence_filesystem_write_denied"; {operator_summary_requested:true, operator_summary_persistence_requested:true, operator_summary_filesystem_write_requested:true}),
      blocked_fixture("provider-router-operator-briefing-persistence-filesystem-request"; "blocked_briefing_noop"; "operator_briefing_persistence_filesystem_write_denied"; {operator_briefing_requested:true, operator_briefing_persistence_requested:true, operator_briefing_filesystem_write_requested:true}),
      blocked_fixture("provider-router-operator-summary-briefing-delivery-request"; "blocked_delivery_noop"; "operator_summary_briefing_channel_delivery_denied"; {operator_summary_requested:true, operator_briefing_requested:true, channel_delivery_requested:true, telegram_send_requested:true}),
      blocked_fixture("provider-router-operator-summary-runtime-activation-request"; "blocked_summary_noop"; "runtime_attachment_memory_kg_provider_summary_denied"; {operator_summary_requested:true, operator_briefing_requested:true, activation_from_summary_requested:true, runtime_attachment_summary_requested:true, live_context_summary_requested:true, memory_kg_summary_requested:true, provider_prompt_summary_requested:true, secret_material_summary_requested:true}),
      blocked_fixture("provider-router-operator-summary-external-public-install-request"; "blocked_delivery_noop"; "external_public_install_restart_active_binary_summary_denied"; {operator_summary_requested:true, operator_briefing_requested:true, external_send_summary_requested:true, public_claim_summary_requested:true, release_artifact_summary_requested:true, install_summary_requested:true, service_restart_summary_requested:true, active_binary_summary_requested:true})
    ]
  '
)"

receipt_observability_report_sha256="$(sha256_text "$RECEIPT_OBSERVABILITY_JSON")"
operator_summary_fixtures_sha256="$(sha256_text "$operator_summary_fixtures_json")"
operator_summary_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-operator-facing-summary-non-persistence:$receipt_observability_report_sha256:$operator_summary_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_summary_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-operator-facing-summary-non-persistence:report-only:no-summary-persistence:no-briefing-delivery:no-receipt-export:no-observability:no-adapter:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "summary=false;briefing=false;delivery=false;receipt_export=false;observability=false;adapter=false;model=false;secret=false;service_restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RECEIPT_OBSERVABILITY_JSON" \
  --argjson fixtures "$operator_summary_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_receipt_observability_denial_gate"
    and $source.receipt_observability_denial_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_receipt_observability_denial_v1"
    and $source.runtime_provider_router_receipt_observability_denial_ready == true
    and $source.runtime_provider_router_receipt_observability_denial_status == "blocked"
    and $source.readback_receipt_skeleton_ready == true
    and $source.readback_receipt_skeleton_status == "blocked"
    and $source.receipt_observability_fixture_count == 10
    and $source.blocked_receipt_observability_fixture_count == 10
    and $source.noop_receipt_observability_fixture_count == 10
    and $source.allowed_receipt_observability_fixture_count == 0
    and $source.accepted_receipt_observability_fixture_count == 0
    and $source.receipt_export_denied_count == 10
    and $source.receipt_query_denied_count == 10
    and $source.receipt_observability_denied_count == 10
    and $source.receipt_export_performed_count == 0
    and $source.receipt_query_performed_count == 0
    and $source.receipt_observability_performed_count == 0
    and $source.receipt_export_recorded == false
    and $source.receipt_export_persisted == false
    and $source.receipt_export_artifact_written == false
    and $source.receipt_export_stream_opened == false
    and $source.receipt_query_registered == false
    and $source.receipt_query_endpoint_materialized == false
    and $source.receipt_observability_metric_emitted == false
    and $source.receipt_observability_log_recorded == false
    and $source.receipt_observability_trace_recorded == false
    and $source.receipt_observability_dashboard_materialized == false
    and $source.receipt_router_event_observability_recorded == false
    and $source.receipt_readback_ledger_observability_recorded == false
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
    and $source.memory_store_mutated == false
    and $source.live_kg_write_performed == false
    and $source.rollback_executed == false
    and $source.external_send_performed == false
    and $source.service_restart_performed == false
    and $source.active_binary_mutated == false
    and ($source.allowed_next_actions | any(.action == "stage_runtime_provider_router_operator_facing_summary_non_persistence" and .status == "allowed_report_only_next_slice" and .persists_summary == false and .exports_receipt == false and .records_observability == false and .invokes_adapter == false and .invokes_model == false))
    and ($source.denied_by_receipt_observability | length) == 27
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.operator_facing_summary_status == "blocked_noop" or .operator_facing_summary_status == "blocked_summary_noop" or .operator_facing_summary_status == "blocked_briefing_noop" or .operator_facing_summary_status == "blocked_delivery_noop")
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
      and .runtime_router_mutated == false
      and .live_context_attached == false
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
      and .summary_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_facing_summary_non_persistence_gate" \
  --arg receipt_observability_report_sha256 "$receipt_observability_report_sha256" \
  --arg operator_summary_fixtures_sha256 "$operator_summary_fixtures_sha256" \
  --arg operator_summary_contract_hash_sha256 "$operator_summary_contract_hash_sha256" \
  --arg operator_summary_policy_hash_sha256 "$operator_summary_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RECEIPT_OBSERVABILITY_JSON" \
  --argjson fixtures "$operator_summary_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    operator_facing_summary_non_persistence_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_operator_facing_summary_non_persistence_v1",
    operator_facing_summary_non_persistence_mode:"runtime_provider_router_operator_facing_summary_non_persistence_no_materialization_no_delivery_no_activation",
    source_receipt_observability_denial_gate:$source.gate,
    source_receipt_observability_denial_ready:$source.runtime_provider_router_receipt_observability_denial_ready,
    source_receipt_observability_denial_status:$source.runtime_provider_router_receipt_observability_denial_status,
    source_receipt_observability_denial_report_sha256:$receipt_observability_report_sha256,
    source_readback_receipt_skeleton_gate:$source.source_readback_receipt_skeleton_gate,
    source_readback_receipt_skeleton_ready:$source.readback_receipt_skeleton_ready,
    source_runtime_model_provider_router:$source.source_runtime_model_provider_router,
    operator_summary_fixtures_sha256:$operator_summary_fixtures_sha256,
    operator_summary_contract_hash_sha256:$operator_summary_contract_hash_sha256,
    operator_summary_policy_hash_sha256:$operator_summary_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_operator_facing_summary_non_persistence_ready:true,
    runtime_provider_router_operator_facing_summary_non_persistence_status:"blocked",
    receipt_observability_denial_ready:$source.runtime_provider_router_receipt_observability_denial_ready,
    receipt_observability_denial_status:$source.runtime_provider_router_receipt_observability_denial_status,
    readback_receipt_skeleton_ready:$source.readback_receipt_skeleton_ready,
    readback_receipt_skeleton_status:$source.readback_receipt_skeleton_status,
    receipt_observability_fixture_count:$source.receipt_observability_fixture_count,
    blocked_receipt_observability_fixture_count:$source.blocked_receipt_observability_fixture_count,
    noop_receipt_observability_fixture_count:$source.noop_receipt_observability_fixture_count,
    allowed_receipt_observability_fixture_count:$source.allowed_receipt_observability_fixture_count,
    accepted_receipt_observability_fixture_count:$source.accepted_receipt_observability_fixture_count,
    receipt_export_denied_count:$source.receipt_export_denied_count,
    receipt_query_denied_count:$source.receipt_query_denied_count,
    receipt_observability_denied_count:$source.receipt_observability_denied_count,
    receipt_export_performed_count:$source.receipt_export_performed_count,
    receipt_query_performed_count:$source.receipt_query_performed_count,
    receipt_observability_performed_count:$source.receipt_observability_performed_count,
    operator_facing_summary_surface_count:12,
    operator_facing_summary_surface_ready_count:12,
    operator_facing_summary_side_effect_free_surface_count:12,
    operator_facing_summary_fixture_count:($fixtures | length),
    blocked_operator_facing_summary_fixture_count:($fixtures | length),
    noop_operator_facing_summary_fixture_count:($fixtures | length),
    allowed_operator_facing_summary_fixture_count:0,
    accepted_operator_facing_summary_fixture_count:0,
    operator_summary_denied_count:10,
    operator_briefing_denied_count:10,
    operator_summary_performed_count:0,
    operator_briefing_performed_count:0,
    operator_summary_allowed:false,
    operator_summary_request_accepted:false,
    operator_summary_recorded:false,
    operator_summary_persisted:false,
    operator_summary_materialized:false,
    operator_summary_filesystem_written:false,
    operator_summary_delivered:false,
    operator_summary_channel_delivery_performed:false,
    operator_briefing_allowed:false,
    operator_briefing_request_accepted:false,
    operator_briefing_recorded:false,
    operator_briefing_persisted:false,
    operator_briefing_materialized:false,
    operator_briefing_filesystem_written:false,
    operator_briefing_delivered:false,
    operator_briefing_channel_delivery_performed:false,
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
    operator_facing_summary_surfaces:[
      "source_receipt_observability_denial_report_required",
      "operator_summary_request_shape_denied",
      "operator_briefing_request_shape_denied",
      "operator_summary_materialization_denied",
      "operator_briefing_materialization_denied",
      "operator_summary_persistence_denied",
      "operator_briefing_persistence_denied",
      "operator_summary_delivery_denied",
      "operator_briefing_delivery_denied",
      "summary_derived_activation_denied",
      "runtime_memory_kg_provider_secret_summary_denied",
      "external_public_install_restart_active_binary_summary_denied"
    ],
    operator_facing_summary_fixtures:$fixtures,
    denied_by_operator_facing_summary_non_persistence:[
      "source_receipt_observability_denial_report_required",
      "operator_summary_request_acceptance_denied",
      "operator_briefing_request_acceptance_denied",
      "operator_summary_recording_denied",
      "operator_briefing_recording_denied",
      "operator_summary_persistence_denied",
      "operator_briefing_persistence_denied",
      "operator_summary_materialization_denied",
      "operator_briefing_materialization_denied",
      "operator_summary_filesystem_write_denied",
      "operator_briefing_filesystem_write_denied",
      "operator_summary_delivery_denied",
      "operator_briefing_delivery_denied",
      "telegram_send_denied",
      "receipt_export_denied",
      "receipt_query_denied",
      "receipt_observability_denied",
      "router_event_summary_denied",
      "readback_ledger_summary_denied",
      "activation_from_summary_denied",
      "runtime_attachment_summary_denied",
      "live_context_summary_denied",
      "memory_kg_summary_denied",
      "rollback_summary_denied",
      "secret_material_summary_denied",
      "provider_model_summary_denied",
      "external_public_install_restart_active_binary_summary_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_operator_facing_summary_non_persistence",
        status:"allowed_report_only",
        persists_summary:false,
        delivers_summary:false,
        invokes_adapter:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_operator_acknowledgement_non_acceptance",
        status:"allowed_report_only_next_slice",
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
    source_receipt_observability_denial_report_required:true,
    operator_summary_persistence_forbidden:true,
    operator_briefing_persistence_forbidden:true,
    operator_summary_delivery_forbidden:true,
    operator_briefing_delivery_forbidden:true,
    receipt_export_forbidden:true,
    receipt_query_forbidden:true,
    receipt_observability_forbidden:true,
    router_handoff_persistence_forbidden:true,
    readback_evidence_persistence_forbidden:true,
    live_context_attachment_forbidden:true,
    adapter_invocation_forbidden:true,
    provider_model_invocation_forbidden:true,
    auth_secret_read_forbidden:true,
    usage_recording_forbidden:true,
    side_effects:{
      operator_summary_recorded:false,
      operator_summary_persisted:false,
      operator_summary_materialized:false,
      operator_summary_filesystem_written:false,
      operator_summary_delivered:false,
      operator_summary_channel_delivery_performed:false,
      operator_briefing_recorded:false,
      operator_briefing_persisted:false,
      operator_briefing_materialized:false,
      operator_briefing_filesystem_written:false,
      operator_briefing_delivered:false,
      operator_briefing_channel_delivery_performed:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_facing_summary_non_persistence_gate"
  and .operator_facing_summary_non_persistence_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_operator_facing_summary_non_persistence_v1"
  and .runtime_provider_router_operator_facing_summary_non_persistence_ready == true
  and .runtime_provider_router_operator_facing_summary_non_persistence_status == "blocked"
  and .receipt_observability_denial_ready == true
  and .receipt_observability_denial_status == "blocked"
  and .readback_receipt_skeleton_ready == true
  and .readback_receipt_skeleton_status == "blocked"
  and .receipt_observability_fixture_count == 10
  and .blocked_receipt_observability_fixture_count == 10
  and .noop_receipt_observability_fixture_count == 10
  and .allowed_receipt_observability_fixture_count == 0
  and .accepted_receipt_observability_fixture_count == 0
  and .receipt_export_denied_count == 10
  and .receipt_query_denied_count == 10
  and .receipt_observability_denied_count == 10
  and .receipt_export_performed_count == 0
  and .receipt_query_performed_count == 0
  and .receipt_observability_performed_count == 0
  and .operator_facing_summary_surface_count == 12
  and .operator_facing_summary_surface_ready_count == 12
  and .operator_facing_summary_side_effect_free_surface_count == 12
  and .operator_facing_summary_fixture_count == 10
  and .blocked_operator_facing_summary_fixture_count == 10
  and .noop_operator_facing_summary_fixture_count == 10
  and .allowed_operator_facing_summary_fixture_count == 0
  and .accepted_operator_facing_summary_fixture_count == 0
  and .operator_summary_denied_count == 10
  and .operator_briefing_denied_count == 10
  and .operator_summary_performed_count == 0
  and .operator_briefing_performed_count == 0
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
  and (.operator_facing_summary_surfaces | length) == 12
  and (.operator_facing_summary_fixtures | length) == 10
  and (.operator_facing_summary_fixtures | all((.operator_facing_summary_status == "blocked_noop" or .operator_facing_summary_status == "blocked_summary_noop" or .operator_facing_summary_status == "blocked_briefing_noop" or .operator_facing_summary_status == "blocked_delivery_noop") and .operator_summary_recorded == false and .operator_summary_persisted == false and .operator_summary_materialized == false and .operator_summary_filesystem_written == false and .operator_summary_delivered == false and .operator_briefing_recorded == false and .operator_briefing_persisted == false and .operator_briefing_materialized == false and .operator_briefing_filesystem_written == false and .operator_briefing_delivered == false and .telegram_send_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .readback_evidence_recorded == false and .readback_evidence_persisted == false and .router_handoff_recorded == false and .runtime_router_mutated == false and .live_context_attached == false and .adapter_invoked == false and .provider_invoked == false and .model_invoked == false and .auth_secret_read == false and .credential_read == false and .secret_file_read == false and .usage_recorded == false and .memory_store_write_performed == false and .memory_store_mutated == false and .live_kg_write_performed == false and .rollback_executed == false and .summary_noop_confirmed == true))
  and (.denied_by_operator_facing_summary_non_persistence | length) == 27
  and (.allowed_next_actions | any(.action == "review_runtime_provider_router_operator_facing_summary_non_persistence" and .status == "allowed_report_only" and .persists_summary == false and .delivers_summary == false and .invokes_adapter == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_operator_acknowledgement_non_acceptance" and .status == "allowed_report_only_next_slice" and .accepts_acknowledgement == false and .persists_summary == false and .exports_receipt == false and .invokes_adapter == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "run_full_light_preflight" and .status == "allowed_verification_only" and .mutates_runtime == false and .attaches_live_context == false and .invokes_model == false and .writes_kg == false))
  and .source_receipt_observability_denial_report_required == true
  and .operator_summary_persistence_forbidden == true
  and .operator_briefing_persistence_forbidden == true
  and .operator_summary_delivery_forbidden == true
  and .operator_briefing_delivery_forbidden == true
  and .receipt_export_forbidden == true
  and .receipt_query_forbidden == true
  and .receipt_observability_forbidden == true
  and .adapter_invocation_forbidden == true
  and .provider_model_invocation_forbidden == true
  and .auth_secret_read_forbidden == true
  and .usage_recording_forbidden == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router operator-facing summary non-persistence gate passed"
