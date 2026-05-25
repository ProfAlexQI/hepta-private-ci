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

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

OUTPUT_PATH_ALLOWLIST_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-output-path-allowlist-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-output-path-allowlist-gate.sh
)"

output_path_allowlist_report_sha256="$(printf '%s' "$OUTPUT_PATH_ALLOWLIST_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson allowlist "$OUTPUT_PATH_ALLOWLIST_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $allowlist.runtime == "hepta"
    and $allowlist.status == "ready"
    and $allowlist.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_gate"
    and $allowlist.payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_ready == true
    and $allowlist.payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_ready == true
    and $allowlist.source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256 != ""
    and $allowlist.minimum_required_samples >= 24
    and $allowlist.required_allowlist_entry_count == 6
    and $allowlist.allowlist_entry_count == 6
    and $allowlist.allowed_output_path_entry_count == 3
    and $allowlist.blocked_output_path_entry_count == 3
    and $allowlist.redacted_output_path_entry_count == 6
    and $allowlist.default_selected_output_path_count == 0
    and $allowlist.selected_output_path_count == 0
    and $allowlist.recorded_output_path_count == 0
    and $allowlist.source_tree_path_allowed == false
    and $allowlist.home_directory_path_allowed == false
    and $allowlist.release_artifact_path_allowed == false
    and $allowlist.public_artifact_path_allowed == false
    and $allowlist.receipt_output_path_allowlist_ready == true
    and $allowlist.receipt_output_path_selected == false
    and $allowlist.receipt_output_path_recorded == false
    and $allowlist.filesystem_persistence_allowed == false
    and $allowlist.filesystem_persistence_allowed_count == 0
    and $allowlist.command_invocation_performed_count == 0
    and $allowlist.command_execution_performed_count == 0
    and $allowlist.receipt_persistence_execution_performed_count == 0
    and $allowlist.materialization_execution_performed_count == 0
    and $allowlist.filesystem_persistence_execution_performed == false
    and $allowlist.filesystem_persistence_execution_performed_count == 0
    and $allowlist.filesystem_write_performed == false
    and $allowlist.filesystem_write_performed_count == 0
    and $allowlist.workspace_write_performed == false
    and $allowlist.workspace_write_performed_count == 0
    and $allowlist.receipt_materialized_count == 0
    and $allowlist.receipt_persisted_count == 0
    and $allowlist.raw_payload_plaintext_recorded == false
    and $allowlist.raw_payload_plaintext_persisted == false
    and $allowlist.live_secret_scan_performed == false
    and $allowlist.receipt_persistence_enabled == false
    and $allowlist.activation_allowed == false
    and $allowlist.live_mutation_execution_ready == false
    and ($allowlist.allowlist_entries | length) == 6
    and ($allowlist.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_gate" \
  --arg output_path_allowlist_report_sha256 "$output_path_allowlist_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson allowlist "$OUTPUT_PATH_ALLOWLIST_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_gate:$allowlist.gate,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_ready:$allowlist.payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_ready,
    source_receipt_payload_sha256:$allowlist.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$allowlist.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$allowlist.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$allowlist.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$allowlist.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$allowlist.source_no_secret_payload_review_report_sha256,
    source_payload_redaction_proof_report_sha256:$allowlist.source_payload_redaction_proof_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$allowlist.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_acceptance_receipt_command_contract_report_sha256:$allowlist.source_payload_redaction_acceptance_receipt_command_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256:$allowlist.source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256:$allowlist.source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256:$allowlist.source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256,
    source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256:$allowlist.source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256:$allowlist.source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256:$output_path_allowlist_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_ready:true,
    payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_ready:true,
    payload_redaction_acceptance_receipt_materialization_dry_run_ready:true,
    payload_redaction_acceptance_receipt_write_enable_fixture_ready:true,
    payload_redaction_acceptance_receipt_no_write_sink_contract_ready:true,
    payload_redaction_acceptance_receipt_invocation_dry_run_ready:true,
    payload_redaction_acceptance_receipt_command_contract_ready:true,
    payload_redaction_acceptance_receipt_command_recorded:false,
    payload_redaction_acceptance_receipt_command_enabled_by_default:false,
    payload_redaction_acceptance_receipt_command_invocation_requested_count:5,
    payload_redaction_acceptance_receipt_command_invocation_performed_count:0,
    payload_redaction_acceptance_receipt_command_execution_performed_count:0,
    payload_redaction_acceptance_receipt_recorded:false,
    payload_redaction_acceptance_receipt_materialized:false,
    payload_redaction_acceptance_receipt_persisted:false,
    filesystem_persistence_approval_packet_recorded:false,
    filesystem_persistence_approval_packet_persisted:false,
    output_path_allowlist_recorded:false,
    output_path_allowlist_persisted:false,
    output_path_evidence_binding_recorded:false,
    output_path_evidence_binding_persisted:false,
    payload_redaction_acceptance_matrix_recorded:false,
    payload_redaction_acceptance_matrix_persisted:false,
    payload_redaction_proof_recorded:false,
    payload_redaction_proof_accepted:false,
    accepted_redaction_proof_count:0,
    reviewed_redaction_proof_count:0,
    required_path_binding_count:11,
    path_binding_count:11,
    required_allowlist_entry_count:6,
    allowlist_entry_count:6,
    allowed_output_path_entry_count:3,
    blocked_output_path_entry_count:3,
    redacted_output_path_entry_count:6,
    default_selected_output_path_count:0,
    selected_output_path_count:0,
    recorded_output_path_count:0,
    recorded_path_binding_count:0,
    eligible_report_only_root_count:3,
    blocked_mutating_root_count:3,
    fresh_pre_activation_soak_evidence_bound_count:0,
    active_binary_sha_bound_count:0,
    operator_scope_bound_count:0,
    accepted_redaction_proof_bound_count:0,
    redacted_or_hashed_binding_count:11,
    trusted_source_bound_count:0,
    source_tree_path_binding_allowed:false,
    home_directory_path_binding_allowed:false,
    release_artifact_path_binding_allowed:false,
    public_artifact_path_binding_allowed:false,
    source_tree_path_allowed:false,
    home_directory_path_allowed:false,
    release_artifact_path_allowed:false,
    public_artifact_path_allowed:false,
    receipt_output_path_allowlist_ready:true,
    receipt_output_path_selected:false,
    receipt_output_path_recorded:false,
    receipt_output_path_evidence_binding_ready:true,
    operator_approval_required:true,
    operator_approval_recorded:false,
    operator_identity_hash_required:true,
    operator_identity_hash_recorded:false,
    single_surface_activation_scope_required:true,
    single_surface_activation_scope_recorded:false,
    receipt_payload_hash_required:true,
    receipt_payload_hash_recorded:false,
    redacted_payload_summary_hash_required:true,
    redacted_payload_summary_hash_recorded:false,
    receipt_output_path_redacted_required:true,
    receipt_output_path_redacted_recorded:false,
    accepted_redaction_proof_ids_required:true,
    accepted_redaction_proof_ids_recorded:false,
    fresh_pre_activation_soak_evidence_required:true,
    fresh_pre_activation_soak_evidence_recorded:false,
    active_binary_sha_required:true,
    active_binary_sha_recorded:false,
    rollback_plan_required:true,
    rollback_plan_recorded:false,
    public_artifact_policy_required:true,
    public_artifact_policy_recorded:false,
    plaintext_payload_attempt_count:1,
    public_claim_attempt_count:1,
    release_artifact_write_attempt_count:1,
    filesystem_persistence_allowed:false,
    filesystem_persistence_allowed_count:0,
    command_invocation_attempt_count:5,
    command_invocation_performed_count:0,
    command_execution_performed_count:0,
    receipt_persistence_execution_performed_count:0,
    materialization_execution_performed_count:0,
    materialization_executed_count:0,
    filesystem_persistence_execution_performed:false,
    filesystem_persistence_execution_performed_count:0,
    filesystem_write_performed:false,
    filesystem_write_performed_count:0,
    workspace_write_performed:false,
    workspace_write_performed_count:0,
    receipt_materialized_count:0,
    receipt_persisted_count:0,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    live_secret_scan_performed:false,
    pre_activation_soak_evidence_persistence_allowed:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    activation_blocked_by_output_path_evidence_binding:true,
    activation_allowed_by_output_path_evidence_binding:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    output_path_evidence_binding_denial_reason:"receipt output path binding is schema-only; no output path is selected, no fresh evidence or active binary SHA is recorded, and filesystem persistence remains disabled",
    evidence_bindings:[
      "activation_request_id",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "receipt_payload_hash",
      "redacted_payload_summary_hash",
      "accepted_redaction_proof_ids",
      "fresh_pre_activation_soak_evidence",
      "active_binary_sha256",
      "rollback_plan_id",
      "public_artifact_policy"
    ],
    allowed_output_path_bindings:[
      "payload_redaction_acceptance_receipts_root",
      "payload_redaction_acceptance_receipt_dry_run_root",
      "payload_redaction_acceptance_receipt_operator_packet_root"
    ],
    denied_by_output_path_evidence_binding:[
      "source_tree_output_path_binding_denied",
      "home_directory_output_path_binding_denied",
      "release_artifact_output_path_binding_denied",
      "public_artifact_output_path_binding_denied",
      "missing_fresh_pre_activation_soak_evidence_denied",
      "missing_active_binary_sha_denied",
      "missing_operator_scope_denied",
      "missing_accepted_redaction_proof_denied",
      "default_output_path_selection_denied",
      "filesystem_persistence_denied",
      "filesystem_write_denied",
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
      filesystem_written:false,
      workspace_write_performed:false,
      release_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      receipt_materialized:false,
      receipt_persisted:false,
      filesystem_persistence_approval_packet_persisted:false,
      output_path_allowlist_persisted:false,
      output_path_evidence_binding_persisted:false,
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
  .status == "ready"
  and .payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_path_binding_count == 11
  and .path_binding_count == 11
  and .required_allowlist_entry_count == 6
  and .allowlist_entry_count == 6
  and .allowed_output_path_entry_count == 3
  and .blocked_output_path_entry_count == 3
  and .redacted_output_path_entry_count == 6
  and .default_selected_output_path_count == 0
  and .selected_output_path_count == 0
  and .recorded_output_path_count == 0
  and .recorded_path_binding_count == 0
  and .fresh_pre_activation_soak_evidence_bound_count == 0
  and .active_binary_sha_bound_count == 0
  and .operator_scope_bound_count == 0
  and .accepted_redaction_proof_bound_count == 0
  and .redacted_or_hashed_binding_count == 11
  and .trusted_source_bound_count == 0
  and .source_tree_path_binding_allowed == false
  and .home_directory_path_binding_allowed == false
  and .release_artifact_path_binding_allowed == false
  and .public_artifact_path_binding_allowed == false
  and .receipt_output_path_evidence_binding_ready == true
  and .filesystem_persistence_allowed == false
  and .filesystem_persistence_allowed_count == 0
  and .command_invocation_attempt_count == 5
  and .command_invocation_performed_count == 0
  and .command_execution_performed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .materialization_execution_performed_count == 0
  and .filesystem_persistence_execution_performed == false
  and .filesystem_persistence_execution_performed_count == 0
  and .filesystem_write_performed == false
  and .filesystem_write_performed_count == 0
  and .workspace_write_performed == false
  and .workspace_write_performed_count == 0
  and .receipt_materialized_count == 0
  and .receipt_persisted_count == 0
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .live_secret_scan_performed == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .activation_blocked_by_output_path_evidence_binding == true
  and .activation_allowed_by_output_path_evidence_binding == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.evidence_bindings | length) == 11
  and (.allowed_output_path_bindings | length) == 3
  and (.denied_by_output_path_evidence_binding | length) == 12
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem output path evidence binding gate passed"
