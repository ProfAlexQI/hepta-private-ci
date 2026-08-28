#!/usr/bin/env bash
set -euo pipefail

SOURCE_SHA="98de1c4c3d11c6644ff46f50e80071f6f15e1652"
SOURCE_BRANCH="codex/hepta-intelligence-local-embedding-index-v1b-20260828"
PATCH_SCRIPT="scripts/qualification/materialize-hepta-intelligence-p1-1b-bounded-read.py"
RUNNER_SCRIPT="scripts/qualification/run-hepta-intelligence-p1-1b-bounded-read.sh"
WRAPPER_PATH=".github/workflows/hepta-intelligence-p1-1b-bounded-read-arm-v2.yml"
OUTPUT_BRANCH="qualification/hepta-intelligence-p1-1b-bounded-read-candidate-98de1c4-20260828"
CRATE_ROOT="codex-rs/hepta-memory-p1-1b-qualification"
MANIFEST_PATH="$CRATE_ROOT/Cargo.toml"
ARTIFACT_DIR="artifacts/hepta-intelligence-p1-1b-bounded-read"
BASE_VERIFIER="scripts/verify-hepta-intelligence-local-embedding-index.py"
HARDENING_VERIFIER="scripts/verify-hepta-intelligence-p1-1b-hardening.py"
BOUNDED_READ_VERIFIER="scripts/verify-hepta-intelligence-p1-1b-bounded-read.py"

mkdir -p "$ARTIFACT_DIR"
git fetch --no-tags origin \
  "+refs/heads/$SOURCE_BRANCH:refs/remotes/origin/$SOURCE_BRANCH"
test "$(git rev-parse "refs/remotes/origin/$SOURCE_BRANCH")" = "$SOURCE_SHA"
git merge-base --is-ancestor "$SOURCE_SHA" HEAD
python3 - "$SOURCE_SHA" "$PATCH_SCRIPT" "$RUNNER_SCRIPT" "$WRAPPER_PATH" <<'PY'
import subprocess
import sys
source, patch, runner, wrapper = sys.argv[1:]
expected = sorted([patch, runner, wrapper])
actual = sorted(subprocess.check_output(
    ["git", "diff", "--name-only", f"{source}..HEAD"], text=True
).splitlines())
if actual != expected:
    raise SystemExit({"expected": expected, "actual": actual})
PY
python3 -m py_compile "$PATCH_SCRIPT"
cp "$PATCH_SCRIPT" "$RUNNER_TEMP/materialize-p1-1b-bounded-read.py"

git checkout -B p1-1b-bounded-read-work "$SOURCE_SHA"
python3 "$RUNNER_TEMP/materialize-p1-1b-bounded-read.py"

export CARGO_NET_OFFLINE=true
export CARGO_TARGET_DIR="$RUNNER_TEMP/hepta-p1-1b-bounded-read-target"
rm -f "$CRATE_ROOT/Cargo.lock"
rm -rf "$CRATE_ROOT/target"

rustc --version | tee "$ARTIFACT_DIR/rustc-version.txt"
cargo --version | tee "$ARTIFACT_DIR/cargo-version.txt"
rustfmt --version | tee "$ARTIFACT_DIR/rustfmt-version.txt"
cargo clippy --version | tee "$ARTIFACT_DIR/clippy-version.txt"
test "$(rustc --version | awk '{print $2}')" = "1.95.0"

cargo fmt --manifest-path "$MANIFEST_PATH" --all
cargo fmt --manifest-path "$MANIFEST_PATH" --all -- --check

python3 - <<'PY'
import json
import subprocess
from pathlib import Path
expected = [
    ".github/workflows/hepta-intelligence-p1-1b-bounded-read.yml",
    "codex-rs/hepta-memory-p1-1b-qualification/src/index/build.rs",
    "codex-rs/hepta-memory-p1-1b-qualification/src/index/tests_module.rs",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_2026-08-28.md",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_RECEIPT_2026-08-28.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_STATUS_2026-08-28.json",
    "scripts/verify-hepta-intelligence-p1-1b-bounded-read.py",
]
tracked = subprocess.check_output(["git", "diff", "--name-only"], text=True).splitlines()
untracked = subprocess.check_output(
    ["git", "ls-files", "--others", "--exclude-standard"], text=True
).splitlines()
actual = sorted(set(tracked + [path for path in untracked if not path.startswith("artifacts/")]))
if actual != expected:
    raise SystemExit(json.dumps({"expected": expected, "actual": actual}, indent=2))
Path("artifacts/hepta-intelligence-p1-1b-bounded-read/changed-paths.txt").write_text(
    "\n".join(actual) + "\n", encoding="utf-8"
)
PY

