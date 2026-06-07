#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

NON_ACCEPTANCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-non-acceptance-authority-replay-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-non-acceptance-authority-replay-denial-gate.sh
)"

non_acceptance_report_sha256="$(sha256_text "$NON_ACCEPTANCE_JSON")"
field_validation_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-field-validation-denial:$non_acceptance_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$NON_ACCEPTANCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_ready == true
    and $source.source_operator_readiness_packet_template_ready == true
    and $source.source_operator_packet_section_count == 10
    and $source.source_operator_packet_required_field_count == 43
    and $source.source_operator_packet_recorded_field_count == 0
    and $source.source_operator_packet_accepted_field_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

field_specs_json="$(
  jq -n '
    {
      operator_authority:["operator_identity_hash","explicit_operator_approval_id","approval_scope","approval_timestamp","approval_nonce"],
      activation_scope:["activation_request_id","memory_scope","intelligence_scope","kg_scope","single_use_activation_nonce"],
      memory_live_mutation_controls:["memory_store_write_enable_id","memory_store_rollback_plan_id","post_write_validation_plan_id","idempotency_replay_plan_id"],
      intelligence_context_controls:["context_attachment_plan_id","prompt_preview_redaction_review_id","context_injection_approval_id","model_invocation_boundary_id"],
      kg_external_adapter_controls:["kg_adapter_manifest_id","credential_reference_review_id","network_allowlist_id","external_write_rollback_plan_id","live_kg_write_validation_id"],
      release_install_boundary:["no_public_release_claim_attestation","no_release_artifact_write_attestation","no_install_restart_attestation","active_binary_no_mutation_attestation"],
      fresh_evidence_and_soak:["fresh_long_soak_sample_set_hash","readiness_index_report_sha256","replay_denial_report_sha256","fresh_evidence_timestamp"],
      rollback_kill_switch:["rollback_plan_id","rollback_dry_run_evidence_id","kill_switch_id","kill_switch_dry_run_evidence_id"],
      audit_receipt_chain:["receipt_persistence_plan_id","ledger_record_plan_id","operator_review_plan_id","completion_ack_policy_id"],
      final_operator_review:["final_review_packet_hash","human_readable_summary_hash","non_delegation_attestation","manual_acceptance_channel"]
    }
    | to_entries
    | map(. as $section | .value[] | {
        section_id: $section.key,
        field_name: .,
        field_present: false,
        field_value_captured: false,
        field_value_hash_recorded: false,
        field_shape_validated: false,
        field_required: true,
        field_missing: true,
        field_recorded: false,
        field_persisted: false,
        field_accepted: false,
        field_authority_derived: false,
        field_live_execution_allowed: false,
        validation_status: "missing_denied",
        denial_reason: "operator_readiness_packet_template_field_value_not_recorded"
      })
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_gate" \
  --arg non_acceptance_report_sha256 "$non_acceptance_report_sha256" \
  --arg field_validation_contract_hash_sha256 "$field_validation_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$NON_ACCEPTANCE_JSON" \
  --argjson fields "$field_specs_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    field_validation_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_v1",
    field_validation_mode:"required_field_shape_matrix_no_values_no_persistence_no_acceptance_no_authority",
    source_template_non_acceptance_gate:$source.gate,
    source_template_non_acceptance_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_ready,
    source_template_non_acceptance_report_sha256:$non_acceptance_report_sha256,
    field_validation_contract_hash_sha256:$field_validation_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_ready:true,
    source_operator_packet_section_count:$source.source_operator_packet_section_count,
    source_operator_packet_required_field_count:$source.source_operator_packet_required_field_count,
    required_field_count:43,
    field_validation_matrix_count:($fields | length),
    missing_field_count:($fields | length),
    present_field_count:0,
    captured_field_value_count:0,
    recorded_field_hash_count:0,
    shape_validated_field_count:0,
    accepted_field_count:0,
    authority_derived_field_count:0,
    live_execution_allowed_field_count:0,
    section_validation_count:10,
    required_field_validation_matrix:$fields,
    denied_by_field_validation:[
      "operator_readiness_packet_template_field_value_capture_denied",
      "operator_readiness_packet_template_field_hash_recording_denied",
      "operator_readiness_packet_template_field_shape_acceptance_denied",
      "operator_readiness_packet_template_field_persistence_denied",
      "operator_readiness_packet_template_field_operator_acceptance_denied",
      "operator_readiness_packet_template_field_authority_derivation_denied",
      "operator_readiness_packet_template_field_live_execution_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_section_completion_non_acceptance_gate",
        status:"allowed_report_only_next_slice",
        records_operator_acceptance:false,
        activates_live:false,
        persists_field_values:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_template_recorded:false,
    packet_template_persisted:false,
    packet_template_materialized:false,
    packet_template_delivered:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    activation_authority_derived:false,
    activation_command_derived:false,
    activation_allowed:false,
    activation_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    hepta_intelligence_context_attached:false,
    prompt_preview_rendered:false,
    context_injection_performed:false,
    provider_invoked:false,
    model_invoked:false,
    external_kg_adapter_read_performed:false,
    external_adapter_client_constructed:false,
    network_call_performed:false,
    external_db_write_performed:false,
    live_kg_write_performed:false,
    credential_read:false,
    secret_file_read:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    public_release_claimed:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    public_artifact_written:false,
    external_send_performed:false,
    side_effects:{
      field_value_captured:false,
      field_value_hash_recorded:false,
      field_shape_accepted:false,
      field_value_persisted:false,
      field_acceptance_recorded:false,
      field_authority_derived:false,
      field_live_execution_allowed:false,
      packet_template_recorded:false,
      packet_template_persisted:false,
      packet_template_materialized:false,
      packet_template_delivered:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      activation_authority_derived:false,
      activation_command_derived:false,
      activation_allowed:false,
      activation_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      hepta_intelligence_context_attached:false,
      prompt_preview_rendered:false,
      context_injection_performed:false,
      provider_invoked:false,
      model_invoked:false,
      external_kg_adapter_read_performed:false,
      external_adapter_client_constructed:false,
      network_call_performed:false,
      external_db_write_performed:false,
      live_kg_write_performed:false,
      credential_read:false,
      secret_file_read:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      external_send_performed:false,
      filesystem_written:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_ready == true
  and .source_template_non_acceptance_ready == true
  and .source_operator_packet_section_count == 10
  and .source_operator_packet_required_field_count == 43
  and .required_field_count == 43
  and .field_validation_matrix_count == 43
  and .missing_field_count == 43
  and .present_field_count == 0
  and .captured_field_value_count == 0
  and .recorded_field_hash_count == 0
  and .shape_validated_field_count == 0
  and .accepted_field_count == 0
  and .authority_derived_field_count == 0
  and .live_execution_allowed_field_count == 0
  and .section_validation_count == 10
  and (.required_field_validation_matrix | all(
    .field_required == true
    and .field_missing == true
    and .field_present == false
    and .field_value_captured == false
    and .field_value_hash_recorded == false
    and .field_shape_validated == false
    and .field_recorded == false
    and .field_persisted == false
    and .field_accepted == false
    and .field_authority_derived == false
    and .field_live_execution_allowed == false
    and .validation_status == "missing_denied"
  ))
  and (.denied_by_field_validation | length) == 7
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_template_recorded == false
  and .packet_template_persisted == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .hepta_intelligence_context_attached == false
  and .prompt_preview_rendered == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_kg_adapter_read_performed == false
  and .external_adapter_client_constructed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .secret_file_read == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template field validation denial gate passed"
