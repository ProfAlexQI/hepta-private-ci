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

SUMMARY_JSON="$(
  capture_json_report \
    "hepta-readiness-denial-review-acceptance-closure-summary-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-readiness-denial-review-acceptance-closure-summary-gate.sh
)"

ACTIVATION_CLOSURE_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-readiness-closure" \
    scripts/hepta-upstream-codex-activation-readiness-closure.sh
)"

SYNC_LANE_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-sync-lane" \
    env HEPTA_UPSTREAM_CODEX_SYNC_REQUIRE_LIVE=0 \
      scripts/hepta-upstream-codex-sync-lane.sh
)"

summary_report_sha256="$(sha256_text "$SUMMARY_JSON")"
activation_closure_report_sha256="$(sha256_text "$ACTIVATION_CLOSURE_JSON")"
sync_lane_report_sha256="$(sha256_text "$SYNC_LANE_JSON")"
terminal_index_hash_sha256="$(sha256_text "hepta-terminal-denial-index:index:$summary_report_sha256:$activation_closure_report_sha256:$sync_lane_report_sha256")"
terminal_policy_hash_sha256="$(sha256_text "hepta-terminal-denial-index:policy:$summary_report_sha256:$activation_closure_report_sha256:$sync_lane_report_sha256")"
terminal_side_effect_hash_sha256="$(sha256_text "hepta-terminal-denial-index:side-effects:$summary_report_sha256:$activation_closure_report_sha256:$sync_lane_report_sha256")"

