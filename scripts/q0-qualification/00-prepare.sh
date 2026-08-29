# shellcheck shell=bash
set -euo pipefail

: "${EXPECTED_BRANCH:?EXPECTED_BRANCH is required}"
: "${EXPECTED_PARENT:?EXPECTED_PARENT is required}"
: "${ARTIFACT_DIR:?ARTIFACT_DIR is required}"
: "${Q0_EXPECTED_UNAME:?Q0_EXPECTED_UNAME is required}"
: "${Q0_EXPECTED_RUST_HOST:?Q0_EXPECTED_RUST_HOST is required}"
: "${Q0_EVIDENCE_CLASS:?Q0_EVIDENCE_CLASS is required}"
: "${Q0_WORKFLOW_PATH:?Q0_WORKFLOW_PATH is required}"
: "${Q0_WORKFLOW_SHA:?Q0_WORKFLOW_SHA is required}"
: "${Q0_REPOSITORY_ID:?Q0_REPOSITORY_ID is required}"
: "${Q0_REPOSITORY_OWNER_ID:?Q0_REPOSITORY_OWNER_ID is required}"

case "$Q0_EVIDENCE_CLASS" in
  E1_LOCAL_EXECUTABLE)
    q0_pass_status=PASS_Q0_E1_LOCAL_EXECUTABLE
    q0_fail_status=FAIL_Q0_E1_LOCAL_EXECUTABLE
    ;;
  E2_INDEPENDENT_RUNNER)
    q0_pass_status=PASS_Q0_E2_INDEPENDENT_RUNNER
    q0_fail_status=FAIL_Q0_E2_INDEPENDENT_RUNNER
    ;;
  *)
    printf 'unsupported Q0 evidence class: %s\n' "$Q0_EVIDENCE_CLASS" >&2
    exit 2
    ;;
esac
export q0_pass_status q0_fail_status

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"
mkdir -p "$ARTIFACT_DIR/gates" "$ARTIFACT_DIR/rust"

test "$GITHUB_REF_NAME" = "$EXPECTED_BRANCH"
test "$(git rev-parse HEAD)" = "$GITHUB_SHA"
test "$(git rev-parse HEAD^)" = "$EXPECTED_PARENT"
test "$Q0_WORKFLOW_SHA" = "$GITHUB_SHA"
test "$GITHUB_REPOSITORY" = "ProfHepta/hepta-private-ci"
test "$Q0_REPOSITORY_ID" = "1320694176"
test "$Q0_REPOSITORY_OWNER_ID" = "102159240"
test "$(uname -m)" = "$Q0_EXPECTED_UNAME"
test -f .github/patches/hepta-intelligence-q0-compile-fix-v1.patch
test -f .github/scripts/hepta-intelligence-q0-manual-clippy-repair-v7.py
test -f .github/scripts/hepta-intelligence-q0-reconstruct-v7.py
test -f .github/scripts/hepta-intelligence-q0-supplemental-repair-v1.py
test -f scripts/hepta-intelligence-status-compat.py
test -f scripts/verify-hepta-intelligence-q0-evidence-pair.py
test -f scripts/verify-hepta-intelligence-repository-identity.py
test -f scripts/verify-hepta-intelligence-q0-workflow-consolidation.py
test -f plans/hepta-intelligence/HEPTA_INTELLIGENCE_REPOSITORY_IDENTITY_V1.json

python3 - "$ARTIFACT_DIR/identity.json" <<'PY'
import json
import os
import pathlib
import subprocess
import sys

path = pathlib.Path(sys.argv[1])
payload = {
    "schema": "hepta.intelligence.q0.execution_identity.v2",
    "repository": {
        "full_name": os.environ["GITHUB_REPOSITORY"],
        "repository_id": int(os.environ["Q0_REPOSITORY_ID"]),
        "owner_id": int(os.environ["Q0_REPOSITORY_OWNER_ID"]),
    },
    "candidate": {
        "head": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(),
        "tree": subprocess.check_output(["git", "rev-parse", "HEAD^{tree}"], text=True).strip(),
        "parent": subprocess.check_output(["git", "rev-parse", "HEAD^"], text=True).strip(),
    },
    "workflow": {
        "path": os.environ["Q0_WORKFLOW_PATH"],
        "sha": os.environ["Q0_WORKFLOW_SHA"],
        "run_id": int(os.environ["GITHUB_RUN_ID"]),
        "run_attempt": int(os.environ["GITHUB_RUN_ATTEMPT"]),
        "job": os.environ["GITHUB_JOB"],
    },
    "host": os.environ["Q0_EXPECTED_RUST_HOST"],
    "evidence_class": os.environ["Q0_EVIDENCE_CLASS"],
    "runtime_wired": False,
    "production_authority": False,
    "source_writeback": False,
}
path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY

expected_toolchain=$(python3 - <<'PY'
import pathlib
import tomllib
print(tomllib.loads(pathlib.Path('codex-rs/rust-toolchain.toml').read_text())['toolchain']['channel'])
PY
)
test "$expected_toolchain" = "1.95.0"
test "$(rustc --version | awk '{print $2}')" = "$expected_toolchain"
test "$(rustc -vV | awk '/^host:/ {print $2}')" = "$Q0_EXPECTED_RUST_HOST"
test -s codex-rs/Cargo.lock

# The historical v7 repair generator intentionally stores one regex fragment
# as two lines in source control.  Repair that generator only in this clean
# worktree, execute it, then restore the generator before evidence capture.
python3 - <<'PY'
from pathlib import Path

