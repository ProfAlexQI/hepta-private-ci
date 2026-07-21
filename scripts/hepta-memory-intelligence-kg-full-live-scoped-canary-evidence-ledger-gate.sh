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

ZERO_RESIDUE_JSON="$(
  capture_live_gate \
    "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-boundary-route-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-boundary-route-gate.sh
)"

DRY_RUN_RECEIPT_JSON="$(
  capture_live_gate \
    "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-boundary-route-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-boundary-route-gate.sh
)"

PUBLICATION_NO_PERSISTENCE_JSON="$(
  capture_live_gate \
    "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary-route-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary-route-gate.sh
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

CLOSURE_LIVE_JSON="$(curl -fsS "$BASE_URL/api/hepta-full-live-activation-closure-index")"

jq -n -e \
  --argjson truth "$TRUTH_JSON" \
  --argjson closure "$CLOSURE_JSON" \
  --argjson closure_live "$CLOSURE_LIVE_JSON" \
  --argjson zero_residue "$ZERO_RESIDUE_JSON" \
  --argjson dry_run_receipt "$DRY_RUN_RECEIPT_JSON" \
  --argjson publication "$PUBLICATION_NO_PERSISTENCE_JSON" \
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
    and $truth.explicit_command_required_for_execution == true
    and $truth.full_live_activation_enabled == false
    and $truth.full_live_activation_status == "blocked_report_only"
    and $truth.full_live_activation_blocked == true
    and $truth.memory_report_route_write_performed == false
    and $truth.intelligence_report_route_context_injection_performed == false
    and $truth.kg_live_write_performed == false
    and $truth.provider_invoked == false
    and $truth.model_invoked == false
    and $truth.credential_read == false
    and $truth.channel_send_performed == false
    and $truth.service_restarted == false
    and $truth.active_binary_mutated == false
    and $closure.status == "ready"
    and $closure.live_endpoint_checked == true
    and $closure.live_route_status == "ready"
    and $closure.live_route_count == $closure.expected_route_count
    and $closure.live_missing_route_count == 0
    and $closure.closure_source_count == 8
    and $closure.ready_closure_source_count == 8
    and $closure.closure_blocker_count == 13
    and $closure.accepted_unrestricted_activation_blocker_count == 0
    and $closure.remaining_unrestricted_activation_blocker_count == 13
    and $closure.unrestricted_full_live_activation_enabled == false
    and $closure.hepta_core_connected == true
    and $closure.hepta_core_full_fusion_complete == true
    and $closure.operator_approved_lanes_ready == true
    and ($closure.side_effects | to_entries | all(.value == false))
    and $closure_live.status == "ready"
    and $closure_live.full_live_activation_closure_index_status == "blocked_report_only"
    and ($closure_live.closure_blockers | length) == 13
    and ($closure_live.closure_blockers | all(.accepted == false))
    and $zero_residue.status == "ready"
    and $zero_residue.require_live_endpoint == 1
    and $zero_residue.live_endpoint_verified == true
    and $zero_residue.zero_residue_acceptance == true
    and $zero_residue.source_single_shot_memory_store_write_count == 1
    and $zero_residue.new_memory_store_write == false
    and $zero_residue.production_durable_write == false
    and $dry_run_receipt.status == "ready"
    and $dry_run_receipt.require_live_endpoint == 1
    and $dry_run_receipt.live_endpoint_verified == true
    and $dry_run_receipt.dry_run_execution_result_receipt_accepted == true
    and $dry_run_receipt.dry_run_execution_result_receipt_persisted == false
    and $dry_run_receipt.dry_run_execution_executed == false
    and $dry_run_receipt.production_durable_write == false
    and $dry_run_receipt.memory_store_write == false
    and $dry_run_receipt.wal_write == false
    and $dry_run_receipt.receipt_persisted == false
    and $publication.status == "ready"
    and $publication.require_live_endpoint == "1"
    and $publication.live_route_status == "ready"
    and $publication.live_route_count == $publication.expected_route_count
    and $publication.live_missing_route_count == 0
    and $publication.records_publication_result_receipt == false
    and $publication.persists_publication_result_receipt == false
    and $publication.publishes_release_artifact == false
    and $publication.claims_public_release == false
    and $publication.writes_production_durable_memory == false
    and $publication.writes_kg == false
    and $publication.invokes_provider == false
    and $publication.reads_credentials == false
    and $publication.sends_externally == false
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
zero_residue_sha256="$(sha256_text "$ZERO_RESIDUE_JSON")"
dry_run_receipt_sha256="$(sha256_text "$DRY_RUN_RECEIPT_JSON")"
publication_no_persistence_sha256="$(sha256_text "$PUBLICATION_NO_PERSISTENCE_JSON")"
dependency_isolation_sha256="$(sha256_text "$DEPENDENCY_ISOLATION_JSON")"
watchdog_sha256="$(sha256_text "$WATCHDOG_JSON")"
ledger_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-scoped-canary-evidence-ledger:v1:$truth_sha256:$closure_sha256:$closure_live_sha256:$zero_residue_sha256:$dry_run_receipt_sha256:$publication_no_persistence_sha256:$dependency_isolation_sha256:$watchdog_sha256"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_scoped_canary_evidence_ledger_gate" \
  --arg ledger_hash_sha256 "$ledger_hash_sha256" \
  --arg truth_sha256 "$truth_sha256" \
  --arg closure_sha256 "$closure_sha256" \
  --arg closure_live_sha256 "$closure_live_sha256" \
  --arg zero_residue_sha256 "$zero_residue_sha256" \
  --arg dry_run_receipt_sha256 "$dry_run_receipt_sha256" \
  --arg publication_no_persistence_sha256 "$publication_no_persistence_sha256" \
  --arg dependency_isolation_sha256 "$dependency_isolation_sha256" \
  --arg watchdog_sha256 "$watchdog_sha256" \
  --argjson truth "$TRUTH_JSON" \
  --argjson closure "$CLOSURE_JSON" \
  --argjson closure_live "$CLOSURE_LIVE_JSON" \
  --argjson zero_residue "$ZERO_RESIDUE_JSON" \
  --argjson dry_run_receipt "$DRY_RUN_RECEIPT_JSON" \
  --argjson publication "$PUBLICATION_NO_PERSISTENCE_JSON" \
  --argjson dependency "$DEPENDENCY_ISOLATION_JSON" \
  --argjson watchdog "$WATCHDOG_JSON" \
  '{
    product: $product,
    runtime: $runtime,
    status: "ready",
    base_url: $base_url,
    gate: $gate,
    ledger_schema_version: "memory_intelligence_kg_full_live_scoped_canary_evidence_ledger_v1",
    ledger_hash_sha256: $ledger_hash_sha256,
    source_hashes: {
      truth_index: $truth_sha256,
      full_live_closure_gate: $closure_sha256,
      full_live_closure_live_endpoint: $closure_live_sha256,
      memory_zero_residue_acceptance_gate: $zero_residue_sha256,
      scoped_production_memory_dry_run_receipt_gate: $dry_run_receipt_sha256,
      scoped_production_memory_publication_no_persistence_gate: $publication_no_persistence_sha256,
      active_dependency_isolation_gate: $dependency_isolation_sha256,
      terminal_watchdog_soak_gate: $watchdog_sha256
    },
    route_count: $truth.expected_route_count,
    live_route_count: $truth.live_route_count,
    live_missing_route_count: $truth.live_missing_route_count,
    hepta_core_connected: $truth.hepta_core_connected,
    hepta_core_full_fusion_complete: $closure.hepta_core_full_fusion_complete,
    memory_intelligence_kg_operator_lanes_ready: true,
    operator_approved_lane_count: $truth.operator_approved_lane_count,
    ready_operator_approved_lane_count: $truth.ready_operator_approved_lane_count,
    full_live_activation_enabled: false,
    full_live_activation_status: "blocked_report_only",
    unrestricted_full_live_activation_enabled: false,
    closure_source_count: $closure.closure_source_count,
    ready_closure_source_count: $closure.ready_closure_source_count,
    closure_blocker_count: $closure.closure_blocker_count,
    accepted_unrestricted_activation_blocker_count: $closure.accepted_unrestricted_activation_blocker_count,
    remaining_unrestricted_activation_blocker_count: $closure.remaining_unrestricted_activation_blocker_count,
    closure_blocker_ids: [$closure_live.closure_blockers[].blocker_id],
    scoped_canary_evidence: {
      memory_zero_residue_acceptance_ready: $zero_residue.zero_residue_acceptance,
      source_single_shot_memory_store_write_count: $zero_residue.source_single_shot_memory_store_write_count,
      source_single_shot_zero_residue: $zero_residue.zero_residue,
      scoped_production_durable_memory_dry_run_receipt_ready: $dry_run_receipt.dry_run_execution_result_receipt_accepted,
      dry_run_execution_executed: false,
      dry_run_receipt_persisted: false,
      production_durable_memory_write_performed: false,
      memory_store_write_performed: false,
      wal_write_performed: false,
      rollback_executed: false,
      release_publication_result_persisted: false,
      public_release_claimed: false
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
    ledger_decision: "scoped memory canary evidence is live-verifiable and side-effect-free; do not accept unrestricted full-live blockers or enable production mutation from this ledger",
    next_actions: [
      {
        action: "keep_full_live_closure_blocked_until_explicit_blocker_acceptance",
        writes_memory: false,
        writes_kg: false,
        invokes_provider: false,
        sends_externally: false,
        installs_or_restarts: false
      },
      {
        action: "build_bounded_intelligence_context_handoff_live_evidence_ledger",
        after_memory_scoped_canary_evidence: true,
        materializes_prompt_payload: false,
        invokes_provider: false
      },
      {
        action: "build_kg_read_only_shadow_rank_live_evidence_ledger",
        reads_credentials: false,
        writes_kg: false,
        invokes_provider: false
      },
      {
        action: "prepare_provider_router_dry_run_evidence_ledger_before_any_model_invocation",
        invokes_provider: false,
        invokes_model: false,
        sends_externally: false
      }
    ],
    side_effects: {
      ledger_written_to_memory: false,
      durable_memory_store_write_performed: false,
      memory_store_mutated: false,
      wal_write_performed: false,
      receipt_persisted: false,
      kg_adapter_read_performed: false,
      live_kg_write_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
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

echo "Hepta Memory/Intelligence/KG full-live scoped canary evidence ledger gate passed"