jq -n -e \
  --argjson summary "$SUMMARY_JSON" \
  --argjson activation "$ACTIVATION_CLOSURE_JSON" \
  --argjson sync "$SYNC_LANE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $summary.runtime == "hepta"
    and $summary.status == "ready"
    and $summary.gate == "hepta_readiness_denial_review_acceptance_closure_summary_gate"
    and $summary.readiness_denial_review_acceptance_closure_summary_ready == true
    and $summary.final_chain_mode == "schema_only_final_summary_activation_blocked"
    and $summary.final_chain_decision == "readiness_denial_review_acceptance_closure_summarized_without_activation"
    and $summary.summary_family_count == 6
    and $summary.ready_summary_family_count == 6
    and $summary.activation_blocking_summary_family_count == 6
    and $summary.inherited_closure_family_count == 5
    and $summary.inherited_blocked_denial_review_acceptance_fixture_count == 4
    and $summary.inherited_denial_reason_count == 19
    and $summary.terminal_summary_closed == true
    and $summary.activation_allowed == false
    and $summary.live_mutation_execution_ready == false
    and ($summary.denied_by_readiness_denial_review_acceptance_closure_summary | length) == 23
    and ($summary.side_effects | to_entries | all(.value == false))
    and $activation.product == "Hepta"
    and $activation.status == "ready"
    and $activation.closure_id == "upstream-codex-activation-readiness-closure-denial"
    and $activation.closure_status.activation_readiness_closure_ready == true
    and $activation.closure_status.activation_denied_by_default == true
    and $activation.closure_status.operator_approved_activation_ready == false
    and $activation.closure_status.active_wiring_allowed == false
    and ($activation.denied_active_decisions | to_entries | length) == 7
    and ($activation.denied_active_decisions | to_entries | all(.value == false))
    and ($activation.side_effects | to_entries | all(.value == false))
    and $sync.product == "Hepta"
    and $sync.status == "ready"
    and $sync.lane_id == "upstream-codex-sync-lane"
    and $sync.sync_mode == "classify_then_absorb_then_gate"
    and $sync.upstream_fetch_performed == false
    and $sync.upstream_latest_claimed == false
    and $sync.upstream_merge_performed == false
    and $sync.active_runtime_auto_rebase_allowed == false
    and $sync.active_runtime_codex_engine_dependency_allowed == false
    and $sync.require_live == false
    and $sync.active_dependency_isolation.local_cargo_tree_isolated == true
    and ($sync.active_dependency_isolation.found_forbidden_codex_engine_crates | length) == 0
    and ($sync.required_next_steps | length) == 5
    and ($sync.side_effects | to_entries | all(.value == false))
    and ($sync.active_dependency_isolation.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_terminal_denial_index_gate" \
  --arg summary_report_sha256 "$summary_report_sha256" \
  --arg activation_closure_report_sha256 "$activation_closure_report_sha256" \
  --arg sync_lane_report_sha256 "$sync_lane_report_sha256" \
  --arg terminal_index_hash_sha256 "$terminal_index_hash_sha256" \
  --arg terminal_policy_hash_sha256 "$terminal_policy_hash_sha256" \
  --arg terminal_side_effect_hash_sha256 "$terminal_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson summary "$SUMMARY_JSON" \
  --argjson activation "$ACTIVATION_CLOSURE_JSON" \
  --argjson sync "$SYNC_LANE_JSON" \
  '
    ($summary.denied_by_readiness_denial_review_acceptance_closure_summary) as $summary_denied
    | ($activation.denied_active_decisions | to_entries | map(select(.value == false) | ("active_decision_" + .key + "_denied"))) as $activation_denied
    | ($sync.required_next_steps | map("sync_lane_required_next_step_" + .)) as $sync_denied
    | ([
        "terminal_index_recording_denied",
        "terminal_index_persistence_denied",
        "terminal_index_materialization_denied",
        "terminal_index_filesystem_write_denied"
      ] + $summary_denied + $activation_denied + $sync_denied) as $terminal_denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_denial_index_schema_version:"terminal_denial_index_v1",
      minimum_required_samples:$min_long_soak_samples,
      terminal_denial_index_ready:true,
      terminal_index_mode:"schema_only_terminal_index_activation_blocked",
      terminal_index_decision:"activation_and_sync_denial_indexed_without_activation",
      source_summary_gate:$summary.gate,
      source_activation_closure_gate:$activation.activation_readiness_closure_gate,
      source_sync_lane_id:$sync.lane_id,
      source_summary_report_sha256:$summary_report_sha256,
      source_activation_closure_report_sha256:$activation_closure_report_sha256,
      source_sync_lane_report_sha256:$sync_lane_report_sha256,
      required_source_count:3,
      ready_source_count:3,
      activation_blocking_source_count:3,
      source_summary_ready:$summary.readiness_denial_review_acceptance_closure_summary_ready,
      source_summary_mode:$summary.final_chain_mode,
      source_summary_decision:$summary.final_chain_decision,
      source_summary_family_count:$summary.summary_family_count,
      source_summary_denied_by_count:($summary_denied | length),
      source_terminal_summary_closed:$summary.terminal_summary_closed,
      source_activation_closure_ready:$activation.closure_status.activation_readiness_closure_ready,
      source_activation_denied_by_default:$activation.closure_status.activation_denied_by_default,
      source_operator_approved_activation_ready:$activation.closure_status.operator_approved_activation_ready,
      source_active_wiring_allowed:$activation.closure_status.active_wiring_allowed,
      source_activation_denied_decision_count:($activation_denied | length),
      source_sync_mode:$sync.sync_mode,
      source_sync_upstream_fetch_performed:$sync.upstream_fetch_performed,
      source_sync_upstream_latest_claimed:$sync.upstream_latest_claimed,
      source_sync_upstream_merge_performed:$sync.upstream_merge_performed,
      source_sync_required_next_step_count:($sync.required_next_steps | length),
      source_sync_active_dependency_isolated:$sync.active_dependency_isolation.local_cargo_tree_isolated,
      source_sync_forbidden_codex_engine_crate_count:($sync.active_dependency_isolation.found_forbidden_codex_engine_crates | length),
      readiness_allowed:false,
      activation_allowed:false,
      active_wiring_allowed:false,
      active_runtime_auto_rebase_allowed:false,
      active_runtime_codex_engine_dependency_allowed:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      release_artifact_write_allowed:false,
      live_mutation_execution_ready:false,
      terminal_index_recorded:false,
      terminal_index_persisted:false,
      terminal_index_materialized:false,
      terminal_index_filesystem_written:false,
      terminal_index_hash_sha256:$terminal_index_hash_sha256,
      terminal_policy_hash_sha256:$terminal_policy_hash_sha256,
      terminal_side_effect_hash_sha256:$terminal_side_effect_hash_sha256,
      terminal_source_hashes:[
        $summary_report_sha256,
        $activation_closure_report_sha256,
        $sync_lane_report_sha256
      ],
      terminal_families:[
        {
          id:"live-mutation-denial-summary-index",
          ready:true,
          blocked:true,
          denied_by_count:($summary_denied | length),
          reason:"live mutation denial summary is terminal and activation-blocking"
        },
        {
          id:"upstream-activation-readiness-closure-index",
          ready:true,
          blocked:true,
          denied_decision_count:($activation_denied | length),
          reason:"upstream activation readiness closure denies active wiring by default"
        },
        {
          id:"upstream-sync-lane-index",
          ready:true,
          blocked:true,
          required_next_step_count:($sync.required_next_steps | length),
          reason:"upstream sync lane remains classify-only and does not fetch, merge, or rebase"
        },
        {
          id:"active-dependency-isolation-index",
          ready:true,
          blocked:true,
          forbidden_codex_engine_crate_count:($sync.active_dependency_isolation.found_forbidden_codex_engine_crates | length),
          reason:"active hepta-cli remains isolated from forbidden Codex engine crates"
        },
        {
          id:"terminal-index-persistence-boundary",
          ready:true,
          blocked:true,
          terminal_index_recorded:false,
          terminal_index_persisted:false,
          terminal_index_materialized:false,
          terminal_index_filesystem_written:false,
          reason:"terminal index is report-only and not persisted or materialized"
        },
        {
          id:"terminal-activation-boundary",
          ready:true,
          blocked:true,
          activation_allowed:false,
          active_wiring_allowed:false,
          upstream_merge_allowed:false,
          live_mutation_execution_ready:false,
          reason:"activation, active wiring, upstream merge, and live mutation remain denied"
        }
      ],
      terminal_denied_by_count:($terminal_denied | length),
      denied_by_terminal_denial_index:$terminal_denied,
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
        terminal_index_recorded:false,
        terminal_index_persisted:false,
        terminal_index_materialized:false,
        terminal_index_filesystem_written:false,
        readiness_denial_review_acceptance_closure_summary_recorded:false,
        readiness_denial_review_acceptance_closure_summary_persisted:false,
        readiness_denial_review_acceptance_closure_summary_materialized:false,
        readiness_denial_review_acceptance_closure_summary_filesystem_written:false,
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
        receipt_materialized:false,
        receipt_persisted:false,
        output_path_selected:false,
        raw_payload_inspected:false,
        live_secret_scan_performed:false,
        external_send_performed:false,
        credential_read:false,
        secret_file_read:false
      }
    }')"

