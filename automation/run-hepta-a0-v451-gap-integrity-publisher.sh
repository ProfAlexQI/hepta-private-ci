#!/usr/bin/env bash
set -euo pipefail

: "${EXPECTED_REPOSITORY:?}"
: "${TARGET_BRANCH:?}"
: "${TARGET_HEAD:?}"
: "${TARGET_TREE:?}"
: "${Q0_HEAD:?}"
: "${Q0_TREE:?}"
: "${PATCH_SHA256:?}"
: "${PATCH_PAYLOAD:?}"
: "${MATRIX_LABEL:?}"

publisher="$GITHUB_WORKSPACE/publisher"
target="$GITHUB_WORKSPACE/target"
report_dir="$target/artifacts/hepta-a0-v451-gap-integrity"
staging_branch="staging/hepta-a0-v451-gap-integrity-20260829-2315-${MATRIX_LABEL}"

[[ "$GITHUB_REPOSITORY" == "$EXPECTED_REPOSITORY" ]]
[[ "$(git -C "$target" rev-parse HEAD)" == "$TARGET_HEAD" ]]
[[ "$(git -C "$target" rev-parse HEAD^{tree})" == "$TARGET_TREE" ]]
[[ "$(git -C "$target" rev-parse HEAD^)" == "$Q0_HEAD" ]]
[[ "$(git -C "$target" rev-parse "$Q0_HEAD^{tree}")" == "$Q0_TREE" ]]
remote_head="$(git -C "$target" ls-remote origin "refs/heads/$TARGET_BRANCH" | awk '{print $1}')"
[[ "$remote_head" == "$TARGET_HEAD" ]]

python3 - "$publisher/$PATCH_PAYLOAD" /tmp/repair.raw.patch /tmp/repair.patch "$PATCH_SHA256" <<'PY'
from __future__ import annotations
import base64
import gzip
import hashlib
from pathlib import Path
import re
import sys

source, raw_output, output, expected = map(Path, sys.argv[1:4]) + [sys.argv[4]] if False else (Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]), sys.argv[4])
raw = gzip.decompress(base64.b64decode(source.read_bytes()))
actual = hashlib.sha256(raw).hexdigest()
if actual != expected:
    raise SystemExit(f"patch digest mismatch: {actual} != {expected}")
raw_output.write_bytes(raw)
text = raw.decode("utf-8")
text = re.sub(r"^--- /tmp/v451candidate/", "--- a/", text, flags=re.MULTILINE)
text = re.sub(r"^\+\+\+ /tmp/v451fix2/", "+++ b/", text, flags=re.MULTILINE)
output.write_text(text, encoding="utf-8")
PY

git -C "$target" apply --check --whitespace=error-all /tmp/repair.patch
git -C "$target" apply --whitespace=error-all /tmp/repair.patch
unexpected="$(git -C "$target" status --porcelain --untracked-files=no | grep -vE '^( M|M ) ' || true)"
[[ -z "$unexpected" ]]

git -C "$target" config user.name "Hepta governed source publisher"
git -C "$target" config user.email "hepta-source-publisher@users.noreply.github.com"
git -C "$target" add -u
tree="$(git -C "$target" write-tree)"
message='fix(intelligence): enforce V4.5.1 gap and evidence integrity

Reject unknown cross-ledger dependencies, bind every multimodal blocker to a registered gap, enforce receipt-specific closure evidence, validate canonical registries structurally, guard security-sensitive Python against optimized execution, and move Python-only A0 source gates to executable Ubuntu 22.04 and macOS 15 lanes. Preserve the exact 17-path A0 surface and every negative authority.'
commit="$(
  printf '%s\n' "$message" |
    GIT_AUTHOR_NAME='Hepta governed source publisher' \
    GIT_AUTHOR_EMAIL='hepta-source-publisher@users.noreply.github.com' \
    GIT_COMMITTER_NAME='Hepta governed source publisher' \
    GIT_COMMITTER_EMAIL='hepta-source-publisher@users.noreply.github.com' \
    GIT_AUTHOR_DATE='2026-08-29T23:15:00Z' \
    GIT_COMMITTER_DATE='2026-08-29T23:15:00Z' \
    git -C "$target" commit-tree "$tree" -p "$Q0_HEAD"
)"
git -C "$target" reset --hard "$commit"
[[ "$(git -C "$target" rev-parse HEAD^)" == "$Q0_HEAD" ]]

cd "$target"
mkdir -p "$report_dir"
python3 -m py_compile \
  scripts/hepta-intelligence-current-truth.py \
  scripts/verify-hepta-intelligence-a0-authority.py \
  scripts/verify-hepta-intelligence-document-authority.py \
  scripts/verify-hepta-intelligence-master-plan.py
