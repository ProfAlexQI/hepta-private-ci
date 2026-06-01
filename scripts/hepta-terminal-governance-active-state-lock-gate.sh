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

GOVERNANCE_CLOSURE_JSON="$(
  capture_json_report \
    "hepta-terminal-governance-closure-summary-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-governance-closure-summary-gate.sh
)"

WATCHDOG_JSON="$(
  capture_json_report \
    "hepta-watchdog" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-watchdog.sh
)"

DEPENDENCY_ISOLATION_JSON="$(
  capture_json_report \
    "hepta-active-service-dependency-isolation" \
    env HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=0 \
      scripts/hepta-active-service-dependency-isolation.sh
)"

governance_closure_report_sha256="$(sha256_text "$GOVERNANCE_CLOSURE_JSON")"
watchdog_report_sha256="$(sha256_text "$WATCHDOG_JSON")"
dependency_isolation_report_sha256="$(sha256_text "$DEPENDENCY_ISOLATION_JSON")"
active_state_lock_hash_sha256="$(sha256_text "hepta-terminal-governance-active-state-lock:index:$governance_closure_report_sha256:$watchdog_report_sha256:$dependency_isolation_report_sha256")"
active_state_lock_policy_hash_sha256="$(sha256_text "hepta-terminal-governance-active-state-lock:policy:$governance_closure_report_sha256:$watchdog_report_sha256:$dependency_isolation_report_sha256")"
active_state_lock_side_effect_hash_sha256="$(sha256_text "hepta-terminal-governance-active-state-lock:side-effects:$governance_closure_report_sha256:$watchdog_report_sha256:$dependency_isolation_report_sha256")"

