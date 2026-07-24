#!/usr/bin/env bash

render_signing_public_status_final_index() {
  local source_report="$ROOT/$source_report_rel"
  [[ -x "$source_report" ]] || {
    echo "$missing_source_message: $source_report" >&2
    exit 1
  }

  local source_json final_index_surface source_file_prefix source_report_display_path
  source_json="$("$source_report")"
  final_index_surface="${attachment_surface%_terminal_public_claim_status_exposure_readback}_terminal_decision_status_promotion_final_index"
  source_file_prefix="$(jq -r '.source_file_prefix' <<<"$spec")"
  source_report_display_path="$(jq -r '.source_report_display_path // .source_report' <<<"$spec")"
  jq -e \
    --arg source_surface "$attachment_surface" \
    --argjson blocker_count "$blocker_count" \
    '
      .surface == $source_surface
      and .[($source_surface + "_ready")] == true
      and .[($source_surface + "_blocked")] == true
      and .readback_check_count == $blocker_count
      and .terminal_public_claim_status_exposure_recorded == false
      and .public_status_claimed == false
      and .public_ga_claimed == false
      and .external_status_sent == false
      and .telegram_status_sent == false
      and .release_publication_authority_from_public_status_derived == false
      and .activation_authority_from_public_status_derived == false
    ' <<<"$source_json" >/dev/null

  jq -n \
    --argjson source "$source_json" \
    --arg source_surface "$attachment_surface" \
    --arg public_status_final_index_surface "$readback_surface" \
    --arg terminal_status_final_index_surface "$final_index_surface" \
    --arg public_status_prefix "$acknowledgement_prefix" \
    --arg terminal_status_prefix "$summary_prefix" \
    --argjson blocker_count "$blocker_count" \
    --arg next_migration_step "$next_migration_step" \
    --arg source_file_prefix "$source_file_prefix" \
    --arg source_report "$source_report_display_path" \
    '{
      runtime: "hepta",
      surface: $public_status_final_index_surface,
      plugin_id: "hepta-system@hepta-local",
      status: "ready_blocked",
      ("source_" + $source_surface + "_surface"): $source.surface,
      ("source_" + $source_surface + "_ready"): $source[($source_surface + "_ready")],
      ("source_" + $source_surface + "_blocked"): $source[($source_surface + "_blocked")],
      ($public_status_final_index_surface + "_ready"): true,
      ($public_status_final_index_surface + "_blocked"): true,
      ($source_surface + "_attached"): true,
      ($terminal_status_final_index_surface + "_attached"): $source[($terminal_status_final_index_surface + "_attached")],
      ($public_status_prefix + "_denial_gate_present"): $source[($public_status_prefix + "_denial_gate_present")],
      ($public_status_prefix + "_denial_doc_present"): $source[($public_status_prefix + "_denial_doc_present")],
      ($public_status_prefix + "_denial_gate_invoked"): false,
      ($terminal_status_prefix + "_denial_gate_invoked"): false,
      terminal_public_claim_status_exposure_recorded: false,
      terminal_public_claim_status_exposure_persisted: false,
      public_status_claimed: false,
      public_release_claimed: false,
      public_ga_claimed: false,
      public_status_exposed: false,
      public_ga_status_exposed: false,
      public_release_status_exposed: false,
      release_status_exposed: false,
      publication_status_exposed: false,
      package_release_channel_status_exposed: false,
      dashboard_status_exposed: false,
      public_badge_exposed: false,
      status_endpoint_exposed: false,
      query_status_exposed: false,
      export_status_exposed: false,
      observability_status_exposed: false,
      artifact_availability_status_exposed: false,
      distribution_queue_status_exposed: false,
      channel_status_delivered: false,
      external_status_sent: false,
      telegram_status_sent: false,
      acceptance_from_public_status_recorded: false,
      operator_approval_from_public_status_derived: false,
      release_publication_authority_from_public_status_derived: false,
      activation_authority_from_public_status_derived: false,
      activation_command_from_public_status_derived: false,
      live_execution_from_public_status_allowed: false,
      download_link_from_public_status_rendered: false,
      install_command_from_public_status_rendered: false,
      install_from_public_status_executed: false,
      service_restart_from_public_status_performed: false,
      active_binary_from_public_status_mutated: false,
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
      public_release_published: false,
      rollback_execution_allowed: false,
      next_migration_step: $next_migration_step,
      source_files: {
        ((if $source_file_prefix == "" then "" else $source_file_prefix + "_" end) + "public_claim_status_exposure_readback_report"): $source_report
      },
      side_effect_free: true,
      side_effects: ($source.side_effects + {
        final_index_report_written: false,
        public_claim_status_exposure_final_index_recorded: false,
        public_claim_status_exposure_denial_gate_invoked: false
      })
    }'
}

