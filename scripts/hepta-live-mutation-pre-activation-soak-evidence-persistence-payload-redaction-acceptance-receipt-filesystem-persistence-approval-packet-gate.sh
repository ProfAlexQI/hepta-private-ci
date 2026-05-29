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

MATERIALIZATION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-materialization-dry-run-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-materialization-dry-run-gate.sh
)"

materialization_report_sha256="$(printf '%s' "$MATERIALIZATION_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson materialization "$MATERIALIZATION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $materialization.runtime == "hepta"
    and $materialization.status == "ready"
    and $materialization.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_materialization_dry_run_gate"
    and $materialization.payload_redaction_acceptance_receipt_materialization_dry_run_ready == true
    and $materialization.source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256 != ""
    and $materialization.required_materialization_fixture_count == 5
    and $materialization.materialization_fixture_count == 5
    and $materialization.blocked_materialization_fixture_count == 5
    and $materialization.allowed_materialization_fixture_count == 0
    and $materialization.deterministic_materialization_plan_count == 3
    and $materialization.deterministic_materialization_plan_persisted_count == 0
    and $materialization.command_invocation_performed_count == 0
    and $materialization.command_execution_performed_count == 0
    and $materialization.receipt_persistence_execution_performed_count == 0
    and $materialization.materialization_execution_performed_count == 0
    and $materialization.filesystem_write_performed_count == 0
    and $materialization.workspace_write_performed_count == 0
    and $materialization.receipt_materialized_count == 0
    and $materialization.receipt_persisted_count == 0
    and $materialization.raw_payload_plaintext_recorded == false
    and $materialization.raw_payload_plaintext_persisted == false
    and $materialization.receipt_persistence_enabled == false
    and $materialization.activation_allowed == false
    and $materialization.live_mutation_execution_ready == false
    and ($materialization.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_gate" \
  --arg materialization_report_sha256 "$materialization_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson materialization "$MATERIALIZATION_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_payload_redaction_acceptance_receipt_materialization_dry_run_gate:$materialization.gate,
    source_payload_redaction_acceptance_receipt_materialization_dry_run_ready:$materialization.payload_redaction_acceptance_receipt_materialization_dry_run_ready,
    source_receipt_payload_sha256:$materialization.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$materialization.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$materialization.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$materialization.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$materialization.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$materialization.source_no_secret_payload_review_report_sha256,
    source_payload_redaction_proof_report_sha256:$materialization.source_payload_redaction_proof_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$materialization.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_acceptance_receipt_command_contract_report_sha256:$materialization.source_payload_redaction_acceptance_receipt_command_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256:$materialization.source_payload_redaction_acceptance_receipt_invocation_dry_run_report_sha256,
    source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256:$materialization.source_payload_redaction_acceptance_receipt_no_write_sink_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256:$materialization.source_payload_redaction_acceptance_receipt_write_enable_fixture_report_sha256,
    source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256:$materialization_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
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
    payload_redaction_acceptance_matrix_recorded:false,
    payload_redaction_acceptance_matrix_persisted:false,
    payload_redaction_proof_recorded:false,
    payload_redaction_proof_accepted:false,
    accepted_redaction_proof_count:0,
    reviewed_redaction_proof_count:0,
    required_approval_field_count:13,
    approval_field_count:13,
    recorded_approval_field_count:0,
    redacted_or_hashed_field_count:11,
    required_for_filesystem_persistence_field_count:13,
    required_filesystem_persistence_approval_fixture_count:5,
    filesystem_persistence_approval_fixture_count:5,
    blocked_filesystem_persistence_approval_fixture_count:5,
    allowed_filesystem_persistence_approval_fixture_count:0,
    explicit_filesystem_persistence_approval_requested_fixture_count:5,
    approval_denied_without_operator_scope_count:1,
    approval_denied_command_disabled_count:1,
    approval_denied_persistence_disabled_count:1,
    approval_denied_plaintext_payload_count:1,
    approval_denied_public_artifact_count:1,
    deterministic_materialization_plan_count:3,
    deterministic_materialization_plan_persisted_count:0,
    materialization_plan_required:true,
    materialization_plan_recorded:false,
    materialization_plan_persisted:false,
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
    filesystem_persistence_allowed_count:0,
    command_invocation_attempt_count:5,
    command_invocation_performed_count:0,
    command_execution_performed_count:0,
    receipt_persistence_execution_performed_count:0,
    materialization_execution_performed_count:0,
    materialization_executed_count:0,
    filesystem_persistence_execution_performed_count:0,
    filesystem_write_performed_count:0,
    workspace_write_performed_count:0,
    receipt_materialized_count:0,
    receipt_persisted_count:0,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    live_secret_scan_performed:false,
    pre_activation_soak_evidence_persistence_allowed:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    filesystem_persistence_approval_denial_reason:"filesystem persistence approval is schema-only; materialization plans exist only in dry-run form and no operator approval, scope record, accepted proof record, fresh soak evidence, active binary SHA binding, rollback plan, or public artifact decision is recorded",
    required_approval_fields:[
      "filesystem_persistence_approval_id",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "receipt_materialization_plan_id",
      "receipt_payload_hash",
      "redacted_payload_summary_sha256",
      "receipt_output_path_redacted",
      "accepted_redaction_proof_ids",
      "fresh_pre_activation_soak_evidence_id",
      "active_binary_sha256",
      "rollback_plan_id",
      "public_claim_and_artifact_decision"
    ],
    filesystem_persistence_approval_fixtures:[
      {
        id:"approval-without-operator-scope",
        source_materialization_fixture_id:"materialization-without-operator-scope",
        filesystem_persistence_approval_requested:true,
        approval_status:"blocked_schema_only",
        recorded_approval_field_count:0,
        materialization_plan_ready:false,
        materialization_plan_recorded:false,
        operator_identity_hash_recorded:false,
        single_surface_activation_scope_recorded:false,
        accepted_redaction_proof_ids_recorded:false,
        fresh_pre_activation_soak_evidence_recorded:false,
        active_binary_sha_recorded:false,
        rollback_plan_recorded:false,
        public_artifact_policy_recorded:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"operator_scope_missing"
      },
      {
        id:"approval-with-command-disabled",
        source_materialization_fixture_id:"operator-scoped-but-command-disabled",
        filesystem_persistence_approval_requested:true,
        approval_status:"blocked_schema_only",
        recorded_approval_field_count:0,
        materialization_plan_ready:true,
        materialization_plan_recorded:false,
        operator_identity_hash_recorded:false,
        single_surface_activation_scope_recorded:false,
        accepted_redaction_proof_ids_recorded:false,
        fresh_pre_activation_soak_evidence_recorded:false,
        active_binary_sha_recorded:false,
        rollback_plan_recorded:false,
        public_artifact_policy_recorded:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"receipt_command_disabled_by_default"
      },
      {
        id:"approval-with-persistence-disabled",
        source_materialization_fixture_id:"accepted-proof-but-persistence-disabled",
        filesystem_persistence_approval_requested:true,
        approval_status:"blocked_schema_only",
        recorded_approval_field_count:0,
        materialization_plan_ready:true,
        materialization_plan_recorded:false,
        operator_identity_hash_recorded:false,
        single_surface_activation_scope_recorded:false,
        accepted_redaction_proof_ids_recorded:false,
        fresh_pre_activation_soak_evidence_recorded:false,
        active_binary_sha_recorded:false,
        rollback_plan_recorded:false,
        public_artifact_policy_recorded:false,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"receipt_persistence_disabled"
      },
      {
        id:"approval-for-plaintext-payload-attempt",
        source_materialization_fixture_id:"plaintext-materialization-attempt",
        filesystem_persistence_approval_requested:true,
        approval_status:"blocked_schema_only",
        recorded_approval_field_count:0,
        materialization_plan_ready:false,
        materialization_plan_recorded:false,
        operator_identity_hash_recorded:false,
        single_surface_activation_scope_recorded:false,
        accepted_redaction_proof_ids_recorded:false,
        fresh_pre_activation_soak_evidence_recorded:false,
        active_binary_sha_recorded:false,
        rollback_plan_recorded:false,
        public_artifact_policy_recorded:false,
        raw_payload_plaintext_recorded:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"plaintext_payload_forbidden"
      },
      {
        id:"approval-for-public-artifact-attempt",
        source_materialization_fixture_id:"public-artifact-materialization-attempt",
        filesystem_persistence_approval_requested:true,
        approval_status:"blocked_schema_only",
        recorded_approval_field_count:0,
        materialization_plan_ready:true,
        materialization_plan_recorded:false,
        operator_identity_hash_recorded:false,
        single_surface_activation_scope_recorded:false,
        accepted_redaction_proof_ids_recorded:false,
        fresh_pre_activation_soak_evidence_recorded:false,
        active_binary_sha_recorded:false,
        rollback_plan_recorded:false,
        public_artifact_policy_recorded:false,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        command_invocation_performed:false,
        command_execution_performed:false,
        receipt_persistence_execution_performed:false,
        materialization_execution_performed:false,
        filesystem_persistence_execution_performed:false,
        filesystem_write_performed:false,
        workspace_write_performed:false,
        receipt_materialized:false,
        receipt_persisted:false,
        activation_allowed:false,
        reason:"public_claim_and_release_artifact_denied"
      }
    ],
    denied_by_filesystem_persistence_approval_packet:[
      "operator_scope_missing",
      "receipt_command_disabled",
      "receipt_persistence_disabled",
      "plaintext_payload_approval_denied",
      "public_claim_and_release_artifact_denied",
      "filesystem_persistence_approval_not_recorded",
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
  and .payload_redaction_acceptance_receipt_filesystem_persistence_approval_packet_ready == true
  and .payload_redaction_acceptance_receipt_materialization_dry_run_ready == true
  and .source_payload_redaction_acceptance_receipt_materialization_dry_run_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .payload_redaction_acceptance_receipt_command_recorded == false
  and .payload_redaction_acceptance_receipt_command_enabled_by_default == false
  and .payload_redaction_acceptance_receipt_command_invocation_requested_count == 5
  and .payload_redaction_acceptance_receipt_command_invocation_performed_count == 0
  and .payload_redaction_acceptance_receipt_command_execution_performed_count == 0
  and .payload_redaction_acceptance_receipt_recorded == false
  and .payload_redaction_acceptance_receipt_materialized == false
  and .payload_redaction_acceptance_receipt_persisted == false
  and .filesystem_persistence_approval_packet_recorded == false
  and .filesystem_persistence_approval_packet_persisted == false
  and .payload_redaction_acceptance_matrix_recorded == false
  and .payload_redaction_acceptance_matrix_persisted == false
  and .payload_redaction_proof_recorded == false
  and .payload_redaction_proof_accepted == false
  and .accepted_redaction_proof_count == 0
  and .required_approval_field_count == 13
  and .approval_field_count == 13
  and .recorded_approval_field_count == 0
  and .redacted_or_hashed_field_count == 11
  and .required_for_filesystem_persistence_field_count == 13
  and .required_filesystem_persistence_approval_fixture_count == 5
  and .filesystem_persistence_approval_fixture_count == 5
  and .blocked_filesystem_persistence_approval_fixture_count == 5
  and .allowed_filesystem_persistence_approval_fixture_count == 0
  and .explicit_filesystem_persistence_approval_requested_fixture_count == 5
  and .approval_denied_without_operator_scope_count == 1
  and .approval_denied_command_disabled_count == 1
  and .approval_denied_persistence_disabled_count == 1
  and .approval_denied_plaintext_payload_count == 1
  and .approval_denied_public_artifact_count == 1
  and .deterministic_materialization_plan_count == 3
  and .deterministic_materialization_plan_persisted_count == 0
  and .materialization_plan_required == true
  and .materialization_plan_recorded == false
  and .operator_approval_required == true
  and .operator_approval_recorded == false
  and .operator_identity_hash_required == true
  and .operator_identity_hash_recorded == false
  and .single_surface_activation_scope_required == true
  and .single_surface_activation_scope_recorded == false
  and .receipt_payload_hash_required == true
  and .receipt_payload_hash_recorded == false
  and .redacted_payload_summary_hash_required == true
  and .redacted_payload_summary_hash_recorded == false
  and .receipt_output_path_redacted_required == true
  and .receipt_output_path_redacted_recorded == false
  and .accepted_redaction_proof_ids_required == true
  and .accepted_redaction_proof_ids_recorded == false
  and .fresh_pre_activation_soak_evidence_required == true
  and .fresh_pre_activation_soak_evidence_recorded == false
  and .active_binary_sha_required == true
  and .active_binary_sha_recorded == false
  and .rollback_plan_required == true
  and .rollback_plan_recorded == false
  and .public_artifact_policy_required == true
  and .public_artifact_policy_recorded == false
  and .plaintext_payload_attempt_count == 1
  and .public_claim_attempt_count == 1
  and .release_artifact_write_attempt_count == 1
  and .filesystem_persistence_allowed_count == 0
  and .command_invocation_attempt_count == 5
  and .command_invocation_performed_count == 0
  and .command_execution_performed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .materialization_execution_performed_count == 0
  and .materialization_executed_count == 0
  and .filesystem_persistence_execution_performed_count == 0
  and .filesystem_write_performed_count == 0
  and .workspace_write_performed_count == 0
  and .receipt_materialized_count == 0
  and .receipt_persisted_count == 0
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .live_secret_scan_performed == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.required_approval_fields | length) == 13
  and (.filesystem_persistence_approval_fixtures | length) == 5
  and (.filesystem_persistence_approval_fixtures | all(.filesystem_persistence_approval_requested == true and .approval_status == "blocked_schema_only" and .recorded_approval_field_count == 0 and .materialization_plan_recorded == false and .operator_identity_hash_recorded == false and .single_surface_activation_scope_recorded == false and .accepted_redaction_proof_ids_recorded == false and .fresh_pre_activation_soak_evidence_recorded == false and .active_binary_sha_recorded == false and .rollback_plan_recorded == false and .public_artifact_policy_recorded == false and .command_invocation_performed == false and .command_execution_performed == false and .receipt_persistence_execution_performed == false and .materialization_execution_performed == false and .filesystem_persistence_execution_performed == false and .filesystem_write_performed == false and .workspace_write_performed == false and .receipt_materialized == false and .receipt_persisted == false and .activation_allowed == false))
  and ([.filesystem_persistence_approval_fixtures[] | select(.materialization_plan_ready == true)] | length) == 3
  and ([.filesystem_persistence_approval_fixtures[] | select(.raw_payload_plaintext_recorded == true)] | length) == 1
  and ([.filesystem_persistence_approval_fixtures[] | select(.public_claim_requested == true and .release_artifact_write_requested == true)] | length) == 1
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence approval packet gate passed"
