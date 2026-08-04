#!/bin/bash -p
set +x
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
for environment_name in "${!GIT_@}"; do unset "$environment_name"; done
PATH="/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin"; export PATH
umask 077

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

EVIDENCE_DIR=""
REPORT_PATH=""
REQUIRE="source"
VERIFY_FEATURES=0
VERIFY_PACKAGE=0
VERIFY_BROWSER=0
VERIFY_WINDOW=0
TARGET_DIR="${HEPTA_NATIVE_CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/hepta-ui-current-readiness-target}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --evidence-dir) EVIDENCE_DIR="${2:-}"; shift 2 ;;
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --verify) VERIFY_FEATURES=1; VERIFY_PACKAGE=1; shift ;;
    --verify-local) VERIFY_FEATURES=1; VERIFY_PACKAGE=1; VERIFY_BROWSER=1; VERIFY_WINDOW=1; shift ;;
    --verify-features) VERIFY_FEATURES=1; shift ;;
    --verify-package) VERIFY_PACKAGE=1; shift ;;
    --verify-browser) VERIFY_BROWSER=1; shift ;;
    --verify-window) VERIFY_WINDOW=1; shift ;;
    --target-dir) TARGET_DIR="${2:-}"; shift 2 ;;
    --require) REQUIRE="${2:-}"; shift 2 ;;
    --help|-h)
      cat <<'EOF'
usage: scripts/hepta-ui-current-readiness.sh [options]

  --evidence-dir DIR       write all current-run receipts under DIR
  --output FILE            write the orchestrator report (default stdout)
  --verify                 run Native feature matrix and local unsigned package
  --verify-local           run every source/local-demo verifier
  --verify-features        run no-default/default/all-features Cargo checks
  --verify-package         build and stage a current-source unsigned Hepta.app
  --verify-browser         run the Control four-viewport browser smoke
  --verify-window          run the independent current-package Native window verifier
  --target-dir DIR         shared Native Cargo target directory
  --require LEVEL          none|source|local|full|ga (default source)

The gate is fail-closed. Missing live Matrix, live Hepta adapter, real-device,
accessibility, signing, notarization, stapling, or distribution receipts remain
false. Historical evidence is never consumed.
EOF
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

case "$REQUIRE" in none|source|local|full|ga) ;; *) echo "invalid --require level: $REQUIRE" >&2; exit 64 ;; esac
case "$REQUIRE" in
  source)
    VERIFY_FEATURES=1
    ;;
  local|full|ga)
    VERIFY_FEATURES=1
    VERIFY_PACKAGE=1
    VERIFY_BROWSER=1
    VERIFY_WINDOW=1
    ;;
esac
for command in /usr/bin/git /usr/bin/jq /usr/bin/ruby /usr/bin/shasum /usr/bin/uuidgen /usr/bin/env; do
  [[ -x "$command" && ! -L "$command" ]] || { echo "trusted tool unavailable: $command" >&2; exit 2; }
