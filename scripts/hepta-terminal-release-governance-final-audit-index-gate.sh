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

PUBLICATION_EVIDENCE_JSON="$(
  capture_json_report \
    "hepta-terminal-publication-evidence-non-persistence-summary-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-terminal-publication-evidence-non-persistence-summary-gate.sh
)"

DEPENDENCY_ISOLATION_JSON="$(
  capture_json_report \
    "hepta-active-service-dependency-isolation" \
    env HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=0 \
      scripts/hepta-active-service-dependency-isolation.sh
)"

MEMORY_INTELLIGENCE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-closure" \
    scripts/hepta-memory-intelligence-closure.sh
)"

publication_evidence_report_sha256="$(sha256_text "$PUBLICATION_EVIDENCE_JSON")"
dependency_isolation_report_sha256="$(sha256_text "$DEPENDENCY_ISOLATION_JSON")"
memory_intelligence_report_sha256="$(sha256_text "$MEMORY_INTELLIGENCE_JSON")"
final_audit_index_hash_sha256="$(sha256_text "hepta-terminal-release-governance-final-audit-index:index:$publication_evidence_report_sha256:$dependency_isolation_report_sha256:$memory_intelligence_report_sha256")"
final_audit_policy_hash_sha256="$(sha256_text "hepta-terminal-release-governance-final-audit-index:policy:$publication_evidence_report_sha256:$dependency_isolation_report_sha256:$memory_intelligence_report_sha256")"
final_audit_side_effect_hash_sha256="$(sha256_text "hepta-terminal-release-governance-final-audit-index:side-effects:$publication_evidence_report_sha256:$dependency_isolation_report_sha256:$memory_intelligence_report_sha256")"

