#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
exec "$ROOT/scripts/hepta-gate-pair-runner" report "hepta-systems-work-graph-dtd8-td3-rbtd-rbtd-rbtd-rt-rba-td-rt-rba-td-rt-rba-rp-preview"
