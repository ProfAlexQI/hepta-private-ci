# shellcheck shell=bash
git diff --check
git diff --name-only | sort > "$ARTIFACT_DIR/changed-files.txt"
git diff --stat > "$ARTIFACT_DIR/repair-stat.txt"
git diff --binary > "$ARTIFACT_DIR/repair.patch"

Q0_PASS_STATUS="$q0_pass_status" Q0_FAIL_STATUS="$q0_fail_status" \
python3 - "$ARTIFACT_DIR" <<'PY'
import hashlib
import json
import os
import pathlib
import subprocess
import sys

root = pathlib.Path(sys.argv[1])
exit_paths = sorted(root.glob("gates/*.exit")) + sorted(root.glob("rust/*.exit"))
results = {}
for path in exit_paths:
    try:
        code = int(path.read_text(encoding="utf-8").strip())
    except (OSError, ValueError):
        code = 255
    results[str(path.relative_to(root))] = code
all_zero = bool(results) and all(code == 0 for code in results.values())
receipt = {
    "schema": "hepta.intelligence.q0.executable_evidence.v1",
    "evidence_class": os.environ["Q0_EVIDENCE_CLASS"],
    "status": os.environ["Q0_PASS_STATUS"] if all_zero else os.environ["Q0_FAIL_STATUS"],
    "candidate": {
        "head": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(),
        "tree": subprocess.check_output(["git", "rev-parse", "HEAD^{tree}"], text=True).strip(),
        "parent": subprocess.check_output(["git", "rev-parse", "HEAD^"], text=True).strip(),
    },
    "workflow": {
        "name": os.environ.get("GITHUB_WORKFLOW"),
        "run_id": int(os.environ["GITHUB_RUN_ID"]),
        "run_attempt": int(os.environ["GITHUB_RUN_ATTEMPT"]),
        "job": os.environ.get("GITHUB_JOB"),
    },
    "runner": {
        "name": os.environ.get("RUNNER_NAME"),
        "os": os.environ.get("RUNNER_OS"),
        "arch": os.environ.get("RUNNER_ARCH"),
    },
    "results": results,
    "all_gates_zero": all_zero,
    "result_manifest_sha256": hashlib.sha256(
        json.dumps(results, sort_keys=True, separators=(",", ":")).encode()
    ).hexdigest(),
    "source_writeback": False,
    "runtime_authority": False,
    "external_effects": False,
    "production_authority": False,
    "operator_acceptance": False,
    "promotion": False,
    "callers_ratchet": False,
    "qualified_candidate": False,
}
filename = (
    "e1-qualification-receipt.json"
    if os.environ["Q0_EVIDENCE_CLASS"] == "E1_LOCAL_EXECUTABLE"
    else "e2-qualification-receipt.json"
)
(root / filename).write_text(
    json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
)
PY

for result in "$ARTIFACT_DIR"/gates/*.exit "$ARTIFACT_DIR"/rust/*.exit; do
  test "$(cat "$result")" = "0"
done
