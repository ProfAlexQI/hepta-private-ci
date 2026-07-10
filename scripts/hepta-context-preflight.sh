#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
skip_runtime_stages="${HEPTA_CONTEXT_PREFLIGHT_SKIP_RUNTIME:-0}"

case "$skip_runtime_stages" in
  0 | 1)
    ;;
  *)
    echo "hepta-context-preflight: HEPTA_CONTEXT_PREFLIGHT_SKIP_RUNTIME must be 0 or 1, got $skip_runtime_stages" >&2
    exit 1
    ;;
esac

run_stage() {
  local label="$1"
  shift
  echo "==> $label"
  "$@"
  echo "ok: $label"
}

run_runtime_stage() {
  local label="$1"
  shift
  if [ "$skip_runtime_stages" = "1" ]; then
    echo "==> $label"
    echo "skip: $label (HEPTA_CONTEXT_PREFLIGHT_SKIP_RUNTIME=1)"
    return 0
  fi
  run_stage "$label" "$@"
}

echo "hepta-context-preflight: lane=$lane"
echo "hepta-context-preflight: CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
echo "hepta-context-preflight: skip-runtime-stages=$skip_runtime_stages"

run_stage "context lane release manifest gate" \
  env HEPTA_CONTEXT_RELEASE_STRICT_GIT=1 \
    bash "$repo_root/scripts/hepta-context-release-manifest-gate.sh"

run_stage "context manifest replay hash boundary gate" \
  bash "$repo_root/scripts/hepta-context-manifest-replay-hash-boundary-gate.sh"

run_stage "context source registry catalog gate" \
  bash "$repo_root/scripts/hepta-context-source-registry-catalog-gate.sh"

run_stage "context source registry rust resolver gate" \
  bash "$repo_root/scripts/hepta-context-source-registry-rust-gate.sh"

run_stage "context source registry health gate" \
  bash "$repo_root/scripts/hepta-context-source-registry-health-gate.sh"

run_stage "generated context inventory gate" \
  bash "$repo_root/scripts/hepta-context-generated-context-inventory-gate.sh"

run_stage "context health/meta report gate" \
  bash "$repo_root/scripts/hepta-context-health-gate.sh"

run_stage "context adaptive budget allocation dry-run report gate" \
  bash "$repo_root/scripts/hepta-context-adaptive-budget-allocation-report-gate.sh"

run_stage "context memory snapshot helper boundary gate" \
  bash "$repo_root/scripts/hepta-context-memory-snapshot-helper-boundary-gate.sh"

run_stage "context memory test module boundary gate" \
  bash "$repo_root/scripts/hepta-context-memory-test-module-boundary-gate.sh"

run_stage "context memory recall helper boundary gate" \
  bash "$repo_root/scripts/hepta-context-memory-recall-helper-boundary-gate.sh"

run_stage "context memory recall manifest payload-light gate" \
  bash "$repo_root/scripts/hepta-context-memory-recall-manifest-payload-light-gate.sh"

run_stage "context memory taxonomy report gate" \
  bash "$repo_root/scripts/hepta-context-memory-taxonomy-report-gate.sh"

run_stage "context memory formation receipt gate" \
  bash "$repo_root/scripts/hepta-context-memory-formation-receipt-gate.sh"

run_stage "context memory formation queue dry-run gate" \
  bash "$repo_root/scripts/hepta-context-memory-formation-queue-gate.sh"

run_stage "context memory namespace policy shadow gate" \
  bash "$repo_root/scripts/hepta-context-memory-namespace-policy-gate.sh"

run_stage "context memory write-chain readiness/readback shadow gate" \
  bash "$repo_root/scripts/hepta-context-memory-write-chain-readiness-gate.sh"

run_stage "context memory write-chain receipt freshness/digest shadow gate" \
  bash "$repo_root/scripts/hepta-context-memory-write-chain-receipt-freshness-gate.sh"

run_stage "context memory formation candidate no-leak export gate" \
  bash "$repo_root/scripts/hepta-context-memory-formation-candidate-no-leak-export-gate.sh"

run_stage "context memory temporal fact schema dry-run gate" \
  bash "$repo_root/scripts/hepta-context-memory-temporal-fact-schema-gate.sh"

