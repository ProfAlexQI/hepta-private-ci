#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"

fail() {
  echo "hepta-context-release-manifest-gate: $*" >&2
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

normalized="$(mktemp -t hepta-context-release-manifest.XXXXXX)"
manifest_paths="$(mktemp -t hepta-context-release-manifest-paths.XXXXXX)"
required_paths="$(mktemp -t hepta-context-release-required-paths.XXXXXX)"

cleanup() {
  rm -f "$normalized" "$manifest_paths" "$required_paths"
}
trap cleanup EXIT

assert_file_contains "$release_manifest" "# version=1" \
  "context lane release manifest version"
assert_file_contains "$release_manifest" "# owner_lane=hepta-context" \
  "context lane release manifest owner"

grep -v '^#' "$release_manifest" >"$normalized"

awk -F '\t' '
  NR == 1 {
    if ($0 != "path\tcategory\towner_lane\trelease_class") {
      printf("bad header: %s\n", $0) > "/dev/stderr";
      exit 1;
    }
    next;
  }
  NF != 4 {
    printf("bad field count on line %d\n", NR) > "/dev/stderr";
    exit 1;
  }
  $1 !~ /^[A-Za-z0-9._\/-]+$/ || $1 ~ /^\// || $1 ~ /(^|\/)\.\.($|\/)/ {
    printf("bad relative path on line %d: %s\n", NR, $1) > "/dev/stderr";
    exit 1;
  }
  $2 !~ /^(contract|registry|release_manifest|rust_context|rust_context_manager|rust_session|rust_memory|rust_protocol|response_debug|gate_script)$/ {
    printf("bad category for %s: %s\n", $1, $2) > "/dev/stderr";
    exit 1;
  }
  $3 != "hepta-context" {
    printf("bad owner lane for %s: %s\n", $1, $3) > "/dev/stderr";
    exit 1;
  }
  $4 != "required" {
    printf("bad release class for %s: %s\n", $1, $4) > "/dev/stderr";
    exit 1;
  }
  seen[$1]++ {
    printf("duplicate path: %s\n", $1) > "/dev/stderr";
    exit 1;
  }
  previous != "" && previous > $1 {
    printf("paths are not sorted: %s before %s\n", previous, $1) > "/dev/stderr";
    exit 1;
  }
  {
    previous = $1;
    print $1;
    count++;
  }
  END {
    if (count == 0) {
      printf("release manifest has no entries\n") > "/dev/stderr";
      exit 1;
    }
  }
' "$normalized" >"$manifest_paths" || fail "context lane release manifest schema validation failed"

