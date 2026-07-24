#!/usr/bin/env bash

render_signing_terminal_status_final_index() {
  local source_report="$ROOT/$source_report_rel"
  [[ -x "$source_report" ]] || {
    echo "$missing_source_message: $source_report" >&2
    exit 1
  }

  local source_json final_ack_attachment_surface source_file_key source_report_display_path
  source_json="$("$source_report")"
  final_ack_attachment_surface="$(jq -r '.final_ack_attachment_surface' <<<"$spec")"
  source_file_key="$(jq -r '.source_file_key // "signing_terminal_decision_status_readback_report"' <<<"$spec")"
  source_report_display_path="$(jq -r '.source_report_display_path // .source_report' <<<"$spec")"
  jq -e \
    --arg source_surface "$attachment_surface" \
    --argjson blocker_count "$blocker_count" \
    '
      .surface == $source_surface
      and .[($source_surface + "_ready")] == true
      and .[($source_surface + "_blocked")] == true
      and .readback_check_count == $blocker_count
      and .terminal_decision_recorded == false
      and .terminal_status_recorded == false
      and .status_promotion_recorded == false
      and .public_status_exposed == false
      and .telegram_decision_sent == false
      and .release_publication_authority_from_terminal_status_derived == false
      and .activation_authority_from_terminal_status_derived == false
    ' <<<"$source_json" >/dev/null

  jq -n \
    --argjson source "$source_json" \
    --arg source_surface "$attachment_surface" \
    --arg final_index_surface "$readback_surface" \
    --arg final_ack_prefix "$acknowledgement_prefix" \
    --arg final_ack_attachment_surface "$final_ack_attachment_surface" \
    --arg terminal_status_prefix "$summary_prefix" \
    --argjson blocker_count "$blocker_count" \
    --arg next_migration_step "$next_migration_step" \
    --arg source_report "$source_report_display_path" \
    --arg source_file_key "$source_file_key" \
    '{
      runtime: "hepta",
      surface: $final_index_surface,
      plugin_id: "hepta-system@hepta-local",
      status: "ready_blocked",
      ("source_" + $source_surface + "_surface"): $source.surface,
      ("source_" + $source_surface + "_ready"): $source[($source_surface + "_ready")],
      ("source_" + $source_surface + "_blocked"): $source[($source_surface + "_blocked")],
      ($final_index_surface + "_ready"): true,
      ($final_index_surface + "_blocked"): true,
      ($source_surface + "_attached"): true,
      ($final_ack_attachment_surface + "_attached"): $source[($final_ack_attachment_surface + "_attached")],
      ($terminal_status_prefix + "_denial_gate_present"): $source[($terminal_status_prefix + "_denial_gate_present")],
      ($terminal_status_prefix + "_denial_doc_present"): $source[($terminal_status_prefix + "_denial_doc_present")],
      ($terminal_status_prefix + "_denial_gate_invoked"): false,
      ($final_ack_prefix + "_denial_gate_invoked"): false,
      terminal_decision_recorded: false,
      terminal_decision_persisted: false,
      terminal_status_recorded: false,
      terminal_status_persisted: false,
      status_promotion_recorded: false,
      public_status_exposed: false,
      public_ga_status_exposed: false,
      public_release_status_exposed: false,
      external_decision_sent: false,
      telegram_decision_sent: false,
      acceptance_from_terminal_decision_recorded: false,
      operator_approval_from_terminal_status_derived: false,
      release_publication_authority_from_terminal_status_derived: false,
      activation_authority_from_terminal_status_derived: false,
      activation_command_from_terminal_status_derived: false,
      live_execution_from_terminal_status_allowed: false,
      download_link_from_terminal_status_rendered: false,
      install_command_from_terminal_status_rendered: false,
      install_from_terminal_status_executed: false,
      service_restart_from_terminal_status_performed: false,
      active_binary_from_terminal_status_mutated: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      secret_file_read: false,
      telegram_send_performed: false,
      external_send_performed: false,
      public_ga_readiness_script_invoked: false,
      public_claim_non_promotion_denial_gate_invoked: false,
      terminal_live_gates_invoked: false,
      final_blocker_count: $blocker_count,
      manual_operator_live_cutover_approval_required: true,
      terminal_live_url_required: false,
      long_soak_required: false,
      public_ga_claim_allowed: false,
      public_ga_claimed: false,
      public_release_published: false,
      rollback_execution_allowed: false,
      next_migration_step: $next_migration_step,
      source_files: {
        ($source_file_key): $source_report
      },
      side_effect_free: true,
      side_effects: ($source.side_effects + {
        final_index_report_written: false,
        terminal_decision_status_final_index_recorded: false,
        terminal_decision_status_promotion_denial_gate_invoked: false
      })
    }'
}

