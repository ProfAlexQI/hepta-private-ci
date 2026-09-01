#!/usr/bin/env python3
from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CURRENT_DEFAULT = "integration/vnext-main-20260811"
FUTURE_DEFAULT = "main"


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, value: dict) -> None:
    path.write_text(
        json.dumps(value, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )


def update_current() -> None:
    path = ROOT / "docs/CURRENT.json"
    current = load_json(path)
    current["repository"]["defaultBranch"] = CURRENT_DEFAULT
    current["candidate"].update(
        {
            "pullRequest": 286,
            "targetBranch": FUTURE_DEFAULT,
            "sourceHead": "resolved_by_exact_main_default_mirror_receipt",
            "sourceTree": "resolved_by_exact_main_default_mirror_receipt",
            "mergeCandidate": "not_applicable_after_ancestry_consolidation",
            "relationship": "main_and_current_default_same_tip_pending_repository_admin_switch",
        }
    )
    current["currentWorkPackage"] = "DOC-2-DEFAULT-BRANCH-SELECTION"
    supersedes = current.setdefault("supersedes", [])
    marker = "pre-consolidation multi-branch repository ref surface"
    if marker not in supersedes:
        supersedes.append(marker)
    write_json(path, current)


def update_work_packages() -> None:
    path = ROOT / "docs/delivery/WORK_PACKAGES.json"
    registry = load_json(path)
    registry["currentPackage"] = "DOC-2-DEFAULT-BRANCH-SELECTION"
    desired_states = {
        "DOC-0-CANONICAL-DOCUMENT-CONSOLIDATION": "source_implemented",
        "DOC-1-V8-SEMANTIC-UPGRADE": "source_implemented",
        "DOC-2-DEFAULT-BRANCH-SELECTION": "blocked_external",
        "DOC-REGISTRY-CLOSED-WORLD": "source_implemented",
    }
    observed: set[str] = set()
    for package in registry["packages"]:
        package_id = package["id"]
        if package_id in desired_states:
            package["state"] = desired_states[package_id]
            observed.add(package_id)
    missing = set(desired_states) - observed
    if missing:
        raise SystemExit(f"missing document packages: {sorted(missing)}")
    write_json(path, registry)


def update_development() -> None:
    path = ROOT / "docs/DEVELOPMENT.md"
    text = path.read_text(encoding="utf-8")

    text, replacements = re.subn(
        r"\*\*Status:\*\*[^\n]*",
        "**Status:** canonical V8 content is mirrored on `main`; GitHub still names "
        "`integration/vnext-main-20260811` as the default branch, so default-branch "
        "selection remains an external repository-administration gate.",
        text,
        count=1,
    )
    if replacements != 1:
        raise SystemExit("unable to update DEVELOPMENT status")

    cleanup_start = text.index(
        "The exact default baseline contains a 138-file Dropbox development snapshot"
    )
    cleanup_end = text.index(
        "\n\n`python3 scripts/hepta-docs.py verify`",
        cleanup_start,
    )
    cleanup = (
        "The pre-selection baseline contained 143 historical development paths. V8 "
        "deleted that complete set in the same commit that installed the canonical "
        "document system. Git ancestry now preserves every pre-consolidation branch "
        "tip without overlaying an obsolete branch tree or reintroducing an in-tree "
        "historical plan. Code-consumed APIs, schemas, policies, migrations, tests "
        "and implementation contracts remain protected."
    )
    text = text[:cleanup_start] + cleanup + text[cleanup_end:]

    section_start = text.index("## 3. Current truthful baseline")
    section_end = text.index(
        "## 4. Immutable execution, authority and data invariants",
        section_start,
    )
    section = (
        "## 3. Current truthful baseline\n\n"
        "The selected V8 content is present on both `main` and "
        "`integration/vnext-main-20260811` at the same exact tip. Every one of the "
        "789 pre-consolidation branch tips is reachable from that line as Git "
        "ancestry; obsolete branch trees were not overlaid. All 64 outstanding pull "
        "requests were closed as superseded, and all 787 other branch refs were "
        "removed by a bounded operation after exact ancestry verification.\n\n"
        "GitHub still reports `integration/vnext-main-20260811` as the repository "
        "default because the Actions integration is not permitted to mutate "
        "repository administration settings. Therefore "
        "`DOC-2-DEFAULT-BRANCH-SELECTION` remains `blocked_external`. The final "
        "administrative transition is to select the existing `main` ref as default "
        "and then delete the old default ref. Until that observable state exists, "
        "canonical registries must not claim that `main` is already the GitHub "
        "default branch.\n\n"
        "Exact live head, tree, CI, review and operator facts remain dynamic evidence "
        "and must be resolved from current external receipts. This consolidation "
        "grants no runtime, model, provider, tool, network, filesystem, secret, "
        "Matrix, fleet, operator, promotion or release authority.\n\n"
    )
    text = text[:section_start] + section + text[section_end:]
    path.write_text(text, encoding="utf-8")


def main() -> None:
    update_current()
    update_work_packages()
    update_development()


if __name__ == "__main__":
    main()
