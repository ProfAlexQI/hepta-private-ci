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

OUTPUT_PATH_EVIDENCE_BINDING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-output-path-evidence-binding-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-output-path-evidence-binding-gate.sh
)"

output_path_evidence_binding_report_sha256="$(
  printf '%s' "$OUTPUT_PATH_EVIDENCE_BINDING_JSON" | shasum -a 256 | awk '{print $1}'
)"

receipt_root_preview_payload_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-sink-preview:receipts-root:$output_path_evidence_binding_report_sha256"
)"
dry_run_root_preview_payload_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-sink-preview:dry-run-root:$output_path_evidence_binding_report_sha256"
)"
operator_packet_root_preview_payload_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-sink-preview:operator-packet-root:$output_path_evidence_binding_report_sha256"
)"

jq -n -e \
  --argjson binding "$OUTPUT_PATH_EVIDENCE_BINDING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $binding.runtime == "hepta"
    and $binding.status == "ready"
    and $binding.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_gate"
    and $binding.payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_ready == true
    and $binding.payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_ready == true
    and $binding.source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256 != ""
    and $binding.minimum_required_samples >= 24
    and $binding.required_path_binding_count == 11
    and $binding.path_binding_count == 11
    and $binding.required_allowlist_entry_count == 6
    and $binding.allowlist_entry_count == 6
    and $binding.allowed_output_path_entry_count == 3
    and $binding.blocked_output_path_entry_count == 3
    and $binding.redacted_output_path_entry_count == 6
    and $binding.default_selected_output_path_count == 0
    and $binding.selected_output_path_count == 0
    and $binding.recorded_output_path_count == 0
    and $binding.recorded_path_binding_count == 0
    and $binding.fresh_pre_activation_soak_evidence_bound_count == 0
    and $binding.active_binary_sha_bound_count == 0
    and $binding.operator_scope_bound_count == 0
    and $binding.accepted_redaction_proof_bound_count == 0
    and $binding.redacted_or_hashed_binding_count == 11
    and $binding.trusted_source_bound_count == 0
    and $binding.source_tree_path_binding_allowed == false
    and $binding.home_directory_path_binding_allowed == false
    and $binding.release_artifact_path_binding_allowed == false
    and $binding.public_artifact_path_binding_allowed == false
    and $binding.receipt_output_path_evidence_binding_ready == true
    and $binding.receipt_output_path_selected == false
    and $binding.receipt_output_path_recorded == false
    and $binding.filesystem_persistence_allowed == false
    and $binding.filesystem_persistence_allowed_count == 0
    and $binding.command_invocation_attempt_count == 5
    and $binding.command_invocation_performed_count == 0
    and $binding.command_execution_performed_count == 0
    and $binding.receipt_persistence_execution_performed_count == 0
    and $binding.materialization_execution_performed_count == 0
    and $binding.filesystem_persistence_execution_performed == false
    and $binding.filesystem_persistence_execution_performed_count == 0
    and $binding.filesystem_write_performed == false
    and $binding.filesystem_write_performed_count == 0
    and $binding.workspace_write_performed == false
    and $binding.workspace_write_performed_count == 0
    and $binding.receipt_materialized_count == 0
    and $binding.receipt_persisted_count == 0
    and $binding.raw_payload_plaintext_recorded == false
    and $binding.raw_payload_plaintext_persisted == false
    and $binding.live_secret_scan_performed == false
    and $binding.receipt_persistence_enabled == false
    and $binding.receipt_persisted == false
    and $binding.activation_allowed == false
    and $binding.live_mutation_execution_ready == false
    and ($binding.evidence_bindings | length) == 11
    and ($binding.allowed_output_path_bindings | length) == 3
    and ($binding.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_gate" \
  --arg output_path_evidence_binding_report_sha256 "$output_path_evidence_binding_report_sha256" \
  --arg receipt_root_preview_payload_sha256 "$receipt_root_preview_payload_sha256" \
  --arg dry_run_root_preview_payload_sha256 "$dry_run_root_preview_payload_sha256" \
  --arg operator_packet_root_preview_payload_sha256 "$operator_packet_root_preview_payload_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson binding "$OUTPUT_PATH_EVIDENCE_BINDING_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_gate:$binding.gate,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_ready:$binding.payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_ready,
    source_receipt_payload_sha256:$binding.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$binding.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$binding.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$binding.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$binding.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$binding.source_no_secret_payload_review_report_sha256,
    source_payload_redaction_proof_report_sha256:$binding.source_payload_redaction_proof_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$binding.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_acceptance_receipt_command_contract_report_sha256:$binding.source_payload_redaction_acceptance_receipt_command_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256:$binding.source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256:$binding.source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256:$binding.source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256,
    source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256:$binding.source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256:$binding.source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256:$binding.source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_report_sha256:$output_path_evidence_binding_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_sink_write_preview_ready:true,
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
    filesystem_sink_write_preview_recorded:false,
    filesystem_sink_write_preview_persisted:false,
    payload_redaction_acceptance_matrix_recorded:false,
    payload_redaction_acceptance_matrix_persisted:false,
    payload_redaction_proof_recorded:false,
    payload_redaction_proof_accepted:false,
    accepted_redaction_proof_count:0,
    reviewed_redaction_proof_count:0,
    required_preview_fixture_count:3,
    preview_fixture_count:3,
    allowed_output_path_entry_count:3,
    blocked_output_path_entry_count:3,
    previewed_output_path_count:3,
    report_only_root_preview_count:3,
    mutating_root_preview_count:0,
    deterministic_payload_hash_count:3,
    redacted_output_path_preview_count:3,
    fresh_pre_activation_soak_evidence_bound_fixture_count:0,
    active_binary_sha_bound_fixture_count:0,
    operator_scope_bound_fixture_count:0,
    accepted_redaction_proof_bound_fixture_count:0,
    trusted_source_bound_fixture_count:0,
    blocked_preview_fixture_count:3,
    allowed_preview_fixture_count:0,
    public_claim_attempt_fixture_count:0,
    release_artifact_write_attempt_fixture_count:0,
    source_tree_path_preview_allowed:false,
    home_directory_path_preview_allowed:false,
    release_artifact_path_preview_allowed:false,
    public_artifact_path_preview_allowed:false,
    default_selected_output_path_count:0,
    selected_output_path_count:0,
    recorded_output_path_count:0,
    recorded_path_binding_count:0,
    receipt_output_path_selected:false,
    receipt_output_path_recorded:false,
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
    filesystem_write_requested_count:3,
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
    activation_blocked_by_sink_write_preview:true,
    activation_allowed_by_sink_write_preview:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    sink_write_preview_denial_reason:"redacted report-only sink write previews are deterministic schema evidence only; no output path is selected, no fresh evidence is recorded, and filesystem persistence remains disabled",
    preview_fixtures:[
      {
        id:"payload-redaction-acceptance-receipts-root-sink-write-preview",
        output_path_binding:"payload_redaction_acceptance_receipts_root",
        preview_status:"blocked_preview",
        report_only_root:true,
        mutating_root:false,
        receipt_output_path_redacted:true,
        deterministic_payload_sha256:$receipt_root_preview_payload_sha256,
        source_output_path_evidence_binding_report_sha256:$output_path_evidence_binding_report_sha256,
        output_path_selected:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false
      },
      {
        id:"payload-redaction-acceptance-receipt-dry-run-root-sink-write-preview",
        output_path_binding:"payload_redaction_acceptance_receipt_dry_run_root",
        preview_status:"blocked_preview",
        report_only_root:true,
        mutating_root:false,
        receipt_output_path_redacted:true,
        deterministic_payload_sha256:$dry_run_root_preview_payload_sha256,
        source_output_path_evidence_binding_report_sha256:$output_path_evidence_binding_report_sha256,
        output_path_selected:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false
      },
      {
        id:"payload-redaction-acceptance-receipt-operator-packet-root-sink-write-preview",
        output_path_binding:"payload_redaction_acceptance_receipt_operator_packet_root",
        preview_status:"blocked_preview",
        report_only_root:true,
        mutating_root:false,
        receipt_output_path_redacted:true,
        deterministic_payload_sha256:$operator_packet_root_preview_payload_sha256,
        source_output_path_evidence_binding_report_sha256:$output_path_evidence_binding_report_sha256,
        output_path_selected:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false
      }
    ],
    preview_denied_roots:[
      "source_tree_root",
      "home_directory_root",
      "release_artifact_root"
    ],
    denied_by_sink_write_preview:[
      "default_output_path_selection_denied",
      "fresh_pre_activation_soak_evidence_missing",
      "active_binary_sha_missing",
      "operator_scope_missing",
      "accepted_redaction_proof_missing",
      "source_tree_output_path_preview_denied",
      "home_directory_output_path_preview_denied",
      "release_artifact_output_path_preview_denied",
      "public_artifact_output_path_preview_denied",
      "command_invocation_execution_denied",
      "materialization_execution_denied",
      "filesystem_persistence_execution_denied",
      "filesystem_write_denied",
      "workspace_write_denied",
      "receipt_persistence_denied",
      "live_mutation_execution_denied"
    ],
    required_before_any_sink_write_execution:[
      "explicit_operator_enablement_for_receipt_persistence",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "accepted_redaction_proof_ids",
      "fresh_pre_activation_soak_evidence",
      "active_binary_sha256",
      "receipt_payload_hash",
      "redacted_payload_summary_sha256",
      "receipt_output_path_redacted",
      "filesystem_persistence_approval_id",
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
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      receipt_materialized:false,
      receipt_persisted:false,
      filesystem_persistence_approval_packet_persisted:false,
      output_path_allowlist_persisted:false,
      output_path_evidence_binding_persisted:false,
      filesystem_sink_write_preview_persisted:false,
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
  and .payload_redaction_acceptance_receipt_filesystem_sink_write_preview_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_preview_fixture_count == 3
  and .preview_fixture_count == 3
  and .allowed_output_path_entry_count == 3
  and .blocked_output_path_entry_count == 3
  and .previewed_output_path_count == 3
  and .report_only_root_preview_count == 3
  and .mutating_root_preview_count == 0
  and .deterministic_payload_hash_count == 3
  and .redacted_output_path_preview_count == 3
  and .fresh_pre_activation_soak_evidence_bound_fixture_count == 0
  and .active_binary_sha_bound_fixture_count == 0
  and .operator_scope_bound_fixture_count == 0
  and .accepted_redaction_proof_bound_fixture_count == 0
  and .trusted_source_bound_fixture_count == 0
  and .blocked_preview_fixture_count == 3
  and .allowed_preview_fixture_count == 0
  and .source_tree_path_preview_allowed == false
  and .home_directory_path_preview_allowed == false
  and .release_artifact_path_preview_allowed == false
  and .public_artifact_path_preview_allowed == false
  and .default_selected_output_path_count == 0
  and .selected_output_path_count == 0
  and .recorded_output_path_count == 0
  and .recorded_path_binding_count == 0
  and .filesystem_persistence_allowed == false
  and .filesystem_persistence_allowed_count == 0
  and .command_invocation_attempt_count == 5
  and .command_invocation_performed_count == 0
  and .command_execution_performed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .materialization_execution_performed_count == 0
  and .filesystem_persistence_execution_performed == false
  and .filesystem_persistence_execution_performed_count == 0
  and .filesystem_write_requested_count == 3
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
  and .activation_blocked_by_sink_write_preview == true
  and .activation_allowed_by_sink_write_preview == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.preview_fixtures | length) == 3
  and (.preview_fixtures | all(.preview_status == "blocked_preview" and .report_only_root == true and .mutating_root == false and .receipt_output_path_redacted == true and .output_path_selected == false and .command_invocation_performed == false and .command_execution_performed == false and .materialization_execution_performed == false and .filesystem_persistence_execution_performed == false and .filesystem_write_requested == true and .filesystem_write_performed == false and .receipt_persisted == false and .activation_allowed == false))
  and (.preview_denied_roots | length) == 3
  and (.denied_by_sink_write_preview | length) == 16
  and (.required_before_any_sink_write_execution | length) == 13
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem sink write preview gate passed"
