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

REPLAY_DENIAL_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial-gate.sh
)"

replay_denial_report_sha256="$(sha256_text "$REPLAY_DENIAL_JSON")"
operator_packet_template_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template:$replay_denial_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$REPLAY_DENIAL_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_ready == true
    and $source.source_readiness_index_ready == true
    and $source.source_full_live_activation_enabled == false
    and $source.source_full_live_activation_status == "blocked_report_only"
    and $source.replay_allowed == false
    and $source.replay_accepted == false
    and $source.idempotency_key_registered == false
    and $source.idempotency_cache_written == false
    and $source.activation_authority_derived == false
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

packet_sections_json="$(
  jq -n '
    [
      {
        section_id:"operator_authority",
        required_fields:["operator_identity_hash","explicit_operator_approval_id","approval_scope","approval_timestamp","approval_nonce"],
        missing_reason:"explicit_operator_authority_not_recorded"
      },
      {
        section_id:"activation_scope",
        required_fields:["activation_request_id","memory_scope","intelligence_scope","kg_scope","single_use_activation_nonce"],
        missing_reason:"activation_scope_not_bound"
      },
      {
        section_id:"memory_live_mutation_controls",
        required_fields:["memory_store_write_enable_id","memory_store_rollback_plan_id","post_write_validation_plan_id","idempotency_replay_plan_id"],
        missing_reason:"memory_live_mutation_controls_not_accepted"
      },
      {
        section_id:"intelligence_context_controls",
        required_fields:["context_attachment_plan_id","prompt_preview_redaction_review_id","context_injection_approval_id","model_invocation_boundary_id"],
        missing_reason:"intelligence_context_controls_not_accepted"
      },
      {
        section_id:"kg_external_adapter_controls",
        required_fields:["kg_adapter_manifest_id","credential_reference_review_id","network_allowlist_id","external_write_rollback_plan_id","live_kg_write_validation_id"],
        missing_reason:"kg_external_adapter_controls_not_accepted"
      },
      {
        section_id:"release_install_boundary",
        required_fields:["no_public_release_claim_attestation","no_release_artifact_write_attestation","no_install_restart_attestation","active_binary_no_mutation_attestation"],
        missing_reason:"release_install_boundary_not_accepted"
      },
      {
        section_id:"fresh_evidence_and_soak",
        required_fields:["fresh_long_soak_sample_set_hash","readiness_index_report_sha256","replay_denial_report_sha256","fresh_evidence_timestamp"],
        missing_reason:"fresh_evidence_and_soak_not_accepted"
      },
      {
        section_id:"rollback_kill_switch",
        required_fields:["rollback_plan_id","rollback_dry_run_evidence_id","kill_switch_id","kill_switch_dry_run_evidence_id"],
        missing_reason:"rollback_kill_switch_not_accepted"
      },
      {
        section_id:"audit_receipt_chain",
        required_fields:["receipt_persistence_plan_id","ledger_record_plan_id","operator_review_plan_id","completion_ack_policy_id"],
        missing_reason:"audit_receipt_chain_not_accepted"
      },
      {
        section_id:"final_operator_review",
        required_fields:["final_review_packet_hash","human_readable_summary_hash","non_delegation_attestation","manual_acceptance_channel"],
        missing_reason:"final_operator_review_not_accepted"
      }
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_gate" \
  --arg replay_denial_report_sha256 "$replay_denial_report_sha256" \
  --arg operator_packet_template_hash_sha256 "$operator_packet_template_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$REPLAY_DENIAL_JSON" \
  --argjson sections "$packet_sections_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    operator_readiness_packet_template_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_v1",
    operator_readiness_packet_template_mode:"report_only_template_no_acceptance_no_activation",
    source_readiness_index_replay_idempotency_denial_gate:$source.gate,
    source_readiness_index_replay_idempotency_denial_ready:$source.memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_ready,
    source_readiness_index_replay_idempotency_denial_report_sha256:$replay_denial_report_sha256,
    operator_packet_template_hash_sha256:$operator_packet_template_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_ready:true,
    source_full_live_activation_enabled:$source.source_full_live_activation_enabled,
    source_full_live_activation_status:$source.source_full_live_activation_status,
    source_replay_allowed:$source.replay_allowed,
    source_activation_authority_derived:$source.activation_authority_derived,
    required_operator_packet_section_count:10,
    operator_packet_section_count:($sections | length),
    missing_operator_packet_section_count:($sections | length),
    accepted_operator_packet_section_count:0,
    recorded_operator_packet_section_count:0,
    operator_packet_required_field_count:([$sections[].required_fields[]] | length),
    operator_packet_recorded_field_count:0,
    operator_packet_accepted_field_count:0,
    operator_packet_sections:($sections | map(. + {
      status:"missing",
      operator_input_required:true,
      template_only:true,
      report_only:true,
      recorded:false,
      persisted:false,
      materialized:false,
      accepted:false,
      delivered:false,
      activation_authority:false,
      mutates_memory_store:false,
      writes_kg:false,
      attaches_intelligence_context:false,
      invokes_provider:false,
      reads_credentials:false,
      installs_or_restarts:false,
      publishes_artifacts:false,
      sends_external:false
    })),
    packet_template_recorded:false,
    packet_template_persisted:false,
    packet_template_materialized:false,
    packet_template_delivered:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    activation_authority_derived:false,
    activation_allowed:false,
    activation_performed:false,
    allowed_next_actions:[
      {
        action:"review_operator_readiness_packet_template",
        status:"allowed_report_only",
        records_operator_acceptance:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    denied_by_operator_readiness_packet_template:[
      "operator_packet_template_persistence_denied",
      "operator_packet_template_materialization_denied",
      "operator_packet_acceptance_recording_denied",
      "operator_packet_approval_recording_denied",
      "operator_packet_activation_authority_denied",
      "memory_live_mutation_from_template_denied",
      "kg_write_from_template_denied",
      "provider_model_from_template_denied",
      "credential_read_from_template_denied",
      "install_restart_active_binary_from_template_denied",
      "release_publication_from_template_denied",
      "external_send_from_template_denied"
    ],
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
      packet_template_recorded:false,
      packet_template_persisted:false,
      packet_template_materialized:false,
      packet_template_delivered:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      activation_authority_derived:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_ready == true
  and .source_readiness_index_replay_idempotency_denial_ready == true
  and .source_full_live_activation_enabled == false
  and .source_full_live_activation_status == "blocked_report_only"
  and .source_replay_allowed == false
  and .source_activation_authority_derived == false
  and .required_operator_packet_section_count == 10
  and .operator_packet_section_count == 10
  and .missing_operator_packet_section_count == 10
  and .accepted_operator_packet_section_count == 0
  and .recorded_operator_packet_section_count == 0
  and .operator_packet_required_field_count == 43
  and .operator_packet_recorded_field_count == 0
  and .operator_packet_accepted_field_count == 0
  and (.operator_packet_sections | all(
    .status == "missing"
    and .operator_input_required == true
    and .template_only == true
    and .report_only == true
    and .recorded == false
    and .persisted == false
    and .materialized == false
    and .accepted == false
    and .delivered == false
    and .activation_authority == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .attaches_intelligence_context == false
    and .invokes_provider == false
    and .reads_credentials == false
    and .installs_or_restarts == false
    and .publishes_artifacts == false
    and .sends_external == false
  ))
  and .packet_template_recorded == false
  and .packet_template_persisted == false
  and .packet_template_materialized == false
  and .packet_template_delivered == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and (.allowed_next_actions | all(.status == "allowed_report_only"))
  and (.denied_by_operator_readiness_packet_template | length) == 12
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template gate passed"
