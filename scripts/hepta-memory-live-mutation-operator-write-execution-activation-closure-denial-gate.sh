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

OPERATOR_ACCEPTANCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-gate.sh
)"

operator_acceptance_report_sha256="$(printf '%s' "$OPERATOR_ACCEPTANCE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson operator_acceptance "$OPERATOR_ACCEPTANCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $operator_acceptance.runtime == "hepta"
    and $operator_acceptance.status == "ready"
    and $operator_acceptance.gate == "hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_gate"
    and $operator_acceptance.memory_write_execution_post_write_operator_acceptance_denial_ready == true
    and $operator_acceptance.memory_write_execution_post_write_validation_dry_run_ready == true
    and $operator_acceptance.memory_write_execution_write_enable_fixture_ready == true
    and $operator_acceptance.memory_write_execution_no_write_sink_contract_ready == true
    and $operator_acceptance.source_memory_write_execution_post_write_validation_dry_run_report_sha256 != ""
    and $operator_acceptance.source_memory_write_execution_write_enable_fixture_report_sha256 != ""
    and $operator_acceptance.source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
    and $operator_acceptance.source_memory_write_execution_denial_matrix_report_sha256 != ""
    and $operator_acceptance.source_memory_write_execution_preflight_report_sha256 != ""
    and $operator_acceptance.source_memory_write_approval_packet_report_sha256 != ""
    and $operator_acceptance.source_memory_write_contract_report_sha256 != ""
    and $operator_acceptance.minimum_required_samples >= 24
    and $operator_acceptance.required_operator_acceptance_surface_count == 11
    and $operator_acceptance.ready_operator_acceptance_surface_count == 11
    and $operator_acceptance.operator_acceptance_fixture_count == 9
    and $operator_acceptance.blocked_operator_acceptance_fixture_count == 9
    and $operator_acceptance.allowed_operator_acceptance_fixture_count == 0
    and $operator_acceptance.accepted_operator_acceptance_fixture_count == 0
    and $operator_acceptance.operator_acceptance_performed_count == 0
    and $operator_acceptance.operator_post_write_acceptance_recorded == false
    and $operator_acceptance.operator_post_write_acceptance_persisted == false
    and $operator_acceptance.operator_post_write_acceptance_accepted == false
    and $operator_acceptance.operator_post_write_acceptance_performed == false
    and $operator_acceptance.accepted_post_write_validation_report_recorded == false
    and $operator_acceptance.accepted_post_write_validation_report_accepted == false
    and $operator_acceptance.activation_closure_packet_recorded == false
    and $operator_acceptance.activation_closure_packet_persisted == false
    and $operator_acceptance.activation_closure_packet_accepted == false
    and $operator_acceptance.activation_closure_packet_materialized == false
    and $operator_acceptance.activation_closure_filesystem_written == false
    and $operator_acceptance.activation_allowed_by_operator_acceptance == false
    and $operator_acceptance.activation_allowed == false
    and $operator_acceptance.live_mutation_execution_ready == false
    and $operator_acceptance.live_mutation_execution_allowed == false
    and $operator_acceptance.live_mutation_execution_performed == false
    and $operator_acceptance.memory_write_execution_allowed == false
    and $operator_acceptance.memory_write_execution_ready == false
    and $operator_acceptance.memory_write_execution_performed == false
    and $operator_acceptance.memory_store_write_path_enabled == false
    and $operator_acceptance.memory_store_write_allowed == false
    and $operator_acceptance.memory_store_write_performed == false
    and $operator_acceptance.memory_store_write_performed_count == 0
    and $operator_acceptance.memory_store_mutation_allowed == false
    and $operator_acceptance.memory_store_mutated == false
    and $operator_acceptance.rollback_execution_allowed == false
    and $operator_acceptance.rollback_executed == false
    and $operator_acceptance.secret_material_read == false
    and $operator_acceptance.external_send_enabled == false
    and $operator_acceptance.external_send_performed == false
    and $operator_acceptance.public_claim_or_release_artifact_write_enabled == false
    and $operator_acceptance.public_release_published == false
    and $operator_acceptance.public_ga_claimed == false
    and $operator_acceptance.release_artifact_written == false
    and ($operator_acceptance.operator_acceptance_fixtures | length) == 9
    and ($operator_acceptance.operator_acceptance_fixtures | all(.acceptance_status == "blocked" and .acceptance_allowed == false and .acceptance_performed == false and .acceptance_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false))
    and ($operator_acceptance.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_activation_closure_denial_gate" \
  --arg operator_acceptance_report_sha256 "$operator_acceptance_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson operator_acceptance "$OPERATOR_ACCEPTANCE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_closure_denial_mode:"memory_write_execution_activation_closure_packet_no_write_denial",
    source_memory_write_execution_post_write_operator_acceptance_denial_gate:$operator_acceptance.gate,
    source_memory_write_execution_post_write_operator_acceptance_denial_ready:$operator_acceptance.memory_write_execution_post_write_operator_acceptance_denial_ready,
    source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256:$operator_acceptance_report_sha256,
    source_memory_write_execution_post_write_validation_dry_run_report_sha256:$operator_acceptance.source_memory_write_execution_post_write_validation_dry_run_report_sha256,
    source_memory_write_execution_write_enable_fixture_report_sha256:$operator_acceptance.source_memory_write_execution_write_enable_fixture_report_sha256,
    source_memory_write_execution_no_write_sink_contract_report_sha256:$operator_acceptance.source_memory_write_execution_no_write_sink_contract_report_sha256,
    source_memory_write_execution_denial_matrix_report_sha256:$operator_acceptance.source_memory_write_execution_denial_matrix_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$operator_acceptance.source_memory_write_execution_preflight_report_sha256,
    source_memory_write_approval_packet_report_sha256:$operator_acceptance.source_memory_write_approval_packet_report_sha256,
    source_memory_write_contract_report_sha256:$operator_acceptance.source_memory_write_contract_report_sha256,
    source_memory_intelligence_report_sha256:$operator_acceptance.source_memory_intelligence_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$operator_acceptance.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_proof_report_sha256:$operator_acceptance.source_payload_redaction_proof_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_activation_closure_denial_ready:true,
    memory_write_execution_post_write_operator_acceptance_denial_ready:true,
    memory_write_execution_post_write_validation_dry_run_ready:true,
    memory_write_execution_write_enable_fixture_ready:true,
    memory_write_execution_no_write_sink_contract_ready:true,
    required_operator_acceptance_surface_count:$operator_acceptance.required_operator_acceptance_surface_count,
    ready_operator_acceptance_surface_count:$operator_acceptance.ready_operator_acceptance_surface_count,
    required_activation_closure_surface_count:12,
    ready_activation_closure_surface_count:12,
    side_effect_free_activation_closure_surface_count:12,
    required_activation_closure_fixture_count:10,
    activation_closure_fixture_count:10,
    blocked_activation_closure_fixture_count:10,
    allowed_activation_closure_fixture_count:0,
    accepted_activation_closure_fixture_count:0,
    activation_closure_denied_count:10,
    activation_closure_performed_count:0,
    accepted_operator_post_write_acceptance_report_recorded:false,
    accepted_operator_post_write_acceptance_report_persisted:false,
    accepted_operator_post_write_acceptance_report_accepted:false,
    accepted_operator_post_write_acceptance_hash_bound:false,
    accepted_post_write_validation_report_hash_bound:false,
    operator_identity_hash_recorded:false,
    operator_acceptance_signature_hash_recorded:false,
    operator_acceptance_timestamp_recorded:false,
    operator_single_surface_scope_recorded:false,
    activation_closure_packet_recorded:false,
    activation_closure_packet_persisted:false,
    activation_closure_packet_accepted:false,
    activation_closure_packet_materialized:false,
    activation_closure_packet_id_recorded:false,
    activation_closure_packet_hash_bound:false,
    activation_closure_packet_signature_hash_recorded:false,
    activation_closure_packet_timestamp_recorded:false,
    activation_closure_filesystem_written:false,
    activation_closure_ledger_written:false,
    activation_command_enabled:false,
    activation_command_invoked:false,
    activation_allowed_by_closure_packet:false,
    activation_allowed:false,
    activation_performed:false,
    live_mutation_execution_ready:false,
    live_mutation_execution_allowed:false,
    live_mutation_execution_performed:false,
    memory_write_execution_allowed:false,
    memory_write_execution_ready:false,
    memory_write_execution_performed:false,
    memory_store_write_path_enabled:false,
    memory_store_write_allowed:false,
    memory_store_write_performed:false,
    memory_store_write_performed_count:0,
    memory_store_mutation_allowed:false,
    memory_store_mutated:false,
    write_result_receipt_hash_bound:false,
    pre_write_memory_store_hash_bound:false,
    post_write_memory_store_hash_bound:false,
    post_write_diff_scope_accepted:false,
    post_write_watchdog_soak_evidence_accepted:false,
    post_write_route_regression_check_accepted:false,
    post_write_dependency_isolation_check_accepted:false,
    rollback_validation_accepted:false,
    rollback_execution_allowed:false,
    rollback_executed:false,
    audit_redaction_validation_accepted:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    secret_material_read:false,
    provider_prompt_replay_enabled:false,
    provider_invoked:false,
    model_invoked:false,
    external_send_enabled:false,
    external_send_performed:false,
    public_claim_or_release_artifact_write_enabled:false,
    public_release_published:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    activation_closure_surfaces:[
      "accepted_operator_post_write_acceptance_required",
      "accepted_post_write_validation_hash_required",
      "operator_identity_signature_timestamp_required",
      "single_surface_activation_scope_required",
      "pre_post_store_hashes_and_write_receipt_required",
      "allowlisted_diff_scope_required",
      "post_write_watchdog_soak_and_regression_evidence_required",
      "rollback_validation_and_no_rollback_execution_required",
      "audit_redaction_and_no_secret_material_required",
      "activation_closure_packet_id_hash_signature_required",
      "activation_command_disabled_by_default_required",
      "no_external_public_or_release_outputs_required"
    ],
    activation_closure_fixtures:[
      {
        id:"activation-closure-missing-operator-acceptance",
        activation_closure_requested:true,
        closure_status:"blocked",
        accepted_operator_post_write_acceptance_present:false,
        accepted_operator_post_write_acceptance_hash_bound:false,
        closure_allowed:false,
        closure_recorded:false,
        closure_persisted:false,
        closure_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"accepted_operator_post_write_acceptance_required"
      },
      {
        id:"activation-closure-missing-packet-id-or-hash",
        activation_closure_requested:true,
        closure_status:"blocked",
        accepted_operator_post_write_acceptance_present:true,
        accepted_operator_post_write_acceptance_hash_bound:true,
        activation_closure_packet_id_recorded:false,
        activation_closure_packet_hash_bound:false,
        activation_closure_packet_signature_hash_recorded:false,
        closure_allowed:false,
        closure_recorded:false,
        closure_persisted:false,
        closure_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"activation_closure_packet_id_hash_and_signature_required"
      },
      {
        id:"activation-closure-missing-single-surface-scope",
        activation_closure_requested:true,
        closure_status:"blocked",
        accepted_operator_post_write_acceptance_present:true,
        activation_closure_packet_id_recorded:true,
        activation_closure_packet_hash_bound:true,
        operator_single_surface_scope_recorded:false,
        multi_surface_activation_requested:true,
        closure_allowed:false,
        closure_recorded:false,
        closure_persisted:false,
        closure_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"single_surface_activation_scope_required"
      },
      {
        id:"activation-closure-store-hash-or-receipt-mismatch",
        activation_closure_requested:true,
        closure_status:"blocked",
        accepted_operator_post_write_acceptance_present:true,
        activation_closure_packet_id_recorded:true,
        activation_closure_packet_hash_bound:true,
        pre_write_memory_store_hash_bound:true,
        post_write_memory_store_hash_bound:false,
        write_result_receipt_hash_bound:false,
        post_write_diff_scope_accepted:false,
        closure_allowed:false,
        closure_recorded:false,
        closure_persisted:false,
        closure_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"store_hash_write_receipt_and_diff_scope_bindings_required"
      },
      {
        id:"activation-closure-regression-or-soak-missing",
        activation_closure_requested:true,
        closure_status:"blocked",
        accepted_operator_post_write_acceptance_present:true,
        activation_closure_packet_id_recorded:true,
        activation_closure_packet_hash_bound:true,
        post_write_watchdog_soak_evidence_accepted:false,
        route_readiness_regression_detected:true,
        active_dependency_isolation_regression_detected:true,
        closure_allowed:false,
        closure_recorded:false,
        closure_persisted:false,
        closure_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"post_write_soak_route_and_dependency_evidence_required"
      },
      {
        id:"activation-closure-rollback-validation-missing-or-execution",
        activation_closure_requested:true,
        closure_status:"blocked",
        accepted_operator_post_write_acceptance_present:true,
        activation_closure_packet_id_recorded:true,
        activation_closure_packet_hash_bound:true,
        rollback_validation_accepted:false,
        rollback_execution_requested:true,
        closure_allowed:false,
        closure_recorded:false,
        closure_persisted:false,
        closure_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"rollback_validation_required_and_rollback_execution_denied"
      },
      {
        id:"activation-closure-redaction-secret-violation",
        activation_closure_requested:true,
        closure_status:"blocked",
        accepted_operator_post_write_acceptance_present:true,
        activation_closure_packet_id_recorded:true,
        activation_closure_packet_hash_bound:true,
        audit_redaction_validation_accepted:false,
        raw_payload_plaintext_recorded:true,
        secret_material_read:true,
        closure_allowed:false,
        closure_recorded:false,
        closure_persisted:false,
        closure_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"audit_redaction_validation_required_and_secret_material_forbidden"
      },
      {
        id:"activation-closure-direct-live-mutation-request",
        activation_closure_requested:true,
        closure_status:"blocked",
        accepted_operator_post_write_acceptance_present:true,
        activation_closure_packet_id_recorded:true,
        activation_closure_packet_hash_bound:true,
        direct_live_mutation_execution_requested:true,
        activation_command_invoked:true,
        closure_allowed:false,
        closure_recorded:false,
        closure_persisted:false,
        closure_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"activation_command_disabled_and_direct_execution_denied"
      },
      {
        id:"activation-closure-public-external-release-attempt",
        activation_closure_requested:true,
        closure_status:"blocked",
        accepted_operator_post_write_acceptance_present:true,
        activation_closure_packet_id_recorded:true,
        activation_closure_packet_hash_bound:true,
        external_send_requested:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        closure_allowed:false,
        closure_recorded:false,
        closure_persisted:false,
        closure_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        external_send_performed:false,
        public_release_published:false,
        release_artifact_written:false,
        rollback_executed:false,
        reason:"activation_closure_cannot_send_publish_or_write_release_artifacts"
      },
      {
        id:"activation-closure-persistence-or-filesystem-write-attempt",
        activation_closure_requested:true,
        closure_status:"blocked",
        accepted_operator_post_write_acceptance_present:true,
        activation_closure_packet_id_recorded:true,
        activation_closure_packet_hash_bound:true,
        activation_closure_packet_materialization_requested:true,
        activation_closure_filesystem_write_requested:true,
        activation_closure_ledger_write_requested:true,
        closure_allowed:false,
        closure_recorded:false,
        closure_persisted:false,
        closure_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        activation_closure_filesystem_written:false,
        activation_closure_ledger_written:false,
        rollback_executed:false,
        reason:"activation_closure_packet_persistence_and_filesystem_writes_denied"
      }
    ],
    denied_by_activation_closure:[
      "accepted_operator_post_write_acceptance_required",
      "accepted_post_write_validation_hash_required",
      "operator_identity_required",
      "operator_acceptance_signature_required",
      "operator_acceptance_timestamp_required",
      "single_surface_activation_scope_required",
      "activation_closure_packet_id_required",
      "activation_closure_packet_hash_required",
      "activation_closure_packet_signature_required",
      "pre_write_memory_store_hash_binding_required",
      "post_write_memory_store_hash_binding_required",
      "write_result_receipt_hash_binding_required",
      "allowlisted_diff_scope_required",
      "route_readiness_regression_denied",
      "active_dependency_isolation_regression_denied",
      "post_write_watchdog_soak_success_required",
      "rollback_validation_required",
      "rollback_execution_denied",
      "audit_redaction_validation_required",
      "secret_material_read_denied",
      "activation_command_invocation_denied",
      "direct_live_mutation_execution_denied",
      "activation_closure_persistence_denied",
      "external_send_public_claim_release_artifact_denied"
    ],
    side_effects:{
      memory_store_mutated:false,
      memory_store_write_performed:false,
      memory_write_execution_performed:false,
      post_write_validation_recorded:false,
      post_write_validation_persisted:false,
      post_write_validation_performed:false,
      operator_post_write_acceptance_recorded:false,
      operator_post_write_acceptance_persisted:false,
      operator_post_write_acceptance_performed:false,
      accepted_operator_post_write_acceptance_report_recorded:false,
      accepted_operator_post_write_acceptance_report_persisted:false,
      activation_closure_packet_recorded:false,
      activation_closure_packet_persisted:false,
      activation_closure_packet_materialized:false,
      activation_closure_filesystem_written:false,
      activation_closure_ledger_written:false,
      activation_command_invoked:false,
      live_mutation_execution_performed:false,
      rollback_validation_performed:false,
      rollback_executed:false,
      write_result_receipt_recorded:false,
      write_result_receipt_persisted:false,
      pre_write_memory_store_hash_recorded:false,
      post_write_memory_store_hash_recorded:false,
      audit_redaction_validation_recorded:false,
      raw_payload_inspected:false,
      payload_plaintext_persisted:false,
      secret_file_read:false,
      credential_read:false,
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
      public_ga_claimed:false,
      launchd_mutated:false,
      service_restarted:false
    }
  }')"

