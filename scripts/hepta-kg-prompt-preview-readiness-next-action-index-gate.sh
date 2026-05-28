#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

KG_OPERATOR_BRIEFING_JSON="$(
  capture_json_report \
    "hepta-kg-prompt-preview-operator-briefing-non-persistence-gate" \
    scripts/hepta-kg-prompt-preview-operator-briefing-non-persistence-gate.sh
)"

source_operator_briefing_report_sha256="$(sha256_text "$KG_OPERATOR_BRIEFING_JSON")"
readiness_index_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-readiness-next-action-index:index:$source_operator_briefing_report_sha256")"
readiness_index_policy_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-readiness-next-action-index:policy:$source_operator_briefing_report_sha256")"
readiness_index_side_effect_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-readiness-next-action-index:side-effects:$source_operator_briefing_report_sha256")"

jq -n -e \
  --argjson source "$KG_OPERATOR_BRIEFING_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_kg_prompt_preview_operator_briefing_non_persistence_gate"
    and $source.operator_briefing_schema_version == "kg_prompt_preview_operator_briefing_non_persistence_v1"
    and $source.operator_briefing_mode == "schema_only_operator_briefing_not_persisted_not_delivered_not_approved"
    and $source.operator_briefing_ready == true
    and $source.operator_briefing_status == "blocked"
    and $source.source_terminal_summary_gate == "hepta_kg_prompt_preview_terminal_summary_gate"
    and $source.source_terminal_summary_schema_version == "kg_prompt_preview_terminal_summary_v1"
    and $source.source_preflight_gate == "hepta_kg_prompt_preview_preflight_gate"
    and $source.source_preflight_contract == "hepta-intelligence-memory-kg-prompt-preview-preflight-v0"
    and $source.source_context_handoff_contract == "hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0"
    and $source.operator_briefing_section_count == 5
    and $source.operator_briefing_sections_all_redacted == true
    and $source.operator_briefing_sections_all_blocked == true
    and $source.operator_briefing_sections_all_not_persisted == true
    and $source.source_gate_count == 5
    and $source.ready_source_gate_count == 5
    and $source.blocked_source_gate_count == 5
    and $source.report_only_source_gate_count == 5
    and $source.required_operator_evidence_count == 7
    and $source.missing_operator_evidence_count == 7
    and $source.required_safety_control_count == 4
    and $source.missing_safety_control_count == 4
    and $source.required_handoff_requirement_count == 6
    and $source.missing_handoff_requirement_count == 6
    and $source.missing_final_review_approval_count == 2
    and $source.required_total_preflight_requirement_count == 19
    and $source.missing_total_preflight_requirement_count == 19
    and $source.source_redacted_refs_only == true
    and $source.source_raw_prompt_diff_count == 0
    and $source.source_prompt_text_included_count == 0
    and $source.source_payload_text_included_count == 0
    and $source.final_operator_approval_recorded == false
    and $source.final_operator_approval_required == true
    and $source.operator_identity_accepted == false
    and $source.operator_scope_accepted == false
    and $source.operator_activation_plan_accepted == false
    and $source.operator_briefing_persistence_allowed == false
    and $source.operator_briefing_persisted == false
    and $source.operator_briefing_filesystem_write_allowed == false
    and $source.operator_briefing_filesystem_written == false
    and $source.operator_briefing_delivery_allowed == false
    and $source.operator_briefing_delivered == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.prompt_preview_allowed == false
    and $source.prompt_preview_rendered == false
    and $source.prompt_payload_materialized == false
    and $source.context_injection_allowed == false
    and $source.context_injection_performed == false
    and $source.model_invocation_allowed == false
    and $source.model_invoked == false
    and $source.external_kg_adapter_read_allowed == false
    and $source.external_kg_adapter_read_performed == false
    and $source.network_call_allowed == false
    and $source.network_call_performed == false
    and $source.live_kg_write_allowed == false
    and $source.live_kg_write_performed == false
    and $source.ci_promotion_allowed == false
    and $source.ci_promotion_disabled == true
    and $source.preflight_execution_allowed == false
    and $source.preflight_execution_performed == false
    and $source.gateway_route_migration_allowed == false
    and $source.source_command_migration_allowed == false
    and $source.active_runtime_wiring_allowed == false
    and $source.install_execution_allowed == false
    and $source.service_restart_allowed == false
    and $source.active_binary_mutation_allowed == false
    and $source.public_release_claim_allowed == false
    and $source.public_ga_claim_allowed == false
    and $source.denied_action_count == 32
    and ($source.denied_actions | length) == 32
    and ($source.operator_briefing_sections | length) == 5
    and ($source.operator_briefing_sections | all(.status == "blocked" and .redacted == true and .persisted == false))
    and ($source.source_gates | length) == 5
    and ($source.source_gates | all(.status == "blocked" and .checks_ready == true and .report_only == true and .blocks_prompt_preview == true and .blocks_context_injection == true))
    and ($source.side_effects | to_entries | all(.value == false))
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg gate "hepta_kg_prompt_preview_readiness_next_action_index_gate" \
  --arg source_operator_briefing_report_sha256 "$source_operator_briefing_report_sha256" \
  --arg readiness_index_hash_sha256 "$readiness_index_hash_sha256" \
  --arg readiness_index_policy_hash_sha256 "$readiness_index_policy_hash_sha256" \
  --arg readiness_index_side_effect_hash_sha256 "$readiness_index_side_effect_hash_sha256" \
  --argjson source "$KG_OPERATOR_BRIEFING_JSON" \
  '
    [
      {
        action:"maintain_report_only_evidence_index",
        status:"allowed_report_only",
        mutates_runtime:false,
        permits_prompt_preview:false
      },
      {
        action:"add_operator_approval_checklist_schema",
        status:"allowed_report_only",
        mutates_runtime:false,
        permits_prompt_preview:false
      },
      {
        action:"add_rollback_kill_switch_evidence_checklist",
        status:"allowed_report_only",
        mutates_runtime:false,
        permits_prompt_preview:false
      },
      {
        action:"add_redacted_diff_review_checklist",
        status:"allowed_report_only",
        mutates_runtime:false,
        permits_prompt_preview:false
      },
      {
        action:"add_context_handoff_checklist",
        status:"allowed_report_only",
        mutates_runtime:false,
        permits_prompt_preview:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        mutates_runtime:false,
        permits_prompt_preview:false
      }
    ] as $allowed_next_actions
    | [
      "prompt_preview_execution",
      "prompt_payload_materialization",
      "context_injection",
      "model_invocation",
      "external_kg_adapter_read",
      "live_kg_write",
      "gateway_route_migration",
      "source_command_migration",
      "ci_promotion",
      "active_runtime_wiring",
      "install_restart",
      "public_release_claim"
    ] as $denied_next_actions
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      gate:$gate,
      readiness_next_action_index_schema_version:"kg_prompt_preview_readiness_next_action_index_v1",
      readiness_next_action_index_mode:"stdout_only_report_only_index_no_prompt_render_no_context_injection_no_runtime_mutation",
      readiness_next_action_index_ready:true,
      readiness_next_action_index_status:"blocked",
      readiness_next_action_decision:"blocked_until_operator_approval_evidence_safety_handoff_review_and_activation_plan_exist",
      source_operator_briefing_gate:$source.gate,
      source_operator_briefing_schema_version:$source.operator_briefing_schema_version,
      source_operator_briefing_decision:$source.operator_briefing_decision,
      source_terminal_summary_gate:$source.source_terminal_summary_gate,
      source_preflight_gate:$source.source_preflight_gate,
      source_preflight_contract:$source.source_preflight_contract,
      source_context_handoff_contract:$source.source_context_handoff_contract,
      source_operator_briefing_report_sha256:$source_operator_briefing_report_sha256,
      readiness_index_hash_sha256:$readiness_index_hash_sha256,
      readiness_index_policy_hash_sha256:$readiness_index_policy_hash_sha256,
      readiness_index_side_effect_hash_sha256:$readiness_index_side_effect_hash_sha256,
      source_gate_count:$source.source_gate_count,
      ready_source_gate_count:$source.ready_source_gate_count,
      blocked_source_gate_count:$source.blocked_source_gate_count,
      report_only_source_gate_count:$source.report_only_source_gate_count,
      source_operator_briefing_section_count:$source.operator_briefing_section_count,
      source_operator_briefing_sections_all_redacted:$source.operator_briefing_sections_all_redacted,
      source_operator_briefing_sections_all_blocked:$source.operator_briefing_sections_all_blocked,
      source_operator_briefing_sections_all_not_persisted:$source.operator_briefing_sections_all_not_persisted,
      required_operator_evidence_count:$source.required_operator_evidence_count,
      missing_operator_evidence_count:$source.missing_operator_evidence_count,
      required_safety_control_count:$source.required_safety_control_count,
      missing_safety_control_count:$source.missing_safety_control_count,
      required_handoff_requirement_count:$source.required_handoff_requirement_count,
      missing_handoff_requirement_count:$source.missing_handoff_requirement_count,
      missing_final_review_approval_count:$source.missing_final_review_approval_count,
      required_total_preflight_requirement_count:$source.required_total_preflight_requirement_count,
      missing_total_preflight_requirement_count:$source.missing_total_preflight_requirement_count,
      source_redacted_refs_only:$source.source_redacted_refs_only,
      source_raw_prompt_diff_count:$source.source_raw_prompt_diff_count,
      source_prompt_text_included_count:$source.source_prompt_text_included_count,
      source_payload_text_included_count:$source.source_payload_text_included_count,
      final_operator_approval_recorded:false,
      final_operator_approval_required:true,
      operator_identity_accepted:false,
      operator_scope_accepted:false,
      operator_activation_plan_accepted:false,
      readiness_index_persistence_allowed:false,
      readiness_index_persisted:false,
      readiness_index_delivery_allowed:false,
      readiness_index_delivered:false,
      allowed_next_actions:$allowed_next_actions,
      allowed_next_action_count:($allowed_next_actions | length),
      denied_next_actions:$denied_next_actions,
      denied_next_action_count:($denied_next_actions | length),
      denied_actions:$source.denied_actions,
      denied_action_count:$source.denied_action_count,
      prompt_preview_allowed:false,
      prompt_preview_rendered:false,
      prompt_payload_materialized:false,
      context_injection_allowed:false,
      context_injection_performed:false,
      model_invocation_allowed:false,
      model_invoked:false,
      external_kg_adapter_read_allowed:false,
      external_kg_adapter_read_performed:false,
      network_call_allowed:false,
      network_call_performed:false,
      live_kg_write_allowed:false,
      live_kg_write_performed:false,
      operator_briefing_persistence_allowed:false,
      operator_briefing_persisted:false,
      operator_briefing_delivery_allowed:false,
      operator_briefing_delivered:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      ci_promotion_allowed:false,
      ci_promotion_disabled:true,
      preflight_execution_allowed:false,
      preflight_execution_performed:false,
      full_light_preflight_rerun_allowed:true,
      gateway_route_migration_allowed:false,
      source_command_migration_allowed:false,
      active_runtime_wiring_allowed:false,
      install_execution_allowed:false,
      service_restart_allowed:false,
      active_binary_mutation_allowed:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      source_gates:$source.source_gates,
      side_effects:{
        readiness_index_persisted:false,
        readiness_index_delivered:false,
        operator_briefing_persisted:false,
        operator_briefing_filesystem_written:false,
        operator_briefing_delivered:false,
        final_operator_approval_recorded:false,
        operator_identity_accepted:false,
        operator_scope_accepted:false,
        operator_activation_plan_accepted:false,
        prompt_preview_rendered:false,
        prompt_payload_materialized:false,
        context_injection_performed:false,
        model_invoked:false,
        external_kg_adapter_read_performed:false,
        graphiti_client_constructed:false,
        neo4j_client_constructed:false,
        cocoindex_client_constructed:false,
        network_call_performed:false,
        external_db_write_performed:false,
        live_kg_write_performed:false,
        native_gateway_route_added:false,
        source_command_migration_performed:false,
        active_runtime_wired:false,
        ci_promotion_performed:false,
        preflight_execution_performed:false,
        telegram_send_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        install_performed:false,
        launchd_restart_performed:false,
        active_binary_mutated:false,
        credential_read_performed:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_kg_prompt_preview_readiness_next_action_index_gate"
  and .readiness_next_action_index_schema_version == "kg_prompt_preview_readiness_next_action_index_v1"
  and .readiness_next_action_index_mode == "stdout_only_report_only_index_no_prompt_render_no_context_injection_no_runtime_mutation"
  and .readiness_next_action_index_ready == true
  and .readiness_next_action_index_status == "blocked"
  and .readiness_next_action_decision == "blocked_until_operator_approval_evidence_safety_handoff_review_and_activation_plan_exist"
  and .source_operator_briefing_gate == "hepta_kg_prompt_preview_operator_briefing_non_persistence_gate"
  and .source_operator_briefing_schema_version == "kg_prompt_preview_operator_briefing_non_persistence_v1"
  and .source_terminal_summary_gate == "hepta_kg_prompt_preview_terminal_summary_gate"
  and .source_preflight_gate == "hepta_kg_prompt_preview_preflight_gate"
  and .source_preflight_contract == "hepta-intelligence-memory-kg-prompt-preview-preflight-v0"
  and .source_context_handoff_contract == "hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0"
  and .source_gate_count == 5
  and .ready_source_gate_count == 5
  and .blocked_source_gate_count == 5
  and .report_only_source_gate_count == 5
  and .source_operator_briefing_section_count == 5
  and .source_operator_briefing_sections_all_redacted == true
  and .source_operator_briefing_sections_all_blocked == true
  and .source_operator_briefing_sections_all_not_persisted == true
  and .required_total_preflight_requirement_count == 19
  and .missing_total_preflight_requirement_count == 19
  and .source_redacted_refs_only == true
  and .source_raw_prompt_diff_count == 0
  and .source_prompt_text_included_count == 0
  and .source_payload_text_included_count == 0
  and .final_operator_approval_recorded == false
  and .final_operator_approval_required == true
  and .operator_identity_accepted == false
  and .operator_scope_accepted == false
  and .operator_activation_plan_accepted == false
  and .readiness_index_persistence_allowed == false
  and .readiness_index_persisted == false
  and .readiness_index_delivery_allowed == false
  and .readiness_index_delivered == false
  and .allowed_next_action_count == 6
  and (.allowed_next_actions | length) == 6
  and (.allowed_next_actions | all(.mutates_runtime == false and .permits_prompt_preview == false))
  and .denied_next_action_count == 12
  and (.denied_next_actions | length) == 12
  and .denied_action_count == 32
  and (.denied_actions | length) == 32
  and .prompt_preview_allowed == false
  and .prompt_preview_rendered == false
  and .prompt_payload_materialized == false
  and .context_injection_allowed == false
  and .context_injection_performed == false
  and .model_invocation_allowed == false
  and .model_invoked == false
  and .external_kg_adapter_read_allowed == false
  and .external_kg_adapter_read_performed == false
  and .network_call_allowed == false
  and .network_call_performed == false
  and .live_kg_write_allowed == false
  and .live_kg_write_performed == false
  and .operator_briefing_persistence_allowed == false
  and .operator_briefing_persisted == false
  and .operator_briefing_delivery_allowed == false
  and .operator_briefing_delivered == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .ci_promotion_allowed == false
  and .ci_promotion_disabled == true
  and .preflight_execution_allowed == false
  and .preflight_execution_performed == false
  and .full_light_preflight_rerun_allowed == true
  and .gateway_route_migration_allowed == false
  and .source_command_migration_allowed == false
  and .active_runtime_wiring_allowed == false
  and .install_execution_allowed == false
  and .service_restart_allowed == false
  and .active_binary_mutation_allowed == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and (.source_gates | length) == 5
  and (.source_gates | all(.status == "blocked" and .checks_ready == true and .report_only == true and .blocks_prompt_preview == true and .blocks_context_injection == true))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta KG prompt-preview readiness next-action index gate passed"
