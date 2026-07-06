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

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

allowed_response_debug_export_paths=(
  "audit"
  "audit.findings"
  "audit.findings.[]"
  "audit.findings.[].code"
  "audit.findings.[].message"
  "audit.findings.[].severity"
  "audit.ok"
  "summary"
  "summary.latest_manifest_adaptive_budget_allocation_budget_classes"
  "summary.latest_manifest_adaptive_budget_allocation_budget_classes.[]"
  "summary.latest_manifest_adaptive_budget_allocation_count"
  "summary.latest_manifest_adaptive_budget_allocation_current_actions"
  "summary.latest_manifest_adaptive_budget_allocation_current_actions.[]"
  "summary.latest_manifest_adaptive_budget_allocation_input_tokens"
  "summary.latest_manifest_adaptive_budget_allocation_invalid"
  "summary.latest_manifest_adaptive_budget_allocation_overflow_tokens"
  "summary.latest_manifest_adaptive_budget_allocation_proposed_actions"
  "summary.latest_manifest_adaptive_budget_allocation_proposed_actions.[]"
  "summary.latest_manifest_adaptive_budget_allocation_proposed_budget_tokens"
  "summary.latest_manifest_adaptive_budget_allocation_reserve_tokens"
  "summary.latest_manifest_adaptive_budget_allocation_schema_version"
  "summary.latest_manifest_adaptive_budget_allocation_sources"
  "summary.latest_manifest_adaptive_budget_allocation_sources.[]"
  "summary.latest_manifest_adaptive_budget_allocation_would_compress_count"
  "summary.latest_manifest_adaptive_budget_allocation_would_drop_count"
  "summary.latest_manifest_budget_exceeded"
  "summary.latest_manifest_budget_tokens"
  "summary.latest_manifest_compression_affected_entries"
  "summary.latest_manifest_compression_candidate_affected_entries"
  "summary.latest_manifest_compression_candidate_count"
  "summary.latest_manifest_compression_candidate_input_tokens"
  "summary.latest_manifest_compression_candidate_invalid"
  "summary.latest_manifest_compression_candidate_output_tokens"
  "summary.latest_manifest_compression_candidate_reasons"
  "summary.latest_manifest_compression_candidate_reasons.[]"
  "summary.latest_manifest_compression_candidate_schema_version"
  "summary.latest_manifest_compression_candidate_sources"
  "summary.latest_manifest_compression_candidate_sources.[]"
  "summary.latest_manifest_compression_candidate_stages"
  "summary.latest_manifest_compression_candidate_stages.[]"
  "summary.latest_manifest_compression_candidate_tiers"
  "summary.latest_manifest_compression_candidate_tiers.[]"
  "summary.latest_manifest_compression_candidate_tokens_saved"
  "summary.latest_manifest_compression_input_tokens"
  "summary.latest_manifest_compression_invalid"
  "summary.latest_manifest_compression_loss_check_statuses"
  "summary.latest_manifest_compression_loss_check_statuses.[]"
  "summary.latest_manifest_compression_output_tokens"
  "summary.latest_manifest_compression_protected_tier_invariants"
  "summary.latest_manifest_compression_protected_tier_invariants.[]"
  "summary.latest_manifest_compression_rollback_source_text_hash_count"
  "summary.latest_manifest_compression_stage_count"
  "summary.latest_manifest_compression_stage_schema_version"
  "summary.latest_manifest_compression_stages"
  "summary.latest_manifest_compression_stages.[]"
  "summary.latest_manifest_compression_tokens_saved"
  "summary.latest_manifest_decision_candidate_omit_count"
  "summary.latest_manifest_decision_candidate_truncate_count"
  "summary.latest_manifest_decision_included_count"
  "summary.latest_manifest_decision_known_count"
  "summary.latest_manifest_decision_omitted_count"
  "summary.latest_manifest_decision_policy_count"
  "summary.latest_manifest_decision_schema_version"
  "summary.latest_manifest_decision_truncated_count"
  "summary.latest_manifest_decision_unknown_count"
  "summary.latest_manifest_estimated_tokens"
  "summary.latest_manifest_memory_taxonomy_available_count"
  "summary.latest_manifest_memory_taxonomy_classes"
  "summary.latest_manifest_memory_taxonomy_classes.[]"
  "summary.latest_manifest_memory_taxonomy_count"
  "summary.latest_manifest_memory_taxonomy_invalid"
  "summary.latest_manifest_memory_taxonomy_omitted_count"
  "summary.latest_manifest_memory_taxonomy_provenance_span_count"
  "summary.latest_manifest_memory_taxonomy_returned_count"
  "summary.latest_manifest_memory_taxonomy_schema_version"
  "summary.latest_manifest_memory_taxonomy_source_count"
  "summary.latest_manifest_memory_formation_receipt_candidate_types"
  "summary.latest_manifest_memory_formation_receipt_candidate_types.[]"
  "summary.latest_manifest_memory_formation_receipt_confidence_basis_points"
  "summary.latest_manifest_memory_formation_receipt_count"
  "summary.latest_manifest_memory_formation_receipt_invalid"
  "summary.latest_manifest_memory_formation_receipt_privacy_classes"
  "summary.latest_manifest_memory_formation_receipt_privacy_classes.[]"
  "summary.latest_manifest_memory_formation_receipt_production_write_count"
  "summary.latest_manifest_memory_formation_receipt_provenance_span_count"
  "summary.latest_manifest_memory_formation_receipt_queued_count"
  "summary.latest_manifest_memory_formation_receipt_schema_version"
  "summary.latest_manifest_memory_formation_receipt_transcript_span_count"
  "summary.latest_manifest_memory_temporal_fact_confidence_basis_points"
  "summary.latest_manifest_memory_temporal_fact_count"
  "summary.latest_manifest_memory_temporal_fact_dry_run_count"
  "summary.latest_manifest_memory_temporal_fact_invalid"
  "summary.latest_manifest_memory_temporal_fact_invalidated_count"
  "summary.latest_manifest_memory_temporal_fact_open_count"
  "summary.latest_manifest_memory_temporal_fact_privacy_classes"
  "summary.latest_manifest_memory_temporal_fact_privacy_classes.[]"
  "summary.latest_manifest_memory_temporal_fact_production_write_count"
  "summary.latest_manifest_memory_temporal_fact_provenance_span_count"
  "summary.latest_manifest_memory_temporal_fact_schema_version"
  "summary.latest_manifest_memory_temporal_fact_supersedes_count"
  "summary.latest_manifest_memory_temporal_fact_types"
  "summary.latest_manifest_memory_temporal_fact_types.[]"
  "summary.latest_manifest_omitted_entries"
  "summary.latest_manifest_present"
  "summary.latest_manifest_recall_low_recency_ranked_item_count"
  "summary.latest_manifest_recall_low_trust_ranked_item_count"
  "summary.latest_manifest_recall_max_per_source"
  "summary.latest_manifest_recall_memory_control_omitted_count"
  "summary.latest_manifest_recall_omitted_by_budget_count"
  "summary.latest_manifest_recall_ranked_item_count"
  "summary.latest_manifest_recall_ranked_source_count"
  "summary.latest_manifest_recall_returned_source_count"
  "summary.latest_manifest_recall_returned_unselected_source_count"
  "summary.latest_manifest_recall_selected_snippet_bounded"
  "summary.latest_manifest_recall_selected_snippet_count"
  "summary.latest_manifest_recall_selected_snippet_max_chars"
  "summary.latest_manifest_recall_selected_snippet_max_snippets"
  "summary.latest_manifest_recall_selected_snippet_omitted_count"
  "summary.latest_manifest_recall_selected_snippet_ready"
  "summary.latest_manifest_recall_selected_snippet_redacted_count"
  "summary.latest_manifest_recall_selected_snippet_truncated_count"
  "summary.latest_manifest_recall_selected_snippets_invalid"
  "summary.latest_manifest_recall_selected_snippets_present"
  "summary.latest_manifest_recall_selected_source_count"
  "summary.latest_manifest_recall_selection_invalid"
  "summary.latest_manifest_recall_selection_present"
  "summary.latest_manifest_recall_source_diversity_met"
  "summary.latest_manifest_recall_source_diversity_target"
  "summary.latest_manifest_sources"
  "summary.latest_manifest_sources.[]"
  "summary.latest_manifest_tiers"
  "summary.latest_manifest_tiers.[]"
  "summary.latest_manifest_truncated"
  "summary.latest_manifest_truncated_decision_count"
  "summary.latest_manifest_truncated_sources"
  "summary.latest_manifest_truncated_sources.[]"
  "summary.latest_manifest_truncation_evidence_invalid"
  "summary.latest_manifest_truncation_evidence_present"
  "summary.latest_manifest_version"
  "summary.line_count"
  "summary.manifest_count"
  "summary.parse_error_count"
  "version"
)

