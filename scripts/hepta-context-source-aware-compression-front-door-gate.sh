#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  echo "hepta-context-source-aware-compression-front-door-gate: $*" >&2
  exit 1
}

assert_file_contains() {
  local file_path="$1"
  local needle="$2"
  local label="$3"

  if ! grep -F "$needle" "$file_path" >/dev/null; then
    fail "$label must contain: $needle"
  fi
}

line_number_of() {
  local file_path="$1"
  local needle="$2"
  local line

  line="$(grep -n -F "$needle" "$file_path" | head -n 1 | cut -d: -f1 || true)"
  if [ -z "$line" ]; then
    fail "$file_path is missing required text: $needle"
  fi
  printf '%s\n' "$line"
}

assert_line_before() {
  local file_path="$1"
  local before_needle="$2"
  local after_needle="$3"
  local label="$4"
  local before_line
  local after_line

  before_line="$(line_number_of "$file_path" "$before_needle")"
  after_line="$(line_number_of "$file_path" "$after_needle")"
  if [ "$before_line" -ge "$after_line" ]; then
    fail "$label expected '$before_needle' before '$after_needle'"
  fi
}

run_contract_gate() {
  local label="$1"
  local script_name="$2"

  echo "front-door: $label"
  bash "$repo_root/scripts/$script_name"
}

context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_script="hepta-context-source-aware-compression-front-door-gate.sh"
front_door_report_script="hepta-context-source-aware-compression-front-door-report.sh"
front_door_report_status_gate_script="hepta-context-source-aware-compression-front-door-report-status-gate.sh"
front_door_report_status_negative_gate_script="hepta-context-source-aware-compression-front-door-report-status-negative-gate.sh"
front_door_report_status_fixture_matrix_gate_script="hepta-context-source-aware-compression-front-door-report-status-fixture-matrix-gate.sh"
front_door_report_status_artifact_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-gate.sh"
front_door_report_status_artifact_export_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-export-gate.sh"
front_door_report_status_artifact_export_negative_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-export-negative-gate.sh"
front_door_report_status_artifact_export_precheck_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-export-precheck-gate.sh"
front_door_report_status_artifact_export_idempotence_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-export-idempotence-gate.sh"
front_door_report_status_artifact_export_atomic_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-export-atomic-gate.sh"
front_door_report_status_artifact_export_writability_precheck_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-export-writability-precheck-gate.sh"
front_door_report_status_artifact_export_symlink_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-export-symlink-gate.sh"
front_door_report_status_artifact_export_hardlink_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-export-hardlink-gate.sh"
front_door_gate_list_parity_gate_script="hepta-context-source-aware-compression-front-door-gate-list-parity-gate.sh"
readiness_export_report_script="hepta-context-source-aware-compression-readiness-export-report.sh"
readiness_export_gate_script="hepta-context-source-aware-compression-readiness-export-gate.sh"
adaptive_budget_allocation_report_gate_script="hepta-context-adaptive-budget-allocation-report-gate.sh"
generated_context_inventory_gate_script="hepta-context-generated-context-inventory-gate.sh"
memory_snapshot_helper_boundary_gate_script="hepta-context-memory-snapshot-helper-boundary-gate.sh"
memory_test_module_boundary_gate_script="hepta-context-memory-test-module-boundary-gate.sh"
memory_recall_helper_boundary_gate_script="hepta-context-memory-recall-helper-boundary-gate.sh"
memory_recall_manifest_payload_light_gate_script="hepta-context-memory-recall-manifest-payload-light-gate.sh"
memory_taxonomy_report_gate_script="hepta-context-memory-taxonomy-report-gate.sh"
memory_formation_receipt_gate_script="hepta-context-memory-formation-receipt-gate.sh"
memory_formation_queue_gate_script="hepta-context-memory-formation-queue-gate.sh"
memory_formation_candidate_no_leak_gate_script="hepta-context-memory-formation-candidate-no-leak-export-gate.sh"
memory_temporal_fact_schema_gate_script="hepta-context-memory-temporal-fact-schema-gate.sh"
memory_temporal_fact_graph_gate_script="hepta-context-memory-temporal-fact-graph-gate.sh"
memory_temporal_graph_shadow_eval_gate_script="hepta-context-memory-temporal-graph-shadow-eval-gate.sh"
memory_eval_harness_seed_gate_script="hepta-context-memory-eval-harness-seed-gate.sh"
memory_adaptive_allocator_eval_shadow_gate_script="hepta-context-memory-adaptive-allocator-eval-shadow-gate.sh"
memory_recall_quality_gate_script="hepta-context-memory-recall-quality-gate.sh"
memory_ranked_recall_shadow_eval_gate_script="hepta-context-memory-ranked-recall-shadow-eval-gate.sh"
memory_provider_boundary_gate_script="hepta-context-memory-provider-boundary-gate.sh"
memory_provider_v2_boundary_gate_script="hepta-context-memory-provider-v2-boundary-gate.sh"
memory_shadow_regression_dashboard_gate_script="hepta-context-memory-shadow-regression-dashboard-gate.sh"
memory_shadow_quality_summary_gate_script="hepta-context-memory-shadow-quality-summary-gate.sh"
memory_shadow_quality_trend_snapshot_gate_script="hepta-context-memory-shadow-quality-trend-snapshot-gate.sh"
memory_shadow_canary_promotion_readiness_gate_script="hepta-context-memory-shadow-canary-promotion-readiness-gate.sh"
memory_shadow_canary_promotion_negative_rehearsal_gate_script="hepta-context-memory-shadow-canary-promotion-negative-rehearsal-gate.sh"
memory_shadow_canary_promotion_audit_digest_gate_script="hepta-context-memory-shadow-canary-promotion-audit-digest-gate.sh"
memory_shadow_canary_promotion_audit_freshness_gate_script="hepta-context-memory-shadow-canary-promotion-audit-freshness-gate.sh"
context_plane_status_report_gate_script="hepta-context-plane-status-report-gate.sh"
context_plane_activation_blocker_matrix_gate_script="hepta-context-plane-activation-blocker-matrix-gate.sh"
context_plane_operator_approval_packet_gate_script="hepta-context-plane-operator-approval-packet-gate.sh"
context_plane_operator_approval_packet_negative_gate_script="hepta-context-plane-operator-approval-packet-negative-export-gate.sh"
context_plane_operator_approval_packet_canonical_digest_gate_script="hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh"
context_plane_operator_approval_packet_digest_tamper_matrix_gate_script="hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh"
context_plane_operator_approval_packet_freshness_gate_script="hepta-context-plane-operator-approval-packet-freshness-gate.sh"
context_plane_operator_approval_packet_freshness_dependency_chain_gate_script="hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate.sh"
context_plane_operator_approval_packet_freshness_dependency_chain_canonical_digest_gate_script="hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate.sh"
context_plane_operator_approval_packet_freshness_dependency_chain_expiry_drift_gate_script="hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-gate.sh"
selected_snippet_live_prompt_compression_gate_script="hepta-context-selected-snippet-live-prompt-compression-gate.sh"
runtime_provider_rollup_manifest_handoff_gate_script="hepta-context-runtime-provider-rollup-manifest-handoff-gate.sh"
selected_snippet_api_surface_gate_script="hepta-context-selected-snippet-api-surface-gate.sh"
prompt_input_summary_gate_script="hepta-context-prompt-input-summary-gate.sh"
front_door_report="$repo_root/scripts/$front_door_report_script"

