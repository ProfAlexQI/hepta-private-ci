#!/bin/bash
#
# Build a fully codesigned, notarized, and stapled macOS DMG for Hepta Native,
# with the Applications-folder icon fix applied.
#
# Why this script exists:
#   1. cargo-packager's built-in DMG output triggers a macOS Tahoe bug
#      where the Applications folder icon is invisible. We have to fix
#      the DMG layout post-build, which invalidates any DMG signature.
#   2. cargo-packager 0.10.1 hard-codes `--timestamp` with no retry, and
#      Apple's timestamp service occasionally returns "A timestamp was
#      expected but was not found." When that happens, cargo-packager
#      dies and a fresh build re-hits the same flaky service.
#   3. cargo-packager's error reporting (shell.rs:86) reads `errno`
#      after a failed subprocess, which is garbage data -- it surfaces
#      as the misleading "File exists (os error 17)".
#
# Strategy: do all codesign and notarization ourselves so we can retry
# on transient timestamp failures. cargo-packager is reduced to building
# the unsigned .app and DMG layout.
#
# Flow:
#   1. Comment out signing_identity in Cargo.toml so cargo-packager
#      skips both codesign and notarize entirely.
#   2. Run `cargo packager --release` with APPLE_* unset. Produces an
#      unsigned .app and unsigned DMG.
#   3. Codesign the standalone .app (binary first, then bundle) with
#      hardened runtime + entitlements + timestamp, retrying on
#      timestamp-service transient failures.
#   4. Apply the Applications-folder icon fix to the DMG.
#   5. Mount the fixed DMG read-write, replace the unsigned .app inside
#      with our signed copy, recompress.
#   6. Codesign the DMG itself (with retry on timestamp failures).
#   7. Submit the DMG to Apple's notary service via xcrun notarytool.
#   8. Staple the notarization ticket and verify with spctl.
#
# Required notarization credentials (none are written into this script):
#   Prefer HEPTA_NATIVE_NOTARYTOOL_PROFILE, a notarytool keychain profile.
#   Or provide the direct Apple credential environment:
#     APPLE_ID        Apple ID email used for notarization
#     APPLE_PASSWORD  App-specific password for that Apple ID
#     APPLE_TEAM_ID   Apple Developer Team ID
#
# Optional machine-readable receipt:
#   HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH=/path/to/release-artifact.json
#   HEPTA_NATIVE_RELEASE_APPROVAL_VALID=1
#
# The Developer ID signing certificate name is read from
# package.metadata.packager.macos.signing_identity in Cargo.toml.
#
# Usage:
#   HEPTA_NATIVE_NOTARYTOOL_PROFILE=hepta ./packaging/build-macos-dmg.sh
#   APPLE_ID=… APPLE_PASSWORD=… APPLE_TEAM_ID=… ./packaging/build-macos-dmg.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_TOML="$PROJECT_DIR/Cargo.toml"
ENTITLEMENTS="$PROJECT_DIR/packaging/Entitlements.plist"
BG_IMAGE="$PROJECT_DIR/packaging/Hepta Native macOS dmg background.png"

cd "$PROJECT_DIR"

# --- Validate required env vars and config files ------------------------------

NOTARY_AUTH_MODE="apple_env"
NOTARY_AUTH_ARGS=()
if [[ -n "${HEPTA_NATIVE_NOTARYTOOL_PROFILE:-}" ]]; then
    NOTARY_AUTH_MODE="keychain_profile"
    NOTARY_AUTH_ARGS=(--keychain-profile "$HEPTA_NATIVE_NOTARYTOOL_PROFILE")
else
    for var in APPLE_ID APPLE_PASSWORD APPLE_TEAM_ID; do
        if [[ -z "${!var:-}" ]]; then
            echo "Error: $var is not set." >&2
            echo "Required notarization auth: HEPTA_NATIVE_NOTARYTOOL_PROFILE or APPLE_ID, APPLE_PASSWORD, APPLE_TEAM_ID" >&2
            exit 1
        fi
    done
    NOTARY_AUTH_ARGS=(--apple-id "$APPLE_ID" --password "$APPLE_PASSWORD" --team-id "$APPLE_TEAM_ID")
fi

if [[ ! -f "$ENTITLEMENTS" ]]; then
    echo "Error: Entitlements file not found at $ENTITLEMENTS" >&2
    exit 1
fi
if [[ ! -f "$BG_IMAGE" ]]; then
    echo "Error: DMG background image not found at $BG_IMAGE" >&2
    exit 1
