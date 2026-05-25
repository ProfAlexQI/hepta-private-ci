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

EXECUTION_DENIAL_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-execution-denial-matrix-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-execution-denial-matrix-gate.sh
)"

execution_denial_matrix_report_sha256="$(
  printf '%s' "$EXECUTION_DENIAL_JSON" | shasum -a 256 | awk '{print $1}'
)"

missing_approval_ledger_entry_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-dry-run-ledger:missing-persistence-approval-id:$execution_denial_matrix_report_sha256"
)"
stale_soak_ledger_entry_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-dry-run-ledger:stale-pre-activation-soak-evidence:$execution_denial_matrix_report_sha256"
)"
workspace_path_ledger_entry_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-dry-run-ledger:workspace-path:$execution_denial_matrix_report_sha256"
)"
public_artifact_ledger_entry_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-dry-run-ledger:public-artifact:$execution_denial_matrix_report_sha256"
)"

jq -n -e \
  --argjson denial "$EXECUTION_DENIAL_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $denial.runtime == "hepta"
    and $denial.status == "ready"
    and $denial.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_gate"
    and $denial.payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_ready == true
    and $denial.payload_redaction_acceptance_receipt_filesystem_sink_write_preview_ready == true
    and $denial.source_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_report_sha256 != ""
    and $denial.minimum_required_samples >= 24
    and $denial.source_preview_fixture_count == 3
    and $denial.required_denial_fixture_count == 4
    and $denial.denial_fixture_count == 4
    and $denial.execution_requested_fixture_count == 4
    and $denial.future_persistence_approval_slot_count == 4
    and $denial.explicit_persistence_approval_id_present_count == 3
    and $denial.explicit_persistence_approval_id_missing_count == 1
    and $denial.stale_or_missing_fresh_pre_activation_soak_evidence_fixture_count == 1
    and $denial.stale_or_missing_fresh_evidence_fixture_count == 1
    and $denial.future_active_binary_sha_bound_fixture_count == 4
    and $denial.future_trusted_source_bound_fixture_count == 4
    and $denial.future_operator_scope_bound_fixture_count == 3
    and $denial.future_accepted_redaction_proof_bound_fixture_count == 3
    and $denial.active_binary_sha_bound_fixture_count == 0
    and $denial.trusted_source_bound_fixture_count == 0
    and $denial.operator_scope_bound_fixture_count == 0
    and $denial.accepted_redaction_proof_bound_fixture_count == 0
    and $denial.blocked_execution_fixture_count == 4
    and $denial.allowed_execution_fixture_count == 0
    and $denial.filesystem_persistence_allowed == false
    and $denial.filesystem_persistence_allowed_count == 0
    and $denial.command_invocation_performed_count == 0
    and $denial.command_execution_performed_count == 0
    and $denial.receipt_persistence_execution_performed_count == 0
    and $denial.materialization_execution_performed_count == 0
    and $denial.filesystem_persistence_execution_performed == false
    and $denial.filesystem_persistence_execution_performed_count == 0
    and $denial.filesystem_write_requested_count == 4
    and $denial.filesystem_write_performed == false
    and $denial.filesystem_write_performed_count == 0
    and $denial.workspace_write_performed == false
    and $denial.workspace_write_performed_count == 0
    and $denial.receipt_materialized_count == 0
    and $denial.receipt_persisted_count == 0
    and $denial.raw_payload_plaintext_recorded == false
    and $denial.raw_payload_plaintext_persisted == false
    and $denial.live_secret_scan_performed == false
    and $denial.receipt_persistence_enabled == false
    and $denial.activation_blocked_by_execution_denial_matrix == true
    and $denial.activation_allowed_by_execution_denial_matrix == false
    and $denial.activation_allowed == false
    and $denial.live_mutation_execution_ready == false
    and ($denial.denial_fixtures | length) == 4
    and ($denial.denial_fixtures | all(.execution_requested == true and .validation_status == "blocked" and .filesystem_persistence_allowed == false and .command_invocation_performed == false and .command_execution_performed == false and .materialization_execution_performed == false and .filesystem_persistence_execution_performed == false and .filesystem_write_requested == true and .filesystem_write_performed == false and .workspace_write_performed == false and .receipt_persisted == false and .activation_allowed == false))
    and ($denial.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_gate" \
  --arg execution_denial_matrix_report_sha256 "$execution_denial_matrix_report_sha256" \
  --arg missing_approval_ledger_entry_sha256 "$missing_approval_ledger_entry_sha256" \
  --arg stale_soak_ledger_entry_sha256 "$stale_soak_ledger_entry_sha256" \
  --arg workspace_path_ledger_entry_sha256 "$workspace_path_ledger_entry_sha256" \
  --arg public_artifact_ledger_entry_sha256 "$public_artifact_ledger_entry_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson denial "$EXECUTION_DENIAL_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_gate:$denial.gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_ready:$denial.payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_ready,
    source_receipt_payload_sha256:$denial.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$denial.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$denial.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$denial.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$denial.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$denial.source_no_secret_payload_review_report_sha256,
    source_payload_redaction_proof_report_sha256:$denial.source_payload_redaction_proof_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$denial.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_acceptance_receipt_command_contract_report_sha256:$denial.source_payload_redaction_acceptance_receipt_command_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256:$denial.source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256:$denial.source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256:$denial.source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256,
    source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256:$denial.source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256:$denial.source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256:$denial.source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_report_sha256:$denial.source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_report_sha256:$denial.source_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256:$execution_denial_matrix_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_ready:true,
    payload_redaction_acceptance_receipt_filesystem_sink_write_preview_ready:true,
    payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_ready:true,
    payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_ready:true,
    payload_redaction_acceptance_receipt_materialization_dry_run_ready:true,
    payload_redaction_acceptance_receipt_write_enable_fixture_ready:true,
    payload_redaction_acceptance_receipt_no_write_sink_contract_ready:true,
    payload_redaction_acceptance_receipt_invocation_dry_run_ready:true,
    payload_redaction_acceptance_receipt_command_contract_ready:true,
    source_denial_fixture_count:4,
    required_dry_run_ledger_entry_count:4,
    dry_run_ledger_entry_count:4,
    dry_run_ledger_entry_hash_count:4,
    denied_dry_run_ledger_entry_count:4,
    allowed_dry_run_ledger_entry_count:0,
    dry_run_receipt_entry_count:4,
    dry_run_receipt_entry_materialized_count:0,
    dry_run_receipt_entry_persisted_count:0,
    dry_run_ledger_recorded:false,
    dry_run_ledger_persisted:false,
    dry_run_ledger_materialized:false,
    dry_run_ledger_filesystem_written:false,
    receipt_persistence_requested_count:4,
    receipt_persistence_allowed_count:0,
    receipt_persistence_execution_performed_count:0,
    filesystem_persistence_execution_requested_count:4,
    filesystem_persistence_execution_performed:false,
    filesystem_persistence_execution_performed_count:0,
    filesystem_write_requested_count:4,
    filesystem_write_performed:false,
    filesystem_write_performed_count:0,
    workspace_write_performed:false,
    workspace_write_performed_count:0,
    command_invocation_requested_count:4,
    command_invocation_performed_count:0,
    command_execution_performed_count:0,
    materialization_execution_requested_count:4,
    materialization_execution_performed_count:0,
    materialization_executed_count:0,
    receipt_materialized_count:0,
    receipt_persisted_count:0,
    selected_output_path_count:0,
    recorded_output_path_count:0,
    recorded_path_binding_count:0,
    receipt_output_path_selected:false,
    receipt_output_path_recorded:false,
    future_persistence_approval_slot_count:4,
    explicit_persistence_approval_id_present_count:3,
    explicit_persistence_approval_id_missing_count:1,
    stale_or_missing_fresh_pre_activation_soak_evidence_fixture_count:1,
    active_binary_sha_bound_fixture_count:0,
    trusted_source_bound_fixture_count:0,
    operator_scope_bound_fixture_count:0,
    accepted_redaction_proof_bound_fixture_count:0,
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
    pre_activation_soak_evidence_persistence_allowed:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    activation_blocked_by_dry_run_ledger:true,
    activation_allowed_by_dry_run_ledger:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    ledger_denial_reason:"dry-run ledger entries are deterministic report-only denial records; no receipt persistence, filesystem persistence execution, filesystem write, workspace write, public artifact write, or live mutation is allowed",
    dry_run_ledger_entries:[
      {
        id:"missing-persistence-approval-id-dry-run-ledger-entry",
        source_denial_fixture_id:"missing-persistence-approval-id-execution-attempt",
        entry_kind:"missing_persistence_approval_id",
        source_execution_denial_matrix_report_sha256:$execution_denial_matrix_report_sha256,
        deterministic_ledger_entry_sha256:$missing_approval_ledger_entry_sha256,
        dry_run_receipt_requested:true,
        receipt_persistence_requested:true,
        filesystem_persistence_execution_requested:true,
        command_invocation_requested:true,
        materialization_execution_requested:true,
        denial_status:"blocked",
        denial_reasons:["persistence_approval_id_missing","command_invocation_denied","materialization_execution_denied","filesystem_persistence_execution_denied","filesystem_write_denied","receipt_persistence_denied","live_mutation_execution_denied"],
        filesystem_persistence_allowed:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false
      },
      {
        id:"stale-pre-activation-soak-evidence-dry-run-ledger-entry",
        source_denial_fixture_id:"stale-pre-activation-soak-evidence-execution-attempt",
        entry_kind:"stale_pre_activation_soak_evidence",
        source_execution_denial_matrix_report_sha256:$execution_denial_matrix_report_sha256,
        deterministic_ledger_entry_sha256:$stale_soak_ledger_entry_sha256,
        dry_run_receipt_requested:true,
        receipt_persistence_requested:true,
        filesystem_persistence_execution_requested:true,
        command_invocation_requested:true,
        materialization_execution_requested:true,
        denial_status:"blocked",
        denial_reasons:["fresh_pre_activation_soak_evidence_missing_or_stale","command_invocation_denied","materialization_execution_denied","filesystem_persistence_execution_denied","filesystem_write_denied","receipt_persistence_denied","live_mutation_execution_denied"],
        filesystem_persistence_allowed:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false
      },
      {
        id:"workspace-path-dry-run-ledger-entry",
        source_denial_fixture_id:"workspace-path-execution-attempt",
        entry_kind:"workspace_path_execution_attempt",
        source_execution_denial_matrix_report_sha256:$execution_denial_matrix_report_sha256,
        deterministic_ledger_entry_sha256:$workspace_path_ledger_entry_sha256,
        dry_run_receipt_requested:true,
        receipt_persistence_requested:true,
        filesystem_persistence_execution_requested:true,
        command_invocation_requested:true,
        materialization_execution_requested:true,
        denial_status:"blocked",
        denial_reasons:["source_tree_workspace_output_path_denied","command_invocation_denied","materialization_execution_denied","filesystem_persistence_execution_denied","filesystem_write_denied","workspace_write_denied","receipt_persistence_denied","live_mutation_execution_denied"],
        filesystem_persistence_allowed:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false
      },
      {
        id:"public-artifact-dry-run-ledger-entry",
        source_denial_fixture_id:"public-artifact-execution-attempt",
        entry_kind:"public_artifact_execution_attempt",
        source_execution_denial_matrix_report_sha256:$execution_denial_matrix_report_sha256,
        deterministic_ledger_entry_sha256:$public_artifact_ledger_entry_sha256,
        dry_run_receipt_requested:true,
        receipt_persistence_requested:true,
        filesystem_persistence_execution_requested:true,
        command_invocation_requested:true,
        materialization_execution_requested:true,
        denial_status:"blocked",
        denial_reasons:["public_artifact_release_output_path_denied","public_release_claim_denied","release_artifact_write_denied","command_invocation_denied","materialization_execution_denied","filesystem_persistence_execution_denied","filesystem_write_denied","receipt_persistence_denied","live_mutation_execution_denied"],
        filesystem_persistence_allowed:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false
      }
    ],
    denied_by_dry_run_ledger:[
      "persistence_approval_id_missing",
      "fresh_pre_activation_soak_evidence_missing_or_stale",
      "source_tree_workspace_output_path_denied",
      "public_artifact_release_output_path_denied",
      "command_invocation_execution_denied",
      "materialization_execution_denied",
      "filesystem_persistence_execution_denied",
      "filesystem_write_denied",
      "workspace_write_denied",
      "receipt_persistence_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "live_mutation_execution_denied"
    ],
    required_before_any_dry_run_ledger_persistence:[
      "explicit_operator_enablement_for_receipt_persistence",
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
      "public_claim_and_artifact_decision"
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
  and .payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .source_denial_fixture_count == 4
  and .required_dry_run_ledger_entry_count == 4
  and .dry_run_ledger_entry_count == 4
  and .dry_run_ledger_entry_hash_count == 4
  and .denied_dry_run_ledger_entry_count == 4
  and .allowed_dry_run_ledger_entry_count == 0
  and .dry_run_receipt_entry_count == 4
  and .dry_run_receipt_entry_materialized_count == 0
  and .dry_run_receipt_entry_persisted_count == 0
  and .dry_run_ledger_recorded == false
  and .dry_run_ledger_persisted == false
  and .dry_run_ledger_materialized == false
  and .dry_run_ledger_filesystem_written == false
  and .receipt_persistence_requested_count == 4
  and .receipt_persistence_allowed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .filesystem_persistence_execution_requested_count == 4
  and .filesystem_persistence_execution_performed == false
  and .filesystem_persistence_execution_performed_count == 0
  and .filesystem_write_requested_count == 4
  and .filesystem_write_performed == false
  and .filesystem_write_performed_count == 0
  and .workspace_write_performed == false
  and .workspace_write_performed_count == 0
  and .command_invocation_requested_count == 4
  and .command_invocation_performed_count == 0
  and .command_execution_performed_count == 0
  and .materialization_execution_requested_count == 4
  and .materialization_execution_performed_count == 0
  and .receipt_materialized_count == 0
  and .receipt_persisted_count == 0
  and .selected_output_path_count == 0
  and .recorded_output_path_count == 0
  and .recorded_path_binding_count == 0
  and .receipt_output_path_selected == false
  and .receipt_output_path_recorded == false
  and .future_persistence_approval_slot_count == 4
  and .explicit_persistence_approval_id_present_count == 3
  and .explicit_persistence_approval_id_missing_count == 1
  and .stale_or_missing_fresh_pre_activation_soak_evidence_fixture_count == 1
  and .active_binary_sha_bound_fixture_count == 0
  and .trusted_source_bound_fixture_count == 0
  and .operator_scope_bound_fixture_count == 0
  and .accepted_redaction_proof_bound_fixture_count == 0
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
  and .activation_blocked_by_dry_run_ledger == true
  and .activation_allowed_by_dry_run_ledger == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.dry_run_ledger_entries | length) == 4
  and (.dry_run_ledger_entries | all(.dry_run_receipt_requested == true and .receipt_persistence_requested == true and .filesystem_persistence_execution_requested == true and .command_invocation_requested == true and .materialization_execution_requested == true and .denial_status == "blocked" and .filesystem_persistence_allowed == false and .command_invocation_performed == false and .command_execution_performed == false and .materialization_execution_performed == false and .filesystem_persistence_execution_performed == false and .filesystem_write_performed == false and .workspace_write_performed == false and .receipt_materialized == false and .receipt_persisted == false and .activation_allowed == false))
  and (.dry_run_ledger_entries | all(.deterministic_ledger_entry_sha256 != "" and .source_execution_denial_matrix_report_sha256 == $root.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256))
  and (.denied_by_dry_run_ledger | length) == 13
  and (.required_before_any_dry_run_ledger_persistence | length) == 14
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence dry-run ledger gate passed"
