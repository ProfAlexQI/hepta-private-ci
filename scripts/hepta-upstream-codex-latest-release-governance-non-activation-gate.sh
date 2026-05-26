#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

ACTIVE_SAFETY_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-latest-active-safety-regression" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      scripts/hepta-upstream-codex-latest-active-safety-regression.sh
)"

FINAL_AUDIT_JSON="$(
  capture_json_report \
    "hepta-terminal-release-governance-final-audit-index-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-release-governance-final-audit-index-gate.sh
)"

active_safety_report_sha256="$(sha256_text "$ACTIVE_SAFETY_JSON")"
final_audit_report_sha256="$(sha256_text "$FINAL_AUDIT_JSON")"
latest_governance_index_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-release-governance-non-activation:index:$active_safety_report_sha256:$final_audit_report_sha256")"
latest_governance_policy_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-release-governance-non-activation:policy:$active_safety_report_sha256:$final_audit_report_sha256")"
latest_governance_side_effect_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-release-governance-non-activation:side-effects:$active_safety_report_sha256:$final_audit_report_sha256")"

jq -n -e \
  --argjson safety "$ACTIVE_SAFETY_JSON" \
  --argjson final "$FINAL_AUDIT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $safety.runtime == "hepta"
    and $safety.status == "ready"
    and $safety.gate == "hepta_upstream_codex_latest_active_safety_regression_gate"
    and $safety.latest_active_safety_regression_ready == true
    and $safety.latest_target_upstream_head == "8a94430bb273623be42b68f144f1ab1df343bb53"
    and $safety.latest_baseline_upstream_head == "9f42c89c0112771dc29100a6f3fc904049b2655f"
    and $safety.latest_commit_count == 12
    and $safety.latest_changed_file_count == 57
    and $safety.latest_ready_family_count == 5
    and $safety.latest_activation_blocking_family_count == 5
    and $safety.active_dependency_isolated == true
    and $safety.forbidden_codex_engine_crate_count == 0
    and $safety.watchdog_binary_sha_match == true
    and $safety.watchdog_full_fusion_complete == true
    and $safety.soak_fail == 0
    and $safety.latest_active_safety_denied_by_count == 20
    and $safety.public_release_claim_allowed == false
    and $safety.public_distribution_publication_allowed == false
    and $safety.release_artifact_write_allowed == false
    and $safety.evidence_persistence_allowed == false
    and $safety.upstream_fetch_allowed == false
    and $safety.upstream_merge_allowed == false
    and ($safety.side_effects | to_entries | all(.value == false))
    and $final.runtime == "hepta"
    and $final.status == "ready"
    and $final.gate == "hepta_terminal_release_governance_final_audit_index_gate"
    and $final.final_audit_index_ready == true
    and $final.final_audit_denied_by_count == 127
    and $final.full_fusion_operational_evidence_observed == true
    and $final.active_binary_sha_consistent == true
    and $final.active_dependency_isolated == true
    and $final.memory_intelligence_absorbed_or_represented == true
    and $final.public_claim_denial_enforced == true
    and $final.public_distribution_denial_enforced == true
    and $final.operator_approval_non_recording_enforced == true
    and $final.active_runtime_mutation_denial_enforced == true
    and $final.public_release_claim_allowed == false
    and $final.public_ga_claim_allowed == false
    and $final.public_distribution_publication_allowed == false
    and $final.release_artifact_write_allowed == false
    and $final.public_artifact_write_allowed == false
    and $final.final_audit_index_persisted == false
    and $final.publication_evidence_summary_persisted == false
    and $final.publication_evidence_receipt_persisted == false
    and $final.publication_evidence_ledger_persisted == false
    and $final.upstream_fetch_allowed == false
    and $final.upstream_merge_allowed == false
    and $final.install_execution_allowed == false
    and $final.active_service_restart_allowed == false
    and ($final.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_upstream_codex_latest_release_governance_non_activation_gate" \
  --arg active_safety_report_sha256 "$active_safety_report_sha256" \
  --arg final_audit_report_sha256 "$final_audit_report_sha256" \
  --arg latest_governance_index_hash_sha256 "$latest_governance_index_hash_sha256" \
  --arg latest_governance_policy_hash_sha256 "$latest_governance_policy_hash_sha256" \
  --arg latest_governance_side_effect_hash_sha256 "$latest_governance_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson safety "$ACTIVE_SAFETY_JSON" \
  --argjson final "$FINAL_AUDIT_JSON" \
  '
    ([
      "latest_release_governance_index_recording_denied",
      "latest_release_governance_index_persistence_denied",
      "latest_release_governance_index_materialization_denied",
      "latest_release_governance_index_filesystem_write_denied",
      "latest_upstream_codex_public_release_claim_denied",
      "latest_upstream_codex_public_distribution_denied",
      "latest_upstream_codex_release_artifact_write_denied",
      "latest_upstream_codex_evidence_persistence_denied",
      "latest_upstream_codex_operator_approval_missing",
      "latest_upstream_codex_runtime_activation_denied",
      "latest_upstream_codex_active_dependency_mutation_denied",
      "latest_upstream_codex_install_restart_denied",
      "latest_upstream_codex_provider_invocation_denied",
      "latest_upstream_codex_channel_delivery_denied",
      "latest_upstream_codex_upstream_fetch_denied",
      "latest_upstream_codex_upstream_merge_denied",
      "latest_upstream_codex_gateway_mutation_denied",
      "latest_upstream_codex_workspace_write_denied"
    ] + $safety.denied_by_latest_active_safety_regression + $final.denied_by_final_audit_index) as $denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      latest_release_governance_non_activation_schema_version:"latest_release_governance_non_activation_v1",
      minimum_required_samples:$min_long_soak_samples,
      latest_release_governance_non_activation_ready:true,
      latest_release_governance_non_activation_mode:"latest_upstream_oracle_bound_to_terminal_release_governance_no_activation",
      latest_release_governance_non_activation_decision:"latest_upstream_codex_observation_does_not_authorize_release_claim_artifact_write_runtime_activation_or_evidence_persistence",
      required_source_count:2,
      ready_source_count:2,
      activation_blocking_source_count:2,
      source_latest_active_safety_gate:$safety.gate,
      source_terminal_final_audit_gate:$final.gate,
      source_latest_active_safety_report_sha256:$active_safety_report_sha256,
      source_terminal_final_audit_report_sha256:$final_audit_report_sha256,
      latest_release_governance_index_hash_sha256:$latest_governance_index_hash_sha256,
      latest_release_governance_policy_hash_sha256:$latest_governance_policy_hash_sha256,
      latest_release_governance_side_effect_hash_sha256:$latest_governance_side_effect_hash_sha256,
      latest_release_governance_source_hashes:[
        $active_safety_report_sha256,
        $final_audit_report_sha256
      ],
      latest_target_upstream_head:$safety.latest_target_upstream_head,
      latest_baseline_upstream_head:$safety.latest_baseline_upstream_head,
      latest_commit_count:$safety.latest_commit_count,
      latest_changed_file_count:$safety.latest_changed_file_count,
      latest_ready_family_count:$safety.latest_ready_family_count,
      latest_activation_blocking_family_count:$safety.latest_activation_blocking_family_count,
      latest_active_safety_denied_by_count:$safety.latest_active_safety_denied_by_count,
      final_audit_denied_by_count:$final.final_audit_denied_by_count,
      source_final_audit_memory_surface_count:$final.source_memory_intelligence_surface_count,
      source_final_audit_memory_absorbed_or_represented_count:$final.source_memory_intelligence_absorbed_or_represented_count,
      source_final_audit_memory_live_mutation_enabled_count:$final.source_memory_intelligence_live_mutation_enabled_count,
      active_binary_sha_consistent:$final.active_binary_sha_consistent,
      active_dependency_isolated:$final.active_dependency_isolated,
      forbidden_codex_engine_crate_count:$safety.forbidden_codex_engine_crate_count,
      full_fusion_operational_evidence_observed:$final.full_fusion_operational_evidence_observed,
      watchdog_binary_sha_match:$safety.watchdog_binary_sha_match,
      watchdog_full_fusion_complete:$safety.watchdog_full_fusion_complete,
      watchdog_route_count:$safety.watchdog_route_count,
      watchdog_missing_route_count:$safety.watchdog_missing_route_count,
      short_soak_samples:$safety.soak_samples,
      short_soak_fail:$safety.soak_fail,
      latest_oracle_only_intake_enforced:true,
      terminal_release_governance_enforced:true,
      public_claim_denial_enforced:true,
      public_distribution_denial_enforced:true,
      release_artifact_write_denial_enforced:true,
      evidence_non_persistence_enforced:true,
      operator_approval_non_recording_enforced:true,
      active_runtime_mutation_denial_enforced:true,
      upstream_fetch_allowed:false,
      upstream_merge_allowed:false,
      upstream_checkout_allowed:false,
      active_runtime_auto_rebase_allowed:false,
      active_runtime_codex_engine_dependency_allowed:false,
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
      public_distribution_artifact_write_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      evidence_persistence_allowed:false,
      final_audit_index_persistence_allowed:false,
      publication_evidence_persistence_allowed:false,
      memory_store_mutation_allowed:false,
      capability_registry_mutation_allowed:false,
      plugin_registry_mutation_allowed:false,
      skill_workshop_write_allowed:false,
      live_mutation_execution_ready:false,
      latest_release_governance_index_recorded:false,
      latest_release_governance_index_persisted:false,
      latest_release_governance_index_materialized:false,
      latest_release_governance_index_filesystem_written:false,
      latest_release_governance_denied_by_count:($denied | length),
      denied_by_latest_release_governance_non_activation:$denied,
      latest_release_governance_families:[
        {
          id:"latest-active-safety-source",
          ready:true,
          blocked:true,
          target_upstream_head:$safety.latest_target_upstream_head,
          changed_file_count:$safety.latest_changed_file_count,
          denied_by_count:$safety.latest_active_safety_denied_by_count,
          reason:"latest Codex delta remains oracle-only and active Hepta runtime safety stays green"
        },
        {
          id:"terminal-release-governance-source",
          ready:true,
          blocked:true,
          final_audit_denied_by_count:$final.final_audit_denied_by_count,
          reason:"terminal release-governance final audit denies publication, artifact writes, activation, and persistence"
        },
        {
          id:"public-claim-distribution-artifact-boundary",
          ready:true,
          blocked:true,
          public_release_claim_allowed:false,
          public_distribution_publication_allowed:false,
          release_artifact_write_allowed:false,
          reason:"latest upstream observation cannot become a public claim, distribution, or release artifact"
        },
        {
          id:"operator-approval-evidence-persistence-boundary",
          ready:true,
          blocked:true,
          operator_approval_recorded:false,
          evidence_persistence_allowed:false,
          reason:"operator approval is not recorded and latest-governance evidence is report-only"
        },
        {
          id:"active-runtime-and-dependency-boundary",
          ready:true,
          blocked:true,
          active_dependency_isolated:$final.active_dependency_isolated,
          active_runtime_codex_engine_dependency_allowed:false,
          active_service_restart_allowed:false,
          reason:"active runtime dependency mutation, install, and restart remain denied"
        },
        {
          id:"workspace-provider-channel-boundary",
          ready:true,
          blocked:true,
          upstream_merge_allowed:false,
          provider_model_invocation_allowed:false,
          channel_delivery_allowed:false,
          reason:"this gate performs no upstream mutation, workspace write, provider invocation, or channel delivery"
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
        latest_release_governance_index_recorded:false,
        latest_release_governance_index_persisted:false,
        latest_release_governance_index_materialized:false,
        latest_release_governance_index_filesystem_written:false,
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        skill_workshop_written:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        gateway_mutation_performed:false,
        runtime_store_mutated:false,
        operator_approval_recorded:false,
        operator_identity_accepted:false,
        filesystem_written:false,
        external_send_performed:false,
        credential_read:false,
        secret_file_read:false
      }
    }'
)"

jq -e '
  .latest_release_governance_non_activation_ready == true
  and .required_source_count == 2
  and .ready_source_count == 2
  and .activation_blocking_source_count == 2
  and .latest_release_governance_denied_by_count == 165
  and .latest_commit_count == 12
  and .latest_changed_file_count == 57
  and .active_dependency_isolated == true
  and .forbidden_codex_engine_crate_count == 0
  and .public_release_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .evidence_persistence_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta upstream Codex latest release-governance non-activation gate passed"
