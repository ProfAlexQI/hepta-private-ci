#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" gate "hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-retention-expiry-gc-final-index-artifact-signing-export-query-observability"
