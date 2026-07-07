#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

capture_live_gate() {
  local name="$1"
  shift

  capture_json_report "$name" \
    env HEPTA_LIVE_URL="$BASE_URL" HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT=1 "$@"
}

TRUTH_JSON="$(
  capture_live_gate \
    "hepta-memory-intelligence-kg-activation-truth-index-route-gate" \
    scripts/hepta-memory-intelligence-kg-activation-truth-index-route-gate.sh
)"

CLOSURE_JSON="$(
  capture_live_gate \
    "hepta-full-live-activation-closure-index-route-gate" \
    scripts/hepta-full-live-activation-closure-index-route-gate.sh
)"

CLOSURE_LIVE_JSON="$(curl -fsS "$BASE_URL/api/hepta-full-live-activation-closure-index")"

SCOPED_CANARY_LEDGER_JSON="$(
  capture_live_gate \
    "hepta-memory-intelligence-kg-full-live-scoped-canary-evidence-ledger-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-scoped-canary-evidence-ledger-gate.sh
)"

BOUNDED_INTELLIGENCE_LEDGER_JSON="$(
  capture_live_gate \
    "hepta-memory-intelligence-kg-full-live-bounded-intelligence-context-handoff-evidence-ledger-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-bounded-intelligence-context-handoff-evidence-ledger-gate.sh
)"

KG_LANE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-preview-read-only-adapter-lane-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-preview-read-only-adapter-lane-gate.sh
)"

KG_STAGING_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-kg-external-adapter-staging-receipt-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-memory-intelligence-kg-full-enablement-kg-external-adapter-staging-receipt-gate.sh
)"

KG_SHADOW_RANK_JSON="$(
  capture_live_gate \
    "hepta-kg-read-only-adapter-shadow-rank-canary-route-gate" \
    scripts/hepta-kg-read-only-adapter-shadow-rank-canary-route-gate.sh
)"

DEPENDENCY_ISOLATION_JSON="$(
  capture_json_report \
    "hepta-active-service-dependency-isolation-require-live-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=1 \
      HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_REQUIRE_LIVE=1 \
      scripts/hepta-active-service-dependency-isolation.sh
)"

WATCHDOG_JSON="$(
  capture_json_report \
    "hepta-terminal-watchdog-soak-regression-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" scripts/hepta-terminal-watchdog-soak-regression-gate.sh
)"

