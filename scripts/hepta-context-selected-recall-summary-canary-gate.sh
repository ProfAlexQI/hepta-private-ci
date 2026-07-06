#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
report_script="$repo_root/scripts/hepta-context-selected-recall-summary-canary-report.sh"
gate_script="$repo_root/scripts/hepta-context-selected-recall-summary-canary-gate.sh"
selected_recall_controller="$repo_root/codex-rs/core/src/context_manager/manifest/selected_recall.rs"
manifest_module="$repo_root/codex-rs/core/src/context_manager/manifest.rs"
manifest_options="$repo_root/codex-rs/core/src/context_manager/manifest/options.rs"
manifest_tests="$repo_root/codex-rs/core/src/context_manager/manifest/tests.rs"
report_output="$(mktemp -t hepta-context-selected-recall-summary-canary-report.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-selected-recall-summary-canary-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "selected recall summary canary report output:" >&2
    cat "$report_output" >&2
  fi
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

assert_report_line() {
  local expected="$1"
  if ! grep -F -x "$expected" "$report_output" >/dev/null; then
    fail "selected recall summary canary report must contain line: $expected"
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

bash "$report_script" >"$report_output"

assert_report_line "selected-recall-summary-canary=pass"
assert_report_line "selected-recall-summary-canary.schema=1"
assert_report_line "selected-recall-summary-canary.mode=shadow-readiness"
assert_report_line "selected-recall-summary-canary.redaction=guarded_envelope"
assert_report_line "selected-recall-summary-canary.quality=recall_quality"
assert_report_line "selected-recall-summary-canary.activation-guard=operator_approval_required"
assert_report_line "selected-recall-summary-canary.rollback=rerun_recall"
assert_report_line "selected-recall-summary-canary.compression=summary"
assert_report_line "selected-recall-summary-canary.metrics.shadow-vs-live=required"
assert_report_line "selected-recall-summary-canary.metrics.token-saved=required"
assert_report_line "selected-recall-summary-canary.metrics.latency-delta=required"
assert_report_line "selected-recall-summary-canary.metrics.quality-delta=required"
assert_report_line "selected-recall-summary-canary.rollback-readback=required"
assert_report_line "selected-recall-summary-canary.prompt-input-proof=required"
assert_report_line "selected-recall-summary-canary.response-debug-proof=payload-light"
assert_report_line "selected-recall-summary-canary.readback.prompt-input=manifest-no-leak"
assert_report_line "selected-recall-summary-canary.readback.response-debug=payload-light-summary"
assert_report_line "selected-recall-summary-canary.readback.rollback=fixture-covered"
assert_report_line "selected-recall-summary-canary.readback.rollback-hash=omitted"
assert_report_line "selected-recall-summary-canary.controller-readback.prompt-input.manifest-consumed=covered"
assert_report_line "selected-recall-summary-canary.controller-readback.prompt-input.shadow-metadata=omitted"
assert_report_line "selected-recall-summary-canary.controller-readback.prompt-input.live-selected-snippet=guarded"
assert_report_line "selected-recall-summary-canary.controller-readback.response-debug.manifest-summary=covered"
assert_report_line "selected-recall-summary-canary.controller-readback.response-debug.payload-light-summary=covered"
assert_report_line "selected-recall-summary-canary.controller-readback.rollback.fixture=covered"
assert_report_line "selected-recall-summary-canary.controller-readback.rollback.hash=omitted"
assert_report_line "selected-recall-summary-canary.operator-approval=required"
assert_report_line "selected-recall-summary-canary.production-route=disabled"
assert_report_line "selected-recall-summary-canary.runtime-activation=disabled"

for term in \
  "Selected-Recall Summary Canary Readiness Gate" \
  "hepta-context-selected-recall-summary-canary-report.sh" \
  "hepta-context-selected-recall-summary-canary-gate.sh" \
  "shadow-vs-live" \
  "token-saved" \
  "latency-delta" \
  "quality-delta" \
  "rollback-readback" \
  "prompt-input-proof" \
  "response-debug-proof" \
  "selected_recall.rs" \
  "SelectedRecallControllerDecision" \
  "SelectedRecallControllerCanaryReadiness" \
  "SelectedRecallControllerCanaryMetrics" \
  "SelectedRecallControllerReadbackProofs" \
  "SelectedRecallControllerReadbackProof" \
  "SelectedRecallControllerReadbackSurface" \
  "controller-readback" \
  "operator approval"; do
  assert_file_contains "$contracts" "$term" "selected-recall summary canary contract"
done

assert_file_contains "$selected_recall_controller" \
  "SelectedRecallControllerDecision" \
  "selected-recall controller decision"
assert_file_contains "$selected_recall_controller" \
  "selected_recall_controller_decision_from_extension_data" \
  "selected-recall controller extension data reader"
assert_file_contains "$selected_recall_controller" \
  "apply_selected_recall_controller_decision" \
  "selected-recall controller manifest apply function"
assert_file_contains "$selected_recall_controller" \
  "selected_snippet_envelope_is_manifest_safe" \
  "selected-recall controller safety gate"
assert_file_contains "$selected_recall_controller" \
  "SelectedRecallControllerCanaryReadiness" \
  "selected-recall controller canary readiness"
assert_file_contains "$selected_recall_controller" \
  "SelectedRecallControllerCanaryMetrics" \
  "selected-recall controller canary metrics"
assert_file_contains "$selected_recall_controller" \
  "SelectedRecallControllerReadbackProofs" \
  "selected-recall controller readback proofs"
assert_file_contains "$selected_recall_controller" \
  "SelectedRecallControllerReadbackProof" \
  "selected-recall controller readback proof"
assert_file_contains "$selected_recall_controller" \
  "SelectedRecallControllerReadbackSurface" \
  "selected-recall controller readback surface"
for term in \
  "shadow_vs_live_required" \
  "token_saved_metric_required" \
  "latency_delta_metric_required" \
  "quality_delta_metric_required" \
  "rollback_readback_required" \
  "prompt_input_proof_required" \
  "response_debug_proof_payload_light" \
  "operator_approval_required" \
  "production_route_enabled: false" \
  "runtime_activation_enabled: false" \
  "has_payload_light_integrity"; do
  assert_file_contains "$selected_recall_controller" "$term" \
    "selected-recall controller canary readiness"
done
for term in \
  "SELECTED_RECALL_CONTROLLER_TOKEN_SAVED_MIN_BASIS_POINTS" \
  "SELECTED_RECALL_CONTROLLER_LATENCY_DELTA_MAX_MS" \
  "SELECTED_RECALL_CONTROLLER_QUALITY_DELTA_MIN_BASIS_POINTS" \
  "SELECTED_RECALL_CONTROLLER_ROLLBACK_READBACK_FIXTURE_COUNT" \
  "token_saved_min_basis_points" \
  "latency_delta_max_ms" \
  "quality_delta_min_basis_points" \
  "rollback_readback_fixture_count" \
  "prompt_input_proof_covered" \
  "response_debug_proof_payload_light" \
  "production_route_enabled: false" \
  "runtime_activation_enabled: false"; do
  assert_file_contains "$selected_recall_controller" "$term" \
    "selected-recall controller canary metrics"
done
for term in \
  "SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_MANIFEST_CONSUMED_PROOF" \
  "SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_SHADOW_METADATA_OMITTED_PROOF" \
  "SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_LIVE_SNIPPET_GUARDED_PROOF" \
  "SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_MANIFEST_SUMMARY_PROOF" \
  "SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_PAYLOAD_LIGHT_PROOF" \
  "SELECTED_RECALL_CONTROLLER_ROLLBACK_FIXTURE_COVERED_PROOF" \
  "SELECTED_RECALL_CONTROLLER_ROLLBACK_HASH_OMITTED_PROOF" \
  "has_prompt_input_readback_proofs" \
  "has_response_debug_readback_proofs" \
  "has_rollback_readback_proofs" \
  "has_controller_readback_proof" \
  "production_route_enabled: false" \
  "runtime_activation_enabled: false"; do
  assert_file_contains "$selected_recall_controller" "$term" \
    "selected-recall controller readback proofs"
done
assert_file_contains "$manifest_module" \
  "mod selected_recall" \
  "selected-recall controller manifest module"
assert_file_contains "$manifest_options" \
  "selected_recall_controller_decision_from_extension_data" \
  "selected-recall controller options intake"
assert_file_contains "$manifest_options" \
  "apply_selected_recall_controller_decision" \
  "selected-recall controller options apply"
assert_file_contains "$manifest_tests" \
  "turn_context_manifest_selected_recall_controller_filters_and_applies_payload_light_inputs" \
  "selected-recall controller focused test"
assert_file_contains "$manifest_tests" \
  "SelectedRecallControllerCanaryReadiness" \
  "selected-recall controller canary readiness focused test"
assert_file_contains "$manifest_tests" \
  "SelectedRecallControllerCanaryMetrics" \
  "selected-recall controller canary metrics focused test"
assert_file_contains "$manifest_tests" \
  "SelectedRecallControllerReadbackProofs" \
  "selected-recall controller readback proofs focused test"
assert_file_contains "$manifest_tests" \
  "SelectedRecallControllerReadbackProof" \
  "selected-recall controller readback proof focused test"
assert_file_contains "$manifest_tests" \
  "SelectedRecallControllerReadbackSurface" \
  "selected-recall controller readback surface focused test"

assert_file_contains "$debug_gate" "hepta-context-selected-recall-summary-canary-gate.sh" \
  "selected-recall summary canary debug gate"
assert_file_contains "$preflight_script" "selected recall summary canary readiness gate" \
  "selected-recall summary canary preflight stage"
assert_file_contains "$release_manifest" "codex-rs/core/src/context_manager/manifest/selected_recall.rs" \
  "selected-recall controller release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-selected-recall-summary-canary-report.sh" \
  "selected-recall summary canary report release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-selected-recall-summary-canary-gate.sh" \
  "selected-recall summary canary gate release manifest"

assert_line_before \
  "$debug_gate" \
  "hepta-context-response-debug-export-gate.sh" \
  "hepta-context-selected-recall-summary-canary-gate.sh" \
  "selected-recall summary canary debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-prompt-input-summary-gate.sh" \
  "hepta-context-selected-recall-summary-canary-gate.sh" \
  "selected-recall summary canary debug order"
assert_line_before \
  "$preflight_script" \
  "context response-debug export gate" \
  "selected recall summary canary readiness gate" \
  "selected-recall summary canary preflight order"
assert_line_before \
  "$preflight_script" \
  "context prompt-input gate" \
  "selected recall summary canary readiness gate" \
  "selected-recall summary canary preflight order"

leak_pattern='(prompt_text|transcript_text|memory_text|answer_text|session_id|memory_id|trace_id|query_text|query_payload|tool_args|tool_outputs|raw_payload|raw_ranked_payload|rank_explanation|score_reason|snippet_hash=|text_hash|rollback_hash|runtime-activation=enabled|production-route=enabled|operator-activation=enabled|graph-write=enabled|production-write=enabled)'
if grep -E "$leak_pattern" "$report_output" >/dev/null; then
  fail "selected recall summary canary report leaked payload or activation state"
fi

bash -n "$report_script"
bash -n "$gate_script"

echo "selected-recall-summary-canary=pass"
echo "selected-recall-summary-canary.payload-light=pass"
echo "selected-recall-summary-canary.metrics=shadow-live-token-latency-quality"
echo "selected-recall-summary-canary.readback.prompt-input=manifest-no-leak"
echo "selected-recall-summary-canary.readback.response-debug=payload-light-summary"
echo "selected-recall-summary-canary.readback.rollback=fixture-covered"
echo "selected-recall-summary-canary.controller-readback=typed-proofs"
echo "selected-recall-summary-canary.operator-approval=required"
echo "selected-recall-summary-canary.runtime-activation=disabled"
