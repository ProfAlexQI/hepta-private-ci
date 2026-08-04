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
  and .registry_separation.status == "ready"
  and .registry_separation.http_registry_independent == true
  and .registry_separation.evidence_registry_independent == true
  and .registry_separation.separation_precondition_satisfied == true
  and .registry_separation.evidence_definition_count == 207
  and .registry_separation.canonical_evidence_selector_count == 206
  and .registry_separation.route_retirement_allowed == false
  and .runtime_window.configured == false
  and .runtime_window.eligible == false
  and .runtime_window.event_file_rehashed_by_gate == false
  and .runtime_window.promotion_authoritative == false
  and (.runtime_window.source_full_head | length) == 40
  and (.runtime_window.compiled_source_head | length) == 12
  and .runtime_window.event_source_resolution_verified == false
  and (.runtime_window.blockers | index("runtime_window_summary_missing")) != null
  and (.runtime_window.blockers | index("trusted_launcher_identity_not_configured")) != null
  and .telemetry_trust.status == "blocked"
  and .telemetry_trust.environment_run_class_has_promotion_authority == false
  and .telemetry_trust.promotion_authoritative == false
  and .observation_epoch.window_reset_required == true
  and .observation_epoch.predecessor_event_path_accepted == false
  and .observation_epoch.authoritative_window_start_allowed == false
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

path_bound_separation="$TMP_ROOT/path-bound-separation.json"
jq '.evidence_registry_independent = false |
    .proof.http_path_to_report_binding_runtime_dependency_count = 1' \
  "$REPO_ROOT/codex-rs/hepta-native-gateway/routes/control_ui_http_evidence_registry_separation_v1.json" \
  > "$path_bound_separation"
if HEPTA_CONTROL_UI_HTTP_EVIDENCE_SEPARATION_CONTRACT="$path_bound_separation" \
  "$GATE" >/dev/null 2>&1; then
  echo "retirement candidate gate accepted a path-bound evidence registry" >&2
  exit 1
fi

events="$TMP_ROOT/events.jsonl"
now_ms=$(( $(date +%s) * 1000 ))
ruby -rjson - "$events" "$(git -C "$REPO_ROOT" rev-parse --short=12 HEAD)" \
  "$(shasum -a 256 "$REPO_ROOT/codex-rs/hepta-native-gateway/routes/control_ui_route_catalog_v1.jsonl" | awk '{print $1}')" \
  "$now_ms" <<'RUBY'
destination, head, catalog, now = ARGV
now = Integer(now, 10)
base = {
  "schema" => "hepta_control_ui_legacy_http_event_v2",
  "process_run_identifier_sha256" => "1" * 64,
  "process_class" => "hepta_native_gateway", "run_class" => "operator",
  "head_sha" => head, "catalog_sha" => catalog,
  "source_binding_valid" => true, "catalog_binding_valid" => true,
  "route_key" => nil, "route_state" => nil, "consumer_class" => nil,
  "preflight" => nil, "http_status" => nil, "write_result" => nil,
  "observation_complete" => true, "dropped_event_count" => 0,
  "persist_error_count" => 0, "incomplete_observation_count" => 0,
  "capacity_reached" => false,
}
events = [
  base.merge("event_type" => "process_start", "sequence" => 1, "time_unix_ms" => now - 1000),
  base.merge("event_type" => "heartbeat", "sequence" => 2, "time_unix_ms" => now - 500),
  base.merge("event_type" => "process_stop", "sequence" => 3, "time_unix_ms" => now),
]
File.write(destination, events.map { |event| JSON.generate(event) }.join("\n") + "\n")
File.chmod(0o600, destination)
RUBY
events_sha="$(shasum -a 256 "$events" | awk '{print $1}')"
toolchain="$(sed -n 's/^channel = "\([^"]*\)"$/\1/p' "$REPO_ROOT/codex-rs/rust-toolchain.toml")"
rustc="$(rustup which --toolchain "$toolchain" rustc)"
rustdoc="$(rustup which --toolchain "$toolchain" rustdoc)"
toolchain_bin="$(dirname "$rustc")"
blocked_window="$TMP_ROOT/blocked-window.json"
env RUSTUP_TOOLCHAIN="$toolchain" RUSTC="$rustc" RUSTDOC="$rustdoc" \
  PATH="$toolchain_bin:$PATH" \
  rustup run "$toolchain" cargo run --offline --quiet \
  --manifest-path "$REPO_ROOT/codex-rs/Cargo.toml" \
  -p hepta-native-gateway --bin hepta-legacy-route-window -- \
  --events "$events" --expected-head "$(git -C "$REPO_ROOT" rev-parse --short=12 HEAD)" \
  --allow-blocked >"$blocked_window"

forged_window="$TMP_ROOT/forged-window.json"
jq '.status="ready" | .decision={eligible:true,blockers:[]}' \
  "$blocked_window" >"$forged_window"
if HEPTA_CONTROL_UI_ROUTE_RETIREMENT_WINDOW_SUMMARY="$forged_window" \
  HEPTA_CONTROL_UI_ROUTE_RETIREMENT_WINDOW_EVENTS="$events" \
  "$GATE" >/dev/null 2>&1; then
  echo "retirement candidate gate accepted a summary that differs from trusted replay" >&2
  exit 1
fi

blocked_receipt="$TMP_ROOT/blocked-receipt.json"
HEPTA_CONTROL_UI_ROUTE_RETIREMENT_WINDOW_SUMMARY="$blocked_window" \
  HEPTA_CONTROL_UI_ROUTE_RETIREMENT_WINDOW_EVENTS="$events" \
  "$GATE" --output "$blocked_receipt" >/dev/null
jq -e --arg sha "$events_sha" '
  .runtime_window.configured == true
  and .runtime_window.event_file_rehashed_by_gate == true
  and .runtime_window.event_file_sha256 == $sha
  and .runtime_window.trusted_replay_match == true
  and (.runtime_window.source_full_head | length) == 40
  and (.runtime_window.compiled_source_head | length) == 12
  and .runtime_window.event_source_resolution_verified == true
  and .runtime_window.producer_eligible == false
  and .runtime_window.eligible == false
  and .runtime_window.promotion_authoritative == false
  and (.runtime_window.blockers | index("trusted_launcher_identity_not_configured")) != null
  and (.runtime_window.blockers | index("independent_shutdown_durability_receipt_missing")) != null
' "$blocked_receipt" >/dev/null

printf '\n' >>"$events"
if HEPTA_CONTROL_UI_ROUTE_RETIREMENT_WINDOW_SUMMARY="$blocked_window" \
  HEPTA_CONTROL_UI_ROUTE_RETIREMENT_WINDOW_EVENTS="$events" \
  "$GATE" >/dev/null 2>&1; then
  echo "retirement candidate gate accepted raw event bytes changed after summary" >&2
  exit 1
fi

echo "hepta-control-ui-route-retirement-candidates-gate-self-test: PASS"
