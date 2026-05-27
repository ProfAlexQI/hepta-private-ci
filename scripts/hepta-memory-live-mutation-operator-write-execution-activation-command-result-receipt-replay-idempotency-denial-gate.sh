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

NO_PERSISTENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-no-persistence-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-no-persistence-gate.sh
)"

no_persistence_report_sha256="$(printf '%s' "$NO_PERSISTENCE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson no_persistence "$NO_PERSISTENCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $no_persistence.runtime == "hepta"
    and $no_persistence.status == "ready"
    and $no_persistence.gate == "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_no_persistence_gate"
    and $no_persistence.activation_command_result_receipt_no_persistence_mode == "memory_write_execution_activation_command_result_receipt_no_persistence_denial"
    and $no_persistence.memory_write_execution_activation_command_result_receipt_no_persistence_ready == true
    and $no_persistence.memory_write_execution_activation_command_noop_handoff_ready == true
    and $no_persistence.memory_write_execution_activation_closure_denial_ready == true
    and $no_persistence.memory_write_execution_post_write_operator_acceptance_denial_ready == true
    and $no_persistence.memory_write_execution_post_write_validation_dry_run_ready == true
    and $no_persistence.memory_write_execution_write_enable_fixture_ready == true
    and $no_persistence.memory_write_execution_no_write_sink_contract_ready == true
    and $no_persistence.source_activation_command_noop_handoff_report_sha256 != ""
    and $no_persistence.source_memory_write_execution_activation_closure_denial_report_sha256 != ""
    and $no_persistence.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256 != ""
    and $no_persistence.source_memory_write_execution_post_write_validation_dry_run_report_sha256 != ""
    and $no_persistence.source_memory_write_execution_write_enable_fixture_report_sha256 != ""
    and $no_persistence.source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
    and $no_persistence.source_memory_write_execution_denial_matrix_report_sha256 != ""
    and $no_persistence.source_memory_write_execution_preflight_report_sha256 != ""
    and $no_persistence.minimum_required_samples >= 24
    and $no_persistence.required_activation_command_handoff_surface_count == 13
    and $no_persistence.ready_activation_command_handoff_surface_count == 13
    and $no_persistence.required_activation_command_result_receipt_surface_count == 12
    and $no_persistence.ready_activation_command_result_receipt_surface_count == 12
    and $no_persistence.side_effect_free_activation_command_result_receipt_surface_count == 12
    and $no_persistence.required_activation_command_result_receipt_fixture_count == 10
    and $no_persistence.activation_command_result_receipt_fixture_count == 10
    and $no_persistence.blocked_activation_command_result_receipt_fixture_count == 10
    and $no_persistence.noop_activation_command_result_receipt_fixture_count == 10
    and $no_persistence.allowed_activation_command_result_receipt_fixture_count == 0
    and $no_persistence.accepted_activation_command_result_receipt_fixture_count == 0
    and $no_persistence.activation_command_result_receipt_performed_count == 0
    and $no_persistence.activation_command_result_receipt_recorded == false
    and $no_persistence.activation_command_result_receipt_persisted == false
    and $no_persistence.activation_command_result_receipt_accepted == false
    and $no_persistence.activation_command_result_receipt_materialized == false
    and $no_persistence.activation_command_result_receipt_filesystem_written == false
    and $no_persistence.activation_command_result_receipt_ledger_written == false
    and $no_persistence.activation_command_result_receipt_indexed == false
    and $no_persistence.activation_command_result_receipt_delivered == false
    and $no_persistence.activation_command_completion_ack_recorded == false
    and $no_persistence.activation_command_completion_ack_persisted == false
    and $no_persistence.activation_command_completion_ack_accepted == false
    and $no_persistence.activation_allowed_by_result_receipt == false
    and $no_persistence.activation_allowed == false
    and $no_persistence.activation_performed == false
    and $no_persistence.live_mutation_execution_performed == false
    and $no_persistence.memory_store_write_performed == false
    and $no_persistence.memory_store_write_performed_count == 0
    and $no_persistence.memory_store_mutated == false
    and $no_persistence.rollback_executed == false
    and $no_persistence.secret_material_read == false
    and $no_persistence.provider_invoked == false
    and $no_persistence.model_invoked == false
    and $no_persistence.external_send_performed == false
    and $no_persistence.public_release_published == false
    and $no_persistence.release_artifact_written == false
    and $no_persistence.install_executed == false
    and $no_persistence.launchd_mutated == false
    and $no_persistence.service_restarted == false
    and $no_persistence.active_binary_mutated == false
    and ($no_persistence.activation_command_result_receipt_fixtures | length) == 10
    and ($no_persistence.activation_command_result_receipt_fixtures | all(.receipt_status == "blocked_noop" and .receipt_allowed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .receipt_materialized == false and .receipt_filesystem_written == false and .receipt_noop_confirmed == true and .completion_ack_recorded == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false))
    and ($no_persistence.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_gate" \
  --arg no_persistence_report_sha256 "$no_persistence_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson no_persistence "$NO_PERSISTENCE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_replay_idempotency_mode:"memory_write_execution_activation_command_result_receipt_replay_idempotency_denial",
    source_activation_command_result_receipt_no_persistence_gate:$no_persistence.gate,
    source_activation_command_result_receipt_no_persistence_ready:$no_persistence.memory_write_execution_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_report_sha256:$no_persistence_report_sha256,
    source_activation_command_noop_handoff_ready:$no_persistence.memory_write_execution_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_report_sha256:$no_persistence.source_activation_command_noop_handoff_report_sha256,
    source_memory_write_execution_activation_closure_denial_report_sha256:$no_persistence.source_memory_write_execution_activation_closure_denial_report_sha256,
    source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256:$no_persistence.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256,
    source_memory_write_execution_post_write_validation_dry_run_report_sha256:$no_persistence.source_memory_write_execution_post_write_validation_dry_run_report_sha256,
    source_memory_write_execution_write_enable_fixture_report_sha256:$no_persistence.source_memory_write_execution_write_enable_fixture_report_sha256,
    source_memory_write_execution_no_write_sink_contract_report_sha256:$no_persistence.source_memory_write_execution_no_write_sink_contract_report_sha256,
    source_memory_write_execution_denial_matrix_report_sha256:$no_persistence.source_memory_write_execution_denial_matrix_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$no_persistence.source_memory_write_execution_preflight_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_no_persistence_ready:true,
    memory_write_execution_activation_command_noop_handoff_ready:true,
    memory_write_execution_activation_closure_denial_ready:true,
    memory_write_execution_post_write_operator_acceptance_denial_ready:true,
    memory_write_execution_post_write_validation_dry_run_ready:true,
    memory_write_execution_write_enable_fixture_ready:true,
    memory_write_execution_no_write_sink_contract_ready:true,
    required_activation_command_result_receipt_replay_idempotency_surface_count:12,
    ready_activation_command_result_receipt_replay_idempotency_surface_count:12,
    side_effect_free_activation_command_result_receipt_replay_idempotency_surface_count:12,
    required_activation_command_result_receipt_replay_idempotency_fixture_count:10,
    activation_command_result_receipt_replay_idempotency_fixture_count:10,
    blocked_activation_command_result_receipt_replay_idempotency_fixture_count:10,
    noop_activation_command_result_receipt_replay_idempotency_fixture_count:10,
    allowed_activation_command_result_receipt_replay_idempotency_fixture_count:0,
    accepted_activation_command_result_receipt_replay_idempotency_fixture_count:0,
    duplicate_activation_command_result_receipt_fixture_count:2,
    cross_scope_activation_command_result_receipt_fixture_count:1,
    status_upgrade_activation_command_result_receipt_fixture_count:1,
    activation_command_result_receipt_replay_denied_count:10,
    activation_command_result_receipt_duplicate_denied_count:10,
    activation_command_result_receipt_idempotency_denied_count:10,
    activation_command_result_receipt_replay_performed_count:0,
    activation_command_result_receipt_duplicate_accepted_count:0,
    activation_command_result_receipt_idempotency_state_recorded_count:0,
    activation_command_result_receipt_replay_allowed:false,
    activation_command_result_receipt_replay_recorded:false,
    activation_command_result_receipt_replay_persisted:false,
    activation_command_result_receipt_duplicate_accepted:false,
    activation_command_result_receipt_duplicate_recorded:false,
    activation_command_result_receipt_duplicate_persisted:false,
    activation_command_result_receipt_idempotency_key_accepted:false,
    activation_command_result_receipt_idempotency_state_recorded:false,
    activation_command_result_receipt_idempotency_state_persisted:false,
    activation_command_result_receipt_replay_nonce_accepted:false,
    activation_command_result_receipt_replay_nonce_recorded:false,
    activation_command_result_receipt_cross_scope_reuse_accepted:false,
    activation_command_result_receipt_status_upgrade_accepted:false,
    activation_command_result_receipt_completed_status_accepted:false,
    activation_command_result_receipt_ack_replay_accepted:false,
    activation_command_result_receipt_ledger_replay_accepted:false,
    activation_command_result_receipt_delivery_replay_accepted:false,
    activation_command_result_receipt_write_replay_accepted:false,
    activation_command_result_receipt_rollback_replay_accepted:false,
    activation_command_result_receipt_secret_provider_replay_accepted:false,
    activation_command_result_receipt_external_public_install_replay_accepted:false,
    activation_command_result_receipt_recorded:false,
    activation_command_result_receipt_persisted:false,
    activation_command_result_receipt_accepted:false,
    activation_command_result_receipt_materialized:false,
    activation_command_result_receipt_filesystem_written:false,
    activation_command_result_receipt_ledger_written:false,
    activation_command_result_receipt_indexed:false,
    activation_command_result_receipt_enqueued:false,
    activation_command_result_receipt_delivered:false,
    activation_command_completion_ack_recorded:false,
    activation_command_completion_ack_persisted:false,
    activation_command_completion_ack_accepted:false,
    activation_command_completion_ack_delivered:false,
    activation_command_enabled:false,
    activation_command_invoked:false,
    activation_command_dispatched:false,
    activation_allowed_by_result_receipt_replay:false,
    activation_allowed_by_result_receipt:false,
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
    rollback_execution_allowed:false,
    rollback_executed:false,
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
    activation_command_result_receipt_replay_idempotency_surfaces:[
      "source_result_receipt_no_persistence_report_required",
      "canonical_noop_result_receipt_identity_required",
      "receipt_replay_nonce_idempotency_key_required",
      "duplicate_receipt_suppression_required",
      "cross_scope_receipt_reuse_denied",
      "blocked_noop_status_transition_denied",
      "completion_ack_replay_denied",
      "ledger_index_delivery_replay_denied",
      "memory_write_live_mutation_replay_denied",
      "rollback_replay_denied",
      "secret_provider_prompt_replay_denied",
      "external_public_install_restart_replay_denied"
    ],
    activation_command_result_receipt_replay_idempotency_fixtures:[
      {
        id:"activation-result-receipt-replay-missing-source-no-persistence-report",
        replay_requested:true,
        replay_status:"blocked_noop",
        source_no_persistence_present:false,
        source_no_persistence_ready:false,
        replay_allowed:false,
        replay_recorded:false,
        replay_persisted:false,
        duplicate_accepted:false,
        idempotency_key_accepted:false,
        idempotency_state_recorded:false,
        idempotency_state_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        receipt_noop_confirmed:true,
        reason:"source_result_receipt_no_persistence_report_required"
      },
      {
        id:"activation-result-receipt-duplicate-receipt-id-replay",
        replay_requested:true,
        duplicate_receipt_id_requested:true,
        replay_status:"blocked_duplicate_noop",
        source_no_persistence_present:true,
        replay_allowed:false,
        replay_recorded:false,
        replay_persisted:false,
        duplicate_accepted:false,
        idempotency_key_accepted:false,
        idempotency_state_recorded:false,
        idempotency_state_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        receipt_noop_confirmed:true,
        reason:"duplicate_result_receipt_id_replay_denied"
      },
      {
        id:"activation-result-receipt-stale-idempotency-key-replay",
        replay_requested:true,
        stale_idempotency_key_requested:true,
        replay_status:"blocked_duplicate_noop",
        source_no_persistence_present:true,
        replay_allowed:false,
        replay_recorded:false,
        replay_persisted:false,
        duplicate_accepted:false,
        idempotency_key_accepted:false,
        idempotency_state_recorded:false,
        idempotency_state_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        receipt_noop_confirmed:true,
        reason:"stale_idempotency_key_replay_denied"
      },
      {
        id:"activation-result-receipt-cross-scope-reuse",
        replay_requested:true,
        cross_scope_reuse_requested:true,
        replay_status:"blocked_noop",
        source_no_persistence_present:true,
        replay_allowed:false,
        replay_recorded:false,
        replay_persisted:false,
        duplicate_accepted:false,
        idempotency_key_accepted:false,
        idempotency_state_recorded:false,
        idempotency_state_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        receipt_noop_confirmed:true,
        reason:"cross_scope_result_receipt_reuse_denied"
      },
      {
        id:"activation-result-receipt-status-upgrade-replay",
        replay_requested:true,
        receipt_status_requested:"completed",
        replay_status:"blocked_noop",
        source_no_persistence_present:true,
        replay_allowed:false,
        replay_recorded:false,
        replay_persisted:false,
        duplicate_accepted:false,
        idempotency_key_accepted:false,
        idempotency_state_recorded:false,
        idempotency_state_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        receipt_noop_confirmed:true,
        reason:"blocked_noop_result_receipt_status_upgrade_denied"
      },
      {
        id:"activation-result-receipt-completion-ack-replay",
        replay_requested:true,
        completion_ack_replay_requested:true,
        replay_status:"blocked_noop",
        source_no_persistence_present:true,
        replay_allowed:false,
        replay_recorded:false,
        replay_persisted:false,
        duplicate_accepted:false,
        idempotency_key_accepted:false,
        idempotency_state_recorded:false,
        idempotency_state_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        completion_ack_recorded:false,
        completion_ack_persisted:false,
        completion_ack_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        receipt_noop_confirmed:true,
        reason:"completion_ack_replay_denied"
      },
      {
        id:"activation-result-receipt-ledger-index-delivery-replay",
        replay_requested:true,
        ledger_replay_requested:true,
        index_replay_requested:true,
        delivery_replay_requested:true,
        replay_status:"blocked_noop",
        source_no_persistence_present:true,
        replay_allowed:false,
        replay_recorded:false,
        replay_persisted:false,
        duplicate_accepted:false,
        idempotency_key_accepted:false,
        idempotency_state_recorded:false,
        idempotency_state_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        receipt_ledger_written:false,
        receipt_indexed:false,
        receipt_delivered:false,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        receipt_noop_confirmed:true,
        reason:"ledger_index_delivery_result_receipt_replay_denied"
      },
      {
        id:"activation-result-receipt-memory-write-live-mutation-replay",
        replay_requested:true,
        memory_write_replay_requested:true,
        live_mutation_replay_requested:true,
        replay_status:"blocked_noop",
        source_no_persistence_present:true,
        replay_allowed:false,
        replay_recorded:false,
        replay_persisted:false,
        duplicate_accepted:false,
        idempotency_key_accepted:false,
        idempotency_state_recorded:false,
        idempotency_state_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        receipt_noop_confirmed:true,
        reason:"memory_write_live_mutation_result_receipt_replay_denied"
      },
      {
        id:"activation-result-receipt-rollback-secret-provider-replay",
        replay_requested:true,
        rollback_replay_requested:true,
        secret_material_replay_requested:true,
        provider_prompt_replay_requested:true,
        replay_status:"blocked_noop",
        source_no_persistence_present:true,
        replay_allowed:false,
        replay_recorded:false,
        replay_persisted:false,
        duplicate_accepted:false,
        idempotency_key_accepted:false,
        idempotency_state_recorded:false,
        idempotency_state_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        secret_material_read:false,
        provider_invoked:false,
        model_invoked:false,
        receipt_noop_confirmed:true,
        reason:"rollback_secret_provider_result_receipt_replay_denied"
      },
      {
        id:"activation-result-receipt-external-public-install-replay",
        replay_requested:true,
        external_send_replay_requested:true,
        public_claim_replay_requested:true,
        release_artifact_replay_requested:true,
        install_replay_requested:true,
        service_restart_replay_requested:true,
        active_binary_mutation_replay_requested:true,
        replay_status:"blocked_noop",
        source_no_persistence_present:true,
        replay_allowed:false,
        replay_recorded:false,
        replay_persisted:false,
        duplicate_accepted:false,
        idempotency_key_accepted:false,
        idempotency_state_recorded:false,
        idempotency_state_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        external_send_performed:false,
        public_release_published:false,
        release_artifact_written:false,
        install_executed:false,
        launchd_mutated:false,
        service_restarted:false,
        active_binary_mutated:false,
        receipt_noop_confirmed:true,
        reason:"external_public_install_restart_result_receipt_replay_denied"
      }
    ],
    denied_by_activation_command_result_receipt_replay_idempotency:[
      "source_result_receipt_no_persistence_report_required",
      "canonical_noop_result_receipt_identity_required",
      "result_receipt_replay_nonce_required_but_not_recorded",
      "result_receipt_idempotency_key_required_but_not_recorded",
      "duplicate_result_receipt_id_replay_denied",
      "stale_idempotency_key_replay_denied",
      "cross_scope_result_receipt_reuse_denied",
      "blocked_noop_status_transition_denied",
      "completed_status_upgrade_denied",
      "completion_ack_replay_denied",
      "ledger_replay_denied",
      "index_replay_denied",
      "delivery_replay_denied",
      "memory_write_replay_denied",
      "live_mutation_replay_denied",
      "rollback_replay_denied",
      "secret_material_replay_denied",
      "provider_prompt_replay_denied",
      "external_send_replay_denied",
      "public_claim_replay_denied",
      "release_artifact_replay_denied",
      "install_replay_denied",
      "launchd_restart_replay_denied",
      "active_binary_mutation_replay_denied"
    ],
    side_effects:{
      activation_command_result_receipt_replay_recorded:false,
      activation_command_result_receipt_replay_persisted:false,
      activation_command_result_receipt_duplicate_accepted:false,
      activation_command_result_receipt_duplicate_recorded:false,
      activation_command_result_receipt_idempotency_state_recorded:false,
      activation_command_result_receipt_idempotency_state_persisted:false,
      activation_command_result_receipt_cross_scope_reuse_accepted:false,
      activation_command_result_receipt_status_upgrade_accepted:false,
      activation_command_result_receipt_recorded:false,
      activation_command_result_receipt_persisted:false,
      activation_command_result_receipt_accepted:false,
      activation_command_result_receipt_materialized:false,
      activation_command_result_receipt_filesystem_written:false,
      activation_command_result_receipt_ledger_written:false,
      activation_command_result_receipt_indexed:false,
      activation_command_result_receipt_enqueued:false,
      activation_command_result_receipt_delivered:false,
      activation_command_completion_ack_recorded:false,
      activation_command_completion_ack_persisted:false,
      activation_command_completion_ack_accepted:false,
      activation_command_completion_ack_delivered:false,
      activation_command_enabled:false,
      activation_command_invoked:false,
      activation_command_dispatched:false,
      activation_performed:false,
      live_mutation_execution_performed:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      rollback_executed:false,
      raw_payload_inspected:false,
      payload_plaintext_persisted:false,
      secret_file_read:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      provider_prompt_replayed:false,
      channel_send_performed:false,
      external_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      skill_workshop_written:false,
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
  and .memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .activation_command_result_receipt_replay_idempotency_mode == "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial"
  and .source_activation_command_result_receipt_no_persistence_ready == true
  and .source_activation_command_result_receipt_no_persistence_report_sha256 != ""
  and .source_activation_command_noop_handoff_ready == true
  and .minimum_required_samples >= 24
  and .required_activation_command_result_receipt_replay_idempotency_surface_count == 12
  and .ready_activation_command_result_receipt_replay_idempotency_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_replay_idempotency_surface_count == 12
  and .required_activation_command_result_receipt_replay_idempotency_fixture_count == 10
  and .activation_command_result_receipt_replay_idempotency_fixture_count == 10
  and .blocked_activation_command_result_receipt_replay_idempotency_fixture_count == 10
  and .noop_activation_command_result_receipt_replay_idempotency_fixture_count == 10
  and .allowed_activation_command_result_receipt_replay_idempotency_fixture_count == 0
  and .accepted_activation_command_result_receipt_replay_idempotency_fixture_count == 0
  and .duplicate_activation_command_result_receipt_fixture_count == 2
  and .cross_scope_activation_command_result_receipt_fixture_count == 1
  and .status_upgrade_activation_command_result_receipt_fixture_count == 1
  and .activation_command_result_receipt_replay_denied_count == 10
  and .activation_command_result_receipt_duplicate_denied_count == 10
  and .activation_command_result_receipt_idempotency_denied_count == 10
  and .activation_command_result_receipt_replay_performed_count == 0
  and .activation_command_result_receipt_duplicate_accepted_count == 0
  and .activation_command_result_receipt_idempotency_state_recorded_count == 0
  and .activation_command_result_receipt_replay_allowed == false
  and .activation_command_result_receipt_replay_recorded == false
  and .activation_command_result_receipt_replay_persisted == false
  and .activation_command_result_receipt_duplicate_accepted == false
  and .activation_command_result_receipt_duplicate_recorded == false
  and .activation_command_result_receipt_duplicate_persisted == false
  and .activation_command_result_receipt_idempotency_key_accepted == false
  and .activation_command_result_receipt_idempotency_state_recorded == false
  and .activation_command_result_receipt_idempotency_state_persisted == false
  and .activation_command_result_receipt_replay_nonce_accepted == false
  and .activation_command_result_receipt_replay_nonce_recorded == false
  and .activation_command_result_receipt_cross_scope_reuse_accepted == false
  and .activation_command_result_receipt_status_upgrade_accepted == false
  and .activation_command_result_receipt_completed_status_accepted == false
  and .activation_command_result_receipt_ack_replay_accepted == false
  and .activation_command_result_receipt_ledger_replay_accepted == false
  and .activation_command_result_receipt_delivery_replay_accepted == false
  and .activation_command_result_receipt_write_replay_accepted == false
  and .activation_command_result_receipt_rollback_replay_accepted == false
  and .activation_command_result_receipt_secret_provider_replay_accepted == false
  and .activation_command_result_receipt_external_public_install_replay_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_result_receipt_ledger_written == false
  and .activation_command_result_receipt_indexed == false
  and .activation_command_result_receipt_enqueued == false
  and .activation_command_result_receipt_delivered == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_persisted == false
  and .activation_command_completion_ack_accepted == false
  and .activation_command_completion_ack_delivered == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_allowed_by_result_receipt_replay == false
  and .activation_allowed_by_result_receipt == false
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
  and (.activation_command_result_receipt_replay_idempotency_surfaces | length) == 12
  and (.activation_command_result_receipt_replay_idempotency_fixtures | length) == 10
  and (.activation_command_result_receipt_replay_idempotency_fixtures | all((.replay_status == "blocked_noop" or .replay_status == "blocked_duplicate_noop") and .replay_allowed == false and .replay_recorded == false and .replay_persisted == false and .duplicate_accepted == false and .idempotency_key_accepted == false and .idempotency_state_recorded == false and .idempotency_state_persisted == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .completion_ack_recorded == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.duplicate_receipt_id_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.stale_idempotency_key_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.cross_scope_reuse_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.receipt_status_requested == "completed")] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.completion_ack_replay_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.ledger_replay_requested == true and .index_replay_requested == true and .delivery_replay_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.memory_write_replay_requested == true and .live_mutation_replay_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.rollback_replay_requested == true and .secret_material_replay_requested == true and .provider_prompt_replay_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.external_send_replay_requested == true and .install_replay_requested == true and .active_binary_mutation_replay_requested == true)] | length) == 1
  and (.denied_by_activation_command_result_receipt_replay_idempotency | length) == 24
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution activation command result receipt replay/idempotency denial gate passed"
