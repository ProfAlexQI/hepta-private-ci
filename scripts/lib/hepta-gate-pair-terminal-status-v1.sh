#!/usr/bin/env bash

render_signing_terminal_status_attachment() {
  local source_report="$ROOT/$source_report_rel"
  local terminal_status_gate_rel terminal_status_doc_rel
  local terminal_status_gate terminal_status_doc
  local missing_gate_message missing_doc_message
  terminal_status_gate_rel="$(jq -r '.terminal_status_gate' <<<"$spec")"
  terminal_status_doc_rel="$(jq -r '.terminal_status_doc' <<<"$spec")"
  terminal_status_gate="$ROOT/$terminal_status_gate_rel"
  terminal_status_doc="$ROOT/$terminal_status_doc_rel"
  missing_gate_message="$(jq -r '.missing_terminal_status_gate_message' <<<"$spec")"
  missing_doc_message="$(jq -r '.missing_terminal_status_doc_message' <<<"$spec")"

  [[ -x "$source_report" ]] || {
    echo "$missing_source_message: $source_report" >&2
    exit 1
  }
  [[ -f "$terminal_status_gate" ]] || {
    echo "$missing_gate_message: $terminal_status_gate" >&2
    exit 1
  }
  [[ -f "$terminal_status_doc" ]] || {
    echo "$missing_doc_message: $terminal_status_doc" >&2
    exit 1
  }

  local source_json terminal_status_static_mention_count
  local attachment_blocker_count terminal_status_static_prefix
  local architecture_note_rel
  source_json="$("$source_report")"
  attachment_blocker_count="$(jq -r '.attachment_blocker_count' <<<"$spec")"
  architecture_note_rel="$(jq -r '.architecture_note' <<<"$spec")"
  terminal_status_static_prefix="${summary_prefix%_promotion}"

  jq -e \
    --arg source_surface "$attachment_surface" \
    --argjson blocker_count "$blocker_count" \
    '
      .surface == $source_surface
      and .[($source_surface + "_ready")] == true
      and .[($source_surface + "_blocked")] == true
      and .final_blocker_count == $blocker_count
      and .signing_receipt_final_acknowledgement_recorded == false
      and .signing_receipt_operator_received_recorded == false
      and .signing_receipt_operator_read_recorded == false
      and .telegram_signing_receipt_acknowledgement_sent == false
      and .release_publication_authority_from_signing_receipt_acknowledgement_derived == false
      and .activation_authority_from_signing_receipt_acknowledgement_derived == false
      and .provider_invoked == false
      and .credential_read == false
      and .public_ga_claimed == false
      and .public_release_published == false
    ' <<<"$source_json" >/dev/null

  terminal_status_static_mention_count="$(
    grep -Eci 'terminal|decision|status|promotion|public|claim|accepted|approval|authority|download|install|restart|active-binary|telegram|external|live' "$terminal_status_gate" || true
  )"

  jq -n \
    --argjson source "$source_json" \
    --arg source_surface "$attachment_surface" \
    --arg attachment_surface "$readback_surface" \
    --arg final_ack_prefix "$acknowledgement_prefix" \
    --arg terminal_status_prefix "$summary_prefix" \
    --arg terminal_status_static_prefix "$terminal_status_static_prefix" \
    --argjson source_blocker_count "$blocker_count" \
    --argjson attachment_blocker_count "$attachment_blocker_count" \
    --argjson terminal_status_static_mention_count "$terminal_status_static_mention_count" \
    --arg next_migration_step "$next_migration_step" \
    --arg local_gate "scripts/$id-gate.sh" \
    --arg architecture_note "$architecture_note_rel" \
    --arg source_report "$source_report_rel" \
    --arg terminal_status_gate "$terminal_status_gate_rel" \
    --arg terminal_status_doc "$terminal_status_doc_rel" \
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
      ($terminal_status_prefix + "_denial_gate_present"): true,
      ($terminal_status_prefix + "_denial_doc_present"): true,
      ($terminal_status_static_prefix + "_static_mention_count"): $terminal_status_static_mention_count,
      ($terminal_status_prefix + "_denial_gate_invoked"): false,
      ($final_ack_prefix + "_denial_gate_invoked"): false,
      long_soak_started: false,
      final_operator_acknowledgement_accepted: false,
      acknowledgement_acceptance_recorded: false,
      terminal_decision_requested: false,
      status_promotion_requested: false,
      terminal_decision_allowed: false,
      status_promotion_allowed: false,
      terminal_decision_accepted: false,
      terminal_decision_recorded: false,
      terminal_decision_persisted: false,
      terminal_decision_materialized: false,
      terminal_decision_filesystem_written: false,
      terminal_decision_delivered: false,
      terminal_status_recorded: false,
      terminal_status_persisted: false,
      terminal_status_materialized: false,
      terminal_status_filesystem_written: false,
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
      attachment_blocker_count: $attachment_blocker_count,
      manual_operator_live_cutover_approval_required: true,
      public_ga_claim_allowed: false,
      public_ga_claimed: false,
      public_release_published: false,
      rollback_execution_allowed: false,
      next_migration_step: $next_migration_step,
      local_gate: $local_gate,
      architecture_note: $architecture_note,
      source_files: {
        signing_final_acknowledgement_final_index_report: $source_report,
        signing_terminal_status_denial_gate: $terminal_status_gate,
        signing_terminal_status_denial_doc: $terminal_status_doc
      },
      side_effect_free: true,
      side_effects: {
        report_written: false,
        git_index_mutated: false,
        terminal_decision_status_promotion_denial_gate_invoked: false,
        final_acknowledgement_denial_gate_invoked: false,
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
        install_from_terminal_status_executed: false,
        service_restart_from_terminal_status_performed: false,
        active_binary_from_terminal_status_mutated: false,
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

verify_signing_terminal_status_attachment() {
  local report="$ROOT/$report_rel"
  local source_gate_rel architecture_note_rel architecture_title
  local source_gate architecture_note fail_prefix
  local missing_source_gate_message missing_architecture_note_message
  source_gate_rel="$(jq -r '.source_gate' <<<"$spec")"
  architecture_note_rel="$(jq -r '.architecture_note' <<<"$spec")"
  architecture_title="$(jq -r '.architecture_title' <<<"$spec")"
  missing_source_gate_message="$(jq -r '.missing_source_gate_message' <<<"$spec")"
  missing_architecture_note_message="$(jq -r '.missing_architecture_note_message' <<<"$spec")"
  source_gate="$ROOT/$source_gate_rel"
  architecture_note="$ROOT/$architecture_note_rel"
  fail_prefix="${pass_message%%: PASS:*}"

  [[ -x "$report" ]] || {
    echo "$fail_prefix: FAIL: $missing_report_message: $report" >&2
    exit 1
  }
  [[ -x "$source_gate" ]] || {
    echo "$fail_prefix: FAIL: $missing_source_gate_message: $source_gate" >&2
    exit 1
  }
  [[ -f "$architecture_note" ]] || {
    echo "$fail_prefix: FAIL: $missing_architecture_note_message: $architecture_note" >&2
    exit 1
  }
  grep -q "$architecture_title" "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document $architecture_title" >&2; exit 1; }
  grep -q 'ready-but-blocked' "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document ready-but-blocked status" >&2; exit 1; }
  grep -q 'does not invoke' "$architecture_note" \
    || { echo "$fail_prefix: FAIL: architecture note must document that attachment does not invoke terminal decision/status gates" >&2; exit 1; }

  local json attachment_blocker_count terminal_status_static_prefix
  json="$("$report")"
  attachment_blocker_count="$(jq -r '.attachment_blocker_count' <<<"$spec")"
  terminal_status_static_prefix="${summary_prefix%_promotion}"
  jq -e \
    --arg source_surface "$attachment_surface" \
    --arg attachment_surface "$readback_surface" \
    --arg final_ack_prefix "$acknowledgement_prefix" \
    --arg terminal_status_prefix "$summary_prefix" \
    --arg terminal_status_static_prefix "$terminal_status_static_prefix" \
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
      and .[($terminal_status_prefix + "_denial_gate_present")] == true
      and .[($terminal_status_prefix + "_denial_doc_present")] == true
      and .[($terminal_status_static_prefix + "_static_mention_count")] >= 30
      and .[($terminal_status_prefix + "_denial_gate_invoked")] == false
      and .[($final_ack_prefix + "_denial_gate_invoked")] == false
      and .terminal_decision_recorded == false
      and .terminal_status_recorded == false
      and .status_promotion_recorded == false
      and .public_status_exposed == false
      and .external_decision_sent == false
      and .telegram_decision_sent == false
      and .operator_approval_from_terminal_status_derived == false
      and .release_publication_authority_from_terminal_status_derived == false
      and .activation_authority_from_terminal_status_derived == false
      and .install_from_terminal_status_executed == false
      and .active_binary_from_terminal_status_mutated == false
      and .attachment_blocker_count == $attachment_blocker_count
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

