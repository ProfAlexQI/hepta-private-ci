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

require_unsigned_integer() {
  local name="$1"
  local value="$2"

  case "$value" in
    ''|*[!0-9]*)
      echo "$name must be an unsigned integer" >&2
      exit 2
      ;;
  esac
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

FINAL_AUDIT_JSON="$(
  capture_json_report \
    "hepta-terminal-release-governance-final-audit-index-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-release-governance-final-audit-index-gate.sh
)"

RELEASE_PUBLICATION_JSON="$(
  capture_json_report \
    "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-gate.sh
)"

DEPENDENCY_ISOLATION_JSON="$(
  capture_json_report \
    "hepta-active-service-dependency-isolation" \
    env HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=0 \
      scripts/hepta-active-service-dependency-isolation.sh
)"

WATCHDOG_SOAK_JSON="$(
  capture_json_report \
    "hepta-terminal-watchdog-soak-regression-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-watchdog-soak-regression-gate.sh
)"

MEMORY_INTELLIGENCE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-closure" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-memory-intelligence-closure.sh
)"

final_audit_report_sha256="$(sha256_text "$FINAL_AUDIT_JSON")"
release_publication_report_sha256="$(sha256_text "$RELEASE_PUBLICATION_JSON")"
dependency_isolation_report_sha256="$(sha256_text "$DEPENDENCY_ISOLATION_JSON")"
watchdog_soak_report_sha256="$(sha256_text "$WATCHDOG_SOAK_JSON")"
memory_intelligence_report_sha256="$(sha256_text "$MEMORY_INTELLIGENCE_JSON")"
summary_index_hash_sha256="$(sha256_text "hepta-core-activation-readiness-summary:index:$final_audit_report_sha256:$release_publication_report_sha256:$dependency_isolation_report_sha256:$watchdog_soak_report_sha256:$memory_intelligence_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_policy_hash_sha256="$(sha256_text "hepta-core-activation-readiness-summary:policy:$final_audit_report_sha256:$release_publication_report_sha256:$dependency_isolation_report_sha256:$watchdog_soak_report_sha256:$memory_intelligence_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-readiness-summary:side-effects:$final_audit_report_sha256:$release_publication_report_sha256:$dependency_isolation_report_sha256:$watchdog_soak_report_sha256:$memory_intelligence_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson final "$FINAL_AUDIT_JSON" \
  --argjson release "$RELEASE_PUBLICATION_JSON" \
  --argjson dependency "$DEPENDENCY_ISOLATION_JSON" \
  --argjson watchdog "$WATCHDOG_SOAK_JSON" \
  --argjson memory "$MEMORY_INTELLIGENCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $final.runtime == "hepta"
    and $final.status == "ready"
    and $final.gate == "hepta_terminal_release_governance_final_audit_index_gate"
    and $final.final_audit_index_ready == true
    and $final.final_release_governance_audit_ready == true
    and $final.full_fusion_operational_evidence_observed == true
    and $final.active_binary_sha_consistent == true
    and $final.active_dependency_isolated == true
    and $final.memory_intelligence_absorbed_or_represented == true
    and $final.publication_evidence_non_persistence_enforced == true
    and $final.public_claim_denial_enforced == true
    and $final.public_distribution_denial_enforced == true
    and $final.operator_approval_non_recording_enforced == true
    and $final.active_runtime_mutation_denial_enforced == true
    and $final.public_release_claim_allowed == false
    and $final.public_ga_claim_allowed == false
    and $final.public_distribution_publication_allowed == false
    and $final.release_artifact_write_allowed == false
    and $final.memory_store_mutation_allowed == false
    and $final.provider_model_invocation_allowed == false
    and $final.channel_delivery_allowed == false
    and $final.live_mutation_execution_ready == false
    and $final.final_audit_denied_by_count == 127
    and ($final.final_audit_families | length) == 7
    and ($final.final_audit_families | all(.ready == true and .blocked == true))
    and ($final.side_effects | to_entries | all(.value == false))
    and $release.runtime == "hepta"
    and $release.status == "ready"
    and $release.gate == "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_release_artifact_publication_denial_gate"
    and $release.memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_ready == true
    and $release.required_activation_command_result_receipt_release_artifact_publication_surface_count == 12
    and $release.ready_activation_command_result_receipt_release_artifact_publication_surface_count == 12
    and $release.side_effect_free_activation_command_result_receipt_release_artifact_publication_surface_count == 12
    and $release.required_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
    and $release.activation_command_result_receipt_release_artifact_publication_fixture_count == 10
    and $release.blocked_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
    and $release.allowed_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
    and $release.release_artifact_publication_allowed == false
    and $release.release_artifact_written == false
    and $release.public_artifact_written == false
    and $release.public_distribution_performed == false
    and $release.public_release_published == false
    and $release.public_ga_claimed == false
    and $release.public_claim_promoted == false
    and $release.telegram_send_performed == false
    and $release.channel_send_performed == false
    and $release.external_send_performed == false
    and $release.activation_allowed == false
    and $release.live_mutation_execution_ready == false
    and $release.live_mutation_execution_performed == false
    and $release.memory_store_mutated == false
    and $release.provider_invoked == false
    and $release.model_invoked == false
    and $release.install_executed == false
    and $release.service_restarted == false
    and $release.active_binary_mutated == false
    and ($release.side_effects | to_entries | all(.value == false))
    and $dependency.runtime == "hepta"
    and $dependency.status == "ready"
    and $dependency.gate == "hepta_active_service_dependency_isolation_gate"
    and $dependency.local_cargo_tree_isolated == true
    and ($dependency.found_forbidden_codex_engine_crates | length) == 0
    and ($dependency.side_effects | to_entries | all(.value == false))
    and $watchdog.runtime == "hepta"
    and $watchdog.status == "ready"
    and $watchdog.gate == "hepta_terminal_watchdog_soak_regression_gate"
    and $watchdog.watchdog_soak_regression_ready == true
    and $watchdog.watchdog_status_known == true
    and ($watchdog.watchdog_status == "ok" or $watchdog.watchdog_known_operator_security_attention == true)
    and $watchdog.watchdog_health == "ready"
    and $watchdog.watchdog_route_count >= 69
    and $watchdog.watchdog_missing_route_count == 0
    and $watchdog.watchdog_binary_sha_match == true
    and $watchdog.watchdog_full_fusion_complete == true
    and $watchdog.soak_status_known == true
    and ($watchdog.soak_status == "ready" or $watchdog.soak_known_operator_security_attention == true)
    and $watchdog.soak_samples >= 3
    and (($watchdog.soak_status == "ready" and $watchdog.soak_ok == $watchdog.soak_samples and $watchdog.soak_fail == 0)
      or ($watchdog.soak_known_operator_security_attention == true and $watchdog.soak_ok == 0 and $watchdog.soak_fail == $watchdog.soak_samples))
    and $watchdog.minimum_long_soak_required_samples >= 24
    and $watchdog.terminal_soak_authorizes_live_mutation == false
    and $watchdog.public_release_claim_allowed == false
    and $watchdog.public_distribution_publication_allowed == false
    and $watchdog.release_artifact_write_allowed == false
    and ($watchdog.side_effects | to_entries | all(.value == false))
    and $memory.runtime == "hepta"
    and $memory.status == "attention"
    and $memory.compatibility_mode == "hepta_memory_intelligence_closure_gate"
    and $memory.active_service_stack_consumes_memory_intelligence == true
    and $memory.hepta_core_direct_memory_intelligence_dependency_count == 0
    and $memory.hepta_core_dependency_boundary_ready == true
    and $memory.runtime_memory_intelligence_dependencies_ready == true
    and $memory.memory_surface_count == 14
    and $memory.absorbed_or_represented_count == 14
    and $memory.gap_report_ready_count == 14
    and $memory.live_mutation_enabled_count == 0
    and $memory.gap_only_surface_count == 0
    and ($memory.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_readiness_summary_gate" \
  --arg final_audit_report_sha256 "$final_audit_report_sha256" \
  --arg release_publication_report_sha256 "$release_publication_report_sha256" \
  --arg dependency_isolation_report_sha256 "$dependency_isolation_report_sha256" \
  --arg watchdog_soak_report_sha256 "$watchdog_soak_report_sha256" \
  --arg memory_intelligence_report_sha256 "$memory_intelligence_report_sha256" \
  --arg summary_index_hash_sha256 "$summary_index_hash_sha256" \
  --arg summary_policy_hash_sha256 "$summary_policy_hash_sha256" \
  --arg summary_side_effect_hash_sha256 "$summary_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson final "$FINAL_AUDIT_JSON" \
  --argjson release "$RELEASE_PUBLICATION_JSON" \
  --argjson dependency "$DEPENDENCY_ISOLATION_JSON" \
  --argjson watchdog "$WATCHDOG_SOAK_JSON" \
  --argjson memory "$MEMORY_INTELLIGENCE_JSON" \
  '
    ([
      "explicit_operator_approval_missing",
      "activation_live_mutation_execution_denied",
      "memory_store_mutation_denied",
      "release_artifact_write_denied",
      "public_artifact_write_denied",
      "public_distribution_publication_denied",
      "public_release_claim_denied",
      "public_ga_claim_denied",
      "terminal_operator_decision_not_release_approval",
      "short_soak_not_release_long_soak_evidence",
      "evidence_persistence_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "install_restart_active_binary_mutation_denied",
      "upstream_fetch_merge_denied",
      "credential_secret_read_denied"
    ]) as $denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      core_activation_readiness_summary_schema_version:"hepta_core_activation_readiness_summary_v1",
      core_activation_readiness_summary_ready:true,
      readiness_summary_mode:"aggregate_read_only_no_activation_no_publication_no_memory_mutation",
      verdict:"blocked_until_explicit_operator_approval_and_fresh_live_evidence",
      public_release_verdict:"blocked",
      live_memory_mutation_verdict:"blocked",
      release_artifact_publication_verdict:"blocked",
      active_runtime_mutation_verdict:"blocked",
      active_service_health_verdict:"observed_ready_not_authorizing_activation",
      minimum_required_long_soak_samples:$min_long_soak_samples,
      source_final_audit_gate:$final.gate,
      source_release_publication_gate:$release.gate,
      source_dependency_isolation_gate:$dependency.gate,
      source_watchdog_soak_gate:$watchdog.gate,
      source_memory_intelligence_gate:$memory.compatibility_mode,
      source_final_audit_report_sha256:$final_audit_report_sha256,
      source_release_publication_report_sha256:$release_publication_report_sha256,
      source_dependency_isolation_report_sha256:$dependency_isolation_report_sha256,
      source_watchdog_soak_report_sha256:$watchdog_soak_report_sha256,
      source_memory_intelligence_report_sha256:$memory_intelligence_report_sha256,
      required_source_count:5,
      ready_source_count:5,
      activation_blocking_source_count:5,
      source_report_hashes:[
        $final_audit_report_sha256,
        $release_publication_report_sha256,
        $dependency_isolation_report_sha256,
        $watchdog_soak_report_sha256,
        $memory_intelligence_report_sha256
      ],
      summary_index_hash_sha256:$summary_index_hash_sha256,
      summary_policy_hash_sha256:$summary_policy_hash_sha256,
      summary_side_effect_hash_sha256:$summary_side_effect_hash_sha256,
      final_audit_ready:$final.final_audit_index_ready,
      final_release_governance_audit_ready:$final.final_release_governance_audit_ready,
      final_audit_denied_by_count:$final.final_audit_denied_by_count,
      release_publication_denial_ready:$release.memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_ready,
      release_publication_surface_count:$release.required_activation_command_result_receipt_release_artifact_publication_surface_count,
      release_publication_fixture_count:$release.activation_command_result_receipt_release_artifact_publication_fixture_count,
      release_publication_blocked_fixture_count:$release.blocked_activation_command_result_receipt_release_artifact_publication_fixture_count,
      release_publication_allowed_count:$release.allowed_activation_command_result_receipt_release_artifact_publication_fixture_count,
      active_dependency_isolated:$dependency.local_cargo_tree_isolated,
      active_binary_package:$dependency.active_binary_package,
      active_binary_target:$dependency.active_binary_target,
      forbidden_codex_engine_crate_count:($dependency.found_forbidden_codex_engine_crates | length),
      tracked_forbidden_codex_engine_crates:$dependency.tracked_forbidden_codex_engine_crates,
      found_forbidden_codex_engine_crates:$dependency.found_forbidden_codex_engine_crates,
      dependency_isolation_live_check_status:$dependency.live_check_status,
      watchdog_route_count:$watchdog.watchdog_route_count,
      watchdog_status:$watchdog.watchdog_status,
      watchdog_status_known:$watchdog.watchdog_status_known,
      watchdog_known_operator_security_attention:$watchdog.watchdog_known_operator_security_attention,
      watchdog_missing_route_count:$watchdog.watchdog_missing_route_count,
      watchdog_binary_sha_match:$watchdog.watchdog_binary_sha_match,
      watchdog_full_fusion_complete:$watchdog.watchdog_full_fusion_complete,
      watchdog_release_sha256:$watchdog.watchdog_release_sha256,
      watchdog_installed_sha256:$watchdog.watchdog_installed_sha256,
      short_soak_status:$watchdog.soak_status,
      short_soak_status_known:$watchdog.soak_status_known,
      short_soak_known_operator_security_attention:$watchdog.soak_known_operator_security_attention,
      short_soak_passed:$watchdog.soak_passed,
      short_soak_samples:$watchdog.soak_samples,
      short_soak_ok:$watchdog.soak_ok,
      short_soak_fail:$watchdog.soak_fail,
      short_soak_is_release_long_soak:$watchdog.terminal_soak_is_release_long_soak,
      short_soak_authorizes_live_mutation:$watchdog.terminal_soak_authorizes_live_mutation,
      hepta_core_direct_memory_intelligence_dependency_count:$memory.hepta_core_direct_memory_intelligence_dependency_count,
      memory_intelligence_consumed_by_active_stack:$memory.active_service_stack_consumes_memory_intelligence,
      memory_intelligence_core_boundary_ready:$memory.hepta_core_dependency_boundary_ready,
      runtime_memory_intelligence_dependencies_ready:$memory.runtime_memory_intelligence_dependencies_ready,
      memory_intelligence_surface_count:$memory.memory_surface_count,
      memory_intelligence_absorbed_or_represented_count:$memory.absorbed_or_represented_count,
      memory_intelligence_gap_report_ready_count:$memory.gap_report_ready_count,
      memory_intelligence_gap_only_surface_count:$memory.gap_only_surface_count,
      memory_intelligence_live_mutation_enabled_count:$memory.live_mutation_enabled_count,
      operator_approval_recorded:false,
      activation_allowed:false,
      active_wiring_allowed:false,
      active_runtime_auto_rebase_allowed:false,
      live_mutation_execution_ready:false,
      live_mutation_execution_allowed:false,
      memory_store_mutation_allowed:false,
      capability_registry_mutation_allowed:false,
      plugin_registry_mutation_allowed:false,
      skill_workshop_write_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      release_artifact_write_allowed:false,
      public_artifact_write_allowed:false,
      public_distribution_publication_allowed:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      public_release_published:false,
      public_ga_claimed:false,
      release_artifact_publication_allowed:false,
      release_artifact_publication_accepted:false,
      release_artifact_publication_recorded:false,
      release_artifact_publication_persisted:false,
      evidence_persistence_allowed:false,
      final_audit_index_persisted:false,
      publication_evidence_summary_persisted:false,
      watchdog_report_persisted:false,
      soak_report_persisted:false,
      install_execution_allowed:false,
      active_service_restart_allowed:false,
      launchd_restart_allowed:false,
      active_binary_mutation_allowed:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      credential_read_allowed:false,
      secret_file_read_allowed:false,
      readiness_families:[
        {
          id:"memory-intelligence",
          ready:true,
          blocked:true,
          status:$memory.status,
          surface_count:$memory.memory_surface_count,
          absorbed_or_represented_count:$memory.absorbed_or_represented_count,
          live_mutation_enabled_count:$memory.live_mutation_enabled_count,
          reason:"memory and intelligence surfaces are represented, but live mutation remains disabled"
        },
        {
          id:"release-publication",
          ready:true,
          blocked:true,
          fixture_count:$release.activation_command_result_receipt_release_artifact_publication_fixture_count,
          allowed_count:$release.allowed_activation_command_result_receipt_release_artifact_publication_fixture_count,
          reason:"release artifacts, public artifacts, public release, public GA, and distribution remain denied"
        },
        {
          id:"final-release-governance",
          ready:true,
          blocked:true,
          denied_by_count:$final.final_audit_denied_by_count,
          reason:"terminal release-governance final audit is ready and still blocks activation"
        },
        {
          id:"active-dependency-isolation",
          ready:true,
          blocked:true,
          forbidden_codex_engine_crate_count:($dependency.found_forbidden_codex_engine_crates | length),
          reason:"active hepta-cli stays isolated from tracked Codex engine crates"
        },
        {
          id:"watchdog-short-soak",
          ready:true,
          blocked:true,
          watchdog_status:$watchdog.watchdog_status,
          watchdog_known_operator_security_attention:$watchdog.watchdog_known_operator_security_attention,
          route_count:$watchdog.watchdog_route_count,
          short_soak_status:$watchdog.soak_status,
          short_soak_samples:$watchdog.soak_samples,
          short_soak_ok:$watchdog.soak_ok,
          short_soak_fail:$watchdog.soak_fail,
          short_soak_known_operator_security_attention:$watchdog.soak_known_operator_security_attention,
          reason:"watchdog and short soak are healthy or classified as known operator-security attention, and do not authorize activation"
        },
        {
          id:"operator-approval-boundary",
          ready:true,
          blocked:true,
          operator_approval_recorded:false,
          reason:"no explicit operator approval is recorded or accepted by this summary"
        }
      ],
      denied_by_core_activation_readiness_summary:$denied,
      core_activation_denied_by_count:($denied | length),
      side_effects:{
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        skill_workshop_written:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        telegram_send_performed:false,
        external_send_performed:false,
        runtime_store_mutated:false,
        gateway_event_enqueued:false,
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        active_runtime_auto_rebase_performed:false,
        active_runtime_dependency_mutated:false,
        install_executed:false,
        release_build_executed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        release_artifact_publication_recorded:false,
        release_artifact_publication_persisted:false,
        release_artifact_publication_materialized:false,
        public_distribution_performed:false,
        public_release_published:false,
        public_ga_claimed:false,
        external_public_claim_performed:false,
        external_public_distribution_performed:false,
        active_binary_mutated:false,
        active_service_restart:false,
        launchd_mutated:false,
        operator_approval_recorded:false,
        final_audit_index_persisted:false,
        publication_evidence_summary_persisted:false,
        watchdog_report_persisted:false,
        soak_report_persisted:false,
        summary_index_persisted:false,
        summary_index_materialized:false,
        summary_index_filesystem_written:false,
        filesystem_written:false,
        workspace_write_performed:false,
        credential_read:false,
        secret_file_read:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .core_activation_readiness_summary_ready == true
  and .verdict == "blocked_until_explicit_operator_approval_and_fresh_live_evidence"
  and .required_source_count == 5
  and .ready_source_count == 5
  and .activation_blocking_source_count == 5
  and .final_audit_ready == true
  and .final_release_governance_audit_ready == true
  and .final_audit_denied_by_count == 127
  and .release_publication_denial_ready == true
  and .release_publication_surface_count == 12
  and .release_publication_fixture_count == 10
  and .release_publication_blocked_fixture_count == 10
  and .release_publication_allowed_count == 0
  and .active_dependency_isolated == true
  and .forbidden_codex_engine_crate_count == 0
  and .watchdog_status_known == true
  and (.watchdog_status == "ok" or .watchdog_known_operator_security_attention == true)
  and .watchdog_route_count >= 69
  and .watchdog_missing_route_count == 0
  and .watchdog_binary_sha_match == true
  and .watchdog_full_fusion_complete == true
  and .short_soak_status_known == true
  and (.short_soak_status == "ready" or .short_soak_known_operator_security_attention == true)
  and .short_soak_samples >= 3
  and ((.short_soak_status == "ready" and .short_soak_ok == .short_soak_samples and .short_soak_fail == 0)
    or (.short_soak_known_operator_security_attention == true and .short_soak_ok == 0 and .short_soak_fail == .short_soak_samples))
  and .short_soak_authorizes_live_mutation == false
  and .hepta_core_direct_memory_intelligence_dependency_count == 0
  and .memory_intelligence_consumed_by_active_stack == true
  and .memory_intelligence_core_boundary_ready == true
  and .runtime_memory_intelligence_dependencies_ready == true
  and .memory_intelligence_surface_count == 14
  and .memory_intelligence_absorbed_or_represented_count == 14
  and .memory_intelligence_gap_only_surface_count == 0
  and .memory_intelligence_live_mutation_enabled_count == 0
  and .operator_approval_recorded == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .memory_store_mutation_allowed == false
  and .release_artifact_write_allowed == false
  and .public_artifact_write_allowed == false
  and .public_distribution_publication_allowed == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .install_execution_allowed == false
  and .active_service_restart_allowed == false
  and .active_binary_mutation_allowed == false
  and .evidence_persistence_allowed == false
  and (.readiness_families | length) == 6
  and (.readiness_families | all(.ready == true and .blocked == true))
  and .core_activation_denied_by_count == 16
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta core activation readiness summary gate passed"
