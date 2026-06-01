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
    echo "missing provider-router negative fixture source text: $label" >&2
    exit 1
  fi
}

RUNTIME_ATTACHMENT_STAGING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-context-attachment-staging-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-context-attachment-staging-gate.sh
)"

RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE="codex-rs/hepta-runtime/src/model_provider_router.rs"

require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub fn record_memory_context_activation_handoff' \
  "runtime memory context activation handoff adapter"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'if !input.operator_confirmed' \
  "operator confirmation denial"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'if !input.cutover_gate_ready' \
  "cutover gate readiness denial"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'if !input.operator_release_approved' \
  "operator release approval denial"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'if input.kill_switch_active' \
  "kill-switch active denial"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'if input.traffic_percent_ppm != 0' \
  "non-shadow traffic denial"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'input.max_context_node_count == 0 || input.max_context_node_count > 128' \
  "context node budget denial"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'router_mutated_by_adapter: false,' \
  "duplicate no router mutation guard"
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

negative_fixtures_json="$(
  jq -n '
    [
      {
        fixture:"missing_operator_confirmation",
        input_mutation:"operator_confirmed=false",
        denied_reason:"explicit_operator_confirmation_required",
        expected_adapter_result:"error",
        shape_declared:true,
        adapter_invoked:false,
        denied:true,
        router_handoff_recorded:false,
        runtime_router_mutated:false,
        live_context_attached:false,
        provider_invoked:false,
        auth_secret_read:false,
        usage_recorded:false
      },
      {
        fixture:"missing_cutover_gate_readiness",
        input_mutation:"cutover_gate_ready=false",
        denied_reason:"ready_cutover_gate_required",
        expected_adapter_result:"error",
        shape_declared:true,
        adapter_invoked:false,
        denied:true,
        router_handoff_recorded:false,
        runtime_router_mutated:false,
        live_context_attached:false,
        provider_invoked:false,
        auth_secret_read:false,
        usage_recorded:false
      },
      {
        fixture:"missing_operator_release_approval",
        input_mutation:"operator_release_approved=false",
        denied_reason:"operator_release_approval_required",
        expected_adapter_result:"error",
        shape_declared:true,
        adapter_invoked:false,
        denied:true,
        router_handoff_recorded:false,
        runtime_router_mutated:false,
        live_context_attached:false,
        provider_invoked:false,
        auth_secret_read:false,
        usage_recorded:false
      },
      {
        fixture:"kill_switch_active",
        input_mutation:"kill_switch_active=true",
        denied_reason:"active_kill_switch_blocks_handoff",
        expected_adapter_result:"error",
        shape_declared:true,
        adapter_invoked:false,
        denied:true,
        router_handoff_recorded:false,
        runtime_router_mutated:false,
        live_context_attached:false,
        provider_invoked:false,
        auth_secret_read:false,
        usage_recorded:false
      },
      {
        fixture:"non_shadow_traffic_requested",
        input_mutation:"traffic_percent_ppm=1000",
        denied_reason:"shadow_only_0ppm_required",
        expected_adapter_result:"error",
        shape_declared:true,
        adapter_invoked:false,
        denied:true,
        router_handoff_recorded:false,
        runtime_router_mutated:false,
        live_context_attached:false,
        provider_invoked:false,
        auth_secret_read:false,
        usage_recorded:false
      },
      {
        fixture:"zero_context_node_budget",
        input_mutation:"max_context_node_count=0",
        denied_reason:"context_node_budget_must_be_1_to_128",
        expected_adapter_result:"error",
        shape_declared:true,
        adapter_invoked:false,
        denied:true,
        router_handoff_recorded:false,
        runtime_router_mutated:false,
        live_context_attached:false,
        provider_invoked:false,
        auth_secret_read:false,
        usage_recorded:false
      },
      {
        fixture:"oversized_context_node_budget",
        input_mutation:"max_context_node_count=129",
        denied_reason:"context_node_budget_must_be_1_to_128",
        expected_adapter_result:"error",
        shape_declared:true,
        adapter_invoked:false,
        denied:true,
        router_handoff_recorded:false,
        runtime_router_mutated:false,
        live_context_attached:false,
        provider_invoked:false,
        auth_secret_read:false,
        usage_recorded:false
      },
      {
        fixture:"side_effect_guard_requested",
        input_mutation:"runtime_no_effect_adapter_guards_missing",
        denied_reason:"runtime_no_effect_adapter_guards_not_accepted",
        expected_adapter_result:"not_invoked",
        shape_declared:true,
        adapter_invoked:false,
        denied:true,
        router_handoff_recorded:false,
        runtime_router_mutated:false,
        live_context_attached:false,
        provider_invoked:false,
        auth_secret_read:false,
        usage_recorded:false
      },
      {
        fixture:"missing_readback_evidence_receipt",
        input_mutation:"runtime_readback_evidence_receipt_missing",
        denied_reason:"runtime_readback_evidence_receipt_not_accepted",
        expected_adapter_result:"not_invoked",
        shape_declared:true,
        adapter_invoked:false,
        denied:true,
        router_handoff_recorded:false,
        runtime_router_mutated:false,
        live_context_attached:false,
        provider_invoked:false,
        auth_secret_read:false,
        usage_recorded:false
      },
      {
        fixture:"missing_idempotency_receipt",
        input_mutation:"idempotency_key_receipt_missing",
        denied_reason:"idempotency_key_receipt_not_accepted",
        expected_adapter_result:"not_invoked",
        shape_declared:true,
        adapter_invoked:false,
        denied:true,
        router_handoff_recorded:false,
        runtime_router_mutated:false,
        live_context_attached:false,
        provider_invoked:false,
        auth_secret_read:false,
        usage_recorded:false
      }
    ]
  '
)"