required_contract_terms=(
  "Context adaptive budget allocation dry-run report"
  "adaptive_budget_allocations"
  "current_heuristic_action"
  "proposed_action"
  "Memory taxonomy report"
  "memory_taxonomy"
  "manifest_memory_taxonomy_invalid"
  "Background memory formation receipt report"
  "memory_formation_receipts"
  "manifest_memory_formation_receipts_invalid"
  "production_write=false"
  "Memory formation queue dry-run report"
  "memory_formation_queue"
  "operator_review_required"
  "revocation_key_hash"
  "hepta-context-memory-formation-queue-gate.sh"
  "Memory formation candidate no-leak/export guard"
  "memory_formation_candidates"
  "memory_formation_candidate_previews"
  "candidate_text"
  "tool_args"
  "raw_idempotency_key"
  "hepta-context-memory-formation-candidate-no-leak-export-gate.sh"
  "Memory temporal fact schema dry-run"
  "memory_temporal_facts"
  "manifest_memory_temporal_facts_invalid"
  "dry_run_only=true"
  "hepta-context-memory-temporal-fact-schema-gate.sh"
  "Memory temporal fact graph dry-run"
  "memory_temporal_fact_graph"
  "fact_hash"
  "edge_hash"
  "validity_window"
  "hepta-context-memory-temporal-fact-graph-gate.sh"
  "Context memory eval harness seed"
  "recall_coverage"
  "missing_critical_fact"
  "token_saved"
  "safety_leak"
  "answer_quality_regression"
  "synthetic_long_session"
  "redacted_trace"
  "hepta-context-memory-eval-harness-seed-gate.sh"
  "Adaptive allocator eval shadow"
  "current_heuristic"
  "proposed_adaptive"
  "comparison_verdict"
  "shadow_threshold_pass"
  "missing_critical_fact_regression_count"
  "token_saved_regression_count"
  "no recall regression"
  "no precision regression"
  "no latency regression"
  "no token-cost regression"
  "no token-saved regression"
  "no adaptive allocator runtime activation"
  "no source-aware runtime activation"
  "no prompt assembly changes"
  "hepta-context-memory-adaptive-allocator-eval-shadow-gate.sh"
  "Source-aware compression readiness export surface"
  "source-aware-readiness-export=pass"
  "source-aware-readiness-export.operator-approval-evidence=contract-only"
  "source-aware-readiness-export.positive-route=unimplemented"
  "source-aware-readiness-export.no-production-consumption=pass"
  "source-aware-readiness-export.no-debug-export-leak=pass"
  "scripts/hepta-context-source-aware-compression-readiness-export-report.sh"
  "scripts/hepta-context-source-aware-compression-readiness-export-gate.sh"
  "codex-rs/hepta-core/src/memory/eval_harness.rs"
  "codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow.rs"
  "codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/comparison.rs"
  "codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/report.rs"
  "codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/result.rs"
  "codex-rs/hepta-core/src/memory/eval_harness/eval_seed.rs"
  "Hepta-memory snapshot helper boundary"
  "codex-rs/hepta-memory/src/snapshot_helpers.rs"
  "codex-rs/hepta-memory/src/snapshot_helpers/inspected_snapshot.rs"
  "codex-rs/hepta-memory/src/snapshot_helpers/snapshot.rs"
  "codex-rs/hepta-memory/src/snapshot_helpers/store.rs"
  "hepta-context-memory-snapshot-helper-boundary-gate.sh"
  "Hepta-memory test module boundary"
  "codex-rs/hepta-memory/src/tests/context_memory.rs"
  "codex-rs/hepta-memory/src/tests/context_plane.rs"
  "codex-rs/hepta-memory/src/tests/context_plane/activation_matrix.rs"
  "codex-rs/hepta-memory/src/tests/context_plane/operator_packet.rs"
  "codex-rs/hepta-memory/src/tests/context_plane/status.rs"
  "codex-rs/hepta-memory/src/tests/mod.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_core.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_helpers.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/availability.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/bundle.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/coverage.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/limit_pressure.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/omission.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_helpers/provenance.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_quality.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_quality/availability.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_quality/coverage.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_quality/inspection.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_quality/limit_pressure.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_quality/omission.rs"
  "codex-rs/hepta-memory/src/tests/recall_context_quality/provenance.rs"
  "codex-rs/hepta-memory/src/tests/recall_memory.rs"
  "codex-rs/hepta-memory/src/tests/recall_memory/formation.rs"
  "codex-rs/hepta-memory/src/tests/recall_memory/taxonomy.rs"
  "codex-rs/hepta-memory/src/tests/recall_memory/temporal.rs"
  "codex-rs/hepta-memory/src/tests/restore_preview.rs"
  "codex-rs/hepta-memory/src/tests/search.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_core.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_inspection.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_inspection/audit.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_inspection/drift.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_inspection/health.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_inspection/inspected.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_integrity.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_inventory.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_inventory/manifest.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_inventory/session_inventory.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_inventory/stats.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_restore.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_restore/impact.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_restore/inspected.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_restore/preview.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_restore/readiness.rs"
  "codex-rs/hepta-memory/src/tests/snapshot_restore/roundtrip.rs"
  "codex-rs/hepta-memory/src/tests/store.rs"
  "hepta-context-memory-test-module-boundary-gate.sh"
  "Hepta-memory recall helper boundary"
  "codex-rs/hepta-memory/src/recall_helpers.rs"
  "codex-rs/hepta-memory/src/recall_helpers/query.rs"
  "codex-rs/hepta-memory/src/recall_helpers/snapshot.rs"
  "codex-rs/hepta-memory/src/recall_helpers/store.rs"
  "hepta-context-memory-recall-helper-boundary-gate.sh"
  "Memory recall manifest payload-light gate"
  "recall snapshot -> recall query"
  "context manifest chain"
  "contain raw memory, transcript, control-record, selected snippet source"
  "metadata, or prior prompt text"
  "store_snapshot_recall_context_report_is_payload_light_across_query_boundaries"
  "turn_context_manifest_resolves_recall_provider_rollup_without_payload_text"
  "turn_context_manifest_resolves_selected_snippets_as_guarded_payload"
  "hepta-context-memory-recall-manifest-payload-light-gate.sh"
  "Selected-snippet live prompt compression gate"
  "record_context_updates_and_set_reference_context_item_rejects_prompt_unsafe_selected_snippets_under_source_aware_compression_opt_in"
  "record_context_updates_and_set_reference_context_item_honors_turn_scoped_source_aware_compression_opt_in"
  "user_input_with_turn_context_selected_snippets_reject_prompt_unsafe_payload"
  "hepta-context-selected-snippet-live-prompt-compression-gate.sh"
  "Runtime provider rollup manifest handoff gate"
  "RuntimeContextRecallTurnHandoff"
  "provider rollup and optional selected-snippet core envelope cannot drift"
  "context_recall_provider_rollup_maps_runtime_recall_to_payload_light_counts"
  "context_recall_turn_handoff_packages_rollup_and_opted_in_core_snippets"
  "native_turn_messages_with_context_recall_handoff_consumes_opted_in_runtime_handoff_without_leak"
  "record_context_updates_and_set_reference_context_item_consumes_turn_scoped_recall_rollup_and_selected_snippets_without_drift"
  "hepta-context-runtime-provider-rollup-manifest-handoff-gate.sh"
  "Selected-snippet API/caller surface gate"
  "app-server/TUI/exec caller surfaces"
  "selected-snippet-api-surface=pass"
  "selected-snippet-api-surface.default-surface=audit-pass"
  "selected-snippet-api-surface.app-server=experimental-turn-start-only"
  "selected-snippet-api-surface.tui=opt-in-no-log"
  "selected-snippet-api-surface.exec=opt-in"
  "selected-snippet-api-surface.history=no-routing-metadata"
  "hepta-context-selected-snippet-api-surface-gate.sh"
  "turn_start_context_recall_selected_snippets_requires_experimental_api_capability"
  "turn_start_source_aware_compression_canary_thread_history_hides_routing_metadata"
  "rollout_context_debug_summary_combines_payload_light_surfaces_without_cross_surface_leaks"
  "response-debug-export=pass"
  "response-debug-export.payload-light=pass"
  "response-debug-export.combined-surfaces=no-leak"
  "response-debug-export.strict-invalid=reject"
  "response-debug-export.runtime-activation=disabled"
  "Prompt Input Gate"
  "build_prompt_input_from_session_consumes_context_manifest_without_shadow_leak"
  "prompt-input=pass"
  "prompt-input.context-manifest=no-leak"
  "prompt-input.live-selected-snippet=guarded"
  "prompt-input.runtime-activation=disabled"
  "hepta-context-prompt-input-summary-gate.sh"
  "codex-rs/hepta-memory/src/context_plane_helpers.rs"
  "Context memory recall quality gate"
  "recall_quality_gate"
  "gate_pass"
  "fixture_matrix"
  "schema version 2"
  "both seeded fixture kinds represented exactly once"
  "minimum recall coverage 7000 basis points"
  "minimum precision 7000 basis points"
  "missing critical fact limit 2"
  "fixture pass count equal to fixture count"
  "zero fixture blocked count"
  "zero blocking reason count"
  "zero missing-critical-fact regressions"
  "zero recall regressions"
  "zero precision regressions"
  "blocking_reasons"
  "blocking_reason_count"
  "recall_coverage_regression"
  "side_effect_flag_enabled"
  "codex-rs/hepta-core/src/memory/recall_quality_gate.rs"
  "hepta-context-memory-recall-quality-gate.sh"
  "Ranked recall shadow eval"
  "ranked-recall-shadow-eval=pass"
  "ranked-recall-shadow-eval.payload-light=pass"
  "deterministic-shadow"
  "hybrid shadow-only"
  "calibrated reranking shadow"
  "lexical_bm25"
  "source_authority"
  "temporal_validity"
  "feedback"
  "query_match"
  "recency_tie_break"
  "budget_pressure"
  "ranked item counts"
  "hybrid-signal-min-basis-points"
  "hybrid-positive-signal-pass-count"
  "hybrid-regression-signal"
  "calibrated-reranking-win-count"
  "reranking-delta-min-basis-points"
  "token-tradeoff-min-basis-points"
  "reranking-regression-delta"
  "recall-floor-basis-points"
  "precision-floor-basis-points"
  "token-saved-min-basis-points"
  "latency-max-ms"
  "regret-max-basis-points"
  "ContextMemoryRankedRecallShadowEvalReport"
  "ContextMemoryRankedRecallShadowHybridSignal"
  "context_memory_ranked_recall_shadow_eval_report"
  "hepta-context-memory-ranked-recall-shadow-eval-report.sh"
  "hepta-context-memory-ranked-recall-shadow-eval-gate.sh"
  "Temporal graph shadow eval"
  "temporal-graph-shadow-eval=pass"
  "temporal-graph-shadow-eval.payload-light=pass"
  "topology_coverage"
  "validity_window_replay"
  "supersedes_replay"
  "node-coverage-floor-basis-points"
  "edge-coverage-floor-basis-points"
  "validity-window-floor-basis-points"
  "supersedes-floor-basis-points"
  "ContextMemoryTemporalGraphShadowEvalReport"
  "context_memory_temporal_graph_shadow_eval_report"
  "hepta-context-memory-temporal-graph-shadow-eval-report.sh"
  "hepta-context-memory-temporal-graph-shadow-eval-gate.sh"
  "MemoryProvider boundary"
  "MemoryProviderContextUpdateEnvelope"
  "MemoryProviderReport"
  "MemoryProviderClearReport"
  "MemoryProvider"
  "payload_light=true"
  "operator_approval_required=true"
  "prompt_payload_exported=false"
  "query_payload_exported=false"
  "ranked_payload_exported=false"
  "hepta-context-memory-provider-boundary-report.sh"
  "hepta-context-memory-provider-boundary-gate.sh"
  "MemoryProviderV2 boundary"
  "MemoryProviderWriteProposalReport"
  "MemoryProviderAddReport"
  "MemoryProviderCloseReport"
  "MemoryProviderV2AuditReport"
  "candidate_payload_exported=false"
  "source_payload_exported=false"
  "graph_write_performed=false"
  "hepta-context-memory-provider-v2-boundary-report.sh"
  "hepta-context-memory-provider-v2-boundary-gate.sh"
  "Memory shadow regression dashboard"
  "memory-shadow-regression-dashboard=pass"
  "memory-shadow-regression-dashboard.payload-light=pass"
  "ContextMemoryShadowRegressionDashboardReport"
  "context_memory_shadow_regression_dashboard_report"
  "input_report_count"
  "input_report_pass_count"
  "regression_blocking_count"
  "ranked_recall_comparison_summary_pass"
  "ranked_recall_min_positive_hybrid_score_basis_points"
  "ranked_recall_min_positive_reranking_delta_basis_points"
  "ranked_recall_min_positive_token_tradeoff_basis_points"
  "provider_payload_light"
  "hepta-context-memory-shadow-regression-dashboard-report.sh"
  "hepta-context-memory-shadow-regression-dashboard-gate.sh"
  "Memory shadow quality summary"
  "memory-shadow-quality-summary=pass"
  "memory-shadow-quality-summary.payload-light=pass"
  "ContextMemoryShadowQualitySummaryReport"
  "ContextMemoryShadowQualityTrend"
  "ContextMemoryShadowQualityOperatorSummary"
  "context_memory_shadow_quality_summary_report"
  "quality_signal_count"
  "quality_signal_pass_count"
  "operator_summary_redacted"
  "ranked_recall_comparison_summary_pass"
  "ranked_recall_min_positive_hybrid_score_basis_points"
  "ranked_recall_min_positive_reranking_delta_basis_points"
  "ready_shadow_only"
  "stable_pass"
  "hepta-context-memory-shadow-quality-summary-report.sh"
  "hepta-context-memory-shadow-quality-summary-gate.sh"
  "Memory shadow quality trend snapshot"
  "memory-shadow-quality-trend-snapshot=pass"
  "memory-shadow-quality-trend-snapshot.payload-light=pass"
  "ContextMemoryShadowQualityTrendSnapshotReport"
  "ContextMemoryShadowQualityTrendSnapshotMode"
  "ContextMemoryShadowQualityTrendWindowVerdict"
  "context_memory_shadow_quality_trend_snapshot_report"
  "window_observation_count"
  "required_pass_streak"
  "observed_pass_streak"
  "regression_window_blocking_count"
  "quality_signal_window_pass_count"
  "ranked_recall_comparison_window_pass_count"
  "ranked_recall_min_positive_hybrid_score_basis_points"
  "ranked_recall_min_positive_reranking_delta_basis_points"
  "operator_snapshot_redacted"
  "history_persistence_write"
  "stable_window"
  "hepta-context-memory-shadow-quality-trend-snapshot-report.sh"
  "hepta-context-memory-shadow-quality-trend-snapshot-gate.sh"
  "Memory shadow canary promotion readiness"
  "memory-shadow-canary-promotion-readiness=pass"
  "memory-shadow-canary-promotion-readiness.payload-light=pass"
  "ContextMemoryShadowCanaryPromotionReadinessReport"
  "ContextMemoryShadowCanaryPromotionMode"
  "ContextMemoryShadowCanaryPromotionDecision"
  "ContextMemoryShadowCanaryRehearsalVerdict"
  "context_memory_shadow_canary_promotion_readiness_report"
  "promotion_blocker_count"
  "rollback_rehearsal_pass_count"
  "kill_switch_rehearsal_pass_count"
  "soak_readback_pass_count"
  "canary_promotion_route_opened"
  "rollback_write"
  "hepta-context-memory-shadow-canary-promotion-readiness-report.sh"
  "hepta-context-memory-shadow-canary-promotion-readiness-gate.sh"
  "Memory shadow canary promotion negative rehearsal"
  "memory-shadow-canary-promotion-negative-rehearsal=pass"
  "memory-shadow-canary-promotion-negative-rehearsal.payload-light=pass"
  "memory-shadow-canary-promotion-negative-rehearsal.activation-shaped-route=blocked"
  "memory-shadow-canary-promotion-negative-rehearsal.rollback-write=blocked"
  "memory-shadow-canary-promotion-negative-rehearsal.canary-promotion-route=disabled"
  "memory-shadow-canary-promotion-negative-rehearsal.runtime-activation-state=disabled"
  "context_memory_shadow_canary_promotion_negative_rehearsal_blocks_activation_shaped_side_effects"
  "hepta-context-memory-shadow-canary-promotion-negative-rehearsal-report.sh"
  "hepta-context-memory-shadow-canary-promotion-negative-rehearsal-gate.sh"
  "Memory shadow canary promotion audit digest"
  "memory-shadow-canary-promotion-audit-digest=pass"
  "memory-shadow-canary-promotion-audit-digest.payload-light=pass"
  "memory-shadow-canary-promotion-audit-digest.readiness-report-lines=32"
  "memory-shadow-canary-promotion-audit-digest.negative-rehearsal-report-lines=14"
  "memory-shadow-canary-promotion-audit-digest.combined-report-lines=46"
  "readiness-report-sha256"
  "negative-rehearsal-report-sha256"
  "combined-report-sha256"
  "hepta-context-memory-shadow-canary-promotion-audit-digest-report.sh"
  "hepta-context-memory-shadow-canary-promotion-audit-digest-gate.sh"
  "Memory shadow canary promotion audit freshness"
  "memory-shadow-canary-promotion-audit-freshness=pass"
  "memory-shadow-canary-promotion-audit-freshness.payload-light=pass"
  "memory-shadow-canary-promotion-audit-freshness.source-audit-digest-report-lines=11"
  "source-audit-digest-report-sha256"
  "memory-shadow-canary-promotion-audit-freshness.audit-readiness-sequence=309"
  "memory-shadow-canary-promotion-audit-freshness.current-readiness-sequence=309"
  "memory-shadow-canary-promotion-audit-freshness.expires-after-sequence=310"
  "memory-shadow-canary-promotion-audit-freshness.stale-sequence=reject"
  "memory-shadow-canary-promotion-audit-freshness.expired-sequence=reject"
  "memory-shadow-canary-promotion-audit-freshness.future-sequence=reject"
  "memory-shadow-canary-promotion-audit-freshness.digest-replay=reject"
  "memory-shadow-canary-promotion-audit-freshness.mixed-source-digest=reject"
  "hepta-context-memory-shadow-canary-promotion-audit-freshness-report.sh"
  "hepta-context-memory-shadow-canary-promotion-audit-freshness-gate.sh"
  "Context Plane status/export report"
  "context-plane-status=pass"
  "source_registry"
  "adaptive_budget_allocation"
  "memory_taxonomy"
  "memory_formation_receipts"
  "memory_formation_queue"
  "memory_temporal_facts"
  "memory_temporal_fact_graph"
  "memory_temporal_graph_shadow_eval"
  "eval_harness_seed"
  "adaptive_allocator_eval_shadow"
  "recall_quality_gate"
  "memory_provider_boundary"
  "memory_shadow_canary_readiness"
  "memory_shadow_canary_promotion_readiness"
  "canary_promotion_checklist_pass_count"
  "canary_promotion_negative_rehearsal_check_pass"
  "canary_promotion_audit_digest_check_pass"
  "canary_promotion_audit_freshness_check_pass"
  "canary_promotion_rollback_rehearsal_pass_count"
  "canary_promotion_kill_switch_rehearsal_pass_count"
  "canary_promotion_soak_readback_pass_count"
  "recall_quality_blocking_reason_count"
  "recall_quality_blocking_reasons"
  "context-plane-status.recall-quality-blocking-reason-count=0"
  "context-plane-status.recall-quality-blocking-reasons=none"
  "context-plane-status.memory-temporal-graph-shadow-eval=shadow"
  "context-plane-status.memory-provider-boundary=shadow"
  "context-plane-status.memory-shadow-canary-readiness=shadow"
  "context-plane-status.memory-shadow-canary-promotion-readiness=shadow"
  "source_aware_front_door"
  "no prompt assembly changes"
  "no operator activation allowance"
  "hepta-context-plane-status-report.sh"
  "hepta-context-plane-status-report-gate.sh"
  "Context Plane activation blocker matrix"
  "context-plane-activation-blockers=pass"
  "operator_approval"
  "adaptive_budget_allocation_shadow_only"
  "temporal_graph_shadow_eval_shadow_only"
  "memory_provider_boundary_shadow_only"
  "memory_shadow_canary_readiness_shadow_only"
  "memory_shadow_canary_promotion_readiness_shadow_only"
  "source_aware_front_door_disabled"
  "operator_approval_missing"
  "side_effect_flag_enabled"
  "activation_allowed=false"
  "context-plane-activation-blockers.memory-temporal-graph-shadow-eval=blocked:temporal_graph_shadow_eval_shadow_only"
  "context-plane-activation-blockers.memory-provider-boundary=blocked:memory_provider_boundary_shadow_only"
  "context-plane-activation-blockers.memory-shadow-canary-readiness=blocked:memory_shadow_canary_readiness_shadow_only"
  "context-plane-activation-blockers.memory-shadow-canary-promotion-readiness=blocked:memory_shadow_canary_promotion_readiness_shadow_only"
  "context-plane-activation-blockers.recall-quality-blocking-reason-count=0"
  "context-plane-activation-blockers.recall-quality-blocking-reasons=none"
  "hepta-context-plane-activation-blocker-matrix-report.sh"
  "hepta-context-plane-activation-blocker-matrix-gate.sh"
  "Context Plane operator approval packet dry-run"
  "context-plane-operator-approval-packet=pass"
  "approval_required"
  "dry_run_only"
  "activation_command_present"
  "matrix row counts"
  "blocker reason counts"
  "recall_quality_blocking_reason_counts"
  "temporal_graph_shadow_eval_shadow_only"
  "memory_provider_boundary_shadow_only"
  "memory_shadow_canary_readiness_shadow_only"
  "memory_shadow_canary_promotion_readiness_shadow_only"
  "canary_promotion_checklist_pass_count"
  "canary_promotion_negative_rehearsal_check_pass"
  "canary_promotion_audit_digest_check_pass"
  "canary_promotion_audit_freshness_check_pass"
  "context-plane-operator-approval-packet.recall-quality-blocking-reason-count=0"
  "context-plane-operator-approval-packet.recall-quality-blocking-reasons=none"
  "context-plane-operator-approval-packet.blocker.temporal-graph-shadow-eval-shadow-only=1"
  "context-plane-operator-approval-packet.blocker.memory-provider-boundary-shadow-only=1"
  "context-plane-operator-approval-packet.blocker.memory-shadow-canary-readiness-shadow-only=1"
  "context-plane-operator-approval-packet.blocker.memory-shadow-canary-promotion-readiness-shadow-only=1"
  "context-plane-operator-approval-packet.canary-promotion.checklist-pass-count=4"
  "context-plane-operator-approval-packet.canary-promotion.negative-rehearsal-check=pass"
  "context-plane-operator-approval-packet.canary-promotion.audit-digest-check=pass"
  "context-plane-operator-approval-packet.canary-promotion.audit-freshness-check=pass"
  "context-plane-operator-approval-packet.canary-promotion.rollback-rehearsal-pass-count=3"
  "context-plane-operator-approval-packet.canary-promotion.kill-switch-rehearsal-pass-count=3"
  "context-plane-operator-approval-packet.canary-promotion.soak-readback-pass-count=3"
  "threshold snapshot"
  "required approval scopes"
  "adaptive_budget_allocation_runtime"
  "production_memory_write"
  "must not include activation commands"
  "hepta-context-plane-operator-approval-packet-report.sh"
  "hepta-context-plane-operator-approval-packet-gate.sh"
  "Context Plane operator approval packet no-activation-command negative export"
  "guard: malformed or activation-shaped operator approval packet inputs"
  "context-plane-operator-approval-packet-negative-export=pass"
  "activation-shaped"
  "activation_command"
  "raw_payload"
  "PII-shaped values"
  "activation-command=absent"
  "hepta-context-plane-operator-approval-packet-negative-export-report.sh"
  "hepta-context-plane-operator-approval-packet-negative-export-gate.sh"
  "Context Plane operator approval packet canonical export digest"
  "context-plane-operator-approval-packet-canonical-export-digest=pass"
  "approval report 107 lines"
  "negative export report 4 lines"
  "combined report 111 lines"
  "deterministic and idempotent"
  "hepta-context-plane-operator-approval-packet-canonical-export-digest-report.sh"
  "hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh"
  "Context Plane operator approval packet digest tamper fixture negative matrix"
  "context-plane-operator-approval-packet-digest-tamper-matrix=pass"
  "line-order tamper"
  "line-count tamper"
  "digest-value tamper"
  "ranked recall hybrid counter tamper"
  "activation-command injection"
  "raw-payload injection"
  "PII-shaped value injection"
  "write/activation flag injection"
  "canonical digest/no-payload guard"
  "hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh"
  "Context Plane operator approval packet freshness/staleness replay-protection dry-run"
  "context-plane-operator-approval-packet-freshness=pass"
  "approval-readiness-sequence=273"
  "current-readiness-sequence=273"
  "expires-after-sequence=274"
  "stale-sequence=reject"
  "expired-sequence=reject"
  "future-sequence=reject"
  "digest-replay=reject"
  "freshness/staleness/replay guard"
  "hepta-context-plane-operator-approval-packet-freshness-report.sh"
  "hepta-context-plane-operator-approval-packet-freshness-gate.sh"
  "Context Plane operator approval packet freshness dependency-chain stale-source negative matrix"
  "context-plane-operator-approval-packet-freshness-dependency-chain=pass"
  "approval report dependency"
  "negative export report dependency"
  "canonical digest report dependency"
  "tamper matrix report dependency"
  "freshness report dependency"
  "readiness-chain-generation=274"
  "freshness-source-sequence=273"
  "stale-source=reject"
  "mixed-generation=reject"
  "source-digest-mismatch=reject"
  "tamper-matrix-replay=reject"
  "dependency-chain guard"
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-report.sh"
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate.sh"
  "Context Plane operator approval packet freshness dependency-chain canonical digest mixed-source tamper matrix"
  "context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest=pass"
  "dependency-chain report 20 lines"
  "dependency-chain report SHA-256"
  "readiness-chain-generation=275"
  "source-readiness-chain-generation=274"
  "source-freshness-sequence=273"
  "reordered dependency rows"
  "mismatched upstream digests"
  "mixed generation/sequence replay windows"
  "injected activation/write/payload fields"
  "dependency-chain canonical digest guard"
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-report.sh"
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate.sh"
  "Context Plane operator approval packet freshness dependency-chain expiry/readiness-window drift guard"
  "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift=pass"
  "source canonical digest report 15 lines"
  "source canonical digest report SHA-256"
  "readiness-chain-generation=276"
  "source-readiness-chain-generation=275"
  "source-dependency-chain-generation=274"
  "readiness-window-start-sequence=273"
  "readiness-window-current-sequence=276"
  "readiness-window-expires-after-sequence=277"
  "readiness-window-max-drift-sequences=0"
  "expired-window=reject"
  "window-start-drift=reject"
  "window-current-drift=reject"
  "window-expiry-drift=reject"
  "source-digest-replay=reject"
  "expiry/readiness-window drift guard"
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-report.sh"
  "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-gate.sh"
  "Source-aware compression compile-independent front-door gate"
  "Source-aware compression front-door report status assertion"
  "Source-aware compression front-door report status negative harness"
  "Source-aware compression front-door report status fixture matrix"
  "Source-aware compression front-door report status artifact consumer"
  "Source-aware compression persisted status artifact export"
  "Source-aware compression persisted status artifact export negative matrix"
  "Source-aware compression persisted status artifact export precheck"
  "Source-aware compression persisted status artifact export overwrite/idempotence"
  "Source-aware compression persisted status artifact export atomic replace"
  "Source-aware compression persisted status artifact export writability precheck"
  "Source-aware compression persisted status artifact export symlink replacement"
  "Source-aware compression persisted status artifact export hardlink replacement"
  "Source-aware compression front-door gate-list parity"
  "source-aware-contracts=pass"
  "source-aware-contracts.runtime-activation=disabled"
  "source-aware-front-door-gate-list-parity=pass"
  "source-aware-front-door-gate-list-parity.gate-count=8"
  "source-aware-front-door-gate-list-parity.preflight-gates="
  "actual front-door run_contract_gate order"
  "the same eight gate tokens must be derivable"
  "HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT"
  "replace the symlink itself"
  "must not follow the symlink"
  "unlink/replace only that output pathname"
  "must not mutate the other hardlink target"
  "victim file"
  "before running the real front-door report"
  "unwritable parent directory"
  "before emitting source-aware-contracts status"
  "without creating a final artifact"
  "preexisting caller-provided file"
  "repeated successful runs"
  "overwritten, not appended"
  "same-directory temporary file"
  "no direct final-path copy"
  "no temporary artifact residue"
  "directory target"
  "missing parent directory"
  "unknown extra source-aware-contracts.* key"
  "no front-door diagnostic noise"
  "stable machine-readable status order"
  "compile-independent"
  "before runtime cargo stages"
  "runtime generated preview"
  "non-blocking classifier"
  "does not enable runtime activation"
)

