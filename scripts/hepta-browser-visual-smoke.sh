#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "$0")" && pwd)"
HEPTA_BROWSER_SMOKE_LIB_DIR="$script_dir/lib/hepta-browser-visual-smoke-v1"
# shellcheck source=scripts/lib/hepta-safe-output-v1.sh
source "$script_dir/lib/hepta-safe-output-v1.sh"

for module in \
  config.sh \
  static-contract.sh \
  capture.sh \
  scenarios.sh \
  validate-results.sh \
  receipt.sh; do
  # shellcheck source=/dev/null
  source "$HEPTA_BROWSER_SMOKE_LIB_DIR/$module"
done

hepta_browser_configure
hepta_browser_validate_static_contract
hepta_browser_run_scenarios
hepta_browser_validate_results
hepta_browser_emit_receipt

echo "Hepta browser visual smoke passed"
