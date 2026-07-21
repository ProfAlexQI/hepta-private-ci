#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
source "$ROOT/scripts/lib/hepta-watchdog-gate-evidence-v1.sh"

base_report='{
  "status":"ok",
  "operator_security_status":"ready",
  "active_health":{"required":true,"status":"ready"},
  "health":"ready"
}'

active_report="$(
  jq -c '. + {
    watchdog_mode:"active-health",
    candidate_artifact:{required:false,evidence:{status:"not_checked",ready:null}},
    deployed_receipt:{required:false,evidence:{status:"not_checked",ready:null}},
    deployment_consistency_required:false,
    binary_sha_match:false,
    release_sha256:"",
    installed_sha256:""
  }' <<<"$base_report"
)"
active_contract="$(hepta_watchdog_gate_evidence_contract_json "$active_report" active-health)"
jq -e '
  .ready == true
  and .active_health_only == true
  and .deployment_consistency_checked == false
  and .binary_sha_match_checked == false
  and .binary_sha_match == false
' >/dev/null <<<"$active_contract"

sha='aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
deployment_report="$(
  jq -c --arg sha "$sha" '. + {
    watchdog_mode:"deployment-consistency",
    candidate_artifact:{required:true,evidence:{status:"ready",ready:true}},
    deployed_receipt:{required:true,evidence:{status:"ready",ready:true}},
    deployment_consistency_required:true,
    binary_sha_match:true,
    release_sha256:$sha,
    installed_sha256:$sha
  }' <<<"$base_report"
)"
deployment_contract="$(
  hepta_watchdog_gate_evidence_contract_json "$deployment_report" deployment-consistency
)"
jq -e '
  .ready == true
  and .active_health_only == false
  and .deployment_consistency_checked == true
  and .binary_sha_match_checked == true
  and .binary_sha_match == true
' >/dev/null <<<"$deployment_contract"

mismatch_report="$(jq -c '.installed_sha256 = ("b" * 64)' <<<"$deployment_report")"
jq -e '.ready == false and .artifact_evidence_ready == false' >/dev/null \
  <<<"$(hepta_watchdog_gate_evidence_contract_json "$mismatch_report" deployment-consistency)"

wrong_mode_contract="$(
  hepta_watchdog_gate_evidence_contract_json "$active_report" deployment-consistency
)"
jq -e '.ready == false and .observed_mode == "active-health"' >/dev/null \
  <<<"$wrong_mode_contract"

unsupported_rc=0
HEPTA_WATCHDOG_GATE_MODE=candidate-artifact hepta_watchdog_gate_mode >/dev/null 2>&1 \
  || unsupported_rc=$?
[[ "$unsupported_rc" == "2" ]]

echo "Hepta watchdog gate evidence self-test passed"
