#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
EVIDENCE_BUNDLE_REPORT_PATH="${HEPTA_UI_EVIDENCE_BUNDLE_REPORT_PATH:-$READINESS_DIR/ui-evidence-bundle-gate.json}"
BUNDLE_DIR="${HEPTA_UI_EVIDENCE_BUNDLE_DIR:-$READINESS_DIR/evidence-bundle}"
ARCHIVE_DIR="${HEPTA_UI_EVIDENCE_ARCHIVE_DIR:-$READINESS_DIR/evidence-archive}"
ARCHIVE_PATH="${HEPTA_UI_EVIDENCE_ARCHIVE_PATH:-$ARCHIVE_DIR/hepta-ui-evidence-bundle.tar.gz}"
REPORT_PATH="${HEPTA_UI_EVIDENCE_ARCHIVE_REPORT_PATH:-$READINESS_DIR/ui-evidence-archive-gate.json}"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI evidence archive gate\n' "$1" >&2
    exit 2
  fi
}

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required evidence-archive input: %s\n' "$path" >&2
    exit 1
  fi
}

require_command jq
require_command shasum
require_command tar

require_file "$EVIDENCE_BUNDLE_REPORT_PATH"

if [[ ! -d "$BUNDLE_DIR" ]]; then
  printf 'Missing required evidence bundle directory: %s\n' "$BUNDLE_DIR" >&2
  exit 1
fi

if ! jq -e '.status == "ready" and .evidence_bundle_gate_ready == true' "$EVIDENCE_BUNDLE_REPORT_PATH" >/dev/null; then
  printf 'Evidence bundle report is not ready: %s\n' "$EVIDENCE_BUNDLE_REPORT_PATH" >&2
  exit 1
fi

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-evidence-archive.XXXXXX")"
STAGE_PARENT="$TMP_DIR/stage"
STAGE_ROOT="$STAGE_PARENT/hepta-ui-evidence-review"
EXTRACT_DIR="$TMP_DIR/extract"
EXTRACT_ROOT="$EXTRACT_DIR/hepta-ui-evidence-review"
EXTRACTED_ITEMS_NDJSON="$TMP_DIR/extracted-items.ndjson"
EXTRACTED_ITEMS_JSON="$TMP_DIR/extracted-items.json"
REPORT_TMP="$TMP_DIR/evidence-archive-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

mkdir -p "$STAGE_ROOT" "$ARCHIVE_DIR"
cp -pR "$BUNDLE_DIR" "$STAGE_ROOT/evidence-bundle"
cp -p "$EVIDENCE_BUNDLE_REPORT_PATH" "$STAGE_ROOT/ui-evidence-bundle-gate.json"

cat >"$STAGE_ROOT/README.txt" <<'README'
Hepta UI local evidence review archive.

This archive is written by the UI lane for local review and retention only. It
contains the normalized evidence bundle plus its machine-readable bundle report.
It is not a signed, notarized, stapled, uploaded, or public distribution
artifact.
README

archive_expected_file_count="$(find "$STAGE_ROOT" -type f | wc -l | tr -d ' ')"
bundle_file_count="$(jq -r '.bundle_file_count' "$EVIDENCE_BUNDLE_REPORT_PATH")"

rm -f "$ARCHIVE_PATH"
(cd "$STAGE_PARENT" && tar -czf "$ARCHIVE_PATH" hepta-ui-evidence-review)

archive_bytes="$(wc -c <"$ARCHIVE_PATH" | tr -d ' ')"
archive_sha="$(file_sha256 "$ARCHIVE_PATH")"

mkdir -p "$EXTRACT_DIR"
tar -xzf "$ARCHIVE_PATH" -C "$EXTRACT_DIR"

archive_extracted_file_count="$(find "$EXTRACT_ROOT" -type f | wc -l | tr -d ' ')"
extracted_bundle_file_count="$(find "$EXTRACT_ROOT/evidence-bundle" -type f | wc -l | tr -d ' ')"
bundle_report_json_valid=false
if jq empty "$EXTRACT_ROOT/ui-evidence-bundle-gate.json" >/dev/null 2>&1; then
  bundle_report_json_valid=true
fi

: >"$EXTRACTED_ITEMS_NDJSON"
while IFS= read -r item; do
  relative_path="$(jq -r '.relative_path' <<<"$item")"
  expected_sha="$(jq -r '.bundle_sha256 // ""' <<<"$item")"
  expected_bytes="$(jq -r '.bundle_bytes // 0' <<<"$item")"
  extracted_path="$EXTRACT_ROOT/evidence-bundle/$relative_path"
  present=false
  bytes=0
  sha=""
  if [[ -s "$extracted_path" ]]; then
    present=true
    bytes="$(wc -c <"$extracted_path" | tr -d ' ')"
    sha="$(file_sha256 "$extracted_path")"
  fi

  jq -n \
    --arg relative_path "$relative_path" \
    --arg extracted_path "$extracted_path" \
    --arg expected_sha "$expected_sha" \
    --arg sha "$sha" \
    --argjson expected_bytes "${expected_bytes:-0}" \
    --argjson present "$present" \
    --argjson bytes "$bytes" \
    '{
      relative_path:$relative_path,
      extracted_path:$extracted_path,
      present:$present,
      bytes:$bytes,
      expected_bytes:$expected_bytes,
      bytes_match:($bytes == $expected_bytes),
      sha256:$sha,
      expected_sha256:$expected_sha,
      sha256_match:(
        $present
        and ($sha | test("^[0-9a-f]{64}$"))
        and $sha == $expected_sha
      ),
      ready:(
        $present
        and ($bytes == $expected_bytes)
        and ($sha | test("^[0-9a-f]{64}$"))
        and $sha == $expected_sha
      )
    }' >>"$EXTRACTED_ITEMS_NDJSON"
