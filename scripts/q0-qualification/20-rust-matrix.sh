# shellcheck shell=bash
rust_out="$ARTIFACT_DIR/rust"
run_case() {
  local name=$1
  shift
  set +e
  (
    cd codex-rs
    "$@"
  ) 2>&1 | tee "$rust_out/$name.log"
  local code=${PIPESTATUS[0]}
  set -e
  printf '%s\n' "$code" > "$rust_out/$name.exit"
  printf '%-44s exit=%s\n' "$name" "$code"
}

require_test_count() {
  local name=$1
  local minimum=$2
  test "$(cat "$rust_out/$name.exit")" = "0" || return 0
  set +e
  python3 - "$rust_out/$name.log" "$minimum" <<'PY'
import pathlib
import re
import sys

text = pathlib.Path(sys.argv[1]).read_text(encoding='utf-8', errors='replace')
minimum = int(sys.argv[2])
passed = sum(int(value) for value in re.findall(r'test result: ok\. (\d+) passed;', text))
print(f'verified_test_count={passed} minimum={minimum}')
raise SystemExit(0 if passed >= minimum else 1)
PY
  local count_code=$?
  set -e
  if test "$count_code" != "0"; then
    printf '%s\n' 1 > "$rust_out/$name.exit"
  fi
}

run_test_case() {
  local name=$1
  local minimum=$2
  shift 2
  run_case "$name" "$@"
  require_test_count "$name" "$minimum"
}

run_test_case p0-2-durable-grounding 1 \
  cargo test --locked -p codex-hepta-memory durable_grounding -- --nocapture
run_test_case p0-1-fact-grounding-regression 1 \
  cargo test --locked -p codex-hepta-memory fact_grounding -- --nocapture
run_test_case p0-3-shadow 1 \
  cargo test --locked -p codex-hepta-memory shadow_ -- --nocapture
run_test_case p0-4a-mutation-state 1 \
  cargo test --locked -p codex-hepta-memory intelligence_mutation_state -- --nocapture
run_test_case p0-4b-mutation-journal 1 \
  cargo test --locked -p codex-hepta-memory intelligence_mutation_journal -- --nocapture
run_test_case p0-4c-shadow-host-memory 3 \
  cargo test --locked -p codex-hepta-memory intelligence_mutation_shadow_host -- --nocapture
run_test_case p0-3-extension-focused 1 \
  cargo test --locked -p codex-hepta-memory-extension grounding_v3 -- --nocapture
run_test_case p0-4c-agentd-focused 5 \
  cargo test --locked -p codex-hepta-agentd \
    --features qualification-intelligence-mutation-shadow \
    shadow_intelligence_mutation_host -- --nocapture
run_test_case memory-full 1 cargo test --locked -p codex-hepta-memory
run_case memory-strict-clippy \
  cargo clippy --locked -p codex-hepta-memory --all-targets --no-deps -- -D warnings
run_case agent-protocol-strict-clippy \
  cargo clippy --locked -p codex-hepta-agent-protocol --all-targets --no-deps -- -D warnings
run_case supervisor-strict-clippy \
  cargo clippy --locked -p codex-hepta-supervisor --all-targets --no-deps -- -D warnings
run_test_case extension-full 1 cargo test --locked -p codex-hepta-memory-extension
run_case extension-strict-clippy \
  cargo clippy --locked -p codex-hepta-memory-extension --all-targets --no-deps -- -D warnings
run_test_case agentd-default-off-full 1 cargo test --locked -p codex-hepta-agentd
run_case agentd-default-off-clippy \
  cargo clippy --locked -p codex-hepta-agentd --all-targets --no-deps -- -D warnings
run_test_case agentd-shadow-full 1 \
  cargo test --locked -p codex-hepta-agentd \
    --features qualification-intelligence-mutation-shadow
run_case agentd-shadow-strict-clippy \
  cargo clippy --locked -p codex-hepta-agentd \
    --features qualification-intelligence-mutation-shadow \
    --all-targets --no-deps -- -D warnings

set +e
(
  cd codex-rs
  cargo tree -e features -p codex-hepta-agentd
) | grep -F 'qualification-intelligence-mutation-shadow' >/dev/null
feature_present_code=${PIPESTATUS[1]}
set -e
if test "$feature_present_code" = "0"; then
  default_feature_code=1
else
  default_feature_code=0
fi
printf '%s\n' "$default_feature_code" > "$rust_out/agentd-feature-default-off.exit"