runtime_attachment_staging_report_sha256="$(sha256_text "$RUNTIME_ATTACHMENT_STAGING_JSON")"
runtime_model_provider_router_source_sha256="$(sha256_file "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE")"
negative_fixtures_sha256="$(sha256_text "$negative_fixtures_json")"
negative_fixture_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-context-attachment-negative-fixture-matrix:$runtime_attachment_staging_report_sha256:$runtime_model_provider_router_source_sha256:$negative_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
negative_fixture_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-context-attachment-negative-fixture-matrix:report-only:no-adapter-invocation:no-router-persistence:no-live-context-attachment:no-model-invocation:no-auth-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "adapter_invoked=false;router_handoff_recorded=false;runtime_router_mutated=false;live_context_attached=false;provider_invoked=false;auth_secret_read=false;usage_recorded=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson staging "$RUNTIME_ATTACHMENT_STAGING_JSON" \
  --argjson fixtures "$negative_fixtures_json" \
  '
    $staging.runtime == "hepta"
    and $staging.status == "ready"
    and $staging.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_context_attachment_staging_gate"
    and $staging.runtime_provider_router_context_attachment_staging_ready == true
    and $staging.runtime_provider_router_context_attachment_staging_status == "blocked"
    and $staging.runtime_attachment_packet_shape_ready == true
    and $staging.runtime_attachment_packet_recorded == false
    and $staging.runtime_attachment_packet_persisted == false
    and $staging.runtime_attachment_packet_accepted == false
    and $staging.router_handoff_recorded == false
    and $staging.runtime_router_mutated == false
    and $staging.memory_context_activation_handoff_persisted == false
    and $staging.readback_evidence_persisted == false
    and $staging.hepta_intelligence_context_attached == false
    and $staging.live_context_attached_to_prompt == false
    and $staging.provider_invoked == false
    and $staging.model_invoked == false
    and $staging.auth_secret_read == false
    and $staging.usage_recorded == false
    and $staging.runtime_attachment_packet_item_count == 12
    and $staging.accepted_runtime_attachment_packet_item_count == 0
    and $staging.missing_runtime_attachment_packet_item_count == 12
    and ($staging.allowed_next_actions | any(.action == "stage_runtime_provider_router_context_attachment_negative_fixture_matrix" and .status == "allowed_report_only_next_slice" and .mutates_runtime == false and .persists_router_handoff == false and .attaches_live_context == false and .invokes_model == false))
    and ($staging.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      .shape_declared == true
      and .adapter_invoked == false
      and .denied == true
      and .router_handoff_recorded == false
      and .runtime_router_mutated == false
      and .live_context_attached == false
      and .provider_invoked == false
      and .auth_secret_read == false
      and .usage_recorded == false
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_context_attachment_negative_fixture_matrix_gate" \
  --arg runtime_attachment_staging_report_sha256 "$runtime_attachment_staging_report_sha256" \
  --arg runtime_model_provider_router_source "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  --arg runtime_model_provider_router_source_sha256 "$runtime_model_provider_router_source_sha256" \
  --arg negative_fixtures_sha256 "$negative_fixtures_sha256" \
  --arg negative_fixture_contract_hash_sha256 "$negative_fixture_contract_hash_sha256" \
  --arg negative_fixture_policy_hash_sha256 "$negative_fixture_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson staging "$RUNTIME_ATTACHMENT_STAGING_JSON" \
  --argjson fixtures "$negative_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    negative_fixture_matrix_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_context_attachment_negative_fixture_matrix_v1",
    negative_fixture_matrix_mode:"runtime_provider_router_context_attachment_negative_fixtures_no_adapter_invocation_no_router_persistence_no_live_attachment",
    source_runtime_attachment_staging_gate:$staging.gate,
    source_runtime_attachment_staging_report_sha256:$runtime_attachment_staging_report_sha256,
    source_runtime_model_provider_router:$runtime_model_provider_router_source,
    source_runtime_model_provider_router_sha256:$runtime_model_provider_router_source_sha256,
    negative_fixtures_sha256:$negative_fixtures_sha256,
    negative_fixture_contract_hash_sha256:$negative_fixture_contract_hash_sha256,
    negative_fixture_policy_hash_sha256:$negative_fixture_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_context_attachment_negative_fixture_matrix_ready:true,
    runtime_provider_router_context_attachment_negative_fixture_matrix_status:"blocked",
    runtime_attachment_staging_ready:$staging.runtime_provider_router_context_attachment_staging_ready,
    runtime_attachment_staging_status:$staging.runtime_provider_router_context_attachment_staging_status,
    runtime_attachment_packet_shape_ready:$staging.runtime_attachment_packet_shape_ready,
    runtime_attachment_packet_recorded:$staging.runtime_attachment_packet_recorded,
    runtime_attachment_packet_persisted:$staging.runtime_attachment_packet_persisted,
    runtime_attachment_packet_accepted:$staging.runtime_attachment_packet_accepted,
    runtime_attachment_packet_item_count:$staging.runtime_attachment_packet_item_count,
    missing_runtime_attachment_packet_item_count:$staging.missing_runtime_attachment_packet_item_count,
    accepted_runtime_attachment_packet_item_count:$staging.accepted_runtime_attachment_packet_item_count,
    provider_router_id:$staging.provider_router_id,
    feature_flag_id:$staging.feature_flag_id,
    activation_contract:$staging.activation_contract,
    selected_canary_stage_id:$staging.selected_canary_stage_id,
    shadow_traffic_percent_ppm:$staging.shadow_traffic_percent_ppm,
    max_context_node_count_cap:$staging.max_context_node_count_cap,
    negative_fixture_count:($fixtures | length),
    declared_negative_fixture_count:($fixtures | map(select(.shape_declared == true)) | length),
    denied_negative_fixture_count:($fixtures | map(select(.denied == true)) | length),
    adapter_invoked_negative_fixture_count:($fixtures | map(select(.adapter_invoked == true)) | length),
    router_handoff_recorded_negative_fixture_count:($fixtures | map(select(.router_handoff_recorded == true)) | length),
    runtime_router_mutated_negative_fixture_count:($fixtures | map(select(.runtime_router_mutated == true)) | length),
    live_context_attached_negative_fixture_count:($fixtures | map(select(.live_context_attached == true)) | length),
    provider_invoked_negative_fixture_count:($fixtures | map(select(.provider_invoked == true)) | length),
    auth_secret_read_negative_fixture_count:($fixtures | map(select(.auth_secret_read == true)) | length),
    usage_recorded_negative_fixture_count:($fixtures | map(select(.usage_recorded == true)) | length),
    negative_fixtures:$fixtures,
    source_denial_guards:[
      "operator_confirmation_denial",
      "cutover_gate_readiness_denial",
      "operator_release_approval_denial",
      "kill_switch_active_denial",
      "non_shadow_traffic_denial",
      "context_node_budget_denial",
      "duplicate_idempotency_no_router_mutation_guard",
      "feature_flag_mutation_disabled",
      "live_prompt_attachment_disabled",
      "provider_invocation_disabled",
      "auth_secret_read_disabled",
      "usage_recording_disabled"
    ],
    source_denial_guard_count:12,
    denied_by_negative_fixture_matrix:[
      "operator_confirmation_missing",
      "cutover_gate_readiness_missing",
      "operator_release_approval_missing",
      "kill_switch_active",
      "non_shadow_traffic_requested",
      "zero_context_node_budget",
      "oversized_context_node_budget",
      "runtime_no_effect_adapter_guards_missing",
      "runtime_readback_evidence_receipt_missing",
      "idempotency_key_receipt_missing",
      "adapter_invocation_denied",
      "router_handoff_persistence_denied",
      "runtime_router_mutation_denied",
      "live_context_attachment_denied",
      "provider_model_invocation_denied",
      "auth_secret_read_denied",
      "usage_recording_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_context_attachment_negative_fixture_matrix",
        status:"allowed_report_only",
        invokes_adapter:false,
        persists_router_handoff:false,
        attaches_live_context:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_readback_receipt_skeleton",
        status:"allowed_report_only_next_slice",
        invokes_adapter:false,
        persists_router_handoff:false,
        persists_evidence:false,
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
    operator_approval_required_before_runtime_attachment:true,
    operator_release_approval_required:true,
    cutover_gate_readiness_required:true,
    kill_switch_absence_required:true,
    shadow_only_traffic_required:true,
    context_node_budget_required:true,
    runtime_no_effect_adapter_guards_required:true,
    runtime_readback_evidence_required:true,
    idempotency_key_required:true,
    adapter_invocation_forbidden:true,
    router_handoff_persistence_forbidden:true,
    runtime_router_mutation_forbidden:true,
    live_context_attachment_forbidden:true,
    provider_model_invocation_forbidden:true,
    auth_secret_read_forbidden:true,
    usage_recording_forbidden:true,
    full_live_enablement_performed:false,
    adapter_invoked:false,
    runtime_router_mutated:false,
    router_handoff_recorded:false,
    memory_context_activation_handoff_persisted:false,
    readback_evidence_persisted:false,
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
      readback_evidence_persisted:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_context_attachment_negative_fixture_matrix_gate"
  and .negative_fixture_matrix_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_context_attachment_negative_fixture_matrix_v1"
  and .runtime_provider_router_context_attachment_negative_fixture_matrix_ready == true
  and .runtime_provider_router_context_attachment_negative_fixture_matrix_status == "blocked"
  and .runtime_attachment_staging_ready == true
  and .runtime_attachment_staging_status == "blocked"
  and .runtime_attachment_packet_shape_ready == true
  and .runtime_attachment_packet_recorded == false
  and .runtime_attachment_packet_persisted == false
  and .runtime_attachment_packet_accepted == false
  and .runtime_attachment_packet_item_count == 12
  and .missing_runtime_attachment_packet_item_count == 12
  and .accepted_runtime_attachment_packet_item_count == 0
  and .provider_router_id == "hepta-native-model-provider-router"
  and .feature_flag_id == "HEPTA_MEMORY_CONTEXT_LIVE_TURN"
  and .activation_contract == "hepta-intelligence-memory-provider-router-activation-gate-v1"
  and .selected_canary_stage_id == "shadow-canary-0ppm"
  and .shadow_traffic_percent_ppm == 0
  and .max_context_node_count_cap == 128
  and .negative_fixture_count == 10
  and .declared_negative_fixture_count == 10
  and .denied_negative_fixture_count == 10
  and .adapter_invoked_negative_fixture_count == 0
  and .router_handoff_recorded_negative_fixture_count == 0
  and .runtime_router_mutated_negative_fixture_count == 0
  and .live_context_attached_negative_fixture_count == 0
  and .provider_invoked_negative_fixture_count == 0
  and .auth_secret_read_negative_fixture_count == 0
  and .usage_recorded_negative_fixture_count == 0
  and (.negative_fixtures | length) == 10
  and (.negative_fixtures | all(.shape_declared == true and .adapter_invoked == false and .denied == true and .router_handoff_recorded == false and .runtime_router_mutated == false and .live_context_attached == false and .provider_invoked == false and .auth_secret_read == false and .usage_recorded == false))
  and .source_denial_guard_count == 12
  and (.source_denial_guards | length) == 12
  and (.denied_by_negative_fixture_matrix | length) == 17
  and (.allowed_next_actions | any(.action == "review_runtime_provider_router_context_attachment_negative_fixture_matrix" and .status == "allowed_report_only" and .invokes_adapter == false and .persists_router_handoff == false and .attaches_live_context == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_readback_receipt_skeleton" and .status == "allowed_report_only_next_slice" and .invokes_adapter == false and .persists_router_handoff == false and .persists_evidence == false and .attaches_live_context == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "run_full_light_preflight" and .status == "allowed_verification_only" and .mutates_runtime == false and .attaches_live_context == false and .invokes_model == false and .writes_kg == false))
  and .operator_approval_required_before_runtime_attachment == true
  and .operator_release_approval_required == true
  and .cutover_gate_readiness_required == true
  and .kill_switch_absence_required == true
  and .shadow_only_traffic_required == true
  and .context_node_budget_required == true
  and .runtime_no_effect_adapter_guards_required == true
  and .runtime_readback_evidence_required == true
  and .idempotency_key_required == true
  and .adapter_invocation_forbidden == true
  and .router_handoff_persistence_forbidden == true
  and .runtime_router_mutation_forbidden == true
  and .live_context_attachment_forbidden == true
  and .provider_model_invocation_forbidden == true
  and .auth_secret_read_forbidden == true
  and .usage_recording_forbidden == true
  and .full_live_enablement_performed == false
  and .adapter_invoked == false
  and .runtime_router_mutated == false
  and .router_handoff_recorded == false
  and .memory_context_activation_handoff_persisted == false
  and .readback_evidence_persisted == false
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
  and .external_send_performed == false
  and .channel_send_performed == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router context attachment negative fixture matrix gate passed"
