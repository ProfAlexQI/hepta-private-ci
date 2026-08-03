#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
protocol="$repo_root/codex-rs/protocol/src/protocol.rs"
protocol_turn_context="$repo_root/codex-rs/protocol/src/protocol/turn_context"
protocol_stable_hash="$protocol_turn_context/stable_hash.rs"
protocol_common="$protocol_turn_context/common.rs"
protocol_tests="$repo_root/codex-rs/protocol/src/protocol/tests.rs"
manifest="$repo_root/codex-rs/core/src/context_manager/manifest.rs"
updates="$repo_root/codex-rs/core/src/context_manager/updates.rs"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
gate_pair_specs="$repo_root/scripts/hepta-gate-pair-specs-v1.json"
typed_report_bindings="$repo_root/scripts/hepta-gate-typed-report-bindings-v2.json"
pair_runner="$repo_root/scripts/hepta-gate-pair-runner"
payload_bundle="$repo_root/scripts/hepta-gate-pair-payload-bundle"
context_plane_compat_report="$repo_root/codex-rs/hepta-runtime/src/compatibility_engine/context_plane.rs"
scratch_dir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-context-manifest-replay-hash.XXXXXX")"
trap 'rm -rf "$scratch_dir"' EXIT

fail() {
  echo "hepta-context-manifest-replay-hash-boundary-gate: $*" >&2
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

assert_file_not_contains() {
  local file_path="$1"
  local needle="$2"
  local label="$3"

  if grep -F "$needle" "$file_path" >/dev/null; then
    fail "$label must not contain: $needle"
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

assert_typed_report_binding() {
  local pair_id="$1"
  jq -e --arg pair_id "$pair_id" '
    .pairs[]
    | select(.id == $pair_id)
    | .template == "typed_rust_report_v1"
      and .source_report == "codex-rs/hepta-runtime/src/compatibility_engine.rs"
      and .typed_report_runner == "scripts/hepta-typed-compat-report"
  ' "$gate_pair_specs" >/dev/null ||
    fail "missing typed report binding for $pair_id"
}

capture_typed_report() {
  local pair_id="$1"
  local output_path="$2"

  "$pair_runner" report "$pair_id" >"$output_path" ||
    fail "typed report failed for $pair_id"
  jq -e --arg pair_id "$pair_id" '
    .gate == $pair_id
    and .runtime == "hepta"
    and .status == "pass"
    and (.legacy_business_field_order | type == "array")
    and (.legacy_business_fields | type == "object")
    and (.legacy_business_field_order | length) == (.legacy_business_fields | length)
    and (.legacy_business_field_order | unique | length) == (.legacy_business_field_order | length)
    and (. as $report | [
      $report.legacy_business_field_order[] as $key
      | ($report.legacy_business_fields | has($key))
      and (($report.legacy_business_fields[$key] | type) as $kind
        | $kind == "string" or $kind == "number" or $kind == "boolean")
    ] | all)
    and (.side_effects | to_entries | all(.value == false))
    and .production_authority_granted == false
    and .write_authority_granted == false
    and .ready_for_live_execution == false
    and .mutation_enabled == false
  ' "$output_path" >/dev/null || fail "typed report envelope drifted for $pair_id"
}

render_legacy_protocol() {
  local report_path="$1"
  local output_path="$2"

  jq -r '
    . as $report
    | $report.legacy_business_field_order[] as $key
    | "\($key)=\($report.legacy_business_fields[$key])"
  ' "$report_path" >"$output_path"
}

source_receipt_matches() {
  local source_report_path="$1"
  local source_protocol_path="$2"
  local receipt_report_path="$3"
  local source_id source_generation source_sequence source_lines source_sha

  source_id="$(jq -er '.gate' "$source_report_path")" || return 1
  source_generation="$(jq -er '.generation' "$source_report_path")" || return 1
  source_sequence="$(jq -er '.sequence' "$source_report_path")" || return 1
  source_lines="$(wc -l <"$source_protocol_path" | tr -d ' ')"
  source_sha="$(shasum -a 256 "$source_protocol_path" | awk '{print $1}')"
  jq -e \
    --arg source_id "$source_id" \
    --arg source_sha "$source_sha" \
    --argjson source_generation "$source_generation" \
    --argjson source_sequence "$source_sequence" \
    --argjson source_lines "$source_lines" '
      [.sources[] | select(.report_id == $source_id)] as $matches
      | ($matches | length) == 1
        and $matches[0].line_count == $source_lines
        and $matches[0].sha256 == $source_sha
        and $matches[0].generation == $source_generation
        and $matches[0].sequence == $source_sequence
    ' "$receipt_report_path" >/dev/null
}

assert_source_receipt() {
  local source_report_path="$1"
  local source_protocol_path="$2"
  local receipt_report_path="$3"
  local label="$4"

  source_receipt_matches "$source_report_path" "$source_protocol_path" "$receipt_report_path" ||
    fail "$label source/report SHA receipt drifted"
}

assert_source_digest_tamper_rejected() {
  local source_report_path="$1"
  local source_protocol_path="$2"
  local receipt_report_path="$3"
  local label="$4"
  local source_id tampered_path

  source_id="$(jq -er '.gate' "$source_report_path")"
  tampered_path="$scratch_dir/${label//[^a-zA-Z0-9]/-}.tampered.json"
  jq --arg source_id "$source_id" '
    .sources |= map(
      if .report_id == $source_id then
        .sha256 = ((if (.sha256 | startswith("0")) then "1" else "0" end) + .sha256[1:])
      else . end
    )
  ' "$receipt_report_path" >"$tampered_path"
  if source_receipt_matches "$source_report_path" "$source_protocol_path" "$tampered_path"; then
    fail "$label accepted source digest tamper"
  fi
}

context_pair_ids=(
  hepta-context-plane-activation-blocker-matrix
  hepta-context-plane-operator-approval-packet
  hepta-context-plane-operator-approval-packet-canonical-export-digest
  hepta-context-plane-operator-approval-packet-freshness
  hepta-context-plane-operator-approval-packet-freshness-dependency-chain
  hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest
  hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift
  hepta-context-plane-operator-approval-packet-negative-export
)
for pair_id in "${context_pair_ids[@]}"; do
  assert_typed_report_binding "$pair_id"
  jq -e --arg pair_id "$pair_id" '
    .equivalence_proofs[$pair_id].captured_snapshots_preserved == false
    and .equivalence_proofs[$pair_id].forward_schema_delta_mode
      == "single_typed_extension_fail_closed_v1"
  ' "$typed_report_bindings" >/dev/null ||
    fail "typed no-snapshot proof drifted for $pair_id"
  for kind in gate report; do
    retired_path="scripts/lib/hepta-gate-pair-compat-v1/$pair_id.$kind"
    if jq -e --arg retired_path "$retired_path" '
      any(.supplemental_payloads[]?; .path == $retired_path)
    ' "$gate_pair_specs" >/dev/null; then
      fail "retired context-plane payload preserved as supplemental: $retired_path"
    fi
    if "$payload_bundle" --decode-to "$retired_path" "$scratch_dir/retired-payload" \
      >/dev/null 2>&1; then
      fail "retired context-plane payload remained decodable: $retired_path"
    fi
  done
done

approval_id="hepta-context-plane-operator-approval-packet"
negative_id="hepta-context-plane-operator-approval-packet-negative-export"
canonical_id="hepta-context-plane-operator-approval-packet-canonical-export-digest"
freshness_id="hepta-context-plane-operator-approval-packet-freshness"
dependency_id="hepta-context-plane-operator-approval-packet-freshness-dependency-chain"
dependency_digest_id="hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest"
expiry_id="hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift"

for pair_id in "$approval_id" "$negative_id" "$canonical_id" "$freshness_id" \
  "$dependency_id" "$dependency_digest_id" "$expiry_id"; do
  capture_typed_report "$pair_id" "$scratch_dir/$pair_id.json"
  render_legacy_protocol "$scratch_dir/$pair_id.json" "$scratch_dir/$pair_id.protocol"
done

for pair_id in "$canonical_id" "$dependency_digest_id"; do
  "$pair_runner" gate "$pair_id" >/dev/null || fail "typed gate failed for $pair_id"
done

assert_source_receipt \
  "$scratch_dir/$approval_id.json" "$scratch_dir/$approval_id.protocol" \
  "$scratch_dir/$canonical_id.json" "canonical-approval"
assert_source_receipt \
  "$scratch_dir/$negative_id.json" "$scratch_dir/$negative_id.protocol" \
  "$scratch_dir/$canonical_id.json" "canonical-negative"
assert_source_receipt \
  "$scratch_dir/$canonical_id.json" "$scratch_dir/$canonical_id.protocol" \
  "$scratch_dir/$freshness_id.json" "canonical-report"
assert_source_receipt \
  "$scratch_dir/$dependency_id.json" "$scratch_dir/$dependency_id.protocol" \
  "$scratch_dir/$dependency_digest_id.json" "dependency-source"
assert_source_receipt \
  "$scratch_dir/$dependency_digest_id.json" "$scratch_dir/$dependency_digest_id.protocol" \
  "$scratch_dir/$expiry_id.json" "dependency-digest-report"

assert_source_digest_tamper_rejected \
  "$scratch_dir/$approval_id.json" "$scratch_dir/$approval_id.protocol" \
  "$scratch_dir/$canonical_id.json" "canonical-approval"
assert_source_digest_tamper_rejected \
  "$scratch_dir/$canonical_id.json" "$scratch_dir/$canonical_id.protocol" \
  "$scratch_dir/$freshness_id.json" "canonical-report"
assert_source_digest_tamper_rejected \
  "$scratch_dir/$dependency_id.json" "$scratch_dir/$dependency_id.protocol" \
  "$scratch_dir/$dependency_digest_id.json" "dependency-source"
assert_source_digest_tamper_rejected \
  "$scratch_dir/$dependency_digest_id.json" "$scratch_dir/$dependency_digest_id.protocol" \
  "$scratch_dir/$expiry_id.json" "dependency-digest-report"

cat "$scratch_dir/$approval_id.protocol" "$scratch_dir/$negative_id.protocol" \
  >"$scratch_dir/canonical-combined.protocol"
combined_lines="$(wc -l <"$scratch_dir/canonical-combined.protocol" | tr -d ' ')"
combined_sha="$(shasum -a 256 "$scratch_dir/canonical-combined.protocol" | awk '{print $1}')"
jq -e --arg combined_sha "$combined_sha" --argjson combined_lines "$combined_lines" '
  .payload.kind == "canonical_export_digest"
  and .payload.combined_report_line_count == $combined_lines
  and .payload.combined_report_sha256 == $combined_sha
' "$scratch_dir/$canonical_id.json" >/dev/null ||
  fail "typed canonical combined report SHA drifted"

for term in \
  "Stable manifest replay hash" \
  "stable_turn_context_manifest_replay_hash" \
  "stable_turn_context_manifest_text_hash" \
  "not canonical trust digests" \
  "canonical SHA-256 digest reports" \
  "hepta-context-manifest-replay-hash-boundary-gate.sh"; do
  assert_file_contains "$contracts" "$term" "manifest replay hash boundary contract"
done

assert_file_contains "$protocol" "mod turn_context;" \
  "protocol turn-context module binding"
for term in \
  "StableManifestReplayHash" \
  "stable_turn_context_manifest_replay_hash" \
  "stable_turn_context_manifest_text_hash" \
  "not a cryptographic trust digest"; do
  assert_file_contains "$protocol_stable_hash" "$term" \
    "protocol manifest replay hash boundary"
done
assert_file_contains "$protocol_common" "is_stable_manifest_replay_hash" \
  "protocol manifest replay hash predicate"
assert_file_contains "$protocol_tests" \
  "turn_context_manifest_hashes_are_replay_hashes_not_trust_digests" \
  "protocol manifest replay/trust digest separation test"
if rg -F "StableManifestHash" "$protocol" "$protocol_turn_context" >/dev/null; then
  fail "protocol legacy manifest hash type name must not be present"
fi
if rg -F "is_stable_manifest_hash" "$protocol" "$protocol_turn_context" >/dev/null; then
  fail "protocol legacy manifest hash predicate name must not be present"
fi
assert_file_not_contains "$manifest" "stable_turn_context_manifest_text_hash" \
  "core manifest should use replay hash naming"
assert_file_not_contains "$updates" "stable_turn_context_manifest_text_hash" \
  "core context updates should use replay hash naming"
assert_file_contains "$manifest" "stable_turn_context_manifest_replay_hash" \
  "core manifest replay hash naming"
assert_file_contains "$updates" "stable_turn_context_manifest_replay_hash" \
  "core context updates replay hash naming"

assert_file_contains "$context_plane_compat_report" "use sha2::Sha256;" \
  "typed canonical trust digest report"
assert_file_contains "$context_plane_compat_report" "fn sha256_hex" \
  "typed canonical trust digest report"
assert_file_contains "$context_plane_compat_report" \
  "sha256_hex(protocol.as_bytes())" \
  "typed source receipt trust digest"
assert_file_contains "$context_plane_compat_report" \
  "combined_report_sha256: sha256_hex(combined_protocol.as_bytes())" \
  "typed combined report trust digest"
assert_file_not_contains "$context_plane_compat_report" \
  "stable_turn_context_manifest" \
  "typed canonical trust digest report must not use manifest replay hash"

assert_file_contains "$debug_gate" "hepta-context-manifest-replay-hash-boundary-gate.sh" \
  "manifest replay hash boundary debug gate"
assert_file_contains "$preflight_script" "context manifest replay hash boundary gate" \
  "manifest replay hash boundary preflight stage"
assert_line_before \
  "$debug_gate" \
  "hepta-context-manifest-replay-hash-boundary-gate.sh" \
  "hepta-context-source-registry-catalog-gate.sh" \
  "manifest replay hash boundary debug order"
assert_line_before \
  "$preflight_script" \
  "context manifest replay hash boundary gate" \
  "context source registry catalog gate" \
  "manifest replay hash boundary preflight order"

echo "context-manifest-replay-hash-boundary=pass"
echo "context-manifest-replay-hash-boundary.replay-hash-hex-len=16"
echo "context-manifest-replay-hash-boundary.trust-digest=sha256"
echo "context-manifest-replay-hash-boundary.runtime-activation=disabled"
echo "Hepta context manifest replay hash boundary gate passed"
