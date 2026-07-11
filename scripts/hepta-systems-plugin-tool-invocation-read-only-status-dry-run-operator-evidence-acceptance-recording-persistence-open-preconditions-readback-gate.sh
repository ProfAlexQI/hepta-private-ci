#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" gate "hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-persistence-open-preconditions-readback"
