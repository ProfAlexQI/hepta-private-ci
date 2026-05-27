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

CORE_SUMMARY_JSON="$(
  capture_json_report \
    "hepta-core-activation-readiness-summary-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-readiness-summary-gate.sh
)"

FRESHNESS_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-freshness-policy" \
    scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh
)"

BINDING_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-binding-record" \
    scripts/hepta-upstream-codex-activation-evidence-binding-record.sh
)"

SCOREBOARD_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-completeness-scoreboard" \
    scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh
)"

PUBLIC_GA_PACKET_JSON="$(
  capture_json_report \
    "hepta-public-ga-operator-approval-packet" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-public-ga-operator-approval-packet.sh
)"

core_summary_report_sha256="$(sha256_text "$CORE_SUMMARY_JSON")"
freshness_policy_report_sha256="$(sha256_text "$FRESHNESS_JSON")"
binding_record_report_sha256="$(sha256_text "$BINDING_JSON")"
scoreboard_report_sha256="$(sha256_text "$SCOREBOARD_JSON")"
public_ga_operator_packet_report_sha256="$(sha256_text "$PUBLIC_GA_PACKET_JSON")"
approval_packet_index_hash_sha256="$(sha256_text "hepta-core-activation-long-soak-operator-approval-packet:index:$core_summary_report_sha256:$freshness_policy_report_sha256:$binding_record_report_sha256:$scoreboard_report_sha256:$public_ga_operator_packet_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
approval_packet_policy_hash_sha256="$(sha256_text "hepta-core-activation-long-soak-operator-approval-packet:policy:$core_summary_report_sha256:$freshness_policy_report_sha256:$binding_record_report_sha256:$scoreboard_report_sha256:$public_ga_operator_packet_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
approval_packet_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-long-soak-operator-approval-packet:side-effects:$core_summary_report_sha256:$freshness_policy_report_sha256:$binding_record_report_sha256:$scoreboard_report_sha256:$public_ga_operator_packet_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson core "$CORE_SUMMARY_JSON" \
  --argjson freshness "$FRESHNESS_JSON" \
  --argjson binding "$BINDING_JSON" \
  --argjson scoreboard "$SCOREBOARD_JSON" \
  --argjson public_packet "$PUBLIC_GA_PACKET_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $core.runtime == "hepta"
    and $core.status == "ready"
    and $core.gate == "hepta_core_activation_readiness_summary_gate"
    and $core.core_activation_readiness_summary_ready == true
    and $core.verdict == "blocked_until_explicit_operator_approval_and_fresh_live_evidence"
    and $core.public_release_verdict == "blocked"
    and $core.live_memory_mutation_verdict == "blocked"
    and $core.release_artifact_publication_verdict == "blocked"
    and $core.active_runtime_mutation_verdict == "blocked"
    and $core.active_service_health_verdict == "observed_ready_not_authorizing_activation"
    and $core.minimum_required_long_soak_samples >= 24
    and $core.required_source_count == 5
    and $core.ready_source_count == 5
    and $core.activation_blocking_source_count == 5
    and $core.final_audit_ready == true
    and $core.final_release_governance_audit_ready == true
    and $core.final_audit_denied_by_count == 127
    and $core.release_publication_denial_ready == true
    and $core.release_publication_allowed_count == 0
    and $core.active_dependency_isolated == true
    and $core.forbidden_codex_engine_crate_count == 0
    and ($core.found_forbidden_codex_engine_crates | length) == 0
    and $core.watchdog_route_count == 69
    and $core.watchdog_missing_route_count == 0
    and $core.watchdog_binary_sha_match == true
    and $core.watchdog_full_fusion_complete == true
    and $core.short_soak_samples >= 3
    and $core.short_soak_ok == $core.short_soak_samples
    and $core.short_soak_fail == 0
    and $core.short_soak_authorizes_live_mutation == false
    and $core.memory_intelligence_consumed_by_active_stack == true
    and $core.memory_intelligence_core_boundary_ready == true
    and $core.runtime_memory_intelligence_dependencies_ready == true
    and $core.memory_intelligence_surface_count == 14
    and $core.memory_intelligence_absorbed_or_represented_count == 14
    and $core.memory_intelligence_gap_only_surface_count == 0
    and $core.memory_intelligence_live_mutation_enabled_count == 0
    and $core.operator_approval_recorded == false
    and $core.activation_allowed == false
    and $core.live_mutation_execution_ready == false
    and $core.memory_store_mutation_allowed == false
    and $core.public_release_claim_allowed == false
    and ($core.readiness_families | length) == 6
    and ($core.readiness_families | all(.ready == true and .blocked == true))
    and ($core.side_effects | to_entries | all(.value == false))
    and $freshness.product == "Hepta"
    and $freshness.status == "ready"
    and $freshness.policy_status.required_evidence_count == 8
    and $freshness.policy_status.policy_entry_count == 8
    and $freshness.policy_status.missing_evidence_count == 8
    and $freshness.policy_status.fresh_evidence_count == 0
    and $freshness.policy_status.freshness_policy_ready == true
    and $freshness.policy_status.activation_blocked_by_freshness_policy == true
    and $freshness.policy_status.activation_allowed_by_freshness_policy == false
    and $freshness.policy_status.active_wiring_allowed == false
    and ($freshness.evidence_freshness_entries | length) == 8
    and ($freshness.evidence_freshness_entries | all(.recorded == false and .fresh == false))
    and $freshness.denied_active_decisions.public_release_claim_allowed == false
    and $freshness.denied_active_decisions.release_artifact_write_allowed == false
    and ($freshness.side_effects | to_entries | all(.value == false))
    and $binding.product == "Hepta"
    and $binding.status == "ready"
    and $binding.manifest_status.required_evidence_count == 8
    and $binding.manifest_status.binding_record_count == 8
    and $binding.manifest_status.missing_binding_record_count == 8
    and $binding.manifest_status.recorded_binding_record_count == 0
    and $binding.manifest_status.required_record_schema_field_count == 7
    and $binding.manifest_status.recorded_record_schema_field_count == 0
    and $binding.manifest_status.binding_manifest_ready == true
    and $binding.manifest_status.activation_blocked_by_binding_manifest == true
    and $binding.manifest_status.activation_allowed_by_binding_manifest == false
    and $binding.manifest_status.active_wiring_allowed == false
    and ($binding.required_record_fields | length) == 7
    and ($binding.binding_records | length) == 8
    and ($binding.binding_records | all(.evidence_recorded == false))
    and $binding.denied_active_decisions.public_release_claim_allowed == false
    and $binding.denied_active_decisions.release_artifact_write_allowed == false
    and ($binding.side_effects | to_entries | all(.value == false))
    and $scoreboard.product == "Hepta"
    and $scoreboard.status == "ready"
    and $scoreboard.scoreboard_status.required_gate_family_count == 10
    and $scoreboard.scoreboard_status.ready_gate_family_count == 10
    and $scoreboard.scoreboard_status.activation_blocking_gate_family_count == 10
    and $scoreboard.scoreboard_status.required_evidence_count == 8
    and $scoreboard.scoreboard_status.required_trusted_record_count == 8
    and $scoreboard.scoreboard_status.accepted_trusted_record_count == 0
    and $scoreboard.scoreboard_status.fresh_trusted_record_count == 0
    and $scoreboard.scoreboard_status.operator_approval_recorded == false
    and $scoreboard.scoreboard_status.activation_request_recorded == false
    and $scoreboard.scoreboard_status.public_claim_attempt_blocked == true
    and $scoreboard.scoreboard_status.release_artifact_write_attempt_blocked == true
    and $scoreboard.scoreboard_status.operator_approved_activation_ready == false
    and $scoreboard.scoreboard_status.evidence_completeness_scoreboard_ready == true
    and $scoreboard.scoreboard_status.activation_blocked_by_scoreboard == true
    and $scoreboard.scoreboard_status.activation_allowed_by_scoreboard == false
    and $scoreboard.scoreboard_status.active_wiring_allowed == false
    and ($scoreboard.gate_families | length) == 10
    and $scoreboard.denied_active_decisions.public_release_claim_allowed == false
    and $scoreboard.denied_active_decisions.release_artifact_write_allowed == false
    and ($scoreboard.side_effects | to_entries | all(.value == false))
    and $public_packet.runtime == "hepta"
    and $public_packet.status == "ready"
    and $public_packet.approval_packet_ready == true
    and $public_packet.safe_default_mode == "plan_only_no_live_mutation"
    and $public_packet.native_gateway_source_command_count == 69
    and $public_packet.missing_route_count == 0
    and $public_packet.required_operator_approval_count == 8
    and $public_packet.reports_synchronized == true
    and ($public_packet.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_long_soak_operator_approval_packet_gate" \
  --arg core_summary_report_sha256 "$core_summary_report_sha256" \
  --arg freshness_policy_report_sha256 "$freshness_policy_report_sha256" \
  --arg binding_record_report_sha256 "$binding_record_report_sha256" \
  --arg scoreboard_report_sha256 "$scoreboard_report_sha256" \
  --arg public_ga_operator_packet_report_sha256 "$public_ga_operator_packet_report_sha256" \
  --arg approval_packet_index_hash_sha256 "$approval_packet_index_hash_sha256" \
  --arg approval_packet_policy_hash_sha256 "$approval_packet_policy_hash_sha256" \
  --arg approval_packet_side_effect_hash_sha256 "$approval_packet_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson core "$CORE_SUMMARY_JSON" \
  --argjson freshness "$FRESHNESS_JSON" \
  --argjson binding "$BINDING_JSON" \
  --argjson scoreboard "$SCOREBOARD_JSON" \
  --argjson public_packet "$PUBLIC_GA_PACKET_JSON" \
  '
    ([
      {id:"core-readiness-summary", ready:true, blocked:true, source_gate:$core.gate, source_report_sha256:$core_summary_report_sha256},
      {id:"freshness-policy", ready:true, blocked:true, source_gate:$freshness.freshness_policy_gate, source_report_sha256:$freshness_policy_report_sha256},
      {id:"binding-record-manifest", ready:true, blocked:true, source_gate:$binding.binding_manifest_gate, source_report_sha256:$binding_record_report_sha256},
      {id:"evidence-completeness-scoreboard", ready:true, blocked:true, source_gate:$scoreboard.evidence_completeness_scoreboard_gate, source_report_sha256:$scoreboard_report_sha256},
      {id:"public-ga-operator-packet", ready:true, blocked:true, source_gate:"scripts/hepta-public-ga-operator-approval-packet.sh", source_report_sha256:$public_ga_operator_packet_report_sha256},
      {id:"long-soak-record", ready:true, blocked:true, missing_record_count:1},
      {id:"operator-identity-approval", ready:true, blocked:true, missing_record_count:3},
      {id:"activation-side-effect-boundary", ready:true, blocked:true, mutation_surface_count:18}
    ]) as $families
    | ([
      "activation_request_id",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "active_binary_sha256",
      "live_dependency_isolation_evidence_id",
      "watchdog_evidence_id",
      "browser_smoke_evidence_id",
      "long_soak_evidence_id",
      "rollback_plan_id",
      "release_publication_denial_evidence_id",
      "memory_intelligence_boundary_evidence_id",
      "final_release_governance_audit_id",
      "no_public_claim_decision",
      "no_release_artifact_write_decision",
      "post_activation_watchdog_soak_plan_id"
    ]) as $required_fields
    | ([
      "operator_approval_not_recorded",
      "activation_request_not_recorded",
      "operator_identity_hash_not_recorded",
      "fresh_24_sample_long_soak_evidence_not_recorded",
      "fresh_trusted_evidence_count_zero",
      "accepted_trusted_record_count_zero",
      "evidence_binding_records_missing",
      "readiness_summary_blocks_activation",
      "public_ga_packet_is_plan_only",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "memory_store_mutation_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "install_restart_active_binary_mutation_denied",
      "upstream_fetch_merge_denied"
    ]) as $denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      core_activation_long_soak_operator_approval_packet_schema_version:"hepta_core_activation_long_soak_operator_approval_packet_v1",
      long_soak_operator_approval_packet_ready:true,
      packet_mode:"schema_only_no_activation_no_persistence",
      approval_packet_decision:"blocked_until_operator_approval_and_fresh_24_sample_evidence_records_exist",
      source_core_activation_readiness_summary_gate:$core.gate,
      source_freshness_policy_gate:$freshness.freshness_policy_gate,
      source_binding_record_gate:$binding.binding_manifest_gate,
      source_completeness_scoreboard_gate:$scoreboard.evidence_completeness_scoreboard_gate,
      source_public_ga_operator_approval_packet_gate:"scripts/hepta-public-ga-operator-approval-packet.sh",
      source_core_activation_readiness_summary_report_sha256:$core_summary_report_sha256,
      source_freshness_policy_report_sha256:$freshness_policy_report_sha256,
      source_binding_record_report_sha256:$binding_record_report_sha256,
      source_completeness_scoreboard_report_sha256:$scoreboard_report_sha256,
      source_public_ga_operator_approval_packet_report_sha256:$public_ga_operator_packet_report_sha256,
      source_report_hashes:[
        $core_summary_report_sha256,
        $freshness_policy_report_sha256,
        $binding_record_report_sha256,
        $scoreboard_report_sha256,
        $public_ga_operator_packet_report_sha256
      ],
      approval_packet_index_hash_sha256:$approval_packet_index_hash_sha256,
      approval_packet_policy_hash_sha256:$approval_packet_policy_hash_sha256,
      approval_packet_side_effect_hash_sha256:$approval_packet_side_effect_hash_sha256,
      required_source_count:5,
      ready_source_count:5,
      activation_blocking_source_count:5,
      minimum_required_long_soak_samples:$min_long_soak_samples,
      required_evidence_count:$freshness.policy_status.required_evidence_count,
      missing_evidence_count:$freshness.policy_status.missing_evidence_count,
      fresh_evidence_count:$freshness.policy_status.fresh_evidence_count,
      required_binding_record_count:$binding.manifest_status.binding_record_count,
      missing_binding_record_count:$binding.manifest_status.missing_binding_record_count,
      recorded_binding_record_count:$binding.manifest_status.recorded_binding_record_count,
      required_trusted_record_count:$scoreboard.scoreboard_status.required_trusted_record_count,
      accepted_trusted_record_count:$scoreboard.scoreboard_status.accepted_trusted_record_count,
      fresh_trusted_record_count:$scoreboard.scoreboard_status.fresh_trusted_record_count,
      required_approval_packet_field_count:($required_fields | length),
      recorded_approval_packet_field_count:0,
      required_operator_approval_count:$public_packet.required_operator_approval_count,
      public_ga_operator_packet_ready:$public_packet.approval_packet_ready,
      public_ga_safe_default_mode:$public_packet.safe_default_mode,
      reports_synchronized:$public_packet.reports_synchronized,
      active_binary_package:$core.active_binary_package,
      active_binary_target:$core.active_binary_target,
      forbidden_codex_engine_crate_count:$core.forbidden_codex_engine_crate_count,
      watchdog_route_count:$core.watchdog_route_count,
      watchdog_missing_route_count:$core.watchdog_missing_route_count,
      watchdog_binary_sha_match:$core.watchdog_binary_sha_match,
      watchdog_full_fusion_complete:$core.watchdog_full_fusion_complete,
      short_soak_samples:$core.short_soak_samples,
      short_soak_ok:$core.short_soak_ok,
      short_soak_fail:$core.short_soak_fail,
      short_soak_authorizes_live_mutation:$core.short_soak_authorizes_live_mutation,
      long_soak_evidence_recorded:false,
      long_soak_evidence_fresh:false,
      operator_approval_recorded:false,
      operator_identity_hash_recorded:false,
      activation_request_recorded:false,
      approval_packet_recorded:false,
      approval_packet_persisted:false,
      approval_packet_accepted:false,
      operator_approved_activation_ready:false,
      activation_allowed:false,
      live_mutation_execution_ready:false,
      memory_store_mutation_allowed:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      public_distribution_publication_allowed:false,
      release_artifact_write_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      install_restart_allowed:false,
      upstream_fetch_merge_allowed:false,
      required_approval_packet_fields:$required_fields,
      approval_packet_readiness_families:$families,
      denied_by_long_soak_operator_approval_packet:$denied,
      denied_by_long_soak_operator_approval_packet_count:($denied | length),
      side_effects:{
        workspace_written:false,
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        telegram_send_performed:false,
        gateway_event_enqueued:false,
        gateway_rpc_performed:false,
        external_network_read:false,
        external_send_performed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        public_release_published:false,
        public_ga_claimed:false,
        install_executed:false,
        launchd_mutated:false,
        service_restarted:false,
        active_binary_mutated:false,
        rollback_executed:false,
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        credential_read:false,
        secret_value_read:false,
        approval_packet_persisted:false,
        long_soak_evidence_persisted:false,
        trusted_record_persisted:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_long_soak_operator_approval_packet_gate"
  and .long_soak_operator_approval_packet_ready == true
  and .packet_mode == "schema_only_no_activation_no_persistence"
  and .approval_packet_decision == "blocked_until_operator_approval_and_fresh_24_sample_evidence_records_exist"
  and .required_source_count == 5
  and .ready_source_count == 5
  and .activation_blocking_source_count == 5
  and .minimum_required_long_soak_samples >= 24
  and .required_evidence_count == 8
  and .missing_evidence_count == 8
  and .fresh_evidence_count == 0
  and .required_binding_record_count == 8
  and .missing_binding_record_count == 8
  and .recorded_binding_record_count == 0
  and .required_trusted_record_count == 8
  and .accepted_trusted_record_count == 0
  and .fresh_trusted_record_count == 0
  and .required_approval_packet_field_count == 16
  and .recorded_approval_packet_field_count == 0
  and .required_operator_approval_count == 8
  and .public_ga_operator_packet_ready == true
  and .public_ga_safe_default_mode == "plan_only_no_live_mutation"
  and .reports_synchronized == true
  and .forbidden_codex_engine_crate_count == 0
  and .watchdog_route_count == 69
  and .watchdog_missing_route_count == 0
  and .watchdog_binary_sha_match == true
  and .watchdog_full_fusion_complete == true
  and .short_soak_samples >= 3
  and .short_soak_ok == .short_soak_samples
  and .short_soak_fail == 0
  and .short_soak_authorizes_live_mutation == false
  and .long_soak_evidence_recorded == false
  and .long_soak_evidence_fresh == false
  and .operator_approval_recorded == false
  and .operator_identity_hash_recorded == false
  and .activation_request_recorded == false
  and .approval_packet_recorded == false
  and .approval_packet_persisted == false
  and .approval_packet_accepted == false
  and .operator_approved_activation_ready == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .memory_store_mutation_allowed == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .public_distribution_publication_allowed == false
  and .release_artifact_write_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .install_restart_allowed == false
  and .upstream_fetch_merge_allowed == false
  and (.required_approval_packet_fields | length) == 16
  and (.approval_packet_readiness_families | length) == 8
  and (.approval_packet_readiness_families | all(.ready == true and .blocked == true))
  and .denied_by_long_soak_operator_approval_packet_count == 16
  and (.denied_by_long_soak_operator_approval_packet | length) == 16
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