assert_response_debug_export_has_no_payload_keys() {
  local export_path="$1"
  local label="$2"

  if jq -r 'paths | map(tostring) | join(".")' "$export_path" \
    | rg -n '(^|\.)(snippets|snippet_hash|text|text_hash|source|source_id|source_ids|source_lane|source_lanes|replay_key|raw_ranked_payload|raw_ranked_payload_exposed|rank_explanation|rank_explanation_exposed|control_marker|control_marker_exposed|query|query_payload|query_payload_exposed|per_origin_list|per_origin_list_exposed|origin_identifiers_exposed|candidate_text|fact_text|entity_text|entity_hash|supersedes_fact_hash|transcript_text|memory_text|raw_fact|raw_transcript|tool_args|tool_arguments|raw_idempotency_key|idempotency_key|idempotency_key_hash|memory_id|memory_ids|memory_label|topic_id|neuron_id|email|phone|user_identifier|per_source_candidates|per_source_list|memory_formation_candidates|memory_formation_candidate_previews)(\.|$)'; then
    echo "response-debug export exposed payload-shaped key in $label" >&2
    exit 1
  fi
}

assert_response_debug_export_paths_are_allowlisted() {
  local export_path="$1"
  local label="$2"
  local unexpected_paths

  unexpected_paths="$(
    comm -23 \
      <(jq -r 'paths | map(if type == "number" then "[]" else tostring end) | join(".")' "$export_path" | sort -u) \
      <(printf '%s\n' "${allowed_response_debug_export_paths[@]}" | sort -u)
  )"

  if [ -n "$unexpected_paths" ]; then
    echo "response-debug export exposed unexpected JSON path in $label" >&2
    echo "$unexpected_paths" >&2
    exit 1
  fi
}

echo "hepta-context-response-debug-export-gate: lane=$lane"
echo "hepta-context-response-debug-export-gate: CARGO_TARGET_DIR=$CARGO_TARGET_DIR"

cargo test --manifest-path "$manifest" -p codex-protocol \
  turn_context_manifest_recall_selection_serializes_payload_light_rollup \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-protocol \
  turn_context_manifest_selected_snippets_serializes_shadow_envelope \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-protocol \
  turn_context_manifest_compression_candidates_are_payload_light_and_hashed \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-protocol \
  turn_context_manifest_adaptive_budget_allocations_are_payload_light_and_hashed \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-protocol \
  turn_context_manifest_memory_taxonomy_is_payload_light_and_hashed \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-protocol \
  turn_context_manifest_memory_formation_receipts_are_payload_light_and_hashed \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-protocol \
  turn_context_manifest_memory_temporal_facts_are_payload_light_and_hashed \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-protocol \
  turn_context_manifest_compression_stages_are_payload_light_and_hashed \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-response-debug-context \
  rollout_context_debug \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-response-debug-context \
  rollout_context_debug_summary_combines_payload_light_surfaces_without_cross_surface_leaks \
  --lib --message-format=short

cat >"$tmpdir/good.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":3,"budget_tokens":4,"recall_selection":{"returned_source_count":4,"selected_source_count":3,"ranked_source_count":3,"returned_unselected_source_count":1,"source_diversity_met":true,"source_diversity_target":3,"max_per_source":2,"ranked_item_count":3,"omitted_by_budget_count":1,"memory_control_omitted_count":2,"low_trust_ranked_item_count":1,"low_recency_ranked_item_count":2,"source_id":"summary-memory-id-should-not-export"},"recall_selected_snippets":{"version":1,"max_snippets":4,"max_snippet_chars":120,"selected_snippet_count":1,"omitted_snippet_count":2,"redacted_snippet_count":1,"truncated_snippet_count":0,"snippets":[{"snippet_hash":"fedcba9876543210","text":"[redacted-query] top-level selected snippet should not export","estimated_tokens":12,"redacted":true,"truncated":false,"source_id":"top-level-snippet-source-id-should-not-export"}],"safety":{"ready_for_shadow_handoff":true,"bounded":true,"origin_identifiers_exposed":false,"raw_ranked_payload_exposed":false,"rank_explanation_exposed":false,"control_marker_exposed":false,"query_payload_exposed":false,"per_origin_list_exposed":false}},"entries":[{"role":"developer","tier":"system","source":"initial_context:permissions:0","replay_key":"initial_context:permissions:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":3}]}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- <"$tmpdir/good.jsonl" >"$tmpdir/good.json"

assert_response_debug_export_has_no_payload_keys "$tmpdir/good.json" "top-level manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/good.json" "top-level manifest export"

