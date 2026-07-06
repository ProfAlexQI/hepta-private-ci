#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
DEMO_EVIDENCE_PATH="${HEPTA_UI_DEMO_EVIDENCE_REPORT_PATH:-$READINESS_DIR/ui-demo-evidence-gate.json}"
BUNDLE_DIR="${HEPTA_UI_EVIDENCE_BUNDLE_DIR:-$READINESS_DIR/evidence-bundle}"
REPORT_PATH="${HEPTA_UI_EVIDENCE_BUNDLE_REPORT_PATH:-$READINESS_DIR/ui-evidence-bundle-gate.json}"
BACKEND_RECEIPT_INPUT_PATH="${HEPTA_UI_BACKEND_RECEIPT_INPUT_PATH:-}"
BACKEND_DELIVERY_RECEIPT_INPUT_PATH="${HEPTA_UI_BACKEND_DELIVERY_RECEIPT_INPUT_PATH:-}"
RELEASE_APPROVAL_INPUT_PATH="${HEPTA_UI_RELEASE_APPROVAL_INPUT_PATH:-}"
RELEASE_ARTIFACT_INPUT_PATH="${HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH:-}"
BACKEND_DISPATCH_PACKET_DIR="${HEPTA_UI_BACKEND_DISPATCH_PACKET_DIR:-$READINESS_DIR/backend-dispatch-packet}"
BACKEND_DISPATCH_PACKET_ARCHIVE_PATH="${HEPTA_UI_BACKEND_DISPATCH_PACKET_ARCHIVE_PATH:-$BACKEND_DISPATCH_PACKET_DIR/backend-dispatch-packet.tar.gz}"
BACKEND_DISPATCH_PACKET_MANIFEST_PATH="${HEPTA_UI_BACKEND_DISPATCH_PACKET_MANIFEST_PATH:-$BACKEND_DISPATCH_PACKET_DIR/backend-dispatch-packet-manifest.json}"
BACKEND_DISPATCH_PACKET_MARKDOWN_PATH="${HEPTA_UI_BACKEND_DISPATCH_PACKET_MARKDOWN_PATH:-$BACKEND_DISPATCH_PACKET_DIR/backend-dispatch-packet.md}"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI evidence bundle gate\n' "$1" >&2
    exit 2
  fi
}

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required evidence-bundle input: %s\n' "$path" >&2
    exit 1
  fi
}

require_command jq
require_command shasum

require_file "$DEMO_EVIDENCE_PATH"

if ! jq -e '.status == "ready" and .demo_evidence_gate_ready == true' "$DEMO_EVIDENCE_PATH" >/dev/null; then
  printf 'Demo evidence report is not ready: %s\n' "$DEMO_EVIDENCE_PATH" >&2
  exit 1
fi

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-evidence-bundle.XXXXXX")"
TMP_BUNDLE_DIR="$TMP_DIR/evidence-bundle"
BUNDLE_ITEMS_NDJSON="$TMP_DIR/bundle-items.ndjson"
BUNDLE_ITEMS_JSON="$TMP_DIR/bundle-items.json"
REPORT_TMP="$TMP_DIR/evidence-bundle-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

mkdir -p "$TMP_BUNDLE_DIR"
: >"$BUNDLE_ITEMS_NDJSON"

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

string_sha12() {
  printf '%s' "$1" | shasum -a 256 | awk '{print substr($1, 1, 12)}'
}

safe_segment() {
  printf '%s' "$1" |
    tr -cs '[:alnum:]._-' '-' |
    sed -E 's/^-+//; s/-+$//; s/^$/item/'
}

extension_for_path() {
  local path="$1"
  local base="${path##*/}"
  if [[ "$base" == *.tar.gz ]]; then
    printf '.tar.gz'
    return 0
  fi
  if [[ "$base" == *.* ]]; then
    printf '.%s' "${base##*.}"
  else
    printf ''
  fi
}

