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

PLAN_VERSION = "4.5.1"
SPEC_VERSION = "1.3.0"
SPEC_RELATIVE = (
    "plans/hepta-intelligence/"
    "HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md"
)

plan_path = Path("plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json")
registry_path = Path(
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json"
)
integration_path = Path(
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
)
spec_path = Path(SPEC_RELATIVE)
verifier_path = Path("scripts/hepta-intelligence-current-truth.py")
target_head = os.environ["TARGET_HEAD"]
required_predecessors = json.loads(os.environ["PREDECESSOR_HEADS_JSON"])
if not isinstance(required_predecessors, list) or not required_predecessors:
    raise SystemExit("required predecessor chain missing")
if required_predecessors[-1] != target_head:
    raise SystemExit("target head must be the final predecessor")
if len(required_predecessors) != len(set(required_predecessors)):
    raise SystemExit("duplicate required predecessor")
if not all(
    isinstance(item, str)
    and len(item) == 40
    and all(char in "0123456789abcdef" for char in item)
    for item in required_predecessors
):
    raise SystemExit("invalid predecessor SHA")

# Preserve append-only A0 replacement provenance in the sole machine authority.
plan = json.loads(plan_path.read_text(encoding="utf-8"))
if plan.get("canonical", {}).get("plan_version") != PLAN_VERSION:
    raise SystemExit("unexpected plan version")
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

# Repair the stale subordinate execution-spec binding in the integration view.
integration = json.loads(integration_path.read_text(encoding="utf-8"))
operational_documents = integration.get("operational_documents")
if not isinstance(operational_documents, list) or len(operational_documents) != 1:
    raise SystemExit("operational_documents must contain exactly one entry")
operational_document = operational_documents[0]
required_operational_keys = {
    "classification",
    "content_sha256",
    "current_plan_authority",
    "path",
    "production_authority",
    "promotion_authority",
    "version",
}
if not isinstance(operational_document, dict) or set(operational_document) != required_operational_keys:
    raise SystemExit("operational document shape drift")
if operational_document.get("classification") != "SUBORDINATE_EXECUTION_SPEC":
    raise SystemExit("unexpected operational document classification")
if operational_document.get("path") != SPEC_RELATIVE:
    raise SystemExit("unexpected operational document path")
for authority_field in (
    "current_plan_authority",
    "production_authority",
    "promotion_authority",
):
    if operational_document.get(authority_field) is not False:
        raise SystemExit(f"positive operational document authority: {authority_field}")
spec_sha256 = hashlib.sha256(spec_path.read_bytes()).hexdigest()
operational_document["version"] = SPEC_VERSION
operational_document["content_sha256"] = spec_sha256

# Make the existing A0 documentation gap explicitly cover this cross-view binding.
global_entries = integration.get("gap_closure_ledger", {}).get("entries")
if not isinstance(global_entries, list):
    raise SystemExit("global gap entries missing")
a0_doc_entries = [entry for entry in global_entries if entry.get("gap_id") == "A0-DOC-001"]
if len(a0_doc_entries) != 1:
    raise SystemExit("A0-DOC-001 uniqueness failure")
a0_doc = a0_doc_entries[0]
acceptance = a0_doc.get("acceptance_tests")
closure = a0_doc.get("closure_evidence")
if not isinstance(acceptance, list) or not isinstance(closure, list):
    raise SystemExit("A0-DOC-001 acceptance/closure evidence missing")
acceptance_item = (
    "integration operational_documents path/version/digest/authority matches the exact "
    "subordinate execution specification"
)
closure_item = (
    "three-way CURRENT_PLAN/document-registry/integration-candidate execution-spec binding"
)
if acceptance_item not in acceptance:
    acceptance.append(acceptance_item)
if closure_item not in closure:
    closure.append(closure_item)
integration_bytes = (
    json.dumps(integration, indent=2, sort_keys=True, ensure_ascii=False) + "\n"
).encode("utf-8")
integration_path.write_bytes(integration_bytes)

# Extend the strict verifier without brittle line-number editing.
verifier = verifier_path.read_text(encoding="utf-8")
syntax = ast.parse(verifier)
matches: list[ast.Call] = []
for node in ast.walk(syntax):
    if not isinstance(node, ast.Call):
        continue
    if not isinstance(node.func, ast.Name) or node.func.id != "require":
        continue
    if len(node.args) < 2:
        continue
    marker = node.args[1]
    if isinstance(marker, ast.Constant) and marker.value == "A0 provenance drift":
        matches.append(node)
if len(matches) != 1:
    raise SystemExit(f"expected one A0 provenance require call, got {len(matches)}")
call = matches[0]
condition = call.args[0]
if not isinstance(condition, ast.Compare) or len(condition.comparators) != 1:
    raise SystemExit("A0 provenance condition is not one comparison")
expected_list = condition.comparators[0]
if not isinstance(expected_list, ast.List):
    raise SystemExit("A0 provenance comparator is not a list")
observed: list[str] = []
for element in expected_list.elts:
    if not isinstance(element, ast.Constant) or not isinstance(element.value, str):
        raise SystemExit("A0 provenance verifier contains non-string item")
    observed.append(element.value)
if observed != previous:
    raise SystemExit("source and verifier predecessor lists differ")
if call.end_lineno is None:
    raise SystemExit("Python parser did not provide end_lineno")
lines = verifier.splitlines(keepends=True)
indent = lines[call.lineno - 1][
    : len(lines[call.lineno - 1]) - len(lines[call.lineno - 1].lstrip())
]
replacement_lines = [
    f"{indent}require(\n",
    f'{indent}    current.get("a0_previous_exact_head_provenance")\n',
    f"{indent}    == [\n",
]
replacement_lines.extend(
    f"{indent}        {json.dumps(item)},\n" for item in provenance
)
replacement_lines.extend(
    [
        f"{indent}    ],\n",
        f'{indent}    "A0 provenance drift",\n',
        f"{indent})\n",
    ]
)
lines[call.lineno - 1 : call.end_lineno] = replacement_lines
verifier = "".join(lines)

