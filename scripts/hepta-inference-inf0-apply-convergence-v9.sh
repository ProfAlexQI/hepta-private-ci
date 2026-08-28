#!/usr/bin/env bash
set -euo pipefail

CANONICAL_WORKFLOW=".github/workflows/hepta-inference-inf0.yml"

python3 "$RUNNER_TEMP/converge.py"
(
  cd codex-rs
  cargo fmt \
    -p codex-lmstudio \
    -p codex-ollama \
    -p codex-utils-oss \
    -p codex-responses-api-proxy
)

printf '%s\n' \
  .github/workflows/hepta-inference-inf0.yml \
  codex-rs/lmstudio/src/lib.rs \
  codex-rs/lmstudio/src/sha256.rs \
  codex-rs/ollama/src/client_support.inc.rs \
  codex-rs/ollama/src/lib.rs \
  codex-rs/responses-api-proxy/src/dump.rs \
  codex-rs/responses-api-proxy/src/sha256.rs \
  codex-rs/utils/oss/src/lib.rs \
  | sort > "$RUNNER_TEMP/full-expected.txt"
printf '%s\n' .github/workflows/hepta-inference-inf0.yml \
  | sort > "$RUNNER_TEMP/post-v8-expected.txt"
git diff --name-only | sort > "$RUNNER_TEMP/changed.txt"
if ! diff -u "$RUNNER_TEMP/full-expected.txt" "$RUNNER_TEMP/changed.txt" \
  && ! diff -u "$RUNNER_TEMP/post-v8-expected.txt" "$RUNNER_TEMP/changed.txt"; then
  echo "unexpected convergence delta" >&2
  exit 1
fi
grep -Fxq "$CANONICAL_WORKFLOW" "$RUNNER_TEMP/changed.txt"
git diff --check

(
  cd codex-rs
  cargo fmt \
    -p codex-lmstudio \
    -p codex-ollama \
    -p codex-utils-oss \
    -p codex-responses-api-proxy \
    -- --check
  rustfmt --edition 2024 --check \
    ollama/src/client.rs \
    ollama/src/client_*.inc.rs \
    lmstudio/src/client.rs \
    lmstudio/src/client_*.inc.rs
  cargo check --locked \
    -p codex-lmstudio \
    -p codex-ollama \
    -p codex-utils-oss \
    -p codex-responses-api-proxy
  cargo test --locked \
    -p codex-lmstudio \
    -p codex-ollama \
    -p codex-utils-oss \
    -p codex-responses-api-proxy
  cargo clippy --locked --all-targets --no-deps \
    -p codex-lmstudio \
    -p codex-ollama \
    -p codex-utils-oss \
    -p codex-responses-api-proxy \
    -- -D warnings
)

python3 - "$CANONICAL_WORKFLOW" <<'PY'
from pathlib import Path
import sys

text = Path(sys.argv[1]).read_text(encoding="utf-8")
required = (
    "Check owned inference package formatting",
    "cargo clippy --locked --all-targets --no-deps",
    "-p codex-lmstudio",
    "-p codex-ollama",
    "-p codex-utils-oss",
    "-p codex-responses-api-proxy",
)
for marker in required:
    if marker not in text:
        raise SystemExit(f"canonical workflow missing {marker}")
if "cargo fmt --all -- --check" in text:
    raise SystemExit("canonical workflow still owns unrelated workspace formatting")
PY

git config user.name "github-actions[bot]"
git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
while IFS= read -r path; do
  test -n "$path"
  git add "$path"
done < "$RUNNER_TEMP/changed.txt"
test -z "$(git diff --cached --name-only | grep -Fx "$RECEIPT" || true)"
git commit -m "fix(inference): converge package gates and compile bindings"
SOURCE_SHA="$(git rev-parse HEAD)"
SOURCE_TREE="$(git rev-parse 'HEAD^{tree}')"
RUSTC_VERSION="$(rustc --version)"
RUSTFMT_VERSION="$(rustfmt --version)"
CARGO_VERSION="$(cargo --version)"
export SOURCE_SHA SOURCE_TREE RUSTC_VERSION RUSTFMT_VERSION CARGO_VERSION

python3 "$RUNNER_TEMP/rebind.py"
python3 -m json.tool "$RECEIPT" >/dev/null
test "$(git diff --name-only)" = "$RECEIPT"
git add "$RECEIPT"
test "$(git diff --cached --name-only)" = "$RECEIPT"
git commit -m "docs(inference): rebind converged qualification receipt"
RECEIPT_SHA="$(git rev-parse HEAD)"
RECEIPT_TREE="$(git rev-parse 'HEAD^{tree}')"

python3 -m py_compile \
  scripts/hepta-inference-inf0-source-gate.py \
  scripts/hepta-inference-inf0c-real-e2e.py
python3 scripts/hepta-inference-inf0-source-gate.py \
  | tee "$RUNNER_TEMP/source-gate.txt"
(
  cd codex-rs
  cargo fmt \
    -p codex-lmstudio \
    -p codex-ollama \
    -p codex-utils-oss \
    -p codex-responses-api-proxy \
    -- --check
  rustfmt --edition 2024 --check \
    ollama/src/client.rs \
    ollama/src/client_*.inc.rs \
    lmstudio/src/client.rs \
    lmstudio/src/client_*.inc.rs
)
test -z "$(git status --porcelain)"

git fetch --no-tags origin "$BRANCH"
test "$(git rev-parse FETCH_HEAD)" = "$INPUT_HEAD"
git push origin HEAD:"$BRANCH"

{
  echo "source_commit=$SOURCE_SHA"
  echo "source_tree=$SOURCE_TREE"
  echo "receipt_commit=$RECEIPT_SHA"
  echo "receipt_tree=$RECEIPT_TREE"
  echo "canonical_format_scope=owned-packages-and-fragments"
  echo "canonical_clippy_scope=owned-packages-all-targets-no-deps-D-warnings"
  echo "qualified=false"
  echo "INF-1=NOT_STARTED"
  echo "production/effect/model-NPU/remote/promotion authority=false"
} >> "$GITHUB_STEP_SUMMARY"
