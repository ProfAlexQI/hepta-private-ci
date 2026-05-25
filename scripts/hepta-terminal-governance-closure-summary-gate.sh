#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

extract_first_json_object() {
  awk '
    BEGIN {
      capture = 0
      depth = 0
    }
    {
      if (!capture && $0 ~ /^[[:space:]]*\{[[:space:]]*$/) {
        capture = 1
      }
      if (capture) {
        print
        line = $0
        open_line = line
        close_line = line
        opens = gsub(/\{/, "", open_line)
        closes = gsub(/\}/, "", close_line)
        depth += opens - closes
        if (depth == 0) {
          exit
        }
      }
    }
  '
}

capture_json_report() {
  local command_name="$1"
  shift

  local output
  output="$("$@")"
  local report
  report="$(printf '%s\n' "$output" | extract_first_json_object)"

  if ! jq -e . >/dev/null <<<"$report"; then
    echo "$command_name did not emit a parseable JSON report" >&2
    exit 1
  fi

  printf '%s\n' "$report"
}

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
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

RELEASE_CLAIM_INDEX_JSON="$(
  capture_json_report \
    "hepta-terminal-non-activation-release-claim-index-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-non-activation-release-claim-index-gate.sh
)"

OPERATOR_READINESS_INDEX_JSON="$(
  capture_json_report \
    "hepta-terminal-operator-readiness-non-approval-index-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-operator-readiness-non-approval-index-gate.sh
)"

terminal_index_report_sha256="$(sha256_text "$TERMINAL_INDEX_JSON")"
release_claim_index_report_sha256="$(sha256_text "$RELEASE_CLAIM_INDEX_JSON")"
operator_readiness_index_report_sha256="$(sha256_text "$OPERATOR_READINESS_INDEX_JSON")"
governance_closure_summary_hash_sha256="$(sha256_text "hepta-terminal-governance-closure-summary:index:$terminal_index_report_sha256:$release_claim_index_report_sha256:$operator_readiness_index_report_sha256")"
governance_closure_policy_hash_sha256="$(sha256_text "hepta-terminal-governance-closure-summary:policy:$terminal_index_report_sha256:$release_claim_index_report_sha256:$operator_readiness_index_report_sha256")"
governance_closure_side_effect_hash_sha256="$(sha256_text "hepta-terminal-governance-closure-summary:side-effects:$terminal_index_report_sha256:$release_claim_index_report_sha256:$operator_readiness_index_report_sha256")"