verify_signing_terminal_status_final_index() {
  local report="$ROOT/$report_rel"
  local source_gate_rel architecture_note_rel architecture_title
  local source_gate architecture_note fail_prefix final_ack_attachment_surface
  local missing_source_gate_message missing_architecture_note_message
  source_gate_rel="$(jq -r '.source_gate' <<<"$spec")"
  architecture_note_rel="$(jq -r '.architecture_note' <<<"$spec")"
  architecture_title="$(jq -r '.architecture_title' <<<"$spec")"
  final_ack_attachment_surface="$(jq -r '.final_ack_attachment_surface' <<<"$spec")"
  missing_source_gate_message="$(jq -r '.missing_source_gate_message' <<<"$spec")"
  missing_architecture_note_message="$(jq -r '.missing_architecture_note_message' <<<"$spec")"
  source_gate="$ROOT/$source_gate_rel"
  architecture_note="$ROOT/$architecture_note_rel"
  fail_prefix="${pass_message%%: PASS:*}"

  [[ -x "$report" ]] || { echo "$fail_prefix: FAIL: $missing_report_message: $report" >&2; exit 1; }
  [[ -x "$source_gate" ]] || { echo "$fail_prefix: FAIL: $missing_source_gate_message: $source_gate" >&2; exit 1; }
  [[ -f "$architecture_note" ]] || { echo "$fail_prefix: FAIL: $missing_architecture_note_message: $architecture_note" >&2; exit 1; }
  grep -q "$architecture_title" "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document $architecture_title" >&2; exit 1; }
  grep -q 'ready-but-blocked' "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document ready-but-blocked status" >&2; exit 1; }
  grep -q 'does not invoke' "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document that final index does not invoke terminal decision/status gates" >&2; exit 1; }

  local json
  json="$("$report")"
  jq -e \
    --arg source_surface "$attachment_surface" \
    --arg final_index_surface "$readback_surface" \
    --arg final_ack_attachment_surface "$final_ack_attachment_surface" \
    --arg terminal_status_prefix "$summary_prefix" \
    --argjson blocker_count "$blocker_count" \
    --arg next_migration_step "$next_migration_step" \
    '
      .runtime == "hepta"
      and .surface == $final_index_surface
      and .plugin_id == "hepta-system@hepta-local"
      and .status == "ready_blocked"
      and .[($final_index_surface + "_ready")] == true
      and .[($final_index_surface + "_blocked")] == true
      and .[($source_surface + "_attached")] == true
      and .[($final_ack_attachment_surface + "_attached")] == true
      and .[($terminal_status_prefix + "_denial_gate_present")] == true
      and .[($terminal_status_prefix + "_denial_doc_present")] == true
      and .[($terminal_status_prefix + "_denial_gate_invoked")] == false
      and .terminal_decision_recorded == false
      and .terminal_decision_persisted == false
      and .terminal_status_recorded == false
      and .terminal_status_persisted == false
      and .status_promotion_recorded == false
      and .public_status_exposed == false
      and .public_ga_status_exposed == false
      and .public_release_status_exposed == false
      and .external_decision_sent == false
      and .telegram_decision_sent == false
      and .operator_approval_from_terminal_status_derived == false
      and .release_publication_authority_from_terminal_status_derived == false
      and .activation_authority_from_terminal_status_derived == false
      and .install_from_terminal_status_executed == false
      and .service_restart_from_terminal_status_performed == false
      and .active_binary_from_terminal_status_mutated == false
      and .provider_invoked == false
      and .credential_read == false
      and .final_blocker_count == $blocker_count
      and .terminal_live_url_required == false
      and .long_soak_required == false
      and .public_ga_claim_allowed == false
      and .public_ga_claimed == false
      and .public_release_published == false
      and .next_migration_step == $next_migration_step
      and .side_effect_free == true
      and (.side_effects | to_entries | all(.value == false))
    ' <<<"$json" >/dev/null

  "$source_gate" >/dev/null
  printf '%s\n' "$pass_message"
}
