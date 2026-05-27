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

NOOP_HANDOFF_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-activation-command-noop-handoff-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-noop-handoff-gate.sh
)"

noop_handoff_report_sha256="$(printf '%s' "$NOOP_HANDOFF_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson noop_handoff "$NOOP_HANDOFF_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $noop_handoff.runtime == "hepta"
    and $noop_handoff.status == "ready"
    and $noop_handoff.gate == "hepta_memory_live_mutation_operator_write_execution_activation_command_noop_handoff_gate"
    and $noop_handoff.activation_command_noop_handoff_mode == "memory_write_execution_activation_command_noop_handoff_denial"
    and $noop_handoff.memory_write_execution_activation_command_noop_handoff_ready == true
    and $noop_handoff.memory_write_execution_activation_closure_denial_ready == true
    and $noop_handoff.memory_write_execution_post_write_operator_acceptance_denial_ready == true
    and $noop_handoff.memory_write_execution_post_write_validation_dry_run_ready == true
    and $noop_handoff.memory_write_execution_write_enable_fixture_ready == true
    and $noop_handoff.memory_write_execution_no_write_sink_contract_ready == true
    and $noop_handoff.source_memory_write_execution_activation_closure_denial_report_sha256 != ""
    and $noop_handoff.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256 != ""
    and $noop_handoff.source_memory_write_execution_post_write_validation_dry_run_report_sha256 != ""
    and $noop_handoff.source_memory_write_execution_write_enable_fixture_report_sha256 != ""
    and $noop_handoff.source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
    and $noop_handoff.source_memory_write_execution_denial_matrix_report_sha256 != ""
    and $noop_handoff.source_memory_write_execution_preflight_report_sha256 != ""
    and $noop_handoff.minimum_required_samples >= 24
    and $noop_handoff.required_activation_command_handoff_surface_count == 13
    and $noop_handoff.ready_activation_command_handoff_surface_count == 13
    and $noop_handoff.side_effect_free_activation_command_handoff_surface_count == 13
    and $noop_handoff.activation_command_fixture_count == 10
    and $noop_handoff.blocked_activation_command_fixture_count == 10
    and $noop_handoff.noop_activation_command_fixture_count == 10
    and $noop_handoff.allowed_activation_command_fixture_count == 0
    and $noop_handoff.accepted_activation_command_fixture_count == 0
    and $noop_handoff.activation_command_performed_count == 0
    and $noop_handoff.activation_command_shape_registered == false
    and $noop_handoff.activation_command_enabled == false
    and $noop_handoff.activation_command_invoked == false
    and $noop_handoff.activation_command_dispatched == false
    and $noop_handoff.activation_command_noop_decision_recorded == false
    and $noop_handoff.activation_command_noop_decision_persisted == false
    and $noop_handoff.activation_command_noop_decision_accepted == false
    and $noop_handoff.activation_command_handoff_recorded == false
    and $noop_handoff.activation_command_handoff_persisted == false
    and $noop_handoff.activation_command_handoff_accepted == false
    and $noop_handoff.activation_command_handoff_materialized == false
    and $noop_handoff.activation_command_handoff_filesystem_written == false
    and $noop_handoff.activation_command_result_receipt_recorded == false
    and $noop_handoff.activation_command_result_receipt_persisted == false
    and $noop_handoff.activation_allowed_by_command_handoff == false
    and $noop_handoff.activation_allowed == false
    and $noop_handoff.activation_performed == false
    and $noop_handoff.live_mutation_execution_ready == false
    and $noop_handoff.live_mutation_execution_allowed == false
    and $noop_handoff.live_mutation_execution_performed == false
    and $noop_handoff.memory_write_execution_allowed == false
    and $noop_handoff.memory_write_execution_ready == false
    and $noop_handoff.memory_write_execution_performed == false
    and $noop_handoff.memory_store_write_path_enabled == false
    and $noop_handoff.memory_store_write_allowed == false
    and $noop_handoff.memory_store_write_performed == false
    and $noop_handoff.memory_store_write_performed_count == 0
    and $noop_handoff.memory_store_mutation_allowed == false
    and $noop_handoff.memory_store_mutated == false
    and $noop_handoff.rollback_execution_allowed == false
    and $noop_handoff.rollback_executed == false
    and $noop_handoff.secret_material_read == false
    and $noop_handoff.provider_prompt_replay_enabled == false
    and $noop_handoff.provider_invoked == false
    and $noop_handoff.model_invoked == false
    and $noop_handoff.external_send_enabled == false
    and $noop_handoff.external_send_performed == false
    and $noop_handoff.public_claim_or_release_artifact_write_enabled == false
    and $noop_handoff.public_release_published == false
    and $noop_handoff.public_ga_claimed == false
    and $noop_handoff.release_artifact_written == false
    and $noop_handoff.install_executed == false
    and $noop_handoff.launchd_mutated == false
    and $noop_handoff.service_restarted == false
    and $noop_handoff.active_binary_mutated == false
    and ($noop_handoff.activation_command_fixtures | length) == 10
    and ($noop_handoff.activation_command_fixtures | all(.command_status == "blocked_noop" and .command_allowed == false and .command_invoked == false and .command_dispatched == false and .command_noop_confirmed == true and .handoff_recorded == false and .handoff_persisted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false))
    and ($noop_handoff.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_no_persistence_gate" \
  --arg noop_handoff_report_sha256 "$noop_handoff_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson noop_handoff "$NOOP_HANDOFF_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_no_persistence_mode:"memory_write_execution_activation_command_result_receipt_no_persistence_denial",
    source_activation_command_noop_handoff_gate:$noop_handoff.gate,
    source_activation_command_noop_handoff_ready:$noop_handoff.memory_write_execution_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_report_sha256:$noop_handoff_report_sha256,
    source_memory_write_execution_activation_closure_denial_report_sha256:$noop_handoff.source_memory_write_execution_activation_closure_denial_report_sha256,
    source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256:$noop_handoff.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256,
    source_memory_write_execution_post_write_validation_dry_run_report_sha256:$noop_handoff.source_memory_write_execution_post_write_validation_dry_run_report_sha256,
    source_memory_write_execution_write_enable_fixture_report_sha256:$noop_handoff.source_memory_write_execution_write_enable_fixture_report_sha256,
    source_memory_write_execution_no_write_sink_contract_report_sha256:$noop_handoff.source_memory_write_execution_no_write_sink_contract_report_sha256,
    source_memory_write_execution_denial_matrix_report_sha256:$noop_handoff.source_memory_write_execution_denial_matrix_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$noop_handoff.source_memory_write_execution_preflight_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_activation_command_result_receipt_no_persistence_ready:true,
    memory_write_execution_activation_command_noop_handoff_ready:true,
    memory_write_execution_activation_closure_denial_ready:true,
    memory_write_execution_post_write_operator_acceptance_denial_ready:true,
    memory_write_execution_post_write_validation_dry_run_ready:true,
    memory_write_execution_write_enable_fixture_ready:true,
    memory_write_execution_no_write_sink_contract_ready:true,
    required_activation_command_handoff_surface_count:$noop_handoff.required_activation_command_handoff_surface_count,
    ready_activation_command_handoff_surface_count:$noop_handoff.ready_activation_command_handoff_surface_count,
    required_activation_command_result_receipt_surface_count:12,
    ready_activation_command_result_receipt_surface_count:12,
    side_effect_free_activation_command_result_receipt_surface_count:12,
    required_activation_command_result_receipt_fixture_count:10,
    activation_command_result_receipt_fixture_count:10,
    blocked_activation_command_result_receipt_fixture_count:10,
    noop_activation_command_result_receipt_fixture_count:10,
    allowed_activation_command_result_receipt_fixture_count:0,
    accepted_activation_command_result_receipt_fixture_count:0,
    activation_command_result_receipt_denied_count:10,
    activation_command_result_receipt_performed_count:0,
    activation_command_result_receipt_shape_registered:false,
    activation_command_result_receipt_schema_accepted:false,
    activation_command_result_receipt_recorded:false,
    activation_command_result_receipt_persisted:false,
    activation_command_result_receipt_accepted:false,
    activation_command_result_receipt_materialized:false,
    activation_command_result_receipt_filesystem_written:false,
    activation_command_result_receipt_ledger_written:false,
    activation_command_result_receipt_indexed:false,
    activation_command_result_receipt_enqueued:false,
    activation_command_result_receipt_delivered:false,
    activation_command_result_receipt_hash_bound:false,
    activation_command_result_receipt_signature_hash_recorded:false,
    activation_command_result_receipt_timestamp_recorded:false,
    activation_command_result_receipt_operator_identity_accepted:false,
    activation_command_result_receipt_status_accepted:false,
    activation_command_result_receipt_blocked_noop_status_accepted:false,
    activation_command_completion_ack_recorded:false,
    activation_command_completion_ack_persisted:false,
    activation_command_completion_ack_accepted:false,
    activation_command_completion_ack_materialized:false,
    activation_command_completion_ack_delivered:false,
    activation_command_shape_registered:false,
    activation_command_enabled:false,
    activation_command_invoked:false,
    activation_command_dispatched:false,
    activation_command_noop_decision_recorded:false,
    activation_command_noop_decision_persisted:false,
    activation_command_handoff_recorded:false,
    activation_command_handoff_persisted:false,
    activation_command_handoff_accepted:false,
    activation_command_handoff_materialized:false,
    activation_allowed_by_result_receipt:false,
    activation_allowed_by_command_handoff:false,
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
    rollback_validation_accepted:false,
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
    activation_command_result_receipt_surfaces:[
      "source_noop_handoff_report_required",
      "accepted_activation_closure_packet_required",
      "activation_command_disabled_and_not_invoked_required",
      "receipt_schema_and_request_id_required",
      "receipt_hash_signature_timestamp_required",
      "receipt_status_must_remain_blocked_noop_required",
      "receipt_record_persist_materialize_denied",
      "receipt_filesystem_ledger_index_delivery_denied",
      "completion_ack_denied",
      "activation_from_receipt_denied",
      "memory_write_live_mutation_rollback_denied",
      "external_public_release_install_restart_denied"
    ],
    activation_command_result_receipt_fixtures:[
      {
        id:"activation-result-receipt-missing-source-noop-handoff",
        receipt_requested:true,
        receipt_status:"blocked_noop",
        source_noop_handoff_present:false,
        source_noop_handoff_ready:false,
        receipt_allowed:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        receipt_materialized:false,
        receipt_filesystem_written:false,
        receipt_noop_confirmed:true,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"source_activation_command_noop_handoff_required"
      },
      {
        id:"activation-result-receipt-record-attempt",
        receipt_requested:true,
        receipt_record_requested:true,
        receipt_status:"blocked_noop",
        source_noop_handoff_present:true,
        receipt_allowed:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        receipt_materialized:false,
        receipt_filesystem_written:false,
        receipt_noop_confirmed:true,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"activation_command_result_receipt_recording_denied"
      },
      {
        id:"activation-result-receipt-persist-attempt",
        receipt_requested:true,
        receipt_persist_requested:true,
        receipt_status:"blocked_noop",
        source_noop_handoff_present:true,
        receipt_allowed:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        receipt_materialized:false,
        receipt_filesystem_written:false,
        receipt_noop_confirmed:true,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"activation_command_result_receipt_persistence_denied"
      },
      {
        id:"activation-result-receipt-materialize-filesystem-attempt",
        receipt_requested:true,
        receipt_materialize_requested:true,
        receipt_filesystem_write_requested:true,
        receipt_status:"blocked_noop",
        source_noop_handoff_present:true,
        receipt_allowed:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        receipt_materialized:false,
        receipt_filesystem_written:false,
        receipt_noop_confirmed:true,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"result_receipt_materialization_and_filesystem_write_denied"
      },
      {
        id:"activation-result-receipt-ledger-index-delivery-attempt",
        receipt_requested:true,
        receipt_ledger_write_requested:true,
        receipt_index_requested:true,
        receipt_delivery_requested:true,
        receipt_status:"blocked_noop",
        source_noop_handoff_present:true,
        receipt_allowed:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        receipt_materialized:false,
        receipt_filesystem_written:false,
        receipt_ledger_written:false,
        receipt_indexed:false,
        receipt_delivered:false,
        receipt_noop_confirmed:true,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"result_receipt_ledger_index_delivery_denied"
      },
      {
        id:"activation-result-receipt-acceptance-as-approval-attempt",
        receipt_requested:true,
        receipt_acceptance_requested:true,
        receipt_status:"blocked_noop",
        source_noop_handoff_present:true,
        receipt_allowed:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        receipt_materialized:false,
        receipt_filesystem_written:false,
        receipt_noop_confirmed:true,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"result_receipt_cannot_become_operator_approval"
      },
      {
        id:"activation-result-receipt-completion-ack-attempt",
        receipt_requested:true,
        completion_ack_requested:true,
        activation_completion_ack_requested:true,
        receipt_status:"blocked_noop",
        source_noop_handoff_present:true,
        receipt_allowed:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        receipt_materialized:false,
        receipt_filesystem_written:false,
        receipt_noop_confirmed:true,
        completion_ack_recorded:false,
        completion_ack_persisted:false,
        completion_ack_accepted:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"activation_completion_ack_denied"
      },
      {
        id:"activation-result-receipt-non-noop-status-attempt",
        receipt_requested:true,
        receipt_status_requested:"completed",
        receipt_status:"blocked_noop",
        source_noop_handoff_present:true,
        receipt_allowed:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        receipt_materialized:false,
        receipt_filesystem_written:false,
        receipt_noop_confirmed:true,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"result_receipt_status_must_remain_blocked_noop"
      },
      {
        id:"activation-result-receipt-memory-write-rollback-attempt",
        receipt_requested:true,
        receipt_status:"blocked_noop",
        source_noop_handoff_present:true,
        memory_store_write_requested:true,
        rollback_execution_requested:true,
        receipt_allowed:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        receipt_materialized:false,
        receipt_filesystem_written:false,
        receipt_noop_confirmed:true,
        completion_ack_recorded:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        reason:"result_receipt_cannot_enable_memory_write_or_rollback"
      },
      {
        id:"activation-result-receipt-external-public-install-attempt",
        receipt_requested:true,
        receipt_status:"blocked_noop",
        source_noop_handoff_present:true,
        external_send_requested:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        install_requested:true,
        launchd_restart_requested:true,
        active_binary_mutation_requested:true,
        receipt_allowed:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        receipt_materialized:false,
        receipt_filesystem_written:false,
        receipt_noop_confirmed:true,
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
        reason:"result_receipt_cannot_send_publish_install_restart_or_mutate_active_binary"
      }
    ],
    denied_by_activation_command_result_receipt:[
      "source_activation_command_noop_handoff_required",
      "accepted_activation_closure_packet_required",
      "activation_command_enabled_denied",
      "activation_command_invocation_denied",
      "activation_command_dispatch_denied",
      "receipt_schema_acceptance_denied",
      "receipt_recording_denied",
      "receipt_persistence_denied",
      "receipt_acceptance_denied",
      "receipt_materialization_denied",
      "receipt_filesystem_write_denied",
      "receipt_ledger_write_denied",
      "receipt_indexing_denied",
      "receipt_delivery_denied",
      "completion_ack_recording_denied",
      "completion_ack_persistence_denied",
      "completion_ack_acceptance_denied",
      "activation_from_receipt_denied",
      "memory_store_write_denied",
      "live_mutation_execution_denied",
      "rollback_execution_denied",
      "secret_material_read_denied",
      "provider_prompt_replay_denied",
      "external_send_public_claim_release_artifact_denied",
      "install_restart_active_binary_mutation_denied"
    ],
    side_effects:{
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
      activation_command_shape_registered:false,
      activation_command_enabled:false,
      activation_command_invoked:false,
      activation_command_dispatched:false,
      activation_command_noop_decision_recorded:false,
      activation_command_noop_decision_persisted:false,
      activation_command_handoff_recorded:false,
      activation_command_handoff_persisted:false,
      activation_command_handoff_materialized:false,
      activation_closure_packet_recorded:false,
      activation_closure_packet_persisted:false,
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
  and .memory_write_execution_activation_command_result_receipt_no_persistence_ready == true
  and .activation_command_result_receipt_no_persistence_mode == "memory_write_execution_activation_command_result_receipt_no_persistence_denial"
  and .source_activation_command_noop_handoff_ready == true
  and .source_activation_command_noop_handoff_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_activation_command_handoff_surface_count == 13
  and .ready_activation_command_handoff_surface_count == 13
  and .required_activation_command_result_receipt_surface_count == 12
  and .ready_activation_command_result_receipt_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_surface_count == 12
  and .required_activation_command_result_receipt_fixture_count == 10
  and .activation_command_result_receipt_fixture_count == 10
  and .blocked_activation_command_result_receipt_fixture_count == 10
  and .noop_activation_command_result_receipt_fixture_count == 10
  and .allowed_activation_command_result_receipt_fixture_count == 0
  and .accepted_activation_command_result_receipt_fixture_count == 0
  and .activation_command_result_receipt_denied_count == 10
  and .activation_command_result_receipt_performed_count == 0
  and .activation_command_result_receipt_shape_registered == false
  and .activation_command_result_receipt_schema_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_result_receipt_ledger_written == false
  and .activation_command_result_receipt_indexed == false
  and .activation_command_result_receipt_enqueued == false
  and .activation_command_result_receipt_delivered == false
  and .activation_command_result_receipt_hash_bound == false
  and .activation_command_result_receipt_signature_hash_recorded == false
  and .activation_command_result_receipt_timestamp_recorded == false
  and .activation_command_result_receipt_operator_identity_accepted == false
  and .activation_command_result_receipt_status_accepted == false
  and .activation_command_result_receipt_blocked_noop_status_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_persisted == false
  and .activation_command_completion_ack_accepted == false
  and .activation_command_completion_ack_materialized == false
  and .activation_command_completion_ack_delivered == false
  and .activation_command_shape_registered == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_command_handoff_recorded == false
  and .activation_command_handoff_persisted == false
  and .activation_command_handoff_accepted == false
  and .activation_command_handoff_materialized == false
  and .activation_allowed_by_result_receipt == false
  and .activation_allowed_by_command_handoff == false
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
  and (.activation_command_result_receipt_surfaces | length) == 12
  and (.activation_command_result_receipt_fixtures | length) == 10
  and (.activation_command_result_receipt_fixtures | all(.receipt_status == "blocked_noop" and .receipt_allowed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .receipt_materialized == false and .receipt_filesystem_written == false and .receipt_noop_confirmed == true and .completion_ack_recorded == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false))
  and ([.activation_command_result_receipt_fixtures[] | select(.receipt_record_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.receipt_persist_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.receipt_materialize_requested == true and .receipt_filesystem_write_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.receipt_ledger_write_requested == true and .receipt_index_requested == true and .receipt_delivery_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.completion_ack_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.receipt_status_requested == "completed")] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.memory_store_write_requested == true and .rollback_execution_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_fixtures[] | select(.external_send_requested == true and .install_requested == true and .active_binary_mutation_requested == true)] | length) == 1
  and (.denied_by_activation_command_result_receipt | length) == 25
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution activation command result receipt no-persistence gate passed"
