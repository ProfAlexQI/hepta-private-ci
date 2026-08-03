#!/bin/bash -p
set -euo pipefail

unset BASH_ENV ENV CDPATH GLOBIGNORE SHELLOPTS 2>/dev/null || true
export LC_ALL=C

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
MATERIALIZER="$ROOT_DIR/scripts/hepta-native-makepad-accessibility-overlay"
PINNED_REVISION="c4335cee10b22aca768510c9d072b0ca1bba15c8"
SOURCE_PATH="${HEPTA_MAKEPAD_SOURCE:-}"

if [[ -z "$SOURCE_PATH" ]]; then
  while IFS= read -r candidate; do
    if [[ "$(git -C "$candidate" rev-parse HEAD 2>/dev/null || true)" == "$PINNED_REVISION" ]]; then
      SOURCE_PATH="$candidate"
      break
    fi
  done < <(find "${CARGO_HOME:-$HOME/.cargo}/git/checkouts" -mindepth 2 -maxdepth 2 -type d 2>/dev/null | sort)
fi

[[ -n "$SOURCE_PATH" ]] || {
  echo "pinned Makepad checkout not found; set HEPTA_MAKEPAD_SOURCE" >&2
  exit 1
}

tmp_root="$(mktemp -d "${TMPDIR:-/tmp}/hepta-makepad-accessibility-overlay-self-test.XXXXXX")"
trap 'rm -rf "$tmp_root"' EXIT

positive_receipt="$tmp_root/positive.json"
"$MATERIALIZER" \
  --source "$SOURCE_PATH" \
  --output "$tmp_root/positive-overlay" >"$positive_receipt"

jq -e '
  .schema == "hepta.makepad-accessibility-overlay-receipt.v1"
  and .status == "ready"
  and .checks.source_revision_exact == true
  and .checks.source_tree_exact == true
  and .checks.source_preimages_exact == true
  and .checks.patch_hunks_exact == true
  and .checks.patched_paths_exact == true
  and .checks.patched_postimages_exact == true
  and .checks.macos_update_consumer_present == true
  and .checks.ios_update_consumer_present == false
  and .checks.android_update_consumer_present == false
  and .cargo.config_override_known_full_app_source_graph_safe == false
  and .cargo.makepad_platform_compile_requested == false
  and .cargo.makepad_platform_compile_verified == false
  and .cargo.app_overlay_compile_verified == false
  and (.cargo.app_overlay_compile_blocker | contains("duplicate path/git Makepad source graphs"))
  and .checks.global_cargo_cache_mutated == null
  and .readiness.macos_backend_materialized == true
  and .readiness.macos_compile_verified == false
  and .readiness.macos_voiceover_runtime_verified == false
  and .readiness.ios_backend_materialized == false
  and .readiness.android_backend_materialized == false
  and .readiness.full_accessibility_ready == false
' "$positive_receipt" >/dev/null

negative_count=0

expect_failure() {
  local expected_reason="$1"
  shift
  local stdout_path="$tmp_root/negative-$negative_count.stdout"
  local stderr_path="$tmp_root/negative-$negative_count.stderr"
  if "$@" >"$stdout_path" 2>"$stderr_path"; then
    echo "negative case unexpectedly passed: $expected_reason" >&2
    exit 1
  fi
  jq -s -e --arg reason "$expected_reason" '
    length == 1
    and .[0].status == "not_ready"
    and .[0].reason == $reason
    and .[0].readiness.macos_backend_materialized == false
    and .[0].readiness.macos_compile_verified == false
    and .[0].readiness.ios_backend_materialized == false
    and .[0].readiness.android_backend_materialized == false
    and .[0].readiness.full_accessibility_ready == false
  ' "$stderr_path" >/dev/null
  negative_count=$((negative_count + 1))
}

expect_failure \
  "output_path_already_exists" \
  "$MATERIALIZER" --source "$SOURCE_PATH" --output "$tmp_root/positive-overlay"

git clone --quiet --no-hardlinks "$SOURCE_PATH" "$tmp_root/drifted-source"
git -C "$tmp_root/drifted-source" checkout --quiet --detach "$PINNED_REVISION"
perl -0pi -e 's/\[package\]/# drift\n[package]/' "$tmp_root/drifted-source/platform/Cargo.toml"
expect_failure \
  "source_preimage_digest_mismatch:platform/Cargo.toml" \
  "$MATERIALIZER" --source "$tmp_root/drifted-source" --output "$tmp_root/drifted-output"

git -C "$tmp_root/drifted-source" checkout --quiet -- platform/Cargo.toml
git -C "$tmp_root/drifted-source" checkout --quiet --detach HEAD^
expect_failure \
  "source_revision_mismatch" \
  "$MATERIALIZER" --source "$tmp_root/drifted-source" --output "$tmp_root/wrong-revision-output"

jq -n \
  --arg source_path "$(cd "$SOURCE_PATH" && pwd -P)" \
  --arg overlay_path "$tmp_root/positive-overlay" \
  --argjson negative_cases "$negative_count" \
  '{
    schema:"hepta.makepad-accessibility-overlay-self-test.v1",
    status:"ready",
    source_path:$source_path,
    overlay_path:$overlay_path,
    positive_cases:1,
    negative_cases:$negative_cases,
    macos_backend_materialization_ready:true,
    macos_platform_compile_verification_available:true,
    macos_compile_verified:false,
    hepta_app_overlay_compile_verified:false,
    macos_voiceover_runtime_verified:false,
    ios_backend_materialized:false,
    android_backend_materialized:false,
    full_accessibility_ready:false
  }'
