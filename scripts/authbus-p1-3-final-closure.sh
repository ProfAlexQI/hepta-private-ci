#!/usr/bin/env bash
set -euo pipefail

cd "${GITHUB_WORKSPACE:?}"

EXPECTED_PARENT=556c314bf345fb218f3fcf533643bdb48eab2aa2
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
run_gate contracts 'cargo check --manifest-path codex-rs/Cargo.toml --locked -p codex-hepta-contracts --features authbus-local-qualification --all-targets && cargo clippy --manifest-path codex-rs/Cargo.toml --locked -p codex-hepta-contracts --featurs authbus-local-qualification --all-targets -- -D warnings'
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
    ('runs-on: ubuntu-slim', 'runs-on: ubuntu-24.04-arm', 4, 'hosted runner'),
    ('f6702be58c499d853d273f3174a2556481a3f5b4284cd9cd0b0a247160d7ac08',
     'dfcab028e1a135a0895b3f9eddec9f5f99cf5f392701b98ad14180058a284bf1',
     3, 'registry digest'),
):
    count = source.count(before)
    if count€„ô•áÁ•Ñ•è(€€€€€€€É…¥Í”MåÍÑ•µá¥Ð¡˜í±…‰•±ôè•áÁ•Ñ•í•áÁ•Ñ•‘ô…¹¡½ÉÌ°™½Õ¹í½Õ¹Ñôœ¤(€€€Í½ÕÉ”€ôÍ½ÕÉ”¹É•Á±…”¡‰•™½É”°…™Ñ•È¤)Á…Ñ ¹ÝÉ¥Ñ•}Ñ•áÐ¡Í½ÕÉ”°•¹½‘¥¹œôÕÑ˜´àœ¤)Ad()%91}AQ!Lô (€½‘•àµÉÌ½¡•ÁÑ„µ½¹ÑÉ…ÑÌ½ÍÉŒ½…ÕÑ¡‰ÕÍ}ˆÄ¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ½¹ÑÉ…ÑÌ½ÍÉŒ½…ÕÑ¡‰ÕÍ}ˆÅ}Ñ•ÍÑÌ¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ½¹ÑÉ…ÑÌ½ÍÉŒ½…ÕÑ¡‰ÕÍ}ˆÈ¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ½¹ÑÉ…ÑÌ½ÍÉŒ½…ÕÑ¡‰ÕÍ}ˆÌ¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ½¹ÑÉ…ÑÌ½ÍÉŒ½…ÕÑ¡‰ÕÍ}ˆÍ}…‘…ÁÑ•È¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ½¹ÑÉ…ÑÌ½ÍÉŒ½…ÕÑ¡‰ÕÍ}ˆÐ¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ½¹ÑÉ…ÑÌ½ÍÉŒ½…ÕÑ¡‰ÕÍ}Ñ•ÍÑÌ¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ½¹ÑÉ…ÑÌ½ÍÉŒ½±¥ˆ¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ½¹ÑÉ…ÑÌ½ÍÉŒ½ÅÕ½Ñ…}É•¥ÍÑÉä¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ½¹ÑÉ…ÑÌ½Ñ•ÍÑÌ½…ÕÑ¡‰ÕÍ}ˆÍ}ÀÁ|Ä¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ…ÕÑ¡‰ÕÌµÅÕ…±¥™¥…Ñ¥½¸½ÍÉŒ½µ½‘•°¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ…ÕÑ¡‰ÕÌµÅÕ…±¥™¥…Ñ¥½¸½ÍÉŒ½ÍÑ½É”¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ…ÕÑ¡‰ÕÌµÀÀ´ÌµÅÕ…±¥™¥…Ñ¥½¸½ÍÉŒ½±¥ˆ¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ…ÕÑ¡‰ÕÌµÀÀ´ÌµÅÕ…±¥™¥…Ñ¥½¸½ÍÉŒ½Í¡•‘Õ±•È¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ…ÕÑ¡‰ÕÌµÀÀ´ÌµÅÕ…±¥™¥…Ñ¥½¸½Ñ•ÍÑÌ½ÀÁ|Ì¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ…ÕÑ¡‰ÕÌµÀÄ´ÌµÅÕ…±¥™¥…Ñ¥½¸½…É¼¹Ñ½µ°(€½‘•àµÉÌ½¡•ÁÑ„µ…ÕÑ¡‰ÕÌµÀÄ´ÌµÅÕ…±¥™¥…Ñ¥½¸½…É¼¹±½¬(€½‘•àµÉÌ½¡•ÁÑ„µ…ÕÑ¡‰ÕÌµÀÄ´ÌµÅÕ…±¥™¥…Ñ¥½¸½I5¹µ(€½‘•àµÉÌ½¡•ÁÑ„µ…ÕÑ¡‰ÕÌµÀÄ´ÌµÅÕ…±¥™¥…Ñ¥½¸½ÍÉŒ½±¥ˆ¹ÉÌ(€½‘•àµÉÌ½¡•ÁÑ„µ…ÕÑ¡‰ÕÌµÀÄ´ÌµÅÕ…±¥™¥…Ñ¥½¸½Ñ•ÍÑÌ½ÀÅ|Ì¹ÉÌ(€‘½Ì½¡•ÁÑ„µÙ¹•áÐ½…ÕÑ¡‰ÕÌ½UQ!	UM}@Å|Í}Y1=A59Q}A19|ÈÀÈØ´Àà´Èä¹µ(€‘½Ì½¡•ÁÑ„µÙ¹•áÐ½…ÕÑ¡‰ÕÌ½UQ!	UM}@Å|Í}%5A159QQ%=9}MQQUM|ÈÀÈØ´Àà´Èä¹©Í½¸(€‘½Ì½¡•ÁÑ„µÙ¹•áÐ½…ÕÑ¡‰ÕÌ½UQ!	UM}@Å|Í}%5A159QQ%=9}MQQUM|ÈÀÈØ´Àà´Èä¹µ(€ÍÉ¥ÁÑÌ½Ù•É¥™äµ…ÕÑ¡‰ÕÌµÀÄ´Ì¹Áä(¤)1Q%=9Lô (€€¹¥Ñ¡Õˆ½Ý½É­™±½ÝÌ½…ÕÑ¡‰ÕÌµÀÄ´Ìµ…Àµ‘¥…¹½ÍÑ¥Œ¹åµ°(€€¹¥Ñ¡Õˆ½Ý½É­™±½ÝÌ½…ÕÑ¡‰ÕÌµÀÄ´Ìµµ…Ñ•É¥…±¥é”¹åµ°(€ÍÉ¥ÁÑÌ½…ÕÑ¡‰ÕÌµÀÄ´Ìµ™¥¹…±¥é”¹Áä(€ÍÉ¥ÁÑÌ½…ÕÑ¡‰ÕÌµÀÄ´ÌµÍ•µ…¹Ñ¥Œµ½µÁ±•Ñ¥½¸¹Áä(¤()Ñ…È€µ˜€½ÑµÀ½…ÕÑ¡‰ÕÌµÀÄ´Ìµ™¥¹…°µÍ½ÕÉ”¹Ñ…Èp(€€¹¥Ñ¡Õˆ½Ý½É­™±½ÝÌ½…ÕÑ¡‰ÕÌµÀÄ´ÌµÅÕ…±¥™¥…Ñ¥½¸¹åµ°€ˆ‘í%91}AQ!Mmuôˆ)¥ÐÉ•Í•Ð€´µ¡…É)¥Ð±•…¸€µ™‘à)¥Ð¡•­½ÕÐ€´µ‘•Ñ… €ˆ‘í	M}M!ôˆ)Ñ•ÍÐ€ˆ¡¥ÐÉ•ØµÁ…ÉÍ”!è¤ˆ€ô€ˆ‘í	M}M!ôˆ)Ñ•ÍÐ€ˆ¡¥ÐÉ•ØµÁ…ÉÍ”!yíÑÉ••ô¤ˆ€ô€ˆ‘í	M}QIôˆ)Ñ…È€µá˜€½ÑµÀ½…ÕÑ¡‰ÕÌµÀÄ´Ìµ™¥¹…°µÍ½ÕÉ”¹Ñ…È)¥ÐÉ´€µ˜€ˆ‘í1Q%=9Mmuôˆ)¥Ð…‘€´´€¹¥Ñ¡Õˆ½Ý½É­™±½ÝÌ½…ÕÑ¡‰ÕÌµÀÄ´ÌµÅÕ…±¥™¥…Ñ¥½¸¹åµ°€ˆ‘í%91}AQ!Mmuôˆ)ÁÉ¥¹Ñ˜€œ•Íq¸œ€ˆ‘í1Q%=9Mmuôˆ€¹¥Ñ¡Õˆ½Ý½É­™±½ÝÌ½…ÕÑ¡‰ÕÌµÀÄ´ÌµÅÕ…±¥™¥…Ñ¥½¸¹åµ°p(€€ˆ‘í%91}AQ!Mmuôˆð1}10õÍ½ÉÐ€µÔ€ø€½ÑµÀ½ÀÄÌµ•áÁ•Ñ•µÁ…Ñ¡Ì¹ÑáÐ)¥Ð‘¥™˜€´µ…¡•€´µ¹…µ”µ½¹±äð1}10õÍ½ÉÐ€ø€½ÑµÀ½ÀÄÌµ…ÑÕ…°µÁ…Ñ¡Ì¹ÑáÐ)Ñ•ÍÐ€ˆ¡ÝŒ€µ°€ð€½ÑµÀ½ÀÄÌµ•áÁ•Ñ•µÁ…Ñ¡Ì¹ÑáÐ¤ˆ€ô€ˆÈäˆ)Ñ•ÍÐ€ˆ¡ÝŒ€µ°€ð€½ÑµÀ½ÀÄÌµ…ÑÕ…°µÁ…Ñ¡Ì¹ÑáÐ¤ˆ€ô€ˆÈäˆ)‘¥™˜€µÔ€½ÑµÀ½ÀÄÌµ•áÁ•Ñ•µÁ…Ñ¡Ì¹ÑáÐ€½ÑµÀ½ÀÄÌµ…ÑÕ…°µÁ…Ñ¡Ì¹ÑáÐ)™½ÈÁ…Ñ ¥¸p(€€¹¥Ñ¡Õˆ½Ý½É­™±½ÝÌ½…ÕÑ¡‰ÕÌµÀÄ´Ìµ…Àµ‘¥…¹½ÍÑ¥Œ¹åµ°p(€€¹¥Ñ¡Õˆ½Ý½É­™±½ÝÌ½…ÕÑ¡‰ÕÌµÀÄ´Ìµµ…Ñ•É¥…±¥é”¹åµ°p(€€¹¥Ñ¡Õˆ½Ý½É­™±½ÝÌ½…ÕÑ¡‰ÕÌµÀÄ´Ìµ±•…¸µÉ•Á±…äµµ…Ñ•É¥…±¥é”¹åµ°p(€€¹¥Ñ¡Õˆ½Ý½É­™±½ÝÌ½…ÕÑ¡‰ÕÌµÀÄ´Ìµ™¥¹…°µ±½ÍÕÉ”¹åµ°p(€ÍÉ¥ÁÑÌ½…ÕÑ¡‰ÕÌµÀÄ´Ìµ™¥¹…±¥é”¹Áäp(€ÍÉ¥ÁÑÌ½…ÕÑ¡‰ÕÌµÀÄ´ÌµÍ•µ…¹Ñ¥Œµ½µÁ±•Ñ¥½¸¹Áäp(€ÍÉ¥ÁÑÌ½…ÕÑ¡‰ÕÌµÀÄ´Ìµ…Àµ±½ÍÕÉ”¹Áäp(€ÍÉ¥ÁÑÌ½…ÕÑ¡‰ÕÌµÀÄ´Ìµ±¥ÁÁäµ½µÁ±•Ñ¥½¸¹Áäp(€ÍÉ¥ÁÑÌ½…ÕÑ¡‰ÕÌµÀÄ´Ìµ™¥¹…°µ±½ÍÕÉ”¹Í )‘¼(€Ñ•ÍÐ€„€µ”€ˆ‘íÁ…Ñ¡ôˆ)‘½¹”)ÁåÑ¡½¸ÌÍÉ¥ÁÑÌ½Ù•É¥™äµ…ÕÑ¡‰ÕÌµÀÄ´È¹ÁäðÑ•”€½ÑµÀ½ÀÄÈµ±•…¸µÍ½ÕÉ”¹©Í½¸)É•ÅÕ¥É•}±¥¹”AMM}UQ!	UM}@Å|É}M=UI}=91d€½ÑµÀ½ÀÄÈµ±•…¸µÍ½ÕÉ”¹©Í½¸)ÁåÑ¡½¸ÌÍÉ¥ÁÑÌ½Ù•É¥™äµ…ÕÑ¡‰ÕÌµÀÄ´Ì¹ÁäðÑ•”€½ÑµÀ½ÀÄÌµ±•…¸µÍ½ÕÉ”¹©Í½¸)É•ÅÕ¥É•}±¥¹”AMM}UQ!	UM}@Å|Í}M=UI}=91d€½ÑµÀ½ÀÄÌµ±•…¸µÍ½ÕÉ”¹©Í½¸)¥Ð‘¥™˜€´µ…¡•€´µ¡•¬()¥Ð½¹™¥œÕÍ•È¹¹…µ”€¥Ñ¡Õˆµ…Ñ¥½¹Ím‰½Ñtœ)¥Ð½¹™¥œÕÍ•È¹•µ…¥°€œÐÄàäàÈàÈ­¥Ñ¡Õˆµ…Ñ¥½¹Ím‰½ÑuÕÍ•ÉÌ¹¹½É•Á±ä¹¥Ñ¡Õˆ¹½´œ)¥Ð½µµ¥Ð€µ´€™•…Ð¡…ÕÑ¡‰ÕÌ¤è±½Í”™Õ±°µÍÑ…¬@Ä¸Ì…ÁÌœ)Í½ÕÉ•}Í¡„ô¡¥ÐÉ•ØµÁ…ÉÍ”!¤)Í½ÕÉ•}ÑÉ•”ô¡¥ÐÉ•ØµÁ…ÉÍ”!yíÑÉ••ô¤)Ñ•ÍÐ€ˆ¡¥ÐÉ•ØµÁ…ÉÍ”!x¤ˆ€ô€ˆ‘í	M}M!ôˆ)Ñ•ÍÐ€ˆ‘íÍ½ÕÉ•}ÑÉ••ôˆ€„ô€ˆ‘í	M}QIôˆ(„¥Ð±ÌµÉ•µ½Ñ”€´µ•á¥Ðµ½‘”€´µ¡•…‘Ì½É¥¥¸€‰É•™Ì½¡•…‘Ì¼‘íQIQ}	I9!ôˆ€ø½‘•Ø½¹Õ±°€Èø˜Ä)¥ÐÁÕÍ ½É¥¥¸€‰!éÉ•™Ì½¡•…‘Ì¼‘íQIQ}	I9!ôˆ)ì(€•¡¼€‰Í½ÕÉ•}Í¡„ô‘íÍ½ÕÉ•}Í¡…ôˆ(€•¡¼€‰Í½ÕÉ•}ÑÉ•”ô‘íÍ½ÕÉ•}ÑÉ••ôˆ(€•¡¼€‰Í½ÕÉ•}Á…É•¹Ðô‘í	M}M!ôˆ(€•¡¼€¡…¹•‘}Á…Ñ¡ÌôÈäœ(€•¡¼€‰Ñ…É•Ñ}‰É…¹ ô‘íQIQ}	I9!ôˆ(€•¡¼€Í½ÕÉ•}Ý½É­¥¹}ÑÉ••}ÅÕ…±¥™¥•õÑÉÕ”œ(€•¡¼€…ÕÑ¡½É¥Ñäõ™…±Í”œ(€•¡¼€•™™•Ñ}…ÕÑ¡½É¥Ñäõ™…±Í”œ(€•¡¼€ÁÉ½‘ÕÑ¥½¹}…±±•Èõ™…±Í”œ(€•¡¼€ÁÉ½‘ÕÑ¥½¹}ÝÉ¥Ñ•Èõ™…±Í”œ(€•¡¼€ÁÉ½Ù¥‘•É}…±±}•¹…‰±•õ™…±Í”œ(€•¡¼€±¥ÍÑ•¹•É}•¹…‰±•õ™…±Í”œ(€•¡¼€½Á•¹‰…½}•¹…‰±•õ™…±Í”œ)ôðÑ•”€µ„€ˆ‘í%Q!U	}MQA}MU55Ieôˆ(