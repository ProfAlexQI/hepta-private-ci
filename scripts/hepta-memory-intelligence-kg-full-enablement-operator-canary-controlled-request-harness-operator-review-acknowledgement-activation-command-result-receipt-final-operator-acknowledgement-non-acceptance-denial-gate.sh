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

SUMMARY_BRIEFING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh
)"

summary_briefing_report_sha256="$(sha256_text "$SUMMARY_BRIEFING_JSON")"

final_acknowledgement_fixtures_json="$(
  jq -n '
    def final_ack_fixture($id; $status; $reason; $extra):
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
        activation_command_result_receipt_recorded:false,
        activation_command_result_receipt_persisted:false,
        activation_command_result_receipt_accepted:false,
        activation_command_result_receipt_materialized:false,
        activation_command_result_receipt_filesystem_written:false,
        activation_command_completion_ack_recorded:false,
        activation_command_completion_ack_persisted:false,
        activation_command_completion_ack_accepted:false,
        activation_command_completion_ack_delivered:false,
        activation_allowed:false,
        activation_performed:false,
        live_mutation_execution_performed:false,
        runtime_router_mutated:false,
        runtime_attachment_performed:false,
        live_context_attached:false,
        context_injection_performed:false,
        adapter_invoked:false,
        provider_invoked:false,
        model_invoked:false,
        provider_prompt_replayed:false,
        auth_secret_read:false,
        credential_read:false,
        secret_file_read:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        rollback_executed:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        release_artifact_written:false,
        install_executed:false,
        launchd_mutated:false,
        service_restart_performed:false,
        active_binary_mutated:false,
        final_acknowledgement_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      final_ack_fixture("operator-canary-controlled-request-harness-final-ack-missing-source-summary-briefing"; "blocked_noop"; "source_summary_briefing_report_required"; {source_summary_briefing_present:false, source_summary_briefing_ready:false, final_operator_acknowledgement_requested:true}),
      final_ack_fixture("operator-canary-controlled-request-harness-final-ack-request"; "blocked_ack_noop"; "final_operator_acknowledgement_request_shape_denied"; {final_operator_acknowledgement_requested:true}),
      final_ack_fixture("operator-canary-controlled-request-harness-final-ack-acceptance-request"; "blocked_acceptance_noop"; "final_operator_acknowledgement_acceptance_denied"; {final_operator_acknowledgement_requested:true, acknowledgement_acceptance_requested:true}),
      final_ack_fixture("operator-canary-controlled-request-harness-final-ack-recording-request"; "blocked_ack_noop"; "final_operator_acknowledgement_recording_denied"; {final_operator_acknowledgement_requested:true, acknowledgement_recording_requested:true}),
      final_ack_fixture("operator-canary-controlled-request-harness-final-ack-persistence-filesystem-write-request"; "blocked_ack_noop"; "final_operator_acknowledgement_persistence_filesystem_write_denied"; {final_operator_acknowledgement_requested:true, acknowledgement_persistence_requested:true, acknowledgement_filesystem_write_requested:true}),
      final_ack_fixture("operator-canary-controlled-request-harness-final-ack-identity-signature-timestamp-request"; "blocked_acceptance_noop"; "operator_identity_signature_timestamp_acknowledgement_acceptance_denied"; {final_operator_acknowledgement_requested:true, operator_identity_acceptance_requested:true, operator_signature_acceptance_requested:true, operator_timestamp_acceptance_requested:true}),
      final_ack_fixture("operator-canary-controlled-request-harness-final-ack-delivery-request"; "blocked_delivery_noop"; "final_operator_acknowledgement_delivery_denied"; {final_operator_acknowledgement_requested:true, acknowledgement_delivery_requested:true, telegram_send_requested:true, channel_delivery_requested:true}),
      final_ack_fixture("operator-canary-controlled-request-harness-final-state-promotion-request"; "blocked_promotion_noop"; "final_state_completion_promotion_denied"; {final_operator_acknowledgement_requested:true, final_state_promotion_requested:true, completion_promotion_requested:true}),
      final_ack_fixture("operator-canary-controlled-request-harness-final-ack-activation-memory-kg-provider-request"; "blocked_ack_noop"; "activation_memory_kg_rollback_secret_provider_acknowledgement_denied"; {final_operator_acknowledgement_requested:true, activation_from_acknowledgement_requested:true, memory_store_acknowledgement_requested:true, live_kg_acknowledgement_requested:true, rollback_acknowledgement_requested:true, secret_material_acknowledgement_requested:true, provider_prompt_acknowledgement_requested:true}),
      final_ack_fixture("operator-canary-controlled-request-harness-final-ack-external-public-install-request"; "blocked_delivery_noop"; "external_public_install_restart_active_binary_acknowledgement_denied"; {final_operator_acknowledgement_requested:true, external_send_acknowledgement_requested:true, public_claim_acknowledgement_requested:true, release_artifact_acknowledgement_requested:true, install_acknowledgement_requested:true, service_restart_acknowledgement_requested:true, active_binary_acknowledgement_requested:true})
    ]
  '
)"