jq -e '
  .version == 1
  and .audit.ok == true
  and .summary.latest_manifest_present == true
  and .summary.latest_manifest_decision_schema_version == 0
  and .summary.latest_manifest_decision_known_count == 0
  and .summary.latest_manifest_decision_unknown_count == 0
  and .summary.latest_manifest_decision_included_count == 0
  and .summary.latest_manifest_decision_policy_count == 0
  and .summary.latest_manifest_decision_candidate_omit_count == 0
  and .summary.latest_manifest_decision_candidate_truncate_count == 0
  and .summary.latest_manifest_decision_omitted_count == 0
  and .summary.latest_manifest_decision_truncated_count == 0
  and .summary.latest_manifest_compression_candidate_schema_version == 0
  and .summary.latest_manifest_compression_candidate_count == 0
  and .summary.latest_manifest_compression_candidate_stages == []
  and .summary.latest_manifest_compression_candidate_tiers == []
  and .summary.latest_manifest_compression_candidate_sources == []
  and .summary.latest_manifest_compression_candidate_reasons == []
  and .summary.latest_manifest_compression_candidate_input_tokens == 0
  and .summary.latest_manifest_compression_candidate_output_tokens == 0
  and .summary.latest_manifest_compression_candidate_tokens_saved == 0
  and .summary.latest_manifest_compression_candidate_affected_entries == 0
  and .summary.latest_manifest_compression_candidate_invalid == false
  and .summary.latest_manifest_adaptive_budget_allocation_schema_version == 0
  and .summary.latest_manifest_adaptive_budget_allocation_count == 0
  and .summary.latest_manifest_adaptive_budget_allocation_sources == []
  and .summary.latest_manifest_adaptive_budget_allocation_budget_classes == []
  and .summary.latest_manifest_adaptive_budget_allocation_current_actions == []
  and .summary.latest_manifest_adaptive_budget_allocation_proposed_actions == []
  and .summary.latest_manifest_adaptive_budget_allocation_input_tokens == 0
  and .summary.latest_manifest_adaptive_budget_allocation_reserve_tokens == 0
  and .summary.latest_manifest_adaptive_budget_allocation_proposed_budget_tokens == 0
  and .summary.latest_manifest_adaptive_budget_allocation_overflow_tokens == 0
  and .summary.latest_manifest_adaptive_budget_allocation_would_drop_count == 0
  and .summary.latest_manifest_adaptive_budget_allocation_would_compress_count == 0
  and .summary.latest_manifest_adaptive_budget_allocation_invalid == false
  and .summary.latest_manifest_compression_stage_schema_version == 0
  and .summary.latest_manifest_compression_stage_count == 0
  and .summary.latest_manifest_compression_stages == []
  and .summary.latest_manifest_compression_input_tokens == 0
  and .summary.latest_manifest_compression_output_tokens == 0
  and .summary.latest_manifest_compression_tokens_saved == 0
  and .summary.latest_manifest_compression_affected_entries == 0
  and .summary.latest_manifest_compression_invalid == false
  and .summary.latest_manifest_tiers == ["system"]
  and .summary.latest_manifest_recall_selection_present == true
  and .summary.latest_manifest_recall_selection_invalid == false
  and .summary.latest_manifest_recall_returned_source_count == 4
  and .summary.latest_manifest_recall_selected_source_count == 3
  and .summary.latest_manifest_recall_ranked_source_count == 3
  and .summary.latest_manifest_recall_returned_unselected_source_count == 1
  and .summary.latest_manifest_recall_source_diversity_met == true
  and .summary.latest_manifest_recall_source_diversity_target == 3
  and .summary.latest_manifest_recall_max_per_source == 2
  and .summary.latest_manifest_recall_ranked_item_count == 3
  and .summary.latest_manifest_recall_omitted_by_budget_count == 1
  and .summary.latest_manifest_recall_memory_control_omitted_count == 2
  and .summary.latest_manifest_recall_low_trust_ranked_item_count == 1
  and .summary.latest_manifest_recall_low_recency_ranked_item_count == 2
  and .summary.latest_manifest_recall_selected_snippets_present == true
  and .summary.latest_manifest_recall_selected_snippets_invalid == false
  and .summary.latest_manifest_recall_selected_snippet_count == 1
  and .summary.latest_manifest_recall_selected_snippet_omitted_count == 2
  and .summary.latest_manifest_recall_selected_snippet_redacted_count == 1
  and .summary.latest_manifest_recall_selected_snippet_truncated_count == 0
  and .summary.latest_manifest_recall_selected_snippet_ready == true
  and .summary.latest_manifest_recall_selected_snippet_bounded == true
' "$tmpdir/good.json" >/dev/null

if grep -q 'summary-memory-id-should-not-export' "$tmpdir/good.json"; then
  echo "response-debug export leaked recall source id" >&2
  exit 1
fi
if grep -q 'top-level selected snippet should not export' "$tmpdir/good.json"; then
  echo "response-debug export leaked selected snippet text" >&2
  exit 1
fi
if grep -q 'top-level-snippet-source-id-should-not-export' "$tmpdir/good.json"; then
  echo "response-debug export leaked selected snippet source id" >&2
  exit 1
fi

cat >"$tmpdir/compression-candidate-good.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":72,"budget_tokens":20,"compression_candidates":[{"kind":"summary","tier":"retrieved_snippets","source_id":"selected_context_recall","input_tokens":40,"estimated_output_tokens":12,"affected_entries":1,"not_executed_reason":"budget_pressure_dry_run","source":"candidate-source-should-not-export","text":"candidate payload should not export"},{"kind":"defragment","tier":"tool","source_id":"available_plugins","input_tokens":12,"estimated_output_tokens":8,"affected_entries":1,"not_executed_reason":"budget_pressure_dry_run","query":"candidate query should not export"}],"entries":[{"role":"developer","tier":"retrieved_snippets","source":"turn_context:developer:selected_context_recall:0","replay_key":"turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":40}]}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/compression-candidate-good.jsonl" >"$tmpdir/compression-candidate-good.json"

assert_response_debug_export_has_no_payload_keys "$tmpdir/compression-candidate-good.json" "compression candidate manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/compression-candidate-good.json" "compression candidate manifest export"

jq -e '
  .audit.ok == true
  and .summary.latest_manifest_compression_candidate_schema_version == 1
  and .summary.latest_manifest_compression_candidate_count == 2
  and .summary.latest_manifest_compression_candidate_stages == ["summary", "defragment"]
  and .summary.latest_manifest_compression_candidate_tiers == ["retrieved_snippets", "tool"]
  and .summary.latest_manifest_compression_candidate_sources == ["selected_context_recall", "available_plugins"]
  and .summary.latest_manifest_compression_candidate_reasons == ["budget_pressure_dry_run"]
  and .summary.latest_manifest_compression_candidate_input_tokens == 52
  and .summary.latest_manifest_compression_candidate_output_tokens == 20
  and .summary.latest_manifest_compression_candidate_tokens_saved == 32
  and .summary.latest_manifest_compression_candidate_affected_entries == 2
  and .summary.latest_manifest_compression_candidate_invalid == false
  and .summary.latest_manifest_compression_stage_count == 0
  and .summary.latest_manifest_compression_stages == []
' "$tmpdir/compression-candidate-good.json" >/dev/null

if grep -q 'candidate-source-should-not-export' "$tmpdir/compression-candidate-good.json"; then
  echo "response-debug export leaked compression candidate raw source" >&2
  exit 1
fi
if grep -q 'candidate payload should not export' "$tmpdir/compression-candidate-good.json"; then
  echo "response-debug export leaked compression candidate payload text" >&2
  exit 1
fi
if grep -q 'candidate query should not export' "$tmpdir/compression-candidate-good.json"; then
  echo "response-debug export leaked compression candidate query" >&2
  exit 1
fi

cat >"$tmpdir/adaptive-budget-allocation-good.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":72,"budget_tokens":20,"adaptive_budget_allocations":[{"tier":"retrieved_snippets","source_id":"selected_context_recall","budget_class":"bounded_recall","input_tokens":40,"reserve_tokens":12,"proposed_budget_tokens":12,"overflow_tokens":28,"omit_priority":50,"compression_kind":"summary","estimated_compressed_tokens":12,"current_heuristic_action":"drop","proposed_action":"compress","would_drop":false,"would_compress":true,"source":"adaptive-source-should-not-export","text":"adaptive allocation payload should not export"},{"tier":"tool","source_id":"available_plugins","budget_class":"tool_inventory","input_tokens":12,"reserve_tokens":8,"proposed_budget_tokens":0,"overflow_tokens":12,"omit_priority":20,"compression_kind":"defragment","estimated_compressed_tokens":8,"current_heuristic_action":"drop","proposed_action":"drop","would_drop":true,"would_compress":false,"query":"adaptive allocation query should not export"}],"entries":[{"role":"developer","tier":"retrieved_snippets","source":"turn_context:developer:selected_context_recall:0","replay_key":"turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":40}]}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/adaptive-budget-allocation-good.jsonl" >"$tmpdir/adaptive-budget-allocation-good.json"

