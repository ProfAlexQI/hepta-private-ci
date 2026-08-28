#!/usr/bin/env python3
"""Format, qualify, squash, and publish the exact P0.3.3 candidate fail-closed."""

from __future__ import annotations

import json
import os
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WORKSPACE = ROOT / "codex-rs"
BRANCH = "codex/hepta-intelligence-evidence-resolver-v4-20260828"
P032_HEAD = os.environ["P032_HEAD"]

RUST_FILES = [
    "ext/hepta-memory/src/framing.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/receipt.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/resolver.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/schema.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/support.rs",
    "ext/hepta-memory/src/cognitive/evidence_resolver_v4/tests.rs",
]

ALLOWED_DELTA = {
    ".github/workflows/hepta-intelligence-evidence-resolver-v4.yml",
    "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4.rs",
    "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/receipt.rs",
    "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/resolver.rs",
    "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/schema.rs",
    "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/support.rs",
    "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/tests.rs",
    "codex-rs/ext/hepta-memory/src/framing.rs",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_KG_DEVELOPMENT_PLAN_V3_2_2026-08-28.md",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_KG_EXECUTION_STATUS_V3_2.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P0_3_3_HOST_EVIDENCE_RESOLVER_2026-08-28.md",
    "scripts/check-hepta-intelligence-p0-3-3-clippy.py",
    "scripts/run-hepta-intelligence-evidence-resolver-v5.py",
    "scripts/verify-hepta-intelligence-evidence-resolver-v4.py",
}


def run(*command: str, cwd: Path = ROOT, capture: bool = False) -> str:
    result = subprocess.run(
        list(command),
        cwd=cwd,
        text=True,
        stdout=subprocess.PIPE if capture else None,
        stderr=subprocess.PIPE if capture else None,
        check=False,
    )
    if result.returncode != 0:
        detail = ""
        if capture:
            detail = f"\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        raise SystemExit(f"command failed ({result.returncode}): {' '.join(command)}{detail}")
    return result.stdout.strip() if capture else ""


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one replacement target, observed {count}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def main() -> None:
    staged_head = run("git", "rev-parse", "HEAD", capture=True)
    run("git", "merge-base", "--is-ancestor", P032_HEAD, staged_head)

    replace_once(
        ROOT / "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/resolver.rs",
        ".try_fold(0usize, |total, count| total.checked_add(count))",
        ".try_fold(0usize, usize::checked_add)",
    )
    replace_once(
        ROOT / "codex-rs/ext/hepta-memory/src/cognitive/evidence_resolver_v4/tests.rs",
        """    assert!(!GROUNDED_TOOL_V4_REGISTERED);
    assert!(!GROUNDED_TOOL_V4_PRODUCTION_AUTHORITY);
    assert!(!MODEL_SUPPLIED_BYTE_OFFSETS);
    assert!(!MODEL_SUPPLIED_DIGESTS);""",
        """    const {
        assert!(!GROUNDED_TOOL_V4_REGISTERED);
        assert!(!GROUNDED_TOOL_V4_PRODUCTION_AUTHORITY);
        assert!(!MODEL_SUPPLIED_BYTE_OFFSETS);
        assert!(!MODEL_SUPPLIED_DIGESTS);
    }""",
    )

    for relative in (
        ".github/workflows/p0-3-3-format-clippy-fix-v1.yml",
        ".github/workflows/p0-3-3-finalize-probe-v1.yml",
        "scripts/finalize-hepta-intelligence-p0-3-3-v2.py",
    ):
        path = ROOT / relative
        if path.exists():
            path.unlink()

    run("rustfmt", "--edition", "2024", *RUST_FILES, cwd=WORKSPACE)
    run("git", "config", "user.name", "Qian QI")
    run(
        "git",
        "config",
        "user.email",
        "102159240+ProfAlexQI@users.noreply.github.com",
    )
    run("git", "add", "-A")
    run("git", "reset", "--soft", P032_HEAD)
    run("git", "diff", "--cached", "--check")
    run(
        "git",
        "commit",
        "--no-gpg-sign",
        "-m",
        "feat(memory): add host-owned evidence resolver P0.3.3",
    )

    candidate_head = run("git", "rev-parse", "HEAD", capture=True)
    count = run("git", "rev-list", "--count", f"{P032_HEAD}..HEAD", capture=True)
    if count != "1":
        raise SystemExit(f"P0.3.3 candidate must contain one commit above P0.3.2, got {count}")
    changed = set(
        run("git", "diff", "--name-only", f"{P032_HEAD}..HEAD", capture=True).splitlines()
    )
    if changed != ALLOWED_DELTA:
        raise SystemExit(
            "P0.3.3 governed delta mismatch\n"
            f"missing={sorted(ALLOWED_DELTA - changed)}\n"
            f"unexpected={sorted(changed - ALLOWED_DELTA)}"
        )

    run("python3", "scripts/run-hepta-intelligence-evidence-resolver-v5.py")
    receipt_path = (
        ROOT
        / "artifacts/hepta-intelligence-evidence-resolver-v5/qualification-receipt.json"
    )
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    failed = [check["id"] for check in receipt["checks"] if not check["passed"]]
    if receipt["head"] != candidate_head:
        raise SystemExit("P0.3.3 receipt is not bound to the candidate head")
    if failed or receipt["qualified"] is not True:
        raise SystemExit(f"P0.3.3 candidate failed preflight: {failed}")
    if receipt["p0_3_2_dependency_qualified"] is not True:
        raise SystemExit("P0.3.2 dependency binding is not qualified")
    for key in (
        "wired",
        "tool_v4_registered",
        "default_projection_pointer_changed",
        "default_recall_query_changed",
        "production_projection_gate",
        "production_authority",
        "external_effects",
        "operator_accepted",
        "promoted",
        "callers_ratchet",
    ):
        if receipt[key] is not False:
            raise SystemExit(f"authority boundary unexpectedly raised: {key}")

    run("git", "diff", "--quiet", "--", ".", ":(exclude)artifacts")
    run(
        "git",
        "push",
        f"--force-with-lease=refs/heads/{BRANCH}:{staged_head}",
        "origin",
        f"HEAD:refs/heads/{BRANCH}",
    )
    print(
        json.dumps(
            {
                "schema": "hepta_intelligence_p0_3_3_finalizer_v2",
                "old_head": staged_head,
                "p0_3_2_head": P032_HEAD,
                "candidate_head": candidate_head,
                "qualified_preflight": True,
                "governed_files": sorted(ALLOWED_DELTA),
                "temporary_files_removed": True,
                "wired": False,
                "production_authority": False,
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