path = Path('.github/scripts/hepta-intelligence-q0-manual-clippy-repair-v7.py')
lines = path.read_text(encoding='utf-8').splitlines()
needle = r"        rf'(?:async[ \t]+)?fn[ \t]+{re.escape(name)}[ \t]*\('"
assert lines.count(needle) == 1, lines.count(needle)
index = lines.index(needle)
assert lines[index + 1] == '    )', lines[index + 1]
lines.insert(index + 1, "        r')'")
path.write_text('\n'.join(lines) + '\n', encoding='utf-8')
PY
python3 -m py_compile \
  .github/scripts/hepta-intelligence-q0-manual-clippy-repair-v7.py \
  .github/scripts/hepta-intelligence-q0-reconstruct-v7.py \
  .github/scripts/hepta-intelligence-q0-supplemental-repair-v1.py
python3 .github/scripts/hepta-intelligence-q0-reconstruct-v7.py
python3 .github/scripts/hepta-intelligence-q0-supplemental-repair-v1.py
git restore --source=HEAD -- .github/scripts/hepta-intelligence-q0-manual-clippy-repair-v7.py

mapfile -t repaired_files < <(git diff --name-only -- '*.rs')
test "${#repaired_files[@]}" -gt 0
rustfmt --edition 2024 --config skip_children=true "${repaired_files[@]}"
git diff --check

run_clippy_fix_diagnostic() {
  local name=$1
  shift
  set +e
  (
    cd codex-rs
    "$@"
  ) 2>&1 | tee "$ARTIFACT_DIR/rust/$name.log"
  local code=${PIPESTATUS[0]}
  set -e
  printf '%s\n' "$code" > "$ARTIFACT_DIR/rust/$name.diagnostic.exit"
}

# Machine-applicable edits are allowed only inside the bounded overlay.  The
# final strict Clippy lanes remain blocking and cannot be bypassed by these
# preparation diagnostics.
run_clippy_fix_diagnostic memory-clippy-fix \
  cargo clippy --fix --locked -p codex-hepta-memory --all-targets \
    --allow-dirty --allow-staged -- -D warnings
run_clippy_fix_diagnostic extension-clippy-fix \
  cargo clippy --fix --locked -p codex-hepta-memory-extension --all-targets \
    --allow-dirty --allow-staged -- -D warnings
run_clippy_fix_diagnostic agentd-default-clippy-fix \
  cargo clippy --fix --locked -p codex-hepta-agentd --all-targets \
    --allow-dirty --allow-staged -- -D warnings
run_clippy_fix_diagnostic agentd-shadow-clippy-fix \
  cargo clippy --fix --locked -p codex-hepta-agentd \
    --features qualification-intelligence-mutation-shadow --all-targets \
    --allow-dirty --allow-staged -- -D warnings

mapfile -t repaired_files < <(git diff --name-only -- '*.rs')
test "${#repaired_files[@]}" -gt 0
rustfmt --edition 2024 --config skip_children=true "${repaired_files[@]}"
rustfmt --edition 2024 --check --config skip_children=true "${repaired_files[@]}"
git diff --check

# Capture the complete executable source overlay before enforcing its bounded
# path and anti-bypass rules.  A rejected candidate still yields reviewable
# evidence, but it cannot reach a passing receipt.
git diff --name-only | LC_ALL=C sort > "$ARTIFACT_DIR/changed-files.txt"
git diff --stat > "$ARTIFACT_DIR/repair-stat.txt"
git diff --binary > "$ARTIFACT_DIR/repair.patch"
while IFS= read -r path; do
  case "$path" in
    codex-rs/hepta-memory/*.rs|\
    codex-rs/state/src/sqlite.rs|\
    codex-rs/ext/hepta-memory/*.rs|\
    codex-rs/hepta-agentd/*.rs|\
    codex-rs/hepta-agent-protocol/*.rs|\
    codex-rs/app-server-protocol/*.rs) ;;
    *) printf 'unexpected executable overlay path: %s\n' "$path" >&2; exit 1 ;;
  esac
done < "$ARTIFACT_DIR/changed-files.txt"
! git diff --unified=0 -- '*.rs' \
  | grep -E '^\+.*(#\!?\[allow\(clippy|#\[ignore\]|todo!\(|unimplemented!\()'

python3 - "$ARTIFACT_DIR" <<'PY'
from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys

root = Path(sys.argv[1])
changed_raw = (root / "changed-files.txt").read_bytes()
paths = [line for line in changed_raw.decode().splitlines() if line]
assert paths == sorted(set(paths)) and paths

def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()

files = []
for name in paths:
    path = Path(name)
    assert path.is_file(), name
    files.append({"path": name, "sha256": digest(path), "size": path.stat().st_size})
base = {
    "changed_files_sha256": hashlib.sha256(changed_raw).hexdigest(),
    "repair_patch_sha256": digest(root / "repair.patch"),
    "repair_patch_size": (root / "repair.patch").stat().st_size,
    "repair_stat_sha256": digest(root / "repair-stat.txt"),
    "files": files,
}
encoded = json.dumps(base, sort_keys=True, separators=(",", ":")).encode()
manifest = {**base, "manifest_sha256": hashlib.sha256(encoded).hexdigest()}
(root / "source-overlay-manifest.json").write_text(
    json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
)
PY

# Full-workspace formatting includes unrelated stacked work.  Preserve it as a
# transparent diagnostic; package-owned rustfmt and strict Clippy remain
# blocking inside the exact Q0 matrix.
set +e
(cd codex-rs && cargo fmt --all -- --check) >"$ARTIFACT_DIR/rust/workspace-fmt.log" 2>&1
workspace_fmt_code=$?
set -e
printf '%s\n' "$workspace_fmt_code" > "$ARTIFACT_DIR/rust/workspace-fmt.diagnostic.exit"
