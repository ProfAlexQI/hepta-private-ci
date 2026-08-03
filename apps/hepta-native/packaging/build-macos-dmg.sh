#!/bin/bash -p
set +x
PS4='+ '
set -Eeuo pipefail

# Capture release authority before the first external command, immediately
# remove it from the exported environment, and never expose it to the unsigned
# package lane or ordinary signing/readback tools.
CAPTURED_SIGNING_IDENTITY="${HEPTA_SIGNING_IDENTITY:-}"
CAPTURED_EXPECTED_TEAM_ID="${HEPTA_EXPECTED_TEAM_ID:-}"
CAPTURED_NOTARY_PROFILE="${HEPTA_NOTARY_PROFILE:-${HEPTA_NATIVE_NOTARYTOOL_PROFILE:-}}"
CAPTURED_DIRECT_APPLE_CREDENTIALS_PRESENT=false
if [[ -n "${APPLE_ID:-}" || -n "${APPLE_PASSWORD:-}" || -n "${APPLE_TEAM_ID:-}" ]]; then
  CAPTURED_DIRECT_APPLE_CREDENTIALS_PRESENT=true
fi
unset HEPTA_SIGNING_IDENTITY HEPTA_EXPECTED_TEAM_ID
unset HEPTA_NOTARY_PROFILE HEPTA_NATIVE_NOTARYTOOL_PROFILE
unset APPLE_ID APPLE_PASSWORD APPLE_TEAM_ID
unset BASH_ENV ENV CDPATH GLOBIGNORE
unset RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
export -n CAPTURED_SIGNING_IDENTITY CAPTURED_EXPECTED_TEAM_ID CAPTURED_NOTARY_PROFILE
export -n CAPTURED_DIRECT_APPLE_CREDENTIALS_PRESENT
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
PATH="$SYSTEM_PATH"
TMPDIR="/private/tmp"
export PATH TMPDIR

# Sign, notarize, and staple a DMG that contains one exact, previously-built
# Hepta.app. The release lane never rebuilds or edits Cargo.toml: when no app is
# supplied it first invokes the pinned formal-unsigned pipeline, then consumes
# that output unchanged apart from Apple code-signing metadata.

APP_PATH=""
APP_RECEIPT_PATH="${HEPTA_NATIVE_UNSIGNED_APP_RECEIPT_PATH:-}"
OUTPUT_PATH=""
RECEIPT_PATH="${HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH:-}"
BOOTSTRAP_TOOLS=0
PREFLIGHT_ONLY=0
RELEASE_APPROVAL_PATH=""
RELEASE_APPROVAL_SIGNATURE_PATH=""
RELEASE_APPROVAL_PUBLIC_KEY_PATH=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --app-path) APP_PATH="${2:-}"; shift 2 ;;
    --app-receipt) APP_RECEIPT_PATH="${2:-}"; shift 2 ;;
    --output) OUTPUT_PATH="${2:-}"; shift 2 ;;
    --receipt) RECEIPT_PATH="${2:-}"; shift 2 ;;
    --release-approval) RELEASE_APPROVAL_PATH="${2:-}"; shift 2 ;;
    --release-approval-signature) RELEASE_APPROVAL_SIGNATURE_PATH="${2:-}"; shift 2 ;;
    --release-approval-public-key) RELEASE_APPROVAL_PUBLIC_KEY_PATH="${2:-}"; shift 2 ;;
    --bootstrap-tools) BOOTSTRAP_TOOLS=1; shift ;;
    --preflight) PREFLIGHT_ONLY=1; shift ;;
    --help|-h)
      cat <<'EOF'
usage: packaging/build-macos-dmg.sh [options]

Options:
  --app-path PATH       Exact formal unsigned Hepta.app to consume. When
                        omitted, the canonical current-package gate creates it.
  --app-receipt PATH    Required current-package JSON receipt for --app-path.
                        May also be set with HEPTA_NATIVE_UNSIGNED_APP_RECEIPT_PATH.
  --output PATH         Output DMG (default: dist/Hepta_<version>_<arch>.dmg).
  --receipt PATH        JSON release receipt (default: <output>.receipt.json).
  --release-approval PATH
                        Signed exact-action approval JSON (absolute path).
  --release-approval-signature PATH
                        Detached RSA PKCS#1 SHA-256 signature (absolute path).
  --release-approval-public-key PATH
                        Trusted RSA public key only (absolute path).
  --bootstrap-tools     Allow the formal unsigned lane to install pinned tools.
  --preflight           Validate identity, credentials, tools, and inputs only.

Required authority:
  HEPTA_SIGNING_IDENTITY, or package.metadata.packager.macos.signing_identity
  HEPTA_NOTARY_PROFILE (keychain profile only; direct password argv is rejected)

Actual signing/notary execution requires all three release-approval inputs and
an explicit --app-path/--app-receipt pair. The approval verifier is read-only;
the detached approval must be produced and signed by the independent authority
pinned in packaging/release-execution-approval-trust-v1.json.
EOF
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

# Signing, notarization submission, stapling, and DMG attachment are release
# approval-scoped actions. Reject missing or partial approval authority before
# tool discovery, output creation, package staging, or any release side effect.
# A complete approval is still untrusted here: it is verified against the exact
# source/app/action tuple after the formal unsigned input is validated and
# immediately before the first signing operation.
RELEASE_APPROVAL_ARGUMENT_COUNT=0
for approval_argument in \
  "$RELEASE_APPROVAL_PATH" \
  "$RELEASE_APPROVAL_SIGNATURE_PATH" \
  "$RELEASE_APPROVAL_PUBLIC_KEY_PATH"; do
  if [[ -n "$approval_argument" ]]; then
    RELEASE_APPROVAL_ARGUMENT_COUNT=$((RELEASE_APPROVAL_ARGUMENT_COUNT + 1))
  fi
done
if [[ "$RELEASE_APPROVAL_ARGUMENT_COUNT" != "0" && "$RELEASE_APPROVAL_ARGUMENT_COUNT" != "3" ]]; then
  printf '%s\n' 'release approval inputs must be supplied as one complete three-file set' >&2
  exit 64
fi
if [[ "$PREFLIGHT_ONLY" != "1" ]]; then
  if [[ "$RELEASE_APPROVAL_ARGUMENT_COUNT" != "3" ]]; then
    printf '%s\n' 'signed_release_execution_approval_missing: actual release execution is disabled; run --preflight only' >&2
    printf '%s\n' 'a complete signed exact-action approval is required for release execution' >&2
    exit 77
  fi
  if [[ -z "$APP_PATH" || -z "$APP_RECEIPT_PATH" ]]; then
    printf '%s\n' 'signed release execution requires an explicit prebuilt --app-path and --app-receipt so approval can bind the exact input' >&2
    exit 77
  fi
fi

