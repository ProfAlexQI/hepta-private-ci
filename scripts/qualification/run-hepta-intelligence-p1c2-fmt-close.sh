#!/usr/bin/env bash
set -euo pipefail

SOURCE_SHA="c3c42f60e5ca11fd49fe5d4d2013c5b21e183619"
DRIVER_PATH="scripts/qualification/run-hepta-intelligence-p1c2-fmt-close.sh"
MATERIALIZER_PATH="scripts/qualification/materialize-hepta-intelligence-p1c2-semantic-closure.py"
WRAPPER_PATH=".github/workflows/hepta-intelligence-p1c2-fmt-close.yml"
OUTPUT_BRANCH="qualification/p1c2-semantic-candidate-c3c42-20260828"
CRATE_ROOT="codex-rs/hepta-memory-p1-1c2-qualification"
MANIFEST_PATH="$CRATE_ROOT/Cargo.toml"
PACKAGE="hepta-memory-p1-1c2-qualification"
WORKFLOW_PATH=".github/workflows/hepta-intelligence-p1-1c2-reviewed-efficacy.yml"
VERIFIER_PATH="scripts/verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py"
PLAN_PATH="plans/hepta-intelligence/P1-1C2_REVIEWED_CORPUS_EFFICACY_PLAN.md"
STATUS_PATH="plans/hepta-intelligence/P1-1C2_EXECUTION_STATUS.json"
RECEIPT_PATH="plans/hepta-intelligence/P1-1C2_IMPLEMENTATION_RECEIPT.json"
ARTIFACT_DIR="artifacts/hepta-intelligence-p1-1c2-semantic-closure"

mkdir -p "$ARTIFACT_DIR"
python3 - "$SOURCE_SHA" "$DRIVER_PATH" "$MATERIALIZER_PATH" "$WRAPPER_PATH" <<'PY'
import subprocess
import sys
source, driver, materializer, wrapper = sys.argv[1:]
expected = sorted([driver, materializer, wrapper])
actual = sorted(subprocess.check_output(
    ["git", "diff", "--name-only", f"{source}..HEAD"], text=True
).splitlines())
if actual != expected:
    raise SystemExit({"expected": expected, "actual": actual})
PY
python3 -m py_compile "$MATERIALIZER_PATH"
cp "$MATERIALIZER_PATH" "$RUNNER_TEMP/materialize-hepta-intelligence-p1c2-semantic-closure.py"

git checkout -B p1c2-semantic-closure-work "$SOURCE_SHA"
python3 "$RUNNER_TEMP/materialize-hepta-intelligence-p1c2-semantic-closure.py"

export CARGO_NET_OFFLINE=true
export CARGO_TARGET_DIR="$RUNNER_TEMP/hepta-p1c2-semantic-closure-target"
rm -f "$CRATE_ROOT/Cargo.lock"
rm -rf "$CRATE_ROOT/target"

rustc --version | tee "$ARTIFACT_DIR/rustc-version.txt"
cargo --version | tee "$ARTIFACT_DIR/cargo-version.txt"
rustfmt --version | tee "$ARTIFACT_DIR/rustfmt-version.txt"
cargo clippy --version | tee "$ARTIFACT_DIR/clippy-version.txt"
test "$(rustc --version | awk '{print $2}')" = "1.95.0"

cargo fmt --manifest-path "$MANIFEST_PATH" --package "$PACKAGE"
cargo fmt --manifest-path "$MANIFEST_PATH" --package "$PACKAGE" -- --check \
  2>&1 | tee "$ARTIFACT_DIR/rustfmt.log"

