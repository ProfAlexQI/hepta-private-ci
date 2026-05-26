#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$(dirname "$0")/.."

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

LATEST_GOVERNANCE_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-latest-release-governance-non-activation" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-upstream-codex-latest-release-governance-non-activation-gate.sh
)"

OPERATOR_PACKET_JSON="$(
  capture_json_report \
    "hepta-public-ga-operator-approval-packet" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-public-ga-operator-approval-packet.sh
)"

latest_governance_report_sha256="$(sha256_text "$LATEST_GOVERNANCE_JSON")"
operator_packet_report_sha256="$(sha256_text "$OPERATOR_PACKET_JSON")"
briefing_index_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-operator-briefing:index:$latest_governance_report_sha256:$operator_packet_report_sha256")"
briefing_policy_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-operator-briefing:policy:$latest_governance_report_sha256:$operator_packet_report_sha256")"
briefing_side_effect_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-operator-briefing:side-effects:$latest_governance_report_sha256:$operator_packet_report_sha256")"

jq -n -e \
  --argjson latest "$LATEST_GOVERNANCE_JSON" \
  --argjson operator_packet "$OPERATOR_PACKET_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $latest.runtime == "hepta"
    and $latest.status == "ready"
    and $latest.gate == "hepta_upstream_codex_latest_release_governance_non_activation_gate"
    and $latest.latest_release_governance_non_activation_ready == true
    and $latest.latest_target_upstream_head == "8a94430bb273623be42b68f144f1ab1df343bb53"
    and $latest.latest_baseline_upstream_head == "9f42c89c0112771dc29100a6f3fc904049b2655f"
    and $latest.latest_commit_count == 12
    and $latest.latest_changed_file_count == 57
    and $latest.required_source_count == 2
    and $latest.ready_source_count == 2
    and $latest.activation_blocking_source_count == 2
    and $latest.latest_release_governance_denied_by_count == 165
    and $latest.active_dependency_isolated == true
    and $latest.forbidden_codex_engine_crate_count == 0
    and $latest.full_fusion_operational_evidence_observed == true
    and $latest.public_release_claim_allowed == false
    and $latest.public_ga_claim_allowed == false
    and $latest.public_distribution_publication_allowed == false
    and $latest.release_artifact_write_allowed == false
    and $latest.public_artifact_write_allowed == false
    and $latest.evidence_persistence_allowed == false
    and $latest.final_audit_index_persistence_allowed == false
    and $latest.publication_evidence_persistence_allowed == false
    and $latest.operator_approval_non_recording_enforced == true
    and $latest.active_runtime_mutation_denial_enforced == true
    and $latest.upstream_fetch_allowed == false
    and $latest.upstream_merge_allowed == false
    and $latest.active_service_restart_allowed == false
    and $latest.provider_model_invocation_allowed == false
    and $latest.channel_delivery_allowed == false
    and ($latest.side_effects | to_entries | all(.value == false))
    and $operator_packet.runtime == "hepta"
    and $operator_packet.status == "ready"
    and $operator_packet.endpoint == "/api/hepta-public-ga-operator-approval-packet"
    and $operator_packet.approval_packet_ready == true
    and $operator_packet.safe_default_mode == "plan_only_no_live_mutation"
    and $operator_packet.required_operator_approval_count == 8
    and $operator_packet.reports_synchronized == true
    and $operator_packet.missing_route_count == 0
    and ($operator_packet.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_upstream_codex_latest_operator_briefing_non_persistence_gate" \
  --arg latest_governance_report_sha256 "$latest_governance_report_sha256" \
  --arg operator_packet_report_sha256 "$operator_packet_report_sha256" \
  --arg briefing_index_hash_sha256 "$briefing_index_hash_sha256" \
  --arg briefing_policy_hash_sha256 "$briefing_policy_hash_sha256" \
  --arg briefing_side_effect_hash_sha256 "$briefing_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson latest "$LATEST_GOVERNANCE_JSON" \
  --argjson operator_packet "$OPERATOR_PACKET_JSON" \
  '
    ([
      "latest_operator_briefing_recording_denied",
      "latest_operator_briefing_persistence_denied",
      "latest_operator_briefing_materialization_denied",
      "latest_operator_briefing_filesystem_write_denied",
      "latest_operator_briefing_channel_delivery_denied",
      "latest_operator_briefing_external_send_denied",
      "latest_operator_briefing_telegram_send_denied",
      "operator_approval_not_recorded",
      "operator_identity_not_accepted",
      "latest_upstream_operator_action_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "evidence_persistence_denied",
      "active_runtime_activation_denied",
      "provider_model_invocation_denied",
      "gateway_mutation_denied"
    ] + $latest.denied_by_latest_release_governance_non_activation) as $denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      latest_operator_briefing_schema_version:"latest_operator_briefing_non_persistence_v1",
      minimum_required_samples:$min_long_soak_samples,
      latest_operator_briefing_ready:true,
      latest_operator_briefing_mode:"latest_upstream_governance_bound_to_operator_packet_no_persistence",
      latest_operator_briefing_decision:"operator_briefing_is_report_only_and_does_not_authorize_approval_persistence_delivery_release_or_activation",
      required_source_count:2,
      ready_source_count:2,
      activation_blocking_source_count:2,
      source_latest_governance_gate:$latest.gate,
      source_operator_packet_endpoint:$operator_packet.endpoint,
      source_latest_governance_report_sha256:$latest_governance_report_sha256,
      source_operator_packet_report_sha256:$operator_packet_report_sha256,
      latest_operator_briefing_index_hash_sha256:$briefing_index_hash_sha256,
      latest_operator_briefing_policy_hash_sha256:$briefing_policy_hash_sha256,
      latest_operator_briefing_side_effect_hash_sha256:$briefing_side_effect_hash_sha256,
      latest_operator_briefing_source_hashes:[
        $latest_governance_report_sha256,
        $operator_packet_report_sha256
      ],
      latest_target_upstream_head:$latest.latest_target_upstream_head,
      latest_baseline_upstream_head:$latest.latest_baseline_upstream_head,
      latest_commit_count:$latest.latest_commit_count,
      latest_changed_file_count:$latest.latest_changed_file_count,
      latest_ready_family_count:$latest.latest_ready_family_count,
      latest_activation_blocking_family_count:$latest.latest_activation_blocking_family_count,
      source_latest_governance_denied_by_count:$latest.latest_release_governance_denied_by_count,
      source_operator_packet_ready:$operator_packet.approval_packet_ready,
      source_operator_safe_default_mode:$operator_packet.safe_default_mode,
      source_required_operator_approval_count:$operator_packet.required_operator_approval_count,
      source_operator_packet_reports_synchronized:$operator_packet.reports_synchronized,
      source_operator_packet_public_ga_ready:$operator_packet.public_ga_ready,
      source_operator_packet_public_ga_blocker_count:$operator_packet.public_ga_blocker_count,
      source_operator_packet_missing_route_count:$operator_packet.missing_route_count,
      active_dependency_isolated:$latest.active_dependency_isolated,
      forbidden_codex_engine_crate_count:$latest.forbidden_codex_engine_crate_count,
      active_binary_sha_consistent:$latest.active_binary_sha_consistent,
      full_fusion_operational_evidence_observed:$latest.full_fusion_operational_evidence_observed,
      public_claim_denial_enforced:$latest.public_claim_denial_enforced,
      public_distribution_denial_enforced:$latest.public_distribution_denial_enforced,
      release_artifact_write_denial_enforced:$latest.release_artifact_write_denial_enforced,
      evidence_non_persistence_enforced:$latest.evidence_non_persistence_enforced,
      operator_approval_non_recording_enforced:$latest.operator_approval_non_recording_enforced,
      active_runtime_mutation_denial_enforced:$latest.active_runtime_mutation_denial_enforced,
      latest_operator_briefing_recorded:false,
      latest_operator_briefing_persisted:false,
      latest_operator_briefing_materialized:false,
      latest_operator_briefing_filesystem_written:false,
      latest_operator_briefing_channel_delivered:false,
      latest_operator_briefing_external_sent:false,
      latest_operator_briefing_telegram_sent:false,
      operator_approval_recorded:false,
      operator_identity_accepted:false,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      upstream_checkout_allowed:false,
      active_runtime_auto_rebase_allowed:false,
      active_runtime_dependency_mutation_allowed:false,
      active_binary_mutation_allowed:false,
      active_service_restart_allowed:false,
      install_execution_allowed:false,
      launchd_restart_allowed:false,
      release_build_required:false,
      release_build_executed:false,
      release_artifact_write_allowed:false,
      public_artifact_write_allowed:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      public_distribution_publication_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      evidence_persistence_allowed:false,
      final_audit_index_persistence_allowed:false,
      publication_evidence_persistence_allowed:false,
      memory_store_mutation_allowed:false,
      capability_registry_mutation_allowed:false,
      plugin_registry_mutation_allowed:false,
      skill_workshop_write_allowed:false,
      gateway_mutation_allowed:false,
      live_mutation_execution_ready:false,
      latest_operator_briefing_denied_by_count:($denied | length),
      denied_by_latest_operator_briefing_non_persistence:$denied,
      latest_operator_briefing_sections:[
        {
          id:"latest-upstream-delta",
          ready:true,
          blocked:true,
          target_upstream_head:$latest.latest_target_upstream_head,
          changed_file_count:$latest.latest_changed_file_count,
          reason:"latest Codex delta is summarized from the non-activation governance gate"
        },
        {
          id:"active-runtime-status",
          ready:true,
          blocked:true,
          active_dependency_isolated:$latest.active_dependency_isolated,
          forbidden_codex_engine_crate_count:$latest.forbidden_codex_engine_crate_count,
          reason:"briefing observes active runtime health but cannot mutate active dependencies or restart services"
        },
        {
          id:"release-governance-boundary",
          ready:true,
          blocked:true,
          public_release_claim_allowed:false,
          release_artifact_write_allowed:false,
          reason:"release claims and artifacts remain denied"
        },
        {
          id:"operator-approval-boundary",
          ready:true,
          blocked:true,
          required_operator_approval_count:$operator_packet.required_operator_approval_count,
          operator_approval_recorded:false,
          reason:"operator packet readiness is not operator approval"
        },
        {
          id:"persistence-and-delivery-boundary",
          ready:true,
          blocked:true,
          latest_operator_briefing_persisted:false,
          channel_delivery_allowed:false,
          reason:"this briefing is emitted to stdout only and is not saved or delivered by the gate"
        }
      ],
      side_effects:{
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        upstream_checkout_performed:false,
        workspace_write_performed:false,
        active_runtime_auto_rebase_performed:false,
        active_runtime_dependency_mutated:false,
        active_binary_mutated:false,
        active_service_restart:false,
        install_executed:false,
        launchd_mutated:false,
        release_build_executed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        public_release_published:false,
        public_ga_claimed:false,
        public_distribution_performed:false,
        final_audit_index_persisted:false,
        publication_evidence_summary_persisted:false,
        publication_evidence_receipt_persisted:false,
        publication_evidence_ledger_persisted:false,
        latest_operator_briefing_recorded:false,
        latest_operator_briefing_persisted:false,
        latest_operator_briefing_materialized:false,
        latest_operator_briefing_filesystem_written:false,
        latest_operator_briefing_channel_delivered:false,
        latest_operator_briefing_external_sent:false,
        latest_operator_briefing_telegram_sent:false,
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        skill_workshop_written:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        telegram_send_performed:false,
        external_send_performed:false,
        gateway_mutation_performed:false,
        runtime_store_mutated:false,
        operator_approval_recorded:false,
        operator_identity_accepted:false,
        filesystem_written:false,
        credential_read:false,
        secret_file_read:false
      }
    }'
)"

jq -e '
  .latest_operator_briefing_ready == true
  and .required_source_count == 2
  and .ready_source_count == 2
  and .activation_blocking_source_count == 2
  and .latest_operator_briefing_denied_by_count == 181
  and .latest_commit_count == 12
  and .latest_changed_file_count == 57
  and .active_dependency_isolated == true
  and .forbidden_codex_engine_crate_count == 0
  and .source_operator_packet_ready == true
  and .source_required_operator_approval_count == 8
  and .operator_approval_recorded == false
  and .latest_operator_briefing_persisted == false
  and .latest_operator_briefing_channel_delivered == false
  and .public_release_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .evidence_persistence_allowed == false
  and (.latest_operator_briefing_sections | length) == 5
  and (.latest_operator_briefing_sections | all(.ready == true and .blocked == true))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta upstream Codex latest operator briefing non-persistence gate passed"
