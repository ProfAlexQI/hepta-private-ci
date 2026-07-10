#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
cd "$REPO_ROOT"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

success_report="$(
  capture_json_report "success-json-fixture" bash -c '
    printf "%s\n" \
      "fixture log before json" \
      "{" \
      "  \"status\": \"ready\"," \
      "  \"gate\": \"success_json_fixture\"," \
      "  \"side_effects\": {\"filesystem_written\": false}" \
      "}" \
      "fixture pass line"
  '
)"

set +e
failing_json_diagnostic="$(
  (
    capture_json_report "failing-json-fixture" bash -c '
      printf "%s\n" \
        "fixture log before failure json" \
        "{" \
        "  \"status\": \"attention\"," \
        "  \"gate\": \"failing_json_fixture\"," \
        "  \"blockers\": [\"synthetic_failure\"]," \
        "  \"side_effects\": {\"filesystem_written\": false}" \
        "}"
      exit 7
    '
  ) 2>&1
)"
failing_json_rc=$?

no_json_diagnostic="$(
  (
    capture_json_report "no-json-fixture" bash -c '
      printf "%s\n" \
        "plain diagnostic line one" \
        "plain diagnostic line two"
    '
  ) 2>&1
)"
no_json_rc=$?

stderr_json_report="$(
  capture_json_report "stderr-json-fixture" bash -c '
    printf "%s\n" "stderr diagnostic before json" >&2
    printf "%s\n" \
      "{" \
      "  \"status\": \"ready\"," \
      "  \"gate\": \"stderr_json_fixture\"," \
      "  \"side_effects\": {\"filesystem_written\": false}" \
      "}"
    printf "%s\n" "stderr diagnostic after json" >&2
  '
)"

multiple_json_report="$(
  capture_json_report "multiple-json-fixture" bash -c '
    printf "%s\n" \
      "{" \
      "  \"status\": \"ready\"," \
      "  \"gate\": \"first_json_fixture\"," \
      "  \"sequence\": 1," \
      "  \"side_effects\": {\"filesystem_written\": false}" \
      "}" \
      "{" \
      "  \"status\": \"attention\"," \
      "  \"gate\": \"second_json_noise\"," \
      "  \"sequence\": 2" \
      "}"
  '
)"

malformed_json_diagnostic="$(
  (
    capture_json_report "malformed-json-fixture" bash -c '
      printf "%s\n" \
        "{" \
        "  \"status\": \"ready\"," \
        "  \"gate\":" \
        "}"
    '
  ) 2>&1
)"
malformed_json_rc=$?

bounded_tail_diagnostic="$(
  (
    HEPTA_JSON_REPORT_CAPTURE_DIAGNOSTIC_LINES=2 \
      capture_json_report "bounded-tail-fixture" bash -c '
        printf "%s\n" \
          "bounded tail line one" \
          "bounded tail line two" \
          "bounded tail line three" \
          "bounded tail line four"
      '
  ) 2>&1
)"
bounded_tail_rc=$?
set -e

cache_fixture_dir="$(mktemp -d /tmp/hepta-json-report-capture-diagnostic.XXXXXX)"
cache_counter_path="$cache_fixture_dir/counter"
cached_first_report="$(
  HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$cache_fixture_dir" \
    HEPTA_JSON_REPORT_CAPTURE_CACHE_SALT="diagnostic-cache-fixture" \
    capture_json_report "cache-json-fixture" bash -c '
      counter_file="$1"
      run_count=0
      if [[ -f "$counter_file" ]]; then
        run_count="$(cat "$counter_file")"
      fi
      run_count=$((run_count + 1))
      printf "%s\n" "$run_count" >"$counter_file"
      printf "%s\n" \
        "{" \
        "  \"status\": \"ready\"," \
        "  \"gate\": \"cache_json_fixture\"," \
        "  \"run_count\": $run_count," \
        "  \"side_effects\": {\"filesystem_written\": false}" \
        "}"
    ' bash "$cache_counter_path"
)"
cached_second_report="$(
  HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$cache_fixture_dir" \
    HEPTA_JSON_REPORT_CAPTURE_CACHE_SALT="diagnostic-cache-fixture" \
    capture_json_report "cache-json-fixture" bash -c '
      counter_file="$1"
      run_count=0
      if [[ -f "$counter_file" ]]; then
        run_count="$(cat "$counter_file")"
      fi
      run_count=$((run_count + 1))
      printf "%s\n" "$run_count" >"$counter_file"
      printf "%s\n" \
        "{" \
        "  \"status\": \"ready\"," \
        "  \"gate\": \"cache_json_fixture\"," \
        "  \"run_count\": $run_count," \
        "  \"side_effects\": {\"filesystem_written\": false}" \
        "}"
    ' bash "$cache_counter_path"
)"
cached_alias_report="$(
  HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$cache_fixture_dir" \
    HEPTA_JSON_REPORT_CAPTURE_CACHE_SALT="diagnostic-cache-fixture" \
    capture_json_report "cache-json-fixture-alias" bash -c '
      counter_file="$1"
      run_count=0
      if [[ -f "$counter_file" ]]; then
        run_count="$(cat "$counter_file")"
      fi
      run_count=$((run_count + 1))
      printf "%s\n" "$run_count" >"$counter_file"
      printf "%s\n" \
        "{" \
        "  \"status\": \"ready\"," \
        "  \"gate\": \"cache_json_fixture\"," \
        "  \"run_count\": $run_count," \
        "  \"side_effects\": {\"filesystem_written\": false}" \
        "}"
    ' bash "$cache_counter_path"
)"
cache_counter_value=0
if [[ -f "$cache_counter_path" ]]; then
  cache_counter_value="$(cat "$cache_counter_path")"
