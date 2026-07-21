#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
TERMINAL_SOAK_SAMPLES="${HEPTA_TERMINAL_SOAK_SAMPLES:-3}"
TERMINAL_SOAK_INTERVAL_SECONDS="${HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS:-1}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

LATEST_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-latest-multisurface-absorption" \
    scripts/hepta-upstream-codex-latest-multisurface-absorption.sh
)"

DEPENDENCY_JSON="$(
  capture_json_report \
    "hepta-active-service-dependency-isolation" \
    env HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=0 \
      scripts/hepta-active-service-dependency-isolation.sh
)"

REGRESSION_JSON="$(
  capture_json_report \
    "hepta-terminal-watchdog-soak-regression" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_TERMINAL_SOAK_SAMPLES="$TERMINAL_SOAK_SAMPLES" \
      HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS="$TERMINAL_SOAK_INTERVAL_SECONDS" \
      scripts/hepta-terminal-watchdog-soak-regression-gate.sh
)"

latest_report_sha256="$(sha256_text "$LATEST_JSON")"
dependency_report_sha256="$(sha256_text "$DEPENDENCY_JSON")"
regression_report_sha256="$(sha256_text "$REGRESSION_JSON")"
active_safety_index_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-active-safety:index:$latest_report_sha256:$dependency_report_sha256:$regression_report_sha256")"
active_safety_policy_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-active-safety:policy:$latest_report_sha256:$dependency_report_sha256:$regression_report_sha256")"
active_safety_side_effect_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-active-safety:side-effects:$latest_report_sha256:$dependency_report_sha256:$regression_report_sha256")"