{
  cat <<'EOF'
codex-rs/CONTEXT_DEBUG_CONTRACTS.md
codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv
codex-rs/CONTEXT_SOURCE_REGISTRY.tsv
codex-rs/core/src/context/collaboration_mode_instructions.rs
codex-rs/core/src/context/contextual_user_message.rs
codex-rs/core/src/context/contextual_user_message_tests.rs
codex-rs/core/src/context/extension_prompt_fragment.rs
codex-rs/core/src/context/mod.rs
codex-rs/core/src/context/model_switch_instructions.rs
codex-rs/core/src/context_manager/budget_planner.rs
codex-rs/core/src/context_manager/controller.rs
codex-rs/core/src/context_manager/history.rs
codex-rs/core/src/context_manager/history_tests.rs
codex-rs/core/src/context_manager/manifest.rs
codex-rs/core/src/context_manager/manifest/classification.rs
codex-rs/core/src/context_manager/manifest/ledger.rs
codex-rs/core/src/context_manager/manifest/options.rs
codex-rs/core/src/context_manager/manifest/policy.rs
codex-rs/core/src/context_manager/manifest/policy/candidate.rs
codex-rs/core/src/context_manager/manifest/policy/compression.rs
codex-rs/core/src/context_manager/manifest/rewrite.rs
codex-rs/core/src/context_manager/manifest/selected_recall.rs
codex-rs/core/src/context_manager/manifest/selected_snippet.rs
codex-rs/core/src/context_manager/manifest/tests.rs
codex-rs/core/src/context_manager/mod.rs
codex-rs/core/src/context_manager/normalize.rs
codex-rs/core/src/context_manager/source_registry.rs
codex-rs/core/src/context_manager/source_registry/catalog.rs
codex-rs/core/src/context_manager/source_registry/entry.rs
codex-rs/core/src/context_manager/source_registry/health.rs
codex-rs/core/src/context_manager/source_registry/tests.rs
codex-rs/core/src/context_manager/updates.rs
codex-rs/core/src/context_manager/updates/capability.rs
codex-rs/core/src/context_manager/updates/extension.rs
codex-rs/core/src/session/handlers.rs
codex-rs/core/src/session/mod.rs
codex-rs/core/src/session/multi_agents.rs
codex-rs/core/src/session/rollout_reconstruction_tests.rs
codex-rs/core/src/session/tests.rs
codex-rs/core/src/session/turn_context.rs
codex-rs/hepta-core/src/memory.rs
codex-rs/hepta-core/src/memory/context_plane.rs
codex-rs/hepta-core/src/memory/context_plane/activation.rs
codex-rs/hepta-core/src/memory/context_plane/activation/matrix.rs
codex-rs/hepta-core/src/memory/context_plane/activation/row.rs
codex-rs/hepta-core/src/memory/context_plane/activation/target.rs
codex-rs/hepta-core/src/memory/context_plane/operator.rs
codex-rs/hepta-core/src/memory/context_plane/status.rs
codex-rs/hepta-core/src/memory/context_plane/status/entry.rs
codex-rs/hepta-core/src/memory/context_plane/status/report.rs
codex-rs/hepta-core/src/memory/context_plane/status/section.rs
codex-rs/hepta-core/src/memory/eval_harness.rs
codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow.rs
codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/comparison.rs
codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/report.rs
codex-rs/hepta-core/src/memory/eval_harness/adaptive_shadow/result.rs
codex-rs/hepta-core/src/memory/eval_harness/eval_seed.rs
codex-rs/hepta-core/src/memory/eval_harness/ranked_recall_shadow.rs
codex-rs/hepta-core/src/memory/eval_harness/selected_recall_canary.rs
codex-rs/hepta-core/src/memory/eval_harness/shadow_canary_promotion.rs
codex-rs/hepta-core/src/memory/eval_harness/shadow_quality_summary.rs
codex-rs/hepta-core/src/memory/eval_harness/shadow_quality_trend_snapshot.rs
codex-rs/hepta-core/src/memory/eval_harness/shadow_regression_dashboard.rs
codex-rs/hepta-core/src/memory/eval_harness/temporal_graph_shadow.rs
codex-rs/hepta-core/src/memory/formation.rs
codex-rs/hepta-core/src/memory/provider_plane.rs
codex-rs/hepta-core/src/memory/provider_plane_v2.rs
codex-rs/hepta-core/src/memory/query.rs
codex-rs/hepta-core/src/memory/recall.rs
codex-rs/hepta-core/src/memory/recall/bundle.rs
codex-rs/hepta-core/src/memory/recall/coverage.rs
codex-rs/hepta-core/src/memory/recall/inspection.rs
codex-rs/hepta-core/src/memory/recall/ranked.rs
codex-rs/hepta-core/src/memory/recall/request.rs
codex-rs/hepta-core/src/memory/recall_quality_gate.rs
codex-rs/hepta-core/src/memory/recall_quality_gate/fixture.rs
codex-rs/hepta-core/src/memory/recall_quality_gate/report.rs
codex-rs/hepta-core/src/memory/restore.rs
codex-rs/hepta-core/src/memory/restore/delta.rs
codex-rs/hepta-core/src/memory/restore/domain.rs
codex-rs/hepta-core/src/memory/restore/planning.rs
codex-rs/hepta-core/src/memory/restore/planning/impact.rs
codex-rs/hepta-core/src/memory/restore/planning/mutation.rs
codex-rs/hepta-core/src/memory/restore/planning/readiness.rs
codex-rs/hepta-core/src/memory/restore/planning/safety.rs
codex-rs/hepta-core/src/memory/restore/preview.rs
codex-rs/hepta-core/src/memory/snapshot.rs
codex-rs/hepta-core/src/memory/snapshot/memory.rs
codex-rs/hepta-core/src/memory/snapshot/transcript.rs
codex-rs/hepta-core/src/memory/snapshot_inspection.rs
codex-rs/hepta-core/src/memory/snapshot_inspection/audit.rs
codex-rs/hepta-core/src/memory/snapshot_inspection/drift.rs
codex-rs/hepta-core/src/memory/snapshot_inspection/health.rs
codex-rs/hepta-core/src/memory/snapshot_inspection/inspected.rs
codex-rs/hepta-core/src/memory/taxonomy.rs
codex-rs/hepta-core/src/memory/temporal.rs
codex-rs/hepta-core/src/memory/temporal/canary_guard.rs
codex-rs/hepta-core/src/memory/temporal/fact.rs
codex-rs/hepta-core/src/memory/temporal/graph.rs
codex-rs/hepta-core/src/memory/temporal/quality.rs
codex-rs/hepta-core/src/memory/temporal/replay.rs
codex-rs/hepta-core/src/memory/temporal/rollback_kill_switch.rs
codex-rs/hepta-core/src/memory/temporal/store.rs
codex-rs/hepta-core/src/memory/temporal/traversal_diff.rs
codex-rs/hepta-core/src/memory/tests.rs
codex-rs/hepta-core/src/memory/tests/context_plane_activation.rs
codex-rs/hepta-core/src/memory/tests/context_plane_operator_packet.rs
codex-rs/hepta-core/src/memory/tests/context_plane_status.rs
codex-rs/hepta-core/src/memory/tests/provider_plane.rs
codex-rs/hepta-core/src/memory/tests/recall_core.rs
codex-rs/hepta-core/src/memory/tests/recall_inspection.rs
codex-rs/hepta-core/src/memory/tests/recall_inspection/bundle.rs
codex-rs/hepta-core/src/memory/tests/recall_inspection/coverage.rs
codex-rs/hepta-core/src/memory/tests/recall_inspection/inspection.rs
codex-rs/hepta-core/src/memory/tests/recall_inspection/limit_pressure.rs
codex-rs/hepta-core/src/memory/tests/recall_inspection/omission.rs
codex-rs/hepta-core/src/memory/tests/recall_inspection/provenance.rs
codex-rs/hepta-core/src/memory/tests/recall_inspection/request.rs
codex-rs/hepta-core/src/memory/tests/recall_summary.rs
codex-rs/hepta-core/src/memory/tests/restore_impact.rs
codex-rs/hepta-core/src/memory/tests/restore_planning.rs
codex-rs/hepta-core/src/memory/tests/restore_planning/delta.rs
codex-rs/hepta-core/src/memory/tests/restore_planning/impact.rs
codex-rs/hepta-core/src/memory/tests/restore_planning/mutation.rs
codex-rs/hepta-core/src/memory/tests/restore_planning/preview.rs
codex-rs/hepta-core/src/memory/tests/restore_preview.rs
codex-rs/hepta-core/src/memory/tests/restore_preview/delta_counts.rs
codex-rs/hepta-core/src/memory/tests/restore_preview/domain_totals.rs
codex-rs/hepta-core/src/memory/tests/restore_preview/records.rs
codex-rs/hepta-core/src/memory/tests/restore_readiness.rs
codex-rs/hepta-core/src/memory/tests/restore_readiness/constructors.rs
codex-rs/hepta-core/src/memory/tests/restore_readiness/readiness.rs
codex-rs/hepta-core/src/memory/tests/restore_readiness/safety.rs
codex-rs/hepta-core/src/memory/tests/session.rs
codex-rs/hepta-core/src/memory/tests/snapshot.rs
codex-rs/hepta-core/src/memory/tests/snapshot/memory_integrity.rs
codex-rs/hepta-core/src/memory/tests/snapshot/memory_inventory.rs
codex-rs/hepta-core/src/memory/tests/snapshot/memory_manifest.rs
codex-rs/hepta-core/src/memory/tests/snapshot/memory_stats.rs
codex-rs/hepta-core/src/memory/tests/snapshot/transcript_integrity.rs
codex-rs/hepta-core/src/memory/tests/snapshot/transcript_inventory.rs
codex-rs/hepta-core/src/memory/tests/snapshot/transcript_manifest.rs
codex-rs/hepta-core/src/memory/tests/snapshot/transcript_stats.rs
codex-rs/hepta-core/src/memory/tests/snapshot_inspection/audit.rs
codex-rs/hepta-core/src/memory/tests/snapshot_inspection/drift.rs
codex-rs/hepta-core/src/memory/tests/snapshot_inspection/health.rs
codex-rs/hepta-core/src/memory/tests/snapshot_inspection/inspected.rs
codex-rs/hepta-core/src/memory/tests/store.rs
codex-rs/hepta-core/src/memory/tests/transcript.rs
codex-rs/hepta-core/tests/memory_eval_harness.rs
codex-rs/hepta-core/tests/memory_query.rs
codex-rs/hepta-core/tests/memory_recall_contracts.rs
codex-rs/hepta-core/tests/memory_recall_contracts/formation.rs
codex-rs/hepta-core/tests/memory_recall_contracts/taxonomy.rs
codex-rs/hepta-core/tests/memory_recall_contracts/temporal.rs
codex-rs/hepta-core/tests/memory_recall_quality.rs
codex-rs/hepta-core/src/memory/transcript.rs
codex-rs/hepta-memory/src/context_plane_helpers.rs
codex-rs/hepta-memory/src/lib.rs
codex-rs/hepta-memory/src/recall_helpers.rs
codex-rs/hepta-memory/src/recall_helpers/query.rs
codex-rs/hepta-memory/src/recall_helpers/ranking.rs
codex-rs/hepta-memory/src/recall_helpers/snapshot.rs
codex-rs/hepta-memory/src/recall_helpers/store.rs
codex-rs/hepta-memory/src/snapshot_helpers.rs
codex-rs/hepta-memory/src/snapshot_helpers/inspected_snapshot.rs
codex-rs/hepta-memory/src/snapshot_helpers/snapshot.rs
codex-rs/hepta-memory/src/snapshot_helpers/store.rs
codex-rs/hepta-memory/src/tests/context_memory.rs
codex-rs/hepta-memory/src/tests/context_plane.rs
codex-rs/hepta-memory/src/tests/context_plane/activation_matrix.rs
codex-rs/hepta-memory/src/tests/context_plane/operator_packet.rs
codex-rs/hepta-memory/src/tests/context_plane/status.rs
codex-rs/hepta-memory/src/tests/mod.rs
codex-rs/hepta-memory/src/tests/recall_context_core.rs
codex-rs/hepta-memory/src/tests/recall_context_helpers.rs
codex-rs/hepta-memory/src/tests/recall_context_helpers/availability.rs
codex-rs/hepta-memory/src/tests/recall_context_helpers/bundle.rs
codex-rs/hepta-memory/src/tests/recall_context_helpers/coverage.rs
codex-rs/hepta-memory/src/tests/recall_context_helpers/limit_pressure.rs
codex-rs/hepta-memory/src/tests/recall_context_helpers/omission.rs
codex-rs/hepta-memory/src/tests/recall_context_helpers/provenance.rs
codex-rs/hepta-memory/src/tests/recall_context_quality.rs
codex-rs/hepta-memory/src/tests/recall_context_quality/availability.rs
codex-rs/hepta-memory/src/tests/recall_context_quality/coverage.rs
codex-rs/hepta-memory/src/tests/recall_context_quality/inspection.rs
codex-rs/hepta-memory/src/tests/recall_context_quality/limit_pressure.rs
codex-rs/hepta-memory/src/tests/recall_context_quality/omission.rs
codex-rs/hepta-memory/src/tests/recall_context_quality/provenance.rs
codex-rs/hepta-memory/src/tests/recall_memory.rs
codex-rs/hepta-memory/src/tests/recall_memory/formation.rs
codex-rs/hepta-memory/src/tests/recall_memory/taxonomy.rs
codex-rs/hepta-memory/src/tests/recall_memory/temporal.rs
codex-rs/hepta-memory/src/tests/restore_preview.rs
codex-rs/hepta-memory/src/tests/search.rs
codex-rs/hepta-memory/src/tests/snapshot_core.rs
codex-rs/hepta-memory/src/tests/snapshot_inspection.rs
codex-rs/hepta-memory/src/tests/snapshot_inspection/audit.rs
codex-rs/hepta-memory/src/tests/snapshot_inspection/drift.rs
codex-rs/hepta-memory/src/tests/snapshot_inspection/health.rs
codex-rs/hepta-memory/src/tests/snapshot_inspection/inspected.rs
codex-rs/hepta-memory/src/tests/snapshot_integrity.rs
codex-rs/hepta-memory/src/tests/snapshot_inventory.rs
codex-rs/hepta-memory/src/tests/snapshot_inventory/manifest.rs
codex-rs/hepta-memory/src/tests/snapshot_inventory/session_inventory.rs
codex-rs/hepta-memory/src/tests/snapshot_inventory/stats.rs
codex-rs/hepta-memory/src/tests/snapshot_restore.rs
codex-rs/hepta-memory/src/tests/snapshot_restore/impact.rs
codex-rs/hepta-memory/src/tests/snapshot_restore/inspected.rs
codex-rs/hepta-memory/src/tests/snapshot_restore/preview.rs
codex-rs/hepta-memory/src/tests/snapshot_restore/readiness.rs
codex-rs/hepta-memory/src/tests/snapshot_restore/roundtrip.rs
codex-rs/hepta-memory/src/tests/store.rs
codex-rs/hepta-runtime/src/bin/hepta-compat-report.rs
codex-rs/hepta-runtime/src/typed_compat_report.rs
codex-rs/protocol/src/protocol.rs
codex-rs/response-debug-context/BUILD.bazel
codex-rs/response-debug-context/Cargo.toml
codex-rs/response-debug-context/src/bin/response_debug_context.rs
codex-rs/response-debug-context/src/lib.rs
codex-rs/response-debug-context/src/rollout_context.rs
codex-rs/response-debug-context/src/rollout_context/memory.rs
codex-rs/response-debug-context/src/tests.rs
scripts/hepta-context-gate-launch
scripts/hepta-context-gate-runner
scripts/hepta-context-gate-specs-v1.json
scripts/hepta-gate-pair-runner
scripts/hepta-gate-pair-specs-v1.json
scripts/hepta-gate-typed-report-bindings-v2.json
scripts/hepta-typed-compat-report
EOF
  (cd "$repo_root" && find scripts -maxdepth 1 \( -type f -o -type l \) -name 'hepta-context*.sh' -print | sort)
  (cd "$repo_root" && find scripts/lib/hepta-context-gates-v1 -maxdepth 1 -type f -print | sort)
} | LC_ALL=C sort -u >"$required_paths"