jq -n -e \
  --argjson truth "$TRUTH_JSON" \
  --argjson closure "$CLOSURE_JSON" \
  --argjson closure_live "$CLOSURE_LIVE_JSON" \
  --argjson scoped "$SCOPED_CANARY_LEDGER_JSON" \
  --argjson bounded "$BOUNDED_INTELLIGENCE_LEDGER_JSON" \
  --argjson kg_lane "$KG_LANE_JSON" \
  --argjson kg_staging "$KG_STAGING_JSON" \
  --argjson kg_shadow "$KG_SHADOW_RANK_JSON" \
  --argjson dependency "$DEPENDENCY_ISOLATION_JSON" \
  --argjson watchdog "$WATCHDOG_JSON" \
  '
    $truth.status == "ready"
    and $truth.live_endpoint_checked == true
    and $truth.live_route_status == "ready"
    and $truth.live_route_count == $truth.expected_route_count
    and $truth.live_missing_route_count == 0
    and $truth.hepta_core_connected == true
    and $truth.memory_intelligence_kg_lanes_connected == true
    and $truth.operator_approved_lane_count == 3
    and $truth.ready_operator_approved_lane_count == 3
    and $truth.full_live_activation_enabled == false
    and $truth.full_live_activation_status == "blocked_report_only"
    and $truth.kg_report_route_adapter_read_performed == false
    and $truth.kg_live_write_performed == false
    and $truth.credential_read == false
    and $truth.provider_invoked == false
    and $truth.model_invoked == false
    and $truth.channel_send_performed == false
    and $truth.service_restarted == false
    and $truth.active_binary_mutated == false
    and $closure.status == "ready"
    and $closure.live_endpoint_checked == true
    and $closure.live_route_status == "ready"
    and $closure.closure_source_count == 8
    and $closure.ready_closure_source_count == 8
    and $closure.closure_blocker_count == 13
    and $closure.accepted_unrestricted_activation_blocker_count == 0
    and $closure.remaining_unrestricted_activation_blocker_count == 13
    and $closure.unrestricted_full_live_activation_enabled == false
    and ($closure.side_effects | to_entries | all(.value == false))
    and $closure_live.status == "ready"
    and $closure_live.full_live_activation_closure_index_status == "blocked_report_only"
    and ($closure_live.closure_blockers | length) == 13
    and ($closure_live.closure_blockers | all(.accepted == false))
    and ($closure_live.closure_blockers | any(.blocker_id == "kg_credential_reference_and_live_read_gate"))
    and ($closure_live.closure_blockers | any(.blocker_id == "kg_live_write_gate"))
    and $scoped.status == "ready"
    and $scoped.full_live_activation_status == "blocked_report_only"
    and $scoped.accepted_unrestricted_activation_blocker_count == 0
    and $scoped.remaining_unrestricted_activation_blocker_count == 13
    and ($scoped.side_effects | to_entries | all(.value == false))
    and $bounded.status == "ready"
    and $bounded.full_live_activation_status == "blocked_report_only"
    and $bounded.bounded_intelligence_handoff_blocker_accepted == false
    and $bounded.bounded_intelligence_context_handoff_evidence.boundary_readback_hash_matched == true
    and $bounded.bounded_intelligence_context_handoff_evidence.prompt_payload_materialized == false
    and $bounded.bounded_intelligence_context_handoff_evidence.provider_invoked == false
    and $bounded.bounded_intelligence_context_handoff_evidence.kg_adapter_read_performed == false
    and $bounded.bounded_intelligence_context_handoff_evidence.live_kg_write_performed == false
    and ($bounded.side_effects | to_entries | all(.value == false))
    and $kg_lane.status == "ready"
    and $kg_lane.operator_approved_activation_lane_effective == true
    and $kg_lane.kg_prompt_preview_lane_enabled == true
    and $kg_lane.kg_external_adapter_read_lane_enabled == true
    and $kg_lane.kg_external_adapter_read_allowed_by_lane == true
    and $kg_lane.kg_external_adapter_read_performed_by_report_route == false
    and $kg_lane.kg_external_adapter_credential_reference_required == true
    and $kg_lane.kg_external_adapter_credential_read_allowed_by_lane == false
    and $kg_lane.kg_external_adapter_credential_read_performed_by_report_route == false
    and $kg_lane.supported_kg_adapter_count == 3
    and ($kg_lane.supported_kg_adapters | index("graphiti") != null)
    and ($kg_lane.supported_kg_adapters | index("neo4j") != null)
    and ($kg_lane.supported_kg_adapters | index("cocoindex") != null)
    and $kg_lane.context_injection_allowed_by_lane == false
    and $kg_lane.kg_live_write_lane_enabled == false
    and $kg_lane.kg_live_write_allowed_by_lane == false
    and $kg_lane.provider_model_invocation_lane_enabled == false
    and $kg_lane.channel_delivery_lane_enabled == false
    and ($kg_lane.side_effects | to_entries | all(.value == false))
    and $kg_staging.status == "ready"
    and $kg_staging.kg_external_adapter_staging_lane_ready == true
    and $kg_staging.kg_external_adapter_staging_lane_current_live_execution_enabled == false
    and $kg_staging.credential_receipt_shape_ready == true
    and $kg_staging.rollback_receipt_shape_ready == true
    and $kg_staging.adapter_staging_receipt_count == 3
    and $kg_staging.supported_adapter_count == 3
    and $kg_staging.credential_reference_slot_count == 3
    and $kg_staging.credential_reference_recorded_count == 0
    and $kg_staging.credential_value_captured_count == 0
    and $kg_staging.credential_read_count == 0
    and $kg_staging.secret_file_read_count == 0
    and $kg_staging.operator_review_accepted_count == 0
    and $kg_staging.dry_run_sample_receipt_accepted_count == 0
    and $kg_staging.rollback_plan_receipt_declared_count == 3
    and $kg_staging.rollback_plan_receipt_accepted_count == 0
    and $kg_staging.kill_switch_receipt_accepted_count == 0
    and $kg_staging.post_write_validation_receipt_declared_count == 3
    and $kg_staging.post_write_validation_receipt_accepted_count == 0
    and $kg_staging.staging_ready_count == 0
    and $kg_staging.network_call_allowed_count == 0
    and $kg_staging.network_call_attempted_count == 0
    and $kg_staging.external_adapter_client_constructed_count == 0
    and $kg_staging.external_adapter_read_performed_count == 0
    and $kg_staging.external_db_write_performed_count == 0
    and $kg_staging.live_kg_write_performed_count == 0
    and $kg_staging.persisted_record_count == 0
    and $kg_staging.live_kg_write_forbidden == true
    and ($kg_staging.side_effects | to_entries | all(.value == false))
    and $kg_shadow.status == "ready"
    and $kg_shadow.live_endpoint_checked == true
    and $kg_shadow.live_route_status == "ready"
    and $kg_shadow.live_route_count == $truth.expected_route_count
    and $kg_shadow.live_missing_route_count == 0
    and $kg_shadow.kg_read_only_adapter_shadow_rank_canary_ready == true
    and $kg_shadow.canary_execution_mode == "kg_read_only_adapter_shadow_rank_fixture_no_credential_value_read_no_kg_write"
    and $kg_shadow.kg_adapter_name == "graphiti"
    and $kg_shadow.kg_adapter_allowlist_enforced == true
    and $kg_shadow.credential_reference_required == true
    and $kg_shadow.credential_reference_provided == true
    and $kg_shadow.credential_reference_kind == "opaque_reference_only"
    and $kg_shadow.credential_value_read == false
    and $kg_shadow.credential_read == false
    and $kg_shadow.secret_file_read == false
    and $kg_shadow.kg_adapter_read_mode == "read_only_shadow_fixture_no_network"
    and $kg_shadow.kg_read_only_adapter_shadow_envelope_rendered == true
    and $kg_shadow.kg_adapter_live_read_performed == false
    and $kg_shadow.kg_adapter_read_performed == false
    and $kg_shadow.external_network_call_performed == false
    and $kg_shadow.kg_shadow_rank_compared_to_transcript_baseline == true
    and $kg_shadow.kg_shadow_rank_compared_to_durable_memory_baseline == true
    and $kg_shadow.kg_shadow_rank_readback_performed == true
    and $kg_shadow.kg_shadow_rank_readback_hash_matched == true
    and $kg_shadow.shadow_rank_receipt_persisted == false
    and $kg_shadow.live_kg_write_performed == false
    and $kg_shadow.provider_invoked == false
    and $kg_shadow.model_invoked == false
    and $kg_shadow.channel_send_performed == false
    and $kg_shadow.external_send_performed == false
    and ($kg_shadow.side_effects | to_entries | all(.value == false))
    and $dependency.status == "ready"
    and $dependency.live_check_ready == true
    and $dependency.live_engine_dependency_closure.status == "ready"
    and $dependency.live_engine_dependency_closure.remaining_direct_dependency_count == 0
    and ($dependency.found_forbidden_codex_engine_crates | length) == 0
    and ($dependency.side_effects | to_entries | all(.value == false))
    and $watchdog.status == "ready"
    and $watchdog.watchdog_soak_regression_ready == true
    and $watchdog.watchdog_route_count == $truth.expected_route_count
    and $watchdog.watchdog_missing_route_count == 0
    and $watchdog.watchdog_binary_sha_match == true
    and $watchdog.soak_passed == true
    and $watchdog.soak_ok == $watchdog.soak_samples
    and ($watchdog.side_effects | to_entries | all(.value == false))
  ' >/dev/null

