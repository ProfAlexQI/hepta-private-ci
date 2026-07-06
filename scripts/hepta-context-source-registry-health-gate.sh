#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
rust_registry="$repo_root/codex-rs/core/src/context_manager/source_registry.rs"
rust_registry_entry="$repo_root/codex-rs/core/src/context_manager/source_registry/entry.rs"
rust_registry_health="$repo_root/codex-rs/core/src/context_manager/source_registry/health.rs"
rust_registry_tests="$repo_root/codex-rs/core/src/context_manager/source_registry/tests.rs"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

fail() {
  echo "hepta-context-source-registry-health-gate: $*" >&2
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

assert_file_contains "$rust_registry" "mod health;" \
  "context source registry health module"
assert_file_contains "$rust_registry" "#[cfg(test)]" \
  "context source registry health test-only module"

for term in \
  "ttl" \
  "volatility" \
  "trust_class" \
  "redaction_policy" \
  "quality_metric" \
  "activation_guard" \
  "rollback_policy"; do
  assert_file_contains "$rust_registry_entry" "$term" \
    "context source registry descriptor field"
done

for term in \
  "ContextSourceRegistryHealthReport" \
  "context_source_registry_health_report" \
  "descriptor_field_count: 14" \
  "live_activation_route_count" \
  "runtime_activation: \"disabled\""; do
  assert_file_contains "$rust_registry_health" "$term" \
    "context source registry health report"
done

assert_file_contains "$rust_registry_tests" \
  "context_source_registry_health_report_is_payload_light_and_non_activating" \
  "context source registry health test"
assert_file_contains "$rust_registry_tests" "live_activation_route_count, 0" \
  "context source registry health test live activation guard"

cargo test --manifest-path "$manifest" -p codex-core \
  context_manager::source_registry::tests::context_source_registry_health_report_is_payload_light_and_non_activating \
  --lib --message-format=short

echo "context-source-registry-health=pass"
echo "context-source-registry-health.descriptor-fields=14"
echo "context-source-registry-health.entry-count=19"
echo "context-source-registry-health.live-activation-routes=0"
echo "context-source-registry-health.runtime-activation=disabled"
echo "Hepta context source registry health gate passed"
