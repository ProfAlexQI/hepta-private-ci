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

POST_WRITE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-post-write-validation-dry-run-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-post-write-validation-dry-run-gate.sh
)"

post_write_report_sha256="$(printf '%s' "$POST_WRITE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson post_write "$POST_WRITE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $post_write.runtime == "hepta"
    and $post_write.status == "ready"
    and $post_write.gate == "hepta_memory_live_mutation_operator_write_execution_post_write_validation_dry_run_gate"
    and $post_write.memory_write_execution_post_write_validation_dry_run_ready == true
    and $post_write.memory_write_execution_write_enable_fixture_ready == true
    and $post_write.memory_write_execution_no_write_sink_contract_ready == true
    and $post_write.source_memory_write_execution_write_enable_fixture_report_sha256 != ""
    and $post_write.source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
    and $post_write.source_memory_write_execution_denial_matrix_report_sha256 != ""
    and $post_write.source_memory_write_execution_preflight_report_sha256 != ""
    and $post_write.source_memory_write_approval_packet_report_sha256 != ""
    and $post_write.source_memory_write_contract_report_sha256 != ""
    and $post_write.source_memory_intelligence_report_sha256 != ""
    and $post_write.source_payload_redaction_acceptance_matrix_report_sha256 != ""
    and $post_write.source_payload_redaction_proof_report_sha256 != ""
    and $post_write.minimum_required_samples >= 24
    and $post_write.required_post_write_validation_surface_count == 9
    and $post_write.ready_post_write_validation_surface_count == 9
    and $post_write.post_write_validation_fixture_count == 8
    and $post_write.blocked_post_write_validation_fixture_count == 8
    and $post_write.allowed_post_write_validation_fixture_count == 0
    and $post_write.passed_post_write_validation_fixture_count == 0
    and $post_write.post_write_validation_performed_count == 0
    and $post_write.post_write_validation_recorded == false
    and $post_write.post_write_validation_persisted == false
    and $post_write.post_write_validation_accepted == false
    and $post_write.post_write_validation_performed == false
    and $post_write.post_write_watchdog_soak_performed == false
    and $post_write.post_write_watchdog_soak_passed == false
    and $post_write.post_write_route_regression_check_performed == false
    and $post_write.post_write_dependency_isolation_check_performed == false
    and $post_write.post_write_memory_store_hash_recorded == false
    and $post_write.write_result_receipt_accepted == false
    and $post_write.rollback_validation_performed == false
    and $post_write.audit_redaction_validation_recorded == false
    and $post_write.secret_material_read == false
    and $post_write.memory_write_execution_allowed == false
    and $post_write.memory_write_execution_performed == false
    and $post_write.memory_store_write_path_enabled == false
    and $post_write.memory_store_write_performed == false
    and $post_write.memory_store_mutated == false
    and $post_write.live_mutation_execution_ready == false
    and $post_write.rollback_execution_allowed == false
    and $post_write.rollback_executed == false
    and $post_write.external_send_enabled == false
    and $post_write.public_claim_or_release_artifact_write_enabled == false
    and ($post_write.post_write_validation_fixtures | length) == 8
    and ($post_write.post_write_validation_fixtures | all(.validation_status == "blocked" and .validation_allowed == false and .validation_performed == false and .validation_passed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .activation_allowed == false))
    and ($post_write.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_gate" \
  --arg post_write_report_sha256 "$post_write_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson post_write "$POST_WRITE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    operator_acceptance_denial_mode:"memory_write_execution_post_write_operator_acceptance_denial_non_activation",
    source_memory_write_execution_post_write_validation_dry_run_gate:$post_write.gate,
    source_memory_write_execution_post_write_validation_dry_run_ready:$post_write.memory_write_execution_post_write_validation_dry_run_ready,
    source_memory_write_execution_post_write_validation_dry_run_report_sha256:$post_write_report_sha256,
    source_memory_write_execution_write_enable_fixture_report_sha256:$post_write.source_memory_write_execution_write_enable_fixture_report_sha256,
    source_memory_write_execution_no_write_sink_contract_report_sha256:$post_write.source_memory_write_execution_no_write_sink_contract_report_sha256,
    source_memory_write_execution_denial_matrix_report_sha256:$post_write.source_memory_write_execution_denial_matrix_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$post_write.source_memory_write_execution_preflight_report_sha256,
    source_memory_write_approval_packet_report_sha256:$post_write.source_memory_write_approval_packet_report_sha256,
    source_memory_write_contract_report_sha256:$post_write.source_memory_write_contract_report_sha256,
    source_memory_intelligence_report_sha256:$post_write.source_memory_intelligence_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$post_write.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_proof_report_sha256:$post_write.source_payload_redaction_proof_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_post_write_operator_acceptance_denial_ready:true,
    memory_write_execution_post_write_validation_dry_run_ready:true,
    memory_write_execution_write_enable_fixture_ready:true,
    memory_write_execution_no_write_sink_contract_ready:true,
    required_post_write_validation_surface_count:$post_write.required_post_write_validation_surface_count,
    ready_post_write_validation_surface_count:$post_write.ready_post_write_validation_surface_count,
    required_operator_acceptance_surface_count:11,
    ready_operator_acceptance_surface_count:11,
    side_effect_free_operator_acceptance_surface_count:11,
    required_operator_acceptance_fixture_count:9,
    operator_acceptance_fixture_count:9,
    blocked_operator_acceptance_fixture_count:9,
    allowed_operator_acceptance_fixture_count:0,
    accepted_operator_acceptance_fixture_count:0,
    operator_acceptance_denied_count:9,
    operator_acceptance_performed_count:0,
    operator_post_write_acceptance_recorded:false,
    operator_post_write_acceptance_persisted:false,
    operator_post_write_acceptance_accepted:false,
    operator_post_write_acceptance_performed:false,
    operator_post_write_acceptance_materialized:false,
    operator_post_write_acceptance_filesystem_written:false,
    operator_identity_hash_recorded:false,
    operator_acceptance_signature_hash_recorded:false,
    operator_acceptance_timestamp_recorded:false,
    operator_single_surface_scope_recorded:false,
    accepted_post_write_validation_report_recorded:false,
    accepted_post_write_validation_report_persisted:false,
    accepted_post_write_validation_report_accepted:false,
    accepted_post_write_validation_report_hash_bound:false,
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
    activation_closure_packet_recorded:false,
    activation_closure_packet_persisted:false,
    activation_closure_packet_accepted:false,
    activation_closure_packet_materialized:false,
    activation_closure_filesystem_written:false,
    activation_allowed_by_operator_acceptance:false,
    activation_allowed:false,
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
    provider_prompt_replay_enabled:false,
    provider_invoked:false,
    model_invoked:false,
    external_send_enabled:false,
    external_send_performed:false,
    public_claim_or_release_artifact_write_enabled:false,
    public_release_published:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    operator_acceptance_surfaces:[
      "accepted_post_write_validation_report_required",
      "operator_identity_signature_timestamp_required",
      "single_surface_acceptance_scope_required",
      "pre_and_post_memory_store_hash_binding_required",
      "accepted_write_result_receipt_hash_required",
      "allowlisted_diff_scope_required",
      "post_write_watchdog_soak_success_required",
      "route_and_dependency_regression_absence_required",
      "rollback_validation_and_no_rollback_execution_required",
      "audit_redaction_and_no_secret_material_required",
      "activation_closure_packet_required"
    ],
    operator_acceptance_fixtures:[
      {
        id:"operator-acceptance-missing-post-write-validation",
        operator_acceptance_requested:true,
        acceptance_status:"blocked",
        accepted_post_write_validation_report_present:false,
        operator_identity_hash_recorded:false,
        validation_accepted:false,
        acceptance_allowed:false,
        acceptance_performed:false,
        acceptance_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"accepted_post_write_validation_report_required"
      },
      {
        id:"operator-acceptance-missing-operator-signature",
        operator_acceptance_requested:true,
        acceptance_status:"blocked",
        accepted_post_write_validation_report_present:true,
        operator_identity_hash_recorded:false,
        operator_acceptance_signature_hash_recorded:false,
        operator_acceptance_timestamp_recorded:false,
        operator_single_surface_scope_recorded:false,
        acceptance_allowed:false,
        acceptance_performed:false,
        acceptance_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"operator_identity_signature_timestamp_and_scope_required"
      },
      {
        id:"operator-acceptance-receipt-or-store-hash-mismatch",
        operator_acceptance_requested:true,
        acceptance_status:"blocked",
        accepted_post_write_validation_report_present:true,
        operator_identity_hash_recorded:true,
        operator_acceptance_signature_hash_recorded:true,
        operator_single_surface_scope_recorded:true,
        write_result_receipt_hash_bound:false,
        pre_write_memory_store_hash_bound:true,
        post_write_memory_store_hash_bound:true,
        post_write_diff_scope_accepted:false,
        acceptance_allowed:false,
        acceptance_performed:false,
        acceptance_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"write_receipt_store_hash_and_diff_scope_bindings_required"
      },
      {
        id:"operator-acceptance-route-or-dependency-regression",
        operator_acceptance_requested:true,
        acceptance_status:"blocked",
        accepted_post_write_validation_report_present:true,
        operator_identity_hash_recorded:true,
        operator_acceptance_signature_hash_recorded:true,
        route_readiness_regression_detected:true,
        active_dependency_isolation_regression_detected:true,
        acceptance_allowed:false,
        acceptance_performed:false,
        acceptance_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"route_readiness_and_dependency_isolation_must_remain_ready"
      },
      {
        id:"operator-acceptance-watchdog-soak-missing",
        operator_acceptance_requested:true,
        acceptance_status:"blocked",
        accepted_post_write_validation_report_present:true,
        operator_identity_hash_recorded:true,
        operator_acceptance_signature_hash_recorded:true,
        post_write_watchdog_soak_evidence_accepted:false,
        acceptance_allowed:false,
        acceptance_performed:false,
        acceptance_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"post_write_watchdog_soak_success_required"
      },
      {
        id:"operator-acceptance-rollback-validation-missing-or-execution",
        operator_acceptance_requested:true,
        acceptance_status:"blocked",
        accepted_post_write_validation_report_present:true,
        operator_identity_hash_recorded:true,
        operator_acceptance_signature_hash_recorded:true,
        rollback_validation_accepted:false,
        rollback_execution_requested:true,
        acceptance_allowed:false,
        acceptance_performed:false,
        acceptance_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"rollback_validation_required_and_rollback_execution_denied"
      },
      {
        id:"operator-acceptance-redaction-or-secret-violation",
        operator_acceptance_requested:true,
        acceptance_status:"blocked",
        accepted_post_write_validation_report_present:true,
        operator_identity_hash_recorded:true,
        operator_acceptance_signature_hash_recorded:true,
        audit_redaction_validation_accepted:false,
        raw_payload_plaintext_recorded:true,
        secret_material_read:true,
        acceptance_allowed:false,
        acceptance_performed:false,
        acceptance_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"audit_redaction_validation_required_and_secret_material_forbidden"
      },
      {
        id:"operator-acceptance-multisurface-or-direct-activation",
        operator_acceptance_requested:true,
        acceptance_status:"blocked",
        accepted_post_write_validation_report_present:true,
        operator_identity_hash_recorded:true,
        operator_acceptance_signature_hash_recorded:true,
        operator_single_surface_scope_recorded:false,
        multi_surface_activation_requested:true,
        direct_live_mutation_execution_requested:true,
        acceptance_allowed:false,
        acceptance_performed:false,
        acceptance_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"single_surface_scope_required_and_direct_activation_denied"
      },
      {
        id:"operator-acceptance-public-or-external-output-attempt",
        operator_acceptance_requested:true,
        acceptance_status:"blocked",
        accepted_post_write_validation_report_present:true,
        operator_identity_hash_recorded:true,
        operator_acceptance_signature_hash_recorded:true,
        external_send_requested:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        acceptance_allowed:false,
        acceptance_performed:false,
        acceptance_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        external_send_performed:false,
        public_release_published:false,
        release_artifact_written:false,
        rollback_executed:false,
        reason:"operator_acceptance_cannot_send_publish_or_write_release_artifacts"
      }
    ],
    denied_by_operator_acceptance:[
      "accepted_post_write_validation_report_required",
      "operator_identity_required",
      "operator_acceptance_signature_required",
      "operator_acceptance_timestamp_required",
      "single_surface_acceptance_scope_required",
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
      "multi_surface_activation_denied",
      "direct_live_mutation_execution_denied",
      "external_send_public_claim_release_artifact_denied",
      "activation_closure_packet_required",
      "live_mutation_execution_denied"
    ],
    side_effects:{
      memory_store_mutated:false,
      memory_store_write_performed:false,
      memory_write_execution_performed:false,
      post_write_validation_recorded:false,
      post_write_validation_persisted:false,
      post_write_validation_performed:false,
      post_write_validation_accepted:false,
      operator_post_write_acceptance_recorded:false,
      operator_post_write_acceptance_persisted:false,
      operator_post_write_acceptance_performed:false,
      operator_post_write_acceptance_materialized:false,
      operator_post_write_acceptance_filesystem_written:false,
      accepted_post_write_validation_report_recorded:false,
      accepted_post_write_validation_report_persisted:false,
      activation_closure_packet_recorded:false,
      activation_closure_packet_persisted:false,
      activation_closure_packet_materialized:false,
      activation_closure_filesystem_written:false,
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
  and .memory_write_execution_post_write_operator_acceptance_denial_ready == true
  and .operator_acceptance_denial_mode == "memory_write_execution_post_write_operator_acceptance_denial_non_activation"
  and .source_memory_write_execution_post_write_validation_dry_run_ready == true
  and .source_memory_write_execution_post_write_validation_dry_run_report_sha256 != ""
  and .source_memory_write_execution_write_enable_fixture_report_sha256 != ""
  and .source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
  and .source_memory_write_execution_denial_matrix_report_sha256 != ""
  and .source_memory_write_execution_preflight_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_post_write_validation_surface_count == 9
  and .ready_post_write_validation_surface_count == 9
  and .required_operator_acceptance_surface_count == 11
  and .ready_operator_acceptance_surface_count == 11
  and .side_effect_free_operator_acceptance_surface_count == 11
  and .required_operator_acceptance_fixture_count == 9
  and .operator_acceptance_fixture_count == 9
  and .blocked_operator_acceptance_fixture_count == 9
  and .allowed_operator_acceptance_fixture_count == 0
  and .accepted_operator_acceptance_fixture_count == 0
  and .operator_acceptance_denied_count == 9
  and .operator_acceptance_performed_count == 0
  and .operator_post_write_acceptance_recorded == false
  and .operator_post_write_acceptance_persisted == false
  and .operator_post_write_acceptance_accepted == false
  and .operator_post_write_acceptance_performed == false
  and .operator_post_write_acceptance_materialized == false
  and .operator_post_write_acceptance_filesystem_written == false
  and .accepted_post_write_validation_report_recorded == false
  and .accepted_post_write_validation_report_accepted == false
  and .accepted_post_write_validation_report_hash_bound == false
  and .write_result_receipt_hash_bound == false
  and .post_write_diff_scope_accepted == false
  and .post_write_watchdog_soak_evidence_accepted == false
  and .rollback_validation_accepted == false
  and .rollback_execution_allowed == false
  and .rollback_executed == false
  and .audit_redaction_validation_accepted == false
  and .secret_material_read == false
  and .activation_closure_packet_recorded == false
  and .activation_closure_packet_accepted == false
  and .activation_allowed_by_operator_acceptance == false
  and .activation_allowed == false
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
  and .external_send_enabled == false
  and .external_send_performed == false
  and .public_claim_or_release_artifact_write_enabled == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and (.operator_acceptance_surfaces | length) == 11
  and (.operator_acceptance_fixtures | length) == 9
  and (.operator_acceptance_fixtures | all(.operator_acceptance_requested == true and .acceptance_status == "blocked" and .acceptance_allowed == false and .acceptance_performed == false and .acceptance_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false))
  and ([.operator_acceptance_fixtures[] | select(.route_readiness_regression_detected == true)] | length) == 1
  and ([.operator_acceptance_fixtures[] | select(.direct_live_mutation_execution_requested == true)] | length) == 1
  and ([.operator_acceptance_fixtures[] | select(.raw_payload_plaintext_recorded == true and .secret_material_read == true)] | length) == 1
  and ([.operator_acceptance_fixtures[] | select(.external_send_requested == true and .release_artifact_write_requested == true)] | length) == 1
  and (.denied_by_operator_acceptance | length) == 21
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution post-write operator acceptance denial gate passed"
