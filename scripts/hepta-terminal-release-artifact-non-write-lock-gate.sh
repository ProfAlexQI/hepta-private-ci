#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

ACTIVE_STATE_LOCK_JSON="$(
  capture_json_report \
    "hepta-terminal-governance-active-state-lock-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-governance-active-state-lock-gate.sh
)"

NATIVE_PACKAGING_JSON="$(
  capture_json_report \
    "hepta-native-packaging-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-native-packaging-gate.sh
)"

RELEASE_HARDENING_JSON="$(
  capture_json_report \
    "hepta-release-hardening-status-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-release-hardening-status-gate.sh
)"

active_state_lock_report_sha256="$(sha256_text "$ACTIVE_STATE_LOCK_JSON")"
native_packaging_report_sha256="$(sha256_text "$NATIVE_PACKAGING_JSON")"
release_hardening_report_sha256="$(sha256_text "$RELEASE_HARDENING_JSON")"
release_artifact_lock_hash_sha256="$(sha256_text "hepta-terminal-release-artifact-non-write-lock:index:$active_state_lock_report_sha256:$native_packaging_report_sha256:$release_hardening_report_sha256")"
release_artifact_policy_hash_sha256="$(sha256_text "hepta-terminal-release-artifact-non-write-lock:policy:$active_state_lock_report_sha256:$native_packaging_report_sha256:$release_hardening_report_sha256")"
release_artifact_side_effect_hash_sha256="$(sha256_text "hepta-terminal-release-artifact-non-write-lock:side-effects:$active_state_lock_report_sha256:$native_packaging_report_sha256:$release_hardening_report_sha256")"

