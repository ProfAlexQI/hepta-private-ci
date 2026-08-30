#!/usr/bin/env bash
set -euo pipefail

cd "${GITHUB_WORKSPACE:?}"

EXPECTED_PARENT=925509b95758f1330f7425af47d57536e0e475dc
BASE_BRANCH=integration/vnext-main-full-ci-authbus-p1-3-20260829
BASE_SHA=6b7aa91d7702a92a50297b1b1bd8170ffb7cb184
BASE_TREE=f46fa8c6d541742eaea62d70ea62c0fe316dbaf9
TARGET_BRANCH=codex/authbus-p1-3-clean-replay-20260830
REGISTRY_SHA256=dfcab028e1a135a0895b3f9eddec9f5f99cf5f392701b98ad14180058a284bf1
P02=codex-rs/hepta-authbus-qualification/Cargo.toml
P03=codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml
P11=codex-rs/hepta-authbus-p1-qualification/Cargo.toml
P12=codex-rs/hepta-authbus-p1-2-qualification/Cargo.toml
P13=codex-rs/hepta-authbus-p1-3-qualification/Cargo.toml

log() { printf '\n==> %s\n' "$*"; }
require_line() { grep -Fq "$1" "$2"; }

log "bind exact seed and canonical base"
test "${GITHUB_REF_NAME}" = "codex/authbus-p1-3-gap-closure-20260829"
test "$(git rev-parse HEAD)" = "${GITHUB_SHA}"
test "$(git rev-parse HEAD^)" = "${EXPECTED_PARENT}"
test "$(rustc -V | awk '{print $2}')" = "1.95.0"
for script in \
  scripts/authbus-p1-3-finalize.py \
  scripts/authbus-p1-3-semantic-completion.py \
  scripts/authbus-p1-3-gap-closure.py \
  scripts/authbus-p1-3-clippy-completion.py
do
  python3 -m py_compile "${script}"
done
git fetch --no-tags --depth=1 origin \
  "refs/heads/${BASE_BRANCH}:refs/remotes/origin/${BASE_BRANCH}"
test "$(git rev-parse "refs/remotes/origin/${BASE_BRANCH}")" = "${BASE_SHA}"
test "$(git rev-parse "${BASE_SHA}^{tree}")" = "${BASE_TREE}"
! git ls-remote --exit-code --heads origin "refs/heads/${TARGET_BRANCH}" >/dev/null 2>&1

log "materialize canonical registry and semantic closure"
python3 scripts/authbus-p1-3-finalize.py | tee /tmp/p13-finalize.txt
require_line 'applied_authbus_p1_3_registry_files=9' /tmp/p13-finalize.txt
require_line 'applied_authbus_p1_3_existing_file_patches=5' /tmp/p13-finalize.txt
require_line 'removed_p0_3_canonical_type_copies=2' /tmp/p13-finalize.txt
scheduler=codex-rs/hepta-authbus-p0-3-qualification/src/scheduler.rs
test "$(grep -F -c 'request_sha256.to_string()' "${scheduler}")" = "1"
sed -i 's/request_sha256\.to_string()/request_sha256.as_str().to_owned()/' "${scheduler}"
! grep -Fq 'request_sha256.to_string()' "${scheduler}"
python3 scripts/authbus-p1-3-semantic-completion.py | tee /tmp/p13-semantic.txt
require_line 'applied_authbus_p1_3_semantic_completion=1' /tmp/p13-semantic.txt
require_line "authbus_p1_3_registry_sha256=${REGISTRY_SHA256}" /tmp/p13-semantic.txt
require_line 'closed_b2_four_dimension_wire_gap=1' /tmp/p13-semantic.txt
require_line 'closed_absent_unknown_rounding_binding_gap=1' /tmp/p13-semantic.txt
require_line 'closed_source_registry_projection_binding_gap=1' /tmp/p13-semantic.txt
python3 scripts/authbus-p1-3-gap-closure.py | tee /tmp/p13-gap.txt
require_line 'closed_authbus_p1_3_materialized_compile_gap=1' /tmp/p13-gap.txt
python3 scripts/authbus-p1-3-clippy-completion.py | tee /tmp/p13-clippy-fix.txt
require_line 'applied_authbus_p1_3_strict_clippy_repairs=11' /tmp/p13-clippy-fix.txt
require_line 'applied_authbus_p0_3_inline_constant_assertion_repairs=1' /tmp/p13-clippy-fix.txt
require_line 'applied_authbus_p0_3_external_constant_assertion_repairs=1' /tmp/p13-clippy-fix.txt
require_line 'applied_authbus_p0_3_constant_assertion_repairs=2' /tmp/p13-clippy-fix.txt
require_line 'closed_authbus_p0_3_constant_assertions_clippy_gap=1' /tmp/p13-clippy-fix.txt
require_line 'closed_authbus_p1_3_strict_clippy_gap=1' /tmp/p13-clippy-fix.txt
python3 scripts/verify-authbus-p1-3.py | tee /tmp/p13-source.json
require_line 'PASS_AUTHBUS_P1_3_SOURCE_ONLY' /tmp/p13-source.json
git diff --check