python3 scripts/verify-hepta-intelligence-master-plan.py
python3 scripts/verify-hepta-intelligence-document-authority.py
python3 scripts/hepta-intelligence-current-truth.py --verify
for tranche in P0.2 P0.3 P0.4a P0.4b P0.4c; do
  python3 scripts/hepta-intelligence-status-compat.py "$tranche" --check-only
done
GITHUB_SHA="$commit" \
GITHUB_HEAD_REF="$TARGET_BRANCH" \
GITHUB_REF_NAME="$TARGET_BRANCH" \
GITHUB_REPOSITORY="$EXPECTED_REPOSITORY" \
  python3 scripts/verify-hepta-intelligence-a0-authority.py \
  | tee "$report_dir/a0-source-evidence-receipt.json"
git diff --check "$Q0_HEAD" "$commit"

python3 - "$Q0_HEAD" "$commit" "$report_dir/changed-paths.json" <<'PY'
import json
from pathlib import Path
import subprocess
import sys

base, head, output = sys.argv[1:]
changed = sorted(filter(None, subprocess.check_output(
    ["git", "diff", "--name-only", base, head], text=True
).splitlines()))
expected = json.loads(Path(
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
).read_text(encoding="utf-8"))["allowed_changed_paths"]
if changed != expected or len(changed) != 17:
    raise SystemExit(f"changed-path drift: {changed}")
Path(output).write_text(json.dumps({
    "changed_paths": changed,
    "count": len(changed),
}, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY
[[ -z "$(git status --porcelain --untracked-files=no)" ]]

CANDIDATE_COMMIT="$commit" CANDIDATE_TREE="$tree" STAGING_BRANCH="$staging_branch" \
python3 - "$report_dir/publisher-receipt.json" <<'PY'
from __future__ import annotations
import hashlib
import json
import os
from pathlib import Path
import sys

output = Path(sys.argv[1])
receipt = {
    "schema": "hepta_a0_v451_gap_integrity_source_publisher_v1",
    "status": "PASS_HEPTA_A0_V451_GAP_INTEGRITY_SOURCE_PUBLISHER",
    "repository": os.environ["EXPECTED_REPOSITORY"],
    "platform": os.environ["MATRIX_LABEL"],
    "candidate": {
        "branch": os.environ["TARGET_BRANCH"],
        "head": os.environ["CANDIDATE_COMMIT"],
        "tree": os.environ["CANDIDATE_TREE"],
        "parent": os.environ["Q0_HEAD"],
    },
    "staging_branch": os.environ["STAGING_BRANCH"],
    "predecessor": {
        "head": os.environ["TARGET_HEAD"],
        "tree": os.environ["TARGET_TREE"],
    },
    "patch_sha256": os.environ["PATCH_SHA256"],
    "resolved_findings": [
        "STRICT_CROSS_LEDGER_DEPENDENCY_NAMESPACE",
        "RECEIPT_SPECIFIC_MULTIMODAL_CLOSURE_EVIDENCE",
        "STRICT_PR_STACK_AND_EVIDENCE_INDEX_VALIDATION",
        "EXACT_AUTHORITY_AND_TOP_LEVEL_SCHEMAS",
        "CLAIM_LADDER_REPOSITORY_ALIAS_CLASSIFICATION",
        "EXACT_WORKFLOW_TRIGGER_PATH_STRUCTURE",
        "PYTHON_OPTIMIZATION_GUARDS_FOR_SECURITY_CHECKS",
        "EXECUTABLE_MULTI_PLATFORM_SOURCE_GATE_LANES",
    ],
    "a0_candidate_qualified": False,
    "selected": False,
    "source_writeback_by_candidate_workflow": False,
    "authority": {
        "runtime_wired": False,
        "default_open_wired": False,
        "memory_write_authority": False,
        "projection_write_authority": False,
        "learning_write_authority": False,
        "model_runtime_authority": False,
        "provider_dispatch_authority": False,
        "external_effects": False,
        "production_authority": False,
        "operator_acceptance": False,
        "promotion": False,
        "release_authority": False,
        "callers_ratchet": False,
    },
}
encoded = json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()
receipt["receipt_binding_sha256"] = hashlib.sha256(encoded).hexdigest()
output.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY
printf '%s\n' "$commit" > "$report_dir/commit.txt"
printf '%s\n' "$tree" > "$report_dir/tree.txt"

remote_head="$(git ls-remote origin "refs/heads/$TARGET_BRANCH" | awk '{print $1}')"
[[ "$remote_head" == "$TARGET_HEAD" ]]
[[ -z "$(git ls-remote origin "refs/heads/$staging_branch")" ]]
git push origin "$commit:refs/heads/$staging_branch"

printf 'PASS_HEPTA_A0_V451_GAP_INTEGRITY_SOURCE_PUBLISHER platform=%s commit=%s tree=%s staging=%s\n' \
  "$MATRIX_LABEL" "$commit" "$tree" "$staging_branch"