assert_response_debug_export_has_no_payload_keys "$tmpdir/adaptive-budget-allocation-good.json" "adaptive budget allocation manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/adaptive-budget-allocation-good.json" "adaptive budget allocation manifest export"

jq -e '
  .audit.ok == true
  and .summary.latest_manifest_adaptive_budget_allocation_schema_version == 1
  and .summary.latest_manifest_adaptive_budget_allocation_count == 2
  and .summary.latest_manifest_adaptive_budget_allocation_sources == ["selected_context_recall", "available_plugins"]
  and .summary.latest_manifest_adaptive_budget_allocation_budget_classes == ["bounded_recall", "tool_inventory"]
  and .summary.latest_manifest_adaptive_budget_allocation_current_actions == ["drop"]
  and .summary.latest_manifest_adaptive_budget_allocation_proposed_actions == ["compress", "drop"]
  and .summary.latest_manifest_adaptive_budget_allocation_input_tokens == 52
  and .summary.latest_manifest_adaptive_budget_allocation_reserve_tokens == 20
  and .summary.latest_manifest_adaptive_budget_allocation_proposed_budget_tokens == 12
  and .summary.latest_manifest_adaptive_budget_allocation_overflow_tokens == 40
  and .summary.latest_manifest_adaptive_budget_allocation_would_drop_count == 1
  and .summary.latest_manifest_adaptive_budget_allocation_would_compress_count == 1
  and .summary.latest_manifest_adaptive_budget_allocation_invalid == false
  and .summary.latest_manifest_compression_stage_count == 0
  and .summary.latest_manifest_compression_stages == []
' "$tmpdir/adaptive-budget-allocation-good.json" >/dev/null

if grep -q 'adaptive-source-should-not-export' "$tmpdir/adaptive-budget-allocation-good.json"; then
  echo "response-debug export leaked adaptive budget allocation raw source" >&2
  exit 1
fi
if grep -q 'adaptive allocation payload should not export' "$tmpdir/adaptive-budget-allocation-good.json"; then
  echo "response-debug export leaked adaptive budget allocation payload text" >&2
  exit 1
fi
if grep -q 'adaptive allocation query should not export' "$tmpdir/adaptive-budget-allocation-good.json"; then
  echo "response-debug export leaked adaptive budget allocation query" >&2
  exit 1
fi

cat >"$tmpdir/memory-taxonomy-good.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":34,"budget_tokens":24,"memory_taxonomy":[{"class":"semantic","source_count":1,"returned_count":2,"available_count":3,"omitted_count":1,"memory_id":"semantic-memory-id-should-not-export","text":"semantic memory payload should not export"},{"class":"episodic","source_count":1,"returned_count":1,"available_count":1,"omitted_count":0,"source_id":"summary-source-id-should-not-export"},{"class":"control","source_count":1,"returned_count":0,"available_count":2,"omitted_count":2,"query":"control query should not export"},{"class":"transcript","source_count":2,"returned_count":3,"available_count":5,"omitted_count":2,"provenance_span_count":2,"text":"transcript payload should not export"}],"entries":[{"role":"developer","tier":"retrieved_snippets","source":"turn_context:developer:selected_context_recall:0","replay_key":"turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":34}]}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/memory-taxonomy-good.jsonl" >"$tmpdir/memory-taxonomy-good.json"

assert_response_debug_export_has_no_payload_keys "$tmpdir/memory-taxonomy-good.json" "memory taxonomy manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/memory-taxonomy-good.json" "memory taxonomy manifest export"

jq -e '
  .audit.ok == true
  and .summary.latest_manifest_memory_taxonomy_schema_version == 1
  and .summary.latest_manifest_memory_taxonomy_count == 4
  and .summary.latest_manifest_memory_taxonomy_classes == ["semantic", "episodic", "control", "transcript"]
  and .summary.latest_manifest_memory_taxonomy_source_count == 5
  and .summary.latest_manifest_memory_taxonomy_returned_count == 6
  and .summary.latest_manifest_memory_taxonomy_available_count == 11
  and .summary.latest_manifest_memory_taxonomy_omitted_count == 5
  and .summary.latest_manifest_memory_taxonomy_provenance_span_count == 2
  and .summary.latest_manifest_memory_taxonomy_invalid == false
' "$tmpdir/memory-taxonomy-good.json" >/dev/null

if grep -q 'semantic-memory-id-should-not-export' "$tmpdir/memory-taxonomy-good.json"; then
  echo "response-debug export leaked memory taxonomy memory id" >&2
  exit 1
fi
if grep -q 'semantic memory payload should not export' "$tmpdir/memory-taxonomy-good.json"; then
  echo "response-debug export leaked memory taxonomy payload text" >&2
  exit 1
fi
if grep -q 'summary-source-id-should-not-export' "$tmpdir/memory-taxonomy-good.json"; then
  echo "response-debug export leaked memory taxonomy source id" >&2
  exit 1
fi
if grep -q 'control query should not export' "$tmpdir/memory-taxonomy-good.json"; then
  echo "response-debug export leaked memory taxonomy query" >&2
  exit 1
fi
if grep -q 'transcript payload should not export' "$tmpdir/memory-taxonomy-good.json"; then
  echo "response-debug export leaked memory taxonomy transcript text" >&2
  exit 1
fi

cat >"$tmpdir/memory-taxonomy-bad.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":10,"memory_taxonomy":[{"class":"semantic","source_count":1,"returned_count":2,"available_count":3,"omitted_count":0}],"entries":[{"role":"developer","tier":"cross_session_memory","source":"turn_context:developer:memory:0","replay_key":"turn_context:developer:memory:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":10}]}}
JSONL

if cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/memory-taxonomy-bad.jsonl" >"$tmpdir/memory-taxonomy-bad.json"; then
  echo "strict response-debug accepted invalid memory taxonomy" >&2
  exit 1
fi

assert_response_debug_export_has_no_payload_keys "$tmpdir/memory-taxonomy-bad.json" "invalid memory taxonomy export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/memory-taxonomy-bad.json" "invalid memory taxonomy export"

jq -e '
  .audit.ok == false
  and .summary.latest_manifest_memory_taxonomy_invalid == true
  and ([.audit.findings[].code] | index("manifest_memory_taxonomy_invalid") != null)
' "$tmpdir/memory-taxonomy-bad.json" >/dev/null

cat >"$tmpdir/memory-formation-good.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":18,"budget_tokens":24,"memory_formation_receipts":[{"candidate_type":"fact","transcript_span_count":2,"provenance_span_count":2,"confidence_basis_points":6400,"idempotency_key_hash":"0123456789abcdef","privacy_class":"user_private","queued_for_background":true,"transcript_text":"receipt transcript payload should not export","memory_id":"receipt-memory-id-should-not-export"},{"candidate_type":"summary","transcript_span_count":2,"provenance_span_count":1,"confidence_basis_points":7000,"idempotency_key_hash":"fedcba9876543210","privacy_class":"user_private","queued_for_background":true,"source_id":"receipt-source-id-should-not-export","query":"receipt query should not export"}],"entries":[{"role":"developer","tier":"retrieved_snippets","source":"turn_context:developer:selected_context_recall:0","replay_key":"turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":18}]}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/memory-formation-good.jsonl" >"$tmpdir/memory-formation-good.json"

