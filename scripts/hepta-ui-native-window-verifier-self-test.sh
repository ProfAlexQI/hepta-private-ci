#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-native-window-verifier-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

scripts/hepta-ui-native-window-verifier-v1 --help >/dev/null

if scripts/hepta-ui-native-window-verifier-v1 \
  --package-report "$TEST_DIR/missing-package.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
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
grep -Fq -- '"$PACKAGE_BINARY" --force-login' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- '/usr/bin/sandbox-exec -f "$SANDBOX_PROFILE"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'peekaboo image --no-remote --mode window --window-id "$WINDOW_ID"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'peekaboo list windows --no-remote --app "PID:$APP_PID"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'scripts/hepta-ui-bundle-fingerprint --root "$PACKAGE_APP"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'scripts/hepta-ui-native-window-sandbox-profile' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'select(.title == "Hepta")' scripts/hepta-ui-native-window-verifier-v1
if rg -n 'ALLOW_BLOCKED|screen_crop|--mode screen|developer-diagnostics|hepta_ui_cargo build' scripts/hepta-ui-native-window-verifier-v1 >/dev/null; then
  echo "native-window promotion verifier contains a permissive capture fallback" >&2
  exit 1
fi

bundle="$TEST_DIR/Hepta.app"
mkdir -p "$bundle/Contents/MacOS"
printf '%s\n' one >"$bundle/Contents/MacOS/hepta-native"
first_fingerprint="$(scripts/hepta-ui-bundle-fingerprint --root "$bundle")"
printf '%s\n' two >"$bundle/Contents/MacOS/hepta-native"
second_fingerprint="$(scripts/hepta-ui-bundle-fingerprint --root "$bundle")"
[[ "$first_fingerprint" =~ ^[0-9a-f]{64}$ && "$second_fingerprint" =~ ^[0-9a-f]{64}$ && "$first_fingerprint" != "$second_fingerprint" ]] || {
  echo "bundle fingerprint did not detect artifact drift" >&2
  exit 1
}

scratch="$TEST_DIR/scratch"
mkdir -p "$scratch"
profile="$TEST_DIR/native-window.sb"
scripts/hepta-ui-native-window-sandbox-profile \
  --data-dir "$TEST_DIR/product-data" \
  --cache-dir "$TEST_DIR/product-cache" \
  --scratch-dir "$scratch" \
  --output "$profile"
[[ "$(stat -f %Lp "$profile")" == "600" ]] || { echo "sandbox profile mode is not 600" >&2; exit 1; }
grep -Fq -- '(deny network*)' "$profile"
grep -Fq -- '(global-name "com.apple.securityd")' "$profile"
/usr/bin/sandbox-exec -f "$profile" /usr/bin/true

echo "hepta-ui native-window verifier fail-closed self-test: PASS"
