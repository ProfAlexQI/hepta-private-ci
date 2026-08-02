#!/usr/bin/env bash
set -euo pipefail

# Compatibility entry point. The only active readiness implementation is
# hepta-ui-current-readiness.sh; historical report chains are not replayed.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
if [[ -z "$READINESS_DIR" ]]; then
  echo "HEPTA_UI_PRODUCT_READINESS_DIR or a readiness directory argument is required" >&2
  exit 2
fi
REPORT_PATH="${HEPTA_UI_CURRENT_SOURCE_READINESS_REPORT_PATH:-$READINESS_DIR/ui-current-source-readiness.json}"
args=(--evidence-dir "$READINESS_DIR" --output "$REPORT_PATH" --require source --verify-features)
if [[ "${HEPTA_UI_CURRENT_READINESS_VERIFY:-0}" == "1" ]]; then args+=(--verify-package); fi
exec "$ROOT_DIR/scripts/hepta-ui-current-readiness.sh" "${args[@]}"
