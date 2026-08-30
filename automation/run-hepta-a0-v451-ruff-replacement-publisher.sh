#!/usr/bin/env bash
set -euo pipefail

: "${EXPECTED_REPOSITORY:?}"
: "${TARGET_BRANCH:?}"
: "${TARGET_HEAD:?}"
: "${TARGET_TREE:?}"
: "${Q0_HEAD:?}"
: "${Q0_TREE:?}"
: "${PREDECESSOR_HEADS_JSON:?}"
: "${STAGING_BRANCH:?}"
: "${REPORT_DIR:?}"

publisher="$GITHUB_WORKSPACE/publisher"
target="$GITHUB_WORKSPACE/target"
report_dir="$target/$REPORT_DIR"

[[ "$GITHUB_REPOSITORY" == "$EXPECTED_REPOSITORY" ]]
[[ "$(git -C "$target" rev-parse HEAD)" == "$TARGET_HEAD" ]]
[[ "$(git -C "$target" rev-parse HEAD^{tree})" == "$TARGET_TREE" ]]
[[ "$(git -C "$target" rev-parse HEAD^)" == "$Q0_HEAD" ]]
[[ "$(git -C "$target" rev-parse "$Q0_HEAD^{tree}")" == "$Q0_TREE" ]]
remote_head="$(git -C "$target" ls-remote origin "refs/heads/$TARGET_BRANCH" | awk '{print $1}')"
[[ "$remote_head" == "$TARGET_HEAD" ]]
[[ -z "$(git -C "$target" ls-remote origin "refs/heads/$STAGING_BRANCH")" ]]

cd "$target"
python3 - <<'PY'
from __future__ import annotations

import ast
import hashlib
import json
import os
from pathlib import Path
import re

plan_path = Path("plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json")
registry_path = Path(
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
)
verifier_path = Path("scripts/hepta-intelligence-current-truth.py")
target_head = os.environ["TARGET_HEAD"]
required_predecessors = json.loads(os.environ["PREDECESSOR_HEADS_JSON"])
if not isinstance(required_predecessors, list) or not required_predecessors:
    raise SystemExit("required predecessor chain missing")
if required_predecessors[-1] != target_head:
    raise SystemExit("target head must be the final predecessor")
if len(required_predecessors) != len(set(required_predecessors)):
    raise SystemExit("duplicate required predecessor")

plan = json.loads(plan_path.read_text(encoding="utf-8"))
provenance = plan.get("a0_previous_exact_head_provenance")
if not isinstance(provenance, list) or not provenance:
    raise SystemExit("invalid A0 provenance list")
previous = list(provenance)
for predecessor in required_predecessors:
    if predecessor not in provenance:
        provenance.append(predecessor)
if provenance[-len(required_predecessors) :] != required_predecessors:
    raise SystemExit("predecessor order drift")
plan_bytes = (
    json.dumps(plan, indent=2, sort_keys=True, ensure_ascii=False) + "\n"
).encode("utf-8")
plan_path.write_bytes(plan_bytes)

verifier = verifier_path.read_text(encoding="utf-8")
pattern = re.compile(
    r"(require\(\s*current\.get\(\s*['\"]a0_previous_exact_head_provenance['\"]\s*\)\s*==\s*)"
    r"(\[[^\]]*\])"
    r"(\s*,\s*['\"]A0 provenance drift['\"]\s*\))",
    re.DOTALL,
)
match = pattern.search(verifier)
if match is None:
    raise SystemExit("A0 provenance verifier clause not found")
observed = ast.literal_eval(match.group(2))
if observed != previous:
    raise SystemExit("source and verifier predecessor lists differ")
replacement = match.group(1) + repr(provenance) + match.group(3)
verifier_path.write_text(
    verifier[: match.start()] + replacement + verifier[match.end() :],
    encoding="utf-8",
)

registry = json.loads(registry_path.read_text(encoding="utf-8"))
inputs = registry.get("registered_canonical_inputs")
if not isinstance(inputs, list):
    raise SystemExit("registered canonical inputs missing")
current_entry = next(
    item for item in inputs if item.get("path") == plan_path.as_posix()
)
current_entry["content_sha256"] = hashlib.sha256(plan_bytes).hexdigest()

self_entry = next(
    item for item in inputs if item.get("path") == registry_path.as_posix()
)
if self_entry.get("digest_scope") != "CANONICAL_JSON_WITH_SELF_DIGEST_NULL":
    raise SystemExit("unexpected document-registry self-digest scope")
self_entry["content_sha256"] = None
canonical = json.dumps(
    registry,
    sort_keys=True,
    separators=(",", ":"),
    ensure_ascii=False,
).encode("utf-8")
self_entry["content_sha256"] = hashlib.sha256(canonical).hexdigest()
registry_path.write_text(
    json.dumps(registry, indent=2, sort_keys=True, ensure_ascii=False) + "\n",
    encoding="utf-8",
)
PY

uv run --frozen --project scripts ruff format \
  scripts/hepta-intelligence-current-truth.py \
  scripts/verify-hepta-intelligence-a0-authority.py \
  scripts/verify-hepta-intelligence-document-authority.py \
  scripts/verify-hepta-intelligence-master-plan.py
