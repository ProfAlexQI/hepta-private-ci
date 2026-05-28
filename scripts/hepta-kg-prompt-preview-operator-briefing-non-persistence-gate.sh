#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

KG_TERMINAL_SUMMARY_JSON="$(
  capture_json_report \
    "hepta-kg-prompt-preview-terminal-summary-gate" \
    scripts/hepta-kg-prompt-preview-terminal-summary-gate.sh
)"

source_terminal_summary_report_sha256="$(sha256_text "$KG_TERMINAL_SUMMARY_JSON")"
operator_briefing_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-operator-briefing-non-persistence:index:$source_terminal_summary_report_sha256")"
operator_briefing_policy_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-operator-briefing-non-persistence:policy:$source_terminal_summary_report_sha256")"
operator_briefing_side_effect_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-operator-briefing-non-persistence:side-effects:$source_terminal_summary_report_sha256")"

jq -n -e \
  --argjson source "$KG_TERMINAL_SUMMARY_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_kg_prompt_preview_terminal_summary_gate"
    and $source.terminal_summary_schema_version == "kg_prompt_preview_terminal_summary_v1"
    and $source.terminal_summary_mode == "schema_only_operator_readable_summary_not_persisted"
    and $source.terminal_summary_ready == true
    and $source.operator_readable_summary_ready == true
    and $source.operator_briefing_ready == true
    and $source.operator_briefing_status == "blocked"
    and $source.source_preflight_gate == "hepta_kg_prompt_preview_preflight_gate"
    and $source.source_preflight_contract == "hepta-intelligence-memory-kg-prompt-preview-preflight-v0"
    and $source.source_context_handoff_contract == "hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0"
    and $source.source_preflight_report_status == "blocked"
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
    and $source.final_operator_approval_recorded == false
    and $source.final_operator_approval_required == true
    and $source.terminal_summary_persistence_allowed == false
    and $source.terminal_summary_persisted == false
    and $source.operator_briefing_persistence_allowed == false
    and $source.operator_briefing_persisted == false
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
    and $source.active_runtime_wiring_allowed == false
    and $source.install_execution_allowed == false
    and $source.service_restart_allowed == false
    and $source.active_binary_mutation_allowed == false
    and $source.public_release_claim_allowed == false
    and $source.public_ga_claim_allowed == false
    and $source.denied_action_count == 22
    and ($source.denied_actions | length) == 22
    and ($source.source_gates | length) == 5
    and ($source.source_gates | all(.status == "blocked" and .checks_ready == true and .report_only == true and .blocks_prompt_preview == true and .blocks_context_injection == true))
    and ($source.side_effects | to_entries | all(.value == false))
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg gate "hepta_kg_prompt_preview_operator_briefing_non_persistence_gate" \
  --arg source_terminal_summary_report_sha256 "$source_terminal_summary_report_sha256" \
  --arg operator_briefing_hash_sha256 "$operator_briefing_hash_sha256" \
  --arg operator_briefing_policy_hash_sha256 "$operator_briefing_policy_hash_sha256" \
  --arg operator_briefing_side_effect_hash_sha256 "$operator_briefing_side_effect_hash_sha256" \
  --argjson source "$KG_TERMINAL_SUMMARY_JSON" \
  '
    [
      {
        section:"source_gate_status",
        status:"blocked",
        redacted:true,
        persisted:false,
        source_gate_count:$source.source_gate_count,
        ready_source_gate_count:$source.ready_source_gate_count,
        blocked_source_gate_count:$source.blocked_source_gate_count,
        report_only_source_gate_count:$source.report_only_source_gate_count
      },
      {
        section:"missing_requirements",
        status:"blocked",
        redacted:true,
        persisted:false,
        missing_operator_evidence_count:$source.missing_operator_evidence_count,
        missing_safety_control_count:$source.missing_safety_control_count,
        missing_handoff_requirement_count:$source.missing_handoff_requirement_count,
        missing_final_review_approval_count:$source.missing_final_review_approval_count,
        missing_total_preflight_requirement_count:$source.missing_total_preflight_requirement_count
      },
      {
        section:"approval_state",
        status:"blocked",
        redacted:true,
        persisted:false,
        final_operator_approval_recorded:$source.final_operator_approval_recorded,
        final_operator_approval_required:$source.final_operator_approval_required,
        ci_promotion_allowed:$source.ci_promotion_allowed,
        active_runtime_wiring_allowed:$source.active_runtime_wiring_allowed
      },
      {
        section:"execution_boundary",
        status:"blocked",
        redacted:true,
        persisted:false,
        prompt_preview_allowed:$source.prompt_preview_allowed,
        context_injection_allowed:$source.context_injection_allowed,
        model_invocation_allowed:$source.model_invocation_allowed,
        external_kg_adapter_read_allowed:$source.external_kg_adapter_read_allowed,
        live_kg_write_allowed:$source.live_kg_write_allowed
      },
      {
        section:"publication_boundary",
        status:"blocked",
        redacted:true,
        persisted:false,
        public_release_claim_allowed:$source.public_release_claim_allowed,
        public_ga_claim_allowed:$source.public_ga_claim_allowed,
        install_execution_allowed:$source.install_execution_allowed,
        service_restart_allowed:$source.service_restart_allowed,
        active_binary_mutation_allowed:$source.active_binary_mutation_allowed
      }
    ] as $sections
    | ([
      "kg_prompt_preview_operator_briefing_artifact_persistence_denied",
      "kg_prompt_preview_operator_briefing_filesystem_write_denied",
      "kg_prompt_preview_operator_briefing_delivery_denied",
      "kg_prompt_preview_operator_approval_recording_denied",
      "kg_prompt_preview_operator_approval_acceptance_denied",
      "kg_prompt_preview_briefing_prompt_preview_execution_denied",
      "kg_prompt_preview_briefing_context_injection_denied",
      "kg_prompt_preview_briefing_model_invocation_denied",
      "kg_prompt_preview_briefing_public_claim_denied",
      "kg_prompt_preview_briefing_active_runtime_mutation_denied"
    ] + $source.denied_actions) as $denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      gate:$gate,
      operator_briefing_schema_version:"kg_prompt_preview_operator_briefing_non_persistence_v1",
      operator_briefing_mode:"schema_only_operator_briefing_not_persisted_not_delivered_not_approved",
      operator_briefing_decision:"blocked_until_terminal_summary_is_approved_with_explicit_operator_identity_scope_evidence_and_activation_plan",
      source_terminal_summary_gate:$source.gate,
      source_terminal_summary_schema_version:$source.terminal_summary_schema_version,
      source_terminal_summary_decision:$source.terminal_summary_decision,
      source_preflight_gate:$source.source_preflight_gate,
      source_preflight_contract:$source.source_preflight_contract,
      source_context_handoff_contract:$source.source_context_handoff_contract,
      source_terminal_summary_report_sha256:$source_terminal_summary_report_sha256,
      operator_briefing_hash_sha256:$operator_briefing_hash_sha256,
      operator_briefing_policy_hash_sha256:$operator_briefing_policy_hash_sha256,
      operator_briefing_side_effect_hash_sha256:$operator_briefing_side_effect_hash_sha256,
      operator_briefing_ready:true,
      operator_briefing_status:"blocked",
      operator_briefing_section_count:($sections | length),
      operator_briefing_sections:$sections,
      operator_briefing_sections_all_redacted:($sections | all(.redacted == true)),
      operator_briefing_sections_all_blocked:($sections | all(.status == "blocked")),
      operator_briefing_sections_all_not_persisted:($sections | all(.persisted == false)),
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
      source_redacted_refs_only:$source.source_redacted_refs_only,
      source_raw_prompt_diff_count:$source.source_raw_prompt_diff_count,
      source_prompt_text_included_count:$source.source_prompt_text_included_count,
      source_payload_text_included_count:$source.source_payload_text_included_count,
      final_operator_approval_recorded:false,
      final_operator_approval_required:true,
      operator_identity_accepted:false,
      operator_scope_accepted:false,
      operator_activation_plan_accepted:false,
      operator_briefing_persistence_allowed:false,
      operator_briefing_persisted:false,
      operator_briefing_filesystem_write_allowed:false,
      operator_briefing_filesystem_written:false,
      operator_briefing_delivery_allowed:false,
      operator_briefing_delivered:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
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
  and .gate == "hepta_kg_prompt_preview_operator_briefing_non_persistence_gate"
  and .operator_briefing_schema_version == "kg_prompt_preview_operator_briefing_non_persistence_v1"
  and .operator_briefing_mode == "schema_only_operator_briefing_not_persisted_not_delivered_not_approved"
  and .operator_briefing_ready == true
  and .operator_briefing_status == "blocked"
  and .source_terminal_summary_gate == "hepta_kg_prompt_preview_terminal_summary_gate"
  and .source_terminal_summary_schema_version == "kg_prompt_preview_terminal_summary_v1"
  and .source_preflight_gate == "hepta_kg_prompt_preview_preflight_gate"
  and .source_preflight_contract == "hepta-intelligence-memory-kg-prompt-preview-preflight-v0"
  and .source_context_handoff_contract == "hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0"
  and .operator_briefing_section_count == 5
  and .operator_briefing_sections_all_redacted == true
  and .operator_briefing_sections_all_blocked == true
  and .operator_briefing_sections_all_not_persisted == true
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
  and .source_redacted_refs_only == true
  and .source_raw_prompt_diff_count == 0
  and .source_prompt_text_included_count == 0
  and .source_payload_text_included_count == 0
  and .final_operator_approval_recorded == false
  and .final_operator_approval_required == true
  and .operator_identity_accepted == false
  and .operator_scope_accepted == false
  and .operator_activation_plan_accepted == false
  and .operator_briefing_persistence_allowed == false
  and .operator_briefing_persisted == false
  and .operator_briefing_filesystem_write_allowed == false
  and .operator_briefing_filesystem_written == false
  and .operator_briefing_delivery_allowed == false
  and .operator_briefing_delivered == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
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
  and .denied_action_count == 32
  and (.denied_actions | length) == 32
  and (.operator_briefing_sections | length) == 5
  and (.operator_briefing_sections | all(.status == "blocked" and .redacted == true and .persisted == false))
  and (.source_gates | length) == 5
  and (.source_gates | all(.status == "blocked" and .checks_ready == true and .report_only == true and .blocks_prompt_preview == true and .blocks_context_injection == true))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta KG prompt-preview operator briefing non-persistence gate passed"
