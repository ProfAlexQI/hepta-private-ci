#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" report "hepta-systems-public-ga-operator-audit-evidence-final-index-retention-expiry-gc"
