#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
registry="$repo_root/codex-rs/CONTEXT_SOURCE_REGISTRY.tsv"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"

fail() {
  echo "hepta-context-health-report: $*" >&2
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

normalized_registry="$(mktemp -t hepta-context-health-registry.XXXXXX)"
normalized_manifest="$(mktemp -t hepta-context-health-release.XXXXXX)"

cleanup() {
  rm -f "$normalized_registry" "$normalized_manifest"
}
trap cleanup EXIT

grep -v '^#' "$registry" >"$normalized_registry"
grep -v '^#' "$release_manifest" >"$normalized_manifest"

descriptor_field_count="$(awk -F '\t' 'NR == 1 { print NF; exit }' "$normalized_registry")"
source_count="$(awk -F '\t' 'NR > 1 && $1 != "" { count++ } END { print count + 0 }' "$normalized_registry")"
release_required_count="$(awk -F '\t' 'NR > 1 && $1 != "" { count++ } END { print count + 0 }' "$normalized_manifest")"
live_activation_routes="$(
  awk -F '\t' '
    NR > 1 && $1 != "" &&
      $11 != "protected" &&
      $11 != "candidate_only" &&
      $11 != "operator_approval_required" {
        count++;
      }
    END { print count + 0 }
  ' "$normalized_registry"
)"

selected_fields="$(
  awk -F '\t' '
    $1 == "selected_context_recall" {
      print $9 "\t" $10 "\t" $11 "\t" $12 "\t" $14;
      found = 1;
    }
    END {
      if (!found) {
        exit 1;
      }
    }
  ' "$normalized_registry"
)" || fail "selected_context_recall registry row is missing"

IFS=$'\t' read -r selected_redaction selected_quality selected_guard selected_rollback selected_actions <<<"$selected_fields"

if [ "$descriptor_field_count" != "14" ]; then
  fail "expected 14 source-registry descriptor fields, got $descriptor_field_count"
fi
if [ "$source_count" != "19" ]; then
  fail "expected 19 context source registry rows, got $source_count"
fi
if [ "$live_activation_routes" != "0" ]; then
  fail "expected zero live activation routes, got $live_activation_routes"
fi
if [ "$selected_redaction" != "guarded_envelope" ]; then
  fail "selected_context_recall redaction policy drifted: $selected_redaction"
fi
if [ "$selected_quality" != "recall_quality" ]; then
  fail "selected_context_recall quality metric drifted: $selected_quality"
fi
if [ "$selected_guard" != "operator_approval_required" ]; then
  fail "selected_context_recall activation guard drifted: $selected_guard"
fi
if [ "$selected_rollback" != "rerun_recall" ]; then
  fail "selected_context_recall rollback policy drifted: $selected_rollback"
fi
if [ "$selected_actions" != "summary" ]; then
  fail "selected_context_recall compression action drifted: $selected_actions"
fi

for term in \
  "Context source registry catalog" \
  "Context source registry Rust resolver" \
  "Runtime activation readiness checklist" \
  "Selected-snippet live prompt compression gate"; do
  assert_file_contains "$contracts" "$term" "context health contract input"
done

for term in \
  "context source registry health gate" \
  "selected-snippet live prompt compression gate" \
  "source-aware compression readiness checklist gate"; do
  assert_file_contains "$preflight_script" "$term" "context health preflight input"
done

cat <<EOF
context-health-report=pass
context-health-report.schema=1
context-health-report.release-manifest.required-paths=$release_required_count
context-health-report.source-registry.entries=$source_count
context-health-report.source-registry.descriptor-fields=$descriptor_field_count
context-health-report.source-registry.live-activation-routes=$live_activation_routes
context-health-report.source-registry.runtime-activation=disabled
context-health-report.selected-context-recall.redaction=$selected_redaction
context-health-report.selected-context-recall.quality=$selected_quality
context-health-report.selected-context-recall.activation-guard=$selected_guard
context-health-report.selected-context-recall.rollback=$selected_rollback
context-health-report.selected-context-recall.compression=$selected_actions
context-health-report.source-aware.canary=feature-and-helper-marker
context-health-report.source-aware.production-route=disabled
context-health-report.prompt-mutation=explicit-canary-only
context-health-report.payload-policy=no-raw-prompt-or-recall-payload
context-health-report.runtime-activation=disabled
EOF
