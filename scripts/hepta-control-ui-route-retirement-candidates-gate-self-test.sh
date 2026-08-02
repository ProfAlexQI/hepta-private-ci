#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
GATE="$REPO_ROOT/scripts/hepta-control-ui-route-retirement-candidates-gate"
MANIFEST="$REPO_ROOT/codex-rs/hepta-native-gateway/routes/control_ui_route_retirement_candidates_v1.json"
TMP_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/hepta-control-ui-route-retirement.XXXXXX")"
ARTIFACT_DECOY="$REPO_ROOT/artifacts/hepta-control-ui-route-retirement-self-test-decoy.json"
cleanup() {
  rm -rf "$TMP_ROOT"
  rm -f "$ARTIFACT_DECOY"
}
trap cleanup EXIT

mkdir -p "$(dirname "$ARTIFACT_DECOY")"
jq -n --arg path "$(jq -r '.candidates[0].path' "$MANIFEST")" '{runtime_evidence_route_decoy:$path}' \
  >"$ARTIFACT_DECOY"

receipt="$TMP_ROOT/receipt.json"
"$GATE" --output "$receipt" >/dev/null
jq -e '
  .schema == "hepta_control_ui_route_retirement_candidate_receipt_v1"
  and .status == "ready"
  and .retirement_status == "blocked"
  and .catalog_metrics.route_count == 284
  and .catalog_metrics.legacy_compatibility_route_count == 206
  and .catalog_metrics.canonical_route_count == 78
  and .catalog_metrics.quarantined_legacy_mutation_route_count == 28
  and .dependency_map.legacy_get_route_count == 206
  and .dependency_map.legacy_get_with_native_report_binding_count == 206
  and .dependency_map.legacy_get_with_unique_native_report_binding_count == 206
  and .dependency_map.legacy_get_with_active_renderer_count == 206
  and .dependency_map.structurally_unbound_legacy_get_route_count == 0
  and .candidate_count == 3
  and .eligible_candidate_count == 0
  and ([.candidates[].exact_path_external_consumers[]] | all(startswith("artifacts/") | not))
  and ([.candidates[].direct_product_surface_consumer_count] | all(. == 0))
  and ([.candidates[].native_report_binding_count] | all(. == 1))
  and ([.candidates[].native_report_renderer_status] | all(. == "active"))
  and ([.candidates[].blockers | length] | all(. == 3))
  and .http_behavior_changed == false
  and .route_count_reduced == false
' "$receipt" >/dev/null

bad_metric_manifest="$TMP_ROOT/bad-metric.json"
jq '.expected_catalog_metrics.legacy_compatibility_route_count = 205' \
  "$MANIFEST" > "$bad_metric_manifest"
if HEPTA_CONTROL_UI_ROUTE_RETIREMENT_MANIFEST="$bad_metric_manifest" \
  "$GATE" >/dev/null 2>&1; then
  echo "retirement candidate gate accepted a stale catalog metric" >&2
  exit 1
fi

missing_evidence_manifest="$TMP_ROOT/missing-evidence.json"
jq '.candidates[0].blockers[0].marker = "__missing_native_report_renderer_marker__"' \
  "$MANIFEST" > "$missing_evidence_manifest"
if HEPTA_CONTROL_UI_ROUTE_RETIREMENT_MANIFEST="$missing_evidence_manifest" \
  "$GATE" >/dev/null 2>&1; then
  echo "retirement candidate gate accepted missing blocker evidence" >&2
  exit 1
fi

false_eligibility_manifest="$TMP_ROOT/false-eligibility.json"
jq '.candidates[0].eligible_for_retirement = true' \
  "$MANIFEST" > "$false_eligibility_manifest"
if HEPTA_CONTROL_UI_ROUTE_RETIREMENT_MANIFEST="$false_eligibility_manifest" \
  "$GATE" >/dev/null 2>&1; then
  echo "retirement candidate gate accepted an unresolved eligible candidate" >&2
  exit 1
fi

echo "hepta-control-ui-route-retirement-candidates-gate-self-test: PASS"