log "generate lockfile and format affected crates"
cargo generate-lockfile --manifest-path "${P13}"
cargo fmt --manifest-path codex-rs/Cargo.toml --package codex-hepta-contracts
for manifest in "${P02}" "${P03}" "${P11}" "${P12}" "${P13}"; do
  cargo fmt --manifest-path "${manifest}" --all
done
cargo fmt --manifest-path codex-rs/Cargo.toml --package codex-hepta-contracts -- --check
for manifest in "${P02}" "${P03}" "${P11}" "${P12}" "${P13}"; do
  cargo fmt --manifest-path "${manifest}" --all -- --check
done
test -s codex-rs/hepta-authbus-p1-3-qualification/Cargo.lock
cargo metadata --manifest-path "${P13}" --locked --format-version 1 --no-deps >/dev/null
python3 scripts/verify-authbus-p1-3.py | tee /tmp/p13-formatted.json
require_line 'PASS_AUTHBUS_P1_3_SOURCE_ONLY' /tmp/p13-formatted.json
git diff --check

log "run P0.2 through P1.3 executable matrices"
cargo test --manifest-path codex-rs/Cargo.toml --locked \
  -p codex-hepta-contracts --features authbus-local-qualification --lib -- --nocapture
cargo test --manifest-path "${P02}" --locked --no-default-features --lib -- --nocapture
cargo test --manifest-path "${P02}" --locked --features sqlite-qualification --tests -- --nocapture
cargo test --manifest-path "${P03}" --locked --no-default-features --lib -- --nocapture
cargo test --manifest-path "${P03}" --locked --features p0-3-qualification --tests -- --nocapture
cargo test --manifest-path "${P11}" --locked --no-default-features --lib -- --nocapture
cargo test --manifest-path "${P11}" --locked --features p1-qualification --tests -- --nocapture
cargo test --manifest-path "${P12}" --locked --no-default-features --lib -- --nocapture
cargo test --manifest-path "${P12}" --locked --features p1-2-qualification --tests -- --nocapture
cargo test --manifest-path "${P13}" --locked --no-default-features --lib -- --nocapture
cargo test --manifest-path "${P13}" --locked --features p1-3-qualification --tests -- --nocapture