run_stage "context memory temporal fact graph dry-run gate" \
  bash "$repo_root/scripts/hepta-context-memory-temporal-fact-graph-gate.sh"

run_stage "context memory temporal graph shadow eval gate" \
  bash "$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-eval-gate.sh"

run_stage "context memory temporal graph shadow store skeleton gate" \
  bash "$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-store-gate.sh"

run_stage "context memory temporal graph shadow replay gate" \
  bash "$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-replay-gate.sh"

run_stage "context memory temporal graph shadow traversal diff gate" \
  bash "$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-traversal-diff-gate.sh"

run_stage "context memory temporal graph shadow traversal quality/SLO gate" \
  bash "$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-traversal-quality-gate.sh"

run_stage "context memory temporal graph shadow retrieval canary guard gate" \
  bash "$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-retrieval-canary-guard-gate.sh"

run_stage "context memory temporal graph shadow retrieval rollback/kill-switch gate" \
  bash "$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-gate.sh"

run_stage "context memory eval harness seed gate" \
  bash "$repo_root/scripts/hepta-context-memory-eval-harness-seed-gate.sh"

run_stage "context memory adaptive allocator eval shadow gate" \
  bash "$repo_root/scripts/hepta-context-memory-adaptive-allocator-eval-shadow-gate.sh"

run_stage "context memory recall quality gate" \
  bash "$repo_root/scripts/hepta-context-memory-recall-quality-gate.sh"

run_stage "context memory ranked recall shadow eval gate" \
  bash "$repo_root/scripts/hepta-context-memory-ranked-recall-shadow-eval-gate.sh"

run_stage "context memory provider boundary gate" \
  bash "$repo_root/scripts/hepta-context-memory-provider-boundary-gate.sh"

run_stage "context memory provider v2 boundary gate" \
  bash "$repo_root/scripts/hepta-context-memory-provider-v2-boundary-gate.sh"

run_stage "context memory shadow regression dashboard gate" \
  bash "$repo_root/scripts/hepta-context-memory-shadow-regression-dashboard-gate.sh"

run_stage "context memory shadow quality summary gate" \
  bash "$repo_root/scripts/hepta-context-memory-shadow-quality-summary-gate.sh"

run_stage "context memory shadow quality trend snapshot gate" \
  bash "$repo_root/scripts/hepta-context-memory-shadow-quality-trend-snapshot-gate.sh"

run_stage "context memory shadow canary promotion readiness gate" \
  bash "$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-readiness-gate.sh"

run_stage "context memory shadow canary promotion negative rehearsal gate" \
  bash "$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-negative-rehearsal-gate.sh"

run_stage "context memory shadow canary promotion audit digest gate" \
  bash "$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-audit-digest-gate.sh"

run_stage "context memory shadow canary promotion audit freshness gate" \
  bash "$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-audit-freshness-gate.sh"

run_stage "context plane status/export report gate" \
  bash "$repo_root/scripts/hepta-context-plane-status-report-gate.sh"

run_stage "context plane activation blocker matrix gate" \
  bash "$repo_root/scripts/hepta-context-plane-activation-blocker-matrix-gate.sh"

run_stage "context plane operator approval packet dry-run gate" \
  bash "$repo_root/scripts/hepta-context-plane-operator-approval-packet-gate.sh"

run_stage "context plane operator approval packet negative export guard" \
  bash "$repo_root/scripts/hepta-context-plane-operator-approval-packet-negative-export-gate.sh"

run_stage "context plane operator approval packet canonical export digest gate" \
  bash "$repo_root/scripts/hepta-context-plane-operator-approval-packet-canonical-export-digest-gate.sh"

run_stage "context plane operator approval packet digest tamper matrix gate" \
  bash "$repo_root/scripts/hepta-context-plane-operator-approval-packet-digest-tamper-matrix-gate.sh"

run_stage "context plane operator approval packet freshness replay-protection gate" \
  bash "$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-gate.sh"

run_stage "context plane operator approval packet freshness dependency-chain gate" \
  bash "$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-gate.sh"

run_stage "context plane operator approval packet freshness dependency-chain canonical digest gate" \
  bash "$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-gate.sh"

run_stage "context plane operator approval packet freshness dependency-chain expiry drift gate" \
  bash "$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift-gate.sh"

run_stage "source-aware compression front-door machine-readable report" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report.sh"

