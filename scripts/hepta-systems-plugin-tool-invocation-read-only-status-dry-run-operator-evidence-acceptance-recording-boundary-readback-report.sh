#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" report "hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-boundary-readback"
