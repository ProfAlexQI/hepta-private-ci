#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SINK_WRITE_PREVIEW_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-sink-write-preview-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-sink-write-preview-gate.sh
)"

sink_write_preview_report_sha256="$(
  printf '%s' "$SINK_WRITE_PREVIEW_JSON" | shasum -a 256 | awk '{print $1}'
)"

missing_approval_attempt_payload_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-execution-denial:missing-persistence-approval-id:$sink_write_preview_report_sha256"
)"
stale_soak_attempt_payload_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-execution-denial:stale-pre-activation-soak-evidence:$sink_write_preview_report_sha256"
)"
workspace_path_attempt_payload_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-execution-denial:workspace-path:$sink_write_preview_report_sha256"
)"
public_artifact_attempt_payload_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-execution-denial:public-artifact:$sink_write_preview_report_sha256"
)"

jq -n -e \
  --argjson preview "$SINK_WRITE_PREVIEW_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $preview.runtime == "hepta"
    and $preview.status == "ready"
    and $preview.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_gate"
    and $preview.payload_redaction_acceptance_receipt_filesystem_sink_write_preview_ready == true
    and $preview.payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_ready == true
    and $preview.source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_report_sha256 != ""
    and $preview.minimum_required_samples >= 24
    and $preview.required_preview_fixture_count == 3
    and $preview.preview_fixture_count == 3
    and $preview.allowed_output_path_entry_count == 3
    and $preview.blocked_output_path_entry_count == 3
    and $preview.previewed_output_path_count == 3
    and $preview.report_only_root_preview_count == 3
    and $preview.mutating_root_preview_count == 0
    and $preview.deterministic_payload_hash_count == 3
    and $preview.redacted_output_path_preview_count == 3
    and $preview.fresh_pre_activation_soak_evidence_bound_fixture_count == 0
    and $preview.active_binary_sha_bound_fixture_count == 0
    and $preview.operator_scope_bound_fixture_count == 0
    and $preview.accepted_redaction_proof_bound_fixture_count == 0
    and $preview.trusted_source_bound_fixture_count == 0
    and $preview.blocked_preview_fixture_count == 3
    and $preview.allowed_preview_fixture_count == 0
    and $preview.source_tree_path_preview_allowed == false
    and $preview.home_directory_path_preview_allowed == false
    and $preview.release_artifact_path_preview_allowed == false
    and $preview.public_artifact_path_preview_allowed == false
    and $preview.default_selected_output_path_count == 0
    and $preview.selected_output_path_count == 0
    and $preview.recorded_output_path_count == 0
    and $preview.recorded_path_binding_count == 0
    and $preview.filesystem_persistence_allowed == false
    and $preview.filesystem_persistence_allowed_count == 0
    and $preview.command_invocation_attempt_count == 5
    and $preview.command_invocation_performed_count == 0
    and $preview.command_execution_performed_count == 0
    and $preview.receipt_persistence_execution_performed_count == 0
    and $preview.materialization_execution_performed_count == 0
    and $preview.filesystem_persistence_execution_performed == false
    and $preview.filesystem_persistence_execution_performed_count == 0
    and $preview.filesystem_write_requested_count == 3
    and $preview.filesystem_write_performed == false
    and $preview.filesystem_write_performed_count == 0
    and $preview.workspace_write_performed == false
    and $preview.workspace_write_performed_count == 0
    and $preview.receipt_materialized_count == 0
    and $preview.receipt_persisted_count == 0
    and $preview.raw_payload_plaintext_recorded == false
    and $preview.raw_payload_plaintext_persisted == false
    and $preview.live_secret_scan_performed == false
    and $preview.receipt_persistence_enabled == false
    and $preview.activation_blocked_by_sink_write_preview == true
    and $preview.activation_allowed_by_sink_write_preview == false
    and $preview.activation_allowed == false
    and $preview.live_mutation_execution_ready == false
    and ($preview.preview_fixtures | length) == 3
    and ($preview.preview_fixtures | all(.preview_status == "blocked_preview" and .filesystem_write_requested == true and .filesystem_write_performed == false and .receipt_persisted == false and .activation_allowed == false))
    and ($preview.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_gate" \
  --arg sink_write_preview_report_sha256 "$sink_write_preview_report_sha256" \
  --arg missing_approval_attempt_payload_sha256 "$missing_approval_attempt_payload_sha256" \
  --arg stale_soak_attempt_payload_sha256 "$stale_soak_attempt_payload_sha256" \
  --arg workspace_path_attempt_payload_sha256 "$workspace_path_attempt_payload_sha256" \
  --arg public_artifact_attempt_payload_sha256 "$public_artifact_attempt_payload_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson preview "$SINK_WRITE_PREVIEW_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_gate:$preview.gate,
    source_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_ready:$preview.payload_redaction_acceptance_receipt_filesystem_sink_write_preview_ready,
    source_receipt_payload_sha256:$preview.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$preview.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$preview.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$preview.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$preview.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$preview.source_no_secret_payload_review_report_sha256,
    source_payload_redaction_proof_report_sha256:$preview.source_payload_redaction_proof_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$preview.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_acceptance_receipt_command_contract_report_sha256:$preview.source_payload_redaction_acceptance_receipt_command_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256:$preview.source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256:$preview.source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256:$preview.source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256,
    source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256:$preview.source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256:$preview.source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256:$preview.source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_report_sha256:$preview.source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_report_sha256:$sink_write_preview_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
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
    execution_denial_matrix_recorded:false,
    execution_denial_matrix_persisted:false,
    payload_redaction_acceptance_matrix_recorded:false,
    payload_redaction_acceptance_matrix_persisted:false,
    payload_redaction_proof_recorded:false,
    payload_redaction_proof_accepted:false,
    accepted_redaction_proof_count:0,
    reviewed_redaction_proof_count:0,
    source_preview_fixture_count:3,
    required_denial_fixture_count:4,
    denial_fixture_count:4,
    execution_requested_fixture_count:4,
    future_persistence_approval_slot_count:4,
    explicit_persistence_approval_id_present_count:3,
    explicit_persistence_approval_id_missing_count:1,
    stale_or_missing_fresh_pre_activation_soak_evidence_fixture_count:1,
    stale_or_missing_fresh_evidence_fixture_count:1,
    future_active_binary_sha_bound_fixture_count:4,
    future_trusted_source_bound_fixture_count:4,
    future_operator_scope_bound_fixture_count:3,
    future_accepted_redaction_proof_bound_fixture_count:3,
    active_binary_sha_bound_fixture_count:0,
    trusted_source_bound_fixture_count:0,
    operator_scope_bound_fixture_count:0,
    accepted_redaction_proof_bound_fixture_count:0,
    source_tree_path_attempt_fixture_count:1,
    workspace_path_attempt_fixture_count:1,
    public_claim_attempt_fixture_count:1,
    release_artifact_write_attempt_fixture_count:1,
    blocked_execution_fixture_count:4,
    allowed_execution_fixture_count:0,
    allowed_output_path_entry_count:3,
    blocked_output_path_entry_count:3,
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
    filesystem_persistence_execution_requested_count:4,
    filesystem_persistence_execution_performed:false,
    filesystem_persistence_execution_performed_count:0,
    filesystem_write_requested_count:4,
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
    activation_blocked_by_execution_denial_matrix:true,
    activation_allowed_by_execution_denial_matrix:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    execution_denial_reason:"filesystem persistence execution remains denied for missing persistence approval id, stale evidence, blocked workspace paths, and public artifact attempts",
    denial_fixtures:[
      {
        id:"missing-persistence-approval-id-execution-attempt",
        fixture_kind:"missing_persistence_approval_id",
        source_sink_write_preview_report_sha256:$sink_write_preview_report_sha256,
        deterministic_payload_sha256:$missing_approval_attempt_payload_sha256,
        execution_requested:true,
        future_persistence_approval_slot_present:true,
        explicit_persistence_approval_id_present:false,
        future_active_binary_sha_bound:true,
        future_trusted_source_bound:true,
        future_operator_scope_bound:false,
        future_accepted_redaction_proof_bound:false,
        fresh_pre_activation_soak_evidence_present:true,
        redacted_output_path_present:true,
        output_path_binding:"payload_redaction_acceptance_receipts_root",
        workspace_path_attempt:false,
        public_claim_requested:false,
        release_artifact_write_requested:false,
        validation_status:"blocked",
        filesystem_persistence_allowed:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false
      },
      {
        id:"stale-pre-activation-soak-evidence-execution-attempt",
        fixture_kind:"stale_pre_activation_soak_evidence",
        source_sink_write_preview_report_sha256:$sink_write_preview_report_sha256,
        deterministic_payload_sha256:$stale_soak_attempt_payload_sha256,
        execution_requested:true,
        future_persistence_approval_slot_present:true,
        explicit_persistence_approval_id_present:true,
        future_active_binary_sha_bound:true,
        future_trusted_source_bound:true,
        future_operator_scope_bound:true,
        future_accepted_redaction_proof_bound:true,
        fresh_pre_activation_soak_evidence_present:false,
        redacted_output_path_present:true,
        output_path_binding:"payload_redaction_acceptance_receipt_dry_run_root",
        workspace_path_attempt:false,
        public_claim_requested:false,
        release_artifact_write_requested:false,
        validation_status:"blocked",
        filesystem_persistence_allowed:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false
      },
      {
        id:"workspace-path-execution-attempt",
        fixture_kind:"workspace_path_execution_attempt",
        source_sink_write_preview_report_sha256:$sink_write_preview_report_sha256,
        deterministic_payload_sha256:$workspace_path_attempt_payload_sha256,
        execution_requested:true,
        future_persistence_approval_slot_present:true,
        explicit_persistence_approval_id_present:true,
        future_active_binary_sha_bound:true,
        future_trusted_source_bound:true,
        future_operator_scope_bound:true,
        future_accepted_redaction_proof_bound:true,
        fresh_pre_activation_soak_evidence_present:true,
        redacted_output_path_present:true,
        output_path_binding:"source_tree_root",
        workspace_path_attempt:true,
        source_tree_path_attempt:true,
        public_claim_requested:false,
        release_artifact_write_requested:false,
        validation_status:"blocked",
        filesystem_persistence_allowed:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false
      },
      {
        id:"public-artifact-execution-attempt",
        fixture_kind:"public_artifact_execution_attempt",
        source_sink_write_preview_report_sha256:$sink_write_preview_report_sha256,
        deterministic_payload_sha256:$public_artifact_attempt_payload_sha256,
        execution_requested:true,
        future_persistence_approval_slot_present:true,
        explicit_persistence_approval_id_present:true,
        future_active_binary_sha_bound:true,
        future_trusted_source_bound:true,
        future_operator_scope_bound:true,
        future_accepted_redaction_proof_bound:true,
        fresh_pre_activation_soak_evidence_present:true,
        redacted_output_path_present:true,
        output_path_binding:"release_artifact_root",
        workspace_path_attempt:false,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        validation_status:"blocked",
        filesystem_persistence_allowed:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false
      }
    ],
    denied_by_execution_denial_matrix:[
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
    required_before_any_filesystem_persistence_execution:[
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
  .status == "ready"
  and .payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_sink_write_preview_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .source_preview_fixture_count == 3
  and .required_denial_fixture_count == 4
  and .denial_fixture_count == 4
  and .execution_requested_fixture_count == 4
  and .future_persistence_approval_slot_count == 4
  and .explicit_persistence_approval_id_present_count == 3
  and .explicit_persistence_approval_id_missing_count == 1
  and .stale_or_missing_fresh_pre_activation_soak_evidence_fixture_count == 1
  and .stale_or_missing_fresh_evidence_fixture_count == 1
  and .future_active_binary_sha_bound_fixture_count == 4
  and .future_trusted_source_bound_fixture_count == 4
  and .future_operator_scope_bound_fixture_count == 3
  and .future_accepted_redaction_proof_bound_fixture_count == 3
  and .active_binary_sha_bound_fixture_count == 0
  and .trusted_source_bound_fixture_count == 0
  and .operator_scope_bound_fixture_count == 0
  and .accepted_redaction_proof_bound_fixture_count == 0
  and .source_tree_path_attempt_fixture_count == 1
  and .workspace_path_attempt_fixture_count == 1
  and .public_claim_attempt_fixture_count == 1
  and .release_artifact_write_attempt_fixture_count == 1
  and .blocked_execution_fixture_count == 4
  and .allowed_execution_fixture_count == 0
  and .allowed_output_path_entry_count == 3
  and .blocked_output_path_entry_count == 3
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
  and .filesystem_persistence_execution_requested_count == 4
  and .filesystem_persistence_execution_performed == false
  and .filesystem_persistence_execution_performed_count == 0
  and .filesystem_write_requested_count == 4
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
  and .activation_blocked_by_execution_denial_matrix == true
  and .activation_allowed_by_execution_denial_matrix == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.denial_fixtures | length) == 4
  and (.denial_fixtures | all(.execution_requested == true and .validation_status == "blocked" and .filesystem_persistence_allowed == false and .command_invocation_performed == false and .command_execution_performed == false and .materialization_execution_performed == false and .filesystem_persistence_execution_performed == false and .filesystem_write_requested == true and .filesystem_write_performed == false and .workspace_write_performed == false and .receipt_persisted == false and .activation_allowed == false))
  and (.denied_by_execution_denial_matrix | length) == 13
  and (.required_before_any_filesystem_persistence_execution | length) == 14
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence execution denial matrix gate passed"