run_stage "source-aware compression front-door report status assertion" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-gate.sh"

run_stage "source-aware compression front-door report status negative harness" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-negative-gate.sh"

run_stage "source-aware compression front-door report status fixture matrix" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-fixture-matrix-gate.sh"

run_stage "source-aware compression front-door report status artifact consumer" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-gate.sh"

run_stage "source-aware compression front-door report persisted status artifact export" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-gate.sh"

run_stage "source-aware compression front-door report persisted status artifact export negative matrix" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-negative-gate.sh"

run_stage "source-aware compression front-door report persisted status artifact export precheck" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-precheck-gate.sh"

run_stage "source-aware compression front-door report persisted status artifact export overwrite/idempotence" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-idempotence-gate.sh"

run_stage "source-aware compression front-door report persisted status artifact export atomic replace" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-atomic-gate.sh"

run_stage "source-aware compression front-door report persisted status artifact export writability precheck" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-writability-precheck-gate.sh"

run_stage "source-aware compression front-door report persisted status artifact export symlink replacement" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-symlink-gate.sh"

run_stage "source-aware compression front-door report persisted status artifact export hardlink replacement" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-hardlink-gate.sh"

run_stage "source-aware compression front-door gate-list parity" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate-list-parity-gate.sh"

run_stage "source-aware compression readiness checklist gate" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-readiness-gate.sh"

run_stage "source-aware compression operator approval evidence contract gate" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-operator-approval-evidence-gate.sh"

run_stage "source-aware compression readiness export surface gate" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-readiness-export-gate.sh"

run_stage "source-aware compression activation negative matrix contract gate" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-activation-negative-matrix-gate.sh"

run_stage "source-aware compression leak bait contract gate" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-leak-bait-gate.sh"

run_stage "source-aware compression positive route readiness review gate" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-positive-route-readiness-gate.sh"

run_stage "source-aware compression positive route implementation-change detector" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-positive-route-change-detector.sh"

run_stage "hepta-memory recall mixed-tier drift fixtures" \
  cargo test --manifest-path "$manifest" -p hepta-memory \
    recall_context \
    --lib --message-format=short

run_stage "hepta-memory recall control fixtures" \
  cargo test --manifest-path "$manifest" -p hepta-memory \
    tombstone \
    --lib --message-format=short

run_stage "hepta-core memory query report fixtures" \
  cargo test --manifest-path "$manifest" -p hepta-core \
    memory_query_report \
    --lib --message-format=short

run_runtime_stage "hepta-runtime recall selector budget fixtures" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    context_recall_support \
    --lib --message-format=short

run_runtime_stage "hepta-runtime memory-control pressure fixtures" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    memory_control \
    --lib --message-format=short

run_runtime_stage "hepta-runtime recall provider rollup fixtures" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    context_recall_provider_rollup \
    --lib --message-format=short

run_runtime_stage "hepta-runtime selected snippet envelope/core conversion fixtures" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    context_recall_selected_snippet_envelope \
    --lib --message-format=short

run_runtime_stage "hepta-runtime selected snippet turn handoff fixtures" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    context_recall_turn_handoff \
    --lib --message-format=short

run_runtime_stage "runtime provider rollup manifest handoff gate" \
  bash "$repo_root/scripts/hepta-context-runtime-provider-rollup-manifest-handoff-gate.sh"

run_runtime_stage "hepta-runtime native selected snippet request assembly fixtures" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    native_turn_messages_with_context_recall_handoff \
    --lib --message-format=short

run_runtime_stage "hepta-runtime native selected snippet opted-in run fixtures" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    run_demo_turn_in_session_with_context_recall_handoff \
    --lib --message-format=short

run_runtime_stage "hepta-runtime worker selected snippet opt-in caller/scheduler fixtures" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    worker_task_context_recall_handoff \
    --lib --message-format=short

run_runtime_stage "hepta-runtime worker selected snippet operator scheduler fixture" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    worker_task_context_recall_operator_scheduler_handoff \
    --lib --message-format=short

run_runtime_stage "hepta-runtime worker selected snippet operator invocation fixture" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    worker_task_context_recall_operator_invocation \
    --lib --message-format=short

run_runtime_stage "hepta-runtime multi-agent selected snippet opt-in fixture" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    multi_agent_context_recall_handoff \
    --lib --message-format=short

