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
    echo "missing provider-router context attachment staging source text: $label" >&2
    exit 1
  fi
}

BOUNDED_ACTIVATION_PACKET_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-bounded-prompt-preview-context-handoff-activation-packet-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-bounded-prompt-preview-context-handoff-activation-packet-gate.sh
)"

ACTIVATION_READINESS_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-activation-readiness-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-activation-readiness-gate.sh
)"

PROVIDER_ROUTER_ACTIVATION_SOURCE="codex-rs/hepta-intelligence/src/memory_provider_router_activation_gate.rs"
TURN_DISPATCH_SOURCE="codex-rs/hepta-intelligence/src/memory_turn_dispatch_gate.rs"
LIVE_TURN_PREFLIGHT_SOURCE="codex-rs/hepta-intelligence/src/memory_live_turn_preflight.rs"
RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE="codex-rs/hepta-runtime/src/model_provider_router.rs"

require_source_text "$PROVIDER_ROUTER_ACTIVATION_SOURCE" \
  'pub const MEMORY_PROVIDER_ROUTER_ACTIVATION_GATE_V1_CONTRACT: &str =' \
  "provider-router activation contract constant"
require_source_text "$PROVIDER_ROUTER_ACTIVATION_SOURCE" \
  'provider_router_id: "hepta-native-model-provider-router",' \
  "provider-router id binding"
require_source_text "$PROVIDER_ROUTER_ACTIVATION_SOURCE" \
  'approved_router_handoff.traffic_percent_ppm == 0' \
  "shadow-only traffic guard"
require_source_text "$PROVIDER_ROUTER_ACTIVATION_SOURCE" \
  '!approved_router_handoff.context_attached_to_live_prompt' \
  "no live context attachment guard"
require_source_text "$PROVIDER_ROUTER_ACTIVATION_SOURCE" \
  '!approved_router_handoff.provider_invoked_by_gate' \
  "no provider invocation guard"
require_source_text "$TURN_DISPATCH_SOURCE" \
  'pub const MEMORY_TURN_DISPATCH_GATE_V1_CONTRACT: &str =' \
  "turn dispatch contract constant"
require_source_text "$TURN_DISPATCH_SOURCE" \
  'context_attachment_mode: "approved_dry_run_plan",' \
  "dry-run context attachment mode"
require_source_text "$TURN_DISPATCH_SOURCE" \
  'dispatch_action == "stage_memory_context_for_dispatch"' \
  "stage memory context dispatch action"
require_source_text "$TURN_DISPATCH_SOURCE" \
  '!approved_dispatch_decision.context_injection_performed' \
  "no dispatch context injection guard"
require_source_text "$LIVE_TURN_PREFLIGHT_SOURCE" \
  'pub const MEMORY_LIVE_TURN_PREFLIGHT_V1_CONTRACT: &str =' \
  "live-turn preflight contract constant"
require_source_text "$LIVE_TURN_PREFLIGHT_SOURCE" \
  'injection_allowed: false,' \
  "live-turn preflight injection denied"
require_source_text "$LIVE_TURN_PREFLIGHT_SOURCE" \
  'visible_to_operator: true,' \
  "operator-visible context preview"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub struct ModelProviderMemoryContextActivationInput' \
  "runtime memory context activation input"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub fn record_memory_context_activation_handoff' \
  "runtime memory context activation handoff adapter"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'if input.traffic_percent_ppm != 0' \
  "runtime shadow-only traffic denial"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'context_attached_to_live_prompt: false,' \
  "runtime no live prompt attachment"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'provider_invoked_by_adapter: false,' \
  "runtime no provider invocation"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'auth_secret_read_by_adapter: false,' \
  "runtime no auth secret read"

