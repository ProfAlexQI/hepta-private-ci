#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
source "$REPO_ROOT/scripts/lib/hepta-watchdog-gate-evidence-v1.sh"
cd "$REPO_ROOT"

WATCHDOG_GATE_MODE="$(hepta_watchdog_gate_mode)"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

capture_json_report_allow_parseable_failure() {
  local command_name="$1"
  shift

  local output
  local rc=0
  output="$("$@" 2>&1)" || rc=$?

  local report
  report="$(printf '%s\n' "$output" | extract_first_json_object)"

  if ! jq -e . >/dev/null <<<"$report"; then
    if [[ "$rc" -ne 0 ]]; then
      echo "$command_name failed with exit code $rc and did not emit a parseable JSON report" >&2
    else
      echo "$command_name did not emit a parseable JSON report" >&2
    fi
    echo "$command_name output tail:" >&2
    hepta_emit_capture_tail "$output"
    exit 1
  fi

  printf '%s\n' "$report"
}

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

PUBLIC_DISTRIBUTION_LOCK_JSON="$(
  capture_json_report \
    "hepta-terminal-public-distribution-non-publication-lock-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-public-distribution-non-publication-lock-gate.sh
)"

WATCHDOG_JSON="$(
  capture_json_report_allow_parseable_failure \
    "hepta-watchdog" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_WATCHDOG_MODE="$WATCHDOG_GATE_MODE" \
      scripts/hepta-watchdog.sh
)"
WATCHDOG_EVIDENCE_CONTRACT_JSON="$(
  hepta_watchdog_gate_evidence_contract_json "$WATCHDOG_JSON" "$WATCHDOG_GATE_MODE"
)"

public_distribution_lock_report_sha256="$(sha256_text "$PUBLIC_DISTRIBUTION_LOCK_JSON")"
watchdog_report_sha256="$(sha256_text "$WATCHDOG_JSON")"
publication_evidence_summary_hash_sha256="$(sha256_text "hepta-terminal-publication-evidence-non-persistence-summary:index:$public_distribution_lock_report_sha256:$watchdog_report_sha256")"
publication_evidence_policy_hash_sha256="$(sha256_text "hepta-terminal-publication-evidence-non-persistence-summary:policy:$public_distribution_lock_report_sha256:$watchdog_report_sha256")"
publication_evidence_side_effect_hash_sha256="$(sha256_text "hepta-terminal-publication-evidence-non-persistence-summary:side-effects:$public_distribution_lock_report_sha256:$watchdog_report_sha256")"

