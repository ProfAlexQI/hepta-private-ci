#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" report "hepta-systems-public-ga-operator-intent-consent-evidence-terminal-decision-status-final-index-terminal-public-claim-status-exposure"