jq -n -e \
  --argjson publication "$PUBLICATION_EVIDENCE_JSON" \
  --argjson dependency "$DEPENDENCY_ISOLATION_JSON" \
  --argjson memory "$MEMORY_INTELLIGENCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $publication.runtime == "hepta"
    and $publication.status == "ready"
    and $publication.gate == "hepta_terminal_publication_evidence_non_persistence_summary_gate"
    and $publication.publication_evidence_non_persistence_summary_ready == true
    and $publication.publication_evidence_denied_by_count == 111
    and $publication.public_distribution_denial_enforced == true
    and $publication.publication_claim_denial_enforced == true
    and $publication.operator_approval_non_recording_enforced == true
    and $publication.active_runtime_evidence_contract_ready == true
    and $publication.public_release_claim_allowed == false
    and $publication.public_ga_claim_allowed == false
    and $publication.public_distribution_publication_allowed == false
    and $publication.release_artifact_write_allowed == false
    and $publication.publication_evidence_summary_persisted == false
    and $publication.publication_evidence_receipt_persisted == false
    and ($publication.side_effects | to_entries | all(.value == false))
    and $dependency.runtime == "hepta"
    and $dependency.status == "ready"
    and $dependency.gate == "hepta_active_service_dependency_isolation_gate"
    and $dependency.local_cargo_tree_isolated == true
    and ($dependency.found_forbidden_codex_engine_crates | length) == 0
    and $dependency.live_check_status == "skipped"
    and ($dependency.side_effects | to_entries | all(.value == false))
    and $memory.runtime == "hepta"
    and $memory.status == "attention"
    and $memory.compatibility_mode == "hepta_memory_intelligence_closure_gate"
    and $memory.active_service_stack_consumes_memory_intelligence == true
    and $memory.hepta_core_dependency_boundary_ready == true
    and $memory.runtime_memory_intelligence_dependencies_ready == true
    and $memory.memory_surface_count == 14
    and $memory.absorbed_or_represented_count == 14
    and $memory.gap_report_ready_count == 14
    and $memory.live_mutation_enabled_count == 0
    and $memory.gap_only_surface_count == 0
    and ($memory.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_terminal_release_governance_final_audit_index_gate" \
  --arg publication_evidence_report_sha256 "$publication_evidence_report_sha256" \
  --arg dependency_isolation_report_sha256 "$dependency_isolation_report_sha256" \
  --arg memory_intelligence_report_sha256 "$memory_intelligence_report_sha256" \
  --arg final_audit_index_hash_sha256 "$final_audit_index_hash_sha256" \
  --arg final_audit_policy_hash_sha256 "$final_audit_policy_hash_sha256" \
  --arg final_audit_side_effect_hash_sha256 "$final_audit_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson publication "$PUBLICATION_EVIDENCE_JSON" \
  --argjson dependency "$DEPENDENCY_ISOLATION_JSON" \
  --argjson memory "$MEMORY_INTELLIGENCE_JSON" \
  '
    ([
      "final_audit_index_recording_denied",
      "final_audit_index_persistence_denied",
      "final_audit_index_materialization_denied",
      "final_audit_index_filesystem_write_denied",
      "final_audit_activation_denied",
      "final_audit_public_release_claim_denied",
      "final_audit_public_distribution_denied",
      "final_audit_release_artifact_write_denied",
      "final_audit_operator_approval_missing",
      "final_audit_install_restart_denied",
      "final_audit_active_dependency_mutation_denied",
      "final_audit_live_mutation_denied",
      "final_audit_memory_live_mutation_denied",
      "final_audit_provider_invocation_denied",
      "final_audit_channel_delivery_denied",
      "final_audit_upstream_merge_denied"
    ] + $publication.denied_by_publication_evidence_summary) as $final_audit_denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_release_governance_final_audit_index_schema_version:"terminal_release_governance_final_audit_index_v1",
      minimum_required_samples:$min_long_soak_samples,
      final_audit_index_ready:true,
      final_audit_index_mode:"schema_only_release_governance_final_audit_no_activation",
      final_audit_index_decision:"release_governance_audited_without_publication_or_live_mutation",
      source_publication_evidence_gate:$publication.gate,
      source_dependency_isolation_gate:$dependency.gate,
      source_memory_intelligence_gate:$memory.compatibility_mode,
      source_publication_evidence_report_sha256:$publication_evidence_report_sha256,
      source_dependency_isolation_report_sha256:$dependency_isolation_report_sha256,
      source_memory_intelligence_report_sha256:$memory_intelligence_report_sha256,
      required_source_count:3,
      ready_source_count:3,
      activation_blocking_source_count:3,
      source_publication_evidence_ready:$publication.publication_evidence_non_persistence_summary_ready,
      source_publication_evidence_denied_by_count:$publication.publication_evidence_denied_by_count,
      source_publication_evidence_family_count:($publication.publication_evidence_families | length),
      source_publication_evidence_persisted:$publication.publication_evidence_summary_persisted,
      source_publication_evidence_public_claim_allowed:$publication.public_release_claim_allowed,
      source_publication_evidence_distribution_allowed:$publication.public_distribution_publication_allowed,
      source_dependency_isolation_ready:$dependency.local_cargo_tree_isolated,
      source_dependency_isolation_active_binary_package:$dependency.active_binary_package,
      source_dependency_isolation_active_binary_target:$dependency.active_binary_target,
      source_dependency_isolation_forbidden_crate_count:($dependency.found_forbidden_codex_engine_crates | length),
      source_dependency_isolation_live_check_status:$dependency.live_check_status,
      source_memory_intelligence_status:$memory.status,
      source_memory_intelligence_consumed_by_active_stack:$memory.active_service_stack_consumes_memory_intelligence,
      source_memory_intelligence_surface_count:$memory.memory_surface_count,
      source_memory_intelligence_absorbed_or_represented_count:$memory.absorbed_or_represented_count,
      source_memory_intelligence_gap_only_surface_count:$memory.gap_only_surface_count,
      source_memory_intelligence_live_mutation_enabled_count:$memory.live_mutation_enabled_count,
      final_release_governance_audit_ready:true,
      full_fusion_operational_evidence_observed:$publication.source_watchdog_full_fusion_complete,
      active_runtime_evidence_contract_ready:$publication.active_runtime_evidence_contract_ready,
      active_binary_sha_consistent:$publication.active_binary_sha_consistent,
      active_dependency_isolated:true,
      memory_intelligence_absorbed_or_represented:true,
      publication_evidence_non_persistence_enforced:true,
      public_claim_denial_enforced:true,
      public_distribution_denial_enforced:true,
      operator_approval_non_recording_enforced:true,
      active_runtime_mutation_denial_enforced:true,
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
      memory_store_mutation_allowed:false,
      capability_registry_mutation_allowed:false,
      plugin_registry_mutation_allowed:false,
      skill_workshop_write_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      release_artifact_write_allowed:false,
      public_artifact_write_allowed:false,
      public_release_published:false,
      public_ga_claimed:false,
      external_public_claim_performed:false,
      external_public_distribution_performed:false,
      final_audit_index_recorded:false,
      final_audit_index_persisted:false,
      final_audit_index_materialized:false,
      final_audit_index_filesystem_written:false,
      publication_evidence_summary_persisted:false,
      publication_evidence_receipt_persisted:false,
      publication_evidence_ledger_persisted:false,
      final_audit_index_hash_sha256:$final_audit_index_hash_sha256,
      final_audit_policy_hash_sha256:$final_audit_policy_hash_sha256,
      final_audit_side_effect_hash_sha256:$final_audit_side_effect_hash_sha256,
      final_audit_source_hashes:[
        $publication_evidence_report_sha256,
        $dependency_isolation_report_sha256,
        $memory_intelligence_report_sha256
      ],
      final_audit_families:[
        {
          id:"publication-evidence-non-persistence-source",
          ready:true,
          blocked:true,
          denied_by_count:$publication.publication_evidence_denied_by_count,
          reason:"publication evidence source denies persistence, public claims, public distribution, artifact writes, and runtime mutation"
        },
        {
          id:"active-dependency-isolation-source",
          ready:true,
          blocked:true,
          forbidden_codex_engine_crate_count:($dependency.found_forbidden_codex_engine_crates | length),
          local_cargo_tree_isolated:$dependency.local_cargo_tree_isolated,
          reason:"active hepta-cli remains isolated from tracked Codex engine crates"
        },
        {
          id:"memory-intelligence-absorption-source",
          ready:true,
          blocked:true,
          absorbed_or_represented_count:$memory.absorbed_or_represented_count,
          live_mutation_enabled_count:$memory.live_mutation_enabled_count,
          reason:"memory/intelligence surfaces are absorbed or represented, while live mutation stays disabled"
        },
        {
          id:"public-claim-distribution-artifact-boundary",
          ready:true,
          blocked:true,
          public_release_claim_allowed:false,
          public_distribution_publication_allowed:false,
          release_artifact_write_allowed:false,
          reason:"public claims, public distribution, and artifact writes remain denied"
        },
        {
          id:"operator-approval-and-live-mutation-boundary",
          ready:true,
          blocked:true,
          operator_approval_recorded:false,
          live_mutation_execution_ready:false,
          reason:"operator approval is not recorded and live mutation execution remains denied"
        },
        {
          id:"final-audit-persistence-boundary",
          ready:true,
          blocked:true,
          final_audit_index_recorded:false,
          final_audit_index_persisted:false,
          final_audit_index_materialized:false,
          final_audit_index_filesystem_written:false,
          reason:"final audit index is report-only and not persisted or materialized"
        },
        {
          id:"active-runtime-state-boundary",
          ready:true,
          blocked:true,
          install_execution_allowed:false,
          active_service_restart_allowed:false,
          active_runtime_codex_engine_dependency_allowed:false,
          reason:"install, restart, active runtime dependency mutation, and active wiring remain denied"
        }
      ],
      final_audit_denied_by_count:($final_audit_denied | length),
      denied_by_final_audit_index:$final_audit_denied,
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
        final_audit_index_recorded:false,
        final_audit_index_persisted:false,
        final_audit_index_materialized:false,
        final_audit_index_filesystem_written:false,
        publication_evidence_summary_persisted:false,
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
  and .final_audit_index_ready == true
  and .source_publication_evidence_ready == true
  and .source_publication_evidence_denied_by_count == 111
  and .source_dependency_isolation_ready == true
  and .source_dependency_isolation_forbidden_crate_count == 0
  and .source_memory_intelligence_absorbed_or_represented_count == 14
  and .source_memory_intelligence_live_mutation_enabled_count == 0
  and .source_memory_intelligence_gap_only_surface_count == 0
  and .active_runtime_evidence_contract_ready == true
  and .active_dependency_isolated == true
  and .memory_intelligence_absorbed_or_represented == true
  and .publication_evidence_non_persistence_enforced == true
  and .public_claim_denial_enforced == true
  and .public_distribution_denial_enforced == true
  and .operator_approval_non_recording_enforced == true
  and .active_runtime_mutation_denial_enforced == true
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .public_distribution_publication_allowed == false
  and .release_artifact_write_allowed == false
  and .memory_store_mutation_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .live_mutation_execution_ready == false
  and .final_audit_index_recorded == false
  and .final_audit_index_persisted == false
  and .final_audit_index_materialized == false
  and .final_audit_index_filesystem_written == false
  and .final_audit_denied_by_count == 127
  and (.final_audit_families | length) == 7
  and (.final_audit_families | all(.ready == true and .blocked == true))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta terminal release-governance final audit index gate passed"