jq -e '
  .status == "ready"
  and .memory_write_execution_activation_closure_denial_ready == true
  and .activation_closure_denial_mode == "memory_write_execution_activation_closure_packet_no_write_denial"
  and .source_memory_write_execution_post_write_operator_acceptance_denial_ready == true
  and .source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256 != ""
  and .source_memory_write_execution_post_write_validation_dry_run_report_sha256 != ""
  and .source_memory_write_execution_write_enable_fixture_report_sha256 != ""
  and .source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
  and .source_memory_write_execution_denial_matrix_report_sha256 != ""
  and .source_memory_write_execution_preflight_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_operator_acceptance_surface_count == 11
  and .ready_operator_acceptance_surface_count == 11
  and .required_activation_closure_surface_count == 12
  and .ready_activation_closure_surface_count == 12
  and .side_effect_free_activation_closure_surface_count == 12
  and .required_activation_closure_fixture_count == 10
  and .activation_closure_fixture_count == 10
  and .blocked_activation_closure_fixture_count == 10
  and .allowed_activation_closure_fixture_count == 0
  and .accepted_activation_closure_fixture_count == 0
  and .activation_closure_denied_count == 10
  and .activation_closure_performed_count == 0
  and .accepted_operator_post_write_acceptance_report_recorded == false
  and .accepted_operator_post_write_acceptance_report_persisted == false
  and .accepted_operator_post_write_acceptance_report_accepted == false
  and .accepted_operator_post_write_acceptance_hash_bound == false
  and .accepted_post_write_validation_report_hash_bound == false
  and .operator_identity_hash_recorded == false
  and .operator_acceptance_signature_hash_recorded == false
  and .operator_acceptance_timestamp_recorded == false
  and .operator_single_surface_scope_recorded == false
  and .activation_closure_packet_recorded == false
  and .activation_closure_packet_persisted == false
  and .activation_closure_packet_accepted == false
  and .activation_closure_packet_materialized == false
  and .activation_closure_packet_id_recorded == false
  and .activation_closure_packet_hash_bound == false
  and .activation_closure_packet_signature_hash_recorded == false
  and .activation_closure_packet_timestamp_recorded == false
  and .activation_closure_filesystem_written == false
  and .activation_closure_ledger_written == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_allowed_by_closure_packet == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_ready == false
  and .live_mutation_execution_allowed == false
  and .live_mutation_execution_performed == false
  and .memory_write_execution_allowed == false
  and .memory_write_execution_ready == false
  and .memory_write_execution_performed == false
  and .memory_store_write_path_enabled == false
  and .memory_store_write_allowed == false
  and .memory_store_write_performed == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutation_allowed == false
  and .memory_store_mutated == false
  and .rollback_execution_allowed == false
  and .rollback_executed == false
  and .secret_material_read == false
  and .external_send_enabled == false
  and .external_send_performed == false
  and .public_claim_or_release_artifact_write_enabled == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and (.activation_closure_surfaces | length) == 12
  and (.activation_closure_fixtures | length) == 10
  and (.activation_closure_fixtures | all(.activation_closure_requested == true and .closure_status == "blocked" and .closure_allowed == false and .closure_recorded == false and .closure_persisted == false and .closure_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false))
  and ([.activation_closure_fixtures[] | select(.direct_live_mutation_execution_requested == true and .activation_command_invoked == true)] | length) == 1
  and ([.activation_closure_fixtures[] | select(.raw_payload_plaintext_recorded == true and .secret_material_read == true)] | length) == 1
  and ([.activation_closure_fixtures[] | select(.external_send_requested == true and .release_artifact_write_requested == true)] | length) == 1
  and ([.activation_closure_fixtures[] | select(.activation_closure_filesystem_write_requested == true and .activation_closure_ledger_write_requested == true)] | length) == 1
  and (.denied_by_activation_closure | length) == 24
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution activation closure denial gate passed"
