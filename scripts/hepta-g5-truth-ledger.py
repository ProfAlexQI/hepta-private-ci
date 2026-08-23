#!/usr/bin/env python3
"""Emit and verify an append-only G5 qualification truth/lineage receipt.

This utility is evidence-only: it binds a detached candidate to its exact
Git identity and ancestry, rechecks selected immutable ``SHA256SUMS`` files,
and keeps qualification, authority, operator, promotion, and integration
status separate.  It never changes CALLERS, grants authority, accepts a
signature, or claims provider physical exactly-once delivery.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Sequence


SCHEMA = "hepta_g5_truth_lineage_receipt_v1"
SCHEMA_VERSION = 1
DEFAULT_CANONICAL_HEAD = "7ed9c9a85fa65aa3cb26cf440a55028ce0b35079"
DEFAULT_CANONICAL_TREE = "7d4306273861564a62fa9614860bdc6239a065d0"
DEFAULT_G5_ANCHOR = "73ff3b438a25d88201169aed7c7c79cf5d9644a8"
DEFAULT_G5_ANCHOR_TREE = "4070f421a63311c66a77d08491c4a9ab1fd52c65"
GIT_ID_RE = re.compile(r"^[0-9a-f]{40}$")
SHA256_RE = re.compile(r"^[0-9a-f]{64}$")
SUM_LINE_RE = re.compile(r"^([0-9a-f]{64})[ \t]+(.+)$")


class LedgerError(RuntimeError):
    """A fail-closed input or verification error."""


def fail(message: str) -> None:
    raise LedgerError(message)


def absolute_path(value: str, *, label: str) -> Path:
    path = Path(value)
    if not path.is_absolute() or str(path) == "/":
        fail(f"{label} must be an absolute non-root path: {value}")
    return path.resolve(strict=False)


def regular_file(path: Path, *, label: str) -> Path:
    if not path.exists() or path.is_symlink() or not path.is_file():
        fail(f"{label} is missing, a symlink, or not a regular file: {path}")
    return path


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def canonical_json(value: Any) -> bytes:
    return (
        json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n"
    ).encode("utf-8")


def read_canonical_json(path: Path, *, label: str) -> tuple[dict[str, Any], bytes]:
    raw = regular_file(path, label=label).read_bytes()
    try:
        value = json.loads(raw.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as exc:
        fail(f"{label} is not valid UTF-8 JSON: {exc}")
    if not isinstance(value, dict):
        fail(f"{label} must contain a JSON object")
    if canonical_json(value) != raw:
        fail(f"{label} is not canonical JSON")
    return value, raw


def run_git(worktree: Path, *args: str, check: bool = True) -> str:
    result = subprocess.run(
        ["git", "-C", str(worktree), *args],
        check=False,
        capture_output=True,
        text=True,
    )
    if check and result.returncode != 0:
        detail = result.stderr.strip() or result.stdout.strip()
        fail(f"git command failed ({' '.join(args)}): {detail}")
    return result.stdout.strip()


def git_identity(worktree_arg: str) -> dict[str, Any]:
    requested = absolute_path(worktree_arg, label="worktree")
    if not requested.is_dir() or requested.is_symlink():
        fail(f"worktree is not a physical directory: {requested}")
    top = Path(run_git(requested, "rev-parse", "--show-toplevel")).resolve()
    if top != requested:
        fail(f"git worktree root differs from requested path: {top} != {requested}")
    head = run_git(requested, "rev-parse", "HEAD")
    tree = run_git(requested, "rev-parse", "HEAD^{tree}")
    dirty = run_git(requested, "status", "--porcelain=v1", "--untracked-files=all")
    if dirty:
        fail(f"candidate worktree is dirty:\n{dirty}")
    return {"worktree": str(requested), "repository": str(top), "head": head, "tree": tree, "clean": True}


def verify_ancestor(worktree: Path, ancestor: str, *, label: str) -> dict[str, Any]:
    if not GIT_ID_RE.fullmatch(ancestor):
        fail(f"{label} is not a 40-character lowercase git id: {ancestor}")
    run_git(worktree, "cat-file", "-e", f"{ancestor}^{{commit}}")
    result = subprocess.run(
        ["git", "-C", str(worktree), "merge-base", "--is-ancestor", ancestor, "HEAD"],
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        detail = result.stderr.strip() or "candidate is not a descendant"
        fail(f"{label} ancestry check failed for {ancestor}: {detail}")
    return {
        "label": label,
        "commit": ancestor,
        "tree": run_git(worktree, "rev-parse", f"{ancestor}^{{tree}}"),
        "verified": True,
    }


def verify_sha256sums(directory_arg: str) -> dict[str, Any]:
    directory = absolute_path(directory_arg, label="evidence directory")
    if not directory.is_dir() or directory.is_symlink():
        fail(f"evidence directory is not a physical directory: {directory}")
    manifest = regular_file(directory / "SHA256SUMS", label="SHA256SUMS")
    entries: list[dict[str, Any]] = []
    for line_number, line in enumerate(manifest.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip():
            continue
        match = SUM_LINE_RE.fullmatch(line)
        if match is None:
            fail(f"malformed SHA256SUMS line {manifest}:{line_number}")
        expected, relative_text = match.groups()
        # Reject escaped names instead of silently decoding a different path.
        if relative_text.startswith("\\"):
            fail(f"escaped checksum path is unsupported: {manifest}:{line_number}")
        relative = Path(relative_text)
        if relative.is_absolute() or ".." in relative.parts:
            fail(f"checksum path escapes its evidence directory: {relative_text}")
        target = (directory / relative).resolve(strict=False)
        try:
            target.relative_to(directory)
        except ValueError:
            fail(f"checksum path resolves outside its evidence directory: {relative_text}")
        regular_file(target, label=f"checksum target {relative_text}")
        observed = sha256_file(target)
        if observed != expected:
            fail(f"checksum mismatch for {target}: expected {expected}, observed {observed}")
        entries.append({"path": relative.as_posix(), "sha256": observed, "size": target.stat().st_size})
    if not entries:
        fail(f"SHA256SUMS is empty: {manifest}")
    return {
        "directory": str(directory),
        "manifest": str(manifest),
        "manifest_sha256": sha256_file(manifest),
        "entry_count": len(entries),
        "entries": entries,
    }


def verify_superseded(path_arg: str) -> dict[str, str]:
    path = absolute_path(path_arg, label="superseded receipt")
    regular_file(path, label="superseded receipt")
    return {"path": str(path), "sha256": sha256_file(path)}


def status_block(
    *,
    qualification: str,
    operator: str,
    provider: str,
    integration: str,
    caller_evidence_ratchet_present: bool,
) -> dict[str, Any]:
    if qualification not in {"bounded_candidate", "qualified_candidate"}:
        fail(f"unsupported qualification status: {qualification}")
    if operator not in {"blocked_prep", "ready_for_challenge"}:
        fail(f"unsupported operator status: {operator}")
    if provider not in {"unproven", "contract_only", "adapter_test_only"}:
        fail(f"unsupported provider status: {provider}")
    if integration not in {"detached_only", "candidate_only"}:
        fail(f"unsupported integration status: {integration}")
    return {
        "qualification_status": qualification,
        "authority_status": "not_granted",
        "operator_status": operator,
        "promotion_status": "not_eligible",
        "integration_status": integration,
        "provider_status": provider,
        "provider_physical_exactly_once": False,
        "caller_evidence_ratchet_present": caller_evidence_ratchet_present,
        "authority_flags": {
            "g5_allowed": False,
            "fleet_and_automation_unfrozen": False,
            "operator_acceptance": False,
            "promotion": False,
            "caller_authority_ratchet": False,
        },
    }


def emit(args: argparse.Namespace) -> dict[str, Any]:
    identity = git_identity(args.worktree)
    worktree = Path(identity["worktree"])
    if args.expected_head and identity["head"] != args.expected_head:
        fail(f"candidate HEAD mismatch: expected {args.expected_head}, observed {identity['head']}")
    if args.expected_tree and identity["tree"] != args.expected_tree:
        fail(f"candidate tree mismatch: expected {args.expected_tree}, observed {identity['tree']}")
    if not GIT_ID_RE.fullmatch(args.canonical_head) or not GIT_ID_RE.fullmatch(args.canonical_tree):
        fail("canonical head/tree must be 40-character lowercase hexadecimal git ids")
    observed_canonical_tree = run_git(worktree, "rev-parse", f"{args.canonical_head}^{{tree}}")
    if observed_canonical_tree != args.canonical_tree:
        fail(
            "canonical tree mismatch: "
            f"expected {args.canonical_tree}, observed {observed_canonical_tree}"
        )
    result = subprocess.run(
        ["git", "-C", str(worktree), "merge-base", "--is-ancestor", args.canonical_head, "HEAD"],
        check=False,
    )
    if result.returncode != 0:
        fail("candidate is not a descendant of canonical main-integration")
    required = [verify_ancestor(worktree, args.g5_anchor, label="g5_minimum_six_anchor")]
    if required[0]["tree"] != args.g5_anchor_tree:
        fail(
            "G5 minimum-six anchor tree mismatch: "
            f"expected {args.g5_anchor_tree}, observed {required[0]['tree']}"
        )
    for index, ancestor in enumerate(args.required_ancestor, 1):
        if ancestor == args.g5_anchor:
            continue
        required.append(verify_ancestor(worktree, ancestor, label=f"required_ancestor_{index}"))
    parent: dict[str, Any] | None = None
    if args.parent_head:
        parent = verify_ancestor(worktree, args.parent_head, label="declared_parent")
        immediate_parent = run_git(worktree, "rev-parse", "HEAD^1")
        if immediate_parent != args.parent_head:
            fail(
                "declared parent is not the candidate's first parent: "
                f"expected {args.parent_head}, observed {immediate_parent}"
            )
        if args.parent_tree and parent["tree"] != args.parent_tree:
            fail(f"declared parent tree mismatch: expected {args.parent_tree}, observed {parent['tree']}")
        parent["declared_tree"] = args.parent_tree or parent["tree"]
    evidence = [verify_sha256sums(path) for path in args.evidence_dir]
    supersedes = [verify_superseded(path) for path in args.supersedes]
    receipt: dict[str, Any] = {
        "schema": SCHEMA,
        "schema_version": SCHEMA_VERSION,
        "kind": "g5_truth_lineage",
        "generated_at": datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        **status_block(
            qualification=args.qualification_status,
            operator=args.operator_status,
            provider=args.provider_status,
            integration=args.integration_status,
            caller_evidence_ratchet_present=args.caller_evidence_ratchet_present,
        ),
        "lineage": {
            "canonical": {"head": args.canonical_head, "tree": args.canonical_tree, "is_ancestor": True},
            "candidate": {
                "head": identity["head"],
                "tree": identity["tree"],
                "worktree": identity["worktree"],
                "clean": identity["clean"],
            },
            "required_ancestors": required,
            "parent": parent,
        },
        "evidence": evidence,
        "supersedes": supersedes,
        "negative_authority": [
            "provider physical exactly-once is not established by local evidence",
            "independent operator signer/trust acceptance is not present",
            "qualification does not grant CALLERS authority or mutate canonical main-integration",
        ],
        "tool": {"name": "scripts/hepta-g5-truth-ledger.py", "mode": "evidence_only", "python": sys.version.split()[0]},
    }
    output = absolute_path(args.output, label="output receipt")
    if output.exists() or output.is_symlink():
        fail(f"append-only output already exists: {output}")
    output.parent.mkdir(parents=True, exist_ok=True)
    raw = canonical_json(receipt)
    try:
        descriptor = os.open(output, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o400)
    except FileExistsError:
        fail(f"append-only output raced with another writer: {output}")
    try:
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(raw)
            handle.flush()
            os.fsync(handle.fileno())
    except Exception:
        try:
            output.unlink()
        except OSError:
            pass
        raise
    receipt_sha256 = sha256_file(output)
    result = dict(receipt)
    result.update({"receipt_path": str(output), "receipt_sha256": receipt_sha256})
    print(json.dumps(result, sort_keys=True))
    return result


def verify_receipt(args: argparse.Namespace) -> dict[str, Any]:
    path = absolute_path(args.receipt, label="receipt")
    value, raw = read_canonical_json(path, label="truth receipt")
    if value.get("schema") != SCHEMA or value.get("schema_version") != SCHEMA_VERSION:
        fail("truth receipt schema/version mismatch")
    if value.get("authority_status") != "not_granted":
        fail("truth receipt asserts authority")
    if value.get("promotion_status") != "not_eligible":
        fail("truth receipt asserts promotion eligibility")
    if value.get("provider_physical_exactly_once") is not False:
        fail("truth receipt asserts provider physical exactly-once")
    flags = value.get("authority_flags")
    if not isinstance(flags, dict) or any(flags.values()):
        fail("truth receipt contains an asserted authority flag")
    lineage = value.get("lineage")
    if not isinstance(lineage, dict):
        fail("truth receipt lineage is missing")
    candidate = lineage.get("candidate")
    canonical = lineage.get("canonical")
    if not isinstance(candidate, dict) or not isinstance(canonical, dict):
        fail("truth receipt candidate/canonical lineage is malformed")
    worktree_value = args.worktree or candidate.get("worktree")
    if not isinstance(worktree_value, str) or not worktree_value:
        fail("truth receipt does not name a candidate worktree")
    identity = git_identity(worktree_value)
    if identity["head"] != candidate.get("head") or identity["tree"] != candidate.get("tree"):
        fail("receipt candidate identity differs from supplied worktree")
    worktree = Path(identity["worktree"])
    if args.canonical_head and canonical.get("head") != args.canonical_head:
        fail("receipt canonical head differs from requested head")
    if args.canonical_tree and canonical.get("tree") != args.canonical_tree:
        fail("receipt canonical tree differs from requested tree")
    canonical_head = canonical.get("head")
    canonical_tree = canonical.get("tree")
    if not isinstance(canonical_head, str) or not isinstance(canonical_tree, str):
        fail("truth receipt canonical identity is malformed")
    if run_git(worktree, "rev-parse", f"{canonical_head}^{{tree}}") != canonical_tree:
        fail("receipt canonical head/tree binding no longer verifies")
    canonical_result = subprocess.run(
        ["git", "-C", str(worktree), "merge-base", "--is-ancestor", canonical_head, "HEAD"],
        check=False,
    )
    if canonical_result.returncode != 0 or canonical.get("is_ancestor") is not True:
        fail("receipt canonical ancestry no longer verifies")
    required_ancestors = lineage.get("required_ancestors")
    if not isinstance(required_ancestors, list) or not required_ancestors:
        fail("truth receipt required ancestry is missing")
    for ancestor in required_ancestors:
        if not isinstance(ancestor, dict):
            fail("truth receipt required ancestor is malformed")
        checked = verify_ancestor(
            worktree,
            str(ancestor.get("commit", "")),
            label=str(ancestor.get("label", "required_ancestor")),
        )
        if checked["tree"] != ancestor.get("tree") or ancestor.get("verified") is not True:
            fail(f"required ancestor binding changed: {ancestor.get('commit')}")
    parent = lineage.get("parent")
    if parent is not None:
        if not isinstance(parent, dict):
            fail("truth receipt parent binding is malformed")
        checked_parent = verify_ancestor(
            worktree,
            str(parent.get("commit", "")),
            label=str(parent.get("label", "declared_parent")),
        )
        if checked_parent["tree"] != parent.get("tree"):
            fail("truth receipt parent tree changed")
        if run_git(worktree, "rev-parse", "HEAD^1") != parent.get("commit"):
            fail("truth receipt parent is not the candidate's first parent")
    for evidence in value.get("evidence", []):
        if not isinstance(evidence, dict) or "directory" not in evidence:
            fail("truth receipt evidence entry is malformed")
        checked = verify_sha256sums(str(evidence["directory"]))
        if checked["manifest_sha256"] != evidence.get("manifest_sha256") or checked["entry_count"] != evidence.get("entry_count"):
            fail(f"evidence manifest changed: {evidence.get('directory')}")
    for superseded in value.get("supersedes", []):
        if not isinstance(superseded, dict):
            fail("truth receipt supersedes entry is malformed")
        checked = verify_superseded(str(superseded.get("path", "")))
        if checked["sha256"] != superseded.get("sha256"):
            fail(f"superseded receipt changed: {superseded.get('path')}")
    result = {
        "verified": True,
        "schema": value["schema"],
        "receipt": str(path),
        "receipt_sha256": sha256_file(path),
        "candidate_head": candidate.get("head"),
        "candidate_tree": candidate.get("tree"),
        "authority_status": value["authority_status"],
        "provider_physical_exactly_once": value["provider_physical_exactly_once"],
    }
    print(json.dumps(result, sort_keys=True))
    return result


def self_test() -> None:
    with tempfile.TemporaryDirectory(prefix="hepta-g5-truth-") as temporary:
        root = Path(temporary)
        repository = root / "repository"
        repository.mkdir()
        subprocess.run(["git", "-C", str(repository), "init", "-q"], check=True)
        for config, value in (("user.name", "G5 self-test"), ("user.email", "g5-self-test@example.invalid")):
            subprocess.run(["git", "-C", str(repository), "config", config, value], check=True)
        (repository / "README").write_text("one\n", encoding="utf-8")
        subprocess.run(["git", "-C", str(repository), "add", "README"], check=True)
        subprocess.run(["git", "-C", str(repository), "commit", "-qm", "genesis"], check=True)
        canonical = run_git(repository, "rev-parse", "HEAD")
        canonical_tree = run_git(repository, "rev-parse", "HEAD^{tree}")
        (repository / "README").write_text("two\n", encoding="utf-8")
        subprocess.run(["git", "-C", str(repository), "add", "README"], check=True)
        subprocess.run(["git", "-C", str(repository), "commit", "-qm", "candidate"], check=True)
        evidence = root / "evidence"
        evidence.mkdir()
        payload = evidence / "child.log"
        payload.write_text("pass\n", encoding="utf-8")
        (evidence / "SHA256SUMS").write_text(f"{sha256_file(payload)}  child.log\n", encoding="utf-8")
        output = root / "receipt.json"
        args = argparse.Namespace(
            worktree=str(repository), output=str(output), expected_head=None, expected_tree=None,
            canonical_head=canonical, canonical_tree=canonical_tree, required_ancestor=[canonical],
            g5_anchor=canonical, g5_anchor_tree=canonical_tree,
            parent_head=None, parent_tree=None, evidence_dir=[str(evidence)], supersedes=[],
            qualification_status="bounded_candidate", operator_status="blocked_prep",
            provider_status="contract_only", integration_status="detached_only",
            caller_evidence_ratchet_present=False,
        )
        emit(args)
        verify_receipt(argparse.Namespace(receipt=str(output), worktree=str(repository), canonical_head=canonical, canonical_tree=canonical_tree))
        try:
            emit(args)
        except LedgerError:
            pass
        else:
            fail("self-test append-only reservation did not reject overwrite")
        payload.write_text("tampered\n", encoding="utf-8")
        try:
            verify_receipt(argparse.Namespace(receipt=str(output), worktree=None, canonical_head=None, canonical_tree=None))
        except LedgerError:
            pass
        else:
            fail("self-test did not detect a changed SHA256SUMS target")
    print("self-test: PASS")


def make_parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(description=__doc__)
    subparsers = root.add_subparsers(dest="command", required=True)
    emit_parser = subparsers.add_parser("emit", help="verify a candidate and publish one new receipt")
    emit_parser.add_argument("--worktree", required=True)
    emit_parser.add_argument("--output", required=True)
    emit_parser.add_argument("--expected-head")
    emit_parser.add_argument("--expected-tree")
    emit_parser.add_argument("--canonical-head", default=DEFAULT_CANONICAL_HEAD)
    emit_parser.add_argument("--canonical-tree", default=DEFAULT_CANONICAL_TREE)
    emit_parser.add_argument("--g5-anchor", default=DEFAULT_G5_ANCHOR)
    emit_parser.add_argument("--g5-anchor-tree", default=DEFAULT_G5_ANCHOR_TREE)
    emit_parser.add_argument("--required-ancestor", action="append", default=[])
    emit_parser.add_argument("--parent-head")
    emit_parser.add_argument("--parent-tree")
    emit_parser.add_argument("--evidence-dir", action="append", default=[])
    emit_parser.add_argument("--supersedes", action="append", default=[])
    emit_parser.add_argument("--qualification-status", default="bounded_candidate")
    emit_parser.add_argument("--operator-status", default="blocked_prep")
    emit_parser.add_argument("--provider-status", default="contract_only")
    emit_parser.add_argument("--integration-status", default="detached_only")
    emit_parser.add_argument("--caller-evidence-ratchet-present", action="store_true")
    verify_parser = subparsers.add_parser("verify", help="recheck one receipt and its evidence")
    verify_parser.add_argument("--receipt", required=True)
    verify_parser.add_argument("--worktree")
    verify_parser.add_argument("--canonical-head")
    verify_parser.add_argument("--canonical-tree")
    subparsers.add_parser("self-test", help="run a temporary repository self-test")
    return root


def main(argv: Sequence[str] | None = None) -> int:
    arguments = make_parser().parse_args(argv)
    try:
        if arguments.command == "emit":
            emit(arguments)
        elif arguments.command == "verify":
            verify_receipt(arguments)
        else:
            self_test()
    except LedgerError as exc:
        print(f"FAIL: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
