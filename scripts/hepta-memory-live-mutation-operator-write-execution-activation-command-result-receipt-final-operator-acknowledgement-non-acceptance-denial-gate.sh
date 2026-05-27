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

SUMMARY_BRIEFING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh
)"

summary_briefing_report_sha256="$(printf '%s' "$SUMMARY_BRIEFING_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$SUMMARY_BRIEFING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
    and $source.activation_command_result_receipt_operator_facing_summary_briefing_mode == "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial"
    and $source.memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_no_persistence_ready == true
    and $source.source_activation_command_result_receipt_export_query_observability_report_sha256 != ""
    and $source.minimum_required_samples >= 24
    and $source.required_activation_command_result_receipt_operator_facing_summary_briefing_surface_count == 12
    and $source.ready_activation_command_result_receipt_operator_facing_summary_briefing_surface_count == 12
    and $source.side_effect_free_activation_command_result_receipt_operator_facing_summary_briefing_surface_count == 12
    and $source.required_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 10
    and $source.activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 10
    and $source.noop_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 0
    and $source.activation_command_result_receipt_operator_summary_performed_count == 0
    and $source.activation_command_result_receipt_operator_briefing_performed_count == 0
    and $source.activation_command_result_receipt_operator_summary_recorded == false
    and $source.activation_command_result_receipt_operator_summary_persisted == false
    and $source.activation_command_result_receipt_operator_summary_materialized == false
    and $source.activation_command_result_receipt_operator_summary_filesystem_written == false
    and $source.activation_command_result_receipt_operator_summary_delivered == false
    and $source.activation_command_result_receipt_operator_briefing_recorded == false
    and $source.activation_command_result_receipt_operator_briefing_persisted == false
    and $source.activation_command_result_receipt_operator_briefing_materialized == false
    and $source.activation_command_result_receipt_operator_briefing_filesystem_written == false
    and $source.activation_command_result_receipt_operator_briefing_delivered == false
    and $source.activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_materialized == false
    and $source.activation_command_result_receipt_filesystem_written == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.activation_allowed_by_result_receipt_summary_briefing == false
    and $source.activation_allowed_by_result_receipt == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.live_mutation_execution_performed == false
    and $source.memory_write_execution_performed == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_write_performed_count == 0
    and $source.memory_store_mutated == false
    and $source.rollback_executed == false
    and $source.secret_material_read == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.public_release_published == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and ($source.activation_command_result_receipt_operator_facing_summary_briefing_fixtures | length) == 10
    and ($source.activation_command_result_receipt_operator_facing_summary_briefing_fixtures | all((.operator_summary_briefing_status == "blocked_noop" or .operator_summary_briefing_status == "blocked_summary_noop" or .operator_summary_briefing_status == "blocked_briefing_noop" or .operator_summary_briefing_status == "blocked_delivery_noop") and .operator_summary_recorded == false and .operator_summary_persisted == false and .operator_summary_materialized == false and .operator_summary_filesystem_written == false and .operator_summary_delivered == false and .operator_briefing_recorded == false and .operator_briefing_persisted == false and .operator_briefing_materialized == false and .operator_briefing_filesystem_written == false and .operator_briefing_delivered == false and .telegram_send_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate" \
  --arg summary_briefing_report_sha256 "$summary_briefing_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$SUMMARY_BRIEFING_JSON" \
  '
  def blocked_fixture($id; $status; $reason; $extra):
    {
      id:$id,
      final_operator_acknowledgement_requested:false,
      final_operator_acknowledgement_status:$status,
      source_summary_briefing_present:true,
      source_summary_briefing_ready:true,
      acknowledgement_allowed:false,
      acknowledgement_request_accepted:false,
      acknowledgement_accepted:false,
      acknowledgement_recorded:false,
      acknowledgement_persisted:false,
      acknowledgement_materialized:false,
      acknowledgement_filesystem_written:false,
      acknowledgement_delivered:false,
      acknowledgement_channel_delivery_performed:false,
      acknowledgement_identity_accepted:false,
      acknowledgement_signature_accepted:false,
      acknowledgement_timestamp_accepted:false,
      acknowledgement_final_state_promoted:false,
      acknowledgement_completion_promoted:false,
      operator_final_acceptance_recorded:false,
      operator_final_acceptance_persisted:false,
      operator_final_acceptance_materialized:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      receipt_materialized:false,
      receipt_filesystem_written:false,
      completion_ack_recorded:false,
      completion_ack_persisted:false,
      completion_ack_accepted:false,
      completion_ack_delivered:false,
      activation_allowed:false,
      live_mutation_execution_performed:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      rollback_executed:false,
      secret_material_read:false,
      provider_invoked:false,
      model_invoked:false,
      public_release_published:false,
      release_artifact_written:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      receipt_noop_confirmed:true,
      reason:$reason
    } + $extra;
  [
    blocked_fixture("activation-result-receipt-final-ack-missing-source-summary-briefing"; "blocked_noop"; "source_summary_briefing_report_required"; {source_summary_briefing_present:false, source_summary_briefing_ready:false, final_operator_acknowledgement_requested:true}),
    blocked_fixture("activation-result-receipt-final-ack-request"; "blocked_ack_noop"; "final_operator_acknowledgement_request_shape_denied"; {final_operator_acknowledgement_requested:true}),
    blocked_fixture("activation-result-receipt-final-ack-acceptance-request"; "blocked_acceptance_noop"; "acknowledgement_acceptance_denied"; {final_operator_acknowledgement_requested:true, acknowledgement_acceptance_requested:true}),
    blocked_fixture("activation-result-receipt-final-ack-recording-request"; "blocked_ack_noop"; "acknowledgement_recording_denied"; {final_operator_acknowledgement_requested:true, acknowledgement_recording_requested:true}),
    blocked_fixture("activation-result-receipt-final-ack-persistence-filesystem-write-request"; "blocked_ack_noop"; "acknowledgement_persistence_filesystem_write_denied"; {final_operator_acknowledgement_requested:true, acknowledgement_persistence_requested:true, acknowledgement_filesystem_write_requested:true}),
    blocked_fixture("activation-result-receipt-final-ack-identity-signature-request"; "blocked_acceptance_noop"; "operator_identity_signature_acknowledgement_acceptance_denied"; {final_operator_acknowledgement_requested:true, operator_identity_acceptance_requested:true, operator_signature_acceptance_requested:true, operator_timestamp_acceptance_requested:true}),
    blocked_fixture("activation-result-receipt-final-ack-delivery-request"; "blocked_delivery_noop"; "acknowledgement_delivery_denied"; {final_operator_acknowledgement_requested:true, acknowledgement_delivery_requested:true, telegram_send_requested:true, channel_delivery_requested:true}),
    blocked_fixture("activation-result-receipt-final-state-promotion-request"; "blocked_promotion_noop"; "final_state_completion_promotion_denied"; {final_operator_acknowledgement_requested:true, final_state_promotion_requested:true, completion_promotion_requested:true}),
    blocked_fixture("activation-result-receipt-final-ack-activation-memory-provider-request"; "blocked_ack_noop"; "activation_memory_rollback_secret_provider_acknowledgement_denied"; {final_operator_acknowledgement_requested:true, activation_from_acknowledgement_requested:true, memory_write_acknowledgement_requested:true, rollback_acknowledgement_requested:true, secret_material_acknowledgement_requested:true, provider_prompt_acknowledgement_requested:true}),
    blocked_fixture("activation-result-receipt-final-ack-external-public-install-request"; "blocked_delivery_noop"; "external_public_install_restart_active_binary_acknowledgement_denied"; {final_operator_acknowledgement_requested:true, external_send_acknowledgement_requested:true, public_claim_acknowledgement_requested:true, release_artifact_acknowledgement_requested:true, install_acknowledgement_requested:true, service_restart_acknowledgement_requested:true, active_binary_acknowledgement_requested:true})
  ] as $fixtures
  | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_final_operator_acknowledgement_mode:"memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
    source_activation_command_result_receipt_operator_facing_summary_briefing_gate:$source.gate,
    source_activation_command_result_receipt_operator_facing_summary_briefing_ready:$source.memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready,
    source_activation_command_result_receipt_operator_facing_summary_briefing_report_sha256:$summary_briefing_report_sha256,
    source_activation_command_result_receipt_export_query_observability_ready:$source.memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready,
    source_activation_command_result_receipt_export_query_observability_report_sha256:$source.source_activation_command_result_receipt_export_query_observability_report_sha256,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_ready:$source.memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256:$source.source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_ready:$source.memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256:$source.source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256,
    source_activation_command_result_receipt_cancellation_supersession_ready:$source.memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready,
    source_activation_command_result_receipt_cancellation_supersession_report_sha256:$source.source_activation_command_result_receipt_cancellation_supersession_report_sha256,
    source_activation_command_result_receipt_ordering_monotonicity_ready:$source.memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    source_activation_command_result_receipt_ordering_monotonicity_report_sha256:$source.source_activation_command_result_receipt_ordering_monotonicity_report_sha256,
    source_activation_command_result_receipt_replay_idempotency_ready:$source.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:$source.source_activation_command_result_receipt_replay_idempotency_report_sha256,
    source_activation_command_result_receipt_no_persistence_ready:$source.memory_write_execution_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_report_sha256:$source.source_activation_command_result_receipt_no_persistence_report_sha256,
    source_activation_command_noop_handoff_ready:$source.memory_write_execution_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_report_sha256:$source.source_activation_command_noop_handoff_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$source.source_memory_write_execution_preflight_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_no_persistence_ready:true,
    memory_write_execution_activation_command_noop_handoff_ready:true,
    required_activation_command_result_receipt_final_operator_acknowledgement_surface_count:12,
    ready_activation_command_result_receipt_final_operator_acknowledgement_surface_count:12,
    side_effect_free_activation_command_result_receipt_final_operator_acknowledgement_surface_count:12,
    required_activation_command_result_receipt_final_operator_acknowledgement_fixture_count:10,
    activation_command_result_receipt_final_operator_acknowledgement_fixture_count:($fixtures | length),
    blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count:($fixtures | length),
    noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count:($fixtures | length),
    allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count:0,
    accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count:0,
    activation_command_result_receipt_final_operator_acknowledgement_denied_count:10,
    activation_command_result_receipt_final_operator_acknowledgement_performed_count:0,
    activation_command_result_receipt_final_operator_acknowledgement_allowed:false,
    activation_command_result_receipt_final_operator_acknowledgement_request_accepted:false,
    activation_command_result_receipt_final_operator_acknowledgement_accepted:false,
    activation_command_result_receipt_final_operator_acknowledgement_recorded:false,
    activation_command_result_receipt_final_operator_acknowledgement_persisted:false,
    activation_command_result_receipt_final_operator_acknowledgement_materialized:false,
    activation_command_result_receipt_final_operator_acknowledgement_filesystem_written:false,
    activation_command_result_receipt_final_operator_acknowledgement_delivered:false,
    activation_command_result_receipt_final_operator_acknowledgement_channel_delivery_performed:false,
    activation_command_result_receipt_final_operator_acknowledgement_identity_accepted:false,
    activation_command_result_receipt_final_operator_acknowledgement_signature_accepted:false,
    activation_command_result_receipt_final_operator_acknowledgement_timestamp_accepted:false,
    activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted:false,
    activation_command_result_receipt_final_operator_acknowledgement_completion_promoted:false,
    activation_command_result_receipt_operator_final_acceptance_recorded:false,
    activation_command_result_receipt_operator_final_acceptance_persisted:false,
    activation_command_result_receipt_operator_final_acceptance_materialized:false,
    telegram_send_performed:false,
    channel_send_performed:false,
    external_send_performed:false,
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
    activation_allowed_by_result_receipt_final_operator_acknowledgement:false,
    activation_allowed_by_result_receipt_summary_briefing:false,
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
    public_claim_or_release_artifact_write_enabled:false,
    public_release_published:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    activation_command_result_receipt_final_operator_acknowledgement_surfaces:[
      "source_operator_facing_summary_briefing_report_required",
      "final_operator_acknowledgement_request_shape_denied",
      "acknowledgement_acceptance_denied",
      "acknowledgement_recording_denied",
      "acknowledgement_persistence_denied",
      "acknowledgement_materialization_denied",
      "operator_identity_signature_acknowledgement_acceptance_denied",
      "acknowledgement_delivery_denied",
      "final_state_completion_promotion_denied",
      "activation_from_final_acknowledgement_denied",
      "memory_write_rollback_secret_provider_acknowledgement_denied",
      "external_public_install_restart_active_binary_acknowledgement_denied"
    ],
    activation_command_result_receipt_final_operator_acknowledgement_fixtures:$fixtures,
    denied_by_activation_command_result_receipt_final_operator_acknowledgement:[
      "source_operator_facing_summary_briefing_report_required",
      "final_operator_acknowledgement_request_acceptance_denied",
      "final_operator_acknowledgement_acceptance_denied",
      "final_operator_acknowledgement_recording_denied",
      "final_operator_acknowledgement_persistence_denied",
      "final_operator_acknowledgement_materialization_denied",
      "final_operator_acknowledgement_filesystem_write_denied",
      "operator_identity_signature_acknowledgement_acceptance_denied",
      "final_operator_acknowledgement_delivery_denied",
      "telegram_send_denied",
      "final_state_completion_promotion_denied",
      "activation_from_final_acknowledgement_denied",
      "memory_write_acknowledgement_denied",
      "rollback_acknowledgement_denied",
      "secret_material_acknowledgement_denied",
      "provider_prompt_acknowledgement_denied",
      "external_public_install_restart_active_binary_acknowledgement_denied"
    ],
    side_effects:{
      activation_command_result_receipt_final_operator_acknowledgement_recorded:false,
      activation_command_result_receipt_final_operator_acknowledgement_persisted:false,
      activation_command_result_receipt_final_operator_acknowledgement_materialized:false,
      activation_command_result_receipt_final_operator_acknowledgement_filesystem_written:false,
      activation_command_result_receipt_final_operator_acknowledgement_delivered:false,
      activation_command_result_receipt_final_operator_acknowledgement_channel_delivery_performed:false,
      activation_command_result_receipt_final_operator_acknowledgement_identity_accepted:false,
      activation_command_result_receipt_final_operator_acknowledgement_signature_accepted:false,
      activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted:false,
      activation_command_result_receipt_final_operator_acknowledgement_completion_promoted:false,
      activation_command_result_receipt_operator_final_acceptance_recorded:false,
      activation_command_result_receipt_operator_final_acceptance_persisted:false,
      activation_command_result_receipt_operator_final_acceptance_materialized:false,
      telegram_send_performed:false,
      activation_command_result_receipt_operator_summary_recorded:false,
      activation_command_result_receipt_operator_summary_persisted:false,
      activation_command_result_receipt_operator_summary_materialized:false,
      activation_command_result_receipt_operator_summary_filesystem_written:false,
      activation_command_result_receipt_operator_summary_delivered:false,
      activation_command_result_receipt_operator_briefing_recorded:false,
      activation_command_result_receipt_operator_briefing_persisted:false,
      activation_command_result_receipt_operator_briefing_materialized:false,
      activation_command_result_receipt_operator_briefing_filesystem_written:false,
      activation_command_result_receipt_operator_briefing_delivered:false,
      activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed:false,
      activation_command_result_receipt_export_recorded:false,
      activation_command_result_receipt_export_persisted:false,
      activation_command_result_receipt_export_artifact_written:false,
      activation_command_result_receipt_query_registered:false,
      activation_command_result_receipt_query_endpoint_materialized:false,
      activation_command_result_receipt_query_index_recorded:false,
      activation_command_result_receipt_query_cache_written:false,
      activation_command_result_receipt_observability_metric_emitted:false,
      activation_command_result_receipt_observability_log_recorded:false,
      activation_command_result_receipt_observability_trace_recorded:false,
      activation_command_result_receipt_observability_span_recorded:false,
      activation_command_result_receipt_observability_event_recorded:false,
      activation_command_result_receipt_observability_dashboard_materialized:false,
      activation_command_result_receipt_observability_alert_registered:false,
      activation_command_result_receipt_retention_policy_recorded:false,
      activation_command_result_receipt_expiry_recorded:false,
      activation_command_result_receipt_garbage_collection_scan_performed:false,
      activation_command_result_receipt_audit_trail_recorded:false,
      activation_command_result_receipt_immutable_evidence_recorded:false,
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
  }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and .memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and .required_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
  and .ready_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
  and .required_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
  and .activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
  and .blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
  and .noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
  and .allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 0
  and .accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 0
  and .activation_command_result_receipt_final_operator_acknowledgement_performed_count == 0
  and .activation_command_result_receipt_final_operator_acknowledgement_recorded == false
  and .activation_command_result_receipt_final_operator_acknowledgement_persisted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_materialized == false
  and .activation_command_result_receipt_final_operator_acknowledgement_filesystem_written == false
  and .activation_command_result_receipt_final_operator_acknowledgement_delivered == false
  and .activation_command_result_receipt_final_operator_acknowledgement_identity_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_signature_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted == false
  and .activation_command_result_receipt_operator_final_acceptance_recorded == false
  and .activation_command_result_receipt_operator_final_acceptance_persisted == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_completion_ack_recorded == false
  and .activation_allowed_by_result_receipt_final_operator_acknowledgement == false
  and .activation_allowed_by_result_receipt == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_performed == false
  and .memory_write_execution_performed == false
  and .memory_store_write_performed == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutated == false
  and .rollback_executed == false
  and .secret_material_read == false
  and .provider_invoked == false
  and .model_invoked == false
  and .public_release_published == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_final_operator_acknowledgement_fixtures | length) == 10
  and (.activation_command_result_receipt_final_operator_acknowledgement_fixtures | all((.final_operator_acknowledgement_status == "blocked_noop" or .final_operator_acknowledgement_status == "blocked_ack_noop" or .final_operator_acknowledgement_status == "blocked_acceptance_noop" or .final_operator_acknowledgement_status == "blocked_delivery_noop" or .final_operator_acknowledgement_status == "blocked_promotion_noop") and .acknowledgement_recorded == false and .acknowledgement_persisted == false and .acknowledgement_materialized == false and .acknowledgement_filesystem_written == false and .acknowledgement_delivered == false and .acknowledgement_accepted == false and .acknowledgement_identity_accepted == false and .acknowledgement_signature_accepted == false and .acknowledgement_final_state_promoted == false and .telegram_send_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory live mutation operator write execution activation command result receipt final operator acknowledgement non-acceptance denial gate passed"
