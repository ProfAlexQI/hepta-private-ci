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
  and .registry_separation.status == "blocked"
  and .registry_separation.http_registry_independent == false
  and .registry_separation.evidence_registry_independent == false
  and .registry_separation.canonical_evidence_selector_count == 206
  and .registry_separation.route_retirement_allowed == false
  and .runtime_window.configured == false
  and .runtime_window.eligible == false
  and .runtime_window.event_file_rehashed_by_gate == false
  and .runtime_window.promotion_authoritative == false
  and .runtime_window.blockers == ["runtime_window_summary_missing"]
  and .observation_epoch.window_reset_required == true
  and .observation_epoch.predecessor_event_path_accepted == false
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

bad_separation="$TMP_ROOT/bad-separation.json"
jq '.decision.route_retirement_allowed = true' \
  "$REPO_ROOT/codex-rs/hepta-native-gateway/routes/control_ui_http_evidence_registry_separation_v1.json" \
  > "$bad_separation"
if HEPTA_CONTROL_UI_HTTP_EVIDENCE_SEPARATION_CONTRACT="$bad_separation" \
  "$GATE" >/dev/null 2>&1; then
  echo "retirement candidate gate accepted an unproven registry separation" >&2
  exit 1
fi

forged_window="$TMP_ROOT/forged-window.json"
jq -n \
  --arg head "$(git -C "$REPO_ROOT" rev-parse HEAD)" \
  --arg catalog "$(shasum -a 256 "$REPO_ROOT/codex-rs/hepta-native-gateway/routes/control_ui_route_catalog_v1.jsonl" | awk '{print $1}')" \
  '{schema:"hepta_control_ui_legacy_http_window_v1",
    producer:"hepta-native-gateway/hepta-legacy-route-window",
    promotion_authoritative:false,
    durable_process_stop_observed:false, continuous_coverage_declared:false,
    shutdown_flush_verified:false,
    event_file_sha256:("1" * 64), source_head_sha:$head, route_catalog_sha256:$catalog,
    status:"ready", event_count:30, process_segment_count:1,
    window_start_unix_ms:1000, window_end_unix_ms:(1000 + 30*86400000),
    window_span_days:30, operator_active_day_count:14, trailing_zero_use_days:14,
    total_legacy_requests:0, non_ci_legacy_requests:0, routes:{},
    decision:{eligible:true,blockers:["forged-contradiction"]}}' > "$forged_window"
if HEPTA_CONTROL_UI_ROUTE_RETIREMENT_WINDOW_SUMMARY="$forged_window" \
  "$GATE" >/dev/null 2>&1; then
  echo "retirement candidate gate accepted a contradictory window decision" >&2
  exit 1
fi

stale_window="$TMP_ROOT/stale-window.json"
now_ms=$(( $(date +%s) * 1000 ))
jq --argjson end "$((now_ms - 3 * 86400000))" \
  '.decision.blockers=[] | .window_end_unix_ms=$end |
   .window_start_unix_ms=($end - 30*86400000)' "$forged_window" > "$stale_window"
if HEPTA_CONTROL_UI_ROUTE_RETIREMENT_WINDOW_SUMMARY="$stale_window" \
  "$GATE" >/dev/null 2>&1; then
  echo "retirement candidate gate accepted a stale eligible window" >&2
  exit 1
fi

echo "hepta-control-ui-route-retirement-candidates-gate-self-test: PASS"
