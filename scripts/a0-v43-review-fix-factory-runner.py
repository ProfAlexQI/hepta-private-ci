#!/usr/bin/env python3
"""Run the A0 review-fix factory with exact post-generation marker sealing."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import runpy

ROOT = Path(__file__).resolve().parents[1]
FACTORY = ROOT / "scripts" / "a0-v43-review-fix-factory.py"
ns = runpy.run_path(str(FACTORY), run_name="a0_v43_factory_module")

ns["patch_master_and_spec"]()
ns["write_current_truth_script"]()
ns["write_master_verifier"]()
ns["write_document_verifier"]()
ns["write_a0_verifier"]()
ns["write_workflows"]()
ns["patch_json_documents"]()

plan_dir = ROOT / "plans" / "hepta-intelligence"
master_path = plan_dir / "HEPTA_INTELLIGENCE_MASTER_PLAN.md"
master = master_path.read_text(encoding="utf-8")
seal = """

### 20.1 Exact machine-comparable markers

```text
implemented → candidate_qualified → selected → wired → runtime_qualified → efficacy_proven → operator_accepted → promoted
candidate_workflow_may_self_qualify=false
a0_candidate_qualified=false
```
"""
if "### 20.1 Exact machine-comparable markers" not in master:
    master += seal
master_path.write_text(master, encoding="utf-8")

sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
current_path = plan_dir / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
current = json.loads(current_path.read_text(encoding="utf-8"))
current["canonical"]["content_sha256"] = sha(master_path)
current_path.write_text(json.dumps(current, indent=2, sort_keys=True, ensure_ascii=False) + "\n", encoding="utf-8")

doc_path = plan_dir / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
doc = json.loads(doc_path.read_text(encoding="utf-8"))
doc["current_authority"]["human"]["content_sha256"] = sha(master_path)
doc_path.write_text(json.dumps(doc, indent=2, sort_keys=True, ensure_ascii=False) + "\n", encoding="utf-8")

ns["validate"]()
ns["build_artifact"]()
print("PASS_A0_V43_REVIEW_FIX_FACTORY_RUNNER")