jq -n -e \
  --argjson governance "$GOVERNANCE_CLOSURE_JSON" \
  --argjson watchdog "$WATCHDOG_JSON" \
  --argjson dependency "$DEPENDENCY_ISOLATION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $governance.runtime == "hepta"
    and $governance.status == "ready"
    and $governance.gate == "hepta_terminal_governance_closure_summary_gate"
    and $governance.terminal_governance_closure_summary_ready == true
    and $governance.governance_closure_mode == "schema_only_terminal_governance_activation_blocked"
    and $governance.governance_closure_decision == "terminal_governance_closed_without_activation_release_claim_or_operator_execution"
    and $governance.required_source_count == 3
    and $governance.ready_source_count == 3
    and $governance.activation_blocking_source_count == 3
    and $governance.governance_closure_denied_by_count == 65
    and ($governance.governance_closure_families | length) == 7
    and ($governance.governance_closure_families | all(.ready == true and .blocked == true))
    and $governance.activation_allowed == false
    and $governance.active_wiring_allowed == false
    and $governance.live_mutation_execution_ready == false
    and $governance.public_release_claim_allowed == false
    and $governance.release_artifact_write_allowed == false
    and ($governance.side_effects | to_entries | all(.value == false))
    and $watchdog.runtime == "hepta"
    and $watchdog.status == "ok"
    and $watchdog.binary_sha_match == true
    and $watchdog.health == "ready"
    and $watchdog.route_count >= 69
    and $watchdog.missing_route_count == 0
    and $watchdog.full_fusion_complete == true
    and $watchdog.phase_4_name_repository_closure_remaining_surface_count == 0
    and $watchdog.phase_5_engine_dependency_closure_remaining_dependency_count == 0
    and $watchdog.release_sha256 == $watchdog.installed_sha256
    and ($watchdog.side_effects | to_entries | all(.value == false))
    and $dependency.runtime == "hepta"
    and $dependency.status == "ready"
    and $dependency.gate == "hepta_active_service_dependency_isolation_gate"
    and $dependency.active_binary_package == "hepta-cli"
    and $dependency.active_binary_target == "hepta"
    and $dependency.local_cargo_tree_isolated == true
    and ($dependency.found_forbidden_codex_engine_crates | length) == 0
    and $dependency.live_check_status == "skipped"
    and ($dependency.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_terminal_governance_active_state_lock_gate" \
  --arg governance_closure_report_sha256 "$governance_closure_report_sha256" \
  --arg watchdog_report_sha256 "$watchdog_report_sha256" \
  --arg dependency_isolation_report_sha256 "$dependency_isolation_report_sha256" \
  --arg active_state_lock_hash_sha256 "$active_state_lock_hash_sha256" \
  --arg active_state_lock_policy_hash_sha256 "$active_state_lock_policy_hash_sha256" \
  --arg active_state_lock_side_effect_hash_sha256 "$active_state_lock_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson governance "$GOVERNANCE_CLOSURE_JSON" \
  --argjson watchdog "$WATCHDOG_JSON" \
  --argjson dependency "$DEPENDENCY_ISOLATION_JSON" \
  '
    ($governance.denied_by_governance_closure_summary) as $governance_denied
    | ([
        "terminal_governance_active_state_lock_recording_denied",
        "terminal_governance_active_state_lock_persistence_denied",
        "terminal_governance_active_state_lock_materialization_denied",
        "terminal_governance_active_state_lock_filesystem_write_denied",
        "install_execution_denied_by_active_state_lock",
        "active_service_restart_denied_by_active_state_lock",
        "active_dependency_mutation_denied_by_active_state_lock",
        "live_dependency_check_not_executed_by_active_state_lock"
      ] + $governance_denied) as $active_state_denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_governance_active_state_lock_schema_version:"terminal_governance_active_state_lock_v1",
      minimum_required_samples:$min_long_soak_samples,
      active_state_lock_ready:true,
      active_state_lock_mode:"schema_only_active_state_locked_no_runtime_mutation",
      active_state_lock_decision:"active_service_state_observed_without_install_restart_or_dependency_mutation",
      source_governance_closure_gate:$governance.gate,
      source_watchdog_gate:"hepta_watchdog",
      source_dependency_isolation_gate:$dependency.gate,
      source_governance_closure_report_sha256:$governance_closure_report_sha256,
      source_watchdog_report_sha256:$watchdog_report_sha256,
      source_dependency_isolation_report_sha256:$dependency_isolation_report_sha256,
      required_source_count:3,
      ready_source_count:3,
      activation_blocking_source_count:3,
      source_governance_closure_ready:$governance.terminal_governance_closure_summary_ready,
      source_governance_closure_denied_by_count:$governance.governance_closure_denied_by_count,
      source_governance_closure_family_count:($governance.governance_closure_families | length),
      source_watchdog_status:$watchdog.status,
      source_watchdog_binary_sha_match:$watchdog.binary_sha_match,
      source_watchdog_health:$watchdog.health,
      source_watchdog_route_count:$watchdog.route_count,
      source_watchdog_missing_route_count:$watchdog.missing_route_count,
      source_watchdog_full_fusion_complete:$watchdog.full_fusion_complete,
      source_watchdog_release_sha256:$watchdog.release_sha256,
      source_watchdog_installed_sha256:$watchdog.installed_sha256,
      source_watchdog_phase_4_remaining_surface_count:$watchdog.phase_4_name_repository_closure_remaining_surface_count,
      source_watchdog_phase_5_remaining_dependency_count:$watchdog.phase_5_engine_dependency_closure_remaining_dependency_count,
      source_dependency_isolation_ready:$dependency.local_cargo_tree_isolated,
      source_dependency_isolation_active_binary_package:$dependency.active_binary_package,
      source_dependency_isolation_active_binary_target:$dependency.active_binary_target,
      source_dependency_isolation_forbidden_crate_count:($dependency.found_forbidden_codex_engine_crates | length),
      source_dependency_isolation_live_check_status:$dependency.live_check_status,
      active_state_observed:true,
      active_binary_sha_consistent:($watchdog.release_sha256 == $watchdog.installed_sha256),
      active_service_state_locked:true,
      active_dependency_isolated:$dependency.local_cargo_tree_isolated,
      readiness_allowed:false,
      activation_allowed:false,
      active_wiring_allowed:false,
      active_runtime_auto_rebase_allowed:false,
      active_runtime_codex_engine_dependency_allowed:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      install_execution_allowed:false,
      release_build_required:false,
      active_binary_mutation_allowed:false,
      active_service_restart_allowed:false,
      launchd_restart_allowed:false,
      rollback_execution_allowed:false,
      rollback_restore_allowed:false,
      live_dependency_check_executed:false,
      post_lock_soak_executed:false,
      live_mutation_execution_ready:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      release_artifact_write_allowed:false,
      public_artifact_write_allowed:false,
      terminal_governance_active_state_lock_recorded:false,
      terminal_governance_active_state_lock_persisted:false,
      terminal_governance_active_state_lock_materialized:false,
      terminal_governance_active_state_lock_filesystem_written:false,
      active_state_lock_hash_sha256:$active_state_lock_hash_sha256,
      active_state_lock_policy_hash_sha256:$active_state_lock_policy_hash_sha256,
      active_state_lock_side_effect_hash_sha256:$active_state_lock_side_effect_hash_sha256,
      active_state_lock_source_hashes:[
        $governance_closure_report_sha256,
        $watchdog_report_sha256,
        $dependency_isolation_report_sha256
      ],
      active_state_lock_families:[
        {
          id:"terminal-governance-closure-source-lock",
          ready:true,
          blocked:true,
          denied_by_count:$governance.governance_closure_denied_by_count,
          reason:"terminal governance closure remains activation-blocking and report-only"
        },
        {
          id:"watchdog-active-binary-integrity-lock",
          ready:true,
          blocked:true,
          release_installed_sha_match:($watchdog.release_sha256 == $watchdog.installed_sha256),
          route_count:$watchdog.route_count,
          full_fusion_complete:$watchdog.full_fusion_complete,
          reason:"watchdog observes active binary and route health without authorizing mutation"
        },
        {
          id:"active-dependency-isolation-lock",
          ready:true,
          blocked:true,
          forbidden_codex_engine_crate_count:($dependency.found_forbidden_codex_engine_crates | length),
          local_cargo_tree_isolated:$dependency.local_cargo_tree_isolated,
          reason:"active service dependency isolation remains green without live dependency mutation"
        },
        {
          id:"install-restart-execution-boundary",
          ready:true,
          blocked:true,
          install_execution_allowed:false,
          active_service_restart_allowed:false,
          launchd_restart_allowed:false,
          reason:"install, service restart, and launchd mutation remain denied"
        },
        {
          id:"active-state-lock-persistence-boundary",
          ready:true,
          blocked:true,
          terminal_governance_active_state_lock_recorded:false,
          terminal_governance_active_state_lock_persisted:false,
          terminal_governance_active_state_lock_materialized:false,
          terminal_governance_active_state_lock_filesystem_written:false,
          reason:"active-state lock is report-only and not persisted or materialized"
        },
        {
          id:"activation-public-claim-live-mutation-boundary",
          ready:true,
          blocked:true,
          activation_allowed:false,
          public_release_claim_allowed:false,
          release_artifact_write_allowed:false,
          live_mutation_execution_ready:false,
          reason:"activation, public claims, artifact writes, rollback execution, and live mutation remain denied"
        }
      ],
      active_state_lock_denied_by_count:($active_state_denied | length),
      denied_by_active_state_lock:$active_state_denied,
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
        active_binary_mutated:false,
        active_service_restart:false,
        launchd_mutated:false,
        rollback_executed:false,
        rollback_restore_executed:false,
        post_lock_soak_executed:false,
        live_dependency_check_executed:false,
        terminal_governance_active_state_lock_recorded:false,
        terminal_governance_active_state_lock_persisted:false,
        terminal_governance_active_state_lock_materialized:false,
        terminal_governance_active_state_lock_filesystem_written:false,
        terminal_governance_closure_summary_recorded:false,
        terminal_governance_closure_summary_persisted:false,
        terminal_governance_closure_summary_materialized:false,
        terminal_governance_closure_summary_filesystem_written:false,
        filesystem_written:false,
        workspace_write_performed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        public_release_published:false,
        public_ga_claimed:false,
        external_public_claim_performed:false,
        external_send_performed:false,
        credential_read:false,
        secret_file_read:false
      }
    }')"

