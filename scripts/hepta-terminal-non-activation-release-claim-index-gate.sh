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

TERMINAL_INDEX_JSON="$(
  capture_json_report \
    "hepta-terminal-denial-index-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-denial-index-gate.sh
)"

WATCHDOG_JSON="$(
  capture_json_report_allow_parseable_failure \
    "hepta-watchdog" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_WATCHDOG_MODE=deployment-consistency \
      scripts/hepta-watchdog.sh
)"

PUBLIC_GA_JSON="$(
  capture_json_report \
    "hepta-public-ga-readiness" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-public-ga-readiness.sh
)"

terminal_index_report_sha256="$(sha256_text "$TERMINAL_INDEX_JSON")"
watchdog_report_sha256="$(sha256_text "$WATCHDOG_JSON")"
public_ga_report_sha256="$(sha256_text "$PUBLIC_GA_JSON")"
release_claim_index_hash_sha256="$(sha256_text "hepta-terminal-non-activation-release-claim-index:index:$terminal_index_report_sha256:$watchdog_report_sha256:$public_ga_report_sha256")"
release_claim_policy_hash_sha256="$(sha256_text "hepta-terminal-non-activation-release-claim-index:policy:$terminal_index_report_sha256:$watchdog_report_sha256:$public_ga_report_sha256")"
release_claim_side_effect_hash_sha256="$(sha256_text "hepta-terminal-non-activation-release-claim-index:side-effects:$terminal_index_report_sha256:$watchdog_report_sha256:$public_ga_report_sha256")"