python3 "$BASE_VERIFIER" 2>&1 | tee "$ARTIFACT_DIR/base-source-gate.log"
python3 "$HARDENING_VERIFIER" 2>&1 | tee "$ARTIFACT_DIR/hardening-source-gate.log"
python3 "$BOUNDED_READ_VERIFIER" 2>&1 | tee "$ARTIFACT_DIR/bounded-read-source-gate.log"

cargo fmt --manifest-path "$MANIFEST_PATH" --all -- --check \
  2>&1 | tee "$ARTIFACT_DIR/rustfmt.log"
cargo test --manifest-path "$MANIFEST_PATH" --all-targets -- --nocapture \
  2>&1 | tee "$ARTIFACT_DIR/tests.log"
python3 - <<'PY'
import re
from pathlib import Path
text = Path("artifacts/hepta-intelligence-p1-1b-bounded-read/tests.log").read_text(
    encoding="utf-8", errors="replace"
)
passed = sum(int(value) for value in re.findall(r"test result: ok\. (\d+) passed", text))
if passed != 25:
    raise SystemExit(f"expected 25 P1.1b tests, observed {passed}")
PY
cargo check --manifest-path "$MANIFEST_PATH" --all-targets \
  2>&1 | tee "$ARTIFACT_DIR/check.log"
cargo clippy --manifest-path "$MANIFEST_PATH" --all-targets -- -D warnings \
  2>&1 | tee "$ARTIFACT_DIR/clippy.log"

rm -f "$CRATE_ROOT/Cargo.lock"
rm -rf "$CRATE_ROOT/target"
git diff --check
test -z "$(git ls-files --others --exclude-standard -- "$CRATE_ROOT")"

python3 - <<'PY'
import json
import subprocess
expected = [
    ".github/workflows/hepta-intelligence-p1-1b-bounded-read.yml",
    "codex-rs/hepta-memory-p1-1b-qualification/src/index/build.rs",
    "codex-rs/hepta-memory-p1-1b-qualification/src/index/tests_module.rs",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_2026-08-28.md",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_RECEIPT_2026-08-28.json",
    "plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_STATUS_2026-08-28.json",
    "scripts/verify-hepta-intelligence-p1-1b-bounded-read.py",
]
tracked = subprocess.check_output(["git", "diff", "--name-only"], text=True).splitlines()
untracked = subprocess.check_output(
    ["git", "ls-files", "--others", "--exclude-standard"], text=True
).splitlines()
actual = sorted(set(tracked + [path for path in untracked if not path.startswith("artifacts/")]))
if actual != expected:
    raise SystemExit(json.dumps({"expected": expected, "actual": actual}, indent=2))
PY

git config user.name "Hepta Qualification Bot"
git config user.email "102159240+ProfAlexQI@users.noreply.github.com"
git add -- \
  .github/workflows/hepta-intelligence-p1-1b-bounded-read.yml \
  "$CRATE_ROOT/src/index/build.rs" \
  "$CRATE_ROOT/src/index/tests_module.rs" \
  plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_2026-08-28.md \
  plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_RECEIPT_2026-08-28.json \
  plans/hepta-intelligence/HEPTA_INTELLIGENCE_P1_1B_BOUNDED_READ_STATUS_2026-08-28.json \
  "$BOUNDED_READ_VERIFIER"
git commit -m "fix(intelligence): bound immutable ANN index reads"
CANDIDATE_SHA="$(git rev-parse HEAD)"
test "$(git rev-parse "$CANDIDATE_SHA^")" = "$SOURCE_SHA"
git push origin "$CANDIDATE_SHA:refs/heads/$OUTPUT_BRANCH"

export CANDIDATE_SHA
python3 - <<'PY'
import json
import os
from pathlib import Path
receipt = {
    "schema": "hepta.intelligence.p1_1b.bounded_read_executable_qualification.v1",
    "status": "PASS_P1_1B_BOUNDED_READ_EXECUTABLE_GATES",
    "parent_commit": "98de1c4c3d11c6644ff46f50e80071f6f15e1652",
    "candidate_commit": os.environ["CANDIDATE_SHA"],
    "tests_passed": 25,
    "gates": {
        "base_source": "success",
        "hardening_source": "success",
        "bounded_read_source": "success",
        "rust_1_95_0": "success",
        "rustfmt": "success",
        "tests": "success",
        "check": "success",
        "clippy": "success",
        "clean_scope": "success",
    },
    "source_qualified": True,
    "qualified": False,
    "runtime_wired": False,
    "default_recall_changed": False,
    "context_attachment": False,
    "physical_send": False,
    "external_effects": False,
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
Path("artifacts/hepta-intelligence-p1-1b-bounded-read/qualification-receipt.json").write_text(
    json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
)
PY
