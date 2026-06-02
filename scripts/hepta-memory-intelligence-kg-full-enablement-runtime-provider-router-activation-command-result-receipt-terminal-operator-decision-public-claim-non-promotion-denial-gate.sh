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

FINAL_ACK_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh
)"

final_ack_report_sha256="$(printf '%s' "$FINAL_ACK_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$FINAL_ACK_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
    and $source.activation_command_result_receipt_final_operator_acknowledgement_mode == "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial"
    and $source.runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
    and $source.source_activation_command_result_receipt_operator_facing_summary_briefing_report_sha256 != ""
    and $source.minimum_required_samples >= 24
    and $source.required_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
    and $source.ready_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
    and $source.side_effect_free_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
    and $source.required_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
    and $source.activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
    and $source.noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 0
    and $source.activation_command_result_receipt_final_operator_acknowledgement_performed_count == 0
    and $source.activation_command_result_receipt_final_operator_acknowledgement_allowed == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_request_accepted == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_accepted == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_recorded == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_persisted == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_materialized == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_filesystem_written == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_delivered == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_identity_accepted == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_signature_accepted == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_timestamp_accepted == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted == false
    and $source.activation_command_result_receipt_final_operator_acknowledgement_completion_promoted == false
    and $source.activation_command_result_receipt_operator_final_acceptance_recorded == false
    and $source.activation_command_result_receipt_operator_final_acceptance_persisted == false
    and $source.activation_command_result_receipt_operator_final_acceptance_materialized == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_materialized == false
    and $source.activation_command_result_receipt_filesystem_written == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.activation_allowed_by_result_receipt_final_operator_acknowledgement == false
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
    and $source.public_ga_claimed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and ($source.activation_command_result_receipt_final_operator_acknowledgement_fixtures | length) == 10
    and ($source.activation_command_result_receipt_final_operator_acknowledgement_fixtures | all((.final_operator_acknowledgement_status == "blocked_noop" or .final_operator_acknowledgement_status == "blocked_ack_noop" or .final_operator_acknowledgement_status == "blocked_acceptance_noop" or .final_operator_acknowledgement_status == "blocked_delivery_noop" or .final_operator_acknowledgement_status == "blocked_promotion_noop") and .acknowledgement_recorded == false and .acknowledgement_persisted == false and .acknowledgement_materialized == false and .acknowledgement_filesystem_written == false and .acknowledgement_delivered == false and .acknowledgement_accepted == false and .acknowledgement_identity_accepted == false and .acknowledgement_signature_accepted == false and .acknowledgement_final_state_promoted == false and .telegram_send_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_gate" \
  --arg final_ack_report_sha256 "$final_ack_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$FINAL_ACK_JSON" \
  '
  def blocked_fixture($id; $status; $reason; $extra):
    {
      id:$id,
      terminal_operator_decision_requested:false,
      terminal_operator_decision_status:$status,
      source_final_acknowledgement_present:true,
      source_final_acknowledgement_ready:true,
      terminal_decision_allowed:false,
      terminal_decision_request_accepted:false,
      terminal_decision_accepted:false,
      terminal_decision_recorded:false,
      terminal_decision_persisted:false,
      terminal_decision_materialized:false,
      terminal_decision_filesystem_written:false,
      terminal_decision_delivered:false,
      terminal_decision_channel_delivery_performed:false,
      terminal_decision_identity_accepted:false,
      terminal_decision_signature_accepted:false,
      terminal_decision_timestamp_accepted:false,
      terminal_decision_final_state_promoted:false,
      terminal_decision_completion_promoted:false,
      public_claim_requested:false,
      public_claim_accepted:false,
      public_claim_recorded:false,
      public_claim_persisted:false,
      public_claim_materialized:false,
      public_claim_promoted:false,
      public_ga_claimed:false,
      public_release_published:false,
      public_distribution_performed:false,
      public_artifact_written:false,
      release_artifact_written:false,
      terminal_operator_decision_noop_confirmed:true,
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
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      reason:$reason
    } + $extra;
  [
    blocked_fixture("provider-router-activation-result-receipt-terminal-decision-missing-final-ack"; "blocked_noop"; "source_final_operator_acknowledgement_report_required"; {source_final_acknowledgement_present:false, source_final_acknowledgement_ready:false, terminal_operator_decision_requested:true}),
    blocked_fixture("provider-router-activation-result-receipt-terminal-decision-request"; "blocked_decision_noop"; "terminal_operator_decision_request_shape_denied"; {terminal_operator_decision_requested:true}),
    blocked_fixture("provider-router-activation-result-receipt-terminal-decision-acceptance-request"; "blocked_acceptance_noop"; "terminal_operator_decision_acceptance_denied"; {terminal_operator_decision_requested:true, terminal_decision_acceptance_requested:true}),
    blocked_fixture("provider-router-activation-result-receipt-terminal-decision-recording-request"; "blocked_decision_noop"; "terminal_operator_decision_recording_denied"; {terminal_operator_decision_requested:true, terminal_decision_recording_requested:true}),
    blocked_fixture("provider-router-activation-result-receipt-terminal-decision-persistence-filesystem-write-request"; "blocked_decision_noop"; "terminal_operator_decision_persistence_filesystem_write_denied"; {terminal_operator_decision_requested:true, terminal_decision_persistence_requested:true, terminal_decision_filesystem_write_requested:true}),
    blocked_fixture("provider-router-activation-result-receipt-terminal-decision-identity-signature-request"; "blocked_acceptance_noop"; "operator_identity_signature_terminal_decision_acceptance_denied"; {terminal_operator_decision_requested:true, operator_identity_acceptance_requested:true, operator_signature_acceptance_requested:true, operator_timestamp_acceptance_requested:true}),
    blocked_fixture("provider-router-activation-result-receipt-terminal-decision-public-claim-request"; "blocked_public_claim_noop"; "public_claim_request_non_promotion_denied"; {terminal_operator_decision_requested:true, public_claim_requested:true, public_claim_promotion_requested:true}),
    blocked_fixture("provider-router-activation-result-receipt-terminal-decision-public-ga-release-request"; "blocked_promotion_noop"; "public_ga_release_publication_promotion_denied"; {terminal_operator_decision_requested:true, public_ga_claim_requested:true, public_release_publish_requested:true, public_distribution_requested:true, release_artifact_write_requested:true}),
    blocked_fixture("provider-router-activation-result-receipt-terminal-decision-activation-memory-provider-request"; "blocked_decision_noop"; "activation_memory_rollback_secret_provider_terminal_decision_denied"; {terminal_operator_decision_requested:true, activation_from_terminal_decision_requested:true, memory_write_terminal_decision_requested:true, rollback_terminal_decision_requested:true, secret_material_terminal_decision_requested:true, provider_prompt_terminal_decision_requested:true}),
    blocked_fixture("provider-router-activation-result-receipt-terminal-decision-external-public-install-request"; "blocked_promotion_noop"; "external_public_install_restart_active_binary_terminal_decision_denied"; {terminal_operator_decision_requested:true, external_send_decision_requested:true, public_claim_decision_requested:true, release_artifact_decision_requested:true, install_decision_requested:true, service_restart_decision_requested:true, active_binary_decision_requested:true})
  ] as $fixtures
  | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_terminal_operator_decision_public_claim_mode:"runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
    source_activation_command_result_receipt_final_operator_acknowledgement_gate:$source.gate,
    source_activation_command_result_receipt_final_operator_acknowledgement_ready:$source.runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready,
    source_activation_command_result_receipt_final_operator_acknowledgement_report_sha256:$final_ack_report_sha256,
    source_activation_command_result_receipt_operator_facing_summary_briefing_ready:$source.runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready,
    source_activation_command_result_receipt_operator_facing_summary_briefing_report_sha256:$source.source_activation_command_result_receipt_operator_facing_summary_briefing_report_sha256,
    source_activation_command_result_receipt_export_query_observability_ready:$source.runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_ready:$source.runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_ready:$source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    source_activation_command_result_receipt_cancellation_supersession_ready:$source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready,
    source_activation_command_result_receipt_ordering_monotonicity_ready:$source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_no_persistence_ready:true,
    required_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count:12,
    ready_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count:12,
    side_effect_free_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count:12,
    required_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count:10,
    activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count:($fixtures | length),
    blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count:($fixtures | length),
    noop_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count:($fixtures | length),
    allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count:0,
    accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count:0,
    activation_command_result_receipt_terminal_operator_decision_performed_count:0,
    activation_command_result_receipt_public_claim_promotion_performed_count:0,
    activation_command_result_receipt_terminal_operator_decision_allowed:false,
    activation_command_result_receipt_terminal_operator_decision_request_accepted:false,
    activation_command_result_receipt_terminal_operator_decision_accepted:false,
    activation_command_result_receipt_terminal_operator_decision_recorded:false,
    activation_command_result_receipt_terminal_operator_decision_persisted:false,
    activation_command_result_receipt_terminal_operator_decision_materialized:false,
    activation_command_result_receipt_terminal_operator_decision_filesystem_written:false,
    activation_command_result_receipt_terminal_operator_decision_delivered:false,
    activation_command_result_receipt_terminal_operator_decision_channel_delivery_performed:false,
    activation_command_result_receipt_terminal_operator_decision_identity_accepted:false,
    activation_command_result_receipt_terminal_operator_decision_signature_accepted:false,
    activation_command_result_receipt_terminal_operator_decision_timestamp_accepted:false,
    activation_command_result_receipt_terminal_operator_decision_final_state_promoted:false,
    activation_command_result_receipt_terminal_operator_decision_completion_promoted:false,
    activation_command_result_receipt_public_claim_requested:false,
    activation_command_result_receipt_public_claim_accepted:false,
    activation_command_result_receipt_public_claim_recorded:false,
    activation_command_result_receipt_public_claim_persisted:false,
    activation_command_result_receipt_public_claim_materialized:false,
    activation_command_result_receipt_public_claim_promoted:false,
    activation_command_result_receipt_public_ga_claimed:false,
    activation_command_result_receipt_public_release_published:false,
    activation_command_result_receipt_public_distribution_performed:false,
    activation_command_result_receipt_public_artifact_written:false,
    telegram_send_performed:false,
    channel_send_performed:false,
    external_send_performed:false,
    activation_command_result_receipt_recorded:false,
    activation_command_result_receipt_persisted:false,
    activation_command_result_receipt_accepted:false,
    activation_command_result_receipt_materialized:false,
    activation_command_result_receipt_filesystem_written:false,
    activation_command_completion_ack_recorded:false,
    activation_allowed_by_result_receipt_terminal_operator_decision:false,
    activation_allowed_by_result_receipt_final_operator_acknowledgement:false,
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
    public_release_published:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    public_artifact_written:false,
    public_distribution_performed:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    activation_command_result_receipt_terminal_operator_decision_public_claim_surfaces:[
      "source_final_operator_acknowledgement_report_required",
      "terminal_operator_decision_request_shape_denied",
      "terminal_operator_decision_acceptance_denied",
      "terminal_operator_decision_recording_denied",
      "terminal_operator_decision_persistence_denied",
      "terminal_operator_decision_materialization_denied",
      "operator_identity_signature_terminal_decision_acceptance_denied",
      "terminal_operator_decision_delivery_denied",
      "public_claim_request_non_promotion_denied",
      "public_ga_release_publication_promotion_denied",
      "activation_from_terminal_operator_decision_denied",
      "external_public_install_restart_active_binary_terminal_decision_denied"
    ],
    activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures:$fixtures,
    denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim:[
      "source_final_operator_acknowledgement_report_required",
      "terminal_operator_decision_request_acceptance_denied",
      "terminal_operator_decision_acceptance_denied",
      "terminal_operator_decision_recording_denied",
      "terminal_operator_decision_persistence_denied",
      "terminal_operator_decision_materialization_denied",
      "terminal_operator_decision_filesystem_write_denied",
      "operator_identity_signature_terminal_decision_acceptance_denied",
      "terminal_operator_decision_delivery_denied",
      "telegram_send_denied",
      "public_claim_non_promotion_denied",
      "public_ga_release_publication_promotion_denied",
      "activation_from_terminal_operator_decision_denied",
      "memory_write_terminal_decision_denied",
      "rollback_terminal_decision_denied",
      "secret_material_terminal_decision_denied",
      "provider_prompt_terminal_decision_denied",
      "external_public_install_restart_active_binary_terminal_decision_denied"
    ],
    side_effects:{
      activation_command_result_receipt_terminal_operator_decision_recorded:false,
      activation_command_result_receipt_terminal_operator_decision_persisted:false,
      activation_command_result_receipt_terminal_operator_decision_materialized:false,
      activation_command_result_receipt_terminal_operator_decision_filesystem_written:false,
      activation_command_result_receipt_terminal_operator_decision_delivered:false,
      activation_command_result_receipt_terminal_operator_decision_channel_delivery_performed:false,
      activation_command_result_receipt_terminal_operator_decision_identity_accepted:false,
      activation_command_result_receipt_terminal_operator_decision_signature_accepted:false,
      activation_command_result_receipt_terminal_operator_decision_final_state_promoted:false,
      activation_command_result_receipt_terminal_operator_decision_completion_promoted:false,
      activation_command_result_receipt_public_claim_recorded:false,
      activation_command_result_receipt_public_claim_persisted:false,
      activation_command_result_receipt_public_claim_materialized:false,
      activation_command_result_receipt_public_claim_promoted:false,
      telegram_send_performed:false,
      activation_command_result_receipt_final_operator_acknowledgement_recorded:false,
      activation_command_result_receipt_final_operator_acknowledgement_persisted:false,
      activation_command_result_receipt_final_operator_acknowledgement_materialized:false,
      activation_command_result_receipt_final_operator_acknowledgement_filesystem_written:false,
      activation_command_result_receipt_final_operator_acknowledgement_delivered:false,
      activation_command_result_receipt_operator_final_acceptance_recorded:false,
      activation_command_result_receipt_operator_final_acceptance_persisted:false,
      activation_command_result_receipt_operator_summary_recorded:false,
      activation_command_result_receipt_operator_summary_persisted:false,
      activation_command_result_receipt_operator_briefing_recorded:false,
      activation_command_result_receipt_operator_briefing_persisted:false,
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
      public_distribution_performed:false,
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
  and .runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and .required_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count == 12
  and .ready_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count == 12
  and .required_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
  and .activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
  and .blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
  and .noop_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
  and .allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 0
  and .accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 0
  and .activation_command_result_receipt_terminal_operator_decision_performed_count == 0
  and .activation_command_result_receipt_public_claim_promotion_performed_count == 0
  and .activation_command_result_receipt_terminal_operator_decision_recorded == false
  and .activation_command_result_receipt_terminal_operator_decision_persisted == false
  and .activation_command_result_receipt_terminal_operator_decision_materialized == false
  and .activation_command_result_receipt_terminal_operator_decision_filesystem_written == false
  and .activation_command_result_receipt_terminal_operator_decision_delivered == false
  and .activation_command_result_receipt_terminal_operator_decision_identity_accepted == false
  and .activation_command_result_receipt_terminal_operator_decision_signature_accepted == false
  and .activation_command_result_receipt_terminal_operator_decision_final_state_promoted == false
  and .activation_command_result_receipt_terminal_operator_decision_completion_promoted == false
  and .activation_command_result_receipt_public_claim_recorded == false
  and .activation_command_result_receipt_public_claim_persisted == false
  and .activation_command_result_receipt_public_claim_materialized == false
  and .activation_command_result_receipt_public_claim_promoted == false
  and .activation_command_result_receipt_public_ga_claimed == false
  and .activation_command_result_receipt_public_release_published == false
  and .activation_command_result_receipt_public_distribution_performed == false
  and .activation_command_result_receipt_public_artifact_written == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .activation_allowed_by_result_receipt_terminal_operator_decision == false
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
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures | length) == 10
  and (.activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures | all((.terminal_operator_decision_status == "blocked_noop" or .terminal_operator_decision_status == "blocked_decision_noop" or .terminal_operator_decision_status == "blocked_acceptance_noop" or .terminal_operator_decision_status == "blocked_public_claim_noop" or .terminal_operator_decision_status == "blocked_promotion_noop") and .terminal_decision_recorded == false and .terminal_decision_persisted == false and .terminal_decision_materialized == false and .terminal_decision_filesystem_written == false and .terminal_decision_delivered == false and .terminal_decision_accepted == false and .terminal_decision_identity_accepted == false and .terminal_decision_signature_accepted == false and .terminal_decision_final_state_promoted == false and .public_claim_promoted == false and .public_release_published == false and .public_ga_claimed == false and .telegram_send_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .terminal_operator_decision_noop_confirmed == true))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt terminal operator decision public-claim non-promotion denial gate passed"
