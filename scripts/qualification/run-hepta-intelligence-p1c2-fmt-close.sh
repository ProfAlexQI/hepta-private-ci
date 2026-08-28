#!/usr/bin/env bash
set -euo pipefail

SOURCE_BRANCH="codex/hepta-p1c2-eval-20260828"
SOURCE_SHA="c3c42f60e5ca11fd49fe5d4d2013c5b21e183619"
TRIGGER_SHA="${GITHUB_SHA:-$(git rev-parse HEAD)}"
DRIVER_PATH="scripts/qualification/run-hepta-intelligence-p1c2-fmt-close.sh"
SEMANTIC_PATH="scripts/qualification/materialize-hepta-intelligence-p1c2-semantic-closure.py"
PROJECTION_PATH="scripts/qualification/materialize-hepta-intelligence-p1c2-projection-state-closure.py"
REFERENCE_PATH="scripts/qualification/materialize-hepta-intelligence-p1c2-reference-closure.py"
PLAN_PATH_MATERIALIZER="scripts/qualification/materialize-hepta-intelligence-p1c2-plan-contract-closure.py"
LEGACY_TRUST_PATH="scripts/qualification/materialize-hepta-intelligence-p1c2-final-trust-closure.py"
FINAL_TRUST_PATH="scripts/qualification/materialize-hepta-intelligence-p1c2-final-trust-v2.py"
WRAPPER_PATH=".github/workflows/hepta-intelligence-p1c2-fmt-close.yml"
OUTPUT_BRANCH="qualification/p1c2-trust-candidate-c3c42-20260828"
CRATE_ROOT="codex-rs/hepta-memory-p1-1c2-qualification"
MANIFEST_PATH="$CRATE_ROOT/Cargo.toml"
PACKAGE="hepta-memory-p1-1c2-qualification"
PRODUCT_WORKFLOW=".github/workflows/hepta-intelligence-p1-1c2-reviewed-efficacy.yml"
VERIFIER_PATH="scripts/verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py"
PLAN_PATH="plans/hepta-intelligence/P1-1C2_REVIEWED_CORPUS_EFFICACY_PLAN.md"
STATUS_PATH="plans/hepta-intelligence/P1-1C2_EXECUTION_STATUS.json"
RECEIPT_PATH="plans/hepta-intelligence/P1-1C2_IMPLEMENTATION_RECEIPT.json"
ARTIFACT_DIR="artifacts/hepta-intelligence-p1-1c2-trust-closure"
MATERIALIZER_DIR="$RUNNER_TEMP/p1c2-materializers"
export ARTIFACT_DIR TRIGGER_SHA

mkdir -p "$ARTIFACT_DIR" "$MATERIALIZER_DIR"
test "$(git rev-parse HEAD)" = "$TRIGGER_SHA"

git fetch --no-tags --depth=1 origin \
  "+refs/heads/$SOURCE_BRANCH:refs/hepta/p1c2-source"
test "$(git rev-parse refs/hepta/p1c2-source)" = "$SOURCE_SHA"

python3 - \
  "$SOURCE_SHA" "$DRIVER_PATH" "$SEMANTIC_PATH" "$PROJECTION_PATH" \
  "$REFERENCE_PATH" "$PLAN_PATH_MATERIALIZER" "$LEGACY_TRUST_PATH" \
  "$FINAL_TRUST_PATH" "$WRAPPER_PATH" <<'PY'
import json
import subprocess
import sys
source, *expected = sys.argv[1:]
actual = subprocess.check_output(
    ["git", "diff", "--name-only", f"{source}..HEAD"], text=True
).splitlines()
if sorted(actual) != sorted(expected):
    raise SystemExit(json.dumps({"expected": sorted(expected), "actual": sorted(actual)}, indent=2))
PY

python3 -m py_compile \
  "$SEMANTIC_PATH" "$PROJECTION_PATH" "$REFERENCE_PATH" \
  "$PLAN_PATH_MATERIALIZER" "$LEGACY_TRUST_PATH" "$FINAL_TRUST_PATH"

for path in \
  "$SEMANTIC_PATH" "$PROJECTION_PATH" "$REFERENCE_PATH" \
  "$PLAN_PATH_MATERIALIZER" "$LEGACY_TRUST_PATH" "$FINAL_TRUST_PATH"; do
  cp "$path" "$MATERIALIZER_DIR/$(basename "$path")"
done

python3 - "$TRIGGER_SHA" "$SOURCE_SHA" "$SOURCE_BRANCH" <<'PY'
import json
import os
import subprocess
import sys
from pathlib import Path
trigger, source, branch = sys.argv[1:]
receipt = {
    "schema": "hepta.intelligence.p1_1c2.qualification_source_binding.v3",
    "trigger_commit": trigger,
    "canonical_branch": branch,
    "canonical_commit": source,
    "canonical_object_type": subprocess.check_output(
        ["git", "cat-file", "-t", source], text=True
    ).strip(),
    "runtime_wired": False,
    "production_authority": False,
}
Path(os.environ["ARTIFACT_DIR"] + "/source-binding.json").write_text(
    json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
)
PY

