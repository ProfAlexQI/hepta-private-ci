#!/usr/bin/env python3
"""Publish the canonical Hepta inference V4 documentation set transactionally."""

from __future__ import annotations

import hashlib
import json
import os
import pathlib
import subprocess
import sys
import tarfile

REPOSITORY = "ProfHepta/hepta-private-ci"
TARGET_BRANCH = "codex/hepta-inference-v4-gap-closure-20260830"
EXPECTED_TARGET = "2d6fa9c01af0c98a81cd804b1f35ba6a171f0c10"
ARCHIVE = pathlib.Path("tools/hepta-inference-v4-docs-publisher/docs-v4.tar.gz")
ARCHIVE_SHA256 = "14f8274f4dcb31de84b2eaa21a2dc397cadff0581477bd3f688679d17bed9a11"
DOC_ROOT = pathlib.PurePosixPath("docs/hepta-vnext/inference")
FILES = (
    "HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V4.md",
    "HEPTA_INFERENCE_CANONICAL_TRUTH_V1.json",
    "HEPTA_INFERENCE_CURRENT_STATUS_V4.json",
    "HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V3.json",
    "HEPTA_INFERENCE_STAGE_MATRIX_V5.json",
    "HEPTA_INFERENCE_BLOCKER_LEDGER_V2.json",
    "HEPTA_INFERENCE_V4_CLOSURE_EVIDENCE_CONTRACT_V1.json",
    "HEPTA_INFERENCE_RUNTIME_RUNBOOK_V1.md",
    "HEPTA_INFERENCE_THREAT_MODEL_V1.md",
    "HEPTA_INFERENCE_TEST_MATRIX_V1.md",
)
EXPECTED_PATHS = {str(DOC_ROOT / name) for name in FILES}


class PublishError(RuntimeError):
    pass


def run(*argv: str, capture: bool = False) -> subprocess.CompletedProcess[str]:
    print("+", " ".join(argv), flush=True)
    result = subprocess.run(argv, check=False, text=True, capture_output=capture)
    if result.returncode != 0:
        if capture:
            print(result.stdout, file=sys.stderr)
            print(result.stderr, file=sys.stderr)
        raise PublishError(f"command failed ({result.returncode}): {' '.join(argv)}")
    return result


def git(*argv: str, capture: bool = True) -> str:
    result = run("git", *argv, capture=capture)
    return result.stdout.strip() if capture else ""


def sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def validate_archive() -> list[tarfile.TarInfo]:
    if not ARCHIVE.is_file():
        raise PublishError(f"missing archive: {ARCHIVE}")
    actual = sha256(ARCHIVE)
    if actual != ARCHIVE_SHA256:
        raise PublishError(f"archive digest mismatch: {actual}")
    with tarfile.open(ARCHIVE, "r:gz") as handle:
        members = handle.getmembers()
    paths: set[str] = set()
    for member in members:
        path = pathlib.PurePosixPath(member.name)
        if path.is_absolute() or ".." in path.parts:
            raise PublishError(f"unsafe archive path: {member.name}")
        if not member.isfile():
            raise PublishError(f"non-regular archive member: {member.name}")
        rendered = path.as_posix()
        if rendered not in EXPECTED_PATHS:
            raise PublishError(f"out-of-scope archive member: {rendered}")
        if rendered in paths:
            raise PublishError(f"duplicate archive member: {rendered}")
        paths.add(rendered)
    if paths != EXPECTED_PATHS:
        raise PublishError(f"archive inventory mismatch: {sorted(EXPECTED_PATHS - paths)}")
    return members


def extract_archive(members: list[tarfile.TarInfo]) -> None:
    with tarfile.open(ARCHIVE, "r:gz") as handle:
        for member in members:
            path = pathlib.Path(member.name)
            if path.exists():
                raise PublishError(f"refusing to overwrite existing path: {path}")
            source = handle.extractfile(member)
            if source is None:
                raise PublishError(f"unreadable archive member: {member.name}")
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(source.read())
            path.chmod(0o644)


