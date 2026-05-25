#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

capture_json_report() {
  local command_name="$1"
  shift

  local output
  output="$("$@")"
  local report
  report="$(printf '%s\n' "$output" | sed '$d')"

  if ! jq -e . >/dev/null <<<"$report"; then
    echo "$command_name did not emit a parseable JSON report" >&2
    exit 1
  fi

  printf '%s\n' "$report"
}

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

DRY_RUN_LEDGER_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-dry-run-ledger-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-dry-run-ledger-gate.sh
)"

dry_run_ledger_report_sha256="$(
  printf '%s' "$DRY_RUN_LEDGER_JSON" | shasum -a 256 | awk '{print $1}'
)"

missing_approval_shape_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-shape-approval:missing-persistence-approval-id:$dry_run_ledger_report_sha256"
)"
stale_soak_shape_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-shape-approval:stale-pre-activation-soak-evidence:$dry_run_ledger_report_sha256"
)"
workspace_path_shape_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-shape-approval:workspace-path:$dry_run_ledger_report_sha256"
)"
public_artifact_shape_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-shape-approval:public-artifact:$dry_run_ledger_report_sha256"
)"

jq -n -e \
  --argjson ledger "$DRY_RUN_LEDGER_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $ledger.runtime == "hepta"
    and $ledger.status == "ready"
    and $ledger.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_gate"
    and $ledger.payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_ready == true
    and $ledger.payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_ready == true
    and $ledger.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256 != ""
    and $ledger.minimum_required_samples >= 24
    and $ledger.source_denial_fixture_count == 4
    and $ledger.required_dry_run_ledger_entry_count == 4
    and $ledger.dry_run_ledger_entry_count == 4
    and $ledger.dry_run_ledger_entry_hash_count == 4
    and $ledger.denied_dry_run_ledger_entry_count == 4
    and $ledger.allowed_dry_run_ledger_entry_count == 0
    and $ledger.dry_run_receipt_entry_count == 4
    and $ledger.dry_run_receipt_entry_materialized_count == 0
    and $ledger.dry_run_receipt_entry_persisted_count == 0
    and $ledger.dry_run_ledger_recorded == false
    and $ledger.dry_run_ledger_persisted == false
    and $ledger.dry_run_ledger_materialized == false
    and $ledger.dry_run_ledger_filesystem_written == false
    and $ledger.receipt_persistence_allowed_count == 0
    and $ledger.receipt_persistence_execution_performed_count == 0
    and $ledger.filesystem_persistence_execution_performed == false
    and $ledger.filesystem_persistence_execution_performed_count == 0
    and $ledger.filesystem_write_performed == false
    and $ledger.filesystem_write_performed_count == 0
    and $ledger.workspace_write_performed == false
    and $ledger.workspace_write_performed_count == 0
    and $ledger.command_invocation_performed_count == 0
    and $ledger.command_execution_performed_count == 0
    and $ledger.materialization_execution_performed_count == 0
    and $ledger.receipt_materialized_count == 0
    and $ledger.receipt_persisted_count == 0
    and $ledger.selected_output_path_count == 0
    and $ledger.recorded_output_path_count == 0
    and $ledger.recorded_path_binding_count == 0
    and $ledger.receipt_output_path_selected == false
    and $ledger.receipt_output_path_recorded == false
    and $ledger.future_persistence_approval_slot_count == 4
    and $ledger.explicit_persistence_approval_id_present_count == 3
    and $ledger.explicit_persistence_approval_id_missing_count == 1
    and $ledger.stale_or_missing_fresh_pre_activation_soak_evidence_fixture_count == 1
    and $ledger.active_binary_sha_bound_fixture_count == 0
    and $ledger.trusted_source_bound_fixture_count == 0
    and $ledger.operator_scope_bound_fixture_count == 0
    and $ledger.accepted_redaction_proof_bound_fixture_count == 0
    and $ledger.future_active_binary_sha_bound_fixture_count == 4
    and $ledger.future_trusted_source_bound_fixture_count == 4
    and $ledger.future_operator_scope_bound_fixture_count == 3
    and $ledger.future_accepted_redaction_proof_bound_fixture_count == 3
    and $ledger.public_claim_allowed == false
    and $ledger.release_artifact_write_allowed == false
    and $ledger.raw_payload_plaintext_recorded == false
    and $ledger.raw_payload_plaintext_persisted == false
    and $ledger.raw_payload_inspected == false
    and $ledger.live_secret_scan_performed == false
    and $ledger.receipt_persistence_enabled == false
    and $ledger.activation_blocked_by_dry_run_ledger == true
    and $ledger.activation_allowed_by_dry_run_ledger == false
    and $ledger.activation_allowed == false
    and $ledger.live_mutation_execution_ready == false
    and ($ledger.dry_run_ledger_entries | length) == 4
    and ($ledger.dry_run_ledger_entries | all(.denial_status == "blocked" and .filesystem_persistence_allowed == false and .command_invocation_performed == false and .command_execution_performed == false and .materialization_execution_performed == false and .filesystem_persistence_execution_performed == false and .filesystem_write_performed == false and .workspace_write_performed == false and .receipt_materialized == false and .receipt_persisted == false and .activation_allowed == false))
    and ($ledger.required_before_any_dry_run_ledger_persistence | length) == 14
    and ($ledger.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_gate" \
  --arg dry_run_ledger_report_sha256 "$dry_run_ledger_report_sha256" \
  --arg missing_approval_shape_sha256 "$missing_approval_shape_sha256" \
  --arg stale_soak_shape_sha256 "$stale_soak_shape_sha256" \
  --arg workspace_path_shape_sha256 "$workspace_path_shape_sha256" \
  --arg public_artifact_shape_sha256 "$public_artifact_shape_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson ledger "$DRY_RUN_LEDGER_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    ledger_schema_version:"payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_v1",
    source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_gate:$ledger.gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_ready:$ledger.payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_ready,
    source_receipt_payload_sha256:$ledger.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$ledger.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$ledger.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$ledger.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$ledger.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$ledger.source_no_secret_payload_review_report_sha256,
    source_payload_redaction_proof_report_sha256:$ledger.source_payload_redaction_proof_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$ledger.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_acceptance_receipt_command_contract_report_sha256:$ledger.source_payload_redaction_acceptance_receipt_command_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256:$ledger.source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256:$ledger.source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256:$ledger.source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256,
    source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256:$ledger.source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256:$ledger.source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256:$ledger.source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_report_sha256:$ledger.source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_report_sha256:$ledger.source_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256:$ledger.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256:$dry_run_ledger_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_ready:true,
    payload_redaction_acceptance_receipt_filesystem_sink_write_preview_ready:true,
    payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_ready:true,
    payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_ready:true,
    required_ledger_shape_field_count:16,
    ledger_shape_field_count:16,
    recorded_ledger_shape_field_count:0,
    redacted_or_hashed_ledger_shape_field_count:15,
    required_before_any_ledger_persistence_count:16,
    source_dry_run_ledger_entry_count:4,
    source_dry_run_ledger_entry_hash_count:4,
    source_denied_dry_run_ledger_entry_count:4,
    source_allowed_dry_run_ledger_entry_count:0,
    required_ledger_shape_fixture_count:4,
    ledger_shape_fixture_count:4,
    blocked_ledger_shape_fixture_count:4,
    allowed_ledger_shape_fixture_count:0,
    ledger_shape_approval_requested_count:4,
    ledger_shape_approval_performed_count:0,
    ledger_shape_approval_recorded:false,
    ledger_shape_approval_persisted:false,
    ledger_shape_materialized:false,
    ledger_shape_filesystem_written:false,
    dry_run_ledger_recorded:false,
    dry_run_ledger_persisted:false,
    dry_run_ledger_materialized:false,
    dry_run_ledger_filesystem_written:false,
    ledger_persistence_allowed:false,
    ledger_persistence_allowed_count:0,
    ledger_persistence_execution_requested_count:0,
    ledger_persistence_execution_performed:false,
    ledger_persistence_execution_performed_count:0,
    receipt_persistence_allowed_count:0,
    receipt_persistence_execution_performed_count:0,
    filesystem_persistence_execution_performed:false,
    filesystem_persistence_execution_performed_count:0,
    filesystem_write_performed:false,
    filesystem_write_performed_count:0,
    workspace_write_performed:false,
    workspace_write_performed_count:0,
    command_invocation_performed_count:0,
    command_execution_performed_count:0,
    materialization_execution_performed_count:0,
    receipt_materialized_count:0,
    receipt_persisted_count:0,
    selected_output_path_count:0,
    recorded_output_path_count:0,
    recorded_path_binding_count:0,
    active_binary_sha_bound_count:0,
    trusted_source_bound_count:0,
    operator_scope_bound_count:0,
    accepted_redaction_proof_bound_count:0,
    future_active_binary_sha_bound_fixture_count:4,
    future_trusted_source_bound_fixture_count:4,
    future_operator_scope_bound_fixture_count:3,
    future_accepted_redaction_proof_bound_fixture_count:3,
    source_tree_path_attempt_fixture_count:1,
    workspace_path_attempt_fixture_count:1,
    public_claim_attempt_fixture_count:1,
    release_artifact_write_attempt_fixture_count:1,
    public_claim_allowed:false,
    release_artifact_write_allowed:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    raw_payload_inspected:false,
    live_secret_scan_performed:false,
    receipt_persistence_enabled:false,
    activation_blocked_by_ledger_shape_approval:true,
    activation_allowed_by_ledger_shape_approval:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    ledger_shape_denial_reason:"ledger shape approval is schema-only and report-only; it defines required future fields while refusing approval recording, ledger persistence, receipt persistence, filesystem persistence execution, filesystem writes, public artifacts, and live mutation",
    required_ledger_shape_fields:[
      "ledger_shape_approval_id",
      "ledger_schema_version",
      "source_dry_run_ledger_gate_report_sha256",
      "source_execution_denial_matrix_report_sha256",
      "dry_run_ledger_entry_hashes",
      "filesystem_persistence_approval_ids",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "fresh_pre_activation_soak_evidence_id",
      "active_binary_sha256",
      "trusted_source_binding",
      "accepted_redaction_proof_ids",
      "redacted_receipt_output_path",
      "rollback_plan_id",
      "public_claim_and_artifact_decision"
    ],
    required_before_any_ledger_persistence:[
      "explicit_operator_enablement_for_ledger_persistence",
      "ledger_shape_approval_id",
      "filesystem_persistence_approval_id",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "accepted_redaction_proof_ids",
      "fresh_pre_activation_soak_evidence",
      "active_binary_sha256",
      "trusted_source_binding",
      "receipt_payload_hash",
      "redacted_payload_summary_sha256",
      "receipt_output_path_redacted",
      "rollback_plan_id",
      "public_claim_and_artifact_decision",
      "rollback_rehearsal_evidence_id"
    ],
    ledger_shape_approval_fixtures:[
      {
        id:"missing-persistence-approval-id-ledger-shape-approval",
        source_dry_run_ledger_entry_id:"missing-persistence-approval-id-dry-run-ledger-entry",
        deterministic_ledger_shape_sha256:$missing_approval_shape_sha256,
        approval_status:"blocked",
        ledger_shape_approval_requested:true,
        ledger_shape_approval_performed:false,
        ledger_shape_approval_recorded:false,
        ledger_shape_approval_persisted:false,
        ledger_persistence_allowed:false,
        ledger_persistence_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        activation_allowed:false,
        denial_reasons:["persistence_approval_id_missing","ledger_shape_approval_recording_denied","ledger_persistence_denied","filesystem_persistence_execution_denied","filesystem_write_denied","live_mutation_execution_denied"]
      },
      {
        id:"stale-pre-activation-soak-evidence-ledger-shape-approval",
        source_dry_run_ledger_entry_id:"stale-pre-activation-soak-evidence-dry-run-ledger-entry",
        deterministic_ledger_shape_sha256:$stale_soak_shape_sha256,
        approval_status:"blocked",
        ledger_shape_approval_requested:true,
        ledger_shape_approval_performed:false,
        ledger_shape_approval_recorded:false,
        ledger_shape_approval_persisted:false,
        ledger_persistence_allowed:false,
        ledger_persistence_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        activation_allowed:false,
        denial_reasons:["fresh_pre_activation_soak_evidence_missing_or_stale","ledger_shape_approval_recording_denied","ledger_persistence_denied","filesystem_persistence_execution_denied","filesystem_write_denied","live_mutation_execution_denied"]
      },
      {
        id:"workspace-path-ledger-shape-approval",
        source_dry_run_ledger_entry_id:"workspace-path-dry-run-ledger-entry",
        deterministic_ledger_shape_sha256:$workspace_path_shape_sha256,
        approval_status:"blocked",
        ledger_shape_approval_requested:true,
        ledger_shape_approval_performed:false,
        ledger_shape_approval_recorded:false,
        ledger_shape_approval_persisted:false,
        ledger_persistence_allowed:false,
        ledger_persistence_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        activation_allowed:false,
        denial_reasons:["source_tree_workspace_output_path_denied","ledger_shape_approval_recording_denied","ledger_persistence_denied","filesystem_persistence_execution_denied","filesystem_write_denied","workspace_write_denied","live_mutation_execution_denied"]
      },
      {
        id:"public-artifact-ledger-shape-approval",
        source_dry_run_ledger_entry_id:"public-artifact-dry-run-ledger-entry",
        deterministic_ledger_shape_sha256:$public_artifact_shape_sha256,
        approval_status:"blocked",
        ledger_shape_approval_requested:true,
        ledger_shape_approval_performed:false,
        ledger_shape_approval_recorded:false,
        ledger_shape_approval_persisted:false,
        ledger_persistence_allowed:false,
        ledger_persistence_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        activation_allowed:false,
        denial_reasons:["public_artifact_release_output_path_denied","public_release_claim_denied","release_artifact_write_denied","ledger_shape_approval_recording_denied","ledger_persistence_denied","filesystem_persistence_execution_denied","filesystem_write_denied","live_mutation_execution_denied"]
      }
    ],
    denied_by_ledger_shape_approval:[
      "persistence_approval_id_missing",
      "fresh_pre_activation_soak_evidence_missing_or_stale",
      "source_tree_workspace_output_path_denied",
      "public_artifact_release_output_path_denied",
      "ledger_shape_approval_recording_denied",
      "ledger_persistence_execution_denied",
      "receipt_persistence_denied",
      "filesystem_persistence_execution_denied",
      "filesystem_write_denied",
      "workspace_write_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "live_mutation_execution_denied"
    ],
    side_effects:{
      memory_store_mutated:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      coding_agent_spawned:false,
      skill_workshop_written:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      command_invocation_performed:false,
      command_execution_performed:false,
      receipt_persistence_execution_performed:false,
      materialization_execution_performed:false,
      filesystem_persistence_execution_performed:false,
      ledger_shape_approval_recorded:false,
      ledger_shape_approval_persisted:false,
      ledger_persistence_execution_performed:false,
      ledger_persisted:false,
      ledger_filesystem_written:false,
      filesystem_written:false,
      workspace_write_performed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      receipt_materialized:false,
      receipt_persisted:false,
      dry_run_ledger_recorded:false,
      dry_run_ledger_materialized:false,
      dry_run_ledger_persisted:false,
      dry_run_ledger_filesystem_written:false,
      filesystem_persistence_approval_packet_persisted:false,
      output_path_allowlist_persisted:false,
      output_path_evidence_binding_persisted:false,
      filesystem_sink_write_preview_persisted:false,
      execution_denial_matrix_persisted:false,
      output_path_selected:false,
      output_path_binding_selected:false,
      materialization_plan_persisted:false,
      pre_activation_soak_evidence_persisted:false,
      approval_packet_persisted:false,
      operator_scope_binding_persisted:false,
      payload_review_persisted:false,
      payload_redaction_proof_persisted:false,
      payload_redaction_acceptance_matrix_persisted:false,
      payload_redaction_acceptance_receipt_command_persisted:false,
      payload_redaction_acceptance_receipt_no_write_sink_persisted:false,
      payload_redaction_acceptance_receipt_write_enable_fixture_persisted:false,
      payload_redaction_acceptance_receipt_materialization_plan_persisted:false,
      payload_redaction_acceptance_receipt_persisted:false,
      payload_plaintext_persisted:false,
      raw_payload_inspected:false,
      live_secret_scan_performed:false,
      external_send_performed:false,
      credential_read:false,
      secret_file_read:false
    }
  }')"

