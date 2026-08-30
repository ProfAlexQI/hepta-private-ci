#!/usr/bin/env python3
"""Exact-head source and closed-authority gate for HEPTA-INFERENCE-RUNTIME-V4.

The gate proves tracked repository source only. It binds the clean Git checkout,
current truth documents and (when present) the GitHub pull-request event. It
never promotes real-provider, native-model, hardware, product, operator,
promotion or release claims.
"""

from __future__ import annotations

import hashlib
import json
import os
import pathlib
import subprocess
import tomllib
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
PLAN_ID = "HEPTA-INFERENCE-RUNTIME-V4"
REPOSITORY = "ProfHepta/hepta-private-ci"
CURRENT_POINTER = "docs/hepta-vnext/inference/HEPTA_INFERENCE_CURRENT_PLAN_V1.json"
CURRENT_TRUTH = {
    "status": "docs/hepta-vnext/inference/HEPTA_INFERENCE_CURRENT_STATUS_V4.json",
    "implementation": (
        "docs/hepta-vnext/inference/HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V3.json"
    ),
    "stage_matrix": "docs/hepta-vnext/inference/HEPTA_INFERENCE_STAGE_MATRIX_V5.json",
    "blocker_ledger": (
        "docs/hepta-vnext/inference/HEPTA_INFERENCE_BLOCKER_LEDGER_V2.json"
    ),
    "evidence_contract": (
        "docs/hepta-vnext/inference/"
        "HEPTA_INFERENCE_V4_CLOSURE_EVIDENCE_CONTRACT_V1.json"
    ),
}
WORKFLOWS = (
    ".github/workflows/hepta-inference-gap-closure.yml",
    ".github/workflows/hepta-inference-v2-remaining-source.yml",
    ".github/workflows/hepta-inference-inf0.yml",
    ".github/workflows/hepta-inference-inf0c-evidence-v2.yml",
    ".github/workflows/hepta-inference-inf0c-protocol-evidence.yml",
)
SCRIPTS = (
    "scripts/hepta-inference-v4-source-truth.py",
    "scripts/hepta-inference-inf0c-historical-receipt-gate.py",
)
INFERENCE_PACKAGES = {
    "hepta-infer-core": "codex-hepta-infer-core",
    "hepta-infer-client": "codex-hepta-infer-client",
    "hepta-inferd": "codex-hepta-inferd",
}
MANAGED_ROOTS = (
    "codex-rs/hepta-infer-core",
    "codex-rs/hepta-infer-client",
    "codex-rs/hepta-inferd",
    "docs/hepta-vnext/inference",
    *WORKFLOWS,
    *SCRIPTS,
    "codex-rs/Cargo.toml",
    "codex-rs/Cargo.lock",
    "codex-rs/rust-toolchain.toml",
)
FORBIDDEN_TRACKED_PATHS = (
    ".github/qualification/hepta-inference-v4-private-contracts.patch.gz",
    ".github/workflows/hepta-inference-v4-one-shot-private-contracts.yml",
    ".github/workflows/hepta-inference-v4-one-shot-rustfmt.yml",
)
AUTHORITY_FALSE_FIELDS = (
    "production_listener",
    "production_writer",
    "provider_effect",
    "external_effect",
    "shared_kg_write",
    "memory_write",
    "route_write",
    "fleet_write",
    "model_npu",
    "remote_inference",
    "automatic_model_install",
    "operator_acceptance",
    "promotion",
    "release",
)
HIGHER_CLAIM_FIELDS = (
    "real_provider_executed",
    "real_native_model_executed",
    "hardware_qualified",
    "product_wired",
    "runtime_activated",
    "production_qualified",
    "operator_accepted",
    "promoted",
    "released",
)


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_INFERENCE_V4_SOURCE_TRUTH: {message}")


def git_bytes(*arguments: str) -> bytes:
    return subprocess.check_output(["git", *arguments], cwd=ROOT)