for term in "${required_contract_terms[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$term" \
    "source-aware compression front-door contract"
done

assert_file_contains \
  "$debug_gate" \
  "$adaptive_budget_allocation_report_gate_script" \
  "adaptive budget allocation report debug gate"

assert_file_contains \
  "$debug_gate" \
  "$generated_context_inventory_gate_script" \
  "generated context inventory debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_snapshot_helper_boundary_gate_script" \
  "memory snapshot helper boundary debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_test_module_boundary_gate_script" \
  "memory test module boundary debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_recall_helper_boundary_gate_script" \
  "memory recall helper boundary debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_recall_manifest_payload_light_gate_script" \
  "memory recall manifest payload-light debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_taxonomy_report_gate_script" \
  "memory taxonomy report debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_formation_receipt_gate_script" \
  "memory formation receipt debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_formation_queue_gate_script" \
  "memory formation queue debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_formation_candidate_no_leak_gate_script" \
  "memory formation candidate no-leak debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_temporal_fact_schema_gate_script" \
  "memory temporal fact schema debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_temporal_fact_graph_gate_script" \
  "memory temporal fact graph debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_temporal_graph_shadow_eval_gate_script" \
  "memory temporal graph shadow eval debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_eval_harness_seed_gate_script" \
  "memory eval harness seed debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_adaptive_allocator_eval_shadow_gate_script" \
  "memory adaptive allocator eval shadow debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_recall_quality_gate_script" \
  "memory recall quality gate debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_ranked_recall_shadow_eval_gate_script" \
  "memory ranked recall shadow eval debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_provider_boundary_gate_script" \
  "memory provider boundary debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_provider_v2_boundary_gate_script" \
  "memory provider v2 boundary debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_shadow_regression_dashboard_gate_script" \
  "memory shadow regression dashboard debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_shadow_quality_summary_gate_script" \
  "memory shadow quality summary debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_shadow_quality_trend_snapshot_gate_script" \
  "memory shadow quality trend snapshot debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_shadow_canary_promotion_readiness_gate_script" \
  "memory shadow canary promotion readiness debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_shadow_canary_promotion_negative_rehearsal_gate_script" \
  "memory shadow canary promotion negative rehearsal debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_shadow_canary_promotion_audit_digest_gate_script" \
  "memory shadow canary promotion audit digest debug gate"

