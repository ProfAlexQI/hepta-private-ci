#!/usr/bin/env python3
"""Retire every P0.2 contents-write bootstrap after verified materialization."""

from __future__ import annotations

import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
WORKFLOW_ROOT = ROOT / ".github/workflows"
RECEIPT = ROOT / "docs/architecture/HEPTA_ARCHITECTURE_P0_2_BOOTSTRAP_RETIREMENT_V1.json"

TARGETS = {
    "hepta-architecture-convergence-p0-2-bootstrap.yml": "Hepta architecture convergence P0.2 bootstrap",
    "hepta-architecture-p0-2-portability-bootstrap.yml": "Hepta architecture P0.2 portability bootstrap",
    "hepta-memory-runtime-extraction-p0-1-bootstrap.yml": "Hepta Memory runtime extraction P0.1 bootstrap",
    "hepta-architecture-convergence-p0-2-finalize.yml": "Hepta architecture convergence P0.2 finalizer",
    "hepta-architecture-convergence-p0-2-retire-bootstrap.yml": "Hepta architecture convergence P0.2 bootstrap retirement",
}


def retired_stub(name: str, filename: str) -> str:
    return f'''name: {name} (retired)\n\non:\n  workflow_dispatch:\n\npermissions:\n  contents: read\n\njobs:\n  provenance:\n    name: Retired bootstrap provenance\n    runs-on: ubuntu-24.04\n    steps:\n      - name: Report retirement\n        shell: bash\n        run: |\n          set -euo pipefail\n          echo "{filename} is a read-only provenance stub."\n          echo "It cannot format, mutate, commit, push, promote, release, or grant authority."\n'''


def main() -> int:
    retired = []
    for filename, name in TARGETS.items():
        path = WORKFLOW_ROOT / filename
        if not path.is_file():
            raise SystemExit(f"bootstrap workflow is missing: {filename}")
        content = retired_stub(name, filename)
        path.write_text(content, encoding="utf-8")
        retired.append(
            {
                "path": f".github/workflows/{filename}",
                "contentsWrite": False,
                "automaticTrigger": False,
                "commitOrPush": False,
                "promotion": False,
                "release": False,
            }
        )

    receipt = {
        "schema": "hepta.architecture-p0-2.bootstrap-retirement.v1",
        "schemaVersion": 1,
        "status": "RETIRED_TO_READ_ONLY_PROVENANCE_STUBS",
        "retired": retired,
        "authority": {
            "productionCaller": False,
            "productionWriter": False,
            "effectAuthority": False,
            "externalEffect": False,
            "modelInvocationAuthority": False,
            "operatorAcceptance": False,
            "promotion": False,
            "release": False,
        },
    }
    RECEIPT.write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
    print("ARCHITECTURE_P0_2_BOOTSTRAPS_RETIRED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
