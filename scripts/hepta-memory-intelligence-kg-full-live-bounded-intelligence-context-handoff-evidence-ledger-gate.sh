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

INTELLIGENCE_PREVIEW_JSON="$(
  capture_live_gate \
    "hepta-intelligence-bounded-context-attachment-preview-readback-route-gate" \
    scripts/hepta-intelligence-bounded-context-attachment-preview-readback-route-gate.sh
)"

BOUNDED_HANDOFF_BOUNDARY_JSON="$(
  capture_live_gate \
    "hepta-bounded-intelligence-context-handoff-prompt-preview-boundary-route-gate" \
    scripts/hepta-bounded-intelligence-context-handoff-prompt-preview-boundary-route-gate.sh
)"

CONTEXT_ATTACHMENT_LANE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane-gate.sh
)"

CONTEXT_HANDOFF_RECEIPT_AUDIT_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane-gate.sh
)"

ACTIVATION_PACKET_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-bounded-prompt-preview-context-handoff-activation-packet-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-memory-intelligence-kg-full-enablement-bounded-prompt-preview-context-handoff-activation-packet-gate.sh
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
  --argjson intelligence_preview "$INTELLIGENCE_PREVIEW_JSON" \
  --argjson bounded_handoff "$BOUNDED_HANDOFF_BOUNDARY_JSON" \
  --argjson context_attachment "$CONTEXT_ATTACHMENT_LANE_JSON" \
  --argjson context_audit "$CONTEXT_HANDOFF_RECEIPT_AUDIT_JSON" \
  --argjson activation_packet "$ACTIVATION_PACKET_JSON" \
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
    and $truth.intelligence_report_route_context_injection_performed == false
    and $truth.provider_invoked == false
    and $truth.model_invoked == false
    and $truth.credential_read == false
    and $truth.kg_live_write_performed == false
    and $truth.channel_send_performed == false
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
    and ($closure_live.closure_blockers | any(.blocker_id == "bounded_intelligence_context_handoff_acceptance"))
    and $intelligence_preview.status == "ready"
    and $intelligence_preview.live_endpoint_checked == true
    and $intelligence_preview.live_route_status == "ready"
    and $intelligence_preview.live_route_count == $truth.expected_route_count
    and $intelligence_preview.live_missing_route_count == 0
    and $intelligence_preview.intelligence_bounded_context_preview_ready == true
    and $intelligence_preview.bounded_context_attachment_preview_rendered == true
    and $intelligence_preview.bounded_context_readback_performed == true
    and $intelligence_preview.bounded_context_readback_hash_matched == true
    and $intelligence_preview.readback_receipt_persisted == false
    and $intelligence_preview.raw_context_materialized == false
    and $intelligence_preview.raw_prompt_payload_materialized == false
    and $intelligence_preview.prompt_payload_materialized == false
    and $intelligence_preview.provider_prompt_injection_performed == false
    and $intelligence_preview.context_injection_performed == false
    and $intelligence_preview.provider_invoked == false
    and $intelligence_preview.model_invoked == false
    and $intelligence_preview.credential_read == false
    and $intelligence_preview.kg_adapter_read_performed == false
    and $intelligence_preview.live_kg_write_performed == false
    and ($intelligence_preview.side_effects | to_entries | all(.value == false))
    and $bounded_handoff.status == "ready"
    and $bounded_handoff.live_endpoint_checked == true
    and $bounded_handoff.live_route_status == "ready"
    and $bounded_handoff.live_route_count == $truth.expected_route_count
    and $bounded_handoff.live_missing_route_count == 0
    and $bounded_handoff.bounded_context_handoff_preview_generated == true
    and $bounded_handoff.prompt_preview_boundary_generated == true
    and $bounded_handoff.boundary_readback_performed == true
    and $bounded_handoff.boundary_readback_hash_matched == true
    and $bounded_handoff.accepted_context_handoff_candidate_count == 0
    and $bounded_handoff.rendered_prompt_preview_candidate_count == 0
    and $bounded_handoff.accepted_prompt_preview_candidate_count == 0
    and ($bounded_handoff.readback_receipt_persisted // $bounded_handoff.side_effects.readback_receipt_persisted) == false
    and ($bounded_handoff.side_effects | to_entries | all(.value == false))
    and $context_attachment.status == "ready"
    and $context_attachment.hepta_intelligence_context_attachment_lane_enabled == true
    and $context_attachment.hepta_intelligence_context_attachment_allowed_by_lane == true
    and $context_attachment.hepta_intelligence_context_attached_by_report_route == false
    and $context_attachment.bounded_prompt_preview_lane_enabled == true
    and $context_attachment.prompt_preview_rendered_by_report_route == false
    and $context_attachment.prompt_payload_materialized_by_report_route == false
    and $context_attachment.context_injection_allowed_by_lane == false
    and $context_attachment.kg_live_write_lane_enabled == false
    and $context_attachment.provider_model_invocation_lane_enabled == false
    and ($context_attachment.side_effects | to_entries | all(.value == false))
    and $context_audit.status == "ready"
    and $context_audit.context_handoff_receipt_audit_lane_enabled == true
    and $context_audit.context_handoff_receipt_audit_allowed_by_lane == true
    and $context_audit.context_handoff_receipt_audit_requires_explicit_command == true
    and $context_audit.context_handoff_receipt_audit_recorded_by_report_route == false
    and $context_audit.context_handoff_receipt_audit_persisted_by_report_route == false
    and $context_audit.context_handoff_receipt_audit_accepted_by_report_route == false
    and $context_audit.context_attachment_performed_by_report_route == false
    and $context_audit.context_injection_allowed_by_lane == false
    and $context_audit.kg_live_write_lane_enabled == false
    and $context_audit.provider_model_invocation_lane_enabled == false
    and ($context_audit.side_effects | to_entries | all(.value == false))
    and $activation_packet.status == "ready"
    and $activation_packet.bounded_prompt_preview_context_handoff_activation_packet_ready == true
    and $activation_packet.bounded_prompt_preview_context_handoff_activation_packet_status == "blocked"
    and $activation_packet.activation_packet_shape_ready == true
    and $activation_packet.activation_packet_recorded == false
    and $activation_packet.activation_packet_persisted == false
    and $activation_packet.activation_packet_accepted == false
    and $activation_packet.context_handoff_checklist_ready == true
    and $activation_packet.context_handoff_checklist_status == "blocked"
    and $activation_packet.accepted_activation_packet_item_count == 0
    and $activation_packet.persisted_activation_packet_item_count == 0
    and $activation_packet.prompt_preview_allowed == false
    and $activation_packet.prompt_preview_rendered == false
    and $activation_packet.prompt_payload_materialized == false
    and $activation_packet.context_handoff_accepted == false
    and $activation_packet.context_injection_allowed == false
    and $activation_packet.context_injection_performed == false
    and $activation_packet.provider_invoked == false
    and $activation_packet.model_invoked == false
    and $activation_packet.credential_read == false
    and $activation_packet.external_kg_adapter_read_performed == false
    and $activation_packet.live_kg_write_performed == false
    and ($activation_packet.side_effects | to_entries | all(.value == false))
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
    and $watchdog.watchdog_evidence_contract_ready == true
    and $watchdog.soak_passed == true
    and $watchdog.soak_ok == $watchdog.soak_samples
    and ($watchdog.side_effects | to_entries | all(.value == false))
  ' >/dev/null

truth_sha256="$(sha256_text "$TRUTH_JSON")"
closure_sha256="$(sha256_text "$CLOSURE_JSON")"
closure_live_sha256="$(sha256_text "$CLOSURE_LIVE_JSON")"
intelligence_preview_sha256="$(sha256_text "$INTELLIGENCE_PREVIEW_JSON")"
bounded_handoff_sha256="$(sha256_text "$BOUNDED_HANDOFF_BOUNDARY_JSON")"
context_attachment_sha256="$(sha256_text "$CONTEXT_ATTACHMENT_LANE_JSON")"
context_handoff_receipt_audit_sha256="$(sha256_text "$CONTEXT_HANDOFF_RECEIPT_AUDIT_JSON")"
activation_packet_sha256="$(sha256_text "$ACTIVATION_PACKET_JSON")"
dependency_isolation_sha256="$(sha256_text "$DEPENDENCY_ISOLATION_JSON")"
watchdog_sha256="$(sha256_text "$WATCHDOG_JSON")"
ledger_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-bounded-intelligence-context-handoff-evidence-ledger:v1:$truth_sha256:$closure_sha256:$closure_live_sha256:$intelligence_preview_sha256:$bounded_handoff_sha256:$context_attachment_sha256:$context_handoff_receipt_audit_sha256:$activation_packet_sha256:$dependency_isolation_sha256:$watchdog_sha256"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_bounded_intelligence_context_handoff_evidence_ledger_gate" \
  --arg ledger_hash_sha256 "$ledger_hash_sha256" \
  --arg truth_sha256 "$truth_sha256" \
  --arg closure_sha256 "$closure_sha256" \
  --arg closure_live_sha256 "$closure_live_sha256" \
  --arg intelligence_preview_sha256 "$intelligence_preview_sha256" \
  --arg bounded_handoff_sha256 "$bounded_handoff_sha256" \
  --arg context_attachment_sha256 "$context_attachment_sha256" \
  --arg context_handoff_receipt_audit_sha256 "$context_handoff_receipt_audit_sha256" \
  --arg activation_packet_sha256 "$activation_packet_sha256" \
  --arg dependency_isolation_sha256 "$dependency_isolation_sha256" \
  --arg watchdog_sha256 "$watchdog_sha256" \
  --argjson truth "$TRUTH_JSON" \
  --argjson closure "$CLOSURE_JSON" \
  --argjson closure_live "$CLOSURE_LIVE_JSON" \
  --argjson intelligence_preview "$INTELLIGENCE_PREVIEW_JSON" \
  --argjson bounded_handoff "$BOUNDED_HANDOFF_BOUNDARY_JSON" \
  --argjson context_attachment "$CONTEXT_ATTACHMENT_LANE_JSON" \
  --argjson context_audit "$CONTEXT_HANDOFF_RECEIPT_AUDIT_JSON" \
  --argjson activation_packet "$ACTIVATION_PACKET_JSON" \
  --argjson dependency "$DEPENDENCY_ISOLATION_JSON" \
  --argjson watchdog "$WATCHDOG_JSON" \
  '{
    product: $product,
    runtime: $runtime,
    status: "ready",
    base_url: $base_url,
    gate: $gate,
    ledger_schema_version: "memory_intelligence_kg_full_live_bounded_intelligence_context_handoff_evidence_ledger_v1",
    ledger_hash_sha256: $ledger_hash_sha256,
    source_hashes: {
      truth_index: $truth_sha256,
      full_live_closure_gate: $closure_sha256,
      full_live_closure_live_endpoint: $closure_live_sha256,
      intelligence_bounded_context_preview_gate: $intelligence_preview_sha256,
      bounded_intelligence_context_handoff_boundary_gate: $bounded_handoff_sha256,
      hepta_intelligence_context_attachment_lane_gate: $context_attachment_sha256,
      context_handoff_receipt_audit_lane_gate: $context_handoff_receipt_audit_sha256,
      bounded_prompt_preview_context_handoff_activation_packet_gate: $activation_packet_sha256,
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
    bounded_intelligence_handoff_blocker_present: ($closure_live.closure_blockers | any(.blocker_id == "bounded_intelligence_context_handoff_acceptance")),
    bounded_intelligence_handoff_blocker_accepted: false,
    required_prior_gates: [
      "hepta_memory_intelligence_kg_full_live_scoped_canary_evidence_ledger_gate"
    ],
    bounded_intelligence_context_handoff_evidence: {
      scoped_canary_ledger_expected_before_this_gate: true,
      intelligence_bounded_context_preview_ready: $intelligence_preview.intelligence_bounded_context_preview_ready,
      bounded_context_attachment_preview_rendered: $intelligence_preview.bounded_context_attachment_preview_rendered,
      bounded_context_readback_performed: $intelligence_preview.bounded_context_readback_performed,
      bounded_context_readback_hash_matched: $intelligence_preview.bounded_context_readback_hash_matched,
      bounded_context_handoff_preview_generated: $bounded_handoff.bounded_context_handoff_preview_generated,
      prompt_preview_boundary_generated: $bounded_handoff.prompt_preview_boundary_generated,
      boundary_readback_performed: $bounded_handoff.boundary_readback_performed,
      boundary_readback_hash_matched: $bounded_handoff.boundary_readback_hash_matched,
      context_attachment_lane_enabled: $context_attachment.hepta_intelligence_context_attachment_lane_enabled,
      context_attachment_allowed_by_lane: $context_attachment.hepta_intelligence_context_attachment_allowed_by_lane,
      context_attachment_performed_by_report_route: false,
      context_handoff_receipt_audit_lane_enabled: $context_audit.context_handoff_receipt_audit_lane_enabled,
      context_handoff_receipt_audit_allowed_by_lane: $context_audit.context_handoff_receipt_audit_allowed_by_lane,
      context_handoff_receipt_audit_recorded_by_report_route: false,
      context_handoff_receipt_audit_persisted_by_report_route: false,
      context_handoff_receipt_audit_accepted_by_report_route: false,
      activation_packet_shape_ready: $activation_packet.activation_packet_shape_ready,
      activation_packet_status: $activation_packet.bounded_prompt_preview_context_handoff_activation_packet_status,
      accepted_activation_packet_item_count: $activation_packet.accepted_activation_packet_item_count,
      persisted_activation_packet_item_count: $activation_packet.persisted_activation_packet_item_count,
      prompt_preview_allowed: false,
      prompt_preview_rendered: false,
      prompt_payload_materialized: false,
      raw_context_materialized: false,
      raw_prompt_payload_materialized: false,
      context_handoff_accepted: false,
      context_injection_allowed: false,
      context_injection_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      kg_adapter_read_performed: false,
      live_kg_write_performed: false
    },
    active_service: {
      dependency_isolation_ready: $dependency.live_check_ready,
      forbidden_codex_engine_crates: $dependency.found_forbidden_codex_engine_crates,
      remaining_direct_dependency_count: $dependency.live_engine_dependency_closure.remaining_direct_dependency_count,
      watchdog_route_count: $watchdog.watchdog_route_count,
      watchdog_missing_route_count: $watchdog.watchdog_missing_route_count,
      watchdog_evidence_contract_ready: $watchdog.watchdog_evidence_contract_ready,
      watchdog_binary_sha_match: $watchdog.watchdog_binary_sha_match,
      short_soak_ok: $watchdog.soak_ok,
      short_soak_samples: $watchdog.soak_samples
    },
    ledger_decision: "bounded Intelligence context handoff evidence is live-verifiable as readback/hash only; keep blocker unaccepted until explicit receipt acceptance can be safely separated from prompt materialization and provider/model invocation",
    next_actions: [
      {
        action: "keep_bounded_intelligence_handoff_blocker_unaccepted",
        evidence_ready: true,
        accepts_blocker: false,
        materializes_prompt_payload: false,
        invokes_provider: false
      },
      {
        action: "build_kg_read_only_shadow_rank_live_evidence_ledger",
        uses_bounded_intelligence_handoff_hashes: true,
        reads_credentials: false,
        writes_kg: false,
        invokes_provider: false
      },
      {
        action: "build_provider_router_dry_run_evidence_ledger",
        invokes_provider: false,
        invokes_model: false,
        sends_externally: false
      }
    ],
    side_effects: {
      ledger_written_to_memory: false,
      durable_memory_store_write_performed: false,
      memory_store_mutated: false,
      hepta_intelligence_context_attached_to_provider_prompt: false,
      prompt_preview_rendered: false,
      prompt_payload_materialized: false,
      raw_context_materialized: false,
      raw_prompt_payload_materialized: false,
      context_handoff_recorded: false,
      context_handoff_persisted: false,
      context_handoff_accepted: false,
      context_handoff_receipt_audit_recorded: false,
      context_handoff_receipt_audit_persisted: false,
      context_handoff_receipt_audit_accepted: false,
      context_injection_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      secret_file_read: false,
      kg_adapter_read_performed: false,
      external_kg_adapter_read_performed: false,
      live_kg_write_performed: false,
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

echo "Hepta Memory/Intelligence/KG full-live bounded Intelligence context handoff evidence ledger gate passed"