final_acknowledgement_fixtures_sha256="$(sha256_text "$final_acknowledgement_fixtures_json")"
final_acknowledgement_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial:$summary_briefing_report_sha256:$final_acknowledgement_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
final_acknowledgement_policy_hash_sha256="$(
  sha256_text "operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement:no-accept:no-record:no-persist:no-deliver:no-promote:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "final_ack=false;acceptance=false;record=false;persist=false;deliver=false;promotion=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false"
)"

jq -n -e \
  --argjson source "$SUMMARY_BRIEFING_JSON" \
  --argjson fixtures "$final_acknowledgement_fixtures_json" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
    and $source.activation_command_result_receipt_operator_facing_summary_briefing_schema_version == "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status == "blocked"
    and $source.source_activation_command_result_receipt_export_query_observability_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready == true
    and $source.minimum_required_samples >= 24
    and $source.operator_facing_summary_briefing_surface_count == 12
    and $source.operator_facing_summary_briefing_surface_ready_count == 12
    and $source.operator_facing_summary_briefing_side_effect_free_surface_count == 12
    and $source.operator_facing_summary_briefing_fixture_count == 10
    and $source.blocked_operator_facing_summary_briefing_fixture_count == 10
    and $source.noop_operator_facing_summary_briefing_fixture_count == 10
    and $source.allowed_operator_facing_summary_briefing_fixture_count == 0
    and $source.accepted_operator_facing_summary_briefing_fixture_count == 0
    and $source.operator_summary_performed_count == 0
    and $source.operator_briefing_performed_count == 0
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
    and $source.activation_allowed_by_result_receipt_summary_briefing == false
    and $source.activation_allowed_by_result_receipt == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_activated == false
    and $source.runtime_router_mutated == false
    and $source.runtime_attachment_performed == false
    and $source.live_context_attached == false
    and $source.context_injection_performed == false
    and $source.adapter_invoked == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.auth_secret_read == false
    and $source.credential_read == false
    and $source.secret_file_read == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.live_kg_write_performed == false
    and $source.rollback_executed == false
    and ($source.operator_facing_summary_briefing_fixtures | length) == 10
    and ($source.operator_facing_summary_briefing_fixtures | all(
      (.operator_summary_briefing_status == "blocked_noop" or .operator_summary_briefing_status == "blocked_summary_noop" or .operator_summary_briefing_status == "blocked_briefing_noop" or .operator_summary_briefing_status == "blocked_delivery_noop")
      and .operator_summary_recorded == false
      and .operator_summary_persisted == false
      and .operator_briefing_recorded == false
      and .operator_briefing_persisted == false
      and .telegram_send_performed == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_activated == false
      and .provider_invoked == false
      and .model_invoked == false
      and .credential_read == false
      and .secret_file_read == false
      and .memory_store_write_performed == false
      and .memory_store_mutated == false
      and .live_kg_write_performed == false
      and .summary_briefing_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(.action == "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial" and .status == "allowed_report_only_next_slice" and .accepts_operator_acknowledgement == false and .persists_acknowledgement == false and .activates_runtime == false and .invokes_model == false and .writes_kg == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.final_operator_acknowledgement_status == "blocked_noop" or .final_operator_acknowledgement_status == "blocked_ack_noop" or .final_operator_acknowledgement_status == "blocked_acceptance_noop" or .final_operator_acknowledgement_status == "blocked_delivery_noop" or .final_operator_acknowledgement_status == "blocked_promotion_noop")
      and .acknowledgement_accepted == false
      and .acknowledgement_recorded == false
      and .acknowledgement_persisted == false
      and .acknowledgement_materialized == false
      and .acknowledgement_filesystem_written == false
      and .acknowledgement_delivered == false
      and .telegram_send_performed == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_allowed == false
      and .activation_performed == false
      and .live_mutation_execution_performed == false
      and .provider_invoked == false
      and .model_invoked == false
      and .memory_store_write_performed == false
      and .memory_store_mutated == false
      and .live_kg_write_performed == false
      and .credential_read == false
      and .secret_file_read == false
      and .final_acknowledgement_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate" \
  --arg summary_briefing_report_sha256 "$summary_briefing_report_sha256" \
  --arg final_acknowledgement_fixtures_sha256 "$final_acknowledgement_fixtures_sha256" \
  --arg final_acknowledgement_contract_hash_sha256 "$final_acknowledgement_contract_hash_sha256" \
  --arg final_acknowledgement_policy_hash_sha256 "$final_acknowledgement_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$SUMMARY_BRIEFING_JSON" \
  --argjson fixtures "$final_acknowledgement_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_final_operator_acknowledgement_schema_version:"memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
    activation_command_result_receipt_final_operator_acknowledgement_mode:"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_no_ack_no_acceptance_no_delivery",
    source_activation_command_result_receipt_operator_facing_summary_briefing_gate:$source.gate,
    source_activation_command_result_receipt_operator_facing_summary_briefing_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready,
    source_activation_command_result_receipt_operator_facing_summary_briefing_status:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status,
    source_activation_command_result_receipt_operator_facing_summary_briefing_report_sha256:$summary_briefing_report_sha256,
    source_activation_command_result_receipt_export_query_observability_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    source_activation_command_result_receipt_cancellation_supersession_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready,
    source_activation_command_result_receipt_ordering_monotonicity_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_no_persistence_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready,
    final_acknowledgement_fixtures_sha256:$final_acknowledgement_fixtures_sha256,
    final_acknowledgement_contract_hash_sha256:$final_acknowledgement_contract_hash_sha256,
    final_acknowledgement_policy_hash_sha256:$final_acknowledgement_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status:"blocked",
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready,
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
    activation_command_enabled:false,
    activation_command_invoked:false,
    activation_command_dispatched:false,
    activation_allowed:false,
    activation_performed:false,
    live_mutation_execution_ready:false,
    live_mutation_execution_allowed:false,
    live_mutation_execution_performed:false,
    runtime_router_mutated:false,
    runtime_attachment_performed:false,
    live_context_attached:false,
    context_injection_performed:false,
    adapter_invoked:false,
    provider_invoked:false,
    model_invoked:false,
    provider_prompt_replayed:false,
    auth_secret_read:false,
    credential_read:false,
    secret_file_read:false,
    memory_store_write_allowed:false,
    memory_store_write_performed:false,
    memory_store_write_performed_count:0,
    memory_store_mutation_allowed:false,
    memory_store_mutated:false,
    live_kg_write_allowed:false,
    live_kg_write_performed:false,
    rollback_execution_allowed:false,
    rollback_executed:false,
    public_release_claimed:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    install_executed:false,
    launchd_mutated:false,
    service_restart_performed:false,
    active_binary_mutated:false,
    activation_command_result_receipt_final_operator_acknowledgement_surfaces:[
      "source_operator_facing_summary_briefing_report_required",
      "final_operator_acknowledgement_request_shape_denied",
      "final_operator_acknowledgement_acceptance_denied",
      "final_operator_acknowledgement_recording_denied",
      "final_operator_acknowledgement_persistence_denied",
      "final_operator_acknowledgement_materialization_denied",
      "operator_identity_signature_timestamp_acknowledgement_acceptance_denied",
      "final_operator_acknowledgement_delivery_denied",
      "final_state_completion_promotion_denied",
      "activation_from_final_operator_acknowledgement_denied",
      "memory_kg_rollback_secret_provider_acknowledgement_denied",
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
      "operator_identity_signature_timestamp_acknowledgement_acceptance_denied",
      "final_operator_acknowledgement_delivery_denied",
      "telegram_send_denied",
      "final_state_completion_promotion_denied",
      "activation_from_final_operator_acknowledgement_denied",
      "memory_kg_acknowledgement_denied",
      "rollback_acknowledgement_denied",
      "secret_material_acknowledgement_denied",
      "provider_prompt_acknowledgement_denied",
      "external_public_install_restart_active_binary_acknowledgement_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
        status:"allowed_report_only",
        accepts_operator_acknowledgement:false,
        persists_acknowledgement:false,
        activates_runtime:false,
        invokes_model:false,
        writes_memory_or_kg:false
      },
      {
        action:"stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
        status:"allowed_report_only_next_slice",
        accepts_terminal_decision:false,
        claims_public_release:false,
        writes_release_artifact:false,
        activates_runtime:false,
        invokes_model:false,
        writes_memory_or_kg:false
      }
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
      activation_command_result_receipt_final_operator_acknowledgement_timestamp_accepted:false,
      activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted:false,
      activation_command_result_receipt_final_operator_acknowledgement_completion_promoted:false,
      activation_command_result_receipt_operator_final_acceptance_recorded:false,
      activation_command_result_receipt_operator_final_acceptance_persisted:false,
      activation_command_result_receipt_operator_final_acceptance_materialized:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
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
      runtime_router_mutated:false,
      runtime_attachment_performed:false,
      live_context_attached:false,
      context_injection_performed:false,
      adapter_invoked:false,
      provider_invoked:false,
      model_invoked:false,
      provider_prompt_replayed:false,
      auth_secret_read:false,
      credential_read:false,
      secret_file_read:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      live_kg_write_performed:false,
      rollback_executed:false,
      filesystem_written:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      install_executed:false,
      launchd_mutated:false,
      service_restart_performed:false,
      active_binary_mutated:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
  and .activation_command_result_receipt_final_operator_acknowledgement_schema_version == "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status == "blocked"
  and .source_activation_command_result_receipt_operator_facing_summary_briefing_ready == true
  and .source_activation_command_result_receipt_operator_facing_summary_briefing_status == "blocked"
  and .source_activation_command_result_receipt_operator_facing_summary_briefing_report_sha256 != ""
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
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
  and .activation_command_result_receipt_final_operator_acknowledgement_allowed == false
  and .activation_command_result_receipt_final_operator_acknowledgement_request_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_recorded == false
  and .activation_command_result_receipt_final_operator_acknowledgement_persisted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_materialized == false
  and .activation_command_result_receipt_final_operator_acknowledgement_filesystem_written == false
  and .activation_command_result_receipt_final_operator_acknowledgement_delivered == false
  and .activation_command_result_receipt_final_operator_acknowledgement_identity_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_signature_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_timestamp_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_completion_promoted == false
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
  and .activation_allowed_by_result_receipt_summary_briefing == false
  and .activation_allowed_by_result_receipt == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_performed == false
  and .runtime_router_mutated == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .provider_prompt_replayed == false
  and .auth_secret_read == false
  and .credential_read == false
  and .secret_file_read == false
  and .memory_store_write_performed == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .rollback_executed == false
  and .public_release_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_final_operator_acknowledgement_surfaces | length) == 12
  and (.activation_command_result_receipt_final_operator_acknowledgement_fixtures | length) == 10
  and (.activation_command_result_receipt_final_operator_acknowledgement_fixtures | all(
    (.final_operator_acknowledgement_status == "blocked_noop" or .final_operator_acknowledgement_status == "blocked_ack_noop" or .final_operator_acknowledgement_status == "blocked_acceptance_noop" or .final_operator_acknowledgement_status == "blocked_delivery_noop" or .final_operator_acknowledgement_status == "blocked_promotion_noop")
    and .acknowledgement_accepted == false
    and .acknowledgement_recorded == false
    and .acknowledgement_persisted == false
    and .acknowledgement_materialized == false
    and .acknowledgement_filesystem_written == false
    and .acknowledgement_delivered == false
    and .acknowledgement_identity_accepted == false
    and .acknowledgement_signature_accepted == false
    and .acknowledgement_final_state_promoted == false
    and .operator_final_acceptance_recorded == false
    and .operator_final_acceptance_persisted == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_allowed == false
    and .activation_performed == false
    and .live_mutation_execution_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .final_acknowledgement_noop_confirmed == true
  ))
  and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.source_summary_briefing_present == false)] | length) == 1
  and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.final_operator_acknowledgement_requested == true)] | length) == 10
  and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.telegram_send_requested == true and .channel_delivery_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.activation_from_acknowledgement_requested == true and .memory_store_acknowledgement_requested == true and .live_kg_acknowledgement_requested == true and .provider_prompt_acknowledgement_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.external_send_acknowledgement_requested == true and .install_acknowledgement_requested == true and .active_binary_acknowledgement_requested == true)] | length) == 1
  and (.denied_by_activation_command_result_receipt_final_operator_acknowledgement | length) == 17
  and (.allowed_next_actions | any(.action == "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial" and .status == "allowed_report_only_next_slice" and .claims_public_release == false and .writes_release_artifact == false and .activates_runtime == false and .invokes_model == false and .writes_memory_or_kg == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt final operator acknowledgement non-acceptance denial gate passed"