if ! diff -u "$required_paths" "$manifest_paths"; then
  fail "context lane release manifest does not match required context-lane release file set"
fi

entry_count="$(wc -l <"$manifest_paths" | tr -d ' ')"
tracked_count=0
untracked_count=0
missing_count=0

while IFS= read -r path; do
  if [ ! -f "$repo_root/$path" ]; then
    echo "missing release manifest path: $path" >&2
    missing_count=$((missing_count + 1))
    continue
  fi

  if git -C "$repo_root" ls-files --error-unmatch -- "$path" >/dev/null 2>&1; then
    tracked_count=$((tracked_count + 1))
  else
    untracked_count=$((untracked_count + 1))
  fi
done <"$manifest_paths"

if [ "$missing_count" -ne 0 ]; then
  fail "context lane release manifest has missing file entries"
fi

strict_git="${HEPTA_CONTEXT_RELEASE_STRICT_GIT:-0}"
if [ "$strict_git" = "1" ] && [ "$untracked_count" -ne 0 ]; then
  fail "context lane release manifest strict git tracking failed: $untracked_count required files are untracked"
fi

for term in \
  "Context lane release manifest" \
  "CONTEXT_LANE_RELEASE_MANIFEST.tsv" \
  "hepta-context-release-manifest-gate.sh" \
  "git-tracked release state" \
  "strict git tracking" \
  "must not become a runtime activation route"; do
  assert_file_contains "$contracts" "$term" "context lane release manifest contract"
