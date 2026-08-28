#!/usr/bin/env bash
set -euo pipefail

SOURCE_SHA="df95a8c9e3b0555f73aa8f943fe1c72133ccdf3c"
SOURCE_BRANCH="codex/p1c3-evidence-intake-20260829"
PATCH_SCRIPT="scripts/qualification/materialize-p1c3-fix-v1.py"
MAC_WORKFLOW=".github/workflows/hepta-intelligence-p1c3-fix-v1.yml"
DRIVER_PATH="scripts/qualification/run-p1c3-fix-x64.sh"
X64_WORKFLOW=".github/workflows/hepta-intelligence-p1c3-fix-x64.yml"
OUTPUT_BRANCH="qualification/p1c3-candidate-x64-v1"
CRATE_ROOT="codex-rs/hepta-memory-p1-1c3-qualification"
MANIFEST_PATH="$CRATE_ROOT/Cargo.toml"
VERIFIER_PATH="scripts/verify-hepta-intelligence-p1-1c3-evidence-intake.py"
ARTIFACT_DIR="artifacts/hepta-intelligence-p1-1c3-fix-x64"

mkdir -p "$ARTIFACT_DIR"
git fetch --no-tags origin \
  "+refs/heads/$SOURCE_BRANCH:refs/remotes/origin/$SOURCE_BRANCH"
test "$(git rev-parse "refs/remotes/origin/$SOURCE_BRANCH")" = "$SOURCE_SHA"
python3 - <<'PY'
import json
import os
import subprocess
expected = sorted([
    os.environ["PATCH_SCRIPT"],
    os.environ["MAC_WORKFLOW"],
    os.environ["DRIVER_PATH"],
    os.environ["X64_WORKFLOW"],
])
actual = sorted(subprocess.check_output(
    ["git", "diff", "--name-only", f"{os.environ['SOURCE_SHA']}..HEAD"],
    text=True,
).splitlines())
if actual != expected:
    raise SystemExit(json.dumps({"expected": expected, "actual": actual}, indent=2))
PY
python3 -m py_compile "$PATCH_SCRIPT"
cp "$PATCH_SCRIPT" "$RUNNER_TEMP/materialize-p1c3-fix-v1.py"

git checkout -B p1c3-x64-work "$SOURCE_SHA"
python3 "$RUNNER_TEMP/materialize-p1c3-fix-v1.py"
export CARGO_TARGET_DIR="$RUNNER_TEMP/hepta-p1c3-x64-target"
rm -f "$CRATE_ROOT/Cargo.lock"
rm -rf "$CRATE_ROOT/target"

python3 "$VERIFIER_PATH" 2>&1 | tee "$ARTIFACT_DIR/source-gate.json"
rustc --version | tee "$ARTIFACT_DIR/rustc-version.txt"
test "$(rustc --version | awk '{print $2}')" = "1.95.0"
cargo fmt --manifest-path "$MANIFEST_PATH" --all
cargo fmt --manifest-path "$MANIFEST_PATH" --all -- --check \
  2>&1 | tee "$ARTIFACT_DIR/rustfmt.log"
cargo test --manifest-path "$MANIFEST_PATH" --all-targets -- --nocapture \
  2>&1 | tee "$ARTIFACT_DIR/tests.log"
python3 - <<'PY'
import re
from pathlib import Path
text = Path("artifacts/hepta-intelligence-p1-1c3-fix-x64/tests.log").read_text(
    encoding="utf-8", errors="replace"
)
passed = sum(int(value) for value in re.findall(r"test result: ok\. (\d+) passed", text))
if passed != 16:
    raise SystemExit(f"expected 16 P1.1c.3 tests, observed {passed}")
PY
cargo check --manifest-path "$MANIFEST_PATH" --all-targets \
  2>&1 | tee "$ARTIFACT_DIR/check.log"
cargo clippy --manifest-path "$MANIFEST_PATH" --all-targets -- -D warnings \
  2>&1 | tee "$ARTIFACT_DIR/clippy.log"

cargo run --quiet --manifest-path "$MANIFEST_PATH" --bin p1_1c3_receipt \
  > "$ARTIFACT_DIR/intake-a.json"
cargo run --quiet --manifest-path "$MANIFEST_PATH" --bin p1_1c3_receipt \
  > "$ARTIFACT_DIR/intake-b.json"
cmp "$ARTIFACT_DIR/intake-a.json" "$ARTIFACT_DIR/intake-b.json"
python3 - <<'PY'
import json
from pathlib import Path
text = Path("artifacts/hepta-intelligence-p1-1c3-fix-x64/intake-a.json").read_text(encoding="utf-8")
receipt = json.loads(text)
assert receipt["status"] == "BLOCKED_P1_1C3_TRUSTED_CORPUS_INTAKE"
assert receipt["external_evidence_complete"] is False
assert receipt["mechanically_accepted"] is False
assert receipt["production_authority"] is False
assert receipt["efficacy_claim"] is False
assert receipt["promotion"] is False
assert receipt["blocked_reasons"] == sorted(set(receipt["blocked_reasons"]))
for forbidden in ["reviewer-a", "public_key", "signature", "license-text"]:
    assert forbidden not in text, forbidden
PY

rm -f "$CRATE_ROOT/Cargo.lock"
rm -rf "$CRATE_ROOT/target"
git diff --check
test -z "$(git ls-files --others --exclude-standard -- "$CRATE_ROOT")"
python3 - <<'PY'
import json
import subprocess
actual = subprocess.check_output(["git", "diff", "--name-only"], text=True).splitlines()
if not actual or any(not path.startswith("codex-rs/hepta-memory-p1-1c3-qualification/") for path in actual):
    raise SystemExit(json.dumps({"actual": actual}, indent=2))
PY

git config user.name "Hepta Qualification Bot"
git config user.email "102159240+ProfAlexQI@users.noreply.github.com"
git add -- "$CRATE_ROOT"
git commit -m "fix(intelligence): qualify P1.1c.3 trust gateway source"
CANDIDATE_SHA="$(git rev-parse HEAD)"
test "$(git rev-parse "$CANDIDATE_SHA^")" = "$SOURCE_SHA"
git push origin "$CANDIDATE_SHA:refs/heads/$OUTPUT_BRANCH"

export CANDIDATE_SHA
python3 - <<'PY'
import json
import os
from pathlib import Path
Path("artifacts/hepta-intelligence-p1-1c3-fix-x64/qualification-receipt.json").write_text(
    json.dumps({
        "schema": "hepta.intelligence.p1_1c3.source_fix_qualification.v1",
        "status": "PASS_P1_1C3_EXECUTABLE_SOURCE_GATES",
        "source_commit": os.environ["SOURCE_SHA"],
        "candidate_commit": os.environ["CANDIDATE_SHA"],
        "tests_passed": 16,
        "source_qualified": True,
        "external_evidence_present": False,
        "trusted_corpus_accepted": False,
        "qualified": False,
        "runtime_wired": False,
        "production_authority": False,
        "efficacy_claim": False,
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
    }, indent=2, sort_keys=True) + "\n",
    encoding="utf-8",
)
PY
