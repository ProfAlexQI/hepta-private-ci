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

RELEASE_CLAIM_INDEX_JSON="$(
  capture_json_report \
    "hepta-terminal-non-activation-release-claim-index-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-non-activation-release-claim-index-gate.sh
)"

OPERATOR_PACKET_JSON="$(
  capture_json_report \
    "hepta-public-ga-operator-approval-packet" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-public-ga-operator-approval-packet.sh
)"

ROLLBACK_DRILL_JSON="$(
  capture_json_report \
    "hepta-live-mutation-rollback-drill-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-live-mutation-rollback-drill-gate.sh
)"

release_claim_index_report_sha256="$(sha256_text "$RELEASE_CLAIM_INDEX_JSON")"
operator_packet_report_sha256="$(sha256_text "$OPERATOR_PACKET_JSON")"
rollback_drill_report_sha256="$(sha256_text "$ROLLBACK_DRILL_JSON")"
operator_readiness_index_hash_sha256="$(sha256_text "hepta-terminal-operator-readiness-non-approval-index:index:$release_claim_index_report_sha256:$operator_packet_report_sha256:$rollback_drill_report_sha256")"
operator_readiness_policy_hash_sha256="$(sha256_text "hepta-terminal-operator-readiness-non-approval-index:policy:$release_claim_index_report_sha256:$operator_packet_report_sha256:$rollback_drill_report_sha256")"
operator_readiness_side_effect_hash_sha256="$(sha256_text "hepta-terminal-operator-readiness-non-approval-index:side-effects:$release_claim_index_report_sha256:$operator_packet_report_sha256:$rollback_drill_report_sha256")"