truth_sha256="$(sha256_text "$TRUTH_JSON")"
closure_sha256="$(sha256_text "$CLOSURE_JSON")"
closure_live_sha256="$(sha256_text "$CLOSURE_LIVE_JSON")"
scoped_canary_ledger_sha256="$(sha256_text "$SCOPED_CANARY_LEDGER_JSON")"
bounded_intelligence_ledger_sha256="$(sha256_text "$BOUNDED_INTELLIGENCE_LEDGER_JSON")"
kg_lane_sha256="$(sha256_text "$KG_LANE_JSON")"
kg_staging_sha256="$(sha256_text "$KG_STAGING_JSON")"
kg_shadow_rank_sha256="$(sha256_text "$KG_SHADOW_RANK_JSON")"
dependency_isolation_sha256="$(sha256_text "$DEPENDENCY_ISOLATION_JSON")"
watchdog_sha256="$(sha256_text "$WATCHDOG_JSON")"
ledger_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-kg-read-only-shadow-rank-adapter-evidence-ledger:v1:$truth_sha256:$closure_sha256:$closure_live_sha256:$scoped_canary_ledger_sha256:$bounded_intelligence_ledger_sha256:$kg_lane_sha256:$kg_staging_sha256:$kg_shadow_rank_sha256:$dependency_isolation_sha256:$watchdog_sha256"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_kg_read_only_shadow_rank_adapter_evidence_ledger_gate" \
  --arg ledger_hash_sha256 "$ledger_hash_sha256" \
  --arg truth_sha256 "$truth_sha256" \
  --arg closure_sha256 "$closure_sha256" \
  --arg closure_live_sha256 "$closure_live_sha256" \
  --arg scoped_canary_ledger_sha256 "$scoped_canary_ledger_sha256" \
  --arg bounded_intelligence_ledger_sha256 "$bounded_intelligence_ledger_sha256" \
  --arg kg_lane_sha256 "$kg_lane_sha256" \
  --arg kg_staging_sha256 "$kg_staging_sha256" \
  --arg kg_shadow_rank_sha256 "$kg_shadow_rank_sha256" \
  --arg dependency_isolation_sha256 "$dependency_isolation_sha256" \
  --arg watchdog_sha256 "$watchdog_sha256" \
  --argjson truth "$TRUTH_JSON" \
  --argjson closure "$CLOSURE_JSON" \
  --argjson closure_live "$CLOSURE_LIVE_JSON" \
  --argjson scoped "$SCOPED_CANARY_LEDGER_JSON" \
  --argjson bounded "$BOUNDED_INTELLIGENCE_LEDGER_JSON" \
  --argjson kg_lane "$KG_LANE_JSON" \
  --argjson kg_staging "$KG_STAGING_JSON" \
  --argjson kg_shadow "$KG_SHADOW_RANK_JSON" \
  --argjson dependency "$DEPENDENCY_ISOLATION_JSON" \
  --argjson watchdog "$WATCHDOG_JSON" \
  '{
    product: $product,
    runtime: $runtime,
    status: "ready",
    base_url: $base_url,
    gate: $gate,
    ledger_schema_version: "memory_intelligence_kg_full_live_kg_read_only_shadow_rank_adapter_evidence_ledger_v1",
    ledger_hash_sha256: $ledger_hash_sha256,
    source_hashes: {
      truth_index: $truth_sha256,
      full_live_closure_gate: $closure_sha256,
      full_live_closure_live_endpoint: $closure_live_sha256,
      scoped_canary_evidence_ledger_gate: $scoped_canary_ledger_sha256,
      bounded_intelligence_context_handoff_ledger_gate: $bounded_intelligence_ledger_sha256,
      kg_prompt_preview_read_only_adapter_lane_gate: $kg_lane_sha256,
      kg_external_adapter_staging_receipt_gate: $kg_staging_sha256,
      kg_read_only_adapter_shadow_rank_canary_gate: $kg_shadow_rank_sha256,
      active_dependency_isolation_gate: $dependency_isolation_sha256,
      terminal_watchdog_soak_gate: $watchdog_sha256
    },
    route_count: $truth.expected_route_count,
    live_route_count: $truth.live_route_count,
    live_missing_route_count: $truth.live_missing_route_count,
    hepta_core_connected: $truth.hepta_core_connected,
    hepta_core_full_fusion_complete: $closure.hepta_core_full_fusion_complete,
    memory_intelligence_kg_operator_lanes_ready: true,
    full_live_activation_status: "blocked_report_only",
    full_live_activation_enabled: false,
    unrestricted_full_live_activation_enabled: false,
    closure_source_count: $closure.closure_source_count,
    ready_closure_source_count: $closure.ready_closure_source_count,
    closure_blocker_count: $closure.closure_blocker_count,
    accepted_unrestricted_activation_blocker_count: $closure.accepted_unrestricted_activation_blocker_count,
    remaining_unrestricted_activation_blocker_count: $closure.remaining_unrestricted_activation_blocker_count,
    kg_credential_reference_and_live_read_blocker_present: ($closure_live.closure_blockers | any(.blocker_id == "kg_credential_reference_and_live_read_gate")),
    kg_credential_reference_and_live_read_blocker_accepted: false,
    kg_live_write_blocker_present: ($closure_live.closure_blockers | any(.blocker_id == "kg_live_write_gate")),
    kg_live_write_blocker_accepted: false,
    required_prior_gates: [
      "hepta_memory_intelligence_kg_full_live_scoped_canary_evidence_ledger_gate",
      "hepta_memory_intelligence_kg_full_live_bounded_intelligence_context_handoff_evidence_ledger_gate"
    ],
    kg_read_only_shadow_rank_adapter_evidence: {
      bounded_intelligence_boundary_readback_hash_matched: $bounded.bounded_intelligence_context_handoff_evidence.boundary_readback_hash_matched,
      kg_prompt_preview_lane_enabled: $kg_lane.kg_prompt_preview_lane_enabled,
      kg_external_adapter_read_lane_enabled: $kg_lane.kg_external_adapter_read_lane_enabled,
      kg_external_adapter_read_allowed_by_lane: $kg_lane.kg_external_adapter_read_allowed_by_lane,
      kg_external_adapter_read_performed_by_report_route: false,
      kg_external_adapter_credential_reference_required: $kg_lane.kg_external_adapter_credential_reference_required,
      kg_external_adapter_credential_read_allowed_by_lane: false,
      supported_kg_adapters: $kg_lane.supported_kg_adapters,
      kg_external_adapter_staging_lane_ready: $kg_staging.kg_external_adapter_staging_lane_ready,
      adapter_staging_receipt_count: $kg_staging.adapter_staging_receipt_count,
      credential_reference_slot_count: $kg_staging.credential_reference_slot_count,
      credential_reference_recorded_count: $kg_staging.credential_reference_recorded_count,
      credential_reference_persisted_count: $kg_staging.credential_reference_persisted_count,
      credential_reference_value_captured_count: $kg_staging.credential_reference_value_captured_count,
      credential_value_captured_count: $kg_staging.credential_value_captured_count,
      credential_read_count: $kg_staging.credential_read_count,
      secret_file_read_count: $kg_staging.secret_file_read_count,
      dry_run_sample_receipt_accepted_count: $kg_staging.dry_run_sample_receipt_accepted_count,
      rollback_plan_receipt_declared_count: $kg_staging.rollback_plan_receipt_declared_count,
      rollback_plan_receipt_accepted_count: $kg_staging.rollback_plan_receipt_accepted_count,
      kill_switch_receipt_accepted_count: $kg_staging.kill_switch_receipt_accepted_count,
      post_write_validation_receipt_declared_count: $kg_staging.post_write_validation_receipt_declared_count,
      post_write_validation_receipt_accepted_count: $kg_staging.post_write_validation_receipt_accepted_count,
      kg_read_only_adapter_shadow_rank_canary_ready: $kg_shadow.kg_read_only_adapter_shadow_rank_canary_ready,
      canary_execution_mode: $kg_shadow.canary_execution_mode,
      kg_adapter_name: $kg_shadow.kg_adapter_name,
      credential_reference_kind: $kg_shadow.credential_reference_kind,
      credential_reference_provided: $kg_shadow.credential_reference_provided,
      kg_adapter_read_mode: $kg_shadow.kg_adapter_read_mode,
      kg_read_only_adapter_shadow_envelope_rendered: $kg_shadow.kg_read_only_adapter_shadow_envelope_rendered,
      kg_shadow_rank_compared_to_transcript_baseline: $kg_shadow.kg_shadow_rank_compared_to_transcript_baseline,
      kg_shadow_rank_compared_to_durable_memory_baseline: $kg_shadow.kg_shadow_rank_compared_to_durable_memory_baseline,
      kg_shadow_rank_readback_performed: $kg_shadow.kg_shadow_rank_readback_performed,
      kg_shadow_rank_readback_hash_matched: $kg_shadow.kg_shadow_rank_readback_hash_matched,
      shadow_rank_receipt_persisted: false,
      credential_value_read: false,
      credential_read: false,
      secret_file_read: false,
      endpoint_value_captured: false,
      external_adapter_client_constructed: false,
      kg_adapter_live_read_performed: false,
      kg_adapter_read_performed: false,
      external_network_call_performed: false,
      external_db_write_performed: false,
      live_kg_write_performed: false,
      provider_invoked: false,
      model_invoked: false,
      channel_send_performed: false,
      external_send_performed: false
    },
    active_service: {
      dependency_isolation_ready: $dependency.live_check_ready,
      forbidden_codex_engine_crates: $dependency.found_forbidden_codex_engine_crates,
      remaining_direct_dependency_count: $dependency.live_engine_dependency_closure.remaining_direct_dependency_count,
      watchdog_route_count: $watchdog.watchdog_route_count,
      watchdog_missing_route_count: $watchdog.watchdog_missing_route_count,
      watchdog_binary_sha_match: $watchdog.watchdog_binary_sha_match,
      short_soak_ok: $watchdog.soak_ok,
      short_soak_samples: $watchdog.soak_samples
    },
    ledger_decision: "KG read-only shadow-rank evidence is live-verifiable as fixture readback/hash only; keep credential/live-read and live-write blockers unaccepted until credential reference acceptance, live adapter read, rollback, and post-write evidence are explicitly separated",
    next_actions: [
      {
        action: "keep_kg_credential_reference_and_live_read_blocker_unaccepted",
        evidence_ready: true,
        accepts_blocker: false,
        reads_credentials: false,
        constructs_external_adapter_client: false,
        performs_live_adapter_read: false
      },
      {
        action: "keep_kg_live_write_blocker_unaccepted",
        evidence_ready: false,
        accepts_blocker: false,
        writes_kg: false,
        requires_rollback_and_post_write_receipts: true
      },
      {
        action: "build_provider_router_dry_run_evidence_ledger",
        invokes_provider: false,
        invokes_model: false,
        sends_externally: false
      },
      {
        action: "build_work_graph_trace_guardrail_span_report_only_chain",
        mutates_runtime: false,
        spawns_agents: false,
        persists_trace: false
      }
    ],
    side_effects: {
      ledger_written_to_memory: false,
      durable_memory_store_write_performed: false,
      memory_store_mutated: false,
      prompt_preview_rendered: false,
      prompt_payload_materialized: false,
      raw_context_materialized: false,
      context_handoff_recorded: false,
      context_handoff_persisted: false,
      context_handoff_accepted: false,
      context_injection_performed: false,
      kg_shadow_rank_receipt_persisted: false,
      kg_credential_reference_recorded: false,
      kg_credential_reference_persisted: false,
      credential_reference_value_captured: false,
      credential_value_captured: false,
      credential_read: false,
      secret_file_read: false,
      endpoint_value_captured: false,
      external_adapter_client_constructed: false,
      external_network_call_performed: false,
      kg_adapter_live_read_performed: false,
      kg_adapter_read_performed: false,
      external_kg_adapter_read_performed: false,
      external_db_write_performed: false,
      live_kg_write_performed: false,
      rollback_executed: false,
      post_write_validation_performed: false,
      provider_invoked: false,
      model_invoked: false,
      channel_send_performed: false,
      external_send_performed: false,
      release_artifact_written: false,
      public_artifact_written: false,
      public_release_claimed: false,
      install_executed: false,
      service_restarted: false,
      active_binary_mutated: false
    }
  }'

echo "Hepta Memory/Intelligence/KG full-live KG read-only shadow-rank adapter evidence ledger gate passed"
