#!/usr/bin/env python3
"""Apply, preflight, squash, and publish the exact P0.3.4 source candidate."""

from __future__ import annotations

import base64
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tarfile
import tempfile
from pathlib import Path, PurePosixPath
from typing import Sequence

EXPECTED_TARGET = os.environ["EXPECTED_TARGET"]
P033_HEAD = os.environ["P033_HEAD"]
TARGET_BRANCH = os.environ["TARGET_BRANCH"]
PAYLOAD_SHA256 = os.environ["PAYLOAD_SHA256"]
HELPER_ROOT = Path(__file__).resolve().parent.parent
PAYLOAD = HELPER_ROOT / "ci/p034-payload/part-00.b64"
STAGING_MARKER = ".p034-staging-head-v2"
RUST_FILES = (
    "hepta-memory/src/fact_grounding/legacy_governance.rs",
    "hepta-memory/src/fact_grounding/durable/backfill.rs",
    "hepta-memory/src/fact_grounding/durable.rs",
    "hepta-memory/src/framing.rs",
    "hepta-memory/src/lib.rs",
)
P02_RUST_FILES = (
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
FALSE_AUTHORITY_FLAGS = (
    "wired",
    "production_projection_gate",
    "production_authority",
    "external_effects",
    "operator_accepted",
    "promoted",
    "callers_ratchet",
)


def run(
    args: Sequence[str],
    *,
    cwd: Path,
    capture: bool = False,
    check: bool = True,
) -> subprocess.CompletedProcess[str]:
    print("+", " ".join(args), flush=True)
    result = subprocess.run(
        list(args),
        cwd=cwd,
        text=True,
        stdout=subprocess.PIPE if capture else None,
        stderr=subprocess.STDOUT if capture else None,
        check=False,
    )
    if capture and result.stdout:
        print(result.stdout, end="" if result.stdout.endswith("\n") else "\n")
    if check and result.returncode != 0:
        raise RuntimeError(
            f"command failed with exit code {result.returncode}: {' '.join(args)}"
        )
    return result


def git(root: Path, *args: str, capture: bool = False, check: bool = True) -> str:
    result = run(("git", *args), cwd=root, capture=capture, check=check)
    return (result.stdout or "").strip()


def is_single_successor(root: Path, head: str) -> bool:
    try:
        return (
            git(root, "rev-parse", f"{head}^", capture=True) == P033_HEAD
            and git(
                root,
                "rev-list",
                "--count",
                f"{P033_HEAD}..{head}",
                capture=True,
            )
            == "1"
        )
    except RuntimeError:
        return False


def is_staging_descendant(root: Path, head: str) -> bool:
    result = run(
        ("git", "merge-base", "--is-ancestor", P033_HEAD, head),
        cwd=root,
        check=False,
    )
    if result.returncode != 0:
        return False
    count = git(
        root,
        "rev-list",
        "--count",
        f"{P033_HEAD}..{head}",
        capture=True,
    )
    return count.isdigit() and int(count) >= 1


def decode_payload(destination: Path) -> Path:
    encoded = PAYLOAD.read_text(encoding="ascii")
    payload = base64.b64decode("".join(encoded.split()), validate=True)
    actual = hashlib.sha256(payload).hexdigest()
    if actual != PAYLOAD_SHA256:
        raise RuntimeError(
            f"P0.3.4 payload digest mismatch: expected {PAYLOAD_SHA256}, got {actual}"
        )

    archive = destination / "payload.tar.gz"
    archive.write_bytes(payload)
    extracted = destination / "tree"
    extracted.mkdir()
    with tarfile.open(archive, mode="r:gz") as tar:
        for member in tar.getmembers():
            path = PurePosixPath(member.name)
            if (
                path.is_absolute()
                or ".." in path.parts
                or member.issym()
                or member.islnk()
                or member.isdev()
            ):
                raise RuntimeError(f"unsafe P0.3.4 payload entry: {member.name}")
        try:
            tar.extractall(extracted, filter="data")
        except TypeError:
            tar.extractall(extracted)

    apply_script = extracted / "p034_apply_patch.py"
    if not apply_script.is_file():
        raise RuntimeError("P0.3.4 payload is missing p034_apply_patch.py")
    return apply_script


def receipt_path(root: Path) -> Path:
    return (
        root
        / "artifacts/hepta-intelligence-legacy-grounding-governance-v1"
        / "qualification-receipt.json"
    )


def enforce_receipt(root: Path) -> None:
    path = receipt_path(root)
    if not path.is_file():
        raise RuntimeError("P0.3.4 qualification receipt was not produced")
    receipt = json.loads(path.read_text(encoding="utf-8"))
    checks = receipt.get("checks")
    if not isinstance(checks, list) or not checks:
        raise RuntimeError("P0.3.4 qualification receipt has no executable checks")
    failed = [
        str(check.get("id"))
        for check in checks
        if check.get("passed") is not True or check.get("exit_code") != 0
    ]
    if failed:
        raise RuntimeError(f"P0.3.4 source candidate gates failed: {failed}")
    source_receipt = receipt.get("source_receipt")
    if not isinstance(source_receipt, dict) or source_receipt.get("status") != (
        "PASS_P0_3_4_LEGACY_GOVERNANCE_SOURCE_ONLY"
    ):
        raise RuntimeError("P0.3.4 source contract did not pass")
    for key in FALSE_AUTHORITY_FLAGS:
        if receipt.get(key) is not False:
            raise RuntimeError(f"P0.3.4 authority boundary drifted: {key}")


def run_predecessor_admission(root: Path) -> None:
    run(
        (sys.executable, "scripts/verify-hepta-intelligence-grounding-ledger.py"),
        cwd=root,
    )
    run(
        (
            "rustfmt",
            "--edition",
            "2024",
            "--config",
            "skip_children=true",
            "--check",
            *P02_RUST_FILES,
        ),
        cwd=root / "codex-rs",
    )


def run_p034_admission(root: Path) -> None:
    qualification = run(
        (
            sys.executable,
            "scripts/run-hepta-intelligence-legacy-grounding-governance-v1.py",
        ),
        cwd=root,
        check=False,
    )
    enforce_receipt(root)
    if qualification.returncode != 0:
        raise RuntimeError(
            f"P0.3.4 qualification runner exited {qualification.returncode}"
        )


def clean_receipt(root: Path) -> None:
    shutil.rmtree(
        root / "artifacts/hepta-intelligence-legacy-grounding-governance-v1",
        ignore_errors=True,
    )


def requalify_existing(root: Path, head: str) -> str:
    if not is_single_successor(root, head):
        raise RuntimeError(
            f"P0.3.4 target drifted to a non-canonical head: {head}"
        )
    if git(root, "status", "--porcelain", capture=True):
        raise RuntimeError("existing P0.3.4 candidate checkout is not clean")
    if (root / STAGING_MARKER).exists():
        raise RuntimeError("canonical P0.3.4 candidate retained the staging marker")
    run_predecessor_admission(root)
    run_p034_admission(root)
    clean_receipt(root)
    if git(root, "status", "--porcelain", capture=True):
        raise RuntimeError("existing P0.3.4 requalification dirtied the source tree")
    print(f"P0.3.4 existing canonical candidate requalified at {head}")
    return head


def publish(root: Path) -> str:
    head = git(root, "rev-parse", "HEAD", capture=True)
    if head != EXPECTED_TARGET:
        return requalify_existing(root, head)
    if not is_staging_descendant(root, head):
        raise RuntimeError("P0.3.4 staging head is not based on qualified P0.3.3")
    if git(root, "status", "--porcelain", capture=True):
        raise RuntimeError("P0.3.4 staging checkout is not clean")

    (root / STAGING_MARKER).unlink(missing_ok=True)
    with tempfile.TemporaryDirectory(prefix="p034-payload-") as temporary:
        apply_script = decode_payload(Path(temporary))
        run((sys.executable, str(apply_script)), cwd=root)

    git(root, "diff", "--check")
    workspace = root / "codex-rs"
    run(
        (
            "rustfmt",
            "--edition",
            "2024",
            "--config",
            "skip_children=true",
            *RUST_FILES,
        ),
        cwd=workspace,
    )
    git(root, "diff", "--check")
    run_predecessor_admission(root)
    run_p034_admission(root)

    clean_receipt(root)
    git(root, "reset", "--soft", P033_HEAD)
    git(root, "add", "-A")
    git(root, "diff", "--cached", "--check")
    changed = git(root, "diff", "--cached", "--name-only", capture=True).splitlines()
    if len(changed) <= 1:
        raise RuntimeError(f"P0.3.4 governed delta is incomplete: {changed}")
    forbidden = [
        path
        for path in changed
        if path == STAGING_MARKER
        or path.startswith("ci/p034-payload/")
        or path.startswith(".github/workflows/p034-")
    ]
    if forbidden:
        raise RuntimeError(f"temporary P0.3.4 files entered candidate: {forbidden}")

    git(root, "config", "user.name", "Qian QI")
    git(
        root,
        "config",
        "user.email",
        "102159240+ProfHepta@users.noreply.github.com",
    )
    git(
        root,
        "commit",
        "--no-gpg-sign",
        "-m",
        "feat(memory): qualify legacy grounding governance P0.3.4",
    )
    final_head = git(root, "rev-parse", "HEAD", capture=True)
    if not is_single_successor(root, final_head):
        raise RuntimeError("P0.3.4 final candidate is not one commit above P0.3.3")
    final_tree = git(root, "rev-parse", "HEAD^{tree}", capture=True)

    pushed = run(
        (
            "git",
            "push",
            f"--force-with-lease=refs/heads/{TARGET_BRANCH}:{EXPECTED_TARGET}",
            "origin",
            f"HEAD:refs/heads/{TARGET_BRANCH}",
        ),
        cwd=root,
        check=False,
    )
    if pushed.returncode != 0:
        git(
            root,
            "fetch",
            "--no-tags",
            "origin",
            f"+refs/heads/{TARGET_BRANCH}:refs/remotes/origin/{TARGET_BRANCH}",
        )
        remote = git(
            root,
            "rev-parse",
            f"refs/remotes/origin/{TARGET_BRANCH}",
            capture=True,
        )
        remote_tree = git(
            root,
            "rev-parse",
            f"{remote}^{{tree}}",
            capture=True,
        )
        if not is_single_successor(root, remote) or remote_tree != final_tree:
            raise RuntimeError(
                "P0.3.4 publish lost lease to a non-equivalent candidate"
            )
        print(f"Equivalent P0.3.4 candidate already published at {remote}")
        return remote

    print(final_head)
    return final_head


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: p034-qualified-publisher.py TARGET_CHECKOUT")
    root = Path(sys.argv[1]).resolve()
    if not (root / ".git").exists():
        raise SystemExit(f"not a Git checkout: {root}")
    publish(root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