git checkout --detach refs/hepta/p1c2-source
python3 "$MATERIALIZER_DIR/$(basename "$SEMANTIC_PATH")"
python3 "$MATERIALIZER_DIR/$(basename "$PROJECTION_PATH")"
python3 "$MATERIALIZER_DIR/$(basename "$REFERENCE_PATH")"
python3 "$MATERIALIZER_DIR/$(basename "$PLAN_PATH_MATERIALIZER")"
python3 "$MATERIALIZER_DIR/$(basename "$FINAL_TRUST_PATH")"

export CARGO_NET_OFFLINE=true
export CARGO_TARGET_DIR="$RUNNER_TEMP/hepta-p1c2-trust-closure-target"
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
    raise SystemExit(json.dumps({"required": sorted(required), "allowed": sorted(allowed), "actual": sorted(actual)}, indent=2))
Path("artifacts/hepta-intelligence-p1-1c2-trust-closure/changed-paths.json").write_text(
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
text = Path("artifacts/hepta-intelligence-p1-1c2-trust-closure/tests.log").read_text(encoding="utf-8", errors="replace")
passed = sum(int(v) for v in re.findall(r"test result: ok\. (\d+) passed", text))
if passed != 25:
    raise SystemExit(f"expected 25 P1.1c.2 tests, observed {passed}")
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
path = Path("artifacts/hepta-intelligence-p1-1c2-trust-closure/evaluation-a.json")
text = path.read_text(encoding="utf-8")
receipt = json.loads(text)
assert receipt["status"] == "BLOCKED_P1_1C2_REVIEWED_CORPUS_DEPENDENCY"
assert receipt["reviewed_corpus_present"] is False
assert receipt["projection_complete"] is False
assert receipt["final_label_bindings_match"] is False
assert receipt["acceptance_policy_matches"] is True
assert receipt["baseline_receipt_matches"] is True
assert receipt["calibration_contract_matches"] is True
assert receipt["efficacy_policy_matches"] is True
assert receipt["reviewed_corpus_evaluated"] is False
assert receipt["efficacy_thresholds_passed"] is False
assert receipt["efficacy_validation"] is False
assert receipt["efficacy_claim"] is False
assert receipt["lanes"] == []
assert receipt["blocked_reasons"] == sorted(set(receipt["blocked_reasons"]))
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
Path("artifacts/hepta-intelligence-p1-1c2-trust-closure/evaluation-a.json.sha256").write_text(
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
git add -- "$PRODUCT_WORKFLOW" "$VERIFIER_PATH" "$PLAN_PATH" "$STATUS_PATH" \
  "$RECEIPT_PATH" "$CRATE_ROOT/src" "$CRATE_ROOT/tests"
git commit -m "fix(intelligence): close P1.1c.2 trust and receipt contracts"
CANDIDATE_SHA="$(git rev-parse HEAD)"
test "$(git rev-parse "$CANDIDATE_SHA^")" = "$SOURCE_SHA"
git push origin "$CANDIDATE_SHA:refs/heads/$OUTPUT_BRANCH"

export CANDIDATE_SHA
python3 - <<'PY'
import json
import os
from pathlib import Path
receipt = {
    "schema": "hepta.intelligence.p1_1c2.trust_closure.v3",
    "status": "PASS_P1_1C2_TRUST_AND_RECEIPT_CLOSURE",
    "trigger_commit": os.environ.get("TRIGGER_SHA"),
    "source_commit": "c3c42f60e5ca11fd49fe5d4d2013c5b21e183619",
    "candidate_commit": os.environ["CANDIDATE_SHA"],
    "tests_passed": 25,
    "gates": {
        "canonical_source_object_fetch": "success",
        "trigger_changed_path_allowlist": "success",
        "source_gate": "success",
        "rust_1_95_0": "success",
        "package_scoped_rustfmt": "success",
        "final_label_eligibility": "success",
        "potential_contradiction_preserved": "success",
        "privacy_redact_and_block_distinguished": "success",
        "reference_acceptance_policy": "success",
        "reference_seed_baseline": "success",
        "reference_calibration": "success",
        "reference_efficacy_policy": "success",
        "receipt_status_blocker_lane_canonicality": "success",
        "tests": "success",
        "check": "success",
        "clippy": "success",
        "receipt_reproducibility": "success",
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
Path("artifacts/hepta-intelligence-p1-1c2-trust-closure/qualification-receipt.json").write_text(
    json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
)
PY