assert_file_contains \
  "$debug_gate" \
  "$memory_shadow_canary_promotion_audit_freshness_gate_script" \
  "memory shadow canary promotion audit freshness debug gate"

assert_file_contains \
  "$debug_gate" \
  "$context_plane_status_report_gate_script" \
  "context plane status report debug gate"

assert_file_contains \
  "$debug_gate" \
  "$context_plane_activation_blocker_matrix_gate_script" \
  "context plane activation blocker matrix debug gate"

assert_file_contains \
  "$debug_gate" \
  "$context_plane_operator_approval_packet_gate_script" \
  "context plane operator approval packet debug gate"

assert_file_contains \
  "$debug_gate" \
  "$context_plane_operator_approval_packet_negative_gate_script" \
  "context plane operator approval packet negative export debug gate"

assert_file_contains \
  "$debug_gate" \
  "$context_plane_operator_approval_packet_canonical_digest_gate_script" \
  "context plane operator approval packet canonical digest debug gate"

assert_file_contains \
  "$debug_gate" \
  "$context_plane_operator_approval_packet_digest_tamper_matrix_gate_script" \
  "context plane operator approval packet digest tamper matrix debug gate"

assert_file_contains \
  "$debug_gate" \
  "$context_plane_operator_approval_packet_freshness_gate_script" \
  "context plane operator approval packet freshness debug gate"

