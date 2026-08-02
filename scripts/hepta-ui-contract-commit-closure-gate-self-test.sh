#!/bin/bash -p
set +x
PS4='+ '
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB
PATH="/usr/bin:/bin:/usr/sbin:/sbin"
export PATH

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
GATE="$ROOT/scripts/hepta-ui-contract-commit-closure-gate"
TEST_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-commit-closure.XXXXXX")"
trap 'rm -rf -- "$TEST_ROOT"' EXIT

fail() {
  echo "hepta-ui-contract-commit-closure-gate-self-test: $1" >&2
  exit 1
}

expect_failure() {
  local label="$1"
  shift
  if "$@" >/dev/null 2>&1; then
    fail "$label unexpectedly passed"
  fi
}

release_closure_paths=(
  "apps/hepta-native/packaging/app-bundle-fingerprint-v1.rb"
  "apps/hepta-native/packaging/build-macos-dmg.sh"
  "apps/hepta-native/packaging/create-macos-dmg-from-app.sh"
  "apps/hepta-native/packaging/fix-dmg-applications-icon.sh"
  "apps/hepta-native/packaging/resolve-finder-bookmark-v1.swift"
  "scripts/fixtures/hepta-ui-release-consumer-ready-v1.tar.gz"
  "scripts/hepta-native-macos-release-chain-self-test.sh"
  "scripts/hepta-ui-backend-delivery-audit-gate.sh"
  "scripts/hepta-ui-backend-delivery-receipt-roundtrip-gate.sh"
  "scripts/hepta-ui-blocker-closure-gate.sh"
  "scripts/hepta-ui-current-plan-refresh-gate.sh"
  "scripts/hepta-ui-release-approval-intake-gate.sh"
  "scripts/hepta-ui-release-artifact-boundary-gate.sh"
  "scripts/hepta-ui-release-artifact-intake-gate.sh"
  "scripts/hepta-ui-release-artifact-intake-v3-self-test.sh"
  "scripts/hepta-ui-release-artifact-roundtrip-gate.sh"
  "scripts/hepta-ui-release-consumer-forged-tuple-self-test.sh"
  "scripts/hepta-ui-release-signing-capability-gate.sh"
  "scripts/hepta-ui-risk-future-plan-gate.sh"
  "scripts/hepta-ui-root-report-replay-gate.sh"
  "scripts/lib/hepta-safe-managed-output-v1.sh"
)

git -C "$TEST_ROOT" init -q
git -C "$TEST_ROOT" config user.email hepta-ui-contract@example.invalid
git -C "$TEST_ROOT" config user.name hepta-ui-contract
mkdir -p "$TEST_ROOT/.github" "$TEST_ROOT/scripts/lib/modules"
printf '%s\n' '#!/usr/bin/env bash' 'source scripts/lib/modules/module.sh' >"$TEST_ROOT/scripts/entry.sh"
printf '%s\n' '#!/usr/bin/env bash' 'true' >"$TEST_ROOT/scripts/lib/modules/module.sh"
printf '%s\n' '#!/usr/bin/env bash' 'true' >"$TEST_ROOT/scripts/gate-placeholder"
printf '%s\n' '#!/usr/bin/env bash' 'true' >"$TEST_ROOT/scripts/self-test-placeholder"
for release_closure_path in "${release_closure_paths[@]}"; do
  mkdir -p "$TEST_ROOT/$(dirname "$release_closure_path")"
  printf '%s\n' '#!/usr/bin/env bash' 'true' >"$TEST_ROOT/$release_closure_path"
  chmod +x "$TEST_ROOT/$release_closure_path"
done
chmod +x \
  "$TEST_ROOT/scripts/entry.sh" \
  "$TEST_ROOT/scripts/lib/modules/module.sh" \
  "$TEST_ROOT/scripts/gate-placeholder" \
  "$TEST_ROOT/scripts/self-test-placeholder"
