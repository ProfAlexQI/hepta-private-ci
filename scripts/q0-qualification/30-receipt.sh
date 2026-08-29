# shellcheck shell=bash
case "$Q0_EVIDENCE_CLASS" in
  E1_LOCAL_EXECUTABLE) receipt_name=e1-qualification-receipt.json ;;
  E2_INDEPENDENT_RUNNER) receipt_name=e2-qualification-receipt.json ;;
  *) printf 'unsupported evidence class: %s\n' "$Q0_EVIDENCE_CLASS" >&2; exit 2 ;;
esac

python3 - "$ARTIFACT_DIR" "$receipt_name" <<'PY'
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Any

root = Path(sys.argv[1])
receipt_name = sys.argv[2]
HEX40 = re.compile(r"[0-9a-f]{40}")
NEGATIVE_AUTHORITY = {
    "runtime_wired": False,
    "external_effects": False,
    "kg_write_authority": False,
    "model_authority": False,
    "provider_effects": False,
    "fleet_authority": False,
    "production_authority": False,
    "operator_acceptance": False,
    "promotion": False,
    "release_authority": False,
    "callers_ratchet": False,
}


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def canonical(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], text=True).strip()


def load_exits(suffix: str) -> dict[str, int]:
    result: dict[str, int] = {}
    for path in sorted(root.rglob(f"*{suffix}")):
        rel = path.relative_to(root).as_posix()
        key = rel[: -len(suffix)]
        raw = path.read_text(encoding="utf-8").strip()
        result[key] = int(raw)
    return result


# Diagnostics use the longer suffix and must never leak into the blocking map.
diagnostics = load_exits(".diagnostic.exit")
blocking: dict[str, int] = {}
for path in sorted(root.rglob("*.exit")):
    if path.name.endswith(".diagnostic.exit"):
        continue
    rel = path.relative_to(root).as_posix()
    blocking[rel[: -len(".exit")]] = int(path.read_text(encoding="utf-8").strip())
assert blocking, "empty blocking result surface"

candidate = {"head": git("rev-parse", "HEAD"), "tree": git("rev-parse", "HEAD^{tree}"), "parent": git("rev-parse", "HEAD^")}
assert all(HEX40.fullmatch(value) for value in candidate.values())
assert candidate["head"] == os.environ["GITHUB_SHA"]
assert candidate["parent"] == os.environ["EXPECTED_PARENT"]
assert os.environ["Q0_WORKFLOW_SHA"] == candidate["head"]

# Recompute and bind the exact source overlay after all gates have completed.
source_path = root / "source-overlay-manifest.json"
source = json.loads(source_path.read_text(encoding="utf-8"))
changed_bytes = (root / "changed-files.txt").read_bytes()
actual_changed = subprocess.check_output(["git", "diff", "--name-only"], text=True)
actual_changed = "\n".join(sorted(line for line in actual_changed.splitlines() if line)) + "\n"
assert changed_bytes == actual_changed.encode(), "changed-file surface drifted after capture"
assert source["changed_files_sha256"] == sha256_bytes(changed_bytes)
assert (root / "repair.patch").read_bytes() == subprocess.check_output(["git", "diff", "--binary"])
assert (root / "repair-stat.txt").read_bytes() == subprocess.check_output(["git", "diff", "--stat"])
assert source["repair_patch_sha256"] == sha256_file(root / "repair.patch")
assert source["repair_patch_size"] == (root / "repair.patch").stat().st_size
assert source["repair_stat_sha256"] == sha256_file(root / "repair-stat.txt")
expected_paths = [line for line in changed_bytes.decode().splitlines() if line]
assert [entry["path"] for entry in source["files"]] == expected_paths
for entry in source["files"]:
    path = Path(entry["path"])
    assert path.is_file(), entry["path"]
    assert entry["sha256"] == sha256_file(path), entry["path"]
    assert entry["size"] == path.stat().st_size, entry["path"]
source_base = {key: value for key, value in source.items() if key != "manifest_sha256"}
assert source["manifest_sha256"] == sha256_bytes(canonical(source_base))

# Bind every uploaded evidence file except the receipt itself.  Pair validation
# rejects missing, extra, or modified files after download.
excluded = {"e1-qualification-receipt.json", "e2-qualification-receipt.json", "q0-evidence-pair-receipt.json"}
artifact_files = []
for path in sorted(item for item in root.rglob("*") if item.is_file()):
    rel = path.relative_to(root).as_posix()
    if rel in excluded:
        continue
    artifact_files.append({"path": rel, "sha256": sha256_file(path), "size": path.stat().st_size})
artifact_manifest_sha256 = sha256_bytes(canonical(artifact_files))
all_gates_zero = all(code == 0 for code in blocking.values())

receipt: dict[str, Any] = {
    "schema": "hepta.intelligence.q0.executable_evidence.v2",
    "status": os.environ["q0_pass_status"] if all_gates_zero else os.environ["q0_fail_status"],
    "evidence_class": os.environ["Q0_EVIDENCE_CLASS"],
    "repository": {
        "full_name": os.environ["GITHUB_REPOSITORY"],
        "repository_id": int(os.environ["Q0_REPOSITORY_ID"]),
        "owner_id": int(os.environ["Q0_REPOSITORY_OWNER_ID"]),
    },
    "candidate": candidate,
    "workflow": {
        "name": os.environ["GITHUB_WORKFLOW"],
        "path": os.environ["Q0_WORKFLOW_PATH"],
        "ref": os.environ["Q0_WORKFLOW_REF"],
        "sha": os.environ["Q0_WORKFLOW_SHA"],
        "run_id": int(os.environ["GITHUB_RUN_ID"]),
        "run_attempt": int(os.environ["GITHUB_RUN_ATTEMPT"]),
        "job": os.environ["GITHUB_JOB"],
    },
    "runner": {
        "name": os.environ["RUNNER_NAME"],
        "os": os.environ["RUNNER_OS"],
        "arch": os.environ["RUNNER_ARCH"],
        "host": os.environ["Q0_EXPECTED_RUST_HOST"],
    },
    "blocking_results": blocking,
    "blocking_results_sha256": sha256_bytes(canonical(blocking)),
    "diagnostics": diagnostics,
    "diagnostics_sha256": sha256_bytes(canonical(diagnostics)),
    "all_gates_zero": all_gates_zero,
    "source_overlay": source,
    "artifact_manifest": artifact_files,
    "artifact_manifest_sha256": artifact_manifest_sha256,
    "source_writeback": False,
    "qualified_candidate": False,
    "authority": NEGATIVE_AUTHORITY,
}
receipt["receipt_binding_sha256"] = sha256_bytes(canonical(receipt))
encoded = json.dumps(receipt, indent=2, sort_keys=True) + "\n"
(root / receipt_name).write_text(encoded, encoding="utf-8")
print(encoded, end="")
raise SystemExit(0 if all_gates_zero else 1)
PY
