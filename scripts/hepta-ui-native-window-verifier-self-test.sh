#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-native-window-verifier-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

scripts/hepta-ui-native-window-verifier-v1 --help >/dev/null

if scripts/hepta-ui-native-window-verifier-v1 \
  --package-report "$TEST_DIR/missing-package.json" \
  --evidence-dir "$TEST_DIR/evidence" \
  --output "$TEST_DIR/receipt.json" >/dev/null 2>&1; then
  echo "native-window verifier accepted a missing package report" >&2
  exit 1
fi
[[ ! -s "$TEST_DIR/receipt.json" ]] || {
  echo "missing-package path produced a non-empty promotion receipt" >&2
  exit 1
}

grep -Fq -- 'producer:"scripts/hepta-ui-native-window-verifier-v1"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'scope:"unauthenticated_local_macos_product_shell"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'and .local_package_ready == true' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'peekaboo image --mode window --window-id "$WINDOW_ID"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'select(.window_title == "Hepta")' scripts/hepta-ui-native-window-verifier-v1
if rg -n 'ALLOW_BLOCKED|screen_crop|--mode screen' scripts/hepta-ui-native-window-verifier-v1 >/dev/null; then
  echo "native-window promotion verifier contains a permissive capture fallback" >&2
  exit 1
fi

echo "hepta-ui native-window verifier fail-closed self-test: PASS"
