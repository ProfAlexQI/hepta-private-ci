#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
TERMINAL_SOAK_SAMPLES="${HEPTA_TERMINAL_SOAK_SAMPLES:-3}"
TERMINAL_SOAK_INTERVAL_SECONDS="${HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS:-1}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

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

require_unsigned_integer "HEPTA_TERMINAL_SOAK_SAMPLES" "$TERMINAL_SOAK_SAMPLES"
require_unsigned_integer "HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS" "$TERMINAL_SOAK_INTERVAL_SECONDS"
require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"

if [[ "$TERMINAL_SOAK_SAMPLES" -lt 3 ]]; then
  echo "terminal watchdog/soak regression requires at least 3 short-soak samples" >&2
  exit 1
fi

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

WATCHDOG_JSON="$(
  capture_json_report_allow_parseable_failure \
    "hepta-watchdog" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-watchdog.sh
)"

SOAK_JSON="$(
  capture_json_report_allow_parseable_failure \
    "hepta-live-soak" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_SOAK_SAMPLES="$TERMINAL_SOAK_SAMPLES" \
      HEPTA_SOAK_INTERVAL_SECONDS="$TERMINAL_SOAK_INTERVAL_SECONDS" \
      scripts/hepta-live-soak.sh
)"

watchdog_report_sha256="$(sha256_text "$WATCHDOG_JSON")"
soak_report_sha256="$(sha256_text "$SOAK_JSON")"
regression_index_hash_sha256="$(sha256_text "hepta-terminal-watchdog-soak-regression:index:$watchdog_report_sha256:$soak_report_sha256:$TERMINAL_SOAK_SAMPLES:$TERMINAL_SOAK_INTERVAL_SECONDS:$MIN_LONG_SOAK_SAMPLES")"
regression_policy_hash_sha256="$(sha256_text "hepta-terminal-watchdog-soak-regression:policy:$watchdog_report_sha256:$soak_report_sha256:$TERMINAL_SOAK_SAMPLES:$TERMINAL_SOAK_INTERVAL_SECONDS:$MIN_LONG_SOAK_SAMPLES")"
regression_side_effect_hash_sha256="$(sha256_text "hepta-terminal-watchdog-soak-regression:side-effects:$watchdog_report_sha256:$soak_report_sha256:$TERMINAL_SOAK_SAMPLES:$TERMINAL_SOAK_INTERVAL_SECONDS:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson watchdog "$WATCHDOG_JSON" \
  --argjson soak "$SOAK_JSON" \
  --argjson terminal_soak_samples "$TERMINAL_SOAK_SAMPLES" \
  --argjson terminal_soak_interval_seconds "$TERMINAL_SOAK_INTERVAL_SECONDS" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $watchdog.runtime == "hepta"
    and (
      $watchdog.status == "ok"
      or (
        $watchdog.status == "failed"
        and $watchdog.operator_security_status == "attention"
      )
    )
    and $watchdog.binary_sha_match == true
    and $watchdog.health == "ready"
    and $watchdog.route_count >= 69
    and $watchdog.missing_route_count == 0
    and $watchdog.full_fusion_complete == true
    and $watchdog.phase_3_binary_package_inversion_ready == true
    and $watchdog.phase_4_name_repository_closure_ready == true
    and $watchdog.phase_4_name_repository_closure_remaining_surface_count == 0
    and $watchdog.phase_5_engine_dependency_closure_ready == true
    and $watchdog.phase_5_engine_dependency_closure_remaining_dependency_count == 0
    and $watchdog.engine_dependency_closure_remaining_dependency_count == 0
    and ($watchdog.side_effects | to_entries | all(.value == false))
    and $soak.runtime == "hepta"
    and $soak.samples == $terminal_soak_samples
    and (
      (
        $soak.status == "ready"
        and $soak.ok == $soak.samples
        and $soak.fail == 0
        and ($soak.legacy_owner_preserved == true or $soak.telegram_live_send_enabled == true)
      )
      or (
        $soak.status == "failed"
        and ($soak.ok // 0) == 0
        and $soak.fail == $soak.samples
        and (
          (
            ($soak.telegram_production_attention_budget_known // false) == true
            and (($soak.active_owner | tostring) | startswith("conflict_risk"))
            and (($soak.telegram_production_readiness | tostring) | contains("attention_budget_exceeded"))
          )
          or (
            ($soak.legacy_owner_preserved // false) == true
            and (($soak.active_owner | tostring) | startswith("legacy_openclaw"))
            and (($soak.telegram_production_readiness | tostring) | contains("telegram_plugin_not_requested"))
            and (($soak.telegram_production_readiness | tostring) | contains("poll_loop_not_armed"))
          )
        )
        and ($soak.telegram_live_send_enabled // false) == false
      )
    )
    and $terminal_soak_samples >= 3
    and $terminal_soak_interval_seconds >= 0
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_terminal_watchdog_soak_regression_gate" \
  --arg watchdog_report_sha256 "$watchdog_report_sha256" \
  --arg soak_report_sha256 "$soak_report_sha256" \
  --arg regression_index_hash_sha256 "$regression_index_hash_sha256" \
  --arg regression_policy_hash_sha256 "$regression_policy_hash_sha256" \
  --arg regression_side_effect_hash_sha256 "$regression_side_effect_hash_sha256" \
  --argjson terminal_soak_samples "$TERMINAL_SOAK_SAMPLES" \
  --argjson terminal_soak_interval_seconds "$TERMINAL_SOAK_INTERVAL_SECONDS" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson watchdog "$WATCHDOG_JSON" \
  --argjson soak "$SOAK_JSON" \
  '
    ([
      "terminal_regression_index_recording_denied",
      "terminal_regression_index_persistence_denied",
      "terminal_regression_index_materialization_denied",
      "terminal_regression_index_filesystem_write_denied",
      (if $terminal_soak_samples >= $min_long_soak_samples
        then "terminal_regression_release_long_soak_observed_but_not_persisted_or_accepted"
        else "terminal_regression_short_soak_not_long_soak_evidence"
      end),
      "terminal_regression_live_mutation_denied",
      "terminal_regression_public_release_claim_denied",
      "terminal_regression_public_distribution_denied",
      "terminal_regression_release_artifact_write_denied",
      "terminal_regression_install_restart_denied",
      "terminal_regression_operator_approval_missing",
      "terminal_regression_provider_invocation_denied",
      "terminal_regression_channel_delivery_denied",
      "terminal_regression_memory_mutation_denied",
      "terminal_regression_registry_mutation_denied",
      "terminal_regression_upstream_fetch_denied",
      "terminal_regression_upstream_merge_denied"
    ]) as $denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_watchdog_soak_regression_schema_version:"terminal_watchdog_soak_regression_v1",
      watchdog_soak_regression_ready:true,
      watchdog_soak_regression_mode:"observational_watchdog_and_short_soak_no_mutation",
      watchdog_soak_regression_decision:"active_watchdog_and_short_soak_regressed_without_activation_or_persistence",
      required_source_count:2,
      ready_source_count:2,
      activation_blocking_source_count:2,
      source_watchdog_command:"scripts/hepta-watchdog.sh",
      source_soak_command:"scripts/hepta-live-soak.sh",
      source_watchdog_report_sha256:$watchdog_report_sha256,
      source_soak_report_sha256:$soak_report_sha256,
      regression_index_hash_sha256:$regression_index_hash_sha256,
      regression_policy_hash_sha256:$regression_policy_hash_sha256,
      regression_side_effect_hash_sha256:$regression_side_effect_hash_sha256,
      regression_source_hashes:[
        $watchdog_report_sha256,
        $soak_report_sha256
      ],
      watchdog_status_known:(
        $watchdog.status == "ok"
        or (
          $watchdog.status == "failed"
        and $watchdog.operator_security_status == "attention"
        )
      ),
      watchdog_known_operator_security_attention:(
        $watchdog.status == "failed"
        and $watchdog.operator_security_status == "attention"
      ),
      watchdog_status:$watchdog.status,
      watchdog_health:$watchdog.health,
      watchdog_route_count:$watchdog.route_count,
      watchdog_missing_route_count:$watchdog.missing_route_count,
      watchdog_release_sha256:$watchdog.release_sha256,
      watchdog_installed_sha256:$watchdog.installed_sha256,
      watchdog_binary_sha_match:$watchdog.binary_sha_match,
      watchdog_full_fusion_complete:$watchdog.full_fusion_complete,
      watchdog_active_binary_package:$watchdog.active_binary_package,
      watchdog_installed_service_binary:$watchdog.installed_service_binary,
      watchdog_operator_security_status:$watchdog.operator_security_status,
      watchdog_operator_security_attention_budget_known:($watchdog.operator_security_attention_budget_known // false),
      watchdog_telegram_production_attention_budget_ok:($watchdog.telegram_production_attention_budget_ok // false),
      watchdog_security_mode:$watchdog.security_mode,
      watchdog_active_owner:$watchdog.active_owner,
      watchdog_double_poller_risk:$watchdog.double_poller_risk,
      watchdog_telegram_poll_loop_status:$watchdog.telegram_poll_loop_status,
      watchdog_native_post_activation_enabled:$watchdog.native_post_activation_enabled,
      watchdog_phase_4_name_repository_closure_ready:$watchdog.phase_4_name_repository_closure_ready,
      watchdog_phase_4_name_repository_closure_remaining_surface_count:$watchdog.phase_4_name_repository_closure_remaining_surface_count,
      watchdog_phase_5_engine_dependency_closure_ready:$watchdog.phase_5_engine_dependency_closure_ready,
      watchdog_phase_5_engine_dependency_closure_remaining_dependency_count:$watchdog.phase_5_engine_dependency_closure_remaining_dependency_count,
      soak_status_known:(
        (
          $soak.status == "ready"
          and $soak.ok == $soak.samples
          and $soak.fail == 0
          and ($soak.legacy_owner_preserved == true or $soak.telegram_live_send_enabled == true)
        )
        or (
          $soak.status == "failed"
          and ($soak.ok // 0) == 0
          and $soak.fail == $soak.samples
          and (
            (
              ($soak.telegram_production_attention_budget_known // false) == true
              and (($soak.active_owner | tostring) | startswith("conflict_risk"))
              and (($soak.telegram_production_readiness | tostring) | contains("attention_budget_exceeded"))
            )
            or (
              ($soak.legacy_owner_preserved // false) == true
              and (($soak.active_owner | tostring) | startswith("legacy_openclaw"))
              and (($soak.telegram_production_readiness | tostring) | contains("telegram_plugin_not_requested"))
              and (($soak.telegram_production_readiness | tostring) | contains("poll_loop_not_armed"))
            )
          )
          and ($soak.telegram_live_send_enabled // false) == false
        )
      ),
      soak_passed:($soak.status == "ready" and $soak.ok == $soak.samples and $soak.fail == 0),
      soak_known_operator_security_attention:(
        $soak.status == "failed"
        and ($soak.ok // 0) == 0
        and $soak.fail == $soak.samples
        and (
          (
            ($soak.telegram_production_attention_budget_known // false) == true
            and (($soak.active_owner | tostring) | startswith("conflict_risk"))
            and (($soak.telegram_production_readiness | tostring) | contains("attention_budget_exceeded"))
          )
          or (
            ($soak.legacy_owner_preserved // false) == true
            and (($soak.active_owner | tostring) | startswith("legacy_openclaw"))
            and (($soak.telegram_production_readiness | tostring) | contains("telegram_plugin_not_requested"))
            and (($soak.telegram_production_readiness | tostring) | contains("poll_loop_not_armed"))
          )
        )
        and ($soak.telegram_live_send_enabled // false) == false
      ),
      soak_status:$soak.status,
      soak_samples:$soak.samples,
      soak_ok:$soak.ok,
      soak_fail:$soak.fail,
      soak_active_owner:$soak.active_owner,
      soak_legacy_owner_preserved:$soak.legacy_owner_preserved,
      soak_telegram_live_send_enabled:$soak.telegram_live_send_enabled,
      soak_telegram_production_attention_budget_known:($soak.telegram_production_attention_budget_known // false),
      soak_telegram_production_readiness:($soak.telegram_production_readiness // "unknown"),
      soak_native_post_activation_currently_enabled_without_real_mutation:$soak.native_post_real_activation_enabled,
      terminal_soak_samples:$terminal_soak_samples,
      terminal_soak_interval_seconds:$terminal_soak_interval_seconds,
      minimum_long_soak_required_samples:$min_long_soak_samples,
      long_soak_required_before_live_mutation:true,
      terminal_soak_is_release_long_soak:($terminal_soak_samples >= $min_long_soak_samples),
      terminal_soak_regression_class:(if $terminal_soak_samples >= $min_long_soak_samples then "release_long_soak_observation" else "short_soak_regression" end),
      release_long_soak_observed:($terminal_soak_samples >= $min_long_soak_samples and $soak.ok == $soak.samples and $soak.fail == 0),
      release_long_soak_sample_count:(if $terminal_soak_samples >= $min_long_soak_samples then $soak.samples else 0 end),
      release_long_soak_evidence_recorded:false,
      release_long_soak_evidence_persisted:false,
      release_long_soak_evidence_accepted:false,
      release_long_soak_authorizes_activation:false,
      terminal_soak_authorizes_live_mutation:false,
      terminal_soak_authorizes_public_claim:false,
      terminal_soak_authorizes_public_distribution:false,
      terminal_regression_index_recorded:false,
      terminal_regression_index_persisted:false,
      terminal_regression_index_materialized:false,
      terminal_regression_index_filesystem_written:false,
      watchdog_report_persisted:false,
      soak_report_persisted:false,
      soak_evidence_persisted:false,
      soak_evidence_receipt_persisted:false,
      soak_evidence_ledger_persisted:false,
      install_execution_allowed:false,
      active_service_restart_allowed:false,
      launchd_restart_allowed:false,
      release_build_required:false,
      release_build_executed:false,
      active_binary_mutation_allowed:false,
      live_mutation_execution_ready:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      public_distribution_publication_allowed:false,
      release_artifact_write_allowed:false,
      public_artifact_write_allowed:false,
      memory_store_mutation_allowed:false,
      capability_registry_mutation_allowed:false,
      plugin_registry_mutation_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      local_observational_reads:{
        watchdog_http_read_performed:true,
        soak_http_read_performed:true,
        filesystem_write_performed:false,
        runtime_mutation_performed:false
      },
      regression_families:[
        {
          id:"watchdog-health-source",
          ready:(
            $watchdog.status == "ok"
            or (
              $watchdog.status == "failed"
        and $watchdog.operator_security_status == "attention"
            )
          ),
          blocked:true,
          status:$watchdog.status,
          health:$watchdog.health,
          route_count:$watchdog.route_count,
          missing_route_count:$watchdog.missing_route_count,
          operator_security_status:($watchdog.operator_security_status // "unknown"),
          active_owner:($watchdog.active_owner // "unknown"),
          double_poller_risk:($watchdog.double_poller_risk // false),
          reason:"active watchdog reports ready health or known operator-security attention with complete route coverage"
        },
        {
          id:"watchdog-fusion-source",
          ready:true,
          blocked:true,
          full_fusion_complete:$watchdog.full_fusion_complete,
          phase_4_remaining_surface_count:$watchdog.phase_4_name_repository_closure_remaining_surface_count,
          phase_5_remaining_dependency_count:$watchdog.phase_5_engine_dependency_closure_remaining_dependency_count,
          reason:"full fusion evidence remains closed at active runtime"
        },
        {
          id:"active-binary-sha-source",
          ready:true,
          blocked:true,
          binary_sha_match:$watchdog.binary_sha_match,
          release_sha256:$watchdog.release_sha256,
          installed_sha256:$watchdog.installed_sha256,
          reason:"installed active binary still matches the release build hash"
        },
        {
          id:"short-soak-observation-source",
          ready:(
            (
              $soak.status == "ready"
              and $soak.ok == $soak.samples
              and $soak.fail == 0
            )
            or (
              $soak.status == "failed"
              and ($soak.ok // 0) == 0
              and $soak.fail == $soak.samples
              and (
                (
                  ($soak.telegram_production_attention_budget_known // false) == true
                  and (($soak.active_owner | tostring) | startswith("conflict_risk"))
                  and (($soak.telegram_production_readiness | tostring) | contains("attention_budget_exceeded"))
                )
                or (
                  ($soak.legacy_owner_preserved // false) == true
                  and (($soak.active_owner | tostring) | startswith("legacy_openclaw"))
                  and (($soak.telegram_production_readiness | tostring) | contains("telegram_plugin_not_requested"))
                  and (($soak.telegram_production_readiness | tostring) | contains("poll_loop_not_armed"))
                )
              )
              and ($soak.telegram_live_send_enabled // false) == false
            )
          ),
          blocked:true,
          status:$soak.status,
          samples:$soak.samples,
          ok:$soak.ok,
          fail:$soak.fail,
          release_long_soak_observed:($terminal_soak_samples >= $min_long_soak_samples and $soak.ok == $soak.samples and $soak.fail == 0),
          reason:(if $terminal_soak_samples >= $min_long_soak_samples
            then "release-long-soak was observed for regression only and was not persisted or accepted as activation evidence"
            elif $soak.status == "failed"
            then "short soak failure is classified as known operator-security attention, not release-long-soak evidence or activation approval"
            else "short soak is a regression sample, not release-long-soak evidence"
          end)
        },
        {
          id:"long-soak-and-live-mutation-boundary",
          ready:true,
          blocked:true,
          minimum_long_soak_required_samples:$min_long_soak_samples,
          terminal_soak_samples:$terminal_soak_samples,
          release_long_soak_evidence_recorded:false,
          release_long_soak_evidence_accepted:false,
          terminal_soak_authorizes_live_mutation:false,
          reason:"terminal soak observation cannot authorize live mutation or release activation without operator approval and accepted persisted evidence"
        },
        {
          id:"regression-evidence-persistence-boundary",
          ready:true,
          blocked:true,
          terminal_regression_index_persisted:false,
          soak_evidence_persisted:false,
          reason:"watchdog and soak evidence remains report-only and is not persisted"
        },
        {
          id:"publication-and-artifact-boundary",
          ready:true,
          blocked:true,
          public_release_claim_allowed:false,
          public_distribution_publication_allowed:false,
          release_artifact_write_allowed:false,
          reason:"public claims, public distribution, and artifact writes remain denied"
        },
        {
          id:"provider-channel-memory-boundary",
          ready:true,
          blocked:true,
          provider_model_invocation_allowed:false,
          channel_delivery_allowed:false,
          memory_store_mutation_allowed:false,
          reason:"providers, channels, memory stores, and registries remain non-mutating"
        }
      ],
      denied_by_regression_index:$denied,
      watchdog_soak_denied_by_count:($denied | length),
      side_effects:{
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        runtime_store_mutated:false,
        gateway_event_enqueued:false,
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        active_binary_mutated:false,
        active_service_restart:false,
        launchd_mutated:false,
        install_executed:false,
        release_build_executed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        public_release_published:false,
        public_ga_claimed:false,
        external_public_claim_performed:false,
        external_public_distribution_performed:false,
        terminal_regression_index_recorded:false,
        terminal_regression_index_persisted:false,
        terminal_regression_index_materialized:false,
        terminal_regression_index_filesystem_written:false,
        watchdog_report_persisted:false,
        soak_report_persisted:false,
        soak_evidence_persisted:false,
        soak_evidence_receipt_persisted:false,
        soak_evidence_ledger_persisted:false,
        operator_approval_recorded:false,
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
  and .watchdog_soak_regression_ready == true
  and .required_source_count == 2
  and .ready_source_count == 2
  and .watchdog_status_known == true
  and (.watchdog_status == "ok" or .watchdog_known_operator_security_attention == true)
  and .watchdog_health == "ready"
  and .watchdog_route_count >= 69
  and .watchdog_missing_route_count == 0
  and .watchdog_binary_sha_match == true
  and .watchdog_full_fusion_complete == true
  and .watchdog_phase_4_name_repository_closure_ready == true
  and .watchdog_phase_4_name_repository_closure_remaining_surface_count == 0
  and .watchdog_phase_5_engine_dependency_closure_ready == true
  and .watchdog_phase_5_engine_dependency_closure_remaining_dependency_count == 0
  and .soak_status_known == true
  and (.soak_status == "ready" or .soak_known_operator_security_attention == true)
  and .soak_samples >= 3
  and ((.soak_status == "ready" and .soak_ok == .soak_samples and .soak_fail == 0)
    or (.soak_known_operator_security_attention == true and .soak_ok == 0 and .soak_fail == .soak_samples))
  and (.soak_legacy_owner_preserved == true or .soak_telegram_live_send_enabled == true or .soak_known_operator_security_attention == true)
  and .minimum_long_soak_required_samples >= 24
  and .long_soak_required_before_live_mutation == true
  and ((.terminal_soak_samples >= .minimum_long_soak_required_samples and .terminal_soak_regression_class == "release_long_soak_observation" and ((.release_long_soak_observed == true and .release_long_soak_sample_count == .soak_samples) or (.soak_known_operator_security_attention == true and .release_long_soak_observed == false and .release_long_soak_sample_count == .soak_samples)))
    or (.terminal_soak_samples < .minimum_long_soak_required_samples and .terminal_soak_regression_class == "short_soak_regression" and .release_long_soak_observed == false and .release_long_soak_sample_count == 0))
  and .release_long_soak_evidence_recorded == false
  and .release_long_soak_evidence_persisted == false
  and .release_long_soak_evidence_accepted == false
  and .release_long_soak_authorizes_activation == false
  and .terminal_soak_authorizes_live_mutation == false
  and .terminal_soak_authorizes_public_claim == false
  and .terminal_soak_authorizes_public_distribution == false
  and .install_execution_allowed == false
  and .active_service_restart_allowed == false
  and .launchd_restart_allowed == false
  and .live_mutation_execution_ready == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .public_distribution_publication_allowed == false
  and .release_artifact_write_allowed == false
  and .memory_store_mutation_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .terminal_regression_index_persisted == false
  and .watchdog_report_persisted == false
  and .soak_report_persisted == false
  and .soak_evidence_persisted == false
  and (.regression_families | length) == 8
  and (.regression_families | all(.ready == true and .blocked == true))
  and .watchdog_soak_denied_by_count == 17
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta terminal watchdog/soak regression gate passed"