def load_json(path: pathlib.Path) -> dict[str, object]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise PublishError(f"JSON root must be an object: {path}")
    return value


def validate_documents() -> None:
    root = pathlib.Path(DOC_ROOT.as_posix())
    values = {
        name: load_json(root / name)
        for name in FILES
        if name.endswith(".json")
    }
    for name, value in values.items():
        if value.get("plan_id") != "HEPTA-INFERENCE-RUNTIME-V4":
            raise PublishError(f"plan id mismatch: {name}")
    canonical = values["HEPTA_INFERENCE_CANONICAL_TRUTH_V1.json"]
    active = canonical.get("active")
    if not isinstance(active, dict):
        raise PublishError("canonical active map missing")
    for path in active.values():
        if not isinstance(path, str) or not pathlib.Path(path).is_file():
            raise PublishError(f"canonical active path missing: {path}")
    status = values["HEPTA_INFERENCE_CURRENT_STATUS_V4.json"]
    if status.get("module_closed_candidate") is not False:
        raise PublishError("status must not claim module closure")
    authority = canonical.get("authority")
    if not isinstance(authority, dict) or authority.get("qualification_only") is not True:
        raise PublishError("canonical authority must remain qualification-only")
    for field in (
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
    ):
        if authority.get(field) is not False:
            raise PublishError(f"canonical authority.{field} must be false")


def main() -> int:
    if os.environ.get("GITHUB_REPOSITORY") != REPOSITORY:
        raise PublishError("repository identity mismatch")
    members = validate_archive()
    run("git", "fetch", "--no-tags", "origin", TARGET_BRANCH, capture=False)
    if git("rev-parse", "FETCH_HEAD") != EXPECTED_TARGET:
        raise PublishError("target branch head drifted")
    run("git", "checkout", "--detach", EXPECTED_TARGET, capture=False)
    if git("status", "--porcelain"):
        raise PublishError("target checkout is dirty")
    extract_archive(members)
    validate_documents()
    run("git", "add", *sorted(EXPECTED_PATHS), capture=False)
    changed = set(git("diff", "--cached", "--name-only").splitlines())
    if changed != EXPECTED_PATHS:
        raise PublishError(f"changed-path mismatch: {sorted(changed)}")
    run("git", "diff", "--cached", "--check", capture=False)
    run("git", "config", "user.name", "github-actions[bot]", capture=False)
    run(
        "git",
        "config",
        "user.email",
        "41898282+github-actions[bot]@users.noreply.github.com",
        capture=False,
    )
    run("git", "commit", "-m", "docs(inference): publish canonical runtime plan v4", capture=False)
    head = git("rev-parse", "HEAD")
    tree = git("rev-parse", "HEAD^{tree}")
    run("git", "fetch", "--no-tags", "origin", TARGET_BRANCH, capture=False)
    if git("rev-parse", "FETCH_HEAD") != EXPECTED_TARGET:
        raise PublishError("target branch drifted before push")
    run("git", "push", "origin", f"HEAD:refs/heads/{TARGET_BRANCH}", capture=False)
    receipt = {
        "schema": "hepta.inference.v4.docs_publisher_receipt.v1",
        "repository": REPOSITORY,
        "target_branch": TARGET_BRANCH,
        "parent_sha": EXPECTED_TARGET,
        "head_sha": head,
        "tree_sha": tree,
        "archive_sha256": f"sha256:{ARCHIVE_SHA256}",
        "files": sorted(EXPECTED_PATHS),
        "json_validated": True,
        "canonical_pointer_validated": True,
        "closed_authority_validated": True,
        "qualification_only": True,
        "operator_accepted": False,
        "promoted": False,
        "released": False,
    }
    pathlib.Path(os.environ["RUNNER_TEMP"], "hepta-inference-v4-docs-receipt.json").write_text(
        json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(json.dumps(receipt, sort_keys=True))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (PublishError, OSError, json.JSONDecodeError, tarfile.TarError) as error:
        print(f"FAIL_HEPTA_INFERENCE_V4_DOCS_PUBLISH: {error}", file=sys.stderr)
        raise SystemExit(1) from error
