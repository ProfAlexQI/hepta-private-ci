#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" gate "hepta-systems-work-graph-dtd8-rbackacktd-rcptret-rbackack-td2-rcptret-rbackack-preview"
