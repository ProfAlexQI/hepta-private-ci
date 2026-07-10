#!/usr/bin/env bash

render_signing_public_status_attachment() {
  local source_report="$ROOT/$source_report_rel"
  local public_status_gate_rel public_status_doc_rel source_file_prefix
  local public_status_gate public_status_doc attachment_blocker_count
  public_status_gate_rel="$(jq -r '.public_status_gate' <<<"$spec")"
  public_status_doc_rel="$(jq -r '.public_status_doc' <<<"$spec")"
  source_file_prefix="$(jq -r '.source_file_prefix' <<<"$spec")"
  attachment_blocker_count="$(jq -r '.attachment_blocker_count' <<<"$spec")"
  public_status_gate="$ROOT/$public_status_gate_rel"
  public_status_doc="$ROOT/$public_status_doc_rel"

  [[ -x "$source_report" ]] || {
    echo "$missing_source_message: $source_report" >&2
    exit 1
  }
  [[ -f "$public_status_gate" ]] || {
    echo "$(jq -r '.missing_public_status_gate_message' <<<"$spec"): $public_status_gate" >&2
    exit 1
  }
  [[ -f "$public_status_doc" ]] || {
    echo "$(jq -r '.missing_public_status_doc_message' <<<"$spec"): $public_status_doc" >&2
    exit 1
  }

  local source_json public_status_static_mention_count local_gate
  source_json="$("$source_report")"
  jq -e \
    --arg source_surface "$attachment_surface" \
    --argjson blocker_count "$blocker_count" \
    '
      .surface == $source_surface
      and .[($source_surface + "_ready")] == true
      and .[($source_surface + "_blocked")] == true
      and .final_blocker_count == $blocker_count
      and .terminal_decision_recorded == false
      and .terminal_status_recorded == false
      and .status_promotion_recorded == false
      and .public_status_exposed == false
      and .public_ga_status_exposed == false
      and .public_release_status_exposed == false
      and .external_decision_sent == false
      and .telegram_decision_sent == false
      and .release_publication_authority_from_terminal_status_derived == false
      and .activation_authority_from_terminal_status_derived == false
      and .install_from_terminal_status_executed == false
      and .active_binary_from_terminal_status_mutated == false
      and .public_ga_claim_allowed == false
      and .public_ga_claimed == false
      and .public_release_published == false
    ' <<<"$source_json" >/dev/null

  public_status_static_mention_count="$(
    grep -Eci 'public|claim|status|exposure|release|channel|dashboard|endpoint|query|export|observability|telegram|external|authority|install|restart|active-binary|live' "$public_status_gate" || true
  )"
  local_gate="${report_rel%-report.sh}-gate.sh"

  jq -n \
    --argjson source "$source_json" \
    --arg source_surface "$attachment_surface" \
    --arg attachment_surface "$readback_surface" \
    --arg public_status_prefix "$acknowledgement_prefix" \
    --arg public_status_static_prefix "${acknowledgement_prefix%_exposure}" \
    --arg terminal_status_prefix "$summary_prefix" \
    --argjson public_status_static_mention_count "$public_status_static_mention_count" \
    --argjson attachment_blocker_count "$attachment_blocker_count" \
    --arg next_migration_step "$next_migration_step" \
    --arg local_gate "$local_gate" \
    --arg architecture_note "$(jq -r '.architecture_note' <<<"$spec")" \
    --arg source_file_prefix "$source_file_prefix" \
    --arg source_report "$source_report_rel" \
    --arg public_status_gate "$public_status_gate_rel" \
    --arg public_status_doc "$public_status_doc_rel" \
    '{
      runtime: "hepta",
      surface: $attachment_surface,
      plugin_id: "hepta-system@hepta-local",
      status: "ready_blocked",
      ("source_" + $source_surface + "_surface"): $source.surface,
      ("source_" + $source_surface + "_ready"): $source[($source_surface + "_ready")],
      ("source_" + $source_surface + "_blocked"): $source[($source_surface + "_blocked")],
      source_final_blocker_count: $source.final_blocker_count,
      ($source_surface + "_attached"): true,
      ($attachment_surface + "_ready"): true,
      ($attachment_surface + "_blocked"): true,
      ($public_status_prefix + "_denial_gate_present"): true,
      ($public_status_prefix + "_denial_doc_present"): true,
      ($public_status_static_prefix + "_static_mention_count"): $public_status_static_mention_count,
      ($public_status_prefix + "_denial_gate_invoked"): false,
      ($terminal_status_prefix + "_denial_gate_invoked"): false,
      long_soak_started: false,
      terminal_decision_recorded: false,
      terminal_status_recorded: false,
      status_promotion_recorded: false,
      terminal_public_claim_status_exposure_requested: false,
      terminal_public_claim_status_exposure_allowed: false,
      terminal_public_claim_status_exposure_accepted: false,
      terminal_public_claim_status_exposure_recorded: false,
      terminal_public_claim_status_exposure_persisted: false,
      terminal_public_claim_status_exposure_materialized: false,
      terminal_public_claim_status_exposure_filesystem_written: false,
      terminal_public_claim_status_exposure_delivered: false,
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
      memory_store_write_performed: false,
      live_kg_write_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      secret_file_read: false,
      telegram_send_performed: false,
      external_send_performed: false,
      public_ga_readiness_script_invoked: false,
      public_claim_non_promotion_denial_gate_invoked: false,
      terminal_live_gates_invoked: false,
      attachment_blocker_count: $attachment_blocker_count,
      manual_operator_live_cutover_approval_required: true,
      public_ga_claim_allowed: false,
      public_release_published: false,
      rollback_execution_allowed: false,
      next_migration_step: $next_migration_step,
      local_gate: $local_gate,
      architecture_note: $architecture_note,
      source_files: {
        ($source_file_prefix + "_terminal_decision_status_final_index_report"): $source_report,
        ($source_file_prefix + "_public_status_denial_gate"): $public_status_gate,
        ($source_file_prefix + "_public_status_denial_doc"): $public_status_doc
      },
      side_effect_free: true,
      side_effects: {
        report_written: false,
        git_index_mutated: false,
        public_claim_status_exposure_denial_gate_invoked: false,
        terminal_decision_status_promotion_denial_gate_invoked: false,
        public_claim_recorded: false,
        public_claim_persisted: false,
        public_status_exposed: false,
        public_ga_status_exposed: false,
        public_release_status_exposed: false,
        release_status_exposed: false,
        publication_status_exposed: false,
        package_release_channel_status_exposed: false,
        dashboard_status_exposed: false,
        status_endpoint_exposed: false,
        query_status_exposed: false,
        export_status_exposed: false,
        observability_status_exposed: false,
        channel_status_delivered: false,
        external_status_sent: false,
        telegram_status_sent: false,
        operator_approval_from_public_status_derived: false,
        release_publication_authority_from_public_status_derived: false,
        activation_authority_from_public_status_derived: false,
        install_from_public_status_executed: false,
        service_restart_from_public_status_performed: false,
        active_binary_from_public_status_mutated: false,
        provider_invoked: false,
        model_invoked: false,
        credential_read: false,
        secret_file_read: false,
        external_send_performed: false,
        telegram_send_performed: false,
        long_soak_started: false,
        terminal_live_gate_invoked: false,
        terminal_live_url_contacted: false,
        public_ga_readiness_script_invoked: false,
        public_claim_non_promotion_denial_gate_invoked: false,
        public_ga_claim_recorded: false,
        public_ga_promoted: false,
        public_release_published: false,
        rollback_executed: false,
        external_network_read: false
      }
    }'
}

