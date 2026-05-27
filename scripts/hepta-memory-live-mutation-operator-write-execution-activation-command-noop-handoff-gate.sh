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

ACTIVATION_CLOSURE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-activation-closure-denial-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-activation-closure-denial-gate.sh
)"

activation_closure_report_sha256="$(printf '%s' "$ACTIVATION_CLOSURE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson activation_closure "$ACTIVATION_CLOSURE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $activation_closure.runtime == "hepta"
    and $activation_closure.status == "ready"
    and $activation_closure.gate == "hepta_memory_live_mutation_operator_write_execution_activation_closure_denial_gate"
    and $activation_closure.memory_write_execution_activation_closure_denial_ready == true
    and $activation_closure.memory_write_execution_post_write_operator_acceptance_denial_ready == true
    and $activation_closure.memory_write_execution_post_write_validation_dry_run_ready == true
    and $activation_closure.memory_write_execution_write_enable_fixture_ready == true
    and $activation_closure.memory_write_execution_no_write_sink_contract_ready == true
    and $activation_closure.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256 != ""
    and $activation_closure.source_memory_write_execution_post_write_validation_dry_run_report_sha256 != ""
    and $activation_closure.source_memory_write_execution_write_enable_fixture_report_sha256 != ""
    and $activation_closure.source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
    and $activation_closure.source_memory_write_execution_denial_matrix_report_sha256 != ""
    and $activation_closure.source_memory_write_execution_preflight_report_sha256 != ""
    and $activation_closure.source_memory_write_approval_packet_report_sha256 != ""
    and $activation_closure.source_memory_write_contract_report_sha256 != ""
    and $activation_closure.minimum_required_samples >= 24
    and $activation_closure.required_activation_closure_surface_count == 12
    and $activation_closure.ready_activation_closure_surface_count == 12
    and $activation_closure.activation_closure_fixture_count == 10
    and $activation_closure.blocked_activation_closure_fixture_count == 10
    and $activation_closure.allowed_activation_closure_fixture_count == 0
    and $activation_closure.accepted_activation_closure_fixture_count == 0
    and $activation_closure.activation_closure_performed_count == 0
    and $activation_closure.activation_closure_packet_recorded == false
    and $activation_closure.activation_closure_packet_persisted == false
    and $activation_closure.activation_closure_packet_accepted == false
    and $activation_closure.activation_closure_packet_materialized == false
    and $activation_closure.activation_closure_packet_id_recorded == false
    and $activation_closure.activation_closure_packet_hash_bound == false
    and $activation_closure.activation_closure_packet_signature_hash_recorded == false
    and $activation_closure.activation_closure_packet_timestamp_recorded == false
    and $activation_closure.activation_closure_filesystem_written == false
    and $activation_closure.activation_closure_ledger_written == false
    and $activation_closure.activation_command_enabled == false
    and $activation_closure.activation_command_invoked == false
    and $activation_closure.activation_allowed_by_closure_packet == false
    and $activation_closure.activation_allowed == false
    and $activation_closure.activation_performed == false
    and $activation_closure.live_mutation_execution_ready == false
    and $activation_closure.live_mutation_execution_allowed == false
    and $activation_closure.live_mutation_execution_performed == false
    and $activation_closure.memory_write_execution_allowed == false
    and $activation_closure.memory_write_execution_ready == false
    and $activation_closure.memory_write_execution_performed == false
    and $activation_closure.memory_store_write_path_enabled == false
    and $activation_closure.memory_store_write_allowed == false
    and $activation_closure.memory_store_write_performed == false
    and $activation_closure.memory_store_write_performed_count == 0
    and $activation_closure.memory_store_mutation_allowed == false
    and $activation_closure.memory_store_mutated == false
    and $activation_closure.rollback_execution_allowed == false
    and $activation_closure.rollback_executed == false
    and $activation_closure.secret_material_read == false
    and $activation_closure.external_send_enabled == false
    and $activation_closure.external_send_performed == false
    and $activation_closure.public_claim_or_release_artifact_write_enabled == false
    and $activation_closure.public_release_published == false
    and $activation_closure.public_ga_claimed == false
    and $activation_closure.release_artifact_written == false
    and ($activation_closure.activation_closure_fixtures | length) == 10
    and ($activation_closure.activation_closure_fixtures | all(.closure_status == "blocked" and .closure_allowed == false and .closure_recorded == false and .closure_persisted == false and .closure_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false))
    and ($activation_closure.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_activation_command_noop_handoff_gate" \
  --arg activation_closure_report_sha256 "$activation_closure_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson activation_closure "$ACTIVATION_CLOSURE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_noop_handoff_mode:"memory_write_execution_activation_command_noop_handoff_denial",
    source_memory_write_execution_activation_closure_denial_gate:$activation_closure.gate,
    source_memory_write_execution_activation_closure_denial_ready:$activation_closure.memory_write_execution_activation_closure_denial_ready,
    source_memory_write_execution_activation_closure_denial_report_sha256:$activation_closure_report_sha256,
    source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256:$activation_closure.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256,
    source_memory_write_execution_post_write_validation_dry_run_report_sha256:$activation_closure.source_memory_write_execution_post_write_validation_dry_run_report_sha256,
    source_memory_write_execution_write_enable_fixture_report_sha256:$activation_closure.source_memory_write_execution_write_enable_fixture_report_sha256,
    source_memory_write_execution_no_write_sink_contract_report_sha256:$activation_closure.source_memory_write_execution_no_write_sink_contract_report_sha256,
    source_memory_write_execution_denial_matrix_report_sha256:$activation_closure.source_memory_write_execution_denial_matrix_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$activation_closure.source_memory_write_execution_preflight_report_sha256,
    source_memory_write_approval_packet_report_sha256:$activation_closure.source_memory_write_approval_packet_report_sha256,
    source_memory_write_contract_report_sha256:$activation_closure.source_memory_write_contract_report_sha256,
    source_memory_intelligence_report_sha256:$activation_closure.source_memory_intelligence_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$activation_closure.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_proof_report_sha256:$activation_closure.source_payload_redaction_proof_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_activation_command_noop_handoff_ready:true,
    memory_write_execution_activation_closure_denial_ready:true,
    memory_write_execution_post_write_operator_acceptance_denial_ready:true,
    memory_write_execution_post_write_validation_dry_run_ready:true,
    memory_write_execution_write_enable_fixture_ready:true,
    memory_write_execution_no_write_sink_contract_ready:true,
    required_activation_closure_surface_count:$activation_closure.required_activation_closure_surface_count,
    ready_activation_closure_surface_count:$activation_closure.ready_activation_closure_surface_count,
    required_activation_command_handoff_surface_count:13,
    ready_activation_command_handoff_surface_count:13,
    side_effect_free_activation_command_handoff_surface_count:13,
    required_activation_command_fixture_count:10,
    activation_command_fixture_count:10,
    blocked_activation_command_fixture_count:10,
    noop_activation_command_fixture_count:10,
    allowed_activation_command_fixture_count:0,
    accepted_activation_command_fixture_count:0,
    activation_command_denied_count:10,
    activation_command_performed_count:0,
    activation_command_shape_registered:false,
    activation_command_enabled:false,
    activation_command_invoked:false,
    activation_command_dispatched:false,
    activation_command_noop_decision_recorded:false,
    activation_command_noop_decision_persisted:false,
    activation_command_noop_decision_accepted:false,
    activation_command_handoff_recorded:false,
    activation_command_handoff_persisted:false,
    activation_command_handoff_accepted:false,
    activation_command_handoff_materialized:false,
    activation_command_handoff_filesystem_written:false,
    activation_command_result_receipt_recorded:false,
    activation_command_result_receipt_persisted:false,
    activation_closure_packet_recorded:false,
    activation_closure_packet_persisted:false,
    activation_closure_packet_accepted:false,
    activation_closure_packet_materialized:false,
    activation_closure_packet_hash_bound:false,
    activation_closure_packet_signature_hash_recorded:false,
    activation_closure_ledger_written:false,
    activation_allowed_by_command_handoff:false,
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
    route_readiness_regression_allowed:false,
    active_dependency_isolation_regression_allowed:false,
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
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    activation_command_handoff_surfaces:[
      "accepted_activation_closure_packet_required",
      "activation_closure_packet_hash_and_signature_required",
      "operator_identity_signature_timestamp_required",
      "single_surface_activation_scope_required",
      "activation_command_disabled_by_default_required",
      "activation_command_invocation_noop_required",
      "pre_post_store_hashes_and_write_receipt_required",
      "post_write_soak_route_dependency_evidence_required",
      "rollback_validation_and_no_rollback_execution_required",
      "audit_redaction_and_no_secret_material_required",
      "no_memory_store_write_or_live_mutation_required",
      "no_install_restart_or_active_binary_mutation_required",
      "no_external_public_or_release_outputs_required"
    ],
    activation_command_fixtures:[
      {
        id:"activation-command-missing-accepted-closure-packet",
        activation_command_requested:true,
        command_status:"blocked_noop",
        accepted_activation_closure_packet_present:false,
        activation_closure_packet_hash_bound:false,
        command_allowed:false,
        command_invoked:false,
        command_dispatched:false,
        command_noop_confirmed:true,
        handoff_recorded:false,
        handoff_persisted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"accepted_activation_closure_packet_required"
      },
      {
        id:"activation-command-disabled-by-default",
        activation_command_requested:true,
        command_status:"blocked_noop",
        accepted_activation_closure_packet_present:true,
        activation_closure_packet_hash_bound:true,
        activation_command_enabled:false,
        command_allowed:false,
        command_invoked:false,
        command_dispatched:false,
        command_noop_confirmed:true,
        handoff_recorded:false,
        handoff_persisted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"activation_command_disabled_by_default"
      },
      {
        id:"activation-command-direct-invocation-attempt",
        activation_command_requested:true,
        command_invocation_attempted:true,
        command_status:"blocked_noop",
        accepted_activation_closure_packet_present:true,
        activation_command_enabled:false,
        command_allowed:false,
        command_invoked:false,
        command_dispatched:false,
        command_noop_confirmed:true,
        handoff_recorded:false,
        handoff_persisted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"direct_activation_command_invocation_denied"
      },
      {
        id:"activation-command-closure-hash-mismatch",
        activation_command_requested:true,
        command_status:"blocked_noop",
        accepted_activation_closure_packet_present:true,
        activation_closure_packet_hash_bound:false,
        activation_closure_packet_signature_hash_recorded:false,
        command_allowed:false,
        command_invoked:false,
        command_dispatched:false,
        command_noop_confirmed:true,
        handoff_recorded:false,
        handoff_persisted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"closure_packet_hash_and_signature_binding_required"
      },
      {
        id:"activation-command-multi-surface-handoff",
        activation_command_requested:true,
        command_status:"blocked_noop",
        accepted_activation_closure_packet_present:true,
        operator_single_surface_scope_recorded:false,
        multi_surface_activation_requested:true,
        command_allowed:false,
        command_invoked:false,
        command_dispatched:false,
        command_noop_confirmed:true,
        handoff_recorded:false,
        handoff_persisted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"single_surface_activation_scope_required"
      },
      {
        id:"activation-command-memory-write-path-attempt",
        activation_command_requested:true,
        command_status:"blocked_noop",
        accepted_activation_closure_packet_present:true,
        memory_store_write_path_enable_requested:true,
        direct_memory_store_write_requested:true,
        command_allowed:false,
        command_invoked:false,
        command_dispatched:false,
        command_noop_confirmed:true,
        handoff_recorded:false,
        handoff_persisted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"activation_command_cannot_enable_or_perform_memory_store_write"
      },
      {
        id:"activation-command-rollback-execution-attempt",
        activation_command_requested:true,
        command_status:"blocked_noop",
        accepted_activation_closure_packet_present:true,
        rollback_validation_accepted:false,
        rollback_execution_requested:true,
        command_allowed:false,
        command_invoked:false,
        command_dispatched:false,
        command_noop_confirmed:true,
        handoff_recorded:false,
        handoff_persisted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"rollback_execution_denied_at_activation_command_handoff"
      },
      {
        id:"activation-command-secret-or-prompt-replay-attempt",
        activation_command_requested:true,
        command_status:"blocked_noop",
        accepted_activation_closure_packet_present:true,
        raw_payload_plaintext_recorded:true,
        secret_material_read:true,
        provider_prompt_replay_requested:true,
        command_allowed:false,
        command_invoked:false,
        command_dispatched:false,
        command_noop_confirmed:true,
        handoff_recorded:false,
        handoff_persisted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"secret_material_and_provider_prompt_replay_forbidden"
      },
      {
        id:"activation-command-external-public-release-attempt",
        activation_command_requested:true,
        command_status:"blocked_noop",
        accepted_activation_closure_packet_present:true,
        external_send_requested:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        command_allowed:false,
        command_invoked:false,
        command_dispatched:false,
        command_noop_confirmed:true,
        handoff_recorded:false,
        handoff_persisted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        external_send_performed:false,
        public_release_published:false,
        release_artifact_written:false,
        rollback_executed:false,
        reason:"activation_command_cannot_send_publish_or_write_release_artifacts"
      },
      {
        id:"activation-command-install-restart-active-binary-attempt",
        activation_command_requested:true,
        command_status:"blocked_noop",
        accepted_activation_closure_packet_present:true,
        install_requested:true,
        launchd_restart_requested:true,
        active_binary_mutation_requested:true,
        command_allowed:false,
        command_invoked:false,
        command_dispatched:false,
        command_noop_confirmed:true,
        handoff_recorded:false,
        handoff_persisted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        install_executed:false,
        launchd_mutated:false,
        service_restarted:false,
        active_binary_mutated:false,
        rollback_executed:false,
        reason:"activation_command_cannot_install_restart_or_mutate_active_binary"
      }
    ],
    denied_by_activation_command_handoff:[
      "accepted_activation_closure_packet_required",
      "activation_closure_packet_hash_required",
      "activation_closure_packet_signature_required",
      "operator_identity_required",
      "operator_acceptance_signature_required",
      "operator_acceptance_timestamp_required",
      "single_surface_activation_scope_required",
      "activation_command_enabled_denied",
      "activation_command_invocation_denied",
      "activation_command_dispatch_denied",
      "activation_command_handoff_persistence_denied",
      "pre_write_memory_store_hash_binding_required",
      "post_write_memory_store_hash_binding_required",
      "write_result_receipt_hash_binding_required",
      "route_readiness_regression_denied",
      "active_dependency_isolation_regression_denied",
      "post_write_watchdog_soak_success_required",
      "memory_store_write_path_enablement_denied",
      "direct_memory_store_write_denied",
      "live_mutation_execution_denied",
      "rollback_execution_denied",
      "secret_material_read_denied",
      "provider_prompt_replay_denied",
      "install_restart_active_binary_mutation_denied",
      "external_send_public_claim_release_artifact_denied",
      "public_release_public_ga_denied"
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
      activation_closure_packet_recorded:false,
      activation_closure_packet_persisted:false,
      activation_closure_packet_materialized:false,
      activation_closure_filesystem_written:false,
      activation_closure_ledger_written:false,
      activation_command_shape_registered:false,
      activation_command_enabled:false,
      activation_command_invoked:false,
      activation_command_dispatched:false,
      activation_command_handoff_recorded:false,
      activation_command_handoff_persisted:false,
      activation_command_result_receipt_recorded:false,
      activation_command_result_receipt_persisted:false,
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
      install_executed:false,
      active_binary_mutated:false,
      launchd_mutated:false,
      service_restarted:false
    }
  }')"

