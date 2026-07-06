#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
response_debug="$repo_root/codex-rs/response-debug-context/src/lib.rs"
response_debug_tests="$repo_root/codex-rs/response-debug-context/src/tests.rs"
response_debug_export_gate="$repo_root/scripts/hepta-context-response-debug-export-gate.sh"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
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

fail() {
  echo "hepta-context-memory-formation-candidate-no-leak-export-gate: $*" >&2
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

assert_export_has_no_candidate_payload_paths() {
  local export_path="$1"
  local label="$2"

  if jq -r 'paths | map(tostring) | join(".")' "$export_path" \
    | rg -n '(^|\.)(memory_formation_candidates|memory_formation_candidate_previews|candidate_text|transcript_text|memory_text|raw_transcript|tool_args|tool_arguments|raw_idempotency_key|idempotency_key|idempotency_key_hash|source_id|source_ids|memory_id|memory_ids|per_source_candidates|per_source_list|email|phone|user_identifier)(\.|$)'; then
    fail "response-debug export exposed memory formation candidate payload-shaped key in $label"
  fi
}

for term in \
  "Memory formation candidate no-leak/export guard" \
  "memory_formation_candidates" \
  "memory_formation_candidate_previews" \
  "candidate_text" \
  "transcript_text" \
  "memory_text" \
  "tool_args" \
  "raw_idempotency_key" \
  "per-source candidate lists" \
  "email-shaped strings" \
  "phone-shaped" \
  "strings, or user identifiers" \
  "must not write production memory" \
  "must not promote preview" \
  "candidate text into durable memory" \
  "must not alter prompt assembly" \
  "hepta-context-memory-formation-candidate-no-leak-export-gate.sh" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" "memory formation candidate no-leak contract"
done

assert_file_contains "$response_debug_tests" \
  "rollout_context_debug_ignores_memory_formation_candidate_preview_payloads" \
  "response-debug candidate preview no-leak test"
assert_file_contains "$response_debug_export_gate" \
  "memory-formation-candidate-preview-good.jsonl" \
  "response-debug CLI candidate preview fixture"
assert_file_contains "$response_debug_export_gate" \
  "raw-idempotency-key-should-not-export" \
  "response-debug CLI raw idempotency leak bait"
assert_file_contains "$debug_gate" \
  "hepta-context-memory-formation-candidate-no-leak-export-gate.sh" \
  "debug gate candidate no-leak wiring"
assert_file_contains "$preflight_script" \
  "context memory formation candidate no-leak export gate" \
  "preflight candidate no-leak stage"
assert_file_contains "$front_door_gate" \
  "hepta-context-memory-formation-candidate-no-leak-export-gate.sh" \
  "front-door static candidate no-leak wiring"

assert_line_before \
  "$preflight_script" \
  "context memory formation receipt gate" \
  "context memory formation candidate no-leak export gate" \
  "candidate no-leak preflight stage order"
assert_line_before \
  "$preflight_script" \
  "context memory formation candidate no-leak export gate" \
  "source-aware compression front-door machine-readable report" \
  "candidate no-leak preflight stage order"

cargo test --manifest-path "$manifest" -p codex-response-debug-context \
  memory_formation_candidate_preview \
  --lib --message-format=short

cat >"$tmpdir/memory-formation-candidate-preview-good.jsonl" <<'JSONL'
{"type":"turn_context_manifest","payload":{"version":1,"estimated_tokens":18,"budget_tokens":24,"memory_formation_receipts":[{"candidate_type":"fact","transcript_span_count":2,"provenance_span_count":2,"confidence_basis_points":6400,"idempotency_key_hash":"0123456789abcdef","privacy_class":"user_private","queued_for_background":true}],"memory_formation_candidate_previews":[{"candidate_type":"fact","candidate_text":"candidate fact payload should not export","transcript_text":"candidate transcript payload should not export","memory_text":"candidate memory payload should not export","tool_args":{"command":"candidate tool args should not export"},"raw_idempotency_key":"raw-idempotency-key-should-not-export","idempotency_key":"idempotency-key-should-not-export","idempotency_key_hash":"candidate-preview-hash-should-not-export","source_id":"candidate-preview-source-id-should-not-export","source_ids":["candidate-preview-source-list-should-not-export"],"memory_id":"candidate-preview-memory-id-should-not-export","memory_ids":["candidate-preview-memory-list-should-not-export"],"per_source_candidates":[{"source_id":"candidate-preview-per-source-id-should-not-export"}],"email":"candidate-email@example.invalid","phone":"+15550101010","user_identifier":"candidate-user-identifier-should-not-export"}],"memory_formation_candidates":[{"candidate_text":"future candidate payload should not export","raw_transcript":"future raw transcript should not export","tool_arguments":"future tool arguments should not export"}],"entries":[{"role":"developer","tier":"retrieved_snippets","source":"turn_context:developer:selected_context_recall:0","replay_key":"turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa","text_hash":"aaaaaaaaaaaaaaaa","estimated_tokens":18}]}}
JSONL

cargo run --quiet --manifest-path "$manifest" -p codex-response-debug-context \
  --bin response_debug_context -- --strict \
  <"$tmpdir/memory-formation-candidate-preview-good.jsonl" \
  >"$tmpdir/memory-formation-candidate-preview-good.json"

assert_export_has_no_candidate_payload_paths \
  "$tmpdir/memory-formation-candidate-preview-good.json" \
  "memory formation candidate preview export"

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
    fail "response-debug export leaked memory formation candidate preview payload: $leaked"
  fi
done

echo "context-memory-formation-candidate-no-leak=pass"
echo "context-memory-formation-candidate-no-leak.production-write=disabled"
echo "context-memory-formation-candidate-no-leak.runtime-activation=disabled"
echo "Hepta context memory formation candidate no-leak export gate passed"