jq -n -e \
  --argjson terminal "$TERMINAL_INDEX_JSON" \
  --argjson release_claim "$RELEASE_CLAIM_INDEX_JSON" \
  --argjson operator_readiness "$OPERATOR_READINESS_INDEX_JSON" \
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
    and $terminal.activation_allowed == false
    and $terminal.active_wiring_allowed == false
    and $terminal.live_mutation_execution_ready == false
    and ($terminal.side_effects | to_entries | all(.value == false))
    and $release_claim.runtime == "hepta"
    and $release_claim.status == "ready"
    and $release_claim.gate == "hepta_terminal_non_activation_release_claim_index_gate"
    and $release_claim.release_claim_index_ready == true
    and $release_claim.release_claim_index_mode == "schema_only_release_claim_index_activation_blocked"
    and $release_claim.release_claim_index_decision == "release_claim_and_artifact_denied_without_activation"
    and $release_claim.required_source_count == 3
    and $release_claim.ready_source_count == 3
    and $release_claim.activation_blocking_source_count == 3
    and $release_claim.release_claim_denied_by_count == 47
    and ($release_claim.release_claim_families | length) == 5
    and ($release_claim.release_claim_families | all(.ready == true and .blocked == true))
    and $release_claim.public_release_claim_allowed == false
    and $release_claim.public_ga_claim_allowed == false
    and $release_claim.release_artifact_write_allowed == false
    and $release_claim.public_artifact_write_allowed == false
    and ($release_claim.side_effects | to_entries | all(.value == false))
    and $operator_readiness.runtime == "hepta"
    and $operator_readiness.status == "ready"
    and $operator_readiness.gate == "hepta_terminal_operator_readiness_non_approval_index_gate"
    and $operator_readiness.operator_readiness_non_approval_index_ready == true
    and $operator_readiness.operator_readiness_mode == "schema_only_operator_readiness_activation_blocked"
    and $operator_readiness.operator_readiness_decision == "operator_readiness_indexed_without_operator_approval_or_execution"
    and $operator_readiness.required_source_count == 3
    and $operator_readiness.ready_source_count == 3
    and $operator_readiness.activation_blocking_source_count == 3
    and $operator_readiness.operator_readiness_denied_by_count == 57
    and ($operator_readiness.operator_readiness_families | length) == 6
    and ($operator_readiness.operator_readiness_families | all(.ready == true and .blocked == true))
    and $operator_readiness.operator_approval_recorded == false
    and $operator_readiness.operator_identity_accepted == false
    and $operator_readiness.rollback_execution_allowed == false
    and $operator_readiness.rollback_restore_allowed == false
    and $operator_readiness.launchd_restart_allowed == false
    and $operator_readiness.post_restore_soak_executed == false
    and $operator_readiness.activation_allowed == false
    and $operator_readiness.active_wiring_allowed == false
    and $operator_readiness.live_mutation_execution_ready == false
    and ($operator_readiness.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_terminal_governance_closure_summary_gate" \
  --arg terminal_index_report_sha256 "$terminal_index_report_sha256" \
  --arg release_claim_index_report_sha256 "$release_claim_index_report_sha256" \
  --arg operator_readiness_index_report_sha256 "$operator_readiness_index_report_sha256" \
  --arg governance_closure_summary_hash_sha256 "$governance_closure_summary_hash_sha256" \
  --arg governance_closure_policy_hash_sha256 "$governance_closure_policy_hash_sha256" \
  --arg governance_closure_side_effect_hash_sha256 "$governance_closure_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson terminal "$TERMINAL_INDEX_JSON" \
  --argjson release_claim "$RELEASE_CLAIM_INDEX_JSON" \
  --argjson operator_readiness "$OPERATOR_READINESS_INDEX_JSON" \
  '
    ($operator_readiness.denied_by_operator_readiness_index) as $operator_denied
    | ([
        "terminal_governance_closure_summary_recording_denied",
        "terminal_governance_closure_summary_persistence_denied",
        "terminal_governance_closure_summary_materialization_denied",
        "terminal_governance_closure_summary_filesystem_write_denied",
        "terminal_governance_activation_denied",
        "public_claim_denied_by_terminal_governance_closure",
        "rollback_execution_denied_by_terminal_governance_closure",
        "live_mutation_denied_by_terminal_governance_closure"
      ] + $operator_denied) as $governance_denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_governance_closure_summary_schema_version:"terminal_governance_closure_summary_v1",
      minimum_required_samples:$min_long_soak_samples,
      terminal_governance_closure_summary_ready:true,
      governance_closure_mode:"schema_only_terminal_governance_activation_blocked",
      governance_closure_decision:"terminal_governance_closed_without_activation_release_claim_or_operator_execution",
      source_terminal_denial_index_gate:$terminal.gate,
      source_release_claim_index_gate:$release_claim.gate,
      source_operator_readiness_index_gate:$operator_readiness.gate,
      source_terminal_index_report_sha256:$terminal_index_report_sha256,
      source_release_claim_index_report_sha256:$release_claim_index_report_sha256,
      source_operator_readiness_index_report_sha256:$operator_readiness_index_report_sha256,
      required_source_count:3,
      ready_source_count:3,
      activation_blocking_source_count:3,
      source_terminal_denial_index_ready:$terminal.terminal_denial_index_ready,
      source_terminal_denied_by_count:$terminal.terminal_denied_by_count,
      source_terminal_family_count:($terminal.terminal_families | length),
      source_release_claim_index_ready:$release_claim.release_claim_index_ready,
      source_release_claim_denied_by_count:$release_claim.release_claim_denied_by_count,
      source_release_claim_family_count:($release_claim.release_claim_families | length),
      source_operator_readiness_index_ready:$operator_readiness.operator_readiness_non_approval_index_ready,
      source_operator_readiness_denied_by_count:$operator_readiness.operator_readiness_denied_by_count,
      source_operator_readiness_family_count:($operator_readiness.operator_readiness_families | length),
      source_operator_approval_recorded:$operator_readiness.operator_approval_recorded,
      source_operator_identity_accepted:$operator_readiness.operator_identity_accepted,
      source_rollback_execution_allowed:$operator_readiness.rollback_execution_allowed,
      source_rollback_restore_allowed:$operator_readiness.rollback_restore_allowed,
      source_launchd_restart_allowed:$operator_readiness.launchd_restart_allowed,
      source_post_restore_soak_executed:$operator_readiness.post_restore_soak_executed,
      readiness_allowed:false,
      activation_allowed:false,
      active_wiring_allowed:false,
      active_runtime_auto_rebase_allowed:false,
      active_runtime_codex_engine_dependency_allowed:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      rollback_execution_allowed:false,
      rollback_restore_allowed:false,
      launchd_restart_allowed:false,
      post_restore_soak_executed:false,
      live_mutation_execution_ready:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      release_artifact_write_allowed:false,
      public_artifact_write_allowed:false,
      terminal_governance_closure_summary_recorded:false,
      terminal_governance_closure_summary_persisted:false,
      terminal_governance_closure_summary_materialized:false,
      terminal_governance_closure_summary_filesystem_written:false,
      governance_closure_summary_hash_sha256:$governance_closure_summary_hash_sha256,
      governance_closure_policy_hash_sha256:$governance_closure_policy_hash_sha256,
      governance_closure_side_effect_hash_sha256:$governance_closure_side_effect_hash_sha256,
      governance_closure_source_hashes:[
        $terminal_index_report_sha256,
        $release_claim_index_report_sha256,
        $operator_readiness_index_report_sha256
      ],
      governance_closure_families:[
        {
          id:"terminal-denial-index-closure",
          ready:true,
          blocked:true,
          denied_by_count:$terminal.terminal_denied_by_count,
          reason:"terminal denial index closes activation and upstream sync denial without activating"
        },
        {
          id:"release-claim-index-closure",
          ready:true,
          blocked:true,
          denied_by_count:$release_claim.release_claim_denied_by_count,
          reason:"release-claim index closes public release, public GA, and artifact denial"
        },
        {
          id:"operator-readiness-index-closure",
          ready:true,
          blocked:true,
          denied_by_count:$operator_readiness.operator_readiness_denied_by_count,
          reason:"operator-readiness index closes non-approval and rollback non-execution"
        },
        {
          id:"operator-execution-boundary",
          ready:true,
          blocked:true,
          operator_approval_recorded:false,
          rollback_execution_allowed:false,
          rollback_restore_allowed:false,
          launchd_restart_allowed:false,
          reason:"operator approval and rollback execution remain unrecorded and disallowed"
        },
        {
          id:"active-binary-integrity-non-activation-boundary",
          ready:true,
          blocked:true,
          release_installed_sha_match:$operator_readiness.source_release_installed_sha_match,
          rollback_would_change_installed_binary:$operator_readiness.source_rollback_would_change_installed_binary,
          reason:"binary integrity evidence is observational and does not authorize activation"
        },
        {
          id:"terminal-governance-summary-persistence-boundary",
          ready:true,
          blocked:true,
          terminal_governance_closure_summary_recorded:false,
          terminal_governance_closure_summary_persisted:false,
          terminal_governance_closure_summary_materialized:false,
          terminal_governance_closure_summary_filesystem_written:false,
          reason:"terminal governance summary is report-only and not persisted or materialized"
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
      governance_closure_denied_by_count:($governance_denied | length),
      denied_by_governance_closure_summary:$governance_denied,
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
        terminal_governance_closure_summary_recorded:false,
        terminal_governance_closure_summary_persisted:false,
        terminal_governance_closure_summary_materialized:false,
        terminal_governance_closure_summary_filesystem_written:false,
        operator_readiness_index_recorded:false,
        operator_readiness_index_persisted:false,
        operator_readiness_index_materialized:false,
        operator_readiness_index_filesystem_written:false,
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
  and .terminal_governance_closure_summary_ready == true
  and .governance_closure_mode == "schema_only_terminal_governance_activation_blocked"
  and .governance_closure_decision == "terminal_governance_closed_without_activation_release_claim_or_operator_execution"
  and .required_source_count == 3
  and .ready_source_count == 3
  and .activation_blocking_source_count == 3
  and .source_terminal_denial_index_ready == true
  and .source_terminal_denied_by_count == 39
  and .source_terminal_family_count == 6
  and .source_release_claim_index_ready == true
  and .source_release_claim_denied_by_count == 47
  and .source_release_claim_family_count == 5
  and .source_operator_readiness_index_ready == true
  and .source_operator_readiness_denied_by_count == 57
  and .source_operator_readiness_family_count == 6
  and .source_operator_approval_recorded == false
  and .source_operator_identity_accepted == false
  and .source_rollback_execution_allowed == false
  and .source_rollback_restore_allowed == false
  and .source_launchd_restart_allowed == false
  and .source_post_restore_soak_executed == false
  and .readiness_allowed == false
  and .activation_allowed == false
  and .active_wiring_allowed == false
  and .active_runtime_auto_rebase_allowed == false
  and .active_runtime_codex_engine_dependency_allowed == false
  and .upstream_fetch_allowed == false
  and .upstream_merge_allowed == false
  and .rollback_execution_allowed == false
  and .rollback_restore_allowed == false
  and .launchd_restart_allowed == false
  and .post_restore_soak_executed == false
  and .live_mutation_execution_ready == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .public_artifact_write_allowed == false
  and .terminal_governance_closure_summary_recorded == false
  and .terminal_governance_closure_summary_persisted == false
  and .terminal_governance_closure_summary_materialized == false
  and .terminal_governance_closure_summary_filesystem_written == false
  and (.governance_closure_source_hashes | length) == 3
  and (.governance_closure_families | length) == 7
  and (.governance_closure_families | all(.ready == true and .blocked == true))
  and .governance_closure_denied_by_count == 65
  and (.denied_by_governance_closure_summary | length) == .governance_closure_denied_by_count
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta terminal governance closure summary gate passed"
