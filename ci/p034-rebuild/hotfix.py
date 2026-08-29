#!/usr/bin/env python3
"""Validate the immutable P0.3.4 v7 reconstruction payload before use."""
from __future__ import annotations
import hashlib, json, py_compile, stat, sys, tempfile
from pathlib import Path

ARCHIVE_SHA256 = '9e8fcf09a0da859a7f0b1495c4a0559a6d3f9f6f24ac85e4721184fc9fd3bd96'
EXPECTED = {'0012_legacy_grounding_governance.sql': (11095, 'e31c69906fd4051ec5a12da310d35d460bff04c057aff5bdf604362cab238da4', '0644'), 'P034_PLAN.md': (5342, 'be7c96c606e915b40a1fb8ae8c704d1d3640f96cc9818df68a756310ea81c145', '0644'), 'P034_STATUS.json': (5754, '8d94029d5ac58acdd388dfcf7bf51ceab72f82c00b79a5f8c9263c17f1ea827f', '0644'), 'P034_TRANCHE.md': (2061, '5cf87b92af2dfdca3036971741b2621895640933c99af9cf9f6c659cdc5ec4a1', '0644'), 'apply.py': (19038, '6780cbea1b5d4b81dee83a2538d5dc8589f80d83db8e4aaeb6837fa9191eb443', '0755'), 'backfill.rs': (13956, 'ee03f42fdb3c533372e8cd3d69c373bce9ae68244c7262a88a9e8d64caf7f35f', '0644'), 'candidate-workflow.yml': (13846, 'db0f7ab3c4d16416e29c1f1813ed8beb3bd1f12e9478a4431af15ef6279485c4', '0644'), 'clippy_delta.py': (24023, 'da1257774a10203b1993f3230df1fd0e56d4996ff8acdc9faa7f2c072e2d2dad', '0755'), 'legacy_governance.rs': (70408, 'fbd9f745c00bf44c1d7b4ef87f4ce96f59b5b77e3c237995f4ad748a2ff88f74', '0644'), 'run.py': (12368, 'feb487ca3af01ecfb0acb7038ba08fd41fe307a2f2e575a3072a8199c9b67fdc', '0755'), 'verify.py': (16379, 'be80388f0f1661a09825708c1c8169f2972160d2e3020f705f55433b0c83a230', '0755')}
FALSE_AUTHORITY = (
    "wired", "default_projection_pointer_changed", "default_recall_query_changed",
    "production_projection_gate", "production_authority", "external_effects",
    "operator_accepted", "promoted", "callers_ratchet",
)

def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: hotfix.py EXTRACTED_PAYLOAD_DIR")
    root=Path(sys.argv[1]).resolve()
    if not root.is_dir():
        raise SystemExit(f"payload directory missing: {root}")
    entries=list(root.iterdir())
    unsafe=[p.name for p in entries if p.is_symlink() or not p.is_file()]
    if unsafe:
        raise SystemExit(f"payload contains non-regular entries: {sorted(unsafe)}")
    actual={p.name for p in entries}
    if actual != set(EXPECTED):
        raise SystemExit(f"payload inventory mismatch: missing={sorted(set(EXPECTED)-actual)} extra={sorted(actual-set(EXPECTED))}")
    observed={}
    for name,(size,sha,mode) in sorted(EXPECTED.items()):
        path=root/name
        actual_mode=f"{stat.S_IMODE(path.stat().st_mode):04o}"
        if path.stat().st_size != size or digest(path) != sha or actual_mode != mode:
            raise SystemExit(f"payload drift: {name} size={path.stat().st_size} sha={digest(path)} mode={actual_mode}")
        observed[name]={"size":size,"sha256":sha,"mode":mode}
    with tempfile.TemporaryDirectory(prefix="p034-v7-pycompile-") as temp:
        for name in ("apply.py","clippy_delta.py","run.py","verify.py"):
            py_compile.compile(str(root/name), cfile=str(Path(temp)/(name+"c")), doraise=True)
    status=json.loads((root/"P034_STATUS.json").read_text())
    current=status.get("current_tranche") or {}
    authority=status.get("authority") or {}
    publication=status.get("publication_policy") or {}
    if not (
        current.get("id")=="P0.3.4" and current.get("implemented") is True
        and current.get("qualified") is False and current.get("wired") is False
        and current.get("exact_changed_path_count")==14
        and current.get("exact_predecessor_clippy_delta_gate") is True
        and current.get("dormant_source_dead_code_expectation") is True
        and current.get("undeclared_lint_suppressions_added") is False
        and current.get("paired_qualification_receipt_required") is True
        and publication.get("base_head")=="eddcb59ca43a76ac83b64507983bd908f406ff48"
        and publication.get("base_branch")=="codex/hepta-intelligence-evidence-resolver-v4-20260828"
        and publication.get("candidate_branch")=="codex/hepta-intelligence-legacy-governance-v3-20260829"
        and publication.get("build_bundle_before_publication") is True
        and publication.get("publish_after_paired_qualification") is True
        and publication.get("force_push") is False and publication.get("history_rewrite") is False
    ):
        raise SystemExit("payload status/publication contract drifted")
    drift=[key for key in FALSE_AUTHORITY if authority.get(key) is not False]
    if drift:
        raise SystemExit(f"payload authority drifted: {drift}")
    apply=(root/"apply.py").read_text()
    required_apply=("CANDIDATE_BUNDLE", "bundle_ref", "qualified_before_publication", "EXPECTED_CHANGED_PATHS", '"published": False')
    if not all(marker in apply for marker in required_apply) or "force-with-lease" in apply:
        raise SystemExit("payload apply publication boundary drifted")
    clippy=(root/"clippy_delta.py").read_text()
    if not all(marker in clippy for marker in ("EXPECTED_CHANGED_PATHS", "--cap-lints=warn", "undeclared_lint_suppressions_added", "current_parent == BASELINE_SHA")):
        raise SystemExit("payload Clippy delta contract drifted")
    workflow=(root/"candidate-workflow.yml").read_text()
    if not all(marker in workflow for marker in ("legacy-governance-v3-20260829", "linux-arm64", "linux-x64", "paired-exact-head-qualification")):
        raise SystemExit("payload candidate workflow drifted")
    receipt={
        "schema":"hepta_intelligence_p0_3_4_payload_validation_v2",
        "archive_sha256":ARCHIVE_SHA256,
        "files":observed,
        "python_compile":True,
        "exact_changed_path_count":14,
        "paired_before_publication":True,
        "force_push":False,
        "history_rewrite":False,
        "authority_all_false":True,
        "status":"PASS_P0_3_4_V7_RECONSTRUCTION_PAYLOAD",
    }
    print(json.dumps(receipt,indent=2,sort_keys=True))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