run_runtime_stage "hepta-runtime multi-agent selected snippet operator invocation fixture" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    multi_agent_context_recall_operator_invocation \
    --lib --message-format=short

run_runtime_stage "hepta-runtime selected snippet unified operator invocation fixture" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    context_recall_operator_invocation \
    --lib --message-format=short

run_runtime_stage "hepta-runtime selected snippet operator command facade fixture" \
  cargo test --manifest-path "$manifest" -p hepta-runtime \
    context_recall_operator_invocation_command \
    --lib --message-format=short

run_stage "selected snippet default surface/schema audit" \
  bash "$repo_root/scripts/hepta-context-selected-snippet-surface-audit.sh"

run_stage "selected snippet API/caller surface gate" \
  bash "$repo_root/scripts/hepta-context-selected-snippet-api-surface-gate.sh"

run_runtime_stage "native gateway context-recall worker scheduler route fixture" \
  cargo test --manifest-path "$manifest" -p hepta-native-gateway \
    hepta_context_recall_worker_scheduler_handoff \
    --lib --message-format=short

run_stage "core context manifest builder fixtures" \
  cargo test --manifest-path "$manifest" -p codex-core \
    turn_context_manifest \
    --lib --message-format=short

run_stage "core context source-aware budget candidate fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    turn_context_manifest_records_source_aware_budget_candidates_without_prompt_mutation \
    --lib --message-format=short

run_stage "core context source-aware tier guard fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    source_aware_budget_candidate_priority_is_tier_guarded \
    --lib --message-format=short

run_stage "core context source-aware omission policy fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    turn_context_manifest_source_aware_omission_policy_omits_low_priority_sources \
    --lib --message-format=short

run_stage "core context source-aware prompt filtering fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    assemble_turn_context_with_policy_filters_omitted_prompt_fragments \
    --lib --message-format=short

run_stage "core context source-aware prompt truncation fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    assemble_turn_context_with_policy_truncates_prompt_and_manifest_text_hash_together \
    --lib --message-format=short

run_stage "core context source-aware prompt summary fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    assemble_turn_context_with_policy_summarizes_selected_recall_prompt_and_manifest_text_hash_together \
    --lib --message-format=short

run_stage "core context source-aware prompt defragment fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    assemble_turn_context_with_policy_defragments_tool_inventory_prompt_and_manifest_text_hash_together \
    --lib --message-format=short

run_stage "core context source-aware prompt prune fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    assemble_turn_context_with_policy_prunes_extension_developer_capabilities_prompt_and_manifest_text_hash_together \
    --lib --message-format=short

run_stage "core context source-aware prompt multi-compression fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    assemble_turn_context_with_policy_executes_summary_defragment_and_prune_together \
    --lib --message-format=short

run_stage "core context source-aware prompt protected-tier compression negative fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    assemble_turn_context_with_policy_does_not_compress_protected_tiers \
    --lib --message-format=short

run_stage "source-aware compression canary feature contract" \
  cargo test --manifest-path "$manifest" -p codex-features \
    source_aware_compression_canary_is_under_development \
    --lib --message-format=short

run_stage "core context source-aware compression marker injection path contract" \
  cargo test --manifest-path "$manifest" -p codex-core \
    turn_context_assembly_policy_requires_named_source_aware_compression_marker_injection_path \
    --lib --message-format=short

run_stage "source-aware compression activation surface audit" \
  bash "$repo_root/scripts/hepta-context-source-aware-compression-activation-surface-audit.sh"

run_stage "core session source-aware prompt filtering handoff fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    record_context_updates_and_set_reference_context_item_with_source_aware_policy_filters_omitted_prompt_fragments \
    --lib --message-format=short

run_stage "core session source-aware prompt truncation handoff fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    record_context_updates_and_set_reference_context_item_with_source_aware_policy_truncates_prompt_fragments \
    --lib --message-format=short

run_stage "core session source-aware prompt summary handoff fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    record_context_updates_and_set_reference_context_item_with_source_aware_policy_summarizes_prompt_fragments \
    --lib --message-format=short

run_stage "core session source-aware prompt defragment handoff fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    record_context_updates_and_set_reference_context_item_with_source_aware_policy_defragments_tool_inventory_prompt_fragments \
    --lib --message-format=short

