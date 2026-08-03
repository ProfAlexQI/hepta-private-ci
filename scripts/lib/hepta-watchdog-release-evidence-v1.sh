#!/usr/bin/env bash

# Candidate/deployed release evidence boundary for scripts/hepta-watchdog.sh.
# The live watchdog intentionally delegates all manifest parsing, provenance
# validation, deployment comparison, and non-live report assembly here.

hepta_watchdog_validate_release_evidence() {
  local repo_root="$1"
  local role="$2"
  local binary="$3"
  local manifest="$4"
  local expected_source_commit="$5"
  local failure_reasons="[]"
  local binary_present=false binary_executable=false binary_sha=""
  local manifest_present=false manifest_contract_valid=false manifest_bound=false
  local manifest_resolved="" manifest_artifact="" manifest_artifact_sha="" manifest_source_commit=""
  local manifest_sha="" release_id="" toolchain_sha="" dependency_sha="" preflight_log_sha=""
  local binary_matches_manifest=false manifest_targets_binary=false

  hepta_watchdog_release_evidence_add_failure_reason() {
    failure_reasons="$(
      jq -cn \
        --argjson reasons "$failure_reasons" \
        --arg reason "$1" \
        '$reasons + [$reason]'
    )"
  }

  if [[ -f "$binary" ]]; then
    binary_present=true
    binary_sha="$(shasum -a 256 "$binary" | awk '{print $1}')"
    if [[ -x "$binary" ]]; then
      binary_executable=true
    else
      hepta_watchdog_release_evidence_add_failure_reason "${role}_binary_not_executable"
    fi
  else
    hepta_watchdog_release_evidence_add_failure_reason "${role}_binary_missing"
  fi

  if [[ -f "$manifest" ]]; then
    manifest_present=true
    manifest_resolved="$(realpath "$manifest")"
    manifest_sha="$(shasum -a 256 "$manifest" | awk '{print $1}')"
    if "$repo_root/scripts/hepta-immutable-release-tree" verify --manifest "$manifest" >/dev/null 2>&1; then
      manifest_contract_valid=true
    else
      hepta_watchdog_release_evidence_add_failure_reason "${role}_manifest_contract_invalid"
    fi

    if jq -e '
      .status == "ready"
      and .source.commit_bound == true
      and (.source.commit | test("^[0-9a-f]{40}$"))
      and .preflight.bound == true
      and .preflight.passed == true
      and (.preflight.log_sha256 | test("^[0-9a-f]{64}$"))
      and (if .policy.release_contract_version == 4 then
        .policy.runtime_companions_required == true
        and .build_provenance.schema_version == "hepta_build_provenance_v2"
        and .build_provenance.runtime_companions.bound == true
        and (.build_provenance.runtime_companions.aggregate_sha256 | test("^[0-9a-f]{64}$"))
        and (.build_provenance.runtime_companions.artifacts | length) == 1
        and .build_provenance.runtime_companions.artifacts[0].id == "code-mode-host"
        and (.build_provenance.runtime_companions.artifacts[0].name == "codex-code-mode-host"
          or .build_provenance.runtime_companions.artifacts[0].name == "codex-code-mode-host.exe")
        and (.runtime_companions | length) == 1
        and .runtime_companions[0].id == "code-mode-host"
        and .runtime_companions[0].sha256 ==
          .build_provenance.runtime_companions.artifacts[0].sha256
        and (.runtime_companions[0].relative_path | split("/")[-1]) ==
          .build_provenance.runtime_companions.artifacts[0].name
      else
        .build_provenance.schema_version == "hepta_build_provenance_v1"
      end)
      and .build_provenance.source.commit_bound == true
      and .build_provenance.source.commit == .source.commit
      and .build_provenance.toolchain.bound == true
      and (.build_provenance.toolchain.aggregate_sha256 | test("^[0-9a-f]{64}$"))
      and .build_provenance.dependencies.bound == true
      and (.build_provenance.dependencies.aggregate_sha256 | test("^[0-9a-f]{64}$"))
      and .build_provenance.artifact.bound == true
      and .build_provenance.artifact.sha256 == .artifact.sha256
      and .build_provenance.preflight_profiles.backend == true
      and .build_provenance.preflight_profiles.native == true
      and .build_provenance.preflight_profiles.release == true
    ' "$manifest" >/dev/null 2>&1; then
      manifest_bound=true
    else
      hepta_watchdog_release_evidence_add_failure_reason \
        "${role}_manifest_not_source_toolchain_dependency_preflight_bound"
    fi

    manifest_artifact_sha="$(jq -r '.artifact.sha256 // ""' "$manifest" 2>/dev/null || true)"
    manifest_source_commit="$(jq -r '.source.commit // ""' "$manifest" 2>/dev/null || true)"
    release_id="$(jq -r '.release_id // ""' "$manifest" 2>/dev/null || true)"
    toolchain_sha="$(
      jq -r '.build_provenance.toolchain.aggregate_sha256 // ""' "$manifest" 2>/dev/null || true
    )"
    dependency_sha="$(
      jq -r '.build_provenance.dependencies.aggregate_sha256 // ""' "$manifest" 2>/dev/null || true
    )"
    preflight_log_sha="$(jq -r '.preflight.log_sha256 // ""' "$manifest" 2>/dev/null || true)"
    manifest_artifact="$(
      jq -r '.artifact.relative_path // ""' "$manifest" 2>/dev/null || true
    )"
    if [[ -n "$manifest_artifact" ]]; then
      manifest_artifact="$(dirname "$manifest_resolved")/$manifest_artifact"
      if [[ -f "$manifest_artifact" && -f "$binary" ]]; then
        if [[ "$(realpath "$manifest_artifact")" == "$(realpath "$binary")" ]]; then
          manifest_targets_binary=true
        else
          hepta_watchdog_release_evidence_add_failure_reason \
            "${role}_manifest_targets_different_binary"
        fi
      else
        hepta_watchdog_release_evidence_add_failure_reason "${role}_manifest_artifact_missing"
      fi
    else
      hepta_watchdog_release_evidence_add_failure_reason "${role}_manifest_artifact_path_missing"
    fi

    if [[ -n "$binary_sha" && "$binary_sha" == "$manifest_artifact_sha" ]]; then
      binary_matches_manifest=true
    else
      hepta_watchdog_release_evidence_add_failure_reason "${role}_binary_manifest_sha_mismatch"
    fi

    if [[ -n "$expected_source_commit" && "$manifest_source_commit" != "$expected_source_commit" ]]; then
      hepta_watchdog_release_evidence_add_failure_reason "${role}_source_commit_mismatch"
    fi
  else
    hepta_watchdog_release_evidence_add_failure_reason "${role}_manifest_missing"
  fi

  local ready=false
  if [[ "$(jq 'length' <<<"$failure_reasons")" == "0" ]]; then
    ready=true
  fi

  jq -cn \
    --arg role "$role" \
    --arg binary "$binary" \
    --arg manifest "$manifest" \
    --arg manifest_resolved "$manifest_resolved" \
    --arg manifest_sha256 "$manifest_sha" \
    --arg binary_sha256 "$binary_sha" \
    --arg manifest_artifact "$manifest_artifact" \
    --arg manifest_artifact_sha256 "$manifest_artifact_sha" \
    --arg source_commit "$manifest_source_commit" \
    --arg release_id "$release_id" \
    --arg toolchain_sha256 "$toolchain_sha" \
    --arg dependency_sha256 "$dependency_sha" \
    --arg preflight_log_sha256 "$preflight_log_sha" \
    --arg expected_source_commit "$expected_source_commit" \
    --argjson binary_present "$binary_present" \
    --argjson binary_executable "$binary_executable" \
    --argjson manifest_present "$manifest_present" \
    --argjson manifest_contract_valid "$manifest_contract_valid" \
    --argjson manifest_bound "$manifest_bound" \
    --argjson manifest_targets_binary "$manifest_targets_binary" \
    --argjson binary_matches_manifest "$binary_matches_manifest" \
    --argjson ready "$ready" \
    --argjson failure_reasons "$failure_reasons" \
    '{
      role:$role,
      status:(if $ready then "ready" else "failed" end),
      ready:$ready,
      binary:$binary,
      binary_present:$binary_present,
      binary_executable:$binary_executable,
      binary_sha256:$binary_sha256,
      manifest:$manifest,
      manifest_resolved:$manifest_resolved,
      manifest_sha256:$manifest_sha256,
      manifest_present:$manifest_present,
      manifest_contract_valid:$manifest_contract_valid,
      manifest_source_toolchain_dependency_preflight_bound:$manifest_bound,
      manifest_artifact:$manifest_artifact,
      manifest_artifact_sha256:$manifest_artifact_sha256,
      manifest_targets_binary:$manifest_targets_binary,
      binary_matches_manifest:$binary_matches_manifest,
      source_commit:$source_commit,
      release_id:$release_id,
      toolchain_sha256:$toolchain_sha256,
      dependency_sha256:$dependency_sha256,
      preflight_log_sha256:$preflight_log_sha256,
      expected_source_commit:(if $expected_source_commit == "" then null else $expected_source_commit end),
      failure_reasons:$failure_reasons
    }'
}