copy_bundle_item() {
  local kind="$1"
  local group="$2"
  local name="$3"
  local source_path="$4"
  local expected_sha="$5"
  local expected_bytes="$6"
  local expected_dimensions="$7"

  local safe_group safe_name path_id extension relative_path tmp_dest final_dest
  local copied=false
  local bundle_bytes=0
  local bundle_sha=""
  local source_present=false
  local source_bytes=0
  local source_sha=""
  local bundle_json_valid=true

  safe_group="$(safe_segment "$group")"
  safe_name="$(safe_segment "$name")"
  path_id="$(string_sha12 "$source_path")"
  extension="$(extension_for_path "$source_path")"
  if [[ "$kind" == "report" || "$kind" == "source_demo_evidence" || "$kind" == "replay_input" ]]; then
    extension=".json"
  fi
  relative_path="$kind/$safe_group/$safe_name-$path_id$extension"
  tmp_dest="$TMP_BUNDLE_DIR/$relative_path"
  final_dest="$BUNDLE_DIR/$relative_path"

  if [[ -s "$source_path" ]]; then
    source_present=true
    source_bytes="$(wc -c <"$source_path" | tr -d ' ')"
    source_sha="$(file_sha256 "$source_path")"
    mkdir -p "$(dirname "$tmp_dest")"
    cp -p "$source_path" "$tmp_dest"
    copied=true
    bundle_bytes="$(wc -c <"$tmp_dest" | tr -d ' ')"
    bundle_sha="$(file_sha256 "$tmp_dest")"
    if [[ "$kind" == "report" || "$kind" == "source_demo_evidence" || "$kind" == "replay_input" ]]; then
      if ! jq empty "$tmp_dest" >/dev/null 2>&1; then
        bundle_json_valid=false
      fi
    fi
  fi

  jq -n \
    --arg kind "$kind" \
    --arg group "$group" \
    --arg name "$name" \
    --arg source_path "$source_path" \
    --arg relative_path "$relative_path" \
    --arg bundle_path "$final_dest" \
    --arg source_sha "$source_sha" \
    --arg bundle_sha "$bundle_sha" \
    --arg expected_sha "$expected_sha" \
    --arg expected_dimensions "$expected_dimensions" \
    --argjson expected_bytes "${expected_bytes:-0}" \
    --argjson source_present "$source_present" \
    --argjson source_bytes "$source_bytes" \
    --argjson copied "$copied" \
    --argjson bundle_bytes "$bundle_bytes" \
    --argjson bundle_json_valid "$bundle_json_valid" \
    '{
      kind:$kind,
      group:$group,
      name:$name,
      source_path:$source_path,
      relative_path:$relative_path,
      bundle_path:$bundle_path,
      source_present:$source_present,
      copied:$copied,
      source_bytes:$source_bytes,
      bundle_bytes:$bundle_bytes,
      bytes_match:($source_bytes == $bundle_bytes),
      source_sha256:$source_sha,
      bundle_sha256:$bundle_sha,
      expected_sha256:$expected_sha,
      sha256_match:(
        $copied
        and ($source_sha | test("^[0-9a-f]{64}$"))
        and $source_sha == $bundle_sha
        and (($expected_sha | length) == 0 or $bundle_sha == $expected_sha)
      ),
      expected_bytes:$expected_bytes,
      expected_dimensions:$expected_dimensions,
      json_valid:$bundle_json_valid,
      ready:(
        $copied
        and ($source_bytes == $bundle_bytes)
        and ($source_sha | test("^[0-9a-f]{64}$"))
        and $source_sha == $bundle_sha
        and (($expected_sha | length) == 0 or $bundle_sha == $expected_sha)
        and ($bundle_json_valid == true)
      )
    }'
}

copy_bundle_item \
  "source_demo_evidence" \
  "ui" \
  "demo_evidence_gate" \
  "$DEMO_EVIDENCE_PATH" \
  "" \
  "0" \
  "" >>"$BUNDLE_ITEMS_NDJSON"

while IFS= read -r item; do
  copy_bundle_item \
    "report" \
    "$(jq -r '.group' <<<"$item")" \
    "$(jq -r '.name' <<<"$item")" \
    "$(jq -r '.path' <<<"$item")" \
    "$(jq -r '.sha256 // ""' <<<"$item")" \
    "$(jq -r '.bytes // 0' <<<"$item")" \
    "" >>"$BUNDLE_ITEMS_NDJSON"
done < <(jq -c '.report_evidence.items[] | select(.required == true)' "$DEMO_EVIDENCE_PATH")

while IFS= read -r item; do
  copy_bundle_item \
    "screenshot" \
    "$(jq -r '.group' <<<"$item")" \
    "$(jq -r '.name' <<<"$item")" \
    "$(jq -r '.path' <<<"$item")" \
    "$(jq -r '.sha256 // ""' <<<"$item")" \
    "$(jq -r '.bytes // 0' <<<"$item")" \
    "$(jq -r '.dimensions // ""' <<<"$item")" >>"$BUNDLE_ITEMS_NDJSON"