anchor = '''    require(
        operational.get("execution_spec_sha256") == sha(PATHS["spec"]),
        "spec digest drift",
    )
'''
if verifier.count(anchor) != 1:
    raise SystemExit("execution-spec verifier anchor drift")
operational_verifier = '''    operational_documents = integration.get("operational_documents")
    require(
        isinstance(operational_documents, list) and len(operational_documents) == 1,
        "operational document registry cardinality",
    )
    operational_document = operational_documents[0]
    operational_document_required = {
        "classification",
        "content_sha256",
        "current_plan_authority",
        "path",
        "production_authority",
        "promotion_authority",
        "version",
    }
    require(
        isinstance(operational_document, dict),
        "operational document entry object",
    )
    exact_keys(
        operational_document,
        operational_document_required,
        operational_document_required,
        "operational document entry",
    )
    require(
        operational_document
        == {
            "classification": "SUBORDINATE_EXECUTION_SPEC",
            "content_sha256": sha(PATHS["spec"]),
            "current_plan_authority": False,
            "path": READ_ORDER[-1],
            "production_authority": False,
            "promotion_authority": False,
            "version": SPEC_VERSION,
        },
        "operational document binding drift",
    )
'''
verifier = verifier.replace(anchor, anchor + operational_verifier)
verifier_path.write_text(verifier, encoding="utf-8")

# Rebind every canonical raw input and then recompute the registry self-digest.
registry = json.loads(registry_path.read_text(encoding="utf-8"))
inputs = registry.get("registered_canonical_inputs")
if not isinstance(inputs, list):
    raise SystemExit("registered canonical inputs missing")
self_entries = [item for item in inputs if item.get("path") == registry_path.as_posix()]
if len(self_entries) != 1:
    raise SystemExit("canonical registry self-entry uniqueness failure")
for item in inputs:
    path_value = item.get("path")
    digest_scope = item.get("digest_scope")
    if not isinstance(path_value, str):
        raise SystemExit("canonical registry path missing")
    source_path = Path(path_value)
    if digest_scope == "RAW_FILE_BYTES":
        item["content_sha256"] = hashlib.sha256(source_path.read_bytes()).hexdigest()
    elif digest_scope == "CANONICAL_JSON_WITH_SELF_DIGEST_NULL":
        if source_path != registry_path:
            raise SystemExit("unexpected canonical self-digest target")
    else:
        raise SystemExit(f"unknown canonical digest scope: {digest_scope}")
self_entry = self_entries[0]
self_entry["content_sha256"] = None
canonical_registry = json.dumps(
    registry,
    sort_keys=True,
    separators=(",", ":"),
    ensure_ascii=False,
).encode("utf-8")
self_entry["content_sha256"] = hashlib.sha256(canonical_registry).hexdigest()
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
  plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json
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
message='fix(intelligence): bind A0 operational specification exactly

Replace the current A0 tree as one Q0 child, retain the superseded exact head, repair the integration-candidate execution-spec version and digest, add strict three-way verifier coverage plus a negative regression, and recompute canonical registry bindings. Preserve the governance-only 17-path surface and every negative authority.'
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

# Prove that the new verifier rejects the exact stale-version defect that escaped before.
integration_path="plans/hepta-intelligence/HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json"
python3 - "$integration_path" <<'PY'
from __future__ import annotations
import json
from pathlib import Path
import sys
path = Path(sys.argv[1])
doc = json.loads(path.read_text(encoding="utf-8"))
doc["operational_documents"][0]["version"] = "1.2.0"
path.write_text(json.dumps(doc, indent=2, sort_keys=True, ensure_ascii=False) + "\n", encoding="utf-8")
PY
if python3 scripts/hepta-intelligence-current-truth.py --verify \
  >"$report_dir/negative-operational-version.stdout" \
  2>"$report_dir/negative-operational-version.stderr"; then
  echo "stale operational document version was incorrectly accepted" >&2
  exit 1
fi
git show "HEAD:$integration_path" > "$integration_path"
python3 scripts/hepta-intelligence-current-truth.py --verify
git diff --exit-code

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
    "schema": "hepta_a0_v451_operational_document_replacement_source_publisher_v1",
    "status": "PASS_HEPTA_A0_V451_OPERATIONAL_DOCUMENT_REPLACEMENT_SOURCE_PUBLISHER",
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
    "predecessor_chain_appended": json.loads(os.environ["PREDECESSOR_HEADS_JSON"]),
    "staging_branch": os.environ["STAGING_BRANCH"],
    "changed_paths": changed,
    "resolved_findings": [
        "A0_OPERATIONAL_DOCUMENT_VERSION_DIGEST_DRIFT",
        "A0_VERIFIER_OPERATIONAL_DOCUMENT_COVERAGE",
        "A0_STALE_VERSION_NEGATIVE_REGRESSION",
        "COMPLETE_SUPERSEDED_EXACT_HEAD_PROVENANCE",
        "CANONICAL_DOCUMENT_REGISTRY_REBINDING",
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
canonical = json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode("utf-8")
receipt["receipt_binding_sha256"] = hashlib.sha256(canonical).hexdigest()
output.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
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

printf 'PASS_HEPTA_A0_V451_OPERATIONAL_DOCUMENT_REPLACEMENT_SOURCE_PUBLISHER commit=%s tree=%s staging=%s\n' \
  "$commit" "$tree" "$STAGING_BRANCH"
