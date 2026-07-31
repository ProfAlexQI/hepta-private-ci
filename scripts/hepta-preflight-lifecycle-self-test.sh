#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
source "$root/scripts/lib/hepta-preflight-lifecycle.sh"
hepta_preflight_lifecycle_self_test