fi
rm -rf "$cache_fixture_dir"

success_fixture_ok=false
failing_json_fixture_ok=false
no_json_fixture_ok=false
stderr_json_fixture_ok=false
multiple_json_fixture_ok=false
malformed_json_fixture_ok=false
bounded_tail_fixture_ok=false
cache_fixture_ok=false

if jq -e '
  .status == "ready"
  and .gate == "success_json_fixture"
  and .side_effects.filesystem_written == false
' >/dev/null <<<"$success_report"; then
  success_fixture_ok=true
fi

if [[ "$failing_json_rc" -eq 7 ]] \
  && grep -Fq "failing-json-fixture failed with exit code 7" <<<"$failing_json_diagnostic" \
  && grep -Fq "\"gate\": \"failing_json_fixture\"" <<<"$failing_json_diagnostic"; then
  failing_json_fixture_ok=true
fi

if [[ "$no_json_rc" -eq 1 ]] \
  && grep -Fq "no-json-fixture did not emit a parseable JSON report" <<<"$no_json_diagnostic" \
  && grep -Fq "plain diagnostic line two" <<<"$no_json_diagnostic"; then
  no_json_fixture_ok=true
fi

if jq -e '
  .status == "ready"
  and .gate == "stderr_json_fixture"
  and .side_effects.filesystem_written == false
' >/dev/null <<<"$stderr_json_report"; then
  stderr_json_fixture_ok=true
fi

if jq -e '
  .status == "ready"
  and .gate == "first_json_fixture"
  and .sequence == 1
  and .side_effects.filesystem_written == false
' >/dev/null <<<"$multiple_json_report"; then
  multiple_json_fixture_ok=true
fi

if [[ "$malformed_json_rc" -eq 1 ]] \
  && grep -Fq "malformed-json-fixture did not emit a parseable JSON report" <<<"$malformed_json_diagnostic" \
  && grep -Fq "malformed-json-fixture output tail:" <<<"$malformed_json_diagnostic" \
  && grep -Fq "  \"gate\":" <<<"$malformed_json_diagnostic"; then
  malformed_json_fixture_ok=true
fi

if [[ "$bounded_tail_rc" -eq 1 ]] \
  && grep -Fq "bounded-tail-fixture did not emit a parseable JSON report" <<<"$bounded_tail_diagnostic" \
  && grep -Fq "bounded tail line three" <<<"$bounded_tail_diagnostic" \
  && grep -Fq "bounded tail line four" <<<"$bounded_tail_diagnostic" \
  && ! grep -Fq "bounded tail line one" <<<"$bounded_tail_diagnostic" \
  && ! grep -Fq "bounded tail line two" <<<"$bounded_tail_diagnostic"; then
  bounded_tail_fixture_ok=true
fi

if jq -e '
  .status == "ready"
  and .gate == "cache_json_fixture"
  and .run_count == 1
  and .side_effects.filesystem_written == false
' >/dev/null <<<"$cached_first_report" \
  && jq -e '
    .status == "ready"
    and .gate == "cache_json_fixture"
    and .run_count == 1
    and .side_effects.filesystem_written == false
  ' >/dev/null <<<"$cached_second_report" \
  && jq -e '
    .status == "ready"
    and .gate == "cache_json_fixture"
    and .run_count == 1
    and .side_effects.filesystem_written == false
  ' >/dev/null <<<"$cached_alias_report" \
  && [[ "$cache_counter_value" -eq 1 ]]; then
  cache_fixture_ok=true
