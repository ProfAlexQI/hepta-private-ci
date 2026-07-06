#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

bash "$repo_root/scripts/hepta-context-source-aware-compression-readiness-gate.sh" >/dev/null
bash "$repo_root/scripts/hepta-context-source-aware-compression-operator-approval-evidence-gate.sh" >/dev/null
bash "$repo_root/scripts/hepta-context-source-aware-compression-positive-route-readiness-gate.sh" >/dev/null

cat <<'EOF'
source-aware-readiness-export=pass
source-aware-readiness-export.schema=1
source-aware-readiness-export.runtime-activation=disabled
source-aware-readiness-export.reserved-route=disabled
source-aware-readiness-export.operator-approval-evidence=contract-only
source-aware-readiness-export.positive-route=unimplemented
source-aware-readiness-export.canary=under-development
source-aware-readiness-export.helper-marker=required
source-aware-readiness-export.no-production-consumption=pass
source-aware-readiness-export.no-debug-export-leak=pass
EOF