hepta_watchdog_release_evidence_bundle() {
  local repo_root="$1"
  local require_candidate="$2"
  local release_bin="$3"
  local candidate_manifest="$4"
  local require_deployed="$5"
  local installed_bin="$6"
  local installed_receipt="$7"
  local expected_source_commit="$8"
  local require_deployment_match="$9"

  local release_sha="" installed_sha=""
  local candidate_evidence='{"status":"not_checked","ready":null,"failure_reasons":[]}'
  local deployed_evidence='{"status":"not_checked","ready":null,"failure_reasons":[]}'
  if [[ "$require_candidate" == "true" ]]; then
    candidate_evidence="$(
      hepta_watchdog_validate_release_evidence \
        "$repo_root" candidate "$release_bin" "$candidate_manifest" "$expected_source_commit"
    )"
    release_sha="$(jq -r '.binary_sha256' <<<"$candidate_evidence")"
  fi
  if [[ "$require_deployed" == "true" ]]; then
    deployed_evidence="$(
      hepta_watchdog_validate_release_evidence \
        "$repo_root" deployed "$installed_bin" "$installed_receipt" "$expected_source_commit"
    )"
    installed_sha="$(jq -r '.binary_sha256' <<<"$deployed_evidence")"
  fi

  local failure_reasons binary_sha_match=false
  failure_reasons="$(
    jq -cn \
      --argjson candidate "$candidate_evidence" \
      --argjson deployed "$deployed_evidence" \
      '$candidate.failure_reasons + $deployed.failure_reasons'
  )"
  if [[ -n "$release_sha" && "$release_sha" == "$installed_sha" ]]; then
    binary_sha_match=true
  fi
  if [[ "$require_deployment_match" == "true" ]]; then
    if [[ -n "$release_sha" && -n "$installed_sha" && "$binary_sha_match" != "true" ]]; then
      failure_reasons="$(
        jq -cn \
          --argjson reasons "$failure_reasons" \
          '$reasons + ["candidate_installed_sha_mismatch"]'
      )"
    fi
    if [[ "$(jq -r '.ready' <<<"$candidate_evidence")" == "true" \
      && "$(jq -r '.ready' <<<"$deployed_evidence")" == "true" ]]; then
      failure_reasons="$(
        jq -cn \
          --argjson reasons "$failure_reasons" \
          --argjson candidate "$candidate_evidence" \
          --argjson deployed "$deployed_evidence" \
          '$reasons
          + (if $candidate.source_commit == $deployed.source_commit then [] else ["candidate_installed_source_commit_mismatch"] end)
          + (if $candidate.toolchain_sha256 == $deployed.toolchain_sha256 then [] else ["candidate_installed_toolchain_mismatch"] end)
          + (if $candidate.dependency_sha256 == $deployed.dependency_sha256 then [] else ["candidate_installed_dependency_mismatch"] end)
          + (if $candidate.release_id == $deployed.release_id then [] else ["candidate_installed_release_id_mismatch"] end)
          + (if $candidate.manifest_sha256 == $deployed.manifest_sha256 then [] else ["candidate_installed_receipt_mismatch"] end)'
      )"
    fi
  fi

  local ready=false
  if [[ "$(jq 'length' <<<"$failure_reasons")" == "0" ]]; then
    ready=true
  fi
  jq -cn \
    --arg release_sha "$release_sha" \
    --arg installed_sha "$installed_sha" \
    --argjson candidate "$candidate_evidence" \
    --argjson deployed "$deployed_evidence" \
    --argjson binary_sha_match "$binary_sha_match" \
    --argjson failure_reasons "$failure_reasons" \
    --argjson ready "$ready" \
    '{
      ready:$ready,
      release_sha256:$release_sha,
      installed_sha256:$installed_sha,
      binary_sha_match:$binary_sha_match,
      candidate:$candidate,
      deployed:$deployed,
      failure_reasons:$failure_reasons
    }'
}

