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

WRITE_ENABLE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-gate.sh
)"

write_enable_report_sha256="$(printf '%s' "$WRITE_ENABLE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson write_enable "$WRITE_ENABLE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $write_enable.runtime == "hepta"
    and $write_enable.status == "ready"
    and $write_enable.gate == "hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_gate"
    and $write_enable.memory_write_execution_write_enable_fixture_ready == true
    and $write_enable.memory_write_execution_no_write_sink_contract_ready == true
    and $write_enable.source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
    and $write_enable.source_memory_write_execution_denial_matrix_report_sha256 != ""
    and $write_enable.source_memory_write_execution_preflight_report_sha256 != ""
    and $write_enable.source_memory_write_approval_packet_report_sha256 != ""
    and $write_enable.source_memory_write_contract_report_sha256 != ""
    and $write_enable.source_memory_intelligence_report_sha256 != ""
    and $write_enable.source_payload_redaction_acceptance_matrix_report_sha256 != ""
    and $write_enable.source_payload_redaction_proof_report_sha256 != ""
    and $write_enable.minimum_required_samples >= 24
    and $write_enable.required_write_enable_surface_count == 10
    and $write_enable.ready_write_enable_surface_count == 10
    and $write_enable.write_enable_fixture_count == 7
    and $write_enable.blocked_write_enable_fixture_count == 7
    and $write_enable.allowed_write_enable_fixture_count == 0
    and $write_enable.memory_write_execution_allowed == false
    and $write_enable.memory_write_execution_ready == false
    and $write_enable.memory_write_execution_performed == false
    and $write_enable.memory_store_write_path_enabled == false
    and $write_enable.memory_store_write_performed_count == 0
    and $write_enable.memory_store_mutation_allowed == false
    and $write_enable.memory_store_mutated == false
    and $write_enable.live_mutation_execution_ready == false
    and $write_enable.rollback_execution_allowed == false
    and $write_enable.rollback_executed == false
    and $write_enable.external_send_enabled == false
    and $write_enable.public_claim_or_release_artifact_write_enabled == false
    and ($write_enable.write_enable_fixtures | length) == 7
    and ($write_enable.write_enable_fixtures | all(.write_enable_status == "blocked" and .execution_allowed == false and .execution_performed == false and .memory_store_write_allowed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .activation_allowed == false))
    and ($write_enable.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_post_write_validation_dry_run_gate" \
  --arg write_enable_report_sha256 "$write_enable_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson write_enable "$WRITE_ENABLE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    post_write_validation_mode:"memory_write_execution_post_write_validation_dry_run_non_activation",
    source_memory_write_execution_write_enable_fixture_gate:$write_enable.gate,
    source_memory_write_execution_write_enable_fixture_ready:$write_enable.memory_write_execution_write_enable_fixture_ready,
    source_memory_write_execution_write_enable_fixture_report_sha256:$write_enable_report_sha256,
    source_memory_write_execution_no_write_sink_contract_report_sha256:$write_enable.source_memory_write_execution_no_write_sink_contract_report_sha256,
    source_memory_write_execution_denial_matrix_report_sha256:$write_enable.source_memory_write_execution_denial_matrix_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$write_enable.source_memory_write_execution_preflight_report_sha256,
    source_memory_write_approval_packet_report_sha256:$write_enable.source_memory_write_approval_packet_report_sha256,
    source_memory_write_contract_report_sha256:$write_enable.source_memory_write_contract_report_sha256,
    source_memory_intelligence_report_sha256:$write_enable.source_memory_intelligence_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$write_enable.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_proof_report_sha256:$write_enable.source_payload_redaction_proof_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_post_write_validation_dry_run_ready:true,
    memory_write_execution_write_enable_fixture_ready:true,
    memory_write_execution_no_write_sink_contract_ready:true,
    required_write_enable_surface_count:$write_enable.required_write_enable_surface_count,
    ready_write_enable_surface_count:$write_enable.ready_write_enable_surface_count,
    required_post_write_validation_surface_count:9,
    ready_post_write_validation_surface_count:9,
    side_effect_free_post_write_validation_surface_count:9,
    required_post_write_validation_fixture_count:8,
    post_write_validation_fixture_count:8,
    blocked_post_write_validation_fixture_count:8,
    allowed_post_write_validation_fixture_count:0,
    passed_post_write_validation_fixture_count:0,
    post_write_validation_denied_count:8,
    post_write_validation_performed_count:0,
    post_write_validation_recorded:false,
    post_write_validation_persisted:false,
    post_write_validation_accepted:false,
    post_write_validation_performed:false,
    post_write_validation_report_written:false,
    post_write_watchdog_soak_plan_recorded:false,
    post_write_watchdog_soak_plan_persisted:false,
    post_write_watchdog_soak_performed:false,
    post_write_watchdog_soak_passed:false,
    post_write_route_regression_check_performed:false,
    post_write_route_regression_passed:false,
    post_write_dependency_isolation_check_performed:false,
    post_write_dependency_isolation_passed:false,
    post_write_memory_store_hash_recorded:false,
    post_write_memory_store_hash_persisted:false,
    post_write_memory_store_hash_changed:false,
    pre_write_memory_store_hash_recorded:false,
    write_result_receipt_hash_recorded:false,
    write_result_receipt_accepted:false,
    rollback_validation_plan_recorded:false,
    rollback_validation_performed:false,
    rollback_validation_passed:false,
    audit_redaction_validation_recorded:false,
    audit_redaction_validation_passed:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    secret_material_read:false,
    memory_write_execution_allowed:false,
    memory_write_execution_ready:false,
    memory_write_execution_performed:false,
    memory_store_write_path_enabled:false,
    memory_store_write_allowed:false,
    memory_store_write_performed:false,
    memory_store_write_performed_count:0,
    memory_store_mutation_allowed:false,
    memory_store_mutated:false,
    live_mutation_execution_ready:false,
    rollback_execution_allowed:false,
    rollback_executed:false,
    provider_prompt_replay_enabled:false,
    provider_invoked:false,
    model_invoked:false,
    external_send_enabled:false,
    external_send_performed:false,
    public_claim_or_release_artifact_write_enabled:false,
    public_release_published:false,
    release_artifact_written:false,
    post_write_validation_surfaces:[
      "pre_write_memory_store_baseline_hash_required",
      "accepted_write_result_receipt_hash_required",
      "post_write_memory_store_hash_and_diff_scope_required",
      "route_readiness_regression_check_required",
      "active_dependency_isolation_regression_check_required",
      "post_write_watchdog_soak_plan_required",
      "rollback_validation_plan_required",
      "audit_redaction_validation_required",
      "operator_post_write_acceptance_required"
    ],
    post_write_validation_fixtures:[
      {
        id:"post-write-missing-pre-write-baseline",
        post_write_validation_requested:true,
        validation_status:"blocked",
        pre_write_memory_store_hash_recorded:false,
        write_result_receipt_hash_recorded:false,
        post_write_memory_store_hash_recorded:false,
        validation_allowed:false,
        validation_performed:false,
        validation_passed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"pre_write_baseline_and_write_receipt_required"
      },
      {
        id:"post-write-missing-write-result-receipt",
        post_write_validation_requested:true,
        validation_status:"blocked",
        pre_write_memory_store_hash_recorded:true,
        write_result_receipt_hash_recorded:false,
        write_result_receipt_accepted:false,
        post_write_memory_store_hash_recorded:true,
        validation_allowed:false,
        validation_performed:false,
        validation_passed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"accepted_write_result_receipt_hash_required"
      },
      {
        id:"post-write-store-hash-mismatch",
        post_write_validation_requested:true,
        validation_status:"blocked",
        pre_write_memory_store_hash_recorded:true,
        write_result_receipt_hash_recorded:true,
        write_result_receipt_accepted:true,
        post_write_memory_store_hash_recorded:true,
        post_write_memory_store_hash_changed:true,
        diff_scope_allowlisted:false,
        validation_allowed:false,
        validation_performed:false,
        validation_passed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"post_write_store_hash_change_requires_allowlisted_diff_scope"
      },
      {
        id:"post-write-route-or-dependency-regression",
        post_write_validation_requested:true,
        validation_status:"blocked",
        route_readiness_regression_detected:true,
        active_dependency_isolation_regression_detected:true,
        validation_allowed:false,
        validation_performed:false,
        validation_passed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"route_readiness_and_dependency_isolation_must_remain_ready"
      },
      {
        id:"post-write-watchdog-soak-missing-or-failed",
        post_write_validation_requested:true,
        validation_status:"blocked",
        post_write_watchdog_soak_plan_recorded:false,
        post_write_watchdog_soak_performed:false,
        post_write_watchdog_soak_passed:false,
        validation_allowed:false,
        validation_performed:false,
        validation_passed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"post_write_watchdog_soak_plan_and_success_required"
      },
      {
        id:"post-write-rollback-validation-missing",
        post_write_validation_requested:true,
        validation_status:"blocked",
        rollback_validation_plan_recorded:false,
        rollback_validation_performed:false,
        rollback_validation_passed:false,
        rollback_execution_requested:true,
        validation_allowed:false,
        validation_performed:false,
        validation_passed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"rollback_validation_plan_required_without_executing_rollback"
      },
      {
        id:"post-write-audit-redaction-or-secret-leak",
        post_write_validation_requested:true,
        validation_status:"blocked",
        audit_redaction_validation_recorded:false,
        raw_payload_plaintext_recorded:true,
        secret_material_read:true,
        validation_allowed:false,
        validation_performed:false,
        validation_passed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"audit_redaction_validation_required_and_secret_material_forbidden"
      },
      {
        id:"post-write-external-send-or-release-artifact-attempt",
        post_write_validation_requested:true,
        validation_status:"blocked",
        external_send_requested:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        validation_allowed:false,
        validation_performed:false,
        validation_passed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        external_send_performed:false,
        public_release_published:false,
        release_artifact_written:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"post_write_validation_cannot_send_publish_or_write_release_artifacts"
      }
    ],
    denied_by_post_write_validation:[
      "pre_write_memory_store_baseline_hash_required",
      "accepted_write_result_receipt_hash_required",
      "post_write_memory_store_hash_required",
      "allowlisted_diff_scope_required",
      "route_readiness_regression_denied",
      "active_dependency_isolation_regression_denied",
      "post_write_watchdog_soak_plan_required",
      "post_write_watchdog_soak_success_required",
      "rollback_validation_plan_required",
      "rollback_execution_denied",
      "audit_redaction_validation_required",
      "secret_material_read_denied",
      "external_send_public_claim_release_artifact_denied",
      "live_mutation_execution_denied"
    ],
    side_effects:{
      memory_store_mutated:false,
      memory_store_write_performed:false,
      memory_write_execution_performed:false,
      post_write_validation_recorded:false,
      post_write_validation_persisted:false,
      post_write_validation_report_written:false,
      post_write_watchdog_soak_performed:false,
      post_write_route_regression_check_performed:false,
      post_write_dependency_isolation_check_performed:false,
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
      launchd_mutated:false,
      service_restarted:false
    }
  }')"