jq -n -e \
  --argjson public_distribution "$PUBLIC_DISTRIBUTION_LOCK_JSON" \
  --argjson watchdog "$WATCHDOG_JSON" \
  --argjson watchdog_contract "$WATCHDOG_EVIDENCE_CONTRACT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $public_distribution.runtime == "hepta"
    and $public_distribution.status == "ready"
    and $public_distribution.gate == "hepta_terminal_public_distribution_non_publication_lock_gate"
    and $public_distribution.public_distribution_non_publication_lock_ready == true
    and $public_distribution.public_distribution_denied_by_count == 99
    and $public_distribution.public_distribution_non_publication_enforced == true
    and $public_distribution.public_ga_non_claim_enforced == true
    and $public_distribution.operator_approval_recorded == false
    and $public_distribution.operator_identity_accepted == false
    and $public_distribution.public_distribution_publication_allowed == false
    and $public_distribution.public_distribution_artifact_write_allowed == false
    and $public_distribution.public_release_claim_allowed == false
    and $public_distribution.public_ga_claim_allowed == false
    and $public_distribution.public_release_published == false
    and $public_distribution.public_ga_claimed == false
    and $public_distribution.external_public_claim_performed == false
    and $public_distribution.external_public_distribution_performed == false
    and $public_distribution.terminal_public_distribution_non_publication_lock_recorded == false
    and $public_distribution.terminal_public_distribution_non_publication_lock_persisted == false
    and $public_distribution.terminal_public_distribution_non_publication_lock_materialized == false
    and $public_distribution.terminal_public_distribution_non_publication_lock_filesystem_written == false
    and ($public_distribution.side_effects | to_entries | all(.value == false))
    and $watchdog.runtime == "hepta"
    and (
      $watchdog.status == "ok"
      or (
        $watchdog.status == "failed"
        and $watchdog.operator_security_status == "attention"
      )
    )
    and $watchdog_contract.ready == true
    and $watchdog.health == "ready"
    and $watchdog.route_count >= 69
    and $watchdog.missing_route_count == 0
    and $watchdog.full_fusion_complete == true
    and $watchdog.phase_4_name_repository_closure_remaining_surface_count == 0
    and $watchdog.phase_5_engine_dependency_closure_remaining_dependency_count == 0
    and ($watchdog.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_terminal_publication_evidence_non_persistence_summary_gate" \
  --arg public_distribution_lock_report_sha256 "$public_distribution_lock_report_sha256" \
  --arg watchdog_report_sha256 "$watchdog_report_sha256" \
  --arg publication_evidence_summary_hash_sha256 "$publication_evidence_summary_hash_sha256" \
  --arg publication_evidence_policy_hash_sha256 "$publication_evidence_policy_hash_sha256" \
  --arg publication_evidence_side_effect_hash_sha256 "$publication_evidence_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson public_distribution "$PUBLIC_DISTRIBUTION_LOCK_JSON" \
  --argjson watchdog "$WATCHDOG_JSON" \
  --argjson watchdog_contract "$WATCHDOG_EVIDENCE_CONTRACT_JSON" \
  '
    ([
      "publication_evidence_summary_recording_denied",
      "publication_evidence_summary_persistence_denied",
      "publication_evidence_summary_materialization_denied",
      "publication_evidence_summary_filesystem_write_denied",
      "publication_evidence_receipt_persistence_denied",
      "publication_evidence_ledger_persistence_denied",
      "publication_evidence_external_send_denied",
      "publication_evidence_public_claim_denied",
      "publication_evidence_public_distribution_denied",
      "publication_evidence_artifact_write_denied",
      "publication_evidence_operator_approval_missing",
      "publication_evidence_active_runtime_mutation_denied"
    ] + $public_distribution.denied_by_public_distribution_non_publication_lock) as $publication_evidence_denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_publication_evidence_non_persistence_summary_schema_version:"terminal_publication_evidence_non_persistence_summary_v1",
      minimum_required_samples:$min_long_soak_samples,
      publication_evidence_non_persistence_summary_ready:true,
      publication_evidence_summary_mode:"schema_only_publication_evidence_summary_not_persisted",
      publication_evidence_summary_decision:"publication_evidence_summarized_without_claim_publication_artifact_write_or_runtime_mutation",
      source_public_distribution_lock_gate:$public_distribution.gate,
      source_watchdog_gate:"hepta_watchdog",
      source_public_distribution_lock_report_sha256:$public_distribution_lock_report_sha256,
      source_watchdog_report_sha256:$watchdog_report_sha256,
      required_source_count:2,
      ready_source_count:2,
      activation_blocking_source_count:2,
      source_public_distribution_lock_ready:$public_distribution.public_distribution_non_publication_lock_ready,
      source_public_distribution_denied_by_count:$public_distribution.public_distribution_denied_by_count,
      source_public_distribution_family_count:($public_distribution.public_distribution_lock_families | length),
      source_public_distribution_publication_allowed:$public_distribution.public_distribution_publication_allowed,
      source_public_distribution_artifact_write_allowed:$public_distribution.public_distribution_artifact_write_allowed,
      source_public_release_published:$public_distribution.public_release_published,
      source_public_ga_claimed:$public_distribution.public_ga_claimed,
      source_external_public_claim_performed:$public_distribution.external_public_claim_performed,
      source_external_public_distribution_performed:$public_distribution.external_public_distribution_performed,
      source_operator_approval_recorded:$public_distribution.operator_approval_recorded,
      source_operator_identity_accepted:$public_distribution.operator_identity_accepted,
      source_public_distribution_lock_persisted:$public_distribution.terminal_public_distribution_non_publication_lock_persisted,
      source_watchdog_status_known:(
        $watchdog.status == "ok"
        or (
          $watchdog.status == "failed"
        and $watchdog.operator_security_status == "attention"
        )
      ),
      source_watchdog_known_operator_security_attention:(
        $watchdog.status == "failed"
        and $watchdog.operator_security_status == "attention"
      ),
      source_watchdog_status:$watchdog.status,
      source_watchdog_operator_security_status:($watchdog.operator_security_status // "unknown"),
      source_watchdog_operator_security_attention_budget_known:($watchdog.operator_security_attention_budget_known // false),
      source_watchdog_telegram_production_attention_budget_ok:($watchdog.telegram_production_attention_budget_ok // false),
      source_watchdog_security_mode:($watchdog.security_mode // "unknown"),
      source_watchdog_active_owner:($watchdog.active_owner // "unknown"),
      source_watchdog_double_poller_risk:($watchdog.double_poller_risk // false),
      source_watchdog_gate_mode:$watchdog_contract.observed_mode,
      source_watchdog_evidence_contract_ready:$watchdog_contract.ready,
      source_watchdog_active_health_only:$watchdog_contract.active_health_only,
      source_watchdog_deployment_consistency_checked:$watchdog_contract.deployment_consistency_checked,
      source_watchdog_binary_sha_match_checked:$watchdog_contract.binary_sha_match_checked,
      source_watchdog_health:$watchdog.health,
      source_watchdog_binary_sha_match:$watchdog.binary_sha_match,
      source_watchdog_route_count:$watchdog.route_count,
      source_watchdog_missing_route_count:$watchdog.missing_route_count,
      source_watchdog_full_fusion_complete:$watchdog.full_fusion_complete,
      source_watchdog_release_sha256:$watchdog.release_sha256,
      source_watchdog_installed_sha256:$watchdog.installed_sha256,
      source_watchdog_phase_4_remaining_surface_count:$watchdog.phase_4_name_repository_closure_remaining_surface_count,
      source_watchdog_phase_5_remaining_dependency_count:$watchdog.phase_5_engine_dependency_closure_remaining_dependency_count,
      active_runtime_evidence_contract_ready:$watchdog_contract.ready,
      active_binary_sha_consistent:(
        if $watchdog_contract.binary_sha_match_checked
        then $watchdog.binary_sha_match
        else null
        end
      ),
      active_route_health_observed:true,
      publication_evidence_non_persistence_enforced:true,
      publication_claim_denial_enforced:true,
      public_distribution_denial_enforced:true,
      operator_approval_non_recording_enforced:true,
      active_state_observed:true,
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
      public_distribution_publication_allowed:false,
      public_distribution_artifact_write_allowed:false,
      release_artifact_pack_execution_allowed:false,
      recurring_watchdog_install_allowed:false,
      live_mutation_execution_ready:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      release_artifact_write_allowed:false,
      public_artifact_write_allowed:false,
      public_release_published:false,
      public_ga_claimed:false,
      external_public_claim_performed:false,
      external_public_distribution_performed:false,
      publication_evidence_summary_recorded:false,
      publication_evidence_summary_persisted:false,
      publication_evidence_summary_materialized:false,
      publication_evidence_summary_filesystem_written:false,
      publication_evidence_receipt_persisted:false,
      publication_evidence_ledger_persisted:false,
      publication_evidence_hash_sha256:$publication_evidence_summary_hash_sha256,
      publication_evidence_policy_hash_sha256:$publication_evidence_policy_hash_sha256,
      publication_evidence_side_effect_hash_sha256:$publication_evidence_side_effect_hash_sha256,
      publication_evidence_source_hashes:[
        $public_distribution_lock_report_sha256,
        $watchdog_report_sha256
      ],
      publication_evidence_families:[
        {
          id:"public-distribution-lock-source",
          ready:true,
          blocked:true,
          denied_by_count:$public_distribution.public_distribution_denied_by_count,
          reason:"public-distribution non-publication lock denies publication, public claims, operator approval recording, and artifact writes"
        },
        {
          id:"watchdog-observational-evidence-boundary",
          ready:(
            $watchdog.status == "ok"
            or (
              $watchdog.status == "failed"
        and $watchdog.operator_security_status == "attention"
            )
          ),
          blocked:true,
          status:$watchdog.status,
          binary_sha_match:$watchdog.binary_sha_match,
          route_count:$watchdog.route_count,
          full_fusion_complete:$watchdog.full_fusion_complete,
          operator_security_status:($watchdog.operator_security_status // "unknown"),
          active_owner:($watchdog.active_owner // "unknown"),
          double_poller_risk:($watchdog.double_poller_risk // false),
          reason:"watchdog evidence observes active health or known operator-security attention but does not authorize publication or persistence"
        },
        {
          id:"publication-evidence-non-persistence-boundary",
          ready:true,
          blocked:true,
          publication_evidence_summary_recorded:false,
          publication_evidence_summary_persisted:false,
          publication_evidence_summary_materialized:false,
          publication_evidence_summary_filesystem_written:false,
          reason:"publication evidence summary is report-only and not persisted or materialized"
        },
        {
          id:"public-claim-distribution-denial-boundary",
          ready:true,
          blocked:true,
          public_release_claim_allowed:false,
          public_ga_claim_allowed:false,
          public_distribution_publication_allowed:false,
          external_public_distribution_performed:false,
          reason:"public release, public GA, and external distribution remain denied"
        },
        {
          id:"operator-approval-non-recording-boundary",
          ready:true,
          blocked:true,
          operator_approval_recorded:false,
          operator_identity_accepted:false,
          reason:"operator approval packet remains plan-only and no identity acceptance is recorded"
        },
        {
          id:"active-runtime-mutation-boundary",
          ready:true,
          blocked:true,
          install_execution_allowed:false,
          active_service_restart_allowed:false,
          active_runtime_codex_engine_dependency_allowed:false,
          live_mutation_execution_ready:false,
          reason:"active runtime mutation, install, restart, and dependency mutation remain denied"
        }
      ],
      publication_evidence_denied_by_count:($publication_evidence_denied | length),
      denied_by_publication_evidence_summary:$publication_evidence_denied,
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
        external_public_distribution_performed:false,
        publication_evidence_summary_recorded:false,
        publication_evidence_summary_persisted:false,
        publication_evidence_summary_materialized:false,
        publication_evidence_summary_filesystem_written:false,
        publication_evidence_receipt_persisted:false,
        publication_evidence_ledger_persisted:false,
        operator_approval_recorded:false,
        operator_identity_accepted:false,
        filesystem_written:false,
        workspace_write_performed:false,
        external_send_performed:false,
        credential_read:false,
        secret_file_read:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .publication_evidence_non_persistence_summary_ready == true
  and .source_public_distribution_lock_ready == true
  and .source_public_distribution_denied_by_count == 99
  and .source_watchdog_status_known == true
  and (.source_watchdog_status == "ok" or .source_watchdog_known_operator_security_attention == true)
  and .source_watchdog_evidence_contract_ready == true
  and (
    (
      .source_watchdog_deployment_consistency_checked == true
      and .source_watchdog_binary_sha_match_checked == true
      and .source_watchdog_binary_sha_match == true
      and .active_binary_sha_consistent == true
    )
    or (
      .source_watchdog_active_health_only == true
      and .source_watchdog_deployment_consistency_checked == false
      and .source_watchdog_binary_sha_match_checked == false
      and .source_watchdog_binary_sha_match == false
      and .active_binary_sha_consistent == null
    )
  )
  and .source_watchdog_route_count >= 69
  and .source_watchdog_full_fusion_complete == true
  and .active_runtime_evidence_contract_ready == true
  and .publication_evidence_non_persistence_enforced == true
  and .publication_claim_denial_enforced == true
  and .public_distribution_denial_enforced == true
  and .operator_approval_non_recording_enforced == true
  and .public_distribution_publication_allowed == false
  and .public_distribution_artifact_write_allowed == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .external_public_claim_performed == false
  and .external_public_distribution_performed == false
  and .publication_evidence_summary_recorded == false
  and .publication_evidence_summary_persisted == false
  and .publication_evidence_summary_materialized == false
  and .publication_evidence_summary_filesystem_written == false
  and .publication_evidence_receipt_persisted == false
  and .publication_evidence_ledger_persisted == false
  and .install_execution_allowed == false
  and .active_service_restart_allowed == false
  and .live_mutation_execution_ready == false
  and .publication_evidence_denied_by_count == 111
  and (.publication_evidence_families | length) == 6
  and (.publication_evidence_families | all(.ready == true and .blocked == true))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta terminal publication evidence non-persistence summary gate passed"
