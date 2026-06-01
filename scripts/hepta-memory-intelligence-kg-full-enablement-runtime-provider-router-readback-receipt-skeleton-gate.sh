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

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing provider-router readback receipt source text: $label" >&2
    exit 1
  fi
}

NEGATIVE_FIXTURE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-context-attachment-negative-fixture-matrix-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-context-attachment-negative-fixture-matrix-gate.sh
)"

RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE="codex-rs/hepta-runtime/src/model_provider_router.rs"

require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub struct ModelProviderMemoryContextActivationRecord' \
  "memory context activation record"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub readback_evidence_id: String,' \
  "readback evidence id field"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub duplicate_idempotency_key: bool,' \
  "idempotency result field"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub fn record_memory_context_activation_handoff' \
  "runtime memory context activation handoff adapter"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'evidence_ledger.append(' \
  "readback evidence ledger append"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  '"model_provider_memory_context_activation_handoff"' \
  "memory context activation handoff evidence subject"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'duplicate_idempotency_key: true,' \
  "duplicate idempotency key branch"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'router_mutated_by_adapter: false,' \
  "duplicate no router mutation guard"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'router_handoff_recorded: true,' \
  "eventual handoff record field"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'feature_flag_mutated_by_adapter: false,' \
  "feature flag mutation disabled"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'context_attached_to_live_prompt: false,' \
  "live prompt attachment disabled"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'provider_invoked_by_adapter: false,' \
  "provider invocation disabled"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'auth_secret_read_by_adapter: false,' \
  "auth secret read disabled"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'usage_recorded_by_adapter: false,' \
  "usage recording disabled"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'persisted: evidence.persisted,' \
  "eventual readback persistence flag"

receipt_skeleton_items_json="$(
  jq -n '
    [
      {
        item:"provider_router_identity",
        evidence_class:"router_identity",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      },
      {
        item:"memory_context_feature_flag",
        evidence_class:"feature_flag_boundary",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      },
      {
        item:"provider_router_activation_contract_binding",
        evidence_class:"activation_contract",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      },
      {
        item:"shadow_canary_stage_0ppm",
        evidence_class:"traffic_shadow_stage",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      },
      {
        item:"max_context_node_budget",
        evidence_class:"context_budget",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      },
      {
        item:"fallback_no_memory_provider_turn_hash",
        evidence_class:"rollback_fallback",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      },
      {
        item:"cutover_gate_readiness_receipt",
        evidence_class:"cutover_readiness",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      },
      {
        item:"operator_release_approval_receipt",
        evidence_class:"operator_authority",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      },
      {
        item:"kill_switch_absence_receipt",
        evidence_class:"rollback_kill_switch",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      },
      {
        item:"idempotency_key_receipt",
        evidence_class:"idempotency",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      },
      {
        item:"runtime_readback_evidence_receipt",
        evidence_class:"readback_evidence",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      },
      {
        item:"runtime_no_effect_adapter_guards",
        evidence_class:"runtime_side_effect_boundary",
        receipt_shape_declared:true,
        receipt_required:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        blocks_runtime_attachment:true,
        blocks_live_context_attachment:true,
        blocks_model_invocation:true
      }
    ]
  '
)"