fi

# Read signing_identity from Cargo.toml. Use [[:space:]] -- BSD sed on macOS
# does not understand \s.
SIGNING_IDENTITY=$(sed -n 's/^signing_identity[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$CARGO_TOML")
if [[ -z "$SIGNING_IDENTITY" ]]; then
    echo "Error: signing_identity not found in Cargo.toml -- required for notarization." >&2
    exit 1
fi

# --- Codesign helper with timestamp-failure retry -----------------------------
#
# Apple's timestamp service occasionally returns "A timestamp was expected
# but was not found." That's transient -- the next attempt, possibly
# minutes later, usually succeeds. We retry with exponential backoff.
#
# kind=app : adds --entitlements + --options runtime (for .app bundles
#            and Mach-O binaries). Required for hardened runtime.
# kind=dmg : timestamp-only (codesigning a DMG file).

codesign_with_retry() {
    local target="$1"
    local kind="$2"
    local cs_args=(--force --sign "$SIGNING_IDENTITY" --timestamp)
    if [[ "$kind" == "app" ]]; then
        cs_args+=(--entitlements "$ENTITLEMENTS" --options runtime)
    fi

    local max_attempts=5
    local attempt=1
    local delay=15
    local logfile
    logfile=$(mktemp)

    while (( attempt <= max_attempts )); do
        if codesign "${cs_args[@]}" "$target" >"$logfile" 2>&1; then
            # Show codesign's stderr lines (e.g. "replacing existing signature")
            # so the user can see what happened.
            cat "$logfile" >&2
            rm -f "$logfile"
            return 0
        fi

        # Codesign exited non-zero. Print what it said.
        cat "$logfile" >&2

        # Anything mentioning "timestamp" in a failure is the Apple
        # timestamp service flaking -- retry. Other failures are real
        # codesign errors and should not retry.
        if grep -qi 'timestamp' "$logfile"; then
            if (( attempt < max_attempts )); then
                echo "  -> Apple timestamp service transient failure; sleeping ${delay}s before retry $((attempt+1))/${max_attempts}..." >&2
                sleep "$delay"
                delay=$(( delay * 2 ))
            fi
            attempt=$(( attempt + 1 ))
        else
            echo "  -> codesign failed with a non-transient error; giving up." >&2
            rm -f "$logfile"
            return 1
        fi
    done

    echo "  -> codesign still failing after ${max_attempts} attempts; Apple's timestamp service is down. Try again later." >&2
    rm -f "$logfile"
    return 1
}

# --- Step 1: Clean prior build artifacts in dist/ -----------------------------

PRODUCT_VERSION=$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$CARGO_TOML" | head -1)
PRODUCT_NAME="Hepta Native"
APP_BUNDLE="$PRODUCT_NAME.app"
BINARY_NAME="hepta-native"
EVIDENCE_DIR="${HEPTA_NATIVE_RELEASE_EVIDENCE_DIR:-$PROJECT_DIR/dist/release-evidence}"
EVIDENCE_STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
NOTARY_LOG="$EVIDENCE_DIR/notarytool-submit-${EVIDENCE_STAMP}.log"
APP_CODESIGN_VERIFY_LOG="$EVIDENCE_DIR/codesign-verify-app-${EVIDENCE_STAMP}.log"
DMG_CODESIGN_VERIFY_LOG="$EVIDENCE_DIR/codesign-verify-dmg-${EVIDENCE_STAMP}.log"
STAPLER_STAPLE_LOG="$EVIDENCE_DIR/stapler-staple-${EVIDENCE_STAMP}.log"
STAPLER_VALIDATE_LOG="$EVIDENCE_DIR/stapler-validate-${EVIDENCE_STAMP}.log"
SPCTL_LOG="$EVIDENCE_DIR/spctl-assess-${EVIDENCE_STAMP}.log"
case "$(uname -m)" in
    arm64)  PACKAGER_ARCH=aarch64 ;;
    x86_64) PACKAGER_ARCH=x86_64 ;;
    *)      PACKAGER_ARCH=$(uname -m) ;;
esac
CANONICAL_DMG="$PROJECT_DIR/dist/${PRODUCT_NAME}_${PRODUCT_VERSION}_${PACKAGER_ARCH}.dmg"

bool_env_enabled() {
    case "${1:-}" in
        1 | true | TRUE | yes | YES | on | ON) echo true ;;
        *) echo false ;;
    esac
}

file_sha256() {
    shasum -a 256 "$1" | awk '{print $1}'
}