done
RG_PATH=""
for rg_candidate in /Users/qianqi/.openclaw/npm/projects/*/node_modules/@openclaw/codex/node_modules/@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/codex-path/rg; do
  if [[ -x "$rg_candidate" && ! -L "$rg_candidate" \
    && "$(/usr/bin/shasum -a 256 "$rg_candidate" | /usr/bin/awk '{print $1}')" == "4fdf1d8365af224bc70e3c1490d8461d859c37cc70e739a11e987af0215f3e94" ]]; then
    RG_PATH="$rg_candidate"; break
  fi
done
[[ -n "$RG_PATH" ]] || { echo "trusted ripgrep unavailable" >&2; exit 2; }
PATH="$PATH:${RG_PATH%/*}"; export PATH

trusted_git() {
  /usr/bin/env -i HOME=/var/empty PATH=/usr/bin:/bin:/usr/sbin:/sbin LC_ALL=C \
    GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null \
    /usr/bin/git -C "$ROOT_DIR" "$@"
}

source_fingerprint() {
  /usr/bin/env -i HOME=/var/empty PATH=/usr/bin:/bin:/usr/sbin:/sbin LC_ALL=C \
    GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null \
    /usr/bin/ruby "$ROOT_DIR/scripts/hepta-ui-source-fingerprint"
}

normalize_future_path() {
  /usr/bin/ruby -e '
    cursor = File.expand_path(ARGV.fetch(0))
    suffix = []
    until File.exist?(cursor) || File.dirname(cursor) == cursor
      suffix.unshift(File.basename(cursor))
      cursor = File.dirname(cursor)
    end
    base = File.realpath(cursor)
    print File.join(base, *suffix)
  ' "$1"
}

path_is_within() {
  /usr/bin/ruby -e '
    path = File.expand_path(ARGV.fetch(0))
    root = File.expand_path(ARGV.fetch(1))
    exit(path.start_with?(root + File::SEPARATOR) ? 0 : 1)
  ' "$1" "$2"
}

if [[ -z "$EVIDENCE_DIR" ]]; then
  EVIDENCE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-current-readiness.XXXXXX")"
elif [[ -L "$EVIDENCE_DIR" || ( -e "$EVIDENCE_DIR" && ! -d "$EVIDENCE_DIR" ) ]]; then
  echo "--evidence-dir must be a real directory, not a symlink or non-directory" >&2
  exit 64
fi
if [[ -L "$TARGET_DIR" || ( -e "$TARGET_DIR" && ! -d "$TARGET_DIR" ) ]]; then
  echo "--target-dir must be a real directory, not a symlink or non-directory" >&2
  exit 64
fi
EVIDENCE_DIR="$(normalize_future_path "$EVIDENCE_DIR")"
TARGET_DIR="$(normalize_future_path "$TARGET_DIR")"
if [[ "$EVIDENCE_DIR" == "$TARGET_DIR" ]] || path_is_within "$EVIDENCE_DIR" "$TARGET_DIR" || path_is_within "$TARGET_DIR" "$EVIDENCE_DIR"; then
  echo "--evidence-dir and --target-dir must not overlap" >&2
  exit 64
fi
mkdir -p "$EVIDENCE_DIR" "$TARGET_DIR"
if [[ -z "$REPORT_PATH" ]]; then REPORT_PATH="$EVIDENCE_DIR/current-readiness.json"; fi
if [[ -L "$REPORT_PATH" || ( -e "$REPORT_PATH" && ! -f "$REPORT_PATH" ) ]]; then
  echo "--output must be a regular file path, not a symlink or special file" >&2
  exit 64
fi
REPORT_PATH="$(normalize_future_path "$REPORT_PATH")"
CANONICAL_CURRENT_REPORT="$EVIDENCE_DIR/current-readiness.json"
if [[ "$REPORT_PATH" == "$EVIDENCE_DIR" || "$REPORT_PATH" == "$TARGET_DIR" ]] \
  || path_is_within "$REPORT_PATH" "$TARGET_DIR" || path_is_within "$TARGET_DIR" "$REPORT_PATH"; then
  echo "--output must not overlap --target-dir or equal --evidence-dir" >&2
  exit 64
fi
if path_is_within "$REPORT_PATH" "$EVIDENCE_DIR" \
  && [[ "$REPORT_PATH" != "$CANONICAL_CURRENT_REPORT" ]]; then
  echo "--output must not be placed inside the producer-owned evidence tree (except current-readiness.json)" >&2
  exit 64
fi
mkdir -p "$(dirname "$REPORT_PATH")"
RUN_NONCE="$(/usr/bin/uuidgen | /usr/bin/tr '[:upper:]' '[:lower:]')"

BINDING_BEFORE="$EVIDENCE_DIR/source-binding-before.json"
BINDING_AFTER="$EVIDENCE_DIR/source-binding-after.json"
BINDING_FINAL="$EVIDENCE_DIR/source-binding-final.json"
SYNC_REPORT="$EVIDENCE_DIR/native-upstream-sync.json"
PRODUCT_REPORT="$EVIDENCE_DIR/native-product-shell.json"
TOKEN_REPORT="$EVIDENCE_DIR/token-sync.json"
FEATURE_REPORT="$EVIDENCE_DIR/native-feature-matrix.json"
PACKAGE_REPORT="$EVIDENCE_DIR/native-current-package.json"
BROWSER_REPORT="$EVIDENCE_DIR/control-browser-smoke.json"
MOBILE_REPORT="$EVIDENCE_DIR/native-mobile-readiness.json"
WINDOW_REPORT="$EVIDENCE_DIR/native-window-current.json"
BROWSER_LOG="$EVIDENCE_DIR/control-browser-smoke.log"
CURRENT_REPORT="$CANONICAL_CURRENT_REPORT"

MATRIX_LIVE_RECEIPT="$EVIDENCE_DIR/matrix-live.json"
BRIDGE_LIVE_RECEIPT="$EVIDENCE_DIR/hepta-bridge-live.json"
DEVICE_RECEIPT="$EVIDENCE_DIR/device-lab.json"
ACCESSIBILITY_RECEIPT="$EVIDENCE_DIR/accessibility.json"
RELEASE_RECEIPT="$EVIDENCE_DIR/release.json"
PROMOTION_TRUST_POLICY="$ROOT_DIR/apps/hepta-native/promotion-trust-policy-v1.json"
PROMOTION_TRUST_POLICY_RELATIVE="apps/hepta-native/promotion-trust-policy-v1.json"
PROMOTION_TRUST_POLICY_SNAPSHOT="$EVIDENCE_DIR/promotion-trust-policy.head.json"

# Freeze the source tuple before consulting any trust material. The source
# fingerprint runs with no caller-controlled Git/Ruby routing environment.
binding_tmp="$(mktemp "$EVIDENCE_DIR/.source-binding-before.XXXXXX")"
source_fingerprint >"$binding_tmp"
mv -f -- "$binding_tmp" "$BINDING_BEFORE"
policy_source_head="$(/usr/bin/jq -r '.head' "$BINDING_BEFORE")"

promotion_trust_policy_ready=false
promotion_trust_policy_blob_oid=""
promotion_trust_policy_sha256=""
promotion_trust_policy_worktree_matches_head=false
promotion_trust_policy_index_flags_clear=false
PROMOTION_TRUST_POLICY_CANONICAL_JSON=""
policy_tree_entry="$(trusted_git ls-tree "$policy_source_head" -- "$PROMOTION_TRUST_POLICY_RELATIVE" 2>/dev/null || true)"
policy_tree_metadata="${policy_tree_entry%%$'\t'*}"
policy_tree_path="${policy_tree_entry#*$'\t'}"
read -r policy_tree_mode policy_tree_type promotion_trust_policy_blob_oid <<<"$policy_tree_metadata"
policy_index_record="$(trusted_git ls-files -v -- "$PROMOTION_TRUST_POLICY_RELATIVE" 2>/dev/null || true)"
if [[ "$policy_index_record" == "H $PROMOTION_TRUST_POLICY_RELATIVE" ]]; then
  promotion_trust_policy_index_flags_clear=true
fi
if [[ ( "$policy_tree_mode" == "100644" || "$policy_tree_mode" == "100755" ) \
  && "$policy_tree_type" == "blob" && "$promotion_trust_policy_blob_oid" =~ ^[0-9a-f]{40}$ \
  && "$policy_tree_path" == "$PROMOTION_TRUST_POLICY_RELATIVE" ]]; then
  policy_snapshot_tmp="$(mktemp "$EVIDENCE_DIR/.promotion-policy.XXXXXX")"
  if trusted_git cat-file blob "$promotion_trust_policy_blob_oid" >"$policy_snapshot_tmp"; then
    mv -f -- "$policy_snapshot_tmp" "$PROMOTION_TRUST_POLICY_SNAPSHOT"
    promotion_trust_policy_sha256="$(/usr/bin/shasum -a 256 "$PROMOTION_TRUST_POLICY_SNAPSHOT" | /usr/bin/awk '{print $1}')"
  else
    rm -f -- "$policy_snapshot_tmp"
  fi
fi
if [[ -n "$promotion_trust_policy_sha256" && -f "$PROMOTION_TRUST_POLICY" && ! -L "$PROMOTION_TRUST_POLICY" ]] \
  && /usr/bin/cmp -s "$PROMOTION_TRUST_POLICY_SNAPSHOT" "$PROMOTION_TRUST_POLICY"; then
  promotion_trust_policy_worktree_matches_head=true
fi
if [[ "$promotion_trust_policy_index_flags_clear" == true && "$promotion_trust_policy_worktree_matches_head" == true ]]; then
  PROMOTION_TRUST_POLICY_CANONICAL_JSON="$(/usr/bin/ruby - "$PROMOTION_TRUST_POLICY_SNAPSHOT" <<'RUBY' 2>/dev/null || true
require "json"
class StrictObject < Hash
  def []=(key, value)
    raise "duplicate key" if key?(key)
    super
  end
end
def exact(value, fields)
  raise "field set" unless value.is_a?(Hash) && value.keys.sort == fields.sort
end
def safe(value)
  value.is_a?(String) && !value.empty? && value.bytesize <= 256 && !value.match?(/[[:cntrl:]]/)
end
policy = JSON.parse(File.binread(ARGV.fetch(0)), object_class:StrictObject, array_class:Array, create_additions:false)
exact(policy, %w[schema_version kind profiles activation_rule external_actions_performed])
raise "header" unless policy["schema_version"] == 1 && policy["kind"] == "hepta-ui-promotion-trust-policy-v1" && policy["external_actions_performed"] == false
profiles = policy["profiles"]
exact(profiles, %w[matrix_live bridge_live device_lab accessibility release])
%w[matrix_live bridge_live device_lab accessibility].each do |name|
  profile = profiles[name]
  exact(profile, %w[configured expected_producer trusted_public_key_sha256])
  raise "configured" unless profile["configured"] == true || profile["configured"] == false
  if profile["configured"]
    raise "trust" unless safe(profile["expected_producer"]) && profile["trusted_public_key_sha256"].is_a?(String) && profile["trusted_public_key_sha256"].match?(/\A[0-9a-f]{64}\z/)
  else
    raise "disabled trust" unless profile["expected_producer"].nil? && profile["trusted_public_key_sha256"].nil?
  end
end
release = profiles["release"]
exact(release, %w[configured expected_producer trusted_public_key_sha256 expected_signing_identity expected_team_id])
raise "release configured" unless release["configured"] == true || release["configured"] == false
if release["configured"]
  raise "release trust" unless safe(release["expected_producer"]) && release["trusted_public_key_sha256"].is_a?(String) && release["trusted_public_key_sha256"].match?(/\A[0-9a-f]{64}\z/) && safe(release["expected_signing_identity"]) && release["expected_team_id"].is_a?(String) && release["expected_team_id"].match?(/\A[A-Z0-9]{10}\z/)
else
  raise "disabled release" unless %w[expected_producer trusted_public_key_sha256 expected_signing_identity expected_team_id].all? { |key| release[key].nil? }
end
raise "activation" unless safe(policy["activation_rule"])
print JSON.generate(policy)
RUBY
)"
  if [[ -n "$PROMOTION_TRUST_POLICY_CANONICAL_JSON" ]]; then promotion_trust_policy_ready=true; fi
fi

trust_value() {
  local profile="$1" field="$2"
  if [[ "$promotion_trust_policy_ready" == "true" ]] \
    && /usr/bin/jq -e --arg profile "$profile" '.profiles[$profile].configured == true' <<<"$PROMOTION_TRUST_POLICY_CANONICAL_JSON" >/dev/null 2>&1; then
    /usr/bin/jq -r --arg profile "$profile" --arg field "$field" '.profiles[$profile][$field] // ""' <<<"$PROMOTION_TRUST_POLICY_CANONICAL_JSON"
  fi
}

# Promotion inputs are raw, independently signed attestations. The canonical
# gate executes each verifier itself during this run; caller-supplied final
# verifier receipts are deliberately ignored because a bare JSON producer
# string is not a trust anchor.
MATRIX_ATTESTATION="${HEPTA_UI_MATRIX_LIVE_ATTESTATION:-}"
MATRIX_ARTIFACT="${HEPTA_UI_MATRIX_LIVE_ARTIFACT:-}"
MATRIX_SIGNATURE="${HEPTA_UI_MATRIX_LIVE_SIGNATURE:-}"
MATRIX_PUBLIC_KEY="${HEPTA_UI_MATRIX_LIVE_TRUSTED_PUBLIC_KEY:-}"
MATRIX_PUBLIC_KEY_SHA256="$(trust_value matrix_live trusted_public_key_sha256)"
MATRIX_EXPECTED_PRODUCER="$(trust_value matrix_live expected_producer)"
BRIDGE_ATTESTATION="${HEPTA_UI_BRIDGE_LIVE_ATTESTATION:-}"
BRIDGE_ARTIFACT="${HEPTA_UI_BRIDGE_LIVE_ARTIFACT:-}"
BRIDGE_SIGNATURE="${HEPTA_UI_BRIDGE_LIVE_SIGNATURE:-}"
BRIDGE_PUBLIC_KEY="${HEPTA_UI_BRIDGE_LIVE_TRUSTED_PUBLIC_KEY:-}"
BRIDGE_PUBLIC_KEY_SHA256="$(trust_value bridge_live trusted_public_key_sha256)"
BRIDGE_EXPECTED_PRODUCER="$(trust_value bridge_live expected_producer)"
DEVICE_ATTESTATION="${HEPTA_UI_DEVICE_LAB_ATTESTATION:-}"
DEVICE_ARTIFACT="${HEPTA_UI_DEVICE_LAB_ARTIFACT:-}"
DEVICE_SIGNATURE="${HEPTA_UI_DEVICE_LAB_SIGNATURE:-}"
DEVICE_PUBLIC_KEY="${HEPTA_UI_DEVICE_LAB_TRUSTED_PUBLIC_KEY:-}"
DEVICE_PUBLIC_KEY_SHA256="$(trust_value device_lab trusted_public_key_sha256)"
DEVICE_EXPECTED_PRODUCER="$(trust_value device_lab expected_producer)"
ACCESSIBILITY_ATTESTATION="${HEPTA_UI_ACCESSIBILITY_ATTESTATION:-}"
ACCESSIBILITY_ARTIFACT="${HEPTA_UI_ACCESSIBILITY_ARTIFACT:-}"
ACCESSIBILITY_SIGNATURE="${HEPTA_UI_ACCESSIBILITY_SIGNATURE:-}"
ACCESSIBILITY_PUBLIC_KEY="${HEPTA_UI_ACCESSIBILITY_TRUSTED_PUBLIC_KEY:-}"
ACCESSIBILITY_PUBLIC_KEY_SHA256="$(trust_value accessibility trusted_public_key_sha256)"
ACCESSIBILITY_EXPECTED_PRODUCER="$(trust_value accessibility expected_producer)"
RELEASE_ATTESTATION="${HEPTA_UI_RELEASE_ATTESTATION:-}"
RELEASE_ARTIFACT="${HEPTA_UI_RELEASE_ARTIFACT:-}"
RELEASE_SIGNATURE="${HEPTA_UI_RELEASE_SIGNATURE:-}"
RELEASE_PUBLIC_KEY="${HEPTA_UI_RELEASE_TRUSTED_PUBLIC_KEY:-}"
RELEASE_PUBLIC_KEY_SHA256="$(trust_value release trusted_public_key_sha256)"
RELEASE_EXPECTED_PRODUCER="$(trust_value release expected_producer)"
RELEASE_EXPECTED_SIGNING_IDENTITY="$(trust_value release expected_signing_identity)"
RELEASE_EXPECTED_TEAM_ID="$(trust_value release expected_team_id)"

writable_paths=(
  "$BINDING_BEFORE" "$BINDING_AFTER" "$BINDING_FINAL" "$SYNC_REPORT" "$PRODUCT_REPORT"
  "$TOKEN_REPORT" "$FEATURE_REPORT" "$PACKAGE_REPORT" "$BROWSER_REPORT"
  "$MOBILE_REPORT" "$WINDOW_REPORT" "$MATRIX_LIVE_RECEIPT" "$BRIDGE_LIVE_RECEIPT"
  "$DEVICE_RECEIPT" "$ACCESSIBILITY_RECEIPT" "$RELEASE_RECEIPT"
  "$BROWSER_LOG" "$CURRENT_REPORT" "$REPORT_PATH"
)
for writable_path in "${writable_paths[@]}"; do
  if [[ -L "$writable_path" || ( -e "$writable_path" && ! -f "$writable_path" ) ]]; then
    echo "current-readiness output path must be a regular file and not a symlink: $writable_path" >&2
    exit 64
  fi
done
promotion_inputs=(
  "$MATRIX_ATTESTATION" "$MATRIX_ARTIFACT" "$MATRIX_SIGNATURE" "$MATRIX_PUBLIC_KEY"
  "$BRIDGE_ATTESTATION" "$BRIDGE_ARTIFACT" "$BRIDGE_SIGNATURE" "$BRIDGE_PUBLIC_KEY"
  "$DEVICE_ATTESTATION" "$DEVICE_ARTIFACT" "$DEVICE_SIGNATURE" "$DEVICE_PUBLIC_KEY"
  "$ACCESSIBILITY_ATTESTATION" "$ACCESSIBILITY_ARTIFACT" "$ACCESSIBILITY_SIGNATURE" "$ACCESSIBILITY_PUBLIC_KEY"
  "$RELEASE_ATTESTATION" "$RELEASE_ARTIFACT" "$RELEASE_SIGNATURE" "$RELEASE_PUBLIC_KEY"
)
for input_path in "${promotion_inputs[@]}"; do
  [[ -n "$input_path" ]] || continue
  input_path="$(normalize_future_path "$input_path")"
  for writable_path in "${writable_paths[@]}"; do
    if [[ "$input_path" == "$writable_path" ]]; then
      echo "current-readiness input receipt collides with an output path: $input_path" >&2
      exit 64
    fi
  done
done
for child_report in "$SYNC_REPORT" "$PRODUCT_REPORT" "$TOKEN_REPORT" "$FEATURE_REPORT" "$PACKAGE_REPORT" "$BROWSER_REPORT" "$MOBILE_REPORT" "$WINDOW_REPORT"; do
  if [[ "$REPORT_PATH" == "$child_report" ]]; then
    echo "--output must not replace a child receipt: $child_report" >&2
    exit 64
  fi
done

# Evidence directories are reusable, but child receipts never are. Atomically
# replace each receipt with a fail-closed sentinel before invoking a producer;
# this neither follows a pre-positioned symlink nor exposes a prior ready row.
for current_run_report in "$SYNC_REPORT" "$PRODUCT_REPORT" "$TOKEN_REPORT" "$FEATURE_REPORT" "$PACKAGE_REPORT" "$BROWSER_REPORT" "$MOBILE_REPORT" "$WINDOW_REPORT" "$MATRIX_LIVE_RECEIPT" "$BRIDGE_LIVE_RECEIPT" "$DEVICE_RECEIPT" "$ACCESSIBILITY_RECEIPT" "$RELEASE_RECEIPT"; do
  sentinel="$(mktemp "$EVIDENCE_DIR/.current-run-receipt.XXXXXX")"
  printf '{"schema_version":0,"status":"not_ready","blockers":["current_run_producer_not_completed"]}\n' >"$sentinel"
  mv -f -- "$sentinel" "$current_run_report"
done

sync_rc=0
scripts/hepta-native-robrix-upstream-sync-check-v2.sh --json --strict >"$SYNC_REPORT" || sync_rc=$?
product_rc=0
scripts/hepta-native-product-shell-gate-v2.sh --json --output "$PRODUCT_REPORT" || product_rc=$?
token_rc=0
scripts/hepta-ui-light-glass-token-sync.rb --check >"$TOKEN_REPORT" || token_rc=$?

mobile_rc=0
scripts/hepta-native-mobile-readiness-gate.sh --output "$MOBILE_REPORT" >/dev/null || mobile_rc=$?

feature_rc=0
if [[ "$VERIFY_FEATURES" == "1" ]]; then
  scripts/hepta-native-feature-matrix-gate.sh --output "$FEATURE_REPORT" --target-dir "$TARGET_DIR" || feature_rc=$?
else
  jq -n '{schema_version:1,kind:"hepta-native-feature-matrix-gate",status:"not_run",feature_matrix_ready:false,blockers:["feature_matrix_not_run_for_current_source"]}' >"$FEATURE_REPORT"
fi

package_rc=0
package_args=(--output "$PACKAGE_REPORT" --target-dir "$TARGET_DIR")
if [[ "$VERIFY_PACKAGE" == "1" ]]; then
  package_args+=(--build --bootstrap-tools --stage-dir "$EVIDENCE_DIR/native-current-package")
fi
scripts/hepta-native-current-package-gate.sh "${package_args[@]}" || package_rc=$?

window_rc=125
window_verifier_executed=false
if [[ "$VERIFY_WINDOW" == "1" ]]; then
  window_rc=0
  window_verifier_executed=true
  scripts/hepta-ui-native-window-verifier-v1 \
    --package-report "$PACKAGE_REPORT" \
    --run-nonce "$RUN_NONCE" \
    --evidence-dir "$EVIDENCE_DIR/native-window" \
    --output "$WINDOW_REPORT" || window_rc=$?
fi

browser_rc=0
if [[ "$VERIFY_BROWSER" == "1" ]]; then
  browser_log_tmp="$(mktemp "$EVIDENCE_DIR/.control-browser-smoke-log.XXXXXX")"
  HEPTA_CONTROL_UI_BROWSER_SMOKE_REPORT_PATH="$BROWSER_REPORT" \
    scripts/hepta-control-ui-browser-smoke.sh >"$browser_log_tmp" 2>&1 || browser_rc=$?
  mv -f -- "$browser_log_tmp" "$BROWSER_LOG"
else
  jq -n '{schema_version:1,kind:"hepta-control-ui-browser-smoke-current-wrapper",status:"not_run",browser_smoke_ready:false,blockers:["control_browser_smoke_not_run_for_current_source"]}' >"$BROWSER_REPORT"
fi

normalize_child_report() {
  local path="$1" schema="$2" kind="$3" reason="$4"
  if ! jq -e 'type == "object"' "$path" >/dev/null 2>&1; then
    jq -n --argjson schema "$schema" --arg kind "$kind" --arg reason "$reason" \
      '{schema_version:$schema,kind:$kind,status:"not_ready",source_stable_during_run:false,normalized_failure:true,blockers:[$reason]}' >"$path"
  fi
}
normalize_child_report "$SYNC_REPORT" 3 hepta-native-robrix-upstream-sync-check upstream_sync_report_invalid
normalize_child_report "$PRODUCT_REPORT" 3 hepta-native-upstream-first-product-shell-gate native_product_report_invalid
normalize_child_report "$TOKEN_REPORT" 3 hepta-ui-light-glass-token-sync token_sync_report_invalid
normalize_child_report "$FEATURE_REPORT" 1 hepta-native-feature-matrix-gate native_feature_report_invalid
normalize_child_report "$PACKAGE_REPORT" 1 hepta-native-current-package-gate native_package_report_invalid
normalize_child_report "$BROWSER_REPORT" 1 hepta-control-ui-browser-smoke-current-wrapper control_browser_report_invalid
normalize_child_report "$MOBILE_REPORT" 1 hepta-native-mobile-readiness-gate native_mobile_report_invalid

binding_tmp="$(mktemp "$EVIDENCE_DIR/.source-binding-after.XXXXXX")"
source_fingerprint >"$binding_tmp"
mv -f -- "$binding_tmp" "$BINDING_AFTER"
source_fingerprint="$(jq -r '.source_fingerprint' "$BINDING_AFTER")"
source_head="$(jq -r '.head' "$BINDING_AFTER")"
source_tree="$(jq -r '.head_tree' "$BINDING_AFTER")"

all_nonempty() {
  local value
  for value in "$@"; do [[ -n "$value" ]] || return 1; done
}

run_promotion_verifier() {
  local prefix="$1" output="$2" log="$3" verifier="$4"
  shift 4
  local output_tmp log_tmp rc=0 receipt_sha=""
  output_tmp="$(mktemp "$EVIDENCE_DIR/.promotion-output.XXXXXX")"
  log_tmp="$(mktemp "$EVIDENCE_DIR/.promotion-log.XXXXXX")"
  "$verifier" "$@" >"$output_tmp" 2>"$log_tmp" || rc=$?
  mv -f -- "$log_tmp" "$log"
  printf -v "${prefix}_verifier_executed" '%s' true
  printf -v "${prefix}_rc" '%s' "$rc"
  if [[ "$rc" == "0" ]] && jq -e 'type == "object"' "$output_tmp" >/dev/null 2>&1; then
    receipt_sha="$(/usr/bin/shasum -a 256 "$output_tmp" | /usr/bin/awk '{print $1}')"
    mv -f -- "$output_tmp" "$output"
  else
    rm -f -- "$output_tmp"
    local failure_tmp
    failure_tmp="$(mktemp "$EVIDENCE_DIR/.promotion-failure.XXXXXX")"
    jq -n --arg verifier "$verifier" --arg log "$log" --argjson exit_code "$rc" \
      '{schema_version:0,status:"not_ready",verifier:$verifier,verifier_executed:true,child_exit_code:$exit_code,stderr_log:$log,blockers:["signed_promotion_verifier_failed"]}' \
      >"$failure_tmp"
    mv -f -- "$failure_tmp" "$output"
  fi
  printf -v "${prefix}_verifier_receipt_sha256" '%s' "$receipt_sha"
}

promotion_evidence_rehash_json() {
  local profile="$1" receipt="$2" executed="$3" verifier_exit_code="$4" expected_receipt_sha="$5"
  local candidate=""
  if [[ "$executed" == true && "$verifier_exit_code" == 0 && "$expected_receipt_sha" =~ ^[0-9a-f]{64}$ \
      && -s "$receipt" && ! -L "$receipt" ]] \
    && candidate="$("$ROOT_DIR/scripts/lib/hepta-ui-promotion-verifier-v1.sh" rehash "$profile" --receipt "$receipt" 2>/dev/null)" \
    && /usr/bin/jq -e --arg profile "$profile" --arg expected "$expected_receipt_sha" '
      .schema_version == 1
      and .kind == "hepta-ui-promotion-evidence-rehash-v1"
      and .status == "ready"
      and .profile == $profile
      and .receipt_sha256 == $expected
      and (.manifest_sha256 | test("^[0-9a-f]{64}$"))
      and (.entry_set_sha256 | test("^[0-9a-f]{64}$"))
      and .entry_count > 0
      and .nofollow_exact_bytes_verified == true
    ' <<<"$candidate" >/dev/null 2>&1; then
    printf '%s\n' "$candidate"
  else
    /usr/bin/jq -nc --arg profile "$profile" \
      '{schema_version:1,kind:"hepta-ui-promotion-evidence-rehash-v1",status:"not_ready",profile:$profile,receipt_sha256:"",manifest_sha256:"",entry_count:0,entry_set_sha256:"",nofollow_exact_bytes_verified:false}'
  fi
}

matrix_rc=125; matrix_verifier_executed=false; matrix_verifier_receipt_sha256=""
bridge_rc=125; bridge_verifier_executed=false; bridge_verifier_receipt_sha256=""
device_rc=125; device_verifier_executed=false; device_verifier_receipt_sha256=""
accessibility_rc=125; accessibility_verifier_executed=false; accessibility_verifier_receipt_sha256=""
release_rc=125; release_verifier_executed=false; release_verifier_receipt_sha256=""

if all_nonempty "$MATRIX_ATTESTATION" "$MATRIX_ARTIFACT" "$MATRIX_SIGNATURE" "$MATRIX_PUBLIC_KEY" "$MATRIX_PUBLIC_KEY_SHA256" "$MATRIX_EXPECTED_PRODUCER"; then
  run_promotion_verifier matrix "$MATRIX_LIVE_RECEIPT" "$EVIDENCE_DIR/matrix-live-verifier.log" scripts/hepta-ui-matrix-live-verifier-v1 \
    --receipt "$MATRIX_ATTESTATION" --artifact "$MATRIX_ARTIFACT" --signature "$MATRIX_SIGNATURE" \
    --trusted-public-key "$MATRIX_PUBLIC_KEY" --expected-public-key-sha256 "$MATRIX_PUBLIC_KEY_SHA256" \
    --expected-producer "$MATRIX_EXPECTED_PRODUCER" --source-head "$source_head" --source-tree "$source_tree" \
    --source-fingerprint "$source_fingerprint" --trust-policy-sha256 "$promotion_trust_policy_sha256"
fi

if all_nonempty "$BRIDGE_ATTESTATION" "$BRIDGE_ARTIFACT" "$BRIDGE_SIGNATURE" "$BRIDGE_PUBLIC_KEY" "$BRIDGE_PUBLIC_KEY_SHA256" "$BRIDGE_EXPECTED_PRODUCER" \
  "$MATRIX_ATTESTATION" "$MATRIX_ARTIFACT" "$MATRIX_SIGNATURE" "$MATRIX_PUBLIC_KEY" "$MATRIX_PUBLIC_KEY_SHA256" "$MATRIX_EXPECTED_PRODUCER"; then
  run_promotion_verifier bridge "$BRIDGE_LIVE_RECEIPT" "$EVIDENCE_DIR/bridge-live-verifier.log" scripts/hepta-ui-bridge-live-verifier-v1 \
    --receipt "$BRIDGE_ATTESTATION" --artifact "$BRIDGE_ARTIFACT" --signature "$BRIDGE_SIGNATURE" \
    --trusted-public-key "$BRIDGE_PUBLIC_KEY" --expected-public-key-sha256 "$BRIDGE_PUBLIC_KEY_SHA256" \
    --expected-producer "$BRIDGE_EXPECTED_PRODUCER" --source-head "$source_head" --source-tree "$source_tree" \
    --source-fingerprint "$source_fingerprint" --trust-policy-sha256 "$promotion_trust_policy_sha256" \
    --matrix-receipt "$MATRIX_ATTESTATION" --matrix-artifact "$MATRIX_ARTIFACT" --matrix-signature "$MATRIX_SIGNATURE" \
    --matrix-trusted-public-key "$MATRIX_PUBLIC_KEY" --expected-matrix-public-key-sha256 "$MATRIX_PUBLIC_KEY_SHA256" \
    --expected-matrix-producer "$MATRIX_EXPECTED_PRODUCER"
fi

if all_nonempty "$DEVICE_ATTESTATION" "$DEVICE_ARTIFACT" "$DEVICE_SIGNATURE" "$DEVICE_PUBLIC_KEY" "$DEVICE_PUBLIC_KEY_SHA256" "$DEVICE_EXPECTED_PRODUCER"; then
  run_promotion_verifier device "$DEVICE_RECEIPT" "$EVIDENCE_DIR/device-lab-verifier.log" scripts/hepta-ui-device-lab-verifier-v1 \
    --receipt "$DEVICE_ATTESTATION" --artifact "$DEVICE_ARTIFACT" --signature "$DEVICE_SIGNATURE" \
    --trusted-public-key "$DEVICE_PUBLIC_KEY" --expected-public-key-sha256 "$DEVICE_PUBLIC_KEY_SHA256" \
    --expected-producer "$DEVICE_EXPECTED_PRODUCER" --source-head "$source_head" --source-tree "$source_tree" \
    --source-fingerprint "$source_fingerprint" --trust-policy-sha256 "$promotion_trust_policy_sha256"
fi

if all_nonempty "$ACCESSIBILITY_ATTESTATION" "$ACCESSIBILITY_ARTIFACT" "$ACCESSIBILITY_SIGNATURE" "$ACCESSIBILITY_PUBLIC_KEY" "$ACCESSIBILITY_PUBLIC_KEY_SHA256" "$ACCESSIBILITY_EXPECTED_PRODUCER"; then
  run_promotion_verifier accessibility "$ACCESSIBILITY_RECEIPT" "$EVIDENCE_DIR/accessibility-verifier.log" scripts/hepta-ui-accessibility-verifier-v1 \
    --receipt "$ACCESSIBILITY_ATTESTATION" --artifact "$ACCESSIBILITY_ARTIFACT" --signature "$ACCESSIBILITY_SIGNATURE" \
    --trusted-public-key "$ACCESSIBILITY_PUBLIC_KEY" --expected-public-key-sha256 "$ACCESSIBILITY_PUBLIC_KEY_SHA256" \
    --expected-producer "$ACCESSIBILITY_EXPECTED_PRODUCER" --source-head "$source_head" --source-tree "$source_tree" \
    --source-fingerprint "$source_fingerprint" --trust-policy-sha256 "$promotion_trust_policy_sha256"
fi

if all_nonempty "$RELEASE_ATTESTATION" "$RELEASE_ARTIFACT" "$RELEASE_SIGNATURE" "$RELEASE_PUBLIC_KEY" "$RELEASE_PUBLIC_KEY_SHA256" "$RELEASE_EXPECTED_PRODUCER" "$RELEASE_EXPECTED_SIGNING_IDENTITY" "$RELEASE_EXPECTED_TEAM_ID"; then
  run_promotion_verifier release "$RELEASE_RECEIPT" "$EVIDENCE_DIR/release-verifier.log" scripts/hepta-ui-release-verifier-v1 \
    --receipt "$RELEASE_ATTESTATION" --artifact "$RELEASE_ARTIFACT" --signature "$RELEASE_SIGNATURE" \
    --trusted-public-key "$RELEASE_PUBLIC_KEY" --expected-public-key-sha256 "$RELEASE_PUBLIC_KEY_SHA256" \
    --expected-producer "$RELEASE_EXPECTED_PRODUCER" --source-head "$source_head" --source-tree "$source_tree" \
    --source-fingerprint "$source_fingerprint" --trust-policy-sha256 "$promotion_trust_policy_sha256" --expected-signing-identity "$RELEASE_EXPECTED_SIGNING_IDENTITY" \
    --expected-team-id "$RELEASE_EXPECTED_TEAM_ID"
fi

# The signed verifiers bind the manifest and leaf digests, but those inputs are
# external evidence. Re-read the verifier receipt, manifest, and every leaf via
# the shared NOFOLLOW/double-read implementation before any readiness truth is
# derived from them.
matrix_evidence_rehash="$(promotion_evidence_rehash_json matrix "$MATRIX_LIVE_RECEIPT" "$matrix_verifier_executed" "$matrix_rc" "$matrix_verifier_receipt_sha256")"
bridge_evidence_rehash="$(promotion_evidence_rehash_json bridge "$BRIDGE_LIVE_RECEIPT" "$bridge_verifier_executed" "$bridge_rc" "$bridge_verifier_receipt_sha256")"
device_evidence_rehash="$(promotion_evidence_rehash_json device "$DEVICE_RECEIPT" "$device_verifier_executed" "$device_rc" "$device_verifier_receipt_sha256")"
accessibility_evidence_rehash="$(promotion_evidence_rehash_json accessibility "$ACCESSIBILITY_RECEIPT" "$accessibility_verifier_executed" "$accessibility_rc" "$accessibility_verifier_receipt_sha256")"
release_evidence_rehash="$(promotion_evidence_rehash_json release "$RELEASE_RECEIPT" "$release_verifier_executed" "$release_rc" "$release_verifier_receipt_sha256")"

if [[ "$VERIFY_BROWSER" == "1" && -s "$BROWSER_REPORT" ]]; then
  tmp_browser="$(mktemp "$EVIDENCE_DIR/.control-browser-smoke.bound.XXXXXX")"
  browser_original_valid=false
  if [[ "$browser_rc" == "0" ]] && jq -e '
      .schema_version == 1
      and .kind == "hepta-browser-visual-smoke"
      and .producer == "scripts/hepta-browser-visual-smoke.sh"
      and .status == "ready"
      and .control_ui_browser_error_page_absent == true
      and .control_ui_horizontal_overflow_free == true
      and .control_ui_320_reflow_ready == true
      and .control_ui_tempered_surface_budget_ready == true
      and .control_ui_visible_text_floor_ready == true
      and .control_ui_mobile_single_topbar_ready == true
      and .control_ui_mobile_topbar_semantics_ready == true
      and .control_ui_mobile_pane_transition_ready == true
      and .control_ui_mobile_single_bottom_action_layer_ready == true
      and .control_ui_narrow_shell_density_ready == true
      and .control_ui_narrow_single_action_row_ready == true
      and .control_ui_route_page_context_complete_ready == true
    ' "$BROWSER_REPORT" >/dev/null 2>&1; then
    browser_original_valid=true
  fi
  if [[ "$browser_original_valid" == "true" ]]; then
    jq --slurpfile binding "$BINDING_AFTER" \
      '. + {schema_version:1,kind:"hepta-control-ui-browser-smoke-current-wrapper",producer:"scripts/hepta-ui-current-readiness.sh",original_receipt:{schema_version:1,kind:"hepta-browser-visual-smoke",producer:"scripts/hepta-browser-visual-smoke.sh"},original_receipt_valid:true,browser_child_exit_code:0,source_binding:$binding[0],browser_smoke_ready:true}' \
      "$BROWSER_REPORT" >"$tmp_browser"
  else
    jq -n --slurpfile binding "$BINDING_AFTER" --argjson exit_code "$browser_rc" \
      '{schema_version:1,kind:"hepta-control-ui-browser-smoke-current-wrapper",producer:"scripts/hepta-ui-current-readiness.sh",status:"not_ready",original_receipt_valid:false,browser_child_exit_code:$exit_code,source_binding:$binding[0],browser_smoke_ready:false,blockers:["browser_child_receipt_identity_or_contract_invalid"]}' >"$tmp_browser"
  fi
  mv "$tmp_browser" "$BROWSER_REPORT"
fi

binding_stable=false
if [[ "$(jq -r '.head' "$BINDING_BEFORE")" == "$source_head" \
  && "$(jq -r '.head_tree' "$BINDING_BEFORE")" == "$source_tree" \
  && "$(jq -r '.source_fingerprint' "$BINDING_BEFORE")" == "$source_fingerprint" ]]; then binding_stable=true; fi

gate_bound() {
  local path="$1" expected_schema="$2" expected_kind="$3"
  jq -e --arg head "$source_head" --arg tree "$source_tree" --arg fingerprint "$source_fingerprint" \
    --argjson schema "$expected_schema" --arg kind "$expected_kind" '
      .schema_version == $schema
      and .kind == $kind
      and .source_stable_during_run == true
      and .source_binding.head == $head
      and .source_binding.head_tree == $tree
      and .source_binding.source_fingerprint == $fingerprint
    ' "$path" >/dev/null 2>&1
}

sync_bound=false; gate_bound "$SYNC_REPORT" 3 hepta-native-robrix-upstream-sync-check && sync_bound=true
product_bound=false; gate_bound "$PRODUCT_REPORT" 3 hepta-native-upstream-first-product-shell-gate && product_bound=true
token_bound=false; gate_bound "$TOKEN_REPORT" 3 hepta-ui-light-glass-token-sync && token_bound=true
feature_bound=false; gate_bound "$FEATURE_REPORT" 1 hepta-native-feature-matrix-gate && feature_bound=true
package_bound=false; gate_bound "$PACKAGE_REPORT" 1 hepta-native-current-package-gate && package_bound=true
mobile_bound=false; gate_bound "$MOBILE_REPORT" 1 hepta-native-mobile-readiness-gate && mobile_bound=true

receipt_json() {
  local name="$1"
  local path="$2"
  local capability="$3"
  local expected_kind="$4"
  local expected_producer="$5"
  if [[ -L "$path" ]]; then
    jq -n --arg name "$name" --arg path "$path" --arg capability "$capability" \
      '{name:$name,path:$path,present:true,bound_to_current_source:false,capability:$capability,ready:false,reason:"receipt_symlink_rejected"}'
    return
  fi
  if [[ ! -s "$path" ]]; then
    jq -n --arg name "$name" --arg path "$path" --arg capability "$capability" \
      '{name:$name,path:$path,present:false,bound_to_current_source:false,capability:$capability,ready:false,reason:"receipt_missing"}'
    return
  fi
  if ! jq -e . "$path" >/dev/null 2>&1; then
    jq -n --arg name "$name" --arg path "$path" --arg capability "$capability" \
      '{name:$name,path:$path,present:true,bound_to_current_source:false,capability:$capability,ready:false,reason:"receipt_invalid_json"}'
    return
  fi
  local artifact_path expected_sha actual_sha="" artifact_hash_valid=false
  artifact_path="$(jq -r '.artifact.path // ""' "$path")"
  expected_sha="$(jq -r '.artifact.sha256 // ""' "$path")"
  if [[ -f "$artifact_path" && ! -L "$artifact_path" && "$expected_sha" =~ ^[0-9a-f]{64}$ ]]; then
    actual_sha="$(shasum -a 256 "$artifact_path" | awk '{print $1}')"
    [[ "$actual_sha" == "$expected_sha" ]] && artifact_hash_valid=true
  fi
  jq -n --arg name "$name" --arg path "$path" --arg capability "$capability" --arg head "$source_head" \
    --arg tree "$source_tree" --arg fingerprint "$source_fingerprint" --arg expected_kind "$expected_kind" \
    --arg expected_producer "$expected_producer" --arg artifact_path "$artifact_path" --arg expected_sha "$expected_sha" \
    --arg actual_sha "$actual_sha" --argjson artifact_hash_valid "$artifact_hash_valid" --slurpfile receipt "$path" '
      ($receipt[0]) as $r |
      (($r.schema_version // null) == 1 and ($r.kind // "") == $expected_kind and ($r.producer // "") == $expected_producer) as $identity |
      (($r.source_binding.head // "") == $head and ($r.source_binding.head_tree // "") == $tree and ($r.source_binding.source_fingerprint // "") == $fingerprint) as $bound |
      (($r.independent_promotion_verifier_ready // false) == true) as $independent |
      {name:$name,path:$path,present:true,schema_and_producer_valid:$identity,bound_to_current_source:$bound,
       artifact:{path:$artifact_path,expected_sha256:$expected_sha,actual_sha256:$actual_sha,hash_valid:$artifact_hash_valid},
       capability:$capability,ready:($identity and $bound and $artifact_hash_valid and (($r.source_stable_during_run // false) == true) and ($r.status == "ready") and (($r[$capability] // false) == true) and $independent),
       reported_status:($r.status // "unknown"),source_stable_during_run:($r.source_stable_during_run // false),
       independent_verifier_ready:$independent,package:($r.package // null),
       run_nonce:($r.run_nonce // null),host_window:($r.host_window // null),automation:($r.automation // null),isolation:($r.isolation // null),
       scope:($r.scope // null),signed:(if $name == "release" then (($r.signed // false) == true) else false end),notarized:(if $name == "release" then (($r.notarized // false) == true) else false end),stapled:(if $name == "release" then (($r.stapled // false) == true) else false end)}'
}

promotion_receipt_json() {
  local name="$1" path="$2" capability="$3" expected_kind="$4" expected_verifier_producer="$5"
  local verifier_executed="$6" verifier_exit_code="$7" expected_attestor="$8" expected_public_key_sha256="$9"
  local expected_policy_sha256="${10}" expected_verifier_receipt_sha256="${11}" evidence_rehash="${12}"
  local base
  base="$(receipt_json "$name" "$path" "$capability" "$expected_kind" "$expected_verifier_producer")"
  if ! jq -e 'type == "object"' "$path" >/dev/null 2>&1; then
    jq -n --argjson base "$base" --argjson executed "$verifier_executed" --argjson exit_code "$verifier_exit_code" \
      --arg verifier_receipt_sha "$expected_verifier_receipt_sha256" --argjson evidence_rehash "$evidence_rehash" \
      '$base + {verifier_executed:$executed,verifier_exit_code:$exit_code,verifier_receipt_sha256:$verifier_receipt_sha,
        evidence_entry_rehash:$evidence_rehash,evidence_exact_bytes_rehashed:false,
        ready:false,reason:"current_run_signed_verifier_receipt_invalid"}'
    return
  fi
  jq -n --argjson base "$base" --argjson executed "$verifier_executed" --argjson exit_code "$verifier_exit_code" \
    --arg expected_attestor "$expected_attestor" --arg expected_key_sha "$expected_public_key_sha256" --arg expected_policy_sha "$expected_policy_sha256" \
    --arg verifier_receipt_sha "$expected_verifier_receipt_sha256" --argjson evidence_rehash "$evidence_rehash" \
    --slurpfile receipt "$path" '
      ($receipt[0]) as $r |
      (($r.attestation_signature.algorithm // "") == "RSA-SHA256"
        and ($r.attestation_signature.signature_verified // false) == true
        and ($r.attestation_signature.expected_producer // "") == $expected_attestor
        and ($r.attestation_signature.trusted_public_key_sha256 // "") == $expected_key_sha
        and ($expected_key_sha | test("^[0-9a-f]{64}$"))) as $signature |
      (($r.artifact.manifest_valid // false) == true
        and (($r.artifact.entry_digests // null) | type) == "array"
        and ($r.artifact.entry_digests | length) > 0
        and ($r.artifact.entry_digests | all(
          (.path | type) == "string" and (.path | startswith("/"))
          and (.sha256 | test("^[0-9a-f]{64}$"))
          and (.size_bytes | type) == "number" and .size_bytes > 0
          and .content_verified == true and .redaction_scan_passed == true
        ))) as $manifest |
      (($r.trust_policy.sha256 // "") == $expected_policy_sha
        and ($r.trust_policy.exact_head_blob_required // false) == true
        and ($expected_policy_sha | test("^[0-9a-f]{64}$"))) as $policy_bound |
      (($r.temporal_binding.freshness_verified // false) == true
        and (($r.temporal_binding.attested_at_unix_ms // 0) | type) == "number"
        and (($r.temporal_binding.expires_at_unix_ms // 0) | type) == "number"
        and ($r.temporal_binding.expires_at_unix_ms // 0) > ($r.temporal_binding.attested_at_unix_ms // 0)) as $fresh |
      (($r.input_receipt.sha256 // "") | test("^[0-9a-f]{64}$")) as $input_bound |
      ($evidence_rehash.status == "ready"
        and $evidence_rehash.nofollow_exact_bytes_verified == true
        and ($verifier_receipt_sha | test("^[0-9a-f]{64}$"))
        and $evidence_rehash.receipt_sha256 == $verifier_receipt_sha
        and $evidence_rehash.manifest_sha256 == ($r.artifact.sha256 // "")
        and ($evidence_rehash.entry_set_sha256 | test("^[0-9a-f]{64}$"))
        and $evidence_rehash.entry_count == (($r.artifact.entry_digests // []) | length)) as $rehash |
      ($base + {
        verifier_executed:$executed,verifier_exit_code:$exit_code,
        verifier_receipt_sha256:$verifier_receipt_sha,
        artifact:($base.artifact + {manifest_valid:($r.artifact.manifest_valid // false),entry_digests:($r.artifact.entry_digests // null),evidence_kind:($r.artifact.evidence_kind // null)}),
        attestation_signature:($r.attestation_signature // null),
        trust_policy:($r.trust_policy // null),
        temporal_binding:($r.temporal_binding // null),input_receipt:($r.input_receipt // null),
        verified_checks:($r.verified_checks // null),live_chain_binding:($r.live_chain_binding // null),
        signed_attestation_ready:$signature,evidence_manifest_ready:$manifest,trust_policy_bound:$policy_bound,freshness_ready:$fresh,
        evidence_entry_rehash:$evidence_rehash,evidence_exact_bytes_rehashed:$rehash
      })
      | .ready = ($base.ready and $executed and $exit_code == 0 and $signature and $manifest and $policy_bound and $fresh and $input_bound and $rehash)
      | if .ready then . else . + {reason:"current_run_signed_promotion_verification_not_ready"} end
    '
}

NATIVE_WINDOW_RECEIPT="$WINDOW_REPORT"

matrix_receipt="$(promotion_receipt_json matrix_live "$MATRIX_LIVE_RECEIPT" matrix_live_ready hepta-ui-matrix-live-receipt-v1 scripts/hepta-ui-matrix-live-verifier-v1 "$matrix_verifier_executed" "$matrix_rc" "$MATRIX_EXPECTED_PRODUCER" "$MATRIX_PUBLIC_KEY_SHA256" "$promotion_trust_policy_sha256" "$matrix_verifier_receipt_sha256" "$matrix_evidence_rehash")"
bridge_receipt="$(promotion_receipt_json hepta_bridge_live "$BRIDGE_LIVE_RECEIPT" hepta_live_bridge_ready hepta-ui-bridge-live-receipt-v1 scripts/hepta-ui-bridge-live-verifier-v1 "$bridge_verifier_executed" "$bridge_rc" "$BRIDGE_EXPECTED_PRODUCER" "$BRIDGE_PUBLIC_KEY_SHA256" "$promotion_trust_policy_sha256" "$bridge_verifier_receipt_sha256" "$bridge_evidence_rehash")"
window_receipt="$(receipt_json native_window "$NATIVE_WINDOW_RECEIPT" native_window_ready hepta-ui-native-window-receipt-v1 scripts/hepta-ui-native-window-verifier-v1)"
device_receipt="$(promotion_receipt_json device_lab "$DEVICE_RECEIPT" real_device_lab_ready hepta-ui-device-lab-receipt-v1 scripts/hepta-ui-device-lab-verifier-v1 "$device_verifier_executed" "$device_rc" "$DEVICE_EXPECTED_PRODUCER" "$DEVICE_PUBLIC_KEY_SHA256" "$promotion_trust_policy_sha256" "$device_verifier_receipt_sha256" "$device_evidence_rehash")"
accessibility_receipt="$(promotion_receipt_json accessibility "$ACCESSIBILITY_RECEIPT" accessibility_ready hepta-ui-accessibility-receipt-v1 scripts/hepta-ui-accessibility-verifier-v1 "$accessibility_verifier_executed" "$accessibility_rc" "$ACCESSIBILITY_EXPECTED_PRODUCER" "$ACCESSIBILITY_PUBLIC_KEY_SHA256" "$promotion_trust_policy_sha256" "$accessibility_verifier_receipt_sha256" "$accessibility_evidence_rehash")"
release_receipt="$(promotion_receipt_json release "$RELEASE_RECEIPT" public_distribution_ready hepta-ui-release-receipt-v1 scripts/hepta-ui-release-verifier-v1 "$release_verifier_executed" "$release_rc" "$RELEASE_EXPECTED_PRODUCER" "$RELEASE_PUBLIC_KEY_SHA256" "$promotion_trust_policy_sha256" "$release_verifier_receipt_sha256" "$release_evidence_rehash")"
package_report_sha256="$(shasum -a 256 "$PACKAGE_REPORT" | awk '{print $1}')"
package_app_path="$(jq -r '.artifact.path // ""' "$PACKAGE_REPORT")"
expected_package_app_path="$EVIDENCE_DIR/native-current-package/Hepta.app"
package_binary_path="$package_app_path/Contents/MacOS/hepta-native"
package_binary_actual_sha256=""
package_bundle_actual_sha256=""
package_artifact_hash_valid=false
promotion_trust_configured_profiles='[]'
if [[ "$promotion_trust_policy_ready" == true ]]; then
  promotion_trust_configured_profiles="$(/usr/bin/jq -c '[.profiles | to_entries[] | select(.value.configured == true) | .key]' <<<"$PROMOTION_TRUST_POLICY_CANONICAL_JSON")"
fi
if [[ -d "$package_app_path" && -x "$package_binary_path" ]]; then
  package_binary_actual_sha256="$(shasum -a 256 "$package_binary_path" | awk '{print $1}')"
  package_bundle_actual_sha256="$(scripts/hepta-ui-bundle-fingerprint --root "$package_app_path")"
  if [[ "$package_binary_actual_sha256" == "$(jq -r '.artifact.binary_sha256 // ""' "$PACKAGE_REPORT")" ]]; then
    package_artifact_hash_valid=true
  fi
fi

report="$(jq -L "$ROOT_DIR/scripts/lib" -n \
  --arg generated_at_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" --arg require "$REQUIRE" --arg evidence_dir "$EVIDENCE_DIR" \
  --argjson binding_before "$(cat "$BINDING_BEFORE")" --argjson binding_after "$(cat "$BINDING_AFTER")" \
  --argjson binding_stable "$binding_stable" --argjson sync "$(cat "$SYNC_REPORT")" --argjson product "$(cat "$PRODUCT_REPORT")" \
  --argjson tokens "$(cat "$TOKEN_REPORT" 2>/dev/null || echo '{"status":"not_ready"}')" \
  --argjson feature "$(cat "$FEATURE_REPORT")" --argjson package "$(cat "$PACKAGE_REPORT")" --argjson mobile "$(cat "$MOBILE_REPORT")" --slurpfile browser_file "$BROWSER_REPORT" \
  --argjson matrix_receipt "$matrix_receipt" --argjson bridge_receipt "$bridge_receipt" --argjson window_receipt "$window_receipt" \
  --argjson device_receipt "$device_receipt" --argjson accessibility_receipt "$accessibility_receipt" --argjson release_receipt "$release_receipt" \
  --argjson sync_bound "$sync_bound" --argjson product_bound "$product_bound" --argjson token_bound "$token_bound" \
  --argjson feature_bound "$feature_bound" --argjson package_bound "$package_bound" --argjson mobile_bound "$mobile_bound" \
  --argjson sync_exit_code "$sync_rc" --argjson product_exit_code "$product_rc" --argjson token_exit_code "$token_rc" \
  --argjson feature_exit_code "$feature_rc" --argjson package_exit_code "$package_rc" --argjson browser_exit_code "$browser_rc" --argjson mobile_exit_code "$mobile_rc" \
  --argjson window_exit_code "$window_rc" --argjson window_verifier_executed "$window_verifier_executed" \
  --arg promotion_trust_policy_path "$PROMOTION_TRUST_POLICY" --arg promotion_trust_policy_sha256 "$promotion_trust_policy_sha256" \
  --arg promotion_trust_policy_blob_oid "$promotion_trust_policy_blob_oid" --arg promotion_trust_policy_source_head "$policy_source_head" \
  --argjson promotion_trust_policy_worktree_matches_head "$promotion_trust_policy_worktree_matches_head" --argjson promotion_trust_policy_index_flags_clear "$promotion_trust_policy_index_flags_clear" \
  --argjson promotion_trust_policy_ready "$promotion_trust_policy_ready" --argjson promotion_trust_configured_profiles "$promotion_trust_configured_profiles" \
  --arg run_nonce "$RUN_NONCE" --arg package_report_path "$PACKAGE_REPORT" --arg package_report_sha256 "$package_report_sha256" \
  --arg expected_package_app_path "$expected_package_app_path" \
  --arg package_binary_path "$package_binary_path" --arg package_binary_actual_sha256 "$package_binary_actual_sha256" \
  --arg package_bundle_actual_sha256 "$package_bundle_actual_sha256" --argjson package_artifact_hash_valid "$package_artifact_hash_valid" '
    include "hepta-ui-current-readiness-v1";
    ($browser_file[0]) as $browser |
    {
      binding_stable:$binding_stable,binding_after:$binding_after,
      sync_exit_code:$sync_exit_code,product_exit_code:$product_exit_code,token_exit_code:$token_exit_code,
      feature_exit_code:$feature_exit_code,package_exit_code:$package_exit_code,mobile_exit_code:$mobile_exit_code,
      sync_bound:$sync_bound,product_bound:$product_bound,token_bound:$token_bound,
      feature_bound:$feature_bound,package_bound:$package_bound,mobile_bound:$mobile_bound,
      sync:$sync,product:$product,tokens:$tokens,feature:$feature,package:$package,mobile:$mobile,browser:$browser,
      window_verifier_executed:$window_verifier_executed,window_exit_code:$window_exit_code,
      window_receipt:$window_receipt,run_nonce:$run_nonce,
      package_report_path:$package_report_path,package_report_sha256:$package_report_sha256,
      expected_package_app_path:$expected_package_app_path,package_binary_path:$package_binary_path,
      package_binary_actual_sha256:$package_binary_actual_sha256,
      package_bundle_actual_sha256:$package_bundle_actual_sha256,
      package_artifact_hash_valid:$package_artifact_hash_valid
    } as $truth_context |
    ($truth_context | hepta_ui_readiness_truth) as $truth |
    $truth.source as $source_ready |
    $truth.browser as $browser_ready |
    $truth.promotion as $promotion_independent_verifiers_ready |
    $truth.local as $local_ready |
    hepta_ui_live_chain_bound($matrix_receipt; $bridge_receipt) as $live_chain_ready |
    hepta_ui_product_promotion_truth($local_ready; $mobile; $matrix_receipt; $bridge_receipt; $device_receipt; $accessibility_receipt; $release_receipt) as $product_truth |
    $product_truth.full as $full_ready |
    $product_truth.release_independent as $release_independent_verification_ready |
    $product_truth.ga as $ga_ready |
    ({none:true,source:$source_ready,local:$local_ready,full:$full_ready,ga:$ga_ready}[$require]) as $required_ready |
    {
      schema_version:1,
      kind:"hepta-ui-current-readiness",
      generated_at_utc:$generated_at_utc,
      status:(if $require == "none" then "report_complete" elif $required_ready then "ready" else "not_ready" end),
      report_only:($require == "none"),
      required_level:$require,
      evidence_dir:$evidence_dir,
      historical_evidence_consumed:false,
      promotion_trust_policy:{path:$promotion_trust_policy_path,sha256:$promotion_trust_policy_sha256,git_blob_oid:$promotion_trust_policy_blob_oid,source_head:$promotion_trust_policy_source_head,loaded_from_exact_head_blob:$promotion_trust_policy_ready,worktree_matches_head:$promotion_trust_policy_worktree_matches_head,index_flags_clear:$promotion_trust_policy_index_flags_clear,contract_ready:$promotion_trust_policy_ready,configured_profiles:$promotion_trust_configured_profiles,runtime_trust_anchor_override_allowed:false},
      source_binding:$binding_after,
      source_stable_during_run:$binding_stable,
      current_head_active_truth_ready:($binding_stable and $binding_after.repository_worktree_clean),
      readiness:{source:$source_ready,local_demo:$local_ready,full_product:$full_ready,public_ga:$ga_ready},
      gates:{
        upstream_sync:{status:$sync.status,exit_code:$sync_exit_code,bound_to_current_source:$sync_bound,path_ledger_ready:($sync.path_ledger_ready // false),report:"native-upstream-sync.json"},
        native_product_shell:{status:$product.status,exit_code:$product_exit_code,bound_to_current_source:$product_bound,ready:($product.native_ui_ready // false),report:"native-product-shell.json"},
        token_sync:{status:($tokens.status // "not_ready"),exit_code:$token_exit_code,bound_to_current_source:$token_bound,schema_version:($tokens.schema_version // null),report:"token-sync.json"},
        native_feature_matrix:{status:$feature.status,exit_code:$feature_exit_code,bound_to_current_source:$feature_bound,ready:($feature.feature_matrix_ready // false),report:"native-feature-matrix.json"},
        native_package:{status:$package.status,exit_code:$package_exit_code,bound_to_current_source:$package_bound,static_ready:($package.static_package_contract_ready // false),local_package_ready:($package.local_package_ready // false),report:"native-current-package.json"},
        native_window:{status:$window_receipt.reported_status,verifier_executed:$window_verifier_executed,exit_code:$window_exit_code,bound_to_current_source:$window_receipt.bound_to_current_source,receipt_ready:$window_receipt.ready,independent_promotion_ready:$promotion_independent_verifiers_ready,scope:$window_receipt.scope,exact_packaged_executable_verified:(($window_receipt.package.visual_capture_binary_is_exact_packaged_executable // false) == true),local_host_forced:(($window_receipt.automation.no_remote // false) == true and ($window_receipt.automation.host_kind // "") == "local"),package_artifact_hash_valid:$package_artifact_hash_valid,report:"native-window-current.json"},
        native_mobile:{status:$mobile.status,exit_code:$mobile_exit_code,bound_to_current_source:$mobile_bound,source_contract_ready:($mobile.mobile_source_contract_ready // false),ios_unauthenticated_login_surface:{software_keyboard_verified:($mobile.hard_boundaries.ios_simulator_unauthenticated_login_surface_software_keyboard_verified // false),coordinate_targeted_keyboard_verified:($mobile.hard_boundaries.ios_simulator_unauthenticated_login_surface_coordinate_targeted_keyboard_verified // false),visible_anchor_safe_area_verified:($mobile.hard_boundaries.ios_simulator_unauthenticated_login_surface_visible_anchor_safe_area_verified // false),homeserver_focus_verified:false},android_unauthenticated_login_surface:{visual_verified:($mobile.hard_boundaries.android_emulator_unauthenticated_login_surface_visual_verified // false),rotation_verified:($mobile.hard_boundaries.android_emulator_unauthenticated_login_surface_rotation_verified // false),ime_verified:($mobile.hard_boundaries.android_emulator_unauthenticated_login_surface_ime_verified // false)},generic_android_visual_rotation_ime_claims_hard_false:(($mobile.hard_boundaries.android_emulator_visual_verified // false) == false and ($mobile.hard_boundaries.android_emulator_rotation_verified // false) == false and ($mobile.hard_boundaries.android_emulator_ime_verified // false) == false),full_product_ready:($mobile.hard_boundaries.mobile_full_product_ready // false),report:"native-mobile-readiness.json"},
        control_browser:{status:($browser.status // "not_run"),exit_code:$browser_exit_code,ready:$browser_ready,report:"control-browser-smoke.json"}
      },
      promotion_receipts:[$window_receipt,$matrix_receipt,$bridge_receipt,$device_receipt,$accessibility_receipt,$release_receipt],
      hard_boundaries:{promotion_independent_verifiers_ready:$promotion_independent_verifiers_ready,matrix_live_ready:$matrix_receipt.ready,hepta_live_bridge_ready:$bridge_receipt.ready,matrix_bridge_live_chain_bound:$live_chain_ready,real_device_lab_ready:$device_receipt.ready,accessibility_ready:$accessibility_receipt.ready,ios_accessibility_update_consumed:($mobile.hard_boundaries.ios_accessibility_update_consumed // false),android_accessibility_update_consumed:($mobile.hard_boundaries.android_accessibility_update_consumed // false),ios_simulator_unauthenticated_login_surface_software_keyboard_verified:($mobile.hard_boundaries.ios_simulator_unauthenticated_login_surface_software_keyboard_verified // false),ios_simulator_unauthenticated_login_surface_coordinate_targeted_keyboard_verified:($mobile.hard_boundaries.ios_simulator_unauthenticated_login_surface_coordinate_targeted_keyboard_verified // false),ios_simulator_unauthenticated_login_surface_visible_anchor_safe_area_verified:($mobile.hard_boundaries.ios_simulator_unauthenticated_login_surface_visible_anchor_safe_area_verified // false),ios_simulator_unauthenticated_login_surface_homeserver_focus_verified:false,android_secure_session_persistence_ready:($mobile.hard_boundaries.android_secure_session_persistence_ready // false),android_emulator_visual_verified:false,android_emulator_rotation_verified:false,android_emulator_ime_verified:false,mobile_full_product_ready:$product_truth.mobile_full,release_independent_verification_ready:$release_independent_verification_ready,signed:($release_receipt.ready and ($release_receipt.signed // false)),notarized:($release_receipt.ready and ($release_receipt.notarized // false)),stapled:($release_receipt.ready and ($release_receipt.stapled // false)),public_distribution_ready:$release_independent_verification_ready},
      external_side_effects_performed:false,
      blockers:([if $binding_stable then empty else "source_changed_during_gate" end,if $binding_after.repository_worktree_clean then empty else "repository_worktree_dirty" end,if $sync_exit_code == 0 then empty else "upstream_sync_child_failed" end,if $product_exit_code == 0 then empty else "native_product_child_failed" end,if $token_exit_code == 0 then empty else "token_sync_child_failed" end,if $feature_exit_code == 0 then empty else "feature_matrix_child_failed" end,if (($package.build_requested // false) == false or $package_exit_code == 0) then empty else "native_package_child_failed" end,if $mobile_exit_code == 0 then empty else "native_mobile_child_failed" end,if $window_exit_code == 0 then empty else "native_window_verifier_failed" end,if $sync_bound then empty else "upstream_sync_receipt_not_bound" end,if $product_bound then empty else "native_product_receipt_not_bound" end,if $token_bound then empty else "token_receipt_not_bound" end,if $feature_bound then empty else "feature_receipt_not_bound" end,if $package_bound then empty else "package_receipt_not_bound" end,if $mobile_bound then empty else "native_mobile_receipt_not_bound" end,if $sync.status == "ready" then empty else "upstream_sync_or_path_ledger_not_ready" end,if $product.status == "ready" then empty else "native_product_shell_not_ready" end,if $tokens.status == "ready" then empty else "token_sync_not_ready" end,if $feature.feature_matrix_ready == true then empty else "native_feature_matrix_not_ready" end,if (($package.build_requested // false) == false or $package.status == "ready") then empty else "native_package_status_not_ready" end,if $package.static_package_contract_ready == true then empty else "package_metadata_not_ready" end,if $mobile.mobile_source_contract_ready == true then empty else "native_mobile_source_contract_not_ready" end,if $package.local_package_ready == true then empty else "current_source_local_package_not_ready" end,if $browser_ready then empty else "control_browser_current_receipt_not_ready" end,if $promotion_independent_verifiers_ready then empty else "native_window_independent_promotion_not_ready" end,if $window_receipt.ready then empty else "native_window_current_receipt_not_ready" end,if $matrix_receipt.ready then empty else "matrix_live_not_ready" end,if $bridge_receipt.ready then empty else "hepta_live_bridge_not_ready" end,if $device_receipt.ready then empty else "real_device_lab_not_ready" end,if $accessibility_receipt.ready then empty else "accessibility_not_ready" end,if (($mobile.hard_boundaries.ios_accessibility_update_consumed // false) and ($mobile.hard_boundaries.android_accessibility_update_consumed // false)) then empty else "mobile_accessibility_backend_not_verified" end,if (($mobile.hard_boundaries.android_secure_session_persistence_ready // false) or $device_receipt.ready) then empty else "android_secure_credential_backend_not_verified" end,if $release_independent_verification_ready then empty else "independent_release_verification_not_ready" end])
    }
    | .blockers |= (
        . + [
          if $promotion_trust_policy_ready then empty else "promotion_trust_policy_invalid" end,
          if $live_chain_ready then empty else "matrix_bridge_live_chain_not_bound" end,
          if ($mobile.hard_boundaries.android_secure_session_persistence_ready // false)
          then empty else "android_secure_credential_backend_not_verified" end
        ]
        | unique
      )')"

write_report_atomically() {
  local destination="$1" temporary
  temporary="$(mktemp "$(dirname "$destination")/.hepta-current-readiness.XXXXXX")"
  printf '%s\n' "$report" >"$temporary"
  mv -f -- "$temporary" "$destination"
}

# A ready promotion is derived from external evidence files rather than the Git
# tree. Rehash every ready receipt and evidence leaf again at the publication
# boundary and require the exact receipt/entry set observed during evaluation.
promotion_evidence_publish_boundary_stable=true
promotion_names=(matrix_live hepta_bridge_live device_lab accessibility release)
promotion_profiles=(matrix bridge device accessibility release)
promotion_receipt_paths=("$MATRIX_LIVE_RECEIPT" "$BRIDGE_LIVE_RECEIPT" "$DEVICE_RECEIPT" "$ACCESSIBILITY_RECEIPT" "$RELEASE_RECEIPT")
for promotion_index in 0 1 2 3 4; do
  promotion_name="${promotion_names[$promotion_index]}"
  if /usr/bin/jq -e --arg name "$promotion_name" '
      any(.promotion_receipts[]; .name == $name and .ready == true)
    ' <<<"$report" >/dev/null 2>&1; then
    expected_receipt_sha="$(/usr/bin/jq -r --arg name "$promotion_name" '.promotion_receipts[] | select(.name == $name) | .evidence_entry_rehash.receipt_sha256' <<<"$report")"
    expected_manifest_sha="$(/usr/bin/jq -r --arg name "$promotion_name" '.promotion_receipts[] | select(.name == $name) | .evidence_entry_rehash.manifest_sha256' <<<"$report")"
    expected_entry_set_sha="$(/usr/bin/jq -r --arg name "$promotion_name" '.promotion_receipts[] | select(.name == $name) | .evidence_entry_rehash.entry_set_sha256' <<<"$report")"
    final_evidence_rehash="$(promotion_evidence_rehash_json "${promotion_profiles[$promotion_index]}" "${promotion_receipt_paths[$promotion_index]}" true 0 "$expected_receipt_sha")"
    if ! /usr/bin/jq -e --arg receipt_sha "$expected_receipt_sha" --arg manifest_sha "$expected_manifest_sha" --arg entry_set_sha "$expected_entry_set_sha" '
        .status == "ready"
        and .receipt_sha256 == $receipt_sha
        and .manifest_sha256 == $manifest_sha
        and .entry_set_sha256 == $entry_set_sha
        and .nofollow_exact_bytes_verified == true
      ' <<<"$final_evidence_rehash" >/dev/null 2>&1; then
      promotion_evidence_publish_boundary_stable=false
      break
    fi
  fi
done

# The child receipts and report assembly above can be expensive. Re-capture the
# source at the publication boundary so a concurrent HEAD/tree/source change
# cannot publish a stale ready receipt merely because the earlier snapshot was
# clean. The final receipt remains structured and fail-closed.
binding_tmp="$(mktemp "$EVIDENCE_DIR/.source-binding-final.XXXXXX")"
source_fingerprint >"$binding_tmp"
mv -f -- "$binding_tmp" "$BINDING_FINAL"
publish_source_stable=false
if jq -e --slurpfile evaluated "$BINDING_AFTER" '
    .head == $evaluated[0].head
    and .head_tree == $evaluated[0].head_tree
    and .source_fingerprint == $evaluated[0].source_fingerprint
    and .repository_worktree_clean == true
    and $evaluated[0].repository_worktree_clean == true
  ' "$BINDING_FINAL" >/dev/null; then
  publish_source_stable=true
fi
report="$(jq -L "$ROOT_DIR/scripts/lib" -c \
  --slurpfile final_binding "$BINDING_FINAL" \
  --argjson publish_source_stable "$publish_source_stable" \
  --argjson promotion_evidence_publish_boundary_stable "$promotion_evidence_publish_boundary_stable" '
    include "hepta-ui-current-readiness-v1";
    . + {
      source_binding_at_publish:$final_binding[0],
      source_publish_boundary_stable:$publish_source_stable,
      promotion_evidence_publish_boundary_stable:$promotion_evidence_publish_boundary_stable
    }
    | if $publish_source_stable then . else
        .source_binding = $final_binding[0]
        | hepta_ui_invalidate_derived_claims("source_changed_or_became_dirty_before_atomic_publish")
      end
    | if $promotion_evidence_publish_boundary_stable then . else
        hepta_ui_invalidate_derived_claims("promotion_evidence_changed_before_atomic_publish")
      end
  ' <<<"$report")"
write_report_atomically "$REPORT_PATH"
if [[ "$REPORT_PATH" != "$CURRENT_REPORT" ]]; then write_report_atomically "$CURRENT_REPORT"; fi
printf '%s\n' "$report"
[[ "$REQUIRE" == "none" ]] && exit 0
jq -e '.status == "ready"' <<<"$report" >/dev/null
