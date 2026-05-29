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

LEDGER_SHAPE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-shape-approval-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-shape-approval-gate.sh
)"

ledger_shape_report_sha256="$(
  printf '%s' "$LEDGER_SHAPE_JSON" | shasum -a 256 | awk '{print $1}'
)"

missing_shape_rehearsal_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-denial:missing-ledger-shape-approval:$ledger_shape_report_sha256"
)"
stale_soak_rehearsal_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-denial:stale-pre-activation-soak-evidence:$ledger_shape_report_sha256"
)"
workspace_path_rehearsal_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-denial:workspace-path:$ledger_shape_report_sha256"
)"
public_artifact_rehearsal_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-denial:public-artifact:$ledger_shape_report_sha256"
)"

jq -n -e \
  --argjson shape "$LEDGER_SHAPE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $shape.runtime == "hepta"
    and $shape.status == "ready"
    and $shape.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_gate"
    and $shape.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_ready == true
    and $shape.payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_ready == true
    and $shape.source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256 != ""
    and $shape.minimum_required_samples >= 24
    and $shape.required_ledger_shape_field_count == 16
    and $shape.ledger_shape_field_count == 16
    and $shape.recorded_ledger_shape_field_count == 0
    and $shape.redacted_or_hashed_ledger_shape_field_count == 15
    and $shape.required_before_any_ledger_persistence_count == 16
    and $shape.source_dry_run_ledger_entry_count == 4
    and $shape.source_dry_run_ledger_entry_hash_count == 4
    and $shape.source_denied_dry_run_ledger_entry_count == 4
    and $shape.source_allowed_dry_run_ledger_entry_count == 0
    and $shape.required_ledger_shape_fixture_count == 4
    and $shape.ledger_shape_fixture_count == 4
    and $shape.blocked_ledger_shape_fixture_count == 4
    and $shape.allowed_ledger_shape_fixture_count == 0
    and $shape.ledger_shape_approval_requested_count == 4
    and $shape.ledger_shape_approval_performed_count == 0
    and $shape.ledger_shape_approval_recorded == false
    and $shape.ledger_shape_approval_persisted == false
    and $shape.ledger_shape_materialized == false
    and $shape.ledger_shape_filesystem_written == false
    and $shape.dry_run_ledger_recorded == false
    and $shape.dry_run_ledger_persisted == false
    and $shape.dry_run_ledger_materialized == false
    and $shape.dry_run_ledger_filesystem_written == false
    and $shape.ledger_persistence_allowed == false
    and $shape.ledger_persistence_allowed_count == 0
    and $shape.ledger_persistence_execution_requested_count == 0
    and $shape.ledger_persistence_execution_performed == false
    and $shape.ledger_persistence_execution_performed_count == 0
    and $shape.receipt_persistence_allowed_count == 0
    and $shape.receipt_persistence_execution_performed_count == 0
    and $shape.filesystem_persistence_execution_performed == false
    and $shape.filesystem_persistence_execution_performed_count == 0
    and $shape.filesystem_write_performed == false
    and $shape.filesystem_write_performed_count == 0
    and $shape.workspace_write_performed == false
    and $shape.workspace_write_performed_count == 0
    and $shape.command_invocation_performed_count == 0
    and $shape.command_execution_performed_count == 0
    and $shape.materialization_execution_performed_count == 0
    and $shape.receipt_materialized_count == 0
    and $shape.receipt_persisted_count == 0
    and $shape.selected_output_path_count == 0
    and $shape.recorded_output_path_count == 0
    and $shape.recorded_path_binding_count == 0
    and $shape.active_binary_sha_bound_count == 0
    and $shape.trusted_source_bound_count == 0
    and $shape.operator_scope_bound_count == 0
    and $shape.accepted_redaction_proof_bound_count == 0
    and $shape.public_claim_allowed == false
    and $shape.release_artifact_write_allowed == false
    and $shape.raw_payload_plaintext_recorded == false
    and $shape.raw_payload_plaintext_persisted == false
    and $shape.raw_payload_inspected == false
    and $shape.live_secret_scan_performed == false
    and $shape.receipt_persistence_enabled == false
    and $shape.activation_blocked_by_ledger_shape_approval == true
    and $shape.activation_allowed_by_ledger_shape_approval == false
    and $shape.activation_allowed == false
    and $shape.live_mutation_execution_ready == false
    and ($shape.required_ledger_shape_fields | length) == 16
    and ($shape.required_before_any_ledger_persistence | length) == 16
    and ($shape.ledger_shape_approval_fixtures | length) == 4
    and ($shape.ledger_shape_approval_fixtures | all(.approval_status == "blocked" and .ledger_shape_approval_requested == true and .ledger_shape_approval_performed == false and .ledger_shape_approval_recorded == false and .ledger_shape_approval_persisted == false and .ledger_persistence_allowed == false and .ledger_persistence_execution_performed == false and .receipt_persistence_execution_performed == false and .filesystem_persistence_execution_performed == false and .filesystem_write_performed == false and .workspace_write_performed == false and .activation_allowed == false))
    and ($shape.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_gate" \
  --arg ledger_shape_report_sha256 "$ledger_shape_report_sha256" \
  --arg missing_shape_rehearsal_sha256 "$missing_shape_rehearsal_sha256" \
  --arg stale_soak_rehearsal_sha256 "$stale_soak_rehearsal_sha256" \
  --arg workspace_path_rehearsal_sha256 "$workspace_path_rehearsal_sha256" \
  --arg public_artifact_rehearsal_sha256 "$public_artifact_rehearsal_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson shape "$LEDGER_SHAPE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    rehearsal_schema_version:"payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_v1",
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_gate:$shape.gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_ready:$shape.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_ready,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256:$ledger_shape_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256:$shape.source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256:$shape.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_report_sha256:$shape.source_payload_redaction_acceptance_receipt_filesystem_sink_write_preview_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_report_sha256:$shape.source_payload_redaction_acceptance_receipt_filesystem_output_path_evidence_binding_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256:$shape.source_payload_redaction_acceptance_receipt_filesystem_output_path_allowlist_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256:$shape.source_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_report_sha256,
    source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256:$shape.source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256:$shape.source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256,
    source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256:$shape.source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256:$shape.source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_command_contract_report_sha256:$shape.source_payload_redaction_acceptance_receipt_command_contract_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$shape.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_proof_report_sha256:$shape.source_payload_redaction_proof_report_sha256,
    source_no_secret_payload_review_report_sha256:$shape.source_no_secret_payload_review_report_sha256,
    source_operator_scope_report_sha256:$shape.source_operator_scope_report_sha256,
    source_approval_packet_report_sha256:$shape.source_approval_packet_report_sha256,
    source_receipt_payload_sha256:$shape.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$shape.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$shape.source_persistence_denial_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_ready:true,
    required_ledger_persistence_rehearsal_fixture_count:4,
    ledger_persistence_rehearsal_fixture_count:4,
    blocked_ledger_persistence_rehearsal_fixture_count:4,
    allowed_ledger_persistence_rehearsal_fixture_count:0,
    ledger_persistence_rehearsal_requested_count:4,
    ledger_persistence_rehearsal_performed_count:0,
    ledger_persistence_allowed:false,
    ledger_persistence_allowed_count:0,
    ledger_persistence_execution_requested_count:4,
    ledger_persistence_execution_performed:false,
    ledger_persistence_execution_performed_count:0,
    ledger_recorded:false,
    ledger_persisted:false,
    ledger_materialized:false,
    ledger_filesystem_written:false,
    ledger_write_path_selected:false,
    ledger_write_path_recorded:false,
    ledger_shape_approval_recorded:false,
    ledger_shape_approval_persisted:false,
    dry_run_ledger_recorded:false,
    dry_run_ledger_persisted:false,
    dry_run_ledger_materialized:false,
    dry_run_ledger_filesystem_written:false,
    receipt_persistence_allowed_count:0,
    receipt_persistence_execution_performed_count:0,
    receipt_materialized_count:0,
    receipt_persisted_count:0,
    filesystem_persistence_allowed:false,
    filesystem_persistence_allowed_count:0,
    filesystem_persistence_execution_requested_count:4,
    filesystem_persistence_execution_performed:false,
    filesystem_persistence_execution_performed_count:0,
    filesystem_write_requested_count:4,
    filesystem_write_performed:false,
    filesystem_write_performed_count:0,
    workspace_write_performed:false,
    workspace_write_performed_count:0,
    command_invocation_requested_count:0,
    command_invocation_performed_count:0,
    command_execution_requested_count:0,
    command_execution_performed_count:0,
    materialization_execution_requested_count:0,
    materialization_execution_performed_count:0,
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
    future_rollback_rehearsal_evidence_slot_count:4,
    rollback_rehearsal_evidence_recorded_count:0,
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
    activation_blocked_by_ledger_persistence_rehearsal_denial:true,
    activation_allowed_by_ledger_persistence_rehearsal_denial:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    ledger_persistence_rehearsal_denial_reason:"future ledger persistence rehearsal fixtures stay denied until ledger shape approval, fresh soak evidence, rollback rehearsal evidence, active binary binding, trusted source binding, operator scope, redaction proof acceptance, and filesystem output policy are all recorded by an explicit operator-approved activation packet",
    required_before_any_ledger_persistence_rehearsal:[
      "explicit_operator_enablement_for_ledger_persistence",
      "ledger_shape_approval_id",
      "ledger_shape_approval_report_sha256",
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
      "rollback_rehearsal_evidence_id",
      "public_claim_and_artifact_decision",
      "ledger_persistence_rehearsal_denial_report_sha256"
    ],
    ledger_persistence_rehearsal_fixtures:[
      {
        id:"missing-ledger-shape-approval-ledger-persistence-rehearsal",
        source_ledger_shape_fixture_id:"missing-persistence-approval-id-ledger-shape-approval",
        deterministic_rehearsal_sha256:$missing_shape_rehearsal_sha256,
        rehearsal_status:"blocked",
        ledger_persistence_rehearsal_requested:true,
        ledger_persistence_rehearsal_performed:false,
        ledger_persistence_allowed:false,
        ledger_persistence_execution_requested:true,
        ledger_persistence_execution_performed:false,
        filesystem_persistence_execution_requested:true,
        filesystem_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_persistence_execution_performed:false,
        ledger_recorded:false,
        ledger_persisted:false,
        activation_allowed:false,
        denial_reasons:["ledger_shape_approval_missing","filesystem_persistence_approval_id_missing","ledger_persistence_denied","filesystem_persistence_execution_denied","filesystem_write_denied","live_mutation_execution_denied"]
      },
      {
        id:"stale-pre-activation-soak-ledger-persistence-rehearsal",
        source_ledger_shape_fixture_id:"stale-pre-activation-soak-evidence-ledger-shape-approval",
        deterministic_rehearsal_sha256:$stale_soak_rehearsal_sha256,
        rehearsal_status:"blocked",
        ledger_persistence_rehearsal_requested:true,
        ledger_persistence_rehearsal_performed:false,
        ledger_persistence_allowed:false,
        ledger_persistence_execution_requested:true,
        ledger_persistence_execution_performed:false,
        filesystem_persistence_execution_requested:true,
        filesystem_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_persistence_execution_performed:false,
        ledger_recorded:false,
        ledger_persisted:false,
        activation_allowed:false,
        denial_reasons:["fresh_pre_activation_soak_evidence_missing_or_stale","rollback_rehearsal_evidence_missing","ledger_persistence_denied","filesystem_persistence_execution_denied","filesystem_write_denied","live_mutation_execution_denied"]
      },
      {
        id:"workspace-path-ledger-persistence-rehearsal",
        source_ledger_shape_fixture_id:"workspace-path-ledger-shape-approval",
        deterministic_rehearsal_sha256:$workspace_path_rehearsal_sha256,
        rehearsal_status:"blocked",
        ledger_persistence_rehearsal_requested:true,
        ledger_persistence_rehearsal_performed:false,
        ledger_persistence_allowed:false,
        ledger_persistence_execution_requested:true,
        ledger_persistence_execution_performed:false,
        filesystem_persistence_execution_requested:true,
        filesystem_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_persistence_execution_performed:false,
        ledger_recorded:false,
        ledger_persisted:false,
        activation_allowed:false,
        denial_reasons:["source_tree_workspace_output_path_denied","ledger_persistence_denied","filesystem_persistence_execution_denied","filesystem_write_denied","workspace_write_denied","live_mutation_execution_denied"]
      },
      {
        id:"public-artifact-ledger-persistence-rehearsal",
        source_ledger_shape_fixture_id:"public-artifact-ledger-shape-approval",
        deterministic_rehearsal_sha256:$public_artifact_rehearsal_sha256,
        rehearsal_status:"blocked",
        ledger_persistence_rehearsal_requested:true,
        ledger_persistence_rehearsal_performed:false,
        ledger_persistence_allowed:false,
        ledger_persistence_execution_requested:true,
        ledger_persistence_execution_performed:false,
        filesystem_persistence_execution_requested:true,
        filesystem_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_persistence_execution_performed:false,
        ledger_recorded:false,
        ledger_persisted:false,
        activation_allowed:false,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        denial_reasons:["public_artifact_release_output_path_denied","public_release_claim_denied","release_artifact_write_denied","ledger_persistence_denied","filesystem_persistence_execution_denied","filesystem_write_denied","live_mutation_execution_denied"]
      }
    ],
    denied_by_ledger_persistence_rehearsal:[
      "ledger_shape_approval_missing",
      "filesystem_persistence_approval_id_missing",
      "fresh_pre_activation_soak_evidence_missing_or_stale",
      "rollback_rehearsal_evidence_missing",
      "source_tree_workspace_output_path_denied",
      "public_artifact_release_output_path_denied",
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
      ledger_persistence_rehearsal_performed:false,
      ledger_persistence_execution_performed:false,
      ledger_recorded:false,
      ledger_persisted:false,
      ledger_filesystem_written:false,
      filesystem_written:false,
      workspace_write_performed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      rollback_rehearsal_evidence_recorded:false,
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
  .status == "ready"
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_ledger_persistence_rehearsal_fixture_count == 4
  and .ledger_persistence_rehearsal_fixture_count == 4
  and .blocked_ledger_persistence_rehearsal_fixture_count == 4
  and .allowed_ledger_persistence_rehearsal_fixture_count == 0
  and .ledger_persistence_rehearsal_requested_count == 4
  and .ledger_persistence_rehearsal_performed_count == 0
  and .ledger_persistence_allowed == false
  and .ledger_persistence_allowed_count == 0
  and .ledger_persistence_execution_requested_count == 4
  and .ledger_persistence_execution_performed == false
  and .ledger_persistence_execution_performed_count == 0
  and .ledger_recorded == false
  and .ledger_persisted == false
  and .ledger_materialized == false
  and .ledger_filesystem_written == false
  and .ledger_write_path_selected == false
  and .ledger_write_path_recorded == false
  and .ledger_shape_approval_recorded == false
  and .ledger_shape_approval_persisted == false
  and .dry_run_ledger_recorded == false
  and .dry_run_ledger_persisted == false
  and .dry_run_ledger_materialized == false
  and .dry_run_ledger_filesystem_written == false
  and .receipt_persistence_allowed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .receipt_materialized_count == 0
  and .receipt_persisted_count == 0
  and .filesystem_persistence_allowed == false
  and .filesystem_persistence_allowed_count == 0
  and .filesystem_persistence_execution_requested_count == 4
  and .filesystem_persistence_execution_performed == false
  and .filesystem_persistence_execution_performed_count == 0
  and .filesystem_write_requested_count == 4
  and .filesystem_write_performed == false
  and .filesystem_write_performed_count == 0
  and .workspace_write_performed == false
  and .workspace_write_performed_count == 0
  and .command_invocation_requested_count == 0
  and .command_invocation_performed_count == 0
  and .command_execution_requested_count == 0
  and .command_execution_performed_count == 0
  and .materialization_execution_requested_count == 0
  and .materialization_execution_performed_count == 0
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
  and .future_rollback_rehearsal_evidence_slot_count == 4
  and .rollback_rehearsal_evidence_recorded_count == 0
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
  and .activation_blocked_by_ledger_persistence_rehearsal_denial == true
  and .activation_allowed_by_ledger_persistence_rehearsal_denial == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.required_before_any_ledger_persistence_rehearsal | length) == 18
  and (.ledger_persistence_rehearsal_fixtures | length) == 4
  and (.ledger_persistence_rehearsal_fixtures | all(.rehearsal_status == "blocked" and .ledger_persistence_rehearsal_requested == true and .ledger_persistence_rehearsal_performed == false and .ledger_persistence_allowed == false and .ledger_persistence_execution_requested == true and .ledger_persistence_execution_performed == false and .filesystem_persistence_execution_requested == true and .filesystem_persistence_execution_performed == false and .filesystem_write_requested == true and .filesystem_write_performed == false and .workspace_write_performed == false and .receipt_persistence_execution_performed == false and .ledger_recorded == false and .ledger_persisted == false and .activation_allowed == false and .deterministic_rehearsal_sha256 != ""))
  and (.denied_by_ledger_persistence_rehearsal | length) == 14
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal denial gate passed"