assert_file_contains \
  "$debug_gate" \
  "$context_plane_operator_approval_packet_freshness_dependency_chain_gate_script" \
  "context plane operator approval packet freshness dependency-chain debug gate"

assert_file_contains \
  "$debug_gate" \
  "$context_plane_operator_approval_packet_freshness_dependency_chain_canonical_digest_gate_script" \
  "context plane operator approval packet freshness dependency-chain canonical digest debug gate"

assert_file_contains \
  "$debug_gate" \
  "$context_plane_operator_approval_packet_freshness_dependency_chain_expiry_drift_gate_script" \
  "context plane operator approval packet freshness dependency-chain expiry drift debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_script" \
  "source-aware compression front-door report debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_gate_script" \
  "source-aware compression front-door report status debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_negative_gate_script" \
  "source-aware compression front-door report status negative debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_fixture_matrix_gate_script" \
  "source-aware compression front-door report status fixture matrix debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_artifact_gate_script" \
  "source-aware compression front-door report status artifact debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_artifact_export_gate_script" \
  "source-aware compression front-door report status artifact export debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_artifact_export_negative_gate_script" \
  "source-aware compression front-door report status artifact export negative debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_artifact_export_precheck_gate_script" \
  "source-aware compression front-door report status artifact export precheck debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_artifact_export_idempotence_gate_script" \
  "source-aware compression front-door report status artifact export idempotence debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_artifact_export_atomic_gate_script" \
  "source-aware compression front-door report status artifact export atomic debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_artifact_export_writability_precheck_gate_script" \
  "source-aware compression front-door report status artifact export writability debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_artifact_export_symlink_gate_script" \
  "source-aware compression front-door report status artifact export symlink debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_report_status_artifact_export_hardlink_gate_script" \
  "source-aware compression front-door report status artifact export hardlink debug gate"

