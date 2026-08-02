#!/usr/bin/env bash
set -euo pipefail

# Build the formal, resource-complete macOS application bundle without using
# an Apple identity or contacting Apple's notarization service. This is the
# local-install boundary only; public distribution has a separate gate.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
APP_DIR="$(cd "$SCRIPT_DIR/.." && pwd -P)"

STAGE_DIR=""
TARGET_DIR="${HEPTA_NATIVE_CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/hepta-native-packaging-target}"
TOOLS_DIR="${HEPTA_NATIVE_PACKAGING_TOOLS_DIR:-${TMPDIR:-/tmp}/hepta-native-packaging-tools-v1}"
BOOTSTRAP_TOOLS=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --stage-dir) STAGE_DIR="${2:-}"; shift 2 ;;
    --target-dir) TARGET_DIR="${2:-}"; shift 2 ;;
    --tools-dir) TOOLS_DIR="${2:-}"; shift 2 ;;
    --bootstrap-tools) BOOTSTRAP_TOOLS=1; shift ;;
    --help|-h)
      cat <<'EOF'
usage: packaging/build-macos-unsigned-app.sh --stage-dir directory
       [--target-dir directory] [--tools-dir directory] [--bootstrap-tools]

Creates a cargo-packager/Robius-built Hepta.app with all Makepad and app
resources. The command never signs, notarizes, staples, uploads, or publishes.
With --bootstrap-tools, exact tool versions are installed under --tools-dir.
EOF
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

[[ "$(uname -s)" == "Darwin" ]] || { echo "formal macOS app packaging requires Darwin" >&2; exit 2; }
[[ -n "$STAGE_DIR" ]] || { echo "--stage-dir is required" >&2; exit 64; }

for command in git rustup uname; do
  command -v "$command" >/dev/null 2>&1 || { echo "$command is required" >&2; exit 2; }
done

PACKAGER_VERSION="0.11.8"
ROBIUS_PACKAGING_VERSION="0.3.3"
RUST_TOOLCHAIN="1.95.0"
mkdir -p "$TOOLS_DIR" "$TARGET_DIR" "$STAGE_DIR"

if [[ "$BOOTSTRAP_TOOLS" == "1" ]]; then
  if [[ ! -x "$TOOLS_DIR/bin/cargo-packager" ]]; then
    if ! rustup run "$RUST_TOOLCHAIN" cargo install cargo-packager \
      --version "$PACKAGER_VERSION" --locked --offline --root "$TOOLS_DIR"; then
      rustup run "$RUST_TOOLCHAIN" cargo install cargo-packager \
        --version "$PACKAGER_VERSION" --locked --root "$TOOLS_DIR"
    fi
  fi
  if [[ ! -x "$TOOLS_DIR/bin/robius-packaging-commands" ]]; then
    if ! rustup run "$RUST_TOOLCHAIN" cargo install robius-packaging-commands \
      --version "$ROBIUS_PACKAGING_VERSION" --locked --offline --root "$TOOLS_DIR"; then
      rustup run "$RUST_TOOLCHAIN" cargo install robius-packaging-commands \
        --version "$ROBIUS_PACKAGING_VERSION" --locked --root "$TOOLS_DIR"
    fi
  fi
fi

PACKAGER="$TOOLS_DIR/bin/cargo-packager"
ROBIUS_PACKAGING="$TOOLS_DIR/bin/robius-packaging-commands"
[[ -x "$PACKAGER" ]] || { echo "missing pinned cargo-packager at $PACKAGER" >&2; exit 2; }
[[ -x "$ROBIUS_PACKAGING" ]] || { echo "missing pinned robius-packaging-commands at $ROBIUS_PACKAGING" >&2; exit 2; }
[[ "$($PACKAGER --version)" == "cargo-packager $PACKAGER_VERSION" ]] || {
  echo "unexpected cargo-packager version" >&2
  exit 2
}
installed_tools="$(rustup run "$RUST_TOOLCHAIN" cargo install --list --root "$TOOLS_DIR")"
grep -Fqx "cargo-packager v$PACKAGER_VERSION:" <<<"$installed_tools" || {
  echo "cargo-packager provenance is not pinned in $TOOLS_DIR" >&2
  exit 2
}
grep -Fqx "robius-packaging-commands v$ROBIUS_PACKAGING_VERSION:" <<<"$installed_tools" || {
  echo "robius-packaging-commands provenance is not pinned in $TOOLS_DIR" >&2
  exit 2
}

TOOLCHAIN_CARGO="$(rustup which --toolchain "$RUST_TOOLCHAIN" cargo)"
TOOLCHAIN_BIN="$(dirname "$TOOLCHAIN_CARGO")"
PACKAGER_DIST="$STAGE_DIR/.packager-dist"
APP_BUNDLE="$STAGE_DIR/Hepta.app"
COLLECTED_RESOURCES="$STAGE_DIR/collected-resources"

for output in "$PACKAGER_DIST" "$APP_BUNDLE" "$COLLECTED_RESOURCES"; do
  [[ ! -e "$output" ]] || { echo "refusing to replace existing packaging output: $output" >&2; exit 1; }
done

# The Robius hook resolves ./dist relative to the app crate and reads the
# externally supplied CARGO_TARGET_DIR directly. A temporary dist symlink keeps
# generated package state out of the source tree without disturbing an existing
# developer target directory.
DIST_LINK="$APP_DIR/dist"
[[ ! -e "$DIST_LINK" && ! -L "$DIST_LINK" ]] || {
  echo "refusing to replace existing app dist path: $DIST_LINK" >&2
  exit 1
}

cleanup() {
  if [[ -L "$DIST_LINK" && "$(readlink "$DIST_LINK")" == "$PACKAGER_DIST" ]]; then rm -f "$DIST_LINK"; fi
}
trap cleanup EXIT INT TERM

mkdir -p "$PACKAGER_DIST"
ln -s "$PACKAGER_DIST" "$DIST_LINK"

cd "$APP_DIR"

# The hook performs the release build with MAKEPAD=apple_bundle and
# MAKEPAD_PACKAGE_DIR=. and then collects Makepad/app resources. Network and
# every Apple credential channel are explicitly removed from this process.
# Relink the top-level binary so install_name_tool never encounters a stale
# @executable_path rpath from a prior repeatable gate run.
rm -f "$TARGET_DIR/release/hepta-native"
env \
  -u APPLE_ID -u APPLE_PASSWORD -u APPLE_TEAM_ID \
  -u APPLE_CERTIFICATE -u APPLE_CERTIFICATE_PASSWORD \
  -u AC_API_KEY_ID -u AC_API_ISSUER_ID -u AC_API_KEY \
  PATH="$TOOLS_DIR/bin:$TOOLCHAIN_BIN:/usr/bin:/bin:/usr/sbin:/sbin" \
  CARGO_TARGET_DIR="$TARGET_DIR" \
  CARGO_NET_OFFLINE=true \
  "$PACKAGER" --release --formats app --manifest-path Cargo.toml

[[ -d "$PACKAGER_DIST/Hepta.app" ]] || { echo "cargo-packager did not produce Hepta.app" >&2; exit 1; }
[[ -d "$PACKAGER_DIST/resources" ]] || { echo "Robius did not collect package resources" >&2; exit 1; }

mv "$PACKAGER_DIST/Hepta.app" "$APP_BUNDLE"
mv "$PACKAGER_DIST/resources" "$COLLECTED_RESOURCES"

printf '%s\n' "$APP_BUNDLE"