done < <(jq -c '.bundle_items[]' "$EVIDENCE_BUNDLE_REPORT_PATH")

jq -s '.' "$EXTRACTED_ITEMS_NDJSON" >"$EXTRACTED_ITEMS_JSON"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_evidence_archive_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg evidence_bundle_report_path "$EVIDENCE_BUNDLE_REPORT_PATH" \
  --arg bundle_dir "$BUNDLE_DIR" \
  --arg archive_dir "$ARCHIVE_DIR" \
  --arg archive_path "$ARCHIVE_PATH" \
  --arg archive_sha "$archive_sha" \
  --arg report_path "$REPORT_PATH" \
  --argjson archive_bytes "$archive_bytes" \
  --argjson archive_expected_file_count "$archive_expected_file_count" \
  --argjson archive_extracted_file_count "$archive_extracted_file_count" \
  --argjson extracted_bundle_file_count "$extracted_bundle_file_count" \
  --argjson bundle_file_count "$bundle_file_count" \
  --argjson bundle_report_json_valid "$bundle_report_json_valid" \
  --slurpfile bundle "$EVIDENCE_BUNDLE_REPORT_PATH" \
  --slurpfile extracted "$EXTRACTED_ITEMS_JSON" \
  '
  ($bundle[0]) as $bundle_report
  | ($extracted[0]) as $extracted_items
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(
        if (
          ($archive_sha | test("^[0-9a-f]{64}$"))
          and $archive_bytes > 0
          and $archive_expected_file_count == $archive_extracted_file_count
          and $extracted_bundle_file_count == $bundle_file_count
          and ($extracted_items | length) == $bundle_report.bundle_file_count
          and ($extracted_items | all(.ready == true))
          and $bundle_report_json_valid == true
        ) then "ready" else "failed" end
      ),
      evidence_archive_gate_ready:(
        ($archive_sha | test("^[0-9a-f]{64}$"))
        and $archive_bytes > 0
        and $archive_expected_file_count == $archive_extracted_file_count
        and $extracted_bundle_file_count == $bundle_file_count
        and ($extracted_items | length) == $bundle_report.bundle_file_count
        and ($extracted_items | all(.ready == true))
        and $bundle_report_json_valid == true
      ),
      readiness_dir:$readiness_dir,
      evidence_bundle_report_path:$evidence_bundle_report_path,
      bundle_dir:$bundle_dir,
      archive_dir:$archive_dir,
      archive_path:$archive_path,
      archive_sha256:$archive_sha,
      archive_bytes:$archive_bytes,
      report_path:$report_path,
      expected_archive_file_count:$archive_expected_file_count,
      extracted_archive_file_count:$archive_extracted_file_count,
      expected_bundle_file_count:$bundle_file_count,
      extracted_bundle_file_count:$extracted_bundle_file_count,
      extracted_bundle_item_count:($extracted_items | length),
      all_extracted_items_ready:($extracted_items | all(.ready == true)),
      all_extracted_items_sha256_match:($extracted_items | all(.sha256_match == true)),
      bundle_report_json_valid:$bundle_report_json_valid,
      source_bundle_sha256:$bundle_report.source_demo_evidence_sha256,
      copied_report_count:$bundle_report.copied_report_count,
      copied_screenshot_count:$bundle_report.copied_screenshot_count,
      hard_true_window_required:$bundle_report.hard_true_window_required,
      r33_hard_demo_evidence_ready:$bundle_report.r33_hard_demo_evidence_ready,
      claim_boundary:{
        local_evidence_archive_ready:(
          ($archive_sha | test("^[0-9a-f]{64}$"))
          and $archive_bytes > 0
          and $archive_expected_file_count == $archive_extracted_file_count
          and $extracted_bundle_file_count == $bundle_file_count
          and ($extracted_items | length) == $bundle_report.bundle_file_count
          and ($extracted_items | all(.ready == true))
          and $bundle_report_json_valid == true
        ),
        local_evidence_bundle_ready:$bundle_report.claim_boundary.local_evidence_bundle_ready,
        hard_true_window_required:$bundle_report.hard_true_window_required,
        r33_hard_demo_evidence_ready:$bundle_report.r33_hard_demo_evidence_ready,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        public_upload_performed:false,
        signing_notarization_performed:false
      },
      extracted_items:$extracted_items,
      side_effects:{
        local_archive_written:true,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .evidence_archive_gate_ready == true
  and .claim_boundary.local_evidence_archive_ready == true
  and (.archive_sha256 | test("^[0-9a-f]{64}$"))
  and .archive_bytes > 0
  and .expected_archive_file_count == .extracted_archive_file_count
  and .expected_bundle_file_count == .extracted_bundle_file_count
  and .extracted_bundle_item_count == .expected_bundle_file_count
  and .all_extracted_items_ready == true
  and .all_extracted_items_sha256_match == true
  and .bundle_report_json_valid == true
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .claim_boundary.signing_notarization_performed == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