assert_file_contains \
  "$debug_gate" \
  "$front_door_gate_list_parity_gate_script" \
  "source-aware compression front-door gate-list parity debug gate"

assert_file_contains \
  "$debug_gate" \
  "$readiness_export_gate_script" \
  "source-aware compression readiness export debug gate"

assert_file_contains \
  "$debug_gate" \
  "$selected_snippet_live_prompt_compression_gate_script" \
  "selected-snippet live prompt compression debug gate"

assert_file_contains \
  "$debug_gate" \
  "$runtime_provider_rollup_manifest_handoff_gate_script" \
  "runtime provider rollup manifest handoff debug gate"

assert_file_contains \
  "$debug_gate" \
  "$selected_snippet_api_surface_gate_script" \
  "selected-snippet API/caller surface debug gate"

assert_file_contains \
  "$debug_gate" \
  "$prompt_input_summary_gate_script" \
  "prompt-input context-manifest debug gate"

assert_file_contains \
  "$preflight_script" \
  "generated context inventory gate" \
  "generated context inventory preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context adaptive budget allocation dry-run report gate" \
  "adaptive budget allocation report preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context prompt-input gate" \
  "prompt-input context-manifest preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory snapshot helper boundary gate" \
  "memory snapshot helper boundary preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory test module boundary gate" \
  "memory test module boundary preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory recall helper boundary gate" \
  "memory recall helper boundary preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory taxonomy report gate" \
  "memory taxonomy report preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory formation receipt gate" \
  "memory formation receipt preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory formation candidate no-leak export gate" \
  "memory formation candidate no-leak preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory temporal fact schema dry-run gate" \
  "memory temporal fact schema preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory temporal fact graph dry-run gate" \
  "memory temporal fact graph preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory temporal graph shadow eval gate" \
  "memory temporal graph shadow eval preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory eval harness seed gate" \
  "memory eval harness seed preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory adaptive allocator eval shadow gate" \
  "memory adaptive allocator eval shadow preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory recall quality gate" \
  "memory recall quality gate preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory ranked recall shadow eval gate" \
  "memory ranked recall shadow eval preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory provider boundary gate" \
  "memory provider boundary preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory shadow regression dashboard gate" \
  "memory shadow regression dashboard preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory shadow quality summary gate" \
  "memory shadow quality summary preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory shadow quality trend snapshot gate" \
  "memory shadow quality trend snapshot preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory shadow canary promotion negative rehearsal gate" \
  "memory shadow canary promotion negative rehearsal preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory shadow canary promotion audit digest gate" \
  "memory shadow canary promotion audit digest preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context memory shadow canary promotion audit freshness gate" \
  "memory shadow canary promotion audit freshness preflight stage"

assert_file_contains \
  "$preflight_script" \
  "selected-snippet live prompt compression gate" \
  "selected-snippet live prompt compression preflight stage"