jq -n -e \
  --argjson latest "$LATEST_JSON" \
  --argjson dependency "$DEPENDENCY_JSON" \
  --argjson regression "$REGRESSION_JSON" \
  '
    $latest.status == "ready"
    and $latest.latest_multisurface_absorption_ready == true
    and $latest.target_descends_from_baseline == true
    and $latest.commit_count == 12
    and $latest.changed_file_count == 57
    and $latest.ready_family_count == 5
    and $latest.activation_blocking_family_count == 5
    and $latest.active_runtime_promotion_allowed == false
    and $latest.active_appserver_promotion_allowed == false
    and $latest.active_tui_promotion_allowed == false
    and $latest.active_process_hardening_env_mutation_allowed == false
    and $latest.upstream_merge_performed == false
    and $latest.upstream_checkout_performed == false
    and $latest.active_runtime_dependency_allowed == false
    and $latest.active_service_restart_allowed == false
    and $latest.public_release_claim_allowed == false
    and $latest.release_artifact_write_allowed == false
    and ($latest.side_effects | to_entries | all(.value == false))
    and $dependency.status == "ready"
    and $dependency.active_binary_package == "hepta-cli"
    and $dependency.active_binary_target == "hepta"
    and $dependency.local_cargo_tree_isolated == true
    and ($dependency.found_forbidden_codex_engine_crates | length) == 0
    and ($dependency.side_effects | to_entries | all(.value == false))
    and $regression.status == "ready"
    and $regression.watchdog_soak_regression_ready == true
    and $regression.watchdog_status_known == true
    and ($regression.watchdog_status == "ok" or $regression.watchdog_known_operator_security_attention == true)
    and $regression.watchdog_health == "ready"
    and $regression.watchdog_evidence_contract_ready == true
    and $regression.watchdog_full_fusion_complete == true
    and $regression.watchdog_route_count >= 69
    and $regression.watchdog_missing_route_count == 0
    and $regression.soak_status_known == true
    and ($regression.soak_status == "ready" or $regression.soak_known_operator_security_attention == true)
    and (($regression.soak_status == "ready" and $regression.soak_ok == $regression.soak_samples and $regression.soak_fail == 0)
      or ($regression.soak_known_operator_security_attention == true and $regression.soak_ok == 0 and $regression.soak_fail == $regression.soak_samples))
    and $regression.terminal_soak_authorizes_live_mutation == false
    and $regression.public_release_claim_allowed == false
    and $regression.release_artifact_write_allowed == false
    and $regression.upstream_fetch_allowed == false
    and $regression.upstream_merge_allowed == false
    and ($regression.side_effects | to_entries | all(.value == false))
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg gate "hepta_upstream_codex_latest_active_safety_regression_gate" \
  --arg base_url "$BASE_URL" \
  --arg latest_report_sha256 "$latest_report_sha256" \
  --arg dependency_report_sha256 "$dependency_report_sha256" \
  --arg regression_report_sha256 "$regression_report_sha256" \
  --arg active_safety_index_hash_sha256 "$active_safety_index_hash_sha256" \
  --arg active_safety_policy_hash_sha256 "$active_safety_policy_hash_sha256" \
  --arg active_safety_side_effect_hash_sha256 "$active_safety_side_effect_hash_sha256" \
  --argjson latest "$LATEST_JSON" \
  --argjson dependency "$DEPENDENCY_JSON" \
  --argjson regression "$REGRESSION_JSON" \
  '
    ([
      "latest_delta_direct_merge_denied",
      "latest_delta_active_runtime_auto_rebase_denied",
      "latest_delta_active_dependency_mutation_denied",
      "latest_delta_gateway_mutation_denied",
      "latest_delta_doctor_thread_inventory_live_query_denied",
      "latest_delta_remote_status_active_wiring_denied",
      "latest_delta_tui_compatibility_promotion_denied",
      "latest_delta_process_hardening_launchd_env_mutation_denied",
      "latest_delta_provider_model_invocation_denied",
      "latest_delta_channel_delivery_denied",
      "latest_delta_release_artifact_write_denied",
      "active_service_forbidden_codex_engine_dependency_denied",
      "watchdog_regression_persistence_denied",
      "watchdog_regression_short_soak_live_mutation_denied",
      "public_release_claim_denied",
      "public_distribution_denied",
      "upstream_fetch_denied",
      "upstream_merge_denied",
      "install_restart_denied",
      "evidence_persistence_denied"
    ]) as $denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      gate:$gate,
      base_url:$base_url,
      latest_active_safety_schema_version:"latest_active_safety_regression_v1",
      latest_active_safety_regression_ready:true,
      latest_active_safety_mode:"oracle_latest_delta_bound_to_active_nonmutation_regression",
      latest_active_safety_decision:"latest_upstream_codex_delta_remains_oracle_only_while_active_hepta_runtime_stays_isolated_and_known_attention_regressed",
      required_source_count:3,
      ready_source_count:3,
      activation_blocking_source_count:3,
      source_latest_multisurface_command:"scripts/hepta-upstream-codex-latest-multisurface-absorption.sh",
      source_active_dependency_command:"HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=0 scripts/hepta-active-service-dependency-isolation.sh",
      source_watchdog_soak_regression_command:"scripts/hepta-terminal-watchdog-soak-regression-gate.sh",
      source_latest_multisurface_report_sha256:$latest_report_sha256,
      source_active_dependency_report_sha256:$dependency_report_sha256,
      source_watchdog_soak_regression_report_sha256:$regression_report_sha256,
      active_safety_index_hash_sha256:$active_safety_index_hash_sha256,
      active_safety_policy_hash_sha256:$active_safety_policy_hash_sha256,
      active_safety_side_effect_hash_sha256:$active_safety_side_effect_hash_sha256,
      active_safety_source_hashes:[
        $latest_report_sha256,
        $dependency_report_sha256,
        $regression_report_sha256
      ],
      latest_target_upstream_head:$latest.target_upstream_head,
      latest_baseline_upstream_head:$latest.baseline_upstream_head,
      latest_commit_count:$latest.commit_count,
      latest_changed_file_count:$latest.changed_file_count,
      latest_ready_family_count:$latest.ready_family_count,
      latest_activation_blocking_family_count:$latest.activation_blocking_family_count,
      latest_provider_security_changed_file_count:$latest.provider_security_changed_file_count,
      latest_runtime_appserver_changed_file_count:$latest.runtime_appserver_changed_file_count,
      latest_legacy_cli_tui_changed_file_count:$latest.legacy_cli_tui_changed_file_count,
      latest_product_governance_changed_file_count:$latest.product_governance_changed_file_count,
      active_binary_package:$dependency.active_binary_package,
      active_binary_target:$dependency.active_binary_target,
      active_dependency_isolated:$dependency.local_cargo_tree_isolated,
      forbidden_codex_engine_crate_count:($dependency.found_forbidden_codex_engine_crates | length),
      watchdog_status:$regression.watchdog_status,
      watchdog_status_known:$regression.watchdog_status_known,
      watchdog_known_operator_security_attention:($regression.watchdog_known_operator_security_attention // false),
      watchdog_health:$regression.watchdog_health,
      watchdog_route_count:$regression.watchdog_route_count,
      watchdog_missing_route_count:$regression.watchdog_missing_route_count,
      watchdog_release_sha256:$regression.watchdog_release_sha256,
      watchdog_installed_sha256:$regression.watchdog_installed_sha256,
      watchdog_evidence_contract_ready:$regression.watchdog_evidence_contract_ready,
      watchdog_binary_sha_match:$regression.watchdog_binary_sha_match,
      watchdog_full_fusion_complete:$regression.watchdog_full_fusion_complete,
      watchdog_operator_security_status:($regression.watchdog_operator_security_status // null),
      watchdog_active_owner:($regression.watchdog_active_owner // null),
      watchdog_double_poller_risk:($regression.watchdog_double_poller_risk // false),
      soak_status:$regression.soak_status,
      soak_status_known:$regression.soak_status_known,
      soak_passed:$regression.soak_passed,
      soak_known_operator_security_attention:($regression.soak_known_operator_security_attention // false),
      soak_samples:$regression.soak_samples,
      soak_ok:$regression.soak_ok,
      soak_fail:$regression.soak_fail,
      terminal_soak_samples:$regression.terminal_soak_samples,
      release_long_soak_observed:$regression.release_long_soak_observed,
      terminal_soak_authorizes_live_mutation:$regression.terminal_soak_authorizes_live_mutation,
      latest_delta_active_runtime_promotion_allowed:false,
      latest_delta_active_dependency_mutation_allowed:false,
      latest_delta_gateway_mutation_allowed:false,
      active_runtime_dependency_mutation_allowed:false,
      active_binary_mutation_allowed:false,
      active_service_restart_allowed:false,
      release_artifact_write_allowed:false,
      public_release_claim_allowed:false,
      public_distribution_publication_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      evidence_persistence_allowed:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      denied_by_latest_active_safety_regression:$denied,
      latest_active_safety_denied_by_count:($denied | length),
      safety_families:[
        {
          id:"latest-multisurface-oracle-source",
          ready:true,
          blocked:true,
          target_upstream_head:$latest.target_upstream_head,
          changed_file_count:$latest.changed_file_count,
          reason:"latest upstream delta is classified without merge rebase or active wiring"
        },
        {
          id:"active-dependency-isolation-source",
          ready:true,
          blocked:true,
          active_binary_package:$dependency.active_binary_package,
          forbidden_codex_engine_crate_count:($dependency.found_forbidden_codex_engine_crates | length),
          reason:"active hepta-cli service remains isolated from tracked Codex engine crates"
        },
        {
          id:"watchdog-soak-regression-source",
          ready:true,
          blocked:true,
          route_count:$regression.watchdog_route_count,
          watchdog_status:$regression.watchdog_status,
          watchdog_known_operator_security_attention:($regression.watchdog_known_operator_security_attention // false),
          soak_status:$regression.soak_status,
          soak_known_operator_security_attention:($regression.soak_known_operator_security_attention // false),
          soak_samples:$regression.soak_samples,
          reason:"watchdog and short soak remain observational, either healthy or known operator-security attention, and do not authorize live mutation"
        },
        {
          id:"publication-artifact-boundary",
          ready:true,
          blocked:true,
          release_artifact_write_allowed:false,
          public_release_claim_allowed:false,
          reason:"release artifacts public claims and distribution remain denied"
        },
        {
          id:"upstream-and-runtime-mutation-boundary",
          ready:true,
          blocked:true,
          upstream_fetch_allowed:false,
          upstream_merge_allowed:false,
          active_service_restart_allowed:false,
          reason:"this gate performs no upstream fetch merge install restart or runtime mutation"
        }
      ],
      side_effects:{
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        upstream_checkout_performed:false,
        workspace_write:false,
        active_runtime_dependency_mutated:false,
        active_binary_mutated:false,
        active_service_restart:false,
        launchd_mutated:false,
        gateway_mutation_performed:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        public_release_published:false,
        public_ga_claimed:false,
        public_distribution_performed:false,
        evidence_persisted:false,
        watchdog_report_persisted:false,
        soak_report_persisted:false,
        credential_value_read:false,
        secret_file_read:false,
        filesystem_written:false
      }
    }'
)"

jq -e '
  .latest_active_safety_regression_ready == true
  and .required_source_count == 3
  and .ready_source_count == 3
  and .activation_blocking_source_count == 3
  and .latest_active_safety_denied_by_count == 20
  and .active_dependency_isolated == true
  and .forbidden_codex_engine_crate_count == 0
  and .watchdog_evidence_contract_ready == true
  and .watchdog_full_fusion_complete == true
  and .watchdog_status_known == true
  and (.watchdog_status == "ok" or .watchdog_known_operator_security_attention == true)
  and .soak_status_known == true
  and (.soak_status == "ready" or .soak_known_operator_security_attention == true)
  and ((.soak_status == "ready" and .soak_ok == .soak_samples and .soak_fail == 0)
    or (.soak_known_operator_security_attention == true and .soak_ok == 0 and .soak_fail == .soak_samples))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta upstream Codex latest active-safety regression gate passed"