log "run every all-target check and strict Clippy gate"
export P02 P03 P11 P12 P13
failures=()
run_gate() {
  local name=$1 command=$2
  printf '\n--- gate: %s ---\n' "${name}"
  if bash -euo pipefail -c "${command}"; then
    printf 'PASS_%s\n' "${name}"
  else
    failures+=("${name}")
  fi
}
run_gate contracts 'cargo check --manifest-path codex-rs/Cargo.toml --locked -p codex-hepta-contracts --features authbus-local-qualification --all-targets && cargo clippy --manifest-path codex-rs/Cargo.toml --locked -p codex-hepta-contracts --features authbus-local-qualification --all-targets -- -D warnings'
run_gate p0_2 'cargo check --manifest-path "$P02" --locked --features sqlite-qualification --all-targets && cargo clippy --manifest-path "$P02" --locked --features sqlite-qualification --all-targets -- -D warnings'
run_gate p0_3 'cargo check --manifest-path "$P03" --locked --features p0-3-qualification --all-targets && cargo clippy --manifest-path "$P03" --locked --features p0-3-qualification --all-targets -- -D warnings'
run_gate p1_1 'cargo check --manifest-path "$P11" --locked --features p1-qualification --all-targets && cargo clippy --manifest-path "$P11" --locked --features p1-qualification --all-targets -- -D warnings'
run_gate p1_2 'cargo check --manifest-path "$P12" --locked --features p1-2-qualification --all-targets && cargo clippy --manifest-path "$P12" --locked --features p1-2-qualification --all-targets -- -D warnings'
run_gate p1_3 'cargo check --manifest-path "$P13" --locked --features p1-3-qualification --all-targets && cargo clippy --manifest-path "$P13" --locked --features p1-3-qualification --all-targets -- -D warnings'
if ((${#failures[@]})); then
  printf 'strict gate failures: %s\n' "${failures[*]}" >&2
  exit 1
fi

log "revalidate inherited source contracts"
python3 scripts/verify-authbus-p0-2.py
python3 scripts/verify-authbus-p0-3.py
python3 scripts/verify-authbus-p1-1.py
python3 scripts/verify-authbus-p1-2.py
python3 scripts/verify-authbus-p1-3.py
git diff --check

log "prepare exact qualification workflow"
python3 - <<'PY'
from pathlib import Path
path = Path('.github/workflows/authbus-p1-3-qualification.yml')
source = path.read_text(encoding='utf-8')
for before, after, expected, label in (
    ('      - integration/vnext-main-full-ci-authbus-p1-3-20260829',
     '      - codex/authbus-p1-3-clean-replay-20260830', 1, 'target branch'),
    ('runs-on: ubuntu-slim', 'runs-on: ubuntu-24.04', 4, 'hosted runner'),
    ('f6702be58c499d853d273f3174a2556481a3f5b4284cd9cd0b0a247160d7ac08',
     'dfcab028e1a135a0895b3f9eddec9f5f99cf5f392701b98ad14180058a284bf1',
     3, 'registry digest'),
):
    count = source.count(before)
    if count != expected:
        raise SystemExit(f'{label}: expected {expected} anchors, found {count}')
    source = source.replace(before, after)
path.write_text(source, encoding='utf-8')
PY

FINAL_PATHS=(
  codex-rs/hepta-contracts/src/authbus_b1.rs
  codex-rs/hepta-contracts/src/authbus_b1_tests.rs
  codex-rs/hepta-contracts/src/authbus_b2.rs
  codex-rs/hepta-contracts/src/authbus_b3.rs
  codex-rs/hepta-contracts/src/authbus_b3_adapter.rs
  codex-rs/hepta-contracts/src/authbus_b4.rs
  codex-rs/hepta-contracts/src/authbus_tests.rs
  codex-rs/hepta-contracts/src/lib.rs
  codex-rs/hepta-contracts/src/quota_registry.rs
  codex-rs/hepta-contracts/tests/authbus_b3_p0_1.rs
  codex-rs/hepta-authbus-qualification/src/model.rs
  codex-rs/hepta-authbus-qualification/src/store.rs
  codex-rs/hepta-authbus-p0-3-qualification/src/lib.rs
  codex-rs/hepta-authbus-p0-3-qualification/src/scheduler.rs
  codex-rs/hepta-authbus-p0-3-qualification/tests/p0_3.rs
  codex-rs/hepta-authbus-p1-3-qualification/Cargo.toml
  codex-rs/hepta-authbus-p1-3-qualification/Cargo.lock
  codex-rs/hepta-authbus-p1-3-qualification/README.md
  codex-rs/hepta-authbus-p1-3-qualification/src/lib.rs
  codex-rs/hepta-authbus-p1-3-qualification/tests/p1_3.rs
  docs/hepta-vnext/authbus/AUTHBUS_P1_3_DEVELOPMENT_PLAN_2026-08-29.md
  docs/hepta-vnext/authbus/AUTHBUS_P1_3_IMPLEMENTATION_STATUS_2026-08-29.json
  docs/hepta-vnext/authbus/AUTHBUS_P1_3_IMPLEMENTATION_STATUS_2026-08-29.md
  scripts/verify-authbus-p1-3.py
)
DELETIONS=(
  .github/workflows/authbus-p1-3-gap-diagnostic.yml
  .github/workflows/authbus-p1-3-materialize.yml
  scripts/authbus-p1-3-finalize.py
  scripts/authbus-p1-3-semantic-completion.py
)

tar -cf /tmp/authbus-p1-3-final-source.tar \
  .github/workflows/authbus-p1-3-qualification.yml "${FINAL_PATHS[@]}"
git reset --hard
git clean -fdx
git checkout --detach "${BASE_SHA}"
test "$(git rev-parse HEAD)" = "${BASE_SHA}"
test "$(git rev-parse HEAD^{tree})" = "${BASE_TREE}"
tar -xf /tmp/authbus-p1-3-final-source.tar
git rm -f "${DELETIONS[@]}"
git add -- .github/workflows/authbus-p1-3-qualification.yml "${FINAL_PATHS[@]}"
printf '%s\n' "${DELETIONS[@]}" .github/workflows/authbus-p1-3-qualification.yml \
  "${FINAL_PATHS[@]}" | LC_ALL=C sort -u > /tmp/p13-expected-paths.txt
git diff --cached --name-only | LC_ALL=C sort > /tmp/p13-actual-paths.txt
test "$(wc -l < /tmp/p13-expected-paths.txt)" = "29"
test "$(wc -l < /tmp/p13-actual-paths.txt)" = "29"
diff -u /tmp/p13-expected-paths.txt /tmp/p13-actual-paths.txt
for path in \
  .github/workflows/authbus-p1-3-gap-diagnostic.yml \
  .github/workflows/authbus-p1-3-materialize.yml \
  .github/workflows/authbus-p1-3-clean-replay-materialize.yml \
  .github/workflows/authbus-p1-3-final-closure.yml \
  scripts/authbus-p1-3-finalize.py \
  scripts/authbus-p1-3-semantic-completion.py \
  scripts/authbus-p1-3-gap-closure.py \
  scripts/authbus-p1-3-clippy-completion.py \
  scripts/authbus-p1-3-final-closure.sh
do
  test ! -e "${path}"
done
python3 scripts/verify-authbus-p1-2.py | tee /tmp/p12-clean-source.json
require_line PASS_AUTHBUS_P1_2_SOURCE_ONLY /tmp/p12-clean-source.json
python3 scripts/verify-authbus-p1-3.py | tee /tmp/p13-clean-source.json
require_line PASS_AUTHBUS_P1_3_SOURCE_ONLY /tmp/p13-clean-source.json
git diff --cached --check

git config user.name 'github-actions[bot]'
git config user.email '41898282+github-actions[bot]@users.noreply.github.com'
git commit -m 'feat(authbus): close full-stack P1.3 gaps'
source_sha=$(git rev-parse HEAD)
source_tree=$(git rev-parse HEAD^{tree})
test "$(git rev-parse HEAD^)" = "${BASE_SHA}"
test "${source_tree}" != "${BASE_TREE}"
! git ls-remote --exit-code --heads origin "refs/heads/${TARGET_BRANCH}" >/dev/null 2>&1
git push origin "HEAD:refs/heads/${TARGET_BRANCH}"
{
  echo "source_sha=${source_sha}"
  echo "source_tree=${source_tree}"
  echo "source_parent=${BASE_SHA}"
  echo 'changed_paths=29'
  echo "target_branch=${TARGET_BRANCH}"
  echo 'source_working_tree_qualified=true'
  echo 'authority=false'
  echo 'effect_authority=false'
  echo 'production_caller=false'
  echo 'production_writer=false'
  echo 'provider_call_enabled=false'
  echo 'listener_enabled=false'
  echo 'openbao_enabled=false'
} | tee -a "${GITHUB_STEP_SUMMARY}"
