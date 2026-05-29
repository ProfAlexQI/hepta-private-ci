#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

KG_CONTEXT_HANDOFF_CHECKLIST_JSON="$(
  capture_json_report \
    "hepta-kg-prompt-preview-context-handoff-checklist-gate" \
    scripts/hepta-kg-prompt-preview-context-handoff-checklist-gate.sh
)"

source_context_handoff_checklist_report_sha256="$(sha256_text "$KG_CONTEXT_HANDOFF_CHECKLIST_JSON")"
terminal_summary_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-terminal-next-action-activation-denial-summary:summary:$source_context_handoff_checklist_report_sha256")"
terminal_policy_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-terminal-next-action-activation-denial-summary:policy:$source_context_handoff_checklist_report_sha256")"
terminal_side_effect_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-terminal-next-action-activation-denial-summary:side-effects:$source_context_handoff_checklist_report_sha256")"

jq -n -e \
  --argjson source "$KG_CONTEXT_HANDOFF_CHECKLIST_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_kg_prompt_preview_context_handoff_checklist_gate"
    and $source.context_handoff_checklist_schema_version == "kg_prompt_preview_context_handoff_checklist_v1"
    and $source.context_handoff_checklist_ready == true
    and $source.context_handoff_checklist_status == "blocked"
    and ($source.allowed_next_actions | any(.action == "maintain_report_only_evidence_index" and .status == "allowed_report_only" and .mutates_runtime == false and .permits_prompt_preview == false))
    and ($source.allowed_next_actions | any(.action == "run_full_light_preflight" and .status == "allowed_verification_only" and .mutates_runtime == false and .permits_prompt_preview == false))
    and $source.source_redacted_diff_review_checklist_gate == "hepta_kg_prompt_preview_redacted_diff_review_checklist_gate"
    and $source.source_rollback_kill_switch_checklist_gate == "hepta_kg_prompt_preview_rollback_kill_switch_evidence_checklist_gate"
    and $source.source_operator_approval_checklist_gate == "hepta_kg_prompt_preview_operator_approval_checklist_schema_gate"
    and $source.source_readiness_index_gate == "hepta_kg_prompt_preview_readiness_next_action_index_gate"
    and $source.source_operator_briefing_gate == "hepta_kg_prompt_preview_operator_briefing_non_persistence_gate"
    and $source.source_terminal_summary_gate == "hepta_kg_prompt_preview_terminal_summary_gate"
    and $source.source_preflight_gate == "hepta_kg_prompt_preview_preflight_gate"
    and $source.source_preflight_contract == "hepta-intelligence-memory-kg-prompt-preview-preflight-v0"
    and $source.source_context_handoff_contract == "hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0"
    and $source.source_gate_count == 5
    and $source.ready_source_gate_count == 5
    and $source.blocked_source_gate_count == 5
    and $source.report_only_source_gate_count == 5
    and $source.source_operator_briefing_section_count == 5
    and $source.source_operator_briefing_sections_all_redacted == true
    and $source.source_operator_briefing_sections_all_blocked == true
    and $source.source_operator_briefing_sections_all_not_persisted == true
    and $source.required_operator_evidence_count == 7
    and $source.missing_operator_evidence_count == 7
    and $source.required_safety_control_count == 4
    and $source.missing_safety_control_count == 4
    and $source.required_handoff_requirement_count == 6
    and $source.missing_handoff_requirement_count == 6
    and $source.missing_final_review_approval_count == 2
    and $source.required_total_preflight_requirement_count == 19
    and $source.missing_total_preflight_requirement_count == 19
    and $source.handoff_checklist_item_count == 6
    and $source.missing_handoff_checklist_item_count == 6
    and $source.handoff_checklist_items_all_required == true
    and $source.handoff_checklist_items_all_missing == true
    and $source.handoff_checklist_items_all_redacted == true
    and $source.handoff_checklist_items_all_block_prompt_preview == true
    and $source.handoff_checklist_items_all_block_context_injection == true
    and $source.handoff_checklist_items_all_not_persisted == true
    and $source.redacted_refs_only == true
    and $source.raw_prompt_diff_count == 0
    and $source.prompt_text_included_count == 0
    and $source.payload_text_included_count == 0
    and $source.context_handoff_checklist_persisted == false
    and $source.context_handoff_checklist_delivered == false
    and $source.redacted_diff_review_accepted == false
    and $source.rollback_plan_accepted == false
    and $source.kill_switch_accepted == false
    and $source.final_operator_approval_recorded == false
    and $source.final_operator_approval_required == true
    and $source.operator_approval_recorded == false
    and $source.operator_approval_accepted == false
    and $source.operator_identity_accepted == false
    and $source.operator_scope_accepted == false
    and $source.operator_activation_plan_accepted == false
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
    and $source.preflight_execution_allowed == false
    and $source.preflight_execution_performed == false
    and $source.full_light_preflight_rerun_allowed == true
    and $source.gateway_route_migration_allowed == false
    and $source.source_command_migration_allowed == false
    and $source.active_runtime_wiring_allowed == false
    and $source.install_execution_allowed == false
    and $source.service_restart_allowed == false
    and $source.active_binary_mutation_allowed == false
    and $source.public_release_claim_allowed == false
    and $source.public_ga_claim_allowed == false
    and ($source.side_effects | to_entries | all(.value == false))
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg gate "hepta_kg_prompt_preview_terminal_next_action_activation_denial_summary_gate" \
  --arg source_context_handoff_checklist_report_sha256 "$source_context_handoff_checklist_report_sha256" \
  --arg terminal_summary_hash_sha256 "$terminal_summary_hash_sha256" \
  --arg terminal_policy_hash_sha256 "$terminal_policy_hash_sha256" \
  --arg terminal_side_effect_hash_sha256 "$terminal_side_effect_hash_sha256" \
  --argjson source "$KG_CONTEXT_HANDOFF_CHECKLIST_JSON" \
  '
    [
      {
        action:"inspect_terminal_activation_denial_summary",
        status:"allowed_report_only",
        mutates_runtime:false,
        permits_prompt_preview:false,
        permits_context_injection:false
      },
      {
        action:"maintain_report_only_evidence_index",
        status:"allowed_report_only",
        mutates_runtime:false,
        permits_prompt_preview:false,
        permits_context_injection:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        mutates_runtime:false,
        permits_prompt_preview:false,
        permits_context_injection:false
      }
    ] as $allowed_terminal_actions
    | [
      "prompt_preview_execution",
      "prompt_payload_materialization",
      "raw_prompt_diff_exposure",
      "prompt_text_exposure",
      "payload_text_exposure",
      "context_injection",
      "context_handoff_acceptance",
      "context_handoff_checklist_persistence",
      "context_handoff_checklist_delivery",
      "context_injection_scope_acceptance",
      "operator_evidence_packet_acceptance",
      "operator_approval_recording",
      "operator_approval_acceptance",
      "final_operator_approval_recording",
      "operator_identity_acceptance",
      "operator_scope_acceptance",
      "operator_activation_plan_acceptance",
      "approval_digest_acceptance",
      "bounded_prompt_preview_scope_acceptance",
      "rollback_plan_acceptance",
      "rollback_dry_run_acceptance",
      "kill_switch_acceptance",
      "kill_switch_dry_run_acceptance",
      "redacted_diff_review_acceptance",
      "redacted_diff_review_approval_acceptance",
      "model_invocation",
      "external_kg_adapter_read",
      "graphiti_client_construction",
      "neo4j_client_construction",
      "cocoindex_client_construction",
      "network_call",
      "external_db_write",
      "live_kg_write",
      "readiness_index_persistence",
      "operator_briefing_persistence",
      "telegram_or_channel_delivery",
      "gateway_route_migration",
      "source_command_migration",
      "ci_promotion",
      "preflight_execution_as_activation",
      "active_runtime_wiring",
      "install_restart",
      "launchd_mutation",
      "active_binary_mutation",
      "release_artifact_write",
      "public_release_claim",
      "public_ga_claim",
      "credential_read"
    ] as $denied_terminal_actions
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      gate:$gate,
      terminal_next_action_activation_denial_summary_schema_version:"kg_prompt_preview_terminal_next_action_activation_denial_summary_v1",
      terminal_next_action_activation_denial_summary_mode:"stdout_only_terminal_summary_no_prompt_render_no_context_injection_no_activation_no_runtime_mutation",
      terminal_next_action_activation_denial_summary_ready:true,
      terminal_next_action_activation_denial_summary_status:"blocked",
      terminal_activation_decision:"activation_denied_until_operator_evidence_safety_review_context_handoff_scope_monitoring_and_final_approval_are_present_reviewed_and_explicitly_accepted",
      source_context_handoff_checklist_gate:$source.gate,
      source_context_handoff_checklist_schema_version:$source.context_handoff_checklist_schema_version,
      source_context_handoff_checklist_decision:$source.context_handoff_checklist_decision,
      source_redacted_diff_review_checklist_gate:$source.source_redacted_diff_review_checklist_gate,
      source_rollback_kill_switch_checklist_gate:$source.source_rollback_kill_switch_checklist_gate,
      source_operator_approval_checklist_gate:$source.source_operator_approval_checklist_gate,
      source_readiness_index_gate:$source.source_readiness_index_gate,
      source_operator_briefing_gate:$source.source_operator_briefing_gate,
      source_terminal_summary_gate:$source.source_terminal_summary_gate,
      source_preflight_gate:$source.source_preflight_gate,
      source_preflight_contract:$source.source_preflight_contract,
      source_context_handoff_contract:$source.source_context_handoff_contract,
      source_context_handoff_checklist_report_sha256:$source_context_handoff_checklist_report_sha256,
      terminal_summary_hash_sha256:$terminal_summary_hash_sha256,
      terminal_policy_hash_sha256:$terminal_policy_hash_sha256,
      terminal_side_effect_hash_sha256:$terminal_side_effect_hash_sha256,
      source_gate_count:$source.source_gate_count,
      ready_source_gate_count:$source.ready_source_gate_count,
      blocked_source_gate_count:$source.blocked_source_gate_count,
      report_only_source_gate_count:$source.report_only_source_gate_count,
      source_operator_briefing_section_count:$source.source_operator_briefing_section_count,
      source_operator_briefing_sections_all_redacted:$source.source_operator_briefing_sections_all_redacted,
      source_operator_briefing_sections_all_blocked:$source.source_operator_briefing_sections_all_blocked,
      source_operator_briefing_sections_all_not_persisted:$source.source_operator_briefing_sections_all_not_persisted,
      required_operator_evidence_count:$source.required_operator_evidence_count,
      missing_operator_evidence_count:$source.missing_operator_evidence_count,
      required_safety_control_count:$source.required_safety_control_count,
      missing_safety_control_count:$source.missing_safety_control_count,
      required_handoff_requirement_count:$source.required_handoff_requirement_count,
      missing_handoff_requirement_count:$source.missing_handoff_requirement_count,
      missing_final_review_approval_count:$source.missing_final_review_approval_count,
      required_total_preflight_requirement_count:$source.required_total_preflight_requirement_count,
      missing_total_preflight_requirement_count:$source.missing_total_preflight_requirement_count,
      source_handoff_checklist_item_count:$source.handoff_checklist_item_count,
      source_missing_handoff_checklist_item_count:$source.missing_handoff_checklist_item_count,
      redacted_refs_only:true,
      raw_prompt_diff_count:0,
      prompt_text_included_count:0,
      payload_text_included_count:0,
      activation_allowed:false,
      prompt_preview_allowed:false,
      prompt_preview_rendered:false,
      prompt_payload_materialized:false,
      context_injection_allowed:false,
      context_injection_performed:false,
      context_handoff_accepted:false,
      context_handoff_checklist_persistence_allowed:false,
      context_handoff_checklist_persisted:false,
      context_handoff_checklist_delivery_allowed:false,
      context_handoff_checklist_delivered:false,
      operator_evidence_packet_accepted:false,
      rollback_kill_switch_safety_packet_accepted:false,
      redacted_diff_review_receipt_accepted:false,
      context_handoff_operator_approval_accepted:false,
      context_injection_scope_record_accepted:false,
      post_handoff_monitoring_plan_accepted:false,
      redacted_diff_review_accepted:false,
      redacted_diff_review_approval_accepted:false,
      rollback_plan_accepted:false,
      rollback_dry_run_evidence_accepted:false,
      kill_switch_accepted:false,
      kill_switch_dry_run_evidence_accepted:false,
      final_operator_approval_recorded:false,
      final_operator_approval_required:true,
      operator_approval_recorded:false,
      operator_approval_accepted:false,
      operator_identity_accepted:false,
      operator_scope_accepted:false,
      operator_activation_plan_accepted:false,
      approval_digest_accepted:false,
      bounded_prompt_preview_scope_accepted:false,
      model_invocation_allowed:false,
      model_invoked:false,
      external_kg_adapter_read_allowed:false,
      external_kg_adapter_read_performed:false,
      graphiti_client_constructed:false,
      neo4j_client_constructed:false,
      cocoindex_client_constructed:false,
      network_call_allowed:false,
      network_call_performed:false,
      external_db_write_performed:false,
      live_kg_write_allowed:false,
      live_kg_write_performed:false,
      readiness_index_persistence_allowed:false,
      readiness_index_persisted:false,
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
      launchd_mutation_allowed:false,
      active_binary_mutation_allowed:false,
      release_artifact_write_allowed:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      credential_read_allowed:false,
      allowed_terminal_actions:$allowed_terminal_actions,
      allowed_terminal_action_count:($allowed_terminal_actions | length),
      denied_terminal_actions:$denied_terminal_actions,
      denied_terminal_action_count:($denied_terminal_actions | length),
      source_allowed_next_actions:$source.allowed_next_actions,
      source_denied_next_actions:$source.denied_next_actions,
      source_denied_next_action_count:$source.denied_next_action_count,
      side_effects:{
        terminal_summary_persisted:false,
        terminal_summary_delivered:false,
        activation_allowed:false,
        prompt_preview_rendered:false,
        prompt_payload_materialized:false,
        raw_prompt_diff_exposed:false,
        prompt_text_exposed:false,
        payload_text_exposed:false,
        context_injection_performed:false,
        context_handoff_accepted:false,
        context_handoff_checklist_persisted:false,
        context_handoff_checklist_delivered:false,
        operator_evidence_packet_accepted:false,
        rollback_kill_switch_safety_packet_accepted:false,
        redacted_diff_review_receipt_accepted:false,
        context_handoff_operator_approval_accepted:false,
        context_injection_scope_record_accepted:false,
        post_handoff_monitoring_plan_accepted:false,
        redacted_diff_review_accepted:false,
        redacted_diff_review_approval_accepted:false,
        rollback_plan_accepted:false,
        rollback_dry_run_evidence_accepted:false,
        kill_switch_accepted:false,
        kill_switch_dry_run_evidence_accepted:false,
        final_operator_approval_recorded:false,
        operator_approval_recorded:false,
        operator_approval_accepted:false,
        operator_identity_accepted:false,
        operator_scope_accepted:false,
        operator_activation_plan_accepted:false,
        approval_digest_accepted:false,
        bounded_prompt_preview_scope_accepted:false,
        model_invoked:false,
        external_kg_adapter_read_performed:false,
        graphiti_client_constructed:false,
        neo4j_client_constructed:false,
        cocoindex_client_constructed:false,
        network_call_performed:false,
        external_db_write_performed:false,
        live_kg_write_performed:false,
        readiness_index_persisted:false,
        operator_briefing_persisted:false,
        operator_briefing_delivered:false,
        telegram_send_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        native_gateway_route_added:false,
        source_command_migration_performed:false,
        ci_promotion_performed:false,
        preflight_execution_performed:false,
        active_runtime_wired:false,
        install_performed:false,
        launchd_restart_performed:false,
        active_binary_mutated:false,
        release_artifact_written:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        credential_read_performed:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_kg_prompt_preview_terminal_next_action_activation_denial_summary_gate"
  and .terminal_next_action_activation_denial_summary_schema_version == "kg_prompt_preview_terminal_next_action_activation_denial_summary_v1"
  and .terminal_next_action_activation_denial_summary_mode == "stdout_only_terminal_summary_no_prompt_render_no_context_injection_no_activation_no_runtime_mutation"
  and .terminal_next_action_activation_denial_summary_ready == true
  and .terminal_next_action_activation_denial_summary_status == "blocked"
  and .terminal_activation_decision == "activation_denied_until_operator_evidence_safety_review_context_handoff_scope_monitoring_and_final_approval_are_present_reviewed_and_explicitly_accepted"
  and .source_context_handoff_checklist_gate == "hepta_kg_prompt_preview_context_handoff_checklist_gate"
  and .source_context_handoff_checklist_schema_version == "kg_prompt_preview_context_handoff_checklist_v1"
  and .source_redacted_diff_review_checklist_gate == "hepta_kg_prompt_preview_redacted_diff_review_checklist_gate"
  and .source_rollback_kill_switch_checklist_gate == "hepta_kg_prompt_preview_rollback_kill_switch_evidence_checklist_gate"
  and .source_operator_approval_checklist_gate == "hepta_kg_prompt_preview_operator_approval_checklist_schema_gate"
  and .source_readiness_index_gate == "hepta_kg_prompt_preview_readiness_next_action_index_gate"
  and .source_operator_briefing_gate == "hepta_kg_prompt_preview_operator_briefing_non_persistence_gate"
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
  and .required_operator_evidence_count == 7
  and .missing_operator_evidence_count == 7
  and .required_safety_control_count == 4
  and .missing_safety_control_count == 4
  and .required_handoff_requirement_count == 6
  and .missing_handoff_requirement_count == 6
  and .missing_final_review_approval_count == 2
  and .required_total_preflight_requirement_count == 19
  and .missing_total_preflight_requirement_count == 19
  and .source_handoff_checklist_item_count == 6
  and .source_missing_handoff_checklist_item_count == 6
  and .redacted_refs_only == true
  and .raw_prompt_diff_count == 0
  and .prompt_text_included_count == 0
  and .payload_text_included_count == 0
  and .activation_allowed == false
  and .prompt_preview_allowed == false
  and .prompt_preview_rendered == false
  and .prompt_payload_materialized == false
  and .context_injection_allowed == false
  and .context_injection_performed == false
  and .context_handoff_accepted == false
  and .context_handoff_checklist_persisted == false
  and .context_handoff_checklist_delivered == false
  and .operator_evidence_packet_accepted == false
  and .rollback_kill_switch_safety_packet_accepted == false
  and .redacted_diff_review_receipt_accepted == false
  and .context_handoff_operator_approval_accepted == false
  and .context_injection_scope_record_accepted == false
  and .post_handoff_monitoring_plan_accepted == false
  and .redacted_diff_review_accepted == false
  and .redacted_diff_review_approval_accepted == false
  and .rollback_plan_accepted == false
  and .rollback_dry_run_evidence_accepted == false
  and .kill_switch_accepted == false
  and .kill_switch_dry_run_evidence_accepted == false
  and .final_operator_approval_recorded == false
  and .final_operator_approval_required == true
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .operator_identity_accepted == false
  and .operator_scope_accepted == false
  and .operator_activation_plan_accepted == false
  and .approval_digest_accepted == false
  and .bounded_prompt_preview_scope_accepted == false
  and .model_invocation_allowed == false
  and .model_invoked == false
  and .external_kg_adapter_read_allowed == false
  and .external_kg_adapter_read_performed == false
  and .graphiti_client_constructed == false
  and .neo4j_client_constructed == false
  and .cocoindex_client_constructed == false
  and .network_call_allowed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_allowed == false
  and .live_kg_write_performed == false
  and .readiness_index_persisted == false
  and .operator_briefing_persisted == false
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
  and .launchd_mutation_allowed == false
  and .active_binary_mutation_allowed == false
  and .release_artifact_write_allowed == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .credential_read_allowed == false
  and .allowed_terminal_action_count == 3
  and (.allowed_terminal_actions | length) == 3
  and (.allowed_terminal_actions | all(.mutates_runtime == false and .permits_prompt_preview == false and .permits_context_injection == false))
  and (.allowed_terminal_actions | any(.action == "run_full_light_preflight" and .status == "allowed_verification_only"))
  and .denied_terminal_action_count == (.denied_terminal_actions | length)
  and .denied_terminal_action_count == 48
  and (.denied_terminal_actions | index("prompt_preview_execution") != null)
  and (.denied_terminal_actions | index("context_injection") != null)
  and (.denied_terminal_actions | index("model_invocation") != null)
  and (.denied_terminal_actions | index("external_kg_adapter_read") != null)
  and (.denied_terminal_actions | index("live_kg_write") != null)
  and (.denied_terminal_actions | index("public_release_claim") != null)
  and .source_denied_next_action_count == 35
  and (.source_denied_next_actions | length) == 35
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta KG prompt-preview terminal next-action activation denial summary gate passed"
