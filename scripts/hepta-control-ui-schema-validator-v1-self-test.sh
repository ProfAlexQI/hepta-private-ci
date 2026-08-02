#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"
HELPER="scripts/hepta-control-ui-schema-validator-v1"
TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-control-ui-schema-validator-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

fail() {
  echo "hepta-control-ui-schema-validator self-test failed: $1" >&2
  exit 1
}

mkdir -p "$TEST_DIR/valid/bin" "$TEST_DIR/wrong/bin" "$TEST_DIR/venv/bin"
printf '%s\n' '#!/usr/bin/env bash' 'echo "check-jsonschema, version 0.37.4"' \
  >"$TEST_DIR/valid/bin/check-jsonschema"
printf '%s\n' '#!/usr/bin/env bash' 'echo "check-jsonschema, version 0.36.0"' \
  >"$TEST_DIR/wrong/bin/check-jsonschema"
cp "$TEST_DIR/valid/bin/check-jsonschema" "$TEST_DIR/venv/bin/check-jsonschema"
chmod 700 "$TEST_DIR/valid/bin/check-jsonschema" \
  "$TEST_DIR/wrong/bin/check-jsonschema" "$TEST_DIR/venv/bin/check-jsonschema"

resolved="$(HEPTA_CHECK_JSONSCHEMA_BIN="$TEST_DIR/valid/bin/check-jsonschema" "$HELPER")"
[[ "$resolved" == "$TEST_DIR/valid/bin/check-jsonschema" ]] \
  || fail "explicit valid executable was not resolved"

if HEPTA_CHECK_JSONSCHEMA_BIN="$TEST_DIR/wrong/bin/check-jsonschema" \
  "$HELPER" >/dev/null 2>&1; then
  fail "wrong explicit version was accepted"
fi

resolved="$(
  PATH=/usr/bin:/bin \
  HEPTA_CHECK_JSONSCHEMA_BIN= \
  HEPTA_CHECK_JSONSCHEMA_VENV="$TEST_DIR/venv" \
    "$HELPER" --bootstrap
)"
[[ "$resolved" == "$TEST_DIR/venv/bin/check-jsonschema" ]] \
  || fail "pre-provisioned exact venv was not reused"

grep -Fq -- 'scripts/hepta-control-ui-schema-validator-v1 --bootstrap' \
  scripts/hepta-control-ui-browser-smoke.sh \
  || fail "browser smoke does not bootstrap the pinned validator"

echo "hepta-control-ui-schema-validator self-test passed"