source_contracts_json="$(
  jq -n \
    --arg provider_router_activation_source "$PROVIDER_ROUTER_ACTIVATION_SOURCE" \
    --arg turn_dispatch_source "$TURN_DISPATCH_SOURCE" \
    --arg live_turn_preflight_source "$LIVE_TURN_PREFLIGHT_SOURCE" \
    --arg runtime_model_provider_router_source "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
    --arg provider_router_activation_sha256 "$(sha256_file "$PROVIDER_ROUTER_ACTIVATION_SOURCE")" \
    --arg turn_dispatch_sha256 "$(sha256_file "$TURN_DISPATCH_SOURCE")" \
    --arg live_turn_preflight_sha256 "$(sha256_file "$LIVE_TURN_PREFLIGHT_SOURCE")" \
    --arg runtime_model_provider_router_sha256 "$(sha256_file "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE")" \
    '[
      {
        source:$provider_router_activation_source,
        source_sha256:$provider_router_activation_sha256,
        contract:"hepta-intelligence-memory-provider-router-activation-gate-v1",
        evidence:"provider_router_activation_handoff_shadow_only",
        compile_checked_by_preflight_cargo_check:true,
        source_pattern_present:true,
        permits_live_context_attachment:false,
        permits_provider_invocation:false
      },
      {
        source:$turn_dispatch_source,
        source_sha256:$turn_dispatch_sha256,
        contract:"hepta-intelligence-memory-turn-dispatch-gate-v1",
        evidence:"approved_dry_run_plan_turn_dispatch",
        compile_checked_by_preflight_cargo_check:true,
        source_pattern_present:true,
        permits_live_context_attachment:false,
        permits_context_injection:false,
        permits_provider_invocation:false
      },
      {
        source:$live_turn_preflight_source,
        source_sha256:$live_turn_preflight_sha256,
        contract:"hepta-intelligence-memory-live-turn-preflight-v1",
        evidence:"operator_visible_preview_injection_denied",
        compile_checked_by_preflight_cargo_check:true,
        source_pattern_present:true,
        permits_context_injection:false,
        permits_model_invocation:false
      },
      {
        source:$runtime_model_provider_router_source,
        source_sha256:$runtime_model_provider_router_sha256,
        contract:"hepta-runtime-model-provider-memory-context-activation-adapter-v0",
        evidence:"runtime_model_provider_router_handoff_adapter_shadow_only",
        compile_checked_by_preflight_cargo_check:true,
        source_pattern_present:true,
        permits_router_mutation_by_this_gate:false,
        permits_live_context_attachment:false,
        permits_provider_invocation:false,
        permits_auth_secret_read:false,
        permits_usage_recording:false
      }
    ]'
)"

runtime_attachment_packet_items_json="$(
  jq -n '
    [
      {
        item:"provider_router_identity",
        evidence_class:"router_identity",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      },
      {
        item:"memory_context_feature_flag",
        evidence_class:"feature_flag_boundary",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      },
      {
        item:"provider_router_activation_contract_binding",
        evidence_class:"activation_contract",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      },
      {
        item:"shadow_canary_stage_0ppm",
        evidence_class:"traffic_shadow_stage",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      },
      {
        item:"max_context_node_budget",
        evidence_class:"context_budget",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      },
      {
        item:"fallback_no_memory_provider_turn_hash",
        evidence_class:"rollback_fallback",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      },
      {
        item:"cutover_gate_readiness_receipt",
        evidence_class:"cutover_readiness",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      },
      {
        item:"operator_release_approval_receipt",
        evidence_class:"operator_authority",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      },
      {
        item:"kill_switch_absence_receipt",
        evidence_class:"rollback_kill_switch",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      },
      {
        item:"idempotency_key_receipt",
        evidence_class:"idempotency",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      },
      {
        item:"runtime_readback_evidence_receipt",
        evidence_class:"readback_evidence",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      },
      {
        item:"runtime_no_effect_adapter_guards",
        evidence_class:"runtime_side_effect_boundary",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_live_context_attachment:true,
        blocks_runtime_mutation:true,
        blocks_model_invocation:true
      }
    ]
  '
)"