assert_response_debug_export_has_no_payload_keys "$tmpdir/memory-formation-good.json" "memory formation receipt manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/memory-formation-good.json" "memory formation receipt manifest export"

jq -e '
  .audit.ok == true
  and .summary.latest_manifest_memory_formation_receipt_schema_version == 1
  and .summary.latest_manifest_memory_formation_receipt_count == 2
  and .summary.latest_manifest_memory_formation_receipt_candidate_types == ["fact", "summary"]
  and .summary.latest_manifest_memory_formation_receipt_privacy_classes == ["user_private"]
  and .summary.latest_manifest_memory_formation_receipt_transcript_span_count == 4
  and .summary.latest_manifest_memory_formation_receipt_provenance_span_count == 3
  and .summary.latest_manifest_memory_formation_receipt_confidence_basis_points == 13400
  and .summary.latest_manifest_memory_formation_receipt_queued_count == 2
  and .summary.latest_manifest_memory_formation_receipt_production_write_count == 0
  and .summary.latest_manifest_memory_formation_receipt_invalid == false
' "$tmpdir/memory-formation-good.json" >/dev/null

for leaked in \
  "0123456789abcdef" \
  "fedcba9876543210" \
  "receipt transcript payload should not export" \
  "receipt-memory-id-should-not-export" \
  "receipt-source-id-should-not-export" \
  "receipt query should not export"; do
  if grep -q "$leaked" "$tmpdir/memory-formation-good.json"; then
    echo "response-debug export leaked memory formation receipt payload: $leaked" >&2
    exit 1
  fi
done

cat >"$tmpdir/memory-temporal-fact-good.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":18,"budget_tokens":24,"memory_temporal_facts":[{"fact_type":"attribute","entity_hash":"0123456789abcdef","provenance_span_count":2,"valid_from_sequence":8,"confidence_basis_points":6200,"privacy_class":"user_private","dry_run_only":true,"fact_text":"temporal fact payload should not export","transcript_text":"temporal transcript payload should not export","memory_text":"temporal memory payload should not export","source_id":"temporal-source-id-should-not-export","memory_id":"temporal-memory-id-should-not-export","query":"temporal query should not export"},{"fact_type":"summary","entity_hash":"fedcba9876543210","provenance_span_count":1,"valid_from_sequence":9,"invalid_at_sequence":12,"confidence_basis_points":7000,"supersedes_fact_hash":"bbbbbbbbbbbbbbbb","privacy_class":"user_private","dry_run_only":true,"raw_fact":"raw temporal fact should not export","entity_text":"temporal entity text should not export"}],"entries":[{"role":"developer","tier":"retrieved_snippets","source":"turn_context:developer:selected_context_recall:0","replay_key":"turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":18}]}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/memory-temporal-fact-good.jsonl" >"$tmpdir/memory-temporal-fact-good.json"

assert_response_debug_export_has_no_payload_keys "$tmpdir/memory-temporal-fact-good.json" "memory temporal fact manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/memory-temporal-fact-good.json" "memory temporal fact manifest export"

jq -e '
  .audit.ok == true
  and .summary.latest_manifest_memory_temporal_fact_schema_version == 1
  and .summary.latest_manifest_memory_temporal_fact_count == 2
  and .summary.latest_manifest_memory_temporal_fact_types == ["attribute", "summary"]
  and .summary.latest_manifest_memory_temporal_fact_privacy_classes == ["user_private"]
  and .summary.latest_manifest_memory_temporal_fact_provenance_span_count == 3
  and .summary.latest_manifest_memory_temporal_fact_confidence_basis_points == 13200
  and .summary.latest_manifest_memory_temporal_fact_open_count == 1
  and .summary.latest_manifest_memory_temporal_fact_invalidated_count == 1
  and .summary.latest_manifest_memory_temporal_fact_supersedes_count == 1
  and .summary.latest_manifest_memory_temporal_fact_dry_run_count == 2
  and .summary.latest_manifest_memory_temporal_fact_production_write_count == 0
  and .summary.latest_manifest_memory_temporal_fact_invalid == false
' "$tmpdir/memory-temporal-fact-good.json" >/dev/null

for leaked in \
  "0123456789abcdef" \
  "fedcba9876543210" \
  "bbbbbbbbbbbbbbbb" \
  "temporal fact payload should not export" \
  "temporal transcript payload should not export" \
  "temporal memory payload should not export" \
  "temporal-source-id-should-not-export" \
  "temporal-memory-id-should-not-export" \
  "temporal query should not export" \
  "raw temporal fact should not export" \
  "temporal entity text should not export"; do
  if grep -q "$leaked" "$tmpdir/memory-temporal-fact-good.json"; then
    echo "response-debug export leaked memory temporal fact payload: $leaked" >&2
    exit 1
  fi
done

cat >"$tmpdir/memory-temporal-fact-bad.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":10,"memory_temporal_facts":[{"fact_type":"attribute","entity_hash":"0123456789abcdef","provenance_span_count":2,"valid_from_sequence":8,"confidence_basis_points":6200,"supersedes_fact_hash":"raw-fact-id","privacy_class":"user_private","dry_run_only":true}],"entries":[{"role":"developer","tier":"retrieved_snippets","source":"turn_context:developer:selected_context_recall:0","replay_key":"turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":10}]}}
JSONL

if cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/memory-temporal-fact-bad.jsonl" >"$tmpdir/memory-temporal-fact-bad.json"; then
  echo "strict response-debug accepted invalid memory temporal facts" >&2
  exit 1
fi

assert_response_debug_export_has_no_payload_keys "$tmpdir/memory-temporal-fact-bad.json" "invalid memory temporal fact export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/memory-temporal-fact-bad.json" "invalid memory temporal fact export"

jq -e '
  .audit.ok == false
  and .summary.latest_manifest_memory_temporal_fact_invalid == true
  and ([.audit.findings[].code] | index("manifest_memory_temporal_facts_invalid") != null)
' "$tmpdir/memory-temporal-fact-bad.json" >/dev/null

cat >"$tmpdir/memory-formation-candidate-preview-good.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":18,"budget_tokens":24,"memory_formation_receipts":[{"candidate_type":"fact","transcript_span_count":2,"provenance_span_count":2,"confidence_basis_points":6400,"idempotency_key_hash":"0123456789abcdef","privacy_class":"user_private","queued_for_background":true}],"memory_formation_candidate_previews":[{"candidate_type":"fact","candidate_text":"candidate fact payload should not export","transcript_text":"candidate transcript payload should not export","memory_text":"candidate memory payload should not export","tool_args":{"command":"candidate tool args should not export"},"raw_idempotency_key":"raw-idempotency-key-should-not-export","idempotency_key":"idempotency-key-should-not-export","idempotency_key_hash":"candidate-preview-hash-should-not-export","source_id":"candidate-preview-source-id-should-not-export","source_ids":["candidate-preview-source-list-should-not-export"],"memory_id":"candidate-preview-memory-id-should-not-export","memory_ids":["candidate-preview-memory-list-should-not-export"],"per_source_candidates":[{"source_id":"candidate-preview-per-source-id-should-not-export"}],"email":"candidate-email@example.invalid","phone":"+15550101010","user_identifier":"candidate-user-identifier-should-not-export"}],"memory_formation_candidates":[{"candidate_text":"future candidate payload should not export","raw_transcript":"future raw transcript should not export","tool_arguments":"future tool arguments should not export"}],"entries":[{"role":"developer","tier":"retrieved_snippets","source":"turn_context:developer:selected_context_recall:0","replay_key":"turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":18}]}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/memory-formation-candidate-preview-good.jsonl" >"$tmpdir/memory-formation-candidate-preview-good.json"

