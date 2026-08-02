#!/bin/bash -p
set +x
PS4='+ '
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
PATH="$SYSTEM_PATH"
export PATH

# Create the unsigned DMG container for an already-built Hepta.app.
#
# This script deliberately does not build, sign, notarize, staple, upload, or
# publish anything. The caller owns those release-authority boundaries. Its
# only input is the exact app bundle that should be placed in the DMG.

SCRIPT_DIR="$(cd "$(/usr/bin/dirname "${BASH_SOURCE[0]}")" && pwd -P)"
BACKGROUND_IMAGE="$SCRIPT_DIR/Hepta Native macOS dmg background.png"
FINGERPRINT_HELPER="$SCRIPT_DIR/app-bundle-fingerprint-v1.rb"
FIX_DMG_TOOL="$SCRIPT_DIR/fix-dmg-applications-icon.sh"
APP_PATH=""
OUTPUT_PATH=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --app-path) APP_PATH="${2:-}"; shift 2 ;;
    --output) OUTPUT_PATH="${2:-}"; shift 2 ;;
    --background) BACKGROUND_IMAGE="${2:-}"; shift 2 ;;
    --help|-h)
      cat <<'EOF'
usage: packaging/create-macos-dmg-from-app.sh \
       --app-path /absolute/path/Hepta.app \
       --output /absolute/path/Hepta.dmg \
       [--background /absolute/path/background.png]

Creates an unsigned DMG from the exact supplied app bundle. It never builds,
signs, notarizes, staples, uploads, or publishes the app or DMG.
EOF
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