jq -n -e \
  --argjson active "$ACTIVE_STATE_LOCK_JSON" \
  --argjson native "$NATIVE_PACKAGING_JSON" \
  --argjson release "$RELEASE_HARDENING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $active.runtime == "hepta"
    and $active.status == "ready"
    and $active.gate == "hepta_terminal_governance_active_state_lock_gate"
    and $active.active_state_lock_ready == true
    and $active.active_state_lock_mode == "schema_only_active_state_locked_no_runtime_mutation"
    and $active.active_state_lock_decision == "active_service_state_observed_without_install_restart_or_dependency_mutation"
    and $active.active_state_lock_denied_by_count == 73
    and $active.active_runtime_evidence_contract_ready == true
    and $active.install_execution_allowed == false
    and $active.release_build_required == false
    and $active.active_binary_mutation_allowed == false
    and $active.active_service_restart_allowed == false
    and $active.public_release_claim_allowed == false
    and $active.public_ga_claim_allowed == false
    and $active.release_artifact_write_allowed == false
    and $active.public_artifact_write_allowed == false
    and $active.live_mutation_execution_ready == false
    and ($active.side_effects | to_entries | all(.value == false))
    and $native.runtime == "hepta"
    and $native.status == "ready"
    and $native.local_packaging_gate_ready == true
    and $native.hepta_native_release_packaging_ready == true
    and $native.signing_notarization_deferred == true
    and $native.public_distribution_artifact_written == false
    and $native.reports_synchronized == true
    and $native.missing_route_count == 0
    and $native.native_gateway_source_command_count >= 69
    and ($native.side_effects | to_entries | all(.value == false))
    and $release.runtime == "hepta"
    and $release.status == "ready"
    and $release.release_hardening_status_gate_ready == true
    and $release.local_status_gate_ready_count == 12
    and $release.live_execution_enabled_count == 0
    and $release.external_production_gate_count == 3
    and $release.filesystem_artifact_write_required_count == 2
    and $release.operator_approval_required_count == 12
    and $release.reports_synchronized == true
    and $release.missing_route_count == 0
    and $release.native_gateway_source_command_count >= 69
    and ($release.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_terminal_release_artifact_non_write_lock_gate" \
  --arg active_state_lock_report_sha256 "$active_state_lock_report_sha256" \
  --arg native_packaging_report_sha256 "$native_packaging_report_sha256" \
  --arg release_hardening_report_sha256 "$release_hardening_report_sha256" \
  --arg release_artifact_lock_hash_sha256 "$release_artifact_lock_hash_sha256" \
  --arg release_artifact_policy_hash_sha256 "$release_artifact_policy_hash_sha256" \
  --arg release_artifact_side_effect_hash_sha256 "$release_artifact_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson active "$ACTIVE_STATE_LOCK_JSON" \
  --argjson native "$NATIVE_PACKAGING_JSON" \
  --argjson release "$RELEASE_HARDENING_JSON" \
  '
    ([
      "terminal_release_artifact_non_write_lock_recording_denied",
      "terminal_release_artifact_non_write_lock_persistence_denied",
      "terminal_release_artifact_non_write_lock_materialization_denied",
      "terminal_release_artifact_non_write_lock_filesystem_write_denied",
      "native_public_distribution_artifact_write_denied",
      "native_signing_execution_denied",
      "native_notarization_execution_denied",
      "native_stapling_execution_denied",
      "release_artifact_pack_execution_denied",
      "external_production_gate_execution_denied",
      "release_hardening_filesystem_artifact_write_denied",
      "release_hardening_launchd_mutation_denied",
      "recurring_watchdog_install_denied",
      "operator_approval_required_before_release_artifact_write"
    ] + $active.denied_by_active_state_lock) as $release_artifact_denied
    | ([
        ($release.external_production_gate_enabled | if . then 1 else 0 end),
        ($release.release_artifact_pack_enabled | if . then 1 else 0 end),
        ($release.launchd_service_mutation_enabled | if . then 1 else 0 end),
        ($release.recurring_watchdog_install_enabled | if . then 1 else 0 end),
        ($release.local_import_execution_enabled | if . then 1 else 0 end)
      ] | add) as $release_hardening_enabled_surface_count
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_release_artifact_non_write_lock_schema_version:"terminal_release_artifact_non_write_lock_v1",
      minimum_required_samples:$min_long_soak_samples,
      release_artifact_non_write_lock_ready:true,
      release_artifact_non_write_lock_mode:"schema_only_release_artifact_write_blocked",
      release_artifact_non_write_lock_decision:"release_artifact_and_public_distribution_state_observed_without_build_install_sign_notarize_publish_or_restart",
      source_active_state_lock_gate:$active.gate,
      source_native_packaging_endpoint:$native.endpoint,
      source_release_hardening_endpoint:$release.endpoint,
      source_active_state_lock_report_sha256:$active_state_lock_report_sha256,
      source_native_packaging_report_sha256:$native_packaging_report_sha256,
      source_release_hardening_report_sha256:$release_hardening_report_sha256,
      required_source_count:3,
      ready_source_count:3,
      activation_blocking_source_count:3,
      source_active_state_lock_ready:$active.active_state_lock_ready,
      source_active_state_lock_denied_by_count:$active.active_state_lock_denied_by_count,
      source_active_state_lock_family_count:($active.active_state_lock_families | length),
      source_active_runtime_evidence_contract_ready:$active.active_runtime_evidence_contract_ready,
      source_active_binary_sha_consistency_checked:$active.source_watchdog_binary_sha_match_checked,
      source_active_state_installed_sha256:$active.source_watchdog_installed_sha256,
      source_active_state_release_sha256:$active.source_watchdog_release_sha256,
      source_native_packaging_ready:$native.local_packaging_gate_ready,
      source_native_release_packaging_ready:$native.hepta_native_release_packaging_ready,
      source_native_signing_notarization_deferred:$native.signing_notarization_deferred,
      source_native_public_distribution_artifact_written:$native.public_distribution_artifact_written,
      source_native_rust_source_file_count:$native.rust_source_file_count,
      source_native_packaging_resource_file_count:$native.packaging_resource_file_count,
      source_release_hardening_ready:$release.release_hardening_status_gate_ready,
      source_release_hardening_status_gate_count:$release.status_gate_count,
      source_release_hardening_local_status_gate_ready_count:$release.local_status_gate_ready_count,
      source_release_hardening_live_execution_enabled_count:$release.live_execution_enabled_count,
      source_release_hardening_external_production_gate_count:$release.external_production_gate_count,
      source_release_hardening_filesystem_artifact_write_required_count:$release.filesystem_artifact_write_required_count,
      source_release_hardening_launchd_mutation_required_count:$release.launchd_mutation_required_count,
      source_release_hardening_operator_approval_required_count:$release.operator_approval_required_count,
      source_release_hardening_enabled_surface_count:$release_hardening_enabled_surface_count,
      source_reports_synchronized:($native.reports_synchronized and $release.reports_synchronized),
      active_state_observed:true,
      active_runtime_evidence_contract_ready:$active.active_runtime_evidence_contract_ready,
      active_binary_sha_consistent:$active.active_binary_sha_consistent,
      native_packaging_state_observed:true,
      release_hardening_state_observed:true,
      release_artifact_write_lock_enforced:true,
      public_distribution_write_lock_enforced:true,
      signing_notarization_execution_lock_enforced:true,
      readiness_allowed:false,
      activation_allowed:false,
      active_wiring_allowed:false,
      active_runtime_auto_rebase_allowed:false,
      active_runtime_codex_engine_dependency_allowed:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      install_execution_allowed:false,
      release_build_required:false,
      release_build_executed:false,
      active_binary_mutation_allowed:false,
      active_service_restart_allowed:false,
      launchd_restart_allowed:false,
      rollback_execution_allowed:false,
      rollback_restore_allowed:false,
      native_packaging_execution_allowed:false,
      native_signing_allowed:false,
      native_notarization_allowed:false,
      native_stapling_allowed:false,
      public_distribution_artifact_write_allowed:false,
      release_artifact_pack_execution_allowed:false,
      recurring_watchdog_install_allowed:false,
      live_mutation_execution_ready:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      release_artifact_write_allowed:false,
      public_artifact_write_allowed:false,
      terminal_release_artifact_non_write_lock_recorded:false,
      terminal_release_artifact_non_write_lock_persisted:false,
      terminal_release_artifact_non_write_lock_materialized:false,
      terminal_release_artifact_non_write_lock_filesystem_written:false,
      release_artifact_lock_hash_sha256:$release_artifact_lock_hash_sha256,
      release_artifact_policy_hash_sha256:$release_artifact_policy_hash_sha256,
      release_artifact_side_effect_hash_sha256:$release_artifact_side_effect_hash_sha256,
      release_artifact_lock_source_hashes:[
        $active_state_lock_report_sha256,
        $native_packaging_report_sha256,
        $release_hardening_report_sha256
      ],
      release_artifact_lock_families:[
        {
          id:"active-state-lock-source",
          ready:true,
          blocked:true,
          denied_by_count:$active.active_state_lock_denied_by_count,
          reason:"active-state lock denies install, restart, rollback, active dependency mutation, public claims, artifact writes, and live mutation"
        },
        {
          id:"native-packaging-non-distribution-boundary",
          ready:true,
          blocked:true,
          local_packaging_gate_ready:$native.local_packaging_gate_ready,
          signing_notarization_deferred:$native.signing_notarization_deferred,
          public_distribution_artifact_written:$native.public_distribution_artifact_written,
          reason:"native packaging readiness is observational and does not sign, notarize, staple, or write public distribution artifacts"
        },
        {
          id:"release-hardening-status-non-execution-boundary",
          ready:true,
          blocked:true,
          status_gate_count:$release.status_gate_count,
          live_execution_enabled_count:$release.live_execution_enabled_count,
          operator_approval_required_count:$release.operator_approval_required_count,
          reason:"release hardening status is ready but no live execution surface has executed"
        },
        {
          id:"release-artifact-write-boundary",
          ready:true,
          blocked:true,
          release_artifact_write_allowed:false,
          public_artifact_write_allowed:false,
          public_distribution_artifact_write_allowed:false,
          reason:"release, public, and native distribution artifact writes remain denied"
        },
        {
          id:"signing-notarization-launchd-boundary",
          ready:true,
          blocked:true,
          native_signing_allowed:false,
          native_notarization_allowed:false,
          launchd_restart_allowed:false,
          reason:"signing, notarization, launchd mutation, recurring watchdog install, and service restart remain denied"
        },
        {
          id:"terminal-release-artifact-lock-persistence-boundary",
          ready:true,
          blocked:true,
          terminal_release_artifact_non_write_lock_recorded:false,
          terminal_release_artifact_non_write_lock_persisted:false,
          terminal_release_artifact_non_write_lock_materialized:false,
          terminal_release_artifact_non_write_lock_filesystem_written:false,
          reason:"release-artifact non-write lock is report-only and not persisted or materialized"
        }
      ],
      release_artifact_non_write_denied_by_count:($release_artifact_denied | length),
      denied_by_release_artifact_non_write_lock:$release_artifact_denied,
      side_effects:{
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        coding_agent_spawned:false,
        skill_workshop_written:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        runtime_store_mutated:false,
        gateway_event_enqueued:false,
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        upstream_checkout_performed:false,
        active_runtime_auto_rebase_performed:false,
        active_runtime_dependency_mutated:false,
        install_executed:false,
        release_build_executed:false,
        native_packaging_executed:false,
        native_signing_executed:false,
        native_notarization_executed:false,
        native_stapling_executed:false,
        public_distribution_artifact_written:false,
        active_binary_mutated:false,
        active_service_restart:false,
        launchd_mutated:false,
        recurring_watchdog_installed:false,
        release_artifact_pack_executed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        public_release_published:false,
        public_ga_claimed:false,
        external_public_claim_performed:false,
        terminal_release_artifact_non_write_lock_recorded:false,
        terminal_release_artifact_non_write_lock_persisted:false,
        terminal_release_artifact_non_write_lock_materialized:false,
        terminal_release_artifact_non_write_lock_filesystem_written:false,
        active_state_lock_recorded:false,
        active_state_lock_persisted:false,
        active_state_lock_materialized:false,
        active_state_lock_filesystem_written:false,
        filesystem_written:false,
        workspace_write_performed:false,
        external_send_performed:false,
        credential_read:false,
        secret_file_read:false
      }
    }')"