jq -n '
  {
    ui_product_contract:{baseline_entrypoint:"scripts/entry.sh verify"},
    ui_commit_closure:{
      capability_version:1,
      gate_entrypoint:"scripts/gate-placeholder verify",
      self_test_entrypoint:"scripts/self-test-placeholder",
      index_semantics:"HEAD plus staged overlay; required paths must match the worktree",
      required_files:[
        ".github/hepta-ci-contract-v1.json",
        "apps/hepta-native/packaging/app-bundle-fingerprint-v1.rb",
        "apps/hepta-native/packaging/build-macos-dmg.sh",
        "apps/hepta-native/packaging/create-macos-dmg-from-app.sh",
        "apps/hepta-native/packaging/fix-dmg-applications-icon.sh",
        "apps/hepta-native/packaging/resolve-finder-bookmark-v1.swift",
        "scripts/fixtures/hepta-ui-release-consumer-ready-v1.tar.gz",
        "scripts/hepta-native-macos-release-chain-self-test.sh",
        "scripts/hepta-ui-backend-delivery-audit-gate.sh",
        "scripts/hepta-ui-backend-delivery-receipt-roundtrip-gate.sh",
        "scripts/hepta-ui-blocker-closure-gate.sh",
        "scripts/hepta-ui-current-plan-refresh-gate.sh",
        "scripts/hepta-ui-release-approval-intake-gate.sh",
        "scripts/hepta-ui-release-artifact-boundary-gate.sh",
        "scripts/hepta-ui-release-artifact-intake-gate.sh",
        "scripts/hepta-ui-release-artifact-intake-v3-self-test.sh",
        "scripts/hepta-ui-release-artifact-roundtrip-gate.sh",
        "scripts/hepta-ui-release-consumer-forged-tuple-self-test.sh",
        "scripts/hepta-ui-release-signing-capability-gate.sh",
        "scripts/hepta-ui-risk-future-plan-gate.sh",
        "scripts/hepta-ui-root-report-replay-gate.sh",
        "scripts/lib/hepta-safe-managed-output-v1.sh"
      ],
      required_directories:["scripts/lib/modules"]
    }
  }
' >"$TEST_ROOT/.github/hepta-ci-contract-v1.json"
git -C "$TEST_ROOT" add .github apps scripts
git -C "$TEST_ROOT" -c commit.gpgsign=false commit -qm fixture

"$GATE" verify --root "$TEST_ROOT" >/dev/null \
  || fail "committed closure fixture was rejected"

printf '%s\n' 'printf "%s\n" injected >"${HEPTA_BASH_ENV_MARKER:?}"' \
  >"$TEST_ROOT/bash-env-injection.sh"
BASH_ENV="$TEST_ROOT/bash-env-injection.sh" \
HEPTA_BASH_ENV_MARKER="$TEST_ROOT/bash-env-injection-ran" \
  "$GATE" verify --root "$TEST_ROOT" >/dev/null \
  || fail "privileged-shell closure fixture was rejected"
[[ ! -e "$TEST_ROOT/bash-env-injection-ran" ]] \
  || fail "closure gate executed a caller-supplied BASH_ENV hook"

mkdir -p "$TEST_ROOT/path-shim"
printf '%s\n' '#!/bin/sh' 'printf "%s\n" shim >"${HEPTA_PATH_SHIM_MARKER:?}"' 'exit 99' \
  >"$TEST_ROOT/path-shim/jq"
chmod +x "$TEST_ROOT/path-shim/jq"
PATH="$TEST_ROOT/path-shim:/usr/bin:/bin:/usr/sbin:/sbin" \
HEPTA_PATH_SHIM_MARKER="$TEST_ROOT/path-shim-ran" \
  "$GATE" verify --root "$TEST_ROOT" >/dev/null \
  || fail "system-PATH closure fixture was rejected"
[[ ! -e "$TEST_ROOT/path-shim-ran" ]] \
  || fail "closure gate executed a caller-supplied PATH shim"

for release_closure_path in "${release_closure_paths[@]}"; do
  jq --arg path "$release_closure_path" \
    '.ui_commit_closure.required_files |= map(select(. != $path))' \
    "$TEST_ROOT/.github/hepta-ci-contract-v1.json" >"$TEST_ROOT/.github/contract.next.json"
  mv "$TEST_ROOT/.github/contract.next.json" "$TEST_ROOT/.github/hepta-ci-contract-v1.json"
  git -C "$TEST_ROOT" add .github/hepta-ci-contract-v1.json
  expect_failure "missing mandatory release closure binding: $release_closure_path" \
    "$GATE" verify --root "$TEST_ROOT"
  git -C "$TEST_ROOT" show HEAD:.github/hepta-ci-contract-v1.json \
    >"$TEST_ROOT/.github/hepta-ci-contract-v1.json"
  git -C "$TEST_ROOT" add .github/hepta-ci-contract-v1.json
done