run_stage "core session source-aware prompt prune handoff fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    record_context_updates_and_set_reference_context_item_with_source_aware_policy_prunes_extension_capabilities_prompt_fragments \
    --lib --message-format=short

run_stage "core session source-aware prompt multi-compression handoff fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    record_context_updates_and_set_reference_context_item_with_source_aware_policy_compresses_summary_defragment_and_prune_together \
    --lib --message-format=short

run_stage "core session source-aware prompt multi-compression explicit opt-in fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    record_context_updates_and_set_reference_context_item_honors_turn_scoped_source_aware_compression_opt_in \
    --lib --message-format=short

run_stage "selected-snippet live prompt compression gate" \
  bash "$repo_root/scripts/hepta-context-selected-snippet-live-prompt-compression-gate.sh"

run_stage "core context contribution ledger settings-diff fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    build_settings_update_items_manifest_uses_semantic_contribution_sources \
    --lib --message-format=short

run_stage "core context collaboration-mode clear diff fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    build_settings_update_items_emits_collaboration_mode_clear_when_instructions_disappear \
    --lib --message-format=short

run_stage "core context user/developer clear diff fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    build_settings_update_items_emits_user_and_developer_instruction_clears \
    --lib --message-format=short

run_stage "core context capability inventory clear diff fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    build_settings_update_items_emits_capability_inventory_clears \
    --lib --message-format=short

run_stage "core context extension fragment diff fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    build_settings_update_items_emits_extension_fragment_replacements_and_clears \
    --lib --message-format=short

run_stage "core session context manifest shadow handoff fixtures" \
  cargo test --manifest-path "$manifest" -p codex-core \
    record_context_updates_and_set_reference_context_item_ \
    --lib --message-format=short

run_stage "core selected snippet rollback trim fixture" \
  cargo test --manifest-path "$manifest" -p codex-core \
    drop_last_n_user_turns_trims_selected_context_recall_above_rolled_back_turn \
    --lib --message-format=short

run_stage "core session context manifest replay fixtures" \
  cargo test --manifest-path "$manifest" -p codex-core \
    record_initial_history_resumed_turn_context_after_compaction_reestablishes_reference_context_item \
    --lib --message-format=short

run_stage "protocol turn context decision ledger summary contract" \
  cargo test --manifest-path "$manifest" -p codex-protocol \
    turn_context_decision_entry_constructors_preserve_legacy_wire_strings \
    --lib --message-format=short

run_stage "protocol turn context compression candidate contract" \
  cargo test --manifest-path "$manifest" -p codex-protocol \
    turn_context_manifest_compression_candidates_are_payload_light_and_hashed \
    --lib --message-format=short

run_stage "protocol turn context compression stage contract" \
  cargo test --manifest-path "$manifest" -p codex-protocol \
    turn_context_manifest_compression_stages_are_payload_light_and_hashed \
    --lib --message-format=short

run_stage "protocol turn context manifest recall-selection contract" \
  cargo test --manifest-path "$manifest" -p codex-protocol \
    turn_context_manifest_recall_selection_serializes_payload_light_rollup \
    --lib --message-format=short

run_stage "protocol turn context manifest memory taxonomy contract" \
  cargo test --manifest-path "$manifest" -p codex-protocol \
    turn_context_manifest_memory_taxonomy_is_payload_light_and_hashed \
    --lib --message-format=short

run_stage "protocol turn context manifest selected-snippet contract" \
  cargo test --manifest-path "$manifest" -p codex-protocol \
    turn_context_manifest_selected_snippets_serializes_shadow_envelope \
    --lib --message-format=short

run_stage "protocol turn context selected-snippet request contract" \
  cargo test --manifest-path "$manifest" -p codex-protocol \
    user_input_with_turn_context_ \
    --lib --message-format=short

run_stage "app-server selected-snippet turn-start protocol contract" \
  cargo test --manifest-path "$manifest" -p codex-app-server-protocol \
    turn_start_params_round_trip_context_recall_selected_snippets \
    --lib --message-format=short

run_stage "app-server selected-snippet caller conversion contract" \
  cargo test --manifest-path "$manifest" -p codex-app-server-protocol \
    context_recall_selected_snippets_from_core \
    --lib --message-format=short

