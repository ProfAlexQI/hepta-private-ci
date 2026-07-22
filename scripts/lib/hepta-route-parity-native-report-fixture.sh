#!/usr/bin/env bash

hepta_load_route_parity_native_reports() {
  local fixture="${HEPTA_ROUTE_PARITY_NATIVE_REPORT_FIXTURE:-}"
  local fixture_json normalized_reports actual_sha256 expected_sha256
  [[ -n "$fixture" && -f "$fixture" ]] || {
    echo "offline route parity Native report fixture is missing" >&2
    return 1
  }
  fixture_json="$(jq -ce . "$fixture")" || {
    echo "offline route parity Native report fixture is malformed" >&2
    return 1
  }
  jq -e '
    .schema == "hepta_route_parity_native_report_fixture_bundle_v1"
    and .provenance.source == "route_native_gateway_request"
    and (.reports | type) == "object"
    and ([
      "core_fusion_readiness",
      "engine_dependency_closure",
      "local_tooling_content_inventory",
      "memory_capability_absorption_inventory",
      "provider_channel_dry_run_plan",
      "public_ga_operator_approval_packet",
      "release_hardening_status_gate",
      "runtime_session_dry_run_inventory"
    ] - (.reports | keys) | length) == 0
  ' <<<"$fixture_json" >/dev/null || {
    echo "offline route parity Native report fixture is stale" >&2
    return 1
  }
  normalized_reports="$(jq -cS '.reports' <<<"$fixture_json")"
  actual_sha256="$(printf '%s' "$normalized_reports" | shasum -a 256 | awk '{print $1}')"
  expected_sha256="$(jq -r '.provenance.normalized_reports_sha256' <<<"$fixture_json")"
  [[ "$actual_sha256" == "$expected_sha256" ]] || {
    echo "offline route parity Native report fixture source drift detected" >&2
    return 1
  }
  HEPTA_ROUTE_PARITY_NATIVE_REPORTS_JSON="$normalized_reports"
}
