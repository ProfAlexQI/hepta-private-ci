# shellcheck shell=bash
set -euo pipefail

: "${EXPECTED_BRANCH:?EXPECTED_BRANCH is required}"
: "${EXPECTED_PARENT:?EXPECTED_PARENT is required}"
: "${ARTIFACT_DIR:?ARTIFACT_DIR is required}"
: "${Q0_EXPECTED_UNAME:?Q0_EXPECTED_UNAME is required}"
: "${Q0_EXPECTED_RUST_HOST:?Q0_EXPECTED_RUST_HOST is required}"
: "${Q0_EVIDENCE_CLASS:?Q0_EVIDENCE_CLASS is required}"

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

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"
mkdir -p "$ARTIFACT_DIR/gates" "$ARTIFACT_DIR/rust"

test "$GITHUB_REF_NAME" = "$EXPECTED_BRANCH"
test "$(git rev-parse HEAD)" = "$GITHUB_SHA"
test "$(git rev-parse HEAD^)" = "$EXPECTED_PARENT"
test "$(uname -m)" = "$Q0_EXPECTED_UNAME"
test -f .github/patches/hepta-intelligence-q0-compile-fix-v1.patch
test -f .github/scripts/hepta-intelligence-q0-manual-clippy-repair-v7.py
test -f .github/scripts/hepta-intelligence-q0-reconstruct-v7.py
test -f scripts/hepta-intelligence-status-compat.py
test -f scripts/verify-hepta-intelligence-q0-evidence-pair.py
test ! -e .github/workflows/hepta-intelligence-q0-compile-fix-v2.yml

python3 - "$ARTIFACT_DIR/identity.json" <<'PY'
import json
import os
import pathlib
import subprocess
import sys

path = pathlib.Path(sys.argv[1])
payload = {
    "head": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(),
    "tree": subprocess.check_output(["git", "rev-parse", "HEAD^{tree}"], text=True).strip(),
    "parent": subprocess.check_output(["git", "rev-parse", "HEAD^"], text=True).strip(),
    "host": os.environ["Q0_EXPECTED_RUST_HOST"],
    "evidence_class": os.environ["Q0_EVIDENCE_CLASS"],
    "runtime_authority": False,
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
  .github/scripts/hepta-intelligence-q0-reconstruct-v7.py
python3 .github/scripts/hepta-intelligence-q0-reconstruct-v7.py
git restore --source=HEAD -- .github/scripts/hepta-intelligence-q0-manual-clippy-repair-v7.py
mapfile -t repaired_files < <(git diff --name-only -- '*.rs')
test "${#repaired_files[@]}" -gt 0
rustfmt --edition 2024 --config skip_children=true "${repaired_files[@]}"
git diff --check

set +e
(
  cd codex-rs
  cargo clippy --fix --locked -p codex-hepta-memory --all-targets \
    --allow-dirty --allow-staged -- -D warnings
) 2>&1 | tee "$ARTIFACT_DIR/rust/memory-clippy-fix.log"
clippy_fix_code=${PIPESTATUS[0]}
set -e
printf '%s\n' "$clippy_fix_code" > "$ARTIFACT_DIR/rust/memory-clippy-fix.exit"

mapfile -t repaired_files < <(git diff --name-only -- '*.rs')
test "${#repaired_files[@]}" -gt 0
rustfmt --edition 2024 --config skip_children=true "${repaired_files[@]}"
rustfmt --edition 2024 --check --config skip_children=true "${repaired_files[@]}"
git diff --check
while IFS= read -r path; do
  case "$path" in
    codex-rs/hepta-memory/src/*.rs|codex-rs/hepta-memory/src/**/*.rs) ;;
    *) printf 'unexpected candidate path: %s\n' "$path" >&2; exit 1 ;;
  esac
done < <(git diff --name-only)
! git diff --unified=0 -- '*.rs' | grep -E '^\+.*(#\!?\[allow\(clippy|#\[ignore\]|todo!\(|unimplemented!\()'
set +e
(cd codex-rs && cargo fmt --all -- --check) >"$ARTIFACT_DIR/rust/workspace-fmt.log" 2>&1
workspace_fmt_code=$?
set -e
printf '%s\n' "$workspace_fmt_code" > "$ARTIFACT_DIR/rust/workspace-fmt.exit"