python3 - <<'PY'
import json
import subprocess
from pathlib import Path
required = {
    ".github/workflows/hepta-intelligence-p1-1c2-reviewed-efficacy.yml",
    "codex-rs/hepta-memory-p1-1c2-qualification/src/evaluation.rs",
    "codex-rs/hepta-memory-p1-1c2-qualification/src/projection.rs",
    "codex-rs/hepta-memory-p1-1c2-qualification/tests/p1_1c2.rs",
    "plans/hepta-intelligence/P1-1C2_EXECUTION_STATUS.json",
    "plans/hepta-intelligence/P1-1C2_IMPLEMENTATION_RECEIPT.json",
    "plans/hepta-intelligence/P1-1C2_REVIEWED_CORPUS_EFFICACY_PLAN.md",
    "scripts/verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py",
}
allowed = required | {
    "codex-rs/hepta-memory-p1-1c2-qualification/src/bin/p1_1c2_receipt.rs",
    "codex-rs/hepta-memory-p1-1c2-qualification/src/digest.rs",
    "codex-rs/hepta-memory-p1-1c2-qualification/src/lib.rs",
}
actual = set(subprocess.check_output(["git", "diff", "--name-only"], text=True).splitlines())
if not required.issubset(actual) or not actual.issubset(allowed):
    raise SystemExit(json.dumps({
        "required": sorted(required),
        "allowed": sorted(allowed),
        "actual": sorted(actual),
    }, indent=2, sort_keys=True))
Path("artifacts/hepta-intelligence-p1-1c2-semantic-closure/changed-paths.json").write_text(
    json.dumps({"changed_paths": sorted(actual)}, indent=2, sort_keys=True) + "\n",
    encoding="utf-8",
)
PY

python3 "$VERIFIER_PATH" 2>&1 | tee "$ARTIFACT_DIR/source-gate.json"

cargo test --manifest-path "$MANIFEST_PATH" --all-targets -- --nocapture \
  2>&1 | tee "$ARTIFACT_DIR/tests.log"
python3 - <<'PY'
import re
from pathlib import Path
text = Path("artifacts/hepta-intelligence-p1-1c2-semantic-closure/tests.log").read_text(
    encoding="utf-8", errors="replace"
)
passed = sum(int(value) for value in re.findall(r"test result: ok\. (\d+) passed", text))
if passed != 18:
    raise SystemExit(f"expected 18 P1.1c.2 tests, observed {passed}")
PY
cargo check --manifest-path "$MANIFEST_PATH" --all-targets \
  2>&1 | tee "$ARTIFACT_DIR/check.log"
cargo clippy --manifest-path "$MANIFEST_PATH" --all-targets -- -D warnings \
  2>&1 | tee "$ARTIFACT_DIR/clippy.log"

cargo run --quiet --manifest-path "$MANIFEST_PATH" --bin p1_1c2_receipt \
  > "$ARTIFACT_DIR/evaluation-a.json"
cargo run --quiet --manifest-path "$MANIFEST_PATH" --bin p1_1c2_receipt \
  > "$ARTIFACT_DIR/evaluation-b.json"
cmp "$ARTIFACT_DIR/evaluation-a.json" "$ARTIFACT_DIR/evaluation-b.json"
python3 - <<'PY'
import hashlib
import json
from pathlib import Path
path = Path("artifacts/hepta-intelligence-p1-1c2-semantic-closure/evaluation-a.json")
text = path.read_text(encoding="utf-8")
receipt = json.loads(text)
assert receipt["status"] == "BLOCKED_P1_1C2_REVIEWED_CORPUS_DEPENDENCY"
assert receipt["reviewed_corpus_present"] is False
assert receipt["projection_complete"] is False
assert receipt["final_label_bindings_match"] is False
assert receipt["reviewed_corpus_evaluated"] is False
assert receipt["efficacy_thresholds_passed"] is False
assert receipt["efficacy_validation"] is False
assert receipt["efficacy_claim"] is False
assert receipt["lanes"] == []
for key in (
    "source_qualified", "product_workspace_member", "product_module_registered",
    "runtime_wired", "default_recall_changed", "federation_recall_changed",
    "context_attachment", "physical_send", "network_access", "model_download",
    "external_effects", "production_authority", "operator_acceptance",
    "promotion", "callers_ratchet",
):
    assert receipt[key] is False, key
for forbidden in (
    "Which evidence binds", "哪些证据", "en-ann-gold", "en-case-001",
    "qualification-reviewer", "qualification-rationale",
):
    assert forbidden not in text, forbidden
Path("artifacts/hepta-intelligence-p1-1c2-semantic-closure/evaluation-a.json.sha256").write_text(
    hashlib.sha256(path.read_bytes()).hexdigest() + "\n", encoding="utf-8"
)
PY