jq -n -e \
  --argjson terminal "$TERMINAL_INDEX_JSON" \
  --argjson watchdog "$WATCHDOG_JSON" \
  --argjson ga "$PUBLIC_GA_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $terminal.runtime == "hepta"
    and $terminal.status == "ready"
    and $terminal.gate == "hepta_terminal_denial_index_gate"
    and $terminal.terminal_denial_index_ready == true
    and $terminal.terminal_index_mode == "schema_only_terminal_index_activation_blocked"
    and $terminal.terminal_index_decision == "activation_and_sync_denial_indexed_without_activation"
    and $terminal.required_source_count == 3
    and $terminal.ready_source_count == 3
    and $terminal.activation_blocking_source_count == 3
    and $terminal.terminal_denied_by_count == 39
    and ($terminal.terminal_families | length) == 6
    and ($terminal.terminal_families | all(.ready == true and .blocked == true))
    and $terminal.readiness_allowed == false
    and $terminal.activation_allowed == false
    and $terminal.active_wiring_allowed == false
    and $terminal.upstream_fetch_allowed == false
    and $terminal.upstream_merge_allowed == false
    and $terminal.public_release_claim_allowed == false
    and $terminal.public_ga_claim_allowed == false
    and $terminal.release_artifact_write_allowed == false
    and $terminal.live_mutation_execution_ready == false
    and $terminal.terminal_index_recorded == false
    and $terminal.terminal_index_persisted == false
    and $terminal.terminal_index_materialized == false
    and $terminal.terminal_index_filesystem_written == false
    and ($terminal.side_effects | to_entries | all(.value == false))
    and $watchdog.product == "Hepta"
    and $watchdog.runtime == "hepta"
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
    and $watchdog.phase_4_name_repository_closure_remaining_surface_count == 0
    and $watchdog.phase_5_engine_dependency_closure_remaining_dependency_count == 0
    and $watchdog.engine_dependency_closure_remaining_dependency_count == 0
    and ($watchdog.side_effects | to_entries | all(.value == false))
    and $ga.product == "Hepta"
    and $ga.runtime == "hepta"
    and $ga.status == "ready"
    and $ga.endpoint == "/api/hepta-public-ga-readiness"
    and $ga.public_ga_claimed == false
    and $ga.reports_synchronized == true
    and $ga.local_gate_matrix_ready == true
    and $ga.local_reports_synchronized == true
    and $ga.native_gateway_source_command_count >= 69
    and $ga.missing_route_count == 0
    and $ga.side_effects.public_release_published == false
    and $ga.side_effects.release_artifact_written == false
    and $ga.side_effects.launchd_mutated == false
    and $ga.side_effects.credential_read == false
    and $ga.side_effects.provider_invoked == false
    and $ga.side_effects.model_invoked == false
    and $ga.side_effects.channel_read_performed == false
    and $ga.side_effects.channel_send_performed == false
    and $ga.side_effects.telegram_owner_handoff_performed == false
    and $ga.side_effects.telegram_read_performed == false
    and $ga.side_effects.telegram_send_performed == false
    and $ga.side_effects.native_post_mutation_performed == false
    and $ga.side_effects.external_network_read == false
    and $ga.side_effects.external_send_performed == false
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_terminal_non_activation_release_claim_index_gate" \
  --arg terminal_index_report_sha256 "$terminal_index_report_sha256" \
  --arg watchdog_report_sha256 "$watchdog_report_sha256" \
  --arg public_ga_report_sha256 "$public_ga_report_sha256" \
  --arg release_claim_index_hash_sha256 "$release_claim_index_hash_sha256" \
  --arg release_claim_policy_hash_sha256 "$release_claim_policy_hash_sha256" \
  --arg release_claim_side_effect_hash_sha256 "$release_claim_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson terminal "$TERMINAL_INDEX_JSON" \
  --argjson watchdog "$WATCHDOG_JSON" \
  --argjson ga "$PUBLIC_GA_JSON" \
  '
    ($terminal.denied_by_terminal_denial_index) as $terminal_denied
    | ([
        "release_claim_index_recording_denied",
        "release_claim_index_persistence_denied",
        "release_claim_index_materialization_denied",
        "release_claim_index_filesystem_write_denied",
        "public_release_claim_denied_by_terminal_index",
        "public_ga_claim_denied_by_terminal_index",
        "release_artifact_write_denied_by_terminal_index",
        "public_artifact_write_denied_by_terminal_index"
      ] + $terminal_denied) as $release_denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_non_activation_release_claim_index_schema_version:"terminal_non_activation_release_claim_index_v1",
      minimum_required_samples:$min_long_soak_samples,
      release_claim_index_ready:true,
      release_claim_index_mode:"schema_only_release_claim_index_activation_blocked",
      release_claim_index_decision:"release_claim_and_artifact_denied_without_activation",
      source_terminal_denial_index_gate:$terminal.gate,
      source_watchdog_status:$watchdog.status,
      source_public_ga_readiness_endpoint:$ga.endpoint,
      source_terminal_index_report_sha256:$terminal_index_report_sha256,
      source_watchdog_report_sha256:$watchdog_report_sha256,
      source_public_ga_report_sha256:$public_ga_report_sha256,
      required_source_count:3,
      ready_source_count:3,
      activation_blocking_source_count:3,
      source_terminal_denial_index_ready:$terminal.terminal_denial_index_ready,
      source_terminal_denied_by_count:$terminal.terminal_denied_by_count,
      source_terminal_family_count:($terminal.terminal_families | length),
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
      source_watchdog_binary_sha_match:$watchdog.binary_sha_match,
      source_watchdog_health:$watchdog.health,
      source_watchdog_status:$watchdog.status,
      source_watchdog_operator_security_status:($watchdog.operator_security_status // "unknown"),
      source_watchdog_operator_security_attention_budget_known:($watchdog.operator_security_attention_budget_known // false),
      source_watchdog_telegram_production_attention_budget_ok:($watchdog.telegram_production_attention_budget_ok // false),
      source_watchdog_security_mode:($watchdog.security_mode // "unknown"),
      source_watchdog_active_owner:($watchdog.active_owner // "unknown"),
      source_watchdog_double_poller_risk:($watchdog.double_poller_risk // false),
      source_watchdog_route_count:$watchdog.route_count,
      source_watchdog_full_fusion_complete:$watchdog.full_fusion_complete,
      source_watchdog_phase_4_remaining_surface_count:$watchdog.phase_4_name_repository_closure_remaining_surface_count,
      source_watchdog_phase_5_remaining_dependency_count:$watchdog.phase_5_engine_dependency_closure_remaining_dependency_count,
      source_public_ga_ready:$ga.public_ga_ready,
      source_public_ga_claimed:$ga.public_ga_claimed,
      source_public_ga_reports_synchronized:$ga.reports_synchronized,
      source_public_ga_missing_route_count:$ga.missing_route_count,
      readiness_allowed:false,
      activation_allowed:false,
      active_wiring_allowed:false,
      live_mutation_execution_ready:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      release_artifact_write_allowed:false,
      public_artifact_write_allowed:false,
      release_claim_index_recorded:false,
      release_claim_index_persisted:false,
      release_claim_index_materialized:false,
      release_claim_index_filesystem_written:false,
      release_claim_index_hash_sha256:$release_claim_index_hash_sha256,
      release_claim_policy_hash_sha256:$release_claim_policy_hash_sha256,
      release_claim_side_effect_hash_sha256:$release_claim_side_effect_hash_sha256,
      release_claim_source_hashes:[
        $terminal_index_report_sha256,
        $watchdog_report_sha256,
        $public_ga_report_sha256
      ],
      release_claim_families:[
        {
          id:"terminal-denial-index-release-claim-boundary",
          ready:true,
          blocked:true,
          denied_by_count:$terminal.terminal_denied_by_count,
          reason:"terminal denial index already denies activation, public claims, and artifact writes"
        },
        {
          id:"watchdog-operational-health-non-claim-boundary",
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
          reason:"operational health or known operator-security attention is denial evidence, not a public release claim"
        },
        {
          id:"public-ga-readiness-non-claim-boundary",
          ready:true,
          blocked:true,
          public_ga_ready:$ga.public_ga_ready,
          public_ga_claimed:$ga.public_ga_claimed,
          reason:"public GA readiness reports remain side-effect-free and never claim release"
        },
        {
          id:"release-artifact-write-boundary",
          ready:true,
          blocked:true,
          release_artifact_write_allowed:false,
          public_artifact_write_allowed:false,
          reason:"release and public artifact writes remain denied by terminal policy"
        },
        {
          id:"release-claim-index-persistence-boundary",
          ready:true,
          blocked:true,
          release_claim_index_recorded:false,
          release_claim_index_persisted:false,
          release_claim_index_materialized:false,
          release_claim_index_filesystem_written:false,
          reason:"release-claim index is report-only and not persisted or materialized"
        }
      ],
      release_claim_denied_by_count:($release_denied | length),
      denied_by_release_claim_index:$release_denied,
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
        release_claim_index_recorded:false,
        release_claim_index_persisted:false,
        release_claim_index_materialized:false,
        release_claim_index_filesystem_written:false,
        terminal_index_recorded:false,
        terminal_index_persisted:false,
        terminal_index_materialized:false,
        terminal_index_filesystem_written:false,
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
  and .release_claim_index_ready == true
  and .release_claim_index_mode == "schema_only_release_claim_index_activation_blocked"
  and .release_claim_index_decision == "release_claim_and_artifact_denied_without_activation"
  and .required_source_count == 3
  and .ready_source_count == 3
  and .activation_blocking_source_count == 3
  and .source_terminal_denial_index_ready == true
  and .source_terminal_denied_by_count == 39
  and .source_terminal_family_count == 6
  and .source_watchdog_status_known == true
  and (.source_watchdog_status == "ok" or .source_watchdog_known_operator_security_attention == true)
  and .source_watchdog_binary_sha_match == true
  and .source_watchdog_health == "ready"
  and .source_watchdog_route_count >= 69
  and .source_watchdog_full_fusion_complete == true
  and .source_watchdog_phase_4_remaining_surface_count == 0
  and .source_watchdog_phase_5_remaining_dependency_count == 0
  and .source_public_ga_claimed == false
  and .source_public_ga_reports_synchronized == true
  and .source_public_ga_missing_route_count == 0
  and .readiness_allowed == false
  and .activation_allowed == false
  and .active_wiring_allowed == false
  and .live_mutation_execution_ready == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .public_artifact_write_allowed == false
  and .release_claim_index_recorded == false
  and .release_claim_index_persisted == false
  and .release_claim_index_materialized == false
  and .release_claim_index_filesystem_written == false
  and (.release_claim_source_hashes | length) == 3
  and (.release_claim_families | length) == 5
  and (.release_claim_families | all(.ready == true and .blocked == true))
  and .release_claim_denied_by_count == 47
  and (.denied_by_release_claim_index | length) == .release_claim_denied_by_count
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta terminal non-activation release-claim index gate passed"