hepta_watchdog_release_evidence_report() {
  local base_url="$1"
  local mode="$2"
  local active_health_required="$3"
  local candidate_required="$4"
  local deployed_required="$5"
  local deployment_match_required="$6"
  local evidence="$7"

  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$base_url" \
    --arg mode "$mode" \
    --argjson active_health_required "$active_health_required" \
    --argjson candidate_required "$candidate_required" \
    --argjson deployed_required "$deployed_required" \
    --argjson deployment_match_required "$deployment_match_required" \
    --argjson evidence "$evidence" \
    '{
      product:$product,
      runtime:$runtime,
      base_url:$base_url,
      watchdog_mode:$mode,
      status:(if $evidence.ready then "ok" else "failed" end),
      active_health:{required:$active_health_required,status:"not_checked"},
      candidate_artifact:{required:$candidate_required,evidence:$evidence.candidate},
      deployed_receipt:{required:$deployed_required,evidence:$evidence.deployed},
      deployment_consistency_required:$deployment_match_required,
      release_sha256:$evidence.release_sha256,
      installed_sha256:$evidence.installed_sha256,
      binary_sha_match:$evidence.binary_sha_match,
      failure_reasons:$evidence.failure_reasons,
      side_effects:{
        live_endpoint_read:false,
        service_restarted:false,
        active_process_mutated:false,
        installed_binary_mutated:false,
        release_artifact_mutated:false
      }
    }'
}