file_bytes() {
    wc -c <"$1" | tr -d ' '
}

write_release_artifact_receipt() {
    local receipt_path="${HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH:-}"
    if [[ -z "$receipt_path" ]]; then
        return 0
    fi
    if ! command -v jq >/dev/null 2>&1; then
        echo "Error: jq is required to write HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH." >&2
        exit 1
    fi

    local receipt_dir
    receipt_dir="$(dirname "$receipt_path")"
    mkdir -p "$receipt_dir"

    local release_approval_valid local_distribution_artifact_written public_distribution_artifact_written credential_value_read
    release_approval_valid="$(bool_env_enabled "${HEPTA_NATIVE_RELEASE_APPROVAL_VALID:-}")"
    local_distribution_artifact_written=true
    # Compatibility field for the UI release gates: this means the local signed,
    # notarized, stapled DMG exists. It does not mean a public upload occurred.
    public_distribution_artifact_written=true
    if [[ "$NOTARY_AUTH_MODE" == "apple_env" ]]; then
        credential_value_read=true
    else
        credential_value_read=false
    fi

    jq -n \
      --arg artifact_kind "signed_notarized_stapled_artifact" \
      --arg owner_lane "release_operator" \
      --arg product "$PRODUCT_NAME" \
      --arg bundle_identifier "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$PROJECT_DIR/packaging/Info.plist")" \
      --arg signed_artifact_path "$DMG_FILE" \
      --arg notarytool_submit_log_path "$NOTARY_LOG" \
      --arg app_codesign_verify_log_path "$APP_CODESIGN_VERIFY_LOG" \
      --arg dmg_codesign_verify_log_path "$DMG_CODESIGN_VERIFY_LOG" \
      --arg stapler_staple_log_path "$STAPLER_STAPLE_LOG" \
      --arg stapler_validate_log_path "$STAPLER_VALIDATE_LOG" \
      --arg spctl_assessment_log_path "$SPCTL_LOG" \
      --arg signing_identity "$SIGNING_IDENTITY" \
      --arg notary_auth_mode "$NOTARY_AUTH_MODE" \
      --arg signed_artifact_sha256 "$(file_sha256 "$DMG_FILE")" \
      --arg notarization_ticket_sha256 "$(file_sha256 "$NOTARY_LOG")" \
      --arg app_codesign_verify_sha256 "$(file_sha256 "$APP_CODESIGN_VERIFY_LOG")" \
      --arg dmg_codesign_verify_sha256 "$(file_sha256 "$DMG_CODESIGN_VERIFY_LOG")" \
      --arg stapler_staple_sha256 "$(file_sha256 "$STAPLER_STAPLE_LOG")" \
      --arg stapler_validate_sha256 "$(file_sha256 "$STAPLER_VALIDATE_LOG")" \
      --arg spctl_assessment_sha256 "$(file_sha256 "$SPCTL_LOG")" \
      --argjson artifact_version 1 \
      --argjson signed_artifact_bytes "$(file_bytes "$DMG_FILE")" \
      --argjson release_approval_valid "$release_approval_valid" \
      --argjson local_distribution_artifact_written "$local_distribution_artifact_written" \
      --argjson public_distribution_artifact_written "$public_distribution_artifact_written" \
      --argjson credential_value_read "$credential_value_read" \
      '{
        artifact_kind:$artifact_kind,
        artifact_version:$artifact_version,
        owner_lane:$owner_lane,
        product:$product,
        bundle_identifier:$bundle_identifier,
        release_approval_valid:$release_approval_valid,
        artifact_evidence:{
          signed:true,
          notarized:true,
          stapled:true,
          local_distribution_artifact_written:$local_distribution_artifact_written,
          public_distribution_artifact_written:$public_distribution_artifact_written,
          public_distribution_artifact_semantics:"local_signed_notarized_stapled_dmg_written_not_public_upload",
          signed_artifact_path:$signed_artifact_path,
          signed_artifact_sha256:$signed_artifact_sha256,
          signed_artifact_bytes:$signed_artifact_bytes,
          notarization_ticket_sha256:$notarization_ticket_sha256,
          codesign_verify_app_sha256:$app_codesign_verify_sha256,
          codesign_verify_dmg_sha256:$dmg_codesign_verify_sha256,
          stapler_staple_sha256:$stapler_staple_sha256,
          stapler_validate_sha256:$stapler_validate_sha256,
          spctl_assessment_sha256:$spctl_assessment_sha256,
          notarytool_submit_log_path:$notarytool_submit_log_path,
          codesign_verify_app_log_path:$app_codesign_verify_log_path,
          codesign_verify_dmg_log_path:$dmg_codesign_verify_log_path,
          stapler_staple_log_path:$stapler_staple_log_path,
          stapler_validate_log_path:$stapler_validate_log_path,
          spctl_assessment_log_path:$spctl_assessment_log_path,
          signing_identity:$signing_identity,
          notary_auth_mode:$notary_auth_mode,
          public_upload_performed:false
        },
        claim_boundary:{
          release_artifact_claim_ready:false,
          release_execution_ready:false,
          live_product_claim_ready:false,
          public_distribution_claim_ready:false,
          release_claim_ready:false
        },
        side_effects:{
          filesystem_write:true,
          credential_value_read:$credential_value_read,
          keychain_identity_lookup_performed:true,
          network_call_performed:true,
          notary_submission_performed:true,
          app_signed:true,
          app_notarized:true,
          app_stapled:true,
          local_distribution_artifact_written:$local_distribution_artifact_written,
          public_distribution_artifact_written:$public_distribution_artifact_written,
          public_upload_performed:false,
          external_mutation:true
        }
      }' >"$receipt_path"

    echo "==> Wrote release artifact receipt: $receipt_path"
}