render_signing_terminal_status_readback() {
  local source_report="$ROOT/$source_report_rel"
  [[ -x "$source_report" ]] || {
    echo "$missing_source_message: $source_report" >&2
    exit 1
  }

  local source_json readback_mode final_ack_attachment_surface
  source_json="$("$source_report")"
  readback_mode="$(jq -r '.readback_mode' <<<"$spec")"
  final_ack_attachment_surface="$(jq -r '.final_ack_attachment_surface' <<<"$spec")"
  jq -e \
    --arg source_surface "$attachment_surface" \
    --argjson blocker_count "$blocker_count" \
    '
      .surface == $source_surface
      and .[($source_surface + "_ready")] == true
      and .[($source_surface + "_blocked")] == true
      and .attachment_blocker_count == $blocker_count
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
    --arg readback_surface "$readback_surface" \
    --arg final_ack_prefix "$acknowledgement_prefix" \
    --arg final_ack_attachment_surface "$final_ack_attachment_surface" \
    --arg terminal_status_prefix "$summary_prefix" \
    --arg readback_mode "$readback_mode" \
    --argjson blocker_count "$blocker_count" \
    --arg next_migration_step "$next_migration_step" \
    --arg source_report "$source_report_rel" \
    '{
      runtime: "hepta",
      surface: $readback_surface,
      plugin_id: "hepta-system@hepta-local",
      status: "ready_blocked",
      ("source_" + $source_surface + "_surface"): $source.surface,
      ("source_" + $source_surface + "_ready"): $source[($source_surface + "_ready")],
      ("source_" + $source_surface + "_blocked"): $source[($source_surface + "_blocked")],
      ($readback_surface + "_ready"): true,
      ($readback_surface + "_blocked"): true,
      ($source_surface + "_attached"): true,
      ($final_ack_attachment_surface + "_attached"): $source[($final_ack_attachment_surface + "_attached")],
      ($terminal_status_prefix + "_denial_gate_present"): $source[($terminal_status_prefix + "_denial_gate_present")],
      ($terminal_status_prefix + "_denial_doc_present"): $source[($terminal_status_prefix + "_denial_doc_present")],
      ($terminal_status_prefix + "_denial_gate_invoked"): false,
      ($final_ack_prefix + "_denial_gate_invoked"): false,
      readback_mode: $readback_mode,
      readback_check_count: $blocker_count,
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
      readback_blocker_count: $blocker_count,
      public_ga_claim_allowed: false,
      public_ga_claimed: false,
      public_release_published: false,
      rollback_execution_allowed: false,
      next_migration_step: $next_migration_step,
      source_files: {
        signing_terminal_decision_status_attachment_report: $source_report
      },
      side_effect_free: true,
      side_effects: ($source.side_effects + {
        readback_report_written: false,
        terminal_decision_status_readback_recorded: false,
        terminal_decision_status_promotion_denial_gate_invoked: false
      })
    }'
}