assert_file_contains \
  "$preflight_script" \
  "runtime provider rollup manifest handoff gate" \
  "runtime provider rollup manifest handoff preflight stage"

assert_file_contains \
  "$preflight_script" \
  "selected snippet API/caller surface gate" \
  "selected-snippet API/caller surface preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context plane status/export report gate" \
  "context plane status report preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context plane activation blocker matrix gate" \
  "context plane activation blocker matrix preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context plane operator approval packet dry-run gate" \
  "context plane operator approval packet preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context plane operator approval packet negative export guard" \
  "context plane operator approval packet negative export preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context plane operator approval packet canonical export digest gate" \
  "context plane operator approval packet canonical digest preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context plane operator approval packet digest tamper matrix gate" \
  "context plane operator approval packet digest tamper matrix preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context plane operator approval packet freshness replay-protection gate" \
  "context plane operator approval packet freshness preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain gate" \
  "context plane operator approval packet freshness dependency-chain preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain canonical digest gate" \
  "context plane operator approval packet freshness dependency-chain canonical digest preflight stage"

assert_file_contains \
  "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain expiry drift gate" \
  "context plane operator approval packet freshness dependency-chain expiry drift preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door machine-readable report" \
  "source-aware compression front-door report preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report status assertion" \
  "source-aware compression front-door report status preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report status negative harness" \
  "source-aware compression front-door report status negative preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report status fixture matrix" \
  "source-aware compression front-door report status fixture matrix preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report status artifact consumer" \
  "source-aware compression front-door report status artifact preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export" \
  "source-aware compression front-door report status artifact export preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export negative matrix" \
  "source-aware compression front-door report status artifact export negative preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export precheck" \
  "source-aware compression front-door report status artifact export precheck preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export overwrite/idempotence" \
  "source-aware compression front-door report status artifact export idempotence preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export atomic replace" \
  "source-aware compression front-door report status artifact export atomic preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export writability precheck" \
  "source-aware compression front-door report status artifact export writability preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export symlink replacement" \
  "source-aware compression front-door report status artifact export symlink preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export hardlink replacement" \
  "source-aware compression front-door report status artifact export hardlink preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door gate-list parity" \
  "source-aware compression front-door gate-list parity preflight stage"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression readiness export surface gate" \
  "source-aware compression readiness export preflight stage"

assert_file_contains \
  "$front_door_report" \
  "$front_door_script" \
  "source-aware compression front-door report runner"

assert_file_contains \
  "$front_door_report" \
  "source-aware-contracts=pass" \
  "source-aware compression front-door report status"

assert_line_before \
  "$preflight_script" \
  "context source registry health gate" \
  "generated context inventory gate" \
  "generated context inventory preflight stage order"

assert_line_before \
  "$preflight_script" \
  "generated context inventory gate" \
  "context adaptive budget allocation dry-run report gate" \
  "generated context inventory preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context source registry rust resolver gate" \
  "context adaptive budget allocation dry-run report gate" \
  "adaptive budget allocation report preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context adaptive budget allocation dry-run report gate" \
  "context memory snapshot helper boundary gate" \
  "memory snapshot helper boundary preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory snapshot helper boundary gate" \
  "context memory test module boundary gate" \
  "memory test module boundary preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory test module boundary gate" \
  "context memory recall helper boundary gate" \
  "memory recall helper boundary preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory recall helper boundary gate" \
  "context memory recall manifest payload-light gate" \
  "memory recall manifest payload-light preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory recall manifest payload-light gate" \
  "context memory taxonomy report gate" \
  "memory taxonomy report preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory taxonomy report gate" \
  "context memory formation receipt gate" \
  "memory formation receipt preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory formation receipt gate" \
  "context memory formation queue dry-run gate" \
  "memory formation queue preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory formation queue dry-run gate" \
  "context memory formation candidate no-leak export gate" \
  "memory formation candidate no-leak preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory formation candidate no-leak export gate" \
  "context memory temporal fact schema dry-run gate" \
  "memory temporal fact schema preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory temporal fact schema dry-run gate" \
  "context memory temporal fact graph dry-run gate" \
  "memory temporal fact graph preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory temporal fact graph dry-run gate" \
  "context memory temporal graph shadow eval gate" \
  "memory temporal graph shadow eval preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory temporal graph shadow eval gate" \
  "context memory eval harness seed gate" \
  "memory eval harness seed preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory eval harness seed gate" \
  "context memory adaptive allocator eval shadow gate" \
  "memory adaptive allocator eval shadow preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory adaptive allocator eval shadow gate" \
  "context memory recall quality gate" \
  "memory recall quality gate preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory recall quality gate" \
  "context memory ranked recall shadow eval gate" \
  "memory ranked recall shadow eval preflight stage order"

assert_line_before \
  "$debug_gate" \
  "$memory_ranked_recall_shadow_eval_gate_script" \
  "$memory_provider_boundary_gate_script" \
  "memory provider boundary debug gate order"

assert_line_before \
  "$debug_gate" \
  "$memory_provider_boundary_gate_script" \
  "$memory_provider_v2_boundary_gate_script" \
  "memory provider v2 boundary debug gate order"

assert_line_before \
  "$debug_gate" \
  "$memory_provider_v2_boundary_gate_script" \
  "$memory_shadow_regression_dashboard_gate_script" \
  "memory shadow regression dashboard debug gate order"

assert_line_before \
  "$debug_gate" \
  "$memory_shadow_regression_dashboard_gate_script" \
  "$memory_shadow_quality_summary_gate_script" \
  "memory shadow quality summary debug gate order"

assert_line_before \
  "$debug_gate" \
  "$memory_shadow_quality_summary_gate_script" \
  "$memory_shadow_quality_trend_snapshot_gate_script" \
  "memory shadow quality trend snapshot debug gate order"

assert_line_before \
  "$debug_gate" \
  "$memory_shadow_quality_trend_snapshot_gate_script" \
  "$memory_shadow_canary_promotion_readiness_gate_script" \
  "memory shadow canary promotion readiness debug gate order"

assert_line_before \
  "$debug_gate" \
  "$memory_shadow_canary_promotion_readiness_gate_script" \
  "$memory_shadow_canary_promotion_negative_rehearsal_gate_script" \
  "memory shadow canary promotion negative rehearsal debug gate order"

assert_line_before \
  "$debug_gate" \
  "$memory_shadow_canary_promotion_negative_rehearsal_gate_script" \
  "$memory_shadow_canary_promotion_audit_digest_gate_script" \
  "memory shadow canary promotion audit digest debug gate order"

assert_line_before \
  "$debug_gate" \
  "$memory_shadow_canary_promotion_audit_digest_gate_script" \
  "$memory_shadow_canary_promotion_audit_freshness_gate_script" \
  "memory shadow canary promotion audit freshness debug gate order"

assert_line_before \
  "$debug_gate" \
  "$memory_shadow_canary_promotion_audit_freshness_gate_script" \
  "$context_plane_status_report_gate_script" \
  "memory shadow canary promotion audit freshness context plane status debug gate order"