done < <(jq -c '.screenshot_evidence.items[] | select(.required == true)' "$DEMO_EVIDENCE_PATH")

expected_replay_input_count=0
copy_replay_input_if_present() {
  local name="$1"
  local source_path="$2"

  if [[ -z "$source_path" ]]; then
    return 0
  fi
  require_file "$source_path"
  copy_bundle_item \
    "replay_input" \
    "accepted-inputs" \
    "$name" \
    "$source_path" \
    "$(file_sha256 "$source_path")" \
    "$(wc -c <"$source_path" | tr -d ' ')" \
    "" >>"$BUNDLE_ITEMS_NDJSON"
  expected_replay_input_count=$((expected_replay_input_count + 1))
}

copy_replay_input_if_present "backend-receipt-input" "$BACKEND_RECEIPT_INPUT_PATH"
copy_replay_input_if_present "backend-delivery-receipt-input" "$BACKEND_DELIVERY_RECEIPT_INPUT_PATH"
copy_replay_input_if_present "release-approval-input" "$RELEASE_APPROVAL_INPUT_PATH"
copy_replay_input_if_present "release-artifact-input" "$RELEASE_ARTIFACT_INPUT_PATH"

expected_replay_artifact_count=0
copy_replay_artifact_required() {
  local name="$1"
  local source_path="$2"

  require_file "$source_path"
  copy_bundle_item \
    "replay_artifact" \
    "backend-dispatch-packet" \
    "$name" \
    "$source_path" \
    "$(file_sha256 "$source_path")" \
    "$(wc -c <"$source_path" | tr -d ' ')" \
    "" >>"$BUNDLE_ITEMS_NDJSON"
  expected_replay_artifact_count=$((expected_replay_artifact_count + 1))
}

backend_dispatch_artifact_present_count=0
for backend_dispatch_artifact_path in \
  "$BACKEND_DISPATCH_PACKET_ARCHIVE_PATH" \
  "$BACKEND_DISPATCH_PACKET_MANIFEST_PATH" \
  "$BACKEND_DISPATCH_PACKET_MARKDOWN_PATH"; do
  if [[ -s "$backend_dispatch_artifact_path" ]]; then
    backend_dispatch_artifact_present_count=$((backend_dispatch_artifact_present_count + 1))
  fi
done

if [[ "$backend_dispatch_artifact_present_count" == "3" ]]; then
  copy_replay_artifact_required "backend-dispatch-packet-archive" "$BACKEND_DISPATCH_PACKET_ARCHIVE_PATH"
  copy_replay_artifact_required "backend-dispatch-packet-manifest" "$BACKEND_DISPATCH_PACKET_MANIFEST_PATH"
  copy_replay_artifact_required "backend-dispatch-packet-markdown" "$BACKEND_DISPATCH_PACKET_MARKDOWN_PATH"
elif [[ "$backend_dispatch_artifact_present_count" != "0" ]]; then
  printf 'Partial backend dispatch packet replay artifacts are invalid: %s of 3 present\n' "$backend_dispatch_artifact_present_count" >&2
  exit 1
fi

jq -s '.' "$BUNDLE_ITEMS_NDJSON" >"$BUNDLE_ITEMS_JSON"