fi

contract_hash_sha256="$(
  sha256_text "hepta-json-report-capture:contract:$success_fixture_ok:$failing_json_fixture_ok:$no_json_fixture_ok:$stderr_json_fixture_ok:$multiple_json_fixture_ok:$malformed_json_fixture_ok:$bounded_tail_fixture_ok:$cache_fixture_ok"
)"
policy_hash_sha256="$(sha256_text "hepta-json-report-capture:policy:no-workspace-write:no-secret-read:no-external-send:ephemeral-cache-only")"
side_effect_hash_sha256="$(sha256_text "hepta-json-report-capture:side-effects:false:false:false:ephemeral-cache")"

jq -n -e \
  --arg contract_hash_sha256 "$contract_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson success_fixture_ok "$success_fixture_ok" \
  --argjson failing_json_fixture_ok "$failing_json_fixture_ok" \
  --argjson no_json_fixture_ok "$no_json_fixture_ok" \
  --argjson stderr_json_fixture_ok "$stderr_json_fixture_ok" \
  --argjson multiple_json_fixture_ok "$multiple_json_fixture_ok" \
  --argjson malformed_json_fixture_ok "$malformed_json_fixture_ok" \
  --argjson bounded_tail_fixture_ok "$bounded_tail_fixture_ok" \
  --argjson cache_fixture_ok "$cache_fixture_ok" \
  '
    if (
      $success_fixture_ok == true
      and $failing_json_fixture_ok == true
      and $no_json_fixture_ok == true
      and $stderr_json_fixture_ok == true
      and $multiple_json_fixture_ok == true
      and $malformed_json_fixture_ok == true
      and $bounded_tail_fixture_ok == true
      and $cache_fixture_ok == true
    ) then {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_json_report_capture_diagnostic_contract_gate",
      json_report_capture_diagnostic_contract_schema_version: "json_report_capture_diagnostic_contract_v3",
      json_report_capture_diagnostic_contract_ready: true,
      diagnostic_mode: "synthetic_child_command_capture_with_ephemeral_cache_no_workspace_write",
      diagnostic_decision: "child_json_report_capture_preserves_success_reports_exposes_bounded_failure_diagnostics_and_reuses_success_reports_from_ephemeral_preflight_cache",
      helper_path: "scripts/lib/hepta-json-report-capture.sh",
      diagnostic_fixture_count: 8,
      success_fixture_ok: $success_fixture_ok,
      failing_json_fixture_ok: $failing_json_fixture_ok,
      no_json_fixture_ok: $no_json_fixture_ok,
      stderr_json_fixture_ok: $stderr_json_fixture_ok,
      multiple_json_fixture_ok: $multiple_json_fixture_ok,
      malformed_json_fixture_ok: $malformed_json_fixture_ok,
      bounded_tail_fixture_ok: $bounded_tail_fixture_ok,
      cache_fixture_ok: $cache_fixture_ok,
      parseable_json_report_preserved: true,
      failing_child_exit_code_preserved: true,
      failing_child_json_report_exposed: true,
      non_json_output_tail_exposed: true,
      stderr_output_tolerated: true,
      first_json_object_preserved_when_multiple_json_objects_follow: true,
      malformed_json_output_tail_exposed: true,
      diagnostic_tail_line_budget_enforced: true,
      ephemeral_cache_reuses_success_report_without_rerunning_child: true,
      ephemeral_cache_persists_no_evidence: true,
      contract_hash_sha256: $contract_hash_sha256,
      policy_hash_sha256: $policy_hash_sha256,
      side_effect_hash_sha256: $side_effect_hash_sha256,
      denied_by_json_report_capture_contract: [
        "json_report_capture_workspace_write_denied",
        "json_report_capture_secret_read_denied",
        "json_report_capture_external_send_denied",
        "json_report_capture_cache_evidence_persistence_denied",
        "json_report_capture_child_recovery_action_denied"
      ],
      side_effects: {
        filesystem_written: false,
        evidence_persisted: false,
        ephemeral_capture_cache_written: true,
        service_restarted: false,
        launchd_mutated: false,
        gateway_mutation_performed: false,
        credential_read: false,
        secret_file_read: false,
        external_send_performed: false
      }
    } else false end
  '

echo "Hepta JSON report capture diagnostic contract gate passed"
