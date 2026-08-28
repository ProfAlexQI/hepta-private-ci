#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    target = Path(path)
    text = target.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one replacement target, found {count}")
    target.write_text(text.replace(old, new, 1), encoding="utf-8")


def patch_parent_workflow() -> None:
    path = ".github/workflows/hepta-intelligence-p1-1c-offline-efficacy.yml"
    replace_once(
        path,
        "  BASE_BRANCH: codex/hepta-intelligence-local-embedding-index-v1b-20260828\n",
        """  BASE_BRANCH: codex/hepta-intelligence-local-embedding-index-v1b-20260828
  P1C_SOURCE_BRANCH: codex/hepta-intelligence-offline-efficacy-p1c-20260828
  P1C_SOURCE_SHA: fe33565ce74c013e574c307e4fab101820c0ea88
""",
    )
    checkout = """      - name: Checkout exact source
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

"""
    route = r'''      - name: Route exact P1.1c parent or preserved descendant stack
        id: p1c_route
        run: |
          set -euo pipefail
          mkdir -p "$ARTIFACT_DIR"
          git fetch --no-tags origin \
            "+refs/heads/$P1C_SOURCE_BRANCH:refs/remotes/origin/$P1C_SOURCE_BRANCH"
          source_sha="$(git rev-parse "refs/remotes/origin/$P1C_SOURCE_BRANCH")"
          test "$source_sha" = "$P1C_SOURCE_SHA"
          test "$(git merge-base HEAD "$source_sha")" = "$source_sha"

          python3 - "$source_sha" <<'PY'
          import json
          import os
          import subprocess
          import sys
          from pathlib import Path

          source = sys.argv[1]
          cargo_path = "codex-rs/hepta-memory-p1-1c-qualification/Cargo.toml"
          workflow_path = ".github/workflows/hepta-intelligence-p1-1c-offline-efficacy.yml"
          frozen_paths = [
              "codex-rs/hepta-memory-p1-1c-qualification/fixtures/p1_1c_multilingual_seed.tsv",
              "codex-rs/hepta-memory-p1-1c-qualification/src/bin/p1_1c_receipt.rs",
              "codex-rs/hepta-memory-p1-1c-qualification/src/corpus.rs",
              "codex-rs/hepta-memory-p1-1c-qualification/src/digest.rs",
              "codex-rs/hepta-memory-p1-1c-qualification/src/evaluation.rs",
              "codex-rs/hepta-memory-p1-1c-qualification/src/kg.rs",
              "codex-rs/hepta-memory-p1-1c-qualification/src/lib.rs",
              "codex-rs/hepta-memory-p1-1c-qualification/src/metrics.rs",
              "codex-rs/hepta-memory-p1-1c-qualification/src/reranker.rs",
              "codex-rs/hepta-memory-p1-1c-qualification/tests/p1_1c.rs",
              "plans/hepta-intelligence/P1-1C_EXECUTION_STATUS.json",
              "plans/hepta-intelligence/P1-1C_MULTILINGUAL_OFFLINE_EFFICACY_PLAN.md",
              "scripts/verify-hepta-intelligence-p1-1c-offline-efficacy.py",
          ]
          unchanged = subprocess.run(
              ["git", "diff", "--quiet", f"{source}..HEAD", "--", *frozen_paths],
              check=False,
          ).returncode == 0
          if not unchanged:
              raise SystemExit("descendant stack changed frozen P1.1c semantic evidence")

          source_cargo = subprocess.check_output(
              ["git", "show", f"{source}:{cargo_path}"], text=True
          )
          current_cargo = Path(cargo_path).read_text(encoding="utf-8")
          workspace_cargo = source_cargo.rstrip("\n") + "\n\n[workspace]\n"
          if current_cargo not in {source_cargo, workspace_cargo}:
              raise SystemExit("P1.1c Cargo manifest drift exceeds the workspace-isolation marker")

          changed = subprocess.check_output(
              ["git", "diff", "--name-only", f"{source}..HEAD"], text=True
          ).splitlines()
          descendant_paths = sorted(
              path for path in changed if path not in {cargo_path, workflow_path}
          )
          mode = "descendant" if descendant_paths else "parent"
          if mode == "descendant":
              child_workflow = Path(
                  ".github/workflows/hepta-intelligence-p1-1c2-reviewed-efficacy.yml"
              )
              child_verifier = Path(
                  "scripts/verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py"
              )
              if not child_workflow.is_file() or not child_verifier.is_file():
                  raise SystemExit("descendant route lacks its dedicated fail-closed qualification")

          receipt = {
              "schema": "hepta.intelligence.p1_1c.stack_route.v1",
              "mode": mode,
              "p1c_source_commit": source,
              "head_commit": subprocess.check_output(
                  ["git", "rev-parse", "HEAD"], text=True
              ).strip(),
              "frozen_semantic_paths": frozen_paths,
              "frozen_semantics_unchanged": True,
              "cargo_workspace_isolation_only": current_cargo == workspace_cargo,
              "descendant_paths": descendant_paths,
              "child_qualification_present": mode == "parent" or True,
              "qualified": False,
              "runtime_wired": False,
              "production_authority": False,
              "promotion": False,
          }
          artifact_dir = Path(os.environ["ARTIFACT_DIR"])
          (artifact_dir / "stack-route.json").write_text(
              json.dumps(receipt, indent=2, sort_keys=True) + "\n",
              encoding="utf-8",
          )
          with Path(os.environ["GITHUB_OUTPUT"]).open("a", encoding="utf-8") as output:
              output.write(f"mode={mode}\n")
          PY

'''
    replace_once(path, checkout, checkout + route)

    parent_steps = (
        "Verify stacked base and exact changed-path allowlist",
        "Run fail-closed source gate",
        "Isolate Cargo outputs",
        "Install Rust 1.95.0",
        "Record and verify pinned toolchain",
        "Check Rust formatting",
        "Run all unit and integration tests",
        "Check all Rust targets",
        "Run strict Clippy",
        "Produce deterministic receipt twice",
        "Validate machine receipt and redaction boundary",
        "Verify clean source tree",
        "Write qualification receipt",
    )
    for name in parent_steps:
        replace_once(
            path,
            f"      - name: {name}\n",
            f"      - name: {name}\n        if: steps.p1c_route.outputs.mode == 'parent'\n",
        )