printf '%s\n' "$report"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .release_artifact_non_write_lock_ready == true
  and .source_reports_synchronized == true
  and .source_active_runtime_evidence_contract_ready == true
  and .active_runtime_evidence_contract_ready == true
  and (
    (.source_active_binary_sha_consistency_checked == true and .active_binary_sha_consistent == true)
    or (.source_active_binary_sha_consistency_checked == false and .active_binary_sha_consistent == null)
  )
  and .source_native_packaging_ready == true
  and .source_native_release_packaging_ready == true
  and .source_native_public_distribution_artifact_written == false
  and .source_release_hardening_ready == true
  and .source_release_hardening_live_execution_enabled_count == 0
  and .release_artifact_write_lock_enforced == true
  and .public_distribution_write_lock_enforced == true
  and .release_artifact_non_write_denied_by_count == 87
  and (.release_artifact_lock_families | length) == 6
  and (.release_artifact_lock_families | all(.ready == true and .blocked == true))
  and .release_build_executed == false
  and .native_packaging_execution_allowed == false
  and .native_signing_allowed == false
  and .native_notarization_allowed == false
  and .public_distribution_artifact_write_allowed == false
  and .release_artifact_pack_execution_allowed == false
  and .release_artifact_write_allowed == false
  and .public_artifact_write_allowed == false
  and .terminal_release_artifact_non_write_lock_recorded == false
  and .terminal_release_artifact_non_write_lock_persisted == false
  and .terminal_release_artifact_non_write_lock_materialized == false
  and .terminal_release_artifact_non_write_lock_filesystem_written == false
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta terminal release artifact non-write lock gate passed"