echo "==> Cleaning prior build artifacts in dist/..."
rm -rf "$PROJECT_DIR/dist/$APP_BUNDLE" \
       "$PROJECT_DIR/dist/.cargo-packager" \
       "$CANONICAL_DMG"
mkdir -p "$EVIDENCE_DIR"

# --- Step 2: Run cargo-packager with signing disabled -------------------------
#
# We comment out signing_identity so cargo-packager's codesign + notarize
# block (gated on `signing_identity.as_ref()` in app/mod.rs:134) is
# completely skipped. We also unset APPLE_* so cargo-packager has no
# way to find notarization credentials. Result: unsigned .app + DMG.

sed -i.bak 's/^signing_identity[[:space:]]*=/#&/' "$CARGO_TOML"
trap 'mv "$CARGO_TOML.bak" "$CARGO_TOML" 2>/dev/null && echo "Restored Cargo.toml"' EXIT

TS_MARKER=$(mktemp)

echo "==> Running cargo packager (unsigned: we sign + notarize ourselves with retries)..."
env -u APPLE_ID -u APPLE_PASSWORD -u APPLE_TEAM_ID cargo packager --release

APP_PATH="$PROJECT_DIR/dist/$APP_BUNDLE"
DMG_FILE=$(find "$PROJECT_DIR/dist" -maxdepth 1 -name '*.dmg' -newer "$TS_MARKER" -print -quit)
rm -f "$TS_MARKER"

if [[ -z "$DMG_FILE" || ! -f "$DMG_FILE" ]]; then
    echo "Error: cargo packager did not produce a DMG in dist/" >&2
    exit 1
fi
if [[ ! -d "$APP_PATH" ]]; then
    echo "Error: $APP_PATH not found after cargo packager run." >&2
    exit 1
fi
echo "==> Found unsigned DMG: $DMG_FILE"

# --- Step 3: Codesign the standalone .app -------------------------------------

echo "==> Codesigning $APP_PATH..."
xattr -cr "$APP_PATH"
codesign_with_retry "$APP_PATH/Contents/MacOS/$BINARY_NAME" app
codesign_with_retry "$APP_PATH" app
if codesign --verify --verbose=2 "$APP_PATH" >"$APP_CODESIGN_VERIFY_LOG" 2>&1; then
    cat "$APP_CODESIGN_VERIFY_LOG"
else
    cat "$APP_CODESIGN_VERIFY_LOG" >&2
    echo "Error: app codesign verification failed." >&2
    exit 1
fi

# --- Step 4: Apply Applications-folder icon fix to DMG ------------------------

echo "==> Applying Applications folder icon fix to DMG..."
"$SCRIPT_DIR/fix-dmg-applications-icon.sh" "$DMG_FILE" "$BG_IMAGE"

# --- Step 5: Embed signed .app into DMG ---------------------------------------
#
# The DMG produced by cargo-packager contains the unsigned .app from
# step 2. The icon fix didn't touch the .app inside (only the
# Applications symlink and DMG-level metadata). We mount the fixed DMG
# read-write, ditto the signed .app over the unsigned one (same name,
# so the .DS_Store icon position survives), unmount, recompress.