def patch_child_workflow_allowlist() -> None:
    path = ".github/workflows/hepta-intelligence-p1-1c2-reviewed-efficacy.yml"
    marker = '              ".github/workflows/hepta-intelligence-p1-1c2-reviewed-efficacy.yml",\n'
    replace_once(
        path,
        marker,
        '              ".github/workflows/hepta-intelligence-p1-1c-offline-efficacy.yml",\n'
        + marker,
    )


def patch_plan_and_documents() -> None:
    plan_path = "plans/hepta-intelligence/P1-1C2_REVIEWED_CORPUS_EFFICACY_PLAN.md"
    section = """
## Parent-tranche stack routing

P1.1c.2 adds only the empty `[workspace]` isolation marker to the frozen P1.1c manifest. The inherited P1.1c workflow is stack-aware and fails closed unless every P1.1c semantic source, fixture, plan and verifier byte remains equal to exact P1.1c source commit `fe33565ce74c013e574c307e4fab101820c0ea88`.

A preserved descendant emits `hepta.intelligence.p1_1c.stack_route.v1`, skips duplicate parent execution, and requires the P1.1c.2 workflow plus source verifier to be present. Any parent semantic drift fails before the child gate. This routing does not qualify P1.1c, P1.1c.1 or P1.1c.2 and grants no runtime or production authority.

"""
    replace_once(plan_path, "## Exit semantics\n", section + "## Exit semantics\n")

    status_path = Path("plans/hepta-intelligence/P1-1C2_EXECUTION_STATUS.json")
    status = json.loads(status_path.read_text(encoding="utf-8"))
    status["implementation"].update(
        {
            "parent_descendant_stack_routing": True,
            "parent_semantic_tree_frozen": True,
            "parent_workspace_isolation_only": True,
        }
    )
    status_path.write_text(json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    receipt_path = Path("plans/hepta-intelligence/P1-1C2_IMPLEMENTATION_RECEIPT.json")
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    receipt["claims"].update(
        {
            "parent_descendant_stack_routing": True,
            "parent_semantic_tree_frozen": True,
            "parent_workspace_isolation_only": True,
        }
    )
    receipt_path.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> None:
    patch_parent_workflow()
    patch_child_workflow_allowlist()
    patch_plan_and_documents()


if __name__ == "__main__":
    main()