assert_line_before \
  "$preflight_script" \
  "context memory ranked recall shadow eval gate" \
  "context memory provider boundary gate" \
  "memory provider boundary preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory provider boundary gate" \
  "context memory provider v2 boundary gate" \
  "memory provider v2 boundary preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory provider v2 boundary gate" \
  "context memory shadow regression dashboard gate" \
  "memory shadow regression dashboard preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory shadow regression dashboard gate" \
  "context memory shadow quality summary gate" \
  "memory shadow quality summary preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory shadow quality summary gate" \
  "context memory shadow quality trend snapshot gate" \
  "memory shadow quality trend snapshot preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory shadow quality trend snapshot gate" \
  "context memory shadow canary promotion readiness gate" \
  "memory shadow canary promotion readiness preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory shadow canary promotion readiness gate" \
  "context memory shadow canary promotion negative rehearsal gate" \
  "memory shadow canary promotion negative rehearsal preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory shadow canary promotion negative rehearsal gate" \
  "context memory shadow canary promotion audit digest gate" \
  "memory shadow canary promotion audit digest preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory shadow canary promotion audit digest gate" \
  "context memory shadow canary promotion audit freshness gate" \
  "memory shadow canary promotion audit freshness preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context memory shadow canary promotion audit freshness gate" \
  "context plane status/export report gate" \
  "memory shadow canary promotion audit freshness context plane status preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context plane status/export report gate" \
  "context plane activation blocker matrix gate" \
  "context plane activation blocker matrix preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context plane activation blocker matrix gate" \
  "context plane operator approval packet dry-run gate" \
  "context plane operator approval packet preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet dry-run gate" \
  "context plane operator approval packet negative export guard" \
  "context plane operator approval packet negative export preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet negative export guard" \
  "context plane operator approval packet canonical export digest gate" \
  "context plane operator approval packet canonical digest preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet canonical export digest gate" \
  "context plane operator approval packet digest tamper matrix gate" \
  "context plane operator approval packet digest tamper matrix preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet digest tamper matrix gate" \
  "context plane operator approval packet freshness replay-protection gate" \
  "context plane operator approval packet freshness preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet freshness replay-protection gate" \
  "context plane operator approval packet freshness dependency-chain gate" \
  "context plane operator approval packet freshness dependency-chain preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain gate" \
  "context plane operator approval packet freshness dependency-chain canonical digest gate" \
  "context plane operator approval packet freshness dependency-chain canonical digest preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain canonical digest gate" \
  "context plane operator approval packet freshness dependency-chain expiry drift gate" \
  "context plane operator approval packet freshness dependency-chain expiry drift preflight stage order"

assert_line_before \
  "$preflight_script" \
  "context plane operator approval packet freshness dependency-chain expiry drift gate" \
  "source-aware compression front-door machine-readable report" \
  "context plane operator approval packet freshness dependency-chain expiry drift front-door preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door machine-readable report" \
  "source-aware compression front-door report status assertion" \
  "source-aware compression front-door report status preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report status assertion" \
  "source-aware compression front-door report status negative harness" \
  "source-aware compression front-door report status negative preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report status negative harness" \
  "source-aware compression front-door report status fixture matrix" \
  "source-aware compression front-door report status fixture matrix preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report status fixture matrix" \
  "source-aware compression front-door report status artifact consumer" \
  "source-aware compression front-door report status artifact preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report status artifact consumer" \
  "source-aware compression front-door report persisted status artifact export" \
  "source-aware compression front-door report status artifact export preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export" \
  "source-aware compression front-door report persisted status artifact export negative matrix" \
  "source-aware compression front-door report status artifact export negative preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export negative matrix" \
  "source-aware compression front-door report persisted status artifact export precheck" \
  "source-aware compression front-door report status artifact export precheck preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export precheck" \
  "source-aware compression front-door report persisted status artifact export overwrite/idempotence" \
  "source-aware compression front-door report status artifact export idempotence preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export overwrite/idempotence" \
  "source-aware compression front-door report persisted status artifact export atomic replace" \
  "source-aware compression front-door report status artifact export atomic preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export atomic replace" \
  "source-aware compression front-door report persisted status artifact export writability precheck" \
  "source-aware compression front-door report status artifact export writability preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export writability precheck" \
  "source-aware compression front-door report persisted status artifact export symlink replacement" \
  "source-aware compression front-door report status artifact export symlink preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export symlink replacement" \
  "source-aware compression front-door report persisted status artifact export hardlink replacement" \
  "source-aware compression front-door report status artifact export hardlink preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export hardlink replacement" \
  "source-aware compression front-door gate-list parity" \
  "source-aware compression front-door gate-list parity preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door gate-list parity" \
  "source-aware compression readiness checklist gate" \
  "source-aware compression front-door gate-list parity preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression operator approval evidence contract gate" \
  "source-aware compression readiness export surface gate" \
  "source-aware compression readiness export preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression readiness export surface gate" \
  "source-aware compression activation negative matrix contract gate" \
  "source-aware compression readiness export preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door machine-readable report" \
  "hepta-memory recall mixed-tier drift fixtures" \
  "source-aware compression front-door report preflight stage order"

assert_line_before \
  "$preflight_script" \
  "core session source-aware prompt multi-compression explicit opt-in fixture" \
  "selected-snippet live prompt compression gate" \
  "selected-snippet live prompt compression preflight stage order"

assert_line_before \
  "$preflight_script" \
  "selected-snippet live prompt compression gate" \
  "core context contribution ledger settings-diff fixture" \
  "selected-snippet live prompt compression preflight stage order"

assert_line_before \
  "$debug_gate" \
  "hepta-context-runtime-provider-rollup-manifest-handoff-gate.sh" \
  "$selected_snippet_api_surface_gate_script" \
  "selected-snippet API/caller surface debug gate order"

assert_line_before \
  "$preflight_script" \
  "selected snippet default surface/schema audit" \
  "selected snippet API/caller surface gate" \
  "selected-snippet API/caller surface preflight stage order"

assert_line_before \
  "$preflight_script" \
  "selected snippet API/caller surface gate" \
  "native gateway context-recall worker scheduler route fixture" \
  "selected-snippet API/caller surface preflight stage order"

runtime_dirty_matches="$(
  {
    git -C "$repo_root" status --porcelain -- codex-rs/hepta-runtime/src || true
  } | awk '
    $2 == "codex-rs/hepta-runtime/src/lib.rs" ||
    $2 ~ /^codex-rs\/hepta-runtime\/src\/(wg_|work_graph).*preview\.rs$/ {
      print
    }
  '
)"

if [ -n "$runtime_dirty_matches" ]; then
  {
    echo "hepta-context-source-aware-compression-front-door-gate: non-blocking sibling runtime generated preview dirty state detected:"
    printf '%s\n' "$runtime_dirty_matches" | awk 'NR <= 20 { print }'
  } >&2
fi

run_contract_gate \
  "readiness checklist" \
  "hepta-context-source-aware-compression-readiness-gate.sh"

run_contract_gate \
  "operator approval evidence" \
  "hepta-context-source-aware-compression-operator-approval-evidence-gate.sh"

run_contract_gate \
  "readiness export surface" \
  "hepta-context-source-aware-compression-readiness-export-gate.sh"

run_contract_gate \
  "activation negative matrix" \
  "hepta-context-source-aware-compression-activation-negative-matrix-gate.sh"

run_contract_gate \
  "activation surface audit" \
  "hepta-context-source-aware-compression-activation-surface-audit.sh"

run_contract_gate \
  "leak bait" \
  "hepta-context-source-aware-compression-leak-bait-gate.sh"

run_contract_gate \
  "positive route readiness" \
  "hepta-context-source-aware-compression-positive-route-readiness-gate.sh"

run_contract_gate \
  "positive route implementation-change detector" \
  "hepta-context-source-aware-compression-positive-route-change-detector.sh"

echo "Hepta context source-aware compression front-door gate passed"
