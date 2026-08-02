#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-control-ui-css-metrics-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

scripts/hepta-control-ui-debt-audit --output "$TEST_DIR/debt.json" >/dev/null
HEPTA_UI_DESIGN_SYSTEM_GATE_DIR="$TEST_DIR/design" \
HEPTA_UI_DESIGN_SYSTEM_GATE_REPORT_PATH="$TEST_DIR/design.json" \
  scripts/hepta-ui-design-system-gate.sh >/dev/null

jq -e --slurpfile design "$TEST_DIR/design.json" '
  ($design[0].control) as $control |
  .status == "ready"
  and $design[0].status == "ready"
  and .css_bytes == $control.css_bytes
  and .metrics.runtime_css_bytes == .css_bytes.gateway_stylesheet_body_bytes
  and .metrics.retired_first_batch_legacy_class_marker_count == 0
  and .checks.retired_first_batch_legacy_classes_absent == true
  and (.retired_first_batch_legacy_class_markers | length) == 0
  and $control.runtime_css_bytes == $control.css_bytes.leaf_source_bytes
  and .css_bytes.gateway_stylesheet_body_bytes == (.css_bytes.leaf_source_bytes + .css_bytes.gateway_join_separator_bytes)
  and .css_bytes.gateway_join_separator_bytes == (.css_bytes.leaf_file_count - 1)
  and .css_bytes.direct_preview_css_payload_bytes == (.css_bytes.direct_preview_manifest_bytes + .css_bytes.leaf_source_bytes)
' "$TEST_DIR/debt.json" >/dev/null

echo "hepta-control-ui CSS receipt metric parity self-test: PASS"