negative_fixture_report_sha256="$(sha256_text "$NEGATIVE_FIXTURE_JSON")"
runtime_model_provider_router_source_sha256="$(sha256_file "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE")"
receipt_skeleton_items_sha256="$(sha256_text "$receipt_skeleton_items_json")"
receipt_skeleton_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-readback-receipt-skeleton:$negative_fixture_report_sha256:$runtime_model_provider_router_source_sha256:$receipt_skeleton_items_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
receipt_skeleton_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-readback-receipt-skeleton:report-only:no-adapter-invocation:no-router-handoff-persistence:no-receipt-persistence:no-live-context-attachment:no-model-invocation:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "adapter_invoked=false;router_handoff_recorded=false;runtime_router_mutated=false;readback_evidence_persisted=false;receipt_persisted=false;live_context_attached=false;model_invoked=false;auth_secret_read=false;usage_recorded=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson negative "$NEGATIVE_FIXTURE_JSON" \
  --argjson receipts "$receipt_skeleton_items_json" \
  '
    $negative.runtime == "hepta"
    and $negative.status == "ready"
    and $negative.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_context_attachment_negative_fixture_matrix_gate"
    and $negative.runtime_provider_router_context_attachment_negative_fixture_matrix_ready == true
    and $negative.runtime_provider_router_context_attachment_negative_fixture_matrix_status == "blocked"
    and $negative.runtime_attachment_staging_ready == true
    and $negative.runtime_attachment_staging_status == "blocked"
    and $negative.runtime_attachment_packet_shape_ready == true
    and $negative.runtime_attachment_packet_recorded == false
    and $negative.runtime_attachment_packet_persisted == false
    and $negative.runtime_attachment_packet_accepted == false
    and $negative.negative_fixture_count == 10
    and $negative.denied_negative_fixture_count == 10
    and $negative.adapter_invoked_negative_fixture_count == 0
    and $negative.router_handoff_recorded_negative_fixture_count == 0
    and $negative.runtime_router_mutated_negative_fixture_count == 0
    and $negative.live_context_attached_negative_fixture_count == 0
    and $negative.provider_invoked_negative_fixture_count == 0
    and $negative.auth_secret_read_negative_fixture_count == 0
    and $negative.usage_recorded_negative_fixture_count == 0
    and $negative.runtime_readback_evidence_required == true
    and $negative.idempotency_key_required == true
    and $negative.runtime_no_effect_adapter_guards_required == true
    and ($negative.allowed_next_actions | any(.action == "stage_runtime_provider_router_readback_receipt_skeleton" and .status == "allowed_report_only_next_slice" and .invokes_adapter == false and .persists_router_handoff == false and .persists_evidence == false and .attaches_live_context == false and .invokes_model == false))
    and ($negative.side_effects | to_entries | all(.value == false))
    and ($receipts | length) == 12
    and ($receipts | all(
      .receipt_shape_declared == true
      and .receipt_required == true
      and .receipt_recorded == false
      and .receipt_persisted == false
      and .receipt_accepted == false
      and .blocks_runtime_attachment == true
      and .blocks_live_context_attachment == true
      and .blocks_model_invocation == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_readback_receipt_skeleton_gate" \
  --arg negative_fixture_report_sha256 "$negative_fixture_report_sha256" \
  --arg runtime_model_provider_router_source "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  --arg runtime_model_provider_router_source_sha256 "$runtime_model_provider_router_source_sha256" \
  --arg receipt_skeleton_items_sha256 "$receipt_skeleton_items_sha256" \
  --arg receipt_skeleton_contract_hash_sha256 "$receipt_skeleton_contract_hash_sha256" \
  --arg receipt_skeleton_policy_hash_sha256 "$receipt_skeleton_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson negative "$NEGATIVE_FIXTURE_JSON" \
  --argjson receipts "$receipt_skeleton_items_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    readback_receipt_skeleton_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_readback_receipt_skeleton_v1",
    readback_receipt_skeleton_mode:"runtime_provider_router_readback_receipt_skeleton_no_adapter_invocation_no_router_persistence_no_receipt_persistence_no_live_attachment",
    source_negative_fixture_matrix_gate:$negative.gate,
    source_negative_fixture_matrix_report_sha256:$negative_fixture_report_sha256,
    source_runtime_model_provider_router:$runtime_model_provider_router_source,
    source_runtime_model_provider_router_sha256:$runtime_model_provider_router_source_sha256,
    receipt_skeleton_items_sha256:$receipt_skeleton_items_sha256,
    receipt_skeleton_contract_hash_sha256:$receipt_skeleton_contract_hash_sha256,
    receipt_skeleton_policy_hash_sha256:$receipt_skeleton_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_readback_receipt_skeleton_ready:true,
    runtime_provider_router_readback_receipt_skeleton_status:"blocked",
    negative_fixture_matrix_ready:$negative.runtime_provider_router_context_attachment_negative_fixture_matrix_ready,
    negative_fixture_matrix_status:$negative.runtime_provider_router_context_attachment_negative_fixture_matrix_status,
    runtime_attachment_staging_ready:$negative.runtime_attachment_staging_ready,
    runtime_attachment_staging_status:$negative.runtime_attachment_staging_status,
    runtime_attachment_packet_shape_ready:$negative.runtime_attachment_packet_shape_ready,
    runtime_attachment_packet_recorded:$negative.runtime_attachment_packet_recorded,
    runtime_attachment_packet_persisted:$negative.runtime_attachment_packet_persisted,
    runtime_attachment_packet_accepted:$negative.runtime_attachment_packet_accepted,
    runtime_attachment_packet_item_count:$negative.runtime_attachment_packet_item_count,
    missing_runtime_attachment_packet_item_count:$negative.missing_runtime_attachment_packet_item_count,
    negative_fixture_count:$negative.negative_fixture_count,
    denied_negative_fixture_count:$negative.denied_negative_fixture_count,
    adapter_invoked_negative_fixture_count:$negative.adapter_invoked_negative_fixture_count,
    router_handoff_recorded_negative_fixture_count:$negative.router_handoff_recorded_negative_fixture_count,
    runtime_router_mutated_negative_fixture_count:$negative.runtime_router_mutated_negative_fixture_count,
    live_context_attached_negative_fixture_count:$negative.live_context_attached_negative_fixture_count,
    provider_invoked_negative_fixture_count:$negative.provider_invoked_negative_fixture_count,
    auth_secret_read_negative_fixture_count:$negative.auth_secret_read_negative_fixture_count,
    usage_recorded_negative_fixture_count:$negative.usage_recorded_negative_fixture_count,
    provider_router_id:$negative.provider_router_id,
    feature_flag_id:$negative.feature_flag_id,
    activation_contract:$negative.activation_contract,
    selected_canary_stage_id:$negative.selected_canary_stage_id,
    shadow_traffic_percent_ppm:$negative.shadow_traffic_percent_ppm,
    max_context_node_count_cap:$negative.max_context_node_count_cap,
    readback_receipt_skeleton_item_count:($receipts | length),
    declared_readback_receipt_skeleton_item_count:($receipts | map(select(.receipt_shape_declared == true)) | length),
    required_readback_receipt_skeleton_item_count:($receipts | map(select(.receipt_required == true)) | length),
    recorded_readback_receipt_skeleton_item_count:($receipts | map(select(.receipt_recorded == true)) | length),
    persisted_readback_receipt_skeleton_item_count:($receipts | map(select(.receipt_persisted == true)) | length),
    accepted_readback_receipt_skeleton_item_count:($receipts | map(select(.receipt_accepted == true)) | length),
    runtime_attachment_blocking_receipt_count:($receipts | map(select(.blocks_runtime_attachment == true)) | length),
    live_context_attachment_blocking_receipt_count:($receipts | map(select(.blocks_live_context_attachment == true)) | length),
    model_invocation_blocking_receipt_count:($receipts | map(select(.blocks_model_invocation == true)) | length),
    readback_receipt_skeleton_items:$receipts,
    receipt_evidence_classes:($receipts | map(.evidence_class) | unique),
    receipt_evidence_class_count:($receipts | map(.evidence_class) | unique | length),
    source_runtime_readback_contracts:[
      "readback_evidence_id_field",
      "readback_evidence_ledger_append",
      "memory_context_activation_handoff_evidence_subject",
      "duplicate_idempotency_key_branch",
      "duplicate_no_router_mutation_guard",
      "eventual_handoff_record_field",
      "feature_flag_mutation_disabled",
      "live_prompt_attachment_disabled",
      "provider_invocation_disabled",
      "auth_secret_read_disabled",
      "usage_recording_disabled",
      "eventual_readback_persistence_flag"
    ],
    source_runtime_readback_contract_count:12,
    denied_by_readback_receipt_skeleton:[
      "provider_router_identity_receipt_not_accepted",
      "memory_context_feature_flag_receipt_not_accepted",
      "activation_contract_binding_receipt_not_accepted",
      "shadow_canary_stage_receipt_not_accepted",
      "max_context_node_budget_receipt_not_accepted",
      "fallback_no_memory_provider_turn_hash_receipt_not_accepted",
      "cutover_gate_readiness_receipt_not_accepted",
      "operator_release_approval_receipt_not_accepted",
      "kill_switch_absence_receipt_not_accepted",
      "idempotency_key_receipt_not_accepted",
      "runtime_readback_evidence_receipt_not_accepted",
      "runtime_no_effect_adapter_guards_receipt_not_accepted",
      "adapter_invocation_denied",
      "router_handoff_persistence_denied",
      "readback_evidence_persistence_denied",
      "live_context_attachment_denied",
      "provider_model_invocation_denied",
      "auth_secret_read_denied",
      "usage_recording_denied",
      "observability_export_query_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_readback_receipt_skeleton",
        status:"allowed_report_only",
        invokes_adapter:false,
        persists_router_handoff:false,
        persists_readback_evidence:false,
        attaches_live_context:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_receipt_observability_denial",
        status:"allowed_report_only_next_slice",
        invokes_adapter:false,
        records_observability:false,
        exports_receipt:false,
        registers_query:false,
        persists_evidence:false,
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
    provider_router_identity_receipt_required:true,
    memory_context_feature_flag_receipt_required:true,
    activation_contract_binding_receipt_required:true,
    shadow_canary_stage_receipt_required:true,
    max_context_node_budget_receipt_required:true,
    rollback_fallback_receipt_required:true,
    cutover_gate_readiness_receipt_required:true,
    operator_release_approval_receipt_required:true,
    kill_switch_absence_receipt_required:true,
    idempotency_key_receipt_required:true,
    runtime_readback_evidence_receipt_required:true,
    runtime_no_effect_adapter_guards_receipt_required:true,
    observability_export_query_denial_required:true,
    adapter_invocation_forbidden:true,
    router_handoff_persistence_forbidden:true,
    readback_evidence_persistence_forbidden:true,
    live_context_attachment_forbidden:true,
    provider_model_invocation_forbidden:true,
    auth_secret_read_forbidden:true,
    usage_recording_forbidden:true,
    full_live_enablement_performed:false,
    adapter_invoked:false,
    runtime_router_mutated:false,
    router_handoff_recorded:false,
    memory_context_activation_handoff_persisted:false,
    readback_evidence_recorded:false,
    readback_evidence_persisted:false,
    receipt_recorded:false,
    receipt_persisted:false,
    receipt_accepted:false,
    hepta_intelligence_context_attached:false,
    live_context_attached_to_prompt:false,
    context_injection_performed:false,
    prompt_preview_rendered:false,
    prompt_payload_materialized:false,
    provider_invoked:false,
    model_invoked:false,
    auth_secret_read:false,
    credential_read:false,
    secret_file_read:false,
    usage_recorded:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    external_adapter_client_constructed:false,
    external_kg_adapter_read_performed:false,
    network_call_performed:false,
    external_db_write_performed:false,
    live_kg_write_performed:false,
    rollback_executed:false,
    observability_metric_emitted:false,
    observability_log_recorded:false,
    observability_trace_recorded:false,
    observability_dashboard_materialized:false,
    receipt_exported:false,
    receipt_query_registered:false,
    external_send_performed:false,
    channel_send_performed:false,
    public_release_claimed:false,
    public_ga_claimed:false,
    service_restart_performed:false,
    active_binary_mutated:false,
    side_effects:{
      full_live_enablement_performed:false,
      adapter_invoked:false,
      runtime_router_mutated:false,
      router_handoff_recorded:false,
      memory_context_activation_handoff_persisted:false,
      readback_evidence_recorded:false,
      readback_evidence_persisted:false,
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      hepta_intelligence_context_attached:false,
      live_context_attached_to_prompt:false,
      context_injection_performed:false,
      prompt_preview_rendered:false,
      prompt_payload_materialized:false,
      provider_invoked:false,
      model_invoked:false,
      auth_secret_read:false,
      credential_read:false,
      secret_file_read:false,
      usage_recorded:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      external_adapter_client_constructed:false,
      external_kg_adapter_read_performed:false,
      network_call_performed:false,
      external_db_write_performed:false,
      live_kg_write_performed:false,
      rollback_executed:false,
      observability_metric_emitted:false,
      observability_log_recorded:false,
      observability_trace_recorded:false,
      observability_dashboard_materialized:false,
      receipt_exported:false,
      receipt_query_registered:false,
      filesystem_written:false,
      external_send_performed:false,
      channel_send_performed:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_readback_receipt_skeleton_gate"
  and .readback_receipt_skeleton_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_readback_receipt_skeleton_v1"
  and .runtime_provider_router_readback_receipt_skeleton_ready == true
  and .runtime_provider_router_readback_receipt_skeleton_status == "blocked"
  and .negative_fixture_matrix_ready == true
  and .negative_fixture_matrix_status == "blocked"
  and .runtime_attachment_staging_ready == true
  and .runtime_attachment_staging_status == "blocked"
  and .runtime_attachment_packet_shape_ready == true
  and .runtime_attachment_packet_recorded == false
  and .runtime_attachment_packet_persisted == false
  and .runtime_attachment_packet_accepted == false
  and .runtime_attachment_packet_item_count == 12
  and .missing_runtime_attachment_packet_item_count == 12
  and .negative_fixture_count == 10
  and .denied_negative_fixture_count == 10
  and .adapter_invoked_negative_fixture_count == 0
  and .router_handoff_recorded_negative_fixture_count == 0
  and .runtime_router_mutated_negative_fixture_count == 0
  and .live_context_attached_negative_fixture_count == 0
  and .provider_invoked_negative_fixture_count == 0
  and .auth_secret_read_negative_fixture_count == 0
  and .usage_recorded_negative_fixture_count == 0
  and .provider_router_id == "hepta-native-model-provider-router"
  and .feature_flag_id == "HEPTA_MEMORY_CONTEXT_LIVE_TURN"
  and .activation_contract == "hepta-intelligence-memory-provider-router-activation-gate-v1"
  and .selected_canary_stage_id == "shadow-canary-0ppm"
  and .shadow_traffic_percent_ppm == 0
  and .max_context_node_count_cap == 128
  and .readback_receipt_skeleton_item_count == 12
  and .declared_readback_receipt_skeleton_item_count == 12
  and .required_readback_receipt_skeleton_item_count == 12
  and .recorded_readback_receipt_skeleton_item_count == 0
  and .persisted_readback_receipt_skeleton_item_count == 0
  and .accepted_readback_receipt_skeleton_item_count == 0
  and .runtime_attachment_blocking_receipt_count == 12
  and .live_context_attachment_blocking_receipt_count == 12
  and .model_invocation_blocking_receipt_count == 12
  and (.readback_receipt_skeleton_items | length) == 12
  and (.readback_receipt_skeleton_items | all(.receipt_shape_declared == true and .receipt_required == true and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .blocks_runtime_attachment == true and .blocks_live_context_attachment == true and .blocks_model_invocation == true))
  and .source_runtime_readback_contract_count == 12
  and (.source_runtime_readback_contracts | length) == 12
  and (.denied_by_readback_receipt_skeleton | length) == 20
  and (.allowed_next_actions | any(.action == "review_runtime_provider_router_readback_receipt_skeleton" and .status == "allowed_report_only" and .invokes_adapter == false and .persists_router_handoff == false and .persists_readback_evidence == false and .attaches_live_context == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_receipt_observability_denial" and .status == "allowed_report_only_next_slice" and .invokes_adapter == false and .records_observability == false and .exports_receipt == false and .registers_query == false and .persists_evidence == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "run_full_light_preflight" and .status == "allowed_verification_only" and .mutates_runtime == false and .attaches_live_context == false and .invokes_model == false and .writes_kg == false))
  and .provider_router_identity_receipt_required == true
  and .memory_context_feature_flag_receipt_required == true
  and .activation_contract_binding_receipt_required == true
  and .shadow_canary_stage_receipt_required == true
  and .max_context_node_budget_receipt_required == true
  and .rollback_fallback_receipt_required == true
  and .cutover_gate_readiness_receipt_required == true
  and .operator_release_approval_receipt_required == true
  and .kill_switch_absence_receipt_required == true
  and .idempotency_key_receipt_required == true
  and .runtime_readback_evidence_receipt_required == true
  and .runtime_no_effect_adapter_guards_receipt_required == true
  and .observability_export_query_denial_required == true
  and .adapter_invocation_forbidden == true
  and .router_handoff_persistence_forbidden == true
  and .readback_evidence_persistence_forbidden == true
  and .live_context_attachment_forbidden == true
  and .provider_model_invocation_forbidden == true
  and .auth_secret_read_forbidden == true
  and .usage_recording_forbidden == true
  and .full_live_enablement_performed == false
  and .adapter_invoked == false
  and .runtime_router_mutated == false
  and .router_handoff_recorded == false
  and .memory_context_activation_handoff_persisted == false
  and .readback_evidence_recorded == false
  and .readback_evidence_persisted == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .hepta_intelligence_context_attached == false
  and .live_context_attached_to_prompt == false
  and .context_injection_performed == false
  and .prompt_preview_rendered == false
  and .prompt_payload_materialized == false
  and .provider_invoked == false
  and .model_invoked == false
  and .auth_secret_read == false
  and .credential_read == false
  and .secret_file_read == false
  and .usage_recorded == false
  and .memory_store_mutated == false
  and .external_adapter_client_constructed == false
  and .external_kg_adapter_read_performed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .rollback_executed == false
  and .observability_metric_emitted == false
  and .observability_log_recorded == false
  and .observability_trace_recorded == false
  and .observability_dashboard_materialized == false
  and .receipt_exported == false
  and .receipt_query_registered == false
  and .external_send_performed == false
  and .channel_send_performed == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router readback receipt skeleton gate passed"
