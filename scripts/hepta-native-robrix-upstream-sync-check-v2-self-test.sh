#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
TMP_PARENT="${TMPDIR:-/tmp}"
TEST_REPO="$(mktemp -d "$TMP_PARENT/hepta-native-sync-portable.XXXXXX")"
trap 'rm -rf "$TEST_REPO"' EXIT

git clone --quiet --no-local "$ROOT_DIR" "$TEST_REPO/repo"
rm -rf "$TEST_REPO/repo/apps/hepta-native"
mkdir -p "$TEST_REPO/repo/apps/hepta-native"
while IFS= read -r -d '' source_path; do
  [[ -f "$ROOT_DIR/$source_path" || -L "$ROOT_DIR/$source_path" ]] || continue
  relative="${source_path#apps/hepta-native/}"
  mkdir -p "$TEST_REPO/repo/apps/hepta-native/$(dirname "$relative")"
  cp -P "$ROOT_DIR/$source_path" "$TEST_REPO/repo/apps/hepta-native/$relative"
done < <(git -C "$ROOT_DIR" ls-files --cached --others --exclude-standard -z -- apps/hepta-native)
cp "$ROOT_DIR/scripts/hepta-native-robrix-upstream-sync-check-v2.sh" "$TEST_REPO/repo/scripts/"
cp "$ROOT_DIR/scripts/hepta-ui-source-fingerprint" "$TEST_REPO/repo/scripts/"
chmod +x "$TEST_REPO/repo/scripts/hepta-native-robrix-upstream-sync-check-v2.sh" "$TEST_REPO/repo/scripts/hepta-ui-source-fingerprint"

# A clean clone does not need a local upstream object or Robrix remote; the
# committed lock+manifest and current-lineage import are the portable proof.
git -C "$TEST_REPO/repo" remote remove robrix-upstream >/dev/null 2>&1 || true
"$TEST_REPO/repo/scripts/hepta-native-robrix-upstream-sync-check-v2.sh" --json --strict >"$TEST_REPO/offline.json"
jq -e '
  .status == "ready"
  and .provenance_ready == true
  and .path_ledger_ready == true
  and .remote.configured == false
  and .remote.hygiene_ready == true
  and .checks.manifest_reconstructs_locked_tree == true
' "$TEST_REPO/offline.json" >/dev/null

# Once configured, a writable/wrong push URL is a hard provenance failure.
git -C "$TEST_REPO/repo" remote add robrix-upstream https://github.com/project-robius/robrix.git
git -C "$TEST_REPO/repo" remote set-url --push robrix-upstream https://example.invalid/writable
if "$TEST_REPO/repo/scripts/hepta-native-robrix-upstream-sync-check-v2.sh" --json --strict >"$TEST_REPO/bad-push.json"; then
  echo "unsafe Robrix push URL unexpectedly passed" >&2
  exit 1
fi
jq -e '.status == "not_ready" and .remote.configured == true and .checks.remote_hygiene_ready == false' "$TEST_REPO/bad-push.json" >/dev/null

echo "hepta-native portable upstream sync self-test: PASS"
