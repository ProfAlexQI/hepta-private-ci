#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

KG_PREFLIGHT_JSON="$(
  capture_json_report \
    "hepta-kg-prompt-preview-preflight-gate" \
    scripts/hepta-kg-prompt-preview-preflight-gate.sh
)"

source_preflight_report_sha256="$(sha256_text "$KG_PREFLIGHT_JSON")"
terminal_summary_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-terminal-summary:index:$source_preflight_report_sha256")"
terminal_summary_policy_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-terminal-summary:policy:$source_preflight_report_sha256")"
terminal_summary_side_effect_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-terminal-summary:side-effects:$source_preflight_report_sha256")"

jq -n -e \
  --argjson source "$KG_PREFLIGHT_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_kg_prompt_preview_preflight_gate"
    and $source.mode == "kg_prompt_preview_preflight_report_only_ci_gate_no_execution"
    and $source.preflight_contract == "hepta-intelligence-memory-kg-prompt-preview-preflight-v0"
    and $source.context_handoff_contract == "hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0"
    and $source.preflight_report_status == "blocked"
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
    and $source.source_gates_all_linked == true
    and $source.source_gates_all_checks_ready == true
    and $source.source_gates_all_blocked == true
    and $source.source_gates_all_report_only == true
    and $source.context_handoff_contract_linked == true
    and $source.operator_evidence_incomplete == true
    and $source.safety_controls_incomplete == true
    and $source.handoff_requirements_incomplete == true
    and $source.redacted_diff_review_required == true
    and $source.context_handoff_approval_required == true
    and $source.redacted_refs_only == true
    and $source.raw_prompt_diff_count == 0
    and $source.prompt_text_included_count == 0
    and $source.payload_text_included_count == 0
    and $source.prompt_preview_allowed == false
    and $source.prompt_preview_rendered == false
    and $source.prompt_payload_materialized == false
    and $source.context_injection_allowed == false
    and $source.context_injection_performed == false
    and $source.model_invoked == false
    and $source.external_read_enabled_count == 0
    and $source.network_call_enabled_count == 0
    and $source.live_write_enabled_count == 0
    and $source.ci_promotion_allowed == false
    and $source.ci_promotion_disabled == true
    and $source.preflight_execution_allowed == false
    and $source.preflight_execution_performed == false
    and ($source.source_gates | length) == 5
    and ($source.source_gates | all(.status == "blocked" and .checks_ready == true and .report_only == true and .blocks_prompt_preview == true and .blocks_context_injection == true))
    and ($source.denied_actions | length) == 11
    and ($source.side_effects | to_entries | all(.value == false))
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg gate "hepta_kg_prompt_preview_terminal_summary_gate" \
  --arg source_preflight_report_sha256 "$source_preflight_report_sha256" \
  --arg terminal_summary_hash_sha256 "$terminal_summary_hash_sha256" \
  --arg terminal_summary_policy_hash_sha256 "$terminal_summary_policy_hash_sha256" \
  --arg terminal_summary_side_effect_hash_sha256 "$terminal_summary_side_effect_hash_sha256" \
  --argjson source "$KG_PREFLIGHT_JSON" \
  '
    ([
      "kg_prompt_preview_terminal_summary_persistence_denied",
      "kg_prompt_preview_terminal_summary_filesystem_write_denied",
      "kg_prompt_preview_operator_briefing_persistence_denied",
      "kg_prompt_preview_final_operator_approval_missing",
      "kg_prompt_preview_ci_promotion_denied",
      "kg_prompt_preview_active_runtime_wiring_denied",
      "kg_prompt_preview_gateway_route_migration_denied",
      "kg_prompt_preview_model_invocation_denied",
      "kg_prompt_preview_external_kg_read_denied",
      "kg_prompt_preview_context_injection_denied",
      "kg_prompt_preview_public_release_claim_denied"
    ] + $source.denied_actions) as $denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      gate:$gate,
      terminal_summary_schema_version:"kg_prompt_preview_terminal_summary_v1",
      terminal_summary_mode:"schema_only_operator_readable_summary_not_persisted",
      terminal_summary_decision:"blocked_until_source_preflight_requirements_final_review_context_handoff_approval_and_explicit_operator_approval_exist",
      source_preflight_gate:$source.gate,
      source_preflight_contract:$source.preflight_contract,
      source_context_handoff_contract:$source.context_handoff_contract,
      source_report_command:$source.source_report_command,
      source_runtime_summary:$source.source_runtime_summary,
      source_preflight_report_sha256:$source_preflight_report_sha256,
      terminal_summary_hash_sha256:$terminal_summary_hash_sha256,
      terminal_summary_policy_hash_sha256:$terminal_summary_policy_hash_sha256,
      terminal_summary_side_effect_hash_sha256:$terminal_summary_side_effect_hash_sha256,
      terminal_summary_ready:true,
      operator_readable_summary_ready:true,
      operator_briefing_ready:true,
      operator_briefing_status:"blocked",
      source_gate_count:$source.source_gate_count,
      ready_source_gate_count:$source.ready_source_gate_count,
      blocked_source_gate_count:$source.blocked_source_gate_count,
      report_only_source_gate_count:$source.report_only_source_gate_count,
      required_operator_evidence_count:$source.required_operator_evidence_count,
      missing_operator_evidence_count:$source.missing_operator_evidence_count,
      required_safety_control_count:$source.required_safety_control_count,
      missing_safety_control_count:$source.missing_safety_control_count,
      required_handoff_requirement_count:$source.required_handoff_requirement_count,
      missing_handoff_requirement_count:$source.missing_handoff_requirement_count,
      missing_final_review_approval_count:$source.missing_final_review_approval_count,
      required_total_preflight_requirement_count:$source.required_total_preflight_requirement_count,
      missing_total_preflight_requirement_count:$source.missing_total_preflight_requirement_count,
      source_preflight_report_status:$source.preflight_report_status,
      source_preflight_report_verdict:$source.preflight_report_verdict,
      source_gates_all_linked:$source.source_gates_all_linked,
      source_gates_all_checks_ready:$source.source_gates_all_checks_ready,
      source_gates_all_blocked:$source.source_gates_all_blocked,
      source_gates_all_report_only:$source.source_gates_all_report_only,
      source_context_handoff_contract_linked:$source.context_handoff_contract_linked,
      source_context_handoff_checks_ready:$source.context_handoff_checks_ready,
      source_context_handoff_blocked:$source.context_handoff_blocked,
      source_redacted_refs_only:$source.redacted_refs_only,
      source_raw_prompt_diff_count:$source.raw_prompt_diff_count,
      source_prompt_text_included_count:$source.prompt_text_included_count,
      source_payload_text_included_count:$source.payload_text_included_count,
      operator_evidence_incomplete:$source.operator_evidence_incomplete,
      safety_controls_incomplete:$source.safety_controls_incomplete,
      handoff_requirements_incomplete:$source.handoff_requirements_incomplete,
      redacted_diff_review_required:$source.redacted_diff_review_required,
      context_handoff_approval_required:$source.context_handoff_approval_required,
      final_operator_approval_recorded:false,
      final_operator_approval_required:true,
      terminal_summary_persistence_allowed:false,
      terminal_summary_persisted:false,
      terminal_summary_filesystem_write_allowed:false,
      terminal_summary_filesystem_written:false,
      operator_briefing_persistence_allowed:false,
      operator_briefing_persisted:false,
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
      ci_promotion_allowed:false,
      ci_promotion_disabled:true,
      preflight_execution_allowed:false,
      preflight_execution_performed:false,
      gateway_route_migration_allowed:false,
      source_command_migration_allowed:false,
      active_runtime_wiring_allowed:false,
      install_execution_allowed:false,
      service_restart_allowed:false,
      active_binary_mutation_allowed:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      denied_actions:$denied,
      denied_action_count:($denied | length),
      source_gates:$source.source_gates,
      side_effects:{
        terminal_summary_persisted:false,
        terminal_summary_filesystem_written:false,
        operator_briefing_persisted:false,
        final_operator_approval_recorded:false,
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
  and .gate == "hepta_kg_prompt_preview_terminal_summary_gate"
  and .terminal_summary_schema_version == "kg_prompt_preview_terminal_summary_v1"
  and .terminal_summary_mode == "schema_only_operator_readable_summary_not_persisted"
  and .terminal_summary_ready == true
  and .operator_readable_summary_ready == true
  and .operator_briefing_ready == true
  and .operator_briefing_status == "blocked"
  and .source_preflight_gate == "hepta_kg_prompt_preview_preflight_gate"
  and .source_preflight_contract == "hepta-intelligence-memory-kg-prompt-preview-preflight-v0"
  and .source_context_handoff_contract == "hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0"
  and .source_preflight_report_status == "blocked"
  and .source_gate_count == 5
  and .ready_source_gate_count == 5
  and .blocked_source_gate_count == 5
  and .report_only_source_gate_count == 5
  and .required_operator_evidence_count == 7
  and .missing_operator_evidence_count == 7
  and .required_safety_control_count == 4
  and .missing_safety_control_count == 4
  and .required_handoff_requirement_count == 6
  and .missing_handoff_requirement_count == 6
  and .missing_final_review_approval_count == 2
  and .required_total_preflight_requirement_count == 19
  and .missing_total_preflight_requirement_count == 19
  and .source_gates_all_linked == true
  and .source_gates_all_checks_ready == true
  and .source_gates_all_blocked == true
  and .source_gates_all_report_only == true
  and .source_context_handoff_contract_linked == true
  and .source_context_handoff_checks_ready == true
  and .source_context_handoff_blocked == true
  and .source_redacted_refs_only == true
  and .source_raw_prompt_diff_count == 0
  and .source_prompt_text_included_count == 0
  and .source_payload_text_included_count == 0
  and .operator_evidence_incomplete == true
  and .safety_controls_incomplete == true
  and .handoff_requirements_incomplete == true
  and .redacted_diff_review_required == true
  and .context_handoff_approval_required == true
  and .final_operator_approval_recorded == false
  and .final_operator_approval_required == true
  and .terminal_summary_persistence_allowed == false
  and .terminal_summary_persisted == false
  and .terminal_summary_filesystem_write_allowed == false
  and .terminal_summary_filesystem_written == false
  and .operator_briefing_persistence_allowed == false
  and .operator_briefing_persisted == false
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
  and .ci_promotion_allowed == false
  and .ci_promotion_disabled == true
  and .preflight_execution_allowed == false
  and .preflight_execution_performed == false
  and .gateway_route_migration_allowed == false
  and .source_command_migration_allowed == false
  and .active_runtime_wiring_allowed == false
  and .install_execution_allowed == false
  and .service_restart_allowed == false
  and .active_binary_mutation_allowed == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .denied_action_count == 22
  and (.denied_actions | length) == 22
  and (.source_gates | length) == 5
  and (.source_gates | all(.status == "blocked" and .checks_ready == true and .report_only == true and .blocks_prompt_preview == true and .blocks_context_injection == true))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta KG prompt-preview terminal summary gate passed"