CANONICAL_HOME="$(/usr/bin/env -i PATH="$SYSTEM_PATH" /usr/bin/ruby --disable-gems -retc -e 'print Etc.getpwuid(Process.uid).dir')"
[[ "$CANONICAL_HOME" == /* && -d "$CANONICAL_HOME" && ! -L "$CANONICAL_HOME" ]] || {
  printf 'could not resolve canonical OS-account home\n' >&2
  exit 2
}
PATH="$SYSTEM_PATH:$CANONICAL_HOME/.cargo/bin"
HOME="$CANONICAL_HOME"
export PATH HOME

SCRIPT_DIR="$(cd "$(/usr/bin/dirname "${BASH_SOURCE[0]}")" && pwd -P)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd -P)"
REPO_ROOT="$(cd "$PROJECT_DIR/../.." && pwd -P)"
CARGO_TOML="$PROJECT_DIR/Cargo.toml"
ENTITLEMENTS="$SCRIPT_DIR/Entitlements.plist"
APP_BUNDLE_FINGERPRINT="$SCRIPT_DIR/app-bundle-fingerprint-v1.rb"
FINDER_BOOKMARK_RESOLVER="$SCRIPT_DIR/resolve-finder-bookmark-v1.swift"
RELEASE_APPROVAL_VERIFIER="$REPO_ROOT/scripts/hepta-ui-release-execution-approval-verifier-v1"
RELEASE_APPROVAL_TRUST_POLICY="$SCRIPT_DIR/release-execution-approval-trust-v1.json"

[[ "$(uname -s)" == "Darwin" ]] || { echo "macOS release packaging requires Darwin" >&2; exit 2; }
for command in awk codesign ditto find hdiutil jq mount plutil readlink ruby security shasum spctl swift swiftc xattr xcrun; do
  command -v "$command" >/dev/null 2>&1 || { echo "$command is required" >&2; exit 2; }
done
PRODUCT_VERSION="$(awk -F '"' '/^version[[:space:]]*=/ { print $2; exit }' "$CARGO_TOML")"
case "$(uname -m)" in
  arm64) PACKAGER_ARCH=aarch64 ;;
  x86_64) PACKAGER_ARCH=x86_64 ;;
  *) PACKAGER_ARCH="$(uname -m)" ;;
esac
if [[ -z "$OUTPUT_PATH" ]]; then
  OUTPUT_PATH="$PROJECT_DIR/dist/Hepta_${PRODUCT_VERSION}_${PACKAGER_ARCH}.dmg"
fi
[[ "$OUTPUT_PATH" == /* ]] || OUTPUT_PATH="$PROJECT_DIR/$OUTPUT_PATH"
if [[ -z "$RECEIPT_PATH" ]]; then RECEIPT_PATH="$OUTPUT_PATH.receipt.json"; fi
[[ "$RECEIPT_PATH" == /* ]] || RECEIPT_PATH="$PROJECT_DIR/$RECEIPT_PATH"
EVIDENCE_DIR="${HEPTA_NATIVE_RELEASE_EVIDENCE_DIR:-${RECEIPT_PATH%.json}.evidence}"
[[ "$EVIDENCE_DIR" == /* ]] || EVIDENCE_DIR="$PROJECT_DIR/$EVIDENCE_DIR"
normalize_path() {
  ruby -e '
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
  ruby -e '
    path = File.expand_path(ARGV.fetch(0))
    root = File.expand_path(ARGV.fetch(1))
    exit(path.start_with?(root + File::SEPARATOR) ? 0 : 1)
  ' "$1" "$2"
}
paths_overlap() {
  [[ "$1" == "$2" ]] || path_is_within "$1" "$2" || path_is_within "$2" "$1"
}
assert_release_inputs_disjoint() {
  local input_path target_path
  for input_path in \
    "$APP_PATH" \
    "$APP_RECEIPT_PATH" \
    "$RELEASE_APPROVAL_PATH" \
    "$RELEASE_APPROVAL_SIGNATURE_PATH" \
    "$RELEASE_APPROVAL_PUBLIC_KEY_PATH"; do
    [[ -n "$input_path" ]] || continue
    for target_path in "$OUTPUT_PATH" "$RECEIPT_PATH" "$EVIDENCE_DIR"; do
      if paths_overlap "$input_path" "$target_path"; then
        release_fail "release_input_output_path_overlap" \
          "formal app/app-receipt must not equal, contain, or be contained by output, receipt, or evidence paths" 64
      fi
    done
  done
  if [[ -n "$APP_PATH" && -n "$APP_RECEIPT_PATH" ]] && paths_overlap "$APP_PATH" "$APP_RECEIPT_PATH"; then
    release_fail "release_input_paths_overlap" "formal app and app-receipt paths must not overlap" 64
  fi
  local -a approval_inputs=("$RELEASE_APPROVAL_PATH" "$RELEASE_APPROVAL_SIGNATURE_PATH" "$RELEASE_APPROVAL_PUBLIC_KEY_PATH")
  local left_index right_index
  for ((left_index=0; left_index<${#approval_inputs[@]}; left_index+=1)); do
    [[ -n "${approval_inputs[$left_index]}" ]] || continue
    for ((right_index=left_index+1; right_index<${#approval_inputs[@]}; right_index+=1)); do
      [[ -n "${approval_inputs[$right_index]}" ]] || continue
      if paths_overlap "${approval_inputs[$left_index]}" "${approval_inputs[$right_index]}"; then
        release_fail "release_approval_input_paths_overlap" "approval, signature, and public-key inputs must be distinct" 64
      fi
    done
  done
}
OUTPUT_PATH="$(normalize_path "$OUTPUT_PATH")"
RECEIPT_PATH="$(normalize_path "$RECEIPT_PATH")"
EVIDENCE_DIR="$(normalize_path "$EVIDENCE_DIR")"
if [[ "$OUTPUT_PATH" == "$RECEIPT_PATH" || "$OUTPUT_PATH" == "$EVIDENCE_DIR" || "$RECEIPT_PATH" == "$EVIDENCE_DIR" ]] \
  || path_is_within "$OUTPUT_PATH" "$EVIDENCE_DIR" \
  || path_is_within "$RECEIPT_PATH" "$EVIDENCE_DIR" \
  || path_is_within "$EVIDENCE_DIR" "$OUTPUT_PATH" \
  || path_is_within "$EVIDENCE_DIR" "$RECEIPT_PATH"; then
  echo "output, receipt, and evidence paths must not overlap" >&2
  exit 64
fi

# Resolve caller-controlled release inputs before creating any output or
# evidence directory.  This early pass deliberately emits only stderr: a
# structured failure receipt cannot be written safely until the receipt path
# has been proven disjoint from both inputs.
if [[ -n "$APP_PATH" ]]; then
  [[ "$APP_PATH" == /* ]] || APP_PATH="$PROJECT_DIR/$APP_PATH"
  APP_PATH="$(normalize_path "$APP_PATH")"
  [[ -d "$APP_PATH" ]] || { echo "formal app not found: $APP_PATH" >&2; exit 2; }
fi
if [[ -n "$APP_RECEIPT_PATH" ]]; then
  [[ "$APP_RECEIPT_PATH" == /* ]] || APP_RECEIPT_PATH="$PROJECT_DIR/$APP_RECEIPT_PATH"
  APP_RECEIPT_PATH="$(normalize_path "$APP_RECEIPT_PATH")"
  [[ -s "$APP_RECEIPT_PATH" ]] || { echo "formal unsigned app receipt not found: $APP_RECEIPT_PATH" >&2; exit 2; }
fi
if [[ "$RELEASE_APPROVAL_ARGUMENT_COUNT" == "3" ]]; then
  for approval_path_name in \
    RELEASE_APPROVAL_PATH \
    RELEASE_APPROVAL_SIGNATURE_PATH \
    RELEASE_APPROVAL_PUBLIC_KEY_PATH; do
    approval_input_path="${!approval_path_name}"
    [[ "$approval_input_path" == /* ]] || {
      echo "release approval inputs must use explicit absolute paths" >&2
      exit 64
    }
    canonical_approval_input_path="$(normalize_path "$approval_input_path")"
    [[ "$canonical_approval_input_path" == "$approval_input_path" ]] || {
      echo "release approval input contains a symlinked or non-canonical component: $approval_input_path" >&2
      exit 64
    }
    [[ -f "$approval_input_path" && ! -L "$approval_input_path" && -s "$approval_input_path" ]] || {
      echo "release approval input is not a non-empty regular file: $approval_input_path" >&2
      exit 64
    }
    printf -v "$approval_path_name" '%s' "$canonical_approval_input_path"
  done
fi
for early_input_path in \
  "$APP_PATH" \
  "$APP_RECEIPT_PATH" \
  "$RELEASE_APPROVAL_PATH" \
  "$RELEASE_APPROVAL_SIGNATURE_PATH" \
  "$RELEASE_APPROVAL_PUBLIC_KEY_PATH"; do
  [[ -n "$early_input_path" ]] || continue
  for early_target_path in "$OUTPUT_PATH" "$RECEIPT_PATH" "$EVIDENCE_DIR"; do
    if paths_overlap "$early_input_path" "$early_target_path"; then
      echo "formal app/app-receipt must not overlap release output, receipt, or evidence paths" >&2
      exit 64
    fi
  done
done
if [[ -n "$APP_PATH" && -n "$APP_RECEIPT_PATH" ]] && paths_overlap "$APP_PATH" "$APP_RECEIPT_PATH"; then
  echo "formal app and app-receipt paths must not overlap" >&2
  exit 64
fi
if [[ "$RELEASE_APPROVAL_ARGUMENT_COUNT" == "3" ]]; then
  approval_pair_paths=("$RELEASE_APPROVAL_PATH" "$RELEASE_APPROVAL_SIGNATURE_PATH" "$RELEASE_APPROVAL_PUBLIC_KEY_PATH")
  for ((approval_left=0; approval_left<${#approval_pair_paths[@]}; approval_left+=1)); do
    for ((approval_right=approval_left+1; approval_right<${#approval_pair_paths[@]}; approval_right+=1)); do
      if paths_overlap "${approval_pair_paths[$approval_left]}" "${approval_pair_paths[$approval_right]}"; then
        echo "approval, signature, and public-key inputs must be distinct" >&2
        exit 64
      fi
    done
  done
fi

[[ ! -e "$OUTPUT_PATH" && ! -L "$OUTPUT_PATH" ]] || { echo "refusing to replace existing DMG: $OUTPUT_PATH" >&2; exit 1; }
[[ ! -e "$RECEIPT_PATH" && ! -L "$RECEIPT_PATH" ]] || { echo "refusing to replace existing receipt: $RECEIPT_PATH" >&2; exit 1; }
[[ ! -e "$EVIDENCE_DIR" && ! -L "$EVIDENCE_DIR" ]] || { echo "refusing to replace existing evidence: $EVIDENCE_DIR" >&2; exit 1; }
mkdir -p "$(dirname "$OUTPUT_PATH")" "$(dirname "$RECEIPT_PATH")" "$(dirname "$EVIDENCE_DIR")"
OUTPUT_PARENT="$(cd "$(dirname "$OUTPUT_PATH")" && pwd -P)"
RECEIPT_PARENT="$(cd "$(dirname "$RECEIPT_PATH")" && pwd -P)"
EVIDENCE_PARENT="$(cd "$(dirname "$EVIDENCE_DIR")" && pwd -P)"

WORK_DIR="$(mktemp -d /private/tmp/hepta-signed-release.XXXXXX)"
EVIDENCE_STAGE_DIR="$(mktemp -d "$EVIDENCE_PARENT/.hepta-release-evidence-stage.XXXXXX")"
chmod 700 "$EVIDENCE_STAGE_DIR"
read -r EVIDENCE_STAGE_DEVICE EVIDENCE_STAGE_INODE < <(/usr/bin/stat -f '%d %i' "$EVIDENCE_STAGE_DIR")
MOUNT_POINT=""
MOUNT_DEVICE_IDS=""
OUTPUT_INSTALLED_BY_THIS_RUN=false
OUTPUT_INSTALL_PENDING=false
OUTPUT_INSTALLED_DEVICE=""
OUTPUT_INSTALLED_INODE=""
SUCCESS_RECEIPT_INSTALLED_BY_THIS_RUN=false
SUCCESS_RECEIPT_INSTALL_PENDING=false
SUCCESS_RECEIPT_EXPECTED_DEVICE=""
SUCCESS_RECEIPT_EXPECTED_INODE=""
EVIDENCE_INSTALLED_BY_THIS_RUN=false
EVIDENCE_INSTALL_PENDING=false
EVIDENCE_INSTALLED_DEVICE=""
EVIDENCE_INSTALLED_INODE=""
EVIDENCE_OWNER_TOKEN="$WORK_DIR/evidence-owner-token"
printf '%s\n' "hepta-release-evidence-owner:$$:${RANDOM:-0}" >"$EVIDENCE_OWNER_TOKEN"
chmod 600 "$EVIDENCE_OWNER_TOKEN"
read -r EVIDENCE_OWNER_TOKEN_DEVICE EVIDENCE_OWNER_TOKEN_INODE < <(/usr/bin/stat -f '%d %i' "$EVIDENCE_OWNER_TOKEN")
RELEASE_STAGE="formal_unsigned_input"
failure_receipt_written=false
FAILURE_SIGNAL=""
RELEASE_APPROVAL_VALID=false
APP_SIGNED=false
DMG_SIGNED=false
NOTARY_ATTEMPTED=false
NOTARY_SUBMITTED=false
NOTARY_ACCEPTED=false
NOTARY_EXIT_CODE=-1
NOTARY_SUBMISSION_ID=""
NOTARY_SUBMISSION_STATE="not_attempted"
NOTARY_SUBMISSION_CONFIRMED=false
NOTARY_SUBMISSION_MAY_HAVE_OCCURRED=false
NOTARYTOOL_LOG="$EVIDENCE_STAGE_DIR/notarytool-submit.log"
NOTARYTOOL_LOG_FINAL="$EVIDENCE_DIR/notarytool-submit.log"
NOTARYTOOL_LOG_SHA=""
NOTARYTOOL_LOG_BYTES=0
STAPLED=false
SPCTL_READY=false
DMG_READBACK_READY=false
refresh_notary_evidence() {
  if [[ -f "$NOTARYTOOL_LOG" ]]; then
    NOTARYTOOL_LOG_SHA="$(shasum -a 256 "$NOTARYTOOL_LOG" 2>/dev/null | awk '{print $1}')"
    NOTARYTOOL_LOG_BYTES="$(wc -c <"$NOTARYTOOL_LOG" | tr -d ' ')"
    candidate_id="$(jq -r '.id // empty' "$NOTARYTOOL_LOG" 2>/dev/null || true)"
    if [[ -n "$candidate_id" ]]; then NOTARY_SUBMISSION_ID="$candidate_id"; fi
  fi
}
publish_staged_evidence() {
  local install_identity
  [[ "$EVIDENCE_INSTALLED_BY_THIS_RUN" == "false" ]] || return 0
  EVIDENCE_INSTALL_PENDING=true
  if ! install_identity="$(ruby -e '
    stage, destination, expected_parent, owner = ARGV
    abort "evidence parent changed" unless File.realpath(File.dirname(destination)) == expected_parent
    abort "evidence target already exists" if File.exist?(destination) || File.symlink?(destination)
    stage_stat = File.lstat(stage)
    owner_stat = File.lstat(owner)
    abort "unsafe evidence stage" unless stage_stat.directory? && !stage_stat.symlink?
    abort "unsafe evidence owner token" unless owner_stat.file? && !owner_stat.symlink? && owner_stat.nlink == 1
    entries = Dir.children(stage).sort
    entries.each do |name|
      abort "unsafe evidence name" if name == "." || name == ".." || name.include?(File::SEPARATOR)
      stat = File.lstat(File.join(stage, name))
      abort "unsafe evidence entry" unless stat.file? && !stat.symlink? && stat.nlink == 1
    end
    created = false
    begin
      Dir.mkdir(destination, 0o700)
      created = true
      destination_stat = File.lstat(destination)
      abort "unsafe installed evidence directory" unless destination_stat.directory? && !destination_stat.symlink?
      File.link(owner, File.join(destination, ".hepta-release-owner"))
      entries.each do |name|
        source = File.join(stage, name)
        target = File.join(destination, name)
        source_stat = File.lstat(source)
        File.link(source, target)
        target_stat = File.lstat(target)
        abort "installed evidence identity mismatch" unless target_stat.file? && !target_stat.symlink? && target_stat.dev == source_stat.dev && target_stat.ino == source_stat.ino
      end
      installed_owner = File.lstat(File.join(destination, ".hepta-release-owner"))
      abort "installed evidence owner mismatch" unless installed_owner.dev == owner_stat.dev && installed_owner.ino == owner_stat.ino
      print [destination_stat.dev, destination_stat.ino, entries.length].join(" ")
    rescue Exception
      if created
        begin
          installed_owner = File.lstat(File.join(destination, ".hepta-release-owner"))
          if installed_owner.dev == owner_stat.dev && installed_owner.ino == owner_stat.ino
            Dir.children(destination).each do |name|
              path = File.join(destination, name)
              stat = File.lstat(path)
              File.unlink(path) if stat.file? && !stat.symlink?
            end
            Dir.rmdir(destination)
          end
        rescue Exception
        end
      end
      raise
    end
  ' "$EVIDENCE_STAGE_DIR" "$EVIDENCE_DIR" "$EVIDENCE_PARENT" "$EVIDENCE_OWNER_TOKEN")"; then
    return 1
  fi
  read -r EVIDENCE_INSTALLED_DEVICE EVIDENCE_INSTALLED_INODE EVIDENCE_INSTALLED_FILE_COUNT <<<"$install_identity"
  EVIDENCE_INSTALLED_BY_THIS_RUN=true
  EVIDENCE_INSTALL_PENDING=false
  [[ "$EVIDENCE_INSTALLED_FILE_COUNT" -ge 0 ]]
}
write_failure_receipt() {
  local exit_code="$1" blocker="$2" receipt_parent temporary
  [[ "$failure_receipt_written" == "false" && ! -e "$RECEIPT_PATH" && ! -L "$RECEIPT_PATH" ]] || return 0
  receipt_parent="$(normalize_path "$(dirname "$RECEIPT_PATH")")"
  temporary="$(mktemp "$receipt_parent/.hepta-release-failure.XXXXXX")"
  refresh_notary_evidence
  publish_staged_evidence || return 1
  if [[ -f "$NOTARYTOOL_LOG" ]]; then
    [[ -f "$NOTARYTOOL_LOG_FINAL" && ! -L "$NOTARYTOOL_LOG_FINAL" ]] || return 1
    [[ "$(shasum -a 256 "$NOTARYTOOL_LOG_FINAL" | awk '{print $1}')" == "$NOTARYTOOL_LOG_SHA" ]] || return 1
    [[ "$(wc -c <"$NOTARYTOOL_LOG_FINAL" | tr -d ' ')" == "$NOTARYTOOL_LOG_BYTES" ]] || return 1
  fi
  if ! jq -n \
    --arg stage "$RELEASE_STAGE" --arg blocker "$blocker" --arg signal "$FAILURE_SIGNAL" --arg output "$OUTPUT_PATH" \
    --argjson exit_code "$exit_code" --argjson output_exists "$(if [[ -e "$OUTPUT_PATH" ]]; then echo true; else echo false; fi)" \
    --argjson app_signed "$APP_SIGNED" --argjson dmg_signed "$DMG_SIGNED" \
    --argjson notary_attempted "$NOTARY_ATTEMPTED" --argjson notary_submitted "$NOTARY_SUBMITTED" --argjson notary_accepted "$NOTARY_ACCEPTED" \
    --argjson notary_exit_code "$NOTARY_EXIT_CODE" --arg notary_submission_id "$NOTARY_SUBMISSION_ID" \
    --arg notary_submission_state "$NOTARY_SUBMISSION_STATE" --argjson notary_submission_confirmed "$NOTARY_SUBMISSION_CONFIRMED" \
    --argjson notary_submission_may_have_occurred "$NOTARY_SUBMISSION_MAY_HAVE_OCCURRED" \
    --arg notarytool_log_path "$NOTARYTOOL_LOG_FINAL" --arg notarytool_log_sha "$NOTARYTOOL_LOG_SHA" --argjson notarytool_log_bytes "$NOTARYTOOL_LOG_BYTES" \
    --argjson notarytool_log_present "$(if [[ -f "$NOTARYTOOL_LOG" ]]; then echo true; else echo false; fi)" \
    --argjson stapled "$STAPLED" --argjson spctl_ready "$SPCTL_READY" --argjson dmg_readback_ready "$DMG_READBACK_READY" \
    --argjson release_approval_valid "$RELEASE_APPROVAL_VALID" \
    '{artifact_kind:"signed_notarized_stapled_artifact",artifact_version:3,receipt_contract_version:3,status:"not_ready",release_approval_valid:$release_approval_valid,failure:{stage:$stage,exit_code:$exit_code,blocker:$blocker,signal:(if $signal == "" then null else $signal end)},artifact_evidence:{signed:$app_signed,dmg_signed:$dmg_signed,notarized:$notary_accepted,stapled:$stapled,dmg_stapled:$stapled,app_stapled:false,spctl_ready:$spctl_ready,dmg_readback_ready:$dmg_readback_ready,local_distribution_artifact_written:$output_exists,public_upload_performed:false,signed_artifact_path:$output,notarytool_exit_code:$notary_exit_code,notary_submission_id:(if $notary_submission_id == "" then null else $notary_submission_id end),notary_submission_state:$notary_submission_state,notary_submission_confirmed:$notary_submission_confirmed,notary_submission_may_have_occurred:$notary_submission_may_have_occurred,notarytool_submit_log_path:(if $notarytool_log_present then $notarytool_log_path else null end),notarytool_submit_log_sha256:(if $notarytool_log_present and $notarytool_log_sha != "" then $notarytool_log_sha else null end),notarytool_submit_log_bytes:(if $notarytool_log_present then $notarytool_log_bytes else 0 end),notary_keychain_profile_only:true,direct_apple_id_password_mode_supported:false,credential_environment_scrubbed_before_first_external_command:true},claim_boundary:{release_artifact_claim_ready:false,release_execution_ready:false,public_distribution_claim_ready:false,release_claim_ready:false,live_product_claim_ready:false},side_effects:{network_call_attempted:$notary_attempted,network_call_confirmed:$notary_submitted,network_call_may_have_occurred:$notary_submission_may_have_occurred,notary_submission_attempted:$notary_attempted,notary_submission_confirmed:$notary_submission_confirmed,notary_submission_may_have_occurred:$notary_submission_may_have_occurred,app_signed:$app_signed,app_notarized:$notary_accepted,app_stapled:false,dmg_stapled:$stapled,public_upload_performed:false}}' \
    >"$temporary"; then
    rm -f "$temporary"
    return 1
  fi
  if ! FAILURE_RECEIPT_INSTALL_IDENTITY="$(ruby -rdigest -rjson -e '
    temporary, receipt, expected_parent = ARGV
    abort "receipt parent changed" unless File.realpath(File.dirname(receipt)) == expected_parent
    abort "receipt target already exists" if File.exist?(receipt) || File.symlink?(receipt)
    stat = File.lstat(temporary)
    abort "unsafe failure receipt temporary" unless stat.file? && !stat.symlink? && stat.nlink == 1
    payload = JSON.parse(File.binread(temporary))
    abort "invalid failure receipt" unless payload["status"] == "not_ready" && payload["artifact_version"] == 3
    temporary_sha = Digest::SHA256.file(temporary).hexdigest
    File.link(temporary, receipt)
    installed = File.lstat(receipt)
    abort "installed failure receipt identity mismatch" unless installed.file? && !installed.symlink? && installed.dev == stat.dev && installed.ino == stat.ino
    abort "installed failure receipt hash mismatch" unless Digest::SHA256.file(receipt).hexdigest == temporary_sha
    JSON.parse(File.binread(receipt))
    print [installed.dev, installed.ino, temporary_sha].join(" ")
  ' "$temporary" "$RECEIPT_PATH" "$receipt_parent")"; then
    rm -f "$temporary"
    return 1
  fi
  rm -f "$temporary"
  failure_receipt_written=true
}
remove_unpaired_final_output() {
  if [[ "$SUCCESS_RECEIPT_INSTALL_PENDING" == "true" && "$SUCCESS_RECEIPT_INSTALLED_BY_THIS_RUN" == "false" ]]; then
    ruby -e '
      path, expected_device, expected_inode = ARGV
      stat = File.lstat(path) rescue nil
      exit 0 unless stat
      abort "pending receipt identity changed; refusing cleanup" unless stat.file? && !stat.symlink? && stat.dev.to_s == expected_device && stat.ino.to_s == expected_inode
      File.unlink(path)
    ' "$RECEIPT_PATH" "$SUCCESS_RECEIPT_EXPECTED_DEVICE" "$SUCCESS_RECEIPT_EXPECTED_INODE" || true
    SUCCESS_RECEIPT_INSTALL_PENDING=false
  fi
  if [[ ( "$OUTPUT_INSTALLED_BY_THIS_RUN" == "true" || "$OUTPUT_INSTALL_PENDING" == "true" ) \
    && "$SUCCESS_RECEIPT_INSTALLED_BY_THIS_RUN" == "false" ]]; then
    ruby -e '
      path, expected_device, expected_inode = ARGV
      stat = File.lstat(path) rescue nil
      exit 0 unless stat
      abort "owned output identity changed; refusing cleanup" unless stat.file? && !stat.symlink? && stat.dev.to_s == expected_device && stat.ino.to_s == expected_inode
      File.unlink(path)
    ' "$OUTPUT_PATH" "$OUTPUT_INSTALLED_DEVICE" "$OUTPUT_INSTALLED_INODE" || true
    OUTPUT_INSTALLED_BY_THIS_RUN=false
    OUTPUT_INSTALL_PENDING=false
  fi
  if [[ ( "$EVIDENCE_INSTALLED_BY_THIS_RUN" == "true" || "$EVIDENCE_INSTALL_PENDING" == "true" ) \
    && "$SUCCESS_RECEIPT_INSTALLED_BY_THIS_RUN" == "false" ]]; then
    ruby -e '
      root, owner_device, owner_inode = ARGV
      stat = File.lstat(root) rescue nil
      exit 0 unless stat
      abort "unsafe owned evidence root" unless stat.directory? && !stat.symlink?
      owner = File.lstat(File.join(root, ".hepta-release-owner")) rescue nil
      abort "owned evidence token missing or changed" unless owner && owner.file? && !owner.symlink? && owner.dev.to_s == owner_device && owner.ino.to_s == owner_inode
      Dir.children(root).each do |name|
        path = File.join(root, name)
        child = File.lstat(path)
        abort "unsafe owned evidence child" unless child.file? && !child.symlink?
        File.unlink(path)
      end
      Dir.rmdir(root)
    ' "$EVIDENCE_DIR" "$EVIDENCE_OWNER_TOKEN_DEVICE" "$EVIDENCE_OWNER_TOKEN_INODE" || true
    EVIDENCE_INSTALLED_BY_THIS_RUN=false
    EVIDENCE_INSTALL_PENDING=false
  fi
}
release_fail() {
  local blocker="$1" message="$2" exit_code="${3:-1}"
  trap - ERR
  trap '' INT TERM
  echo "$message" >&2
  remove_unpaired_final_output
  if ! write_failure_receipt "$exit_code" "$blocker"; then
    echo "unable to persist structured release failure receipt/evidence" >&2
  fi
  exit "$exit_code"
}
on_release_error() {
  local exit_code=$?
  trap - ERR
  trap '' INT TERM
  set +e
  remove_unpaired_final_output
  write_failure_receipt "$exit_code" "unexpected_release_command_failure" || true
  exit "$exit_code"
}
on_release_signal() {
  local signal="$1" exit_code="$2"
  trap - ERR INT TERM
  set +e
  FAILURE_SIGNAL="$signal"
  RELEASE_STAGE="terminated"
  if [[ "$NOTARY_ATTEMPTED" == "true" && "$NOTARY_SUBMISSION_CONFIRMED" == "false" ]]; then
    NOTARY_SUBMISSION_STATE="unknown_after_interruption"
    NOTARY_SUBMISSION_MAY_HAVE_OCCURRED=true
  fi
  remove_unpaired_final_output
  write_failure_receipt "$exit_code" "release_terminated" || true
  exit "$exit_code"
}
cleanup() {
  local device detach_failed=false
  if [[ -n "$MOUNT_DEVICE_IDS" ]]; then
    while IFS= read -r device; do
      [[ -n "$device" ]] || continue
      hdiutil detach "$device" -force >/dev/null 2>&1 || detach_failed=true
    done <<<"$MOUNT_DEVICE_IDS"
  fi
  if [[ "$detach_failed" == "true" || ( -z "$MOUNT_DEVICE_IDS" && -n "$MOUNT_POINT" ) ]]; then
    [[ -z "$MOUNT_POINT" ]] || hdiutil detach "$MOUNT_POINT" -force >/dev/null 2>&1 || true
  fi
  ruby -e '
    root, expected_device, expected_inode = ARGV
    stat = File.lstat(root) rescue nil
    exit 0 unless stat
    abort "evidence stage identity changed" unless stat.directory? && !stat.symlink? && stat.dev.to_s == expected_device && stat.ino.to_s == expected_inode
    Dir.children(root).each do |name|
      path = File.join(root, name)
      child = File.lstat(path)
      abort "unsafe evidence stage child" unless child.file? && !child.symlink?
      File.unlink(path)
    end
    Dir.rmdir(root)
  ' "$EVIDENCE_STAGE_DIR" "$EVIDENCE_STAGE_DEVICE" "$EVIDENCE_STAGE_INODE" >/dev/null 2>&1 || true
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT
trap on_release_error ERR
trap 'on_release_signal INT 130' INT
trap 'on_release_signal TERM 143' TERM

[[ -f "$ENTITLEMENTS" && ! -L "$ENTITLEMENTS" ]] \
  || release_fail "entitlements_missing" "entitlements file not found or unsafe: $ENTITLEMENTS" 2
[[ -s "$APP_BUNDLE_FINGERPRINT" ]] || release_fail "bundle_fingerprint_helper_missing" "app bundle fingerprint helper not found: $APP_BUNDLE_FINGERPRINT" 2
[[ -s "$FINDER_BOOKMARK_RESOLVER" ]] || release_fail "finder_bookmark_resolver_missing" "Finder bookmark resolver not found: $FINDER_BOOKMARK_RESOLVER" 2
if [[ "$PREFLIGHT_ONLY" != "1" ]]; then
  [[ -f "$RELEASE_APPROVAL_TRUST_POLICY" && ! -L "$RELEASE_APPROVAL_TRUST_POLICY" ]] \
    || release_fail "release_approval_trust_policy_missing" "fixed release approval trust policy is unavailable" 77
  [[ -x "$RELEASE_APPROVAL_VERIFIER" && ! -L "$RELEASE_APPROVAL_VERIFIER" ]] \
    || release_fail "release_approval_verifier_missing" "independent release approval verifier is unavailable" 77
fi
ruby -c "$APP_BUNDLE_FINGERPRINT" >/dev/null
swiftc -parse "$FINDER_BOOKMARK_RESOLVER"

if [[ -n "$APP_PATH" ]]; then
  [[ "$APP_PATH" == /* ]] || APP_PATH="$PROJECT_DIR/$APP_PATH"
  APP_PATH="$(normalize_path "$APP_PATH")"
  [[ -d "$APP_PATH" ]] || release_fail "formal_app_missing" "formal app not found: $APP_PATH" 2
  [[ -n "$APP_RECEIPT_PATH" ]] || release_fail "formal_app_receipt_required" "--app-receipt is required with --app-path; arbitrary app bundles are not release inputs" 2
fi
if [[ -n "$APP_RECEIPT_PATH" ]]; then
  [[ "$APP_RECEIPT_PATH" == /* ]] || APP_RECEIPT_PATH="$PROJECT_DIR/$APP_RECEIPT_PATH"
  APP_RECEIPT_PATH="$(normalize_path "$APP_RECEIPT_PATH")"
  [[ -s "$APP_RECEIPT_PATH" ]] || release_fail "formal_app_receipt_missing" "formal unsigned app receipt not found: $APP_RECEIPT_PATH" 2
fi
assert_release_inputs_disjoint

SIGNING_IDENTITY="$CAPTURED_SIGNING_IDENTITY"
if [[ -z "$SIGNING_IDENTITY" ]]; then
  SIGNING_IDENTITY="$(awk -F '"' '/^signing_identity[[:space:]]*=/ { print $2; exit }' "$CARGO_TOML")"
fi
[[ -n "$SIGNING_IDENTITY" ]] || release_fail "developer_id_identity_not_configured" "Developer ID signing identity is not configured" 2
EXPECTED_TEAM_ID="$CAPTURED_EXPECTED_TEAM_ID"
[[ "$EXPECTED_TEAM_ID" =~ ^[A-Z0-9]{10}$ ]] \
  || release_fail "expected_team_identifier_missing_or_invalid" "set HEPTA_EXPECTED_TEAM_ID to the trusted 10-character Team ID" 2
if [[ ! "$SIGNING_IDENTITY" =~ ^Developer\ ID\ Application:\ .+\ \(([A-Z0-9]{10})\)$ \
  || "${BASH_REMATCH[1]:-}" != "$EXPECTED_TEAM_ID" ]]; then
  release_fail "developer_id_identity_invalid_or_team_mismatch" "Developer ID identity must be exact and end in the trusted Team ID" 2
fi
if ! INSTALLED_SIGNING_IDENTITIES="$(/usr/bin/security find-identity -v -p codesigning 2>&1)"; then
  release_fail "developer_id_identity_lookup_failed" "unable to query installed Developer ID signing identities" 2
fi
if ! SIGNING_CERTIFICATE_SHA1="$(/usr/bin/ruby -e '
  expected = ARGV.fetch(0)
  matches = STDIN.each_line.map do |line|
    match = line.match(/^\s*\d+\)\s+([0-9A-Fa-f]{40})\s+"([^"]+)"\s*$/)
    match[1].downcase if match && match[2] == expected
  end.compact
  abort "identity must resolve uniquely" unless matches.length == 1
  print matches.fetch(0)
' "$SIGNING_IDENTITY" <<<"$INSTALLED_SIGNING_IDENTITIES")"; then
  release_fail "developer_id_identity_not_installed" "configured Developer ID signing identity is not installed: $SIGNING_IDENTITY" 2
fi

NOTARY_PROFILE="$CAPTURED_NOTARY_PROFILE"
if [[ "$CAPTURED_DIRECT_APPLE_CREDENTIALS_PRESENT" == true ]]; then
  release_fail "direct_apple_id_password_notary_mode_unsupported" "unset APPLE_ID, APPLE_PASSWORD, and APPLE_TEAM_ID; release notarization is keychain-profile-only" 2
fi
if [[ -z "$NOTARY_PROFILE" ]]; then
  release_fail "notary_keychain_profile_required" "set HEPTA_NOTARY_PROFILE; direct APPLE_ID/APPLE_PASSWORD credentials are rejected because argv is observable" 2
fi

if [[ "$PREFLIGHT_ONLY" == "1" ]]; then
  RELEASE_APPROVAL_VERIFIER_AVAILABLE=false
  if [[ -f "$RELEASE_APPROVAL_VERIFIER" && ! -L "$RELEASE_APPROVAL_VERIFIER" && -x "$RELEASE_APPROVAL_VERIFIER" ]]; then
    RELEASE_APPROVAL_VERIFIER_AVAILABLE=true
  fi
  RELEASE_APPROVAL_TRUST_CONFIGURED=false
  if [[ -f "$RELEASE_APPROVAL_TRUST_POLICY" && ! -L "$RELEASE_APPROVAL_TRUST_POLICY" ]] \
    && jq -e '
      keys | sort == ["kind","minimum_rsa_bits","public_key_sha256","schema_version","signature_algorithm","signer_id","status"]
    ' "$RELEASE_APPROVAL_TRUST_POLICY" >/dev/null 2>&1 \
    && jq -e '
      .schema_version == 1
      and .kind == "hepta-ui-release-execution-approval-trust-v1"
      and .status == "ready"
      and (.signer_id | type == "string" and length > 0)
      and (.public_key_sha256 | type == "string" and test("^[0-9a-f]{64}$"))
      and .signature_algorithm == "rsa-pkcs1-sha256"
      and (.minimum_rsa_bits | type == "number" and . >= 3072)
    ' "$RELEASE_APPROVAL_TRUST_POLICY" >/dev/null 2>&1; then
    RELEASE_APPROVAL_TRUST_CONFIGURED=true
  fi
  jq -n \
    --arg identity "$SIGNING_IDENTITY" \
    --arg signing_certificate_sha1 "$SIGNING_CERTIFICATE_SHA1" \
    --arg expected_team_id "$EXPECTED_TEAM_ID" \
    --arg notary_mode "keychain_profile" \
    --arg app_path "$APP_PATH" \
    --arg app_receipt_path "$APP_RECEIPT_PATH" \
    --arg output "$OUTPUT_PATH" \
    --argjson approval_inputs_supplied "$(if [[ "$RELEASE_APPROVAL_ARGUMENT_COUNT" == "3" ]]; then echo true; else echo false; fi)" \
    --argjson verifier_available "$RELEASE_APPROVAL_VERIFIER_AVAILABLE" \
    --argjson approval_trust_configured "$RELEASE_APPROVAL_TRUST_CONFIGURED" \
    '{status:"ready",preflight_scope:"tools_credentials_and_path_shape_only",signing_identity:$identity,signing_certificate_sha1:$signing_certificate_sha1,expected_team_identifier:$expected_team_id,notary_mode:$notary_mode,notary_keychain_profile_only:true,direct_apple_id_password_mode_supported:false,credential_environment_scrubbed_before_first_external_command:true,independent_approval_verifier_ready:$verifier_available,release_execution_approval_verifier_available:$verifier_available,release_approval_trust_configured:$approval_trust_configured,release_approval_inputs_supplied:$approval_inputs_supplied,release_approval_verified:false,release_execution_ready:false,actual_release_execution_supported:false,conditional_release_execution_supported:($verifier_available and $approval_trust_configured),blockers:(["signed_release_approval_not_verified_in_preflight"] + (if $verifier_available then [] else ["independent_release_approval_verifier_unavailable"] end) + (if $approval_trust_configured then [] else ["release_approval_trust_not_configured"] end) + (if $approval_inputs_supplied then [] else ["signed_release_approval_inputs_not_supplied"] end)),app_path:(if $app_path=="" then null else $app_path end),app_receipt_path:(if $app_receipt_path=="" then null else $app_receipt_path end),output:$output,canonical_input_verified:false,consumes_canonical_current_package:false,receipt_json_validated:false,exact_app_source_binding_validated:false,builds_second_app:false,publishes:false}'
  exit 0
fi

if [[ -z "$APP_PATH" ]]; then
  # Keep the exact unsigned input beside the release receipt.  The copied
  # current-package receipt contains this absolute artifact path, so staging
  # under WORK_DIR would leave a dangling source-evidence reference after the
  # release command's cleanup trap runs.
  FORMAL_STAGE="${RECEIPT_PATH}.unsigned-app"
  FORMAL_STAGE="$(normalize_path "$FORMAL_STAGE")"
  [[ ! -e "$FORMAL_STAGE" ]] || release_fail "formal_unsigned_stage_exists" "refusing to replace formal unsigned app stage: $FORMAL_STAGE" 2
  if paths_overlap "$FORMAL_STAGE" "$OUTPUT_PATH" \
    || paths_overlap "$FORMAL_STAGE" "$RECEIPT_PATH" \
    || paths_overlap "$FORMAL_STAGE" "$EVIDENCE_DIR"; then
    release_fail "formal_unsigned_stage_path_overlap" "formal unsigned app stage must not overlap release outputs" 64
  fi
  FORMAL_TARGET="$WORK_DIR/formal-target"
  FORMAL_RECEIPT="$WORK_DIR/formal-unsigned-package-receipt.json"
  FORMAL_ARGS=(--build --no-launch --output "$FORMAL_RECEIPT" --stage-dir "$FORMAL_STAGE" --target-dir "$FORMAL_TARGET")
  if [[ "$BOOTSTRAP_TOOLS" == "1" ]]; then FORMAL_ARGS+=(--bootstrap-tools); fi
  /usr/bin/env \
    -u APPLE_ID -u APPLE_PASSWORD -u APPLE_TEAM_ID \
    -u HEPTA_NOTARY_PROFILE -u HEPTA_NATIVE_NOTARYTOOL_PROFILE \
    PATH="$PATH" HOME="$HOME" TMPDIR="$TMPDIR" \
    "$REPO_ROOT/scripts/hepta-native-current-package-gate.sh" "${FORMAL_ARGS[@]}" >/dev/null
  APP_RECEIPT_PATH="$FORMAL_RECEIPT"
fi

PRIVATE_APP_RECEIPT="$WORK_DIR/input-current-package-receipt.json"
cp "$APP_RECEIPT_PATH" "$PRIVATE_APP_RECEIPT"
if ! jq empty "$PRIVATE_APP_RECEIPT" >/dev/null 2>&1; then
  release_fail "formal_app_receipt_invalid_json" "formal unsigned app receipt is not valid JSON" 2
fi
if [[ -z "$APP_PATH" ]]; then
  APP_PATH="$(jq -r '.artifact.path // empty' "$PRIVATE_APP_RECEIPT")"
  [[ "$APP_PATH" == /* ]] || release_fail "formal_app_path_not_absolute" "formal package receipt app path is not absolute" 2
  APP_PATH="$(normalize_path "$APP_PATH")"
fi
assert_release_inputs_disjoint

[[ -f "$APP_PATH/Contents/Info.plist" ]] || release_fail "formal_app_info_plist_missing" "formal app Info.plist is missing" 2
[[ -x "$APP_PATH/Contents/MacOS/hepta-native" ]] || release_fail "formal_app_executable_missing" "formal app executable is missing" 2

SOURCE_APP_PATH="$APP_PATH"
SOURCE_BINARY_SHA="$(shasum -a 256 "$SOURCE_APP_PATH/Contents/MacOS/hepta-native" | awk '{print $1}')"
SOURCE_APP_FINGERPRINT="$(ruby "$APP_BUNDLE_FINGERPRINT" "$SOURCE_APP_PATH")"
if [[ "$(jq -r '.symlinks_rejected and .supported_entry_types_only' <<<"$SOURCE_APP_FINGERPRINT")" != "true" ]]; then
  release_fail "formal_app_contains_symlinks_or_unsupported_entries" "formal unsigned app contains rejected symlinks or unsupported filesystem entries"
fi
CURRENT_SOURCE_BINDING="$($REPO_ROOT/scripts/hepta-ui-source-fingerprint)"
if ! jq -e \
  --arg app_path "$SOURCE_APP_PATH" \
  --arg binary_sha256 "$SOURCE_BINARY_SHA" \
  --argjson bundle_fingerprint "$SOURCE_APP_FINGERPRINT" \
  --argjson current_binding "$CURRENT_SOURCE_BINDING" \
  '
    .schema_version == 1
    and .kind == "hepta-native-current-package-gate"
    and .status == "ready"
    and .local_package_ready == true
    and .signed == false
    and .notarized == false
    and .stapled == false
    and .artifact.path == $app_path
    and .artifact.binary_sha256 == $binary_sha256
    and .artifact.bundle_fingerprint == $bundle_fingerprint
    and .artifact.full_head_embedded == true
    and .artifact.developer_id_signed == false
    and .source_binding.worktree_clean == true
    and .source_binding.repository_worktree_clean == true
    and .repository_worktree_clean == true
    and .source_stable_during_run == true
    and .source_binding.head == $current_binding.head
    and .source_binding.head_tree == $current_binding.head_tree
    and .source_binding.source_fingerprint == $current_binding.source_fingerprint
    and $current_binding.worktree_clean == true
    and $current_binding.repository_worktree_clean == true
  ' "$PRIVATE_APP_RECEIPT" >/dev/null; then
  release_fail "formal_app_receipt_not_exact_current_source" "formal unsigned app receipt does not bind this exact full app bundle to the current clean stable source"
fi
PERSISTED_UNSIGNED_RECEIPT="$EVIDENCE_STAGE_DIR/formal-unsigned-package-receipt.json"
cp "$PRIVATE_APP_RECEIPT" "$PERSISTED_UNSIGNED_RECEIPT"
UNSIGNED_RECEIPT_SHA="$(shasum -a 256 "$PERSISTED_UNSIGNED_RECEIPT" | awk '{print $1}')"
SOURCE_HEAD="$(jq -r '.source_binding.head' "$PERSISTED_UNSIGNED_RECEIPT")"
SOURCE_TREE="$(jq -r '.source_binding.head_tree' "$PERSISTED_UNSIGNED_RECEIPT")"
SOURCE_FINGERPRINT="$(jq -r '.source_binding.source_fingerprint' "$PERSISTED_UNSIGNED_RECEIPT")"

# Independently verify the detached operator approval only after the exact
# clean source and formal unsigned app have been proven. This remains before
# ditto/codesign/notarytool/stapler and therefore cannot authorize a different
# input or retroactively bless a release action that already occurred.
RELEASE_STAGE="release_approval_verification"
SOURCE_APP_FINGERPRINT_SHA="$(jq -S -c . <<<"$SOURCE_APP_FINGERPRINT" | shasum -a 256 | awk '{print $1}')"
PACKAGING_SCRIPT_SHA="$(shasum -a 256 "$SCRIPT_DIR/build-macos-dmg.sh" | awk '{print $1}')"
RELEASE_APPROVAL_VERIFIER_SHA="$(shasum -a 256 "$RELEASE_APPROVAL_VERIFIER" | awk '{print $1}')"
RELEASE_APPROVAL_TRUST_POLICY_SHA_BEFORE="$(shasum -a 256 "$RELEASE_APPROVAL_TRUST_POLICY" | awk '{print $1}')"
ENTITLEMENTS_SHA="$(shasum -a 256 "$ENTITLEMENTS" | awk '{print $1}')"
SIGNING_ENTITLEMENTS="$WORK_DIR/Entitlements.plist"
/bin/cp "$ENTITLEMENTS" "$SIGNING_ENTITLEMENTS"
/bin/chmod 400 "$SIGNING_ENTITLEMENTS"
[[ "$(shasum -a 256 "$SIGNING_ENTITLEMENTS" | awk '{print $1}')" == "$ENTITLEMENTS_SHA" ]] \
  || release_fail "release_entitlements_private_copy_mismatch" \
    "private signing entitlements do not match the exact approved entitlements" 77
NOTARY_PROFILE_SHA="$(printf '%s' "$NOTARY_PROFILE" | shasum -a 256 | awk '{print $1}')"
RELEASE_APPROVAL_VERIFICATION_TMP="$WORK_DIR/release-approval-verification.json"
if ! "$RELEASE_APPROVAL_VERIFIER" \
  --approval "$RELEASE_APPROVAL_PATH" \
  --signature "$RELEASE_APPROVAL_SIGNATURE_PATH" \
  --public-key "$RELEASE_APPROVAL_PUBLIC_KEY_PATH" \
  --trust-policy "$RELEASE_APPROVAL_TRUST_POLICY" \
  --source-head "$SOURCE_HEAD" \
  --source-tree "$SOURCE_TREE" \
  --source-fingerprint "$SOURCE_FINGERPRINT" \
  --unsigned-app-path "$SOURCE_APP_PATH" \
  --unsigned-app-receipt-path "$APP_RECEIPT_PATH" \
  --unsigned-app-receipt-sha256 "$UNSIGNED_RECEIPT_SHA" \
  --unsigned-app-bundle-fingerprint-sha256 "$SOURCE_APP_FINGERPRINT_SHA" \
  --unsigned-app-binary-sha256 "$SOURCE_BINARY_SHA" \
  --packaging-script-sha256 "$PACKAGING_SCRIPT_SHA" \
  --approval-verifier-sha256 "$RELEASE_APPROVAL_VERIFIER_SHA" \
  --product-version "$PRODUCT_VERSION" \
  --packager-arch "$PACKAGER_ARCH" \
  --output-path "$OUTPUT_PATH" \
  --release-receipt-path "$RECEIPT_PATH" \
  --evidence-dir "$EVIDENCE_DIR" \
  --signing-identity "$SIGNING_IDENTITY" \
  --signing-certificate-sha1 "$SIGNING_CERTIFICATE_SHA1" \
  --team-id "$EXPECTED_TEAM_ID" \
  --entitlements-sha256 "$ENTITLEMENTS_SHA" \
  --notary-profile-sha256 "$NOTARY_PROFILE_SHA" \
  >"$RELEASE_APPROVAL_VERIFICATION_TMP"; then
  rm -f "$RELEASE_APPROVAL_VERIFICATION_TMP"
  release_fail "release_execution_approval_rejected" \
    "signed release execution approval is missing, invalid, stale, untrusted, or bound to a different source/action tuple" 77
fi
if ! jq -e \
  --arg approval "$RELEASE_APPROVAL_PATH" \
  --arg signature "$RELEASE_APPROVAL_SIGNATURE_PATH" \
  --arg key "$RELEASE_APPROVAL_PUBLIC_KEY_PATH" \
  --arg trust_policy "$RELEASE_APPROVAL_TRUST_POLICY" \
  --arg trust_policy_sha "$RELEASE_APPROVAL_TRUST_POLICY_SHA_BEFORE" \
  '
    .schema_version == 1
    and .kind == "hepta-ui-release-execution-approval-verification-v1"
    and .producer == "scripts/hepta-ui-release-execution-approval-verifier-v1"
    and .status == "ready"
    and .approval_valid == true
    and .signature_verified == true
    and .release_execution_approved == true
    and .public_distribution_authorized == false
    and .public_upload_authorized == false
    and .public_upload_performed == false
    and .approval.path == $approval
    and .signature.path == $signature
    and .trust_policy.path == $trust_policy
    and .trust_policy.sha256 == $trust_policy_sha
    and .trust_policy.status == "ready"
    and .approval.signer_id == .trust_policy.signer_id
    and .trusted_public_key.path == $key
    and (.trusted_public_key.sha256 | test("^[0-9a-f]{64}$"))
    and (.verifier_actions | all(. == false))
  ' "$RELEASE_APPROVAL_VERIFICATION_TMP" >/dev/null; then
  rm -f "$RELEASE_APPROVAL_VERIFICATION_TMP"
  release_fail "release_execution_approval_verifier_output_invalid" \
    "independent release approval verifier did not emit its strict ready receipt" 77
fi
CURRENT_SOURCE_BINDING_AFTER_APPROVAL="$($REPO_ROOT/scripts/hepta-ui-source-fingerprint)"
SOURCE_APP_FINGERPRINT_AFTER_APPROVAL="$(ruby "$APP_BUNDLE_FINGERPRINT" "$SOURCE_APP_PATH")"
if [[ "$CURRENT_SOURCE_BINDING_AFTER_APPROVAL" != "$CURRENT_SOURCE_BINDING" \
  || "$(shasum -a 256 "$APP_RECEIPT_PATH" | awk '{print $1}')" != "$UNSIGNED_RECEIPT_SHA" \
  || "$(shasum -a 256 "$SOURCE_APP_PATH/Contents/MacOS/hepta-native" | awk '{print $1}')" != "$SOURCE_BINARY_SHA" \
  || "$SOURCE_APP_FINGERPRINT_AFTER_APPROVAL" != "$SOURCE_APP_FINGERPRINT" \
  || "$(shasum -a 256 "$SCRIPT_DIR/build-macos-dmg.sh" | awk '{print $1}')" != "$PACKAGING_SCRIPT_SHA" \
  || "$(shasum -a 256 "$RELEASE_APPROVAL_VERIFIER" | awk '{print $1}')" != "$RELEASE_APPROVAL_VERIFIER_SHA" \
  || "$(shasum -a 256 "$RELEASE_APPROVAL_TRUST_POLICY" | awk '{print $1}')" != "$RELEASE_APPROVAL_TRUST_POLICY_SHA_BEFORE" \
  || "$(shasum -a 256 "$ENTITLEMENTS" | awk '{print $1}')" != "$ENTITLEMENTS_SHA" ]]; then
  rm -f "$RELEASE_APPROVAL_VERIFICATION_TMP"
  release_fail "release_approval_tuple_changed_during_verification" \
    "source, unsigned input, packaging script, approval verifier, trust policy, or entitlements changed during approval verification" 77
fi
RELEASE_APPROVAL_VERIFICATION="$EVIDENCE_STAGE_DIR/release-approval-verification.json"
cp "$RELEASE_APPROVAL_VERIFICATION_TMP" "$RELEASE_APPROVAL_VERIFICATION"
RELEASE_APPROVAL_VERIFICATION_SHA="$(shasum -a 256 "$RELEASE_APPROVAL_VERIFICATION" | awk '{print $1}')"
RELEASE_APPROVAL_VERIFICATION_JSON="$(cat "$RELEASE_APPROVAL_VERIFICATION")"
RELEASE_APPROVAL_VALID=true

SIGNED_APP="$WORK_DIR/Hepta.app"
SIGNED_DMG="$WORK_DIR/Hepta.signed.dmg"
ditto "$SOURCE_APP_PATH" "$SIGNED_APP"
PRIVATE_COPY_FINGERPRINT="$(ruby "$APP_BUNDLE_FINGERPRINT" "$SIGNED_APP")"
SOURCE_APP_FINGERPRINT_AFTER_COPY="$(ruby "$APP_BUNDLE_FINGERPRINT" "$SOURCE_APP_PATH")"
CURRENT_SOURCE_BINDING_AFTER_COPY="$($REPO_ROOT/scripts/hepta-ui-source-fingerprint)"
if [[ "$PRIVATE_COPY_FINGERPRINT" != "$SOURCE_APP_FINGERPRINT" \
  || "$SOURCE_APP_FINGERPRINT_AFTER_COPY" != "$SOURCE_APP_FINGERPRINT" \
  || "$CURRENT_SOURCE_BINDING_AFTER_COPY" != "$CURRENT_SOURCE_BINDING" ]]; then
  release_fail "release_input_changed_before_signing" "source, formal app bundle, or private signing copy changed before signing"
fi
cmp "$SOURCE_APP_PATH/Contents/MacOS/hepta-native" "$SIGNED_APP/Contents/MacOS/hepta-native"

codesign_with_retry() {
  local target="$1"
  local kind="$2"
  # The approval binds the exact installed certificate fingerprint. Selecting
  # by SHA-1 prevents a same-team/same-label certificate from satisfying an
  # approval that names a different certificate.
  local arguments=(--force --sign "$SIGNING_CERTIFICATE_SHA1" --timestamp)
  if [[ "$kind" == app ]]; then
    arguments+=(--entitlements "$SIGNING_ENTITLEMENTS" --options runtime)
  fi
  local attempt=1 delay=15 log_file="$WORK_DIR/codesign.$(basename "$target").log"
  while (( attempt <= 5 )); do
    if codesign "${arguments[@]}" "$target" >"$log_file" 2>&1; then return 0; fi
    if ! grep -qi timestamp "$log_file" || (( attempt == 5 )); then
      cat "$log_file" >&2
      return 1
    fi
    sleep "$delay"
    delay=$((delay * 2))
    attempt=$((attempt + 1))
  done
}

xattr -cr "$SIGNED_APP"
if [[ "$(shasum -a 256 "$ENTITLEMENTS" | awk '{print $1}')" != "$ENTITLEMENTS_SHA" \
  || "$(shasum -a 256 "$SIGNING_ENTITLEMENTS" | awk '{print $1}')" != "$ENTITLEMENTS_SHA" ]]; then
  release_fail "release_entitlements_changed_before_signing" \
    "approved entitlements changed before the first signing operation" 77
fi
RELEASE_STAGE="codesign_app"
codesign_with_retry "$SIGNED_APP/Contents/MacOS/hepta-native" app
codesign_with_retry "$SIGNED_APP" app
CODESIGN_APP_LOG="$EVIDENCE_STAGE_DIR/codesign-verify-app.log"
codesign --verify --strict --deep --verbose=2 "$SIGNED_APP" >"$CODESIGN_APP_LOG" 2>&1
CODESIGN_APP_DETAILS_LOG="$EVIDENCE_STAGE_DIR/codesign-details-app.log"
codesign -d --verbose=4 "$SIGNED_APP" >"$CODESIGN_APP_DETAILS_LOG" 2>&1
SIGNED_APP_TEAM_ID="$(awk -F= '/^TeamIdentifier=/ {sub(/^[^=]*=/, ""); print; exit}' "$CODESIGN_APP_DETAILS_LOG")"
SIGNED_APP_RUNTIME_VERSION="$(awk -F= '/^Runtime Version=/ {sub(/^[^=]*=/, ""); print; exit}' "$CODESIGN_APP_DETAILS_LOG")"
SIGNED_APP_TIMESTAMP="$(awk -F= '/^Timestamp=/ {sub(/^[^=]*=/, ""); print; exit}' "$CODESIGN_APP_DETAILS_LOG")"
SIGNED_APP_FLAGS="$(awk '/^CodeDirectory / {for (field = 1; field <= NF; field += 1) if ($field ~ /^flags=/) {sub(/^flags=/, "", $field); print $field; exit}}' "$CODESIGN_APP_DETAILS_LOG")"
[[ "$SIGNED_APP_TEAM_ID" == "$EXPECTED_TEAM_ID" ]] || release_fail "signed_app_team_identifier_mismatch" "signed app TeamIdentifier does not match expected Team ID"
[[ -n "$SIGNED_APP_RUNTIME_VERSION" && "$SIGNED_APP_FLAGS" == *runtime* ]] \
  || release_fail "signed_app_hardened_runtime_missing" "signed app does not carry a system-reported hardened runtime signature"
[[ -n "$SIGNED_APP_TIMESTAMP" ]] || release_fail "signed_app_secure_timestamp_missing" "signed app signature has no system-reported secure timestamp"
APP_SIGNED=true
SIGNED_APP_FINGERPRINT="$(ruby "$APP_BUNDLE_FINGERPRINT" "$SIGNED_APP")"
if [[ "$(jq -r '.symlinks_rejected and .supported_entry_types_only' <<<"$SIGNED_APP_FINGERPRINT")" != "true" ]]; then
  release_fail "signed_app_contains_symlinks_or_unsupported_entries" "private signed app contains rejected symlinks or unsupported filesystem entries"
fi
SIGNED_BINARY_SHA="$(shasum -a 256 "$SIGNED_APP/Contents/MacOS/hepta-native" | awk '{print $1}')"
SIGNED_BUNDLE_ID="$(plutil -extract CFBundleIdentifier raw "$SIGNED_APP/Contents/Info.plist" 2>/dev/null || true)"
[[ "$SIGNED_BUNDLE_ID" == "ai.hepta.nativeapp" ]] || release_fail "signed_app_bundle_identifier_mismatch" "signed app bundle identifier is not ai.hepta.nativeapp"

RELEASE_STAGE="create_and_codesign_dmg"
"$SCRIPT_DIR/create-macos-dmg-from-app.sh" \
  --app-path "$SIGNED_APP" \
  --output "$SIGNED_DMG" >/dev/null
codesign_with_retry "$SIGNED_DMG" dmg
CODESIGN_DMG_LOG="$EVIDENCE_STAGE_DIR/codesign-verify-dmg.log"
codesign --verify --strict --verbose=2 "$SIGNED_DMG" >"$CODESIGN_DMG_LOG" 2>&1
CODESIGN_DMG_DETAILS_LOG="$EVIDENCE_STAGE_DIR/codesign-details-dmg.log"
codesign -d --verbose=4 "$SIGNED_DMG" >"$CODESIGN_DMG_DETAILS_LOG" 2>&1
SIGNED_DMG_TEAM_ID="$(awk -F= '/^TeamIdentifier=/ {sub(/^[^=]*=/, ""); print; exit}' "$CODESIGN_DMG_DETAILS_LOG")"
SIGNED_DMG_TIMESTAMP="$(awk -F= '/^Timestamp=/ {sub(/^[^=]*=/, ""); print; exit}' "$CODESIGN_DMG_DETAILS_LOG")"
[[ "$SIGNED_DMG_TEAM_ID" == "$EXPECTED_TEAM_ID" ]] || release_fail "signed_dmg_team_identifier_mismatch" "signed DMG TeamIdentifier does not match expected Team ID"
[[ -n "$SIGNED_DMG_TIMESTAMP" ]] || release_fail "signed_dmg_secure_timestamp_missing" "signed DMG signature has no system-reported secure timestamp"
DMG_SIGNED=true

RELEASE_STAGE="notary_submission"
NOTARY_ATTEMPTED=true
NOTARY_SUBMISSION_MAY_HAVE_OCCURRED=true
NOTARY_SUBMISSION_STATE="attempted_unconfirmed"
trap - ERR
set +e
NOTARY_AUTH_MODE=keychain_profile
/usr/bin/env -i PATH="$SYSTEM_PATH" HOME="$HOME" TMPDIR="$TMPDIR" \
  /usr/bin/xcrun notarytool submit "$SIGNED_DMG" --keychain-profile "$NOTARY_PROFILE" \
  --wait --output-format json >"$NOTARYTOOL_LOG" 2>&1
NOTARY_EXIT_CODE=$?
set -e
trap on_release_error ERR
refresh_notary_evidence
if [[ "$NOTARY_EXIT_CODE" -ne 0 ]]; then
  NOTARY_SUBMISSION_STATE="unknown_after_nonzero_exit"
  cat "$NOTARYTOOL_LOG" >&2
  release_fail "notarytool_nonzero_exit" "notarytool exited non-zero; submission may have occurred and is not confirmed" "$NOTARY_EXIT_CODE"
fi
if [[ -z "$NOTARY_SUBMISSION_ID" ]]; then
  NOTARY_SUBMISSION_STATE="unconfirmed_missing_submission_id"
  cat "$NOTARYTOOL_LOG" >&2
  release_fail "notary_submission_id_missing" "notarytool did not return a submission id"
fi
NOTARY_SUBMITTED=true
NOTARY_SUBMISSION_CONFIRMED=true
if ! jq -e '.status == "Accepted"' "$NOTARYTOOL_LOG" >/dev/null; then
  NOTARY_SUBMISSION_STATE="rejected"
  cat "$NOTARYTOOL_LOG" >&2
  release_fail "notary_submission_not_accepted" "notarytool did not return an Accepted submission receipt"
fi
NOTARY_ACCEPTED=true
NOTARY_SUBMISSION_STATE="accepted"
STAPLER_STAPLE_LOG="$EVIDENCE_STAGE_DIR/stapler-staple.log"
STAPLER_VALIDATE_LOG="$EVIDENCE_STAGE_DIR/stapler-validate.log"
SPCTL_LOG="$EVIDENCE_STAGE_DIR/spctl-assessment.log"
xcrun stapler staple "$SIGNED_DMG" >"$STAPLER_STAPLE_LOG" 2>&1
xcrun stapler validate "$SIGNED_DMG" >"$STAPLER_VALIDATE_LOG" 2>&1
STAPLED=true
RELEASE_STAGE="spctl_assessment"
if ! spctl --assess --type open --context context:primary-signature --verbose "$SIGNED_DMG" >"$SPCTL_LOG" 2>&1; then
  cat "$SPCTL_LOG" >&2
  release_fail "spctl_assessment_failed" "spctl assessment failed"
fi
SPCTL_READY=true

RELEASE_STAGE="readonly_dmg_payload_verification"
DMG_ATTACH_PLIST="$EVIDENCE_STAGE_DIR/dmg-readonly-attach.plist"
DMG_MOUNT_LOG="$EVIDENCE_STAGE_DIR/dmg-readonly-mount.log"
set +e
hdiutil attach -readonly -nobrowse -noautoopen -plist "$SIGNED_DMG" >"$DMG_ATTACH_PLIST"
DMG_ATTACH_STATUS=$?
set -e
MOUNT_DEVICE_IDS="$(grep -Eo '/dev/disk[0-9]+' "$DMG_ATTACH_PLIST" | sort -u || true)"
[[ "$DMG_ATTACH_STATUS" -eq 0 ]] || release_fail "dmg_readonly_attach_failed" "hdiutil attach failed after device evidence capture" "$DMG_ATTACH_STATUS"
MOUNT_POINTS="$(ruby -r rexml/document -e '
  document = REXML::Document.new(File.binread(ARGV.fetch(0)))
  points = REXML::XPath.match(document, "//key").map do |item|
    item.next_element&.text.to_s if item.text == "mount-point"
  end.compact.reject(&:empty?).uniq
  puts points
' "$DMG_ATTACH_PLIST")"
MOUNT_POINT="$(printf '%s\n' "$MOUNT_POINTS" | sed -n '1p')"
MOUNT_POINT_COUNT="$(printf '%s\n' "$MOUNT_POINTS" | awk 'NF {count += 1} END {print count + 0}')"
[[ -n "$MOUNT_DEVICE_IDS" ]] || release_fail "dmg_attached_device_identifier_missing" "hdiutil attach did not return a detachable device identifier"
[[ "$MOUNT_POINT_COUNT" == "1" && -n "$MOUNT_POINT" && -d "$MOUNT_POINT" ]] \
  || release_fail "dmg_readonly_mount_point_missing_or_ambiguous" "hdiutil did not return exactly one mounted volume"
mount | awk -v mount_point="$MOUNT_POINT" 'index($0, " on " mount_point " (") {print; found=1} END {exit(found ? 0 : 1)}' >"$DMG_MOUNT_LOG"
if ! grep -Eq '\(([^)]*,[[:space:]]*)?(read-only|rdonly)(,[^)]*)?\)' "$DMG_MOUNT_LOG"; then
  release_fail "dmg_not_mounted_read_only" "final DMG mount was not read-only"
fi
TOP_LEVEL_APP_COUNT="$(find "$MOUNT_POINT" -mindepth 1 -maxdepth 1 -type d -name '*.app' -print | wc -l | tr -d ' ')"
[[ "$TOP_LEVEL_APP_COUNT" == "1" && -d "$MOUNT_POINT/Hepta.app" ]] || release_fail "dmg_exact_hepta_app_missing" "final DMG does not contain exactly one top-level Hepta.app"
MOUNTED_APP_FINGERPRINT="$(ruby "$APP_BUNDLE_FINGERPRINT" "$MOUNT_POINT/Hepta.app")"
printf '%s\n' "$SIGNED_APP_FINGERPRINT" >"$EVIDENCE_STAGE_DIR/signed-app-bundle-fingerprint.json"
printf '%s\n' "$MOUNTED_APP_FINGERPRINT" >"$EVIDENCE_STAGE_DIR/mounted-app-bundle-fingerprint.json"
MOUNTED_BINARY_SHA="$(shasum -a 256 "$MOUNT_POINT/Hepta.app/Contents/MacOS/hepta-native" | awk '{print $1}')"
MOUNTED_BUNDLE_ID="$(plutil -extract CFBundleIdentifier raw "$MOUNT_POINT/Hepta.app/Contents/Info.plist" 2>/dev/null || true)"
[[ "$MOUNTED_APP_FINGERPRINT" == "$SIGNED_APP_FINGERPRINT" ]] || release_fail "dmg_app_bundle_fingerprint_mismatch" "mounted DMG Hepta.app does not match the exact signed app bundle fingerprint"
[[ "$MOUNTED_BINARY_SHA" == "$SIGNED_BINARY_SHA" ]] || release_fail "dmg_app_binary_sha_mismatch" "mounted DMG Hepta.app binary hash does not match the signed binary"
[[ "$MOUNTED_BUNDLE_ID" == "$SIGNED_BUNDLE_ID" ]] || release_fail "dmg_app_bundle_identifier_mismatch" "mounted DMG Hepta.app bundle identifier does not match the signed app"
APPLICATIONS_ALIAS_KIND=""
APPLICATIONS_ALIAS_TARGET=""
APPLICATIONS_ALIAS_RESOLUTION_PATH="$EVIDENCE_STAGE_DIR/applications-alias-resolution.json"
if [[ ! -f "$MOUNT_POINT/Applications" || -L "$MOUNT_POINT/Applications" ]]; then
  release_fail "dmg_applications_alias_missing" "final DMG does not contain a valid Applications alias"
fi
if ! swift "$FINDER_BOOKMARK_RESOLVER" "$MOUNT_POINT/Applications" >"$APPLICATIONS_ALIAS_RESOLUTION_PATH" 2>"$EVIDENCE_STAGE_DIR/applications-alias-resolution.stderr.log"; then
  release_fail "dmg_applications_alias_unresolvable" "final DMG Applications entry is not a resolvable Finder bookmark alias"
fi
APPLICATIONS_ALIAS_TARGET="$(jq -r '.resolved_target // empty' "$APPLICATIONS_ALIAS_RESOLUTION_PATH")"
if [[ "$APPLICATIONS_ALIAS_TARGET" != "/Applications" || "$(jq -r '.bookmark_data_stale == false' "$APPLICATIONS_ALIAS_RESOLUTION_PATH")" != "true" ]]; then
  release_fail "dmg_applications_alias_wrong_target" "final DMG Applications alias does not resolve exactly to /Applications"
fi
APPLICATIONS_ALIAS_KIND="finder_bookmark_alias"
DMG_READBACK_READY=true
DETACH_FAILED=false
while IFS= read -r attached_device; do
  [[ -n "$attached_device" ]] || continue
  hdiutil detach "$attached_device" >/dev/null 2>&1 || DETACH_FAILED=true
done <<<"$MOUNT_DEVICE_IDS"
[[ "$DETACH_FAILED" == "false" ]] || release_fail "dmg_detach_failed" "final DMG device could not be detached cleanly"
MOUNT_POINT=""
MOUNT_DEVICE_IDS=""

DMG_SHA="$(shasum -a 256 "$SIGNED_DMG" | awk '{print $1}')"
DMG_BYTES="$(wc -c <"$SIGNED_DMG" | tr -d ' ')"
NOTARY_SHA="$(shasum -a 256 "$NOTARYTOOL_LOG" | awk '{print $1}')"
CODESIGN_APP_SHA="$(shasum -a 256 "$CODESIGN_APP_LOG" | awk '{print $1}')"
CODESIGN_DMG_SHA="$(shasum -a 256 "$CODESIGN_DMG_LOG" | awk '{print $1}')"
STAPLER_STAPLE_SHA="$(shasum -a 256 "$STAPLER_STAPLE_LOG" | awk '{print $1}')"
STAPLER_VALIDATE_SHA="$(shasum -a 256 "$STAPLER_VALIDATE_LOG" | awk '{print $1}')"
SPCTL_SHA="$(shasum -a 256 "$SPCTL_LOG" | awk '{print $1}')"
DMG_ATTACH_SHA="$(shasum -a 256 "$DMG_ATTACH_PLIST" | awk '{print $1}')"
DMG_MOUNT_SHA="$(shasum -a 256 "$DMG_MOUNT_LOG" | awk '{print $1}')"
APPLICATIONS_ALIAS_RESOLUTION_SHA="$(shasum -a 256 "$APPLICATIONS_ALIAS_RESOLUTION_PATH" | awk '{print $1}')"
RELEASE_STAGE="final_artifact_placement"
PERSISTED_UNSIGNED_RECEIPT_FINAL="$EVIDENCE_DIR/formal-unsigned-package-receipt.json"
RELEASE_APPROVAL_VERIFICATION_FINAL="$EVIDENCE_DIR/release-approval-verification.json"
APPLICATIONS_ALIAS_RESOLUTION_PATH_FINAL="$EVIDENCE_DIR/applications-alias-resolution.json"
CODESIGN_APP_LOG_FINAL="$EVIDENCE_DIR/codesign-verify-app.log"
CODESIGN_DMG_LOG_FINAL="$EVIDENCE_DIR/codesign-verify-dmg.log"
STAPLER_STAPLE_LOG_FINAL="$EVIDENCE_DIR/stapler-staple.log"
STAPLER_VALIDATE_LOG_FINAL="$EVIDENCE_DIR/stapler-validate.log"
SPCTL_LOG_FINAL="$EVIDENCE_DIR/spctl-assessment.log"
DMG_ATTACH_PLIST_FINAL="$EVIDENCE_DIR/dmg-readonly-attach.plist"
DMG_MOUNT_LOG_FINAL="$EVIDENCE_DIR/dmg-readonly-mount.log"
jq -n \
  --arg source_app "$SOURCE_APP_PATH" \
  --arg source_binary_sha256 "$SOURCE_BINARY_SHA" \
  --arg signed_binary_sha256 "$SIGNED_BINARY_SHA" \
  --argjson source_app_bundle_fingerprint "$SOURCE_APP_FINGERPRINT" \
  --argjson signed_app_bundle_fingerprint "$SIGNED_APP_FINGERPRINT" \
  --argjson mounted_app_bundle_fingerprint "$MOUNTED_APP_FINGERPRINT" \
  --arg mounted_binary_sha256 "$MOUNTED_BINARY_SHA" \
  --arg mounted_bundle_identifier "$MOUNTED_BUNDLE_ID" \
  --arg applications_alias_kind "$APPLICATIONS_ALIAS_KIND" \
  --arg applications_alias_resolved_target "$APPLICATIONS_ALIAS_TARGET" \
  --arg applications_alias_resolution_path "$APPLICATIONS_ALIAS_RESOLUTION_PATH_FINAL" \
  --arg applications_alias_resolution_sha256 "$APPLICATIONS_ALIAS_RESOLUTION_SHA" \
  --arg unsigned_package_receipt_path "$PERSISTED_UNSIGNED_RECEIPT_FINAL" \
  --arg unsigned_package_receipt_sha256 "$UNSIGNED_RECEIPT_SHA" \
  --arg source_head "$SOURCE_HEAD" \
  --arg source_tree "$SOURCE_TREE" \
  --arg source_fingerprint "$SOURCE_FINGERPRINT" \
  --arg signed_artifact_path "$OUTPUT_PATH" \
  --arg signed_artifact_sha256 "$DMG_SHA" \
  --argjson signed_artifact_bytes "$DMG_BYTES" \
  --arg notarization_ticket_sha256 "$NOTARY_SHA" \
  --arg codesign_verify_app_sha256 "$CODESIGN_APP_SHA" \
  --arg codesign_verify_dmg_sha256 "$CODESIGN_DMG_SHA" \
  --arg stapler_staple_sha256 "$STAPLER_STAPLE_SHA" \
  --arg stapler_validate_sha256 "$STAPLER_VALIDATE_SHA" \
  --arg spctl_assessment_sha256 "$SPCTL_SHA" \
  --arg dmg_readonly_attach_sha256 "$DMG_ATTACH_SHA" \
  --arg dmg_readonly_mount_sha256 "$DMG_MOUNT_SHA" \
  --arg notarytool_submit_log_path "$NOTARYTOOL_LOG_FINAL" \
  --arg notarytool_submit_log_sha256 "$NOTARYTOOL_LOG_SHA" \
  --argjson notarytool_submit_log_bytes "$NOTARYTOOL_LOG_BYTES" \
  --argjson notarytool_exit_code "$NOTARY_EXIT_CODE" \
  --arg notary_submission_id "$NOTARY_SUBMISSION_ID" \
  --arg notary_submission_state "$NOTARY_SUBMISSION_STATE" \
  --argjson notary_submission_confirmed "$NOTARY_SUBMISSION_CONFIRMED" \
  --argjson notary_submission_may_have_occurred "$NOTARY_SUBMISSION_MAY_HAVE_OCCURRED" \
  --arg codesign_verify_app_log_path "$CODESIGN_APP_LOG_FINAL" \
  --arg codesign_verify_dmg_log_path "$CODESIGN_DMG_LOG_FINAL" \
  --arg stapler_staple_log_path "$STAPLER_STAPLE_LOG_FINAL" \
  --arg stapler_validate_log_path "$STAPLER_VALIDATE_LOG_FINAL" \
  --arg spctl_assessment_log_path "$SPCTL_LOG_FINAL" \
  --arg dmg_readonly_attach_path "$DMG_ATTACH_PLIST_FINAL" \
  --arg dmg_readonly_mount_log_path "$DMG_MOUNT_LOG_FINAL" \
  --arg identity "$SIGNING_IDENTITY" \
  --arg signing_certificate_sha1 "$SIGNING_CERTIFICATE_SHA1" \
  --arg signing_team_identifier "$EXPECTED_TEAM_ID" \
  --arg entitlements_sha256 "$ENTITLEMENTS_SHA" \
  --arg codesign_app_runtime_version "$SIGNED_APP_RUNTIME_VERSION" \
  --arg codesign_app_flags "$SIGNED_APP_FLAGS" \
  --arg codesign_app_timestamp "$SIGNED_APP_TIMESTAMP" \
  --arg codesign_dmg_timestamp "$SIGNED_DMG_TIMESTAMP" \
  --arg notary_auth_mode "$NOTARY_AUTH_MODE" \
  --argjson release_approval_valid "$RELEASE_APPROVAL_VALID" \
  --arg release_approval_verification_path "$RELEASE_APPROVAL_VERIFICATION_FINAL" \
  --arg release_approval_verification_sha256 "$RELEASE_APPROVAL_VERIFICATION_SHA" \
  --argjson release_approval_verification "$RELEASE_APPROVAL_VERIFICATION_JSON" \
  '{
    artifact_kind:"signed_notarized_stapled_artifact",
    artifact_version:3,
    receipt_contract_version:3,
    owner_lane:"release_operator",
    product:"Hepta Native",
    bundle_identifier:"ai.hepta.nativeapp",
    release_approval_valid:$release_approval_valid,
    release_approval:{verification_path:$release_approval_verification_path,verification_sha256:$release_approval_verification_sha256,verification:$release_approval_verification,public_distribution_authorized:false,public_upload_authorized:false,public_upload_performed:false},
    status:"ready",
    source_evidence:{source_app:$source_app,source_binary_sha256:$source_binary_sha256,signed_binary_sha256:$signed_binary_sha256,source_app_bundle_fingerprint:$source_app_bundle_fingerprint,signed_app_bundle_fingerprint:$signed_app_bundle_fingerprint,unsigned_package_receipt_path:$unsigned_package_receipt_path,unsigned_package_receipt_sha256:$unsigned_package_receipt_sha256,source_head:$source_head,source_tree:$source_tree,source_fingerprint:$source_fingerprint,source_worktree_clean:true,source_stable_during_unsigned_package_run:true,private_copy_recomputed_before_signing:true,consumed_exact_formal_app:true,built_second_product_app:false},
    artifact_evidence:{
      signed:true,notarized:true,stapled:true,dmg_stapled:true,app_stapled:false,
      local_distribution_artifact_written:true,
      public_distribution_artifact_written:true,
      public_distribution_artifact_semantics:"local_signed_notarized_stapled_dmg_written_not_public_upload",
      public_upload_performed:false,
      signed_artifact_path:$signed_artifact_path,
      signed_artifact_sha256:$signed_artifact_sha256,
      signed_artifact_bytes:$signed_artifact_bytes,
      notarization_ticket_sha256:$notarization_ticket_sha256,
      codesign_verify_app_sha256:$codesign_verify_app_sha256,
      codesign_verify_dmg_sha256:$codesign_verify_dmg_sha256,
      stapler_staple_sha256:$stapler_staple_sha256,
      stapler_validate_sha256:$stapler_validate_sha256,
      spctl_assessment_sha256:$spctl_assessment_sha256,
      dmg_mounted_read_only:true,
      mounted_app_bundle_fingerprint:$mounted_app_bundle_fingerprint,
      mounted_binary_sha256:$mounted_binary_sha256,
      mounted_bundle_identifier:$mounted_bundle_identifier,
      applications_alias_verified:true,
      applications_alias_kind:$applications_alias_kind,
      applications_alias_resolved_target:$applications_alias_resolved_target,
      applications_alias_resolution_path:$applications_alias_resolution_path,
      applications_alias_resolution_sha256:$applications_alias_resolution_sha256,
      dmg_readonly_attach_sha256:$dmg_readonly_attach_sha256,
      dmg_readonly_mount_sha256:$dmg_readonly_mount_sha256,
      notarytool_submit_log_path:$notarytool_submit_log_path,
      notarytool_submit_log_sha256:$notarytool_submit_log_sha256,
      notarytool_submit_log_bytes:$notarytool_submit_log_bytes,
      notarytool_exit_code:$notarytool_exit_code,
      notary_submission_id:$notary_submission_id,
      notary_submission_state:$notary_submission_state,
      notary_submission_confirmed:$notary_submission_confirmed,
      notary_submission_may_have_occurred:$notary_submission_may_have_occurred,
      codesign_verify_app_log_path:$codesign_verify_app_log_path,
      codesign_verify_dmg_log_path:$codesign_verify_dmg_log_path,
      stapler_staple_log_path:$stapler_staple_log_path,
      stapler_validate_log_path:$stapler_validate_log_path,
      spctl_assessment_log_path:$spctl_assessment_log_path,
      dmg_readonly_attach_path:$dmg_readonly_attach_path,
      dmg_readonly_mount_log_path:$dmg_readonly_mount_log_path,
      signing_identity:$identity,
      signing_certificate_sha1:$signing_certificate_sha1,
      signing_team_identifier:$signing_team_identifier,
      entitlements_sha256:$entitlements_sha256,
      codesign_app_runtime_version:$codesign_app_runtime_version,
      codesign_app_flags:$codesign_app_flags,
      codesign_app_timestamp:$codesign_app_timestamp,
      codesign_dmg_timestamp:$codesign_dmg_timestamp,
      notary_auth_mode:$notary_auth_mode,
      notary_keychain_profile_only:true,
      direct_apple_id_password_mode_supported:false,
      credential_environment_scrubbed_before_first_external_command:true
    },
    claim_boundary:{release_artifact_claim_ready:true,release_execution_ready:true,public_distribution_claim_ready:false,release_claim_ready:false,live_product_claim_ready:false},
    side_effects:{credential_value_captured:false,credential_environment_scrubbed_before_first_external_command:true,keychain_identity_lookup_performed:true,network_call_performed:true,notary_submission_performed:true,app_signed:true,app_notarized:true,app_stapled:false,dmg_stapled:true,local_distribution_artifact_written:true,public_distribution_artifact_written:true,public_upload_performed:false,external_mutation:true}
  }' \
  >"$WORK_DIR/release-success.json"

OUTPUT_INSTALL_TMP="$(mktemp "$OUTPUT_PARENT/.hepta-release-output.XXXXXX")"
/bin/cp "$SIGNED_DMG" "$OUTPUT_INSTALL_TMP"
if [[ "$(shasum -a 256 "$OUTPUT_INSTALL_TMP" | awk '{print $1}')" != "$DMG_SHA" ]]; then
  rm -f "$OUTPUT_INSTALL_TMP"
  release_fail "final_artifact_hash_changed" "staged local DMG hash changed during final placement"
fi
read -r OUTPUT_INSTALLED_DEVICE OUTPUT_INSTALLED_INODE < <(/usr/bin/stat -f '%d %i' "$OUTPUT_INSTALL_TMP")
OUTPUT_INSTALL_PENDING=true
if ! OUTPUT_INSTALL_IDENTITY="$(ruby -rdigest -e '
  temporary, destination, expected_parent = ARGV
  abort "output parent changed" unless File.realpath(File.dirname(destination)) == expected_parent
  abort "output target already exists" if File.exist?(destination) || File.symlink?(destination)
  stat = File.lstat(temporary)
  abort "unsafe output temporary" unless stat.file? && !stat.symlink? && stat.nlink == 1
  File.link(temporary, destination)
  installed = File.lstat(destination)
  abort "installed output identity mismatch" unless installed.file? && !installed.symlink? && installed.dev == stat.dev && installed.ino == stat.ino
  digest = Digest::SHA256.file(destination).hexdigest
  print [installed.dev, installed.ino, digest].join(" ")
' "$OUTPUT_INSTALL_TMP" "$OUTPUT_PATH" "$OUTPUT_PARENT")"; then
  rm -f "$OUTPUT_INSTALL_TMP"
  release_fail "final_artifact_exclusive_install_failed" "refusing to replace a raced or unsafe final DMG target"
fi
read -r OUTPUT_INSTALLED_DEVICE OUTPUT_INSTALLED_INODE OUTPUT_INSTALLED_SHA <<<"$OUTPUT_INSTALL_IDENTITY"
OUTPUT_INSTALLED_BY_THIS_RUN=true
OUTPUT_INSTALL_PENDING=false
[[ "$OUTPUT_INSTALLED_SHA" == "$DMG_SHA" ]] || {
  rm -f "$OUTPUT_INSTALL_TMP"
  release_fail "final_artifact_hash_changed" "installed local DMG hash changed during exclusive placement"
}
rm -f "$OUTPUT_INSTALL_TMP"
if [[ "$(shasum -a 256 "$OUTPUT_PATH" | awk '{print $1}')" != "$DMG_SHA" ]]; then
  release_fail "final_artifact_hash_changed" "published local DMG hash changed during final placement"
fi

if ! publish_staged_evidence; then
  release_fail "release_evidence_exclusive_install_failed" "refusing to publish raced or unsafe release evidence"
fi

SUCCESS_RECEIPT_TMP="$(mktemp "$RECEIPT_PARENT/.hepta-release-success.XXXXXX")"
/bin/cp "$WORK_DIR/release-success.json" "$SUCCESS_RECEIPT_TMP"
read -r SUCCESS_RECEIPT_EXPECTED_DEVICE SUCCESS_RECEIPT_EXPECTED_INODE < <(/usr/bin/stat -f '%d %i' "$SUCCESS_RECEIPT_TMP")
SUCCESS_RECEIPT_INSTALL_PENDING=true
if ! SUCCESS_RECEIPT_INSTALL_IDENTITY="$(ruby -rdigest -rjson -e '
  temporary, destination, expected_parent, output, output_device, output_inode, output_sha, evidence, evidence_device, evidence_inode = ARGV
  abort "receipt parent changed" unless File.realpath(File.dirname(destination)) == expected_parent
  abort "receipt target already exists" if File.exist?(destination) || File.symlink?(destination)
  stat = File.lstat(temporary)
  abort "unsafe success receipt temporary" unless stat.file? && !stat.symlink? && stat.nlink == 1
  output_stat = File.lstat(output)
  abort "installed output identity changed" unless output_stat.file? && !output_stat.symlink? && output_stat.dev.to_s == output_device && output_stat.ino.to_s == output_inode
  abort "installed output hash changed" unless Digest::SHA256.file(output).hexdigest == output_sha
  evidence_stat = File.lstat(evidence)
  abort "installed evidence identity changed" unless evidence_stat.directory? && !evidence_stat.symlink? && evidence_stat.dev.to_s == evidence_device && evidence_stat.ino.to_s == evidence_inode
  receipt = JSON.parse(File.binread(temporary))
  abort "invalid success receipt" unless receipt["status"] == "ready" && receipt["artifact_version"] == 3
  abort "release approval missing from success receipt" unless receipt["release_approval_valid"] == true && receipt.dig("release_approval", "verification", "approval_valid") == true
  abort "release approval/public upload boundary invalid" unless receipt.dig("release_approval", "public_distribution_authorized") == false && receipt.dig("release_approval", "public_upload_authorized") == false && receipt.dig("release_approval", "public_upload_performed") == false
  abort "receipt/output path mismatch" unless receipt.dig("artifact_evidence", "signed_artifact_path") == output
  abort "receipt/output hash mismatch" unless receipt.dig("artifact_evidence", "signed_artifact_sha256") == output_sha
  temporary_sha = Digest::SHA256.file(temporary).hexdigest
  File.link(temporary, destination)
  installed = File.lstat(destination)
  abort "installed receipt identity mismatch" unless installed.file? && !installed.symlink? && installed.dev == stat.dev && installed.ino == stat.ino
  abort "installed receipt hash mismatch" unless Digest::SHA256.file(destination).hexdigest == temporary_sha
  JSON.parse(File.binread(destination))
  print [installed.dev, installed.ino, temporary_sha].join(" ")
' "$SUCCESS_RECEIPT_TMP" "$RECEIPT_PATH" "$RECEIPT_PARENT" "$OUTPUT_PATH" "$OUTPUT_INSTALLED_DEVICE" "$OUTPUT_INSTALLED_INODE" "$DMG_SHA" "$EVIDENCE_DIR" "$EVIDENCE_INSTALLED_DEVICE" "$EVIDENCE_INSTALLED_INODE")"; then
  rm -f "$SUCCESS_RECEIPT_TMP"
  release_fail "release_receipt_exclusive_install_failed" "refusing to replace a raced or unsafe release receipt target"
fi
read -r SUCCESS_RECEIPT_INSTALLED_DEVICE SUCCESS_RECEIPT_INSTALLED_INODE SUCCESS_RECEIPT_INSTALLED_SHA <<<"$SUCCESS_RECEIPT_INSTALL_IDENTITY"
rm -f "$SUCCESS_RECEIPT_TMP"
SUCCESS_RECEIPT_INSTALLED_BY_THIS_RUN=true
SUCCESS_RECEIPT_INSTALL_PENDING=false
failure_receipt_written=true

printf 'DMG: %s\nReceipt: %s\n' "$OUTPUT_PATH" "$RECEIPT_PATH"