jq -e '
  .status == "ready"
  and .memory_write_execution_activation_command_noop_handoff_ready == true
  and .activation_command_noop_handoff_mode == "memory_write_execution_activation_command_noop_handoff_denial"
  and .source_memory_write_execution_activation_closure_denial_ready == true
  and .source_memory_write_execution_activation_closure_denial_report_sha256 != ""
  and .source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256 != ""
  and .source_memory_write_execution_post_write_validation_dry_run_report_sha256 != ""
  and .source_memory_write_execution_write_enable_fixture_report_sha256 != ""
  and .source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
  and .source_memory_write_execution_denial_matrix_report_sha256 != ""
  and .source_memory_write_execution_preflight_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_activation_closure_surface_count == 12
  and .ready_activation_closure_surface_count == 12
  and .required_activation_command_handoff_surface_count == 13
  and .ready_activation_command_handoff_surface_count == 13
  and .side_effect_free_activation_command_handoff_surface_count == 13
  and .required_activation_command_fixture_count == 10
  and .activation_command_fixture_count == 10
  and .blocked_activation_command_fixture_count == 10
  and .noop_activation_command_fixture_count == 10
  and .allowed_activation_command_fixture_count == 0
  and .accepted_activation_command_fixture_count == 0
  and .activation_command_denied_count == 10
  and .activation_command_performed_count == 0
  and .activation_command_shape_registered == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_command_noop_decision_recorded == false
  and .activation_command_noop_decision_persisted == false
  and .activation_command_noop_decision_accepted == false
  and .activation_command_handoff_recorded == false
  and .activation_command_handoff_persisted == false
  and .activation_command_handoff_accepted == false
  and .activation_command_handoff_materialized == false
  and .activation_command_handoff_filesystem_written == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_closure_packet_recorded == false
  and .activation_closure_packet_persisted == false
  and .activation_closure_packet_accepted == false
  and .activation_closure_packet_materialized == false
  and .activation_closure_packet_hash_bound == false
  and .activation_closure_packet_signature_hash_recorded == false
  and .activation_closure_ledger_written == false
  and .activation_allowed_by_command_handoff == false
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
  and .provider_prompt_replay_enabled == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_send_enabled == false
  and .external_send_performed == false
  and .public_claim_or_release_artifact_write_enabled == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_handoff_surfaces | length) == 13
  and (.activation_command_fixtures | length) == 10
  and (.activation_command_fixtures | all(.activation_command_requested == true and .command_status == "blocked_noop" and .command_allowed == false and .command_invoked == false and .command_dispatched == false and .command_noop_confirmed == true and .handoff_recorded == false and .handoff_persisted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false))
  and ([.activation_command_fixtures[] | select(.command_invocation_attempted == true)] | length) == 1
  and ([.activation_command_fixtures[] | select(.memory_store_write_path_enable_requested == true and .direct_memory_store_write_requested == true)] | length) == 1
  and ([.activation_command_fixtures[] | select(.raw_payload_plaintext_recorded == true and .secret_material_read == true)] | length) == 1
  and ([.activation_command_fixtures[] | select(.external_send_requested == true and .release_artifact_write_requested == true)] | length) == 1
  and ([.activation_command_fixtures[] | select(.install_requested == true and .launchd_restart_requested == true and .active_binary_mutation_requested == true)] | length) == 1
  and (.denied_by_activation_command_handoff | length) == 26
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution activation command no-op handoff gate passed"