assert_response_debug_export_has_no_payload_keys "$tmpdir/memory-formation-candidate-preview-good.json" "memory formation candidate preview export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/memory-formation-candidate-preview-good.json" "memory formation candidate preview export"

jq -e '
  .audit.ok == true
  and .summary.latest_manifest_memory_formation_receipt_schema_version == 1
  and .summary.latest_manifest_memory_formation_receipt_count == 1
  and .summary.latest_manifest_memory_formation_receipt_candidate_types == ["fact"]
  and .summary.latest_manifest_memory_formation_receipt_privacy_classes == ["user_private"]
  and .summary.latest_manifest_memory_formation_receipt_transcript_span_count == 2
  and .summary.latest_manifest_memory_formation_receipt_provenance_span_count == 2
  and .summary.latest_manifest_memory_formation_receipt_confidence_basis_points == 6400
  and .summary.latest_manifest_memory_formation_receipt_queued_count == 1
  and .summary.latest_manifest_memory_formation_receipt_production_write_count == 0
  and .summary.latest_manifest_memory_formation_receipt_invalid == false
' "$tmpdir/memory-formation-candidate-preview-good.json" >/dev/null

for leaked in \
  "memory_formation_candidate_previews" \
  "memory_formation_candidates" \
  "candidate fact payload should not export" \
  "candidate transcript payload should not export" \
  "candidate memory payload should not export" \
  "candidate tool args should not export" \
  "raw-idempotency-key-should-not-export" \
  "idempotency-key-should-not-export" \
  "candidate-preview-hash-should-not-export" \
  "candidate-preview-source-id-should-not-export" \
  "candidate-preview-source-list-should-not-export" \
  "candidate-preview-memory-id-should-not-export" \
  "candidate-preview-memory-list-should-not-export" \
  "candidate-preview-per-source-id-should-not-export" \
  "candidate-email@example.invalid" \
  "+15550101010" \
  "candidate-user-identifier-should-not-export" \
  "future candidate payload should not export" \
  "future raw transcript should not export" \
  "future tool arguments should not export"; do
  if grep -q "$leaked" "$tmpdir/memory-formation-candidate-preview-good.json"; then
    echo "response-debug export leaked memory formation candidate preview payload: $leaked" >&2
    exit 1
  fi
done

cat >"$tmpdir/memory-formation-bad.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":10,"memory_formation_receipts":[{"candidate_type":"fact","transcript_span_count":2,"provenance_span_count":2,"confidence_basis_points":6400,"idempotency_key_hash":"0123456789abcdef","privacy_class":"user_private","queued_for_background":true,"production_write":true}],"entries":[{"role":"developer","tier":"retrieved_snippets","source":"turn_context:developer:selected_context_recall:0","replay_key":"turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":10}]}}
JSONL

if cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/memory-formation-bad.jsonl" >"$tmpdir/memory-formation-bad.json"; then
  echo "strict response-debug accepted invalid memory formation receipt" >&2
  exit 1
fi

assert_response_debug_export_has_no_payload_keys "$tmpdir/memory-formation-bad.json" "invalid memory formation receipt export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/memory-formation-bad.json" "invalid memory formation receipt export"

jq -e '
  .audit.ok == false
  and .summary.latest_manifest_memory_formation_receipt_invalid == true
  and .summary.latest_manifest_memory_formation_receipt_production_write_count == 1
  and ([.audit.findings[].code] | index("manifest_memory_formation_receipts_invalid") != null)
' "$tmpdir/memory-formation-bad.json" >/dev/null

