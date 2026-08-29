#!/usr/bin/env python3
"""Make legacy P0.2 gates monotonic across fail-closed successors."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path.cwd()
VERIFIER = ROOT / "scripts/verify-hepta-intelligence-grounding-ledger.py"
WORKFLOW = ROOT / ".github/workflows/hepta-intelligence-grounding-ledger.yml"
STATUS = ROOT / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json"
P033_HEAD = "eddcb59ca43a76ac83b64507983bd908f406ff48"
P033_RUN = 33226392404
P033_ARTIFACT = 9707307831

OLD_STATUS_CHECKS = '''    checks["status.p0_2"] = (
        status.get("current_tranche", {}).get("id") == "P0.2"
        and status.get("current_tranche", {}).get("qualified") is False
    )
    authority = status.get("authority", {})
    checks["status.authority_false"] = bool(authority) and all(
        value is False for value in authority.values()
    )
    checks["status.p0_3_inactive"] = (
        status.get("next_tranche", {}).get("id") == "P0.3"
        and status.get("next_tranche", {}).get("activation") == "blocked"
    )
'''
NEW_STATUS_CHECKS = '''    capabilities = {
        capability.get("id"): capability
        for capability in status.get("capabilities", [])
        if isinstance(capability, dict)
    }
    durable = capabilities.get("durable_fact_grounding_ledger", {})
    checks["status.p0_2_lineage_retained"] = (
        durable.get("implemented") is True
        and durable.get("wired") is False
        and durable.get("promoted") is False
    )
    authority = status.get("authority", {})
    checks["status.authority_false"] = bool(authority) and all(
        value is False for value in authority.values()
    )
    current = status.get("current_tranche", {})
    current_id = str(current.get("id", ""))
    next_tranche = status.get("next_tranche", {})
    claims = current.get("claims", {})
    original_boundary = (
        current_id == "P0.2"
        and next_tranche.get("id") == "P0.3"
        and next_tranche.get("activation") == "blocked"
    )
    fail_closed_successor = (
        (current_id == "P0.3" or current_id.startswith("P0.3."))
        and bool(authority)
        and all(value is False for value in authority.values())
        and claims.get("production_projection_gate") is not True
        and claims.get("production_authority") is not True
        and claims.get("external_effects") is not True
    )
    checks["status.forward_progress_fail_closed"] = (
        original_boundary or fail_closed_successor
    )
'''
OLD_FILES_END = '''    "tranche": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P0_2_IMPLEMENTATION_2026-08-28.md",
}
'''
NEW_FILES_END = '''    "tranche": ROOT
    / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P0_2_IMPLEMENTATION_2026-08-28.md",
    "workflow": ROOT
    / ".github/workflows/hepta-intelligence-grounding-ledger.yml",
}
'''
OLD_READS = '''    tranche = FILES["tranche"].read_text(encoding="utf-8")
    status = json.loads(FILES["status"].read_text(encoding="utf-8"))
'''
NEW_READS = '''    tranche = FILES["tranche"].read_text(encoding="utf-8")
    workflow = FILES["workflow"].read_text(encoding="utf-8")
    status = json.loads(FILES["status"].read_text(encoding="utf-8"))
'''
OLD_BAZEL_CHECK = '''    checks["bazel.component_data"] = '"grounding-migrations/**"' in bazel
'''
NEW_BAZEL_CHECK = '''    checks["bazel.component_data"] = '"grounding-migrations/**"' in bazel
    checks["ci.p0_2_governed_formatter"] = (
        "cargo fmt --all -- --check" not in workflow
        and "toolchain: 1.95.0" in workflow
        and "rustfmt --edition 2024 --config skip_children=true --check" in workflow
        and all(
            path in workflow
            for path in (
                "hepta-memory/src/framing.rs",
                "hepta-memory/src/fact_grounding/durable.rs",
                "hepta-memory/src/fact_grounding/durable/schema.rs",
                "hepta-memory/src/fact_grounding/durable/grounding.rs",
                "hepta-memory/src/fact_grounding/durable/grounding/prepare.rs",
                "hepta-memory/src/fact_grounding/durable/grounding/ledger.rs",
                "hepta-memory/src/fact_grounding/durable/grounding/ledger/insert.rs",
                "hepta-memory/src/fact_grounding/durable/grounding/ledger/verify.rs",
                "hepta-memory/src/fact_grounding/durable/grounding/ledger/support.rs",
                "hepta-memory/src/fact_grounding/durable/tests.rs",
            )
        )
    )
'''
OLD_FORMAT_STEP = '''      - name: Check workspace formatting
        working-directory: codex-rs
        run: cargo fmt --all -- --check
'''
NEW_FORMAT_STEP = '''      - name: Check P0.2 governed Rust formatting
        working-directory: codex-rs
        run: |
          rustfmt --edition 2024 --config skip_children=true --check \\
            hepta-memory/src/framing.rs \\
            hepta-memory/src/fact_grounding/durable.rs \\
            hepta-memory/src/fact_grounding/durable/schema.rs \\
            hepta-memory/src/fact_grounding/durable/grounding.rs \\
            hepta-memory/src/fact_grounding/durable/grounding/prepare.rs \\
            hepta-memory/src/fact_grounding/durable/grounding/ledger.rs \\
            hepta-memory/src/fact_grounding/durable/grounding/ledger/insert.rs \\
            hepta-memory/src/fact_grounding/durable/grounding/ledger/verify.rs \\
            hepta-memory/src/fact_grounding/durable/grounding/ledger/support.rs \\
            hepta-memory/src/fact_grounding/durable/tests.rs
'''


def replace_once(path: Path, old: str, new: str, label: str) -> None:
    text = path.read_text(encoding="utf-8")
    if new in text:
        return
    if text.count(old) != 1:
        raise SystemExit(f"{label} drifted")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def patch_verifier() -> None:
    replace_once(
        VERIFIER,
        OLD_FILES_END,
        NEW_FILES_END,
        "legacy P0.2 verifier file inventory",
    )
    replace_once(
        VERIFIER,
        OLD_READS,
        NEW_READS,
        "legacy P0.2 verifier workflow read",
    )
    replace_once(
        VERIFIER,
        OLD_BAZEL_CHECK,
        NEW_BAZEL_CHECK,
        "legacy P0.2 verifier CI contract",
    )
    replace_once(
        VERIFIER,
        OLD_STATUS_CHECKS,
        NEW_STATUS_CHECKS,
        "legacy P0.2 status assertion block",
    )


def patch_workflow() -> None:
    text = WORKFLOW.read_text(encoding="utf-8")
    if "toolchain: 1.95.0" not in text:
        if text.count("toolchain: 1.88.0") != 1:
            raise SystemExit("legacy P0.2 workflow toolchain drifted")
        text = text.replace("toolchain: 1.88.0", "toolchain: 1.95.0", 1)
    if NEW_FORMAT_STEP not in text:
        if text.count(OLD_FORMAT_STEP) != 1:
            raise SystemExit("legacy P0.2 workspace formatter step drifted")
        text = text.replace(OLD_FORMAT_STEP, NEW_FORMAT_STEP, 1)
    WORKFLOW.write_text(text, encoding="utf-8")


def patch_status() -> None:
    status = json.loads(STATUS.read_text(encoding="utf-8"))
    status["repository"] = "ProfHepta/hepta-private-ci"
    status["repository_renamed_from"] = "ProfAlexQI/hepta-private-ci"
    status["generated_at"] = "2026-08-29"
    status["lineage_revalidation"] = {
        "p0_2_durable_grounding_ledger": {
            "independent_branch_qualified": False,
            "source_contract_monotonic": True,
            "formatter_scope": "p0_2_governed_rust_files",
            "revalidated_by_descendant_exact_head": P033_HEAD,
            "revalidated_by_descendant_run": P033_RUN,
            "revalidated_by_descendant_artifact": P033_ARTIFACT,
            "same_snapshot_ledger_verification": True,
        },
        "p0_3_2_shared_semantic_projection_planner": {
            "qualified": True,
            "exact_head": "fa59bb090043ba8d6fbf0991b167779d2385888c",
            "exact_run": 33190943793,
            "exact_artifact": 9693847531,
        },
        "p0_3_3_host_owned_evidence_resolution": {
            "qualified": True,
            "exact_head": P033_HEAD,
            "exact_run": P033_RUN,
            "exact_artifact": P033_ARTIFACT,
        },
    }
    current = status.get("current_tranche")
    if isinstance(current, dict) and current.get("id") == "P0.3":
        qualification = current.setdefault("qualification", {})
        qualification["p0_3_3_source_qualified"] = True
        qualification["p0_3_3_exact_head"] = P033_HEAD
        qualification["p0_3_3_exact_run"] = P033_RUN
        qualification["p0_3_3_exact_artifact"] = P033_ARTIFACT
    STATUS.write_text(
        json.dumps(status, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def main() -> int:
    patch_workflow()
    patch_verifier()
    patch_status()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