jq -e '
  .status == "ready"
  and .terminal_denial_index_ready == true
  and .terminal_index_mode == "schema_only_terminal_index_activation_blocked"
  and .terminal_index_decision == "activation_and_sync_denial_indexed_without_activation"
  and .required_source_count == 3
  and .ready_source_count == 3
  and .activation_blocking_source_count == 3
  and .source_summary_denied_by_count == 23
  and .source_activation_denied_decision_count == 7
  and .source_sync_required_next_step_count == 5
  and .source_sync_forbidden_codex_engine_crate_count == 0
  and .source_terminal_summary_closed == true
  and .source_activation_denied_by_default == true
  and .source_active_wiring_allowed == false
  and .source_sync_upstream_fetch_performed == false
  and .source_sync_upstream_merge_performed == false
  and .source_sync_active_dependency_isolated == true
  and .readiness_allowed == false
  and .activation_allowed == false
  and .active_wiring_allowed == false
  and .upstream_fetch_allowed == false
  and .upstream_merge_allowed == false
  and .live_mutation_execution_ready == false
  and .terminal_index_recorded == false
  and .terminal_index_persisted == false
  and .terminal_index_materialized == false
  and .terminal_index_filesystem_written == false
  and (.terminal_source_hashes | length) == 3
  and (.terminal_families | length) == 6
  and (.terminal_families | all(.ready == true and .blocked == true))
  and .terminal_denied_by_count == 39
  and (.denied_by_terminal_denial_index | length) == .terminal_denied_by_count
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta terminal denial index gate passed"
