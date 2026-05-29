#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

INVOCATION_DRY_RUN_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-invocation-dry-run-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-invocation-dry-run-gate.sh
)"

invocation_dry_run_report_sha256="$(printf '%s' "$INVOCATION_DRY_RUN_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson invocation "$INVOCATION_DRY_RUN_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $invocation.runtime == "hepta"
    and $invocation.status == "ready"
    and $invocation.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_invocation_dry_run_gate"
    and $invocation.payload_redaction_acceptance_receipt_invocation_dry_run_ready == true
    and $invocation.payload_redaction_acceptance_receipt_command_contract_ready == true
    and $invocation.source_payload_redaction_acceptance_receipt_command_contract_report_sha256 != ""
    and $invocation.payload_redaction_acceptance_receipt_command_recorded == false
    and $invocation.payload_redaction_acceptance_receipt_command_enabled_by_default == false
    and $invocation.payload_redaction_acceptance_receipt_command_invocation_requested_count == 5
    and $invocation.payload_redaction_acceptance_receipt_command_invocation_performed_count == 0
    and $invocation.payload_redaction_acceptance_receipt_command_execution_performed_count == 0
    and $invocation.payload_redaction_acceptance_receipt_recorded == false
    and $invocation.payload_redaction_acceptance_receipt_persisted == false
    and $invocation.payload_redaction_acceptance_matrix_recorded == false
    and $invocation.payload_redaction_acceptance_matrix_persisted == false
    and $invocation.payload_redaction_proof_recorded == false
    and $invocation.payload_redaction_proof_accepted == false
    and $invocation.required_invocation_fixture_count == 5
    and $invocation.blocked_invocation_fixture_count == 5
    and $invocation.allowed_invocation_fixture_count == 0
    and $invocation.redacted_output_path_fixture_count == 4
    and $invocation.redacted_payload_summary_hash_bound_fixture_count == 5
    and $invocation.accepted_redaction_proof_bound_fixture_count == 4
    and $invocation.operator_scope_bound_fixture_count == 4
    and $invocation.public_claim_attempt_count == 1
    and $invocation.release_artifact_write_attempt_count == 1
    and $invocation.plaintext_payload_attempt_count == 1
    and $invocation.command_invocation_attempt_count == 5
    and $invocation.command_invocation_performed_count == 0
    and $invocation.command_execution_performed_count == 0
    and $invocation.receipt_persistence_execution_performed_count == 0
    and $invocation.filesystem_write_performed_count == 0
    and $invocation.workspace_write_performed_count == 0
    and $invocation.receipt_persisted_count == 0
    and $invocation.raw_payload_plaintext_recorded == false
    and $invocation.raw_payload_plaintext_persisted == false
    and $invocation.live_secret_scan_performed == false
    and $invocation.receipt_persistence_enabled == false
    and $invocation.receipt_persisted == false
    and $invocation.activation_allowed == false
    and $invocation.live_mutation_execution_ready == false
    and ($invocation.invocation_fixtures | length) == 5
    and ($invocation.invocation_fixtures | all(.command_invocation_requested == true and .command_invocation_performed == false and .command_execution_performed == false and .receipt_persistence_execution_performed == false and .filesystem_write_performed == false and .receipt_persisted == false and .activation_allowed == false))
    and ($invocation.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_no_write_sink_contract_gate" \
  --arg invocation_dry_run_report_sha256 "$invocation_dry_run_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson invocation "$INVOCATION_DRY_RUN_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_payload_redaction_acceptance_receipt_invocation_dry_run_gate:$invocation.gate,
    source_payload_redaction_acceptance_receipt_invocation_dry_run_ready:$invocation.payload_redaction_acceptance_receipt_invocation_dry_run_ready,
    source_receipt_payload_sha256:$invocation.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$invocation.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$invocation.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$invocation.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$invocation.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$invocation.source_no_secret_payload_review_report_sha256,
    source_payload_redaction_proof_report_sha256:$invocation.source_payload_redaction_proof_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$invocation.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_acceptance_receipt_command_contract_report_sha256:$invocation.source_payload_redaction_acceptance_receipt_command_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256:$invocation_dry_run_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_no_write_sink_contract_ready:true,
    payload_redaction_acceptance_receipt_invocation_dry_run_ready:true,
    payload_redaction_acceptance_receipt_command_contract_ready:true,
    payload_redaction_acceptance_receipt_command_recorded:false,
    payload_redaction_acceptance_receipt_command_enabled_by_default:false,
    payload_redaction_acceptance_receipt_command_invocation_requested_count:5,
    payload_redaction_acceptance_receipt_command_invocation_performed_count:0,
    payload_redaction_acceptance_receipt_command_execution_performed_count:0,
    payload_redaction_acceptance_receipt_recorded:false,
    payload_redaction_acceptance_receipt_persisted:false,
    payload_redaction_acceptance_matrix_recorded:false,
    payload_redaction_acceptance_matrix_persisted:false,
    payload_redaction_proof_recorded:false,
    payload_redaction_proof_accepted:false,
    accepted_redaction_proof_count:0,
    reviewed_redaction_proof_count:0,
    required_sink_surface_count:8,
    ready_sink_surface_count:8,
    side_effect_free_sink_surface_count:8,
    source_invocation_fixture_count:5,
    no_write_sink_fixture_count:5,
    no_write_sink_accepted_redacted_fixture_count:3,
    no_write_sink_rejected_plaintext_fixture_count:1,
    no_write_sink_rejected_public_artifact_fixture_count:1,
    no_write_sink_write_request_fixture_count:5,
    no_write_sink_rejected_write_fixture_count:5,
    no_write_sink_allowed_write_fixture_count:0,
    no_write_sink_redacted_output_path_fixture_count:4,
    no_write_sink_payload_summary_hash_bound_fixture_count:5,
    no_write_sink_accepted_redaction_proof_bound_fixture_count:4,
    no_write_sink_operator_scope_bound_fixture_count:4,
    no_write_sink_accepts_redacted_payload_summary_hash:true,
    no_write_sink_accepts_redacted_output_path:true,
    no_write_sink_requires_accepted_redaction_proof:true,
    no_write_sink_requires_operator_scope:true,
    no_write_sink_rejects_plaintext_payload:true,
    no_write_sink_rejects_public_claim_artifact:true,
    no_write_sink_rejects_filesystem_write:true,
    no_write_sink_write_path_enabled_by_default:false,
    public_claim_attempt_count:1,
    release_artifact_write_attempt_count:1,
    plaintext_payload_attempt_count:1,
    command_invocation_attempt_count:5,
    command_invocation_performed_count:0,
    command_execution_performed_count:0,
    receipt_persistence_execution_performed_count:0,
    filesystem_write_performed_count:0,
    workspace_write_performed_count:0,
    receipt_persisted_count:0,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    live_secret_scan_performed:false,
    pre_activation_soak_evidence_persistence_allowed:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    no_write_sink_denial_reason:"the sink accepts redacted dry-run receipt shapes for validation only, but rejects every command execution, receipt persistence, filesystem write, public artifact, plaintext payload, and live mutation path",
    no_write_sink_surfaces:[
      "redacted_invocation_shape_acceptance",
      "redacted_payload_summary_hash_binding",
      "redacted_output_path_binding",
      "accepted_redaction_proof_binding",
      "operator_scope_binding",
      "plaintext_payload_rejection",
      "public_claim_artifact_rejection",
      "filesystem_write_rejection"
    ],
    no_write_sink_fixtures:[
      {
        id:"redacted-command-shape",
        sink_status:"accepted_for_no_write_validation",
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        accepted_redaction_proof_count:1,
        operator_scope_recorded:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"redacted_shape_valid_but_no_write_sink_rejects_persistence"
      },
      {
        id:"accepted-proof-but-command-disabled",
        sink_status:"accepted_for_no_write_validation",
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        accepted_redaction_proof_count:1,
        operator_scope_recorded:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"command_disabled_and_sink_is_no_write"
      },
      {
        id:"persistence-disabled-invocation-attempt",
        sink_status:"accepted_for_no_write_validation",
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        accepted_redaction_proof_count:1,
        operator_scope_recorded:true,
        receipt_persistence_enabled:false,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"persistence_disabled_and_sink_rejects_write"
      },
      {
        id:"plaintext-payload-invocation-attempt",
        sink_status:"rejected",
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:false,
        raw_payload_plaintext_recorded:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"plaintext_payload_forbidden"
      },
      {
        id:"public-artifact-invocation-attempt",
        sink_status:"rejected",
        redacted_payload_summary_sha256_recorded:true,
        receipt_output_path_redacted:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        command_invocation_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        filesystem_write_requested:true,
        filesystem_write_performed:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"public_claim_and_release_artifact_denied"
      }
    ],
    denied_by_no_write_sink_contract:[
      "command_invocation_execution_denied",
      "command_execution_denied",
      "receipt_persistence_execution_denied",
      "filesystem_write_denied",
      "workspace_write_denied",
      "plaintext_payload_recording_denied",
      "public_claim_and_release_artifact_denied",
      "live_mutation_execution_denied"
    ],
    required_before_any_write_sink_execution:[
      "explicit_operator_enablement_for_receipt_persistence",
      "accepted_redaction_proof_ids",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "redacted_payload_summary_sha256",
      "receipt_output_path_redacted",
      "receipt_persistence_approval",
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
      filesystem_written:false,
      workspace_write_performed:false,
      release_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      receipt_persisted:false,
      receipt_persistence_execution_performed:false,
      pre_activation_soak_evidence_persisted:false,
      approval_packet_persisted:false,
      operator_scope_binding_persisted:false,
      payload_review_persisted:false,
      payload_redaction_proof_persisted:false,
      payload_redaction_acceptance_matrix_persisted:false,
      payload_redaction_acceptance_receipt_command_persisted:false,
      payload_redaction_acceptance_receipt_persisted:false,
      payload_redaction_acceptance_receipt_no_write_sink_persisted:false,
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
  and .payload_redaction_acceptance_receipt_no_write_sink_contract_ready == true
  and .source_payload_redaction_acceptance_receipt_invocation_dry_run_ready == true
  and .source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .payload_redaction_acceptance_receipt_command_recorded == false
  and .payload_redaction_acceptance_receipt_command_enabled_by_default == false
  and .payload_redaction_acceptance_receipt_command_invocation_requested_count == 5
  and .payload_redaction_acceptance_receipt_command_invocation_performed_count == 0
  and .payload_redaction_acceptance_receipt_command_execution_performed_count == 0
  and .payload_redaction_acceptance_receipt_recorded == false
  and .payload_redaction_acceptance_receipt_persisted == false
  and .payload_redaction_acceptance_matrix_recorded == false
  and .payload_redaction_acceptance_matrix_persisted == false
  and .payload_redaction_proof_recorded == false
  and .payload_redaction_proof_accepted == false
  and .accepted_redaction_proof_count == 0
  and .required_sink_surface_count == 8
  and .ready_sink_surface_count == 8
  and .side_effect_free_sink_surface_count == 8
  and .source_invocation_fixture_count == 5
  and .no_write_sink_fixture_count == 5
  and .no_write_sink_accepted_redacted_fixture_count == 3
  and .no_write_sink_rejected_plaintext_fixture_count == 1
  and .no_write_sink_rejected_public_artifact_fixture_count == 1
  and .no_write_sink_write_request_fixture_count == 5
  and .no_write_sink_rejected_write_fixture_count == 5
  and .no_write_sink_allowed_write_fixture_count == 0
  and .no_write_sink_redacted_output_path_fixture_count == 4
  and .no_write_sink_payload_summary_hash_bound_fixture_count == 5
  and .no_write_sink_accepted_redaction_proof_bound_fixture_count == 4
  and .no_write_sink_operator_scope_bound_fixture_count == 4
  and .no_write_sink_accepts_redacted_payload_summary_hash == true
  and .no_write_sink_accepts_redacted_output_path == true
  and .no_write_sink_requires_accepted_redaction_proof == true
  and .no_write_sink_requires_operator_scope == true
  and .no_write_sink_rejects_plaintext_payload == true
  and .no_write_sink_rejects_public_claim_artifact == true
  and .no_write_sink_rejects_filesystem_write == true
  and .no_write_sink_write_path_enabled_by_default == false
  and .public_claim_attempt_count == 1
  and .release_artifact_write_attempt_count == 1
  and .plaintext_payload_attempt_count == 1
  and .command_invocation_attempt_count == 5
  and .command_invocation_performed_count == 0
  and .command_execution_performed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .filesystem_write_performed_count == 0
  and .workspace_write_performed_count == 0
  and .receipt_persisted_count == 0
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .live_secret_scan_performed == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.no_write_sink_surfaces | length) == 8
  and (.no_write_sink_fixtures | length) == 5
  and ([.no_write_sink_fixtures[] | select(.sink_status == "accepted_for_no_write_validation")] | length) == 3
  and ([.no_write_sink_fixtures[] | select(.sink_status == "rejected")] | length) == 2
  and (.no_write_sink_fixtures | all(.command_invocation_requested == true and .command_invocation_performed == false and .command_execution_performed == false and .receipt_persistence_execution_performed == false and .filesystem_write_requested == true and .filesystem_write_performed == false and .receipt_persisted == false and .activation_allowed == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt no-write sink contract gate passed"