verify_signing_terminal_status_readback() {
  local report="$ROOT/$report_rel"
  local source_gate_rel architecture_note_rel architecture_title
  local source_gate architecture_note fail_prefix
  local missing_source_gate_message missing_architecture_note_message
  source_gate_rel="$(jq -r '.source_gate' <<<"$spec")"
  architecture_note_rel="$(jq -r '.architecture_note' <<<"$spec")"
  architecture_title="$(jq -r '.architecture_title' <<<"$spec")"
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
    || { echo "$fail_prefix: FAIL: architecture note must document that readback does not invoke terminal decision/status gates" >&2; exit 1; }

  local json
  json="$("$report")"
  jq -e \
    --arg source_surface "$attachment_surface" \
    --arg readback_surface "$readback_surface" \
    --arg terminal_status_prefix "$summary_prefix" \
    --argjson blocker_count "$blocker_count" \
    --arg next_migration_step "$next_migration_step" \
    '
      .runtime == "hepta"
      and .surface == $readback_surface
      and .[($readback_surface + "_ready")] == true
      and .[($readback_surface + "_blocked")] == true
      and .[($source_surface + "_attached")] == true
      and .[($terminal_status_prefix + "_denial_gate_invoked")] == false
      and .terminal_decision_recorded == false
      and .terminal_status_recorded == false
      and .status_promotion_recorded == false
      and .public_status_exposed == false
      and .telegram_decision_sent == false
      and .release_publication_authority_from_terminal_status_derived == false
      and .activation_authority_from_terminal_status_derived == false
      and .install_from_terminal_status_executed == false
      and .active_binary_from_terminal_status_mutated == false
      and .provider_invoked == false
      and .credential_read == false
      and .readback_blocker_count == $blocker_count
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