verify_signing_public_status_final_index() {
  local report="$ROOT/$report_rel"
  local source_gate_rel architecture_note_rel architecture_title
  local source_gate architecture_note fail_prefix final_index_surface
  source_gate_rel="$(jq -r '.source_gate' <<<"$spec")"
  architecture_note_rel="$(jq -r '.architecture_note' <<<"$spec")"
  architecture_title="$(jq -r '.architecture_title' <<<"$spec")"
  source_gate="$ROOT/$source_gate_rel"
  architecture_note="$ROOT/$architecture_note_rel"
  fail_prefix="${pass_message%%: PASS:*}"
  final_index_surface="${attachment_surface%_terminal_public_claim_status_exposure_readback}_terminal_decision_status_promotion_final_index"

  [[ -x "$report" ]] || { echo "$fail_prefix: FAIL: $missing_report_message: $report" >&2; exit 1; }
  [[ -x "$source_gate" ]] || { echo "$fail_prefix: FAIL: $(jq -r '.missing_source_gate_message' <<<"$spec"): $source_gate" >&2; exit 1; }
  [[ -f "$architecture_note" ]] || { echo "$fail_prefix: FAIL: $(jq -r '.missing_architecture_note_message' <<<"$spec"): $architecture_note" >&2; exit 1; }
  grep -q "$architecture_title" "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document $architecture_title" >&2; exit 1; }
  grep -q 'ready-but-blocked' "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document ready-but-blocked status" >&2; exit 1; }
  grep -q 'does not invoke' "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document that final index does not invoke public status gates" >&2; exit 1; }

  local json
  json="$("$report")"
  jq -e \
    --arg source_surface "$attachment_surface" \
    --arg public_status_final_index_surface "$readback_surface" \
    --arg terminal_status_final_index_surface "$final_index_surface" \
    --arg public_status_prefix "$acknowledgement_prefix" \
    --argjson blocker_count "$blocker_count" \
    --arg next_migration_step "$next_migration_step" \
    '
      .runtime == "hepta"
      and .surface == $public_status_final_index_surface
      and .plugin_id == "hepta-system@hepta-local"
      and .status == "ready_blocked"
      and .[($public_status_final_index_surface + "_ready")] == true
      and .[($public_status_final_index_surface + "_blocked")] == true
      and .[($source_surface + "_attached")] == true
      and .[($terminal_status_final_index_surface + "_attached")] == true
      and .[($public_status_prefix + "_denial_gate_present")] == true
      and .[($public_status_prefix + "_denial_doc_present")] == true
      and .[($public_status_prefix + "_denial_gate_invoked")] == false
      and .terminal_public_claim_status_exposure_recorded == false
      and .public_status_claimed == false
      and .public_release_claimed == false
      and .public_ga_claimed == false
      and .public_status_exposed == false
      and .public_ga_status_exposed == false
      and .public_release_status_exposed == false
      and .external_status_sent == false
      and .telegram_status_sent == false
      and .operator_approval_from_public_status_derived == false
      and .release_publication_authority_from_public_status_derived == false
      and .activation_authority_from_public_status_derived == false
      and .install_from_public_status_executed == false
      and .service_restart_from_public_status_performed == false
      and .active_binary_from_public_status_mutated == false
      and .provider_invoked == false
      and .credential_read == false
      and .final_blocker_count == $blocker_count
      and .terminal_live_url_required == false
      and .long_soak_required == false
      and .public_ga_claim_allowed == false
      and .public_release_published == false
      and .next_migration_step == $next_migration_step
      and .side_effect_free == true
      and (.side_effects | to_entries | all(.value == false))
    ' <<<"$json" >/dev/null

  "$source_gate" >/dev/null
  printf '%s\n' "$pass_message"
}