jq -e '
  .status == "ready"
  and .active_state_lock_ready == true
  and .active_state_lock_mode == "schema_only_active_state_locked_no_runtime_mutation"
  and .active_state_lock_decision == "active_service_state_observed_without_install_restart_or_dependency_mutation"
  and .required_source_count == 3
  and .ready_source_count == 3
  and .activation_blocking_source_count == 3
  and .source_governance_closure_ready == true
  and .source_governance_closure_denied_by_count == 65
  and .source_governance_closure_family_count == 7
  and .source_watchdog_status == "ok"
  and .source_watchdog_binary_sha_match == true
  and .source_watchdog_health == "ready"
  and .source_watchdog_route_count >= 69
  and .source_watchdog_missing_route_count == 0
  and .source_watchdog_full_fusion_complete == true
  and .source_watchdog_release_sha256 == .source_watchdog_installed_sha256
  and .source_watchdog_phase_4_remaining_surface_count == 0
  and .source_watchdog_phase_5_remaining_dependency_count == 0
  and .source_dependency_isolation_ready == true
  and .source_dependency_isolation_active_binary_package == "hepta-cli"
  and .source_dependency_isolation_active_binary_target == "hepta"
  and .source_dependency_isolation_forbidden_crate_count == 0
  and .source_dependency_isolation_live_check_status == "skipped"
  and .active_state_observed == true
  and .active_binary_sha_consistent == true
  and .active_service_state_locked == true
  and .active_dependency_isolated == true
  and .readiness_allowed == false
  and .activation_allowed == false
  and .active_wiring_allowed == false
  and .active_runtime_auto_rebase_allowed == false
  and .active_runtime_codex_engine_dependency_allowed == false
  and .upstream_fetch_allowed == false
  and .upstream_merge_allowed == false
  and .install_execution_allowed == false
  and .release_build_required == false
  and .active_binary_mutation_allowed == false
  and .active_service_restart_allowed == false
  and .launchd_restart_allowed == false
  and .rollback_execution_allowed == false
  and .rollback_restore_allowed == false
  and .live_dependency_check_executed == false
  and .post_lock_soak_executed == false
  and .live_mutation_execution_ready == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .public_artifact_write_allowed == false
  and .terminal_governance_active_state_lock_recorded == false
  and .terminal_governance_active_state_lock_persisted == false
  and .terminal_governance_active_state_lock_materialized == false
  and .terminal_governance_active_state_lock_filesystem_written == false
  and (.active_state_lock_source_hashes | length) == 3
  and (.active_state_lock_families | length) == 6
  and (.active_state_lock_families | all(.ready == true and .blocked == true))
  and .active_state_lock_denied_by_count == 73
  and (.denied_by_active_state_lock | length) == .active_state_lock_denied_by_count
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta terminal governance active-state lock gate passed"