done

assert_file_contains "$debug_gate" "hepta-context-release-manifest-gate.sh" \
  "context lane release manifest debug gate"
assert_file_contains "$preflight_script" "context lane release manifest gate" \
  "context lane release manifest preflight stage"
assert_file_contains "$preflight_script" "HEPTA_CONTEXT_RELEASE_STRICT_GIT=1" \
  "context lane release manifest preflight strict git mode"
assert_line_before \
  "$preflight_script" \
  "context lane release manifest gate" \
  "context source registry catalog gate" \
  "context lane release manifest preflight order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-release-manifest-gate.sh" \
  "hepta-context-source-registry-catalog-gate.sh" \
  "context lane release manifest debug order"

release_ready="true"
if [ "$untracked_count" -ne 0 ]; then
  release_ready="blocked-untracked"
fi

echo "context-lane-release-manifest=pass"
echo "context-lane-release-manifest.version=1"
echo "context-lane-release-manifest.entry-count=$entry_count"
echo "context-lane-release-manifest.git-tracked-count=$tracked_count"
echo "context-lane-release-manifest.git-untracked-count=$untracked_count"
echo "context-lane-release-manifest.strict-git=$strict_git"
echo "context-lane-release-manifest.release-ready=$release_ready"
echo "context-lane-release-manifest.runtime-activation=disabled"
echo "Hepta context lane release manifest gate passed"
