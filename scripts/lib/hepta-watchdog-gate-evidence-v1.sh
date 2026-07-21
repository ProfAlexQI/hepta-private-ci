#!/usr/bin/env bash

# Mode-aware evidence contract for read-only gates that call hepta-watchdog.
# Standalone gates stay fail-closed on deployment-consistency. Candidate
# preflight may explicitly select active-health before a source-bound release
# artifact and deployed receipt exist; that mode never makes an artifact or
# deployment-consistency claim.

hepta_watchdog_gate_mode() {
  local mode="${HEPTA_WATCHDOG_GATE_MODE:-deployment-consistency}"
  case "$mode" in
    deployment-consistency|active-health)
      printf '%s\n' "$mode"
      ;;
    *)
      echo "unsupported HEPTA_WATCHDOG_GATE_MODE: $mode" >&2
      return 2
      ;;
  esac
}

hepta_watchdog_gate_evidence_contract_json() {
  local watchdog_json="$1"
  local expected_mode="$2"

  jq -cn \
    --arg expected_mode "$expected_mode" \
    --argjson watchdog "$watchdog_json" \
    '
      (
        $watchdog.status == "ok"
        or (
          $watchdog.status == "failed"
          and $watchdog.operator_security_status == "attention"
        )
      ) as $status_known
      | (
        $watchdog.active_health.required == true
        and $watchdog.active_health.status == "ready"
        and $watchdog.health == "ready"
      ) as $active_health_ready
      | (
        $watchdog.candidate_artifact.required == true
        and $watchdog.candidate_artifact.evidence.ready == true
        and $watchdog.deployed_receipt.required == true
        and $watchdog.deployed_receipt.evidence.ready == true
        and $watchdog.deployment_consistency_required == true
        and $watchdog.binary_sha_match == true
        and ($watchdog.release_sha256 | test("^[0-9a-f]{64}$"))
        and ($watchdog.installed_sha256 | test("^[0-9a-f]{64}$"))
        and $watchdog.release_sha256 == $watchdog.installed_sha256
      ) as $deployment_consistency_ready
      | (
        $watchdog.candidate_artifact.required == false
        and $watchdog.candidate_artifact.evidence.status == "not_checked"
        and $watchdog.deployed_receipt.required == false
        and $watchdog.deployed_receipt.evidence.status == "not_checked"
        and $watchdog.deployment_consistency_required == false
        and $watchdog.binary_sha_match == false
        and $watchdog.release_sha256 == ""
        and $watchdog.installed_sha256 == ""
      ) as $active_health_only_ready
      | {
          schema_version:"hepta_watchdog_gate_evidence_v1",
          expected_mode:$expected_mode,
          observed_mode:$watchdog.watchdog_mode,
          status_known:$status_known,
          active_health_ready:$active_health_ready,
          active_health_only:($expected_mode == "active-health"),
          deployment_consistency_checked:($expected_mode == "deployment-consistency"),
          binary_sha_match_checked:($expected_mode == "deployment-consistency"),
          binary_sha_match:$watchdog.binary_sha_match,
          artifact_evidence_ready:(
            if $expected_mode == "deployment-consistency"
            then $deployment_consistency_ready
            else $active_health_only_ready
            end
          ),
          ready:(
            $watchdog.watchdog_mode == $expected_mode
            and $status_known
            and $active_health_ready
            and (
              if $expected_mode == "deployment-consistency"
              then $deployment_consistency_ready
              else $active_health_only_ready
              end
            )
          )
        }
    '
}
