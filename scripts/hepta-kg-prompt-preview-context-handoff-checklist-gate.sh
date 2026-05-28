#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

KG_REDACTED_DIFF_REVIEW_CHECKLIST_JSON="$(
  capture_json_report \
    "hepta-kg-prompt-preview-redacted-diff-review-checklist-gate" \
    scripts/hepta-kg-prompt-preview-redacted-diff-review-checklist-gate.sh
)"

source_redacted_diff_review_checklist_report_sha256="$(sha256_text "$KG_REDACTED_DIFF_REVIEW_CHECKLIST_JSON")"
context_handoff_checklist_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-context-handoff-checklist:schema:$source_redacted_diff_review_checklist_report_sha256")"
context_handoff_policy_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-context-handoff-checklist:policy:$source_redacted_diff_review_checklist_report_sha256")"
context_handoff_side_effect_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-context-handoff-checklist:side-effects:$source_redacted_diff_review_checklist_report_sha256")"

jq -n -e \
  --argjson source "$KG_REDACTED_DIFF_REVIEW_CHECKLIST_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_kg_prompt_preview_redacted_diff_review_checklist_gate"
    and $source.redacted_diff_review_checklist_schema_version == "kg_prompt_preview_redacted_diff_review_checklist_v1"
    and $source.redacted_diff_review_checklist_ready == true
    and $source.redacted_diff_review_checklist_status == "blocked"
    and $source.redacted_diff_review_checklist_decision == "blocked_until_redacted_diff_review_and_approval_records_are_provided_reviewed_and_explicitly_accepted"
    and ($source.allowed_next_actions | any(.action == "add_context_handoff_checklist" and .status == "allowed_report_only" and .mutates_runtime == false and .permits_prompt_preview == false))
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
    and $source.source_safety_checklist_item_count == 4
    and $source.source_missing_safety_checklist_item_count == 4
    and $source.review_checklist_item_count == 2
    and $source.missing_review_checklist_item_count == 2
    and $source.review_checklist_items_all_required == true
    and $source.review_checklist_items_all_missing == true
    and $source.review_checklist_items_all_redacted == true
    and $source.review_checklist_items_all_block_prompt_preview == true
    and $source.review_checklist_items_all_block_context_injection == true
    and $source.review_checklist_items_all_not_persisted == true
    and $source.redacted_refs_only == true
    and $source.raw_prompt_diff_count == 0
    and $source.prompt_text_included_count == 0
    and $source.payload_text_included_count == 0
    and $source.redacted_diff_review_accepted == false
    and $source.redacted_diff_review_approval_accepted == false
    and $source.redacted_diff_review_checklist_persistence_allowed == false
    and $source.redacted_diff_review_checklist_persisted == false
    and $source.redacted_diff_review_checklist_delivery_allowed == false
    and $source.redacted_diff_review_checklist_delivered == false
    and $source.rollback_plan_accepted == false
    and $source.rollback_dry_run_evidence_accepted == false
    and $source.kill_switch_accepted == false
    and $source.kill_switch_dry_run_evidence_accepted == false
    and $source.final_operator_approval_recorded == false
    and $source.final_operator_approval_required == true
    and $source.operator_approval_recorded == false
    and $source.operator_approval_accepted == false
    and $source.operator_identity_accepted == false
    and $source.operator_scope_accepted == false
    and $source.operator_activation_plan_accepted == false
    and $source.approval_digest_accepted == false
    and $source.bounded_prompt_preview_scope_accepted == false
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
  --arg gate "hepta_kg_prompt_preview_context_handoff_checklist_gate" \
  --arg source_redacted_diff_review_checklist_report_sha256 "$source_redacted_diff_review_checklist_report_sha256" \
  --arg context_handoff_checklist_hash_sha256 "$context_handoff_checklist_hash_sha256" \
  --arg context_handoff_policy_hash_sha256 "$context_handoff_policy_hash_sha256" \
  --arg context_handoff_side_effect_hash_sha256 "$context_handoff_side_effect_hash_sha256" \
  --argjson source "$KG_REDACTED_DIFF_REVIEW_CHECKLIST_JSON" \
  '
    [
      {
        requirement_index:1,
        checklist_item:"operator_evidence_packet",
        requirement_kind:"operator_evidence",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-context-handoff:operator_evidence_packet",
        blocks_prompt_preview:true,
        blocks_context_injection:true,
        persisted:false
      },
      {
        requirement_index:2,
        checklist_item:"rollback_kill_switch_safety_packet",
        requirement_kind:"safety",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-context-handoff:rollback_kill_switch_safety_packet",
        blocks_prompt_preview:true,
        blocks_context_injection:true,
        persisted:false
      },
      {
        requirement_index:3,
        checklist_item:"redacted_diff_review_receipt",
        requirement_kind:"review",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-context-handoff:redacted_diff_review_receipt",
        blocks_prompt_preview:true,
        blocks_context_injection:true,
        persisted:false
      },
      {
        requirement_index:4,
        checklist_item:"context_handoff_operator_approval",
        requirement_kind:"operator_approval",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-context-handoff:context_handoff_operator_approval",
        blocks_prompt_preview:true,
        blocks_context_injection:true,
        persisted:false
      },
      {
        requirement_index:5,
        checklist_item:"context_injection_scope_record",
        requirement_kind:"scope",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-context-handoff:context_injection_scope_record",
        blocks_prompt_preview:true,
        blocks_context_injection:true,
        persisted:false
      },
      {
        requirement_index:6,
        checklist_item:"post_handoff_monitoring_plan",
        requirement_kind:"monitoring",
        required:true,
        present:false,
        redacted_evidence_ref:"missing:kg-prompt-preview-context-handoff:post_handoff_monitoring_plan",
        blocks_prompt_preview:true,
        blocks_context_injection:true,
        persisted:false
      }
    ] as $handoff_items
    | [
      {
        action:"maintain_report_only_evidence_index",
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
      "context_handoff_acceptance",
      "context_handoff_approval_acceptance",
      "context_handoff_checklist_persistence",
      "context_handoff_checklist_delivery",
      "operator_evidence_packet_acceptance",
      "rollback_kill_switch_safety_packet_acceptance",
      "redacted_diff_review_receipt_acceptance",
      "context_injection_scope_acceptance",
      "post_handoff_monitoring_plan_acceptance",
      "redacted_diff_review_acceptance",
      "redacted_diff_review_approval_acceptance",
      "raw_prompt_diff_exposure",
      "prompt_text_exposure",
      "payload_text_exposure",
      "rollback_plan_acceptance",
      "rollback_dry_run_acceptance",
      "kill_switch_acceptance",
      "kill_switch_dry_run_acceptance",
      "operator_approval_recording",
      "operator_approval_acceptance",
      "operator_identity_acceptance",
      "operator_scope_acceptance",
      "operator_activation_plan_acceptance",
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
      context_handoff_checklist_schema_version:"kg_prompt_preview_context_handoff_checklist_v1",
      context_handoff_checklist_mode:"stdout_only_schema_only_no_handoff_acceptance_no_context_injection_no_prompt_render_no_runtime_mutation",
      context_handoff_checklist_ready:true,
      context_handoff_checklist_status:"blocked",
      context_handoff_checklist_decision:"blocked_until_operator_evidence_safety_redacted_diff_context_handoff_approval_scope_and_monitoring_records_are_provided_reviewed_and_explicitly_accepted",
      source_redacted_diff_review_checklist_gate:$source.gate,
      source_redacted_diff_review_checklist_schema_version:$source.redacted_diff_review_checklist_schema_version,
      source_redacted_diff_review_checklist_decision:$source.redacted_diff_review_checklist_decision,
      source_rollback_kill_switch_checklist_gate:$source.source_rollback_kill_switch_checklist_gate,
      source_operator_approval_checklist_gate:$source.source_operator_approval_checklist_gate,
      source_readiness_index_gate:$source.source_readiness_index_gate,
      source_operator_briefing_gate:$source.source_operator_briefing_gate,
      source_terminal_summary_gate:$source.source_terminal_summary_gate,
      source_preflight_gate:$source.source_preflight_gate,
      source_preflight_contract:$source.source_preflight_contract,
      source_context_handoff_contract:$source.source_context_handoff_contract,
      source_redacted_diff_review_checklist_report_sha256:$source_redacted_diff_review_checklist_report_sha256,
      context_handoff_checklist_hash_sha256:$context_handoff_checklist_hash_sha256,
      context_handoff_policy_hash_sha256:$context_handoff_policy_hash_sha256,
      context_handoff_side_effect_hash_sha256:$context_handoff_side_effect_hash_sha256,
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
      source_review_checklist_item_count:$source.review_checklist_item_count,
      source_missing_review_checklist_item_count:$source.missing_review_checklist_item_count,
      handoff_checklist_item_count:($handoff_items | length),
      required_handoff_checklist_item_count:($handoff_items | map(select(.required == true)) | length),
      missing_handoff_checklist_item_count:($handoff_items | map(select(.present == false)) | length),
      handoff_checklist_items:$handoff_items,
      handoff_checklist_items_all_required:($handoff_items | all(.required == true)),
      handoff_checklist_items_all_missing:($handoff_items | all(.present == false)),
      handoff_checklist_items_all_redacted:(($handoff_items | map(.redacted_evidence_ref | startswith("missing:kg-prompt-preview-context-handoff:"))) | all(. == true)),
      handoff_checklist_items_all_block_prompt_preview:($handoff_items | all(.blocks_prompt_preview == true)),
      handoff_checklist_items_all_block_context_injection:($handoff_items | all(.blocks_context_injection == true)),
      handoff_checklist_items_all_not_persisted:($handoff_items | all(.persisted == false)),
      redacted_refs_only:true,
      raw_prompt_diff_count:0,
      prompt_text_included_count:0,
      payload_text_included_count:0,
      operator_evidence_packet_present:false,
      operator_evidence_packet_accepted:false,
      rollback_kill_switch_safety_packet_present:false,
      rollback_kill_switch_safety_packet_accepted:false,
      redacted_diff_review_receipt_present:false,
      redacted_diff_review_receipt_accepted:false,
      context_handoff_operator_approval_present:false,
      context_handoff_operator_approval_accepted:false,
      context_injection_scope_record_present:false,
      context_injection_scope_record_accepted:false,
      post_handoff_monitoring_plan_present:false,
      post_handoff_monitoring_plan_accepted:false,
      context_handoff_checklist_persistence_allowed:false,
      context_handoff_checklist_persisted:false,
      context_handoff_checklist_delivery_allowed:false,
      context_handoff_checklist_delivered:false,
      redacted_diff_review_accepted:false,
      redacted_diff_review_approval_accepted:false,
      redacted_diff_review_checklist_persistence_allowed:false,
      redacted_diff_review_checklist_persisted:false,
      redacted_diff_review_checklist_delivery_allowed:false,
      redacted_diff_review_checklist_delivered:false,
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
      operator_approval_checklist_persistence_allowed:false,
      operator_approval_checklist_persisted:false,
      operator_approval_checklist_delivery_allowed:false,
      operator_approval_checklist_delivered:false,
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
        operator_evidence_packet_accepted:false,
        rollback_kill_switch_safety_packet_accepted:false,
        redacted_diff_review_receipt_accepted:false,
        context_handoff_operator_approval_accepted:false,
        context_injection_scope_record_accepted:false,
        post_handoff_monitoring_plan_accepted:false,
        context_handoff_checklist_persisted:false,
        context_handoff_checklist_delivered:false,
        redacted_diff_review_accepted:false,
        redacted_diff_review_approval_accepted:false,
        raw_prompt_diff_exposed:false,
        prompt_text_exposed:false,
        payload_text_exposed:false,
        rollback_plan_accepted:false,
        rollback_dry_run_evidence_accepted:false,
        kill_switch_accepted:false,
        kill_switch_dry_run_evidence_accepted:false,
        operator_approval_recorded:false,
        operator_approval_accepted:false,
        operator_identity_accepted:false,
        operator_scope_accepted:false,
        operator_activation_plan_accepted:false,
        approval_digest_accepted:false,
        bounded_prompt_preview_scope_accepted:false,
        operator_approval_checklist_persisted:false,
        operator_approval_checklist_delivered:false,
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
  and .gate == "hepta_kg_prompt_preview_context_handoff_checklist_gate"
  and .context_handoff_checklist_schema_version == "kg_prompt_preview_context_handoff_checklist_v1"
  and .context_handoff_checklist_mode == "stdout_only_schema_only_no_handoff_acceptance_no_context_injection_no_prompt_render_no_runtime_mutation"
  and .context_handoff_checklist_ready == true
  and .context_handoff_checklist_status == "blocked"
  and .context_handoff_checklist_decision == "blocked_until_operator_evidence_safety_redacted_diff_context_handoff_approval_scope_and_monitoring_records_are_provided_reviewed_and_explicitly_accepted"
  and .source_redacted_diff_review_checklist_gate == "hepta_kg_prompt_preview_redacted_diff_review_checklist_gate"
  and .source_redacted_diff_review_checklist_schema_version == "kg_prompt_preview_redacted_diff_review_checklist_v1"
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
  and .source_review_checklist_item_count == 2
  and .source_missing_review_checklist_item_count == 2
  and .handoff_checklist_item_count == 6
  and .required_handoff_checklist_item_count == 6
  and .missing_handoff_checklist_item_count == 6
  and (.handoff_checklist_items | length) == 6
  and .handoff_checklist_items_all_required == true
  and .handoff_checklist_items_all_missing == true
  and .handoff_checklist_items_all_redacted == true
  and .handoff_checklist_items_all_block_prompt_preview == true
  and .handoff_checklist_items_all_block_context_injection == true
  and .handoff_checklist_items_all_not_persisted == true
  and (.handoff_checklist_items | all(.required == true and .present == false and .blocks_prompt_preview == true and .blocks_context_injection == true and .persisted == false and (.redacted_evidence_ref | startswith("missing:kg-prompt-preview-context-handoff:"))))
  and .redacted_refs_only == true
  and .raw_prompt_diff_count == 0
  and .prompt_text_included_count == 0
  and .payload_text_included_count == 0
  and .operator_evidence_packet_accepted == false
  and .rollback_kill_switch_safety_packet_accepted == false
  and .redacted_diff_review_receipt_accepted == false
  and .context_handoff_operator_approval_accepted == false
  and .context_injection_scope_record_accepted == false
  and .post_handoff_monitoring_plan_accepted == false
  and .context_handoff_checklist_persistence_allowed == false
  and .context_handoff_checklist_persisted == false
  and .context_handoff_checklist_delivery_allowed == false
  and .context_handoff_checklist_delivered == false
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
  and .operator_approval_checklist_persistence_allowed == false
  and .operator_approval_checklist_persisted == false
  and .operator_approval_checklist_delivery_allowed == false
  and .operator_approval_checklist_delivered == false
  and .allowed_next_action_count == 2
  and (.allowed_next_actions | length) == 2
  and (.allowed_next_actions | all(.mutates_runtime == false and .permits_prompt_preview == false))
  and .denied_next_action_count == 35
  and (.denied_next_actions | length) == 35
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
echo "Hepta KG prompt-preview context handoff checklist gate passed"
