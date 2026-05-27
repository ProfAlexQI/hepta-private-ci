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

NO_WRITE_SINK_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-no-write-sink-contract-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-no-write-sink-contract-gate.sh
)"

no_write_sink_report_sha256="$(printf '%s' "$NO_WRITE_SINK_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson sink "$NO_WRITE_SINK_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $sink.runtime == "hepta"
    and $sink.status == "ready"
    and $sink.gate == "hepta_memory_live_mutation_operator_write_execution_no_write_sink_contract_gate"
    and $sink.memory_write_execution_no_write_sink_contract_ready == true
    and $sink.source_memory_write_execution_denial_matrix_ready == true
    and $sink.source_memory_write_execution_denial_matrix_report_sha256 != ""
    and $sink.source_memory_write_execution_preflight_report_sha256 != ""
    and $sink.source_memory_write_approval_packet_report_sha256 != ""
    and $sink.source_memory_write_contract_report_sha256 != ""
    and $sink.source_memory_intelligence_report_sha256 != ""
    and $sink.source_payload_redaction_acceptance_matrix_report_sha256 != ""
    and $sink.source_payload_redaction_proof_report_sha256 != ""
    and $sink.minimum_required_samples >= 24
    and $sink.required_pre_execution_validation_check_count == 17
    and $sink.required_no_write_sink_surface_count == 8
    and $sink.ready_no_write_sink_surface_count == 8
    and $sink.no_write_sink_fixture_count == 6
    and $sink.no_write_sink_write_path_enabled_by_default == false
    and $sink.no_write_sink_persistence_enabled_by_default == false
    and $sink.memory_write_execution_allowed == false
    and $sink.memory_write_execution_ready == false
    and $sink.memory_write_execution_performed == false
    and $sink.memory_store_write_path_enabled == false
    and $sink.memory_store_write_performed_count == 0
    and $sink.memory_store_mutation_allowed == false
    and $sink.memory_store_mutated == false
    and $sink.live_mutation_execution_ready == false
    and $sink.rollback_execution_allowed == false
    and $sink.rollback_executed == false
    and $sink.external_send_enabled == false
    and $sink.public_claim_or_release_artifact_write_enabled == false
    and ($sink.no_write_sink_fixtures | length) == 6
    and ($sink.no_write_sink_fixtures | all(.execution_requested == true and .execution_allowed == false and .execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .activation_allowed == false))
    and ($sink.required_before_any_memory_write_execution | length) == 17
    and ($sink.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_gate" \
  --arg no_write_sink_report_sha256 "$no_write_sink_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson sink "$NO_WRITE_SINK_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    write_enable_fixture_mode:"memory_write_execution_write_enable_fixture_non_activation",
    source_memory_write_execution_no_write_sink_contract_gate:$sink.gate,
    source_memory_write_execution_no_write_sink_contract_ready:$sink.memory_write_execution_no_write_sink_contract_ready,
    source_memory_write_execution_no_write_sink_contract_report_sha256:$no_write_sink_report_sha256,
    source_memory_write_execution_denial_matrix_report_sha256:$sink.source_memory_write_execution_denial_matrix_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$sink.source_memory_write_execution_preflight_report_sha256,
    source_memory_write_approval_packet_report_sha256:$sink.source_memory_write_approval_packet_report_sha256,
    source_memory_write_contract_report_sha256:$sink.source_memory_write_contract_report_sha256,
    source_memory_intelligence_report_sha256:$sink.source_memory_intelligence_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$sink.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_proof_report_sha256:$sink.source_payload_redaction_proof_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_write_enable_fixture_ready:true,
    memory_write_execution_no_write_sink_contract_ready:true,
    memory_write_execution_denial_matrix_ready:$sink.memory_write_execution_denial_matrix_ready,
    required_pre_execution_validation_check_count:$sink.required_pre_execution_validation_check_count,
    accepted_pre_execution_validation_check_count:0,
    required_write_enable_surface_count:10,
    ready_write_enable_surface_count:10,
    side_effect_free_write_enable_surface_count:10,
    required_write_enable_fixture_count:7,
    write_enable_fixture_count:7,
    blocked_write_enable_fixture_count:7,
    allowed_write_enable_fixture_count:0,
    explicit_write_enable_requested_fixture_count:7,
    write_enable_denied_missing_approval_preflight_count:1,
    write_enable_denied_missing_operator_scope_count:1,
    write_enable_denied_allowlist_mismatch_count:1,
    write_enable_denied_payload_binding_count:1,
    write_enable_denied_stale_soak_rollback_validation_count:1,
    write_enable_denied_public_artifact_count:1,
    write_enable_denied_store_or_rollback_execution_count:1,
    memory_write_execution_denied_count:7,
    memory_write_execution_allowed_count:0,
    memory_write_execution_performed_count:0,
    memory_store_write_requested_fixture_count:7,
    memory_store_write_allowed_count:0,
    memory_store_write_performed_count:0,
    memory_store_mutation_allowed:false,
    memory_store_mutated:false,
    explicit_write_enablement_recorded:false,
    explicit_write_enablement_persisted:false,
    explicit_write_enablement_accepted:false,
    write_enable_fixture_recorded:false,
    write_enable_fixture_persisted:false,
    write_enable_fixture_materialized:false,
    write_enable_fixture_filesystem_written:false,
    memory_write_execution_no_write_sink_contract_recorded:false,
    memory_write_execution_no_write_sink_contract_persisted:false,
    pre_execution_validation_recorded:false,
    pre_execution_validation_persisted:false,
    pre_execution_validation_accepted:false,
    memory_write_approval_packet_recorded:false,
    memory_write_approval_packet_persisted:false,
    memory_write_approval_packet_accepted:false,
    memory_write_request_recorded:false,
    memory_write_request_accepted:false,
    memory_write_request_persisted:false,
    operator_approval_recorded:false,
    operator_identity_hash_recorded:false,
    operator_approval_signature_hash_recorded:false,
    operator_approval_timestamp_recorded:false,
    single_surface_activation_scope_recorded:false,
    memory_namespace_recorded:false,
    memory_write_operation_recorded:false,
    memory_retention_class_recorded:false,
    accepted_redaction_proof_recorded:false,
    accepted_redaction_proof_count:0,
    source_report_hash_bindings_recorded:false,
    raw_payload_sha256_bound:false,
    redacted_payload_summary_sha256_bound:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    fresh_pre_activation_soak_evidence_recorded:false,
    rollback_plan_recorded:false,
    post_write_validation_plan_recorded:false,
    post_write_watchdog_soak_plan_recorded:false,
    memory_write_execution_allowed:false,
    memory_write_execution_ready:false,
    memory_write_execution_performed:false,
    memory_store_write_path_enabled:false,
    no_write_sink_write_path_enabled_by_default:false,
    live_mutation_execution_ready:false,
    rollback_execution_allowed:false,
    rollback_executed:false,
    provider_prompt_replay_enabled:false,
    external_send_enabled:false,
    external_send_performed:false,
    public_claim_or_release_artifact_write_enabled:false,
    public_release_published:false,
    release_artifact_written:false,
    write_enable_surfaces:[
      "accepted_operator_approval_packet_required",
      "accepted_pre_execution_validation_record_required",
      "operator_identity_signature_timestamp_required",
      "single_surface_activation_scope_required",
      "namespace_operation_retention_allowlist_required",
      "accepted_redaction_proof_and_payload_hash_bindings_required",
      "source_report_hash_bindings_required",
      "fresh_soak_rollback_validation_required",
      "explicit_write_path_enablement_required",
      "post_write_watchdog_soak_plan_required"
    ],
    write_enable_fixtures:[
      {
        id:"write-enable-missing-approval-preflight",
        explicit_write_enable_requested:true,
        write_enable_status:"blocked",
        accepted_operator_approval_packet_present:false,
        accepted_pre_execution_validation_record_present:false,
        memory_store_write_requested:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_allowed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"accepted_operator_approval_packet_and_pre_execution_validation_required"
      },
      {
        id:"write-enable-missing-operator-scope",
        explicit_write_enable_requested:true,
        write_enable_status:"blocked",
        accepted_operator_approval_packet_present:true,
        accepted_pre_execution_validation_record_present:true,
        operator_identity_hash_recorded:false,
        operator_approval_signature_hash_recorded:false,
        single_surface_activation_scope_recorded:false,
        memory_store_write_requested:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_allowed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"operator_identity_signature_and_single_surface_scope_required"
      },
      {
        id:"write-enable-allowlist-mismatch",
        explicit_write_enable_requested:true,
        write_enable_status:"blocked",
        accepted_operator_approval_packet_present:true,
        accepted_pre_execution_validation_record_present:true,
        operator_identity_hash_recorded:true,
        single_surface_activation_scope_recorded:true,
        memory_namespace_allowed:false,
        memory_write_operation_allowed:false,
        memory_retention_class_allowed:false,
        memory_store_write_requested:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_allowed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"namespace_operation_and_retention_allowlists_required"
      },
      {
        id:"write-enable-payload-binding-missing-or-plaintext",
        explicit_write_enable_requested:true,
        write_enable_status:"blocked",
        accepted_operator_approval_packet_present:true,
        accepted_pre_execution_validation_record_present:true,
        operator_identity_hash_recorded:true,
        single_surface_activation_scope_recorded:true,
        memory_namespace_allowed:true,
        memory_write_operation_allowed:true,
        memory_retention_class_allowed:true,
        accepted_redaction_proof_count:0,
        raw_payload_sha256_bound:false,
        redacted_payload_summary_sha256_bound:false,
        raw_payload_plaintext_recorded:true,
        memory_store_write_requested:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_allowed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"accepted_redaction_proof_and_payload_hash_binding_without_plaintext_required"
      },
      {
        id:"write-enable-stale-soak-or-missing-rollback-validation",
        explicit_write_enable_requested:true,
        write_enable_status:"blocked",
        accepted_operator_approval_packet_present:true,
        accepted_pre_execution_validation_record_present:true,
        operator_identity_hash_recorded:true,
        single_surface_activation_scope_recorded:true,
        memory_namespace_allowed:true,
        memory_write_operation_allowed:true,
        memory_retention_class_allowed:true,
        accepted_redaction_proof_count:1,
        raw_payload_sha256_bound:true,
        redacted_payload_summary_sha256_bound:true,
        fresh_pre_activation_soak_evidence_recorded:false,
        rollback_plan_recorded:false,
        post_write_validation_plan_recorded:false,
        memory_store_write_requested:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_allowed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"fresh_soak_rollback_and_post_write_validation_required"
      },
      {
        id:"write-enable-public-artifact-or-external-send",
        explicit_write_enable_requested:true,
        write_enable_status:"blocked",
        accepted_operator_approval_packet_present:true,
        accepted_pre_execution_validation_record_present:true,
        accepted_redaction_proof_count:1,
        external_send_requested:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        memory_store_write_requested:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_allowed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        external_send_performed:false,
        public_release_published:false,
        release_artifact_written:false,
        activation_allowed:false,
        reason:"external_send_public_claim_and_release_artifact_denied"
      },
      {
        id:"write-enable-direct-store-or-rollback-execution",
        explicit_write_enable_requested:true,
        write_enable_status:"blocked",
        accepted_operator_approval_packet_present:true,
        accepted_pre_execution_validation_record_present:true,
        accepted_redaction_proof_count:1,
        explicit_write_path_enablement_recorded:true,
        post_write_watchdog_soak_plan_recorded:false,
        memory_store_mutation_requested:true,
        rollback_execution_requested:true,
        memory_store_write_requested:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_allowed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"post_write_watchdog_soak_plan_and_live_activation_boundary_required"
      }
    ],
    denied_by_write_enable_fixture:[
      "accepted_operator_approval_packet_required",
      "accepted_pre_execution_validation_record_required",
      "operator_identity_signature_scope_required",
      "namespace_operation_retention_allowlists_required",
      "accepted_redaction_proof_required",
      "payload_hash_binding_without_plaintext_required",
      "source_report_hash_bindings_required",
      "fresh_soak_rollback_validation_required",
      "external_send_public_claim_release_artifact_denied",
      "direct_store_mutation_denied",
      "rollback_execution_denied",
      "post_write_watchdog_soak_plan_required",
      "live_mutation_execution_denied"
    ],
    side_effects:{
      memory_store_mutated:false,
      memory_store_write_performed:false,
      memory_write_request_recorded:false,
      memory_write_request_persisted:false,
      memory_write_approval_packet_recorded:false,
      memory_write_approval_packet_persisted:false,
      memory_write_execution_preflight_recorded:false,
      memory_write_execution_preflight_persisted:false,
      memory_write_execution_denial_matrix_recorded:false,
      memory_write_execution_denial_matrix_persisted:false,
      memory_write_execution_no_write_sink_contract_recorded:false,
      memory_write_execution_no_write_sink_contract_persisted:false,
      memory_write_execution_write_enable_fixture_recorded:false,
      memory_write_execution_write_enable_fixture_persisted:false,
      memory_write_execution_write_enable_fixture_materialized:false,
      memory_write_execution_write_enable_fixture_filesystem_written:false,
      explicit_write_enablement_recorded:false,
      explicit_write_enablement_persisted:false,
      pre_execution_validation_recorded:false,
      pre_execution_validation_persisted:false,
      payload_plaintext_persisted:false,
      raw_payload_inspected:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      skill_workshop_written:false,
      provider_invoked:false,
      model_invoked:false,
      provider_prompt_replayed:false,
      channel_send_performed:false,
      external_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      filesystem_written:false,
      release_artifact_written:false,
      public_artifact_written:false,
      public_release_published:false,
      approval_record_persisted:false,
      preflight_record_persisted:false,
      denial_matrix_persisted:false,
      no_write_sink_contract_persisted:false,
      write_enable_fixture_persisted:false,
      receipt_persisted:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      credential_read:false,
      secret_file_read:false
    }
  }')"