[[ "$(uname -s)" == "Darwin" ]] || { echo "DMG creation requires Darwin" >&2; exit 2; }
[[ -n "$APP_PATH" ]] || { echo "--app-path is required" >&2; exit 64; }
[[ -n "$OUTPUT_PATH" ]] || { echo "--output is required" >&2; exit 64; }
[[ "$APP_PATH" == /* ]] || { echo "--app-path must be absolute" >&2; exit 64; }
[[ "$OUTPUT_PATH" == /* ]] || { echo "--output must be absolute" >&2; exit 64; }
for command in ditto hdiutil jq ruby shasum; do
  command -v "$command" >/dev/null 2>&1 || { echo "$command is required" >&2; exit 2; }
done

canonical_path() {
  ruby -e '
    cursor = File.expand_path(ARGV.fetch(0))
    suffix = []
    until File.exist?(cursor) || File.symlink?(cursor) || File.dirname(cursor) == cursor
      suffix.unshift(File.basename(cursor))
      cursor = File.dirname(cursor)
    end
    abort "missing path ancestor" unless File.exist?(cursor) || File.symlink?(cursor)
    print File.join(File.realpath(cursor), *suffix)
  ' "$1"
}

normalize_no_symlink() {
  local label="$1" path="$2" absolute canonical
  [[ "$path" == /* ]] || { echo "$label must be absolute" >&2; return 64; }
  absolute="$(ruby -e 'print File.expand_path(ARGV.fetch(0))' "$path")"
  canonical="$(canonical_path "$absolute")"
  [[ "$absolute" == "$canonical" ]] || { echo "$label contains a symlinked component: $path" >&2; return 64; }
  printf '%s' "$canonical"
}

paths_overlap() {
  ruby -e '
    left, right = ARGV.map { |path| File.expand_path(path) }
    overlap = left == right || left.start_with?(right + File::SEPARATOR) || right.start_with?(left + File::SEPARATOR)
    exit(overlap ? 0 : 1)
  ' "$1" "$2"
}

APP_PATH="$(normalize_no_symlink app_path "$APP_PATH")"
OUTPUT_PATH="$(normalize_no_symlink output "$OUTPUT_PATH")"
BACKGROUND_IMAGE="$(normalize_no_symlink background "$BACKGROUND_IMAGE")"
FINGERPRINT_HELPER="$(normalize_no_symlink fingerprint_helper "$FINGERPRINT_HELPER")"
FIX_DMG_TOOL="$(normalize_no_symlink fix_dmg_tool "$FIX_DMG_TOOL")"
OUTPUT_DIR="$(normalize_no_symlink output_parent "$(/usr/bin/dirname "$OUTPUT_PATH")")"

[[ -d "$APP_PATH" && ! -L "$APP_PATH" ]] || { echo "app bundle not found or unsafe: $APP_PATH" >&2; exit 2; }
[[ -f "$APP_PATH/Contents/Info.plist" && ! -L "$APP_PATH/Contents/Info.plist" ]] || { echo "app Info.plist is missing or unsafe" >&2; exit 2; }
[[ -x "$APP_PATH/Contents/MacOS/hepta-native" && ! -L "$APP_PATH/Contents/MacOS/hepta-native" ]] || { echo "app executable is missing or unsafe" >&2; exit 2; }
[[ -f "$BACKGROUND_IMAGE" && ! -L "$BACKGROUND_IMAGE" ]] || { echo "DMG background image is missing or unsafe: $BACKGROUND_IMAGE" >&2; exit 2; }
[[ -f "$FINGERPRINT_HELPER" && ! -L "$FINGERPRINT_HELPER" ]] || { echo "app fingerprint helper is unsafe" >&2; exit 2; }
[[ -x "$FIX_DMG_TOOL" && ! -L "$FIX_DMG_TOOL" ]] || { echo "DMG fixer is unsafe" >&2; exit 2; }
[[ -d "$OUTPUT_DIR" && ! -L "$OUTPUT_DIR" ]] || { echo "output parent must already be a safe directory: $OUTPUT_DIR" >&2; exit 64; }
[[ ! -e "$OUTPUT_PATH" && ! -L "$OUTPUT_PATH" ]] || { echo "refusing to replace existing DMG: $OUTPUT_PATH" >&2; exit 1; }

for protected_path in "$APP_PATH" "$BACKGROUND_IMAGE" "$FINGERPRINT_HELPER" "$FIX_DMG_TOOL" "$SCRIPT_DIR"; do
  if paths_overlap "$OUTPUT_PATH" "$protected_path"; then
    echo "output overlaps protected app/tool input: $protected_path" >&2
    exit 64
  fi
done
for optional_protected in \
  "${HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH:-}" \
  "${HEPTA_NATIVE_RELEASE_EVIDENCE_DIR:-}"; do
  [[ -n "$optional_protected" ]] || continue
  optional_protected="$(normalize_no_symlink protected_release_path "$optional_protected")"
  if paths_overlap "$OUTPUT_PATH" "$optional_protected" || paths_overlap "$APP_PATH" "$optional_protected"; then
    echo "app/output overlaps protected release receipt or evidence path" >&2
    exit 64
  fi
done

APP_FINGERPRINT_BEFORE="$(ruby "$FINGERPRINT_HELPER" "$APP_PATH")"
if [[ "$(jq -r '.symlinks_rejected and .supported_entry_types_only' <<<"$APP_FINGERPRINT_BEFORE")" != true ]]; then
  echo "app bundle contains symlinks or unsupported filesystem entries" >&2
  exit 64
fi

WORK_DIR="$(/usr/bin/mktemp -d /private/tmp/hepta-dmg-from-app.XXXXXX)"
OUTPUT_INSTALL_TMP=""
cleanup() {
  [[ -z "$OUTPUT_INSTALL_TMP" ]] || /bin/rm -f "$OUTPUT_INSTALL_TMP"
  /bin/rm -rf "$WORK_DIR"
}
trap cleanup EXIT INT TERM

for stage_protected in "$APP_PATH" "$OUTPUT_PATH" "$BACKGROUND_IMAGE" "$FINGERPRINT_HELPER" "$FIX_DMG_TOOL"; do
  if paths_overlap "$WORK_DIR" "$stage_protected"; then
    echo "trusted DMG stage overlaps an input, tool, or output path" >&2
    exit 64
  fi
done

PAYLOAD_DIR="$WORK_DIR/payload"
RAW_DMG="$WORK_DIR/Hepta.raw.dmg"
mkdir -p "$PAYLOAD_DIR"
ditto "$APP_PATH" "$PAYLOAD_DIR/Hepta.app"
APP_FINGERPRINT_STAGED="$(ruby "$FINGERPRINT_HELPER" "$PAYLOAD_DIR/Hepta.app")"
APP_FINGERPRINT_AFTER_COPY="$(ruby "$FINGERPRINT_HELPER" "$APP_PATH")"
[[ "$APP_FINGERPRINT_BEFORE" == "$APP_FINGERPRINT_STAGED" \
  && "$APP_FINGERPRINT_BEFORE" == "$APP_FINGERPRINT_AFTER_COPY" ]] || {
  echo "app input changed or was not copied exactly" >&2
  exit 1
}
ln -s /Applications "$PAYLOAD_DIR/Applications"

# Start with a compressed image so the existing Finder-alias/layout fixer can
# perform its single read-write conversion and final recompression.
hdiutil create \
  -volname "Hepta" \
  -srcfolder "$PAYLOAD_DIR" \
  -format UDZO \
  -imagekey zlib-level=9 \
  "$RAW_DMG" >/dev/null

"$FIX_DMG_TOOL" "$RAW_DMG" "$BACKGROUND_IMAGE" >/dev/null
[[ -s "$RAW_DMG" && -f "$RAW_DMG" && ! -L "$RAW_DMG" ]] || { echo "unsafe final staged DMG" >&2; exit 1; }
[[ "$(ruby "$FINGERPRINT_HELPER" "$APP_PATH")" == "$APP_FINGERPRINT_BEFORE" ]] || {
  echo "app input changed during DMG creation" >&2
  exit 1
}

OUTPUT_INSTALL_TMP="$(/usr/bin/mktemp "$OUTPUT_DIR/.hepta-dmg-install.XXXXXX")"
/bin/cp "$RAW_DMG" "$OUTPUT_INSTALL_TMP"
/bin/chmod 0644 "$OUTPUT_INSTALL_TMP"
ruby -e '
  temporary, destination, expected_parent = ARGV
  abort "output parent changed" unless File.realpath(File.dirname(destination)) == expected_parent
  abort "output appeared before atomic install" if File.exist?(destination) || File.symlink?(destination)
  stat = File.lstat(temporary)
  abort "unsafe staged output" unless stat.file? && !stat.symlink?
  File.link(temporary, destination)
' "$OUTPUT_INSTALL_TMP" "$OUTPUT_PATH" "$OUTPUT_DIR"
/bin/rm -f "$OUTPUT_INSTALL_TMP"
OUTPUT_INSTALL_TMP=""

APP_SHA="$(shasum -a 256 "$APP_PATH/Contents/MacOS/hepta-native" | awk '{print $1}')"
DMG_SHA="$(shasum -a 256 "$OUTPUT_PATH" | awk '{print $1}')"
printf 'app=%s\napp_binary_sha256=%s\ndmg=%s\ndmg_sha256=%s\n' \
  "$APP_PATH" "$APP_SHA" "$OUTPUT_PATH" "$DMG_SHA"
