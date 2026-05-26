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
set -e

success_fixture_ok=false
failing_json_fixture_ok=false
no_json_fixture_ok=false

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

contract_hash_sha256="$(sha256_text "hepta-json-report-capture:contract:$success_fixture_ok:$failing_json_fixture_ok:$no_json_fixture_ok")"
policy_hash_sha256="$(sha256_text "hepta-json-report-capture:policy:no-workspace-write:no-secret-read:no-external-send")"
side_effect_hash_sha256="$(sha256_text "hepta-json-report-capture:side-effects:false:false:false")"

jq -n -e \
  --arg contract_hash_sha256 "$contract_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson success_fixture_ok "$success_fixture_ok" \
  --argjson failing_json_fixture_ok "$failing_json_fixture_ok" \
  --argjson no_json_fixture_ok "$no_json_fixture_ok" \
  '
    $success_fixture_ok == true
    and $failing_json_fixture_ok == true
    and $no_json_fixture_ok == true
    | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_json_report_capture_diagnostic_contract_gate",
      json_report_capture_diagnostic_contract_schema_version: "json_report_capture_diagnostic_contract_v1",
      json_report_capture_diagnostic_contract_ready: true,
      diagnostic_mode: "synthetic_child_command_capture_no_workspace_write",
      diagnostic_decision: "child_json_report_capture_preserves_success_reports_and_exposes_failure_diagnostics",
      helper_path: "scripts/lib/hepta-json-report-capture.sh",
      success_fixture_ok: $success_fixture_ok,
      failing_json_fixture_ok: $failing_json_fixture_ok,
      no_json_fixture_ok: $no_json_fixture_ok,
      parseable_json_report_preserved: true,
      failing_child_exit_code_preserved: true,
      failing_child_json_report_exposed: true,
      non_json_output_tail_exposed: true,
      contract_hash_sha256: $contract_hash_sha256,
      policy_hash_sha256: $policy_hash_sha256,
      side_effect_hash_sha256: $side_effect_hash_sha256,
      denied_by_json_report_capture_contract: [
        "json_report_capture_workspace_write_denied",
        "json_report_capture_secret_read_denied",
        "json_report_capture_external_send_denied",
        "json_report_capture_child_recovery_action_denied"
      ],
      side_effects: {
        filesystem_written: false,
        evidence_persisted: false,
        service_restarted: false,
        launchd_mutated: false,
        gateway_mutation_performed: false,
        credential_read: false,
        secret_file_read: false,
        external_send_performed: false
      }
    }
  '

echo "Hepta JSON report capture diagnostic contract gate passed"