jq -e '
  .status == "ready"
  and .memory_write_execution_post_write_validation_dry_run_ready == true
  and .post_write_validation_mode == "memory_write_execution_post_write_validation_dry_run_non_activation"
  and .source_memory_write_execution_write_enable_fixture_ready == true
  and .source_memory_write_execution_write_enable_fixture_report_sha256 != ""
  and .source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
  and .source_memory_write_execution_denial_matrix_report_sha256 != ""
  and .source_memory_write_execution_preflight_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_write_enable_surface_count == 10
  and .ready_write_enable_surface_count == 10
  and .required_post_write_validation_surface_count == 9
  and .ready_post_write_validation_surface_count == 9
  and .side_effect_free_post_write_validation_surface_count == 9
  and .required_post_write_validation_fixture_count == 8
  and .post_write_validation_fixture_count == 8
  and .blocked_post_write_validation_fixture_count == 8
  and .allowed_post_write_validation_fixture_count == 0
  and .passed_post_write_validation_fixture_count == 0
  and .post_write_validation_denied_count == 8
  and .post_write_validation_performed_count == 0
  and .post_write_validation_recorded == false
  and .post_write_validation_persisted == false
  and .post_write_validation_accepted == false
  and .post_write_validation_performed == false
  and .post_write_validation_report_written == false
  and .post_write_watchdog_soak_performed == false
  and .post_write_watchdog_soak_passed == false
  and .post_write_route_regression_check_performed == false
  and .post_write_dependency_isolation_check_performed == false
  and .post_write_memory_store_hash_recorded == false
  and .post_write_memory_store_hash_changed == false
  and .write_result_receipt_hash_recorded == false
  and .rollback_validation_performed == false
  and .rollback_validation_passed == false
  and .audit_redaction_validation_recorded == false
  and .secret_material_read == false
  and .memory_write_execution_allowed == false
  and .memory_write_execution_ready == false
  and .memory_write_execution_performed == false
  and .memory_store_write_path_enabled == false
  and .memory_store_write_allowed == false
  and .memory_store_write_performed == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutation_allowed == false
  and .memory_store_mutated == false
  and .live_mutation_execution_ready == false
  and .rollback_execution_allowed == false
  and .rollback_executed == false
  and .external_send_enabled == false
  and .external_send_performed == false
  and .public_claim_or_release_artifact_write_enabled == false
  and .public_release_published == false
  and .release_artifact_written == false
  and (.post_write_validation_surfaces | length) == 9
  and (.post_write_validation_fixtures | length) == 8
  and (.post_write_validation_fixtures | all(.post_write_validation_requested == true and .validation_status == "blocked" and .validation_allowed == false and .validation_performed == false and .validation_passed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .activation_allowed == false))
  and ([.post_write_validation_fixtures[] | select(.route_readiness_regression_detected == true)] | length) == 1
  and ([.post_write_validation_fixtures[] | select(.post_write_watchdog_soak_passed == false)] | length) >= 1
  and ([.post_write_validation_fixtures[] | select(.raw_payload_plaintext_recorded == true and .secret_material_read == true)] | length) == 1
  and ([.post_write_validation_fixtures[] | select(.external_send_requested == true and .release_artifact_write_requested == true)] | length) == 1
  and (.denied_by_post_write_validation | length) == 14
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution post-write validation dry-run gate passed"
