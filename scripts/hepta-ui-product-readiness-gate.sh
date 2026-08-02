#!/usr/bin/env bash
set -euo pipefail

# Compatibility entry point for the retired multi-generation product gate.
# Full-product readiness is now a current-source, receipt-bound promotion level
# in hepta-ui-current-readiness.sh. Missing live/device/release receipts fail
# closed instead of being inferred from marker counts or historical screenshots.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
if [[ -z "$READINESS_DIR" ]]; then READINESS_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-product-readiness.XXXXXX")"; fi
REPORT_PATH="${HEPTA_UI_PRODUCT_READINESS_REPORT_PATH:-$READINESS_DIR/ui-product-readiness.json}"
args=(--evidence-dir "$READINESS_DIR" --output "$REPORT_PATH" --require full)
if [[ "${HEPTA_UI_CURRENT_READINESS_VERIFY:-0}" == "1" ]]; then args+=(--verify --verify-browser); fi
exec "$ROOT_DIR/scripts/hepta-ui-current-readiness.sh" "${args[@]}"