rm -rf "$BUNDLE_DIR"
mkdir -p "$(dirname "$BUNDLE_DIR")"
mv "$TMP_BUNDLE_DIR" "$BUNDLE_DIR"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_evidence_bundle_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg demo_evidence_report_path "$DEMO_EVIDENCE_PATH" \
  --arg bundle_dir "$BUNDLE_DIR" \
  --arg report_path "$REPORT_PATH" \
  --argjson expected_replay_input_count "$expected_replay_input_count" \
  --argjson expected_replay_artifact_count "$expected_replay_artifact_count" \
  --slurpfile demo "$DEMO_EVIDENCE_PATH" \
  --slurpfile items "$BUNDLE_ITEMS_JSON" \
  '
  ($demo[0]) as $demo_evidence
  | ($items[0]) as $bundle_items
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(
        if (
          ($bundle_items | all(.ready == true))
          and ($bundle_items | map(select(.kind == "report")) | length) == $demo_evidence.report_evidence.required_report_count
          and ($bundle_items | map(select(.kind == "screenshot")) | length) == $demo_evidence.screenshot_evidence.required_screenshot_count
          and ($bundle_items | map(select(.kind == "source_demo_evidence")) | length) == 1
          and ($bundle_items | map(select(.kind == "replay_input")) | length) == $expected_replay_input_count
          and ($bundle_items | map(select(.kind == "replay_artifact")) | length) == $expected_replay_artifact_count
        ) then "ready" else "failed" end
      ),
      evidence_bundle_gate_ready:(
        ($bundle_items | all(.ready == true))
        and ($bundle_items | map(select(.kind == "report")) | length) == $demo_evidence.report_evidence.required_report_count
        and ($bundle_items | map(select(.kind == "screenshot")) | length) == $demo_evidence.screenshot_evidence.required_screenshot_count
        and ($bundle_items | map(select(.kind == "source_demo_evidence")) | length) == 1
        and ($bundle_items | map(select(.kind == "replay_input")) | length) == $expected_replay_input_count
        and ($bundle_items | map(select(.kind == "replay_artifact")) | length) == $expected_replay_artifact_count
      ),
      readiness_dir:$readiness_dir,
      demo_evidence_report_path:$demo_evidence_report_path,
      bundle_dir:$bundle_dir,
      report_path:$report_path,
      source_demo_evidence_sha256:($bundle_items[] | select(.kind == "source_demo_evidence") | .bundle_sha256),
      copied_report_count:($bundle_items | map(select(.kind == "report")) | length),
      copied_screenshot_count:($bundle_items | map(select(.kind == "screenshot")) | length),
      copied_source_report_count:($bundle_items | map(select(.kind == "source_demo_evidence")) | length),
      copied_replay_input_count:($bundle_items | map(select(.kind == "replay_input")) | length),
      expected_replay_input_count:$expected_replay_input_count,
      copied_replay_artifact_count:($bundle_items | map(select(.kind == "replay_artifact")) | length),
      expected_replay_artifact_count:$expected_replay_artifact_count,
      bundle_file_count:($bundle_items | length),
      bundle_total_bytes:($bundle_items | map(.bundle_bytes) | add // 0),
      required_report_count:$demo_evidence.report_evidence.required_report_count,
      required_screenshot_count:$demo_evidence.screenshot_evidence.required_screenshot_count,
      hard_true_window_required:$demo_evidence.claim_boundary.hard_true_window_required,
      r33_hard_demo_evidence_ready:$demo_evidence.claim_boundary.r33_hard_demo_evidence_ready,
      all_bundle_items_ready:($bundle_items | all(.ready == true)),
      all_bundle_items_sha256_match:($bundle_items | all(.sha256_match == true)),
      all_bundle_reports_json_valid:($bundle_items | map(select(.kind == "report" or .kind == "source_demo_evidence" or .kind == "replay_input")) | all(.json_valid == true)),
      claim_boundary:{
        local_evidence_bundle_ready:(
          ($bundle_items | all(.ready == true))
          and ($bundle_items | map(select(.kind == "report")) | length) == $demo_evidence.report_evidence.required_report_count
          and ($bundle_items | map(select(.kind == "screenshot")) | length) == $demo_evidence.screenshot_evidence.required_screenshot_count
          and ($bundle_items | map(select(.kind == "replay_input")) | length) == $expected_replay_input_count
          and ($bundle_items | map(select(.kind == "replay_artifact")) | length) == $expected_replay_artifact_count
        ),
        local_fixture_demo_evidence_ready:$demo_evidence.claim_boundary.local_fixture_demo_evidence_ready,
        hard_true_window_required:$demo_evidence.claim_boundary.hard_true_window_required,
        r33_hard_demo_evidence_ready:$demo_evidence.claim_boundary.r33_hard_demo_evidence_ready,
        replay_inputs_archived:(($bundle_items | map(select(.kind == "replay_input")) | length) == $expected_replay_input_count),
        replay_artifacts_archived:(($bundle_items | map(select(.kind == "replay_artifact")) | length) == $expected_replay_artifact_count),
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        public_upload_performed:false
      },
      bundle_items:$bundle_items,
      side_effects:{
        local_bundle_written:true,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .evidence_bundle_gate_ready == true
  and .claim_boundary.local_evidence_bundle_ready == true
  and .copied_source_report_count == 1
  and .copied_report_count == .required_report_count
  and .copied_screenshot_count == .required_screenshot_count
  and .copied_replay_input_count == .expected_replay_input_count
  and .copied_replay_artifact_count == .expected_replay_artifact_count
  and .claim_boundary.replay_inputs_archived == true
  and .claim_boundary.replay_artifacts_archived == true
  and .all_bundle_items_ready == true
  and .all_bundle_items_sha256_match == true
  and .all_bundle_reports_json_valid == true
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