printf '%s\n' '# staged self-test update' >>"$TEST_ROOT/scripts/self-test-placeholder"
git -C "$TEST_ROOT" add scripts/self-test-placeholder
printf '%s\n' '# unstaged release producer drift' \
  >>"$TEST_ROOT/apps/hepta-native/packaging/build-macos-dmg.sh"
expect_failure "partial-stage release producer drift" \
  "$GATE" verify --root "$TEST_ROOT"
git -C "$TEST_ROOT" show :apps/hepta-native/packaging/build-macos-dmg.sh \
  >"$TEST_ROOT/apps/hepta-native/packaging/build-macos-dmg.sh"
git -C "$TEST_ROOT" show HEAD:scripts/self-test-placeholder \
  >"$TEST_ROOT/scripts/self-test-placeholder"
chmod +x "$TEST_ROOT/scripts/self-test-placeholder"
git -C "$TEST_ROOT" add scripts/self-test-placeholder

printf '%s\n' '# staged self-test update for helper closure' \
  >>"$TEST_ROOT/scripts/self-test-placeholder"
git -C "$TEST_ROOT" add scripts/self-test-placeholder
printf '%s\n' '# unstaged managed-output helper drift' \
  >>"$TEST_ROOT/scripts/lib/hepta-safe-managed-output-v1.sh"
expect_failure "partial-stage managed-output helper drift" \
  "$GATE" verify --root "$TEST_ROOT"
git -C "$TEST_ROOT" show :scripts/lib/hepta-safe-managed-output-v1.sh \
  >"$TEST_ROOT/scripts/lib/hepta-safe-managed-output-v1.sh"
git -C "$TEST_ROOT" show HEAD:scripts/self-test-placeholder \
  >"$TEST_ROOT/scripts/self-test-placeholder"
chmod +x "$TEST_ROOT/scripts/self-test-placeholder"
git -C "$TEST_ROOT" add scripts/self-test-placeholder

printf '%s\n' '#!/usr/bin/env bash' 'true' >"$TEST_ROOT/scripts/lib/modules/untracked.sh"
expect_failure "untracked required-directory member" \
  "$GATE" verify --root "$TEST_ROOT"
rm "$TEST_ROOT/scripts/lib/modules/untracked.sh"

printf '%s\n' '# unstaged drift' >>"$TEST_ROOT/scripts/entry.sh"
expect_failure "unstaged entrypoint drift" \
  "$GATE" verify --root "$TEST_ROOT"
git -C "$TEST_ROOT" show :scripts/entry.sh >"$TEST_ROOT/scripts/entry.sh"

jq '.ui_commit_closure.required_files += ["scripts/staged-new.sh"]' \
  "$TEST_ROOT/.github/hepta-ci-contract-v1.json" >"$TEST_ROOT/.github/contract.next.json"
mv "$TEST_ROOT/.github/contract.next.json" "$TEST_ROOT/.github/hepta-ci-contract-v1.json"
printf '%s\n' '#!/usr/bin/env bash' 'true' >"$TEST_ROOT/scripts/staged-new.sh"
chmod +x "$TEST_ROOT/scripts/staged-new.sh"
git -C "$TEST_ROOT" add .github/hepta-ci-contract-v1.json
expect_failure "untracked newly required entrypoint" \
  "$GATE" verify --root "$TEST_ROOT"
git -C "$TEST_ROOT" add scripts/staged-new.sh
"$GATE" verify --root "$TEST_ROOT" >/dev/null \
  || fail "HEAD plus staged file overlay was rejected"

git -C "$TEST_ROOT" rm -q scripts/entry.sh
expect_failure "staged deletion" \
  "$GATE" verify --root "$TEST_ROOT"
git -C "$TEST_ROOT" show HEAD:scripts/entry.sh >"$TEST_ROOT/scripts/entry.sh"
chmod +x "$TEST_ROOT/scripts/entry.sh"
git -C "$TEST_ROOT" add scripts/entry.sh

jq '.ui_commit_closure.required_files += ["../outside"]' \
  "$TEST_ROOT/.github/hepta-ci-contract-v1.json" >"$TEST_ROOT/.github/contract.next.json"
mv "$TEST_ROOT/.github/contract.next.json" "$TEST_ROOT/.github/hepta-ci-contract-v1.json"
git -C "$TEST_ROOT" add .github/hepta-ci-contract-v1.json
expect_failure "path traversal" \
  "$GATE" verify --root "$TEST_ROOT"

echo "hepta-ui-contract-commit-closure-gate-self-test: PASS"