uv run --frozen --project scripts ruff format --check \
  scripts/hepta-intelligence-current-truth.py \
  scripts/verify-hepta-intelligence-a0-authority.py \
  scripts/verify-hepta-intelligence-document-authority.py \
  scripts/verify-hepta-intelligence-master-plan.py

mapfile -t repair_paths < <(git diff --name-only | sort)
printf '%s\n' "${repair_paths[@]}"
expected_repairs=(
  plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json
  plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json
  scripts/hepta-intelligence-current-truth.py
)
[[ "${#repair_paths[@]}" -eq "${#expected_repairs[@]}" ]]
for index in "${!expected_repairs[@]}"; do
  [[ "${repair_paths[$index]}" == "${expected_repairs[$index]}" ]]
done
git diff --check

git config user.name "Hepta governed source publisher"
git config user.email "hepta-source-publisher@users.noreply.github.com"
git add "${expected_repairs[@]}"
tree="$(git write-tree)"
message='fix(intelligence): complete V4.5.1 A0 predecessor provenance

Retain the Ruff-clean verifier sources, record every superseded exact A0 head, and recompute canonical document-registry bindings. Preserve the 17-path governance-only surface and every negative authority.'
commit="$(printf '%s\n' "$message" | git commit-tree "$tree" -p "$Q0_HEAD")"
git reset --hard "$commit"
[[ "$(git rev-parse HEAD^)" == "$Q0_HEAD" ]]
[[ "$(git rev-list --count "$Q0_HEAD"..HEAD)" -eq 1 ]]

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
uv run --frozen --project scripts ruff format --check \
  scripts/hepta-intelligence-current-truth.py \
  scripts/verify-hepta-intelligence-a0-authority.py \
  scripts/verify-hepta-intelligence-document-authority.py \
  scripts/verify-hepta-intelligence-master-plan.py
git diff --check "$Q0_HEAD" "$commit"
[[ -z "$(git status --porcelain --untracked-files=no)" ]]

CANDIDATE_COMMIT="$commit" CANDIDATE_TREE="$tree" \
python3 - "$report_dir/publisher-receipt.json" <<'PY'
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

output = Path(sys.argv[1])
changed = sorted(
    filter(
        None,
        subprocess.check_output(
            [
                "git",
                "diff",
                "--name-only",
                os.environ["Q0_HEAD"],
                os.environ["CANDIDATE_COMMIT"],
            ],
            text=True,
        ).splitlines(),
    )
)
expected = json.loads(
    Path(
        "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
    ).read_text(encoding="utf-8")
)["allowed_changed_paths"]
if changed != expected or len(changed) != 17:
    raise SystemExit(f"changed-path drift: {changed}")
receipt = {
    "schema": "hepta_a0_v451_ruff_replacement_source_publisher_v2",
    "status": "PASS_HEPTA_A0_V451_RUFF_REPLACEMENT_SOURCE_PUBLISHER",
    "repository": os.environ["EXPECTED_REPOSITORY"],
    "candidate": {
        "branch": os.environ["TARGET_BRANCH"],
        "head": os.environ["CANDIDATE_COMMIT"],
        "tree": os.environ["CANDIDATE_TREE"],
        "parent": os.environ["Q0_HEAD"],
    },
    "predecessor": {
        "head": os.environ["TARGET_HEAD"],
        "tree": os.environ["TARGET_TREE"],
    },
    "predecessor_chain_appended": json.loads(
        os.environ["PREDECESSOR_HEADS_JSON"]
    ),
    "staging_branch": os.environ["STAGING_BRANCH"],
    "changed_paths": changed,
    "resolved_findings": [
        "A0_OWNED_PYTHON_RUFF_FORMATTING",
        "COMPLETE_SUPERSEDED_EXACT_HEAD_PROVENANCE",
        "CANONICAL_DOCUMENT_REGISTRY_REBINDING",
        "BOT_PUSH_ACTION_REQUIRED_AVOIDED_BY_ACCOUNT_REF_CAS",
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
canonical = json.dumps(
    receipt, sort_keys=True, separators=(",", ":")
).encode("utf-8")
receipt["receipt_binding_sha256"] = hashlib.sha256(canonical).hexdigest()
output.write_text(
    json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
)
Path(os.environ["REPORT_DIR"], "commit.txt").write_text(
    os.environ["CANDIDATE_COMMIT"] + "\n", encoding="utf-8"
)
Path(os.environ["REPORT_DIR"], "tree.txt").write_text(
    os.environ["CANDIDATE_TREE"] + "\n", encoding="utf-8"
)
PY

remote_head="$(git ls-remote origin "refs/heads/$TARGET_BRANCH" | awk '{print $1}')"
[[ "$remote_head" == "$TARGET_HEAD" ]]
[[ -z "$(git ls-remote origin "refs/heads/$STAGING_BRANCH")" ]]
git push origin "$commit:refs/heads/$STAGING_BRANCH"

printf 'PASS_HEPTA_A0_V451_RUFF_REPLACEMENT_SOURCE_PUBLISHER commit=%s tree=%s staging=%s\n' \
  "$commit" "$tree" "$STAGING_BRANCH"