rm -f "$CRATE_ROOT/Cargo.lock"
rm -rf "$CRATE_ROOT/target"
git diff --check
test -z "$(git ls-files --others --exclude-standard -- "$CRATE_ROOT")"

python3 - <<'PY'
import json
import subprocess
required = {
    ".github/workflows/hepta-intelligence-p1-1c2-reviewed-efficacy.yml",
    "codex-rs/hepta-memory-p1-1c2-qualification/src/evaluation.rs",
    "codex-rs/hepta-memory-p1-1c2-qualification/src/projection.rs",
    "codex-rs/hepta-memory-p1-1c2-qualification/tests/p1_1c2.rs",
    "plans/hepta-intelligence/P1-1C2_EXECUTION_STATUS.json",
    "plans/hepta-intelligence/P1-1C2_IMPLEMENTATION_RECEIPT.json",
    "plans/hepta-intelligence/P1-1C2_REVIEWED_CORPUS_EFFICACY_PLAN.md",
    "scripts/verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py",
}
allowed = required | {
    "codex-rs/hepta-memory-p1-1c2-qualification/src/bin/p1_1c2_receipt.rs",
    "codex-rs/hepta-memory-p1-1c2-qualification/src/digest.rs",
    "codex-rs/hepta-memory-p1-1c2-qualification/src/lib.rs",
}
actual = set(subprocess.check_output(["git", "diff", "--name-only"], text=True).splitlines())
if not required.issubset(actual) or not actual.issubset(allowed):
    raise SystemExit(json.dumps({"allowed": sorted(allowed), "actual": sorted(actual)}, indent=2))
PY

git config user.name "Hepta Qualification Bot"
git config user.email "102159240+ProfAlexQI@users.noreply.github.com"
git add -- \
  "$WORKFLOW_PATH" \
  "$VERIFIER_PATH" \
  "$PLAN_PATH" \
  "$STATUS_PATH" \
  "$RECEIPT_PATH" \
  "$CRATE_ROOT/src" \
  "$CRATE_ROOT/tests"
git commit -m "fix(intelligence): bind P1.1c.2 to final reviewed labels"
CANDIDATE_SHA="$(git rev-parse HEAD)"
test "$(git rev-parse "$CANDIDATE_SHA^")" = "$SOURCE_SHA"
git push origin "$CANDIDATE_SHA:refs/heads/$OUTPUT_BRANCH"

export CANDIDATE_SHA
python3 - <<'PY'
import json
import os
from pathlib import Path
receipt = {
    "schema": "hepta.intelligence.p1_1c2.semantic_closure.v1",
    "status": "PASS_P1_1C2_FINAL_LABEL_SEMANTIC_CLOSURE",
    "source_commit": "c3c42f60e5ca11fd49fe5d4d2013c5b21e183619",
    "candidate_commit": os.environ["CANDIDATE_SHA"],
    "tests_passed": 18,
    "gates": {
        "source_gate": "success",
        "rust_1_95_0": "success",
        "package_scoped_rustfmt": "success",
        "final_relevance_binding": "success",
        "citation_representation_gate": "success",
        "contradiction_binding": "success",
        "privacy_materialization_gate": "success",
        "tests": "success",
        "check": "success",
        "clippy": "success",
        "receipt_reproducibility": "success",
        "blocker_semantics": "success",
        "redaction": "success",
        "clean_scope": "success",
    },
    "parent_crates_reformatted": False,
    "source_qualified": False,
    "reviewed_corpus_present": False,
    "reviewed_corpus_evaluated": False,
    "efficacy_validation": False,
    "efficacy_claim": False,
    "runtime_wired": False,
    "production_authority": False,
    "operator_acceptance": False,
    "promotion": False,
    "runner": {
        "name": os.environ.get("RUNNER_NAME"),
        "os": os.environ.get("RUNNER_OS"),
        "arch": os.environ.get("RUNNER_ARCH"),
    },
    "workflow": {
        "run_id": int(os.environ["GITHUB_RUN_ID"]),
        "run_attempt": int(os.environ["GITHUB_RUN_ATTEMPT"]),
    },
}
Path("artifacts/hepta-intelligence-p1-1c2-semantic-closure/qualification-receipt.json").write_text(
    json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
)
PY