jq -e '
  .status == "ready"
  and .memory_write_execution_write_enable_fixture_ready == true
  and .write_enable_fixture_mode == "memory_write_execution_write_enable_fixture_non_activation"
  and .source_memory_write_execution_no_write_sink_contract_ready == true
  and .source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
  and .source_memory_write_execution_denial_matrix_report_sha256 != ""
  and .source_memory_write_execution_preflight_report_sha256 != ""
  and .source_memory_write_approval_packet_report_sha256 != ""
  and .source_memory_write_contract_report_sha256 != ""
  and .source_memory_intelligence_report_sha256 != ""
  and .source_payload_redaction_acceptance_matrix_report_sha256 != ""
  and .source_payload_redaction_proof_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_pre_execution_validation_check_count == 17
  and .accepted_pre_execution_validation_check_count == 0
  and .required_write_enable_surface_count == 10
  and .ready_write_enable_surface_count == 10
  and .side_effect_free_write_enable_surface_count == 10
  and .required_write_enable_fixture_count == 7
  and .write_enable_fixture_count == 7
  and .blocked_write_enable_fixture_count == 7
  and .allowed_write_enable_fixture_count == 0
  and .explicit_write_enable_requested_fixture_count == 7
  and .memory_write_execution_denied_count == 7
  and .memory_write_execution_allowed_count == 0
  and .memory_write_execution_performed_count == 0
  and .memory_store_write_requested_fixture_count == 7
  and .memory_store_write_allowed_count == 0
  and .memory_store_write_performed_count == 0
  and .memory_store_mutation_allowed == false
  and .memory_store_mutated == false
  and .explicit_write_enablement_recorded == false
  and .explicit_write_enablement_persisted == false
  and .explicit_write_enablement_accepted == false
  and .write_enable_fixture_recorded == false
  and .write_enable_fixture_persisted == false
  and .write_enable_fixture_materialized == false
  and .write_enable_fixture_filesystem_written == false
  and .memory_write_approval_packet_accepted == false
  and .memory_write_request_accepted == false
  and .operator_approval_recorded == false
  and .accepted_redaction_proof_count == 0
  and .source_report_hash_bindings_recorded == false
  and .raw_payload_sha256_bound == false
  and .redacted_payload_summary_sha256_bound == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .memory_write_execution_allowed == false
  and .memory_write_execution_ready == false
  and .memory_write_execution_performed == false
  and .memory_store_write_path_enabled == false
  and .no_write_sink_write_path_enabled_by_default == false
  and .live_mutation_execution_ready == false
  and .rollback_execution_allowed == false
  and .rollback_executed == false
  and .external_send_enabled == false
  and .external_send_performed == false
  and .public_claim_or_release_artifact_write_enabled == false
  and .public_release_published == false
  and .release_artifact_written == false
  and (.write_enable_surfaces | length) == 10
  and (.write_enable_fixtures | length) == 7
  and (.write_enable_fixtures | all(.explicit_write_enable_requested == true and .write_enable_status == "blocked" and .execution_allowed == false and .execution_performed == false and .memory_store_write_allowed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .activation_allowed == false))
  and ([.write_enable_fixtures[] | select(.raw_payload_plaintext_recorded == true)] | length) == 1
  and ([.write_enable_fixtures[] | select(.public_claim_requested == true and .release_artifact_write_requested == true)] | length) == 1
  and ([.write_enable_fixtures[] | select(.rollback_execution_requested == true)] | length) == 1
  and (.denied_by_write_enable_fixture | length) == 13
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution write-enable fixture gate passed"