echo "==> Embedding signed .app into DMG..."
DMG_DIR="$(dirname "$DMG_FILE")"
DMG_BASE="$(basename "$DMG_FILE" .dmg)"
DMG_RW="$DMG_DIR/${DMG_BASE}_signing.dmg"

hdiutil convert "$DMG_FILE" -format UDRW -o "$DMG_RW" >/dev/null
MOUNT_OUTPUT=$(hdiutil attach "$DMG_RW" -readwrite -noverify -noautoopen)
MOUNT_DIR=$(echo "$MOUNT_OUTPUT" | grep -oE '/Volumes/.*' | head -1)
DEV_NAME=$(echo "$MOUNT_OUTPUT" | head -1 | awk '{print $1}')

if [[ -z "$MOUNT_DIR" || -z "$DEV_NAME" ]]; then
    echo "Error: failed to mount $DMG_RW" >&2
    rm -f "$DMG_RW"
    exit 1
fi

cleanup_rw() {
    hdiutil detach "$DEV_NAME" -force >/dev/null 2>&1 || true
    rm -f "$DMG_RW"
}
trap 'cleanup_rw; mv "$CARGO_TOML.bak" "$CARGO_TOML" 2>/dev/null && echo "Restored Cargo.toml"' EXIT

if [[ ! -d "$MOUNT_DIR/$APP_BUNDLE" ]]; then
    echo "Error: $APP_BUNDLE not found inside mounted DMG at $MOUNT_DIR" >&2
    exit 1
fi
rm -rf "$MOUNT_DIR/$APP_BUNDLE"
ditto "$APP_PATH" "$MOUNT_DIR/$APP_BUNDLE"

sync
sleep 2
hdiutil detach "$DEV_NAME" >/dev/null
sleep 1

rm -f "$DMG_FILE"
hdiutil convert "$DMG_RW" -format UDZO -imagekey zlib-level=9 -o "$DMG_FILE" >/dev/null
rm -f "$DMG_RW"

# RW DMG is gone; drop that part of the trap.
trap 'mv "$CARGO_TOML.bak" "$CARGO_TOML" 2>/dev/null && echo "Restored Cargo.toml"' EXIT

# --- Step 6: Codesign the DMG itself ------------------------------------------

echo "==> Codesigning DMG..."
codesign_with_retry "$DMG_FILE" dmg
if codesign --verify --verbose=2 "$DMG_FILE" >"$DMG_CODESIGN_VERIFY_LOG" 2>&1; then
    cat "$DMG_CODESIGN_VERIFY_LOG"
else
    cat "$DMG_CODESIGN_VERIFY_LOG" >&2
    echo "Error: DMG codesign verification failed." >&2
    exit 1
fi

# --- Step 7: Notarize ---------------------------------------------------------
#
# notarytool exits non-zero if the submission ends in any state other
# than "Accepted", so set -e catches a rejection.

echo "==> Submitting DMG for notarization (this can take several minutes)..."
if xcrun notarytool submit "$DMG_FILE" "${NOTARY_AUTH_ARGS[@]}" --wait >"$NOTARY_LOG" 2>&1; then
    cat "$NOTARY_LOG"
else
    cat "$NOTARY_LOG" >&2
    echo "Error: notarytool submission failed." >&2
    exit 1
fi

# --- Step 8: Staple and verify ------------------------------------------------

echo "==> Stapling notarization ticket to DMG..."
if xcrun stapler staple "$DMG_FILE" >"$STAPLER_STAPLE_LOG" 2>&1; then
    cat "$STAPLER_STAPLE_LOG"
else
    cat "$STAPLER_STAPLE_LOG" >&2
    echo "Error: stapler staple failed." >&2
    exit 1
fi
if xcrun stapler validate "$DMG_FILE" >"$STAPLER_VALIDATE_LOG" 2>&1; then
    cat "$STAPLER_VALIDATE_LOG"
else
    cat "$STAPLER_VALIDATE_LOG" >&2
    echo "Error: stapler validate failed." >&2
    exit 1
fi

echo "==> Verifying DMG with spctl..."
if spctl --assess --type open --context context:primary-signature --verbose "$DMG_FILE" >"$SPCTL_LOG" 2>&1; then
    cat "$SPCTL_LOG"
else
    cat "$SPCTL_LOG" >&2
    echo "Error: spctl assessment failed." >&2
    exit 1
fi

write_release_artifact_receipt

echo ""
echo "==> Done!"
echo "    DMG:      $DMG_FILE"
echo "    Identity: $SIGNING_IDENTITY"