bounded_activation_packet_report_sha256="$(sha256_text "$BOUNDED_ACTIVATION_PACKET_JSON")"
activation_readiness_report_sha256="$(sha256_text "$ACTIVATION_READINESS_JSON")"
source_contracts_sha256="$(sha256_text "$source_contracts_json")"
runtime_attachment_packet_items_sha256="$(sha256_text "$runtime_attachment_packet_items_json")"
runtime_attachment_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-context-attachment-staging:$bounded_activation_packet_report_sha256:$activation_readiness_report_sha256:$source_contracts_sha256:$runtime_attachment_packet_items_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
runtime_attachment_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-context-attachment-staging:report-only:no-runtime-mutation:no-live-context-attachment:no-model-invocation:no-auth-secret-read:no-usage-recording"
)"
side_effect_hash_sha256="$(
  sha256_text "router_handoff_recorded=false;runtime_router_mutated=false;hepta_intelligence_context_attached=false;context_injection_performed=false;model_invoked=false;auth_secret_read=false;usage_recorded=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson bounded "$BOUNDED_ACTIVATION_PACKET_JSON" \
  --argjson readiness "$ACTIVATION_READINESS_JSON" \
  --argjson source_contracts "$source_contracts_json" \
  --argjson packet_items "$runtime_attachment_packet_items_json" \
  '
    $bounded.runtime == "hepta"
    and $bounded.status == "ready"
    and $bounded.gate == "hepta_memory_intelligence_kg_full_enablement_bounded_prompt_preview_context_handoff_activation_packet_gate"
    and $bounded.bounded_prompt_preview_context_handoff_activation_packet_ready == true
    and $bounded.bounded_prompt_preview_context_handoff_activation_packet_status == "blocked"
    and $bounded.activation_packet_shape_ready == true
    and $bounded.activation_packet_recorded == false
    and $bounded.activation_packet_persisted == false
    and $bounded.activation_packet_accepted == false
    and $bounded.prompt_preview_rendered == false
    and $bounded.prompt_payload_materialized == false
    and $bounded.context_injection_performed == false
    and $bounded.model_invoked == false
    and $bounded.live_kg_write_performed == false
    and ($bounded.allowed_next_actions | any(.action == "stage_runtime_provider_router_context_attachment_packet" and .status == "allowed_report_only_next_slice" and .attaches_live_context == false and .mutates_runtime == false and .invokes_model == false))
    and ($bounded.side_effects | to_entries | all(.value == false))
    and $readiness.runtime == "hepta"
    and $readiness.status == "ready"
    and $readiness.gate == "hepta_memory_intelligence_kg_full_enablement_activation_readiness_gate"
    and $readiness.full_enablement_activation_readiness_ready == true
    and $readiness.current_live_enabled_lane_count == 0
    and ($readiness.enablement_lanes | any(.lane == "hepta_intelligence_live_context" and .readiness == "ready_for_operator_approved_activation_slice" and .current_live_execution_enabled == false))
    and ($readiness.enablement_lanes | any(.lane == "runtime_provider_router_context_attachment" and .readiness == "ready_for_operator_approved_activation_slice" and .current_live_execution_enabled == false))
    and ($readiness.rust_contracts | any(.contract == "hepta-intelligence-memory-provider-router-activation-gate-v1" and .compile_checked_by_preflight_cargo_check == true))
    and ($readiness.rust_contracts | any(.contract == "hepta-intelligence-memory-turn-dispatch-gate-v1" and .compile_checked_by_preflight_cargo_check == true))
    and ($readiness.rust_contracts | any(.contract == "hepta-intelligence-memory-live-turn-preflight-v1" and .compile_checked_by_preflight_cargo_check == true))
    and $readiness.hepta_intelligence_context_attached == false
    and $readiness.context_injection_performed == false
    and $readiness.model_invoked == false
    and $readiness.provider_invoked == false
    and $readiness.credential_read == false
    and ($readiness.side_effects | to_entries | all(.value == false))
    and ($source_contracts | length) == 4
    and ($source_contracts | all(.source_pattern_present == true and .compile_checked_by_preflight_cargo_check == true))
    and ($source_contracts | all((.permits_live_context_attachment // false) == false))
    and ($source_contracts | all((.permits_provider_invocation // false) == false))
    and ($packet_items | length) == 12
    and ($packet_items | all(
      .required == true
      and .shape_declared == true
      and .accepted == false
      and .persisted == false
      and .blocks_live_context_attachment == true
      and .blocks_runtime_mutation == true
      and .blocks_model_invocation == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_context_attachment_staging_gate" \
  --arg bounded_activation_packet_report_sha256 "$bounded_activation_packet_report_sha256" \
  --arg activation_readiness_report_sha256 "$activation_readiness_report_sha256" \
  --arg source_contracts_sha256 "$source_contracts_sha256" \
  --arg runtime_attachment_packet_items_sha256 "$runtime_attachment_packet_items_sha256" \
  --arg runtime_attachment_contract_hash_sha256 "$runtime_attachment_contract_hash_sha256" \
  --arg runtime_attachment_policy_hash_sha256 "$runtime_attachment_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson bounded "$BOUNDED_ACTIVATION_PACKET_JSON" \
  --argjson readiness "$ACTIVATION_READINESS_JSON" \
  --argjson source_contracts "$source_contracts_json" \
  --argjson packet_items "$runtime_attachment_packet_items_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    runtime_attachment_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_context_attachment_staging_v1",
    runtime_attachment_mode:"runtime_provider_router_context_attachment_packet_shape_no_live_attachment_no_runtime_mutation_no_model_invocation",
    source_bounded_activation_packet_gate:$bounded.gate,
    source_activation_readiness_gate:$readiness.gate,
    source_bounded_activation_packet_report_sha256:$bounded_activation_packet_report_sha256,
    source_activation_readiness_report_sha256:$activation_readiness_report_sha256,
    source_contracts_sha256:$source_contracts_sha256,
    runtime_attachment_packet_items_sha256:$runtime_attachment_packet_items_sha256,
    runtime_attachment_contract_hash_sha256:$runtime_attachment_contract_hash_sha256,
    runtime_attachment_policy_hash_sha256:$runtime_attachment_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_context_attachment_staging_ready:true,
    runtime_provider_router_context_attachment_staging_status:"blocked",
    runtime_attachment_packet_shape_ready:true,
    runtime_attachment_packet_recorded:false,
    runtime_attachment_packet_persisted:false,
    runtime_attachment_packet_accepted:false,
    runtime_attachment_packet_delivered:false,
    bounded_activation_packet_shape_ready:$bounded.activation_packet_shape_ready,
    bounded_activation_packet_status:$bounded.bounded_prompt_preview_context_handoff_activation_packet_status,
    bounded_activation_packet_recorded:$bounded.activation_packet_recorded,
    bounded_activation_packet_accepted:$bounded.activation_packet_accepted,
    full_enablement_activation_readiness_ready:$readiness.full_enablement_activation_readiness_ready,
    enablement_lane_count:$readiness.enablement_lane_count,
    ready_enablement_lane_count:$readiness.ready_enablement_lane_count,
    current_live_enabled_lane_count:$readiness.current_live_enabled_lane_count,
    source_contract_count:($source_contracts | length),
    source_contract_pattern_ready_count:($source_contracts | map(select(.source_pattern_present == true)) | length),
    source_contract_compile_checked_count:($source_contracts | map(select(.compile_checked_by_preflight_cargo_check == true)) | length),
    source_contracts:$source_contracts,
    provider_router_id:"hepta-native-model-provider-router",
    feature_flag_id:"HEPTA_MEMORY_CONTEXT_LIVE_TURN",
    activation_contract:"hepta-intelligence-memory-provider-router-activation-gate-v1",
    selected_canary_stage_id:"shadow-canary-0ppm",
    shadow_traffic_percent_ppm:0,
    max_context_node_count_cap:128,
    runtime_attachment_packet_item_count:($packet_items | length),
    required_runtime_attachment_packet_item_count:($packet_items | map(select(.required == true)) | length),
    declared_runtime_attachment_packet_item_count:($packet_items | map(select(.shape_declared == true)) | length),
    accepted_runtime_attachment_packet_item_count:($packet_items | map(select(.accepted == true)) | length),
    persisted_runtime_attachment_packet_item_count:($packet_items | map(select(.persisted == true)) | length),
    missing_runtime_attachment_packet_item_count:($packet_items | map(select(.accepted == false)) | length),
    live_context_attachment_blocking_packet_item_count:($packet_items | map(select(.blocks_live_context_attachment == true)) | length),
    runtime_mutation_blocking_packet_item_count:($packet_items | map(select(.blocks_runtime_mutation == true)) | length),
    model_invocation_blocking_packet_item_count:($packet_items | map(select(.blocks_model_invocation == true)) | length),
    runtime_attachment_packet_items:$packet_items,
    denied_by_runtime_attachment_packet:[
      "provider_router_identity_not_accepted",
      "memory_context_feature_flag_not_accepted",
      "activation_contract_binding_not_accepted",
      "shadow_canary_stage_not_accepted",
      "max_context_node_budget_not_accepted",
      "fallback_no_memory_provider_turn_hash_not_accepted",
      "cutover_gate_readiness_receipt_not_accepted",
      "operator_release_approval_receipt_not_accepted",
      "kill_switch_absence_receipt_not_accepted",
      "idempotency_key_receipt_not_accepted",
      "runtime_readback_evidence_receipt_not_accepted",
      "runtime_no_effect_adapter_guards_not_accepted",
      "runtime_router_mutation_denied",
      "live_context_attachment_denied",
      "context_injection_denied",
      "model_invocation_denied",
      "auth_secret_read_denied",
      "usage_recording_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_context_attachment_staging_packet_shape",
        status:"allowed_report_only",
        mutates_runtime:false,
        attaches_live_context:false,
        injects_context:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_context_attachment_negative_fixture_matrix",
        status:"allowed_report_only_next_slice",
        mutates_runtime:false,
        persists_router_handoff:false,
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
    operator_activation_receipt_required:true,
    bounded_prompt_preview_activation_packet_required:true,
    provider_router_identity_required:true,
    feature_flag_boundary_required:true,
    shadow_canary_stage_required:true,
    cutover_gate_readiness_receipt_required:true,
    kill_switch_absence_receipt_required:true,
    idempotency_key_required:true,
    runtime_readback_evidence_required:true,
    runtime_no_effect_adapter_guards_required:true,
    runtime_router_mutation_forbidden:true,
    live_context_attachment_forbidden:true,
    context_injection_forbidden:true,
    provider_model_invocation_forbidden:true,
    auth_secret_read_forbidden:true,
    usage_recording_forbidden:true,
    full_live_enablement_performed:false,
    runtime_router_mutated:false,
    router_handoff_recorded:false,
    runtime_attachment_packet_recorded:false,
    runtime_attachment_packet_persisted:false,
    memory_context_activation_handoff_persisted:false,
    readback_evidence_persisted:false,
    hepta_intelligence_context_attached:false,
    live_context_attached_to_prompt:false,
    context_injection_allowed:false,
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
      runtime_router_mutated:false,
      router_handoff_recorded:false,
      runtime_attachment_packet_recorded:false,
      runtime_attachment_packet_persisted:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_context_attachment_staging_gate"
  and .runtime_attachment_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_context_attachment_staging_v1"
  and .runtime_provider_router_context_attachment_staging_ready == true
  and .runtime_provider_router_context_attachment_staging_status == "blocked"
  and .runtime_attachment_packet_shape_ready == true
  and .runtime_attachment_packet_recorded == false
  and .runtime_attachment_packet_persisted == false
  and .runtime_attachment_packet_accepted == false
  and .runtime_attachment_packet_delivered == false
  and .bounded_activation_packet_shape_ready == true
  and .bounded_activation_packet_status == "blocked"
  and .bounded_activation_packet_recorded == false
  and .bounded_activation_packet_accepted == false
  and .full_enablement_activation_readiness_ready == true
  and .enablement_lane_count == 6
  and .ready_enablement_lane_count == 6
  and .current_live_enabled_lane_count == 0
  and .source_contract_count == 4
  and .source_contract_pattern_ready_count == 4
  and .source_contract_compile_checked_count == 4
  and (.source_contracts | all(.source_pattern_present == true and .compile_checked_by_preflight_cargo_check == true))
  and .provider_router_id == "hepta-native-model-provider-router"
  and .feature_flag_id == "HEPTA_MEMORY_CONTEXT_LIVE_TURN"
  and .activation_contract == "hepta-intelligence-memory-provider-router-activation-gate-v1"
  and .selected_canary_stage_id == "shadow-canary-0ppm"
  and .shadow_traffic_percent_ppm == 0
  and .max_context_node_count_cap == 128
  and .runtime_attachment_packet_item_count == 12
  and .required_runtime_attachment_packet_item_count == 12
  and .declared_runtime_attachment_packet_item_count == 12
  and .accepted_runtime_attachment_packet_item_count == 0
  and .persisted_runtime_attachment_packet_item_count == 0
  and .missing_runtime_attachment_packet_item_count == 12
  and .live_context_attachment_blocking_packet_item_count == 12
  and .runtime_mutation_blocking_packet_item_count == 12
  and .model_invocation_blocking_packet_item_count == 12
  and (.runtime_attachment_packet_items | length) == 12
  and (.runtime_attachment_packet_items | all(.required == true and .shape_declared == true and .accepted == false and .persisted == false and .blocks_live_context_attachment == true and .blocks_runtime_mutation == true and .blocks_model_invocation == true))
  and (.denied_by_runtime_attachment_packet | length) == 18
  and (.allowed_next_actions | any(.action == "review_runtime_provider_router_context_attachment_staging_packet_shape" and .status == "allowed_report_only" and .mutates_runtime == false and .attaches_live_context == false and .injects_context == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_context_attachment_negative_fixture_matrix" and .status == "allowed_report_only_next_slice" and .mutates_runtime == false and .persists_router_handoff == false and .attaches_live_context == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "run_full_light_preflight" and .status == "allowed_verification_only" and .mutates_runtime == false and .attaches_live_context == false and .invokes_model == false and .writes_kg == false))
  and .operator_approval_required_before_runtime_attachment == true
  and .operator_activation_receipt_required == true
  and .bounded_prompt_preview_activation_packet_required == true
  and .provider_router_identity_required == true
  and .feature_flag_boundary_required == true
  and .shadow_canary_stage_required == true
  and .cutover_gate_readiness_receipt_required == true
  and .kill_switch_absence_receipt_required == true
  and .idempotency_key_required == true
  and .runtime_readback_evidence_required == true
  and .runtime_no_effect_adapter_guards_required == true
  and .runtime_router_mutation_forbidden == true
  and .live_context_attachment_forbidden == true
  and .context_injection_forbidden == true
  and .provider_model_invocation_forbidden == true
  and .auth_secret_read_forbidden == true
  and .usage_recording_forbidden == true
  and .full_live_enablement_performed == false
  and .runtime_router_mutated == false
  and .router_handoff_recorded == false
  and .memory_context_activation_handoff_persisted == false
  and .readback_evidence_persisted == false
  and .hepta_intelligence_context_attached == false
  and .live_context_attached_to_prompt == false
  and .context_injection_allowed == false
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
echo "Hepta memory/intelligence/KG full enablement runtime provider-router context attachment staging gate passed"
