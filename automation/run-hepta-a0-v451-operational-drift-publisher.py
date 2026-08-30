#!/usr/bin/env python3
"""Build a validated Q0-single-parent A0 operational-document drift closure."""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import textwrap
from typing import Any, NoReturn


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_A0_OPERATIONAL_DRIFT_PUBLISHER: {message}")


def run(*args: str, cwd: Path, capture: bool = False) -> str:
    result = subprocess.run(
        list(args),
        cwd=cwd,
        check=True,
        text=True,
        stdout=subprocess.PIPE if capture else None,
    )
    return result.stdout.strip() if capture else ""


def pairs(items: list[tuple[str, Any]]) -> dict[str, Any]:
    value: dict[str, Any] = {}
    for key, item in items:
        if key in value:
            fail(f"duplicate JSON key: {key}")
        value[key] = item
    return value


def load(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=pairs)
    except SystemExit:
        raise
    except Exception as exc:
        fail(f"cannot parse {path}: {exc}")
    if not isinstance(value, dict):
        fail(f"{path} must contain an object")
    return value


def dump(path: Path, value: Any) -> None:
    path.write_text(
        json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def canonical(value: Any) -> bytes:
    return json.dumps(
        value, sort_keys=True, separators=(",", ":"), ensure_ascii=False
    ).encode("utf-8")


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def exact_env(name: str) -> str:
    value = os.environ.get(name)
    if not value:
        fail(f"missing environment variable {name}")
    return value


def patch_source(root: Path, report_dir: Path) -> dict[str, Any]:
    plan = root / "plans/hepta-intelligence"
    integration_path = plan / "HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
    current_path = plan / "HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
    registry_path = plan / "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
    validator_path = root / "scripts/hepta-intelligence-current-truth.py"

    spec_rel = (
        "plans/hepta-intelligence/"
        "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
    )
    integration_rel = (
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
    )
    current_rel = "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json"
    registry_rel = (
        "plans/hepta-intelligence/"
        "HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
    )
    validator_rel = "scripts/hepta-intelligence-current-truth.py"

    old_head = exact_env("OLD_A0_HEAD")
    old_tail = exact_env("PREVIOUS_PROVENANCE_TAIL")
    spec_version = exact_env("SPEC_VERSION")
    spec_sha = exact_env("SPEC_SHA256")
    old_spec_version = exact_env("OLD_SPEC_VERSION")
    old_spec_sha = exact_env("OLD_SPEC_SHA256")

    integration = load(integration_path)
    expected_old_document = {
        "classification": "SUBORDINATE_EXECUTION_SPEC",
        "content_sha256": old_spec_sha,
        "current_plan_authority": False,
        "path": spec_rel,
        "production_authority": False,
        "promotion_authority": False,
        "version": old_spec_version,
    }
    if integration.get("operational_documents") != [expected_old_document]:
        fail("unexpected pre-repair operational_documents bytes")
    integration["operational_documents"] = [
        {
            **expected_old_document,
            "content_sha256": spec_sha,
            "version": spec_version,
        }
    ]
    dump(integration_path, integration)
    integration_sha = sha256_file(integration_path)

    current = load(current_path)
    provenance = current.get("a0_previous_exact_head_provenance")
    if not isinstance(provenance, list) or not provenance:
        fail("A0 provenance list missing")
    if provenance[-1] != old_tail or old_head in provenance:
        fail("A0 provenance tail does not match the expected predecessor")
    provenance.append(old_head)
    dump(current_path, current)
    current_sha = sha256_file(current_path)

    validator = validator_path.read_text(encoding="utf-8")
    if "def validate_operational_documents(" in validator:
        fail("operational-document validator already exists unexpectedly")

    helper_marker = "\ndef validate_gap_entries(\n"
    helper = textwrap.dedent(
        '''

        def validate_operational_documents(
            integration: dict[str, Any], spec_sha256: str
        ) -> None:
            expected = [
                {
                    "classification": "SUBORDINATE_EXECUTION_SPEC",
                    "content_sha256": spec_sha256,
                    "current_plan_authority": False,
                    "path": "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md",
                    "production_authority": False,
                    "promotion_authority": False,
                    "version": SPEC_VERSION,
                }
            ]
            require(
                integration.get("operational_documents") == expected,
                "integration operational document drift",
            )
        '''
    )
    if validator.count(helper_marker) != 1:
        fail("cannot locate validator helper insertion point")
    validator = validator.replace(helper_marker, helper + helper_marker, 1)

    call_marker = '    global_ledger = integration.get("gap_closure_ledger", {})\n'
    if validator.count(call_marker) != 1:
        fail("cannot locate operational-document call insertion point")
    validator = validator.replace(
        call_marker,
        '    validate_operational_documents(integration, sha(PATHS["spec"]))\n'
        + call_marker,
        1,
    )

    provenance_marker = (
        f'            "{old_tail}",\n'
        '        ],\n'
        '        "A0 provenance drift",\n'
    )
    provenance_replacement = (
        f'            "{old_tail}",\n'
        f'            "{old_head}",\n'
        '        ],\n'
        '        "A0 provenance drift",\n'
    )
    if validator.count(provenance_marker) != 1:
        fail("cannot locate exact A0 provenance assertion")
    validator = validator.replace(provenance_marker, provenance_replacement, 1)
    validator_path.write_text(validator, encoding="utf-8")

    registry = load(registry_path)
    inputs = registry.get("registered_canonical_inputs")
    if not isinstance(inputs, list):
        fail("registered_canonical_inputs missing")
    by_path = {item.get("path"): item for item in inputs if isinstance(item, dict)}
    current_entry = by_path.get(current_rel)
    integration_entry = by_path.get(integration_rel)
    self_entry = by_path.get(registry_rel)
    if current_entry is None or integration_entry is None or self_entry is None:
        fail("required canonical input entry missing")
    if (
        current_entry.get("content_sha256")
        != "3d0124669946e1b97a1c0ae6c46a38cfd8d4e64a0bde4a9b85c9529b8ec4454f"
    ):
        fail("unexpected pre-repair current-plan digest")
    if (
        integration_entry.get("content_sha256")
        != "95d79fd4786c726e165fbc35e097b9bb511fa0023fd36f482f361b36edeb1b16"
    ):
        fail("unexpected pre-repair integration-candidate digest")
    if (
        self_entry.get("content_sha256")
        != "b4a600fb4e376ec68746034bba3528371aeb1db7519c31ef8d2d0638b373751d"
    ):
        fail("unexpected pre-repair registry self digest")

    current_entry["content_sha256"] = current_sha
    integration_entry["content_sha256"] = integration_sha
    self_entry["content_sha256"] = None
    registry_self_sha = hashlib.sha256(canonical(registry)).hexdigest()
    self_entry["content_sha256"] = registry_self_sha
    dump(registry_path, registry)

    registry_check = load(registry_path)
    check_self = next(
        item
        for item in registry_check["registered_canonical_inputs"]
        if item["path"] == registry_rel
    )
    observed_self = check_self["content_sha256"]
    check_self["content_sha256"] = None
    if hashlib.sha256(canonical(registry_check)).hexdigest() != observed_self:
        fail("registry self digest did not reproduce")

    report = {
        "schema": "hepta_a0_operational_document_drift_source_patch_v1",
        "old_a0_head": old_head,
        "q0_head": exact_env("Q0_HEAD"),
        "changed_paths": [current_rel, registry_rel, integration_rel, validator_rel],
        "current_plan_sha256": current_sha,
        "document_registry_sha256": sha256_file(registry_path),
        "document_registry_self_sha256": registry_self_sha,
        "integration_candidate_sha256": integration_sha,
        "current_truth_validator_sha256": sha256_file(validator_path),
        "spec_version": spec_version,
        "spec_sha256": spec_sha,
        "all_authority_false": True,
        "candidate_evidence_workflow_source_writeback": False,
    }
    (report_dir / "source-patch-report.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return report


def run_source_gates(root: Path) -> None:
    scripts = [
        "scripts/hepta-intelligence-current-truth.py",
        "scripts/verify-hepta-intelligence-a0-authority.py",
        "scripts/verify-hepta-intelligence-document-authority.py",
        "scripts/verify-hepta-intelligence-master-plan.py",
    ]
    run("python3", "-m", "py_compile", *scripts, cwd=root)
    run(
        "uv",
        "run",
        "--project",
        "scripts",
        "--locked",
        "ruff",
        "format",
        "--check",
        *scripts,
        cwd=root,
    )
    run(
        "uv",
        "run",
        "--project",
        "scripts",
        "--locked",
        "ruff",
        "check",
        *scripts,
        cwd=root,
    )
    run("python3", "scripts/verify-hepta-intelligence-master-plan.py", cwd=root)
    run("python3", "scripts/verify-hepta-intelligence-document-authority.py", cwd=root)
    run("python3", "scripts/hepta-intelligence-current-truth.py", "--verify", cwd=root)
    for tranche in ("P0.2", "P0.3", "P0.4a", "P0.4b", "P0.4c"):
        run(
            "python3",
            "scripts/hepta-intelligence-status-compat.py",
            tranche,
            "--check-only",
            cwd=root,
        )


def run_negative_fixtures(root: Path) -> None:
    validator_path = root / "scripts/hepta-intelligence-current-truth.py"
    module_spec = importlib.util.spec_from_file_location(
        "hepta_current_truth_operational_drift", validator_path
    )
    if module_spec is None or module_spec.loader is None:
        fail("cannot load patched current-truth validator")
    module = importlib.util.module_from_spec(module_spec)
    module_spec.loader.exec_module(module)
    integration = module.load(module.PATHS["integration"])
    spec_sha = module.sha(module.PATHS["spec"])

    for field, value in (
        ("version", "1.2.0"),
        ("content_sha256", "0" * 64),
        ("production_authority", True),
    ):
        bad = copy.deepcopy(integration)
        bad["operational_documents"][0][field] = value
        try:
            module.validate_operational_documents(bad, spec_sha)
        except SystemExit as error:
            if "integration operational document drift" not in str(error):
                raise
        else:
            fail(f"negative operational-document fixture accepted: {field}")

    bad = copy.deepcopy(integration)
    bad["operational_documents"][0]["unknown_positive_authority"] = True
    try:
        module.validate_operational_documents(bad, spec_sha)
    except SystemExit:
        pass
    else:
        fail("unknown operational-document authority field was accepted")
    print("PASS_HEPTA_A0_OPERATIONAL_DOCUMENT_NEGATIVE_FIXTURES")


def build_and_publish(root: Path, report_dir: Path) -> tuple[str, str]:
    old_head = exact_env("OLD_A0_HEAD")
    old_tree = exact_env("OLD_A0_TREE")
    q0_head = exact_env("Q0_HEAD")
    q0_tree = exact_env("Q0_TREE")
    staging_branch = exact_env("STAGING_BRANCH")

    if run("git", "rev-parse", "HEAD", cwd=root, capture=True) != old_head:
        fail("target checkout head drift")
    if run("git", "rev-parse", "HEAD^{tree}", cwd=root, capture=True) != old_tree:
        fail("target checkout tree drift")
    if run("git", "rev-parse", "HEAD^", cwd=root, capture=True) != q0_head:
        fail("target checkout parent drift")
    if run("git", "rev-parse", "HEAD^^{tree}", cwd=root, capture=True) != q0_tree:
        fail("Q0 tree drift")

    patch_source(root, report_dir)
    run_source_gates(root)
    run_negative_fixtures(root)
    run("git", "diff", "--check", old_head, cwd=root)

    expected_patch = sorted(
        [
            "plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json",
            "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json",
            "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json",
            "scripts/hepta-intelligence-current-truth.py",
        ]
    )
    actual_patch = sorted(
        run("git", "diff", "--name-only", old_head, cwd=root, capture=True).splitlines()
    )
    if actual_patch != expected_patch:
        fail(f"unexpected patch surface: {actual_patch}")

    run("git", "config", "user.name", exact_env("GITHUB_ACTOR"), cwd=root)
    actor_id = exact_env("GITHUB_ACTOR_ID")
    actor = exact_env("GITHUB_ACTOR")
    run(
        "git",
        "config",
        "user.email",
        f"{actor_id}+{actor}@users.noreply.github.com",
        cwd=root,
    )
    run("git", "add", *expected_patch, cwd=root)
    tree = run("git", "write-tree", cwd=root, capture=True)
    commit = run(
        "git",
        "commit-tree",
        tree,
        "-p",
        q0_head,
        cwd=root,
        capture=True,
    )
    if not commit:
        fail("commit-tree returned no commit")

    if run("git", "rev-parse", f"{commit}^", cwd=root, capture=True) != q0_head:
        fail("replacement commit is not a sole child of Q0")
    if run("git", "rev-parse", f"{commit}^{{tree}}", cwd=root, capture=True) != tree:
        fail("replacement tree drift")
    if run("git", "rev-list", "--count", f"{q0_head}..{commit}", cwd=root, capture=True) != "1":
        fail("replacement topology is not exactly one commit")

    integration = load(
        root
        / "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
    )
    expected_replacement = sorted(integration["allowed_changed_paths"])
    actual_replacement = sorted(
        run(
            "git",
            "diff",
            "--name-only",
            q0_head,
            commit,
            cwd=root,
            capture=True,
        ).splitlines()
    )
    if actual_replacement != expected_replacement:
        fail(f"replacement path allowlist drift: {actual_replacement}")

    (report_dir / "replacement-changed-paths.txt").write_text(
        "\n".join(actual_replacement) + "\n", encoding="utf-8"
    )
    receipt = {
        "schema": "hepta_a0_v451_operational_drift_publisher_receipt_v1",
        "repository": exact_env("REPOSITORY"),
        "publisher_workflow": (
            ".github/workflows/hepta-a0-v451-operational-drift-publisher.yml"
        ),
        "publisher_run_id": int(exact_env("GITHUB_RUN_ID")),
        "old_a0_head": old_head,
        "candidate_commit": commit,
        "candidate_tree": tree,
        "sole_parent": q0_head,
        "staging_branch": staging_branch,
        "changed_paths": actual_replacement,
        "changed_paths_sha256": hashlib.sha256(
            ("\n".join(actual_replacement) + "\n").encode()
        ).hexdigest(),
        "source_validators_passed": True,
        "negative_fixtures_passed": True,
        "candidate_evidence_workflow_source_writeback": False,
        "a0_candidate_qualified": False,
        "selected": False,
        "runtime_wired": False,
        "production_authority": False,
    }
    (report_dir / "replacement-receipt.json").write_text(
        json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    (report_dir / "CANDIDATE-COMMIT").write_text(commit + "\n", encoding="utf-8")
    (report_dir / "CANDIDATE-TREE").write_text(tree + "\n", encoding="utf-8")

    run(
        "git",
        "push",
        "--force",
        "origin",
        f"{commit}:refs/heads/{staging_branch}",
        cwd=root,
    )

    files = sorted(path for path in report_dir.iterdir() if path.name != "SHA256SUMS")
    sums = "".join(
        f"{sha256_file(path)}  {path.name}\n" for path in files if path.is_file()
    )
    (report_dir / "SHA256SUMS").write_text(sums, encoding="utf-8")
    return commit, tree


def main() -> int:
    if len(sys.argv) != 2:
        fail("usage: run-hepta-a0-v451-operational-drift-publisher.py TARGET_ROOT")
    root = Path(sys.argv[1]).resolve()
    if not (root / ".git").exists():
        fail(f"target checkout missing: {root}")
    report_dir = root / exact_env("REPORT_DIR")
    report_dir.mkdir(parents=True, exist_ok=True)
    commit, tree = build_and_publish(root, report_dir)
    print(
        "PASS_HEPTA_A0_V451_OPERATIONAL_DRIFT_STAGING "
        f"commit={commit} tree={tree} parent={exact_env('Q0_HEAD')}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