jq -n -e \
  --argjson release_claim "$RELEASE_CLAIM_INDEX_JSON" \
  --argjson operator_packet "$OPERATOR_PACKET_JSON" \
  --argjson rollback "$ROLLBACK_DRILL_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $release_claim.runtime == "hepta"
    and $release_claim.status == "ready"
    and $release_claim.gate == "hepta_terminal_non_activation_release_claim_index_gate"
    and $release_claim.release_claim_index_ready == true
    and $release_claim.release_claim_index_mode == "schema_only_release_claim_index_activation_blocked"
    and $release_claim.release_claim_index_decision == "release_claim_and_artifact_denied_without_activation"
    and $release_claim.required_source_count == 3
    and $release_claim.ready_source_count == 3
    and $release_claim.activation_blocking_source_count == 3
    and $release_claim.source_terminal_denied_by_count == 39
    and $release_claim.source_watchdog_evidence_contract_ready == true
    and $release_claim.source_watchdog_route_count >= 69
    and $release_claim.source_watchdog_full_fusion_complete == true
    and $release_claim.source_public_ga_claimed == false
    and $release_claim.source_public_ga_reports_synchronized == true
    and $release_claim.release_claim_denied_by_count == 47
    and ($release_claim.release_claim_families | length) == 5
    and ($release_claim.release_claim_families | all(.ready == true and .blocked == true))
    and $release_claim.readiness_allowed == false
    and $release_claim.activation_allowed == false
    and $release_claim.active_wiring_allowed == false
    and $release_claim.live_mutation_execution_ready == false
    and $release_claim.public_release_claim_allowed == false
    and $release_claim.public_ga_claim_allowed == false
    and $release_claim.release_artifact_write_allowed == false
    and $release_claim.public_artifact_write_allowed == false
    and ($release_claim.side_effects | to_entries | all(.value == false))
    and $operator_packet.runtime == "hepta"
    and $operator_packet.status == "ready"
    and $operator_packet.endpoint == "/api/hepta-public-ga-operator-approval-packet"
    and $operator_packet.approval_packet_ready == true
    and $operator_packet.safe_default_mode == "plan_only_no_live_mutation"
    and $operator_packet.required_operator_approval_count == 8
    and $operator_packet.reports_synchronized == true
    and $operator_packet.missing_route_count == 0
    and ($operator_packet.side_effects | to_entries | all(.value == false))
    and $rollback.runtime == "hepta"
    and $rollback.status == "ready"
    and $rollback.gate == "hepta_live_mutation_rollback_drill_gate"
    and $rollback.drill_mode == "dry_run_no_restore_no_restart"
    and $rollback.rollback_plan_ready == true
    and $rollback.rollback_execution_enabled == false
    and $rollback.operator_approval_required_before_execution == true
    and $rollback.release_installed_sha_match == true
    and $rollback.rollback_would_change_installed_binary == true
    and $rollback.rollback_backup_count >= 1
    and $rollback.rollback_backup_executable == true
    and $rollback.minimum_long_soak_required_samples >= 24
    and $rollback.live_mutation_enabled_count == 0
    and $rollback.live_execution_enabled_count == 0
    and $rollback.safe_default_mode == "plan_only_no_live_mutation"
    and $rollback.core_full_fusion_complete == true
    and $rollback.remaining_direct_dependency_count == 0
    and ($rollback.required_before_execution | length) == 7
    and ($rollback.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_terminal_operator_readiness_non_approval_index_gate" \
  --arg release_claim_index_report_sha256 "$release_claim_index_report_sha256" \
  --arg operator_packet_report_sha256 "$operator_packet_report_sha256" \
  --arg rollback_drill_report_sha256 "$rollback_drill_report_sha256" \
  --arg operator_readiness_index_hash_sha256 "$operator_readiness_index_hash_sha256" \
  --arg operator_readiness_policy_hash_sha256 "$operator_readiness_policy_hash_sha256" \
  --arg operator_readiness_side_effect_hash_sha256 "$operator_readiness_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson release_claim "$RELEASE_CLAIM_INDEX_JSON" \
  --argjson operator_packet "$OPERATOR_PACKET_JSON" \
  --argjson rollback "$ROLLBACK_DRILL_JSON" \
  '
    ($release_claim.denied_by_release_claim_index) as $release_denied
    | ([
        "operator_readiness_index_recording_denied",
        "operator_readiness_index_persistence_denied",
        "operator_readiness_index_materialization_denied",
        "operator_readiness_index_filesystem_write_denied",
        "operator_approval_not_recorded",
        "operator_identity_not_accepted",
        "rollback_execution_denied",
        "rollback_restore_denied",
        "launchd_restart_denied",
        "post_restore_soak_not_executed"
      ] + $release_denied) as $operator_denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_operator_readiness_non_approval_index_schema_version:"terminal_operator_readiness_non_approval_index_v1",
      minimum_required_samples:$min_long_soak_samples,
      operator_readiness_non_approval_index_ready:true,
      operator_readiness_mode:"schema_only_operator_readiness_activation_blocked",
      operator_readiness_decision:"operator_readiness_indexed_without_operator_approval_or_execution",
      source_release_claim_index_gate:$release_claim.gate,
      source_operator_packet_endpoint:$operator_packet.endpoint,
      source_rollback_drill_gate:$rollback.gate,
      source_release_claim_index_report_sha256:$release_claim_index_report_sha256,
      source_operator_packet_report_sha256:$operator_packet_report_sha256,
      source_rollback_drill_report_sha256:$rollback_drill_report_sha256,
      required_source_count:3,
      ready_source_count:3,
      activation_blocking_source_count:3,
      source_release_claim_index_ready:$release_claim.release_claim_index_ready,
      source_release_claim_denied_by_count:$release_claim.release_claim_denied_by_count,
      source_public_release_claim_allowed:$release_claim.public_release_claim_allowed,
      source_public_ga_claim_allowed:$release_claim.public_ga_claim_allowed,
      source_release_artifact_write_allowed:$release_claim.release_artifact_write_allowed,
      source_operator_packet_ready:$operator_packet.approval_packet_ready,
      source_operator_safe_default_mode:$operator_packet.safe_default_mode,
      source_required_operator_approval_count:$operator_packet.required_operator_approval_count,
      source_operator_packet_reports_synchronized:$operator_packet.reports_synchronized,
      source_operator_packet_public_ga_ready:$operator_packet.public_ga_ready,
      source_operator_packet_missing_route_count:$operator_packet.missing_route_count,
      source_rollback_plan_ready:$rollback.rollback_plan_ready,
      source_rollback_drill_mode:$rollback.drill_mode,
      source_rollback_execution_enabled:$rollback.rollback_execution_enabled,
      source_rollback_operator_approval_required_before_execution:$rollback.operator_approval_required_before_execution,
      source_release_installed_sha_match:$rollback.release_installed_sha_match,
      source_rollback_would_change_installed_binary:$rollback.rollback_would_change_installed_binary,
      source_rollback_backup_count:$rollback.rollback_backup_count,
      source_rollback_backup_executable:$rollback.rollback_backup_executable,
      source_live_execution_enabled_count:$rollback.live_execution_enabled_count,
      operator_approval_recorded:false,
      operator_identity_accepted:false,
      rollback_execution_allowed:false,
      rollback_restore_allowed:false,
      launchd_restart_allowed:false,
      post_restore_soak_executed:false,
      readiness_allowed:false,
      activation_allowed:false,
      active_wiring_allowed:false,
      live_mutation_execution_ready:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      release_artifact_write_allowed:false,
      public_artifact_write_allowed:false,
      operator_readiness_index_recorded:false,
      operator_readiness_index_persisted:false,
      operator_readiness_index_materialized:false,
      operator_readiness_index_filesystem_written:false,
      operator_readiness_index_hash_sha256:$operator_readiness_index_hash_sha256,
      operator_readiness_policy_hash_sha256:$operator_readiness_policy_hash_sha256,
      operator_readiness_side_effect_hash_sha256:$operator_readiness_side_effect_hash_sha256,
      operator_readiness_source_hashes:[
        $release_claim_index_report_sha256,
        $operator_packet_report_sha256,
        $rollback_drill_report_sha256
      ],
      operator_readiness_families:[
        {
          id:"release-claim-denial-boundary",
          ready:true,
          blocked:true,
          denied_by_count:$release_claim.release_claim_denied_by_count,
          reason:"release-claim index denies public release, public GA, and artifact writes"
        },
        {
          id:"operator-packet-non-approval-boundary",
          ready:true,
          blocked:true,
          approval_packet_ready:$operator_packet.approval_packet_ready,
          required_operator_approval_count:$operator_packet.required_operator_approval_count,
          operator_approval_recorded:false,
          reason:"operator packet is a plan-only approval checklist, not a recorded approval"
        },
        {
          id:"rollback-plan-dry-run-boundary",
          ready:true,
          blocked:true,
          rollback_plan_ready:$rollback.rollback_plan_ready,
          rollback_execution_enabled:$rollback.rollback_execution_enabled,
          reason:"rollback evidence is dry-run only and cannot restore or restart without explicit approval"
        },
        {
          id:"active-binary-integrity-non-activation-boundary",
          ready:true,
          blocked:true,
          release_installed_sha_match:$rollback.release_installed_sha_match,
          rollback_would_change_installed_binary:$rollback.rollback_would_change_installed_binary,
          reason:"binary integrity evidence confirms state but does not authorize rollback or activation"
        },
        {
          id:"operator-readiness-index-persistence-boundary",
          ready:true,
          blocked:true,
          operator_readiness_index_recorded:false,
          operator_readiness_index_persisted:false,
          operator_readiness_index_materialized:false,
          operator_readiness_index_filesystem_written:false,
          reason:"operator-readiness index is report-only and not persisted or materialized"
        },
        {
          id:"activation-public-claim-boundary",
          ready:true,
          blocked:true,
          activation_allowed:false,
          active_wiring_allowed:false,
          public_release_claim_allowed:false,
          release_artifact_write_allowed:false,
          reason:"activation, public claims, artifact writes, rollback execution, and live mutation remain denied"
        }
      ],
      operator_readiness_denied_by_count:($operator_denied | length),
      denied_by_operator_readiness_index:$operator_denied,
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
        active_service_restart:false,
        operator_readiness_index_recorded:false,
        operator_readiness_index_persisted:false,
        operator_readiness_index_materialized:false,
        operator_readiness_index_filesystem_written:false,
        release_claim_index_recorded:false,
        release_claim_index_persisted:false,
        release_claim_index_materialized:false,
        release_claim_index_filesystem_written:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        receipt_persistence_execution_performed:false,
        ledger_persistence_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_written:false,
        workspace_write_performed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        launchd_mutated:false,
        service_restarted:false,
        rollback_executed:false,
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
  and .operator_readiness_non_approval_index_ready == true
  and .operator_readiness_mode == "schema_only_operator_readiness_activation_blocked"
  and .operator_readiness_decision == "operator_readiness_indexed_without_operator_approval_or_execution"
  and .required_source_count == 3
  and .ready_source_count == 3
  and .activation_blocking_source_count == 3
  and .source_release_claim_index_ready == true
  and .source_release_claim_denied_by_count == 47
  and .source_public_release_claim_allowed == false
  and .source_public_ga_claim_allowed == false
  and .source_release_artifact_write_allowed == false
  and .source_operator_packet_ready == true
  and .source_operator_safe_default_mode == "plan_only_no_live_mutation"
  and .source_required_operator_approval_count == 8
  and .source_operator_packet_reports_synchronized == true
  and .source_operator_packet_missing_route_count == 0
  and .source_rollback_plan_ready == true
  and .source_rollback_drill_mode == "dry_run_no_restore_no_restart"
  and .source_rollback_execution_enabled == false
  and .source_rollback_operator_approval_required_before_execution == true
  and .source_release_installed_sha_match == true
  and .source_rollback_would_change_installed_binary == true
  and .source_rollback_backup_count >= 1
  and .source_rollback_backup_executable == true
  and .source_live_execution_enabled_count == 0
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .rollback_execution_allowed == false
  and .rollback_restore_allowed == false
  and .launchd_restart_allowed == false
  and .post_restore_soak_executed == false
  and .readiness_allowed == false
  and .activation_allowed == false
  and .active_wiring_allowed == false
  and .live_mutation_execution_ready == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .public_artifact_write_allowed == false
  and .operator_readiness_index_recorded == false
  and .operator_readiness_index_persisted == false
  and .operator_readiness_index_materialized == false
  and .operator_readiness_index_filesystem_written == false
  and (.operator_readiness_source_hashes | length) == 3
  and (.operator_readiness_families | length) == 6
  and (.operator_readiness_families | all(.ready == true and .blocked == true))
  and .operator_readiness_denied_by_count == 57
  and (.denied_by_operator_readiness_index | length) == .operator_readiness_denied_by_count
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta terminal operator-readiness non-approval index gate passed"