jq -e '
  . as $root
  | .status == "ready"
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_ledger_shape_field_count == 16
  and .ledger_shape_field_count == 16
  and .recorded_ledger_shape_field_count == 0
  and .redacted_or_hashed_ledger_shape_field_count == 15
  and .required_before_any_ledger_persistence_count == 16
  and .source_dry_run_ledger_entry_count == 4
  and .source_dry_run_ledger_entry_hash_count == 4
  and .source_denied_dry_run_ledger_entry_count == 4
  and .source_allowed_dry_run_ledger_entry_count == 0
  and .required_ledger_shape_fixture_count == 4
  and .ledger_shape_fixture_count == 4
  and .blocked_ledger_shape_fixture_count == 4
  and .allowed_ledger_shape_fixture_count == 0
  and .ledger_shape_approval_requested_count == 4
  and .ledger_shape_approval_performed_count == 0
  and .ledger_shape_approval_recorded == false
  and .ledger_shape_approval_persisted == false
  and .ledger_shape_materialized == false
  and .ledger_shape_filesystem_written == false
  and .dry_run_ledger_recorded == false
  and .dry_run_ledger_persisted == false
  and .dry_run_ledger_materialized == false
  and .dry_run_ledger_filesystem_written == false
  and .ledger_persistence_allowed == false
  and .ledger_persistence_allowed_count == 0
  and .ledger_persistence_execution_requested_count == 0
  and .ledger_persistence_execution_performed == false
  and .ledger_persistence_execution_performed_count == 0
  and .receipt_persistence_allowed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .filesystem_persistence_execution_performed == false
  and .filesystem_persistence_execution_performed_count == 0
  and .filesystem_write_performed == false
  and .filesystem_write_performed_count == 0
  and .workspace_write_performed == false
  and .workspace_write_performed_count == 0
  and .command_invocation_performed_count == 0
  and .command_execution_performed_count == 0
  and .materialization_execution_performed_count == 0
  and .receipt_materialized_count == 0
  and .receipt_persisted_count == 0
  and .selected_output_path_count == 0
  and .recorded_output_path_count == 0
  and .recorded_path_binding_count == 0
  and .active_binary_sha_bound_count == 0
  and .trusted_source_bound_count == 0
  and .operator_scope_bound_count == 0
  and .accepted_redaction_proof_bound_count == 0
  and .future_active_binary_sha_bound_fixture_count == 4
  and .future_trusted_source_bound_fixture_count == 4
  and .future_operator_scope_bound_fixture_count == 3
  and .future_accepted_redaction_proof_bound_fixture_count == 3
  and .source_tree_path_attempt_fixture_count == 1
  and .workspace_path_attempt_fixture_count == 1
  and .public_claim_attempt_fixture_count == 1
  and .release_artifact_write_attempt_fixture_count == 1
  and .public_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .raw_payload_inspected == false
  and .live_secret_scan_performed == false
  and .receipt_persistence_enabled == false
  and .activation_blocked_by_ledger_shape_approval == true
  and .activation_allowed_by_ledger_shape_approval == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.required_ledger_shape_fields | length) == 16
  and (.required_before_any_ledger_persistence | length) == 16
  and (.ledger_shape_approval_fixtures | length) == 4
  and (.ledger_shape_approval_fixtures | all(.approval_status == "blocked" and .ledger_shape_approval_requested == true and .ledger_shape_approval_performed == false and .ledger_shape_approval_recorded == false and .ledger_shape_approval_persisted == false and .ledger_persistence_allowed == false and .ledger_persistence_execution_performed == false and .receipt_persistence_execution_performed == false and .filesystem_persistence_execution_performed == false and .filesystem_write_performed == false and .workspace_write_performed == false and .activation_allowed == false and .deterministic_ledger_shape_sha256 != ""))
  and (.denied_by_ledger_shape_approval | length) == 13
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger shape approval gate passed"
