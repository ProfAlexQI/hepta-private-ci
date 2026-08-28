#!/usr/bin/env bash
set -euo pipefail

python3 "$RUNNER_TEMP/repair.py"
(
  cd codex-rs
  cargo fmt \
    -p codex-lmstudio \
    -p codex-ollama \
    -p codex-utils-oss \
    -p codex-responses-api-proxy
)

printf '%s\n' \
  codex-rs/lmstudio/src/lib.rs \
  codex-rs/lmstudio/src/sha256.rs \
  codex-rs/ollama/src/client_support.inc.rs \
  codex-rs/ollama/src/lib.rs \
  codex-rs/responses-api-proxy/src/dump.rs \
  codex-rs/responses-api-proxy/src/sha256.rs \
  codex-rs/utils/oss/src/lib.rs \
  | sort > "$RUNNER_TEMP/expected.txt"
git diff --name-only | sort > "$RUNNER_TEMP/changed.txt"
diff -u "$RUNNER_TEMP/expected.txt" "$RUNNER_TEMP/changed.txt"
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
  cargo clippy --locked --all-targets \
    -p codex-lmstudio \
    -p codex-ollama \
    -p codex-utils-oss \
    -p codex-responses-api-proxy \
    -- -D warnings
)

git config user.name "github-actions[bot]"
git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
git add \
  codex-rs/lmstudio/src/lib.rs \
  codex-rs/lmstudio/src/sha256.rs \
  codex-rs/ollama/src/client_support.inc.rs \
  codex-rs/ollama/src/lib.rs \
  codex-rs/responses-api-proxy/src/dump.rs \
  codex-rs/responses-api-proxy/src/sha256.rs \
  codex-rs/utils/oss/src/lib.rs
test "$(git diff --cached --name-only | wc -l | tr -d ' ')" = 7
git commit -m "fix(inference): repair package compile and formatting"
SOURCE_SHA="$(git rev-parse HEAD)"
SOURCE_TREE="$(git rev-parse 'HEAD^{tree}')"
export SOURCE_SHA SOURCE_TREE

python3 "$RUNNER_TEMP/rebind.py"
python3 -m json.tool "$RECEIPT" >/dev/null
test "$(git diff --name-only)" = "$RECEIPT"
git add "$RECEIPT"
test "$(git diff --cached --name-only)" = "$RECEIPT"
git commit -m "docs(inference): rebind repaired qualification receipt"
RECEIPT_SHA="$(git rev-parse HEAD)"
RECEIPT_TREE="$(git rev-parse 'HEAD^{tree}')"

python3 -m py_compile \
  scripts/hepta-inference-inf0-source-gate.py \
  scripts/hepta-inference-inf0c-real-e2e.py
python3 scripts/hepta-inference-inf0-source-gate.py
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
test "$(git rev-parse FETCH_HEAD)" = "$EXPECTED_HEAD"
git push origin HEAD:"$BRANCH"

{
  echo "source_commit=$SOURCE_SHA"
  echo "source_tree=$SOURCE_TREE"
  echo "receipt_commit=$RECEIPT_SHA"
  echo "receipt_tree=$RECEIPT_TREE"
  echo "qualified=false"
  echo "INF-1=NOT_STARTED"
  echo "production/effect/model-NPU/remote/promotion authority=false"
} >> "$GITHUB_STEP_SUMMARY"