def git(*arguments: str) -> str:
    return git_bytes(*arguments).decode("utf-8").strip()


def sha256_bytes(value: bytes) -> str:
    return "sha256:" + hashlib.sha256(value).hexdigest()


def sha256_file(relative: str) -> str:
    return sha256_bytes((ROOT / relative).read_bytes())


def load_json(relative: str) -> dict[str, Any]:
    try:
        value = json.loads((ROOT / relative).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse {relative}: {error}")
    if not isinstance(value, dict):
        fail(f"{relative} must contain a JSON object")
    return value


def load_toml(relative: str) -> dict[str, Any]:
    try:
        value = tomllib.loads((ROOT / relative).read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        fail(f"cannot parse {relative}: {error}")
    if not isinstance(value, dict):
        fail(f"{relative} must contain a TOML table")
    return value


def require_clean_exact_checkout() -> dict[str, str]:
    if git("status", "--porcelain"):
        fail("checkout is not clean")
    identity = {
        "head": git("rev-parse", "HEAD"),
        "tree": git("rev-parse", "HEAD^{tree}"),
        "parent": git("rev-parse", "HEAD^"),
    }
    if any(len(value) != 40 for value in identity.values()):
        fail("invalid exact Git identity")
    return identity


def require_current_truth(
    identity: dict[str, str],
) -> tuple[dict[str, Any], dict[str, str]]:
    pointer = load_json(CURRENT_POINTER)
    plan_version = pointer.get("active_plan_version")
    plan_path = pointer.get("active_plan_path")
    development_pr = pointer.get("development_pr")
    development_branch = pointer.get("development_branch")
    expected_pointer = {
        "active_plan_id": PLAN_ID,
        "repository": REPOSITORY,
        "canonical_default_branch": "integration/vnext-main-20260811",
    }
    for field, expected in expected_pointer.items():
        if pointer.get(field) != expected:
            fail(f"{CURRENT_POINTER} {field} drift")
    if not isinstance(plan_version, str) or not plan_version:
        fail(f"{CURRENT_POINTER} active_plan_version invalid")
    if not isinstance(plan_path, str) or not plan_path.endswith(".md"):
        fail(f"{CURRENT_POINTER} active_plan_path invalid")
    if not isinstance(development_pr, int) or development_pr <= 0:
        fail(f"{CURRENT_POINTER} development_pr invalid")
    if not isinstance(development_branch, str) or not development_branch:
        fail(f"{CURRENT_POINTER} development_branch invalid")
    if not (ROOT / plan_path).is_file():
        fail(f"active plan is missing: {plan_path}")
    require_closed_authority(pointer, CURRENT_POINTER)

    pointer_binding = pointer.get("source_binding")
    if not isinstance(pointer_binding, dict):
        fail(f"{CURRENT_POINTER} source_binding missing")
    if pointer_binding.get("observed_parent_commit") != identity["parent"]:
        fail(f"{CURRENT_POINTER} observed parent commit does not bind HEAD^")

    links = pointer.get("current_truth")
    if not isinstance(links, dict):
        fail(f"{CURRENT_POINTER} current_truth must be an object")
    for key, relative in CURRENT_TRUTH.items():
        if links.get(key) != pathlib.Path(relative).name:
            fail(f"{CURRENT_POINTER} current_truth.{key} drift")

    digests = {CURRENT_POINTER: sha256_file(CURRENT_POINTER)}
    expected_common = {
        "plan_id": PLAN_ID,
        "plan_version": plan_version,
        "repository": REPOSITORY,
        "development_pr": development_pr,
    }
    for relative in CURRENT_TRUTH.values():
        document = load_json(relative)
        for field, expected in expected_common.items():
            if document.get(field) != expected:
                fail(f"{relative} {field} drift")
        branch = document.get("development_branch")
        if branch is not None and branch != development_branch:
            fail(f"{relative} development_branch drift")
        binding = document.get("source_binding")
        if not isinstance(binding, dict):
            fail(f"{relative} source_binding missing")
        if binding.get("observed_parent_commit") != identity["parent"]:
            fail(f"{relative} observed parent commit does not bind HEAD^")
        require_closed_authority(document, relative)
        digests[relative] = sha256_file(relative)

    status = load_json(CURRENT_TRUTH["status"])
    claims = status.get("claims")
    if not isinstance(claims, dict):
        fail("current status has no claims object")
    if claims.get("source_candidate_present") is not True:
        fail("current status must acknowledge tracked source candidate")
    if claims.get("source_candidate_qualified") is not False:
        fail("tracked source may not self-qualify")
    opened = [field for field in HIGHER_CLAIM_FIELDS if claims.get(field) is not False]
    if opened:
        fail(f"tracked status opens unexecuted claims: {opened}")

    implementation = load_json(CURRENT_TRUTH["implementation"])
    if implementation.get("status") != "SOURCE_PRESENT_NOT_QUALIFIED":
        fail("implementation status exceeds source-present posture")

    stage_matrix = load_json(CURRENT_TRUTH["stage_matrix"])
    global_claims = stage_matrix.get("global_claims")
    if not isinstance(global_claims, dict):
        fail("stage matrix has no global_claims")
    if any(value is not False for value in global_claims.values()):
        fail("stage matrix opens a global claim")

    ledger = load_json(CURRENT_TRUTH["blocker_ledger"])
    summary = ledger.get("summary")
    if not isinstance(summary, dict) or summary.get("qualified") is not False:
        fail("blocker ledger must remain unqualified while blockers are open")

    contract = load_json(CURRENT_TRUTH["evidence_contract"])
    source_gate = contract.get("current_source_gate")
    if not isinstance(source_gate, dict):
        fail("evidence contract has no current_source_gate")
    expected_gate = {
        "toolchain": "1.95.0",
        "github_event_binding_required": True,
        "managed_inventory_digest_required": True,
        "non_empty_steps_required": True,
        "runner_id_nonzero_required": True,
        "skipped_is_pass": False,
        "ci_source_mutation_allowed": False,
    }
    for field, expected in expected_gate.items():
        if source_gate.get(field) != expected:
            fail(f"evidence contract current_source_gate.{field} drift")

    plan = (ROOT / plan_path).read_text(encoding="utf-8")
    for marker in (
        PLAN_ID,
        plan_version,
        REPOSITORY,
        f"Development PR: `#{development_pr}`",
        f"Development branch: `{development_branch}`",
        "qualification_only: true",
        "production_listener: false",
        "real_provider_executed=false",
        "real_native_model_executed=false",
    ):
        if marker not in plan:
            fail(f"active plan missing marker {marker!r}")
    digests[plan_path] = sha256_file(plan_path)
    return pointer, digests


def require_closed_authority(document: dict[str, Any], relative: str) -> None:
    authority = document.get("authority")
    if not isinstance(authority, dict):
        fail(f"{relative} has no authority object")
    if authority.get("qualification_only") is not True:
        fail(f"{relative} qualification_only must be true")
    opened = [
        field for field in AUTHORITY_FALSE_FIELDS if authority.get(field) is not False
    ]
    if opened:
        fail(f"{relative} authority is not closed: {opened}")


def require_event_binding(
    identity: dict[str, str], pointer: dict[str, Any]
) -> dict[str, Any]:
    result: dict[str, Any] = {
        "binding_source": "tracked_pointer_and_git",
        "pull_request_number": pointer["development_pr"],
        "development_branch": pointer["development_branch"],
    }
    event_path = os.environ.get("GITHUB_EVENT_PATH")
    if not event_path:
        return result
    try:
        event = json.loads(pathlib.Path(event_path).read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse GITHUB_EVENT_PATH: {error}")
    pull_request = event.get("pull_request")
    if pull_request is None:
        result["github_event"] = os.environ.get("GITHUB_EVENT_NAME", "unknown")
        return result
    if not isinstance(pull_request, dict):
        fail("pull_request event shape invalid")
    head = pull_request.get("head")
    if not isinstance(head, dict):
        fail("pull_request.head missing")
    number = event.get("number")
    branch = head.get("ref")
    head_sha = head.get("sha")
    if number != pointer["development_pr"]:
        fail("GitHub event PR does not match current pointer")
    if branch != pointer["development_branch"]:
        fail("GitHub event head branch does not match current pointer")
    if head_sha != identity["head"]:
        fail("GitHub event head SHA does not match checkout")
    result.update(
        {
            "binding_source": "github_pull_request_event",
            "pull_request_number": number,
            "development_branch": branch,
            "event_head": head_sha,
        }
    )
    return result


def require_managed_inventory() -> dict[str, Any]:
    for path in FORBIDDEN_TRACKED_PATHS:
        try:
            git("ls-files", "--error-unmatch", path)
        except subprocess.CalledProcessError:
            continue
        fail(f"forbidden mutation payload/helper remains tracked: {path}")

    raw = git_bytes("ls-files", "-z", "--", *MANAGED_ROOTS)
    paths = sorted(item.decode("utf-8") for item in raw.split(b"\0") if item)
    if not paths:
        fail("managed source inventory is empty")
    required = {
        CURRENT_POINTER,
        *CURRENT_TRUTH.values(),
        *WORKFLOWS,
        *SCRIPTS,
        "codex-rs/Cargo.toml",
        "codex-rs/Cargo.lock",
        "codex-rs/rust-toolchain.toml",
        "codex-rs/hepta-infer-core/Cargo.toml",
        "codex-rs/hepta-infer-core/src/adapter.rs",
        "codex-rs/hepta-infer-core/src/capability.rs",
        "codex-rs/hepta-infer-core/src/controller.rs",
        "codex-rs/hepta-infer-core/src/hashing.rs",
        "codex-rs/hepta-infer-core/src/lib.rs",
        "codex-rs/hepta-infer-core/src/model.rs",
        "codex-rs/hepta-infer-core/src/protocol.rs",
        "codex-rs/hepta-infer-core/src/security.rs",
        "codex-rs/hepta-infer-core/src/tests.rs",
        "codex-rs/hepta-infer-core/src/worker.rs",
        "codex-rs/hepta-infer-client/Cargo.toml",
        "codex-rs/hepta-infer-client/src/lib.rs",
        "codex-rs/hepta-infer-client/src/tests.rs",
        "codex-rs/hepta-inferd/Cargo.toml",
        "codex-rs/hepta-inferd/src/lib.rs",
        "codex-rs/hepta-inferd/src/main.rs",
        "codex-rs/hepta-inferd/src/tests.rs",
    }
    missing = sorted(required.difference(paths))
    if missing:
        fail(f"managed source inventory missing required paths: {missing}")

    accumulator = hashlib.sha256()
    file_digests: dict[str, str] = {}
    for relative in paths:
        absolute = ROOT / relative
        if not absolute.is_file():
            fail(f"managed tracked path is not a file: {relative}")
        digest = sha256_file(relative)
        file_digests[relative] = digest
        accumulator.update(relative.encode("utf-8"))
        accumulator.update(b"\0")
        accumulator.update(digest.encode("ascii"))
        accumulator.update(b"\n")
    return {
        "file_count": len(paths),
        "digest": "sha256:" + accumulator.hexdigest(),
        "files": file_digests,
    }


def require_no_inference_workflow_mutation() -> dict[str, str]:
    paths = git("ls-files", ".github/workflows/hepta-inference*.yml").splitlines()
    if not paths:
        fail("no inference workflows found")
    digests: dict[str, str] = {}
    for relative in paths:
        text = (ROOT / relative).read_text(encoding="utf-8")
        if "contents: write" in text:
            fail(f"inference workflow has contents write authority: {relative}")
        if "git push" in text:
            fail(f"inference workflow mutates a branch: {relative}")
        digests[relative] = sha256_file(relative)
    return digests


def require_workspace() -> dict[str, Any]:
    workspace = load_toml("codex-rs/Cargo.toml").get("workspace")
    if not isinstance(workspace, dict):
        fail("codex-rs/Cargo.toml has no workspace table")
    members = workspace.get("members")
    if not isinstance(members, list) or not all(
        isinstance(item, str) for item in members
    ):
        fail("workspace.members must be a string array")
    package_names: dict[str, str] = {}
    for member, expected_name in INFERENCE_PACKAGES.items():
        if member not in members:
            fail(f"workspace missing inference member {member}")
        manifest_path = f"codex-rs/{member}/Cargo.toml"
        package = load_toml(manifest_path).get("package")
        if not isinstance(package, dict) or package.get("name") != expected_name:
            fail(f"{manifest_path} package.name drift")
        package_names[member] = expected_name
    toolchain = load_toml("codex-rs/rust-toolchain.toml").get("toolchain")
    if not isinstance(toolchain, dict) or toolchain.get("channel") != "1.95.0":
        fail("codex-rs/rust-toolchain.toml must remain pinned to 1.95.0")
    return {
        "workspace_member_count": len(members),
        "inference_packages": package_names,
        "cargo_lock_sha256": sha256_file("codex-rs/Cargo.lock"),
        "rust_toolchain_sha256": sha256_file("codex-rs/rust-toolchain.toml"),
    }


def require_historical_gate_isolation() -> dict[str, str]:
    stale = (
        "python3 scripts/hepta-inference-inf0c-evidence-v2-source-gate.py",
        "python3 scripts/hepta-inference-inf0-source-gate.py",
    )
    digests: dict[str, str] = {}
    for relative in WORKFLOWS[2:]:
        text = (ROOT / relative).read_text(encoding="utf-8")
        if "hepta-inference-inf0c-historical-receipt-gate.py" not in text:
            fail(f"{relative} does not use immutable historical receipt gate")
        present = [marker for marker in stale if marker in text]
        if present:
            fail(f"{relative} replays mutable historical gates: {present}")
        digests[relative] = sha256_file(relative)
    return digests


def main() -> None:
    identity = require_clean_exact_checkout()
    pointer, truth_digests = require_current_truth(identity)
    event_binding = require_event_binding(identity, pointer)
    inventory = require_managed_inventory()
    workflow_digests = require_no_inference_workflow_mutation()
    historical_digests = require_historical_gate_isolation()
    workspace = require_workspace()

    plan_path = pointer["active_plan_path"]
    receipt = {
        "schema": "hepta.inference.v4.exact_source_truth.v2",
        "plan_id": PLAN_ID,
        "plan_version": pointer["active_plan_version"],
        "repository": REPOSITORY,
        **event_binding,
        **identity,
        "source_truth": "PASS",
        "plan_digest": sha256_file(plan_path),
        "current_pointer_digest": sha256_file(CURRENT_POINTER),
        "truth_digests": truth_digests,
        "managed_inventory": inventory,
        "workflow_digests": workflow_digests,
        "historical_workflow_digests": historical_digests,
        "workspace": workspace,
        "current_plan_pointer_unique": True,
        "github_event_bound_when_present": True,
        "historical_current_gate_isolated": True,
        "ci_source_mutation_allowed": False,
        "qualification_only": True,
        "real_provider_executed": False,
        "real_native_model_executed": False,
        "hardware_qualified": False,
        "product_wired": False,
        "product_default_route_changed": False,
        "operator_accepted": False,
        "promoted": False,
        "released": False,
    }
    print(json.dumps(receipt, indent=2, sort_keys=True))


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError as error:
        fail(f"command failed with status {error.returncode}: {error.cmd}")