cat >"$tmpdir/compression-good.jsonl" <<'JSONL'
{"type":"response_item","payload":{"type":"message","role":"developer","content":[{"type":"input_text","text":"[context summarized for budget] rollout prompt text should not export"}],"source_id":"response-item-source-id-should-not-export","turn_context_policy_canary_feature":"source_aware_compression_canary","turn_context_policy_opt_in_marker":"TurnContextAssemblyPolicyOptIn::SourceAwareCompression","turn_context_policy_opt_in_value":"SourceAwareCompression","source_aware_compression_negative_matrix_case":"missing-route+canary+helper-marker+approval-evidence","source_aware_compression_operator_approval_evidence":"response item evidence bait should not export"}}
{"type":"turn_context","payload":{"model":"gpt-test","turn_context_policy_canary_feature":"source_aware_compression_canary","turn_context_policy_opt_in_marker":"TurnContextAssemblyPolicyOptIn::SourceAwareCompression","turn_context_policy_opt_in_value":"SourceAwareCompression","source_aware_compression_negative_matrix_cases":["missing-route","missing-canary","missing-helper-marker","missing-approval-evidence","missing-route+canary","missing-route+helper-marker","missing-route+approval-evidence","missing-canary+helper-marker","missing-canary+approval-evidence","missing-helper-marker+approval-evidence","missing-route+canary+helper-marker","missing-route+canary+approval-evidence","missing-route+helper-marker+approval-evidence","missing-canary+helper-marker+approval-evidence","missing-route+canary+helper-marker+approval-evidence"],"source_aware_compression_operator_approval_evidence":{"type":"SourceAwareCompressionOperatorApprovalEvidence","source_aware_compression_operator_approval_id":"approval-id-should-not-export","source_aware_compression_operator_identity_hash":"operator-identity-hash-should-not-export","source_aware_compression_activation_request_id":"activation-request-id-should-not-export","source_aware_compression_operator_approval_scope_hash":"scope-hash-should-not-export","source_aware_compression_operator_approval_nonce":"nonce-should-not-export","source_aware_compression_operator_approval_expires_at":"expiry-should-not-export"},"context_manifest":{"version":1,"estimated_tokens":43,"budget_tokens":20,"turn_context_policy_canary_feature":"source_aware_compression_canary","turn_context_policy_opt_in_marker":"TurnContextAssemblyPolicyOptIn::SourceAwareCompression","turn_context_policy_opt_in_value":"SourceAwareCompression","source_aware_compression_negative_matrix_case":"missing-route+canary+helper-marker+approval-evidence","source_aware_compression_operator_approval_evidence":"manifest evidence bait should not export","compression_stages":[{"kind":"summary","input_tokens":40,"output_tokens":12,"affected_entries":1,"loss_check_status":"marker_boundary_only","rollback_source_text_hash":"bbbbbbbbbbbbbbbb","protected_tier_invariant":"preserved","source_id":"summary-stage-source-id-should-not-export","replay_key":"summary-stage-replay-key-should-not-export","text_hash":"summary-stage-text-hash-should-not-export","text":"[context summarized for budget] stage payload should not export"},{"kind":"defragment","input_tokens":30,"output_tokens":21,"affected_entries":1,"loss_check_status":"marker_boundary_only","rollback_source_text_hash":"cccccccccccccccc","protected_tier_invariant":"preserved","source_id":"defragment-stage-source-id-should-not-export","text":"[context defragmented for budget] stage payload should not export"},{"kind":"prune","input_tokens":12,"output_tokens":10,"affected_entries":1,"loss_check_status":"marker_boundary_only","rollback_source_text_hash":"dddddddddddddddd","protected_tier_invariant":"preserved","query":"prune stage query should not export","text":"[context pruned for budget] stage payload should not export"}],"entries":[{"role":"developer","tier":"tool","source":"turn_context:developer:compression_stage_matrix:0","replay_key":"turn_context:developer:compression_stage_matrix:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":43}]}}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/compression-good.jsonl" >"$tmpdir/compression-good.json"

assert_response_debug_export_has_no_payload_keys "$tmpdir/compression-good.json" "compression manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/compression-good.json" "compression manifest export"

jq -e '
  .audit.ok == true
  and .summary.line_count == 2
  and .summary.manifest_count == 1
  and .summary.latest_manifest_compression_stage_schema_version == 2
  and .summary.latest_manifest_compression_stage_count == 3
  and .summary.latest_manifest_compression_stages == ["summary", "defragment", "prune"]
  and .summary.latest_manifest_compression_loss_check_statuses == ["marker_boundary_only"]
  and .summary.latest_manifest_compression_rollback_source_text_hash_count == 3
  and .summary.latest_manifest_compression_protected_tier_invariants == ["preserved"]
  and .summary.latest_manifest_compression_input_tokens == 82
  and .summary.latest_manifest_compression_output_tokens == 43
  and .summary.latest_manifest_compression_tokens_saved == 39
  and .summary.latest_manifest_compression_affected_entries == 3
  and .summary.latest_manifest_compression_invalid == false
' "$tmpdir/compression-good.json" >/dev/null

if grep -q 'response-item-source-id-should-not-export' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked response item source id" >&2
  exit 1
fi
if grep -q 'rollout prompt text should not export' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked response item prompt text" >&2
  exit 1
fi
if grep -q 'source_aware_compression_canary' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked source-aware compression canary feature key" >&2
  exit 1
fi
if grep -q 'TurnContextAssemblyPolicyOptIn' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked source-aware compression opt-in marker type" >&2
  exit 1
fi
if grep -q 'SourceAwareCompression' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked source-aware compression opt-in marker value" >&2
  exit 1
fi
source_aware_compression_negative_matrix_and_evidence_bait=(
  "missing-route"
  "missing-canary"
  "missing-helper-marker"
  "missing-approval-evidence"
  "missing-route+canary"
  "missing-route+helper-marker"
  "missing-route+approval-evidence"
  "missing-canary+helper-marker"
  "missing-canary+approval-evidence"
  "missing-helper-marker+approval-evidence"
  "missing-route+canary+helper-marker"
  "missing-route+canary+approval-evidence"
  "missing-route+helper-marker+approval-evidence"
  "missing-canary+helper-marker+approval-evidence"
  "missing-route+canary+helper-marker+approval-evidence"
  "source_aware_compression_operator_approval_evidence"
  "SourceAwareCompressionOperatorApprovalEvidence"
  "source_aware_compression_operator_approval_id"
  "source_aware_compression_operator_identity_hash"
  "source_aware_compression_activation_request_id"
  "source_aware_compression_operator_approval_scope_hash"
  "source_aware_compression_operator_approval_nonce"
  "source_aware_compression_operator_approval_expires_at"
  "approval-id-should-not-export"
  "operator-identity-hash-should-not-export"
  "activation-request-id-should-not-export"
  "scope-hash-should-not-export"
  "nonce-should-not-export"
  "expiry-should-not-export"
)
for bait in "${source_aware_compression_negative_matrix_and_evidence_bait[@]}"; do
  if grep -q "$bait" "$tmpdir/compression-good.json"; then
    echo "response-debug export leaked source-aware compression negative-matrix/operator evidence bait: $bait" >&2
    exit 1
  fi
done
if grep -q 'summary-stage-source-id-should-not-export' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked compression source id" >&2
  exit 1
fi
if grep -q 'summary-stage-replay-key-should-not-export' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked compression replay key" >&2
  exit 1
fi
if grep -q 'summary-stage-text-hash-should-not-export' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked compression text hash" >&2
  exit 1
fi
if grep -q 'defragment-stage-source-id-should-not-export' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked defragment source id" >&2
  exit 1
fi
if grep -q 'stage payload should not export' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked compression payload text" >&2
  exit 1
fi
if grep -q 'prune stage query should not export' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked compression query" >&2
  exit 1
fi
for rollback_hash_bait in bbbbbbbbbbbbbbbb cccccccccccccccc dddddddddddddddd; do
  if grep -q "$rollback_hash_bait" "$tmpdir/compression-good.json"; then
    echo "response-debug export leaked compression rollback hash: $rollback_hash_bait" >&2
    exit 1
  fi
done
if grep -q '\[context \(summarized\|defragmented\|pruned\) for budget\]' "$tmpdir/compression-good.json"; then
  echo "response-debug export leaked compression marker text" >&2
  exit 1
fi

cat >"$tmpdir/nested-good.jsonl" <<'JSONL'
{"type":"turn_context","payload":{"model":"gpt-test","context_manifest":{"version":1,"estimated_tokens":3,"recall_selection":{"returned_source_count":2,"selected_source_count":2,"ranked_source_count":0,"returned_unselected_source_count":0,"source_diversity_met":true,"source_diversity_target":2,"max_per_source":2,"ranked_item_count":0,"memory_control_omitted_count":1,"low_trust_ranked_item_count":0,"low_recency_ranked_item_count":0,"source_id":"nested-source-id-should-not-export"},"recall_selected_snippets":{"version":1,"max_snippets":4,"max_snippet_chars":120,"selected_snippet_count":1,"omitted_snippet_count":0,"redacted_snippet_count":1,"truncated_snippet_count":0,"snippets":[{"snippet_hash":"fedcba9876543210","text":"[redacted-query] nested selected snippet should not export","estimated_tokens":12,"redacted":true,"truncated":false,"source_id":"nested-snippet-source-id-should-not-export"}],"safety":{"ready_for_shadow_handoff":true,"bounded":true,"origin_identifiers_exposed":false,"raw_ranked_payload_exposed":false,"rank_explanation_exposed":false,"control_marker_exposed":false,"query_payload_exposed":false,"per_origin_list_exposed":false}},"entries":[{"role":"developer","source":"turn_context:developer:0","replay_key":"turn_context:developer:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":3}]}}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/nested-good.jsonl" >"$tmpdir/nested-good.json"

assert_response_debug_export_has_no_payload_keys "$tmpdir/nested-good.json" "nested manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/nested-good.json" "nested manifest export"

jq -e '
  .audit.ok == true
  and .summary.manifest_count == 1
  and .summary.latest_manifest_present == true
  and .summary.latest_manifest_sources == ["turn_context:developer:0"]
  and .summary.latest_manifest_recall_selection_present == true
  and .summary.latest_manifest_recall_selected_source_count == 2
  and .summary.latest_manifest_recall_memory_control_omitted_count == 1
  and .summary.latest_manifest_recall_selected_snippets_present == true
  and .summary.latest_manifest_recall_selected_snippet_count == 1
  and .summary.latest_manifest_recall_selected_snippet_omitted_count == 0
' "$tmpdir/nested-good.json" >/dev/null

if grep -q 'nested-source-id-should-not-export' "$tmpdir/nested-good.json"; then
  echo "response-debug export leaked nested recall source id" >&2
  exit 1
fi
if grep -q 'nested selected snippet should not export' "$tmpdir/nested-good.json"; then
  echo "response-debug export leaked nested selected snippet text" >&2
  exit 1
fi
if grep -q 'nested-snippet-source-id-should-not-export' "$tmpdir/nested-good.json"; then
  echo "response-debug export leaked nested selected snippet source id" >&2
  exit 1
fi

cat >"$tmpdir/unranked-good.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":3,"budget_tokens":4,"recall_selection":{"returned_source_count":2,"selected_source_count":2,"ranked_source_count":0,"returned_unselected_source_count":0,"source_diversity_met":true,"source_diversity_target":2,"max_per_source":2,"ranked_item_count":0,"omitted_by_budget_count":0,"memory_control_omitted_count":0,"low_trust_ranked_item_count":0,"low_recency_ranked_item_count":0},"entries":[{"role":"developer","source":"initial_context:permissions:0","replay_key":"initial_context:permissions:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":3}]}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/unranked-good.jsonl" >"$tmpdir/unranked-good.json"

assert_response_debug_export_has_no_payload_keys "$tmpdir/unranked-good.json" "unranked manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/unranked-good.json" "unranked manifest export"

jq -e '
  .audit.ok == true
  and .summary.latest_manifest_recall_selected_source_count == 2
  and .summary.latest_manifest_recall_ranked_source_count == 0
  and .summary.latest_manifest_recall_ranked_item_count == 0
  and .summary.latest_manifest_recall_source_diversity_target == 2
  and .summary.latest_manifest_recall_max_per_source == 2
  and .summary.latest_manifest_recall_omitted_by_budget_count == 0
  and .summary.latest_manifest_recall_memory_control_omitted_count == 0
  and .summary.latest_manifest_recall_selected_snippets_present == false
  and .summary.latest_manifest_recall_selected_snippet_count == 0
' "$tmpdir/unranked-good.json" >/dev/null

cat >"$tmpdir/truncated-good.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":3,"budget_tokens":4,"truncated":true,"decision_ledger":[{"source":"turn_context:developer:selected_context_recall:0","decision":"included:recall_selected_snippets","reason_hash":"aaaaaaaaaaaaaaaa"},{"source":"turn_context:developer:selected_context_recall:0","decision":"truncated:selected_context_recall:original_tokens:24:tokens:3","reason_hash":"bbbbbbbbbbbbbbbb"}],"entries":[{"role":"developer","source":"turn_context:developer:selected_context_recall:0","replay_key":"turn_context:developer:selected_context_recall:0:cccccccccccccccc","text_hash":"cccccccccccccccc","estimated_tokens":3}]}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/truncated-good.jsonl" >"$tmpdir/truncated-good.json"