run_runtime_stage "tui selected-snippet caller opt-in helper fixture" \
  cargo test --manifest-path "$manifest" -p codex-tui \
    context_recall_selected_snippets_for_turn_start \
    --lib --message-format=short

run_runtime_stage "tui selected-snippet outbound command no-log fixture" \
  cargo test --manifest-path "$manifest" -p codex-tui \
    user_turn_selected_snippets_are_not_serialized \
    --lib --message-format=short

run_stage "exec selected-snippet caller opt-in helper fixture" \
  cargo test --manifest-path "$manifest" -p codex-exec \
    context_recall_selected_snippets_for_turn_start \
    --lib --message-format=short

run_runtime_stage "app-server selected-snippet turn-start handoff fixture" \
  cargo test --manifest-path "$manifest" -p codex-app-server \
    context_recall_selected_snippets_v2 \
    --test all --message-format=short -- --test-threads=1

run_runtime_stage "app-server selected-snippet experimental opt-in fixture" \
  cargo test --manifest-path "$manifest" -p codex-app-server \
    turn_start_context_recall_selected_snippets_requires_experimental_api_capability \
    --test all --message-format=short

run_runtime_stage "app-server source-aware compression canary list contract" \
  cargo test --manifest-path "$manifest" -p codex-app-server \
    experimental_feature_list_returns_feature_metadata_with_stage \
    --test all --message-format=short

run_runtime_stage "app-server source-aware compression canary enablement contract" \
  cargo test --manifest-path "$manifest" -p codex-app-server \
    experimental_feature_enablement_set_only_updates_named_features \
    --test all --message-format=short

run_runtime_stage "app-server source-aware compression canary no-marker turn-start contract" \
  cargo test --manifest-path "$manifest" -p codex-app-server \
    turn_start_source_aware_compression_canary_without_marker_keeps_selected_snippets_v2 \
    --test all --message-format=short

run_runtime_stage "app-server source-aware compression canary thread-history no-leak contract" \
  cargo test --manifest-path "$manifest" -p codex-app-server \
    turn_start_source_aware_compression_canary_thread_history_hides_routing_metadata \
    --test all --message-format=short

run_stage "response-debug rollout context export contract" \
  cargo test --manifest-path "$manifest" -p codex-response-debug-context \
    rollout_context_debug \
    --all-targets --message-format=short

run_stage "response-debug rollout context truncation evidence fixture" \
  cargo test --manifest-path "$manifest" -p codex-response-debug-context \
    rollout_context_debug_summary_surfaces_truncation_evidence_without_payloads \
    --lib --message-format=short

run_stage "response-debug rollout context compression candidate fixture" \
  cargo test --manifest-path "$manifest" -p codex-response-debug-context \
    rollout_context_debug_summary_surfaces_compression_candidates_without_payloads \
    --lib --message-format=short

run_stage "response-debug rollout context compression stage fixture" \
  cargo test --manifest-path "$manifest" -p codex-response-debug-context \
    rollout_context_debug_summary_surfaces_compression_stages_without_payloads \
    --lib --message-format=short

run_stage "response-debug rollout context executed compression stage matrix fixture" \
  cargo test --manifest-path "$manifest" -p codex-response-debug-context \
    rollout_context_debug_summary_surfaces_executed_compression_stage_matrix_without_payloads \
    --lib --message-format=short

run_stage "context response-debug export gate" \
  "$repo_root/scripts/hepta-context-response-debug-export-gate.sh"

run_stage "context prompt-input gate" \
  "$repo_root/scripts/hepta-context-prompt-input-summary-gate.sh"

run_stage "selected recall summary canary readiness gate" \
  bash "$repo_root/scripts/hepta-context-selected-recall-summary-canary-gate.sh"

run_stage "selected recall summary canary eval replay gate" \
  bash "$repo_root/scripts/hepta-context-selected-recall-summary-canary-eval-gate.sh"

if [ "$skip_runtime_stages" = "1" ]; then
  echo "hepta-context-preflight.scope=non-runtime"
  echo "hepta-context-preflight.runtime-stages=skipped"
  echo "hepta-context-preflight.runtime-activation=disabled"
  echo "Hepta context preflight non-runtime scope passed"
else
  echo "Hepta context preflight passed"
fi
