#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

KG_READINESS_INDEX_JSON="$(
  capture_json_report \
    "hepta-kg-prompt-preview-readiness-next-action-index-gate" \
    scripts/hepta-kg-prompt-preview-readiness-next-action-index-gate.sh
)"

source_readiness_index_report_sha256="$(sha256_text "$KG_READINESS_INDEX_JSON")"
operator_approval_checklist_schema_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-operator-approval-checklist-schema:schema:$source_readiness_index_report_sha256")"
operator_approval_checklist_policy_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-operator-approval-checklist-schema:policy:$source_readiness_index_report_sha256")"
operator_approval_checklist_side_effect_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-operator-approval-checklist-schema:side-effects:$source_readiness_index_report_sha256")"

jq -n -e \
  --argjson source "$KG_READINESS_INDEX_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_kg_prompt_preview_readiness_next_action_index_gate"
    and $source.readiness_next_action_index_schema_version == "kg_prompt_preview_readiness_next_action_index_v1"
    and $source.readiness_next_action_index_ready == true
    and $source.readiness_next_action_index_status == "blocked"
    and $source.readiness_next_action_decision == "blocked_until_operator_approval_evidence_safety_handoff_review_and_activation_plan_exist"
    and ($source.allowed_next_actions | any(.action == "add_operator_approval_checklist_schema" and .status == "allowed_report_only" and .mutates_runtime == false and .permits_prompt_preview == false))
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
    and $source.source_redacted_refs_only == true
    and $source.source_raw_prompt_diff_count == 0
    and $source.source_prompt_text_included_count == 0
    and $source.source_payload_text_included_count == 0
    and $source.final_operator_approval_recorded == false
    and $source.final_operator_approval_required == true
    and $source.operator_identity_accepted == false
    and $source.operator_scope_accepted == false
    and $source.operator_activation_plan_accepted == false
    and $source.readiness_index_persistence_allowed == false
    and $source.readiness_index_persisted == false
    and $source.readiness_index_delivery_allowed == false
    and $source.readiness_index_delivered == false
    and $source.allowed_next_action_count == 6
    and $source.denied_next_action_count == 12
    and $source.denied_action_count == 32
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
    and $source.operator_briefing_persistence_allowed == false
    and $source.operator_briefing_persisted == false
    and $source.operator_briefing_delivery_allowed == false
    and $source.operator_briefing_delivered == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
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
    and ($source.side_effects | to_entries | all(.value == false))
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg gate "hepta_kg_prompt_preview_operator_approval_checklist_schema_gate" \
  --arg source_readiness_index_report_sha256 "$source_readiness_index_report_sha256" \
  --arg operator_approval_checklist_schema_hash_sha256 "$operator_approval_checklist_schema_hash_sha256" \
  --arg operator_approval_checklist_policy_hash_sha256 "$operator_approval_checklist_policy_hash_sha256" \
  --arg operator_approval_checklist_side_effect_hash_sha256 "$operator_approval_checklist_side_effect_hash_sha256" \
  --argjson source "$KG_READINESS_INDEX_JSON" \
  '
    [
      {
        checklist_item:"operator_approval_record",
        evidence_kind:"operator_approval",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-operator-approval:operator-approval-record",
        blocks_prompt_preview:true,
        persisted:false
      },
      {
        checklist_item:"rollback_plan_record",
        evidence_kind:"rollback_plan",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-operator-approval:rollback-plan-record",
        blocks_prompt_preview:true,
        persisted:false
      },
      {
        checklist_item:"kill_switch_record",
        evidence_kind:"kill_switch",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-operator-approval:kill-switch-record",
        blocks_prompt_preview:true,
        persisted:false
      },
      {
        checklist_item:"reviewer_identity_record",
        evidence_kind:"operator_identity",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-operator-approval:reviewer-identity-record",
        blocks_prompt_preview:true,
        persisted:false
      },
      {
        checklist_item:"approval_timestamp_record",
        evidence_kind:"approval_timestamp",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-operator-approval:approval-timestamp-record",
        blocks_prompt_preview:true,
        persisted:false
      },
      {
        checklist_item:"signed_approval_digest",
        evidence_kind:"signed_digest",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-operator-approval:signed-approval-digest",
        blocks_prompt_preview:true,
        persisted:false
      },
      {
        checklist_item:"bounded_prompt_preview_scope",
        evidence_kind:"bounded_scope",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-operator-approval:bounded-prompt-preview-scope",
        blocks_prompt_preview:true,
        persisted:false
      }
    ] as $checklist_items
    | [
      {
        action:"maintain_report_only_evidence_index",
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
    | ([
      "operator_approval_recording",
      "operator_approval_acceptance",
      "operator_identity_acceptance",
      "operator_scope_acceptance",
      "operator_activation_plan_acceptance",
      "operator_approval_checklist_persistence",
      "operator_approval_checklist_delivery",
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
    ]) as $denied_next_actions
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      gate:$gate,
      operator_approval_checklist_schema_version:"kg_prompt_preview_operator_approval_checklist_schema_v1",
      operator_approval_checklist_mode:"stdout_only_schema_only_no_approval_recording_no_prompt_render_no_context_injection_no_runtime_mutation",
      operator_approval_checklist_ready:true,
      operator_approval_checklist_status:"blocked",
      operator_approval_checklist_decision:"blocked_until_all_required_operator_approval_evidence_records_are_provided_reviewed_signed_scoped_and_explicitly_accepted",
      source_readiness_index_gate:$source.gate,
      source_readiness_index_schema_version:$source.readiness_next_action_index_schema_version,
      source_readiness_index_decision:$source.readiness_next_action_decision,
      source_operator_briefing_gate:$source.source_operator_briefing_gate,
      source_terminal_summary_gate:$source.source_terminal_summary_gate,
      source_preflight_gate:$source.source_preflight_gate,
      source_preflight_contract:$source.source_preflight_contract,
      source_context_handoff_contract:$source.source_context_handoff_contract,
      source_readiness_index_report_sha256:$source_readiness_index_report_sha256,
      operator_approval_checklist_schema_hash_sha256:$operator_approval_checklist_schema_hash_sha256,
      operator_approval_checklist_policy_hash_sha256:$operator_approval_checklist_policy_hash_sha256,
      operator_approval_checklist_side_effect_hash_sha256:$operator_approval_checklist_side_effect_hash_sha256,
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
      checklist_item_count:($checklist_items | length),
      required_checklist_item_count:($checklist_items | map(select(.required == true)) | length),
      missing_checklist_item_count:($checklist_items | map(select(.present == false)) | length),
      checklist_items:$checklist_items,
      checklist_items_all_required:($checklist_items | all(.required == true)),
      checklist_items_all_missing:($checklist_items | all(.present == false)),
      checklist_items_all_redacted:(($checklist_items | map(.redacted_evidence_ref | startswith("missing:kg-prompt-preview-operator-approval:"))) | all(. == true)),
      checklist_items_all_block_prompt_preview:($checklist_items | all(.blocks_prompt_preview == true)),
      checklist_items_all_not_persisted:($checklist_items | all(.persisted == false)),
      final_operator_approval_recorded:false,
      final_operator_approval_required:true,
      operator_approval_recorded:false,
      operator_approval_accepted:false,
      operator_identity_accepted:false,
      operator_scope_accepted:false,
      operator_activation_plan_accepted:false,
      approval_digest_accepted:false,
      bounded_prompt_preview_scope_accepted:false,
      operator_approval_checklist_persistence_allowed:false,
      operator_approval_checklist_persisted:false,
      operator_approval_checklist_delivery_allowed:false,
      operator_approval_checklist_delivered:false,
      readiness_index_persistence_allowed:false,
      readiness_index_persisted:false,
      readiness_index_delivery_allowed:false,
      readiness_index_delivered:false,
      allowed_next_actions:$allowed_next_actions,
      allowed_next_action_count:($allowed_next_actions | length),
      denied_next_actions:$denied_next_actions,
      denied_next_action_count:($denied_next_actions | length),
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
      side_effects:{
        operator_approval_recorded:false,
        operator_approval_accepted:false,
        operator_identity_accepted:false,
        operator_scope_accepted:false,
        operator_activation_plan_accepted:false,
        approval_digest_accepted:false,
        bounded_prompt_preview_scope_accepted:false,
        operator_approval_checklist_persisted:false,
        operator_approval_checklist_delivered:false,
        readiness_index_persisted:false,
        readiness_index_delivered:false,
        operator_briefing_persisted:false,
        operator_briefing_filesystem_written:false,
        operator_briefing_delivered:false,
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
  and .gate == "hepta_kg_prompt_preview_operator_approval_checklist_schema_gate"
  and .operator_approval_checklist_schema_version == "kg_prompt_preview_operator_approval_checklist_schema_v1"
  and .operator_approval_checklist_mode == "stdout_only_schema_only_no_approval_recording_no_prompt_render_no_context_injection_no_runtime_mutation"
  and .operator_approval_checklist_ready == true
  and .operator_approval_checklist_status == "blocked"
  and .operator_approval_checklist_decision == "blocked_until_all_required_operator_approval_evidence_records_are_provided_reviewed_signed_scoped_and_explicitly_accepted"
  and .source_readiness_index_gate == "hepta_kg_prompt_preview_readiness_next_action_index_gate"
  and .source_readiness_index_schema_version == "kg_prompt_preview_readiness_next_action_index_v1"
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
  and .checklist_item_count == 7
  and .required_checklist_item_count == 7
  and .missing_checklist_item_count == 7
  and (.checklist_items | length) == 7
  and .checklist_items_all_required == true
  and .checklist_items_all_missing == true
  and .checklist_items_all_redacted == true
  and .checklist_items_all_block_prompt_preview == true
  and .checklist_items_all_not_persisted == true
  and (.checklist_items | all(.required == true and .present == false and .blocks_prompt_preview == true and .persisted == false and (.redacted_evidence_ref | startswith("missing:kg-prompt-preview-operator-approval:"))))
  and .final_operator_approval_recorded == false
  and .final_operator_approval_required == true
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .operator_identity_accepted == false
  and .operator_scope_accepted == false
  and .operator_activation_plan_accepted == false
  and .approval_digest_accepted == false
  and .bounded_prompt_preview_scope_accepted == false
  and .operator_approval_checklist_persistence_allowed == false
  and .operator_approval_checklist_persisted == false
  and .operator_approval_checklist_delivery_allowed == false
  and .operator_approval_checklist_delivered == false
  and .readiness_index_persistence_allowed == false
  and .readiness_index_persisted == false
  and .readiness_index_delivery_allowed == false
  and .readiness_index_delivered == false
  and .allowed_next_action_count == 5
  and (.allowed_next_actions | length) == 5
  and (.allowed_next_actions | all(.mutates_runtime == false and .permits_prompt_preview == false))
  and .denied_next_action_count == 19
  and (.denied_next_actions | length) == 19
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
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta KG prompt-preview operator approval checklist schema gate passed"