verify_signing_public_status_attachment() {
  local report="$ROOT/$report_rel"
  local source_gate_rel architecture_note_rel architecture_title
  local source_gate architecture_note fail_prefix attachment_blocker_count
  source_gate_rel="$(jq -r '.source_gate' <<<"$spec")"
  architecture_note_rel="$(jq -r '.architecture_note' <<<"$spec")"
  architecture_title="$(jq -r '.architecture_title' <<<"$spec")"
  attachment_blocker_count="$(jq -r '.attachment_blocker_count' <<<"$spec")"
  source_gate="$ROOT/$source_gate_rel"
  architecture_note="$ROOT/$architecture_note_rel"
  fail_prefix="${pass_message%%: PASS:*}"

  [[ -x "$report" ]] || { echo "$fail_prefix: FAIL: $missing_report_message: $report" >&2; exit 1; }
  [[ -x "$source_gate" ]] || { echo "$fail_prefix: FAIL: $(jq -r '.missing_source_gate_message' <<<"$spec"): $source_gate" >&2; exit 1; }
  [[ -f "$architecture_note" ]] || { echo "$fail_prefix: FAIL: $(jq -r '.missing_architecture_note_message' <<<"$spec"): $architecture_note" >&2; exit 1; }
  grep -q "$architecture_title" "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document $architecture_title" >&2; exit 1; }
  grep -q 'ready-but-blocked' "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document ready-but-blocked status" >&2; exit 1; }
  grep -q 'does not invoke' "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document that attachment does not invoke public status gates" >&2; exit 1; }

  local json
  json="$("$report")"
  jq -e \
    --arg source_surface "$attachment_surface" \
    --arg attachment_surface "$readback_surface" \
    --arg public_status_prefix "$acknowledgement_prefix" \
    --arg public_status_static_prefix "${acknowledgement_prefix%_exposure}" \
    --arg terminal_status_prefix "$summary_prefix" \
    --argjson source_blocker_count "$blocker_count" \
    --argjson attachment_blocker_count "$attachment_blocker_count" \
    --arg next_migration_step "$next_migration_step" \
    '
      .runtime == "hepta"
      and .surface == $attachment_surface
      and .plugin_id == "hepta-system@hepta-local"
      and .status == "ready_blocked"
      and .[("source_" + $source_surface + "_ready")] == true
      and .[("source_" + $source_surface + "_blocked")] == true
      and .source_final_blocker_count == $source_blocker_count
      and .[($attachment_surface + "_ready")] == true
      and .[($attachment_surface + "_blocked")] == true
      and .[($public_status_prefix + "_denial_gate_present")] == true
      and .[($public_status_prefix + "_denial_doc_present")] == true
      and .[($public_status_static_prefix + "_static_mention_count")] >= 30
      and .[($public_status_prefix + "_denial_gate_invoked")] == false
      and .[($terminal_status_prefix + "_denial_gate_invoked")] == false
      and .terminal_public_claim_status_exposure_recorded == false
      and .public_status_claimed == false
      and .public_ga_claimed == false
      and .public_release_claimed == false
      and .public_status_exposed == false
      and .external_status_sent == false
      and .telegram_status_sent == false
      and .release_publication_authority_from_public_status_derived == false
      and .activation_authority_from_public_status_derived == false
      and .install_from_public_status_executed == false
      and .active_binary_from_public_status_mutated == false
      and .attachment_blocker_count == $attachment_blocker_count
      and .public_ga_claim_allowed == false
      and .public_release_published == false
      and .next_migration_step == $next_migration_step
      and .side_effect_free == true
      and (.side_effects | to_entries | all(.value == false))
    ' <<<"$json" >/dev/null

  "$source_gate" >/dev/null
  printf '%s\n' "$pass_message"
}