assert_response_debug_export_has_no_payload_keys "$tmpdir/truncated-good.json" "truncated manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/truncated-good.json" "truncated manifest export"

jq -e '
  .audit.ok == true
  and .summary.latest_manifest_truncated == true
  and .summary.latest_manifest_decision_schema_version == 1
  and .summary.latest_manifest_decision_known_count == 2
  and .summary.latest_manifest_decision_unknown_count == 0
  and .summary.latest_manifest_decision_included_count == 1
  and .summary.latest_manifest_decision_policy_count == 0
  and .summary.latest_manifest_decision_candidate_omit_count == 0
  and .summary.latest_manifest_decision_candidate_truncate_count == 0
  and .summary.latest_manifest_decision_omitted_count == 0
  and .summary.latest_manifest_decision_truncated_count == 1
  and .summary.latest_manifest_truncated_decision_count == 1
  and .summary.latest_manifest_truncated_sources == ["turn_context:developer:selected_context_recall:0"]
  and .summary.latest_manifest_truncation_evidence_present == true
  and .summary.latest_manifest_truncation_evidence_invalid == false
' "$tmpdir/truncated-good.json" >/dev/null

if grep -q 'truncated:selected_context_recall' "$tmpdir/truncated-good.json"; then
  echo "response-debug export leaked raw truncation decision text" >&2
  exit 1
fi
if grep -q 'cccccccccccccccc' "$tmpdir/truncated-good.json"; then
  echo "response-debug export leaked truncated manifest text hash" >&2
  exit 1
fi

cat >"$tmpdir/bad.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":3,"budget_tokens":4,"recall_selection":{"returned_source_count":2,"selected_source_count":2,"ranked_source_count":0,"returned_unselected_source_count":0,"source_diversity_met":false,"source_diversity_target":2,"max_per_source":2,"ranked_item_count":0,"omitted_by_budget_count":0,"memory_control_omitted_count":0,"low_trust_ranked_item_count":0,"low_recency_ranked_item_count":0},"recall_selected_snippets":{"version":1,"max_snippets":4,"max_snippet_chars":120,"selected_snippet_count":2,"omitted_snippet_count":0,"redacted_snippet_count":1,"truncated_snippet_count":0,"snippets":[{"snippet_hash":"fedcba9876543210","text":"[redacted-query] invalid selected snippet should not export","estimated_tokens":12,"redacted":true,"truncated":false}],"safety":{"ready_for_shadow_handoff":true,"bounded":true,"origin_identifiers_exposed":false,"raw_ranked_payload_exposed":false,"rank_explanation_exposed":false,"control_marker_exposed":false,"query_payload_exposed":false,"per_origin_list_exposed":false}},"entries":[{"role":"developer","source":"initial_context:permissions:0","replay_key":"initial_context:permissions:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":3}]}}
JSONL

if cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/bad.jsonl" >"$tmpdir/bad.json"; then
  echo "strict response-debug accepted invalid recall-selection rollup" >&2
  exit 1
fi

assert_response_debug_export_has_no_payload_keys "$tmpdir/bad.json" "invalid manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/bad.json" "invalid manifest export"

jq -e '
  .audit.ok == false
  and .summary.latest_manifest_recall_selection_invalid == true
  and .summary.latest_manifest_recall_selected_snippets_invalid == true
  and ([.audit.findings[].code] | index("manifest_recall_selection_invalid") != null)
  and ([.audit.findings[].code] | index("manifest_recall_selected_snippets_invalid") != null)
' "$tmpdir/bad.json" >/dev/null

if grep -q 'invalid selected snippet should not export' "$tmpdir/bad.json"; then
  echo "response-debug export leaked invalid selected snippet text" >&2
  exit 1
fi

cat >"$tmpdir/truncated-bad.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":3,"budget_tokens":4,"truncated":true,"entries":[{"role":"developer","source":"turn_context:developer:selected_context_recall:0","replay_key":"turn_context:developer:selected_context_recall:0:cccccccccccccccc","text_hash":"cccccccccccccccc","estimated_tokens":3}]}}
JSONL

if cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict <"$tmpdir/truncated-bad.jsonl" >"$tmpdir/truncated-bad.json"; then
  echo "strict response-debug accepted truncated manifest without evidence" >&2
  exit 1
fi

assert_response_debug_export_has_no_payload_keys "$tmpdir/truncated-bad.json" "invalid truncated manifest export"
assert_response_debug_export_paths_are_allowlisted "$tmpdir/truncated-bad.json" "invalid truncated manifest export"

jq -e '
  .audit.ok == false
  and .summary.latest_manifest_truncated == true
  and .summary.latest_manifest_truncated_decision_count == 0
  and .summary.latest_manifest_truncation_evidence_present == false
  and ([.audit.findings[].code] | index("manifest_truncation_evidence_missing") != null)
' "$tmpdir/truncated-bad.json" >/dev/null

echo "response-debug-export=pass"
echo "response-debug-export.payload-light=pass"
echo "response-debug-export.combined-surfaces=no-leak"
echo "response-debug-export.strict-invalid=reject"
echo "response-debug-export.runtime-activation=disabled"
echo "Hepta context response-debug export gate passed"
